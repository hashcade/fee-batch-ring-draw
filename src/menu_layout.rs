//! Adjust only this menu's two price columns, after native text construction.

use crate::{
  config,
  layout::{self, Point, Rect},
  native::{base, field, list_count, object_at, Method, Object},
  profile,
};
use core::ptr;

const NONE: Method = ptr::null();

#[repr(C)]
struct Size {
  width: f32,
  height: f32,
}

#[derive(Clone, Copy)]
struct Row {
  text: Object,
  amount: Object,
  icon: Object,
}

impl Row {
  unsafe fn new(content: Object) -> Option<Self> {
    if content.is_null() {
      return None;
    }
    let text: Object = unsafe { field(content, 0x50) };
    let image: Object = unsafe { field(content, 0x58) };
    if text.is_null() || image.is_null() {
      return None;
    }
    let get_amount: unsafe extern "C" fn(Object, Method) -> Object =
      unsafe { core::mem::transmute(base() + profile::TMP_RECT) };
    let class: Object = unsafe { field(image, 0) };
    // Graphic.get_rectTransform is virtual slot 30, inherited by Image.
    let get_icon: unsafe extern "C" fn(Object, Method) -> Object =
      unsafe { field(class, 0x138 + 30 * 16) };
    let method: Method = unsafe { field(class, 0x140 + 30 * 16) };
    let amount = unsafe { get_amount(text, NONE) };
    let icon = unsafe { get_icon(image, method) };
    if amount.is_null() || icon.is_null() {
      return None;
    }
    Some(Self { text, amount, icon })
  }

  unsafe fn width(self) -> f32 {
    let measure: unsafe extern "C" fn(Object, Method) -> Size =
      unsafe { core::mem::transmute(base() + profile::PREFERRED_VALUES) };
    unsafe { measure(self.text, NONE).width }
  }

  unsafe fn apply(self, width: f32) {
    let get_rect: unsafe extern "C" fn(Object, Method) -> Rect =
      unsafe { core::mem::transmute(base() + profile::RECT) };
    let set_width: unsafe extern "C" fn(Object, i32, f32, Method) =
      unsafe { core::mem::transmute(base() + profile::SET_WIDTH) };
    let transform: unsafe extern "C" fn(Object, Point, Method) -> Point =
      unsafe { core::mem::transmute(base() + profile::TRANSFORM_POINT) };
    let inverse: unsafe extern "C" fn(Object, Point, Method) -> Point =
      unsafe { core::mem::transmute(base() + profile::INVERSE_POINT) };
    let position: unsafe extern "C" fn(Object, Method) -> Point =
      unsafe { core::mem::transmute(base() + profile::GET_POSITION) };
    let set_position: unsafe extern "C" fn(Object, Point, Method) =
      unsafe { core::mem::transmute(base() + profile::SET_POSITION) };
    unsafe {
      let old_right = transform(self.amount, get_rect(self.amount, NONE).right(), NONE);
      set_width(self.amount, 0, width, NONE);
      let rect = get_rect(self.amount, NONE);
      let new_right = transform(self.amount, rect.right(), NONE);
      set_position(self.amount, position(self.amount, NONE).translated(new_right, old_right), NONE);

      // Convert through the amount's coordinates: preserve vertical placement,
      // even when the image and text have different parents, pivots, or scales.
      let icon_right = transform(self.icon, get_rect(self.icon, NONE).right(), NONE);
      let edge = inverse(self.amount, icon_right, NONE);
      let target = transform(self.amount, rect.icon_edge(edge), NONE);
      set_position(self.icon, position(self.icon, NONE).translated(icon_right, target), NONE);
    }
  }
}

pub unsafe fn refresh(content: Object) {
  if content.is_null() {
    return;
  }
  let item: Object = unsafe { field(content, 0x18) };
  if item.is_null() {
    return;
  }
  let menu: Object = unsafe { field(item, 0x10) };
  if menu.is_null() {
    return;
  }
  let items: Object = unsafe { field(menu, 0x78) };
  if unsafe { list_count(items) } != Some(2) {
    return;
  }
  let mut rows = [None, None];
  for (index, row) in rows.iter_mut().enumerate() {
    let Some(entry) = (unsafe { object_at(items, index) }) else { return };
    let expected = if index == 0 { 1 } else { config::draw_count() as i32 };
    if unsafe { field::<i32>(entry, 0x64) } != expected {
      return;
    }
    let view = if entry == item { content } else { unsafe { field(entry, 0x18) } };
    *row = unsafe { Row::new(view) };
  }
  // Initial binding can construct the first row before the second exists.
  // The second BuildText call then lays out both freshly built prices together.
  let [Some(first), Some(second)] = rows else { return };
  let Some(width) = layout::column_width(unsafe { [first.width(), second.width()] }) else {
    return;
  };
  unsafe {
    first.apply(width);
    second.apply(width);
  }
}
