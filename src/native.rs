//! Version-pinned IL2CPP calls. No mutable inventory entries are cached.

use crate::{
  batch::{Inventory, Stock},
  config, labels, profile,
  random::{self, Rates, State},
};
use core::{ffi::c_void, ptr};

pub type Object = *mut c_void;
pub type Method = *const c_void;
pub type Draw = unsafe extern "C" fn(Object, i32, *mut Object, Method) -> Object;
const NONE: Method = ptr::null();

pub fn base() -> usize {
  unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize }
}

pub unsafe fn field<T: Copy>(object: Object, offset: usize) -> T {
  unsafe { object.cast::<u8>().add(offset).cast::<T>().read_unaligned() }
}

pub(crate) unsafe fn metadata(offset: usize) -> Object {
  let cell = unsafe { ((base() + offset) as *const Object).read() };
  if cell.is_null() {
    return ptr::null_mut();
  }
  let initialize: unsafe extern "C" fn(Object) =
    unsafe { core::mem::transmute(base() + profile::INITIALIZE_METADATA) };
  unsafe {
    initialize(cell);
    cell.cast::<Object>().read()
  }
}

pub(crate) unsafe fn reference(object: Object, offset: usize, value: Object) {
  let slot = unsafe { object.cast::<u8>().add(offset).cast::<Object>() };
  let barrier: unsafe extern "C" fn(*mut Object, Object) =
    unsafe { core::mem::transmute(base() + profile::WRITE_BARRIER) };
  unsafe {
    slot.write(value);
    barrier(slot, value);
  }
}

pub(crate) unsafe fn user_data() -> Object {
  let method = unsafe { metadata(0x062840F0) };
  if method.is_null() {
    return ptr::null_mut();
  }
  let get: unsafe extern "C" fn(Method) -> Object =
    unsafe { core::mem::transmute(base() + profile::SINGLETON_GET) };
  unsafe { get(method) }
}

unsafe fn parameter(slot: usize) -> Option<i32> {
  let key = unsafe { metadata(slot) };
  if key.is_null() {
    return None;
  }
  let get: unsafe extern "C" fn(Object, Method) -> i32 =
    unsafe { core::mem::transmute(base() + profile::PARAMETER) };
  Some(unsafe { get(key, NONE) })
}

pub unsafe fn cost() -> Option<i32> {
  unsafe { parameter(0x062C2B08) }
    .filter(|value| *value > 0 && *value <= i32::MAX / config::MAX_DRAW_COUNT as i32)
}

unsafe fn string(string: Object) -> Option<Vec<u16>> {
  if string.is_null() {
    return None;
  }
  let count: i32 = unsafe { field(string, 0x10) };
  if count < 0 {
    return None;
  }
  Some(
    unsafe {
      core::slice::from_raw_parts(string.cast::<u8>().add(0x14).cast::<u16>(), count as usize)
    }
    .to_vec(),
  )
}

pub unsafe fn set_menu_text(content: Object) {
  if content.is_null() {
    return;
  }
  let item: Object = unsafe { field(content, 0x18) };
  let count = config::draw_count();
  if item.is_null() || count == 10 || unsafe { field::<i32>(item, 0x64) } != count as i32 {
    return;
  }
  let language: unsafe extern "C" fn(Method) -> i32 =
    unsafe { core::mem::transmute(base() + profile::GET_LANGUAGE) };
  unsafe {
    set_text(field(content, 0x48), &labels::count(language(NONE), count));
  }
  // The original constructor owns the full price and affordability. Only the
  // menu's display text is abbreviated; confirmation and charging stay native.
  let price: i32 = unsafe { field(item, 0x68) };
  if let Some(price) = labels::price(price) {
    unsafe {
      set_text(field(content, 0x50), &price);
    }
  }
}

