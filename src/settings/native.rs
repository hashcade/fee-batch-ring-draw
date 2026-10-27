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
static ITEM_CLASSES: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];
static SAVE_FAILED: AtomicBool = AtomicBool::new(false);

extern "C" {
  fn cobapi_register_configmenuitem_cb(callback: unsafe extern "C" fn() -> Object);
  fn il2cpp_assembly_get_image(name: *const u8) -> Object;
  fn il2cpp_class_from_name(image: Object, namespace: *const u8, name: *const u8) -> Object;
  fn il2cpp_class_init(class: Object);
}

pub(crate) fn install() {
  for callback in [create, create_chef, create_outcome, create_support] {
    unsafe { cobapi_register_configmenuitem_cb(callback) };
  }
}

unsafe fn named(pointer: *const u8, expected: &[u8]) -> bool {
  !pointer.is_null()
    && unsafe {
      expected.iter().enumerate().all(|(i, byte)| pointer.add(i).read() == *byte)
        && pointer.add(expected.len()).read() == 0
    }
}

unsafe fn private_class(class: Object, kind: usize) -> Option<usize> {
  let cache = &ITEM_CLASSES[kind];
  let current = cache.load(Ordering::Relaxed);
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
  cache.store(class, Ordering::Relaxed);
  Some(class)
}

unsafe extern "C" fn create() -> Object {
  unsafe { create_kind(0) }
}
unsafe extern "C" fn create_chef() -> Object {
  unsafe { create_kind(1) }
}
unsafe extern "C" fn create_outcome() -> Object {
  unsafe { create_kind(2) }
}
unsafe extern "C" fn create_support() -> Object {
  unsafe { create_kind(3) }
}

unsafe fn kind(item: Object) -> usize {
  let class = unsafe { field::<usize>(item, 0) };
  ITEM_CLASSES.iter().position(|cache| cache.load(Ordering::Relaxed) == class).unwrap_or(0)
}

unsafe fn create_kind(kind: usize) -> Object {
  unsafe {
    let class = lookup_class(b"\0", b"ConfigBasicMenuItem\0");
    if class.is_null() {
      return ptr::null_mut();
    }
    let allocate: unsafe extern "C" fn(Object) -> Object =
      core::mem::transmute(base() + profile::ALLOCATE_OBJECT);
    let ctor: unsafe extern "C" fn(Object, Method) = core::mem::transmute(base() + ITEM_CTOR);
    let item = allocate(class);
    if item.is_null() {
      return item;
    }
    ctor(item, NONE);
    let Some(private) = private_class(class, kind) else { return item };
    item.cast::<usize>().write(private);
    // ConfigMethodKind.Switch, using the game's existing left/right repeat.
    item.cast::<u8>().add(0x64).cast::<i32>().write(0);
    item.cast::<u8>().add(0x80).write(1);
    reference(item, 0x68, new_text(option_title(text_language(), kind)));
    command(item, NONE);
    description(item, NONE);
    item
  }
}

pub(crate) unsafe fn lookup_class(namespace: &[u8], name: &[u8]) -> Object {
  unsafe {
    let mut image = il2cpp_assembly_get_image(b"Assembly-CSharp.dll\0".as_ptr());
    if image.is_null() {
      image = il2cpp_assembly_get_image(b"Assembly-CSharp\0".as_ptr());
    }
    if image.is_null() {
      return ptr::null_mut();
    }
    let class = il2cpp_class_from_name(image, namespace.as_ptr(), name.as_ptr());
    if class.is_null() {
      return ptr::null_mut();
    }
    il2cpp_class_init(class);
    class
  }
}

unsafe extern "C" fn command(item: Object, _: Method) {
  unsafe {
    let settings = config::current();
    let language = text_language();
    let label = match kind(item) {
      1 => crate::cooking::native::chefs()
        .into_iter()
        .find(|(pid, _)| *pid == settings.chef)
        .map(|(_, name)| name)
        .unwrap_or_else(|| default_label(language).to_owned()),
      2 => crate::cooking::native::outcome_name(settings.cooking_outcome)
        .unwrap_or_else(|| outcome_label(language, settings.cooking_outcome).to_owned()),
      3 => support_label(language, settings.auto_support).to_owned(),
      _ => settings.draw_count.to_string(),
    };
    reference(item, 0x70, new_text(&label));
  }
}

unsafe extern "C" fn description(item: Object, _: Method) {
  unsafe {
    let language = text_language();
    let label = if SAVE_FAILED.load(Ordering::Relaxed) {
      help(language, true)
    } else {
      option_help(language, kind(item))
    };
    reference(item, 0x78, new_text(label));
  };
}

unsafe extern "C" fn change(item: Object, _: Method) -> i32 {
  let mut settings = config::current();
  let old = settings.clone();
  let change: unsafe extern "C" fn(i32, i32, i32, i32, Method) -> i32 =
    unsafe { core::mem::transmute(base() + CHANGE_NUMBER) };
  match unsafe { kind(item) } {
    1 => {
      let mut chefs = unsafe { crate::cooking::native::chefs() };
      chefs.insert(0, (String::new(), String::new()));
      let current = chefs.iter().position(|(pid, _)| *pid == settings.chef).unwrap_or(0);
      let next = unsafe { change(current as i32, 0, chefs.len() as i32 - 1, 1, NONE) };
      if let Some((pid, _)) = chefs.get(next as usize) {
        settings.chef = pid.clone();
      }
    }
    2 => {
      let next = unsafe { change(i32::from(settings.cooking_outcome), 0, 5, 1, NONE) };
      if (0..=5).contains(&next) {
        settings.cooking_outcome = next as u8;
      }
    }
    3 => {
      let next = unsafe { change(i32::from(settings.auto_support), 0, 1, 1, NONE) };
      if (0..=1).contains(&next) {
        settings.auto_support = next == 1;
      }
    }
    _ => {
      let next =
        unsafe { change(settings.draw_count as i32, 2, config::MAX_DRAW_COUNT as i32, 1, NONE) };
      if next >= 0 {
        settings.draw_count = next as u32;
      }
    }
  }
  if settings == old {
    return 0;
  }
  let saved = config::set(settings).is_ok();
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
