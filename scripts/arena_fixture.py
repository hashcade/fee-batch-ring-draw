"""Exercise the compiled per-instance arena edit, not Unity's combat flow."""

from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class ArenaFixture(CollectorFixture):
  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.heap = game + 0x7D00000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    if name == "CREATE_COMBAT":
      self.original_calls += 1
    elif name == "WRITE_BARRIER":
      slot = self.cpu.reg_read(reg.UC_ARM64_REG_X0)
      assert self.array + 0x20 <= slot < self.array + 0x90
      assert self.pointer(slot) == self.cpu.reg_read(reg.UC_ARM64_REG_X1)
      self.writes += 1
    else:
      return False
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack):
    self.active = True
    functions = [0x1CA38C0, 0x1CA3940, 0, 0, 0x1CA39A0, 0x1CA3A20,
      0x1CA3A80, 0x1CA3B00, 0x1CA3B80, 0x1CA3C00, 0, 0, 0x1CA3DB0, 0]
    kinds = [11, 11, 10, 15, 11, 11, 10, 11, 11, 10, 10, 15, 11, 0]
    kept = [2, 3, 7, 8, 9, 10, 11, 13] + [13] * 6
    for case in ("paid", "ordinary", "ordinary_exp", "ordinary_special", "special", "zero", "started", "changed", "wrong_target", "wrong_calculator", "missing", "size"):
      parent, child, calculator, simulation = [self.alloc() for _ in range(4)]
      emblem = int(not case.startswith("ordinary"))
      special = int(case in ("special", "ordinary_special"))
      exp = 0 if case in ("zero", "ordinary", "ordinary_special") else 6000
      self.qword(parent + 0x38, 0 if case == "missing" else child)
      self.write(parent + 0x88, bytes([emblem, special]))
      self.integer(parent + 0x8C, emblem)
      self.qword(parent + 0xB0, calculator)
      self.qword(parent + 0xB8, simulation)
      self.qword(child + 0x30, parent)
      self.integer(child + 0x18, int(case == "started"))
      self.qword(child + 0x88, simulation if case == "wrong_calculator" else calculator)
      self.qword(child + 0x90, simulation)
      self.integer(child + 0x98, exp)
      self.write(child + 0x9C, bytes([emblem, special]))
      self.array = self.alloc()
      self.qword(child + 0x10, self.array)
      self.qword(self.array + 0x18, 13 if case == "size" else 14)
      descriptors, snapshots = [], []
      for index, (kind, function) in enumerate(zip(kinds, functions)):
        descriptor, callback = self.alloc(), self.alloc()
        self.integer(descriptor + 0x10, 7 if case == "changed" and index == 3 else kind)
        self.qword(descriptor + 0x18, callback)
        self.qword(callback + 0x10, self.game + function)
        self.qword(callback + 0x20, parent if case == "wrong_target" else child)
        self.qword(self.array + 0x20 + index * 8, descriptor)
        descriptors.append(descriptor)
        snapshots.extend((value, bytes(self.cpu.mem_read(value, 0x100))) for value in (descriptor, callback))
      snapshots.extend((value, bytes(self.cpu.mem_read(value, 0x100))) for value in (parent, child, calculator, simulation))
      self.original_calls, self.writes = 0, 0
      args = (parent, calculator, simulation, emblem, special, exp, 0)
      for index in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(replacement, stop + 4, count=100_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      assert self.original_calls == 1
      actual = [self.pointer(self.array + 0x20 + index * 8) for index in range(14)]
      eligible = case in ("paid", "ordinary")
      assert actual == ([descriptors[index] for index in kept] if eligible else descriptors), case
      assert self.writes == (14 if eligible else 0), case
      assert all(bytes(self.cpu.mem_read(address, len(before))) == before for address, before in snapshots), case
    self.active = False
