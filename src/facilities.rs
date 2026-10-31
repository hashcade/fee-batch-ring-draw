//! Somniel-only shortcuts into native facility sequences, with native cleanup.

pub const MENU_CTOR: usize = 0x024533F0;
pub const BIND_PARENT: usize = 0x02454310;
pub const BIND: usize = 0x0280A6B0;
pub const CONTENT: usize = 0x024EB420;
pub const PAGE_CONTENT: usize = 0x028B78C0;
pub const SET_SUBMENU_TRANSFORM: usize = 0x0245E330;
pub const SHOW_FOOTER: usize = 0x021EDE30;
pub const SCENE_NAME: usize = 0x02A71860;
pub const VARIABLE_NUMBER: usize = 0x0251E9D0;
pub const FACILITY_GET: usize = 0x00203E90;
pub const FACILITY_COMPLETE: usize = 0x028A80D0;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x02ADECF0, [0xAA1F03E1, 0x17D0933F, 0xD503201F, 0xD503201F]),
  (0x00203E90, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xA90257F6]),
  (0x028A80D0, [0xD10143FF, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x024533F0, [0xD10203FF, 0xA9027BFD, 0x910083FD, 0xA9036FFC]),
  (0x02454310, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0x9001F634]),
  (0x0280A6B0, [0x52800024, 0x14004F87, 0xD503201F, 0xD503201F]),
  (0x024EB420, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x0245E330, [0xF9403800, 0x14000003, 0xD503201F, 0xD503201F]),
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
  (0x01CA5330, [0xD10283FF, 0xA9047BFD, 0x910103FD, 0xA9056FFC]),
  (0x02D84750, [0xD10283FF, 0xA9047BFD, 0x910103FD, 0xA9056FFC]),
  (0x028B5E40, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x02DD8250, [0xD10283FF, 0xA9047BFD, 0x910103FD, 0xA9056FFC]),
  (0x0260A4A0, [0xD10203FF, 0xA9027BFD, 0x910083FD, 0xA9036FFC]),
  (0x02AD6C40, [0xD102C3FF, 0xA9057BFD, 0x910143FD, 0xA9066FFC]),
  (0x022BC190, [0xA9BC7BFD, 0xA9015FF8, 0x910003FD, 0xA90257F6]),
  (0x0260B190, [0xA9BA7BFD, 0xF9000BFB, 0x910003FD, 0xA90267FA]),
  (0x0251E9D0, [0xD10103FF, 0xA9017BFD, 0x910043FD, 0xA90257F6]),
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
  Facility { name: "MID_Hub_Arena", aid: "AID_闘技場", bind: 0x01CA5330 },
  Facility { name: "MID_Hub_Cafe", aid: "AID_料理", bind: 0x02D84750 },
  Facility { name: "MID_Hub_Mascot_Spot", aid: "M004", bind: 0x028B5E40 },
  Facility { name: "MID_Hub_TrainingSpot", aid: "AID_筋肉体操", bind: 0x02DD8250 },
  Facility { name: "MID_Hub_Talk_Fishing", aid: "AID_釣り", bind: 0x0260A4A0 },
  Facility { name: "MID_Hub_Talk_DragonRide", aid: "AID_ドラゴンライド", bind: 0x02AD6C40 },
  Facility { name: "MID_Hub_Poolside", aid: "AID_水遊び", bind: 0x022BC190 },
];

pub struct MenuGroup {
  pub facilities: &'static [usize],
}

// One outdoor entry opens these categories; all destination lists fit the
// original prefab without custom scrolling or additional outdoor entries.
pub const GROUPS: &[MenuGroup] = &[
  MenuGroup { facilities: &[0, 1, 2, 3, 9, 10] },
  MenuGroup { facilities: &[11, 12, 13, 14, 15, 16] },
  MenuGroup { facilities: &[4, 5, 6, 7, 8] },
];

