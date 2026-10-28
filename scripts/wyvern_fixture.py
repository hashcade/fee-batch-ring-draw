"""Exercise the compiled Wyvern hook against modeled native game boundaries."""

import struct

from unicorn import arm64_const as registers


class WyvernFixture:
  def __init__(self, cpu, game, write, profile):
    import re
    self.cpu, self.write = cpu, write
    self.addresses = {
      game + int(value, 16): name for name, value in re.findall(
        r"pub const (MAIN|CALC_RANK|FINISH_RAIL|SET_SCORE|GET_RETIRE): usize = (0x[0-9A-Fa-f]+)", profile)
    }
    self.sequence = game + 0x6500000
    self.camera, self.config = self.sequence + 0x1000, self.sequence + 0x2000
    self.arrays = [self.sequence + 0x3000 + index * 0x1000 for index in range(3)]
    self.method = 0x1234
    self.active = False

  def qword(self, address, value):
    self.write(address, struct.pack("<Q", value))

  def integer(self, address, value):
    self.write(address, struct.pack("<i", value))

  def reset(self, level=1):
    self.events, self.score = [], 123
    self.retire, self.retire_during_tick = False, False
    for obj in [self.sequence, self.camera, self.config, *self.arrays]:
      self.write(obj, bytes(0x600))
    self.qword(self.sequence + 0xA8, self.camera)
    self.qword(self.sequence + 0xE0, self.config)
    self.integer(self.sequence + 0xB8, level)
    self.write(self.camera + 0xA2, b"\x01")
    # Synthetic thresholds, not claims about the original game's values.
    for index, array in enumerate(self.arrays):
      self.qword(self.config + 0x20 + index * 8, array)
      self.qword(array + 0x18, 9)
      self.write(array + 0x20, struct.pack("<9i", *[
        (index + 1) * 1000 * value for value in range(8, -1, -1)]))

  def handle(self, pc):
    name = self.addresses.get(pc) if self.active else None
    if name is None:
      return False
    x0 = self.cpu.reg_read(registers.UC_ARM64_REG_X0)
    if name == "MAIN":
      assert x0 in (0, self.sequence)
      assert self.cpu.reg_read(registers.UC_ARM64_REG_X1) == self.method
      self.events.append("tick")
      # A pending target effect in the final native tick must not inflate SSS.
      self.score += 17
      self.retire |= self.retire_during_tick
    elif name == "GET_RETIRE":
      assert x0 == 0
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, int(self.retire))
    elif name == "FINISH_RAIL":
      assert x0 == self.camera
      assert self.cpu.reg_read(registers.UC_ARM64_REG_X1) == 0
      self.events.append("finish")
      self.write(self.camera + 0x1B9, b"\x01")
    elif name == "SET_SCORE":
      assert self.cpu.reg_read(registers.UC_ARM64_REG_X1) == 0
      self.events.append("score")
      self.score = x0
    elif name == "CALC_RANK":
      raise AssertionError("native SSS borders must not be revalidated through rank calls")
    self.cpu.reg_write(registers.UC_ARM64_REG_PC, self.cpu.reg_read(registers.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack):
    self.active = True

    def call(sequence=None):
      self.cpu.reg_write(registers.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, self.sequence if sequence is None else sequence)
      self.cpu.reg_write(registers.UC_ARM64_REG_X1, self.method)
      self.cpu.reg_write(registers.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(replacement, stop + 4, count=100_000)
      assert self.cpu.reg_read(registers.UC_ARM64_REG_PC) == stop

    for level in (1, 2, 3):
      self.reset(level)
      before = bytes(self.cpu.mem_read(self.sequence, 0x600))
      call()
      assert self.events == ["finish", "tick", "score"], self.events
      assert self.score == level * 8000
      assert bytes(self.cpu.mem_read(self.sequence, 0x600)) == before
      # A finished run must not dispatch completion a second time.
      self.events = []
      call()
      assert self.events == ["tick"]

    self.reset(2)
    self.integer(self.arrays[1] + 0x20, 123456)
    call()
    assert self.score == 123456, "thresholds must be read dynamically"

    for owner, offset, value in [
      (self.sequence, 0xA0, 1), (self.sequence, 0x114, 1),
      (self.camera, 0xA2, 0), (self.camera, 0x1B9, 1),
      (self.camera, 0x1BB, 1), (self.camera, 0x1F8, 1), (self.camera, 0x1F9, 1),
    ]:
      self.reset()
      self.write(owner + offset, bytes([value]))
      call()
      assert self.events == ["tick"], (hex(offset), self.events)
      assert self.score == 140

    for owner, offset in [(self.sequence, 0xA8), (self.sequence, 0xE0), (self.config, 0x20)]:
      self.reset()
      self.qword(owner + offset, 0)
      call()
      assert self.events == ["tick"]

    for level in (-1, 0, 4, 2147483647):
      self.reset(level)
      call()
      assert self.events == ["tick"]

    for count in (0, 1, 10, 0xFFFFFFFFFFFFFFFF):
      self.reset()
      self.qword(self.arrays[0] + 0x18, count)
      call()
      assert self.events == (["tick"] if count == 0 else ["finish", "tick", "score"])
      assert self.score == (140 if count == 0 else 8000)

    for values in [(0, -1), (100, 200), (100, 100), (100, -1)]:
      self.reset()
      self.qword(self.arrays[0] + 0x18, 2)
      self.write(self.arrays[0] + 0x20, struct.pack("<2i", *values))
      call()
      assert self.events == ["finish", "tick", "score"]
      assert self.score == values[0]

    self.reset()
    self.retire = True
    call()
    assert self.events == ["tick"]

    self.reset()
    self.retire_during_tick = True
    call()
    assert self.events == ["finish", "tick"]
    assert self.score == 140, "a retirement must not receive an SSS score"

    self.reset()
    call(0)
    assert self.events == ["tick"]
    self.active = False
