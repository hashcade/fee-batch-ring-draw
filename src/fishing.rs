//! Nondecreasing fishing uses, independent of fast bite/landing patches.

pub const DECREASE: usize = 0x026FFF90;
pub const SET_NUMBER: usize = 0x0251F290;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x026FFF90, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xA90257F6]),
  (0x0251F290, [0xD10143FF, 0xA9017BFD, 0x910043FD, 0xF90013F7]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    config::{self, Feature},
    native::{base, field, user_data, Method, Object},
  };
  use core::ptr;

  pub(crate) fn install() {
    skyline::install_hooks!(decrease);
  }

  unsafe fn read_remaining(variables: Object) -> i32 {
    unsafe {
      let get: unsafe extern "C" fn(Object, Object, Method) -> i32 =
        core::mem::transmute(base() + crate::facilities::VARIABLE_NUMBER);
      get(variables, crate::meld::native::new_text("G_Fishing_PlayCount"), ptr::null())
    }
  }

  pub(crate) unsafe fn remaining() -> i32 {
    unsafe {
      let user = user_data();
      if user.is_null() {
        return 0;
      }
      let variables: Object = field(user, 0x50);
      if variables.is_null() {
        return 0;
      }
      read_remaining(variables)
    }
  }

  pub(crate) fn ensure_allowance() {
    if !config::enabled(Feature::FishingUnlimited) {
      return;
    }
    unsafe {
      let user = user_data();
      if user.is_null() || field::<i32>(user, 0x28) != 4 {
        return;
      }
      let variables: Object = field(user, 0x50);
      if variables.is_null() || read_remaining(variables) > 0 {
        return;
      }
      // An exhausted session needs one native remaining use to enter from
      // both the original fishing dialogue and the facility shortcut.
      let set: unsafe extern "C" fn(Object, Object, i32, Method) =
        core::mem::transmute(base() + SET_NUMBER);
      set(variables, crate::meld::native::new_text("G_Fishing_PlayCount"), 1, ptr::null());
    }
  }

  #[skyline::hook(offset = DECREASE)]
  fn decrease(sequence: Object, method: Method) {
    if !config::enabled(Feature::FishingUnlimited) {
      call_original!(sequence, method);
    }
  }
}
