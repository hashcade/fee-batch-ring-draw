"""Exercise the compiled settings callback and SDK file transaction boundaries."""

import struct
from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class SettingsFixture(CollectorFixture):
  VTABLE_BYTES = 0x138 + 28 * 16

  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.heap = game + 0x8100000

  def path(self, address):
    value = bytearray()
    for index in range(128):
      byte = self.cpu.mem_read(address + index, 1)[0]
      if not byte: return bytes(value)
      value.append(byte)
    raise AssertionError("unterminated path")

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(8)]
    x0, x1 = args[:2]
    result = None
    if name == "il2cpp_assembly_get_image":
      assert self.path(x0) in (b"Assembly-CSharp.dll", b"Assembly-CSharp")
      # Cobalt 1.31's scanned 0x42918C entry returns Il2CppAssembly*,
      # despite its export name. The image is the assembly's first field.
      result = self.assembly
    elif name == "il2cpp_class_from_name":
      assert x0 == self.image and self.path(x1) == b"" and self.path(args[2]) == b"ConfigBasicMenuItem"
      result = self.item_class
    elif name == "il2cpp_class_init": assert x0 == self.item_class
    elif name == "ALLOCATE_OBJECT":
      assert x0 == self.item_class
      result = self.alloc()
      self.qword(result, x0)
    elif name == "ITEM_CTOR": pass
    elif name in ("BASE_ON_SELECT", "BASE_ON_DESELECT"): pass
    elif name == "STRING_NEW_SIZE":
      result = self.alloc(0x20 + x0 * 2)
      self.integer(result + 0x10, x0)
    elif name == "WRITE_BARRIER": assert self.pointer(x0) == x1
    elif name == "GET_LANGUAGE": result = 10
    elif name == "CHANGE_NUMBER":
      assert args[1:4] == [2, 1000, 10]
      result = self.next_count
    elif name == "UPDATE_TEXT": self.refreshes += 1
    elif name == "_ZN2nn2fs11MountSdCardEPKc":
      assert self.path(x0) == b"feesomniel"
      result = int(self.failure == "mount")
      if result == 0: self.mounts += 1
    elif name == "_ZN2nn2fs7UnmountEPKc":
      assert self.path(x0) == b"feesomniel"
      self.unmounts += 1
    elif name == "_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci":
      path = self.path(x1)
      assert path in self.paths
      result = 0x202 if path not in self.files else 0
      if result == 0:
        self.next_handle += 1
        self.handles[self.next_handle] = path
        self.cpu.mem_write(x0, struct.pack("<Q", self.next_handle))
    elif name == "_ZN2nn2fs11GetFileSizeEPlNS0_10FileHandleE":
      self.cpu.mem_write(x0, struct.pack("<Q", len(self.files[self.handles[x1]])))
      result = 0
    elif name == "_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm":
      data = self.files[self.handles[x1]]
      assert args[2] == 0 and args[4] == len(data)
      if data: self.write(args[3], data)
      self.cpu.mem_write(x0, struct.pack("<Q", len(data)))
      result = 0
    elif name == "_ZN2nn2fs9CloseFileENS0_10FileHandleE": self.handles.pop(x0)
    elif name == "_ZN2nn2fs10CreateFileEPKcl":
      path = self.path(x0)
      assert path == self.paths[1] and path not in self.files
      self.files[path] = bytes(x1)
      result = 0
    elif name == "_ZN2nn2fs9WriteFileENS0_10FileHandleElPKvmRKNS0_11WriteOptionE":
      assert self.handles[x0] == self.paths[1] and x1 == 0 and self.number(args[4]) == 1
      result = int(self.failure == "write")
      if result == 0: self.files[self.handles[x0]] = bytes(self.cpu.mem_read(args[2], args[3]))
    elif name == "_ZN2nn2fs10DeleteFileEPKc":
      path = self.path(x0)
      assert path in self.paths
      result = 0 if self.files.pop(path, None) is not None else 0x202
    elif name == "_ZN2nn2fs10RenameFileEPKcS2_":
      source, target = self.path(x0), self.path(x1)
      assert source in self.paths and target in self.paths
      if self.failure == "rename" and source == self.paths[1]: result = 1
      else:
        assert source in self.files and target not in self.files
        self.files[target] = self.files.pop(source)
        result = 0
    elif name == "getRegionAddress": result = self.game
    elif name in ("malloc", "calloc"):
      result = self.alloc(max(32, x0 * (x1 if name == "calloc" else 1)))
    elif name == "free": pass
    elif name in ("memcpy", "memmove"):
      if args[2]: self.write(x0, bytes(self.cpu.mem_read(x1, args[2])))
      result = x0
    elif name == "memset":
      self.write(x0, bytes([x1 & 255]) * args[2])
      result = x0
    else: return super().handle(pc, import_name)
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def exercise(self, callbacks, stop, stack):
    self.active = True
    self.image, self.item_class = self.alloc(), self.alloc(self.VTABLE_BYTES)
    self.assembly = self.alloc()
    self.qword(self.assembly, self.image)
    self.integer(self.item_class + 0xF8, 0x88)
    self.write(self.item_class + 0x126, struct.pack("<H", 28))
    # ConfigBasicMenuItem adds InitContent to BasicMenuItem's 27 slots.
    for slot, name in ((8, "BuildAttribute"), (18, "ACall"), (19, "BCall"), (26, "CustomCall"), (27, "InitContent")):
      method = self.alloc()
      self.qword(method + 0x10, self.c_string(name))
      self.qword(self.item_class + 0x138 + slot * 16, self.game + 0x3E00000 + slot * 16)
      self.qword(self.item_class + 0x138 + slot * 16 + 8, method)
    # Execute the pinned ConfigBasicMenuItem selection routines themselves.
    # Only their base colour/cursor callbacks and UpdateText are modeled.
    for slot, offset, words in (
      (12, 0x02536FD0, (0xA9BE7BFD, 0xF9000BF3, 0x910003FD, 0xAA1F03E1,
        0xAA0003F3, 0x97FCBD63, 0x52800028, 0x39020268,
        0xAA1303E0, 0xF9400BF3, 0xA8C27BFD, 0x14000001)),
      (13, 0x025370F0, (0xA9BE7BFD, 0xF9000BF3, 0x910003FD, 0xAA1F03E1,
        0xAA0003F3, 0x97FCBD73, 0x3902027F, 0xAA1303E0,
        0xF9400BF3, 0xA8C27BFD, 0x17FFFFBA, 0xD503201F)),
    ):
      self.write(self.game + offset, struct.pack("<12I", *words))
      self.qword(self.item_class + 0x138 + slot * 16, self.game + offset)
    self.functions[self.game + 0x02466570] = "BASE_ON_SELECT"
    self.functions[self.game + 0x024666D0] = "BASE_ON_DESELECT"
    for offset in (0x02466570, 0x024666D0):
      self.write(self.game + offset, struct.pack("<I", 0xD65F03C0))
    original = bytes(self.cpu.mem_read(self.item_class, self.VTABLE_BYTES))
    self.paths = tuple(b"feesomniel:/fee-better-somniel.cfg" + tail for tail in (b"", b".tmp", b".previous"))
    self.files = {self.paths[0]: b"draw_count=300\n"}
    self.handles, self.next_handle = {}, 100
    self.mounts, self.unmounts, self.refreshes, self.failure = 0, 0, 0, ""
    self.user = 0
    self.metadata(0x062840F0)

    def call(address, *args):
      for index in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
      self.cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      try:
        self.cpu.emu_start(address, stop + 4, count=500_000)
      except Exception as error:
        pc = self.cpu.reg_read(reg.UC_ARM64_REG_PC)
        raise AssertionError(f"settings call {address:#x}, PC {pc:#x}, X0 {self.cpu.reg_read(reg.UC_ARM64_REG_X0):#x}") from error
      assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == stop
      return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

    self.invoke = call

    order = [3, 4, 5, 6, 7, 0, 8, 9, 11, 12, 13, 10, 1, 2]
    assert len(callbacks) == 14
    self.factories = [callbacks[order.index(kind)] for kind in range(14)]
    items = [call(callback) for callback in callbacks]
    assert [self.read_string(self.pointer(item + 0x68)) for item in items] == [
      "自动完成支援对话", "自动研磨", "研磨角色", "一般锻炼次数",
      "钓鱼次数不减", "牵绊戒指精炼次数", "商店库存无限", "礼物/食材不消耗", "快速钓鱼",
      "肌肉体操快速满分", "飞龙射手快速SSS", "重复料理", "料理厨师", "料理结果"]
    for kind, item in zip(order, items):
      hint = ""
      assert self.read_string(self.pointer(item + 0x78)) == hint, "retained preferences have no footer descriptions"
      copy = self.alloc(self.VTABLE_BYTES)
      self.write(copy, bytes(self.cpu.mem_read(self.pointer(item), self.VTABLE_BYTES)))
      self.qword(item, copy)
      for slot in (12, 13, 19, 27):
        start = 0x138 + slot * 16
        assert bytes(self.cpu.mem_read(copy + start, 16)) == original[start:start + 16]
      assert self.cpu.mem_read(item + 0x81, 1)[0] == 0
    def arrows():
      return [self.cpu.mem_read(item + 0x80, 1)[0] for item in items]
    assert not any(arrows())
    # Invoke the pinned native callbacks across all fourteen rows, including
    # first visits beyond the initial viewport, reverse traversal and wraps.
    saved = dict(self.files)
    for index in [*range(14), *range(13, -1, -1), 13, 0]:
      item = items[index]
      cls = self.pointer(item)
      call(self.pointer(cls + 0x138 + 12 * 16), item)
      assert arrows() == [int(i == index) for i in range(14)]
      call(self.pointer(cls + 0x138 + 13 * 16), item)
      assert not any(arrows())
    assert self.files == saved
    reopened = [call(callback) for callback in callbacks]
    assert not any(self.cpu.mem_read(item + 0x80, 1)[0] for item in reopened)
    callback = self.factories[0]
    item = call(callback)
    assert self.read_string(self.pointer(item + 0x68)) == "牵绊戒指精炼次数"
    assert self.read_string(self.pointer(item + 0x70)) == "300"
    assert self.pointer(item) != self.item_class
    # PluginSubmenu clones the returned class once more before menu binding.
    private = self.pointer(item)
    cobalt_class = self.alloc(len(original))
    self.write(cobalt_class, bytes(self.cpu.mem_read(private, len(original))))
    self.qword(item, cobalt_class)
    assert bytes(self.cpu.mem_read(cobalt_class + 0x138 + 27 * 16, 16)) == original[-16:], "native InitContent must survive both class clones"
    assert call(self.pointer(cobalt_class + 0x138 + 8 * 16), item, 0) == 1
    assert call(self.pointer(cobalt_class + 0x138 + 18 * 16), item, 0) == 0
    change = self.pointer(cobalt_class + 0x138 + 26 * 16)
    for count in (310, 300, 10, 2, 1000):
      self.next_count = count
      assert call(change, item, 0) == 4096
      assert self.files == {self.paths[0]: f"draw_count={count}\n".encode()}
      assert self.read_string(self.pointer(item + 0x70)) == str(count)
      reopened = call(callback)
      assert self.read_string(self.pointer(reopened + 0x70)) == str(count)
    for failure in ("mount", "write", "rename"):
      self.failure, self.next_count = failure, 600
      assert call(change, item, 0) == 2048
      assert self.files[self.paths[0]] == b"draw_count=1000\n"
      assert self.read_string(self.pointer(item + 0x70)) == "1000"
      assert self.read_string(self.pointer(item + 0x78)) == ""
    self.failure, self.next_count = "", 300
    assert call(change, item, 0) == 4096
    assert self.files == {self.paths[0]: b"draw_count=300\n"}
    mounts = self.mounts
    assert call(change, item, 0) == 0 and self.mounts == mounts
    assert self.mounts == self.unmounts and not self.handles
    assert bytes(self.cpu.mem_read(self.item_class, len(original))) == original
    self.active = False
