"""Execute the shop-local call-site bridge and modeled native stock boundaries."""

import re
import struct
from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class ShopFixture(CollectorFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0xA800000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(5)]
    if pc in (self.game + 0x031FD0F4, self.game + 0x031FD620):
      assert self.cpu.reg_read(reg.UC_ARM64_REG_SP) == self.stack
      assert self.cpu.reg_read(reg.UC_ARM64_REG_X30) == self.game + 0x031FD60C
      for index, expected in self.saved.items():
        assert self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{index}")) == expected
      for index in range(8, 16):
        assert self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_D{index}")) == 0x1000 + index
      self.unlimited = pc == self.game + 0x031FD0F4
      assert bool(self.cpu.reg_read(reg.UC_ARM64_REG_NZCV) & (1 << 30)) == self.unlimited
      self.effective = self.cpu.reg_read(reg.UC_ARM64_REG_W8)
      self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.stop)
      return True
    if name == "LOOKUP":
      assert args[:2] == [self.dictionary, self.key]
      self.reads += 1
      found = self.count != -999
      self.cpu.mem_write(args[2], struct.pack("<iiQ", self.count if found else 0, 17 if found else 0, 0))
      # The native BL may destroy caller-saved integer registers.
      for index in range(1, 19):
        self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), 0xDEAD0000 + index)
      result = int(found)
    elif name in ("STOCK", "INFINITY", "PURCHASE"):
      self.calls.append(name)
      result = 7 if name == "STOCK" else 0
    else:
      return super().handle(pc, import_name)
    self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def run(self, address, args, list_method=None):
    for index in range(8):
      self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
    self.saved = {index: 0xBEEF0000 + index for index in range(19, 30)}
    if list_method is not None:
      self.saved[19] = list_method + 0x18
    for index, value in self.saved.items():
      self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), value)
    for index in range(8, 16):
      self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_D{index}"), 0x1000 + index)
    self.cpu.reg_write(reg.UC_ARM64_REG_SP, self.stack)
    self.cpu.reg_write(reg.UC_ARM64_REG_X30, self.stop)
    self.cpu.emu_start(address, self.stop + 4, count=100_000)
    assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == self.stop
    return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

  def exercise(self, replacements, stop, stack, profile):
    self.active, self.stop, self.stack = True, stop, stack - 0x100
    self.key, self.dictionary = [self.alloc() for _ in range(2)]
    self.value = self.stack + 0x28
    methods = {}
    for kind, values in re.findall(r"pub const (\w+_METHODS): \[usize; 3\] = \[([^\]]+)\]", profile):
      methods[kind] = [self.metadata(int(value, 16)) for value in re.findall(r"0x[0-9A-Fa-f]+", values)]
    other = self.alloc()
    self.count, self.reads, self.calls = 0, 0, []
    assert set(replacements) == {"LIST_LOOKUP", "STOCK", "INFINITY", "PURCHASE"}
    site = self.game + 0x031FD608
    # Model A64HookFunction's aligned far jump, including overwritten native
    # post-call instructions. Enter through the game site, not the Rust helper.
    self.write(site, struct.pack("<IIQ", 0x58000051, 0xD61F0220, replacements["LIST_LOOKUP"]))
    for kind, args, expected in (
      ("STOCK", [self.key, 0], 0xFFFFFFFF),
      ("INFINITY", [self.key, 0], 1),
      ("PURCHASE", [self.key, 3, 0, 0], None),
    ):
      for method in methods[kind + "_METHODS"]:
        before = len(self.calls)
        result = self.run(replacements[kind], args + [method])
        assert len(self.calls) == before
        if expected is not None: assert result == expected
      self.run(replacements[kind], args + [other])
      assert self.calls[-1] == kind
    for method in methods["CONTENT_METHODS"] + [other, 0]:
      for count in (-999, -1, 0, 1, 99):
        self.count = count
        before = self.reads
        result = self.run(site, [self.dictionary, self.key, self.value, 0], method)
        assert self.reads == before + 1
        allowed = method in methods["CONTENT_METHODS"]
        expected = -1 if allowed else (count if count != -999 else 0)
        assert self.effective == expected & 0xFFFFFFFF
        assert self.unlimited == (expected == -1)
        assert result == int(allowed or count != -999)
        assert self.number(self.value) == expected
        assert self.number(self.value + 4) == (17 if count != -999 else 0)
        assert self.pointer(self.value + 8) == 0
        # Same-key reads outside this site never enter plugin code.
        self.run(self.game + 0x03C742D0, [self.dictionary, self.key, self.value, 0])
        assert self.number(self.value) == (count if count != -999 else 0)
    self.active = False
