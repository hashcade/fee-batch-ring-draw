//! In-game settings persisted independently of the game's save variables.

#[cfg(target_os = "switch")]
mod sdk;
pub mod storage;

use core::sync::atomic::{AtomicU32, Ordering};

pub const DEFAULT_DRAW_COUNT: u32 = 300;
pub const MAX_DRAW_COUNT: u32 = 1000;
pub const MAX_CONFIG_BYTES: usize = 4096;
static DRAW_COUNT: AtomicU32 = AtomicU32::new(DEFAULT_DRAW_COUNT);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
  pub draw_count: u32,
  pub chef: String,
  pub cooking_outcome: u8,
  pub auto_support: bool,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      draw_count: DEFAULT_DRAW_COUNT,
      chef: String::new(),
      cooking_outcome: 0,
      auto_support: true,
    }
  }
}

impl Settings {
  pub fn valid(&self) -> bool {
    valid_count(self.draw_count)
      && self.cooking_outcome <= 5
      && (self.chef.is_empty()
        || self.chef.starts_with("PID_")
          && self.chef.len() <= 128
          && !self.chef.chars().any(|c| c.is_control() || matches!(c, '=' | '#' | ' ')))
  }

  pub fn encode(&self) -> String {
    let mut result = format!("draw_count={}\n", self.draw_count);
    if !self.chef.is_empty() {
      result.push_str(&format!("chef={}\n", self.chef));
    }
    if self.cooking_outcome != 0 {
      result.push_str(&format!("cooking_outcome={}\n", self.cooking_outcome));
    }
    if !self.auto_support {
      result.push_str("auto_support=0\n");
    }
    result
  }
}

#[cfg(target_os = "switch")]
std::thread_local! {
  static SETTINGS: core::cell::RefCell<Settings> = core::cell::RefCell::new(Settings::default());
}

#[cfg(target_os = "switch")]
pub fn current() -> Settings {
  SETTINGS.with(|value| value.borrow().clone())
}

#[cfg(target_os = "switch")]
pub fn auto_support() -> bool {
  SETTINGS.with(|value| value.borrow().auto_support)
}

pub fn valid_count(count: u32) -> bool {
  (2..=MAX_DRAW_COUNT).contains(&count)
}

pub fn draw_count() -> u32 {
  DRAW_COUNT.load(Ordering::Relaxed)
}

pub fn parse(contents: &str) -> Result<u32, &'static str> {
  parse_settings(contents).map(|settings| settings.draw_count)
}

pub fn parse_settings(contents: &str) -> Result<Settings, &'static str> {
  if contents.len() > MAX_CONFIG_BYTES {
    return Err("configuration exceeds 4096 bytes");
  }
  let mut settings = Settings::default();
  let mut seen = std::collections::BTreeSet::new();
  for line in contents.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    let (key, value) = line.split_once('=').ok_or("expected key=value")?;
    let key = key.trim();
    if !seen.insert(key) {
      return Err("duplicate configuration key");
    }
    match key {
      "draw_count" => {
        settings.draw_count = value.trim().parse().map_err(|_| "invalid draw_count")?
      }
      "chef" => settings.chef = value.trim().to_owned(),
      "cooking_outcome" => {
        settings.cooking_outcome = value.trim().parse().map_err(|_| "invalid cooking_outcome")?
      }
      "auto_support" => {
        settings.auto_support = match value.trim() {
          "0" => false,
          "1" => true,
          _ => return Err("invalid auto_support"),
        }
      }
      _ => return Err("unknown configuration key"),
    }
  }
  if !settings.valid() {
    return Err("invalid settings");
  }
  Ok(settings)
}

#[cfg(target_os = "switch")]
pub fn load() {
  let settings = sdk::SdCard::mount()
    .ok()
    .and_then(|mut sd| storage::load_settings(&mut sd))
    .unwrap_or_default();
  DRAW_COUNT.store(settings.draw_count, Ordering::Relaxed);
  SETTINGS.with(|value| *value.borrow_mut() = settings);
}

#[cfg(target_os = "switch")]
pub fn set_draw_count(count: u32) -> Result<(), storage::Error> {
  let mut settings = current();
  settings.draw_count = count;
  set(settings)
}

#[cfg(target_os = "switch")]
pub fn set(settings: Settings) -> Result<(), storage::Error> {
  storage::save_settings(&mut sdk::SdCard::mount()?, &settings)?;
  DRAW_COUNT.store(settings.draw_count, Ordering::Relaxed);
  SETTINGS.with(|value| *value.borrow_mut() = settings);
  Ok(())
}
