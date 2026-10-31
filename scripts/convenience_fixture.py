"""Compiled settings and cooking-context boundary checks."""

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
    elif name == "TASTE_NAME": return False  # Execute the NSO-verified resolver below.
    elif name == "LOAD_TEXT":
      assert self.read_string(a) == "Cook"
      self.cook_text_refs = getattr(self, "cook_text_refs", 0) + 1
      result = 1
    elif name == "FREE_TEXT":
      assert self.read_string(a) == "Cook" and self.cook_text_refs > 0
      self.cook_text_refs -= 1
      result = 1
    elif name in ("MESS_GET", "MESSAGE"):
      key = self.read_string(a)
      if "FOODRANK" in key:
        assert getattr(self, "cook_text_refs", 0) > 0, "load Cook messages before resolving dish names"
      messages = getattr(self, "taste_messages", {})
      if key in messages:
        result = self.string(messages[key])
      elif key.startswith("MID_"):
        result = a
      else:
        result = self.string("")
    elif name == "ANIMATION_CREATE":
      self.animation_calls += 1
      result = self.iterator
    elif name == "DETERMINE_TASTE":
      self.qualities.append((a, b))
      result = self.taste_result
    elif name == "COOK": return False  # Execute the modeled native call below.
    elif name == "CHANGE_NUMBER":
      assert args[2] >= self.next_count >= args[1]
      assert args[3] == (10 if args[1:3] == [2, 1000] else 1)
      result = self.next_count
    elif name == "ALLOCATE_OBJECT":
      result = self.alloc()
      self.qword(result, a)
    else: return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise_preferences(self, hooks):
    self.active = True
    item = self.invoke(self.factories[6])
    change = self.pointer(self.pointer(item) + 0x138 + 26 * 16)
    self.arena_spent = 3
    arena_key = self.metadata(0x06295CA8, self.string("ArenaLimit"))
    other_key = self.metadata(0x06291A58, self.string("G_拠点_闘技場済み"))
    self.metadata(0x062840F0)
    self.qword(self.user + 0x50, self.alloc())
    for count in (1, 37, 1000, 0, 10):
      self.next_count = count
      assert self.invoke(change, item) == 4096
      for spent in (0, 3, 27):
        self.arena_spent = spent
        assert self.invoke(hooks[0x02280C20], arena_key) == (spent + 3 if count == 0 else count)
      assert self.invoke(hooks[0x02280C20], other_key) == 3
      assert self.number(self.game + 0x01CAA7A4) & 0xFFFFFFFF == (0x2A1703F4 if count == 0 else 0x1A9706D4)
    self.failure, self.next_count = "mount", 0
    assert self.invoke(change, item) == 2048
    assert self.number(self.game + 0x01CAA7A4) & 0xFFFFFFFF == 0x1A9706D4
    self.failure = ""
    self.active = False

  def exercise_features(self, hooks, stop, stack, check_disabled, check_stock):
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
    items = [call(self.factories[kind]) for kind in range(7, 14)]
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
    check_stock(False)
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
    # One shared item switch restores gifts and both food paths together,
    # including after reopening. Failed saving restores all previous patches.
    assert change(items[2], 0) == 4096
    check_stock(False)
    reopened = call(self.factories[9])
    assert self.read_string(self.pointer(reopened + 0x70)) == "关闭"
    assert change(reopened, 1) == 4096
    self.failure = "write"
    assert change(items[2], 0) == 2048
    self.failure = ""
    assert self.read_string(self.pointer(items[2] + 0x70)) == "开启"
    check_stock(True)
    self.active = False
    print("compiled seven switches: shared gift/food gate/cost restoration, persistence, failed-save rollback and separate fishing controls passed")

  def exercise_new(self, callbacks, hooks, stop, stack):
    self.active = True
    self.functions[self.game + 0x02393F60] = "UNIT_LIST"
    self.user = self.alloc()
    self.integer(self.user + 0x28, 4)
    self.metadata(0x062840F0)
    self.metadata(0x0628E250)
    self.metadata(0x06292CE0)
    self.metadata(0x062846A0)
    self.write(self.game + 0x06317681, b"\0")
    # TasteData's native display-name resolver, including its final Mess.Get
    # tail call. All 0x70 bytes are checked against the supplied NSO offline.
    self.write(self.game + 0x021DF0C0, bytes.fromhex(
      "fd7bbda9f50b00f9fd030091f44f02a9d5090290340502b0a8065a39945243f9"
      "f30300aac8000037200502b0005043f9dccb8a9728008052a8061a39800240f9"
      "731640f908cc44398800083608e040b9480000351bcc8a97e00313aaf44f42a9"
      "f50b40f9e1031faafd7bc3a845a80f14"))
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
    self.qualities = []
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

    name_keys = ("MID_FOODRANK_Woman", "MID_FOODRANK_Delicious", "MID_FOODRANK_Melt")
    for index, key in enumerate(name_keys):
      self.qword(self.tastes[f"taste{index}"] + 0x28, self.string(key))
    # Real entries from Xzonn/FireEmblemEngageData/messages, not invented
    # translations. The Cook file starts unloaded in this scenario.
    localized_names = (
      ("大胆的", "美味的", "入口即化的"),
      ("Audacious ", "Delicious ", "Mouthwatering "),
      ("大胆な", "おいしい", "とろける"),
    )
    for names in localized_names:
      self.taste_messages = dict(zip(name_keys, names))
      for option, label in enumerate(names, 1):
        assert change(outcome, option) == 4096
        assert self.read_string(self.pointer(outcome + 0x70)) == label, "resolve the stored message key through the native name function"
        assert self.read_string(self.pointer(outcome + 0x78)) == ""
        assert self.cook_text_refs == 0, "reading a name must not leak a message-file reference"
    assert change(outcome, 0) == 4096
    for missing in ("", " ", name_keys[0]):
      self.taste_messages = {name_keys[0]: missing}
      assert change(outcome, 1) == 4096
      assert self.read_string(self.pointer(outcome + 0x70)) == "厨师特色1"
      assert self.cook_text_refs == 0
      assert change(outcome, 0) == 4096
    self.taste_messages = {"MID_FOODRANK_DefinitelySuffer": "令人窒息的"}
    assert change(outcome, 5) == 4096
    assert self.read_string(self.pointer(outcome + 0x70)) == "令人窒息的"
    assert self.cook_text_refs == 0
    self.taste_messages = dict(zip(name_keys, localized_names[0]))
    assert change(outcome, 3) == 4096
    assert self.read_string(self.pointer(outcome + 0x70)) == "入口即化的"
    assert self.read_string(self.pointer(outcome + 0x78)) == ""
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

    assert change(support, 0) == 4096 and b"auto_support=0\n" in self.files[self.paths[0]]
    assert change(support, 1) == 4096
    assert change(chef, 0) == 4096
    assert self.files == {self.paths[0]: b"draw_count=300\n"}
    self.active = False
