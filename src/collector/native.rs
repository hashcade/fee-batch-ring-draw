//! Original hub menu entry; no polling, generated drops, or custom save variables.

use super::profile as offsets;
use crate::{
  meld::native::{new_text, text},
  menu_item::{named, VTABLE},
  native::{base, field, list_count, metadata, object_at, reference, user_data, Method, Object},
  profile,
};
use core::{
  ptr,
  sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::collections::BTreeSet;

const NONE: Method = ptr::null();
static ITEM_CLASS: AtomicUsize = AtomicUsize::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);

macro_rules! native {
  ($offset:expr, $signature:ty) => {
    core::mem::transmute::<usize, $signature>(base() + $offset)
  };
}

pub(crate) fn install() {
  skyline::install_hooks!(create_menu);
}

unsafe fn message(key: &str) -> Object {
  let get = unsafe {
    native!(crate::meld::profile::MESS_GET, unsafe extern "C" fn(Object, Method) -> Object)
  };
  unsafe { get(new_text(key), NONE) }
}

unsafe extern "C" fn name(_: Object, _: Method) -> Object {
  unsafe { message("MID_Hub_Area_ItemReturn") }
}

unsafe extern "C" fn help(_: Object, _: Method) -> Object {
  unsafe { message("MID_Hub_Well_ItemExchange_Get") }
}

unsafe fn make_item(first: Object) -> Option<Object> {
  unsafe {
    crate::menu_item::create(
      first,
      &ITEM_CLASS,
      &[
        (4, name as *const () as usize),
        (8, attribute as *const () as usize),
        (18, collect as *const () as usize),
        (28, help as *const () as usize),
      ],
    )
  }
}

#[skyline::hook(offset = offsets::CREATE_MENU)]
fn create_menu(parent: Object, method: Method) {
  call_original!(parent, method);
  unsafe { append_collection(parent) };
  unsafe { crate::facilities::native::append(parent) };
  unsafe { crate::supports::native::on_hub_menu(parent) };
}

unsafe fn append_collection(parent: Object) {
  if parent.is_null() {
    return;
  }
  unsafe {
    let menu: Object = field(parent, 0x38);
    if menu.is_null() {
      return;
    }
    let list: Object = field(menu, 0x80);
    let Some(count) = list_count(list) else {
      return;
    };
    let Some(first) = object_at(list, 0) else {
      return;
    };
    if field::<usize>(first, 0) == ITEM_CLASS.load(Ordering::Relaxed) {
      return;
    }
    let Some(item) = make_item(first) else {
      return;
    };
    let add_method = metadata(0x06286A70);
    if add_method.is_null() {
      return;
    }
    let add = native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method));
    add(list, item, add_method);
    if list_count(list) != Some(count + 1) {
      return;
    }
    let array: Object = field(list, 0x10);
    for index in (0..count).rev() {
      reference(array, 0x20 + (index + 1) * 8, field(array, 0x20 + index * 8));
    }
    reference(array, 0x20, item);
    let rows: i32 = field(menu, 0xB8);
    menu.cast::<u8>().add(0xB8).cast::<i32>().write(rows + 1);
  }
}

struct Context {
  hub: Object,
  variables: Object,
  sequence: i32,
  accesses: Object,
}

unsafe fn context() -> Option<Context> {
  let user = unsafe { user_data() };
  if user.is_null() {
    return None;
  }
  let sequence: i32 = unsafe { field(user, 0x28) };
  if !matches!(sequence, 4 | 5) {
    return None;
  }
  let method = unsafe { metadata(0x0628A970) };
  if method.is_null() {
    return None;
  }
  let get = unsafe { native!(offsets::HUB_GET, unsafe extern "C" fn(Method) -> Object) };
  let hub = unsafe { get(method) };
  if hub.is_null() {
    return None;
  }
  let manager = unsafe {
    native!(offsets::CURRENT_ACCESS, unsafe extern "C" fn(Object, Method) -> Object)(hub, NONE)
  };
  let variables: Object = unsafe { field(user, 0x50) };
  if manager.is_null() || variables.is_null() {
    return None;
  }
  let accesses = unsafe { field(manager, 0x18) };
  unsafe { list_count(accesses) }?;
  Some(Context { hub, variables, sequence, accesses })
}

