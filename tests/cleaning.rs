use fee_better_somniel::cleaning::{label, Candidate, Session};

#[test]
fn native_list_is_deduplicated_by_shared_ring_not_dlc_portrait() {
  let mut session = Session::default();
  assert!(session.prepare(
    1,
    [
      Candidate { god: 10, kind: -1, dirty: 255 },
      Candidate { god: 20, kind: 0, dirty: 60 },
      Candidate { god: 20, kind: 1, dirty: 60 },
      Candidate { god: 20, kind: 2, dirty: 60 },
      Candidate { god: 30, kind: 3, dirty: 1 },
      Candidate { god: 40, kind: -1, dirty: 0 },
      Candidate { god: 50, kind: 4, dirty: 60 },
      Candidate { god: 0, kind: -1, dirty: 60 },
    ]
  ));
  let rings = session.take(1).unwrap();
  assert_eq!(rings.iter().map(|r| r.god).collect::<Vec<_>>(), vec![10, 20, 30]);
  assert!(session.take(1).is_none());
}

#[test]
fn cancel_selection_change_and_wrong_owner_discard_the_plan() {
  let ring = Candidate { god: 10, kind: -1, dirty: 1 };
  let mut session = Session::default();
  assert!(!session.prepare(0, [ring]));
  assert!(session.prepare(1, [ring]));
  assert!(session.take(2).is_none());
  assert!(session.take(1).is_none());
  assert!(session.prepare(1, [ring]));
  session.clear();
  assert!(session.take(1).is_none());
  assert!(!session.prepare(1, [Candidate { dirty: 0, ..ring }]));
}

#[test]
fn every_game_language_has_a_clean_all_label() {
  for language in 0..12 {
    assert!(!label(language).is_empty());
    for actor in [false, true] {
      assert!(!fee_better_somniel::cleaning::option_title(language, actor).is_empty());
    }
  }
  assert_eq!(fee_better_somniel::cleaning::option_title(10, false), "自动研磨");
  assert_eq!(fee_better_somniel::cleaning::option_title(10, true), "研磨角色");
  assert_eq!(label(10), "全部清洁");
  assert_eq!(label(-1), label(1));
}

#[test]
fn native_selection_is_not_truncated_to_an_arbitrary_limit() {
  let mut session = Session::default();
  assert!(session.prepare(1, (1..=160).map(|god| Candidate { god, kind: -1, dirty: 1 })));
  assert_eq!(session.take(1).unwrap().len(), 160);
}
