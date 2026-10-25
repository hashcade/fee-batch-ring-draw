//! One-time executable patches migrated from the user's Engage 2.0.0 cheats.
//! No inventory polling, allowance changes, or feature switches are added.

/// Main-relative offset, original instruction, replacement instruction.
pub const PATCHES: &[(usize, u32, u32)] =
  &[(0x02D8158C, 0x52800021, 0x52800001), (0x01E546B4, 0x5400026B, 0xD503201F)];

/// Pin the surrounding instructions as well as each replaced instruction.
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02D8158C, [0x52800021, 0xAA1F03E2, 0xA9007FFF, 0x97C3BD7A]),
  (0x01E546B4, [0x5400026B, 0xF0022228, 0xF9400A94, 0xF9410108]),
];

pub trait CodeMemory {
  fn read(&self, offset: usize) -> u32;
  fn write(&mut self, offset: usize, word: u32) -> bool;
}

/// Check the complete set before writing; restore startup writes on failure.
pub fn apply(memory: &mut impl CodeMemory) -> bool {
  if PATCHES.iter().any(|&(offset, original, _)| memory.read(offset) != original) {
    return false;
  }
  for (index, &(offset, _, replacement)) in PATCHES.iter().enumerate() {
    if !memory.write(offset, replacement) || memory.read(offset) != replacement {
      for &(offset, original, _) in PATCHES[..=index].iter().rev() {
        memory.write(offset, original);
      }
      return false;
    }
  }
  true
}

#[cfg(target_os = "switch")]
pub(crate) fn install() -> bool {
  struct NativeCode;

  impl CodeMemory for NativeCode {
    fn read(&self, offset: usize) -> u32 {
      unsafe { ((crate::native::base() + offset) as *const u32).read_volatile() }
    }

    fn write(&mut self, offset: usize, word: u32) -> bool {
      skyline::patching::Patch::in_text(offset).data(word).is_ok()
    }
  }

  apply(&mut NativeCode)
}
