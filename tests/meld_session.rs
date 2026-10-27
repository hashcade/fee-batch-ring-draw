use fee_better_somniel::meld::{
  batch::{Cost, Plan},
  session::{Dispatch, Execution, Selection, Session},
};

fn selection(menu: usize) -> Selection {
  Selection {
    menu,
    rnid: "RNID_Test_C".into(),
    prompt: "Test prompt".into(),
    plan: Plan { cost: Cost { rings: 2, fragments: 100 }, count: 3 },
  }
}

#[test]
fn cannot_execute_without_showing_the_batch_confirmation() {
  let mut session = Session::default();
  session.prepare(Some(selection(1)));
  session.confirm(1);
  assert!(session.take_confirmed().is_none());
}

#[test]
fn confirmed_plan_survives_menu_destruction_but_is_consumed_only_once() {
  let mut session = Session::default();
  session.prepare(Some(selection(1)));
  session.mark_displayed();
  session.confirm(1);
  session.destroy_menu(1);
  assert!(session.take_confirmed().is_some());
  assert!(session.take_confirmed().is_none());
}

#[test]
fn cancel_and_unconfirmed_destruction_clear_the_plan() {
  let mut session = Session::default();
  session.prepare(Some(selection(1)));
  session.mark_displayed();
  session.destroy_menu(1);
  assert!(session.prompt().is_none());
  session.prepare(Some(selection(1)));
  session.mark_displayed();
  session.confirm(1);
  session.clear();
  assert!(session.take_confirmed().is_none());
}

#[test]
fn another_menu_cannot_confirm_a_plan() {
  let mut session = Session::default();
  session.prepare(Some(selection(1)));
  session.mark_displayed();
  session.confirm(2);
  assert!(session.take_confirmed().is_none());
}

#[test]
fn changing_selection_invalidates_prior_confirmation() {
  let mut session = Session::default();
  session.prepare(Some(selection(1)));
  session.mark_displayed();
  session.confirm(1);
  session.prepare(Some(selection(2)));
  assert!(session.take_confirmed().is_none());
}

#[test]
fn repeated_or_nested_dispatch_cannot_add_an_extra_native_transaction() {
  let mut run = Execution::default();
  assert!(matches!(run.dispatch(), Dispatch::Vanilla));
  assert!(run.begin(Some(selection(1))));
  assert!(!run.begin(Some(selection(2))));
  assert!(matches!(run.dispatch(), Dispatch::Batch(_)));
  assert!(matches!(run.dispatch(), Dispatch::Suppress));
  run.end();
  assert!(matches!(run.dispatch(), Dispatch::Vanilla));
}

#[test]
fn original_execution_without_a_batch_is_not_changed() {
  let mut run = Execution::default();
  assert!(run.begin(None));
  assert!(matches!(run.dispatch(), Dispatch::Vanilla));
  run.end();
}

#[test]
fn single_choice_discards_the_batch_but_batch_choice_keeps_original_unit_costs() {
  let mut session = Session::default();
  session.prepare(Some(selection(1)));
  assert_eq!(session.batch_count(), Some(3));
  session.mark_displayed();
  session.confirm_choice(1, false);
  assert!(session.take_confirmed().is_none());
  session.prepare(Some(selection(1)));
  session.mark_displayed();
  session.confirm_choice(1, true);
  let selected = session.take_confirmed().unwrap();
  assert_eq!(selected.plan.cost, Cost { rings: 2, fragments: 100 });
  assert_eq!(selected.plan.count, 3);
  let mut single = selection(1);
  single.plan.count = 1;
  session.prepare(Some(single));
  assert_eq!(session.batch_count(), None);
}
