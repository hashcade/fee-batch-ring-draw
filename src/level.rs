//! Read-only current/cumulative level labels in the native unit status screen.

pub const SET_LEVEL: usize = 0x01C66980;
pub const ENHANCED_LEVEL: usize = 0x01A33AA0;
pub const SET_TEXT: usize = 0x0290F0A0;

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x01C66980, [0xD101C3FF, 0xA9017BFD, 0x910043FD, 0xA9026FFC]),
  (0x01A33AA0, [0xD10283FF, 0xA9047BFD, 0x910103FD, 0xA9056FFC]),
  (0x0290F0A0, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x01C66A10, [0xF94026A8, 0xD10013A1, 0x394226BB, 0xF940C678]),
  (0x01A54FA0, [0x39C56000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
];

pub fn cumulative(current: u8, internal: i8) -> Option<u16> {
  if !(1..=99).contains(&current) || internal < 0 {
    return None;
  }
  Some(u16::from(current) + internal as u16)
}

pub fn title(language: i32) -> &'static str {
  match language {
    0 => "Lv./累計",
    2 | 5 => "Niv./Total",
    3 | 6 => "Nv./Total",
    7 => "Lv./Gesamt",
    8 => "Lv./Totale",
    9 => "等級/累計",
    10 => "等级/累计",
    11 => "레벨/누적",
    _ => "Lv./Total",
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    meld::native::{new_text, text_language},
    native::{base, field, Method, Object},
  };

  pub(crate) fn install() {
    skyline::install_hooks!(set_level);
  }

  #[skyline::hook(offset = SET_LEVEL)]
  fn set_level(setter: Object, unit: Object, original_unit: Object, method: Method) {
    call_original!(setter, unit, original_unit, method);
    if setter.is_null() || unit.is_null() || original_unit.is_null() {
      return;
    }
    unsafe {
      let Some(total) = cumulative(field(original_unit, 0x89), field(original_unit, 0x158)) else {
        return;
      };
      let parameter: Object = field(setter, 0x188);
      if parameter.is_null() {
        return;
      }
      let title_text: Object = field(parameter, 0x20);
      let value_text: Object = field(parameter, 0x28);
      if title_text.is_null() || value_text.is_null() {
        return;
      }
      let enhanced: unsafe extern "C" fn(Object, Method) -> i32 =
        core::mem::transmute(base() + ENHANCED_LEVEL);
      let set_text: unsafe extern "C" fn(Object, Object, Method) =
        core::mem::transmute(base() + SET_TEXT);
      let displayed = enhanced(unit, core::ptr::null());
      if !(1..=999).contains(&displayed) {
        return;
      }
      let heading = new_text(title(text_language()));
      let value = new_text(&format!("{displayed}/{total}"));
      if !heading.is_null() && !value.is_null() {
        // Only replace text after the complete native setter. Preserve its
        // level-limit colors, enhancement arrows, EXP bar and capabilities.
        set_text(title_text, heading, core::ptr::null());
        set_text(value_text, value, core::ptr::null());
      }
    }
  }
}
