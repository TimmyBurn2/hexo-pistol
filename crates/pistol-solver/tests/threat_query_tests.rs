//! The threat state's answers, pinned against fourteen golden positions.
//!
//! Each test below is a claim about ONE behaviour, checked on the position that
//! exhibits it, and each was written against a MUTANT of the reference and
//! confirmed to fail before it was allowed to pass: a row no fixture separates
//! is inert however true its prose, and this design shipped two such rows before
//! anyone made the mutation.
//!
//! The fixture's answers were computed by the from-scratch reference in
//! `common::reference`, cross-checked against the independent region scan in
//! `common::region`, and compared value for value against a reference written by
//! a different session. Nothing in the file was read off the code it tests.
//!
//! # RULE9-JUSTIFICATION: one golden fixture, one suite (CLAUDE.md rule 9).
//!
//! Every test here reads the same sha-pinned file through the same loader and
//! plays it through the same builder. Splitting the suite would either duplicate
//! that scaffolding per file or hoist it into shared code no other suite uses,
//! and it would separate each behaviour from the golden comparison that gives it
//! its numbers. The suite splits when the solver grows a second fixture class —
//! forcing sequences, which WP-1.5b brings — and that is when the split pays.

mod common;

use common::fixtures::{THREAT_FIXTURE_FILE, ThreatCase, threat_case, threat_cases};
use common::reference::Reference;
use common::region::region_scan;
use common::sha256::sha256_hex;
use common::{assert_pinned, cell_list, play, window_list};
use pistol_core::{Coord, GameState, Outcome, Player};
use pistol_solver::{
    Cover, HitBudget, LiveCount, MinimalCover, NearHot, StonesLeft, ThreatState, WinWitness,
};

/// The SHA-256 of `tests/fixtures/threat_v0.txt`.
///
/// Changing the fixture means changing this line, in the same commit, having
/// looked at what changed.
const THREAT_V0_SHA256: &str = "fec6db7aedfcab717f2e972bc35a1ee04074b23be47835abed3d4c4d0fa3d990";

#[test]
fn threat_v0_fixture_matches_its_pinned_sha256() {
    assert_pinned(THREAT_FIXTURE_FILE, THREAT_V0_SHA256);
}

