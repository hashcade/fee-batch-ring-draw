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
static ITEM_CLASSES: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
const ITEM_NAMES: [&[u8]; 8] = [
  b"SomnielDrawCount\0",
  b"SomnielChef\0",
  b"SomnielCookingOutcome\0",
  b"SomnielAutoSupport\0",
  b"SomnielRewindCount\0",
  b"SomnielBattleSave\0",
  b"SomnielPersonalOutfits\0",
  b"SomnielArenaCount\0",
];
static SAVE_FAILED: AtomicBool = AtomicBool::new(false);

extern "C" {
  fn cobapi_register_configmenuitem_cb(callback: unsafe extern "C" fn() -> Object);
  // Cobalt 1.31 scans Assembly::Load (0x42918C), which returns an assembly,
  // not an image, despite the exported symbol's name.
  #[link_name = "il2cpp_assembly_get_image"]
  fn find_assembly(name: *const u8) -> Object;
  fn il2cpp_class_from_name(image: Object, namespace: *const u8, name: *const u8) -> Object;
  fn il2cpp_class_init(class: Object);
}

pub(crate) fn install() {
  let callbacks = [
    create,
    create_chef,
    create_outcome,
    create_support,
    create_rewind,
    create_save,
    create_outfits,
    create_arena,
  ];
  for kind in MENU_ORDER {
    unsafe { cobapi_register_configmenuitem_cb(callbacks[kind]) };
  }
}

unsafe fn private_class(class: Object, kind: usize) -> Option<usize> {
  let cache = &ITEM_CLASSES[kind];
  let current = cache.load(Ordering::Relaxed);
  if current != 0 {
    return Some(current);
  }
  let count: u16 = unsafe { field(class, 0x126) };
  if count <= 26 {
    return None;
  }
  let words = (VTABLE + usize::from(count) * 16) / 8;
  let mut copy = unsafe { core::slice::from_raw_parts(class.cast::<usize>(), words) }
    .to_vec()
    .into_boxed_slice();
  // ConfigBasicMenuItem inherits these slots from BasicMenuItem. Its command
  // and help texts are fields, not extra virtual methods.
  for (slot, callback) in [
    (8, enabled as *const () as usize),
    (18, no_action as *const () as usize),
    (26, change as *const () as usize),
  ] {
    copy[(VTABLE + slot * 16) / 8] = callback;
  }
  copy[0x10 / 8] = ITEM_NAMES[kind].as_ptr() as usize;
  let private = Box::leak(copy).as_mut_ptr();
  for offset in [0x20 / 8, 0x30 / 8] {
    if unsafe { private.add(offset).read() } == class as usize {
      unsafe { private.add(offset).write(private as usize) };
    }
  }
  cache.store(private as usize, Ordering::Relaxed);
  Some(private as usize)
}

unsafe extern "C" fn enabled(_: Object, _: Method) -> i32 {
  1
}

unsafe extern "C" fn no_action(_: Object, _: Method) -> i32 {
  0
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
unsafe extern "C" fn create_rewind() -> Object {
  unsafe { create_kind(4) }
}
unsafe extern "C" fn create_save() -> Object {
  unsafe { create_kind(5) }
}
unsafe extern "C" fn create_outfits() -> Object {
  unsafe { create_kind(6) }
}
unsafe extern "C" fn create_arena() -> Object {
  unsafe { create_kind(7) }
}

unsafe fn kind(item: Object) -> usize {
  // Cobalt clones each returned item's class before adding it to the menu.
  // Its shallow copy preserves the name, not the original class address.
  let class: Object = unsafe { field(item, 0) };
  let name: usize = unsafe { field(class, 0x10) };
  ITEM_NAMES.iter().position(|value| value.as_ptr() as usize == name).unwrap_or(0)
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
    // Native OnSelect/OnDeselect own arrow visibility. Setting every row here
    // leaves arrows visible until each row has been selected and deselected.
    reference(item, 0x68, new_text(&localized_title(text_language(), kind)));
    command(item, NONE);
    description(item, NONE);
    item
  }
}

