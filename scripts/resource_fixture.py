"""One-shot resource UI checks; execute pinned gold/bond/SP getter/setter bodies."""

import struct
from unicorn import arm64_const as reg
from settings_fixture import SettingsFixture

NATIVE_BODIES = [
  [
    38855744,
    "005840b9c0035fd61f2003d51f2003d5"
  ],
  [
    38855760,
    "fd7bbea9f30b00f9fd030091f30300aae2cf92520213a072e003012ae1031f2ae3031faaab453794605a00b9f30b40f9fd7bc2a8c0035fd61f2003d51f2003d5"
  ],
  [
    38857904,
    "006c40b9c0035fd61f2003d51f2003d5"
  ],
  [
    38857920,
    "fd7bbda9f50b00f9fd030091f44f02a9f4cf92521413a072f30300aae003012ae103142ae2031faabe413794686e40b90800084bf503002a1f050071eb000054697240b92001080be103142ae2031faab4413794607200b9756e00b9f44f42a9f50b40f9fd7bc3a8c0035fd61f2003d5"
  ],
  [
    27610448,
    "007cc379c0035fd61f2003d51f2003d5"
  ],
  [
    27320464,
    "017c0379c0035fd61f2003d51f2003d5"
  ]
]

IIDS = ["IID_てつの晶石", "IID_はがねの晶石", "IID_ぎんの晶石",
  "IID_素材_威力", "IID_素材_必殺", "IID_素材_技", "IID_素材_速さ",
  "IID_素材_守備", "IID_素材_魔防", "IID_素材_命中", "IID_素材_回避", "IID_素材_必避"]


