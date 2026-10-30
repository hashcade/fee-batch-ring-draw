"""Compiled reward hooks with the pinned native AddExp body, not Unity rendering."""

import re
import struct
from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class GainsFixture(CollectorFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0xAD00000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(5)]
    if name == "ADD_EXP":
      self.chunks.append((args[0], args[1] & 0xFFFFFFFF))
      return False  # Execute the actual pinned native function below.
    if name == "GROWTH_PREPARE":
      assert args[0] == self.growth
      if self.cpu.mem_read(self.growth + 0x94, 1)[0]:
        self.growth_calls = 0
        self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
        return True
      self.write(self.grow + 0x89, bytes([self.old_level + 1]))
      self.growth_calls = 1
      self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
      return True
    if name == "LEVEL_UP":
      assert args[:3] == [self.grow, 2, 0], "grow the native preview, not the real unit"
      self.write(self.grow + 0x89, bytes([self.cpu.mem_read(self.grow + 0x89, 1)[0] + 1]))
      self.growth_calls += 1
      self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
      return True
    if name == "CREATE_EXP":
      parent, unit, exp, sp, method = args
      self.entries.append((parent, unit, exp, sp, method))
      self.qword(self.sequence + 0x70, unit)
      self.integer(self.sequence + 0x78, exp)
      self.integer(self.sequence + 0x7C, sp)
      if not self.skip or not unit:
        self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
        return True
    elif name == "EXP_TICK":
      assert args[0] == self.sequence
      unit = self.pointer(self.sequence + 0x70)
      pending_exp, pending_sp = (self.number(self.sequence + offset) for offset in (0x78, 0x7C))
      exp = pending_exp if self.skip else min(1, max(0, pending_exp))
      sp = pending_sp if self.skip else min(1, max(0, pending_sp))
    else:
      return False
    # Model the native reward flow around AddExp. Only AddExp itself executes
    # the original game instructions; SP settlement/history/UI are boundaries.
    owned = struct.unpack("<h", self.cpu.mem_read(unit + 0x1BE, 2))[0]
    self.write(unit + 0x1BE, struct.pack("<h", min(9999, owned + sp)))
    self.integer(self.sequence + 0x78, max(0, self.number(self.sequence + 0x78) - exp))
    self.integer(self.sequence + 0x7C, max(0, self.number(self.sequence + 0x7C) - sp))
    self.cpu.reg_write(reg.UC_ARM64_REG_X0, unit)
    self.cpu.reg_write(reg.UC_ARM64_REG_X1, exp)
    self.cpu.reg_write(reg.UC_ARM64_REG_X2, 0)
    # Tail-dispatch into the compiled AddExp hook while its EXP scope is active.
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.hooks[0x01A39D40])
    return True

  def exercise(self, hooks, stop, stack, configure, profile):
    self.active, self.hooks = True, hooks
    self.unit, self.other, self.job, self.sequence, self.parent = [self.alloc() for _ in range(5)]
    self.qword(self.unit + 0x48, self.job)
    self.qword(self.other + 0x48, self.job)
    # Execute every original AddExp instruction, all covered by NSO signatures.
    windows = {int(address, 16): [int(word, 16) for word in re.findall(r"0x[0-9A-Fa-f]+", words)]
      for address, words in re.findall(r"\((0x[0-9A-Fa-f]+),\s*\[([^\]]+)\]\)", profile)}
    words = sum([windows[offset] for offset in range(0x01A39D40, 0x01A39DA0, 0x10)], [])[:22]
    self.write(self.game + 0x01A39D40, struct.pack("<22I", *words))

    def call(address, *args):
      for i in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{i}"), args[i] if i < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(address, stop + 4, count=1_000_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop

    def state(level=1, exp=75, sp=100, maximum=20):
      self.write(self.unit + 0x89, bytes([level, exp]))
      self.write(self.job + 0x70, bytes([maximum]))
      self.write(self.unit + 0x1BE, struct.pack("<h", sp))
      self.entries, self.chunks = [], []

    def run(exp, sp): call(hooks[0x024E8C20], self.parent, self.unit, exp, sp, 0x1234)
    def configure_rates(exp, sp):
      self.active = False
      configure(exp, sp)
      self.active = True

    self.growth, self.grow = self.alloc(), self.alloc()
    self.qword(self.growth + 0x80, self.unit)
    self.qword(self.growth + 0x88, self.grow)
    def growth(old_level):
      self.old_level = old_level
      before = bytes(self.cpu.mem_read(self.unit, 0x600))
      call(hooks[0x01BE1260], self.growth, 0x3456)
      assert self.cpu.mem_read(self.grow + 0x89, 1)[0] == self.cpu.mem_read(self.unit + 0x89, 1)[0]
      assert self.growth_calls == self.cpu.mem_read(self.unit + 0x89, 1)[0] - old_level
      assert bytes(self.cpu.mem_read(self.unit, 0x600)) == before

    for multiplier in (1, 2, 3, 5, 10):
      configure_rates(multiplier, 1)
      state()
      self.skip = True
      run(100, 10)
      assert self.entries == [(self.parent, self.unit, multiplier * 100, 10, 0x1234)]
      assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [1 + multiplier, 75]
      assert struct.unpack("<h", self.cpu.mem_read(self.unit + 0x1BE, 2))[0] == 110
      assert len(self.chunks) == multiplier and all(amount == 100 for _, amount in self.chunks)
      growth(1)

    configure_rates(2, 3)
    state(exp=0)
    run(20, 10)
    assert self.entries[-1][2:4] == (40, 30)
    assert self.cpu.mem_read(self.unit + 0x8A, 1)[0] == 40
    assert struct.unpack("<h", self.cpu.mem_read(self.unit + 0x1BE, 2))[0] == 130

    for multiplier in (2, 3, 5, 10):
      configure_rates(1, multiplier)
      state(exp=0)
      run(20, 10)
      assert self.entries[-1][2:4] == (20, 10 * multiplier)
      assert self.cpu.mem_read(self.unit + 0x8A, 1)[0] == 20

    configure_rates(0, 3)
    for level, maximum, progress in ((1, 20, 75), (1, 40, 0), (19, 20, 99)):
      state(level=level, maximum=maximum, exp=progress)
      run(1, 10)
      assert self.entries[-1][2:4] == ((maximum - level) * 100 - progress, 30)
      assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [maximum, 0]
      growth(level)
    state()
    run(0, 0)
    assert self.entries[-1][2:4] == (0, 0), "max-class mode requires a real EXP reward"

    # Both inventory menus and in-map item use share UnitGrow/ExpSequence.
    # Recognize their process context, not zero EXP (capped battle units get SP).
    for name in ("InventorySubMenu", "ItemUseSequence"):
      owner, cls, middle = self.alloc(), self.alloc(), self.alloc()
      self.qword(cls + 0x10, self.c_string(name))
      self.qword(owner, cls)
      self.qword(middle + 0x30, owner)
      self.qword(self.parent + 0x30, middle)
      state(exp=0)
      run(0, 100)
      assert self.entries[-1][2:4] == (0, 100), "SP books must not be multiplied"
      assert struct.unpack("<h", self.cpu.mem_read(self.unit + 0x1BE, 2))[0] == 200
    self.qword(self.parent + 0x30, 0)

    configure_rates(10, 10)
    for animated in (False, True):
      state(level=19, exp=75, sp=9990)
      self.skip = not animated
      run(100, 100)
      assert self.entries[-1][2:4] == (25, 9)
      if animated:
        self.skip = True
        call(hooks[0x024E8370], self.sequence, 0x2345)
      assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [20, 0]
      assert struct.unpack("<h", self.cpu.mem_read(self.unit + 0x1BE, 2))[0] == 9999

    # Normal ticks and skipping midway must settle exactly the same 1000 EXP.
    state()
    self.skip = False
    run(100, 0)
    assert self.entries[-1][2:4] == (1000, 0)
    for _ in range(7): call(hooks[0x024E8370], self.sequence, 0x2345)
    assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [1, 82]
    self.skip = True
    call(hooks[0x024E8370], self.sequence, 0x2345)
    assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [11, 75]
    assert self.number(self.sequence + 0x78) == 0

    state()
    self.skip = False
    run(30, 0)
    for _ in range(300): call(hooks[0x024E8370], self.sequence, 0x2345)
    assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [4, 75]
    assert self.number(self.sequence + 0x78) == 0, "normal animation must not require skip input"

    state(level=20, exp=0)
    self.skip = True
    run(0, 10)
    assert self.entries[-1][2:4] == (0, 100), "max level must still permit native SP rewards"
    state()
    run(0, 0)
    assert self.entries[-1][2:4] == (0, 0)
    # Direct AddExp outside either settlement scope retains vanilla one-level
    # behavior. Merely selecting 10x must not multiply unrelated native calls.
    state()
    call(hooks[0x01A39D40], self.unit, 250, 0)
    assert list(self.cpu.mem_read(self.unit + 0x89, 2)) == [2, 25]
    assert self.chunks == [(self.unit, 250)]
    # Reclass previews and default settings must keep the original one-step
    # behavior even if some other source supplies a larger level difference.
    self.old_level = 1
    self.write(self.growth + 0x94, b"\x01")
    call(hooks[0x01BE1260], self.growth, 0x3456)
    assert self.growth_calls == 0
    self.write(self.growth + 0x94, b"\x00")
    configure_rates(1, 1)
    call(hooks[0x01BE1260], self.growth, 0x3456)
    assert self.growth_calls == 1
    self.active = False
    print("compiled EXP/SP: independent multipliers, native AddExp, multi-level growth dispatch, normal/manual-skip playback, max-level/SP caps, SP-book exclusion and passthrough passed")