pub fn group_title(language: i32, group: usize) -> &'static str {
  let labels = match language {
    0 => ["施設", "アクティビティ", "ショップ"],
    2 | 5 => ["Installations", "Activités", "Boutiques"],
    3 | 6 => ["Instalaciones", "Actividades", "Tiendas"],
    7 => ["Einrichtungen", "Aktivitäten", "Läden"],
    8 => ["Strutture", "Attività", "Negozi"],
    9 => ["設施", "活動", "商店"],
    10 => ["设施", "活动", "商店"],
    11 => ["시설", "활동", "상점"],
    _ => ["Facilities", "Activities", "Shops"],
  };
  labels[group]
}

#[cfg(test)]
mod tests {
  use super::{group_title, FACILITIES, GROUPS};

  #[test]
  fn short_categories_keep_every_destination_once() {
    assert_eq!(GROUPS.iter().map(|group| group.facilities.len()).collect::<Vec<_>>(), [6, 6, 5]);
    assert!(GROUPS[0].facilities.contains(&10), "the arena belongs to facilities");
    assert!(!GROUPS[1].facilities.contains(&10), "do not duplicate the arena in activities");
    let mut actual: Vec<_> =
      GROUPS.iter().flat_map(|group| group.facilities.iter().copied()).collect();
    actual.sort_unstable();
    assert_eq!(actual, (0..FACILITIES.len()).collect::<Vec<_>>());
  }

  #[test]
  fn category_labels_cover_game_languages() {
    for language in 0..12 {
      for group in 0..GROUPS.len() {
        assert!(!group_title(language, group).is_empty());
      }
    }
    assert_eq!(group_title(10, 0), "设施");
    assert_eq!(group_title(10, 1), "活动");
    assert_eq!(group_title(10, 2), "商店");
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    meld::native::{new_text, text, text_language},
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
  static GROUP_CLASSES: [AtomicUsize; GROUPS.len()] = [const { AtomicUsize::new(0) }; GROUPS.len()];
  static MENU_CLASS: AtomicUsize = AtomicUsize::new(0);
  static PAGE_CLASS: AtomicUsize = AtomicUsize::new(0);
  static ITEM_CLASSES: [AtomicUsize; FACILITIES.len()] =
    [const { AtomicUsize::new(0) }; FACILITIES.len()];
  static SEQUENCE_CLASSES: [AtomicUsize; FACILITIES.len()] =
    [const { AtomicUsize::new(0) }; FACILITIES.len()];

  macro_rules! native {
    ($address:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $address)
    };
  }

