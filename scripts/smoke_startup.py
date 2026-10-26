#!/usr/bin/env python3
"""Execute NRO initialization in an AArch64 sandbox with modeled loader calls.

Requires unicorn==2.1.4. This checks startup control flow, not the game itself.
"""

import argparse
from pathlib import Path
import re
import struct

from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE
from unicorn import arm64_const as registers
from nro_imports import required_imports
from verify_nro import verify_nro
from text_fixture import TextFixture
from wyvern_fixture import WyvernFixture
from collector_fixture import CollectorFixture


ROOT = Path(__file__).resolve().parent.parent
BASE, GAME, STUB, STACK = (0x100000000, 0x200000000, 0x300000000, 0x400000000)
STOP = STUB + 0xF000


def metadata(data: bytes) -> tuple[list[tuple], list[tuple]]:
  # The bounded structure/import verifier has already accepted this image.
  mod, = struct.unpack_from("<I", data, 4)
  relative, = struct.unpack_from("<i", data, mod + 4)
  cursor = mod + relative
  tags = {}
  while True:
    tag, value = struct.unpack_from("<QQ", data, cursor)
    cursor += 16
    if not tag:
      break
    tags[tag] = value
  count, = struct.unpack_from("<I", data, tags[4] + 4)
  symbols = []
  for index in range(count):
    name, _, _, section, value, _ = struct.unpack_from("<IBBHQQ", data, tags[6] + index * 24)
    start = tags[5] + name
    symbols.append((data[start:data.index(0, start)].decode(), section, value))
  relocations = []
  for table, size in ((7, 8), (23, 2)):
    if table in tags:
      for cursor in range(tags[table], tags[table] + tags[size], 24):
        relocations.append(struct.unpack_from("<QQq", data, cursor))
  return symbols, relocations


