//! One-time activity patches and scoped gift extensions for Engage 2.0.0.
//! No inventory polling, allowance changes, or feature switches are added.

/// Main-relative offset, original instruction, replacement instruction.
pub const PATCHES: &[(usize, u32, u32)] = &[
  // Sommie: retain native food/reaction/reward paths, shorten fixed waits.
  (0x01E54244, 0x54001584, 0xD503201F),
  (0x01E542C0, 0x540011E4, 0xD503201F),
  (0x01E543D4, 0x54000C64, 0xD503201F),
  (0x01E54494, 0x54000285, 0x14000014),
  (0x01E544E0, 0x54000124, 0xD503201F),
  // Keep the initial petting interval, then use the original finish-input
  // branch, including motion cleanup, instead of polling for another button.
  (0x01E54F04, 0xD0022198, 0x140000C2),
  (0x01E55110, 0x54000B64, 0xD503201F),
  (0x01E5518C, 0x54000244, 0xD503201F),
  (0x02D8158C, 0x52800021, 0x52800001),
  (0x01E546B4, 0x5400026B, 0xD503201F),
  (0x026F2624, 0x540000E9, 0xD503201F),
  (0x026F26BC, 0x54FFFBAB, 0xD503201F),
  (0x026F275C, 0x54FFF6A5, 0xD503201F),
  (0x026F2964, 0x39424668, 0x52800008),
  (0x026F29D0, 0x1E212000, 0x1E212020),
  (0x026F4714, 0x1E203820, 0x1E213820),
  (0x0278F4C4, 0x394A0268, 0x52800008),
  (0x0278FAF0, 0x54000825, 0xD503201F),
  (0x0278FAFC, 0x540007C5, 0xD503201F),
  (0x0278F9B4, 0x6B0A011F, 0x6B08011F),
  (0x0279039C, 0xB9426669, 0xB9425E69),
  (0x02791438, 0xB9426669, 0xB9425E69),
  (0x02DC5FC4, 0x6B0A011F, 0x6B08011F),
  (0x02DC6A5C, 0xB941FE69, 0xB9428669),
  (0x02DC7AF0, 0xB941FE69, 0xB9428669),
  (0x02DD10E0, 0x36000068, 0xD503201F),
  (0x02DD1C88, 0xB941EE69, 0xB941B269),
  (0x02DD2D28, 0xB941EE69, 0xB941B269),
  (0x0241F920, 0x4B130108, 0x2A1F03E8),
  // Ordinary gift menu/confirmation only; never change ItemData globally.
  (0x025245A8, 0x540002EB, 0xD503201F),
  (0x02524E6C, 0x54FFFC8B, 0xD503201F),
  // Keep owned counts unchanged as well as allowing zero-stock gifts.
  (0x025245F8, 0x940A358E, 0xD503201F),
];

