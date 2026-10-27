//! SDK-only filesystem access, restricted to the mod's three settings paths.

use super::{storage::*, MAX_CONFIG_BYTES};
use skyline::nn::fs;

const MOUNT: &[u8] = b"feesomniel\0";
const NOT_FOUND: u32 = 0x202;

pub struct SdCard;

impl SdCard {
  pub fn mount() -> Result<Self, Error> {
    if unsafe { fs::MountSdCard(MOUNT.as_ptr()) } == 0 {
      Ok(Self)
    } else {
      Err(Error)
    }
  }
}

impl Drop for SdCard {
  fn drop(&mut self) {
    unsafe { fs::Unmount(MOUNT.as_ptr()) };
  }
}

fn path(file: File) -> *const u8 {
  match file {
    File::Settings => b"feesomniel:/fee-better-somniel.cfg\0".as_ptr(),
    File::Pending => b"feesomniel:/fee-better-somniel.cfg.tmp\0".as_ptr(),
    File::Recovery => b"feesomniel:/fee-better-somniel.cfg.previous\0".as_ptr(),
  }
}

struct Handle(fs::FileHandle);
impl Drop for Handle {
  fn drop(&mut self) {
    unsafe { fs::CloseFile(self.0) };
  }
}

impl Storage for SdCard {
  fn read(&mut self, file: File) -> Result<Option<Vec<u8>>, Error> {
    let mut handle = fs::FileHandle { handle: 0 };
    let result =
      unsafe { fs::OpenFile(&mut handle, path(file), fs::OpenMode_OpenMode_Read as i32) };
    if result == NOT_FOUND {
      return Ok(None);
    }
    if result != 0 {
      return Err(Error);
    }
    let handle = Handle(handle);
    let mut size = 0i64;
    if unsafe { fs::GetFileSize(&mut size, handle.0) } != 0
      || !(0..=MAX_CONFIG_BYTES as i64).contains(&size)
    {
      return Err(Error);
    }
    let mut contents = vec![0; size as usize];
    let mut read = 0u64;
    if size > 0
      && (unsafe { fs::ReadFile1(&mut read, handle.0, 0, contents.as_mut_ptr(), size as u64) } != 0
        || read != size as u64)
    {
      return Err(Error);
    }
    Ok(Some(contents))
  }

  fn write_pending(&mut self, contents: &[u8]) -> Result<(), Error> {
    if contents.len() > MAX_CONFIG_BYTES
      || unsafe { fs::CreateFile(path(File::Pending), contents.len() as i64) } != 0
    {
      return Err(Error);
    }
    let mut handle = fs::FileHandle { handle: 0 };
    if unsafe { fs::OpenFile(&mut handle, path(File::Pending), fs::OpenMode_OpenMode_Write as i32) }
      != 0
    {
      return Err(Error);
    }
    let handle = Handle(handle);
    let options = fs::WriteOption { flags: fs::WriteOptionFlag_WriteOptionFlag_Flush as i32 };
    let result =
      unsafe { fs::WriteFile(handle.0, 0, contents.as_ptr(), contents.len() as u64, &options) };
    if result == 0 {
      Ok(())
    } else {
      Err(Error)
    }
  }

  fn remove(&mut self, file: File) -> Result<(), Error> {
    match unsafe { fs::DeleteFile(path(file)) } {
      0 | NOT_FOUND => Ok(()),
      _ => Err(Error),
    }
  }

  fn rename(&mut self, from: File, to: File) -> Result<(), Error> {
    if unsafe { fs::RenameFile(path(from), path(to)) } == 0 {
      Ok(())
    } else {
      Err(Error)
    }
  }
}
