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
  assert!(!apply(&mut memory));
  assert!(memory.writes.is_empty());
}

#[test]
fn any_conflicting_cheat_prevents_every_write() {
  for &(offset, _, replacement) in PATCHES {
    let mut memory = Memory::original();
    memory.words.insert(offset, replacement);
    assert!(!apply(&mut memory));
    assert!(memory.writes.is_empty());
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
