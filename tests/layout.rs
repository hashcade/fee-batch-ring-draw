use fee_batch_ring_draw::layout::{column_width, Point, Rect, ICON_GAP};

#[test]
fn shared_column_tracks_actual_glyph_width_not_digit_count() {
  assert_eq!(column_width([42.0, 96.25]), Some(99.0));
  assert_eq!(column_width([96.25, 42.0]), Some(99.0));
  assert_eq!(column_width([42.0, 121.5]), Some(124.0));
  assert_eq!(column_width([42.0, 48.1]), Some(51.0));
}

#[test]
fn invalid_or_not_yet_built_text_does_not_move_the_layout() {
  for width in [0.0, -1.0, f32::NAN, f32::INFINITY] {
    assert_eq!(column_width([42.0, width]), None);
  }
}

#[test]
fn both_icon_edges_share_the_column_and_preserve_vertical_positions() {
  let rect = Rect { x: -99.0, y: -10.0, width: 99.0, height: 20.0 };
  for y in [0.0, 60.0] {
    let edge = rect.icon_edge(Point { x: -30.0, y, z: 2.0 });
    assert_eq!(edge, Point { x: -99.0 - ICON_GAP, y, z: 2.0 });
  }
}

#[test]
fn absolute_edge_correction_does_not_accumulate_on_refresh() {
  let target = Point { x: 200.0, y: 0.0, z: 0.0 };
  let mut position = Point { x: 140.0, y: 30.0, z: 1.0 };
  let mut edge = Point { x: 240.0, y: 0.0, z: 0.0 };
  for _ in 0..100 {
    position = position.translated(edge, target);
    edge = target;
  }
  assert_eq!(position, Point { x: 100.0, y: 30.0, z: 1.0 });
}
