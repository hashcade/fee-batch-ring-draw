//! Independent multipliers at native EXP/SP reward settlement.

pub const MULTIPLIERS: [u32; 5] = [1, 2, 3, 5, 10];
/// Zero selects the current class level cap, not zero experience.
pub const EXP_MULTIPLIERS: [u32; 6] = [1, 2, 3, 5, 10, 0];
pub const CREATE_EXP: usize = 0x024E8C20;
pub const EXP_TICK: usize = 0x024E8370;
pub const ADD_EXP: usize = 0x01A39D40;
pub const GROWTH_PREPARE: usize = 0x01BE1260;
pub const LEVEL_UP: usize = 0x01A3A040;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x024E8C20, [0xA9BA7BFD, 0xF9000BFB, 0x910003FD, 0xA90267FA]),
  (0x024E8370, [0xA9BA7BFD, 0xA9016FFC, 0x910003FD, 0xA90267FA]),
  (0x01A39D40, [0xF940240B, 0x39422408, 0x3941C16A, 0x6B0A011F]),
  (0x01A39D50, [0x54000222, 0x39422809, 0x0B010129, 0x7101913F]),
  (0x01A39D60, [0x5400014B, 0x5290A3EA, 0x72AA3D6A, 0x9BAA7D2A]),
  (0x01A39D70, [0x11000508, 0x52800C8C, 0x39022408, 0xD365FD4A]),
  (0x01A39D80, [0x1B0CA549, 0x3941C16A, 0x6B28015F, 0x1A8903E8]),
  (0x01A39D90, [0x39022808, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01BE1260, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x01A3A040, [0xD10243FF, 0xA9037BFD, 0x9100C3FD, 0xA9046FFC]),
];

pub fn exp_capacity(level: u8, maximum: u8, exp: u8) -> i32 {
  ((i32::from(maximum) - i32::from(level)) * 100 - i32::from(exp)).max(0)
}

pub fn scaled(value: i32, multiplier: u32, capacity: i32) -> i32 {
  if value <= 0 || multiplier == 1 {
    value
  } else {
    value.saturating_mul(multiplier as i32).min(capacity.max(0))
  }
}

pub fn experience(value: i32, multiplier: u32, capacity: i32) -> i32 {
  if value > 0 && multiplier == 0 {
    capacity.max(0)
  } else {
    scaled(value, multiplier, capacity)
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    config,
    native::{base, field, Method, Object},
  };
  use core::{cell::Cell, ptr};

  std::thread_local! {
    static EXP_UNIT: Cell<Object> = const { Cell::new(ptr::null_mut()) };
  }

  fn in_settlement(unit: Object, action: impl FnOnce()) {
    let previous = EXP_UNIT.with(|value| value.replace(unit));
    action();
    EXP_UNIT.with(|value| value.set(previous));
  }

  pub(crate) fn install() {
    skyline::install_hooks!(create_exp, exp_tick, add_exp, growth_prepare);
  }

  unsafe fn item_reward(mut parent: Object) -> bool {
    unsafe {
      while !parent.is_null() {
        let class: Object = field(parent, 0);
        if !class.is_null() {
          let name = field(class, 0x10);
          if crate::menu_item::named(name, b"ItemUseSequence")
            || crate::menu_item::named(name, b"InventorySubMenu")
          {
            return true;
          }
        }
        parent = field(parent, 0x30);
      }
      false
    }
  }

  #[skyline::hook(offset = CREATE_EXP)]
  fn create_exp(parent: Object, unit: Object, exp: i32, sp: i32, method: Method) {
    let (exp_multiplier, sp_multiplier) = config::gain_multipliers();
    if unit.is_null()
      || (exp_multiplier == 1 && sp_multiplier == 1)
      || unsafe { item_reward(parent) }
    {
      return call_original!(parent, unit, exp, sp, method);
    }
    let (exp, sp) = unsafe {
      let job: Object = field(unit, 0x48);
      if job.is_null() {
        return call_original!(parent, unit, exp, sp, method);
      }
      (
        experience(
          exp,
          exp_multiplier,
          exp_capacity(field(unit, 0x89), field(job, 0x70), field(unit, 0x8A)),
        ),
        scaled(sp, sp_multiplier, 9999 - i32::from(field::<i16>(unit, 0x1BE))),
      )
    };
    // CreateBind's skip branch awards everything synchronously. Keep the
    // original history/descriptor path, splitting only its native AddExp call.
    in_settlement(unit, || call_original!(parent, unit, exp, sp, method));
  }

  #[skyline::hook(offset = EXP_TICK)]
  fn exp_tick(sequence: Object, method: Method) {
    if sequence.is_null() || unsafe { field::<i32>(sequence, 0x78) } <= 100 {
      return call_original!(sequence, method);
    }
    // The normal tick awards points individually; pressing skip mid-animation
    // instead awards the remaining amount at once. Both retain native flow.
    let unit = unsafe { field(sequence, 0x70) };
    in_settlement(unit, || call_original!(sequence, method));
  }

  #[skyline::hook(offset = ADD_EXP)]
  fn add_exp(unit: Object, exp: i32, method: Method) {
    if exp <= 100 || unit.is_null() || !EXP_UNIT.with(|value| value.get() == unit) {
      return call_original!(unit, exp, method);
    }
    // Native AddExp increments at most one level per call, even for 1000 EXP.
    // Do not multiply here: animated ticks, other calls and SP books stay native.
    let mut remaining = exp;
    while remaining > 100 {
      call_original!(unit, 100, method);
      remaining -= 100;
    }
    call_original!(unit, remaining, method);
  }

  #[skyline::hook(offset = GROWTH_PREPARE)]
  fn growth_prepare(sequence: Object, method: Method) {
    call_original!(sequence, method);
    if sequence.is_null() || config::gain_multipliers().0 == 1 {
      return;
    }
    unsafe {
      if field::<u8>(sequence, 0x94) != 0 {
        return; // Native class-change preview is not an EXP level-up.
      }
      let unit: Object = field(sequence, 0x80);
      let grow: Object = field(sequence, 0x88);
      if unit.is_null() || grow.is_null() {
        return;
      }
      let target: u8 = field(unit, 0x89);
      let level_up: unsafe extern "C" fn(Object, i32, Method) =
        core::mem::transmute(base() + LEVEL_UP);
      // Prepare clones the unit at its old level and grows it exactly once.
      // Complete the same native preview before Reflect copies it back, so
      // every earned level gets its growth, seeds and native skill handling.
      while field::<u8>(grow, 0x89) < target {
        level_up(grow, 2, ptr::null());
      }
    }
  }
}