unsafe fn localized_title(language: i32, kind: usize) -> String {
  // Confirmed JP/EN/ZH labels stay compact. Other game languages supply their
  // own system nouns; only the generic uses suffix belongs to this plugin.
  let key = match kind {
    4 => "MID_MENU_REWIND",
    7 => "MID_Hub_Arena_Start",
    _ => return option_title(language, kind).to_owned(),
  };
  if matches!(language, 0 | 1 | 4 | 9 | 10) {
    return option_title(language, kind).to_owned();
  }
  let lookup: unsafe extern "C" fn(Object, Method) -> Object =
    unsafe { core::mem::transmute(base() + crate::meld::profile::MESS_GET) };
  let value = unsafe { lookup(new_text(key), NONE) };
  let Some(name) = (unsafe { crate::meld::native::text(value) }) else {
    return option_title(language, kind).to_owned();
  };
  let suffix = match language {
    2 | 5 => " : utilisations",
    3 | 6 => ": usos",
    7 => ": Nutzungen",
    8 => ": usi",
    11 => " 횟수",
    _ => " Uses",
  };
  format!("{name}{suffix}")
}

pub(crate) unsafe fn lookup_class(namespace: &[u8], name: &[u8]) -> Object {
  unsafe {
    let mut assembly = find_assembly(b"Assembly-CSharp.dll\0".as_ptr());
    if assembly.is_null() {
      assembly = find_assembly(b"Assembly-CSharp\0".as_ptr());
    }
    if assembly.is_null() {
      return ptr::null_mut();
    }
    let image: Object = field(assembly, 0);
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
      1 if !settings.chef.is_empty() => crate::cooking::native::chefs()
        .into_iter()
        .find(|(pid, _)| *pid == settings.chef)
        .map(|(_, name)| name)
        .unwrap_or_else(|| default_label(language).to_owned()),
      1 => default_label(language).to_owned(),
      2 => crate::cooking::native::outcome_name(settings.cooking_outcome)
        .unwrap_or_else(|| outcome_label(language, settings.cooking_outcome).to_owned()),
      3 => support_label(language, settings.auto_support).to_owned(),
      4 => count_label(language, settings.rewind_count),
      5 => support_label(language, settings.battle_save).to_owned(),
      6 => support_label(language, settings.personal_outfits).to_owned(),
      7 => count_label(language, settings.arena_count),
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
    let preview = if !SAVE_FAILED.load(Ordering::Relaxed) && kind(item) == 2 {
      crate::cooking::native::outcome_detail(config::current().cooking_outcome, language)
    } else {
      None
    };
    reference(item, 0x78, new_text(preview.as_deref().unwrap_or(label)));
  };
}

unsafe extern "C" fn change(item: Object, _: Method) -> i32 {
  let mut settings = config::current();
  let old = settings.clone();
  let kind = unsafe { kind(item) };
  let change: unsafe extern "C" fn(i32, i32, i32, i32, Method) -> i32 =
    unsafe { core::mem::transmute(base() + CHANGE_NUMBER) };
  match kind {
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
    4 | 7 => {
      let value = if kind == 4 { &mut settings.rewind_count } else { &mut settings.arena_count };
      let next = unsafe { change(*value as i32, 0, config::MAX_USE_COUNT as i32, 1, NONE) };
      if (0..=config::MAX_USE_COUNT as i32).contains(&next) {
        *value = next as u32;
      }
    }
    5 | 6 => {
      let value =
        if kind == 5 { &mut settings.battle_save } else { &mut settings.personal_outfits };
      let next = unsafe { change(i32::from(*value), 0, 1, 1, NONE) };
      if (0..=1).contains(&next) {
        *value = next == 1;
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
