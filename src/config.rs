//! In-game settings persisted independently of the game's save variables.

#[cfg(target_os = "switch")]
mod sdk;
pub mod storage;

use core::sync::atomic::{AtomicU32, Ordering};

pub const DEFAULT_DRAW_COUNT: u32 = 300;
pub const MAX_DRAW_COUNT: u32 = 1000;
pub const MAX_USE_COUNT: u32 = 1000;
pub const MAX_CONFIG_BYTES: usize = 4096;
pub const DEFAULT_POLISHER: &str = "PID_リュール";
static DRAW_COUNT: AtomicU32 = AtomicU32::new(DEFAULT_DRAW_COUNT);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
  pub draw_count: u32,
  pub chef: String,
  pub cooking_outcome: u8,
  pub auto_support: bool,
  pub auto_polish: bool,
  pub polish_actor: String,
  /// Zero means nondecreasing, not zero available uses.
  pub arena_count: u32,
  pub fishing_unlimited: bool,
  pub shop_stock: bool,
  /// Shared gift, Sommie food and cooking stock switch; retain its existing key.
  pub gift_stock: bool,
  pub repeat_cooking: bool,
  pub fast_fishing: bool,
  pub fast_training: bool,
  pub fast_wyvern: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Feature {
  FishingUnlimited,
  ShopStock,
  GiftStock,
  RepeatCooking,
  FastFishing,
  FastTraining,
  FastWyvern,
}

impl Feature {
  pub const ALL: [Self; 7] = [
    Self::FishingUnlimited,
    Self::ShopStock,
    Self::GiftStock,
    Self::RepeatCooking,
    Self::FastFishing,
    Self::FastTraining,
    Self::FastWyvern,
  ];

  pub fn key(self) -> &'static str {
    match self {
      Self::FishingUnlimited => "fishing_unlimited",
      Self::ShopStock => "shop_stock",
      Self::GiftStock => "gift_stock",
      Self::RepeatCooking => "repeat_cooking",
      Self::FastFishing => "fast_fishing",
      Self::FastTraining => "fast_training",
      Self::FastWyvern => "fast_wyvern",
    }
  }

  pub fn value(self, settings: &Settings) -> bool {
    match self {
      Self::FishingUnlimited => settings.fishing_unlimited,
      Self::ShopStock => settings.shop_stock,
      Self::GiftStock => settings.gift_stock,
      Self::RepeatCooking => settings.repeat_cooking,
      Self::FastFishing => settings.fast_fishing,
      Self::FastTraining => settings.fast_training,
      Self::FastWyvern => settings.fast_wyvern,
    }
  }

  pub fn set(self, settings: &mut Settings, value: bool) {
    match self {
      Self::FishingUnlimited => settings.fishing_unlimited = value,
      Self::ShopStock => settings.shop_stock = value,
      Self::GiftStock => settings.gift_stock = value,
      Self::RepeatCooking => settings.repeat_cooking = value,
      Self::FastFishing => settings.fast_fishing = value,
      Self::FastTraining => settings.fast_training = value,
      Self::FastWyvern => settings.fast_wyvern = value,
    }
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      draw_count: DEFAULT_DRAW_COUNT,
      chef: String::new(),
      cooking_outcome: 0,
      auto_support: true,
      auto_polish: true,
      polish_actor: DEFAULT_POLISHER.to_owned(),
      arena_count: 10,
      fishing_unlimited: true,
      shop_stock: true,
      gift_stock: true,
      repeat_cooking: true,
      fast_fishing: true,
      fast_training: true,
      fast_wyvern: true,
    }
  }
}

impl Settings {
  pub fn valid(&self) -> bool {
    valid_count(self.draw_count)
      && self.arena_count <= MAX_USE_COUNT
      && self.cooking_outcome <= 5
      && self.polish_actor.starts_with("PID_")
      && self.polish_actor.len() <= 128
      && !self.polish_actor.chars().any(|c| c.is_control() || matches!(c, '=' | '#' | ' '))
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
    if !self.auto_polish {
      result.push_str("auto_polish=0\n");
    }
    if self.polish_actor != DEFAULT_POLISHER {
      result.push_str(&format!("polish_actor={}\n", self.polish_actor));
    }
    if self.arena_count != 10 {
      result.push_str(&format!("arena_count={}\n", self.arena_count));
    }
    for feature in Feature::ALL {
      if !feature.value(self) {
        result.push_str(&format!("{}=0\n", feature.key()));
      }
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

#[cfg(target_os = "switch")]
pub fn auto_polish() -> bool {
  SETTINGS.with(|value| value.borrow().auto_polish)
}

#[cfg(target_os = "switch")]
pub fn polish_actor() -> String {
  SETTINGS.with(|value| value.borrow().polish_actor.clone())
}

#[cfg(target_os = "switch")]
pub fn arena_count() -> u32 {
  SETTINGS.with(|value| value.borrow().arena_count)
}

#[cfg(target_os = "switch")]
pub fn enabled(feature: Feature) -> bool {
  SETTINGS.with(|value| feature.value(&value.borrow()))
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
      // Removed preferences remain readable in existing independent files.
      "exp_multiplier" | "sp_multiplier" | "rewind_count" | "battle_save" | "personal_outfits" => {}
      "draw_count" => {
        settings.draw_count = value.trim().parse().map_err(|_| "invalid draw_count")?
      }
      "chef" => settings.chef = value.trim().to_owned(),
      "polish_actor" => settings.polish_actor = value.trim().to_owned(),
      "arena_count" => {
        settings.arena_count = value.trim().parse().map_err(|_| "invalid arena_count")?
      }
      "cooking_outcome" => {
        settings.cooking_outcome = value.trim().parse().map_err(|_| "invalid cooking_outcome")?
      }
      "auto_support" | "auto_polish" => {
        let enabled = match value.trim() {
          "0" => false,
          "1" => true,
          _ => return Err("invalid automatic action switch"),
        };
        if key == "auto_support" {
          settings.auto_support = enabled;
        } else {
          settings.auto_polish = enabled;
        }
      }
      _ => {
        let feature = Feature::ALL
          .into_iter()
          .find(|feature| feature.key() == key)
          .ok_or("unknown configuration key")?;
        let enabled = match value.trim() {
          "0" => false,
          "1" => true,
          _ => return Err("invalid switch value"),
        };
        feature.set(&mut settings, enabled);
      }
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
  if !settings.valid() {
    return Err(storage::Error);
  }
  let old = current();
  let enable_fishing = !old.fishing_unlimited && settings.fishing_unlimited;
  if !crate::preferences::native::apply(&settings) {
    return Err(storage::Error);
  }
  let saved = (|| storage::save_settings(&mut sdk::SdCard::mount()?, &settings))();
  if saved.is_err() {
    crate::preferences::native::apply(&old);
    return Err(storage::Error);
  }
  DRAW_COUNT.store(settings.draw_count, Ordering::Relaxed);
  SETTINGS.with(|value| *value.borrow_mut() = settings);
  if enable_fishing {
    crate::fishing::native::ensure_allowance();
  }
  Ok(())
}
