//! Select an existing joined chef and native taste; never rewrite cooking tables.

pub const CURRENT_CHEF: usize = 0x02A64450;
pub const GET_COOK_DATA: usize = 0x02D86CE0;
pub const COOK: usize = 0x025447B0;
pub const DETERMINE_TASTE: usize = 0x02544B30;
pub const COOK_DATA: usize = 0x001FD900;
pub const TASTE_DATA: usize = 0x00206E50;
pub const UNIT_LIST: usize = 0x02393F60;
pub const UNIT_PID: usize = 0x01A35BF0;
pub const UNIT_NAME: usize = 0x01A22770;
pub const ANIMATION_CREATE: usize = 0x02D82720;
pub const ANIMATION_MOVE_NEXT: usize = 0x01CE3CF0;
pub const STAT_NAME: usize = 0x025BCED0;
pub const GRADE_NAME: usize = 0x021DF2E0;
pub const TASTE_NAME: usize = 0x021DF0C0;
pub const IS_CONDITIONAL: usize = 0x021DF130;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02A64450, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD001C5B4]),
  (0x02D86CE0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x025447B0, [0xA9BB7BFD, 0xF9000BF9, 0x910003FD, 0xA9025FF8]),
  (0x02544B30, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x001FD900, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xA90257F6]),
  (0x00206E50, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xA90257F6]),
  (0x02393F60, [0xD10303FF, 0xA9067BFD, 0x910183FD, 0xA9076FFC]),
  (0x01A35BF0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD0024294]),
  (0x01A22770, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x01CE3CF0, [0xFC1C0FE8, 0xA900FBFD, 0x910023FD, 0xF9000FF7]),
  (0x02D82720, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02D81260, [0xD10183FF, 0xA9017BFD, 0x910043FD, 0xF90013F9]),
  (0x025BCED0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x021DF2E0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x900209D4]),
  (0x021DF130, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x021DF0C0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x021DF0D0, [0x900209D5, 0xB0020534, 0x395A06A8, 0xF9435294]),
  (0x021DF0E0, [0xAA0003F3, 0x370000C8, 0xB0020520, 0xF9435000]),
  (0x021DF0F0, [0x978ACBDC, 0x52800028, 0x391A06A8, 0xF9400280]),
  (0x021DF100, [0xF9401673, 0x3944CC08, 0x36080088, 0xB940E008]),
  (0x021DF110, [0x35000048, 0x978ACC1B, 0xAA1303E0, 0xA9424FF4]),
  (0x021DF120, [0xF9400BF5, 0xAA1F03E1, 0xA8C37BFD, 0x140FA845]),
];

