"""Exercise compiled single/batch dialog callbacks against native ABI models."""

import re
import struct
from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class MeldDialogFixture(CollectorFixture):
  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.functions = {game + int(value, 16): name for name, value in re.findall(
      r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", common + profile)}
    self.functions[game + 0x00492060] = "INITIALIZE_METADATA"
    self.heap = game + 0x7900000
    self.tls = {}

  def handle(self, pc, import_name):
    if not self.active:
      return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(6)]
    x0, x1 = args[:2]
    result = None
    if name == "pthread_key_create":
      key = len(self.tls) + 1
      self.cpu.mem_write(x0, struct.pack("<i", key))
      self.tls[key] = 0
      result = 0
    elif name == "pthread_key_delete": result = 0
    elif name == "pthread_getspecific": result = self.tls.get(x0, 0)
    elif name == "pthread_setspecific": self.tls[x0], result = x1, 0
    elif name in ("malloc", "ALLOCATE_OBJECT"):
      result = self.alloc(x0 if name == "malloc" else 0x600)
      if name == "ALLOCATE_OBJECT": self.qword(result, x0)
    elif name in ("free", "INITIALIZE_METADATA", "WRITE_BARRIER", "SET_MENU_ROWS"):
      pass
    elif name == "memcpy":
      self.write(x0, bytes(self.cpu.mem_read(x1, args[2])))
    elif name == "STRING_NEW_SIZE":
      result = self.alloc(x0 * 2 + 0x20)
      self.integer(result + 0x10, x0)
    elif name == "NEXT_RANK": result = self.output
    elif name == "SINGLETON_GET": result = self.user
    elif name == "STOCK_COUNT": result = 6 if x0 == self.source and x1 == 0 else 0
    elif name == "ALL_COUNT": result = 0
    elif name == "CAN_ADD": result = 1
    elif name == "GET_LANGUAGE": result = 10
    elif name == "MESS_GET": result = self.string("Ring")
    elif name == "ON_DECIDE":
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.owner)
      self.cpu.reg_write(reg.UC_ARM64_REG_X1, self.string("original prompt"))
      self.cpu.reg_write(reg.UC_ARM64_REG_X2, self.yes)
      self.cpu.reg_write(reg.UC_ARM64_REG_X3, self.no)
      self.cpu.reg_write(reg.UC_ARM64_REG_X4, 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.replacements["YES_NO_DIALOG"])
      return True
    elif name == "YES_ITEM_CTOR":
      self.qword(x0 + 0x70, x1)
    elif name == "YES_NO_DIALOG":
      self.prompt = self.read_string(x1)
      self.full = self.list([args[2], self.extra, args[3]] if self.extra else args[2:4])
      self.qword(self.dialog + 0x80, self.full)
      result = self.dialog
    elif name == "LIST_ADD": self.append(x0, x1)
    elif name == "YES_ITEM_CALL":
      self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.shim)
      return True
    elif name in ("ON_YES", "EXECUTE_MERGE", "UNLOAD"):
      pass
    elif name == "CREATE_DEMO": self.demos += 1
    else:
      return False
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacements, stop, stack):
    self.replacements, self.active = replacements, True
    self.owner, self.source, self.output, self.user, self.yes, self.no, self.dialog = (
      self.alloc() for _ in range(7))
    self.qword(self.source + 0x20, self.string("RNID_Test_C"))
    self.qword(self.output + 0x20, self.string("RNID_Test_B"))
    self.qword(self.output + 0x28, self.string("MID_Test"))
    self.integer(self.output + 0x4C, 1)
    self.integer(self.user + 0x6C, 1000)
    self.metadata(0x062840F0)
    self.metadata(0x06286A70)
    yes_class = self.alloc()
    self.integer(yes_class + 0xF8, 0x78)
    self.write(yes_class + 0x126, struct.pack("<H", 27))
    self.qword(yes_class + 0x138 + 18 * 16, self.game + 0x021FA240)
    self.write(yes_class + 0x138 + 19 * 16, struct.pack("<QQ", self.alloc(), self.alloc()))
    original_class = bytes(self.cpu.mem_read(yes_class, 0x138 + 27 * 16))
    self.qword(self.yes, yes_class)
    self.qword(self.yes + 0x70, self.alloc())
    self.shim = self.alloc()
    # Keep the real callback scope live while invoking the compiled OnYes hook.
    words = [0xA9BF7BFD, 0x580000E0, 0x58000110, 0xD63F0200,
      0x52801080, 0xA8C17BFD, 0xD65F03C0, 0xD503201F]
    self.write(self.shim, struct.pack("<8IQQ", *words, self.owner, replacements["ON_YES"]))

    def call(address, args=()):
      for i in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{i}"), args[i] if i < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(address, stop + 4, count=500_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop

    for batch, extended in ((False, False), (True, False), (True, True), (False, False)):
      self.extra = self.alloc() if extended else 0
      call(replacements["ON_DECIDE"], (self.owner, self.source, 2, 100, 0, 0, 0))
      items = self.values(self.full)
      assert len(items) == (4 if extended else 3) and items[-1] == self.no
      if extended: assert items[1] == self.extra
      for item in (items[0], items[-2]):
        private = self.pointer(item)
        for slot in (12, 13, 19):
          start = 0x138 + slot * 16
          assert bytes(self.cpu.mem_read(private + start, 16)) == original_class[start:start + 16], "single/batch choices must retain native selection and B callbacks"
      assert bytes(self.cpu.mem_read(yes_class, len(original_class))) == original_class
      assert self.read_string(self.pointer(items[0] + 0x68)) == "合成 1 枚"
      assert self.read_string(self.pointer(items[-2] + 0x68)) == "合成 3 枚"
      assert self.prompt.count("Ring") == 1 and "×600" not in self.prompt
      selected = items[-2] if batch else items[0]
      callback = self.pointer(self.pointer(selected) + 0x138 + 18 * 16)
      call(callback, (selected, 0))
      call(replacements["EXECUTE_MERGE"], (self.owner, 0))
      self.demos = 0
      call(replacements["CREATE_DEMO"], (self.owner, 0))
      assert self.demos == (0 if batch else 1)
      call(replacements["UNLOAD"], (self.owner, 0))
    self.active = False
