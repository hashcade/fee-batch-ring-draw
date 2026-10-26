use fee_better_somniel::collector::{Candidate, Capacity, Reservations, MAX_REWARD_COPIES};

fn drop(flag: &str, requirements: &[(usize, u32)], copies: u32) -> Candidate {
  Candidate { identity: flag.into(), requirements: requirements.into(), copies }
}

#[test]
fn donation_adjusted_drops_share_inventory_capacity() {
  let mut plan = Reservations::default();
  assert!(plan.capacity(10, Capacity { owned: 95, maximum: 99 }));
  assert!(plan.reserve(&drop("a", &[(10, 3)], 3)));
  assert!(!plan.reserve(&drop("b", &[(10, 3)], 3)));
  assert!(plan.capacity(10, Capacity { owned: 95, maximum: 99 }));
  assert!(!plan.reserve(&drop("b", &[(10, 3)], 3)));
  assert!(plan.reserve(&drop("c", &[(10, 1)], 1)));
}

#[test]
fn claim_identity_deduplicates_sources_not_item_kinds() {
  let mut plan = Reservations::default();
  plan.capacity(10, Capacity { owned: 0, maximum: 99 });
  assert!(plan.reserve(&drop("a", &[(10, 1)], 1)));
  assert!(!plan.reserve(&drop("a", &[(10, 1)], 1)));
  assert!(plan.reserve(&drop("b", &[(10, 1)], 1)));
}

#[test]
fn source_reservation_is_atomic_even_with_repeated_resources() {
  let mut plan = Reservations::default();
  plan.capacity(1, Capacity { owned: 0, maximum: 3 });
  plan.capacity(2, Capacity { owned: 0, maximum: 1 });
  assert!(!plan.reserve(&drop("a", &[(1, 2), (1, 2), (2, 1)], 4)));
  assert!(plan.reserve(&drop("a", &[(1, 3), (2, 1)], 3)));
}

#[test]
fn animal_species_and_convoy_slots_are_shared_budgets() {
  let mut plan = Reservations::default();
  plan.capacity(1, Capacity { owned: 98, maximum: 99 });
  plan.capacity(2, Capacity { owned: 998, maximum: 999 });
  assert!(plan.reserve(&drop("animal1", &[(1, 1)], 0)));
  assert!(!plan.reserve(&drop("animal2", &[(1, 1)], 0)));
  assert!(plan.reserve(&drop("sword", &[(2, 1)], 1)));
  assert!(!plan.reserve(&drop("staff", &[(2, 1)], 1)));
}

#[test]
fn invalid_or_unbounded_sources_cannot_consume_budget() {
  let mut plan = Reservations::default();
  assert!(!plan.capacity(5, Capacity { owned: 100, maximum: 99 }));
  plan.capacity(1, Capacity { owned: 0, maximum: u32::MAX });
  for source in [
    drop("", &[(1, 1)], 1),
    drop("a", &[], 0),
    drop("a", &[(1, 0)], 1),
    drop("a", &[(2, 1)], 1),
    drop("a", &[(1, u32::MAX), (1, 1)], 1),
    drop("a", &[(1, 1)], MAX_REWARD_COPIES + 1),
  ] {
    assert!(!plan.reserve(&source));
  }
  assert!(plan.reserve(&drop("a", &[(1, MAX_REWARD_COPIES)], MAX_REWARD_COPIES)));
  assert!(!plan.reserve(&drop("b", &[(1, 1)], 1)));
}