pub fn preview_labels(language: i32) -> [&'static str; 3] {
  match language {
    0 => ["料理の基本能力", "その他の能力", "条件で変化"],
    2 | 5 => ["Stats du plat", "Autres stats", "Selon conditions"],
    3 | 6 => ["Atributos del plato", "Otros atributos", "Según condiciones"],
    7 => ["Gerichtswerte", "Andere Werte", "Bedingungsabhängig"],
    8 => ["Statistiche del piatto", "Altre statistiche", "Secondo condizioni"],
    9 => ["料理基礎能力", "其他能力", "依條件變化"],
    10 => ["料理基础能力", "其他能力", "依条件变化"],
    11 => ["요리 기본 능력치", "기타 능력치", "조건에 따라 변경"],
    _ => ["Recipe stats", "Other stats", "Conditional variants"],
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    config,
    meld::native::text,
    native::{base, field, list_count, metadata, object_at, Method, Object},
  };
  use core::{cell::Cell, ptr};
  const NONE: Method = ptr::null();
  std::thread_local! { static COOKING: Cell<usize> = const { Cell::new(0) }; }

  macro_rules! native {
    ($address:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $address)
    };
  }

  pub(crate) fn install() {
    skyline::install_hooks!(current_chef, get_cook_data, cook, determine_taste, animation_create);
  }

  unsafe fn cook_data(pid: Object) -> Object {
    unsafe {
      let method = metadata(0x0628E250);
      if pid.is_null() || method.is_null() {
        return ptr::null_mut();
      }
      native!(COOK_DATA, unsafe extern "C" fn(Object, Method) -> Object)(pid, method)
    }
  }

  pub(crate) unsafe fn chefs() -> Vec<(String, String)> {
    unsafe {
      let user = crate::native::user_data();
      if user.is_null() || field::<i32>(user, 0x28) != 4 {
        return Vec::new();
      }
      let list = native!(UNIT_LIST, unsafe extern "C" fn(Method) -> Object)(NONE);
      let Some(count) = list_count(list) else { return Vec::new() };
      let mut result = Vec::new();
      for index in 0..count {
        let Some(unit) = object_at(list, index).filter(|v| !v.is_null()) else { continue };
        let pid = native!(UNIT_PID, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE);
        let Some(key) = text(pid) else { continue };
        if cook_data(pid).is_null() || result.iter().any(|(p, _)| *p == key) {
          continue;
        }
        let name = native!(UNIT_NAME, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE);
        result.push((key.clone(), text(name).unwrap_or(key)));
      }
      result
    }
  }

  unsafe fn taste_for(pid: Object, option: u8) -> Option<Object> {
    if !(1..=4).contains(&option) {
      return None;
    }
    unsafe {
      let key = if option == 4 {
        metadata(0x062B5868)
      } else {
        let data = cook_data(pid);
        if data.is_null() {
          return None;
        }
        field(data, 0x28 + usize::from(option - 1) * 8)
      };
      let method = metadata(0x06292CE0);
      if key.is_null() || method.is_null() {
        return None;
      }
      let taste = native!(TASTE_DATA, unsafe extern "C" fn(Object, Method) -> Object)(key, method);
      (!taste.is_null()).then_some(taste)
    }
  }

  unsafe fn preview_taste(option: u8) -> Option<Object> {
    unsafe {
      let user = crate::native::user_data();
      if user.is_null() || field::<i32>(user, 0x28) != 4 {
        return None;
      }
      let pid = native!(CURRENT_CHEF, unsafe extern "C" fn(Method) -> Object)(NONE);
      taste_for(pid, option)
    }
  }

  pub(crate) unsafe fn outcome_name(option: u8) -> Option<String> {
    unsafe { taste_name(preview_taste(option)?) }
  }

  unsafe fn taste_name(taste: Object) -> Option<String> {
    // The stored Name is a message key. Reuse the native display-name resolver.
    unsafe {
      text(native!(TASTE_NAME, unsafe extern "C" fn(Object, Method) -> Object)(taste, NONE))
    }
  }

  pub(crate) unsafe fn outcome_detail(option: u8, language: i32) -> Option<String> {
    unsafe {
      let taste = preview_taste(option)?;
      let labels = preview_labels(language);
      let grade = native!(GRADE_NAME, unsafe extern "C" fn(Object, Method) -> Object)(taste, NONE);
      let mut parts = vec![format!("{} · {}", text(grade)?, taste_name(taste)?)];
      for (offset, label) in [(0x34, labels[0]), (0x35, labels[1])] {
        let value: i8 = field(taste, offset);
        if value != 0 {
          parts.push(format!("{label} {value:+}"));
        }
      }
      let capability: Object = field(taste, 0x38);
      if !capability.is_null() {
        let array: Object = field(capability, 0x10);
        if !array.is_null() {
          for index in 0..field::<usize>(array, 0x18).min(11) {
            let value: i8 = field(array, 0x20 + index);
            if value != 0 {
              let name =
                native!(STAT_NAME, unsafe extern "C" fn(i32, Method) -> Object)(index as i32, NONE);
              if let Some(name) = text(name) {
                parts.push(format!("{name} {value:+}"));
              }
            }
          }
        }
      }
      // The taste has native derived/conditional definitions. Do not claim
      // that a template preview is the final recipe/recipient-specific total.
      if native!(IS_CONDITIONAL, unsafe extern "C" fn(Object, Method) -> bool)(taste, NONE) {
        parts.push(labels[2].to_owned());
      }
      Some(parts.join(" · "))
    }
  }

  #[skyline::hook(offset = ANIMATION_CREATE)]
  fn animation_create(owner: Object, method: Method) -> Object {
    let iterator = call_original!(owner, method);
    unsafe {
      let user = crate::native::user_data();
      if !iterator.is_null()
        && !user.is_null()
        && field::<i32>(user, 0x28) == 4
        && field::<i32>(iterator, 0x10) == 0
        && !owner.is_null()
        && field::<Object>(iterator, 0x20) == owner
        && !field::<Object>(owner, 0xA0).is_null()
      {
        // This hook is the cooking-animation factory itself. Native state -1
        // completes its iterator without skipping cooking/reward settlement.
        iterator.cast::<u8>().add(0x10).cast::<i32>().write(-1);
      }
    }
    iterator
  }

  unsafe fn selected_unit() -> Option<Object> {
    let selected = config::current().chef;
    if selected.is_empty() {
      return None;
    }
    unsafe {
      let user = crate::native::user_data();
      if user.is_null() || field::<i32>(user, 0x28) != 4 {
        return None;
      }
      let list = native!(UNIT_LIST, unsafe extern "C" fn(Method) -> Object)(NONE);
      let count = list_count(list)?;
      for index in 0..count {
        let Some(unit) = object_at(list, index).filter(|v| !v.is_null()) else { continue };
        let pid = native!(UNIT_PID, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE);
        if text(pid).as_deref() == Some(selected.as_str()) && !cook_data(pid).is_null() {
          return Some(unit);
        }
      }
      None
    }
  }

  unsafe fn selected() -> Option<Object> {
    unsafe {
      selected_unit()
        .map(|unit| native!(UNIT_PID, unsafe extern "C" fn(Object, Method) -> Object)(unit, NONE))
    }
  }

  #[skyline::hook(offset = CURRENT_CHEF)]
  fn current_chef(method: Method) -> Object {
    // This is a read override, not SetCurrentCookingPid or a save-variable write.
    unsafe { selected() }.unwrap_or_else(|| call_original!(method))
  }

  #[skyline::hook(offset = GET_COOK_DATA)]
  fn get_cook_data(method: Method) -> Object {
    if let Some(pid) = unsafe { selected() } {
      let data = unsafe { cook_data(pid) };
      if !data.is_null() {
        return data;
      }
    }
    call_original!(method)
  }

  #[skyline::hook(offset = COOK)]
  fn cook(chef: Object, food: Object, ingredients: Object, method: Method) -> Object {
    let chef = unsafe { selected_unit() }.unwrap_or(chef);
    let previous = COOKING.with(|value| value.replace(chef as usize));
    let result = call_original!(chef, food, ingredients, method);
    COOKING.with(|value| value.set(previous));
    result
  }

  #[skyline::hook(offset = DETERMINE_TASTE)]
  fn determine_taste(chef: Object, quality: i32, method: Method) -> Object {
    let option = config::current().cooking_outcome;
    if chef.is_null() || option == 0 || !COOKING.with(|value| value.get() == chef as usize) {
      return call_original!(chef, quality, method);
    }
    if option <= 4 {
      let pid =
        unsafe { native!(UNIT_PID, unsafe extern "C" fn(Object, Method) -> Object)(chef, NONE) };
      if unsafe { taste_for(pid, option) }.is_none() {
        return call_original!(chef, quality, method);
      }
    }
    // Use the chef-specific native branch, including conditional outcomes and
    // ordinary RNG. No arbitrary TasteData from another character is injected.
    call_original!(chef, i32::from(option - 1), method)
  }
}
