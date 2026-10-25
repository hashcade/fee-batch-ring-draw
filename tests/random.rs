use fee_better_somniel::random::{preview, Rates, State};

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
    assert!(fee_better_somniel::labels::count(language, 300).contains("300"));
  }
  assert_eq!(fee_better_somniel::labels::count(10, 300), "精炼300次");
  for language in -1..=12 {
    assert!(fee_better_somniel::labels::count(language, 600).contains("600"));
  }
}

#[test]
fn native_instruction_vectors_match_300_draws_including_high_bits() {
  // Golden vectors executed from the supplied NSO's Random.GetValue machine
  // code in Unicorn, using alternating moduli 100 and 10 (two calls per ring).
  let cases = [
    ([1, 2, 3, 4], 0x23725ddb5cf4ddc8_u64, [4259486305, 1074253361, 466384795, 3333082406]),
    (
      [123456789, 362436069, 521288629, 88675123],
      0xec02af7437887973,
      [1531322273, 3778893239, 634599342, 4055889606],
    ),
    (
      [0xffffffff, 0x80000000, 0x12345678, 0x87654321],
      0xa7a51b287aa06f96,
      [525393096, 3918500895, 2384445792, 3355973262],
    ),
  ];
  for (seed, expected_hash, expected_state) in cases {
    let mut state = State(seed);
    let mut hash = 0xcbf29ce484222325_u64;
    for _ in 0..300 {
      for modulus in [100, 10] {
        hash = (hash ^ state.value(modulus).unwrap() as u64).wrapping_mul(0x100000001b3);
      }
    }
    assert_eq!(hash, expected_hash);
    assert_eq!(state.0, expected_state);
  }
}
