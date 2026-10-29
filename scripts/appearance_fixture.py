"""Run compiled battle/preparation hooks with modeled native calls, not Unity."""

import struct
from unicorn import arm64_const as registers
from verify_outfits import catalog


class AppearanceFixture:
  def __init__(self, cpu, game, write, offset, replacements, stop, stack):
    self.active = False
    self.cpu, self.write, self.stop, self.stack = cpu, write, stop, stack
    self.get_hook = replacements[offset("GET_FROM_UNIT")]
    self.appearance_hook = replacements[offset("APPEARANCE")]
    self.info_hook = replacements[offset("UNIT_INFO")]
    self.addresses = {game + offset(name): name for name in (
      "APPEARANCE", "GET_FROM_UNIT", "UNIT_INFO", "GET_STATE", "GET_PID",
      "DRESS_GENDER", "STRING_NEW_SIZE", "SET_DRESS")}
    self.root = game + 0x8700000
    self.status, self.unit, self.result = (self.root + i * 0x1000 for i in range(3))
    self.heap = self.root + 0x10000
    self.tls = {}
    self.sets, self.original_queries = 0, 0
    self.state, self.conditions, self.fail_allocation = 0, 0, False
    self.bridge = self.root + 0x8000
    words = [0xA9BF7BFD, 0xAA0203E3, 0xF9401401, 0x52800040, 0xAA1F03E2,
      0xAA1F03E4, 0x58000090, 0xD63F0200, 0xA8C17BFD, 0xD65F03C0]
    self.write(self.bridge, struct.pack("<10IQ", *words, self.get_hook))
    self.write(self.status, bytes(0x100))
    self.write(self.unit, bytes([0xA5]) * 0x400)
    self.qword(self.status + 0x28, self.unit)
    # An inaccessible accessory list makes the old Somniel path fail the test.
    self.qword(self.unit + 0xC8, 0xBAD)
    self.qword(self.unit + 0x48, self.string("JID_ウォーリアー"))

  def qword(self, address, value):
    self.write(address, struct.pack("<Q", value))

  def allocate(self, size):
    result = self.heap
    self.heap += max(16, (size + 15) & ~15)
    assert self.heap < self.root + 0x200000
    self.write(result, bytes(max(size, 16)))
    return result

  def string(self, value):
    encoded = value.encode("utf-16-le")
    obj = self.allocate(0x16 + len(encoded))
    self.write(obj + 0x10, struct.pack("<i", len(encoded) // 2) + encoded + bytes(2))
    return obj

  def string_at(self, obj):
    length, = struct.unpack("<i", self.cpu.mem_read(obj + 0x10, 4))
    return bytes(self.cpu.mem_read(obj + 0x14, length * 2)).decode("utf-16-le")

  def prepare(self, outfit, rig=None):
    self.pid = self.string(outfit["pid"])
    self.gender, self.expected_dress = outfit["gender"], outfit["dress"]
    self.write(self.result, bytes([0x5A]) * 0x200)
    self.qword(self.result + 0x20, self.string(rig or outfit["rig"]))
    self.qword(self.result + 0x28, self.string("uBody_TestCurrentClass"))
    self.state, self.conditions, self.fail_allocation = 0, 0, False

  def return_value(self, value=0):
    self.cpu.reg_write(registers.UC_ARM64_REG_X0, value)
    self.cpu.reg_write(registers.UC_ARM64_REG_PC, self.cpu.reg_read(registers.UC_ARM64_REG_X30))

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.addresses.get(pc, import_name)
    if name is None:
      return False
    args = [self.cpu.reg_read(getattr(registers, f"UC_ARM64_REG_X{i}")) for i in range(3)]
    if name == "APPEARANCE":
      assert args[0] == self.status
      self.cpu.reg_write(registers.UC_ARM64_REG_PC, self.bridge)
      return True
    if name in ("GET_FROM_UNIT", "UNIT_INFO"):
      assert args[1 if name == "GET_FROM_UNIT" else 0] == self.unit
      self.original_queries += 1
      self.return_value(self.result)
    elif name == "GET_STATE":
      assert args[0] == self.unit
      self.return_value(self.state)
    elif name == "GET_PID":
      assert args[0] == self.unit
      self.return_value(self.pid)
    elif name == "DRESS_GENDER":
      assert args[0] == self.unit
      self.return_value(self.gender)
    elif name == "STRING_NEW_SIZE":
      if self.fail_allocation:
        self.return_value()
      else:
        obj = self.allocate(0x16 + args[0] * 2)
        self.write(obj + 0x10, struct.pack("<i", args[0]))
        self.return_value(obj)
    elif name == "SET_DRESS":
      assert args[0] == self.result
      assert self.string_at(args[1]) == self.expected_dress
      self.qword(self.result + 0x28, args[1])
      self.sets += 1
      self.return_value()
    elif name in ("malloc", "calloc"):
      self.return_value(self.allocate(args[0] * args[1] if name == "calloc" else args[0]))
    elif name == "posix_memalign":
      self.qword(args[0], self.allocate(args[2]))
      self.return_value()
    elif name == "free":
      self.return_value()
    elif name in ("memcpy", "memmove"):
      self.write(args[0], bytes(self.cpu.mem_read(args[1], args[2])))
      self.return_value(args[0])
    else:
      return False
    return True

  def call(self, entry="battle", changes=True):
    if entry == "battle":
      target, args = self.appearance_hook, [self.status, 1, self.conditions, 0]
    elif entry == "info":
      target, args = self.info_hook, [self.unit, 0, 0]
    else:
      target, args = self.get_hook, [2, self.unit, 0, self.conditions, 0]
    before = bytes(self.cpu.mem_read(self.result, 0x200))
    unit_before = bytes(self.cpu.mem_read(self.unit, 0x400))
    sets, queries = self.sets, self.original_queries
    self.cpu.reg_write(registers.UC_ARM64_REG_SP, self.stack)
    for index, value in enumerate(args):
      self.cpu.reg_write(getattr(registers, f"UC_ARM64_REG_X{index}"), value)
    self.cpu.reg_write(registers.UC_ARM64_REG_X30, self.stop)
    self.cpu.emu_start(target, self.stop + 4, count=1_000_000)
    assert self.cpu.reg_read(registers.UC_ARM64_REG_PC) == self.stop
    assert self.cpu.reg_read(registers.UC_ARM64_REG_X0) == self.result
    assert self.original_queries == queries + 1
    after = bytes(self.cpu.mem_read(self.result, 0x200))
    if changes:
      assert self.sets == sets + 1
      assert after[:0x28] == before[:0x28] and after[0x30:] == before[0x30:]
    else:
      assert after == before and self.sets == sets
    assert bytes(self.cpu.mem_read(self.unit, 0x400)) == unit_before

  def exercise(self):
    self.active = True
    outfits = catalog()
    for outfit in outfits:
      for entry in ("battle", "info"):
        self.prepare(outfit)
        self.call(entry)
        self.call("outside", changes=False)
    panette = next(o for o in outfits if o["pid"] == "PID_パネトネ")
    for entry in ("battle", "info"):
      for case in ("engaged", "unknown", "wrong_gender", "incompatible_rig", "allocation_failure", "already_original"):
        self.prepare(panette, "uRig_Tik1AT" if case == "incompatible_rig" else None)
        if case == "engaged":
          self.state = 1
        elif case == "unknown":
          self.pid = self.string("PID_UnknownEnemy")
        elif case == "wrong_gender":
          self.gender = 1
        elif case == "allocation_failure":
          self.fail_allocation = True
        elif case == "already_original":
          self.qword(self.result + 0x28, self.string(panette["dress"]))
        self.call(entry, changes=False)
    for condition in ("会話", "デモ", "私服", "エンゲージ技"):
      self.prepare(panette)
      self.conditions = self.allocate(0x28)
      self.qword(self.conditions + 0x18, 1)
      self.qword(self.conditions + 0x20, self.string(condition))
      self.call(changes=False)
    print("42 original dresses: compiled battle + preparation paths; only dress changed; unit unchanged; fallback cases passed")

    self.active = False

  def exercise_disabled(self):
    self.active = True
    self.prepare(next(o for o in catalog() if o["pid"] == "PID_パネトネ"))
    self.call("battle", changes=False)
    self.call("info", changes=False)
    self.active = False
