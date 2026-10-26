//! Finish ordinary Wyvern Ride at the selected difficulty's native SSS boundary.

#[cfg(target_os = "switch")]
pub(crate) mod native;

pub const MAIN: usize = 0x02ACA740;
pub const CALC_RANK: usize = 0x02ACC570;
pub const FINISH_RAIL: usize = 0x01D02670;
pub const SET_SCORE: usize = 0x02A649B0;
pub const GET_RETIRE: usize = 0x02A65040;

/// Config fields match the array selection in DragonRideGameSequence.CalcRankNum.
pub fn border_offset(level: i32) -> Option<usize> {
  match level {
    1 => Some(0x20),
    2 => Some(0x28),
    3 => Some(0x30),
    _ => None,
  }
}

/// The first descending boundary is SSS, not a theoretical perfect score.
pub fn sss_score(borders: &[i32]) -> Option<i32> {
  if !(2..=9).contains(&borders.len())
    || borders[0] <= 0
    || borders.iter().any(|&score| score < 0)
    || borders.windows(2).any(|pair| pair[0] <= pair[1])
  {
    return None;
  }
  Some(borders[0])
}

/// Entry points, field layouts, rank selection, and the native completion branch.
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02ACA740, [0xD10583FF, 0x6D0C3BEF, 0x6D0D33ED, 0x6D0E2BEB]),
  (0x02ACA8D4, [0xF84A8EC0, 0x97C8DBCE, 0xF9400340, 0x3944CC08]),
  (0x02ACC8F0, [0xB940BB08, 0x71000D1F, 0x54000560, 0x7100091F]),
  (0x01D01C24, [0x39428A68, 0x35000408, 0xF9404A60, 0x52800021]),
  (0x02ACC570, [0xA9BF7BFD, 0x910003FD, 0xF9407008, 0x91008109]),
  (0x02ACC580, [0x9100A10A, 0x71000C3F, 0x9100C108, 0x9A881128]),
  (0x02ACC590, [0x7100083F, 0x9A880148, 0xF9400108, 0xB9401901]),
  (0x02ACC5B8, [0x8B2AC90A, 0xB940214A, 0x6B02015F, 0x5400018D]),
  (0x02ACC5F4, [0x2A0903E0, 0xAA1F03E2, 0xA8C17BFD, 0x14204B78]),
  (0x01D02670, [0x52800028, 0x3906E408, 0xD65F03C0, 0xD503201F]),
  (0x02A649B0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02A65040, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xB001C5B4]),
  (0x02AC6220, [0x39428000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x02AC6240, [0x39445000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CFCAD0, [0x3946E400, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CFCB10, [0x3946EC00, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CFCCA0, [0x3947E000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CFCCC0, [0x3947E400, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x02ACB7B0, [0xF94002C8, 0x3946E508, 0x34000088, 0xAA1303E0]),
  (0x02ACB7BC, [0xAA1303E0, 0xAA1F03E1, 0x97F54877, 0x6D4F23E9]),
];
