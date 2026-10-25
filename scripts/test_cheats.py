"""Keep the optional cheat companion separate from migrated Somniel effects."""

from pathlib import Path
import re
import unittest


ROOT = Path(__file__).resolve().parent.parent
CHEATS = ROOT / "cheats/8C08B9719E085F91.txt"


class CheatCompanionTests(unittest.TestCase):
  def test_only_the_nine_retained_entries_remain_and_battle_save_is_last(self):
    names = re.findall(r"^\[([^\]]+)\]$", CHEATS.read_text(), re.M)
    self.assertEqual(names, [
      "Max Gold", "Max Bond Fragments", "Max Forging Materials", "Max Crystals",
      "Max SP", "Infinite Shop Stock", "Infinite Arena Training",
      "Infinite Time Rewind", "Battle Save",
    ])

  def test_retained_direct_writes_do_not_overlap_activity_patches(self):
    source = (ROOT / "src/activities.rs").read_text()
    patches = {int(value, 16) for value in re.findall(
      r"\((0x[0-9A-Fa-f]+),\s*0x[0-9A-Fa-f]+,\s*0x[0-9A-Fa-f]+\)", source)}
    writes = {int(value, 16) for value in re.findall(
      r"^(?:040[0B]0000|080[0B]0000) ([0-9A-Fa-f]{8}) ", CHEATS.read_text(), re.M)}
    self.assertTrue(patches)
    self.assertTrue(writes)
    self.assertFalse(patches & writes)


if __name__ == "__main__":
  unittest.main()
