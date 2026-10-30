use fee_better_somniel::{
  config::{parse_settings, Settings},
  gains::*,
};

#[test]
fn independent_defaults_and_every_multiplier_pair_round_trip() {
  let defaults = Settings::default();
  assert_eq!((defaults.exp_multiplier, defaults.sp_multiplier), (1, 1));
  assert_eq!(parse_settings("draw_count=527").unwrap().exp_multiplier, 1);
  for exp_multiplier in EXP_MULTIPLIERS {
    for sp_multiplier in MULTIPLIERS {
      let settings = Settings { exp_multiplier, sp_multiplier, ..defaults.clone() };
      assert_eq!(parse_settings(&settings.encode()), Ok(settings));
    }
  }
  for key in ["exp_multiplier", "sp_multiplier"] {
    for value in ["4", "11", "-1", "1.5", "4294967296"] {
      assert!(parse_settings(&format!("{key}={value}")).is_err());
    }
    assert!(parse_settings(&format!("{key}=2\n{key}=3")).is_err());
  }
  assert!(parse_settings("sp_multiplier=0").is_err());
}

#[test]
fn gains_are_independent_and_zero_never_becomes_a_reward() {
  assert_eq!((scaled(20, 2, 1000), scaled(10, 3, 9999)), (40, 30));
  for multiplier in MULTIPLIERS {
    assert_eq!(scaled(0, multiplier, 9999), 0);
    assert_eq!(scaled(-10, multiplier, 9999), -10);
    assert_eq!(scaled(100, multiplier, 9999), 100 * multiplier as i32);
  }
  assert_eq!(scaled(20, 1, 0), 20, "1x preserves the original arguments");
  assert_eq!(scaled(i32::MAX, 10, 9999), 9999);
}

#[test]
fn multiple_levels_and_native_caps_do_not_wrap() {
  assert_eq!(exp_capacity(1, 20, 75), 1825);
  assert_eq!(scaled(100, 10, exp_capacity(1, 20, 75)), 1000);
  assert_eq!(scaled(100, 10, exp_capacity(19, 20, 75)), 25);
  assert_eq!(scaled(100, 10, exp_capacity(20, 20, 0)), 0);
  assert_eq!(exp_capacity(40, 20, 99), 0);
  assert_eq!(scaled(100, 10, 9999 - 9990), 9);
}

#[test]
fn max_class_level_requires_a_real_exp_reward_and_preserves_sp() {
  assert_eq!(fee_better_somniel::settings::multiplier_label(10, 0), "满级");
  assert_eq!(experience(1, 0, exp_capacity(1, 20, 75)), 1825);
  assert_eq!(experience(1, 0, exp_capacity(1, 40, 0)), 3900);
  assert_eq!(experience(0, 0, 3900), 0);
  assert_eq!(experience(-1, 0, 3900), -1);
  assert_eq!(experience(10, 0, exp_capacity(20, 20, 0)), 0);
  assert_eq!(scaled(10, 3, 9999), 30);
  for language in 0..=11 {
    assert!(!fee_better_somniel::settings::multiplier_label(language, 0).is_empty());
  }
}