def smoke(data: bytes, case: str) -> int:
  profile = (ROOT / "src/profile.rs").read_text(encoding="utf-8")
  meld_profile = (ROOT / "src/meld/profile.rs").read_text(encoding="utf-8")
  activities = (ROOT / "src/activities.rs").read_text(encoding="utf-8")
  wyvern = (ROOT / "src/wyvern.rs").read_text(encoding="utf-8")
  collector = (ROOT / "src/collector/profile.rs").read_text(encoding="utf-8")
  patches = [(int(address, 16), int(original, 16), int(replacement, 16))
    for address, original, replacement in re.findall(
      r"\((0x[0-9A-Fa-f]+),\s*(0x[0-9A-Fa-f]+),\s*(0x[0-9A-Fa-f]+)\)", activities)]

  def offset(name: str) -> int:
    return int(re.search(rf"pub const {name}: usize = (0x[0-9A-Fa-f]+)", profile).group(1), 16)

  signatures = [(int(address, 16), b"".join(
    struct.pack("<I", int(word, 16)) for word in re.findall(r"0x[0-9A-Fa-f]+", words)
  )) for address, words in re.findall(r"\((0x[0-9A-Fa-f]+),\s*\[([^\]]+)\]\)", profile + meld_profile + activities + wyvern + collector)]
  id_values = re.search(r"pub const BUILD_ID:.*?=\s*\[(.*?)\];", profile, re.S).group(1)
  build_id = bytes(int(word, 16) for word in re.findall(r"0x[0-9A-Fa-f]+", id_values))
  expected_hooks = {offset(name) for name in (
    "COUNT_CTOR", "BUILD_TEXT", "EXECUTE_GACHA", "UNLOAD", "DRAW", "CREATE_DEMO"
  )}
  expected_hooks.update(int(re.search(
    rf"pub const {name}: usize = (0x[0-9A-Fa-f]+)", meld_profile).group(1), 16)
    for name in ("ON_SELECT", "ON_DECIDE", "ON_YES", "ON_CANCEL", "DESTROY_MENU",
      "EXECUTE_MERGE", "CREATE_DEMO", "UNLOAD", "MELD_ONE", "YES_NO_DIALOG"))
  wyvern_main = int(re.search(r"pub const MAIN: usize = (0x[0-9A-Fa-f]+)", wyvern).group(1), 16)
  expected_hooks.add(wyvern_main)
  collector_hooks = {int(re.search(rf"pub const {name}: usize = (0x[0-9A-Fa-f]+)", collector).group(1), 16)
    for name in ("CREATE_MENU",)}
  expected_hooks.update(collector_hooks)
  symbols, relocations = metadata(data)
  main = next(value for name, section, value in symbols if name == "main" and section)
  cpu = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
  image_size = (len(data) + struct.unpack_from("<I", data, 0x38)[0] + 0xFFF) & ~0xFFF
  cpu.mem_map(BASE, image_size)
  cpu.mem_write(BASE, data)
  cpu.mem_map(STUB, 0x10000)
  cpu.mem_map(STACK, 0x20000)
  pages = set()

  def write_game(address: int, value: bytes) -> None:
    for page in range(address & ~0xFFF, (address + len(value) + 0xFFF) & ~0xFFF, 0x1000):
      if page not in pages:
        cpu.mem_map(page, 0x1000)
        pages.add(page)
    cpu.mem_write(address, value)

  for address, words in signatures:
    write_game(GAME + address, words)
  write_game(GAME + offset("BUILD_ID_OFFSET"), build_id if case != "wrong_build_id" else bytes(16))
  if case == "wrong_instruction":
    write_game(GAME + signatures[0][0], bytes(4))
  if case == "wrong_activity_instruction":
    write_game(GAME + patches[0][0], bytes(4))
  if case == "wrong_wyvern_instruction":
    write_game(GAME + wyvern_main, bytes(4))
  if case == "wrong_collector_instruction":
    write_game(GAME + min(collector_hooks), bytes(4))
  stubs = {name: STUB + index * 16 for index, name in enumerate(sorted(required_imports(data)))}
  names = {address: name for name, address in stubs.items()}
  for target, info, addend in relocations:
    index, kind = info >> 32, info & 0xFFFFFFFF
    if kind == 1027:  # R_AARCH64_RELATIVE
      value = BASE + addend
    elif kind in (257, 1025, 1026):  # ABS64, GLOB_DAT, JUMP_SLOT
      name, section, value = symbols[index]
      if not section:
        value = stubs[name]
      elif section != 0xFFF1:
        value += BASE
      value += addend
    else:
      raise ValueError(f"unsupported startup relocation {kind}")
    cpu.mem_write(BASE + target, struct.pack("<Q", value))
  hooks = []
  replacements = {}
  selected_counts = []
  fs_calls = []
  patch_calls = []
  patch_failed = False
  configs = {"config_low": b"draw_count=2\n", "config_custom": b"draw_count=600\n",
    "config_high": b"draw_count=1000\n", "config_invalid": b"draw_count=1001\n",
    "config_utf8": b"\xff", "config_large": b" " * 4097,
    "config_short": b"draw_count=600\n", "config_empty": b""}
  text = TextFixture(cpu, GAME, STUB + 0xE000, write_game, offset)
  wyvern_game = WyvernFixture(cpu, GAME, write_game, wyvern)
  collector_game = CollectorFixture(cpu, GAME, write_game, collector, profile)

  def string_at(address: int) -> bytes:
    result = bytearray()
    for index in range(128):
      value = cpu.mem_read(address + index, 1)[0]
      if not value:
        return bytes(result)
      result.append(value)
    raise AssertionError("unterminated SDK path")

  def native_call(cpu: Uc, pc: int, size: int, user: object) -> None:
    nonlocal patch_failed
    if pc == STOP:
      cpu.emu_stop()
      return
    if wyvern_game.handle(pc):
      return
    if collector_game.handle(pc, names.get(pc)):
      return
    if text.handle(pc, names.get(pc)):
      return
    if pc == GAME + offset("COUNT_CTOR"):
      selected_counts.append(cpu.reg_read(registers.UC_ARM64_REG_X1))
      cpu.reg_write(registers.UC_ARM64_REG_PC, cpu.reg_read(registers.UC_ARM64_REG_X30))
      return
    if pc not in names:
      return
    name = names[pc]
    argument = cpu.reg_read(registers.UC_ARM64_REG_X0)
    if name == "getRegionAddress":
      delta = {0: 0, 1: offset("RODATA_OFFSET"), 2: 0x5D7C000}[argument]
      if case == "wrong_segment_layout" and argument == 1:
        delta += 0x1000
      cpu.reg_write(registers.UC_ARM64_REG_X0, GAME + delta)
    elif name == "_ZN2nn2oe17GetDisplayVersionEPNS0_14DisplayVersionE":
      cpu.mem_write(argument, b"1.3.0\0" if case == "wrong_version" else b"2.0.0\0")
    elif name == "sky_memcpy":
      length = cpu.reg_read(registers.UC_ARM64_REG_X2)
      assert length == 4 and argument - GAME in {p[0] for p in patches}
      word = bytes(cpu.mem_read(cpu.reg_read(registers.UC_ARM64_REG_X1), length))
      patch_calls.append((argument - GAME, struct.unpack("<I", word)[0]))
      fail = case == "patch_failed" and len(patch_calls) == 2 and not patch_failed
      patch_failed |= fail
      if not fail:
        cpu.mem_write(argument, word)
      cpu.reg_write(registers.UC_ARM64_REG_X0, 1 if fail else 0)
    elif name == "A64HookFunction":
      replacement = cpu.reg_read(registers.UC_ARM64_REG_X1)
      if not BASE <= replacement < BASE + len(data):
        raise AssertionError("hook replacement is outside the plugin")
      hooks.append(argument - GAME)
      replacements[argument - GAME] = replacement
      cpu.mem_write(cpu.reg_read(registers.UC_ARM64_REG_X2), struct.pack("<Q", argument))
    elif name == "_ZN2nn2fs11MountSdCardEPKc":
      assert string_at(argument) == b"feesomniel"
      fs_calls.append("mount")
      cpu.reg_write(registers.UC_ARM64_REG_X0, 1 if case == "mount_failed" else 0)
    elif name == "_ZN2nn2fs7UnmountEPKc":
      assert string_at(argument) == b"feesomniel"
      fs_calls.append("unmount")
    elif name == "_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci":
      assert string_at(cpu.reg_read(registers.UC_ARM64_REG_X1)) == b"feesomniel:/fee-better-somniel.cfg"
      assert cpu.reg_read(registers.UC_ARM64_REG_X2) == 1
      fs_calls.append("open")
      cpu.mem_write(argument, struct.pack("<Q", 42))
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0 if case in configs else 1)
    elif name == "_ZN2nn2fs11GetFileSizeEPlNS0_10FileHandleE":
      cpu.mem_write(argument, struct.pack("<q", len(configs[case])))
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0)
    elif name == "_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm":
      length = cpu.reg_read(registers.UC_ARM64_REG_X4)
      assert 0 < length <= 4096
      contents = configs[case][:length]
      if case == "config_short":
        contents = contents[:-1]
      cpu.mem_write(cpu.reg_read(registers.UC_ARM64_REG_X3), contents)
      cpu.mem_write(argument, struct.pack("<Q", len(contents)))
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0)
    elif name == "_ZN2nn2fs9CloseFileENS0_10FileHandleE":
      assert argument == 42
      fs_calls.append("close")
    elif name == "memset":
      value = cpu.reg_read(registers.UC_ARM64_REG_X1) & 0xFF
      count = cpu.reg_read(registers.UC_ARM64_REG_X2)
      assert count <= 4096, "unexpected startup memory initialization"
      cpu.mem_write(argument, bytes([value]) * count)
    elif name == "memcmp":
      other = cpu.reg_read(registers.UC_ARM64_REG_X1)
      count = cpu.reg_read(registers.UC_ARM64_REG_X2)
      assert count <= 4096
      left, right = bytes(cpu.mem_read(argument, count)), bytes(cpu.mem_read(other, count))
      cpu.reg_write(registers.UC_ARM64_REG_X0, (int(left > right) - int(left < right)) & 0xFFFFFFFFFFFFFFFF)
    else:
      raise AssertionError(f"unexpected initialization import: {name}")
    cpu.reg_write(registers.UC_ARM64_REG_PC, cpu.reg_read(registers.UC_ARM64_REG_X30))

  cpu.hook_add(UC_HOOK_CODE, native_call)
  cpu.reg_write(registers.UC_ARM64_REG_SP, STACK + 0x20000 - 0x20)
  cpu.reg_write(registers.UC_ARM64_REG_X30, STOP)
  cpu.emu_start(BASE + main, STOP + 4, count=100_000)
  if cpu.reg_read(registers.UC_ARM64_REG_PC) != STOP:
    raise AssertionError(f"{case}: initialization did not return")
  supported = case == "supported" or case == "mount_failed" or case in configs
  expected = expected_hooks if supported else set()
  if set(hooks) != expected or len(hooks) != len(expected):
    raise AssertionError(f"{case}: incorrect hook registration: {hooks}")
  if supported:
    assert fs_calls.count("mount") == 1
    assert fs_calls.count("unmount") == (case != "mount_failed")
    assert fs_calls.count("close") == (case in configs)
    expected_count = {"config_low": 2, "config_custom": 600, "config_high": 1000}.get(case, 300)
    for count in (10, 1):
      cpu.reg_write(registers.UC_ARM64_REG_SP, STACK + 0x20000 - 0x20)
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0)
      cpu.reg_write(registers.UC_ARM64_REG_X1, count)
      cpu.reg_write(registers.UC_ARM64_REG_X2, 0)
      cpu.reg_write(registers.UC_ARM64_REG_X3, 0)
      cpu.reg_write(registers.UC_ARM64_REG_X30, STOP)
      cpu.emu_start(replacements[offset("COUNT_CTOR")], STOP + 4, count=10_000)
      assert cpu.reg_read(registers.UC_ARM64_REG_PC) == STOP
    assert selected_counts == [expected_count, 1], (case, selected_counts)
    if case in ("supported", "config_low", "config_custom", "config_high"):
      text.exercise(replacements[offset("BUILD_TEXT")], STOP, STACK + 0x20000 - 0x20,
        expected_count)
      print(f"{case}: compiled row text uses exact k prices; native geometry and single row unchanged")
    if case == "supported":
      wyvern_game.exercise(replacements[wyvern_main], STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled Wyvern hook uses per-difficulty SSS boundaries; guarded fallbacks and native tick preserved")
      collector_game.exercise(replacements[next(iter(collector_hooks))], STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled collection menu handles native claim gates, donation quantities, capacity, adoption restrictions and repeated dispatch")
  else:
    assert not fs_calls, "unsupported executables must not read configuration"
  if supported:
    assert patch_calls == [(address, replacement) for address, _, replacement in patches]
    for address, _, replacement in patches:
      assert struct.unpack("<I", cpu.mem_read(GAME + address, 4))[0] == replacement
  elif case == "patch_failed":
    assert patch_failed and patch_calls
    for address, original, _ in patches:
      assert struct.unpack("<I", cpu.mem_read(GAME + address, 4))[0] == original
  else:
    assert not patch_calls, "unsupported executables must not be modified"
  return len(hooks)


def main() -> None:
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument("nro", type=Path)
  args = parser.parse_args()
  verify_nro(args.nro)
  for case in ("supported", "wrong_version", "wrong_build_id", "wrong_instruction", "wrong_segment_layout",
    "wrong_activity_instruction", "wrong_wyvern_instruction", "wrong_collector_instruction", "patch_failed",
    "mount_failed", "config_low", "config_custom", "config_high", "config_invalid", "config_utf8",
    "config_large", "config_short", "config_empty"):
    count = smoke(args.nro.read_bytes(), case)
    print(f"{case}: initialization returned normally; {count} hooks; no unavailable API call")


if __name__ == "__main__":
  main()
