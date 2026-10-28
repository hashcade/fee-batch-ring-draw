//! Private native BasicMenuItem classes, shared by facility menu extensions.

use crate::native::{base, field, Method, Object};
use core::{
  ffi::c_char,
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

pub(crate) const VTABLE: usize = 0x138;

pub(crate) unsafe fn override_class(
  object: Object,
  cache: &AtomicUsize,
  callbacks: &[(usize, usize)],
) -> bool {
  if object.is_null() {
    return false;
  }
  unsafe {
    let class: Object = field(object, 0);
    if class.is_null() {
      return false;
    }
    let count: u16 = field(class, 0x126);
    if callbacks.iter().any(|(slot, _)| *slot >= usize::from(count)) {
      return false;
    }
    let mut private = cache.load(Ordering::Relaxed);
    if private == 0 {
      let words = (VTABLE + usize::from(count) * 16) / 8;
      let mut copy =
        core::slice::from_raw_parts(class.cast::<usize>(), words).to_vec().into_boxed_slice();
      for &(slot, function) in callbacks {
        copy[(VTABLE + slot * 16) / 8] = function;
      }
      private = Box::leak(copy).as_mut_ptr() as usize;
      cache.store(private, Ordering::Relaxed);
    }
    object.cast::<usize>().write(private);
    true
  }
}

pub(crate) unsafe fn named(pointer: *const c_char, expected: &[u8]) -> bool {
  !pointer.is_null()
    && unsafe {
      expected.iter().enumerate().all(|(i, &byte)| pointer.add(i).read() as u8 == byte)
        && pointer.add(expected.len()).read() == 0
    }
}

pub(crate) unsafe fn create(
  first: Object,
  cache: &AtomicUsize,
  callbacks: &[(usize, usize)],
) -> Option<Object> {
  if first.is_null() {
    return None;
  }
  // Help/flag/map-attribute slots belong to MapBasicMenuItem, not its
  // 27-slot BasicMenuItem parent. Keep that native type for map entries.
  let map_item = callbacks.iter().any(|(slot, _)| *slot >= 27);
  let (expected, ctor_offset): (&[u8], usize) = if map_item {
    (b"MapBasicMenuItem", crate::collector::profile::MAP_ITEM_CTOR)
  } else {
    (b"BasicMenuItem", crate::collector::profile::ITEM_CTOR)
  };
  let mut class: Object = unsafe { field(first, 0) };
  loop {
    if class.is_null() {
      return None;
    }
    if unsafe { named(field(class, 0x10), expected) } {
      break;
    }
    class = unsafe { field(class, 0x58) };
  }
  let count: u16 = unsafe { field(class, 0x126) };
  if callbacks.iter().any(|(slot, _)| *slot >= usize::from(count)) {
    return None;
  }
  let allocate: unsafe extern "C" fn(Object) -> Object =
    unsafe { core::mem::transmute(base() + crate::profile::ALLOCATE_OBJECT) };
  let ctor: unsafe extern "C" fn(Object, Method) =
    unsafe { core::mem::transmute(base() + ctor_offset) };
  let item = unsafe { allocate(class) };
  if item.is_null() {
    return None;
  }
  unsafe { ctor(item, ptr::null()) };
  unsafe { override_class(item, cache, callbacks) }.then_some(item)
}
