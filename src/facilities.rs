//! Somniel-only shortcuts into native facility sequences, with native cleanup.

pub const MENU_CTOR: usize = 0x024533F0;
pub const BIND_PARENT: usize = 0x02454310;
pub const BIND: usize = 0x0280A6B0;
pub const CONTENT: usize = 0x028B78C0;
pub const SHOW_FOOTER: usize = 0x021EDE30;
pub const SCENE_NAME: usize = 0x02A71860;
pub const RING_MENU: usize = 0x023397D0;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x024533F0, [0xD10203FF, 0xA9027BFD, 0x910083FD, 0xA9036FFC]),
  (0x02454310, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x9001F634]),
  (0x0280A6B0, [0x52800024, 0x14004F87, 0xD503201F, 0xD503201F]),
  (0x028B78C0, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x023374F0, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x0293AC80, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02A744F0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x028AB140, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x023DCCB0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02174D70, [0xA9BA7BFD, 0xF9000BFB, 0x910003FD, 0xA90267FA]),
  (0x028A8700, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x01F19B30, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x01C9BB30, [0xD101C3FF, 0xA9027BFD, 0x910083FD, 0xA90367FA]),
  (0x0205B740, [0xA9BC7BFD, 0xA9015FF8, 0x910003FD, 0xA90257F6]),
  (0x021EDE30, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD0020954]),
  (0x02A71860, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xF90013F5]),
  (0x023397D0, [0xA9BA7BFD, 0xA9016FFC, 0x910003FD, 0xA90267FA]),
];

pub struct Facility {
  pub name: &'static str,
  pub aid: &'static str,
  pub bind: usize,
}

