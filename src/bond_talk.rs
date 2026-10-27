//! Use the native dialogue skip branch only inside paid Emblem training.

pub const TALK_TICK: usize = 0x020C79B0;
pub const JUMP_LABEL: usize = 0x0281E6F0;
pub const GET_LABEL: usize = 0x0281EB10;
pub const DIALOG_CLASS: usize = 0x062A3CB8;
pub const COMBAT_CLASS: usize = 0x06291678;
pub const ORDER_CLASS: usize = 0x062917C0;

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x020C79B0, [0xFC1A0FE8, 0xA9017BFD, 0x910043FD, 0xA90267FA]),
  (0x0281E6F0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x0281EB10, [0xA9BF7BFD, 0x910003FD, 0xB9401808, 0x37F801E8]),
  (0x020C85E0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02097358, [0xF9465ED6, 0xAA0703F4, 0xAA0603F5, 0x2A0403F7]),
  (0x01CA3034, [0xF9433E94, 0xD0022F7C, 0xF9466A73, 0xF943FB7B]),
  (0x01CA5370, [0xF943E294, 0xB0022F13, 0xF9466AF7, 0xF943FB5A]),
  (0x02097E2C, [0xAA1403E0, 0xAA1503E1, 0xAA1F03E2, 0x94112666]),
  (0x020C7AEC, [0xAA1F03E0, 0x9406FA8C, 0x360001A0, 0x528000C8]),
  (0x02097AF0, [0xF9403C08, 0xB4000048, 0x14000006, 0xF9404408]),
  (0x02097E70, [0xA9BC7BFD, 0xA9015FF8, 0x910003FD, 0xA90257F6]),
  (0x020C86E0, [0xD10243FF, 0xA9037BFD, 0x9100C3FD, 0xF90023FB]),
  (0x02098280, [0xF9405A60, 0xB4000060, 0xAA1F03E1, 0x944D7009]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::native::{base, field, metadata, Method, Object};

  pub(crate) fn install() {
    skyline::install_hooks!(talk_tick);
  }

  unsafe fn is_class(object: Object, class: Object) -> bool {
    !object.is_null() && !class.is_null() && unsafe { field::<Object>(object, 0) == class }
  }

  unsafe fn is_training_dialogue(talk: Object) -> bool {
    if talk.is_null() {
      return false;
    }
    unsafe {
      let dialog_class = metadata(DIALOG_CLASS);
      let mut ancestor = field::<Object>(talk, 0x30);
      // Talk -> Lua/event children -> MyRoomRelianceCallSequence. Keep the
      // native script running; only skip its message presentation.
      for _ in 0..8 {
        if ancestor.is_null() {
          return false;
        }
        if is_class(ancestor, dialog_class) {
          if field::<Object>(ancestor, 0x70).is_null()
            || !field::<Object>(ancestor, 0x78).is_null()
            || field::<Object>(ancestor, 0x88).is_null()
          {
            return false;
          }
          let combat: Object = field(ancestor, 0x30);
          if !is_class(combat, metadata(COMBAT_CLASS)) {
            return false;
          }
          let order: Object = field(combat, 0x30);
          return is_class(order, metadata(ORDER_CLASS))
            && field::<Object>(order, 0x38) == combat
            && field::<u8>(order, 0x88) == 1
            && field::<u8>(order, 0x89) == 0
            && field::<u8>(combat, 0x9C) == 1
            && field::<u8>(combat, 0x9D) == 0
            && crate::arena::selection(true, false, field(order, 0x8C), field(combat, 0x98));
        }
        ancestor = field(ancestor, 0x30);
      }
    }
    false
  }

  #[skyline::hook(offset = TALK_TICK)]
  fn talk_tick(talk: Object, method: Method) {
    unsafe {
      if is_training_dialogue(talk) {
        let get_label: unsafe extern "C" fn(Object, Method) -> i32 =
          core::mem::transmute(base() + GET_LABEL);
        if get_label(talk, core::ptr::null()) == 3 {
          // Match Tick's native GameSkip branch: Main (3) -> Skip (6).
          // Skip processes remaining tags, then Close/Finish return to Lua.
          // Do not set a global skip/input flag or bypass event callbacks.
          let jump: unsafe extern "C" fn(Object, i32, Method) =
            core::mem::transmute(base() + JUMP_LABEL);
          jump(talk, 6, core::ptr::null());
          return;
        }
      }
    }
    call_original!(talk, method);
  }
}
