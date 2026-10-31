//! Joined allies used only by the polishing-character selector.

use crate::{
  cooking,
  meld::native::text,
  native::{base, list_count, object_at, Method, Object},
};
use core::ptr;

pub(crate) unsafe fn joined() -> Vec<(String, String, Object)> {
  unsafe {
    if crate::native::user_data().is_null() {
      return Vec::new();
    }
    let list_fn: unsafe extern "C" fn(Method) -> Object =
      core::mem::transmute(base() + cooking::UNIT_LIST);
    let pid_fn: unsafe extern "C" fn(Object, Method) -> Object =
      core::mem::transmute(base() + cooking::UNIT_PID);
    let name_fn: unsafe extern "C" fn(Object, Method) -> Object =
      core::mem::transmute(base() + cooking::UNIT_NAME);
    let list = list_fn(ptr::null());
    let Some(count) = list_count(list) else { return Vec::new() };
    let mut result: Vec<(String, String, Object)> = Vec::new();
    for index in 0..count {
      let Some(unit) = object_at(list, index).filter(|unit| !unit.is_null()) else { continue };
      let Some(pid) = text(pid_fn(unit, ptr::null())) else { continue };
      if result.iter().any(|entry| entry.0 == pid) {
        continue;
      }
      let name = text(name_fn(unit, ptr::null())).unwrap_or_else(|| pid.clone());
      result.push((pid, name, unit));
    }
    result
  }
}
