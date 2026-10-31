"""Automatic polishing at native API boundaries, not a Unity scene test."""

import struct
from unicorn import arm64_const as reg
from convenience_fixture import ConvenienceFixture


class PolishingFixture(ConvenienceFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0xC800000
    self.functions[self.game + 0x02393F60] = "UNIT_LIST"
    for offset, name in (
      (0x01A39DF0, "UNIT_CAN_BOND"),
      (0x02B4F3E0, "BOND_LOOKUP"),
      (0x02B4D400, "BOND_READY"),
      (0x02B4DCE0, "BOND_CAP"),
      (0x02B4D990, "BOND_ADD"),
      (0x027C8D10, "POLISH_COUNT"),
    ):
      self.functions[self.game + offset] = name
      self.write(self.game + offset, bytes.fromhex("c0035fd6"))

  def handle(self, pc, import_name):
    if not self.active:
      return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(4)]
    a, b = args[:2]
    result = None
    if name == "GOD_POOL":
      assert a == self.god_list_method
      self.scans += 1
      result = self.pool
    elif name == "UNIT_CAN_BOND":
      assert a in self.pids
      result = 1
    elif name == "BOND_LOOKUP": result = self.relations.get((a, b), 0)
    elif name == "BOND_READY": result = int(self.exp(a) < self.caps[a] - 1)
    elif name == "BOND_CAP": result = self.caps[a]
    elif name == "BOND_ADD":
      self.rewards.append((a, b))
      self.write(a + 0x2A, struct.pack("<H", self.exp(a) + b))
      result = 1
    elif name == "POLISH_COUNT":
      assert (a, b) == (31, 1)
      self.completions += 1
    elif name == "RECORD_GOD_POLISH": self.god_completions.append(self.read_string(a))
    elif name == "HUB_IS_MAIN": result = int(self.is_main)
    elif name in ("HUB_INIT", "HUB_TICK"): pass
    elif name == "RELATION": result = 0
    elif name in ("HAS_BOND", "ADD_BOND", "GOD_DATA", "GET_DIRTY", "SET_DIRTY", "GET_STATE", "RECORD_POLISH"):
      return False  # Run the independently verified native function bodies.
    else:
      return super().handle(pc, import_name)
    if result is not None:
      self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exp(self, relation):
    return struct.unpack("<H", self.cpu.mem_read(relation + 0x2A, 2))[0]

  def exercise_polishing(self, hooks, menu_hook):
    self.active = True
    self.user, self.hub = self.alloc(), self.alloc()
    self.integer(self.user + 0x28, 4)
    self.qword(self.user + 0x50, self.alloc())
    self.metadata(0x062840F0)
    self.is_main, self.scans, self.completions = True, 0, 0
    self.rewards, self.god_completions = [], []
    self.arena_spent, self.fishing_remaining = 0, 3
    alear, panette = self.alloc(), self.alloc()
    self.units = self.list([alear, panette, alear])
    self.pids = {alear: self.string("PID_リュール"), panette: self.string("PID_パネトネ")}
    self.names = {alear: self.string("Custom Alear"), panette: self.string("帕涅托涅")}
    self.god_list_method = self.metadata(0x0628DD48)
    for offset, code in (
      (0x02340600, "fd7bbea9f44f01a9fd030091f30301aaf40300aae00301aae1031faaf5e5db976000003688c24139a8000034f44f41a9e0031f2afd7bc2a8c0035fd6801e40f9e10313aae2031faa663b209400ffffb4f44f41a9e1031faafd7bc2a869332014"),
      (0x02340700, "fd7bbea9f44f01a9fd030091001c40f9f303022ae2031faa323b2094c00200b4e1031faaf40300aa6e352094885640790801130b1f00086b2c010054e00314aae1031faa6735209488564079e803282a1300080b7f060071eb000054e00314aae103132ae2031faaf44f41a9fd7bc2a888342014f44f41a9e0031f2afd7bc2a8c0035fd6"),
      (0x0233EB20, "fd7bbda9f50b00f9fd030091f44f02a9f30300aa001040f9e00200b4741640f9b40200b435fa01d0b5ea40f9a8024039c800003520fa01d000e440f9414d859728008052a8020039940a40f9e00314aa880240f9098958a90110a05220013fd6880a40f91f0100ea601240f940000054008040f9f44f42a9f50b40f9fd7bc3a8c0035fd6"),
      (0x023353F0, "081440f9a80000b4082940f9e80000b4002140b9c0035fd6081040f9680000b4009540b9c0035fd6e0008052c0035fd6"),
      (0x027CA680, "fd7bbea9f44f01a9fd03009173da01f0f4d501b0684a7439946a45f9c8000037e0d501b0006845f96e1e739728008052684a3439800240f908cc44398800083608e040b948000035ae1e7397f44f41a9e003805221008052fd7bc2a88df9ff17"),
    ):
      self.write(self.game + offset, bytes.fromhex(code))
    # These raw methods have no scene, UI or static selected-cleaner accesses.
    self.write(self.game + 0x0233EB00, bytes.fromhex("00304139c0035fd6"))
    self.write(self.game + 0x0233EB10, bytes.fromhex("01300139c0035fd6"))
    self.metadata(0x06287AD0)
    self.write(self.game + 0x06319D12, b"\0")
    self.relations, self.caps = {}, {}
    gods = []
    for index in range(8):
      god, data, dictionary = self.alloc(), self.alloc(), self.alloc()
      self.qword(god + 0x20, data)
      self.qword(god + 0x38, dictionary)
      self.qword(data + 0x20, self.string(f"GID_fixture_{index}"))
      self.integer(data + 0x94, 3 if index == 1 else 0)
      self.write(god + 0x4C, bytes([0 if index == 2 else 10]))
      for actor in (alear, panette):
        relation = self.alloc()
        self.relations[(dictionary, actor)] = relation
        self.caps[relation] = 100
        self.write(relation + 0x2A, struct.pack("<H", 98 if index == 1 else 99 if index == 7 else 10))
      gods.append(god)
    self.write(gods[3] + 0x4A, b"\1")
    self.write(gods[4] + 0x4B, b"\1")
    self.integer(self.pointer(gods[5] + 0x20) + 0x94, 7)
    self.write(gods[6] + 0x70, b"\1")
    self.rings = self.list(gods + [gods[0], 0])
    self.pool = self.alloc()
    self.qword(self.pool + 0x28, self.rings)
    # Pool is not a List: its list-header offsets intentionally remain zero.
    assert self.pointer(self.pool + 0x10) == 0 and self.number(self.pool + 0x18) == 0
    actor_item = self.invoke(self.factories[5])
    toggle = self.invoke(self.factories[4])
    assert self.read_string(self.pointer(actor_item + 0x70)) == "Custom Alear"
    assert self.read_string(self.pointer(toggle + 0x70)) == "开启"
    assert not self.read_string(self.pointer(actor_item + 0x78))
    assert not self.read_string(self.pointer(toggle + 0x78))

    def change(item, index):
      self.next_count = index
      return self.invoke(self.pointer(self.pointer(item) + 0x138 + 26 * 16), item)

    def relation(index, actor):
      return self.relations[(self.pointer(gods[index] + 0x38), actor)]

    def dirty(index):
      return self.cpu.mem_read(gods[index] + 0x4C, 1)[0]

    self.invoke(hooks["HUB_INIT"], self.hub)
    self.is_main = False
    self.invoke(hooks["HUB_TICK"], self.hub)
    self.is_main = True
    self.qword(self.hub + 0x38, self.alloc())
    self.invoke(hooks["HUB_TICK"], self.hub)
    assert self.scans == 0
    self.qword(self.hub + 0x38, 0)
    self.invoke(hooks["HUB_TICK"], self.hub)
    assert [dirty(i) for i in range(8)] == [0, 0, 0, 10, 10, 10, 0, 0]
    assert self.exp(relation(0, alear)) == 13 and self.exp(relation(0, panette)) == 10
    assert self.exp(relation(1, alear)) == 99, "the native reward must respect the bond threshold"
    assert len(self.rewards) == 2 and self.completions == 4
    assert self.god_completions == [f"GID_fixture_{i}" for i in (0, 1, 6, 7)]
    scans = self.scans
    for _ in range(16): self.invoke(hooks["HUB_TICK"], self.hub)
    assert self.scans == scans, "idle frames must not rescan rings"

    support = self.invoke(self.factories[3])
    assert change(support, 0) == 4096
    assert change(actor_item, 1) == 4096
    assert b"polish_actor=PID_" in self.files[self.paths[0]]
    assert self.read_string(self.pointer(actor_item + 0x70)) == "帕涅托涅"
    assert self.read_string(self.pointer(self.invoke(self.factories[5]) + 0x70)) == "帕涅托涅"
    # Re-enter without opening a selector; the selected cleaner owns the reward.
    self.write(gods[0] + 0x4C, b"\5")
    self.invoke(hooks["HUB_INIT"], self.hub)
    self.invoke(hooks["HUB_TICK"], self.hub)
    assert self.exp(relation(0, panette)) == 13 and self.exp(relation(0, alear)) == 13
    assert self.completions == 5 and not self.pointer(self.hub + 0x38)

    # The same scan on ordinary menu opening is idempotent, without changing selection.
    self.reset(4)
    self.write(gods[0] + 0x4C, b"\5")
    self.integer(self.menu + 0xA0, 2)
    self.invoke(menu_hook, self.parent)
    assert dirty(0) == 0 and self.exp(relation(0, panette)) == 16
    assert self.number(self.menu + 0xA0) == 2 and not self.pointer(self.menu + 0x38)
    self.invoke(menu_hook, self.parent)
    assert self.completions == 6

    self.write(gods[0] + 0x4C, b"\5")
    assert change(toggle, 0) == 4096 and b"auto_polish=0\n" in self.files[self.paths[0]]
    self.invoke(hooks["HUB_INIT"], self.hub)
    self.invoke(hooks["HUB_TICK"], self.hub)
    self.invoke(menu_hook, self.parent)
    assert dirty(0) == 5 and self.completions == 6
    assert change(toggle, 1) == 4096
    for sequence in (2, 3, 5, 6):
      self.integer(self.user + 0x28, sequence)
      self.invoke(hooks["HUB_INIT"], self.hub)
      self.invoke(hooks["HUB_TICK"], self.hub)
      self.invoke(menu_hook, self.parent)
    assert dirty(0) == 5 and self.completions == 6, "no automatic polishing outside the Somniel"
    self.integer(self.user + 0x28, 4)
    self.units = self.list([alear])
    self.invoke(menu_hook, self.parent)
    assert dirty(0) == 5, "do not consume a dirty ring when the selected cleaner is absent"
    self.units = self.list([alear, panette])
    self.invoke(menu_hook, self.parent)
    assert dirty(0) == 0 and self.exp(relation(0, panette)) == 19
    self.active = False
