//! Unlimited native stock, scoped to armory, item shop and flea market tables.

pub const LIST_LOOKUP: usize = 0x031FD608;
pub const STOCK: usize = 0x031FE610;
pub const INFINITY: usize = 0x031FE750;
pub const PURCHASE: usize = 0x031FE360;
pub const LOOKUP: usize = 0x03C742D0;

// Closed generic MethodInfo slots: armory, item shop, flea market. Other
// ShopDataBase instantiations must retain their original behavior.
pub const CONTENT_METHODS: [usize; 3] = [0x062A7898, 0x062A2FB8, 0x062B8228];
pub const STOCK_METHODS: [usize; 3] = [0x062A7A00, 0x062A3120, 0x062B8240];
pub const INFINITY_METHODS: [usize; 3] = [0x062A7A08, 0x062A3128, 0x062B8248];
pub const PURCHASE_METHODS: [usize; 3] = [0x062A79F8, 0x062A3118, 0x062B8238];

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x0260E884, [0xD001E55A, 0xF941175A, 0xF9400260, 0x977A0E58]),
  (0x0260E8DC, [0xF9400342, 0xAA1303E0, 0xAA1403E1, 0xA9444FF4]),
  (0x031FC8E0, [0xD10443FF, 0xA90B7BFD, 0x9102C3FD, 0xA90C6FFC]),
  (0x031FE610, [0xD10143FF, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x031FE750, [0xD10143FF, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x031FE360, [0xD10183FF, 0xA9017BFD, 0x910043FD, 0xF90013F9]),
  (0x031FC908, [0xAA0203F3, 0xF9000BE1, 0xF90003E0, 0x37000A28]),
  (0x031FCA68, [0xF8418E74, 0x3944CA88, 0x360000E8, 0xF9406288]),
  (0x031FD600, [0x9100A3E2, 0xAA1403E1, 0x9429DB32, 0xB9402BE8]),
  (0x031FD610, [0x7200001F, 0x1A9F1108, 0x3100051F, 0x54FFD6C0]),
  (0x03C742D0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xF9400C68]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::native::{base, field, Method, Object};
  use core::sync::atomic::{AtomicUsize, Ordering};

  static LIST_RESUME: AtomicUsize = AtomicUsize::new(0);

  // Replace only this BL site, not Dictionary.TryGetValue itself. At this
  // point x19 is MethodInfo+0x18 and the native out Value is at sp+0x28.
  // The replaced call follows AAPCS64, so Rust preserves x19-x29 and d8-d15.
  // Replay all four following instructions (including NZCV), then resume at
  // the native conditional branch. x16 is call-clobbered by the original BL.
  core::arch::global_asm!(
    ".p2align 2",
    ".global fee_shop_list_lookup",
    ".hidden fee_shop_list_lookup",
    "fee_shop_list_lookup:",
    "sub x4, x19, #0x18",
    "bl {lookup}",
    "ldr w8, [sp, #0x28]",
    "tst w0, #1",
    "csel w8, w8, wzr, ne",
    "cmn w8, #1",
    "adrp x16, {resume}",
    "ldr x16, [x16, :lo12:{resume}]",
    "sub x30, x16, #0x10",
    "br x16",
    lookup = sym list_lookup,
    resume = sym LIST_RESUME,
  );

  extern "C" {
    fn fee_shop_list_lookup();
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
    LIST_RESUME.store(base() + LIST_LOOKUP + 0x14, Ordering::Relaxed);
    unsafe {
      let mut unused = core::ptr::null_mut();
      skyline::hooks::A64HookFunction(
        (base() + LIST_LOOKUP) as _,
        fee_shop_list_lookup as *const () as _,
        &mut unused,
      );
    }
    skyline::install_hooks!(stock, infinity, purchase);
  }

  unsafe extern "C" fn list_lookup(
    dictionary: Object,
    key: Object,
    value: *mut i32,
    method: Method,
    list_method: Method,
  ) -> bool {
    let lookup: unsafe extern "C" fn(Object, Object, *mut i32, Method) -> bool =
      unsafe { core::mem::transmute(base() + LOOKUP) };
    let found = unsafe { lookup(dictionary, key, value, method) };
    if crate::config::enabled(crate::config::Feature::ShopStock)
      && !value.is_null()
      && unsafe { matches(list_method, &CONTENT_METHODS) }
    {
      // Replace only the returned integer, not the dictionary, variant tag,
      // reference field, unlock keys or persistent stock. -1 is native infinity.
      unsafe { value.write(-1) };
      return true;
    }
    found
  }

  #[skyline::hook(offset = STOCK)]
  fn stock(iid: Object, key: Object, method: Method) -> i32 {
    if crate::config::enabled(crate::config::Feature::ShopStock)
      && !iid.is_null()
      && unsafe { matches(method, &STOCK_METHODS) }
    {
      -1
    } else {
      call_original!(iid, key, method)
    }
  }

  #[skyline::hook(offset = INFINITY)]
  fn infinity(iid: Object, key: Object, method: Method) -> bool {
    if crate::config::enabled(crate::config::Feature::ShopStock)
      && !iid.is_null()
      && unsafe { matches(method, &INFINITY_METHODS) }
    {
      true
    } else {
      call_original!(iid, key, method)
    }
  }

  #[skyline::hook(offset = PURCHASE)]
  fn purchase(iid: Object, count: i32, added_key: Object, stock_key: Object, method: Method) {
    if crate::config::enabled(crate::config::Feature::ShopStock)
      && !iid.is_null()
      && unsafe { matches(method, &PURCHASE_METHODS) }
    {
      // This method only decrements the shop's saved stock. Native callers
      // still validate capacity, deduct the price and grant the purchased item.
      return;
    }
    call_original!(iid, count, added_key, stock_key, method);
  }
}
