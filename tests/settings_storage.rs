use fee_better_somniel::config::storage::{load, save, Error, File, Storage};
use std::collections::BTreeMap;

#[derive(Default, Clone)]
struct Memory {
  files: BTreeMap<File, Vec<u8>>,
  calls: usize,
  fail: usize,
}

impl Memory {
  fn step(&mut self) -> Result<(), Error> {
    self.calls += 1;
    if self.calls == self.fail {
      Err(Error)
    } else {
      Ok(())
    }
  }
}

impl Storage for Memory {
  fn read(&mut self, file: File) -> Result<Option<Vec<u8>>, Error> {
    self.step()?;
    Ok(self.files.get(&file).cloned())
  }
  fn write_pending(&mut self, contents: &[u8]) -> Result<(), Error> {
    self.step()?;
    self.files.insert(File::Pending, contents.to_vec());
    Ok(())
  }
  fn remove(&mut self, file: File) -> Result<(), Error> {
    self.step()?;
    self.files.remove(&file);
    Ok(())
  }
  fn rename(&mut self, from: File, to: File) -> Result<(), Error> {
    self.step()?;
    if self.files.contains_key(&to) {
      return Err(Error);
    }
    let bytes = self.files.remove(&from).ok_or(Error)?;
    self.files.insert(to, bytes);
    Ok(())
  }
}

#[test]
fn independent_file_round_trips_without_game_variables() {
  let mut disk = Memory::default();
  assert_eq!(load(&mut disk), None);
  for count in [300, 2, 1000, 10, 527] {
    save(&mut disk, count).unwrap();
    assert_eq!(load(&mut disk), Some(count));
    assert_eq!(disk.files.len(), 1);
    assert!(disk.files.contains_key(&File::Settings));
  }
  let before = disk.files.clone();
  for count in [0, 1, 1001, u32::MAX] {
    assert_eq!(save(&mut disk, count), Err(Error));
  }
  assert_eq!(disk.files, before);
}

#[test]
fn all_preferences_survive_each_transaction_failure() {
  use fee_better_somniel::config::{
    storage::{load_settings, save_settings},
    Settings,
  };
  let old = Settings {
    draw_count: 527,
    chef: "PID_パネトネ".into(),
    cooking_outcome: 3,
    auto_support: false,
    rewind_count: 7,
    battle_save: false,
    personal_outfits: false,
    arena_count: 0,
  };
  let new = Settings {
    draw_count: 612,
    chef: "PID_エーティエ".into(),
    cooking_outcome: 2,
    auto_support: true,
    ..Settings::default()
  };
  let mut initial = Memory::default();
  save_settings(&mut initial, &old).unwrap();
  let mut complete = initial.clone();
  complete.calls = 0;
  save_settings(&mut complete, &new).unwrap();
  for fail in 1..=complete.calls {
    let mut disk = initial.clone();
    disk.calls = 0;
    disk.fail = fail;
    let result = save_settings(&mut disk, &new);
    disk.fail = 0;
    assert_eq!(
      load_settings(&mut disk),
      Some(if result.is_ok() { new.clone() } else { old.clone() }),
      "call {fail}"
    );
  }
}

#[test]
fn every_single_io_failure_retains_a_loadable_committed_value() {
  let mut initial = Memory::default();
  save(&mut initial, 300).unwrap();
  let mut complete = initial.clone();
  complete.calls = 0;
  save(&mut complete, 527).unwrap();
  for fail in 1..=complete.calls {
    let mut disk = initial.clone();
    disk.calls = 0;
    disk.fail = fail;
    let result = save(&mut disk, 527);
    disk.fail = 0;
    assert_eq!(load(&mut disk), Some(if result.is_ok() { 527 } else { 300 }), "call {fail}");
    save(&mut disk, 600).unwrap();
    assert_eq!(load(&mut disk), Some(600));
    assert_eq!(disk.files.len(), 1);
  }
}

#[test]
fn interrupted_rotation_recovers_old_settings_and_next_save_cleans_up() {
  for primary in [None, Some(b"invalid".to_vec())] {
    let mut disk = Memory::default();
    if let Some(primary) = primary {
      disk.files.insert(File::Settings, primary);
    }
    disk.files.insert(File::Recovery, b"draw_count=527\n".to_vec());
    disk.files.insert(File::Pending, b"draw_count=600\n".to_vec());
    assert_eq!(load(&mut disk), Some(527));
    save(&mut disk, 800).unwrap();
    assert_eq!(load(&mut disk), Some(800));
    assert_eq!(disk.files.len(), 1);
  }
}

#[test]
fn first_save_failures_do_not_promote_an_uncommitted_temporary_value() {
  let mut complete = Memory::default();
  save(&mut complete, 527).unwrap();
  for fail in 1..=complete.calls {
    let mut disk = Memory { fail, ..Memory::default() };
    let saved = save(&mut disk, 527).is_ok();
    disk.fail = 0;
    assert_eq!(load(&mut disk), if saved { Some(527) } else { None });
  }
}
