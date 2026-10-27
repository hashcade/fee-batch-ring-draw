"""Exercise the compiled extra draw row while preserving native one/ten items."""

import struct
from unicorn import arm64_const as reg


class DrawMenuFixture:
  def __init__(self, cpu, game, write, offset):
    self.cpu, self.game, self.write = cpu, game, write
    self.entries = {game + offset(name): name for name in (
      "COUNT_MENU", "COUNT_CTOR", "SET_MENU_ROWS", "INITIALIZE_METADATA",
      "ALLOCATE_OBJECT", "LIST_ADD")}
    self.active = False
    self.root = game + 0x6800000
    self.menu, self.list, self.array, self.item, self.handler = (
      self.root + n * 0x1000 for n in range(5))
    for address in range(self.root, self.root + 0x9000, 0x1000):
      write(address, bytes(0x1000))
    for index, slot in enumerate((0x062C2B88, 0x06286A70)):
      cell = self.root + 0x8000 + index * 16
      self.qword(game + slot, cell)
      self.qword(cell, cell + 8)

  def qword(self, address, value):
    self.write(address, struct.pack("<Q", value))

  def handle(self, pc):
    if not self.active or pc not in self.entries:
      return False
    name = self.entries[pc]
    x0, x1, x2 = (self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{n}")) for n in range(3))
    if name == "COUNT_MENU":
      assert x1 == self.handler
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.menu)
    elif name == "INITIALIZE_METADATA":
      pass
    elif name == "ALLOCATE_OBJECT":
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.item)
    elif name == "COUNT_CTOR":
      assert x0 == self.item and x2 == self.handler
      self.write(x0 + 0x64, struct.pack("<ii", x1, x1 * 100))
      self.qword(x0 + 0x70, x2)
      self.constructed.append(x1)
    elif name == "LIST_ADD":
      assert x0 == self.list and x1 == self.item
      self.qword(self.array + 0x30, x1)
      self.write(self.list + 0x18, struct.pack("<i", 3))
    elif name == "SET_MENU_ROWS":
      assert x0 == self.menu and x1 == 3
      self.rows.append(x1)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack, count):
    self.qword(self.menu + 0x80, self.list)
    self.qword(self.list + 0x10, self.array)
    self.qword(self.array + 0x18, 4)
    self.write(self.list + 0x18, struct.pack("<i", 2))
    originals = (self.root + 0x5000, self.root + 0x6000)
    for index, (item, times) in enumerate(zip(originals, (1, 10))):
      self.qword(self.array + 0x20 + index * 8, item)
      self.write(item + 0x64, struct.pack("<ii", times, times * 100))
    before = [bytes(self.cpu.mem_read(item, 0x100)) for item in originals]
    self.constructed, self.rows = [], []
    self.active = True
    for _ in range(2):
      for register, value in ((reg.UC_ARM64_REG_SP, stack), (reg.UC_ARM64_REG_X0, 0),
          (reg.UC_ARM64_REG_X1, self.handler), (reg.UC_ARM64_REG_X2, 0),
          (reg.UC_ARM64_REG_X30, stop)):
        self.cpu.reg_write(register, value)
      self.cpu.emu_start(replacement, stop + 4, count=100_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      assert self.cpu.reg_read(reg.UC_ARM64_REG_X0) == self.menu
    self.active = False
    assert self.constructed == ([] if count == 10 else [count])
    assert self.rows == ([] if count == 10 else [3])
    assert before == [bytes(self.cpu.mem_read(item, 0x100)) for item in originals]
