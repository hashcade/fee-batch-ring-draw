"""Compiled settings, cooking-context and native support-action boundary checks."""

import struct
from unicorn import arm64_const as reg
from settings_fixture import SettingsFixture


class ConvenienceFixture(SettingsFixture):
  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(6)]
    a, b = args[:2]
    result = None
    if name == "SINGLETON_GET": result = self.user
    elif name == "INITIALIZE_METADATA": pass
    elif name == "UNIT_LIST": result = self.units
    elif name == "UNIT_PID": result = self.pids[a]
    elif name == "UNIT_NAME": result = self.names[a]
    elif name == "COOK_DATA": result = self.cooks.get(self.read_string(a), 0)
    elif name == "CURRENT_CHEF": result = self.pids[self.unit_a]
    elif name == "GET_COOK_DATA": result = self.cooks["PID_a"]
    elif name == "TASTE_DATA": result = self.tastes.get(self.read_string(a), 0)
    elif name in ("MESS_GET", "MESSAGE"): result = a
    elif name == "GRADE_NAME": result = self.string("SS")
    elif name == "STAT_NAME": result = self.string(f"Stat{a}")
    elif name == "IS_CONDITIONAL": result = 1
    elif name == "ANIMATION_CREATE":
      self.animation_calls += 1
      result = self.iterator
    elif name == "DETERMINE_TASTE":
      self.qualities.append((a, b))
      result = self.taste_result
    elif name == "COOK": return False  # Execute the modeled native call below.
    elif name == "IS_BOND": result = self.bond
    elif name == "READY_ONLY": result = self.ready_only
    elif name == "RELATION": result = self.relations.get(tuple(sorted((a, b))), 0)
    elif name == "CAN_ADVANCE": result = int(tuple(sorted((a, b))) in self.relations)
    elif name == "PAIR_LIST": result = self.pair_list
    elif name in ("CREATE_SEQUENCE", "CREATE_UNIT_MENU", "CREATE_PAIR_MENU"): pass
    elif name == "SELECT": self.selection[a] = b
    elif name == "A_CALL":
      self.actions.append((a, "A", self.selection[a]))
      result = 128
    elif name == "B_CALL":
      self.actions.append((a, "B", -1))
      result = 33
    elif name == "ORIGINAL_KEY": result = 99
    elif name == "CHANGE_NUMBER":
      assert args[2] >= self.next_count >= args[1] and args[3] == 1
      result = self.next_count
    elif name == "ALLOCATE_OBJECT":
      result = self.alloc()
      self.qword(result, a)
    else: return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise_new(self, callbacks, hooks, stop, stack):
    self.active = True
    self.functions[self.game + 0x02393F60] = "UNIT_LIST"
    self.user = self.alloc()
    self.integer(self.user + 0x28, 4)
    self.metadata(0x062840F0)
    self.metadata(0x0628E250)
    self.metadata(0x06292CE0)
    self.metadata(0x062B5868, self.string("ordinary"))
    self.unit_a, self.unit_b, invalid = (self.alloc() for _ in range(3))
    self.units = self.list([self.unit_a, self.unit_b, invalid, self.unit_a])
    self.pids = {unit: self.string(pid) for unit, pid in zip((self.unit_a, self.unit_b, invalid), ("PID_a", "PID_b", "PID_missing"))}
    self.names = {unit: self.string(name) for unit, name in zip((self.unit_a, self.unit_b, invalid), ("A", "B", "Missing"))}
    self.cooks = {pid: self.alloc() for pid in ("PID_a", "PID_b")}
    self.tastes = {}
    for data in self.cooks.values():
      for index in range(3):
        key = f"taste{index}"
        self.qword(data + 0x28 + index * 8, self.string(key))
        taste = self.tastes.setdefault(key, self.alloc())
        self.qword(taste + 0x28, self.string(key))
        capability, values = self.alloc(), self.alloc()
        self.qword(taste + 0x38, capability)
        self.qword(capability + 0x10, values)
        self.qword(values + 0x18, 11)
        self.write(values + 0x20, struct.pack("<11b", 0, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0))
        self.write(taste + 0x34, bytes([2, 1]))
    self.bond, self.qualities, self.actions, self.selection = False, [], [], {}
    self.ready_only = True
    self.taste_result = self.alloc()

    def call(address, *args):
      for i in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{i}"), args[i] if i < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      try:
        self.cpu.emu_start(address, stop + 4, count=1_000_000)
      except Exception as error:
        raise AssertionError(f"native boundary at {self.cpu.reg_read(reg.UC_ARM64_REG_PC):#x}; entry {address:#x}") from error
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

    chef, outcome, support = (call(callback) for callback in callbacks[1:])
    # Cobalt's PluginSubmenu wraps every callback result in another class clone.
    for item in (chef, outcome, support):
      copied = self.alloc(0x138 + 27 * 16)
      self.write(copied, bytes(self.cpu.mem_read(self.pointer(item), 0x138 + 27 * 16)))
      self.qword(item, copied)
    def change(item, value):
      self.next_count = value
      return call(self.pointer(self.pointer(item) + 0x138 + 26 * 16), item)

    assert change(chef, 2) == 4096
    assert self.read_string(self.pointer(chef + 0x70)) == "B"
    assert b"chef=PID_b\n" in self.files[self.paths[0]]
    assert call(hooks["CURRENT_CHEF"]) == self.pids[self.unit_b]
    assert call(hooks["GET_COOK_DATA"]) == self.cooks["PID_b"]
    # The choice is stable by PID and safely falls back in another save.
    self.units = self.list([self.unit_a, invalid])
    assert call(hooks["CURRENT_CHEF"]) == self.pids[self.unit_a]
    self.units = self.list([self.unit_a, self.unit_b, invalid])

    assert change(outcome, 3) == 4096
    detail = self.read_string(self.pointer(outcome + 0x78))
    for part in ("SS", "Stat1 +3", "Stat2 -1", "料理基础能力 +2", "其他能力 +1", "依条件变化"):
      assert part in detail, detail
    assert b"chef=PID_b\n" in self.files[self.paths[0]] and b"cooking_outcome=3\n" in self.files[self.paths[0]]
    call(hooks["DETERMINE_TASTE"], self.unit_b, 4)
    assert self.qualities[-1] == (self.unit_b, 4), "outside cooking must remain native"
    # A native Cook stand-in calls the compiled taste hook, preserving LR/SP.
    original_cook = self.game + 0x025447B0
    code = struct.pack("<7I", 0xA9BF7BFD, 0x910003FD, 0x52800061,
      0x580000B0, 0xD63F0200, 0xA8C17BFD, 0xD65F03C0) + bytes(4) + struct.pack("<Q", hooks["DETERMINE_TASTE"])
    self.write(original_cook, code)
    call(hooks["COOK"], self.unit_a, 0, 0)
    assert self.qualities[-1] == (self.unit_b, 2), "native chef branch, not an arbitrary TasteData"
    taste = self.tastes.pop("taste2")
    call(hooks["COOK"], self.unit_a, 0, 0)
    assert self.qualities[-1] == (self.unit_b, 3), "missing native data must fall back"
    self.tastes["taste2"] = taste
    assert change(outcome, 0) == 4096
    call(hooks["COOK"], self.unit_a, 0, 0)
    assert self.qualities[-1] == (self.unit_b, 3)

    # The animated coroutine is distinct from ExecuteCooking and its result
    # flow. Skip only before animation starts and only after a real dish exists.
    self.animation_calls = 0
    for state, sequence, has_dish, changed, skipped in (
      (0, 4, True, False, True), (1, 4, True, False, False),
      (0, 5, True, False, False), (0, 4, False, False, False), (0, 4, True, True, True)):
      iterator, owner, cls = self.alloc(), self.alloc(), self.alloc()
      self.iterator = iterator
      self.qword(iterator, cls)
      self.write(cls + 0x126, struct.pack("<H", 8))
      self.qword(cls + 0x138 + 4 * 16, self.game + 0x01CE3CF0 + 4 * changed)
      self.qword(iterator + 0x20, owner)
      self.qword(owner + 0xA0, taste if has_dish else 0)
      self.integer(iterator + 0x10, state)
      self.integer(self.user + 0x28, sequence)
      before = bytes(self.cpu.mem_read(owner, 0x150))
      assert call(hooks["ANIMATION_CREATE"], owner) == iterator
      assert bytes(self.cpu.mem_read(owner, 0x150)) == before
      assert self.number(iterator + 0x10) == (-1 if skipped else state)
    assert self.animation_calls == 5
    self.integer(self.user + 0x28, 4)

    def stub(name, slot):
      address = self.game + 0x03D00000 + slot * 16
      self.write(address, struct.pack("<I", 0xD65F03C0))
      self.functions[address] = name
      return address

    def menu(parent, items):
      result, cls = self.alloc(), self.alloc(0x138 + 64 * 16)
      self.write(cls + 0x126, struct.pack("<H", 64))
      for slot, name in ((34, "SELECT"), (49, "ORIGINAL_KEY"), (50, "A_CALL"), (51, "B_CALL")):
        method = self.alloc()
        self.qword(method, stub(name, slot))
        self.qword(cls + 0x138 + slot * 16, self.pointer(method))
        self.qword(cls + 0x138 + slot * 16 + 8, method)
      self.qword(result, cls)
      self.qword(result + 0x80, self.list(items))
      self.qword(result + 0x30, parent)
      self.qword(parent + 0x38, result)
      return result

    unit_item, good, pact, bond, unavailable = [self.alloc() for _ in range(5)]
    self.qword(unit_item + 0x68, self.unit_a)
    for item in (good, pact, bond, unavailable):
      self.qword(item + 0x68, self.unit_a)
      self.qword(item + 0x70, self.unit_b)
      self.write(item + 0x81, b"\x01")
    self.qword(bond + 0x78, self.alloc())
    self.write(unavailable + 0x81, b"\0")
    self.pair_list = self.list([good])
    relation = self.alloc()
    self.integer(relation + 0x18, 1)
    self.relations = {tuple(sorted((self.unit_a, self.unit_b))): relation}
    parent = self.alloc()
    root = menu(parent, [unit_item])
    call(hooks["CREATE_UNIT_MENU"], parent)
    root_key = self.pointer(self.pointer(root) + 0x138 + 49 * 16)
    root_method = self.pointer(self.pointer(root) + 0x138 + 49 * 16 + 8)
    assert call(root_key, root, root_method) == 128 and self.actions[-1] == (root, "A", 0)
    pairs = menu(root, [bond, unavailable, good])
    call(hooks["CREATE_PAIR_MENU"], root, self.unit_a)
    pair_key = self.pointer(self.pointer(pairs) + 0x138 + 49 * 16)
    pair_method = self.pointer(self.pointer(pairs) + 0x138 + 49 * 16 + 8)
    snapshot = bytes(self.cpu.mem_read(relation, 0x40))
    assert call(pair_key, pairs, pair_method) == 128 and self.actions[-1] == (pairs, "A", 2)
    assert bytes(self.cpu.mem_read(relation, 0x40)) == snapshot, "dispatch must not write ranks or EXP"
    assert call(pair_key, pairs, pair_method) == 33 and self.actions[-1][1] == "B", "never retry a dispatched pair"
    assert call(root_key, root, root_method) == 33
    self.ready_only = False
    assert call(root_key, root, root_method) == 99, "leave the recall gallery alone"
    self.ready_only, self.bond = True, True
    assert call(root_key, root, root_method) == 99
    self.bond = False
    call(hooks["CREATE_SEQUENCE"], parent)
    assert call(root_key, root, root_method) == 128, "a new opening resets attempts even when addresses are reused"
    assert change(support, 0) == 4096 and b"auto_support=0\n" in self.files[self.paths[0]]
    assert call(root_key, root, root_method) == 99
    assert change(support, 1) == 4096
    assert change(chef, 0) == 4096
    assert self.files == {self.paths[0]: b"draw_count=300\n"}
    self.active = False
