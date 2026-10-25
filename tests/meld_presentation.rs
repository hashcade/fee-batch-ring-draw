use fee_better_somniel::meld::session::Presentation;

#[test]
fn only_the_matching_confirmed_batch_skips_once() {
  let mut presentation = Presentation::default();
  assert!(!presentation.take(1));
  presentation.arm(1);
  assert!(!presentation.take(2));
  assert!(presentation.take(1));
  assert!(!presentation.take(1));
}

#[test]
fn cancel_unload_and_selection_changes_clear_the_pending_skip() {
  let mut presentation = Presentation::default();
  presentation.arm(1);
  presentation.clear();
  assert!(!presentation.take(1));
  presentation.arm(0);
  assert!(!presentation.take(0));
}

#[test]
fn a_new_batch_cannot_reuse_an_older_sequence() {
  let mut presentation = Presentation::default();
  presentation.arm(1);
  presentation.arm(2);
  assert!(!presentation.take(1));
  assert!(presentation.take(2));
}
