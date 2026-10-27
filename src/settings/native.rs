//! One private ConfigBasicMenuItem class; no global vtable/save-variable edits.

use super::*;
use crate::{
  config,
  meld::native::{new_text, text_language},
  native::{base, field, reference, Method, Object},
  profile,
};
use core::{
  ptr,
  sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

const VTABLE: usize = 0x138;
const NONE: Method = ptr::null();
static ITEM_CLASS: AtomicUsize = AtomicUsize::new(0);
static SAVE_FAILED: AtomicBool = AtomicBool::new(false);

extern "C" {
  fn cobapi_register_configmenuitem_cb(callback: unsafe extern "C" fn() -> Object);
  fn il2cpp_assembly_get_image(name: *const u8) -> Object;
  fn il2cpp_class_from_name(image: Object, namespace: *const u8, name: *const u8) -> Object;
  fn il2cpp_class_init(class: Object);
}

pub(crate) fn install() {
  unsafe { cobapi_register_configmenuitem_cb(create) };
}

unsafe fn named(pointer: *const u8, expected: &[u8]) -> bool {
  !pointer.is_null()
    && unsafe {
      expected.iter().enumerate().all(|(i, byte)| pointer.add(i).read() == *byte)
        && pointer.add(expected.len()).read() == 0
    }
}

unsafe fn private_class(class: Object) -> Option<usize> {
  let current = ITEM_CLASS.load(Ordering::Relaxed);
  if current != 0 {
    return Some(current);
  }
  let count: u16 = unsafe { field(class, 0x126) };
  if !(32..=128).contains(&count) || unsafe { field::<u32>(class, 0xF8) } != 0x88 {
    return None;
  }
  let words = (VTABLE + usize::from(count) * 16) / 8;
  let mut copy = unsafe { core::slice::from_raw_parts(class.cast::<usize>(), words) }
    .to_vec()
    .into_boxed_slice();
  for (name, callback) in [
    (b"CustomCall".as_slice(), change as *const () as usize),
    (b"SetCommandText".as_slice(), command as *const () as usize),
    (b"SetHelpText".as_slice(), description as *const () as usize),
  ] {
    let slot = (0..usize::from(count)).find(|&slot| unsafe {
      let method: Object = field(class, VTABLE + slot * 16 + 8);
      !method.is_null() && named(field(method, 0x10), name)
    })?;
    copy[(VTABLE + slot * 16) / 8] = callback;
  }
  let class = Box::leak(copy).as_mut_ptr() as usize;
  ITEM_CLASS.store(class, Ordering::Relaxed);
  Some(class)
}

unsafe extern "C" fn create() -> Object {
  unsafe {
    let mut image = il2cpp_assembly_get_image(b"Assembly-CSharp.dll\0".as_ptr());
    if image.is_null() {
      image = il2cpp_assembly_get_image(b"Assembly-CSharp\0".as_ptr());
    }
    if image.is_null() {
      return ptr::null_mut();
    }
    let class = il2cpp_class_from_name(image, b"\0".as_ptr(), b"ConfigBasicMenuItem\0".as_ptr());
    if class.is_null() {
      return ptr::null_mut();
    }
    il2cpp_class_init(class);
    let allocate: unsafe extern "C" fn(Object) -> Object =
      core::mem::transmute(base() + profile::ALLOCATE_OBJECT);
    let ctor: unsafe extern "C" fn(Object, Method) = core::mem::transmute(base() + ITEM_CTOR);
    let item = allocate(class);
    if item.is_null() {
      return item;
    }
    ctor(item, NONE);
    let Some(private) = private_class(class) else { return item };
    item.cast::<usize>().write(private);
    // ConfigMethodKind.Switch, using the game's existing left/right repeat.
    item.cast::<u8>().add(0x64).cast::<i32>().write(0);
    item.cast::<u8>().add(0x80).write(1);
    reference(item, 0x68, new_text(title(text_language())));
    command(item, NONE);
    description(item, NONE);
    item
  }
}

unsafe extern "C" fn command(item: Object, _: Method) {
  unsafe { reference(item, 0x70, new_text(&config::draw_count().to_string())) };
}

unsafe extern "C" fn description(item: Object, _: Method) {
  unsafe {
    reference(item, 0x78, new_text(help(text_language(), SAVE_FAILED.load(Ordering::Relaxed))))
  };
}

unsafe extern "C" fn change(item: Object, _: Method) -> i32 {
  let old = config::draw_count();
  let change: unsafe extern "C" fn(i32, i32, i32, i32, Method) -> i32 =
    unsafe { core::mem::transmute(base() + CHANGE_NUMBER) };
  let next = unsafe { change(old as i32, 2, config::MAX_DRAW_COUNT as i32, 1, NONE) };
  if next == old as i32 {
    return 0;
  }
  let saved = next >= 0 && config::set_draw_count(next as u32).is_ok();
  SAVE_FAILED.store(!saved, Ordering::Relaxed);
  unsafe {
    command(item, NONE);
    description(item, NONE);
    let refresh: unsafe extern "C" fn(Object, Method) = core::mem::transmute(base() + UPDATE_TEXT);
    refresh(item, NONE);
  }
  if saved {
    4096
  } else {
    2048
  }
}
