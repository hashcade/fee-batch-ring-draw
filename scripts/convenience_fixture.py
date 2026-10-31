"""Compiled settings, cooking-context and native support-action boundary checks."""

import struct
from unicorn import arm64_const as reg
from settings_fixture import SettingsFixture


class ConvenienceFixture(SettingsFixture):
  def configure_multipliers(self, exp, sp):
    self.active = True
    for kind, rate in ((15, exp), (16, sp)):
      item = self.invoke(self.factories[kind])
      self.next_count = ([1, 2, 3, 5, 10, 0] if kind == 15 else [1, 2, 3, 5, 10]).index(rate)
      result = self.invoke(self.pointer(self.pointer(item) + 0x138 + 26 * 16), item)
      assert result in (0, 4096)
      assert self.read_string(self.pointer(item + 0x70)) == (f"{rate}×" if rate else "满级")
      saved = self.files[self.paths[0]]
      key = "exp_multiplier" if kind == 15 else "sp_multiplier"
      assert (f"{key}={rate}\n".encode() in saved) == (rate != 1)
    self.active = False

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(6)]
    a, b = args[:2]
    result = None
    if name == "SINGLETON_GET": result = self.user
    elif name == "PARAMETER": result = 3
    elif pc == self.game + 0x0251E9D0:
      key = self.read_string(b)
      assert key in ("G_拠点_闘技場済み", "G_Fishing_PlayCount")
      result = self.arena_spent if key == "G_拠点_闘技場済み" else self.fishing_remaining
    elif name == "SET_NUMBER":
      assert self.read_string(b) == "G_Fishing_PlayCount" and args[2] == 1
      self.fishing_remaining = 1
      self.fishing_refills += 1
    elif name == "DECREASE":
      self.fishing_remaining -= 1
      self.fishing_decrements += 1
    elif name == "INITIALIZE_METADATA": pass
    elif name == "UNIT_LIST": result = self.units
    elif name == "UNIT_PID": result = self.pids[a]
    elif name == "UNIT_NAME": result = self.names[a]
    elif name == "COOK_DATA": result = self.cooks.get(self.read_string(a), 0)
    elif name == "CURRENT_CHEF": result = self.pids[self.unit_a]
    elif name == "GET_COOK_DATA": result = self.cooks["PID_a"]
    elif name == "TASTE_DATA": result = self.tastes.get(self.read_string(a), 0)
    elif name in ("MESS_GET", "MESSAGE"):
      # Literal localized text is not a message key. The previous identity
      # stand-in hid accidental second translations of TasteData.Name.
      result = a if self.read_string(a).startswith("MID_") else self.string("")
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

  def exercise_preferences(self, callbacks, hooks, stop, stack):
    self.active = True
    def call(address, *args):
      for i in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{i}"), args[i] if i < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(address, stop + 4, count=1_000_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(reg.UC_ARM64_REG_X0)
    items = [call(callback) for callback in callbacks]
    for item in items:
      copied = self.alloc(self.VTABLE_BYTES)
      original = bytes(self.cpu.mem_read(self.pointer(item), self.VTABLE_BYTES))
      self.write(copied, original)
      self.qword(item, copied)
      assert bytes(self.cpu.mem_read(copied + 0x138 + 27 * 16, 16)) == bytes(self.cpu.mem_read(self.item_class + 0x138 + 27 * 16, 16))
    expected = ["龙之时水晶次数", "战斗存档", "专属服装", "一般锻炼次数", "牵绊戒指精炼次数", "自动完成支援对话", "料理厨师", "料理结果"]
    assert [self.read_string(self.pointer(item + 0x68)) for item in items] == expected
    assert [self.read_string(self.pointer(item + 0x70)) for item in items[:4]] == ["不减", "开启", "开启", "10"]
    def arrows():
      return [bool(self.cpu.mem_read(item + 0x80, 1)[0]) for item in items]
    def selection(index, slot):
      item = items[index]
      call(self.pointer(self.pointer(item) + 0x138 + slot * 16), item)
    assert not any(arrows()), "unselected rows must not start with arrows enabled"
    saved = dict(self.files)
    # First entry, one-row moves, reverse traversal and wrap; no prior visits
    # or preference changes may be needed to initialize a row correctly.
    current = None
    for index in [0, 1, 0, 7, *range(7, -1, -1), *range(8)]:
      if current is not None: selection(current, 13)
      selection(index, 12)
      assert arrows() == [i == index for i in range(8)]
      current = index
    selection(current, 13)
    assert not any(arrows()) and self.files == saved
    reopened = [call(callback) for callback in callbacks]
    assert not any(self.cpu.mem_read(item + 0x80, 1)[0] for item in reopened)
    def change(index, value):
      self.next_count = value
      item = items[index]
      return call(self.pointer(self.pointer(item) + 0x138 + 26 * 16), item)
    self.arena_spent = 3
    arena_key = self.metadata(0x06295CA8, self.string("ArenaLimit"))
    used_key = self.metadata(0x06291A58, self.string("G_拠点_闘技場済み"))
    self.metadata(0x062840F0)
    self.qword(self.user + 0x50, self.alloc())
    for count in (1, 37, 1000, 0):
      assert change(0, count) == 4096
      actual = call(hooks[0x02707DD0], self.user)
      assert actual == (0xFFFFFFFF if count == 0 else count)
      word = self.number(self.game + 0x0270DB44) & 0xFFFFFFFF
      assert word == (0x11000108 if count == 0 else 0x11000508)
    assert call(hooks[0x02280C20], arena_key) == 10
    assert change(3, 0) == 4096
    for used in (0, 3, 27):
      self.arena_spent = used
      assert call(hooks[0x02280C20], arena_key) == used + 3
    assert call(hooks[0x02280C20], used_key) == 3, "unrelated parameters must remain native"
    assert change(3, 10) == 4096
    for enabled in (0, 1):
      assert change(1, enabled) == 4096
      for address in (0x02677284, 0x01E40CF8, 0x01E40E88):
        assert self.number(self.game + address) & 0xFFFFFFFF == (0x52800028 if enabled else 0xB9402808)
    # Failure must restore runtime instructions as well as the previous label.
    self.failure = "mount"
    assert change(0, 20) == 2048
    assert call(hooks[0x02707DD0], self.user) == 0xFFFFFFFF
    assert self.number(self.game + 0x0270DB44) & 0xFFFFFFFF == 0x11000108
    self.failure = ""
    assert change(2, 0) == 4096
    assert self.read_string(self.pointer(items[2] + 0x70)) == "关闭"
    assert b"personal_outfits=0\n" in self.files[self.paths[0]]
    self.active = False
    self.appearance.exercise_disabled()
    self.active = True
    assert change(2, 1) == 4096
    self.active = False
    self.appearance.exercise()
    print("compiled settings: initial arrows, finite/nondecreasing counts, reversible saves, disabled outfits and mount-failure rollback passed")

  def exercise_features(self, hooks, stop, stack, check_disabled):
    self.active = True
    self.fishing_remaining, self.fishing_refills, self.fishing_decrements = 3, 0, 0
    self.integer(self.user + 0x28, 4)
    self.qword(self.user + 0x50, self.alloc())
    self.metadata(0x062840F0)

    def call(address, *args):
      for i in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{i}"), args[i] if i < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(address, stop + 4, count=1_000_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

    def change(item, value):
      self.next_count = value
      return call(self.pointer(self.pointer(item) + 0x138 + 26 * 16), item)

    keys = ["fishing_unlimited", "shop_stock", "gift_stock", "repeat_cooking", "fast_fishing", "fast_training", "fast_wyvern"]
    items = [call(self.factories[kind]) for kind in range(8, 15)]
    assert [self.read_string(self.pointer(item + 0x70)) for item in items] == ["开启"] * 7
    call(hooks[0x026FFF90], self.user)
    assert self.fishing_remaining == 3 and self.fishing_decrements == 0
    for item, key in zip(items, keys):
      assert change(item, 0) == 4096
      assert self.read_string(self.pointer(item + 0x70)) == "关闭"
      assert f"{key}=0\n".encode() in self.files[self.paths[0]]
    call(hooks[0x026FFF90], self.user)
    assert self.fishing_remaining == 2 and self.fishing_decrements == 1
    check_disabled()
    for key in keys:
      assert f"{key}=0\n".encode() in self.files[self.paths[0]]
    self.fishing_remaining = 0
    for item, key in zip(items, keys):
      assert change(item, 1) == 4096
      assert f"{key}=0\n".encode() not in self.files[self.paths[0]]
    assert self.fishing_remaining == self.fishing_refills == 1
    call(hooks[0x026FFF90], self.user)
    assert self.fishing_remaining == 1 and self.fishing_decrements == 1
    # Turning fast fishing off must not turn nondecreasing uses off.
    assert change(items[4], 0) == 4096
    call(hooks[0x026FFF90], self.user)
    assert self.fishing_remaining == 1
    assert change(items[4], 1) == 4096
    self.active = False
    print("compiled seven independent switches: persisted off/on, native fishing decrement, exhausted-use recovery and separate fast-fishing control passed")

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
      original_init = bytes(self.cpu.mem_read(self.item_class + 0x138 + 27 * 16, 16))
      copied = self.alloc(self.VTABLE_BYTES)
      self.write(copied, bytes(self.cpu.mem_read(self.pointer(item), self.VTABLE_BYTES)))
      self.qword(item, copied)
      assert bytes(self.cpu.mem_read(copied + 0x138 + 27 * 16, 16)) == original_init
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

    localized_names = (
      ("霸王的", "美味的", "入口即化的"),
      ("Savory", "Delicious", "Melting"),
      ("覇王の", "おいしい", "とろける"),
    )
    for names in localized_names:
      for index, label in enumerate(names):
        self.qword(self.tastes[f"taste{index}"] + 0x28, self.string(label))
      for option, label in enumerate(names, 1):
        assert change(outcome, option) == 4096
        assert self.read_string(self.pointer(outcome + 0x70)) == label, "display localized TasteData.Name directly, without a second message lookup"
        assert label in self.read_string(self.pointer(outcome + 0x78))
    assert change(outcome, 0) == 4096
    for index, label in enumerate(localized_names[0]):
      self.qword(self.tastes[f"taste{index}"] + 0x28, self.string(label))
    assert change(outcome, 3) == 4096
    detail = self.read_string(self.pointer(outcome + 0x78))
    for part in ("SS", "入口即化的", "Stat1 +3", "Stat2 -1", "料理基础能力 +2", "其他能力 +1", "依条件变化"):
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
    for sequence in (2, 3, 5, 6):
      self.integer(self.user + 0x28, sequence)
      before_actions = len(self.actions)
      assert call(pair_key, pairs, pair_method) == 99, "outside the Somniel keep native partner-menu input"
      assert call(root_key, root, root_method) == 99, "outside the Somniel keep native unit-menu input"
      assert len(self.actions) == before_actions, "never synthesize A/B while native input owns the menu"
    self.integer(self.user + 0x28, 4)
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
