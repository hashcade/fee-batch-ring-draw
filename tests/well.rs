use fee_better_somniel::well::{available, label};

#[test]
fn only_a_pending_exchange_with_a_valid_level_can_be_previewed_or_rerolled() {
  for flag in -1..=3 {
    for level in 0..=6 {
      assert_eq!(available(flag, level), matches!(flag, 1 | 2) && (1..=5).contains(&level));
    }
  }
}

#[test]
fn preview_and_reroll_have_distinct_localized_menu_entries() {
  for language in 0..12 {
    assert!(!label(language, false).is_empty());
    assert_ne!(label(language, false), label(language, true));
  }
}
