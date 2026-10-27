use fee_better_somniel::level::{cumulative, title};

#[test]
fn total_is_current_plus_internal_not_internal_alone() {
  assert_eq!(cumulative(20, 50), Some(70));
  assert_eq!(cumulative(1, 0), Some(1));
  assert_eq!(cumulative(99, 127), Some(226));
  for (level, internal) in [(0, 0), (100, 0), (255, 0), (20, -1), (20, -128)] {
    assert_eq!(cumulative(level, internal), None);
  }
}

#[test]
fn every_game_language_has_a_compact_explicit_heading() {
  for language in 0..12 {
    assert!(title(language).contains('/'));
    assert!(title(language).chars().count() <= 10);
  }
}