unsafe fn claimed(variables: Object, flag: Object) -> bool {
  let get = unsafe {
    native!(offsets::VARIABLE_BOOL, unsafe extern "C" fn(Object, Object, Method) -> bool)
  };
  unsafe { get(variables, flag, NONE) }
}

unsafe fn ordinary_gift(data: Object) -> bool {
  // Villager rewards only. Story, support, Emblem and party conversations stay native.
  unsafe {
    field::<u8>(data, 0x25) != 0
      && [0x20, 0x21, 0x22, 0x23, 0x24].iter().all(|&offset| field::<u8>(data, offset) == 0)
  }
}

#[derive(Clone, Copy)]
enum Reward {
  Item(Object, u32),
  Animal(Object),
}
enum Claim {
  Pickup,
  Gift { flag: Object, next_count: i32 },
}
struct Source {
  data: Object,
  reward: Reward,
  claim: Claim,
  identity: String,
}

unsafe fn resolve_item(key: Object) -> Option<Object> {
  if key.is_null() {
    return None;
  }
  let method = unsafe { metadata(0x06287888) };
  if method.is_null() {
    return None;
  }
  let get = unsafe { native!(offsets::ITEM_GET, unsafe extern "C" fn(Object, Method) -> Object) };
  let item = unsafe { get(key, method) };
  (!item.is_null()).then_some(item)
}

unsafe fn source(context: &Context, data: Object) -> Option<Source> {
  let dispos: Object = unsafe { field(data, 0x18) };
  if dispos.is_null() {
    return None;
  }
  let kind: i32 = unsafe { field(dispos, 0x68) };
  let gift = (2..=5).contains(&kind) && unsafe { ordinary_gift(data) };
  // 0xB5 is DisabledTalk, not a disabled pickup flag. It only gates NPC gifts.
  if gift && unsafe { field::<u8>(dispos, 0xB5) } != 0 {
    return None;
  }
  let (identity, claim) = if gift {
    // Match ExecEventAfter's result-index/access-count gate. TalkItem alone
    // may still contain a reward belonging to an already-consumed conversation.
    let count = unsafe {
      native!(offsets::ACCESS_COUNT, unsafe extern "C" fn(Object, Method) -> i32)(data, NONE)
    };
    if count < 0 || count == i32::MAX || count != unsafe { field::<i32>(data, 0x28) } {
      return None;
    }
    let flag = unsafe {
      native!(offsets::ACCESS_FLAG, unsafe extern "C" fn(Object, Method) -> Object)(data, NONE)
    };
    let key = unsafe { text(flag) }.filter(|s| !s.is_empty())?;
    (format!("talk:{key}:{count}"), Claim::Gift { flag, next_count: count + 1 })
  } else {
    let flag: Object = unsafe { field(data, 0x40) };
    let key = unsafe { text(flag) }.filter(|s| !s.is_empty())?;
    if unsafe { claimed(context.variables, flag) } {
      return None;
    }
    (format!("drop:{key}"), Claim::Pickup)
  };
  let reward = if matches!(kind, 6 | 8 | 9) {
    let item = unsafe { resolve_item(field(data, 0x10)) }?;
    let count: i32 = unsafe { field(data, 0x38) };
    // Type 8 already contains the native post-battle material quantity.
    let count = if kind == 8 {
      count
    } else {
      unsafe {
        native!(offsets::ITEM_BONUS, unsafe extern "C" fn(Object, i32, Method) -> i32)(
          item, count, NONE,
        )
      }
    };
    Reward::Item(item, u32::try_from(count).ok().filter(|n| *n > 0)?)
  } else if context.sequence == 5 && kind == 7 && unsafe { field::<u8>(data, 0x24) } != 0 {
    let barn = unsafe { metadata(0x062A6028) };
    if barn.is_null()
      || !unsafe {
        native!(offsets::IS_COMPLETE, unsafe extern "C" fn(Object, Method) -> bool)(barn, NONE)
      }
    {
      return None;
    }
    let pid = unsafe {
      native!(offsets::TRY_PID, unsafe extern "C" fn(Object, Method) -> Object)(data, NONE)
    };
    if pid.is_null() {
      return None;
    }
    let animal = unsafe {
      native!(offsets::ANIMAL_GET, unsafe extern "C" fn(Object, Method) -> Object)(pid, NONE)
    };
    if animal.is_null()
      || !unsafe {
        native!(offsets::CAN_CAPTURE, unsafe extern "C" fn(Object, Method) -> bool)(animal, NONE)
      }
    {
      return None;
    }
    Reward::Animal(animal)
  } else if gift {
    Reward::Item(unsafe { resolve_item(field(data, 0x30)) }?, 1)
  } else {
    return None;
  };
  Some(Source { data, reward, claim, identity })
}

