//! Arrival/menu-triggered native cleaning settlement, without a rubbing scene.

use crate::{
  cleaning::{GET_DIRTY, GET_STATE, SET_DIRTY},
  config,
  native::{base, field, list_count, metadata, object_at, Method, Object},
};
use core::ptr;
use std::collections::BTreeSet;

pub const GOD_POOL: usize = 0x001FD360;
pub const GOD_POOL_METHOD: usize = 0x0628DD48;
pub const HAS_BOND: usize = 0x02340600;
pub const ADD_BOND: usize = 0x02340700;
pub const GOD_DATA: usize = 0x0233EB20;
pub const RECORD_POLISH: usize = 0x027CA680;
pub const RECORD_GOD_POLISH: usize = 0x027CCA60;

const NONE: Method = ptr::null();

macro_rules! native {
  ($address:expr, $signature:ty) => {
    core::mem::transmute::<usize, $signature>(base() + $address)
  };
}

pub(crate) unsafe fn settle() {
  unsafe {
    let method = metadata(GOD_POOL_METHOD);
    if method.is_null() {
      return;
    }
    let pool = native!(GOD_POOL, unsafe extern "C" fn(Method) -> Object)(method);
    if pool.is_null() {
      return;
    }
    // Native GodPool.GetEnumerator (0x03207830) enumerates the used list.
    let rings: Object = field(pool, 0x28);
    let Some(count) = list_count(rings) else { return };
    let actor_pid = config::polish_actor();
    let Some((_, _, actor)) =
      crate::roster::joined().into_iter().find(|entry| entry.0 == actor_pid)
    else {
      return;
    };
    let mut seen = BTreeSet::new();
    for index in 0..count {
      let Some(god) = object_at(rings, index).filter(|god| !god.is_null()) else { continue };
      if !seen.insert(god as usize)
        || field::<u8>(god, 0x4A) != 0
        || field::<u8>(god, 0x4B) != 0
        || !matches!(
          native!(GET_STATE, unsafe extern "C" fn(Object, Method) -> i32)(god, NONE),
          0 | 3
        )
        || native!(GET_DIRTY, unsafe extern "C" fn(Object, Method) -> u8)(god, NONE) == 0
      {
        continue;
      }
      native!(SET_DIRTY, unsafe extern "C" fn(Object, i32, Method))(god, 0, NONE);
      if native!(GET_DIRTY, unsafe extern "C" fn(Object, Method) -> u8)(god, NONE) != 0 {
        continue;
      }
      if native!(HAS_BOND, unsafe extern "C" fn(Object, Object, Method) -> bool)(god, actor, NONE) {
        // The normal finish/Weak reward is three points. Native AddBondExp
        // retains the real bond caps; the outer helper only adds presentation.
        native!(ADD_BOND, unsafe extern "C" fn(Object, Object, i32, Method) -> bool)(
          god, actor, 3, NONE,
        );
      }
      native!(RECORD_POLISH, unsafe extern "C" fn(Method))(NONE);
      let data = native!(GOD_DATA, unsafe extern "C" fn(Object, Method) -> Object)(god, NONE);
      if !data.is_null() {
        let gid: Object = field(data, 0x20);
        if !gid.is_null() {
          native!(RECORD_GOD_POLISH, unsafe extern "C" fn(Object, Method))(gid, NONE);
        }
      }
    }
  }
}
