//! Bounded transactions. Native execution remains responsible for game state.

pub const DRAWS: u32 = 300;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stock {
  pub total: i32,
  pub equipped: i32,
  pub fragments: i32,
  pub can_add: bool,
}

impl Stock {
  fn valid(self) -> bool {
    (0..=99).contains(&self.total)
      && (0..=self.total).contains(&self.equipped)
      && self.fragments >= 0
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
  Complete,
  Funds,
  Capacity,
  Invalid,
  UnexpectedTransaction,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
  pub completed: u32,
  pub stop: Stop,
}

pub trait Inventory {
  type Expected;
  fn fragments(&mut self) -> Option<i32>;
  fn preview(&mut self) -> Option<Self::Expected>;
  fn stock(&mut self, expected: &Self::Expected) -> Option<Stock>;
  fn transact(&mut self, expected: &Self::Expected) -> bool;
}

pub fn execute<I: Inventory>(inventory: &mut I, cost: i32) -> Outcome {
  let mut outcome = Outcome { completed: 0, stop: Stop::Invalid };
  if cost <= 0 {
    return outcome;
  }
  let Some(fragments) = inventory.fragments().filter(|value| *value >= 0) else {
    return outcome;
  };
  // A fragment-refill cheat cannot extend the initial budget or the 300 cap.
  let budget = DRAWS.min((fragments / cost) as u32);
  for _ in 0..budget {
    let Some(expected) = inventory.preview() else { return outcome };
    let Some(before) = inventory.stock(&expected).filter(|stock| stock.valid()) else {
      return outcome;
    };
    if before.fragments < cost {
      outcome.stop = Stop::Funds;
      return outcome;
    }
    if !before.can_add || before.total == 99 {
      outcome.stop = Stop::Capacity;
      return outcome;
    }
    // Once attempted, a failed/mismatched transaction is never retried.
    if !inventory.transact(&expected) {
      outcome.stop = Stop::UnexpectedTransaction;
      return outcome;
    }
    let Some(after) = inventory.stock(&expected).filter(|stock| stock.valid()) else {
      outcome.stop = Stop::UnexpectedTransaction;
      return outcome;
    };
    if after.total != before.total + 1
      || after.equipped != before.equipped
      || after.fragments < before.fragments - cost
    {
      outcome.stop = Stop::UnexpectedTransaction;
      return outcome;
    }
    outcome.completed += 1;
  }
  outcome.stop = if budget == DRAWS { Stop::Complete } else { Stop::Funds };
  outcome
}