unsafe fn extend_draw_content(menu: Object, rows: i32) -> bool {
  unsafe {
    let content: Object = field(menu, 0x70);
    if content.is_null() {
      return false;
    }
    let object: Object = field(content, 0x68);
    if object.is_null() {
      return false;
    }
    let transform: unsafe extern "C" fn(Object, Method) -> Object =
      core::mem::transmute(base() + profile::GAME_OBJECT_TRANSFORM);
    let child_count: unsafe extern "C" fn(Object, Method) -> i32 =
      core::mem::transmute(base() + profile::TRANSFORM_CHILD_COUNT);
    let child: unsafe extern "C" fn(Object, i32, Method) -> Object =
      core::mem::transmute(base() + profile::TRANSFORM_CHILD);
    let clone: unsafe extern "C" fn(Object, Object, bool, Method) -> Object =
      core::mem::transmute(base() + profile::CLONE_WITH_PARENT);
    let position: unsafe extern "C" fn(Object, *mut [f32; 3], Method) =
      core::mem::transmute(base() + profile::LOCAL_POSITION_GET);
    let set_position: unsafe extern "C" fn(Object, *const [f32; 3], Method) =
      core::mem::transmute(base() + profile::LOCAL_POSITION_SET);
    let parent = transform(object, NONE);
    if parent.is_null() {
      return false;
    }
    // GetShowRowMax counts these real children. SetShowRowNum alone cannot
    // expand the original two-row prefab. Clone its complete row/component
    // hierarchy before the native menu builds, retaining all serialized refs.
    let existing = child_count(parent, NONE);
    if existing >= rows {
      return true;
    }
    if existing < 2 {
      return false;
    }
    let previous = child(parent, existing - 2, NONE);
    let mut last = child(parent, existing - 1, NONE);
    if previous.is_null() || last.is_null() {
      return false;
    }
    let (mut before, mut next) = ([0.0; 3], [0.0; 3]);
    position(previous, &mut before, NONE);
    position(last, &mut next, NONE);
    let spacing: [f32; 3] = core::array::from_fn(|i| next[i] - before[i]);
    for _ in existing..rows {
      let added = clone(last, parent, false, NONE);
      if added.is_null() {
        return false;
      }
      for i in 0..3 {
        next[i] += spacing[i];
      }
      set_position(added, &next, NONE);
      last = added;
    }
    true
  }
}

pub unsafe fn append_draw_count(menu: Object, handler: Object) {
  let count = config::draw_count();
  if menu.is_null() || count == 10 {
    return;
  }
  let list: Object = unsafe { field(menu, 0x80) };
  let Some(length) = (unsafe { list_count(list) }) else {
    return;
  };
  // Preserve all native/other-mod rows and only avoid our own duplicate.
  for index in 0..length {
    let Some(item) = (unsafe { object_at(list, index) }) else { return };
    if unsafe { field::<i32>(item, 0x64) == count as i32 && field::<Object>(item, 0x70) == handler }
    {
      return;
    }
  }
  let class = unsafe { metadata(0x062C2B88) };
  let add_method = unsafe { metadata(0x06286A70) };
  if class.is_null() || add_method.is_null() {
    return;
  }
  if !unsafe { extend_draw_content(menu, (length + 1) as i32) } {
    return;
  }
  let allocate: unsafe extern "C" fn(Object) -> Object =
    unsafe { core::mem::transmute(base() + profile::ALLOCATE_OBJECT) };
  let ctor: unsafe extern "C" fn(Object, i32, Object, Method) =
    unsafe { core::mem::transmute(base() + profile::COUNT_CTOR) };
  let add: unsafe extern "C" fn(Object, Object, Method) =
    unsafe { core::mem::transmute(base() + profile::LIST_ADD) };
  let rows: unsafe extern "C" fn(Object, i32, Method) =
    unsafe { core::mem::transmute(base() + profile::SET_MENU_ROWS) };
  let item = unsafe { allocate(class) };
  if !item.is_null() {
    unsafe {
      ctor(item, count as i32, handler, NONE);
      add(list, item, add_method);
      if list_count(list) == Some(length + 1) {
        rows(menu, (length + 1) as i32, NONE);
      }
    }
  }
}