#[test]
fn sha256_matches_published_test_vectors() {
    // A fixture pin computed by an unverified hash pins nothing.
    let vectors: [(&[u8], &str); 3] = [
        (
            b"",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
    ];
    for (input, expected) in vectors {
        assert_eq!(sha256_hex(input), expected, "vector {input:?}");
    }
}

/// The whole surface, on every case: the shipped state against the file.
#[test]
fn threat_state_answers_match_the_golden_fixture() {
    for case in threat_cases() {
        let (game, threats) = play(&case.plies);
        assert_eq!(
            (
                game.to_move(),
                game.phase(),
                game.stones_owed(),
                game.outcome()
            ),
            (
                case.state.to_move,
                case.state.phase,
                case.state.stones_owed,
                case.state.outcome
            ),
            "case `{}` (line {}): the position is not the one the record is about",
            case.name,
            case.line
        );
        for side in [Player::P1, Player::P2] {
            let want = case.side(side);
            let where_ = format!("case `{}` / {side}", case.name);
            assert_eq!(
                window_list(threats.hot_windows(side)),
                window_list(&want.hot),
                "{where_}: hot"
            );
            assert_eq!(
                window_list(threats.win_in_one_ply_windows(side)),
                window_list(&want.win1),
                "{where_}: win1"
            );
            assert_eq!(
                window_list(threats.completed_windows(side)),
                window_list(&want.completed),
                "{where_}: completed"
            );
            assert_eq!(
                window_list(threats.live_windows_at_count(side, LiveCount::Three)),
                window_list(&want.live3),
                "{where_}: live3"
            );
            assert_eq!(
                window_list(threats.live_windows_at_count(side, LiveCount::Two)),
                window_list(&want.live2),
                "{where_}: live2"
            );
            let mut cells = Vec::new();
            threats.threat_cells(side, &mut cells);
            assert_eq!(
                cell_list(&cells),
                cell_list(&want.threat_cells),
                "{where_}: threat_cells"
            );
            threats.cells_raising_to_hot(side, NearHot::Three, &mut cells);
            assert_eq!(
                cell_list(&cells),
                cell_list(&want.raise_cells),
                "{where_}: raise_cells"
            );
            for (index, budget) in [HitBudget::One, HitBudget::Two].into_iter().enumerate() {
                assert_eq!(
                    threats.blocking_covers(side.opponent(), budget),
                    want.cover[index],
                    "{where_}: cover at {budget:?}"
                );
            }
            for (index, left) in [StonesLeft::One, StonesLeft::Two].into_iter().enumerate() {
                assert_eq!(
                    threats.can_win_this_turn(side, left),
                    want.canwin[index],
                    "{where_}: canwin at {left:?}"
                );
            }
        }
    }
}

/// R1 shares the implementation's assumption that only a window holding a stone
/// can matter. R2 assumes nothing, and this is where the two meet.
#[test]
fn the_reference_agrees_with_an_independent_region_scan() {
    for case in threat_cases() {
        let (game, _) = play(&case.plies);
        let reference = Reference::from_board(game.board());
        let scanned = region_scan(game.board());
        for (window, held) in &scanned {
            let known = reference.table().get(window);
            if held.p1 != 0 || held.p2 != 0 {
                assert_eq!(
                    known,
                    Some(held),
                    "case `{}`: the region scan found {window:?} the reference missed",
                    case.name
                );
            } else {
                assert!(
                    known.is_none(),
                    "case `{}`: the reference kept the stoneless {window:?}",
                    case.name
                );
            }
        }
        for window in reference.table().keys() {
            assert!(
                scanned.contains_key(window),
                "case `{}`: the reference holds {window:?} outside the scanned box",
                case.name
            );
        }
    }
}

#[test]
fn hot_window_requires_four_and_live() {
    // Four is the count, and it is not the whole condition. The second position
    // is the first with a P2 stone inside the window: same four P1 stones, and
    // the window is DEAD.
    let (_, live) = play(&threat_case("single_hot_window").plies);
    assert_eq!(
        window_list(live.hot_windows(Player::P1)),
        "ConstR@0,0",
        "four own stones and no opponent stone is hot"
    );
    let (_, dead) = play(&threat_case("single_hot_window_dead_when_opponent_enters").plies);
    assert!(
        dead.hot_windows(Player::P1).is_empty(),
        "a window holding an opponent stone is dead however many own stones it holds"
    );
    // And the count is four and not three: the count-3 windows of the live
    // position are not in its hot set.
    for &window in live.live_windows_at_count(Player::P1, LiveCount::Three) {
        assert!(
            !live.hot_windows(Player::P1).contains(&window),
            "{window:?} holds three and is not hot"
        );
    }
}

#[test]
fn hot_window_is_win_in_one_turn_for_owner_on_move() {
    // D-243 CHECKED, not quoted: play the witness through the rules and see the
    // game end. THE SIDE IS NAMED in each case, because a fixture with hot
    // windows on both sides would otherwise have the witness played by whoever
    // happens to be on move — and the second stone of a pair would silently go
    // to the opponent while `place` returned Ok.
    for (name, owner) in [
        ("win_in_one_ply_each_side", Player::P2),
        ("counter_threat_defender_can_win", Player::P2),
        ("double_threat_but_defender_wins_first", Player::P1),
    ] {
        let case = threat_case(name);
        let (mut game, threats) = play(&case.plies);
        assert_eq!(game.to_move(), owner, "{name}: the owner must be on move");
        let left = StonesLeft::from_state(&game).expect("an ongoing position owes stones");
        let witness = threats
            .can_win_this_turn(owner, left)
            .unwrap_or_else(|| panic!("{name}: the owner can win this turn"));
        match witness {
            WinWitness::OnePly { at, .. } => {
                game.place(at).expect("the witness cell is legal");
            }
            WinWitness::Pair { first, second, .. } => {
                assert_eq!(left, StonesLeft::Two, "{name}: a pair needs two stones");
                game.place(first).expect("the first witness cell is legal");
                assert!(
                    !game.outcome().is_decided(),
                    "{name}: this pair's first stone does not win alone"
                );
                game.place(second)
                    .expect("the second witness cell is legal");
            }
        }
        assert_eq!(
            game.outcome().winner(),
            Some(owner),
            "{name}: playing the witness must win FOR THE OWNER"
        );
    }
}

#[test]
fn win_in_one_ply_requires_five_and_live() {
    // Five, live, and one empty cell each.
    let (_, five) = play(&threat_case("win_in_one_ply_each_side").plies);
    let mut cells = Vec::new();
    five.win_in_one_ply_cells(Player::P1, &mut cells);
    assert_eq!(cell_list(&cells), "-1,0 5,0");
    five.win_in_one_ply_cells(Player::P2, &mut cells);
    assert_eq!(cell_list(&cells), "-1,8 5,8");

    // Four is not five.
    let (_, four) = play(&threat_case("pair_completion_win").plies);
    four.win_in_one_ply_cells(Player::P1, &mut cells);
    assert!(
        cells.is_empty(),
        "two hot windows at four own stones report no win-in-one-ply cell"
    );

    // And the LIVENESS half, which no other fixture exercises: five own stones
    // in each of two windows, and an opponent stone in both.
    //
    // THE ASSERTION IS ON THE WINDOWS AND NOT ONLY ON THE CELLS, and that is
    // the whole of this half. A dead five-window here is FULL — five own stones
    // and the opponent's in the sixth cell — so it has no empty and contributes
    // no cell either way. A version of this test that asked only for cells
    // passed against a mutant that put dead five-windows straight into the set,
    // which is a row testing nothing.
    let (_, dead) = play(&threat_case("win_in_one_ply_dead_when_opponent_enters").plies);
    assert!(
        dead.win_in_one_ply_windows(Player::P1).is_empty(),
        "a five-window holding an opponent stone is dead and completes nothing, got {}",
        window_list(dead.win_in_one_ply_windows(Player::P1))
    );
    dead.win_in_one_ply_cells(Player::P1, &mut cells);
    assert!(cells.is_empty(), "and so it offers no completing cell");
    assert!(dead.hot_windows(Player::P1).is_empty());
    // The control, so the assertion above is not vacuous: the live position DOES
    // report win-in-one-ply windows.
    assert_eq!(five.win_in_one_ply_windows(Player::P1).len(), 2);
}

#[test]
fn live_windows_at_count_three_reported_for_both_sides() {
    let case = threat_case("count_three_live_both_sides");
    let (_, threats) = play(&case.plies);
    assert_eq!(
        window_list(threats.live_windows_at_count(Player::P1, LiveCount::Three)),
        "ConstS@-3,3 ConstS@-2,2 ConstS@-1,1 ConstS@0,0"
    );
    assert_eq!(
        window_list(threats.live_windows_at_count(Player::P2, LiveCount::Three)),
        "ConstR@-3,8 ConstR@-2,8 ConstR@-1,8 ConstR@0,8"
    );
    for side in [Player::P1, Player::P2] {
        assert!(
            threats.hot_windows(side).is_empty(),
            "{side} holds three, which is not four"
        );
    }
}

#[test]
fn can_win_this_turn_true_via_pair_completion_of_hot_window() {
    let (_, threats) = play(&threat_case("pair_completion_win").plies);
    assert_eq!(
        threats.can_win_this_turn(Player::P1, StonesLeft::Two),
        Some(WinWitness::Pair {
            first: Coord::new(-1, 0),
            second: Coord::new(2, 0),
            window: common::parse_window("ConstR@-1,0", 0),
        }),
        "two stones fill a four-window's two empties"
    );
    // D-243's PHASE CONDITION: the same position with one stone left is not a
    // win now. A generator reading the count without the phase claims a mate it
    // cannot play.
    assert_eq!(
        threats.can_win_this_turn(Player::P1, StonesLeft::One),
        None,
        "one stone does not fill two empties"
    );
}

#[test]
fn can_win_this_turn_false_when_only_new_hot_window_creatable() {
    // D-243 consequence (1): CREATING a hot window is not winning this turn.
    // Both sides here can make one with two stones, and neither can win.
    let (_, threats) = play(&threat_case("count_three_live_both_sides").plies);
    for side in [Player::P1, Player::P2] {
        assert_eq!(threats.can_win_this_turn(side, StonesLeft::Two), None);
        assert_eq!(threats.can_win_this_turn(side, StonesLeft::One), None);
        assert!(
            !threats
                .live_windows_at_count(side, LiveCount::Three)
                .is_empty(),
            "{side} does have windows one stone short of hot"
        );
    }
}

#[test]
fn overlapping_hot_windows_share_hitting_cell() {
    let (_, threats) = play(&threat_case("overlapping_hot_windows_one_shared_cell").plies);
    assert_eq!(threats.hot_windows(Player::P1).len(), 3);
    assert!(
        !threats.min_hitting_set_exceeds(HitBudget::One, threats.hot_windows(Player::P1)),
        "one cell hits all three"
    );
    let one = threats.blocking_covers(Player::P2, HitBudget::One);
    assert_eq!(
        one,
        Cover::Minimal(vec![
            MinimalCover::One(Coord::new(3, 0)),
            MinimalCover::One(Coord::new(5, 0)),
        ])
    );
    // THE NON-DEGENERACY PIN: a second stone in hand adds no cover here. Without
    // inclusion-minimality every pair containing 3,0 or 5,0 would come back.
    assert_eq!(
        threats.blocking_covers(Player::P2, HitBudget::Two),
        one,
        "a bigger budget is not a longer list of supersets"
    );
}

#[test]
fn unblockable_when_hitting_set_exceeds_two() {
    let (_, threats) = play(&threat_case("true_double_threat").plies);
    let hot = threats.hot_windows(Player::P1);
    assert!(
        threats.min_hitting_set_exceeds(HitBudget::Two, hot),
        "an open four and a blocked four twenty rows away need three cells"
    );
    assert_eq!(
        threats.blocking_covers(Player::P2, HitBudget::Two),
        Cover::Impossible
    );
    // F-4's stated case, both directions: at zero budget a standing hot window
    // is unhit, and an EMPTY family is not.
    assert!(threats.min_hitting_set_exceeds(HitBudget::Zero, hot));
    assert!(!threats.min_hitting_set_exceeds(HitBudget::Zero, &[]));
}

#[test]
fn unblockable_primitive_does_not_claim_win_when_defender_can_win() {
    // THE PURITY BOUNDARY. The primitive is a statement about hitting sets, so
    // it stays TRUE here even though P1 does not win: P2 is on move and has a
    // one-stone completion. What is false is the COMPOSITION, and that is the
    // caller's to make.
    let case = threat_case("double_threat_defender_to_move_wins_first");
    let (game, threats) = play(&case.plies);
    assert_eq!(game.to_move(), Player::P2);
    assert!(
        threats.unblockable_double_threat(Player::P1, HitBudget::Two),
        "the primitive answers about hitting sets and nothing else"
    );
    assert!(matches!(
        threats.can_win_this_turn(Player::P2, StonesLeft::Two),
        Some(WinWitness::OnePly { .. })
    ));
    assert!(
        !composed_win(&game, &threats, Player::P1),
        "and the composition, which is what a search would ask, is false"
    );
}

#[test]
fn min_hitting_set_exceeds_is_false_on_an_empty_hot_set() {
    // The minimum hitting set of an empty family is zero, so it exceeds NO
    // budget. Every side of every fixture that owns no hot window, enumerated
    // from the fixture rather than by hand: getting this backwards claims an
    // unblockable double threat for a side that owns nothing.
    let mut instances = Vec::new();
    for case in threat_cases() {
        let (_, threats) = play(&case.plies);
        for side in [Player::P1, Player::P2] {
            if !threats.hot_windows(side).is_empty() {
                continue;
            }
            instances.push(format!("{}/{side}", case.name));
            for budget in [HitBudget::Zero, HitBudget::One, HitBudget::Two] {
                assert!(
                    !threats.min_hitting_set_exceeds(budget, threats.hot_windows(side)),
                    "{}/{side}: nothing to hit does not exceed {budget:?}",
                    case.name
                );
                assert!(
                    !threats.unblockable_double_threat(side, budget),
                    "{}/{side}: a side with no hot window threatens nothing",
                    case.name
                );
                assert_eq!(
                    threats.blocking_covers(side.opponent(), budget),
                    Cover::NothingToBlock,
                    "{}/{side}: and there is nothing to block, which is not `no cover exists`",
                    case.name
                );
            }
        }
    }
    assert_eq!(
        instances.len(),
        13,
        "the empty-hot census moved: {instances:?}"
    );
    // The control: one hot window is not an empty family.
    let (_, one) = play(&threat_case("single_hot_window").plies);
    let hot = one.hot_windows(Player::P1);
    assert!(!one.min_hitting_set_exceeds(HitBudget::One, hot));
    assert!(!one.min_hitting_set_exceeds(HitBudget::Two, hot));
    assert!(one.min_hitting_set_exceeds(HitBudget::Zero, hot));
}

#[test]
fn blocking_covers_impossible_at_one_stone_succeeds_at_two() {
    // The stones-left discrimination: the same position answers differently to
    // a defender with one stone and to one with two.
    for name in [
        "win_in_one_ply_each_side",
        "counter_threat_defender_can_win",
    ] {
        let (_, threats) = play(&threat_case(name).plies);
        assert_eq!(
            threats.blocking_covers(Player::P2, HitBudget::One),
            Cover::Impossible,
            "{name}: one stone cannot answer P1 here"
        );
        let two = threats.blocking_covers(Player::P2, HitBudget::Two);
        assert!(
            matches!(&two, Cover::Minimal(covers) if !covers.is_empty()),
            "{name}: two stones can, and the answer is a non-empty list, got {two:?}"
        );
    }
}

#[test]
fn threat_cells_are_the_empties_of_hot_windows() {
    let mut cells = Vec::new();
    for name in [
        "single_hot_window",
        "overlapping_hot_windows_one_shared_cell",
        "counter_threat_defender_can_win",
    ] {
        let (_, threats) = play(&threat_case(name).plies);
        for side in [Player::P1, Player::P2] {
            threats.threat_cells(side, &mut cells);
            let mut expected: Vec<Coord> = threats
                .hot_windows(side)
                .iter()
                .flat_map(|&window| {
                    let masks = threats.masks(window);
                    (0..6u8)
                        .filter(move |index| masks.empties() & (1 << index) != 0)
                        .map(move |index| window.cell(index))
                })
                .collect();
            expected.sort_unstable();
            expected.dedup();
            assert_eq!(cells, expected, "{name}/{side}");
        }
    }
    // And they are NOT the cells that would make a window hot. On a position
    // with no hot window and four count-three windows, one set is empty and the
    // other is not.
    let (_, near) = play(&threat_case("count_three_live_both_sides").plies);
    near.threat_cells(Player::P1, &mut cells);
    assert!(cells.is_empty(), "no hot window, no threat cell");
    near.cells_raising_to_hot(Player::P1, NearHot::Three, &mut cells);
    assert!(!cells.is_empty(), "but there are cells that would make one");
}

#[test]
fn cells_raising_to_hot_reports_the_fourth_stone() {
    // The UNION of the count-three windows' empties, and not any one window's:
    // P1 has four such windows here and no window's three empties equal the
    // answer.
    let (_, threats) = play(&threat_case("count_three_live_both_sides").plies);
    let mut cells = Vec::new();
    threats.cells_raising_to_hot(Player::P1, NearHot::Three, &mut cells);
    let windows = threats.live_windows_at_count(Player::P1, LiveCount::Three);
    assert_eq!(windows.len(), 4);
    for &window in windows {
        let masks = threats.masks(window);
        let empties: Vec<Coord> = (0..6u8)
            .filter(|index| masks.empties() & (1 << index) != 0)
            .map(|index| window.cell(index))
            .collect();
        assert_eq!(
            empties.len(),
            3,
            "a live count-three window has three empties"
        );
        assert_ne!(empties, cells, "no single window's empties are the answer");
        for cell in empties {
            assert!(cells.contains(&cell), "{cell} is missing from the union");
        }
    }
    // One stone on any of them makes a hot window, which is the claim.
    for &cell in &cells {
        let mut extended = threats.clone();
        extended.apply(cell, Player::P1);
        assert!(
            !extended.hot_windows(Player::P1).is_empty(),
            "{cell} does not raise anything to hot"
        );
    }
}

#[test]
fn composition_requires_the_threatened_side_to_be_on_move() {
    // The pair differs ONLY in whose move it is: two padding stones are appended
    // and every P1 answer is unchanged. So this is the one pair that separates
    // clause (a) of the composition from the rest of it.
    let (threatened_to_move, threats_a) = play(&threat_case("true_double_threat").plies);
    let (threatener_to_move, threats_b) = play(&threat_case("true_double_threat_p1_to_move").plies);
    assert_eq!(threatened_to_move.to_move(), Player::P2);
    assert_eq!(threatener_to_move.to_move(), Player::P1);
    // Clause (b) holds in BOTH, so it cannot be what separates them.
    for (game, threats) in [
        (&threatened_to_move, &threats_a),
        (&threatener_to_move, &threats_b),
    ] {
        assert!(threats.unblockable_double_threat(Player::P1, HitBudget::Two));
        let left = StonesLeft::from_state(game).expect("both positions are ongoing");
        assert_eq!(threats.can_win_this_turn(Player::P2, left), None);
    }
    assert!(
        composed_win(&threatened_to_move, &threats_a, Player::P1),
        "P2 to move against an unblockable P1 threat it cannot outrun: P1 wins"
    );
    assert!(
        !composed_win(&threatener_to_move, &threats_b, Player::P1),
        "P1 to move: asking whether P2 can win asks about a turn P2 does not have"
    );
}

#[test]
fn counter_threat_position_reports_both_sides_correctly() {
    let (_, threats) = play(&threat_case("counter_threat_defender_can_win").plies);
    assert_eq!(threats.hot_windows(Player::P1).len(), 3);
    assert!(threats.min_hitting_set_exceeds(HitBudget::One, threats.hot_windows(Player::P1)));
    assert!(!threats.min_hitting_set_exceeds(HitBudget::Two, threats.hot_windows(Player::P1)));
    assert!(matches!(
        threats.can_win_this_turn(Player::P2, StonesLeft::One),
        Some(WinWitness::OnePly { .. })
    ));
    // THE CROSS-WINDOW ASSERTION: exactly three minimal two-covers, and a pair
    // drawn from their own cell union that is NOT among them.
    let covers = threats.blocking_covers(Player::P2, HitBudget::Two);
    assert_eq!(
        covers,
        Cover::Minimal(vec![
            MinimalCover::Two {
                first: Coord::new(-2, 0),
                second: Coord::new(4, 0),
            },
            MinimalCover::Two {
                first: Coord::new(-1, 0),
                second: Coord::new(4, 0),
            },
            MinimalCover::Two {
                first: Coord::new(-1, 0),
                second: Coord::new(5, 0),
            },
        ])
    );
    let cells = covers.cells();
    assert!(cells.contains(&Coord::new(-2, 0)) && cells.contains(&Coord::new(5, 0)));
    assert!(
        !matches!(&covers, Cover::Minimal(list) if list.contains(&MinimalCover::Two {
            first: Coord::new(-2, 0),
            second: Coord::new(5, 0),
        })),
        "both cells are in the union and together they cover nothing in the middle"
    );
}

#[test]
fn distant_cluster_threats_detected_identically_to_local() {
    // The same shape 42 columns away, behind a window-disjoint bridge: the
    // answers are the local ones translated, cell for cell.
    let shift = Coord::new(42, 0);
    let (_, local) = play(&threat_case("single_hot_window").plies);
    let (_, distant) = play(&threat_case("distant_cluster_threat").plies);
    let translated: Vec<String> = local
        .hot_windows(Player::P1)
        .iter()
        .map(|window| {
            common::window_token(pistol_core::window::Window {
                axis: window.axis,
                start: Coord::new(window.start.q + shift.q, window.start.r + shift.r),
            })
        })
        .collect();
    assert_eq!(
        window_list(distant.hot_windows(Player::P1)),
        translated.join(" ")
    );
    let (mut here, mut there) = (Vec::new(), Vec::new());
    local.threat_cells(Player::P1, &mut here);
    distant.threat_cells(Player::P1, &mut there);
    assert_eq!(
        there,
        here.iter()
            .map(|cell| Coord::new(cell.q + shift.q, cell.r + shift.r))
            .collect::<Vec<_>>()
    );
}

#[test]
fn overline_completion_counts_as_win_window() {
    let (game, threats) = play(&threat_case("overline_seven_run").plies);
    assert_eq!(
        game.outcome(),
        Outcome::Win {
            winner: Player::P1,
            turn: 7
        }
    );
    assert_eq!(
        window_list(threats.completed_windows(Player::P1)),
        "ConstR@0,0 ConstR@1,0",
        "a run of seven completes two windows"
    );
    // A completed window has no empty cell, so it cannot be hit at any budget.
    for budget in [HitBudget::Zero, HitBudget::One, HitBudget::Two] {
        assert_eq!(
            threats.blocking_covers(Player::P2, budget),
            Cover::Impossible,
            "nothing blocks a line that is already six"
        );
    }
}

#[test]
fn threat_state_answers_on_a_decided_position() {
    // A search legitimately stands on a decided position, so the queries answer
    // rather than panic. They answer about a game the rules have already ended,
    // which is why the CALLER checks the outcome.
    let (game, threats) = play(&threat_case("overline_seven_run").plies);
    assert!(game.outcome().is_decided());
    assert_eq!(
        StonesLeft::from_state(&game),
        None,
        "a decided position owes no stones, and there is no honest StonesLeft for it"
    );
    assert!(!threats.completed_windows(Player::P1).is_empty());
    assert_eq!(
        threats.can_win_this_turn(Player::P1, StonesLeft::One),
        Some(WinWitness::OnePly {
            at: Coord::new(-1, 0),
            window: common::parse_window("ConstR@-1,0", 0),
        }),
        "asked about a turn that will never be played, it answers anyway"
    );
}

/// D-243 consequence (3)'s COMPOSITION, written here because it is the caller's
/// and not the primitive's.
///
/// Both conditions, in the order they bite: `side` is NOT the one to move, and
/// the side that is cannot win this turn. Then, and only then, an unblockable
/// double threat is a win.
fn composed_win(game: &GameState, threats: &ThreatState, side: Player) -> bool {
    if game.to_move() == side {
        return false;
    }
    let Some(left) = StonesLeft::from_state(game) else {
        return false;
    };
    if threats.can_win_this_turn(side.opponent(), left).is_some() {
        return false;
    }
    threats.unblockable_double_threat(side, HitBudget::from(left))
}

/// Silence the unused-import warning in binaries that do not read every helper.
#[allow(dead_code)]
fn _uses(case: &ThreatCase) -> usize {
    case.plies.len()
}