pub const FACILITIES: &[Facility] = &[
  Facility { name: "MID_MENU_NOTICE_BOARD_TITLE", aid: "", bind: 0x01F19B30 },
  Facility { name: "MID_Hub_GodRoom", aid: "AID_紋章士の間", bind: 0x023374F0 },
  Facility { name: "MID_Hub_Well", aid: "AID_不思議な井戸", bind: 0x0293AC80 },
  Facility { name: "MID_Hub_Animal", aid: "AID_動物小屋", bind: 0x01C9BB30 },
  Facility { name: "MID_Hub_WeaponShop", aid: "AID_武器屋", bind: 0x02A744F0 },
  Facility { name: "MID_Hub_ItemShop", aid: "AID_道具屋", bind: 0x028AB140 },
  Facility { name: "MID_Hub_Blacksmith", aid: "AID_錬成屋", bind: 0x023DCCB0 },
  Facility { name: "MID_Hub_AccessoryShop", aid: "AID_アクセサリ", bind: 0x02174D70 },
  Facility { name: "MID_Hub_FreeMarket", aid: "AID_蚤の市", bind: 0x028A8700 },
  Facility {
    name: "MID_Hub_CafeTerrace_Jukebox", aid: "AID_ジュークボックス", bind: 0x0205B740
  },
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    meld::native::{new_text, text},
    menu_item::{self, VTABLE},
    native::{base, field, list_count, metadata, object_at, user_data, Method, Object},
    profile,
  };
  use core::{
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
  };
  const NONE: Method = ptr::null();
  static ENTRY_CLASS: AtomicUsize = AtomicUsize::new(0);
  static MENU_CLASS: AtomicUsize = AtomicUsize::new(0);
  static CLEANING_CLASS: AtomicUsize = AtomicUsize::new(0);
  static ITEM_CLASSES: [AtomicUsize; 10] = [const { AtomicUsize::new(0) }; 10];
  static SEQUENCE_CLASSES: [AtomicUsize; 10] = [const { AtomicUsize::new(0) }; 10];

  macro_rules! native {
    ($address:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $address)
    };
  }

  pub(crate) fn install() {
    skyline::install_hooks!(ring_menu);
  }

  unsafe extern "C" fn hidden(_: Object, _: Method) -> i32 {
    4
  }

  #[skyline::hook(offset = RING_MENU)]
  fn ring_menu(parent: Object, selected: i32, handler: Object, method: Method) {
    call_original!(parent, selected, handler, method);
    if parent.is_null() {
      return;
    }
    unsafe {
      let quick: Object = field(parent, 0x30);
      if quick.is_null() || field::<usize>(quick, 0) != MENU_CLASS.load(Ordering::Relaxed) {
        return;
      }
      let menu: Object = field(parent, 0x38);
      if menu.is_null() {
        return;
      }
      let list: Object = field(menu, 0x80);
      let Some(count) = list_count(list).filter(|n| *n <= 32) else { return };
      for index in 0..count {
        let Some(item) = object_at(list, index).filter(|item| !item.is_null()) else { continue };
        let class: Object = field(item, 0);
        if menu_item::named(field(class, 0x10), b"RingCleaningItem") {
          // Remote rubbing requires the actual ring-room scene. The original
          // facility still offers individual and all-ring cleaning unchanged.
          menu_item::override_class(item, &CLEANING_CLASS, &[(8, hidden as *const () as usize)]);
        }
      }
    }
  }

  unsafe fn somniel() -> bool {
    let user = unsafe { user_data() };
    !user.is_null()
      && unsafe {
        field::<i32>(user, 0x28) == 4
          && text(native!(SCENE_NAME, unsafe extern "C" fn(Method) -> Object)(NONE)).as_deref()
            == Some("Hub_Solanel")
      }
  }

  unsafe fn message(key: &str) -> Object {
    unsafe {
      native!(crate::meld::profile::MESS_GET, unsafe extern "C" fn(Object, Method) -> Object)(
        new_text(key),
        NONE,
      )
    }
  }

  unsafe fn index(item: Object) -> Option<usize> {
    let class = unsafe { field::<usize>(item, 0) };
    ITEM_CLASSES.iter().position(|cache| cache.load(Ordering::Relaxed) == class)
  }

  unsafe extern "C" fn name(item: Object, _: Method) -> Object {
    unsafe {
      message(index(item).map(|index| FACILITIES[index].name).unwrap_or("MID_MENU_TITLE_HUB"))
    }
  }

  unsafe extern "C" fn attribute(item: Object, _: Method) -> i32 {
    if !unsafe { somniel() } {
      return 4;
    }
    let Some(index) = (unsafe { index(item) }) else { return 1 };
    let facility = &FACILITIES[index];
    if facility.aid.is_empty()
      || unsafe {
        native!(
          crate::collector::profile::IS_COMPLETE,
          unsafe extern "C" fn(Object, Method) -> bool
        )(new_text(facility.aid), NONE)
      }
    {
      1
    } else {
      4
    }
  }

  unsafe fn virtual_void(object: Object, slot: usize) {
    unsafe {
      let class: Object = field(object, 0);
      let call: unsafe extern "C" fn(Object, Method) =
        core::mem::transmute(field::<usize>(class, VTABLE + slot * 16));
      call(object, field(class, VTABLE + slot * 16 + 8));
    }
  }

  unsafe extern "C" fn disposed(sequence: Object, method: Method) {
    unsafe {
      let parent: Object = field(sequence, 0x30);
      let original: unsafe extern "C" fn(Object, Method) =
        core::mem::transmute(field::<usize>(method as Object, 0));
      original(sequence, method);
      restore(parent);
    }
  }

  pub(crate) unsafe fn restore(menu: Object) {
    if menu.is_null() {
      return;
    }
    unsafe {
      virtual_void(menu, 21);
      let class = crate::settings::native::lookup_class(b"App\0", b"TitleBar\0");
      if class.is_null() {
        return;
      }
      let fields: Object = field(class, 0xB8);
      if fields.is_null() {
        return;
      }
      let title: Object = field(fields, 0);
      if !title.is_null() {
        native!(SHOW_FOOTER, unsafe extern "C" fn(Object, Method))(title, NONE);
      }
    }
  }

  pub(crate) unsafe fn append(parent: Object) {
    if parent.is_null() || !unsafe { somniel() } {
      return;
    }
    unsafe {
      let menu: Object = field(parent, 0x38);
      if menu.is_null() {
        return;
      }
      let list: Object = field(menu, 0x80);
      let Some(count) = list_count(list).filter(|n| (1..128).contains(n)) else { return };
      if (0..count).any(|i| {
        object_at(list, i)
          .is_some_and(|item| field::<usize>(item, 0) == ENTRY_CLASS.load(Ordering::Relaxed))
      }) {
        return;
      }
      let Some(first) = object_at(list, 0) else { return };
      let Some(item) = menu_item::create(
        first,
        &ENTRY_CLASS,
        &[
          (4, name as *const () as usize),
          (8, attribute as *const () as usize),
          (18, open as *const () as usize),
          (28, name as *const () as usize),
        ],
      ) else {
        return;
      };
      let method = metadata(0x06286A70);
      if method.is_null() {
        return;
      }
      native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method))(list, item, method);
      if list_count(list) == Some(count + 1) {
        let rows: i32 = field(menu, 0xB8);
        if (0..128).contains(&rows) {
          menu.cast::<u8>().add(0xB8).cast::<i32>().write(rows + 1);
        }
      }
    }
  }

  unsafe fn new_list(template: Object) -> Option<Object> {
    unsafe {
      let class: Object = field(template, 0);
      let methods: *const Method = field(class, 0x98);
      let count: u16 = field(class, 0x11C);
      if methods.is_null() || count > 256 {
        return None;
      }
      let ctor = (0..usize::from(count)).map(|i| methods.add(i).read()).find(|method| {
        !method.is_null()
          && menu_item::named(field(*method as Object, 0x10), b".ctor")
          && field::<usize>(*method as Object, 0)
            == base() + crate::collector::profile::DEFAULT_LIST_CTOR
      })?;
      let list = native!(profile::ALLOCATE_OBJECT, unsafe extern "C" fn(Object) -> Object)(class);
      if list.is_null() {
        return None;
      }
      native!(crate::collector::profile::DEFAULT_LIST_CTOR, unsafe extern "C" fn(Object, Method))(
        list, ctor,
      );
      Some(list)
    }
  }

  unsafe fn bind_menu(parent: Object) -> bool {
    unsafe {
      let list: Object = field(parent, 0x80);
      let Some(first) = object_at(list, 0) else { return false };
      let Some(items) = new_list(list) else { return false };
      let add_method = metadata(0x06286A70);
      if add_method.is_null() {
        return false;
      }
      for cache in &ITEM_CLASSES {
        let Some(item) = menu_item::create(
          first,
          cache,
          &[
            (4, name as *const () as usize),
            (8, attribute as *const () as usize),
            (18, activate as *const () as usize),
            (28, name as *const () as usize),
          ],
        ) else {
          return false;
        };
        native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method))(
          items, item, add_method,
        );
      }
      let mut class: Object = field(parent, 0);
      for _ in 0..16 {
        if class.is_null() {
          return false;
        }
        if menu_item::named(field(class, 0x10), b"BasicMenu") {
          break;
        }
        class = field(class, 0x58);
      }
      if class.is_null() || !menu_item::named(field(class, 0x10), b"BasicMenu") {
        return false;
      }
      let content = native!(CONTENT, unsafe extern "C" fn(Method) -> Object)(NONE);
      if content.is_null() {
        return false;
      }
      let menu = native!(profile::ALLOCATE_OBJECT, unsafe extern "C" fn(Object) -> Object)(class);
      if menu.is_null() {
        return false;
      }
      native!(MENU_CTOR, unsafe extern "C" fn(Object, Object, Object, Method))(
        menu, items, content, NONE,
      );
      if !menu_item::override_class(menu, &MENU_CLASS, &[(10, disposed as *const () as usize)]) {
        return false;
      }
      let class: Object = field(menu, 0);
      let create_desc: unsafe extern "C" fn(Object, Method) -> Object =
        core::mem::transmute(field::<usize>(class, VTABLE + 18 * 16));
      let desc = create_desc(menu, field(class, VTABLE + 18 * 16 + 8));
      if desc.is_null() {
        return false;
      }
      native!(BIND, unsafe extern "C" fn(Object, Object, Object, Object, Method))(
        menu,
        parent,
        desc,
        new_text("SomnielFacilities"),
        NONE,
      );
      native!(BIND_PARENT, unsafe extern "C" fn(Object, Method))(menu, NONE);
      native!(profile::SET_MENU_ROWS, unsafe extern "C" fn(Object, i32, Method))(menu, 10, NONE);
      true
    }
  }

  unsafe extern "C" fn open(item: Object, _: Method) -> i32 {
    unsafe {
      if attribute(item, NONE) != 1 {
        return 2048;
      }
      let menu: Object = field(item, 0x10);
      if menu.is_null() || !field::<Object>(menu, 0x38).is_null() {
        return 2048;
      }
      if !bind_menu(menu) {
        return 2048;
      }
      virtual_void(menu, 23);
      128
    }
  }

  unsafe extern "C" fn activate(item: Object, _: Method) -> i32 {
    unsafe {
      let Some(index) = index(item) else { return 2048 };
      if attribute(item, NONE) != 1 {
        return 2048;
      }
      let menu: Object = field(item, 0x10);
      if menu.is_null() || !field::<Object>(menu, 0x38).is_null() {
        return 2048;
      }
      let entry = FACILITIES[index].bind;
      if entry == 0x028A8700 {
        native!(entry, unsafe extern "C" fn(Object, bool, Method))(menu, false, NONE);
      } else if entry == 0x02174D70 {
        // Enter the native accessory room first; the shop child needs its
        // preview/resources. Do not bind HubAccessoryShopSequence directly.
        native!(entry, unsafe extern "C" fn(Object, i32, Method))(menu, 0, NONE);
      } else if entry == 0x01C9BB30 {
        let _ = native!(entry, unsafe extern "C" fn(Object, Method) -> Object)(menu, NONE);
      } else {
        native!(entry, unsafe extern "C" fn(Object, Method))(menu, NONE);
      }
      let child: Object = field(menu, 0x38);
      if child.is_null() || field::<Object>(child, 0x30) != menu {
        return 2048;
      }
      if !menu_item::override_class(
        child,
        &SEQUENCE_CLASSES[index],
        &[(10, disposed as *const () as usize)],
      ) {
        return 2048;
      }
      virtual_void(menu, 23);
      128
    }
  }
}
