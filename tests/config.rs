use fee_better_somniel::config::{parse, DEFAULT_DRAW_COUNT, MAX_CONFIG_BYTES};

#[test]
fn missing_setting_and_comments_keep_300_default() {
  assert_eq!(DEFAULT_DRAW_COUNT, 300);
  assert_eq!(parse(""), Ok(300));
  assert_eq!(parse("# optional config\r\n\n"), Ok(300));
}

#[test]
fn convenience_switches_default_on_and_round_trip_independently() {
  use fee_better_somniel::config::{parse_settings, Feature, Settings};
  let defaults = Settings::default();
  for mask in 0..1 << Feature::ALL.len() {
    let mut settings = defaults.clone();
    for (index, feature) in Feature::ALL.into_iter().enumerate() {
      assert!(feature.value(&defaults));
      feature.set(&mut settings, mask & (1 << index) != 0);
    }
    assert_eq!(parse_settings(&settings.encode()), Ok(settings));
  }
  for feature in Feature::ALL {
    let key = feature.key();
    assert!(feature.value(&parse_settings(&format!("{key}=1")).unwrap()));
    for invalid in [format!("{key}=2"), format!("{key}=true"), format!("{key}=0\n{key}=1")] {
      assert!(parse_settings(&invalid).is_err());
    }
  }
}

#[test]
fn extended_settings_round_trip_without_save_variables() {
  use fee_better_somniel::config::{parse_settings, Settings};
  assert_eq!(
    parse_settings("draw_count=527").unwrap(),
    Settings { draw_count: 527, ..Settings::default() }
  );
  let settings = Settings {
    draw_count: 527,
    chef: "PID_パネトネ".to_owned(),
    cooking_outcome: 3,
    auto_support: false,
    auto_polish: false,
    polish_actor: "PID_パネトネ".to_owned(),
    ..Settings::default()
  };
  assert_eq!(parse_settings(&settings.encode()), Ok(settings));
  for value in [
    "chef=IID_Food",
    "chef=PID_a=b",
    "chef=PID_a\tbad",
    "chef=PID_x\nchef=PID_y",
    "cooking_outcome=6",
    "auto_support=2",
    "auto_support=1\nauto_support=0",
    "auto_polish=2",
    "auto_polish=0\nauto_polish=1",
    "polish_actor=",
    "polish_actor=IID_Food",
    "polish_actor=PID_a=b",
  ] {
    assert!(parse_settings(value).is_err(), "{value}");
  }
}

#[test]
fn automatic_polishing_defaults_to_alear_and_keeps_legacy_files() {
  use fee_better_somniel::config::{parse_settings, DEFAULT_POLISHER};
  let settings = parse_settings("draw_count=300\n").unwrap();
  assert!(settings.auto_polish);
  assert_eq!(settings.polish_actor, DEFAULT_POLISHER);
  assert_eq!(settings.encode(), "draw_count=300\n");
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
    for kind in 0..fee_better_somniel::settings::OPTION_COUNT {
      assert!(!fee_better_somniel::settings::option_title(language, kind).is_empty());
    }
    for option in 0..=5 {
      let label = fee_better_somniel::settings::outcome_label(language, option);
      assert!(!label.contains(['(', '（']), "choice labels must not include implementation notes");
    }
  }
  assert_eq!(fee_better_somniel::settings::default_label(10), "默认");
  assert_eq!(fee_better_somniel::settings::outcome_label(10, 5), "失败");
}
