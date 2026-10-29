"""Exercise the compiled extra draw row while preserving native one/ten items."""

import struct
from unicorn import arm64_const as reg


class DrawMenuFixture:
  def __init__(self, cpu, game, write, offset):
    self.cpu, self.game, self.write = cpu, game, write
    self.entries = {game + offset(name): name for name in (
      "COUNT_MENU", "COUNT_CTOR", "SET_MENU_ROWS", "INITIALIZE_METADATA",
      "ALLOCATE_OBJECT", "LIST_ADD", "GAME_OBJECT_TRANSFORM", "TRANSFORM_CHILD_COUNT",
      "TRANSFORM_CHILD", "CLONE_WITH_PARENT", "LOCAL_POSITION_GET", "LOCAL_POSITION_SET")}
    self.active = False
    self.root = game + 0x6800000
    self.menu, self.list, self.array, self.item, self.handler = (
      self.root + n * 0x1000 for n in range(5))
    self.content, self.content_object, self.transform = (self.root + n * 0x1000 for n in (9, 10, 11))
    for address in range(self.root, self.root + 0x10000, 0x1000):
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
      length, = struct.unpack("<i", self.cpu.mem_read(self.list + 0x18, 4))
      self.qword(self.array + 0x20 + length * 8, x1)
      self.write(self.list + 0x18, struct.pack("<i", length + 1))
    elif name == "SET_MENU_ROWS":
      assert x0 == self.menu and x1 == self.expected_rows
      assert len(self.children) >= x1, "native row setter rejects requests beyond real prefab children"
      self.write(self.menu + 0xB8, struct.pack("<i", x1))
      self.rows.append(x1)
    elif name == "GAME_OBJECT_TRANSFORM":
      assert x0 == self.content_object
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.transform)
    elif name == "TRANSFORM_CHILD_COUNT":
      assert x0 == self.transform
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, len(self.children))
    elif name == "TRANSFORM_CHILD":
      assert x0 == self.transform and x1 < len(self.children)
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, self.children[x1])
    elif name == "CLONE_WITH_PARENT":
      assert x0 == self.children[-1] and x1 == self.transform and x2 == 0
      added = self.root + 0xC000 + len(self.children) * 0x100
      self.positions[added] = self.positions[x0]
      self.children.append(added)
      self.clones += 1
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, added)
    elif name == "LOCAL_POSITION_GET":
      self.cpu.mem_write(x1, struct.pack("<fff", *self.positions[x0]))
    elif name == "LOCAL_POSITION_SET":
      assert x0 == self.children[-1]
      self.positions[x0] = struct.unpack("<fff", self.cpu.mem_read(x1, 12))
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack, count, extra=False):
    self.qword(self.menu + 0x70, self.content)
    self.qword(self.content + 0x68, self.content_object)
    self.children = [self.root + 0xC000 + i * 0x100 for i in range(3 if extra else 2)]
    self.positions = {child: (13.0, -57.0 * index, 2.0) for index, child in enumerate(self.children)}
    original_positions = dict(self.positions)
    self.clones = 0
    self.write(self.menu + 0xB8, struct.pack("<i", len(self.children)))
    self.qword(self.menu + 0x80, self.list)
    self.qword(self.list + 0x10, self.array)
    self.qword(self.array + 0x18, 8)
    self.write(self.list + 0x18, struct.pack("<i", 3 if extra else 2))
    originals = (self.root + 0x5000, self.root + 0x6000)
    for index, (item, times) in enumerate(zip(originals, (1, 10))):
      self.qword(self.array + 0x20 + index * 8, item)
      self.write(item + 0x64, struct.pack("<ii", times, times * 100))
    if extra:
      added = self.root + 0x7000
      self.qword(self.array + 0x30, added)
      self.write(added + 0x64, struct.pack("<ii", 20, 2000))
      originals += (added,)
    self.expected_rows = 4 if extra else 3
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
    assert self.rows == ([] if count == 10 else [self.expected_rows])
    assert self.clones == (0 if count == 10 else 1)
    assert all(self.positions[child] == value for child, value in original_positions.items())
    if count != 10:
      assert len(self.children) == self.expected_rows
      assert self.positions[self.children[-1]] == (13.0, -57.0 * (self.expected_rows - 1), 2.0)
      shown, = struct.unpack("<i", self.cpu.mem_read(self.menu + 0xB8, 4))
      assert shown == self.expected_rows
    assert before == [bytes(self.cpu.mem_read(item, 0x100)) for item in originals]
