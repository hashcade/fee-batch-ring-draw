//! Optional, bounded key=value configuration, read once at game startup.

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
  use skyline::nn::fs;
  const MOUNT: &[u8] = b"feesomniel\0";
  const PATH: &[u8] = b"feesomniel:/fee-better-somniel.cfg\0";

  fn read() -> Option<u32> {
    let mut handle = fs::FileHandle { handle: 0 };
    // SDK paths are NUL-terminated. Only successfully opened handles are used.
    unsafe {
      if fs::OpenFile(&mut handle, PATH.as_ptr(), fs::OpenMode_OpenMode_Read as i32) != 0 {
        return None;
      }
      let mut size = 0i64;
      let result = fs::GetFileSize(&mut size, handle);
      if result != 0 || !(0..=MAX_CONFIG_BYTES as i64).contains(&size) {
        fs::CloseFile(handle);
        return None;
      }
      let mut bytes = [0u8; MAX_CONFIG_BYTES];
      let mut read = 0u64;
      let result = if size == 0 {
        0
      } else {
        fs::ReadFile1(&mut read, handle, 0, bytes.as_mut_ptr(), size as u64)
      };
      fs::CloseFile(handle);
      if result != 0 || read != size as u64 {
        return None;
      }
      parse(core::str::from_utf8(&bytes[..size as usize]).ok()?).ok()
    }
  }

  // Use an isolated mount name and release it on every successful-mount path.
  let count = if unsafe { fs::MountSdCard(MOUNT.as_ptr()) } == 0 {
    let count = read().unwrap_or(DEFAULT_DRAW_COUNT);
    unsafe {
      fs::Unmount(MOUNT.as_ptr());
    }
    count
  } else {
    DEFAULT_DRAW_COUNT
  };
  DRAW_COUNT.store(count, Ordering::Relaxed);
}
