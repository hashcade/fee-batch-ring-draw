use fee_better_somniel::session::Session;

#[test]
fn dispatch_and_presentation_are_parent_scoped_and_once_only() {
  let mut session = Session::default();
  assert!(!session.begin(0));
  assert!(session.begin(123));
  assert!(!session.begin(456));
  assert_eq!(session.dispatch(1, 300), None);
  assert_eq!(session.dispatch(300, 300), Some(123));
  assert_eq!(session.dispatch(300, 300), None);
  assert!(!session.presentation(123));
  session.end();
  assert!(!session.presentation(456));
  assert!(session.presentation(123));
  assert!(!session.presentation(123));
}

#[test]
fn single_draw_and_unload_never_skip_an_unrelated_presentation() {
  let mut session = Session::default();
  assert!(session.begin(123));
  assert_eq!(session.dispatch(1, 300), None);
  session.end();
  assert!(!session.presentation(123));
  assert!(session.begin(123));
  assert_eq!(session.dispatch(300, 300), Some(123));
  session.end();
  session.clear();
  assert!(!session.presentation(123));
}

#[test]
fn configured_count_not_a_literal_300_is_dispatched() {
  for count in [2, 600, 1000] {
    let mut session = Session::default();
    assert!(session.begin(123));
    assert_eq!(session.dispatch(1, count), None);
    assert_eq!(session.dispatch(300, count), None);
    assert_eq!(session.dispatch(count as i32, count), Some(123));
    session.end();
    assert!(session.presentation(123));
  }
}

#[test]
fn original_ten_never_arms_batch_presentation_even_when_configured() {
  let mut session = Session::default();
  assert!(session.begin(123));
  assert_eq!(session.dispatch(10, 10), None);
  session.end();
  assert!(!session.presentation(123));
}