unsafe fn set_text(text: Object, value: &str) {
  if text.is_null() {
    return;
  }
  let units: Vec<u16> = value.encode_utf16().collect();
  let allocate: unsafe extern "C" fn(i32, Method) -> Object =
    unsafe { core::mem::transmute(base() + profile::STRING_NEW_SIZE) };
  let value = unsafe { allocate(units.len() as i32, NONE) };
  if value.is_null() {
    return;
  }
  unsafe {
    ptr::copy_nonoverlapping(
      units.as_ptr(),
      value.cast::<u8>().add(0x14).cast::<u16>(),
      units.len(),
    );
  }
  let class: Object = unsafe { field(text, 0) };
  if class.is_null() {
    return;
  }
  let set: unsafe extern "C" fn(Object, Object, Method) = unsafe { field(class, 0x558) };
  let method: Method = unsafe { field(class, 0x560) };
  unsafe {
    set(text, value, method);
  }
}

unsafe fn list(sequence: Object, boolean: bool) -> Object {
  let (class_slot, ctor_slot, ctor_offset, field_offset) = if boolean {
    (0x062B5148, 0x062C2AE8, profile::BOOL_LIST_CTOR, 0xA0)
  } else {
    (0x062AB780, 0x062AB788, profile::LIST_CTOR, 0x98)
  };
  let class = unsafe { metadata(class_slot) };
  let method = unsafe { metadata(ctor_slot) };
  if class.is_null() || method.is_null() {
    return ptr::null_mut();
  }
  let allocate: unsafe extern "C" fn(Object) -> Object =
    unsafe { core::mem::transmute(base() + profile::ALLOCATE_OBJECT) };
  let ctor: unsafe extern "C" fn(Object, i32, Method) =
    unsafe { core::mem::transmute(base() + ctor_offset) };
  let value = unsafe { allocate(class) };
  if !value.is_null() {
    unsafe {
      ctor(value, config::draw_count() as i32, method);
      reference(sequence, field_offset, value);
    }
  }
  value
}

pub(crate) unsafe fn list_count(list: Object) -> Option<usize> {
  if list.is_null() {
    return None;
  }
  let count: i32 = unsafe { field(list, 0x18) };
  let array: Object = unsafe { field(list, 0x10) };
  if count < 0 || array.is_null() {
    return None;
  }
  let capacity: usize = unsafe { field(array, 0x18) };
  if count as usize > capacity {
    return None;
  }
  Some(count as usize)
}

pub(crate) unsafe fn object_at(list: Object, index: usize) -> Option<Object> {
  if index >= unsafe { list_count(list) }? {
    return None;
  }
  let array: Object = unsafe { field(list, 0x10) };
  let object: Object = unsafe { field(array, 0x20 + index * 8) };
  (!object.is_null()).then_some(object)
}

#[derive(Clone, Copy)]
pub struct Expected {
  data: Object,
  state: State,
}

pub struct GameInventory {
  god: Object,
  original: Draw,
  rings: Object,
  flags: Object,
  candidates: [Vec<Object>; 4],
  price: i32,
}

impl GameInventory {
  pub unsafe fn new(sequence: Object, god: Object, original: Draw) -> Option<Self> {
    let rings = unsafe { list(sequence, false) };
    let flags = unsafe { list(sequence, true) };
    let mut value = Self { god, original, rings, flags, candidates: Default::default(), price: 0 };
    if rings.is_null() || flags.is_null() || god.is_null() {
      return None;
    }
    value.price = unsafe { cost() }?;
    let method = unsafe { metadata(0x06287110) };
    if method.is_null() {
      return None;
    }
    let get: unsafe extern "C" fn(Method) -> Object =
      unsafe { core::mem::transmute(base() + profile::RING_LIST) };
    let entries = unsafe { get(method) };
    let gid = unsafe { string(field(god, 0x20)) }?;
    for index in 0..unsafe { list_count(entries) }? {
      let data = unsafe { object_at(entries, index) }?;
      if unsafe { string(field(data, 0x38)) }.as_ref() != Some(&gid) {
        continue;
      }
      let rank: i32 = unsafe { field(data, 0x4C) };
      if !(0..=3).contains(&rank) || unsafe { field::<i32>(data, 0x48) } != 0 {
        return None;
      }
      value.candidates[rank as usize].push(data);
    }
    // Every native draw chooses from ten characters at the selected rank.
    if value.candidates.iter().any(|rank| rank.len() != 10) {
      return None;
    }
    Some(value)
  }

