"""Exercise compiled facility selection and native cleaning settlement boundaries."""

import struct
from unicorn import arm64_const as reg
from meld_dialog_fixture import MeldDialogFixture


class CleaningFixture(MeldDialogFixture):
  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.heap = game + 0x7B00000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(8)]
    x0, x1 = args[:2]
    result = None
    if name == "CREATE_MENU":
      self.full = self.list(self.originals)
      self.qword(self.menu + 0x80, self.full)
      self.qword(self.owner + 0x38, self.menu)
    elif name == "ITEM_CTOR":
      for offset, value in ((0x68, x1), (0x70, args[2]), (0x78, args[3])):
        self.qword(x0 + offset, value)
      kind = args[6] & 0xFFFFFFFF
      self.write(x0 + 0x84, struct.pack("<I", kind))
    elif name == "ITEM_DECIDE":
      self.selected_god, self.selected_kind = self.pointer(x0 + 0x68), self.number(x0 + 0x84)
      self.decisions += 1
      result = 129
    elif name == "ITEM_NAME": result = self.string("original ring name")
    elif name == "CREATE_CLEANING": self.scenes += 1
    elif name == "END_SEQUENCE": pass
    elif name == "GET_GOD": result = self.selected_god
    elif name == "GET_TYPE": result = self.selected_kind
    elif name == "GET_UNIT": result = self.cleaner
    elif name == "SET_GOD": self.selected_god = x0
    elif name == "SET_TYPE": self.selected_kind = x0 & 0xFFFFFFFF
    elif name == "GET_DIRTY": result = self.cpu.mem_read(x0 + 0x4C, 1)[0]
    elif name == "SET_DIRTY":
      if x0 != self.failed_setter: self.write(x0 + 0x4C, bytes([x1]))
    elif name == "GET_STATE": result = self.states.get(x0, 0)
    elif name == "ADD_BOND_EXP":
      assert x0 == 0 and x1 == 1 and args[2] == 1
      self.rewards.append((self.selected_god, self.cleaner))
    else:
      return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacements, stop, stack):
    self.active = True
    self.owner, self.menu, self.cleaner = (self.alloc() for _ in range(3))
    self.qword(self.menu + 0x30, self.owner)
    self.integer(self.menu + 0xB8, 7)
    self.selected_god, self.selected_kind = 0, -1
    self.states, self.rewards = {}, []
    self.scenes, self.decisions, self.failed_setter = 0, 0, 0
    self.metadata(0x06286A70)
    item_class = self.alloc()
    self.integer(item_class + 0xF8, 0x88)
    self.write(item_class + 0x126, struct.pack("<H", 30))
    self.qword(item_class + 0x138 + 18 * 16, self.game + 0x02347210)
    self.gods = [self.alloc() for _ in range(5)]
    self.originals = []
    for god, kind in zip((self.gods[0], self.gods[1], self.gods[1], *self.gods[2:]), (-1, 0, 1, -1, 3, -1)):
      item = self.alloc()
      self.qword(item, item_class)
      self.qword(item + 0x68, god)
      self.integer(item + 0x84, kind)
      self.originals.append(item)

    def call(address, *args):
      for index in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(address, stop + 4, count=500_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

    def select_all():
      for god in self.gods: self.write(god + 0x4B, b"\0\x80")
      self.write(self.gods[2] + 0x4C, b"\0")
      call(replacements["CREATE_MENU"], self.owner, 0)
      items = self.values(self.full)
      assert items[1:] == self.originals
      item = items[0]
      self.qword(item + 0x10, self.menu)
      label = call(replacements["ITEM_NAME"], item, 0)
      assert self.read_string(label) == "全部清洁"
      before = self.decisions
      callback = self.pointer(self.pointer(item) + 0x138 + 18 * 16)
      assert call(callback, item, 0) == 129
      assert self.decisions == before + 1
      return self.selected_god, self.selected_kind

    before = select_all()
    self.write(self.gods[3] + 0x4B, b"\1")
    self.states[self.gods[4]] = 2
    call(replacements["CREATE_CLEANING"], self.owner, 0)
    assert self.rewards == [(self.gods[0], self.cleaner), (self.gods[1], self.cleaner)]
    assert self.scenes == 0
    assert (self.selected_god, self.selected_kind & 0xFFFFFFFF) == (before[0], before[1] & 0xFFFFFFFF)
    assert [self.cpu.mem_read(god + 0x4C, 1)[0] for god in self.gods] == [0, 0, 0, 128, 128]
    call(replacements["CREATE_CLEANING"], self.owner, 0)
    assert self.scenes == 1 and len(self.rewards) == 2
    select_all()
    call(replacements["END_SEQUENCE"], self.owner, 0)
    call(replacements["CREATE_CLEANING"], self.owner, 0)
    assert self.scenes == 2 and len(self.rewards) == 2
    select_all()
    self.failed_setter = self.gods[0]
    self.write(self.gods[3] + 0x4B, b"\1")
    call(replacements["CREATE_CLEANING"], self.owner, 0)
    assert self.cpu.mem_read(self.gods[0] + 0x4C, 1)[0] == 128
    assert self.rewards[-1] == (self.gods[1], self.cleaner) and len(self.rewards) == 3
    select_all()
    self.cleaner = 0
    call(replacements["CREATE_CLEANING"], self.owner, 0)
    assert self.scenes == 3 and len(self.rewards) == 3
    self.active = False
