"""Execute native food availability/settlement windows with compiled patch words.

API calls outside these NSO-pinned windows are modeled, not a running Unity scene.
"""

import struct
from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE
from unicorn import arm64_const as reg


NATIVE_BODIES = (
  (0x020388E0, "fd7bbfa9fd030091003440f9000100b4e1031faa1f6217941f0400712800805200a5881afd7bc1a8c0035fd640008052fd7bc1a8c0035fd6"),
  (0x01E63610, "fd7bbfa9fd030091086440b9a80000341f09007100010054003440f903000014003440f9400100b4e1031faacdb61e948000003420008052fd7bc1a8c0035fd640008052fd7bc1a8c0035fd640018052fd7bc1a8c0035fd6"),
  (0x01E54134, "400100b4e1031faaf50300aaf87525941f040071ab00005401008012e00315aae2031faab6762594"),
  (0x02D813A8, "743e40f9880e40f91f050071ab010054f5031faa087d409296820091bf4228eb62130054c07a75f8e1031faa3b3fe297881a40b9b5060091bfc228eb0bffff54"),
  (0x01E546AC, "9d7425941f0400716b020054"),
)

FOOD_PATCHES = (0x02038900, 0x01E63640, 0x01E54158, 0x02D813D4, 0x01E546B4)


def exercise_food(compiled_words, enabled=True):
  for patched in (False, True):
    stock_free = patched and enabled
    cpu = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
    pages = set()

    def write(address, data):
      for page in range(address & ~0xFFF, (address + len(data) + 0xFFF) & ~0xFFF, 0x1000):
        if page not in pages:
          cpu.mem_map(page, 0x1000)
          pages.add(page)
      cpu.mem_write(address, data)

    def qword(address, value):
      write(address, struct.pack("<Q", value))

    for address, body in NATIVE_BODIES:
      write(address, bytes.fromhex(body))
    if patched:
      for address in FOOD_PATCHES:
        write(address, struct.pack("<I", compiled_words[address]))

    stock_read, food_count = 0x027B1920, 0x02611170
    stock_add, food_sub = 0x027B1C30, 0x026110C0
    stack, stop = 0x1001000, 0x1000000
    item, food, owner, selected = 0x1100000, 0x1101000, 0x1102000, 0x1103000
    for address in (stock_read, food_count, stock_add, food_sub, stop):
      write(address, bytes.fromhex("c0035fd6"))
    write(stack - 0x1000, bytes(0x1000))
    for address in (item, food, owner, selected):
      write(address, bytes(0x1000))
    stocks, decrements = {}, []
    terminals = set()

    def native_call(machine, pc, size, _):
      if pc in terminals:
        machine.emu_stop()
        return
      if pc not in (stock_read, food_count, stock_add, food_sub):
        return
      data = machine.reg_read(reg.UC_ARM64_REG_X0)
      assert data in stocks, f"unexpected food receiver {data:#x}"
      if pc in (stock_read, food_count):
        machine.reg_write(reg.UC_ARM64_REG_X0, stocks[data])
      else:
        if pc == stock_add:
          assert machine.reg_read(reg.UC_ARM64_REG_W1) == 0xFFFFFFFF
        if stocks[data] > 0:
          stocks[data] -= 1
          decrements.append(data)
      machine.reg_write(reg.UC_ARM64_REG_PC, machine.reg_read(reg.UC_ARM64_REG_X30))

    cpu.hook_add(UC_HOOK_CODE, native_call)

    def call(entry, ends, **registers):
      terminals.clear()
      terminals.update(ends)
      cpu.reg_write(reg.UC_ARM64_REG_SP, stack)
      cpu.reg_write(reg.UC_ARM64_REG_X30, stop)
      for name, value in registers.items():
        cpu.reg_write(getattr(reg, f"UC_ARM64_REG_{name.upper()}"), value)
      cpu.emu_start(entry, stop + 4, count=1000)
      assert cpu.reg_read(reg.UC_ARM64_REG_PC) in terminals
      return cpu.reg_read(reg.UC_ARM64_REG_X0)

    for count in (0, 1, 17):
      stocks[food] = count
      qword(item + 0x68, food)
      assert call(0x020388E0, {stop}, x0=item) == (1 if stock_free or count else 2)
      for kind in (0, 1):  # Normal ingredients and rare additions.
        write(item + 0x64, struct.pack("<I", kind))
        assert call(0x01E63610, {stop}, x0=item) == (1 if stock_free or count else 2)
      assert stocks[food] == count

      call(0x01E546AC, {0x01E546B8, 0x01E54700}, x0=food)
      assert cpu.reg_read(reg.UC_ARM64_REG_PC) == (0x01E546B8 if stock_free or count else 0x01E54700)
      decrements.clear()
      call(0x01E54134, {0x01E5415C}, x0=food, x20=owner)
      assert stocks[food] == (count if stock_free else max(0, count - 1))
      assert decrements == ([] if stock_free or count == 0 else [food])

    # Missing data remains invalid; the native confirm row still works.
    qword(item + 0x68, 0)
    assert call(0x020388E0, {stop}, x0=item) == 2
    write(item + 0x64, struct.pack("<I", 0))
    assert call(0x01E63610, {stop}, x0=item) == 0xA
    write(item + 0x64, struct.pack("<I", 2))
    assert call(0x01E63610, {stop}, x0=item) == 1

    # The actual selected array, including a zero-stock ingredient and addition,
    # survives settlement unchanged. Repeating the transaction cannot drain it.
    foods = [food, food + 0x100, food + 0x200]
    original_stocks = dict(zip(foods, (0, 1, 17)))
    stocks.update(original_stocks)
    qword(owner + 0x78, selected)
    qword(selected + 0x18, len(foods))
    for index, data in enumerate(foods):
      qword(selected + 0x20 + index * 8, data)
    selection_bytes = bytes(cpu.mem_read(selected, 0x38))
    for iteration in (1, 2):
      decrements.clear()
      call(0x02D813A8, {0x02D813E8}, x19=owner)
      assert [stocks[data] for data in foods] == [
        count if stock_free else max(0, count - iteration) for count in original_stocks.values()]
      expected_decrements = []
      if not stock_free:
        expected_decrements = [data for data, count in original_stocks.items() if count >= iteration]
      assert decrements == expected_decrements
      assert bytes(cpu.mem_read(selected, 0x38)) == selection_bytes
    qword(selected + 0x18, 0)
    call(0x02D813A8, {0x02D813E8}, x19=owner)
  print(f"native food windows (stock-free={enabled}): zero/owned stock, choices and repeated settlement passed")
