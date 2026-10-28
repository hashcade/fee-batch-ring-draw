//! Facility-scoped, once-only cleaning of the native eligible selection list.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Candidate {
  pub god: usize,
  pub kind: i32,
  pub dirty: u8,
}

#[derive(Default)]
pub struct Session {
  owner: usize,
  rings: Vec<Candidate>,
}

impl Session {
  pub fn prepare(&mut self, owner: usize, candidates: impl IntoIterator<Item = Candidate>) -> bool {
    self.clear();
    if owner == 0 {
      return false;
    }
    let mut seen = std::collections::BTreeSet::new();
    self.rings = candidates
      .into_iter()
      .filter(|ring| {
        ring.god != 0 && (-1..=3).contains(&ring.kind) && ring.dirty > 0 && seen.insert(ring.god)
      })
      .take(128)
      .collect();
    if self.rings.is_empty() {
      return false;
    }
    self.owner = owner;
    true
  }

  pub fn take(&mut self, owner: usize) -> Option<Vec<Candidate>> {
    if owner == 0 || owner != self.owner {
      self.clear();
      return None;
    }
    self.owner = 0;
    Some(core::mem::take(&mut self.rings))
  }

  pub fn clear(&mut self) {
    self.owner = 0;
    self.rings.clear();
  }
}

pub fn label(language: i32) -> &'static str {
  match language {
    0 => "まとめて磨く",
    2 | 5 => "Tout polir",
    3 | 6 => "Pulir todos",
    7 => "Alle polieren",
    8 => "Lucida tutti",
    9 => "全部清潔",
    10 => "全部清洁",
    11 => "모두 닦기",
    _ => "Polish All",
  }
}

