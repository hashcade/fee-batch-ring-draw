//! Shared price-column geometry in the menu's local UI units.

pub const ICON_GAP: f32 = 8.0;

pub fn column_width(widths: [f32; 2]) -> Option<f32> {
  if widths.iter().any(|width| !width.is_finite() || *width <= 0.0) {
    return None;
  }
  // Round outward so fractional glyph advances cannot clip the final digit.
  Some(widths[0].max(widths[1]).ceil() + 2.0)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Point {
  pub fn is_close(self, other: Self) -> bool {
    (self.x - other.x).abs() < 0.01
      && (self.y - other.y).abs() < 0.01
      && (self.z - other.z).abs() < 0.01
  }

  pub fn translated(self, from: Self, to: Self) -> Self {
    Self { x: self.x + to.x - from.x, y: self.y + to.y - from.y, z: self.z + to.z - from.z }
  }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Rect {
  pub fn right(self) -> Point {
    Point { x: self.x + self.width, ..Point::default() }
  }

  pub fn icon_edge(self, current: Point) -> Point {
    Point { x: self.x - ICON_GAP, ..current }
  }
}
