//! Game-thread one-shot edits; no hooks, refills or save/config variables.

use super::*;
use crate::{
  cooking,
  meld::native::{new_text, text},
  native::{base, list_count, metadata, object_at, Method, Object},
};
use core::{cell::RefCell, ptr};

const NONE: Method = ptr::null();
pub const GET_GOLD: usize = 0x0250E440;
pub const SET_GOLD: usize = 0x0250E450;
pub const GET_BONDS: usize = 0x0250ECB0;
pub const SET_BONDS: usize = 0x0250ECC0;
pub const GET_INVENTORY: usize = 0x027B1920;
pub const SET_INVENTORY: usize = 0x027B1A30;
pub const GET_SP: usize = 0x01A54D50;
pub const SET_SP: usize = 0x01A0E090;
pub const PAD_IS_TRIGGER: usize = 0x01F23010;

#[derive(Default)]
struct Editor {
  targets: [Option<i32>; VALUE_COUNT],
  pid: Option<String>,
  sp_item: usize,
}

std::thread_local! { static EDITOR: RefCell<Editor> = RefCell::new(Editor::default()); }

macro_rules! native {
  ($address:expr, $signature:ty) => {
    core::mem::transmute::<usize, $signature>(base() + $address)
  };
}

pub(crate) fn reset() {
  EDITOR.with(|state| *state.borrow_mut() = Editor::default());
}

pub(crate) fn attach(item: Object, kind: usize) {
  if kind == FIRST_VALUE_KIND {
    EDITOR.with(|state| state.borrow_mut().sp_item = item as usize);
  }
}

unsafe fn roster() -> Vec<(String, String, Object)> {
  unsafe {
    if crate::native::user_data().is_null() {
      return Vec::new();
    }
    let list = native!(cooking::UNIT_LIST, unsafe extern "C" fn(Method) -> Object)(NONE);
    let Some(count) = list_count(list) else { return Vec::new() };
    let mut result: Vec<(String, String, Object)> = Vec::new();
    for index in 0..count {
      let Some(unit) = object_at(list, index) else { continue };
      let pid =
        native!(cooking::UNIT_PID, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE);
      let Some(pid) = text(pid) else { continue };
      if result.iter().any(|entry| entry.0 == pid) {
        continue;
      }
      let name =
        native!(cooking::UNIT_NAME, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE);
      result.push((pid.clone(), text(name).unwrap_or(pid), unit));
    }
    result
  }
}

unsafe fn selected_units() -> Vec<Object> {
  let pid = EDITOR.with(|state| state.borrow().pid.clone());
  unsafe { roster() }
    .into_iter()
    .filter(|entry| pid.as_ref().map_or(true, |pid| *pid == entry.0))
    .map(|entry| entry.2)
    .collect()
}

unsafe fn item_data(index: usize) -> Object {
  unsafe {
    let Some(iid) = RESOURCES[index].iid else { return ptr::null_mut() };
    let method = metadata(0x06287888);
    if method.is_null() {
      return ptr::null_mut();
    }
    native!(crate::collector::profile::ITEM_GET, unsafe extern "C" fn(Object, Method) -> Object)(
      new_text(iid),
      method,
    )
  }
}

unsafe fn actual(index: usize) -> Option<i32> {
  unsafe {
    let user = crate::native::user_data();
    if user.is_null() {
      return None;
    }
    match index {
      0 => common_value(
        selected_units()
          .into_iter()
          .map(|unit| native!(GET_SP, unsafe extern "C" fn(Object, Method) -> i32)(unit, NONE)),
      ),
      1 => Some(native!(GET_GOLD, unsafe extern "C" fn(Object, Method) -> i32)(user, NONE)),
      2 => Some(native!(GET_BONDS, unsafe extern "C" fn(Object, Method) -> i32)(user, NONE)),
      _ => {
        let item = item_data(index);
        (!item.is_null())
          .then(|| native!(GET_INVENTORY, unsafe extern "C" fn(Object, Method) -> i32)(item, NONE))
      }
    }
  }
}

