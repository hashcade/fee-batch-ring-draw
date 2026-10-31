use fee_better_somniel::cleaning::option_title;

#[test]
fn polishing_preferences_have_concise_localized_names() {
  for language in 0..12 {
    assert!(!option_title(language, false).is_empty());
    assert!(!option_title(language, true).is_empty());
  }
  assert_eq!(option_title(10, false), "自动研磨");
  assert_eq!(option_title(10, true), "研磨角色");
}
