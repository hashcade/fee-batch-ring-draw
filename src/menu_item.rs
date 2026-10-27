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
    if !(1..=128).contains(&count) || callbacks.iter().any(|(slot, _)| *slot >= usize::from(count))
    {
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
  let mut class: Object = unsafe { field(first, 0) };
  let mut found = false;
  for _ in 0..16 {
    if class.is_null() {
      return None;
    }
    if unsafe { named(field(class, 0x10), b"BasicMenuItem") } {
      found = true;
      break;
    }
    class = unsafe { field(class, 0x58) };
  }
  if !found {
    return None;
  }
  let count: u16 = unsafe { field(class, 0x126) };
  let size: u32 = unsafe { field(class, 0xF8) };
  if !(29..=128).contains(&count)
    || !(0x60..=0x200).contains(&size)
    || callbacks.iter().any(|(slot, _)| *slot >= usize::from(count))
  {
    return None;
  }
  let allocate: unsafe extern "C" fn(Object) -> Object =
    unsafe { core::mem::transmute(base() + crate::profile::ALLOCATE_OBJECT) };
  let ctor: unsafe extern "C" fn(Object, Method) =
    unsafe { core::mem::transmute(base() + crate::collector::profile::ITEM_CTOR) };
  let item = unsafe { allocate(class) };
  if item.is_null() {
    return None;
  }
  unsafe { ctor(item, ptr::null()) };
  unsafe { override_class(item, cache, callbacks) }.then_some(item)
}