pub(crate) unsafe fn enabled(kind: usize) -> bool {
  unsafe {
    if crate::native::user_data().is_null() {
      return false;
    }
    if kind <= FIRST_VALUE_KIND {
      return !selected_units().is_empty();
    }
    kind < FIRST_VALUE_KIND + 3 || !item_data(kind - FIRST_VALUE_KIND).is_null()
  }
}

pub(crate) unsafe fn label(language: i32, kind: usize) -> String {
  unsafe {
    if kind == TARGET_KIND {
      return EDITOR.with(|state| {
        let pid = state.borrow().pid.clone();
        match pid {
          None => all_label(language).to_owned(),
          Some(pid) => roster()
            .into_iter()
            .find(|entry| entry.0 == pid)
            .map(|entry| entry.1)
            .unwrap_or_else(|| "—".to_owned()),
        }
      });
    }
    let index = kind - FIRST_VALUE_KIND;
    let value = EDITOR.with(|state| state.borrow().targets[index]).or_else(|| actual(index));
    value.map(number).unwrap_or_else(|| {
      if index == 0 && enabled(kind) {
        mixed_label(language).to_owned()
      } else {
        "—".to_owned()
      }
    })
  }
}

unsafe fn refresh(item: Object) {
  if !item.is_null() {
    unsafe { crate::settings::native::refresh(item) };
  }
}

pub(crate) unsafe fn change(item: Object, kind: usize) -> i32 {
  unsafe {
    // ConfigMenu routes settings input through CustomCall. Match Cobalt's
    // own command items: inspect native A trigger here, not item ACall.
    if kind >= FIRST_VALUE_KIND
      && native!(PAD_IS_TRIGGER, unsafe extern "C" fn(u64, Method) -> bool)(1, NONE)
    {
      return apply(item, kind);
    }
    let change = native!(
      crate::settings::CHANGE_NUMBER,
      unsafe extern "C" fn(i32, i32, i32, i32, Method) -> i32
    );
    // Idle settings frames must not enumerate allies or touch inventories.
    // Use the native repeat handler for direction, not a wrapping quantity.
    let direction = change(1, 0, 2, 1, NONE) - 1;
    if direction == 0 || !enabled(kind) {
      return 0;
    }
    if kind == TARGET_KIND {
      let units = roster();
      let pid = EDITOR.with(|state| state.borrow().pid.clone());
      let current =
        pid.and_then(|pid| units.iter().position(|entry| entry.0 == pid)).map_or(0, |i| i + 1);
      let next = (current as i32 + direction).rem_euclid(units.len() as i32 + 1) as usize;
      let sp_item = EDITOR.with(|state| {
        let mut state = state.borrow_mut();
        state.pid = (next > 0).then(|| units[next - 1].0.clone());
        state.targets[0] = None;
        state.sp_item as Object
      });
      refresh(sp_item);
    } else {
      let index = kind - FIRST_VALUE_KIND;
      let pending = EDITOR.with(|state| state.borrow().targets[index]);
      let current = pending.or_else(|| actual(index));
      let next = adjusted(index, current.unwrap_or(0), direction);
      if current == Some(next) {
        return 0;
      }
      EDITOR.with(|state| state.borrow_mut().targets[index] = Some(next));
    }
    refresh(item);
    4096
  }
}

pub(crate) unsafe fn apply(item: Object, kind: usize) -> i32 {
  unsafe {
    if kind == TARGET_KIND || !enabled(kind) {
      return 0;
    }
    let index = kind - FIRST_VALUE_KIND;
    let Some(value) = EDITOR.with(|state| state.borrow().targets[index]) else { return 0 };
    match index {
      0 => {
        for unit in selected_units() {
          native!(SET_SP, unsafe extern "C" fn(Object, i32, Method))(unit, value, NONE);
        }
      }
      1 | 2 => {
        let user = crate::native::user_data();
        native!(
          if index == 1 { SET_GOLD } else { SET_BONDS },
          unsafe extern "C" fn(Object, i32, Method)
        )(user, value, NONE);
      }
      _ => native!(SET_INVENTORY, unsafe extern "C" fn(Object, i32, Method))(
        item_data(index),
        value,
        NONE,
      ),
    }
    EDITOR.with(|state| state.borrow_mut().targets[index] = None);
    refresh(item);
    128 // Decide sound only; keep the settings menu open.
  }
}
