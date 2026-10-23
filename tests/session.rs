use fee_batch_ring_draw::session::Session;

#[test]
fn dispatch_and_presentation_are_parent_scoped_and_once_only() {
  let mut session = Session::default();
  assert!(!session.begin(0));
  assert!(session.begin(123));
  assert!(!session.begin(456));
  assert_eq!(session.dispatch(1), None);
  assert_eq!(session.dispatch(300), Some(123));
  assert_eq!(session.dispatch(300), None);
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
  assert_eq!(session.dispatch(1), None);
  session.end();
  assert!(!session.presentation(123));
  assert!(session.begin(123));
  assert_eq!(session.dispatch(300), Some(123));
  session.end();
  session.clear();
  assert!(!session.presentation(123));
}
