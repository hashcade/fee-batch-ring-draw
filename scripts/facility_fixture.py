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
    elif name == "MENU_CTOR": self.qword(a + 0x80, b)
    elif name == "SET_MENU_ROWS": self.integer(a + 0xB8, b)
    elif name == "CONTENT": result = self.alloc()
    elif name == "CREATE_DESC": result = self.alloc()
    elif name == "BIND":
      self.qword(a + 0x30, b)
      self.qword(b + 0x38, a)
    elif name == "BIND_PARENT": pass
    elif name == "EMPTY_UNIT_LIST": result = self.ready_units if self.ready_units else self.list()
    elif name == "RELATION": result = self.relation
    elif name == "CAN_ADVANCE": result = 1
    elif name in ("HUB_INIT", "HUB_TICK"): self.events.append((name, a))
    elif name == "HUB_IS_MAIN": result = self.is_main
    elif name == "VARIABLE_NUMBER": result = self.activity_used
    elif name == "FISHING_USES": result = 3 if not self.activity_used else 0
    elif name == "IS_BOND": result = self.mode[0]
    elif name == "READY_ONLY": result = self.mode[1]
    elif name == "SET_BOND": self.mode[0] = bool(a)
    elif name == "SET_READY_ONLY": self.mode[1] = bool(a)
    elif name == "CREATE_SEQUENCE":
      child = self.alloc()
      self.qword(child, self.sequence_class)
      self.qword(child + 0x30, a)
      self.qword(a + 0x38, child)
      self.events.append(("supports", a))
    elif name == "RING_MENU": pass
    elif name == "ENABLED": result = 1
    elif name == "IS_COMPLETE": result = int(self.unlocked)
    elif name == "FACILITY_BIND":
      self.events.append(("bind", pc))
      if pc in (self.game + 0x028A8700, self.game + 0x02174D70, self.game + 0x02AD6C40): assert b == 0
      if pc == self.game + 0x022BC190: assert self.read_string(b) == "AID_水遊び"
      child = self.alloc()
      self.qword(child, self.sequence_class)
      self.qword(child + 0x30, a)
      self.qword(a + 0x38, child)
      result = child
    elif name == "DISPOSE":
      self.events.append(("dispose", a))
      self.qword(self.pointer(a + 0x30) + 0x38, 0)
    elif name == "OPEN": self.events.append(("open", a))
    elif name == "CLOSE": self.events.append(("close", a))
    elif name == "SHOW_FOOTER":
      assert a == self.title
      self.events.append(("footer", a))
    elif name == "il2cpp_assembly_get_image": result = self.image
    elif name == "il2cpp_class_from_name":
      assert self.c_read(b) == b"App" and self.c_read(args[2]) == b"TitleBar"
      result = self.title_class
    elif name == "il2cpp_class_init": assert a == self.title_class
    else: return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def c_read(self, pointer):
    return bytes(self.cpu.mem_read(pointer, 128)).split(b"\0", 1)[0]

  def exercise(self, hook, ring_hook, stop, stack, profile, support_hooks=None):
    self.active = True
    self.reset(4)
    self.ready_units = 0
    self.unlocked = True
    self.activity_used, self.is_main = 0, True
    self.qword(self.user + 0x50, self.alloc())
    self.image, self.title, self.title_class, fields = (self.alloc() for _ in range(4))
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

    self.qword(self.menu_class + 0x10, self.c_string("BasicMenu"))
    self.write(self.menu_class + 0x126, struct.pack("<H", 64))
    for slot, name in ((10, "DISPOSE"), (18, "CREATE_DESC"), (21, "OPEN"), (23, "CLOSE")):
      vtable(self.menu_class, slot, name)
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
    call(hook, self.parent)
    entry = self.values(self.full_list)[-1]
    self.qword(entry + 0x10, self.menu)
    assert call(method(entry, 18), entry) == 128
    quick = self.pointer(self.menu + 0x38)
    assert quick and self.pointer(quick + 0x30) == self.menu
    choices = self.values(self.pointer(quick + 0x80))
    assert len(choices) == len(binds)
    original = bytes(self.cpu.mem_read(self.sequence_class, 0x138 + 32 * 16))
    for index, item in enumerate(choices):
      self.qword(item + 0x10, quick)
      assert call(method(item, 8), item) == 1
      assert call(method(item, 18), item) == 128
      child = self.pointer(quick + 0x38)
      assert self.events[-2:] == [("bind", self.game + binds[index]), ("close", quick)]
      if index == 1:
        cleaner, ordinary = self.alloc(), self.alloc()
        cleaner_class = self.alloc(0x138 + 30 * 16)
        self.qword(cleaner_class + 0x10, self.c_string("RingCleaningItem"))
        self.write(cleaner_class + 0x126, struct.pack("<H", 30))
        vtable(cleaner_class, 8, "ENABLED")
        self.qword(cleaner, cleaner_class)
        self.qword(ordinary, cleaner_class)
        top = self.alloc()
        self.qword(top + 0x80, self.list([cleaner]))
        self.qword(child + 0x38, top)
        call(ring_hook, child, 0, 0)
        assert call(method(cleaner, 8), cleaner) == 4
        assert call(method(ordinary, 8), ordinary) == 1, "original ring cleaning must remain available"
      method_info = self.pointer(self.pointer(child) + 0x138 + 10 * 16 + 8)
      call(method(child, 10), child, method_info)
      assert self.events[-3:] == [("dispose", child), ("open", quick), ("footer", self.title)]
      assert self.pointer(quick + 0x38) == 0
    assert bytes(self.cpu.mem_read(self.sequence_class, len(original))) == original
    self.activity_used = 1
    for index in (13, 14, 15):
      assert call(method(choices[index], 8), choices[index]) == 2
      assert call(method(choices[index], 18), choices[index]) == 2048
    self.activity_used = 0
    self.unlocked = False
    assert call(method(choices[1], 8), choices[1]) == 4
    assert call(method(choices[1], 18), choices[1]) == 2048
    self.integer(self.user + 0x28, 5)
    assert call(method(entry, 18), entry) == 2048
    self.integer(self.user + 0x28, 4)
    method_info = self.pointer(self.pointer(quick) + 0x138 + 10 * 16 + 8)
    call(method(quick, 10), quick, method_info)
    assert self.events[-3:] == [("dispose", quick), ("open", self.menu), ("footer", self.title)]
    self.ready_units = self.list([self.alloc(), self.alloc()])
    self.relation = self.alloc()
    self.integer(self.relation + 0x18, 1)
    self.mode = [True, False]
    before = bytes(self.cpu.mem_read(self.relation, 0x40))
    call(hook, self.parent)
    support = self.pointer(self.menu + 0x38)
    assert support and self.mode == [False, True]
    assert self.events[-2:] == [("supports", self.menu), ("close", self.menu)]
    method_info = self.pointer(self.pointer(support) + 0x138 + 10 * 16 + 8)
    call(method(support, 10), support, method_info)
    assert self.mode == [True, False] and self.pointer(self.menu + 0x38) == 0
    assert bytes(self.cpu.mem_read(self.relation, 0x40)) == before
    assert self.events[-3:] == [("dispose", support), ("open", self.menu), ("footer", self.title)]
    if support_hooks:
      hub = self.alloc()
      for sequence in (4, 5):
        self.integer(self.user + 0x28, sequence)
        call(support_hooks["HUB_INIT"], hub)
        self.is_main = False
        call(support_hooks["HUB_TICK"], hub)
        assert self.pointer(hub + 0x38) == 0
        self.is_main = True
        busy = self.alloc()
        self.qword(hub + 0x38, busy)
        call(support_hooks["HUB_TICK"], hub)
        assert self.pointer(hub + 0x38) == busy
        self.qword(hub + 0x38, 0)
        call(support_hooks["HUB_TICK"], hub)
        child = self.pointer(hub + 0x38)
        assert child and self.mode == [False, True]
        method_info = self.pointer(self.pointer(child) + 0x138 + 10 * 16 + 8)
        before_events = len(self.events)
        call(method(child, 10), child, method_info)
        assert self.events[before_events:] == [("dispose", child)], "HubSequence is not a BasicMenu"
        assert self.mode == [True, False]
        call(support_hooks["HUB_TICK"], hub)
        assert self.pointer(hub + 0x38) == 0, "scan only once per scene visit"
    self.active = False
