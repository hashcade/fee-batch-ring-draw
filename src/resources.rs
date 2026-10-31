//! One-shot resource targets. Draft values are never persistent preferences.

pub const TARGET_KIND: usize = 17;
pub const FIRST_VALUE_KIND: usize = 18;
pub const VALUE_COUNT: usize = 15;

#[derive(Clone, Copy, Debug)]
pub struct Resource {
  pub maximum: i32,
  pub step: i32,
  pub iid: Option<&'static str>,
}

pub const RESOURCES: [Resource; VALUE_COUNT] = [
  Resource { maximum: 9999, step: 100, iid: None },
  Resource { maximum: 9_999_999, step: 100_000, iid: None },
  Resource { maximum: 9_999_999, step: 100_000, iid: None },
  Resource { maximum: 9999, step: 100, iid: Some("IID_てつの晶石") },
  Resource { maximum: 9999, step: 100, iid: Some("IID_はがねの晶石") },
  Resource { maximum: 9999, step: 100, iid: Some("IID_ぎんの晶石") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_威力") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_必殺") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_技") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_速さ") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_守備") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_魔防") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_命中") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_回避") },
  Resource { maximum: 999, step: 10, iid: Some("IID_素材_必避") },
];

pub fn adjusted(index: usize, current: i32, direction: i32) -> i32 {
  let resource = RESOURCES[index];
  current.saturating_add(direction.signum() * resource.step).clamp(0, resource.maximum)
}

pub fn common_value(values: impl IntoIterator<Item = i32>) -> Option<i32> {
  let mut values = values.into_iter();
  let first = values.next()?;
  values.all(|value| value == first).then_some(first)
}

pub fn number(value: i32) -> String {
  let value = value.to_string();
  let mut result = String::new();
  for (index, digit) in value.chars().enumerate() {
    if index != 0 && (value.len() - index) % 3 == 0 {
      result.push(',');
    }
    result.push(digit);
  }
  result
}

pub fn all_label(language: i32) -> &'static str {
  match language {
    0 => "全員",
    2 | 5 => "Tous",
    3 | 6 => "Todos",
    7 => "Alle",
    8 => "Tutti",
    9 => "全員",
    10 => "全员",
    11 => "전원",
    _ => "All",
  }
}

pub fn mixed_label(language: i32) -> &'static str {
  match language {
    0 => "不一致",
    2 | 5 => "Différents",
    3 | 6 => "Distintos",
    7 => "Unterschiedlich",
    8 => "Diversi",
    9 | 10 => "不一致",
    11 => "서로 다름",
    _ => "Mixed",
  }
}

pub fn help(language: i32, index: Option<usize>) -> String {
  let Some(index) = index else {
    return match language {
      0 => "全員または加入済みの仲間を選択。数値は次の行で変更します。",
      9 => "選擇全員或已加入角色，在下一行調整SP。",
      10 => "选择全员或已加入角色，在下一行调整SP。",
      _ => "Choose all joined allies or one ally; adjust SP on the next row.",
    }
    .to_owned();
  };
  let resource = RESOURCES[index];
  let (maximum, step) = (number(resource.maximum), number(resource.step));
  match language {
    0 => format!("上限{maximum}・刻み{step}。左右で調整、Aで適用。固定しません。"),
    9 => format!("上限{maximum}・步長{step}。左右調整，A套用一次，不鎖定。"),
    10 => format!("上限{maximum}・步长{step}。左右调整，A应用一次，不锁定。"),
    _ => format!("Max {maximum} · Step {step}. Left/right: adjust. A: apply once, not lock."),
  }
}

pub mod profile;
mod titles;
pub use titles::title;

#[cfg(target_os = "switch")]
pub(crate) mod native;
