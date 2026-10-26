use fee_better_somniel::wyvern::{border_offset, sss_score};

#[test]
fn each_difficulty_uses_its_own_native_array() {
  assert_eq!(border_offset(1), Some(0x20));
  assert_eq!(border_offset(2), Some(0x28));
  assert_eq!(border_offset(3), Some(0x30));
  for level in [i32::MIN, -1, 0, 4, i32::MAX] {
    assert_eq!(border_offset(level), None);
  }
}

#[test]
fn score_comes_from_the_current_config_not_a_saved_record_or_fixed_constant() {
  for threshold in [1, 30_000, 85_000, 180_000, i32::MAX] {
    assert_eq!(sss_score(&[threshold, 0]), Some(threshold));
  }
  assert_eq!(sss_score(&[90, 80, 70, 60, 50, 40, 30, 20, 0]), Some(90));
}

#[test]
fn malformed_config_falls_back_to_vanilla() {
  for borders in [
    vec![],
    vec![100],
    vec![0, 0],
    vec![-1, -2],
    vec![2, -1],
    vec![10, 20],
    vec![10, 10],
    vec![10, 5, 6],
    (0..10).rev().collect(),
  ] {
    assert_eq!(sss_score(&borders), None, "{borders:?}");
  }
}
