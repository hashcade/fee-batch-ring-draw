use fee_better_somniel::config::{parse, DEFAULT_DRAW_COUNT, MAX_CONFIG_BYTES};

#[test]
fn missing_setting_and_comments_keep_300_default() {
  assert_eq!(DEFAULT_DRAW_COUNT, 300);
  assert_eq!(parse(""), Ok(300));
  assert_eq!(parse("# optional config\r\n\n"), Ok(300));
}

#[test]
fn accepts_whitespace_and_safe_boundary_values() {
  for count in [2, 10, 300, 600, 1000] {
    assert_eq!(parse(&format!("# count\r\n draw_count = {count}\r\n")), Ok(count));
  }
}

#[test]
fn rejects_ambiguous_unknown_or_out_of_range_values() {
  for value in [
    "draw_count=0",
    "draw_count=1",
    "draw_count=1001",
    "draw_count=-2",
    "draw_count=4294967296",
    "draw_count=1.5",
    "draw_count=",
    "draw_count=many",
    "other=600",
    "draw_count",
    "draw_count=600\ndraw_count=500",
  ] {
    assert!(parse(value).is_err(), "{value}");
  }
  assert!(parse(&" ".repeat(MAX_CONFIG_BYTES + 1)).is_err());
}

#[test]
fn settings_labels_follow_all_supported_game_languages() {
  for language in 0..=11 {
    assert!(!fee_better_somniel::settings::title(language).is_empty());
    assert!(!fee_better_somniel::settings::help(language, false).is_empty());
    assert!(!fee_better_somniel::settings::help(language, true).is_empty());
    assert_ne!(
      fee_better_somniel::settings::help(language, false),
      fee_better_somniel::settings::help(language, true)
    );
  }
}
