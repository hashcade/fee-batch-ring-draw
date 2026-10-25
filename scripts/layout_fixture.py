"""Model Unity geometry while executing the compiled menu hook, not a scene renderer."""

import math
import struct

from unicorn import arm64_const as registers


class LayoutFixture:
  def __init__(self, cpu, game, stub, write, offset):
    self.cpu, self.game, self.write = cpu, game, write
    self.addresses = {game + offset(name): name for name in (
      "MENU_UPDATE", "TMP_RECT", "PREFERRED_VALUES", "RECT", "SET_WIDTH",
      "GET_POSITION", "SET_POSITION", "TRANSFORM_POINT", "INVERSE_POINT")}
    self.addresses[stub] = "ICON_RECT"
    root = game + 0x06400000
    self.menu, items, array, image_class = (root + i * 0x1000 for i in range(4))
    self.write(image_class, bytes(0x600))
    self.content, self.menu_class = root + 0x18000, root + 0x19000
    self.write(self.menu_class, bytes(0x600))
    self.qword(self.menu_class + 0x138 + 30 * 16, game + offset("DRAW_MENU_NAME"))
    self.qword(self.menu, self.menu_class)
    self.qword(self.menu + 0x70, self.content)
    self.qword(self.content + 0x28, self.menu)
    self.qword(image_class + 0x138 + 30 * 16, stub)
    self.qword(self.menu + 0x78, items)
    self.qword(items + 0x10, array)
    self.write(items + 0x18, struct.pack("<i", 2))
    self.qword(array + 0x18, 2)
    self.rows, self.rects, self.text_rects, self.icon_rects, self.widths = [], {}, {}, {}, {}
    self.writes = 0
    self.update_calls = 0
    self.reset_icons = False
    for index in range(2):
      item, view, text, image, amount, icon = (
        root + 0x4000 + index * 0x6000 + j * 0x1000 for j in range(6))
      self.qword(array + 0x20 + index * 8, item)
      self.qword(item + 0x10, self.menu)
      self.qword(item + 0x18, view)
      self.qword(view + 0x18, item)
      self.qword(view + 0x50, text)
      self.qword(view + 0x58, image)
      self.qword(image, image_class)
      self.text_rects[text], self.icon_rects[image] = amount, icon
      # Different pivots and transform scales exercise the native ABI and edge correction.
      pivot = [0.5, 1.0][index]
      self.rects[amount] = dict(width=60.0, pivot=pivot, scale=1.25,
        position=[380.0 - (1.0 - pivot) * 60.0 * 1.25, 50.0 * index, 0.0])
      self.rects[icon] = dict(width=20.0, pivot=1.0 - pivot, scale=0.9,
        position=[300.0, 50.0 * index + 3.0, 0.0])
      self.rows.append((item, view, text, amount, icon))

  def qword(self, address, value):
    self.write(address, struct.pack("<Q", value))

  def floats(self, values):
    for index, value in enumerate(values):
      self.cpu.reg_write(getattr(registers, f"UC_ARM64_REG_S{index}"),
        struct.unpack("<I", struct.pack("<f", value))[0])

  def arguments(self, count):
    return [struct.unpack("<f", struct.pack("<I",
      self.cpu.reg_read(getattr(registers, f"UC_ARM64_REG_S{index}"))))[0]
      for index in range(count)]

  def handle(self, pc):
    name = self.addresses.get(pc)
    if name is None:
      return False
    obj = self.cpu.reg_read(registers.UC_ARM64_REG_X0)
    if name == "MENU_UPDATE":
      self.update_calls += 1
      if self.reset_icons:
        for _, _, _, _, icon in self.rows:
          self.rects[icon]["position"][0] = 300.0
        self.reset_icons = False
    elif name == "TMP_RECT":
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, self.text_rects[obj])
    elif name == "ICON_RECT":
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, self.icon_rects[obj])
    elif name == "PREFERRED_VALUES":
      self.floats([self.widths[obj], 20.0])
    elif name == "RECT":
      state = self.rects[obj]
      self.floats([-state["pivot"] * state["width"], -10.0, state["width"], 20.0])
    elif name == "SET_WIDTH":
      assert self.cpu.reg_read(registers.UC_ARM64_REG_X1) == 0
      self.rects[obj]["width"] = self.arguments(1)[0]
      self.writes += 1
    elif name == "GET_POSITION":
      self.floats(self.rects[obj]["position"])
    elif name == "SET_POSITION":
      self.rects[obj]["position"] = self.arguments(3)
      self.writes += 1
    elif name in ("TRANSFORM_POINT", "INVERSE_POINT"):
      state, point = self.rects[obj], self.arguments(3)
      if name == "TRANSFORM_POINT":
        result = [p * state["scale"] + origin for p, origin in zip(point, state["position"])]
      else:
        result = [(p - origin) / state["scale"] for p, origin in zip(point, state["position"])]
      self.floats(result)
    self.cpu.reg_write(registers.UC_ARM64_REG_PC, self.cpu.reg_read(registers.UC_ARM64_REG_X30))
    return True

  def exercise(self, replacement, stop, stack, count):
    for index, (item, _, _, _, _) in enumerate(self.rows):
      self.write(item + 0x64, struct.pack("<i", 1 if index == 0 else count))

    def call():
      self.cpu.reg_write(registers.UC_ARM64_REG_SP, stack)
      self.cpu.reg_write(registers.UC_ARM64_REG_X0, self.content)
      self.cpu.reg_write(registers.UC_ARM64_REG_X1, 0)
      self.cpu.reg_write(registers.UC_ARM64_REG_X30, stop)
      self.cpu.emu_start(replacement, stop + 4, count=100_000)
      assert self.cpu.reg_read(registers.UC_ARM64_REG_PC) == stop

    # During the native BuildText call the current row's reverse link is not
    # populated yet (main+0x02459008 writes it afterward). Recover on later frames.
    self.qword(self.rows[1][0] + 0x18, 0)
    self.widths = {row[2]: 0.0 for row in self.rows}
    call()
    assert self.writes == 0
    self.qword(self.rows[1][0] + 0x18, self.rows[1][1])
    call()
    assert self.writes == 0, "zero-width fonts must be retried, not applied"

    # No cached maximum: both growing and shrinking glyph widths must take effect.
    for widths in ([42.0, 96.25], [42.0, 131.5], [40.0, 55.2]):
      self.widths = {row[2]: width for row, width in zip(self.rows, widths)}
      expected = math.ceil(max(widths)) + 2.0
      for frame in range(20):
        writes = self.writes
        if frame == 10:
          # A native update/animation can restore prefab geometry. The hook
          # must run afterward, not before, and repair it in this same frame.
          self.reset_icons = True
        call()
        icon_edges = []
        for _, _, _, amount, icon in self.rows:
          number, image = self.rects[amount], self.rects[icon]
          right = number["position"][0] + (1.0 - number["pivot"]) * number["width"] * number["scale"]
          left = right - number["width"] * number["scale"]
          edge = image["position"][0] + (1.0 - image["pivot"]) * image["width"] * image["scale"]
          assert abs(number["width"] - expected) < 0.001
          assert abs(right - 380.0) < 0.001, "amount right edge drifted"
          assert abs(left - edge - 8.0 * number["scale"]) < 0.001, "icon gap is incorrect"
          assert abs(image["position"][1] - number["position"][1] - 3.0) < 0.001
          icon_edges.append(edge)
        assert abs(icon_edges[0] - icon_edges[1]) < 0.001, "row icons are misaligned"
        if frame not in (0, 10):
          assert self.writes == writes, "unchanged frames dirtied the canvas"
    writes = self.writes
    self.widths[self.rows[1][2]] = float("nan")
    call()
    assert self.writes == writes, "invalid text metrics changed geometry"
    self.qword(self.rows[1][0] + 0x18, 0)
    call()
    assert self.writes == writes, "partially bound rows changed geometry"
    self.qword(self.rows[1][0] + 0x18, self.rows[1][1])
    self.widths = {row[2]: 300.0 for row in self.rows}
    self.qword(self.menu_class + 0x138 + 30 * 16, 0)
    call()
    assert self.writes == writes, "unrelated menus changed geometry"
    assert self.update_calls == 65, "the original menu update was skipped"