pub const CREATE_MENU: usize = 0x0241A430;
pub const CREATE_CLEANING: usize = 0x0241AF90;
pub const END_SEQUENCE: usize = 0x0241C110;
pub const ITEM_NAME: usize = 0x023472D0;
pub const ITEM_CTOR: usize = 0x02347110;
pub const ITEM_DECIDE: usize = 0x02347210;
pub const GET_DIRTY: usize = 0x0233EB00;
pub const SET_DIRTY: usize = 0x0233EB10;
pub const GET_STATE: usize = 0x023353F0;
pub const GET_GOD: usize = 0x0241C8B0;
pub const SET_GOD: usize = 0x0241C900;
pub const GET_TYPE: usize = 0x0241C960;
pub const SET_TYPE: usize = 0x0241C9B0;
pub const GET_UNIT: usize = 0x0241CA10;
pub const ADD_BOND_EXP: usize = 0x0241FB10;

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x0241A430, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x0241AF90, [0xD10203FF, 0xA9027BFD, 0x910083FD, 0xF9001BFB]),
  (0x0241C110, [0xA9BB7BFD, 0xF9000BF9, 0x910003FD, 0xA9025FF8]),
  (0x023472D0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02347110, [0xA9BB7BFD, 0xF9000BF9, 0x910003FD, 0xA9025FF8]),
  (0x02347210, [0xA9BF7BFD, 0x910003FD, 0xAA0003E8, 0xF9403C00]),
  (0x0233EB00, [0x39413000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x0233EB10, [0x39013001, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x023353F0, [0xF9401408, 0xB40000A8, 0xF9402908, 0xB40000E8]),
  (0x0241C8B0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x9001F7F3]),
  (0x0241C900, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x0241C960, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x9001F7F3]),
  (0x0241C9B0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x0241CA10, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x9001F7F3]),
  (0x0241FB10, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x0241AE44, [0xAA1303E0, 0x94000052, 0xF9400280, 0x3944CC08]),
  (0x0241BFA4, [0x52800028, 0xB81FC3A8, 0xD001F4A8, 0xF946F508]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    meld::native::{new_text, text_language},
    native::{base, field, list_count, metadata, object_at, reference, Method, Object},
    profile,
  };
  use core::{
    cell::RefCell,
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
  };

  const VTABLE: usize = 0x138;
  const NONE: Method = ptr::null();
  static ITEM_CLASS: AtomicUsize = AtomicUsize::new(0);
  std::thread_local! { static SESSION: RefCell<Session> = RefCell::new(Session::default()); }

  macro_rules! call {
    ($offset:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $offset)
    };
  }

  pub(crate) fn install() {
    skyline::install_hooks!(cleaning_menu, cleaning_create, cleaning_end, cleaning_name);
  }

  unsafe fn dirty(god: Object) -> u8 {
    unsafe { call!(GET_DIRTY, unsafe extern "C" fn(Object, Method) -> u8)(god, NONE) }
  }

  unsafe fn candidates(menu: Object) -> Vec<Candidate> {
    if menu.is_null() {
      return Vec::new();
    }
    let list: Object = unsafe { field(menu, 0x80) };
    let Some(count) = (unsafe { list_count(list) }).filter(|n| *n <= 128) else {
      return Vec::new();
    };
    (0..count)
      .filter_map(|index| unsafe {
        let item = object_at(list, index)?;
        let class: Object = field(item, 0);
        if class.is_null()
          || !(0x88..=0x100).contains(&field::<u32>(class, 0xF8))
          || !(27..=128).contains(&field::<u16>(class, 0x126))
        {
          return None;
        }
        let decide: usize = field(class, VTABLE + 18 * 16);
        if decide != base() + ITEM_DECIDE {
          return None;
        }
        let god: Object = field(item, 0x68);
        let kind: i32 = field(item, 0x84);
        if god.is_null() || !(-1..=3).contains(&kind) {
          return None;
        }
        Some(Candidate { god: god as usize, kind, dirty: dirty(god) })
      })
      .collect()
  }

  unsafe extern "C" fn name(_: Object, _: Method) -> Object {
    new_text(label(text_language()))
  }

  unsafe extern "C" fn attribute(item: Object, _: Method) -> i32 {
    let menu: Object = unsafe { field(item, 0x10) };
    if unsafe { candidates(menu) }.iter().any(|ring| ring.dirty > 0) {
      1
    } else {
      2
    }
  }

  unsafe extern "C" fn decide(item: Object, method: Method) -> i32 {
    let menu: Object = unsafe { field(item, 0x10) };
    if menu.is_null() {
      return 2048;
    }
    let owner: Object = unsafe { field(menu, 0x30) };
    if !SESSION.with(|s| s.borrow_mut().prepare(owner as usize, unsafe { candidates(menu) })) {
      return 2048;
    }
    // Preserve the original callback and its transition to cleaner selection.
    unsafe { call!(ITEM_DECIDE, unsafe extern "C" fn(Object, Method) -> i32)(item, method) }
  }

  unsafe fn make_item(first: Object, representative: Candidate) -> Option<Object> {
    let class: Object = unsafe { field(first, 0) };
    if class.is_null() {
      return None;
    }
    let slots: u16 = unsafe { field(class, 0x126) };
    let size: u32 = unsafe { field(class, 0xF8) };
    if !(27..=128).contains(&slots)
      || !(0x88..=0x100).contains(&size)
      || unsafe { field::<usize>(class, VTABLE + 18 * 16) } != base() + ITEM_DECIDE
    {
      return None;
    }
    let allocate =
      unsafe { call!(profile::ALLOCATE_OBJECT, unsafe extern "C" fn(Object) -> Object) };
    let ctor = unsafe {
      call!(
        ITEM_CTOR,
        unsafe extern "C" fn(Object, Object, Object, Object, bool, bool, i32, Method)
      )
    };
    let item = unsafe { allocate(class) };
    if item.is_null() {
      return None;
    }
    unsafe {
      ctor(
        item,
        representative.god as Object,
        field(first, 0x70),
        field(first, 0x78),
        false,
        false,
        representative.kind,
        NONE,
      );
    }
    let mut private = ITEM_CLASS.load(Ordering::Relaxed);
    if private == 0 {
      let words = (VTABLE + usize::from(slots) * 16) / 8;
      let mut copy = unsafe { core::slice::from_raw_parts(class.cast::<usize>(), words) }
        .to_vec()
        .into_boxed_slice();
      for (slot, function) in [
        (4, name as *const () as usize),
        (8, attribute as *const () as usize),
        (18, decide as *const () as usize),
      ] {
        copy[(VTABLE + slot * 16) / 8] = function;
      }
      private = Box::leak(copy).as_mut_ptr() as usize;
      ITEM_CLASS.store(private, Ordering::Relaxed);
    }
    unsafe { item.cast::<usize>().write(private) };
    Some(item)
  }

  #[skyline::hook(offset = CREATE_MENU)]
  fn cleaning_menu(owner: Object, method: Method) {
    SESSION.with(|s| s.borrow_mut().clear());
    call_original!(owner, method);
    if owner.is_null() {
      return;
    }
    unsafe {
      let menu: Object = field(owner, 0x38);
      let Some(representative) = candidates(menu).into_iter().find(|r| r.dirty > 0) else { return };
      let list: Object = field(menu, 0x80);
      let Some(count) = list_count(list).filter(|n| *n > 0 && *n < 128) else { return };
      let Some(first) = object_at(list, 0) else { return };
      if field::<usize>(first, 0) == ITEM_CLASS.load(Ordering::Relaxed) {
        return;
      }
      let Some(item) = make_item(first, representative) else { return };
      let method = metadata(0x06286A70);
      if method.is_null() {
        return;
      }
      call!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method))(list, item, method);
      if list_count(list) != Some(count + 1) {
        return;
      }
      let array: Object = field(list, 0x10);
      for index in (0..count).rev() {
        reference(array, 0x28 + index * 8, field(array, 0x20 + index * 8));
      }
      reference(array, 0x20, item);
      let rows: i32 = field(menu, 0xB8);
      if (0..128).contains(&rows) {
        menu.cast::<u8>().add(0xB8).cast::<i32>().write(rows + 1);
      }
    }
  }

  #[skyline::hook(offset = ITEM_NAME)]
  fn cleaning_name(item: Object, method: Method) -> Object {
    if !item.is_null() && unsafe { field::<usize>(item, 0) } == ITEM_CLASS.load(Ordering::Relaxed) {
      return unsafe { name(item, method) };
    }
    call_original!(item, method)
  }

  #[skyline::hook(offset = CREATE_CLEANING)]
  fn cleaning_create(owner: Object, method: Method) {
    let Some(rings) = SESSION.with(|s| s.borrow_mut().take(owner as usize)) else {
      call_original!(owner, method);
      return;
    };
    unsafe {
      let unit = call!(GET_UNIT, unsafe extern "C" fn(Method) -> Object)(NONE);
      if unit.is_null() {
        call_original!(owner, method);
        return;
      }
      let previous = call!(GET_GOD, unsafe extern "C" fn(Method) -> Object)(NONE);
      let previous_type = call!(GET_TYPE, unsafe extern "C" fn(Method) -> i32)(NONE);
      let set_god = call!(SET_GOD, unsafe extern "C" fn(Object, Method));
      let set_type = call!(SET_TYPE, unsafe extern "C" fn(i32, Method));
      let state = call!(GET_STATE, unsafe extern "C" fn(Object, Method) -> i32);
      let set_dirty = call!(SET_DIRTY, unsafe extern "C" fn(Object, u8, Method));
      let reward = call!(ADD_BOND_EXP, unsafe extern "C" fn(Object, bool, i32, Method));
      for ring in rings {
        let god = ring.god as Object;
        if dirty(god) == 0 || field::<u8>(god, 0x4B) != 0 || !matches!(state(god, NONE), 0 | 3) {
          continue;
        }
        set_god(god, NONE);
        set_type(ring.kind, NONE);
        set_dirty(god, 0, NONE);
        if dirty(god) == 0 {
          // Native finish=true, Weak=1: normal finish reward and bond guards.
          // This helper uses static selection, not the 3D cleaning controller.
          reward(ptr::null_mut(), true, 1, NONE);
        }
      }
      set_god(previous, NONE);
      set_type(previous_type, NONE);
    }
    // The outer Start/EndRingCleaning UI and return-label flow still runs.
    // No 3D scene, rubbing controller, or per-ring input child is created.
  }

  #[skyline::hook(offset = END_SEQUENCE)]
  fn cleaning_end(owner: Object, method: Method) {
    SESSION.with(|s| s.borrow_mut().clear());
    call_original!(owner, method);
  }
}