/// Pin the surrounding instructions as well as each replaced instruction.
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x01E54244, [0x54001584, 0xAA1403E0, 0xAA1F03E1, 0x94078494]),
  (0x01E542C0, [0x540011E4, 0xF0022229, 0xF9403E88, 0xF940E129]),
  (0x01E543D4, [0x54000C64, 0xAA1403E0, 0xAA1F03E1, 0x94078418]),
  (0x01E54494, [0x54000285, 0x528000B5, 0x14000032, 0xB9001275]),
  (0x01E544E0, [0x54000124, 0x528000F5, 0x1400001F, 0x52800035]),
  (0x01E54F04, [0xD0022198, 0xF9420318, 0xF9400300, 0x3944CC08]),
  (0x01E55110, [0x54000B64, 0xAA1F03E0, 0x943269F6, 0x370003A0]),
  (0x01E5518C, [0x54000244, 0xAA1303E0, 0xAA1F03E1, 0xF8018C1F]),
  (0x02D8158C, [0x52800021, 0xAA1F03E2, 0xA9007FFF, 0x97C3BD7A]),
  (0x01E546B4, [0x5400026B, 0xF0022228, 0xF9400A94, 0xF9410108]),
  (0x026F2624, [0x540000E9, 0x3943A268, 0x340002A8, 0xA9424FF4]),
  (0x026F26BC, [0x54FFFBAB, 0x52A85908, 0x1E270108, 0x1E2703E0]),
  (0x026F275C, [0x54FFF6A5, 0xB9001A74, 0xB900727F, 0xAA1303E0]),
  (0x026F2964, [0x39424668, 0x340002E8, 0x9001DCA8, 0xF9413908]),
  (0x026F29D0, [0x1E212000, 0x54001509, 0x39424E68, 0x340012E8]),
  (0x026F4714, [0x1E203820, 0xF9406C08, 0xBD006000, 0xB900C91F]),
  (0x0278F4C4, [0x394A0268, 0x34000168, 0xF001D7F5, 0xF9414E74]),
  (0x0278FAF0, [0x54000825, 0x1E222821, 0x1E212000, 0x540007C5]),
  (0x0278FAFC, [0x540007C5, 0xD001DC54, 0x396DEA88, 0x370000C8]),
  (0x0278F9B4, [0x6B0A011F, 0x540000CA, 0xB9426268, 0x71000D1F]),
  (0x0279039C, [0xB9426669, 0x1B087D28, 0xB942526A, 0xB9426A6B]),
  (0x02791438, [0xB9426669, 0x1B087D28, 0xB942526A, 0xB9426A6B]),
  (0x02DC5FC4, [0x6B0A011F, 0x5400010A, 0xB941FA68, 0x7100051F]),
  (0x02DC6A5C, [0xB941FE69, 0x1B087D28, 0xB9427A6A, 0xB942026B]),
  (0x02DC7AF0, [0xB941FE69, 0x1B087D28, 0xB9427A6A, 0xB942026B]),
  (0x02DD10E0, [0x36000068, 0xAA1303E0, 0x97FFFC0A, 0xFD4063E8]),
  (0x02DD1C88, [0xB941EE69, 0x1B087D28, 0xB941DA6A, 0xB941F26B]),
  (0x02DD2D28, [0xB941EE69, 0x1B087D28, 0xB941DA6A, 0xB941F26B]),
  (0x0241F920, [0x4B130108, 0x0AA87D13, 0x7100011F, 0x5400020C]),
  (0x025245A8, [0x540002EB, 0x2A1F03E0, 0xAA1F03E1, 0x97DCBF33]),
  (0x02524E6C, [0x54FFFC8B, 0xF9400340, 0x977DB4DF, 0xAA1703E1]),
  (0x025245F8, [0x940A358E, 0x12000297, 0x14000002, 0x2A1F03F7]),
];

pub trait CodeMemory {
  fn read(&self, offset: usize) -> u32;
  fn write(&mut self, offset: usize, word: u32) -> bool;
}

/// Apply the target-version replacements; restore writes on an actual write failure.
pub fn apply(memory: &mut impl CodeMemory) -> bool {
  let previous: Vec<_> = PATCHES.iter().map(|&(offset, _, _)| memory.read(offset)).collect();
  for (index, &(offset, _, replacement)) in PATCHES.iter().enumerate() {
    if !memory.write(offset, replacement) || memory.read(offset) != replacement {
      for (position, &(offset, _, _)) in PATCHES[..=index].iter().enumerate().rev() {
        memory.write(offset, previous[position]);
      }
      return false;
    }
  }
  true
}

#[cfg(target_os = "switch")]
pub(crate) fn install() -> bool {
  // Import only Cobalt's verified memory writer. Pulling in Skyline's broader
  // patching module also links NRO-loader APIs that Cobalt 1.31 does not export.
  extern "C" {
    fn sky_memcpy(destination: *const core::ffi::c_void, source: *const u32, size: usize) -> u32;
  }
  struct NativeCode;

  impl CodeMemory for NativeCode {
    fn read(&self, offset: usize) -> u32 {
      unsafe { ((crate::native::base() + offset) as *const u32).read_volatile() }
    }

    fn write(&mut self, offset: usize, word: u32) -> bool {
      unsafe { sky_memcpy((crate::native::base() + offset) as *const _, &word, 4) == 0 }
    }
  }

  apply(&mut NativeCode)
}