unsafe fn sources(context: &Context) -> Vec<Source> {
  let mut seen = BTreeSet::new();
  let mut selected = Vec::new();
  for index in 0..unsafe { list_count(context.accesses) }.unwrap_or(0) {
    let Some(data) = (unsafe { object_at(context.accesses, index) }) else {
      continue;
    };
    let Some(source) = (unsafe { source(context, data) }) else {
      continue;
    };
    if seen.insert(source.identity.clone()) {
      selected.push(source);
    }
  }
  selected
}

unsafe extern "C" fn attribute(_: Object, _: Method) -> i32 {
  if ACTIVE.load(Ordering::Relaxed) {
    return 2;
  }
  match unsafe { context() } {
    Some(context) if !unsafe { sources(&context) }.is_empty() => 1,
    Some(_) => 2,
    _ => 4,
  }
}

unsafe fn item_list(sources: &[Source]) -> Option<Object> {
  let class = unsafe { metadata(0x062923C0) };
  let method = unsafe { metadata(0x062923B8) };
  if class.is_null() || method.is_null() {
    return None;
  }
  let allocate =
    unsafe { native!(profile::ALLOCATE_OBJECT, unsafe extern "C" fn(Object) -> Object) };
  let list = unsafe { allocate(class) };
  if list.is_null() {
    return None;
  }
  unsafe {
    native!(offsets::DEFAULT_LIST_CTOR, unsafe extern "C" fn(Object, Method))(list, method)
  };
  let count: u16 = unsafe { field(class, 0x11C) };
  let methods: *const Method = unsafe { field(class, 0x98) };
  if methods.is_null() {
    return None;
  }
  let mut add_method = NONE;
  for index in 0..usize::from(count) {
    let candidate = unsafe { methods.add(index).read() };
    if !candidate.is_null() && unsafe { named(field(candidate as Object, 0x10), b"Add") } {
      add_method = candidate;
      break;
    }
  }
  if add_method.is_null() {
    return None;
  }
  let add = unsafe { native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method)) };
  for source in sources {
    if let Reward::Item(item, count) = source.reward {
      for _ in 0..count {
        unsafe { add(list, item, add_method) };
      }
    }
  }
  Some(list)
}

