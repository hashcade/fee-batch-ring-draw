//! Bounded, all-or-nothing reservations for already-generated exploration rewards.

use std::collections::{BTreeMap, BTreeSet};

#[cfg(target_os = "switch")]
pub(crate) mod native;
pub mod profile;

pub const MAX_REWARD_COPIES: u32 = 8192;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capacity {
  pub owned: u32,
  pub maximum: u32,
}

#[derive(Clone, Debug)]
pub struct Candidate {
  /// The native claim flag, not the item ID: two drops may contain the same item.
  pub identity: String,
  pub requirements: Vec<(usize, u32)>,
  pub copies: u32,
}

#[derive(Default)]
pub struct Reservations {
  remaining: BTreeMap<usize, u32>,
  claimed: BTreeSet<String>,
  copies: u32,
}

impl Reservations {
  pub fn capacity(&mut self, key: usize, capacity: Capacity) -> bool {
    let Some(remaining) = capacity.maximum.checked_sub(capacity.owned) else {
      return false;
    };
    // A shared budget must never be reset by a second source of the same item.
    self.remaining.entry(key).or_insert(remaining);
    true
  }

  pub fn reserve(&mut self, candidate: &Candidate) -> bool {
    if candidate.identity.is_empty()
      || self.claimed.contains(&candidate.identity)
      || candidate.requirements.is_empty()
    {
      return false;
    }
    let Some(copies) = self.copies.checked_add(candidate.copies) else { return false };
    if copies > MAX_REWARD_COPIES {
      return false;
    }
    let mut totals = BTreeMap::<usize, u32>::new();
    for &(key, amount) in &candidate.requirements {
      if amount == 0 {
        return false;
      }
      let total = totals.entry(key).or_default();
      let Some(sum) = total.checked_add(amount) else { return false };
      *total = sum;
    }
    if totals.iter().any(|(key, amount)| self.remaining.get(key).is_none_or(|left| left < amount)) {
      return false;
    }
    for (key, amount) in totals {
      *self.remaining.get_mut(&key).unwrap() -= amount;
    }
    self.copies = copies;
    self.claimed.insert(candidate.identity.clone());
    true
  }
}
