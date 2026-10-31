use fee_better_somniel::activities::CodeMemory;
use fee_better_somniel::{
  config::{parse_settings, Settings},
  preferences::*,
  settings,
};
use std::collections::BTreeMap;

#[test]
fn defaults_legacy_files_and_every_preference_round_trip() {
  let defaults = Settings::default();
  assert_eq!(defaults.rewind_count, 0);
  assert_eq!(defaults.arena_count, 10);
  assert!(defaults.battle_save && defaults.personal_outfits);
  assert_eq!(parse_settings("draw_count=527\nchef=PID_a\n").unwrap().arena_count, 10);
  for count in [0, 1, 10, 1000] {
    let settings = Settings {
      rewind_count: count,
      arena_count: count,
      battle_save: false,
      personal_outfits: false,
      ..defaults.clone()
    };
    assert_eq!(parse_settings(&settings.encode()), Ok(settings));
  }
  for bad in [
    "arena_count=1001",
    "rewind_count=-1",
    "battle_save=2",
    "personal_outfits=yes",
    "rewind_count=1\nrewind_count=0",
  ] {
    assert!(parse_settings(bad).is_err());
  }
}

#[test]
fn official_chinese_menu_names_and_order() {
  let mut kinds = settings::MENU_ORDER;
  kinds.sort();
  assert_eq!(kinds, std::array::from_fn(|i| i));
  assert_eq!(
    settings::MENU_ORDER.map(|kind| settings::option_title(10, kind)),
    [
      "经验倍率",
      "SP倍率",
      "SP对象",
      "SP数量",
      "金钱",
      "牵绊碎片",
      "铁晶石",
      "钢晶石",
      "银晶石",
      "威力纹晶石",
      "必杀纹晶石",
      "技巧纹晶石",
      "速度纹晶石",
      "防守纹晶石",
      "魔防纹晶石",
      "命中纹晶石",
      "回避纹晶石",
      "必避纹晶石",
      "战斗存档",
      "专属服装",
      "自动完成支援对话",
      "自动研磨",
      "研磨角色",
      "龙之时水晶次数",
      "一般锻炼次数",
      "钓鱼次数不减",
      "牵绊戒指精炼次数",
      "商店库存无限",
      "赠送礼物不消耗",
      "快速钓鱼",
      "肌肉体操快速满分",
      "飞龙射手快速SSS",
      "重复料理",
      "料理厨师",
      "料理结果",
    ]
  );
  assert_eq!(settings::count_label(10, 0), "不减");
  assert_eq!(settings::count_label(10, 10), "10");
}

#[test]
fn use_limits_preserve_spent_counts_and_native_resets() {
  assert_eq!(rewind_limit(0), -1);
  assert_eq!(rewind_limit(20), 20);
  assert_eq!(arena_limit(10, 3, 3) - 3, 7);
  assert_eq!(arena_limit(10, 10, 3) - 10, 0);
  assert_eq!(arena_limit(10, 0, 3), 10);
  for spent in [0, 3, 27] {
    assert_eq!(arena_limit(0, spent, 3) - spent, 3);
  }
  assert_eq!(patches(&Settings { arena_count: 0, ..Settings::default() })[4].1, 0x2A1703F4);
}

struct Memory {
  words: BTreeMap<usize, u32>,
  fail: Option<usize>,
  writes: usize,
}
impl CodeMemory for Memory {
  fn read(&self, address: usize) -> u32 {
    self.words[&address]
  }
  fn write(&mut self, address: usize, value: u32) -> bool {
    self.writes += 1;
    if self.fail == Some(self.writes) {
      return false;
    }
    self.words.insert(address, value);
    true
  }
}

#[test]
fn patches_switch_both_ways_and_rollback_the_actual_previous_words() {
  let defaults = Settings::default();
  let finite =
    Settings { rewind_count: 25, battle_save: false, arena_count: 0, ..defaults.clone() };
  let words = |settings: &Settings| {
    patches(settings)
      .into_iter()
      .chain(fee_better_somniel::activities::desired(settings))
      .collect::<BTreeMap<_, _>>()
  };
  let mut memory = Memory { words: words(&defaults), writes: 0, fail: None };
  assert!(apply(&mut memory, &finite));
  assert_eq!(memory.words, words(&finite));
  assert!(apply(&mut memory, &defaults));
  let before = memory.words.clone();
  memory.writes = 0;
  memory.fail = Some(3);
  assert!(!apply(&mut memory, &finite));
  assert_eq!(memory.words, before);
}

#[test]
fn disabling_convenience_features_restores_only_their_native_instructions() {
  use fee_better_somniel::{activities, config::Feature};
  let defaults = Settings::default();
  for feature in Feature::ALL {
    let mut settings = defaults.clone();
    feature.set(&mut settings, false);
    let before = activities::desired(&defaults);
    let after = activities::desired(&settings);
    let changed = before.iter().zip(&after).filter(|(old, new)| old != new).count();
    let expected = match feature {
      Feature::RepeatCooking => 1,
      Feature::GiftStock => 3,
      Feature::FastFishing => 6,
      Feature::FastTraining => 12,
      _ => 0, // Event hooks, not instruction patches.
    };
    assert_eq!(changed, expected, "{feature:?}");
    for &(address, value) in &after {
      let (_, original, replacement) =
        activities::PATCHES.iter().find(|&&(offset, _, _)| offset == address).unwrap();
      assert_eq!(value, if before.contains(&(address, value)) { *replacement } else { *original });
    }
  }
}
