use fee_better_somniel::resources::*;

#[test]
fn resource_steps_keep_tails_and_stop_at_limits() {
  assert_eq!(adjusted(2, 327_459, 1), 427_459);
  assert_eq!(adjusted(2, 327_459, -1), 227_459);
  for (index, resource) in RESOURCES.iter().enumerate() {
    assert_eq!(adjusted(index, resource.maximum, 1), resource.maximum);
    assert_eq!(adjusted(index, resource.maximum - 1, 1), resource.maximum);
    assert_eq!(adjusted(index, 0, -1), 0);
    assert_eq!(adjusted(index, 1, -1), 0);
    assert_eq!(adjusted(index, 0, 1), resource.step);
    assert_eq!(adjusted(index, 37, 0), 37);
  }
}

#[test]
fn mixed_sp_never_impersonates_one_ally() {
  assert_eq!(common_value([]), None);
  assert_eq!(common_value([123, 124]), None);
  assert_eq!(common_value([827, 827]), Some(827));
  assert_eq!(common_value([0]), Some(0));
}

#[test]
fn resource_values_and_help_are_explicit() {
  assert_eq!(number(9_999_999), "9,999,999");
  assert_eq!(number(999), "999");
  assert_eq!(number(0), "0");
  assert!(help(10, Some(2)).contains("步长100,000"));
  assert!(help(10, Some(0)).contains("A应用一次"));
}
