//! One-shot resource targets. Draft values are never persistent preferences.

pub const TARGET_KIND: usize = 17;
pub const FIRST_VALUE_KIND: usize = 18;
pub const VALUE_COUNT: usize = 15;

pub fn is_setting(kind: usize) -> bool {
  (TARGET_KIND..FIRST_VALUE_KIND + VALUE_COUNT).contains(&kind)
}

pub fn is_value(kind: usize) -> bool {
  (FIRST_VALUE_KIND..FIRST_VALUE_KIND + VALUE_COUNT).contains(&kind)
}

pub fn apply_hint(language: i32) -> &'static str {
  match language {
    0 => "Aボタンで適用",
    2 | 5 => "A pour appliquer",
    3 | 6 => "Pulsa A para aplicar",
    7 => "Mit A anwenden",
    8 => "Premi A per applicare",
    9 => "按 A 鍵套用",
    10 => "按 A 键应用",
    11 => "A로 적용",
    _ => "Press A to apply",
  }
}

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

pub mod profile;
mod titles;
pub use titles::title;

#[cfg(target_os = "switch")]
pub(crate) mod native;
