//! Explicit pending-well preview/reseed actions; never grant previewed items.

pub const CREATE_MENU: usize = 0x0293C6A0;
pub const USE_FLAG: usize = 0x02939950;
pub const EXCHANGE_LEVEL: usize = 0x02939C90;
pub const GET_SEED: usize = 0x02939FD0;
pub const SET_SEED: usize = 0x0293A100;
pub const CALCULATE: usize = 0x0293C890;
pub const SYSTEM_RANDOM: usize = 0x023746E0;
pub const RANDOM_VALUE: usize = 0x023748D0;
pub const RANDOM_CTOR: usize = 0x02374F10;
pub const MESSAGE: usize = 0x02270D00;

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x0293C6A0, [0xA9BA7BFD, 0xA9016FFC, 0x910003FD, 0xA90267FA]),
  (0x02939950, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xF90013F5]),
  (0x02939C90, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xF90013F5]),
  (0x02939FD0, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xF90013F5]),
  (0x0293A100, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xA90257F6]),
  (0x0293C890, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x023746E0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x9001FD33]),
  (0x023748D0, [0x29422408, 0x4A082D08, 0xB9001009, 0x29432809]),
  (0x02374F10, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x2A0103F3]),
  (0x02270D00, [0x52800022, 0x52800063, 0x140037B2, 0xD503201F]),
  (0x0293CBF0, [0x97FFF428, 0x2A0003F4, 0x97FFF4F6, 0xF001CA48]),
  (0x0293CC20, [0x2A1403E1, 0xAA1603E2, 0x97FFFF1A, 0xB4000100]),
  (0x027AAC10, [0xF9401400, 0xD65F03C0, 0xD503201F, 0xD503201F]),
];

pub fn available(flag: i32, level: i32) -> bool {
  matches!(flag, 1 | 2) && (1..=5).contains(&level)
}

