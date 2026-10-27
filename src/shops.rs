//! Unlimited native stock, scoped to armory, item shop and flea market tables.

pub const CONTENTS: usize = 0x031FC8E0;
pub const STOCK: usize = 0x031FE610;
pub const INFINITY: usize = 0x031FE750;
pub const PURCHASE: usize = 0x031FE360;
pub const WEAPON_KEY: usize = 0x021D2AA0;
pub const ITEM_KEY: usize = 0x02050CE0;
pub const FLEA_KEY: usize = 0x0260E5C0;
pub const LOOKUP: usize = 0x03C742D0;

// Closed generic MethodInfo slots: armory, item shop, flea market. Other
// ShopDataBase instantiations must retain their original behavior.
pub const CONTENT_METHODS: [usize; 3] = [0x062A7898, 0x062A2FB8, 0x062B8220];
pub const STOCK_METHODS: [usize; 3] = [0x062A7A00, 0x062A3120, 0x062B8240];
pub const INFINITY_METHODS: [usize; 3] = [0x062A7A08, 0x062A3128, 0x062B8248];
pub const PURCHASE_METHODS: [usize; 3] = [0x062A79F8, 0x062A3118, 0x062B8238];

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x031FC8E0, [0xD10443FF, 0xA90B7BFD, 0x9102C3FD, 0xA90C6FFC]),
  (0x031FE610, [0xD10143FF, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x031FE750, [0xD10143FF, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x031FE360, [0xD10183FF, 0xA9017BFD, 0x910043FD, 0xF90013F9]),
  (0x021D2AA0, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x02050CE0, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x0260E5C0, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x03C742D0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xF9400C68]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::native::{base, field, Method, Object};
  use core::cell::Cell;

  // The generated key stays rooted by the native caller. Consume it at the
  // very next matching lookup, and never keep it after the list build ends.
  std::thread_local! {
    static LIST: Cell<(bool, usize)> = const { Cell::new((false, 0)) };
  }

  unsafe fn matches(method: Method, slots: &[usize; 3]) -> bool {
    !method.is_null()
      && slots.iter().any(|offset| unsafe {
        // Wrappers initialize their own metadata before entering these shared
        // bodies. Read only; do not initialize unrelated tables inside a hook.
        let cell = field::<Object>((base() + offset) as Object, 0);
        !cell.is_null() && field::<Object>(cell, 0) == method as Object
      })
  }

  pub(crate) fn install() {
    skyline::install_hooks!(
      contents, stock, infinity, purchase, weapon_key, item_key, flea_key, lookup
    );
  }

  #[skyline::hook(offset = CONTENTS)]
  fn contents(added_key: Object, stock_key: Object, method: Method) -> Object {
    let active = unsafe { matches(method, &CONTENT_METHODS) };
    let previous = LIST.with(|state| state.replace((active, 0)));
    let result = call_original!(added_key, stock_key, method);
    LIST.with(|state| state.set(previous));
    result
  }

  fn remember(key: Object) -> Object {
    LIST.with(|state| {
      if state.get().0 {
        state.set((true, key as usize));
      }
    });
    key
  }

  #[skyline::hook(offset = WEAPON_KEY)]
  fn weapon_key(iid: Object, method: Method) -> Object {
    remember(call_original!(iid, method))
  }

  #[skyline::hook(offset = ITEM_KEY)]
  fn item_key(iid: Object, method: Method) -> Object {
    remember(call_original!(iid, method))
  }

  #[skyline::hook(offset = FLEA_KEY)]
  fn flea_key(iid: Object, method: Method) -> Object {
    remember(call_original!(iid, method))
  }

  #[skyline::hook(offset = LOOKUP)]
  fn lookup(dictionary: Object, key: Object, value: *mut i32, method: Method) -> bool {
    let override_stock = LIST.with(|state| {
      let (active, expected) = state.get();
      state.set((active, 0));
      active && expected != 0 && expected == key as usize
    });
    let found = call_original!(dictionary, key, value, method);
    if override_stock && !value.is_null() {
      // Replace only the returned integer, not the dictionary, variant tag,
      // reference field, unlock keys or persistent stock. -1 is native infinity.
      unsafe { value.write(-1) };
      return true;
    }
    found
  }

  #[skyline::hook(offset = STOCK)]
  fn stock(iid: Object, key: Object, method: Method) -> i32 {
    if !iid.is_null() && unsafe { matches(method, &STOCK_METHODS) } {
      -1
    } else {
      call_original!(iid, key, method)
    }
  }

  #[skyline::hook(offset = INFINITY)]
  fn infinity(iid: Object, key: Object, method: Method) -> bool {
    if !iid.is_null() && unsafe { matches(method, &INFINITY_METHODS) } {
      true
    } else {
      call_original!(iid, key, method)
    }
  }

  #[skyline::hook(offset = PURCHASE)]
  fn purchase(iid: Object, count: i32, added_key: Object, stock_key: Object, method: Method) {
    if !iid.is_null() && unsafe { matches(method, &PURCHASE_METHODS) } {
      // This method only decrements the shop's saved stock. Native callers
      // still validate capacity, deduct the price and grant the purchased item.
      return;
    }
    call_original!(iid, count, added_key, stock_key, method);
  }
}
