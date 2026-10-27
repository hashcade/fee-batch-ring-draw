"""Check compiled dialogue scoping and native jump dispatch, not Lua completion."""

from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class BondTalkFixture(CollectorFixture):
  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.heap = game + 0x7E00000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    this = self.cpu.reg_read(reg.UC_ARM64_REG_X0)
    if name == "INITIALIZE_METADATA": pass
    elif name == "GET_LABEL":
      assert this == self.talk
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.label)
    elif name == "JUMP_LABEL":
      assert this == self.talk and self.label == 3
      assert self.cpu.reg_read(reg.UC_ARM64_REG_X1) == 6
      self.jumps += 1
    elif name == "TALK_TICK":
      assert this == self.talk
      self.original_calls += 1
    else: return False
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack):
    self.active = True
    dialog_class = self.metadata(0x062A3CB8)
    combat_class = self.metadata(0x06291678)
    order_class = self.metadata(0x062917C0)
    cases = ("paid", "nested", "repeat", "support", "story", "outside", "ordinary",
      "special", "zero", "wrong_combat", "wrong_order", "stale", "cycle", "no_unit", "no_god")
    for case in cases:
      self.talk, dialog, combat, order, unit, god = [self.alloc() for _ in range(6)]
      self.qword(self.talk + 0x30, dialog)
      self.qword(dialog, dialog_class)
      self.qword(dialog + 0x70, 0 if case == "no_unit" else unit)
      self.qword(dialog + 0x78, unit if case == "support" else 0)
      self.qword(dialog + 0x88, 0 if case == "no_god" else god)
      self.qword(dialog + 0x30, combat)
      self.qword(combat, 0 if case == "wrong_combat" else combat_class)
      self.qword(combat + 0x30, order)
      self.qword(order, 0 if case == "wrong_order" else order_class)
      self.qword(order + 0x38, 0 if case == "stale" else combat)
      self.integer(order + 0x8C, int(case != "ordinary"))
      self.write(order + 0x88, bytes([int(case != "ordinary"), int(case == "special")]))
      self.write(combat + 0x9C, bytes([int(case != "ordinary"), int(case == "special")]))
      self.integer(combat + 0x98, 0 if case == "zero" else 6000)
      if case == "story": self.qword(dialog, combat_class)
      if case == "outside": self.qword(dialog + 0x30, 0)
      if case == "cycle":
        self.qword(dialog, combat_class)
        self.qword(dialog + 0x30, dialog)
      if case == "nested":
        child = self.alloc()
        self.qword(child + 0x30, dialog)
        self.qword(self.talk + 0x30, child)
      snapshots = [(value, bytes(self.cpu.mem_read(value, 0x100))) for value in
        (self.talk, dialog, combat, order, unit, god)]
      for self.label in (0, 1, 2, 3, 4, 6, 7, 8, 9):
        self.jumps, self.original_calls = 0, 0
        for index in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), self.talk if index == 0 else 0)
        self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
        self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
        self.cpu.emu_start(replacement, stop + 4, count=100_000)
        assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
        skip = case in ("paid", "nested", "repeat") and self.label == 3
        assert self.jumps == int(skip), (case, self.label)
        assert self.original_calls == int(not skip), (case, self.label)
        assert all(bytes(self.cpu.mem_read(address, len(before))) == before for address, before in snapshots)
    self.active = False
