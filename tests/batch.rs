use fee_batch_ring_draw::batch::{execute, Inventory, Stock, Stop};

struct Pool {
  values: [i32; 40],
  fragments: i32,
  equipped: i32,
  next: usize,
  calls: usize,
  refill: bool,
  room: bool,
  fail: bool,
  bad_delta: bool,
  predictions: usize,
}

impl Default for Pool {
  fn default() -> Self {
    Self {
      values: [0; 40],
      fragments: 30000,
      equipped: 0,
      next: 0,
      calls: 0,
      refill: false,
      room: true,
      fail: false,
      bad_delta: false,
      predictions: 0,
    }
  }
}

impl Inventory for Pool {
  type Expected = usize;
  fn fragments(&mut self) -> Option<i32> {
    Some(self.fragments)
  }
  fn preview(&mut self) -> Option<usize> {
    self.predictions += 1;
    Some(self.next)
  }
  fn stock(&mut self, expected: &usize) -> Option<Stock> {
    Some(Stock {
      total: self.values[*expected],
      equipped: self.equipped,
      fragments: self.fragments,
      can_add: self.room,
    })
  }
  fn transact(&mut self, expected: &usize) -> bool {
    self.calls += 1;
    if self.fail {
      return false;
    }
    self.values[*expected] += if self.bad_delta { 0 } else { 1 };
    if !self.refill {
      self.fragments -= 100;
    }
    self.next = (self.next + 1) % 40;
    true
  }
}

#[test]
fn full_batch_uses_normal_cost_and_exactly_300_transactions() {
  let mut pool = Pool::default();
  let result = execute(&mut pool, 100, 300);
  assert_eq!((result.completed, result.stop), (300, Stop::Complete));
  assert_eq!(pool.calls, 300);
  assert_eq!(pool.values.iter().sum::<i32>(), 300);
  assert_eq!(pool.fragments, 0);
}

#[test]
fn fragment_cheat_does_not_extend_batch() {
  let mut pool = Pool { refill: true, fragments: 100000, ..Pool::default() };
  assert_eq!(execute(&mut pool, 100, 300).completed, 300);
  assert_eq!(pool.calls, 300);
}

#[test]
fn initial_partial_budget_remains_fixed_with_fragment_cheat() {
  let mut pool = Pool { refill: true, fragments: 350, ..Pool::default() };
  let result = execute(&mut pool, 100, 300);
  assert_eq!((result.completed, result.stop), (3, Stop::Funds));
}

#[test]
fn insufficient_funds_do_not_draw_or_advance_prediction() {
  let mut pool = Pool { fragments: 99, ..Pool::default() };
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::Funds);
  assert_eq!((pool.calls, pool.predictions), (0, 0));
}

#[test]
fn full_next_result_stops_without_reroll_or_payment() {
  let mut pool = Pool::default();
  pool.values[0] = 99;
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::Capacity);
  assert_eq!((pool.calls, pool.next, pool.fragments), (0, 0, 30000));
}

#[test]
fn unrelated_full_stack_does_not_block_other_results() {
  let mut pool = Pool::default();
  pool.values[7] = 99;
  let result = execute(&mut pool, 100, 300);
  assert_eq!((result.completed, result.stop), (7, Stop::Capacity));
  assert_eq!((pool.next, pool.calls, pool.fragments), (7, 7, 29300));
}

#[test]
fn total_of_99_counts_equipped_copies() {
  let mut pool = Pool { equipped: 1, ..Pool::default() };
  pool.values.fill(1);
  pool.values[0] = 99;
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::Capacity);
  assert_eq!(pool.calls, 0);
}

#[test]
fn pool_capacity_is_checked_before_any_transaction() {
  let mut pool = Pool { room: false, ..Pool::default() };
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::Capacity);
  assert_eq!(pool.calls, 0);
}

#[test]
fn failed_transaction_never_falls_back_or_retries() {
  let mut pool = Pool { fail: true, ..Pool::default() };
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::UnexpectedTransaction);
  assert_eq!(pool.calls, 1);
}

#[test]
fn no_inventory_increase_is_detected() {
  let mut pool = Pool { bad_delta: true, ..Pool::default() };
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::UnexpectedTransaction);
  assert_eq!(pool.calls, 1);
}

#[test]
fn invalid_prices_and_stock_are_rejected() {
  for price in [0, -1] {
    let mut pool = Pool::default();
    assert_eq!(execute(&mut pool, price, 300).stop, Stop::Invalid);
    assert_eq!(pool.calls, 0);
  }
  let mut pool = Pool::default();
  pool.values[0] = 100;
  assert_eq!(execute(&mut pool, 100, 300).stop, Stop::Invalid);
  assert_eq!(pool.calls, 0);
}

#[test]
fn configured_counts_drive_budget_and_result_count() {
  for count in [2, 10, 600, 1000] {
    let mut pool = Pool { fragments: 1000000, refill: true, ..Pool::default() };
    let result = execute(&mut pool, 100, count);
    assert_eq!((result.completed, result.stop), (count, Stop::Complete));
    assert_eq!(pool.calls, count as usize);
  }
  for count in [0, 1, 1001, u32::MAX] {
    let mut pool = Pool::default();
    assert_eq!(execute(&mut pool, 100, count).stop, Stop::Invalid);
    assert_eq!(pool.calls, 0);
  }
}
