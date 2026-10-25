"""Keep the optional cheat companion separate from migrated Somniel effects."""

from pathlib import Path
import hashlib
import re
import unittest


ROOT = Path(__file__).resolve().parent.parent
CHEATS = ROOT / "cheats/8C08B9719E085F91.txt"


class CheatCompanionTests(unittest.TestCase):
  def test_resources_are_combined_sp_is_independent_and_battle_save_is_last(self):
    names = re.findall(r"^\[([^\]]+)\]$", CHEATS.read_text(), re.M)
    self.assertEqual(names, [
      "Max Resources", "Max SP", "Infinite Shop Stock", "Infinite Arena Training",
      "Infinite Time Rewind", "Battle Save",
    ])

  def test_combined_resource_instructions_are_unchanged_and_in_original_order(self):
    body = CHEATS.read_text().split("[Max Resources]\n", 1)[1].split("[Max SP]", 1)[0]
    instructions = "\n".join(line.strip() for line in body.splitlines() if line.strip())
    self.assertEqual(hashlib.sha256(instructions.encode()).hexdigest(),
      "59cc1b76c8f3ef358eee8bc7a32d2e2248ace613bbb9c3c432b2fc629440504a")

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
