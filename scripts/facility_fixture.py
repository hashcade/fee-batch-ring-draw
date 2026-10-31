"""Model facility binding/cleanup boundaries; does not run Unity facility scenes."""

import re
import struct
from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class FacilityFixture(CollectorFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0x9200000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(6)]
    a, b = args[:2]
    result = None
    if name == "ALLOCATE_OBJECT":
      result = self.alloc()
      self.qword(result, a)
    elif name == "DEFAULT_LIST_CTOR": self.init_list(a)
    elif name == "MENU_CTOR":
      self.qword(a + 0x80, b)
      self.qword(a + 0x78, b)
      self.qword(a + 0x70, args[2])
      self.qword(args[2] + 0x28, a)
      assert self.class_name(self.pointer(args[2])) in (b"SortieSubMenuContent", b"HubMenuContent")
    elif name == "SET_MENU_ROWS":
      assert 0 < b <= len(self.row_nodes), "submenu rows must fit the original prefab"
      self.integer(a + 0xB8, b)
    elif name == "GET_LANGUAGE": result = 10
    elif name in ("CONTENT", "PAGE_CONTENT"):
      result = self.alloc()
      self.qword(result, self.submenu_content_class if name == "CONTENT" else self.page_content_class)
    elif name == "CREATE_DESC": result = self.alloc()
    elif name == "BIND":
      self.qword(a + 0x30, b)
      self.qword(b + 0x38, a)
      self.visible[a] = True
    elif name == "BIND_PARENT":
      parent = self.pointer(a + 0x30)
      self.integer(parent + 0xC0, self.number(parent + 0xC0) + 1)
      self.suspensions[a] = parent
    elif name == "SET_SUBMENU_TRANSFORM":
      item = args[2]
      assert self.pointer(a + 0x30) == b and self.pointer(item + 0x10) == b
      assert self.number(a + 0xB8) > 0, "native row count must be configured before positioning"
      assert self.pointer(a + 0x70) != self.pointer(b + 0x70), "each panel needs its own content"
      assert self.class_name(self.pointer(self.pointer(a + 0x70))) == b"SortieSubMenuContent", "only categories open beside the outdoor menu"
      self.anchors[a] = (b, item, self.number(b + 0xA0))
      self.events.append(("anchor", a, b, item))
    elif name == "EMPTY_UNIT_LIST": result = self.list()
    elif name == "VARIABLE_NUMBER":
      result = self.fishing_remaining if self.read_string(b) == "G_Fishing_PlayCount" else self.activity_used
    elif pc == self.game + 0x0251F290:
      assert self.read_string(b) == "G_Fishing_PlayCount" and args[2] == 1
      self.fishing_remaining = 1
    elif name == "RING_MENU": pass
    elif name == "RING_ATTRIBUTE": result = self.ring_attribute
    elif name == "IS_COMPLETE":
      assert not self.read_string(a).startswith("AID_"), "chapter completion must never receive a facility ID"
      result = int(self.chapter_cleared)
    elif name == "FACILITY_GET":
      aid = self.read_string(a)
      assert aid.startswith("AID_") and b == self.facility_get_method
      result = self.facility_data.get(aid, 0)
    elif name == "FACILITY_COMPLETE":
      assert a in self.facility_data.values(), "facility completion requires the facility object, not its ID"
      result = int(self.unlocked and a not in self.locked_facilities)
    elif name == "FACILITY_BIND":
      self.events.append(("bind", pc))
      if pc in (self.game + 0x028A8700, self.game + 0x02174D70, self.game + 0x02AD6C40): assert b == 0
      if pc == self.game + 0x022BC190: assert self.read_string(b) == "AID_水遊び"
      child = self.alloc()
      self.qword(child, self.sequence_class)
      self.qword(child + 0x30, a)
      self.qword(a + 0x38, child)
      self.visible[child] = True
      result = child
    elif name == "DISPOSE":
      self.events.append(("dispose", a))
      self.visible[a] = False
      self.qword(self.pointer(a + 0x30) + 0x38, 0)
      if a in self.suspensions:
        parent = self.suspensions.pop(a)
        self.integer(parent + 0xC0, self.number(parent + 0xC0) - 1)
        assert self.number(parent + 0xC0) >= 0
    elif name == "OPEN_ALL":
      raise AssertionError("side-menu restoration must use each panel's OpenAnime, not OpenAnimeAll")
    elif name in ("OPEN", "CLOSE", "CLOSE_ALL"):
      opening = name.startswith("OPEN")
      self.visible[a] = opening
      # Native *AnimeAll calls the single-menu operation on self and its
      # immediate BasicMenu parent, not merely the menu passed by the plugin.
      parent = self.pointer(a + 0x30)
      if name.endswith("_ALL") and parent in self.visible:
        self.visible[parent] = opening
      if opening:
        event = "open"
      else:
        event = "close"
      self.events.append((event, a))
    elif name == "SHOW_FOOTER":
      assert a == self.title
      self.events.append(("footer", a))
    elif name == "il2cpp_assembly_get_image": result = self.assembly
    elif name == "il2cpp_class_from_name":
      assert a == self.image and self.c_read(b) == b"App" and self.c_read(args[2]) == b"TitleBar"
      result = self.title_class
    elif name == "il2cpp_class_init": assert a == self.title_class
    else: return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def c_read(self, pointer):
    return bytes(self.cpu.mem_read(pointer, 128)).split(b"\0", 1)[0]

  def exercise(self, hook, ring_hook, stop, stack, profile):
    self.active = True
    self.reset(4)
    self.visible = {self.menu: True}
    self.anchors, self.suspensions = {}, {}
    self.unlocked = True
    self.chapter_cleared = True
    self.locked_facilities = set()
    aids = re.findall(r'aid: "(AID_[^"]+)"', profile)
    self.facility_data = {aid: self.alloc() for aid in aids}
    self.facility_get_method = self.alloc()
    cell = self.alloc()
    self.qword(cell, self.facility_get_method)
    self.qword(self.game + 0x06290F98, cell)
    self.functions[self.game + 0x00203E90] = "FACILITY_GET"
    self.functions[self.game + 0x028A80D0] = "FACILITY_COMPLETE"
    self.activity_used = 0
    self.fishing_remaining = 3
    self.qword(self.user + 0x50, self.alloc())
    self.image, self.title, self.title_class, fields = (self.alloc() for _ in range(4))
    self.assembly = self.alloc()
    self.qword(self.assembly, self.image)
    self.qword(self.title_class + 0xB8, fields)
    self.qword(fields, self.title)

    def stub(name, slot):
      address = self.game + 0x03D10000 + slot * 16
      self.write(address, struct.pack("<I", 0xD65F03C0))
      self.functions[address] = name
      return address

    def vtable(cls, slot, name):
      method = self.alloc()
      self.qword(method, stub(name, slot))
      self.qword(cls + 0x138 + slot * 16, self.pointer(method))
      self.qword(cls + 0x138 + slot * 16 + 8, method)

    self.content, content_class, self.submenu_content_class, self.page_content_class = (self.alloc() for _ in range(4))
    self.qword(self.content, content_class)
    self.qword(content_class + 0x10, self.c_string("HubMenuContent"))
    self.qword(self.submenu_content_class + 0x10, self.c_string("SortieSubMenuContent"))
    self.qword(self.page_content_class + 0x10, self.c_string("HubMenuContent"))
    self.qword(self.menu + 0x70, self.content)
    self.write(content_class + 0x126, struct.pack("<H", 31))
    self.row_nodes = [self.alloc() for _ in range(10)]
    self.qword(self.menu_class + 0x10, self.c_string("BasicMenu"))
    self.write(self.menu_class + 0x126, struct.pack("<H", 64))
    for slot, name in ((10, "DISPOSE"), (18, "CREATE_DESC"), (20, "OPEN"),
        (21, "OPEN_ALL"), (22, "CLOSE"), (23, "CLOSE_ALL")):
      vtable(self.menu_class, slot, name)
    hierarchy = self.alloc()
    self.qword(hierarchy, self.menu_class)
    self.qword(self.menu_class + 0xC8, hierarchy)
    self.write(self.menu_class + 0x12C, b"\x01")
    self.metadata(0x0628BC30, self.menu_class)
    self.sequence_class = self.alloc(0x138 + 32 * 16)
    self.write(self.sequence_class + 0x126, struct.pack("<H", 19))
    vtable(self.sequence_class, 10, "DISPOSE")
    cls, methods, ctor = (self.alloc() for _ in range(3))
    self.qword(ctor, self.game + 0x03DE8880)
    self.qword(ctor + 0x10, self.c_string(".ctor"))
    self.qword(methods, ctor)
    self.qword(cls + 0x98, methods)
    self.write(cls + 0x11C, struct.pack("<H", 1))
    self.qword(self.full_list, cls)
    binds = [int(value, 16) for value in re.findall(r'bind: (0x[0-9A-Fa-f]+)', profile)]
    assert len(binds) == 17
    for offset in binds: self.functions[self.game + offset] = "FACILITY_BIND"

    def call(address, *args):
      for i in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{i}"), args[i] if i < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      try:
        self.cpu.emu_start(address, stop + 4, count=1_000_000)
      except Exception as error:
        raise AssertionError(f"facility boundary at {self.cpu.reg_read(reg.UC_ARM64_REG_PC):#x}") from error
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

    def method(object, slot): return self.pointer(self.pointer(object) + 0x138 + slot * 16)

    def shown(*menus):
      actual = {item for item, visible in self.visible.items() if visible}
      assert actual == set(menus), f"unexpected visible panels: {actual}, expected {menus}"
      for parent, child in zip(menus, menus[1:]):
        owner, trigger, selection = self.anchors[child]
        assert owner == parent and self.pointer(trigger + 0x10) == parent
        assert self.number(parent + 0xA0) == selection, "keep the parent's selected row"
        assert self.number(parent + 0xC0) > 0, "visible parent panels must stay suspended"
      interactive = [menu for menu in menus if not self.pointer(menu + 0x38) and not self.number(menu + 0xC0)]
      assert interactive == [menus[-1]], "only the deepest visible panel receives input"

    def chain(menu):
      result = [menu]
      while menu in self.anchors:
        menu = self.anchors[menu][0]
        result.append(menu)
      return result

    def sidebars_visible(menu):
      shown(*reversed(chain(menu)))

    def cancel_result(item):
      result = call(method(item, 19), item)
      assert result & 1, "B must close the current submenu"
      assert not result & 0x3E, "B must not close/delete ancestors or the complete menu chain"
      assert result & 512, "B must retain the native cancel sound"
      return result

    def back(item):
      menu = self.pointer(item + 0x10)
      parent = self.pointer(menu + 0x30)
      ancestor = self.pointer(parent + 0x30)
      cancel_result(item)
      before = len(self.events)
      selection = self.number(parent + 0xA0)
      method_info = self.pointer(self.pointer(menu) + 0x138 + 10 * 16 + 8)
      call(method(menu, 10), menu, method_info)
      sidebars_visible(parent)
      assert self.pointer(parent + 0x38) == 0
      if ancestor:
        assert self.pointer(ancestor + 0x38) == parent, "B must leave the previous menu attached"
      expected = [("dispose", menu)]
      if menu not in self.anchors:
        expected += [("open", parent), ("open", ancestor), ("footer", self.title)]
      assert self.events[before:] == expected, "only destination pages restore hidden ancestors"
      assert self.number(parent + 0xA0) == selection

    call(hook, self.parent)
    entry = self.values(self.full_list)[-1]
    assert self.read_string(call(method(entry, 4), entry)) == "MID_MENU_TITLE_HUB"
    assert self.read_string(call(method(entry, 28), entry)) == "", "do not repeat the entry title as inline help"
    count = len(self.values(self.full_list))
    assert self.number(self.menu + 0xB8) == 9, "only collection and one hub entry extend the outdoor menu"
    call(hook, self.parent)
    assert len(self.values(self.full_list)) == count, "reopening must not duplicate the hub entry"
    self.qword(entry + 0x10, self.menu)
    self.integer(self.menu + 0xA0, len(self.values(self.full_list)) - 1)
    assert call(method(entry, 18), entry) == 128
    categories = self.pointer(self.menu + 0x38)
    sidebars_visible(categories)
    assert self.anchors[categories][1] == entry
    entries = self.values(self.pointer(categories + 0x80))
    assert len(entries) == self.number(categories + 0xB8) == 3
    assert [self.read_string(call(method(item, 4), item)) for item in entries] == ["设施", "活动", "商店"]
    for item in entries:
      self.qword(item + 0x10, categories)
      cancel_result(item)
      assert self.read_string(call(method(item, 28), item)) == ""
    names = re.findall(r'Facility\s*\{\s*name: "([^"]+)"', profile)
    groups = [[int(value) for value in values.split(",")] for values in re.findall(
      r'MenuGroup\s*\{\s*facilities: &\[([^\]]+)\]', profile)]
    assert groups == [[0, 1, 2, 3, 9, 10], [11, 12, 13, 14, 15, 16], [4, 5, 6, 7, 8]]
    choices = [None] * len(binds)
    original = bytes(self.cpu.mem_read(self.sequence_class, 0x138 + 32 * 16))
    for category, indices in zip(entries, groups):
      self.integer(categories + 0xA0, entries.index(category))
      assert call(method(category, 18), category) == 128
      quick = self.pointer(categories + 0x38)
      content = self.pointer(quick + 0x70)
      assert self.class_name(self.pointer(content)) == b"HubMenuContent", "destinations use the original full-page hub prefab"
      sidebars_visible(quick)
      assert quick not in self.anchors, "a full-page destination must never use side-menu coordinates"
      assert quick and self.pointer(quick + 0x30) == categories
      items = self.values(self.pointer(quick + 0x80))
      assert len(items) == len(indices) == self.number(quick + 0xB8)
      assert len(items) <= len(self.row_nodes), "every item must fit without cycling prefab rows"
      assert [self.read_string(call(method(item, 4), item)) for item in items] == [names[i] for i in indices]
      for item in [entry, category] + items:
        assert self.class_name(self.pointer(item)) == b"HubMenuItem"
        for slot, offset in ((12, 0x02ADF650), (13, 0x02ADF760)):
          assert method(item, slot) == self.game + offset, "native hub selection/help lifecycle must survive cloning"
      for index, item in zip(indices, items):
        choices[index] = item
        self.qword(item + 0x10, quick)
        cancel_result(item)
        assert self.read_string(call(method(item, 28), item)) == "", "facility names must not be repeated as inline help"
        assert call(method(item, 8), item) == 1
        self.integer(quick + 0xA0, items.index(item))
        assert call(method(item, 18), item) == 128
        child = self.pointer(quick + 0x38)
        shown(child)
        expected = [("bind", self.game + binds[index])] + [("close", menu) for menu in chain(quick)]
        assert self.events[-len(expected):] == expected
        if index == 1:
          self.ring_attribute = 1
          cleaner_class = self.alloc(0x138 + 30 * 16)
          self.qword(cleaner_class + 0x10, self.c_string("RingCleaningItem"))
          self.write(cleaner_class + 0x126, struct.pack("<H", 30))
          vtable(cleaner_class, 8, "RING_ATTRIBUTE")
          original_cleaner_class = bytes(self.cpu.mem_read(cleaner_class, 0x138 + 30 * 16))
          cleaner, ordinary, other = self.alloc(), self.alloc(), self.alloc()
          other_class = self.alloc(0x138 + 30 * 16)
          self.write(other_class, original_cleaner_class)
          self.qword(other_class + 0x10, self.c_string("SkillInheritanceItem"))
          self.qword(cleaner, cleaner_class)
          self.qword(ordinary, cleaner_class)
          self.qword(other, other_class)
          top = self.alloc()
          self.qword(top + 0x80, self.list([cleaner, other]))
          self.qword(child + 0x38, top)
          call(ring_hook, child, 0, 0)
          assert call(method(cleaner, 8), cleaner) == 4, "hide polishing only in the outdoor shortcut"
          assert self.pointer(other) == other_class and call(method(other, 8), other) == 1
          assert bytes(self.cpu.mem_read(cleaner_class, len(original_cleaner_class))) == original_cleaner_class
          # Native room entry remains unmodified, even after visiting the shortcut.
          room_sequence, room_menu = self.alloc(), self.alloc()
          self.qword(room_sequence + 0x30, self.hub)
          self.qword(room_sequence + 0x38, room_menu)
          self.qword(room_menu + 0x80, self.list([ordinary]))
          for native_attribute in (1, 2):
            self.ring_attribute = native_attribute
            call(ring_hook, room_sequence, 0, 0)
            assert self.pointer(ordinary) == cleaner_class
            assert call(method(ordinary, 8), ordinary) == native_attribute
          self.ring_attribute = 1
          self.qword(child + 0x38, 0)
        method_info = self.pointer(self.pointer(child) + 0x138 + 10 * 16 + 8)
        call(method(child, 10), child, method_info)
        sidebars_visible(quick)
        expected = [("dispose", child)] + [("open", menu) for menu in chain(quick)] + [("footer", self.title)]
        assert self.events[-len(expected):] == expected
        assert self.pointer(quick + 0x38) == 0
      back(items[-1])
      assert self.pointer(categories + 0x38) == 0
      assert self.pointer(self.menu + 0x38) == categories
    assert all(choices) and set(self.item_ctors) == {"HUB_ITEM_CTOR"}
    assert bytes(self.cpu.mem_read(self.sequence_class, len(original))) == original
    self.activity_used = 1
    self.fishing_remaining = 0
    for index in (13, 14, 15):
      assert call(method(choices[index], 8), choices[index]) == 2
      assert call(method(choices[index], 18), choices[index]) == 2048
    self.activity_used = 0
    self.fishing_remaining = 3
    self.unlocked = False
    assert call(method(choices[1], 8), choices[1]) == 4
    assert call(method(choices[1], 18), choices[1]) == 2048
    self.unlocked = True
    self.locked_facilities.add(self.facility_data["AID_紋章士の間"])
    assert call(method(choices[1], 8), choices[1]) == 4
    assert call(method(choices[4], 8), choices[4]) == 1
    self.locked_facilities.clear()
    well_data = self.facility_data.pop("AID_不思議な井戸")
    assert call(method(choices[2], 8), choices[2]) == 4
    self.facility_data["AID_不思議な井戸"] = well_data
    self.chapter_cleared = False
    assert call(method(choices[12], 8), choices[12]) == 4
    assert call(method(choices[1], 8), choices[1]) == 1
    self.chapter_cleared = True
    self.integer(self.user + 0x28, 5)
    assert call(method(entry, 18), entry) == 2048
    self.integer(self.user + 0x28, 4)
    back(entries[-1])
    assert self.pointer(self.menu + 0x38) == 0
    # Reopening after visiting every group must recreate only the three
    # categories, with no direct destination or shopping entry outdoors.
    assert call(method(entry, 18), entry) == 128
    reopened = self.pointer(self.menu + 0x38)
    sidebars_visible(reopened)
    reopened_entries = self.values(self.pointer(reopened + 0x80))
    assert len(reopened_entries) == 3
    for item in reopened_entries:
      self.qword(item + 0x10, reopened)
    back(reopened_entries[0])
    assert self.pointer(self.menu + 0x38) == 0
    self.active = False
