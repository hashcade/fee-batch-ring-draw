//! Event-driven use limits and reversible settings patches. No frame polling.

use crate::{activities::CodeMemory, config::Settings};

pub const REWIND_MAX: usize = 0x02707DD0;
pub const ARENA_PARAMETER_KEY: usize = 0x06295CA8;
pub const ARENA_USED_KEY: usize = 0x06291A58;

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02707DD0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x0270DB44, [0x11000508, 0xB9003668, 0xA9434FF4, 0xF9400BF7]),
  (0x02677284, [0xB9402808, 0x71000D1F, 0x54000341, 0xB001E074]),
  (0x01E40CF8, [0xB9402808, 0x71000D1F, 0x54000321, 0x90022234]),
  (0x01E40E88, [0xB9402808, 0x71000D1F, 0x54000321, 0x90022234]),
  (0x01CAA7A4, [0x1A9706D4, 0xD0022ED7, 0xF9402815, 0xF94392F7]),
  (0x01D70E50, [0x94143F74, 0x4B160001, 0xAA1303E0, 0x94000169]),
  (0x021176A0, [0x9405A560, 0x2A0003F8, 0xF9400280, 0x3944CC08]),
];

pub fn patches(settings: &Settings) -> [(usize, u32); 5] {
  let save = if settings.battle_save { 0x52800028 } else { 0xB9402808 };
  [
    (0x0270DB44, if settings.rewind_count == 0 { 0x11000108 } else { 0x11000508 }),
    (0x02677284, save),
    (0x01E40CF8, save),
    (0x01E40E88, save),
    // Keep the already-used value, including an absent-key zero; do not refill it.
    (0x01CAA7A4, if settings.arena_count == 0 { 0x2A1703F4 } else { 0x1A9706D4 }),
  ]
}

pub fn apply(memory: &mut impl CodeMemory, settings: &Settings) -> bool {
  let desired = patches(settings);
  let previous = desired.map(|(address, _)| memory.read(address));
  for (index, &(address, word)) in desired.iter().enumerate() {
    if memory.read(address) == word {
      continue;
    }
    if !memory.write(address, word) || memory.read(address) != word {
      for position in (0..=index).rev() {
        memory.write(desired[position].0, previous[position]);
      }
      return false;
    }
  }
  true
}

pub fn rewind_limit(count: u32) -> i32 {
  if count == 0 {
    -1
  } else {
    count as i32
  }
}

pub fn arena_limit(count: u32, used: i32, native_limit: i32) -> i32 {
  if count == 0 {
    used.max(0).saturating_add(native_limit.max(1))
  } else {
    count as i32
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    config,
    native::{base, field, metadata, user_data, Method, Object},
    profile,
  };
  use core::ptr;

  pub(crate) fn install() {
    apply(&config::current());
    skyline::install_hooks!(rewind_max, parameter);
  }

  pub(crate) fn apply(settings: &Settings) -> bool {
    extern "C" {
      fn sky_memcpy(destination: *const core::ffi::c_void, source: *const u32, size: usize) -> u32;
    }
    struct Code;
    impl CodeMemory for Code {
      fn read(&self, offset: usize) -> u32 {
        unsafe { ((base() + offset) as *const u32).read_volatile() }
      }
      fn write(&mut self, offset: usize, word: u32) -> bool {
        unsafe { sky_memcpy((base() + offset) as *const _, &word, 4) == 0 }
      }
    }
    super::apply(&mut Code, settings)
  }

  #[skyline::hook(offset = REWIND_MAX)]
  fn rewind_max(_: Object, _: Method) -> i32 {
    // The native negative sentinel already hides the finite remaining-use UI.
    rewind_limit(config::rewind_count())
  }

  #[skyline::hook(offset = profile::PARAMETER)]
  fn parameter(key: Object, method: Method) -> i32 {
    let result = call_original!(key, method);
    // Compare interned metadata identity only: no string parsing/allocation on
    // unrelated parameter reads, including those during battle.
    let cell = unsafe { ((base() + ARENA_PARAMETER_KEY) as *const Object).read() };
    if cell.is_null() || key != unsafe { field::<Object>(cell, 0) } {
      return result;
    }
    let count = config::arena_count();
    if count != 0 {
      return count as i32;
    }
    unsafe {
      let user = user_data();
      if user.is_null() {
        return result;
      }
      let variables: Object = field(user, 0x50);
      let key = metadata(ARENA_USED_KEY);
      if variables.is_null() || key.is_null() {
        return result;
      }
      let read: unsafe extern "C" fn(Object, Object, Method) -> i32 =
        core::mem::transmute(base() + crate::facilities::VARIABLE_NUMBER);
      arena_limit(0, read(variables, key, ptr::null()), result)
    }
  }
}