class ResourceFixture(SettingsFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0xBC00000
    self.resource_ready = False
    self.idle = False
    for offset, name in (
      (0x01F23010, "PAD_TRIGGER"), (0x02393F60, "RESOURCE_UNITS"),
      (0x01A35BF0, "RESOURCE_PID"), (0x01A22770, "RESOURCE_NAME"),
      (0x001EA710, "RESOURCE_ITEM"), (0x027B1920, "RESOURCE_GET"),
      (0x027B1A30, "RESOURCE_SET"), (0x032DFB20, "CLAMP"),
      (0x032DF3E0, "MINIMUM"),
    ):
      self.functions[self.game + offset] = name
      self.write(self.game + offset, struct.pack("<I", 0xD65F03C0))

  def handle(self, pc, import_name):
    if not self.active or not self.resource_ready:
      return super().handle(pc, import_name)
    name = self.functions.get(pc, import_name)
    if self.idle:
      assert name not in ("RESOURCE_UNITS", "RESOURCE_ITEM", "RESOURCE_GET", "SINGLETON_GET",
        "malloc", "calloc", "ALLOCATE_OBJECT"), "idle numeric rows must not scan, read inventory or allocate"
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(5)]
    a, b = args[:2]
    result = None
    if name == "PAD_TRIGGER":
      assert a == 1
      result = int(self.confirm)
    elif name == "RESOURCE_UNITS": result = self.units
    elif name == "RESOURCE_PID": result = self.pids[a]
    elif name == "RESOURCE_NAME": result = self.unit_names[a]
    elif name == "RESOURCE_ITEM":
      assert b == self.item_method
      result = self.inventory_items.get(self.read_string(a), 0)
    elif name == "RESOURCE_GET": result = self.quantities[a]
    elif name == "RESOURCE_SET":
      assert a in self.quantities and 0 <= b <= (9999 if a in self.ingots else 999)
      self.quantities[a] = b
      self.writes.append((a, b))
    elif name == "CHANGE_NUMBER":
      assert args[1:4] == [0, 2, 1] and a == 1
      result = self.direction + 1
    elif name == "CLAMP": result = min(max(a, b), args[2])
    elif name == "MINIMUM": result = min(a, b)
    else: return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise_resources(self, callbacks, stop, stack):
    # Check all 33 titles/inherited callbacks and legacy configuration first.
    self.exercise(callbacks, stop, stack)
    self.active = self.resource_ready = True
    self.confirm, self.direction, self.target = False, 0, 0
    self.user = self.alloc()
    self.integer(self.user + 0x58, 327459)
    self.integer(self.user + 0x6C, 327459)
    self.integer(self.user + 0x70, 456789)
    self.item_method = self.metadata(0x06287888)
    first, bench = self.alloc(0x300), self.alloc(0x300)
    self.units = self.list([first, bench, first, 0])
    self.pids = {first: self.string("PID_first"), bench: self.string("PID_bench")}
    self.unit_names = {first: self.string("琉尔"), bench: self.string("安娜")}
    self.write(first + 0x1BE, struct.pack("<h", 827))
    self.write(bench + 0x1BE, struct.pack("<h", 1243))
    self.inventory_items = {iid: self.alloc() for iid in IIDS}
    self.quantities = {item: 237 for item in self.inventory_items.values()}
    self.ingots = set(list(self.inventory_items.values())[:3])
    self.writes = []
    for offset, body in NATIVE_BODIES:
      self.write(self.game + offset, bytes.fromhex(body))
    self.resource_rows = {}

    def open_rows():
      self.invoke(self.factories[15])  # New page resets transient targets.
      self.target = 0
      self.resource_rows = {kind: self.invoke(self.factories[kind]) for kind in range(17, 33)}
      return self.resource_rows

    def label(kind):
      return self.read_string(self.pointer(self.resource_rows[kind] + 0x70))

    def custom(kind, direction=0, confirm=False, target=None):
      self.direction, self.confirm = direction, confirm
      if target is not None:
        self.direction = 1 if (target - self.target) % 3 == 1 else -1
        self.target = target
      item = self.resource_rows[kind]
      callback = self.pointer(self.pointer(item) + 0x138 + 26 * 16)
      result = self.invoke(callback, item)
      self.confirm = False
      return result

    rows = open_rows()
    saved_files = dict(self.files)
    native_before = bytes(self.cpu.mem_read(self.user, 0x100))
    assert label(17) == "全员" and label(18) == "不一致"
    assert label(19) == label(20) == "327,459"
    assert all(label(kind) == "237" for kind in range(21, 33))
    self.idle = True
    for kind in range(17, 33):
      for _ in range(4): assert custom(kind) == 0
    self.idle = False
    assert custom(19, 1) == 4096 and label(19) == "427,459"
    assert bytes(self.cpu.mem_read(self.user, 0x100)) == native_before, "preview must not write game data"
    assert self.files == saved_files and not self.writes
    assert custom(19, confirm=True) == 128
    assert self.number(self.user + 0x58) == 427459
    assert self.number(self.user + 0x6C) == 327459 and self.number(self.user + 0x70) == 456789
    assert custom(19, confirm=True) == 0, "confirm consumes the draft once"
    self.integer(self.user + 0x58, 1234)
    assert custom(19) == 0 and label(19) == "427,459"  # No background UI polling.
    open_rows()
    assert label(19) == "1,234", "reopening reads actual quantities; no lock/refill"

    assert custom(20, 1) == 4096 and custom(20, confirm=True) == 128
    assert self.number(self.user + 0x6C) == 427459
    assert self.number(self.user + 0x70) == 556789, "native positive bond delta retains lifetime accounting"
    assert custom(20, -1) == 4096 and custom(20, confirm=True) == 128
    assert self.number(self.user + 0x6C) == 327459 and self.number(self.user + 0x70) == 556789

    for kind in range(21, 33):
      step = 100 if kind <= 23 else 10
      item = self.inventory_items[IIDS[kind - 21]]
      before = dict(self.quantities)
      assert custom(kind, 1) == 4096 and self.quantities == before
      assert custom(kind, confirm=True) == 128
      assert self.quantities[item] == 237 + step
      assert all(self.quantities[key] == value for key, value in before.items() if key != item)

    # All SP is an absolute assignment, not a per-unit increment.
    units_before = {unit: bytes(self.cpu.mem_read(unit, 0x300)) for unit in (first, bench)}
    assert custom(18, 1) == 4096 and label(18) == "100"
    assert self.cpu.mem_read(first + 0x1BE, 2) == struct.pack("<h", 827)
    assert custom(18, confirm=True) == 128
    assert [int.from_bytes(self.cpu.mem_read(unit + 0x1BE, 2), "little") for unit in (first, bench)] == [100, 100]
    for unit, original in units_before.items():
      expected = original[:0x1BE] + struct.pack("<h", 100) + original[0x1C0:]
      assert bytes(self.cpu.mem_read(unit, 0x300)) == expected, "SP edits leave all other unit fields untouched"
    assert custom(17, target=1) == 4096 and label(17) == "琉尔" and label(18) == "100"
    assert custom(18, 1) == 4096
    assert custom(17, target=2) == 4096 and label(17) == "安娜" and label(18) == "100"
    assert custom(18, -1) == 4096 and custom(18, confirm=True) == 128
    assert int.from_bytes(self.cpu.mem_read(first + 0x1BE, 2), "little") == 100
    assert int.from_bytes(self.cpu.mem_read(bench + 0x1BE, 2), "little") == 0
    assert custom(17, target=1) == 4096 and label(18) == "100", "discard previous ally's unconfirmed draft"

    for unit in (first, bench): self.write(unit + 0x1BE, struct.pack("<h", 9998))
    open_rows()
    assert label(18) == "9,998"
    assert custom(18, 1) == 4096 and custom(18, 1) == 0
    assert custom(18, confirm=True) == 128 and label(18) == "9,999"
    for kind, maximum in ((21, 9999), (24, 999)):
      item = self.inventory_items[IIDS[kind - 21]]
      self.quantities[item] = maximum - 1
      open_rows()
      assert custom(kind, 1) == 4096 and custom(kind, 1) == 0
      assert custom(kind, confirm=True) == 128 and self.quantities[item] == maximum

    # Limits clamp without wrapping or rounding the existing tail.
    self.integer(self.user + 0x58, 9_999_998)
    open_rows()
    assert custom(19, 1) == 4096 and label(19) == "9,999,999"
    assert custom(19, 1) == 0 and custom(19, confirm=True) == 128
    assert self.number(self.user + 0x58) == 9_999_999
    self.integer(self.user + 0x58, 1)
    open_rows()
    assert custom(19, -1) == 4096 and label(19) == "0"
    assert custom(19, -1) == 0
    open_rows()
    assert label(19) == "1", "cancel/reopen discards pending quantities"
    assert self.files == saved_files, "resource targets never enter the config file"

    # No loaded save disables editing, rather than creating game variables.
    self.user = 0
    open_rows()
    for kind, item in self.resource_rows.items():
      enabled = self.pointer(self.pointer(item) + 0x138 + 8 * 16)
      assert self.invoke(enabled, item) == 0
      assert custom(kind, 1) == custom(kind, confirm=True) == 0
    self.active = self.resource_ready = False
