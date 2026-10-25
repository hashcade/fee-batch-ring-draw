//! One-time executable patches migrated from the user's Engage 2.0.0 cheats.
//! No inventory polling, allowance changes, or feature switches are added.

/// Main-relative offset, original instruction, replacement instruction.
pub const PATCHES: &[(usize, u32, u32)] = &[
  (0x02D8158C, 0x52800021, 0x52800001),
  (0x01E546B4, 0x5400026B, 0xD503201F),
  (0x026F2624, 0x540000E9, 0xD503201F),
  (0x026F26BC, 0x54FFFBAB, 0xD503201F),
  (0x026F275C, 0x54FFF6A5, 0xD503201F),
  (0x026F2964, 0x39424668, 0x52800008),
  (0x026F29D0, 0x1E212000, 0x1E212020),
  (0x026F4714, 0x1E203820, 0x1E213820),
];

/// Pin the surrounding instructions as well as each replaced instruction.
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02D8158C, [0x52800021, 0xAA1F03E2, 0xA9007FFF, 0x97C3BD7A]),
  (0x01E546B4, [0x5400026B, 0xF0022228, 0xF9400A94, 0xF9410108]),
  (0x026F2624, [0x540000E9, 0x3943A268, 0x340002A8, 0xA9424FF4]),
  (0x026F26BC, [0x54FFFBAB, 0x52A85908, 0x1E270108, 0x1E2703E0]),
  (0x026F275C, [0x54FFF6A5, 0xB9001A74, 0xB900727F, 0xAA1303E0]),
  (0x026F2964, [0x39424668, 0x340002E8, 0x9001DCA8, 0xF9413908]),
  (0x026F29D0, [0x1E212000, 0x54001509, 0x39424E68, 0x340012E8]),
  (0x026F4714, [0x1E203820, 0xF9406C08, 0xBD006000, 0xB900C91F]),
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
