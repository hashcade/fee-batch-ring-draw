"""Execute the compiled row text hook with modeled TMP setters; no UI geometry API."""

import struct

from unicorn import arm64_const as registers


class TextFixture:
  def __init__(self, cpu, game, setter, write, offset):
    self.cpu, self.write = cpu, write
    self.addresses = {game + offset(name): name for name in
      ("BUILD_TEXT", "GET_LANGUAGE", "STRING_NEW_SIZE")}
    self.addresses[setter] = "SET_TEXT"
    self.root = game + 0x6400000
    self.content, self.item, self.label, self.amount, self.text_class = (
      self.root + index * 0x1000 for index in range(5))
    self.heap = self.root + 0x10000
    for obj in (self.content, self.item, self.label, self.amount, self.text_class):
      self.write(obj, bytes(0x600))
    self.qword(self.content + 0x18, self.item)
    self.qword(self.content + 0x48, self.label)
    self.qword(self.content + 0x50, self.amount)
    self.qword(self.label, self.text_class)
    self.qword(self.amount, self.text_class)
    self.qword(self.text_class + 0x558, setter)
    self.language, self.cost = 10, 30000
    self.values = {}
    self.original_calls = 0

  def qword(self, address, value):
    self.write(address, struct.pack("<Q", value))

  def handle(self, pc, import_name):
    name = self.addresses.get(pc, import_name)
    if name is None:
      return False
    obj = self.cpu.reg_read(registers.UC_ARM64_REG_X0)
    if name == "BUILD_TEXT":
      assert obj == self.content
      self.original_calls += 1
      self.values = {self.label: "native label", self.amount: f"{self.cost:,}"}
    elif name == "GET_LANGUAGE":
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, self.language)
    elif name == "STRING_NEW_SIZE":
      assert 0 < obj < 128
      value = self.heap
      self.heap += 0x200
      self.write(value, bytes(0x200))
      self.write(value + 0x10, struct.pack("<i", obj))
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, value)
    elif name == "SET_TEXT":
      assert obj in (self.label, self.amount)
      value = self.cpu.reg_read(registers.UC_ARM64_REG_X1)
      size, = struct.unpack("<i", self.cpu.mem_read(value + 0x10, 4))
      self.values[obj] = bytes(self.cpu.mem_read(value + 0x14, size * 2)).decode("utf-16-le")
    elif name == "malloc":
      value = self.heap
      self.heap += max(16, (obj + 15) & ~15)
      self.write(value, bytes(max(16, obj)))
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, value)
    elif name == "free":
      pass
    elif name in ("memcpy", "memmove"):
      source = self.cpu.reg_read(registers.UC_ARM64_REG_X1)
      count = self.cpu.reg_read(registers.UC_ARM64_REG_X2)
      self.cpu.mem_write(obj, bytes(self.cpu.mem_read(source, count)))
    else:
      return False
    self.cpu.reg_write(registers.UC_ARM64_REG_PC, self.cpu.reg_read(registers.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack, count):
    def call(times, price):
      self.cost = price
      self.write(self.item + 0x64, struct.pack("<ii", times, price))
      content_before = bytes(self.cpu.mem_read(self.content, 0x600))
      item_before = bytes(self.cpu.mem_read(self.item, 0x600))
      amount_before = bytes(self.cpu.mem_read(self.amount, 0x600))
      calls = self.original_calls
      self.cpu.reg_write(registers.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, self.content)
      self.cpu.reg_write(registers.UC_ARM64_REG_X1, 0)
      self.cpu.reg_write(registers.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(replacement, stop + 4, count=100_000)
      assert self.cpu.reg_read(registers.UC_ARM64_REG_PC) == stop
      assert self.original_calls == calls + 1
      assert bytes(self.cpu.mem_read(self.content, 0x600)) == content_before
      assert bytes(self.cpu.mem_read(self.item, 0x600)) == item_before
      assert bytes(self.cpu.mem_read(self.amount, 0x600)) == amount_before

    for language in range(12):
      self.language = language
      call(count, count * 100)
      if count < 10:
        assert self.values[self.amount] == str(count * 100)
      else:
        expected = f"{count / 10:g}k"
        assert self.values[self.amount] == expected, (language, self.values)
      assert str(count) in self.values[self.label]
      call(1, 100)
      assert self.values == {self.label: "native label", self.amount: "100"}
    # Price comes from the item's native total, not count multiplied again.
    for price, expected in [(30000, "30k"), (99900, "99.9k"), (100000, "100k"),
      (1234, "1.234k"), (999, "999")]:
      call(count, price)
      assert self.values[self.amount] == expected
    self.qword(self.content + 0x50, 0)
    call(count, 30000)
    assert self.values[self.amount] == "30,000", "missing price text must be ignored"
    self.qword(self.content + 0x50, self.amount)