unsafe fn settle(context: &Context, sources: &[Source]) {
  let mut collected = BTreeSet::new();
  for source in sources {
    if let Reward::Animal(animal) = source.reward {
      unsafe {
        native!(offsets::INC_CAPTURE, unsafe extern "C" fn(Object, i32, Method))(animal, 1, NONE)
      };
    }
    let (flag, value) = match source.claim {
      Claim::Pickup => (unsafe { field(source.data, 0x40) }, 1),
      Claim::Gift { flag, next_count } => (flag, next_count),
    };
    unsafe {
      // Entry creates a missing native flag; SetNumber also handles existing
      // false/zero flags. Entry alone does not update an existing variable.
      native!(offsets::VARIABLE_ENTRY, unsafe extern "C" fn(Object, Object, i32, Method) -> bool)(
        context.variables,
        flag,
        value,
        NONE,
      );
      native!(offsets::VARIABLE_SET, unsafe extern "C" fn(Object, Object, i32, Method))(
        context.variables,
        flag,
        value,
        NONE,
      );
      if matches!(source.claim, Claim::Gift { .. }) {
        reference(source.data, 0x30, ptr::null_mut());
      }
    }
    collected.insert(source.data as usize);
  }
  // Only update loaded visuals belonging to settled sources. Do not disable
  // other NPCs, conversations, adoption markers, or unrelated drops.
  let group: Object = unsafe { field(context.hub, 0xC8) };
  if group.is_null() {
    return;
  }
  let accesses: Object = unsafe { field(group, 0x20) };
  for index in 0..unsafe { list_count(accesses) }.unwrap_or(0) {
    let Some(access) = (unsafe { object_at(accesses, index) }) else {
      continue;
    };
    let data: Object = unsafe { field(access, 0x60) };
    if !collected.contains(&(data as usize)) || unsafe { ordinary_gift(data) } {
      continue;
    }
    unsafe { native!(offsets::DONE_ACCESS, unsafe extern "C" fn(Object, Method))(access, NONE) };
    let effect: Object = unsafe { field(access, 0x68) };
    if !effect.is_null() {
      unsafe {
        native!(offsets::SET_ACTIVE, unsafe extern "C" fn(Object, bool, Method))(
          effect, false, NONE,
        )
      };
    }
  }
}

unsafe fn execute(item: Object) -> bool {
  if item.is_null() {
    return false;
  }
  let menu: Object = unsafe { field(item, 0x10) };
  if menu.is_null() || !unsafe { field::<Object>(menu, 0x38) }.is_null() {
    return false;
  }
  let class: Object = unsafe { field(menu, 0) };
  if class.is_null() || unsafe { field::<u16>(class, 0x126) } < 24 {
    return false;
  }
  let close_address: usize = unsafe { field(class, VTABLE + 23 * 16) };
  let close: unsafe extern "C" fn(Object, Method) = unsafe { core::mem::transmute(close_address) };
  let close_method: Method = unsafe { field(class, VTABLE + 23 * 16 + 8) };
  let Some(context) = (unsafe { context() }) else {
    return false;
  };
  let sources = unsafe { sources(&context) };
  if sources.is_empty() {
    return false;
  }
  let Some(list) = (unsafe { item_list(&sources) }) else {
    return false;
  };
  // CreateBindForWell stores TitleMID and translates it later, not display text.
  let title =
    new_text(if context.sequence == 4 { "MID_MENU_TITLE_HUB" } else { "MID_MENU_TITLE_KIZUNA" });
  if title.is_null() {
    return false;
  }
  let Some(copies) = (unsafe { list_count(list) }) else {
    return false;
  };
  unsafe { settle(&context, &sources) };
  unsafe {
    let minimap =
      native!(offsets::MINIMAP, unsafe extern "C" fn(Object, Method) -> Object)(context.hub, NONE);
    if !minimap.is_null() {
      if native!(offsets::MINIMAP_IS_SHOW, unsafe extern "C" fn(Object, Method) -> bool)(
        minimap, NONE,
      ) {
        native!(offsets::MINIMAP_MODE, unsafe extern "C" fn(Object, i32, Method))(minimap, 0, NONE);
      }
      native!(offsets::HIDE_SYSTEM_MENU, unsafe extern "C" fn(Object, Method))(minimap, NONE);
    }
    close(menu, close_method);
    if copies > 0 {
      // Use the complete native sequence: load/wait for prefabs, grant once,
      // display results, handle overflow and unload. A bare reward dialog
      // has no content prefab on a fresh exploration visit.
      native!(offsets::REWARD_SEQUENCE, unsafe extern "C" fn(Object, Object, Object, Method))(
        menu, list, title, NONE,
      );
    }
  }
  true
}

unsafe extern "C" fn collect(item: Object, _: Method) -> i32 {
  if ACTIVE.swap(true, Ordering::Relaxed) {
    return 2048;
  }
  let success = unsafe { execute(item) };
  ACTIVE.store(false, Ordering::Relaxed);
  if success {
    129
  } else {
    2048
  }
}
