"""Run compiled shop hooks against modeled stock and native settlement boundaries."""

import re
import struct
from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class ShopFixture(CollectorFixture):
  def __init__(self, *args):
    super().__init__(*args)
    self.heap = self.game + 0xA800000

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(5)]
    if name == "CONTENTS":
      self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.driver)
      return True
    if name in ("WEAPON_KEY", "ITEM_KEY", "FLEA_KEY"):
      result = self.key
    elif name == "LOOKUP":
      self.integer(args[2], self.count)
      self.integer(args[2] + 4, 17)
      self.qword(args[2] + 8, 0)
      self.reads += 1
      result = int(self.count != -999)
    elif name in ("STOCK", "INFINITY", "PURCHASE"):
      self.calls.append(name)
      result = 7 if name == "STOCK" else 0
    else:
      return super().handle(pc, import_name)
    self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def run(self, address, args):
    for index in range(8):
      self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
    self.cpu.reg_write(reg.UC_ARM64_REG_SP, self.stack)
    self.cpu.reg_write(reg.UC_ARM64_REG_X30, self.stop)
    self.cpu.emu_start(address, self.stop + 4, count=100_000)
    assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == self.stop
    return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

  def build_driver(self, key_hook, lookup_hook, other_key=False):
    words = [0xA9BF7BFD]  # Preserve the list hook's original return address.
    def load(register, value):
      words.append(0xD2800000 | ((value & 0xFFFF) << 5) | register)
      for shift in (1, 2, 3):
        words.append(0xF2800000 | (shift << 21) | (((value >> (shift * 16)) & 0xFFFF) << 5) | register)
    load(0, self.key)
    load(1, 0)
    load(16, key_hook)
    words.append(0xD63F0200)
    load(0, self.dictionary)
    load(1, self.key + (0x20 if other_key else 0))
    load(2, self.value)
    load(3, 0)
    load(16, lookup_hook)
    words.extend((0xD63F0200, 0xA8C17BFD, 0xD65F03C0))
    self.driver = self.alloc(len(words) * 4)
    self.write(self.driver, struct.pack(f"<{len(words)}I", *words))

  def exercise(self, replacements, stop, stack, profile):
    self.active, self.stop, self.stack = True, stop, stack
    self.key, self.dictionary, self.value = [self.alloc() for _ in range(3)]
    methods = {}
    for kind, values in re.findall(r"pub const (\w+_METHODS): \[usize; 3\] = \[([^\]]+)\]", profile):
      methods[kind] = [self.metadata(int(value, 16)) for value in re.findall(r"0x[0-9A-Fa-f]+", values)]
    other = self.alloc()
    self.count, self.reads, self.calls = 0, 0, []
    for kind, args, expected in (
      ("STOCK", [self.key, 0], 0xFFFFFFFF),
      ("INFINITY", [self.key, 0], 1),
      ("PURCHASE", [self.key, 3, 0, 0], None),
    ):
      for method in methods[kind + "_METHODS"]:
        before = len(self.calls)
        result = self.run(replacements[kind], args + [method])
        assert len(self.calls) == before
        if expected is not None: assert result == expected
      self.run(replacements[kind], args + [other])
      assert self.calls[-1] == kind
    for method, key_hook in zip(methods["CONTENT_METHODS"], ("WEAPON_KEY", "ITEM_KEY", "FLEA_KEY")):
      for count in (-999, -1, 0, 1, 99):
        self.count = count
        self.build_driver(replacements[key_hook], replacements["LOOKUP"])
        self.run(replacements["CONTENTS"], [0, 0, method])
        assert self.number(self.value) == -1
        assert self.number(self.value + 4) == 17 and self.pointer(self.value + 8) == 0
        # Same key outside the list build must not be overridden.
        self.run(replacements["LOOKUP"], [self.dictionary, self.key, self.value, 0])
        assert self.number(self.value) == count
      self.count = 0
      self.build_driver(replacements[key_hook], replacements["LOOKUP"], other_key=True)
      self.run(replacements["CONTENTS"], [0, 0, method])
      assert self.number(self.value) == 0, "chapter/unlock lookups must remain native"
    self.build_driver(replacements["WEAPON_KEY"], replacements["LOOKUP"])
    self.run(replacements["CONTENTS"], [0, 0, other])
    assert self.number(self.value) == 0, "unrelated generic tables must remain native"
    self.active = False
