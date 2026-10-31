//! Drive eligible native support menu actions, not rank or completion writes.

pub const CREATE_UNIT_MENU: usize = 0x02393B40;
pub const CREATE_PAIR_MENU: usize = 0x023970A0;
pub const PAIR_LIST: usize = 0x023971A0;
pub const CREATE_SEQUENCE: usize = 0x02396290;
pub const IS_BOND: usize = 0x02396050;
pub const RELATION: usize = 0x01C5A040;
pub const CAN_ADVANCE: usize = 0x01C5A930;
pub const SET_BOND: usize = 0x023960C0;
pub const SET_READY_ONLY: usize = 0x023961B0;
pub const READY_ONLY: usize = 0x02396140;
pub const HUB_INIT: usize = 0x023EBCA0;
pub const HUB_TICK: usize = 0x023F26D0;
pub const HUB_IS_MAIN: usize = 0x023E6BD0;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02393B40, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x023970A0, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x023971A0, [0xD10303FF, 0xA9067BFD, 0x910183FD, 0xA9076FFC]),
  (0x02396290, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x02396050, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD001FC14]),
  (0x01C5A040, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01C5A930, [0xB40000A0, 0xB4000081, 0xF9402000, 0xF9402021]),
  (0x023960C0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x023961B0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02396140, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD001FC14]),
  (0x023EBCA0, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x023F26D0, [0xFC1B0FE8, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x023E6BD0, [0xA9BE7BFD, 0xF9000BF3, 0x910003FD, 0xAA1F03E1]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    config,
    menu_item::VTABLE,
    native::{base, field, list_count, object_at, Method, Object},
  };
  use core::{
    cell::{Cell, RefCell},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
  };
  use std::collections::BTreeSet;
  const NONE: Method = ptr::null();
  static UNIT_CLASS: AtomicUsize = AtomicUsize::new(0);
  static PAIR_CLASS: AtomicUsize = AtomicUsize::new(0);
  static SEQUENCE_CLASS: AtomicUsize = AtomicUsize::new(0);
  static OWNER: AtomicUsize = AtomicUsize::new(0);

  // Keys are identifiers only; managed objects remain rooted by the native
  // menus. One attempt per pair per opening also bounds failures/cancellation.
  std::thread_local! {
    static ATTEMPTED: RefCell<BTreeSet<(usize, usize)>> = const { RefCell::new(BTreeSet::new()) };
    static PREVIOUS_FLAGS: RefCell<Option<(bool, bool, bool)>> = const { RefCell::new(None) };
    static ENTERED: Cell<usize> = const { Cell::new(0) };
  }

  macro_rules! native {
    ($address:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $address)
    };
  }

  pub(crate) fn install() {
    skyline::install_hooks!(
      create_sequence,
      create_unit_menu,
      create_pair_menu,
      hub_init,
      hub_tick
    );
  }

  #[skyline::hook(offset = HUB_INIT)]
  fn hub_init(hub: Object, method: Method) {
    // Reset per visit, including when the managed allocator reuses an address.
    ENTERED.with(|entered| entered.set(0));
    call_original!(hub, method);
    crate::fishing::native::ensure_allowance();
  }

  #[skyline::hook(offset = HUB_TICK)]
  fn hub_tick(hub: Object, method: Method) {
    call_original!(hub, method);
    if hub.is_null()
      || ENTERED.with(|entered| entered.get() == hub as usize)
      || !config::auto_support()
    {
      return;
    }
    unsafe {
      let user = crate::native::user_data();
      if user.is_null()
        || field::<i32>(user, 0x28) != 4
        || !field::<Object>(hub, 0x38).is_null()
        || !native!(HUB_IS_MAIN, unsafe extern "C" fn(Object, Method) -> bool)(hub, NONE)
      {
        return;
      }
      // Main is reached after scene entry/scripts. Never interrupt loading,
      // a story child, battle logic or player interaction. Scan once per visit.
      ENTERED.with(|entered| entered.set(hub as usize));
      begin(hub, false);
    }
  }

  #[skyline::hook(offset = CREATE_SEQUENCE)]
  fn create_sequence(parent: Object, method: Method) {
    // A new opening always owns a fresh attempt set, even if managed addresses
    // are reused after disposal. Rebuilding its unit menu does not reset it.
    OWNER.store(0, Ordering::Relaxed);
    ATTEMPTED.with(|set| set.borrow_mut().clear());
    call_original!(parent, method);
  }

  pub(crate) unsafe fn on_hub_menu(parent: Object) {
    if parent.is_null() || !config::auto_support() {
      return;
    }
    unsafe {
      let user = crate::native::user_data();
      if user.is_null() || field::<i32>(user, 0x28) != 4 {
        return;
      }
      let menu: Object = field(parent, 0x38);
      if menu.is_null() || !field::<Object>(menu, 0x38).is_null() {
        return;
      }
      begin(menu, true);
    }
  }

  unsafe fn begin(parent: Object, from_menu: bool) {
    unsafe {
      if PREVIOUS_FLAGS.with(|flags| flags.borrow().is_some()) {
        return;
      }
      let units = native!(crate::cooking::UNIT_LIST, unsafe extern "C" fn(Method) -> Object)(NONE);
      let Some(count) = list_count(units) else { return };
      let pending = (0..count).any(|i| {
        (i + 1..count).any(|j| {
          let (Some(a), Some(b)) = (object_at(units, i), object_at(units, j)) else { return false };
          if a.is_null() || b.is_null() {
            return false;
          }
          let data =
            native!(RELATION, unsafe extern "C" fn(Object, Object, Method) -> Object)(a, b, NONE);
          !data.is_null()
            && (0..=2).contains(&field::<i32>(data, 0x18))
            && native!(CAN_ADVANCE, unsafe extern "C" fn(Object, Object, Method) -> bool)(
              a, b, NONE,
            )
        })
      });
      if !pending {
        return;
      }
      let old_bond = native!(IS_BOND, unsafe extern "C" fn(Method) -> bool)(NONE);
      let old_ready = native!(READY_ONLY, unsafe extern "C" fn(Method) -> bool)(NONE);
      native!(SET_BOND, unsafe extern "C" fn(bool, Method))(false, NONE);
      native!(SET_READY_ONLY, unsafe extern "C" fn(bool, Method))(true, NONE);
      native!(CREATE_SEQUENCE, unsafe extern "C" fn(Object, Method))(parent, NONE);
      let child: Object = field(parent, 0x38);
      if child.is_null() {
        native!(SET_BOND, unsafe extern "C" fn(bool, Method))(old_bond, NONE);
        native!(SET_READY_ONLY, unsafe extern "C" fn(bool, Method))(old_ready, NONE);
        return;
      }
      if !crate::menu_item::override_class(
        child,
        &SEQUENCE_CLASS,
        &[(10, finished as *const () as usize)],
      ) {
        native!(SET_BOND, unsafe extern "C" fn(bool, Method))(old_bond, NONE);
        native!(SET_READY_ONLY, unsafe extern "C" fn(bool, Method))(old_ready, NONE);
        return;
      }
      PREVIOUS_FLAGS.with(|flags| *flags.borrow_mut() = Some((old_bond, old_ready, from_menu)));
      if from_menu {
        let class: Object = field(parent, 0);
        let close: unsafe extern "C" fn(Object, Method) =
          core::mem::transmute(field::<usize>(class, VTABLE + 23 * 16));
        close(parent, field(class, VTABLE + 23 * 16 + 8));
      }
    }
  }

  unsafe extern "C" fn finished(sequence: Object, method: Method) {
    unsafe {
      let menu: Object = field(sequence, 0x30);
      let original: unsafe extern "C" fn(Object, Method) =
        core::mem::transmute(field::<usize>(method as Object, 0));
      original(sequence, method);
      if let Some((bond, ready, from_menu)) = PREVIOUS_FLAGS.with(|flags| flags.borrow_mut().take())
      {
        native!(SET_BOND, unsafe extern "C" fn(bool, Method))(bond, NONE);
        native!(SET_READY_ONLY, unsafe extern "C" fn(bool, Method))(ready, NONE);
        if from_menu {
          crate::facilities::native::restore(menu);
        }
      }
      OWNER.store(0, Ordering::Relaxed);
      ATTEMPTED.with(|set| set.borrow_mut().clear());
    }
  }

  unsafe fn active() -> bool {
    if !config::auto_support() {
      return false;
    }
    unsafe {
      let user = crate::native::user_data();
      !user.is_null()
        && field::<i32>(user, 0x28) == 4
        && !native!(IS_BOND, unsafe extern "C" fn(Method) -> bool)(NONE)
        && native!(READY_ONLY, unsafe extern "C" fn(Method) -> bool)(NONE)
    }
  }

  unsafe fn key(item: Object) -> Option<(usize, usize)> {
    if item.is_null() {
      return None;
    }
    unsafe {
      let a: Object = field(item, 0x68);
      let b: Object = field(item, 0x70);
      if a.is_null()
        || b.is_null()
        || !field::<Object>(item, 0x78).is_null()
        || field::<u8>(item, 0x81) == 0
      {
        return None;
      }
      let data =
        native!(RELATION, unsafe extern "C" fn(Object, Object, Method) -> Object)(a, b, NONE);
      if data.is_null()
        || !(0..=2).contains(&field::<i32>(data, 0x18))
        || !native!(CAN_ADVANCE, unsafe extern "C" fn(Object, Object, Method) -> bool)(a, b, NONE)
      {
        return None;
      }
      Some((a as usize, b as usize).min((b as usize, a as usize)))
    }
  }

  unsafe fn ready(item: Object) -> bool {
    unsafe { key(item) }.is_some_and(|key| ATTEMPTED.with(|set| !set.borrow().contains(&key)))
  }

  unsafe fn patch_child(parent: Object, cache: &AtomicUsize) {
    if parent.is_null() {
      return;
    }
    unsafe {
      let menu: Object = field(parent, 0x38);
      if menu.is_null() {
        return;
      }
      crate::menu_item::override_class(menu, cache, &[(49, dispatch as *const () as usize)]);
    }
  }

  #[skyline::hook(offset = CREATE_UNIT_MENU)]
  fn create_unit_menu(parent: Object, method: Method) {
    call_original!(parent, method);
    if unsafe { active() } {
      if OWNER.swap(parent as usize, Ordering::Relaxed) != parent as usize {
        ATTEMPTED.with(|set| set.borrow_mut().clear());
      }
      unsafe { patch_child(parent, &UNIT_CLASS) };
    }
  }

  #[skyline::hook(offset = CREATE_PAIR_MENU)]
  fn create_pair_menu(parent: Object, unit: Object, method: Method) {
    call_original!(parent, unit, method);
    if unsafe { active() } {
      unsafe { patch_child(parent, &PAIR_CLASS) };
    }
  }

  unsafe fn menu_call(menu: Object, slot: usize) -> i32 {
    unsafe {
      let class: Object = field(menu, 0);
      let function: unsafe extern "C" fn(Object, Method) -> i32 =
        core::mem::transmute(field::<usize>(class, VTABLE + slot * 16));
      function(menu, field(class, VTABLE + slot * 16 + 8))
    }
  }

  unsafe extern "C" fn dispatch(menu: Object, method: Method) -> i32 {
    unsafe {
      if !active() {
        let original: unsafe extern "C" fn(Object, Method) -> i32 =
          core::mem::transmute(field::<usize>(method as Object, 0));
        return original(menu, method);
      }
      let list: Object = field(menu, 0x80);
      let Some(count) = list_count(list) else { return menu_call(menu, 51) };
      let pairs = field::<usize>(menu, 0) == PAIR_CLASS.load(Ordering::Relaxed);
      for index in 0..count {
        let Some(item) = object_at(list, index).filter(|item| !item.is_null()) else { continue };
        let eligible = if pairs {
          ready(item)
        } else {
          let unit: Object = field(item, 0x68);
          if unit.is_null() {
            continue;
          }
          let candidates =
            native!(PAIR_LIST, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE);
          list_count(candidates).is_some_and(|count| {
            (0..count).any(|i| object_at(candidates, i).is_some_and(|item| ready(item)))
          })
        };
        if !eligible {
          continue;
        }
        let class: Object = field(menu, 0);
        let select: unsafe extern "C" fn(Object, i32, Method) =
          core::mem::transmute(field::<usize>(class, VTABLE + 34 * 16));
        select(menu, index as i32, field(class, VTABLE + 34 * 16 + 8));
        if pairs {
          let Some(key) = key(item) else { continue };
          ATTEMPTED.with(|set| set.borrow_mut().insert(key));
        }
        // Return the original A/B result to TickInput. It handles opening,
        // closing, child suspension and cleanup exactly as manual input does.
        return menu_call(menu, 50);
      }
      menu_call(menu, 51)
    }
  }
}
