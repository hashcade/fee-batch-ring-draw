//! Two choices in the existing confirmation, using its native yes callback.

use super::{confirmation, native::text_language, plugin, profile};
use crate::{
  meld::native::new_text,
  native::{base, field, list_count, metadata, reference, Method, Object},
};
use core::{
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

const VTABLE: usize = 0x138;
const NONE: Method = ptr::null();
static SINGLE_CLASS: AtomicUsize = AtomicUsize::new(0);
static BATCH_CLASS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn name(item: Object, _: Method) -> Object {
  unsafe { field(item, 0x68) }
}

unsafe extern "C" fn decide(item: Object, method: Method) -> i32 {
  let batch = unsafe { field::<usize>(item, 0) } == BATCH_CLASS.load(Ordering::Relaxed);
  plugin::choose(batch, || {
    let original: unsafe extern "C" fn(Object, Method) -> i32 =
      unsafe { core::mem::transmute(base() + profile::YES_ITEM_CALL) };
    unsafe { original(item, method) }
  })
}

pub(super) unsafe fn choice(yes: Object, count: u32) -> Option<Object> {
  if yes.is_null() {
    return None;
  }
  let class: Object = unsafe { field(yes, 0) };
  if class.is_null() {
    return None;
  }
  let slots: u16 = unsafe { field(class, 0x126) };
  let size: u32 = unsafe { field(class, 0xF8) };
  if !(29..=128).contains(&slots)
    || !(0x78..=0x100).contains(&size)
    || unsafe { field::<usize>(class, VTABLE + 18 * 16) } != base() + profile::YES_ITEM_CALL
  {
    return None;
  }
  let allocate: unsafe extern "C" fn(Object) -> Object =
    unsafe { core::mem::transmute(base() + crate::profile::ALLOCATE_OBJECT) };
  let ctor: unsafe extern "C" fn(Object, Object, Method) =
    unsafe { core::mem::transmute(base() + profile::YES_ITEM_CTOR) };
  let item = unsafe { allocate(class) };
  if item.is_null() {
    return None;
  }
  let label = new_text(&confirmation::action(text_language(), count));
  if label.is_null() {
    return None;
  }
  unsafe {
    ctor(item, field(yes, 0x70), NONE);
    reference(item, 0x68, label);
  }
  let cache = if count == 1 { &SINGLE_CLASS } else { &BATCH_CLASS };
  let mut private = cache.load(Ordering::Relaxed);
  if private == 0 {
    let words = (VTABLE + usize::from(slots) * 16) / 8;
    let mut copy = unsafe { core::slice::from_raw_parts(class.cast::<usize>(), words) }
      .to_vec()
      .into_boxed_slice();
    copy[(VTABLE + 4 * 16) / 8] = name as *const () as usize;
    copy[(VTABLE + 18 * 16) / 8] = decide as *const () as usize;
    private = Box::leak(copy).as_mut_ptr() as usize;
    cache.store(private, Ordering::Relaxed);
  }
  unsafe { item.cast::<usize>().write(private) };
  Some(item)
}

pub(super) unsafe fn append_batch(dialog: Object, batch: Object, no: Object) -> bool {
  if dialog.is_null() {
    return false;
  }
  let list: Object = unsafe { field(dialog, 0x80) };
  if unsafe { list_count(list) } != Some(2) {
    return false;
  }
  let method = unsafe { metadata(0x06286A70) };
  if method.is_null() {
    return false;
  }
  let add: unsafe extern "C" fn(Object, Object, Method) =
    unsafe { core::mem::transmute(base() + crate::profile::LIST_ADD) };
  unsafe { add(list, batch, method) };
  if unsafe { list_count(list) } != Some(3) {
    return false;
  }
  unsafe {
    let array: Object = field(list, 0x10);
    reference(array, 0x28, batch);
    reference(array, 0x30, no);
    let rows: unsafe extern "C" fn(Object, i32, Method) =
      core::mem::transmute(base() + crate::profile::SET_MENU_ROWS);
    rows(dialog, 3, NONE);
  }
  true
}