pub fn label(language: i32, reroll: bool) -> &'static str {
  let labels = match language {
    0 => ["交換結果を確認", "交換結果を再抽選"],
    2 | 5 => ["Aperçu des résultats", "Relancer les résultats"],
    3 | 6 => ["Ver resultados", "Volver a sortear"],
    7 => ["Ergebnisse ansehen", "Ergebnisse neu auslosen"],
    8 => ["Anteprima risultati", "Estrai nuovi risultati"],
    9 => ["預覽交換結果", "重新抽取結果"],
    10 => ["预览交换结果", "重新抽取结果"],
    11 => ["교환 결과 미리 보기", "교환 결과 다시 뽑기"],
    _ => ["Preview Results", "Reroll Results"],
  };
  labels[usize::from(reroll)]
}

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    meld::native::{new_text, text, text_language},
    native::{base, field, list_count, metadata, object_at, Method, Object},
    profile,
  };
  use core::{
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
  };

  const NONE: Method = ptr::null();
  static PREVIEW_CLASS: AtomicUsize = AtomicUsize::new(0);
  static REROLL_CLASS: AtomicUsize = AtomicUsize::new(0);

  macro_rules! native {
    ($offset:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $offset)
    };
  }

  pub(crate) fn install() {
    skyline::install_hooks!(well_create_menu);
  }

  unsafe fn pending() -> bool {
    unsafe {
      let flag = native!(USE_FLAG, unsafe extern "C" fn(Method) -> i32)(NONE);
      let level = native!(EXCHANGE_LEVEL, unsafe extern "C" fn(Method) -> i32)(NONE);
      available(flag, level)
    }
  }

  unsafe extern "C" fn attribute(_: Object, _: Method) -> i32 {
    if unsafe { pending() } {
      1
    } else {
      2
    }
  }

  unsafe extern "C" fn name(item: Object, _: Method) -> Object {
    let reroll = unsafe { field::<usize>(item, 0) } == REROLL_CLASS.load(Ordering::Relaxed);
    new_text(label(text_language(), reroll))
  }

  #[skyline::hook(offset = CREATE_MENU)]
  fn well_create_menu(parent: Object, selected: i32, handler: Object, method: Method) {
    call_original!(parent, selected, handler, method);
    if parent.is_null() {
      return;
    }
    unsafe {
      let menu: Object = field(parent, 0x38);
      if menu.is_null() {
        return;
      }
      let list: Object = field(menu, 0x80);
      let Some(count) = list_count(list) else { return };
      // Never append twice to a reused menu.
      for index in 0..count {
        let Some(item) = object_at(list, index) else { return };
        let class: usize = field(item, 0);
        if class == PREVIEW_CLASS.load(Ordering::Relaxed)
          || class == REROLL_CLASS.load(Ordering::Relaxed)
        {
          return;
        }
      }
      let Some(first) = object_at(list, 0) else { return };
      let mut items = Vec::new();
      for cache in [&PREVIEW_CLASS, &REROLL_CLASS] {
        let Some(item) = crate::menu_item::create(
          first,
          cache,
          &[
            (4, name as *const () as usize),
            (8, attribute as *const () as usize),
            (18, activate as *const () as usize),
          ],
        ) else {
          return;
        };
        items.push(item);
      }
      let add_method = metadata(0x06286A70);
      if add_method.is_null() {
        return;
      }
      let add = native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method));
      for item in items {
        add(list, item, add_method);
      }
      if list_count(list) == Some(count + 2) {
        native!(profile::SET_MENU_ROWS, unsafe extern "C" fn(Object, i32, Method))(
          menu,
          (count + 2) as i32,
          NONE,
        );
      }
    }
  }

  unsafe fn preview(sequence: Object) -> Option<Object> {
    unsafe {
      let class = metadata(0x06287BF8);
      if class.is_null() {
        return None;
      }
      let random = native!(profile::ALLOCATE_OBJECT, unsafe extern "C" fn(Object) -> Object)(class);
      if random.is_null() {
        return None;
      }
      let seed = native!(GET_SEED, unsafe extern "C" fn(Method) -> i32)(NONE);
      native!(RANDOM_CTOR, unsafe extern "C" fn(Object, u32, Method))(random, seed as u32, NONE);
      let level = native!(EXCHANGE_LEVEL, unsafe extern "C" fn(Method) -> i32)(NONE);
      let items = native!(CALCULATE, unsafe extern "C" fn(Object, i32, Object, Method) -> Object)(
        sequence, level, random, NONE,
      );
      let count = list_count(items)?;
      let mut grouped: Vec<(Object, usize)> = Vec::new();
      for index in 0..count {
        let item = object_at(items, index)?;
        if let Some((_, count)) = grouped.iter_mut().find(|(key, _)| *key == item) {
          *count += 1;
        } else {
          grouped.push((item, 1));
        }
      }
      let message =
        native!(crate::meld::profile::MESS_GET, unsafe extern "C" fn(Object, Method) -> Object);
      let mut lines = Vec::new();
      for (item, count) in grouped {
        let key: Object = field(item, 0x28);
        if key.is_null() {
          return None;
        }
        let name = text(message(key, NONE))?;
        lines.push(format!("{name} ×{count}"));
      }
      let result = new_text(&lines.join("\n"));
      if result.is_null() {
        None
      } else {
        Some(result)
      }
    }
  }

  unsafe extern "C" fn activate(item: Object, _: Method) -> i32 {
    unsafe {
      if item.is_null() || !pending() {
        return 2048;
      }
      let menu: Object = field(item, 0x10);
      if menu.is_null() || !field::<Object>(menu, 0x38).is_null() {
        return 2048;
      }
      let sequence: Object = field(menu, 0x30);
      if sequence.is_null() {
        return 2048;
      }
      let reroll = field::<usize>(item, 0) == REROLL_CLASS.load(Ordering::Relaxed);
      let old_seed = native!(GET_SEED, unsafe extern "C" fn(Method) -> i32)(NONE);
      if reroll {
        let system = native!(SYSTEM_RANDOM, unsafe extern "C" fn(Method) -> Object)(NONE);
        if system.is_null() {
          return 2048;
        }
        let mut seed =
          native!(RANDOM_VALUE, unsafe extern "C" fn(Object, Method) -> i32)(system, NONE);
        if seed == old_seed {
          seed ^= 1;
        }
        native!(SET_SEED, unsafe extern "C" fn(i32, Method))(seed, NONE);
      }
      let Some(result) = preview(sequence) else {
        if reroll {
          native!(SET_SEED, unsafe extern "C" fn(i32, Method))(old_seed, NONE);
        }
        return 2048;
      };
      let dialog = native!(MESSAGE, unsafe extern "C" fn(Object, Object, Method) -> Object)(
        menu, result, NONE,
      );
      if dialog.is_null() {
        if reroll {
          native!(SET_SEED, unsafe extern "C" fn(i32, Method))(old_seed, NONE);
        }
        return 2048;
      }
      128
    }
  }
}
