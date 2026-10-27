"""Exercise compiled collector callbacks at modeled IL2CPP boundaries, not Unity."""

import re
import struct

from unicorn import arm64_const as registers


class CollectorFixture:
  def __init__(self, cpu, game, write, profile, common):
    self.cpu, self.game, self.write = cpu, game, write
    self.functions = {game + int(value, 16): name for name, value in re.findall(
      r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", profile + common)}
    self.functions[game + 0x025C9240] = "MESSAGE"
    self.close = game + 0x02459A70
    self.write(self.close, struct.pack("<I", 0xD65F03C0))
    self.functions[self.close] = "CLOSE"
    self.functions[game + 0x02393F60] = "EMPTY_UNIT_LIST"
    self.functions[game + 0x02A71860] = "SCENE_NAME"
    self.heap = game + 0x7000000
    self.active = False

  def alloc(self, size=0x600):
    value = self.heap
    self.heap += (size + 31) & ~31
    self.write(value, bytes(size))
    return value

  def qword(self, address, value):
    self.write(address, struct.pack("<Q", value))

  def integer(self, address, value):
    self.write(address, struct.pack("<i", value))

  def number(self, address):
    return struct.unpack("<i", self.cpu.mem_read(address, 4))[0]

  def pointer(self, address):
    return struct.unpack("<Q", self.cpu.mem_read(address, 8))[0]

  def string(self, value):
    encoded = value.encode("utf-16-le")
    result = self.alloc(len(encoded) + 0x20)
    self.integer(result + 0x10, len(encoded) // 2)
    self.write(result + 0x14, encoded)
    return result

  def read_string(self, value):
    return bytes(self.cpu.mem_read(value + 0x14, self.number(value + 0x10) * 2)).decode("utf-16-le")

  def c_string(self, value):
    result = self.alloc(len(value) + 32)
    self.write(result, value.encode() + b"\0")
    return result

  def metadata(self, offset, value=None):
    value = self.alloc() if value is None else value
    cell = self.alloc(8)
    self.qword(cell, value)
    self.qword(self.game + offset, cell)
    return value

  def list(self, values=(), capacity=128):
    result = self.alloc()
    self.init_list(result, capacity)
    for value in values:
      self.append(result, value)
    return result

  def init_list(self, result, capacity=128):
    array = self.alloc(0x20 + capacity * 8)
    self.qword(array + 0x18, capacity)
    self.qword(result + 0x10, array)
    self.integer(result + 0x18, 0)

  def append(self, result, value):
    count = self.number(result + 0x18)
    array = self.pointer(result + 0x10)
    assert count < self.pointer(array + 0x18)
    self.qword(array + 0x20 + count * 8, value)
    self.integer(result + 0x18, count + 1)

  def values(self, result):
    array = self.pointer(result + 0x10)
    return [self.pointer(array + 0x20 + index * 8) for index in range(self.number(result + 0x18))]

  def reset(self, sequence=5):
    self.flags, self.inventory, self.items, self.animals = {}, {}, {}, {}
    self.counts, self.effects, self.events, self.adoptions = {}, [], [], []
    self.user, self.hub, self.manager, self.group = [self.alloc() for _ in range(4)]
    self.variables = self.alloc()
    self.qword(self.user + 0x50, self.variables)
    self.integer(self.user + 0x28, sequence)
    self.integer(self.user + 0x58, 1234)
    self.integer(self.user + 0x6C, 5678)
    self.accesses, self.visuals = self.list(), self.list()
    self.qword(self.manager + 0x18, self.accesses)
    self.qword(self.group + 0x20, self.visuals)
    self.qword(self.hub + 0xC8, self.group)
    self.metadata(0x062840F0)
    self.metadata(0x0628A970)
    self.metadata(0x06287888)
    self.metadata(0x062A6028, self.string("barn"))
    self.metadata(0x06286A70)
    self.item_list_class = self.metadata(0x062923C0)
    self.metadata(0x062923B8)
    method, methods = self.alloc(), self.alloc()
    self.qword(method, self.game + 0x03DE9550)
    self.qword(method + 0x10, self.c_string("Add"))
    self.qword(methods, method)
    self.qword(self.item_list_class + 0x98, methods)
    self.write(self.item_list_class + 0x11C, struct.pack("<H", 1))
    self.base_class = self.alloc()
    self.qword(self.base_class + 0x10, self.c_string("BasicMenuItem"))
    self.integer(self.base_class + 0xF8, 0x60)
    self.write(self.base_class + 0x126, struct.pack("<H", 30))
    self.parent, self.menu, self.original = self.alloc(), self.alloc(), self.alloc()
    self.qword(self.original, self.base_class)
    self.menu_class = self.alloc()
    self.write(self.menu_class + 0x126, struct.pack("<H", 24))
    self.qword(self.menu_class + 0x138 + 23 * 16, self.close)
    self.qword(self.menu, self.menu_class)
    self.qword(self.parent + 0x38, self.menu)
    self.full_list = self.list([self.original])
    self.qword(self.menu + 0x80, self.full_list)
    self.integer(self.menu + 0xB8, 7)
    self.barn, self.can_capture = True, True

  def add_item(self, key, owned=0, maximum=99, kind=1, price=1):
    item = self.alloc()
    self.integer(item + 0x48, kind)
    self.integer(item + 0x70, price)
    self.items[key] = item
    self.inventory[item] = [owned, maximum]
    return item

  def add_source(self, flag, key, kind=6, count=1, claimed=False, story=False):
    data, dispos, visual, effect = [self.alloc() for _ in range(4)]
    self.qword(data + 0x10, self.string(key))
    self.qword(data + 0x18, dispos)
    self.qword(data + 0x40, self.string(flag))
    self.integer(data + 0x38, count)
    self.integer(dispos + 0x68, kind)
    if kind == 7:
      self.write(data + 0x24, b"\x01")
    if 2 <= kind <= 5:
      self.write(data + 0x25, b"\x01")
      self.write(data + 0x20, bytes([story]))
      self.qword(data + 0x30, self.string(key))
      self.counts[data] = [self.string("talk-" + flag), 0]
    if claimed:
      self.flags[flag] = 1
    self.qword(visual + 0x60, data)
    self.qword(visual + 0x68, effect)
    self.append(self.accesses, data)
    self.append(self.visuals, visual)
    return data, effect

  def handle(self, pc, import_name):
    if not self.active:
      return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(registers.UC_ARM64_REG_X0 + index) for index in range(6)]
    x0, x1, x2, x3, _, _ = args
    result = None
    if name in ("INITIALIZE_METADATA", "WRITE_BARRIER", "ITEM_CTOR", "free"):
      pass
    elif name == "CREATE_MENU":
      assert x0 == self.parent
      self.events.append("original menu")
    elif name == "SINGLETON_GET":
      result = self.user
    elif name == "HUB_GET":
      result = self.hub
    elif name == "EMPTY_UNIT_LIST": result = self.list()
    elif name == "SCENE_NAME": result = self.string("Hub_Solanel")
    elif name == "CURRENT_ACCESS":
      result = self.manager
    elif name == "VARIABLE_BOOL":
      assert x0 == self.variables
      result = int(bool(self.flags.get(self.read_string(x1), 0)))
    elif name in ("VARIABLE_ENTRY", "VARIABLE_SET"):
      assert x0 == self.variables
      key = self.read_string(x1)
      if name == "VARIABLE_SET" or key not in self.flags:
        self.flags[key] = x2
      result = 1
    elif name == "ITEM_GET":
      result = self.items.get(self.read_string(x0), 0)
    elif name == "ITEM_BONUS":
      self.events.append("bonus")
      result = x1 * 3
    elif name == "IS_INVENTORY":
      result = 1
    elif name in ("GET_INVENTORY", "MAX_INVENTORY"):
      result = self.inventory[x0][name == "MAX_INVENTORY"]
    elif name == "IS_COMPLETE":
      result = int(self.barn)
    elif name == "TRY_PID":
      result = self.pointer(x0 + 0x10)
    elif name == "ANIMAL_GET":
      result = self.animals.get(self.read_string(x0), (0, 0))[0]
    elif name == "CAN_CAPTURE":
      result = int(self.can_capture)
    elif name == "CAPTURE_COUNT":
      result = next(value[1] for value in self.animals.values() if value[0] == x0)
    elif name == "INC_CAPTURE":
      for value in self.animals.values():
        if value[0] == x0:
          value[1] += x1
      self.adoptions.append(x0)
    elif name == "ACCESS_COUNT":
      key, initial = self.counts[x0]
      result = self.flags.get(self.read_string(key), initial)
    elif name == "ACCESS_FLAG":
      result = self.counts[x0][0]
    elif name == "ALLOCATE_OBJECT":
      result = self.alloc()
      self.qword(result, x0)
    elif name == "DEFAULT_LIST_CTOR":
      self.init_list(x0, 8192)
    elif name == "LIST_ADD":
      self.append(x0, x1)
    elif name == "STRING_NEW_SIZE":
      result = self.alloc(x0 * 2 + 0x20)
      self.integer(result + 0x10, x0)
    elif name == "MESSAGE":
      result = self.string(self.read_string(x0))
    elif name == "CALC_REWARDS":
      self.events.append("grant")
      self.reward_items = self.values(x0)
      for item in self.reward_items:
        assert self.inventory[item][0] < self.inventory[item][1]
        self.inventory[item][0] += 1
      self.cpu.mem_write(x1, struct.pack("<i", 0))
      self.cpu.mem_write(x2, b"\0")
      self.reward_dictionary = self.alloc()
      result = self.reward_dictionary
    elif name == "DONE_ACCESS":
      data = self.pointer(x0 + 0x60)
      assert self.flags[self.read_string(self.pointer(data + 0x40))] == 1
      self.events.append("visual done")
    elif name == "SET_ACTIVE":
      assert x1 == 0
      self.effects.append(x0)
    elif name == "CLOSE":
      assert x0 == self.menu
      self.events.append("close")
    elif name == "REWARD_DIALOG":
      assert x0 == self.menu and x2 == self.reward_dictionary
      self.events.append("dialog")
    elif name == "malloc":
      result = self.alloc(x0)
    elif name in ("memcpy", "memmove"):
      self.cpu.mem_write(x0, bytes(self.cpu.mem_read(x1, x2)))
      result = x0
    elif name == "memset":
      self.cpu.mem_write(x0, bytes([x1 & 255]) * x2)
      result = x0
    elif name == "memcmp":
      left, right = bytes(self.cpu.mem_read(x0, x2)), bytes(self.cpu.mem_read(x1, x2))
      result = (int(left > right) - int(left < right)) & 0xFFFFFFFFFFFFFFFF
    else:
      return False
    if result is not None:
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(registers.UC_ARM64_REG_PC, self.cpu.reg_read(registers.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack):
    self.active = True

    def call(function, argument):
      self.cpu.reg_write(registers.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, argument)
      self.cpu.reg_write(registers.UC_ARM64_REG_X1, 0)
      self.cpu.reg_write(registers.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(function, stop + 4, count=2_000_000)
      assert self.cpu.reg_read(registers.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(registers.UC_ARM64_REG_X0)

    def menu():
      original_class = bytes(self.cpu.mem_read(self.base_class, 0x400))
      call(replacement, self.parent)
      values = self.values(self.full_list)
      extra = int(self.number(self.user + 0x28) == 4)
      assert len(values) == 2 + extra and values[1] == self.original
      assert self.number(self.menu + 0xB8) == 8 + extra
      assert bytes(self.cpu.mem_read(self.base_class, 0x400)) == original_class
      item = values[0]
      self.qword(item + 0x10, self.menu)
      private = self.pointer(item)
      self.callbacks = {slot: self.pointer(private + 0x138 + slot * 16) for slot in (4, 8, 18, 28)}
      call(replacement, self.parent)
      assert len(self.values(self.full_list)) == 2 + extra, "do not insert a second menu entry"
      return item

    self.reset()
    item = menu()
    assert call(self.callbacks[8], item) == 4
    assert call(self.callbacks[18], item) == 2048
    assert "grant" not in self.events
    assert self.read_string(call(self.callbacks[4], item)) == "MID_Hub_Area_ItemReturn"
    assert self.read_string(call(self.callbacks[28], item)) == "MID_Hub_Well_ItemExchange_Get"

    for sequence in (4, 5):
      self.reset(sequence)
      herb = self.add_item("herb", owned=95)
      ore = self.add_item("ore")
      gift = self.add_item("seal")
      self.add_source("a", "herb")
      self.add_source("a", "herb")  # Duplicate native source, not two pickups.
      skipped, skipped_effect = self.add_source("b", "herb")
      self.add_source("ore", "ore", kind=8, count=5)
      self.add_source("gift", "seal", kind=2)
      self.add_source("plot", "seal", kind=2, story=True)
      self.add_source("old", "ore", claimed=True)
      animal = self.alloc()
      self.animals["cat"] = [animal, 98]
      self.add_source("cat1", "cat", kind=7)
      self.add_source("cat2", "cat", kind=7)
      item = menu()
      assert call(self.callbacks[8], item) == 1
      call(self.callbacks[18], item)
      assert self.inventory[herb][0] == 98
      assert self.inventory[ore][0] == 5, "type 8 must not receive donation bonus twice"
      assert self.inventory[gift][0] == 1
      assert self.flags["talk-gift"] == 1
      assert "plot" not in self.flags and "talk-plot" not in self.flags
      assert "b" not in self.flags and skipped_effect not in self.effects
      assert self.animals["cat"][1] == (99 if sequence == 5 else 98)
      assert "cat2" not in self.flags
      assert self.events.count("grant") == 1 and self.events[-2:] == ["close", "dialog"]
      assert call(self.callbacks[18], item) == 2048
      assert self.events.count("grant") == 1, "repeat dispatch must not duplicate rewards"
      assert call(self.callbacks[8], item) == 4

    for blocked in ("barn", "can_capture"):
      self.reset()
      self.animals["cat"] = [self.alloc(), 0]
      self.add_source("cat", "cat", kind=7)
      setattr(self, blocked, False)
      item = menu()
      assert call(self.callbacks[18], item) == 2048
      assert not self.adoptions and "cat" not in self.flags

    self.reset()
    animal = self.alloc()
    self.animals["cat"] = [animal, 0]
    self.add_source("cat", "cat", kind=7)
    item = menu()
    assert call(self.callbacks[18], item) == 129
    assert self.animals["cat"][1] == 1
    assert "grant" not in self.events and "dialog" not in self.events

    self.reset()
    herb = self.add_item("herb")
    self.add_source("zero", "herb")
    self.flags["zero"] = 0
    item = menu()
    assert call(self.callbacks[18], item) == 129
    assert self.flags["zero"] == 1 and self.inventory[herb][0] == 3
    assert call(self.callbacks[18], item) == 2048

    self.reset()
    self.add_item("seal")
    data, _ = self.add_source("gift", "seal", kind=2)
    # Reconstructed TalkItem after reload must still fail the native count gate.
    self.flags["talk-gift"] = 1
    item = menu()
    assert call(self.callbacks[18], item) == 2048
    assert self.pointer(data + 0x30) != 0, "do not mutate unselected conversations"

    self.reset()
    self.add_item("herb", owned=99)
    self.add_source("full", "herb")
    item = menu()
    assert call(self.callbacks[18], item) == 2048
    assert "full" not in self.flags

    self.reset(sequence=3)
    self.add_item("herb")
    self.add_source("a", "herb")
    item = menu()
    assert call(self.callbacks[18], item) == 2048
    assert not self.flags
    self.active = False
