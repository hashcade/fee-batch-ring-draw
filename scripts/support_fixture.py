"""Direct support settlement with pinned native level-up and achievement bodies."""

from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class SupportFixture(CollectorFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0xB800000
    for offset, name in (
      (0x00492060, "INITIALIZE_METADATA"),
      (0x02229A50, "RELIANCE_LOOKUP"),
      (0x01C5AA50, "LINK_GOD_UNITS"),
      (0x01F2A0B0, "IS_HERO"),
      (0x027C8CB0, "ACHIEVE_B"),
      (0x027C91D0, "ACHIEVE_A"),
      (0x027C8910, "ACHIEVE_CLEAR"),
      (0x0226F100, "FRIEND_STATE"),
      (0x01A35BF0, "UNIT_PID"),
      (0x0251E9D0, "VARIABLE_NUMBER"),
    ):
      self.functions[self.game + offset] = name
      self.write(self.game + offset, bytes.fromhex("c0035fd6"))
    for offset in (0x02396290, 0x02393B40, 0x023970A0):
      self.functions[self.game + offset] = "FORBIDDEN_MENU"

  def handle(self, pc, import_name):
    if not self.active:
      return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(5)]
    a, b = args[:2]
    result = None
    if self.idle:
      assert name not in ("UNIT_LIST", "EMPTY_UNIT_LIST", "RELATION", "CAN_ADVANCE",
        "LEVEL_UP", "UPDATE_ACHIEVE", "ALLOCATE_OBJECT", "malloc"), "idle ticks must not rescan or allocate"
    if name in ("LEVEL_UP", "UPDATE_ACHIEVE"):
      if name == "LEVEL_UP":
        self.level_calls.append(tuple(sorted((a, b))))
      else:
        assert a == 0, "the verified UpdateAchive body does not use its receiver"
        self.achievement_calls.append((b, args[2], args[3]))
      return False  # Execute the original native bodies installed below.
    if name in ("UNIT_LIST", "EMPTY_UNIT_LIST"):
      self.scans += 1
      result = self.units
    elif name == "RELATION":
      result = self.relations.get(tuple(sorted((a, b))), (0, False))[0]
    elif name == "CAN_ADVANCE":
      pair = tuple(sorted((a, b)))
      relation, ready = self.relations[pair]
      result = int(ready and self.number(relation + 0x18) == self.required_levels[pair])
    elif name == "RELIANCE_LOOKUP":
      assert a == self.dictionary and args[4] == self.lookup_method
      relation = self.index_relations.get(tuple(sorted((b, args[2]))), 0)
      self.cpu.mem_write(args[3], relation.to_bytes(8, "little"))
      result = int(bool(relation))
    elif name == "LINK_GOD_UNITS":
      self.links.append((a, b, args[2]))
    elif name == "IS_HERO": result = int(a == self.hero)
    elif name in ("ACHIEVE_B", "ACHIEVE_A"):
      self.achievements[name] += 1
    elif name == "ACHIEVE_CLEAR": self.cleared.append(a)
    elif name == "UNIT_PID": result = self.pids[a]
    elif name == "FRIEND_STATE": self.friends.append(self.read_string(a))
    elif name == "HUB_IS_MAIN": result = int(self.is_main)
    elif name in ("HUB_INIT", "HUB_TICK"): pass
    elif name == "VARIABLE_NUMBER": result = 3
    elif name == "FORBIDDEN_MENU":
      raise AssertionError("automatic support settlement must never open a selector")
    else:
      return super().handle(pc, import_name)
    if result is not None:
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, hooks, menu_hook, stop, stack, enabled=True):
    self.active = True
    self.idle = False
    self.reset(4)
    self.is_main, self.scans = True, 0
    self.level_calls, self.achievement_calls, self.links = [], [], []
    self.achievements = {"ACHIEVE_B": 0, "ACHIEVE_A": 0}
    self.cleared, self.friends = [], []
    self.write(self.game + 0x1C5A950, bytes.fromhex(
      "fd7bbda9f50b00f9fd030091f44f02a9d53502f0a89e4939f30301aaf40300aa28010037a03102b000f445f9b9dda097"
      "a03102b000f845f9b6dda09728008052a89e0939bf0f00f9940300b4730300b4a83102b008f945f9a93102b029f545f9"
      "a3630091811640b9621640b9080140f9085d40f9000540f9240140f9213c1794a80f40f9a80100b480010036021940b9"
      "5f080071cc0000540971403942040011021900b92905001109710039e00314aae10313aa13000094f44f42a9f50b40f9"
      "fd7bc3a8c0035fd61f2003d51f2003d5"))
    self.write(self.game + 0x1C5ABF0, bytes.fromhex(
      "a00000b4810000b4002040f9212040f954ffff17c0035fd61f2003d51f2003d5"))
    self.write(self.game + 0x2098500, bytes.fromhex(
      "fd7bbda9f65701a9fd030091f44f02a9f61302d0c88a7839f503032af30302aaf40301aac8000037600f02f0006845f9"
      "cce68f9728008052c88a3839802240f9e1031faadb46fa97a0000037602240f9e1031faad746fa9700020036bf0e0071"
      "40020054bf0a007141030054680f02f0086945f9000140f908cc44398800083608e040b948000035fee68f97e0031faa"
      "c8c11c940f000014f44f42a9f65741a9fd7bc3a8c0035fd6680f02f0086945f9000140f908cc44398800083608e040b9"
      "48000035efe68f97e0031faa01c31c94802240f9e1031faab646fa97752240f9e0000036680f02f0086945f9000140f9"
      "08cc4439880108370e000014e00315aae1031faaab46fa97a0010036680f02f0086945f9952240f9000140f908cc4439"
      "8800083608e040b948000035d5e68f97e00315aae1031faab6c01c94e00314aae1031faa6b75e697e1031faaad5a0794"
      "e00313aae1031faa6675e697f44f42a9e1031faaf65741a9fd7bc3a8a55a0714"))
    self.write(self.game + 0x06315267, b"\0")
    self.write(self.game + 0x06316E22, b"\0")
    self.lookup_method = self.metadata(0x0628FBE8)
    klass, fields, self.dictionary = (self.alloc() for _ in range(3))
    self.qword(klass + 0xB8, fields)
    self.qword(fields + 8, self.dictionary)
    self.metadata(0x0628FBF0, klass)
    self.metadata(0x06287AD0)
    units = [self.alloc() for _ in range(6)]
    persons = [self.alloc() for _ in units]
    self.hero = persons[0]
    self.pids = {}
    for index, (unit, person) in enumerate(zip(units, persons), 1):
      self.qword(unit + 0x40, person)
      self.integer(person + 0x14, index)
      self.pids[unit] = self.string(f"PID_fixture_{index}")
    self.units = self.list(units + [units[0], 0])
    # Native eligibility is deliberately independent of the plugin's rank gate.
    definitions = [(0, 1, 0, True), (0, 2, 1, True), (0, 3, 2, True),
      (1, 2, 2, True), (0, 4, 1, False), (0, 5, 3, True), (1, 5, 4, True)]
    self.relations, self.index_relations, self.required_levels = {}, {}, {}
    data = []
    for i, j, level, ready in definitions:
      relation = self.alloc()
      self.integer(relation + 0x18, level)
      self.write(relation + 0x1C, bytes([10 + i + j]))
      pair = tuple(sorted((units[i], units[j])))
      self.relations[pair] = (relation, ready)
      self.required_levels[pair] = level
      self.index_relations[(i + 1, j + 1)] = relation
      data.append(relation)

    def call(address, *args):
      for index in range(8):
        self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(address, stop + 4, count=1_000_000)
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop

    def snapshot():
      return [bytes(self.cpu.mem_read(relation, 0x30)) for relation in data]

    # Do not interrupt a story/activity child or any native loading stage.
    call(hooks["HUB_INIT"], self.hub)
    before = snapshot()
    self.is_main = False
    call(hooks["HUB_TICK"], self.hub)
    self.is_main = True
    busy = self.alloc()
    self.qword(self.hub + 0x38, busy)
    call(hooks["HUB_TICK"], self.hub)
    assert snapshot() == before and not self.level_calls
    self.qword(self.hub + 0x38, 0)
    call(hooks["HUB_TICK"], self.hub)
    assert self.pointer(self.hub + 0x38) == 0, "settlement must not bind any UI"
    expected_levels = [1, 2, 3, 3, 1, 3, 4] if enabled else [0, 1, 2, 2, 1, 3, 4]
    assert [self.number(relation + 0x18) for relation in data] == expected_levels
    assert len(self.level_calls) == (4 if enabled else 0)
    assert len(set(self.level_calls)) == len(self.level_calls), "one native level-up per unordered pair"
    assert len(self.links) == len(self.level_calls), "retain native linked-Emblem updates"
    assert len(self.achievement_calls) == len(self.level_calls)
    # Native achievement rules count protagonist B/A supports only.
    assert self.achievements == {"ACHIEVE_B": int(enabled), "ACHIEVE_A": int(enabled)}
    assert self.cleared == (persons[1:4] if enabled else [])
    assert len(self.friends) == (6 if enabled else 0)
    for index, relation in enumerate(data):
      if enabled and index < 4:
        assert self.number(relation + 0x18) == int.from_bytes(before[index][0x18:0x1C], "little") + 1
        assert self.cpu.mem_read(relation + 0x1C, 1)[0] == before[index][0x1C] + 1
      else:
        assert bytes(self.cpu.mem_read(relation, 0x30)) == before[index]
    after = snapshot()
    self.idle = True
    for _ in range(16):
      call(hooks["HUB_TICK"], self.hub)
    self.idle = False
    assert snapshot() == after

    # Opening an ordinary hub menu settles newly ready pairs in place.
    self.relations[tuple(sorted((units[0], units[4])))] = (data[4], True)
    self.integer(self.menu + 0xA0, 2)
    menu_class = self.pointer(self.menu)
    original_vtable = bytes(self.cpu.mem_read(menu_class, 0x300))
    call(menu_hook, self.parent)
    assert self.pointer(self.parent + 0x38) == self.menu
    assert self.pointer(self.menu + 0x38) == 0
    assert self.pointer(self.menu) == menu_class
    assert bytes(self.cpu.mem_read(menu_class, 0x300)) == original_vtable
    assert self.number(self.menu + 0xA0) == 2
    assert self.number(data[4] + 0x18) == (2 if enabled else 1)
    assert len(self.level_calls) == (5 if enabled else 0)

    # A failed native dictionary lookup cannot award an achievement twice.
    self.integer(data[0] + 0x18, 0)
    self.index_relations.pop((1, 2))
    before_achievements = len(self.achievement_calls)
    call(menu_hook, self.parent)
    assert self.number(data[0] + 0x18) == 0
    assert len(self.achievement_calls) == before_achievements

    for sequence in (2, 3, 5, 6):
      self.integer(self.user + 0x28, sequence)
      call(hooks["HUB_INIT"], self.hub)
      self.scans = 0
      before = snapshot()
      self.idle = True
      for _ in range(4):
        call(hooks["HUB_TICK"], self.hub)
      self.idle = False
      call(menu_hook, self.parent)
      assert self.scans == 0 and snapshot() == before, "only the Somniel may settle automatically"
      assert self.pointer(self.menu + 0x38) == 0
    self.active = False
