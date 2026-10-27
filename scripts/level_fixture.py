"""Check compiled read-only level labels, not Unity layout or rendering."""

from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class LevelFixture(CollectorFixture):
  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.heap = game + 0x8200000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    this = self.cpu.reg_read(reg.UC_ARM64_REG_X0)
    if name == "SET_LEVEL":
      self.original_calls += 1
    elif name == "ENHANCED_LEVEL":
      assert this == self.unit and self.original_calls == 1
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.enhanced)
    elif name == "GET_LANGUAGE":
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.language)
    elif name == "SET_TEXT":
      assert this in (self.heading, self.value) and self.original_calls == 1
      self.labels[this] = self.read_string(self.cpu.reg_read(reg.UC_ARM64_REG_X1))
    elif name == "STRING_NEW_SIZE":
      length = this
      string = self.alloc(0x20 + length * 2)
      self.integer(string + 0x10, length)
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, string)
    else: return False
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack):
    self.active = True
    for case in ("normal", "initial", "enhanced", "cap", "negative", "zero", "missing", "null", "invalid_display"):
      for self.language in range(12):
        setter, self.unit, original, parameter, self.heading, self.value = [self.alloc() for _ in range(6)]
        self.qword(setter + 0x188, 0 if case == "missing" else parameter)
        self.qword(parameter + 0x20, self.heading)
        self.qword(parameter + 0x28, self.value)
        level = 0 if case == "zero" else 99 if case == "cap" else 20
        internal = 255 if case == "negative" else 0 if case == "initial" else 50
        self.write(original + 0x89, bytes([level, 77]))
        self.write(original + 0x158, bytes([internal]))
        self.enhanced = 0 if case == "invalid_display" else 25 if case == "enhanced" else level
        snapshots = [(value, bytes(self.cpu.mem_read(value, 0x200))) for value in
          (setter, self.unit, original, parameter, self.heading, self.value)]
        self.original_calls, self.labels = 0, {}
        args = (0 if case == "null" else setter, self.unit, original, 0)
        for index in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
        self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
        self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
        self.cpu.emu_start(replacement, stop + 4, count=100_000)
        assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
        assert self.original_calls == 1
        expected = case in ("normal", "initial", "enhanced", "cap")
        assert len(self.labels) == (2 if expected else 0), case
        if expected:
          assert self.labels[self.value] == f"{self.enhanced}/{level + internal}", case
          assert "/" in self.labels[self.heading]
        assert all(bytes(self.cpu.mem_read(address, len(before))) == before for address, before in snapshots), case
    self.active = False
