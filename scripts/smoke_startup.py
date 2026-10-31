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
from draw_menu_fixture import DrawMenuFixture
from meld_dialog_fixture import MeldDialogFixture
from cleaning_fixture import CleaningFixture
from arena_fixture import ArenaFixture
from bond_talk_fixture import BondTalkFixture
from settings_fixture import SettingsFixture
from convenience_fixture import ConvenienceFixture
from facility_fixture import FacilityFixture
from well_fixture import WellFixture
from shop_fixture import ShopFixture
from appearance_fixture import AppearanceFixture
from gains_fixture import GainsFixture
from support_fixture import SupportFixture
from resource_fixture import ResourceFixture
from polishing_fixture import PolishingFixture


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
  cleaning = (ROOT / "src/cleaning.rs").read_text(encoding="utf-8") + (ROOT / "src/cleaning/automatic.rs").read_text(encoding="utf-8")
  arena = (ROOT / "src/arena.rs").read_text(encoding="utf-8")
  bond_talk = (ROOT / "src/bond_talk.rs").read_text(encoding="utf-8")
  settings_profile = (ROOT / "src/settings.rs").read_text(encoding="utf-8")
  fishing_profile = (ROOT / "src/fishing.rs").read_text(encoding="utf-8")
  gains_profile = (ROOT / "src/gains.rs").read_text(encoding="utf-8")
  well_profile = (ROOT / "src/well.rs").read_text(encoding="utf-8")
  cooking_profile = (ROOT / "src/cooking.rs").read_text(encoding="utf-8")
  support_profile = (ROOT / "src/supports.rs").read_text(encoding="utf-8")
  facility_profile = (ROOT / "src/facilities.rs").read_text(encoding="utf-8")
  shop_profile = (ROOT / "src/shops.rs").read_text(encoding="utf-8")
  preferences_profile = (ROOT / "src/preferences.rs").read_text(encoding="utf-8")
  outfit_profile = (ROOT / "src/outfits/profile.rs").read_text(encoding="utf-8")
  preference_patches = [(0x0270DB44, 0x11000508, 0x11000108), (0x02677284, 0xB9402808, 0x52800028), (0x01E40CF8, 0xB9402808, 0x52800028), (0x01E40E88, 0xB9402808, 0x52800028), (0x01CAA7A4, 0x1A9706D4, 0x1A9706D4)]
  patches = [(int(address, 16), int(original, 16), int(replacement, 16))
    for address, original, replacement in re.findall(
      r"\((0x[0-9A-Fa-f]+),\s*(0x[0-9A-Fa-f]+),\s*(0x[0-9A-Fa-f]+)\)", activities)]
  switchable_addresses = {address for address, _, _ in patches
    if address in (0x02D8158C, 0x025245A8, 0x02524E6C, 0x025245F8)
    or 0x026F0000 <= address < 0x02700000
    or 0x02780000 <= address < 0x027A0000
    or 0x02DC0000 <= address < 0x02DE0000}

  def offset(name: str) -> int:
    return int(re.search(rf"pub const {name}: usize = (0x[0-9A-Fa-f]+)", profile).group(1), 16)

  signatures = [(int(address, 16), b"".join(
    struct.pack("<I", int(word, 16)) for word in re.findall(r"0x[0-9A-Fa-f]+", words)
  )) for address, words in re.findall(r"\((0x[0-9A-Fa-f]+),\s*\[([^\]]+)\]\)", profile + meld_profile + activities + wyvern + collector + cleaning + arena + bond_talk + settings_profile + well_profile + cooking_profile + support_profile + facility_profile + shop_profile + preferences_profile + outfit_profile + fishing_profile + gains_profile)]
  expected_cooking_hooks = {int(value, 16) for name, value in re.findall(
    r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", cooking_profile)
    if name in ("CURRENT_CHEF", "GET_COOK_DATA", "COOK", "DETERMINE_TASTE", "ANIMATION_CREATE")}
  id_values = re.search(r"pub const BUILD_ID:.*?=\s*\[(.*?)\];", profile, re.S).group(1)
  build_id = bytes(int(word, 16) for word in re.findall(r"0x[0-9A-Fa-f]+", id_values))
  expected_hooks = {offset(name) for name in (
    "COUNT_MENU", "BUILD_TEXT", "EXECUTE_GACHA", "UNLOAD", "DRAW", "CREATE_DEMO"
  )}
  expected_hooks.update(expected_cooking_hooks)
  expected_hooks.update((0x02707DD0, offset("PARAMETER"), 0x02B0EA10, 0x01BB5ED0, 0x01BB6C50))
  expected_hooks.add(0x026FFF90)
  expected_hooks.update((0x024E8C20, 0x024E8370, 0x01A39D40, 0x01BE1260))
  expected_hooks.update(int(value, 16) for name, value in re.findall(
    r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", shop_profile) if name != "LOOKUP")
  expected_hooks.update(int(value, 16) for name, value in re.findall(
    r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", support_profile)
    if name in ("HUB_INIT", "HUB_TICK"))
  expected_hooks.update(int(re.search(
    rf"pub const {name}: usize = (0x[0-9A-Fa-f]+)", meld_profile).group(1), 16)
    for name in ("ON_SELECT", "ON_DECIDE", "ON_YES", "ON_CANCEL", "DESTROY_MENU",
      "EXECUTE_MERGE", "CREATE_DEMO", "UNLOAD", "MELD_ONE", "YES_NO_DIALOG"))
  wyvern_main = int(re.search(r"pub const MAIN: usize = (0x[0-9A-Fa-f]+)", wyvern).group(1), 16)
  expected_hooks.add(wyvern_main)
  collector_hooks = {int(re.search(rf"pub const {name}: usize = (0x[0-9A-Fa-f]+)", collector).group(1), 16)
    for name in ("CREATE_MENU",)}
  expected_hooks.update(collector_hooks)
  cleaning_hooks = {int(value, 16) for name, value in re.findall(
    r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", cleaning)
    if name in ("CREATE_MENU", "CREATE_CLEANING", "END_SEQUENCE", "ITEM_NAME")}
  expected_hooks.update(cleaning_hooks)
  arena_create = int(re.search(r"pub const CREATE_COMBAT: usize = (0x[0-9A-Fa-f]+)", arena).group(1), 16)
  expected_hooks.add(arena_create)
  talk_tick = int(re.search(r"pub const TALK_TICK: usize = (0x[0-9A-Fa-f]+)", bond_talk).group(1), 16)
  expected_hooks.add(talk_tick)
  well_menu = int(re.search(r"pub const CREATE_MENU: usize = (0x[0-9A-Fa-f]+)", well_profile).group(1), 16)
  expected_hooks.add(well_menu)
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
  write_game(GAME + 0x0628DD48, bytes(8))
  # Cobalt 1.31 hooks ProcInst.CreateBind before calling any plugin main.
  write_game(GAME + 0x0280A6B0, struct.pack("<I", 0x15186392))
  if case == "wrong_activity_instruction":
    write_game(GAME + patches[0][0], bytes(4))
  if case == "resource_stock_patch":
    write_game(GAME + 0x027B1920, struct.pack("<I", 0x145E5678))
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
  fs_calls = []
  settings_callbacks = []
  thread_keys = {}
  allocation_cursor = GAME + 0x7F00000
  patch_calls = []
  patch_failed = False
  configs = {"config_ten": b"draw_count=10\n", "config_low": b"draw_count=2\n", "config_custom": b"draw_count=600\n",
    "config_high": b"draw_count=1000\n", "config_invalid": b"draw_count=1001\n",
    "config_utf8": b"\xff", "config_large": b" " * 4097,
    "config_short": b"draw_count=600\n", "config_empty": b"",
    "supports_off": b"auto_support=0\n",
    "performance": b"draw_count=300\nchef=PID_performance\n",
    "config_switches_off": b"fishing_unlimited=0\nshop_stock=0\ngift_stock=0\nrepeat_cooking=0\nfast_fishing=0\nfast_training=0\nfast_wyvern=0\n"}
  text = TextFixture(cpu, GAME, STUB + 0xE000, write_game, offset)
  wyvern_game = WyvernFixture(cpu, GAME, write_game, wyvern)
  collector_game = CollectorFixture(cpu, GAME, write_game, collector, profile)
  draw_menu = DrawMenuFixture(cpu, GAME, write_game, offset)
  meld_dialog = MeldDialogFixture(cpu, GAME, write_game, meld_profile, profile)
  cleaning_game = CleaningFixture(cpu, GAME, write_game, cleaning, profile)
  cleaning_game.tls = meld_dialog.tls
  arena_game = ArenaFixture(cpu, GAME, write_game, arena, profile)
  bond_dialogue = BondTalkFixture(cpu, GAME, write_game, bond_talk, profile)
  settings_game = ConvenienceFixture(cpu, GAME, write_game, settings_profile + cooking_profile + support_profile + fishing_profile, profile)
  well_game = WellFixture(cpu, GAME, write_game, well_profile + collector, profile)
  facility_game = FacilityFixture(cpu, GAME, write_game, facility_profile + collector + support_profile, profile)
  shop_game = ShopFixture(cpu, GAME, write_game, shop_profile, profile)
  gains_game = GainsFixture(cpu, GAME, write_game, gains_profile, profile)
  support_game = SupportFixture(cpu, GAME, write_game, support_profile + collector + cooking_profile, profile)
  resource_game = ResourceFixture(cpu, GAME, write_game, settings_profile, profile)
  polishing_game = PolishingFixture(cpu, GAME, write_game,
    settings_profile + cooking_profile + support_profile + cleaning + collector, profile)
  outfit_offsets = dict((name, int(value, 16)) for name, value in re.findall(
    r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", outfit_profile))
  appearance = None

  def string_at(address: int) -> bytes:
    result = bytearray()
    for index in range(128):
      value = cpu.mem_read(address + index, 1)[0]
      if not value:
        return bytes(result)
      result.append(value)
    raise AssertionError("unterminated SDK path")

  def native_call(cpu: Uc, pc: int, size: int, user: object) -> None:
    nonlocal patch_failed, allocation_cursor
    if pc == STOP:
      cpu.emu_stop()
      return
    if polishing_game.handle(pc, names.get(pc)):
      return
    if resource_game.handle(pc, names.get(pc)):
      return
    if support_game.handle(pc, names.get(pc)):
      return
    if gains_game.handle(pc, names.get(pc)):
      return
    if appearance is not None and appearance.handle(pc, names.get(pc)):
      return
    if shop_game.handle(pc, names.get(pc)):
      return
    if wyvern_game.handle(pc):
      return
    if arena_game.handle(pc, names.get(pc)):
      return
    if bond_dialogue.handle(pc, names.get(pc)):
      return
    if settings_game.handle(pc, names.get(pc)):
      return
    if facility_game.handle(pc, names.get(pc)):
      return
    if well_game.handle(pc, names.get(pc)):
      return
    if collector_game.handle(pc, names.get(pc)):
      return
    if meld_dialog.handle(pc, names.get(pc)):
      return
    if cleaning_game.handle(pc, names.get(pc)):
      return
    if draw_menu.handle(pc):
      return
    if text.handle(pc, names.get(pc)):
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
    elif name == "cobapi_register_configmenuitem_cb":
      assert BASE <= argument < BASE + len(data)
      settings_callbacks.append(argument)
    elif name == "_ZN2nn2oe17GetDisplayVersionEPNS0_14DisplayVersionE":
      cpu.mem_write(argument, b"1.3.0\0" if case == "wrong_version" else b"2.0.0\0")
    elif name == "sky_memcpy":
      length = cpu.reg_read(registers.UC_ARM64_REG_X2)
      assert length == 4 and argument - GAME in {p[0] for p in patches + preference_patches}
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
      path = string_at(cpu.reg_read(registers.UC_ARM64_REG_X1))
      assert path in (b"feesomniel:/fee-better-somniel.cfg", b"feesomniel:/fee-better-somniel.cfg.previous")
      assert cpu.reg_read(registers.UC_ARM64_REG_X2) == 1
      fs_calls.append("open")
      cpu.mem_write(argument, struct.pack("<Q", 42))
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0 if case in configs and path.endswith(b".cfg") else 0x202)
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
    elif name in ("malloc", "calloc"):
      count = argument * (cpu.reg_read(registers.UC_ARM64_REG_X1) if name == "calloc" else 1)
      assert count <= 4096
      write_game(allocation_cursor, bytes(max(count, 32)))
      cpu.reg_write(registers.UC_ARM64_REG_X0, allocation_cursor)
      allocation_cursor += (max(count, 32) + 31) & ~31
    elif name == "free":
      pass
    elif name in ("memcpy", "memmove"):
      count = cpu.reg_read(registers.UC_ARM64_REG_X2)
      assert count <= 4096
      if count:
        cpu.mem_write(argument, bytes(cpu.mem_read(cpu.reg_read(registers.UC_ARM64_REG_X1), count)))
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
    elif name == "pthread_key_create":
      key = len(thread_keys) + 1
      thread_keys[key] = 0
      cpu.mem_write(argument, struct.pack("<I", key))
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0)
    elif name == "pthread_getspecific":
      cpu.reg_write(registers.UC_ARM64_REG_X0, thread_keys[argument])
    elif name == "pthread_setspecific":
      thread_keys[argument] = cpu.reg_read(registers.UC_ARM64_REG_X1)
      cpu.reg_write(registers.UC_ARM64_REG_X0, 0)
    else:
      raise AssertionError(f"unexpected initialization import: {name}")
    cpu.reg_write(registers.UC_ARM64_REG_PC, cpu.reg_read(registers.UC_ARM64_REG_X30))

  cpu.hook_add(UC_HOOK_CODE, native_call)
  cpu.reg_write(registers.UC_ARM64_REG_SP, STACK + 0x20000 - 0x20)
  cpu.reg_write(registers.UC_ARM64_REG_X30, STOP)
  cpu.emu_start(BASE + main, STOP + 4, count=100_000)
  if cpu.reg_read(registers.UC_ARM64_REG_PC) != STOP:
    raise AssertionError(f"{case}: initialization did not return")
  startup_patch_calls = patch_calls[:]
  supported = case in ("supported", "polishing", "shops", "supports", "resources", "facilities", "convenience", "dialogue", "menus", "mount_failed", "resource_stock_patch", "wrong_activity_instruction", "patch_failed") or case in configs
  expected = expected_hooks if supported else set()
  if set(hooks) != expected or len(hooks) != len(expected):
    raise AssertionError(f"{case}: incorrect hook registration: {hooks}")
  assert 0x023397D0 not in hooks, "do not intercept the native ring-room menu to hide polishing"
  if supported:
    assert len(settings_callbacks) == 35
    appearance = AppearanceFixture(cpu, GAME, write_game, outfit_offsets.__getitem__, replacements, STOP, STACK + 0x20000 - 0x20)
    settings_game.appearance = appearance
    assert fs_calls.count("mount") == 1
    assert fs_calls.count("unmount") == (case != "mount_failed")
    assert fs_calls.count("close") == (case in configs)
    expected_count = {"config_ten": 10, "config_low": 2, "config_custom": 600, "config_high": 1000}.get(case, 300)
    if case not in ("polishing", "shops", "supports", "supports_off", "resources", "facilities", "convenience", "dialogue", "menus", "performance"):
      draw_menu.exercise(replacements[offset("COUNT_MENU")], STOP,
        STACK + 0x20000 - 0x20, expected_count)
    if case in ("supported", "shops"):
      shop_replacements = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", shop_profile) if int(value, 16) in replacements}
      shop_game.exercise(shop_replacements, STOP, STACK + 0x20000 - 0x20, shop_profile)
      print(f"{case}: compiled shop hooks retain unlock lookups, scope unlimited stock and leave saved counts unchanged")
    if case == "resources":
      resource_game.exercise_resources(settings_callbacks, STOP, STACK + 0x20000 - 0x20)
      print(f"{case}: 35 flat settings rows; one-shot resources/SP use native setters, transient previews and no configuration writes")
    if case == "polishing":
      polishing_game.exercise(settings_callbacks, STOP, STACK + 0x20000 - 0x20)
      support_hooks = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", support_profile)
        if name in ("HUB_INIT", "HUB_TICK")}
      polishing_game.exercise_polishing(support_hooks, replacements[next(iter(collector_hooks))])
      print("polishing: configurable cleaner, native bond caps/records, once-only settlement and Somniel-only/off paths passed modeled boundaries")
    if case in ("supported", "supports", "supports_off", "menus"):
      support_hooks = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", support_profile)
        if name in ("HUB_INIT", "HUB_TICK")}
      support_game.exercise(support_hooks, replacements[next(iter(collector_hooks))],
        STOP, STACK + 0x20000 - 0x20, enabled=case != "supports_off")
      print(f"{case}: direct supports use native level-up and achievement bodies without menus; Somniel-only and disabled paths passed")
    if case in ("supported", "config_low", "config_custom", "config_high"):
      text.exercise(replacements[offset("BUILD_TEXT")], STOP, STACK + 0x20000 - 0x20,
        expected_count)
      print(f"{case}: compiled row text uses exact k prices; original row geometry and single row unchanged")
    if case in ("supported", "dialogue"):
      bond_dialogue.exercise(replacements[talk_tick], STOP, STACK + 0x20000 - 0x20)
      print(f"{case}: compiled dialogue hook retains native skip scoping; non-Main labels avoid metadata and classes initialize once")
    if case == "supported":
      well_game.exercise(replacements[well_menu], STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled well entries preview grouped native results with private RNG; explicit reseed only, no grants or global class edits")
      arena_game.exercise(replacements[arena_create], STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled arena hook keeps native growth/fades for ordinary/paid training without callback-address or descriptor-shape gates")
      meld_replacements = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", meld_profile) if int(value, 16) in replacements}
      meld_dialog.exercise(meld_replacements, STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled meld dialog separates single/batch choices, costs and presentation")
      cleaning_replacements = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", cleaning) if int(value, 16) in replacements}
      cleaning_game.exercise(cleaning_replacements, STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled clean-all menu preserves cleaner attribution, eligibility, deduplication, cancellation and selection restoration")
      wyvern_game.exercise(replacements[wyvern_main], STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled Wyvern hook uses native per-difficulty SSS borders directly; original tick and eligibility preserved")
      collector_game.exercise(replacements[next(iter(collector_hooks))], STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled collection menu handles native claim gates, donation quantities, full-stock dispatch, adoption rules and repeated dispatch")
    if case in ("menus", "resource_stock_patch"):
      if case == "menus":
        draw_menu.exercise(replacements[offset("COUNT_MENU")], STOP,
          STACK + 0x20000 - 0x20, expected_count, extra=True)
        arena_game.exercise(replacements[arena_create], STOP, STACK + 0x20000 - 0x20)
        wyvern_game.exercise(replacements[wyvern_main], STOP, STACK + 0x20000 - 0x20)
      well_game.exercise(replacements[well_menu], STOP, STACK + 0x20000 - 0x20)
      collector_game.exercise(replacements[next(iter(collector_hooks))], STOP, STACK + 0x20000 - 0x20)
      meld_replacements = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", meld_profile) if int(value, 16) in replacements}
      meld_dialog.exercise(meld_replacements, STOP, STACK + 0x20000 - 0x20)
      cleaning_replacements = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", cleaning) if int(value, 16) in replacements}
      cleaning_game.exercise(cleaning_replacements, STOP, STACK + 0x20000 - 0x20)
      print("menus: compiled well/collection entries use real 27-slot basic and 30-slot hub/map inheritance; repeated openings and callbacks passed")
    if case in ("supported", "convenience", "menus", "resource_stock_patch"):
      settings_game.exercise(settings_callbacks, STOP, STACK + 0x20000 - 0x20)
      print("supported: compiled Plugin Settings adjusts counts, persists independent files, retains old values on failure and leaves original classes unchanged")
      new_hooks = {name: replacements[int(value, 16)] for name, value in re.findall(
        r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", cooking_profile + support_profile)
        if int(value, 16) in replacements}
      settings_game.exercise_new([settings_game.factories[i] for i in (0, 1, 2, 3)], new_hooks, STOP, STACK + 0x20000 - 0x20)
      settings_game.exercise_preferences([settings_game.factories[i] for i in (4, 5, 6, 7, 0, 3, 1, 2)], replacements, STOP, STACK + 0x20000 - 0x20)
      def check_disabled():
        for address, original, replacement in patches:
          assert struct.unpack("<I", cpu.mem_read(GAME + address, 4))[0] == (original if address in switchable_addresses else replacement)
        shop_replacements = {name: replacements[int(value, 16)] for name, value in re.findall(
          r"pub const (\w+): usize = (0x[0-9A-Fa-f]+)", shop_profile) if int(value, 16) in replacements}
        settings_game.active = False
        shop_game.exercise(shop_replacements, STOP, STACK + 0x20000 - 0x20, shop_profile, enabled=False)
        wyvern_game.exercise(replacements[wyvern_main], STOP, STACK + 0x20000 - 0x20, enabled=False)
        settings_game.active = True
      settings_game.exercise_features(replacements, STOP, STACK + 0x20000 - 0x20, check_disabled)
      gains_game.exercise(replacements, STOP, STACK + 0x20000 - 0x20, settings_game.configure_multipliers, gains_profile)
      print(f"{case}: compiled cooking choices preserve independent settings and context; automatic-support preferences persist independently")
    if case in ("supported", "facilities", "performance", "menus"):
      facility_game.exercise(replacements[next(iter(collector_hooks))], STOP, STACK + 0x20000 - 0x20, facility_profile)
      print(f"{case}: one outdoor hub entry, three 6/6/5 destination groups, allowances and cleanup passed modeled boundaries")
  else:
    assert not fs_calls, "unsupported executables must not read configuration"
    assert not settings_callbacks, "unsupported executables must not register settings"
  if supported:
    assert struct.unpack("<I", cpu.mem_read(GAME + 0x0280A6B0, 4))[0] == 0x15186392
    if case == "resource_stock_patch":
      assert struct.unpack("<I", cpu.mem_read(GAME + 0x027B1920, 4))[0] == 0x145E5678
    if case == "patch_failed":
      assert patch_failed and patch_calls
      for address, original, _ in patches:
        assert struct.unpack("<I", cpu.mem_read(GAME + address, 4))[0] == original
    else:
      enabled_patches = [(address, replacement) for address, _, replacement in patches
        if case != "config_switches_off" or address not in switchable_addresses]
      assert startup_patch_calls == enabled_patches + [(address, replacement) for address, original, replacement in preference_patches if original != replacement]
      for address, original, replacement in patches:
        expected_word = original if case == "config_switches_off" and address in switchable_addresses else replacement
        assert struct.unpack("<I", cpu.mem_read(GAME + address, 4))[0] == expected_word
  else:
    assert not patch_calls, "unsupported executables must not be modified"
  return len(hooks)


def main() -> None:
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument("nro", type=Path)
  args = parser.parse_args()
  verify_nro(args.nro)
  for case in ("supported", "wrong_version", "wrong_build_id", "wrong_segment_layout",
    "wrong_activity_instruction", "patch_failed", "menus", "resource_stock_patch", "supports_off", "resources", "polishing",
    "mount_failed", "performance", "config_ten", "config_low", "config_custom", "config_high", "config_invalid", "config_utf8",
    "config_large", "config_short", "config_empty", "config_switches_off"):
    count = smoke(args.nro.read_bytes(), case)
    print(f"{case}: initialization returned normally; {count} hooks; no unavailable API call")


if __name__ == "__main__":
  main()