  pub fn price(&self) -> i32 {
    self.price
  }
  pub fn results(&self) -> Object {
    self.rings
  }

  unsafe fn state() -> Option<State> {
    let get: unsafe extern "C" fn(Method) -> Object =
      unsafe { core::mem::transmute(base() + profile::GAME_RANDOM) };
    let rng = unsafe { get(NONE) };
    if rng.is_null() {
      return None;
    }
    Some(State(unsafe { field(rng, 0x10) }))
  }
}

impl Inventory for GameInventory {
  type Expected = Expected;

  fn fragments(&mut self) -> Option<i32> {
    let user = unsafe { user_data() };
    if user.is_null() {
      return None;
    }
    Some(unsafe { field(user, 0x6C) })
  }

  fn preview(&mut self) -> Option<Expected> {
    if unsafe { cost() }? != self.price {
      return None;
    }
    let rates = Rates {
      s: u32::try_from(unsafe { parameter(0x062C2AF0) }?).ok()?,
      a: u32::try_from(unsafe { parameter(0x062C2B00) }?).ok()?,
      b: u32::try_from(unsafe { parameter(0x062C2AF8) }?).ok()?,
    };
    let (rank, index, state) =
      random::preview(unsafe { Self::state() }?, rates, self.candidates.each_ref().map(Vec::len))?;
    Some(Expected { data: self.candidates[rank][index], state })
  }

  fn stock(&mut self, expected: &Expected) -> Option<Stock> {
    let total: unsafe extern "C" fn(Object, Method) -> i32 =
      unsafe { core::mem::transmute(base() + profile::ALL_COUNT) };
    let equipped: unsafe extern "C" fn(Object, bool, Method) -> i32 =
      unsafe { core::mem::transmute(base() + profile::STOCK_COUNT) };
    let can_add: unsafe extern "C" fn(Object, i32, Method) -> bool =
      unsafe { core::mem::transmute(base() + profile::CAN_ADD) };
    let rnid: Object = unsafe { field(expected.data, 0x20) };
    if rnid.is_null() {
      return None;
    }
    Some(Stock {
      total: unsafe { total(expected.data, NONE) },
      equipped: unsafe { equipped(expected.data, true, NONE) },
      fragments: self.fragments()?,
      can_add: unsafe { can_add(rnid, 1, NONE) },
    })
  }

  fn transact(&mut self, expected: &Expected) -> bool {
    let mut flags = ptr::null_mut();
    let results = unsafe { (self.original)(self.god, 1, &mut flags, NONE) };
    if unsafe { list_count(results) } != Some(1) || unsafe { list_count(flags) } != Some(1) {
      return false;
    }
    let Some(ring) = (unsafe { object_at(results, 0) }) else {
      return false;
    };
    // Preserve actual acquired results for the original outer achievement pass.
    let array: Object = unsafe { field(flags, 0x10) };
    let new: bool = unsafe { field(array, 0x20) };
    let add: unsafe extern "C" fn(Object, Object, Method) =
      unsafe { core::mem::transmute(base() + profile::LIST_ADD) };
    let add_bool: unsafe extern "C" fn(Object, bool, Method) =
      unsafe { core::mem::transmute(base() + profile::BOOL_LIST_ADD) };
    let ring_method = unsafe { metadata(0x062AB790) };
    let flag_method = unsafe { metadata(0x062B5110) };
    if ring_method.is_null() || flag_method.is_null() {
      return false;
    }
    unsafe {
      add(self.rings, ring, ring_method);
      add_bool(self.flags, new, flag_method);
    }
    unsafe { field::<Object>(ring, 0x20) == expected.data && Self::state() == Some(expected.state) }
  }
}

pub unsafe fn empty_results(sequence: Object) -> Object {
  let rings: Object = unsafe { field(sequence, 0x98) };
  if rings.is_null() {
    unsafe { list(sequence, false) }
  } else {
    rings
  }
}
