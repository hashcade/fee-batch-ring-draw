//! In-game settings persisted independently of the game's save variables.

#[cfg(target_os = "switch")]
mod sdk;
pub mod storage;

use core::sync::atomic::{AtomicU32, Ordering};

pub const DEFAULT_DRAW_COUNT: u32 = 300;
pub const MAX_DRAW_COUNT: u32 = 1000;
pub const MAX_CONFIG_BYTES: usize = 4096;
static DRAW_COUNT: AtomicU32 = AtomicU32::new(DEFAULT_DRAW_COUNT);

pub fn valid_count(count: u32) -> bool {
  (2..=MAX_DRAW_COUNT).contains(&count)
}

pub fn draw_count() -> u32 {
  DRAW_COUNT.load(Ordering::Relaxed)
}

pub fn parse(contents: &str) -> Result<u32, &'static str> {
  if contents.len() > MAX_CONFIG_BYTES {
    return Err("configuration exceeds 4096 bytes");
  }
  let mut count = None;
  for line in contents.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    let (key, value) = line.split_once('=').ok_or("expected key=value")?;
    if key.trim() != "draw_count" {
      return Err("unknown configuration key");
    }
    if count.is_some() {
      return Err("duplicate draw_count");
    }
    let value: u32 = value.trim().parse().map_err(|_| "invalid draw_count")?;
    if !valid_count(value) {
      return Err("draw_count must be between 2 and 1000");
    }
    count = Some(value);
  }
  Ok(count.unwrap_or(DEFAULT_DRAW_COUNT))
}

#[cfg(target_os = "switch")]
pub fn load() {
  let count = sdk::SdCard::mount()
    .ok()
    .and_then(|mut sd| storage::load(&mut sd))
    .unwrap_or(DEFAULT_DRAW_COUNT);
  DRAW_COUNT.store(count, Ordering::Relaxed);
}

#[cfg(target_os = "switch")]
pub fn set_draw_count(count: u32) -> Result<(), storage::Error> {
  storage::save(&mut sdk::SdCard::mount()?, count)?;
  DRAW_COUNT.store(count, Ordering::Relaxed);
  Ok(())
}
