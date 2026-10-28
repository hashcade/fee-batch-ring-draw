use fee_better_somniel::activities::{apply, CodeMemory, PATCHES, TEXT_SIGNATURES};
use std::collections::BTreeMap;

struct Memory {
  words: BTreeMap<usize, u32>,
  writes: Vec<(usize, u32)>,
  fail_at: Option<usize>,
}

impl Memory {
  fn original() -> Self {
    Self {
      words: PATCHES.iter().map(|&(offset, original, _)| (offset, original)).collect(),
      writes: Vec::new(),
      fail_at: None,
    }
  }
}

impl CodeMemory for Memory {
  fn read(&self, offset: usize) -> u32 {
    self.words[&offset]
  }

  fn write(&mut self, offset: usize, word: u32) -> bool {
    if self.fail_at == Some(self.writes.len()) {
      self.fail_at = None;
      return false;
    }
    self.writes.push((offset, word));
    self.words.insert(offset, word);
    true
  }
}

#[test]
fn applies_every_instruction_once_without_polling() {
  let mut memory = Memory::original();
  assert!(apply(&mut memory));
  assert_eq!(memory.writes.len(), PATCHES.len());
  for &(offset, _, replacement) in PATCHES {
    assert_eq!(memory.read(offset), replacement);
  }
  memory.writes.clear();
  assert!(apply(&mut memory));
  assert_eq!(memory.writes.len(), PATCHES.len());
}

#[test]
fn existing_activity_changes_do_not_block_initialization() {
  for &(offset, _, replacement) in PATCHES {
    let mut memory = Memory::original();
    memory.words.insert(offset, replacement);
    assert!(apply(&mut memory));
    assert_eq!(memory.writes.len(), PATCHES.len());
  }
}

#[test]
fn each_failed_write_restores_the_original_instructions() {
  for failed in 0..PATCHES.len() {
    let mut memory = Memory::original();
    let expected = memory.words.clone();
    memory.fail_at = Some(failed);
    assert!(!apply(&mut memory));
    assert_eq!(memory.words, expected);
  }
}

#[test]
fn write_failure_restores_existing_changes_instead_of_vanilla_words() {
  let mut memory = Memory::original();
  memory.words.insert(PATCHES[0].0, 0xD503201F);
  let expected = memory.words.clone();
  memory.fail_at = Some(2);
  assert!(!apply(&mut memory));
  assert_eq!(memory.words, expected);
}

#[test]
fn every_patch_has_a_matching_original_signature_and_unique_address() {
  let offsets: BTreeMap<_, _> = PATCHES
    .iter()
    .map(|&(offset, old, new)| {
      assert_eq!(offset % 4, 0);
      assert_ne!(old, new);
      assert!(TEXT_SIGNATURES.iter().any(|&(address, words)| address == offset && words[0] == old));
      (offset, old)
    })
    .collect();
  assert_eq!(offsets.len(), PATCHES.len());
  // Leave arena counts, rewind counts, and resource-cheat code caves alone.
  for offset in [0x01CAA7A4, 0x0270DB44] {
    assert!(!offsets.contains_key(&offset));
  }
  assert!(PATCHES.iter().all(|&(offset, _, _)| offset < 0x03F47100));
}

#[test]
fn all_migrated_activity_instructions_and_the_three_scoped_gift_changes_are_present() {
  assert_eq!(PATCHES.len(), 32);
  let offsets: Vec<_> = PATCHES.iter().map(|&(offset, _, _)| offset).collect();
  for required in [0x02D8158C, 0x01E546B4, 0x0241F920, 0x025245A8, 0x02524E6C, 0x025245F8] {
    assert!(offsets.contains(&required));
  }
  // Keep the native ordinary-gift filter, support-award call, and global item
  // inventory methods untouched. Battle item consumption must remain native.
  for excluded in [0x02524E54, 0x025245E4, 0x027B1920, 0x027B1C30, 0x027B1D38] {
    assert!(!offsets.contains(&excluded));
  }
}

#[test]
fn sommie_reuses_native_finish_path_without_patching_reward_eligibility() {
  assert!(PATCHES.contains(&(0x01E54F04, 0xD0022198, 0x140000C2)));
  assert_eq!(0x01E54F04 + (0xC2 * 4), 0x01E5520C);
  // Preserve the initial petting interval, food choice/consumption, affinity,
  // native GetBond eligibility/settlement, and original exit routines.
  for address in
    [0x01E54E0C, 0x01E54158, 0x020361E0, 0x02036AB0, 0x0203A3B0, 0x0203A6A0, 0x02AEF6F0, 0x02AEFA10]
  {
    assert!(!PATCHES.iter().any(|&(offset, _, _)| offset == address));
  }
}
