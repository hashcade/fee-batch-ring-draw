//! Private ConfigBasicMenuItem classes; native scrolling and selection.

use super::*;
use crate::{
  config,
  meld::native::{new_text, text_language},
  native::{base, field, reference, Method, Object},
  profile,
};
use core::{
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

const VTABLE: usize = 0x138;
const NONE: Method = ptr::null();
static ITEM_CLASSES: [AtomicUsize; OPTION_COUNT] = [const { AtomicUsize::new(0) }; OPTION_COUNT];
const ITEM_NAMES: [&[u8]; OPTION_COUNT] = [
  b"SomnielDrawCount\0",
  b"SomnielChef\0",
  b"SomnielCookingOutcome\0",
  b"SomnielAutoSupport\0",
  b"SomnielRewindCount\0",
  b"SomnielBattleSave\0",
  b"SomnielPersonalOutfits\0",
  b"SomnielArenaCount\0",
  b"SomnielFishingUses\0",
  b"SomnielShopStock\0",
  b"SomnielGiftStock\0",
  b"SomnielRepeatCooking\0",
  b"SomnielFastFishing\0",
  b"SomnielFastTraining\0",
  b"SomnielFastWyvern\0",
  b"SomnielExpMultiplier\0",
  b"SomnielSpMultiplier\0",
  b"SomnielSpTarget\0",
  b"SomnielSpAmount\0",
  b"SomnielGold\0",
  b"SomnielBondFragments\0",
  b"SomnielIron\0",
  b"SomnielSteel\0",
  b"SomnielSilver\0",
  b"SomnielMtCrystal\0",
  b"SomnielCritCrystal\0",
  b"SomnielDexCrystal\0",
  b"SomnielSpdCrystal\0",
  b"SomnielDefCrystal\0",
  b"SomnielResCrystal\0",
  b"SomnielHitCrystal\0",
  b"SomnielAvoCrystal\0",
  b"SomnielDdgCrystal\0",
  b"SomnielAutoPolish\0",
  b"SomnielPolishActor\0",
];

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
    create_draw,
    create_chef,
    create_outcome,
    create_support,
    create_rewind,
    create_save,
    create_outfits,
    create_arena,
    create_fishing_uses,
    create_shop_stock,
    create_gift_stock,
    create_repeat_cooking,
    create_fast_fishing,
    create_fast_training,
    create_fast_wyvern,
    create_exp_multiplier,
    create_sp_multiplier,
    create_option::<17>,
    create_option::<18>,
    create_option::<19>,
    create_option::<20>,
    create_option::<21>,
    create_option::<22>,
    create_option::<23>,
    create_option::<24>,
    create_option::<25>,
    create_option::<26>,
    create_option::<27>,
    create_option::<28>,
    create_option::<29>,
    create_option::<30>,
    create_option::<31>,
    create_option::<32>,
    create_option::<33>,
    create_option::<34>,
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

unsafe extern "C" fn enabled(item: Object, _: Method) -> i32 {
  let kind = unsafe { kind(item) };
  if crate::resources::is_setting(kind) {
    return i32::from(unsafe { crate::resources::native::enabled(kind) });
  }
  1
}

unsafe extern "C" fn no_action(_: Object, _: Method) -> i32 {
  0
}

unsafe extern "C" fn create_option<const KIND: usize>() -> Object {
  unsafe { create_kind(KIND) }
}

unsafe extern "C" fn create_draw() -> Object {
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
unsafe extern "C" fn create_fishing_uses() -> Object {
  unsafe { create_kind(8) }
}
unsafe extern "C" fn create_shop_stock() -> Object {
  unsafe { create_kind(9) }
}
unsafe extern "C" fn create_gift_stock() -> Object {
  unsafe { create_kind(10) }
}
unsafe extern "C" fn create_repeat_cooking() -> Object {
  unsafe { create_kind(11) }
}
unsafe extern "C" fn create_fast_fishing() -> Object {
  unsafe { create_kind(12) }
}
unsafe extern "C" fn create_fast_training() -> Object {
  unsafe { create_kind(13) }
}
unsafe extern "C" fn create_fast_wyvern() -> Object {
  unsafe { create_kind(14) }
}
unsafe extern "C" fn create_exp_multiplier() -> Object {
  unsafe { create_kind(15) }
}
unsafe extern "C" fn create_sp_multiplier() -> Object {
  unsafe { create_kind(16) }
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
    if kind == 15 {
      crate::resources::native::reset();
    }
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
    if crate::resources::is_setting(kind) {
      crate::resources::native::attach(item, kind);
    }
    command(item, NONE);
    let hint = if crate::resources::is_value(kind) {
      crate::resources::apply_hint(text_language())
    } else {
      ""
    };
    reference(item, 0x78, new_text(hint));
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
    let kind = kind(item);
    if crate::resources::is_setting(kind) {
      reference(item, 0x70, new_text(&crate::resources::native::label(text_language(), kind)));
      return;
    }
    let settings = config::current();
    let language = text_language();
    let label = match kind {
      1 if !settings.chef.is_empty() => crate::cooking::native::chefs()
        .into_iter()
        .find(|(pid, _)| *pid == settings.chef)
        .map(|(_, name)| name)
        .unwrap_or_else(|| default_label(language).to_owned()),
      1 => default_label(language).to_owned(),
      2 => crate::cooking::native::outcome_name(settings.cooking_outcome)
        .unwrap_or_else(|| outcome_label(language, settings.cooking_outcome).to_owned()),
      3 => support_label(language, settings.auto_support).to_owned(),
      33 => support_label(language, settings.auto_polish).to_owned(),
      34 => crate::resources::native::roster()
        .into_iter()
        .find(|entry| entry.0 == settings.polish_actor)
        .map(|entry| entry.1)
        .unwrap_or_else(|| "—".to_owned()),
      4 => count_label(language, settings.rewind_count),
      5 => support_label(language, settings.battle_save).to_owned(),
      6 => support_label(language, settings.personal_outfits).to_owned(),
      7 => count_label(language, settings.arena_count),
      15 => multiplier_label(language, settings.exp_multiplier),
      16 => format!("{}×", settings.sp_multiplier),
      kind @ 8..=14 => {
        support_label(language, config::Feature::ALL[kind - 8].value(&settings)).to_owned()
      }
      _ => settings.draw_count.to_string(),
    };
    reference(item, 0x70, new_text(&label));
  }
}

unsafe extern "C" fn change(item: Object, _: Method) -> i32 {
  let kind = unsafe { kind(item) };
  if crate::resources::is_setting(kind) {
    return unsafe { crate::resources::native::change(item, kind) };
  }
  let mut settings = config::current();
  let old = settings.clone();
  let change: unsafe extern "C" fn(i32, i32, i32, i32, Method) -> i32 =
    unsafe { core::mem::transmute(base() + CHANGE_NUMBER) };
  match kind {
    3 | 33 => {
      let value = if kind == 3 { &mut settings.auto_support } else { &mut settings.auto_polish };
      let next = unsafe { change(i32::from(*value), 0, 1, 1, NONE) };
      if (0..=1).contains(&next) {
        *value = next == 1;
      }
    }
    34 => {
      let actors = unsafe { crate::resources::native::roster() };
      if actors.is_empty() {
        return 0;
      }
      let index = actors.iter().position(|entry| entry.0 == settings.polish_actor).unwrap_or(0);
      let next = unsafe { change(index as i32, 0, actors.len() as i32 - 1, 1, NONE) };
      if let Some(actor) = actors.get(next as usize) {
        settings.polish_actor = actor.0.clone();
      }
    }
    15 | 16 => {
      let value =
        if kind == 15 { &mut settings.exp_multiplier } else { &mut settings.sp_multiplier };
      let choices: &[u32] =
        if kind == 15 { &crate::gains::EXP_MULTIPLIERS } else { &crate::gains::MULTIPLIERS };
      let index = choices.iter().position(|choice| choice == value).unwrap_or(0);
      let next = unsafe { change(index as i32, 0, choices.len() as i32 - 1, 1, NONE) };
      if let Some(choice) = choices.get(next as usize) {
        *value = *choice;
      }
    }
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
    kind @ 8..=14 => {
      let feature = config::Feature::ALL[kind - 8];
      let next = unsafe { change(i32::from(feature.value(&settings)), 0, 1, 1, NONE) };
      if (0..=1).contains(&next) {
        feature.set(&mut settings, next == 1);
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
  unsafe {
    refresh(item);
  }
  if saved {
    4096
  } else {
    2048
  }
}

pub(crate) unsafe fn refresh(item: Object) {
  unsafe {
    command(item, NONE);
    let update: unsafe extern "C" fn(Object, Method) = core::mem::transmute(base() + UPDATE_TEXT);
    update(item, NONE);
  }
}
