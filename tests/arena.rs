use fee_better_somniel::arena::{
  layout, quick_training, selection, Step, DESCRIPTOR_COUNT, KEPT_STEPS, STEP_FUNCTIONS, STEP_TYPES,
};

fn original() -> [Step; DESCRIPTOR_COUNT] {
  core::array::from_fn(|i| Step { kind: STEP_TYPES[i], function: STEP_FUNCTIONS[i] })
}

#[test]
fn only_paid_normal_emblem_training_is_eligible() {
  assert!(selection(true, false, 1, 6000));
  assert!(!selection(false, false, 0, 6000));
  assert!(!selection(true, false, 0, 6000));
  assert!(!selection(true, true, 1, 6000));
  for amount in [0, -1, i32::MIN] {
    assert!(!selection(true, false, 1, amount));
  }
}

#[test]
fn ordinary_training_skips_presentation_without_becoming_paid_training() {
  assert!(quick_training(false, false, 0, 0));
  assert!(!selection(false, false, 0, 0));
  assert!(quick_training(true, false, 1, 6000));
  for (emblem, special, kind, exp) in [
    (false, true, 0, 0),
    (false, false, 0, 1),
    (false, false, 1, 0),
    (true, false, 0, 0),
    (true, true, 1, 6000),
  ] {
    assert!(!quick_training(emblem, special, kind, exp));
  }
}

#[test]
fn all_native_growth_dialogue_and_fade_steps_are_preserved_in_order() {
  let order = layout(&original()).unwrap();
  assert_eq!(&order[..8], &KEPT_STEPS);
  assert!(order[8..].iter().all(|i| *i == 13));
  for index in [7, 8, 9] {
    assert_eq!(order.iter().filter(|i| **i == index).count(), 1);
  }
  for index in [0, 1, 4, 5, 6, 12] {
    assert!(!order.contains(&index));
  }
}

#[test]
fn changed_or_already_modified_layout_is_not_touched() {
  for index in 0..DESCRIPTOR_COUNT {
    let mut steps = original();
    steps[index].kind = -1;
    assert!(layout(&steps).is_none());
    if STEP_FUNCTIONS[index] != 0 {
      steps = original();
      steps[index].function += 4;
      assert!(layout(&steps).is_none());
    }
  }
  assert!(layout(&original()[..13]).is_none());
  let source = original();
  let changed = layout(&source).unwrap().map(|i| source[i]);
  assert!(layout(&changed).is_none());
}