  unsafe extern "C" fn cancel(_: Object, _: Method) -> i32 {
    // BasicMenu.Result: CloseThis | SeCancel, never DeleteAll (32).
    1 | 512
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
      if let Some(index) = index(item) {
        message(FACILITIES[index].name)
      } else if let Some(group) = group(item) {
        new_text(group_title(text_language(), group))
      } else {
        message("MID_MENU_TITLE_HUB")
      }
    }
  }

  unsafe fn group(item: Object) -> Option<usize> {
    let class = unsafe { field::<usize>(item, 0) };
    GROUP_CLASSES.iter().position(|cache| cache.load(Ordering::Relaxed) == class)
  }

  unsafe extern "C" fn attribute(item: Object, _: Method) -> i32 {
    if !unsafe { somniel() } {
      return 4;
    }
    let Some(index) = (unsafe { index(item) }) else { return 1 };
    let facility = &FACILITIES[index];
    if unsafe { unlocked(facility.aid) } {
      unsafe { available(facility.bind) }
    } else {
      4
    }
  }

  pub(crate) unsafe fn unlocked(aid: &str) -> bool {
    if aid.is_empty() {
      return true;
    }
    unsafe {
      if aid == "M004" {
        return native!(
          crate::collector::profile::IS_COMPLETE,
          unsafe extern "C" fn(Object, Method) -> bool
        )(new_text(aid), NONE);
      }
      let method = metadata(0x06290F98);
      if method.is_null() {
        return false;
      }
      let data = native!(FACILITY_GET, unsafe extern "C" fn(Object, Method) -> Object)(
        new_text(aid),
        method,
      );
      !data.is_null()
        && native!(FACILITY_COMPLETE, unsafe extern "C" fn(Object, Method) -> bool)(data, NONE)
    }
  }

  unsafe fn available(entry: usize) -> i32 {
    unsafe {
      if entry == 0x0260A4A0 {
        return if crate::fishing::native::remaining() > 0 { 1 } else { 2 };
      }
      let key = match entry {
        0x02DD8250 => "G_MuscleExercise_GetPrize",
        0x02AD6C40 => "G_DragonRide_GetPrize",
        _ => return 1,
      };
      let variable: Object = field(user_data(), 0x50);
      if variable.is_null() {
        return 2;
      }
      if native!(VARIABLE_NUMBER, unsafe extern "C" fn(Object, Object, Method) -> i32)(
        variable,
        new_text(key),
        NONE,
      ) == 0
      {
        1
      } else {
        2
      }
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

  unsafe fn animate_visible_menus(menu: Object, slot: usize) {
    unsafe {
      virtual_void(menu, slot);
      // Only the category panel and the outdoor menu are visible together.
      // A destination page replaces both, including on facility returns.
      if field::<usize>(menu, 0) == MENU_CLASS.load(Ordering::Relaxed) {
        virtual_void(field(menu, 0x30), slot);
      }
    }
  }

  pub(crate) unsafe extern "C" fn disposed(sequence: Object, method: Method) {
    unsafe {
      let parent: Object = field(sequence, 0x30);
      let original: unsafe extern "C" fn(Object, Method) =
        core::mem::transmute(field::<usize>(method as Object, 0));
      original(sequence, method);
      restore(parent);
    }
  }

  pub(crate) unsafe fn hide(menu: Object) {
    unsafe { animate_visible_menus(menu, 22) };
  }

  pub(crate) unsafe fn restore(menu: Object) {
    if menu.is_null() {
      return;
    }
    unsafe {
      animate_visible_menus(menu, 20);
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
      let Some(first) = object_at(list, 0) else { return };
      let method = metadata(0x06286A70);
      if method.is_null() {
        return;
      }
      let Some(count) = list_count(list) else { return };
      if (0..count).any(|i| {
        object_at(list, i)
          .is_some_and(|item| field::<usize>(item, 0) == ENTRY_CLASS.load(Ordering::Relaxed))
      }) {
        return;
      }
      let Some(item) = menu_item::create_hub(
        first,
        &ENTRY_CLASS,
        &[
          (4, name as *const () as usize),
          (8, attribute as *const () as usize),
          (18, open as *const () as usize),
          (28, menu_item::no_help as *const () as usize),
        ],
      ) else {
        return;
      };
      native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method))(list, item, method);
      if list_count(list) == Some(count + 1) {
        let rows: i32 = field(menu, 0xB8);
        menu.cast::<u8>().add(0xB8).cast::<i32>().write(rows + 1);
      }
    }
  }

  unsafe fn new_list(template: Object) -> Option<Object> {
    unsafe {
      let class: Object = field(template, 0);
      let methods: *const Method = field(class, 0x98);
      let count: u16 = field(class, 0x11C);
      if methods.is_null() {
        return None;
      }
      let ctor = (0..usize::from(count)).map(|i| methods.add(i).read()).find(|method| {
        !method.is_null()
          && menu_item::named(field(*method as Object, 0x10), b".ctor")
          && field::<u8>(*method as Object, 0x4A) == 0
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

  unsafe fn add_choice(
    items: Object,
    first: Object,
    cache: &AtomicUsize,
    action: usize,
    method: Method,
  ) -> bool {
    unsafe {
      let Some(item) = menu_item::create_hub(
        first,
        cache,
        &[
          (4, name as *const () as usize),
          (8, attribute as *const () as usize),
          (18, action),
          (19, cancel as *const () as usize),
          (28, menu_item::no_help as *const () as usize),
        ],
      ) else {
        return false;
      };
      native!(profile::LIST_ADD, unsafe extern "C" fn(Object, Object, Method))(items, item, method);
      true
    }
  }

  unsafe fn bind_menu(parent: Object, item: Object, group: Option<usize>) -> bool {
    unsafe {
      let list: Object = field(parent, 0x80);
      let Some(first) = object_at(list, 0) else { return false };
      let Some(items) = new_list(list) else { return false };
      let add_method = metadata(0x06286A70);
      if add_method.is_null() {
        return false;
      }
      let rows = if let Some(group) = group {
        for &index in GROUPS[group].facilities {
          if !add_choice(
            items,
            first,
            &ITEM_CLASSES[index],
            activate as *const () as usize,
            add_method,
          ) {
            return false;
          }
        }
        GROUPS[group].facilities.len()
      } else {
        for cache in &GROUP_CLASSES {
          if !add_choice(items, first, cache, open_group as *const () as usize, add_method) {
            return false;
          }
        }
        GROUPS.len()
      };
      let mut class: Object = field(parent, 0);
      loop {
        if class.is_null() {
          return false;
        }
        if menu_item::named(field(class, 0x10), b"BasicMenu") {
          break;
        }
        class = field(class, 0x58);
      }
      let content = native!(
        if group.is_some() { PAGE_CONTENT } else { CONTENT },
        unsafe extern "C" fn(Method) -> Object
      )(NONE);
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
      let copied = if group.is_some() {
        menu_item::override_class(menu, &PAGE_CLASS, &[(10, disposed as *const () as usize)])
      } else {
        menu_item::override_class(menu, &MENU_CLASS, &[])
      };
      if !copied {
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
      native!(profile::SET_MENU_ROWS, unsafe extern "C" fn(Object, i32, Method))(
        menu,
        rows as i32,
        NONE,
      );
      if group.is_none() {
        native!(SET_SUBMENU_TRANSFORM, unsafe extern "C" fn(Object, Object, Object, Method))(
          menu, parent, item, NONE,
        );
      }
      true
    }
  }

  unsafe extern "C" fn open(item: Object, _: Method) -> i32 {
    unsafe { open_submenu(item, None) }
  }

  unsafe extern "C" fn open_group(item: Object, _: Method) -> i32 {
    let Some(group) = (unsafe { group(item) }) else { return 2048 };
    unsafe { open_submenu(item, Some(group)) }
  }

  unsafe fn open_submenu(item: Object, group: Option<usize>) -> i32 {
    unsafe {
      if attribute(item, NONE) != 1 {
        return 2048;
      }
      let menu: Object = field(item, 0x10);
      if menu.is_null() || !field::<Object>(menu, 0x38).is_null() {
        return 2048;
      }
      if !bind_menu(menu, item, group) {
        return 2048;
      }
      if group.is_some() {
        hide(menu);
      }
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
      if matches!(entry, 0x028A8700 | 0x02AD6C40) {
        native!(entry, unsafe extern "C" fn(Object, bool, Method))(menu, false, NONE);
      } else if entry == 0x022BC190 {
        native!(entry, unsafe extern "C" fn(Object, Object, Method))(
          menu,
          new_text(FACILITIES[index].aid),
          NONE,
        );
      } else if entry == 0x02174D70 {
        // Enter the native accessory room first; the shop child needs its
        // preview/resources. Do not bind HubAccessoryShopSequence directly.
        native!(entry, unsafe extern "C" fn(Object, i32, Method))(menu, 0, NONE);
      } else if entry == 0x01C9BB30 {
        let _ = native!(entry, unsafe extern "C" fn(Object, Method) -> Object)(menu, NONE);
      } else {
        // Keep each activity's complete native scene/resource setup and
        // settlement; do not construct fake fish, actors or reward flows.
        native!(entry, unsafe extern "C" fn(Object, Method))(menu, NONE);
      }
      let child: Object = field(menu, 0x38);
      if child.is_null() {
        return 2048;
      }
      if !menu_item::override_class(
        child,
        &SEQUENCE_CLASSES[index],
        &[(10, disposed as *const () as usize)],
      ) {
        return 2048;
      }
      hide(menu);
      128
    }
  }
}
