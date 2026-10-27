//! Small recoverable settings transaction. These are mod files, not game saves.

use super::{parse_settings, Settings};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
  Settings,
  Pending,
  Recovery,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error;

pub trait Storage {
  fn read(&mut self, file: File) -> Result<Option<Vec<u8>>, Error>;
  fn write_pending(&mut self, contents: &[u8]) -> Result<(), Error>;
  fn remove(&mut self, file: File) -> Result<(), Error>;
  fn rename(&mut self, from: File, to: File) -> Result<(), Error>;
}

fn value(bytes: &[u8]) -> Option<Settings> {
  parse_settings(core::str::from_utf8(bytes).ok()?).ok()
}

pub fn load(storage: &mut impl Storage) -> Option<u32> {
  load_settings(storage).map(|value| value.draw_count)
}

pub fn load_settings(storage: &mut impl Storage) -> Option<Settings> {
  if let Some(bytes) = storage.read(File::Settings).ok()? {
    if let Some(count) = value(&bytes) {
      return Some(count);
    }
  }
  value(&storage.read(File::Recovery).ok()??)
}

pub fn save(storage: &mut impl Storage, count: u32) -> Result<(), Error> {
  save_settings(storage, &Settings { draw_count: count, ..Settings::default() })
}

pub fn save_settings(storage: &mut impl Storage, settings: &Settings) -> Result<(), Error> {
  if !settings.valid() {
    return Err(Error);
  }
  let current = storage.read(File::Settings)?;
  let recovery = storage.read(File::Recovery)?;
  // A shutdown between the two renames leaves the last committed settings
  // here. Restore them before starting another transaction.
  if recovery.as_deref().and_then(value).is_some() && current.as_deref().and_then(value).is_none() {
    storage.remove(File::Settings)?;
    storage.rename(File::Recovery, File::Settings)?;
  }
  let had_current = storage.read(File::Settings)?.is_some();
  storage.remove(File::Pending)?;
  let contents = settings.encode();
  storage.write_pending(contents.as_bytes())?;
  if storage.read(File::Pending)?.as_deref() != Some(contents.as_bytes()) {
    return Err(Error);
  }
  storage.remove(File::Recovery)?;
  if had_current {
    storage.rename(File::Settings, File::Recovery)?;
  }
  if storage.rename(File::Pending, File::Settings).is_err() {
    if had_current {
      // A failed rollback still leaves the old, loadable Recovery file.
      let _ = storage.rename(File::Recovery, File::Settings);
    }
    return Err(Error);
  }
  // Cleanup is not part of commit: the new Settings file is already durable.
  let _ = storage.remove(File::Recovery);
  Ok(())
}
