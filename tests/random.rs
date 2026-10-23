use fee_batch_ring_draw::random::{preview, Rates, State};

#[test]
fn preview_copies_state_and_consumes_exactly_two_values() {
  let state = State([123456789, 362436069, 521288629, 88675123]);
  let rates = Rates { s: 5, a: 10, b: 30 };
  let (rank, index, next) = preview(state, rates, [10; 4]).unwrap();
  let mut native = state;
  assert_eq!(rank, rates.rank(native.value(100).unwrap()).unwrap());
  assert_eq!(index, native.value(10).unwrap() as usize);
  assert_eq!(next, native);
  assert_eq!(state, State([123456789, 362436069, 521288629, 88675123]));
}

#[test]
fn rank_boundaries_match_native_cumulative_comparisons() {
  let rates = Rates { s: 5, a: 10, b: 30 };
  for (roll, rank) in [(0, 3), (4, 3), (5, 2), (14, 2), (15, 1), (44, 1), (45, 0), (99, 0)] {
    assert_eq!(rates.rank(roll), Some(rank));
  }
  assert_eq!(rates.rank(100), None);
}

#[test]
fn invalid_rates_state_and_empty_candidates_fail_closed() {
  let state = State([1, 2, 3, 4]);
  assert!(preview(state, Rates { s: 101, a: 0, b: 0 }, [10; 4]).is_none());
  assert!(preview(state, Rates { s: u32::MAX, a: 1, b: 0 }, [10; 4]).is_none());
  assert!(preview(State([0; 4]), Rates { s: 5, a: 10, b: 30 }, [10; 4]).is_none());
  assert!(preview(state, Rates { s: 100, a: 0, b: 0 }, [0; 4]).is_none());
}

#[test]
fn labels_cover_all_game_languages_and_fallback() {
  for language in -1..=12 {
    assert!(fee_batch_ring_draw::labels::count(language).contains("300"));
  }
  assert_eq!(fee_batch_ring_draw::labels::count(10), "精炼300次");
}
