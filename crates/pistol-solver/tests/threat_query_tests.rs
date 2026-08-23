//! The threat state's answers, pinned against sixteen golden positions.
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

use common::fixtures::{
    SideExpectation, StateExpectation, THREAT_FIXTURE_FILE, ThreatCase, render_case, threat_case,
    threat_cases,
};
use common::reference::Reference;
use common::region::region_scan;
use common::sha256::sha256_hex;
use common::{assert_pinned, cell_list, fixture_text, play, window_list};
use pistol_core::{Coord, GameState, Outcome, Player};
use pistol_solver::{
    Cover, HitBudget, LiveCount, MinimalCover, NearHot, StonesLeft, ThreatState, WinWitness,
};

/// The SHA-256 of `tests/fixtures/threat_v0.txt`.
///
/// Changing the fixture means changing this line, in the same commit, having
/// looked at what changed.
const THREAT_V0_SHA256: &str = "541609a6815b2d7024b02976c1155f859c0513b85cb23026f45b7791658fae5d";

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

/// The fixture is DERIVED, and this is the derivation.
///
/// Every record is rendered from the from-scratch reference over the committed
/// ply lists and compared with the pinned bytes, line for line. Three things
/// follow, and the third is the one that was missing:
///
/// 1. the file is reproducible from the repository — no scratch generator, no
///    author's transcript, nothing uncommitted stands behind it;
/// 2. R1's own answers are compared against the file, which nothing else did:
///    the golden test compares the SHIPPED state against the file, so without
///    this row "computed by a from-scratch reference" was an authoring claim no
///    gate enforced;
/// 3. the sha pin becomes a live derivation rather than a checksum. An edit to
///    an expectation must be justified against R1; re-hashing it is no longer
///    enough to make the suite green, which is exactly the door a reviewer
///    walked through to show that a fixture's PRECONDITION could be edited away
///    without any expected answer changing.
///
/// Comments and blank lines are the file's own prose and are not derived; every
/// other line is (docs/decisions.md D-259).
#[test]
fn threat_v0_is_what_the_reference_prints() {
    let mut rendered = String::new();
    for case in threat_cases() {
        let (game, _) = play(&case.plies);
        let reference = Reference::from_board(game.board());
        let derived = ThreatCase {
            name: case.name.clone(),
            line: case.line,
            plies: case.plies.clone(),
            sides: [Player::P1, Player::P2].map(|side| SideExpectation {
                hot: reference.hot(side),
                win1: reference.win_in_one_ply(side),
                completed: reference.completed(side),
                live3: reference.live_at(side, LiveCount::Three),
                live2: reference.live_at(side, LiveCount::Two),
                threat_cells: reference.threat_cells(side),
                raise_cells: reference.cells_raising_to_hot(side, NearHot::Three),
                cover: [HitBudget::One, HitBudget::Two]
                    .map(|budget| reference.blocking_covers(side.opponent(), budget)),
                canwin: [StonesLeft::One, StonesLeft::Two]
                    .map(|left| reference.can_win_this_turn(side, left)),
            }),
            state: StateExpectation {
                to_move: game.to_move(),
                phase: game.phase(),
                stones_owed: game.stones_owed(),
                outcome: game.outcome(),
            },
        };
        rendered.push_str(&render_case(&derived));
    }
    let text = fixture_text(THREAT_FIXTURE_FILE);
    let on_disk: Vec<&str> = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .collect();
    let derived: Vec<&str> = rendered.lines().collect();
    for (index, (derived_line, disk_line)) in derived.iter().zip(&on_disk).enumerate() {
        assert_eq!(
            derived_line,
            disk_line,
            "record line {}: the reference prints one thing and {THREAT_FIXTURE_FILE} says \
             another; the file is derived, so change the reference's mind or the plies, not \
             the expectation",
            index + 1
        );
    }
    assert_eq!(
        derived.len(),
        on_disk.len(),
        "{THREAT_FIXTURE_FILE} has {} derivable lines and the reference printed {}",
        on_disk.len(),
        derived.len()
    );
}

/// THE CLASS THE DERIVATION ABOVE CANNOT SEE, ENUMERATED AND CLOSED.
///
/// `threat_v0_is_what_the_reference_prints` catches an edited `plies` line
/// whenever the edit MOVES a derived answer — a `plies` line is an input, so it
/// re-derives, and the comparison objects to any answer that changed. What it
/// cannot catch is an ANSWER-INVARIANT edit, and answer-invariance is TOTAL on
/// a record with no positive expectation: every row reads `-`, `nothing` or
/// `none` whatever the position turns out to be, so the precondition that makes
/// the record the right fixture can be edited away and nothing anywhere goes
/// red. That is not hypothetical on either member — the drift was reproduced on
/// both before this row existed (docs/decisions.md D-260, D-264).
///
/// So the class is closed by admission requirement rather than by name: a
/// record whose every row is negative names below the window that makes it the
/// right fixture and what that window must hold. A THIRD such record fails the
/// census until its author writes one down.
#[test]
fn a_record_stating_nothing_states_its_own_shape() {
    // case, window token, own count, opp count, empties — hand-written, because
    // a shape read out of the state is a shape the state cannot contradict.
    const SHAPES: [(&str, &str, u32, u32, u32); 3] = [
        (
            "single_hot_window_dead_when_opponent_enters",
            "ConstR@0,0",
            4,
            1,
            1,
        ),
        (
            "win_in_one_ply_dead_when_opponent_enters",
            "ConstR@-1,0",
            5,
            1,
            0,
        ),
        (
            "win_in_one_ply_dead_when_opponent_enters",
            "ConstR@0,0",
            5,
            1,
            0,
        ),
    ];

    let mut census: Vec<String> = threat_cases()
        .into_iter()
        .filter(|case| {
            [Player::P1, Player::P2]
                .into_iter()
                .all(|side| case.side(side).states_nothing())
        })
        .map(|case| case.name)
        .collect();
    census.sort();
    let mut named: Vec<String> = SHAPES
        .iter()
        .map(|(name, ..)| String::from(*name))
        .collect();
    named.sort();
    named.dedup();
    assert_eq!(
        census, named,
        "the all-negative class moved: a record every one of whose rows reads negative is \
         invisible to the derivation, so it owes a shape in SHAPES and a row that asserts it"
    );

    for (name, token, own, opp, empties) in SHAPES {
        let (_, threats) = play(&threat_case(name).plies);
        let masks = threats.masks(common::parse_window(token, 0));
        assert_eq!(
            (
                masks.own_count(Player::P1),
                masks.opp_count(Player::P1),
                masks.empties().count_ones()
            ),
            (own, opp, empties),
            "{name} / {token}: the shape this record's rows are about"
        );
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
    // THE PRECONDITION, STATED WHERE THIS ROW CAN SEE IT — the same treatment
    // D-260 gave the twin, and for the same reason. The assertion above is that
    // a set is EMPTY, which holds just as well of a position exhibiting
    // nothing: drop P1 to THREE in that window and it is non-hot on the COUNT,
    // the row proves nothing about liveness, and no expectation in the record
    // moves, so the derivation cannot object either. So the row states what the
    // position must be: the window that is hot in `single_hot_window` holds THE
    // SAME four P1 stones here, plus the opponent stone that kills it
    // (docs/decisions.md D-264).
    let window = common::parse_window("ConstR@0,0", 0);
    let (live_masks, dead_masks) = (live.masks(window), dead.masks(window));
    assert_eq!(
        dead_masks.own_count(Player::P1),
        4,
        "ConstR@0,0 must hold four P1 stones for this case to be about liveness"
    );
    assert_eq!(
        dead_masks.own(Player::P1),
        live_masks.own(Player::P1),
        "and they must be the SAME four, or the pair compares two positions"
    );
    assert_eq!(
        dead_masks.opp_count(Player::P1),
        1,
        "ConstR@0,0 must hold the opponent stone that kills it"
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
    // THE PRECONDITION, STATED WHERE THIS ROW CAN SEE IT. Everything above is an
    // assertion that a set is EMPTY, so it holds just as well of a position that
    // exhibits nothing — and the control below guards a different fixture. A
    // reviewer edited this case's ninth ply so P1 held four rather than five,
    // re-pinned the sha, and the whole suite stayed green: every expected answer
    // read `-` either way and the row silently reverted to the inert state it
    // was written to escape. So the row now states what the position must be:
    // two windows, five own stones each, the opponent in the sixth cell, hence
    // FULL and offering no empty cell either way (docs/decisions.md D-260).
    for token in ["ConstR@-1,0", "ConstR@0,0"] {
        let window = common::parse_window(token, 0);
        let masks = dead.masks(window);
        assert_eq!(
            masks.own_count(Player::P1),
            5,
            "{token} must hold five P1 stones for this case to be about liveness"
        );
        assert_eq!(
            masks.opp_count(Player::P1),
            1,
            "{token} must hold the opponent stone that kills it"
        );
        assert_eq!(masks.empties(), 0, "{token}: a dead five-window is FULL");
    }
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
        15,
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

/// The SIZE-ONE CASE of the two-cell cover enumeration, which nothing else here
/// reaches.
///
/// `min_hitting_set_exceeds` at `HitBudget::Two` walks pairs over
/// `universe[index..]` — from `index`, not `index + 1` — because `second ==
/// first` IS the one-cell cover. Where the universe holds a SINGLE cell,
/// `universe[index + 1..]` is empty, the scan finds no pair at all, and the
/// predicate answers "no cover within two stones" for a threat that one stone
/// answers: a mate score for the wrong side, reached by an off-by-one in an
/// index range.
///
/// That mutant survived the entire suite before this position existed, and not
/// by luck: over the registered playout regime's 1703 plies and 805 hot
/// side-positions, the number whose hot-window empties union to ONE cell is
/// zero, so no floor and no longer playout could have closed it. A position
/// where the hot set is a single five-window with both four-window extensions
/// already dead is the configuration the regime does not produce
/// (docs/decisions.md D-260).
#[test]
fn a_one_cell_hot_universe_is_covered_within_every_positive_budget() {
    let (_, threats) = play(&threat_case("hot_universe_of_a_single_cell").plies);
    let hot = threats.hot_windows(Player::P1);
    // The precondition first: ONE hot window, and its empties are ONE cell, so
    // the universe the enumeration walks has a single member.
    assert_eq!(window_list(hot), "ConstR@-1,0");
    let mut cells = Vec::new();
    threats.threat_cells(Player::P1, &mut cells);
    assert_eq!(
        cell_list(&cells),
        "-1,0",
        "the hot universe must be a single cell for this row to be about anything"
    );
    assert!(
        !threats.min_hitting_set_exceeds(HitBudget::One, hot),
        "one stone hits the only window there is"
    );
    assert!(
        !threats.min_hitting_set_exceeds(HitBudget::Two, hot),
        "and so does one of two, which is the case an exclusive pair range drops"
    );
    assert!(
        !threats.unblockable_double_threat(Player::P1, HitBudget::Two),
        "a threat one stone answers is not an unblockable double threat"
    );
    // Both directions, so the row is not merely asserting `false` everywhere:
    // zero stones still cover nothing, and the cover itself is that one cell.
    assert!(threats.min_hitting_set_exceeds(HitBudget::Zero, hot));
    assert_eq!(
        threats.blocking_covers(Player::P2, HitBudget::Two),
        Cover::Minimal(vec![MinimalCover::One(Coord::new(-1, 0))])
    );
}

/// The SECOND HALF of the witness tie-break: the window, when the cell does not
/// decide it.
///
/// `can_win_this_turn` returns a witness that carries a WINDOW, so where one
/// cell is the single empty of two five-windows, two conforming implementations
/// agree on the cell and differ on a field the oracle compares. The rule is the
/// least window by `(axis, start)`. Across the fourteen positions this suite
/// had before, no cell was the single empty of two five-windows — both
/// reviewers checked independently — so a determinism-relevant tie-break was
/// pinned by the playout oracle alone, and a shrunken regime would have unpinned
/// it silently (docs/decisions.md D-260).
#[test]
fn the_witness_window_is_the_least_among_those_sharing_the_winning_cell() {
    let case = threat_case("two_win_in_one_ply_windows_share_their_cell");
    let (_, threats) = play(&case.plies);
    let win1 = threats.win_in_one_ply_windows(Player::P1);
    // The precondition: TWO live five-windows, and the same single empty cell in
    // both. Two P1 lines crossing at 3,0.
    assert_eq!(window_list(win1), "ConstQ@3,-3 ConstR@0,0");
    for &window in win1 {
        let masks = threats.masks(window);
        assert_eq!(masks.own_count(Player::P1), 5, "{window:?}");
        assert_eq!(masks.opp_count(Player::P1), 0, "{window:?} must be live");
        let empties: Vec<Coord> = (0..6u8)
            .filter(|index| masks.empties() & (1 << index) != 0)
            .map(|index| window.cell(index))
            .collect();
        assert_eq!(
            empties,
            vec![Coord::new(3, 0)],
            "{window:?} must be a single stone from six, at the shared cell"
        );
    }
    // So the cell cannot decide the witness, and the window rule does: ConstQ
    // precedes ConstR.
    for left in [StonesLeft::One, StonesLeft::Two] {
        assert_eq!(
            threats.can_win_this_turn(Player::P1, left),
            Some(WinWitness::OnePly {
                at: Coord::new(3, 0),
                window: common::parse_window("ConstQ@3,-3", 0),
            }),
            "at {left:?}: the least window by (axis, start) among those sharing the cell"
        );
    }
}

#[test]
fn live_cells_at_count_two_is_the_union_of_that_counts_windows_empties() {
    // `single_hot_window` carries exactly one P1 live-two window (`ConstR@2,0`),
    // so the query's answer is checked against an independent walk of that one
    // window's own empties rather than against another query.
    let (_, threats) = play(&threat_case("single_hot_window").plies);
    let mut cells = Vec::new();
    threats.live_cells_at_count(Player::P1, LiveCount::Two, &mut cells);
    let windows = threats.live_windows_at_count(Player::P1, LiveCount::Two);
    assert_eq!(
        windows.len(),
        1,
        "single_hot_window carries exactly one P1 live-two window"
    );
    let masks = threats.masks(windows[0]);
    let expected: Vec<Coord> = (0..6u8)
        .filter(|index| masks.empties() & (1 << index) != 0)
        .map(|index| windows[0].cell(index))
        .collect();
    assert_eq!(cells, expected);
    assert_eq!(cells.len(), 4, "a live count-two window has four empties");
}

#[test]
fn live_cells_at_count_three_agrees_with_cells_raising_to_hot_at_that_count() {
    // The recorded coincidence (docs/decisions.md D-267, D-352): both are the
    // deduplicated union of the live count-three windows' empties, under
    // different names for different questions — `live_cells_at_count` as Tier
    // T's own qualification, `cells_raising_to_hot` as what a single stone would
    // activate. There is no such coincidence at `LiveCount::Two`, which `NearHot`
    // cannot even express.
    let (_, threats) = play(&threat_case("count_three_live_both_sides").plies);
    for side in [Player::P1, Player::P2] {
        let mut by_count = Vec::new();
        threats.live_cells_at_count(side, LiveCount::Three, &mut by_count);
        let mut by_raise = Vec::new();
        threats.cells_raising_to_hot(side, NearHot::Three, &mut by_raise);
        assert_eq!(by_count, by_raise, "{side}: same union, two names");
        assert!(!by_count.is_empty(), "{side}: the fixture carries live-three windows");
    }
}

/// D-243 consequence (3)'s COMPOSITION, written here because it is the caller's
/// and not the primitive's.
///
/// The conditions, in the order they bite: the game is NOT ALREADY DECIDED,
/// `side` is NOT the one to move, and the side that is cannot win this turn.
/// Then, and only then, an unblockable double threat is a win.
///
/// THE OUTCOME CHECK IS PERFORMED AND NOT ASSUMED, and the reason is
/// transmission rather than correctness. `StonesLeft::from_state` answers
/// `None` on a decided position and would carry the class alone, which is what
/// `cover.rs`'s recipe itself calls "correct, and it is thin". But this
/// function is that recipe's ONLY worked example, and an exemplar that skips
/// the step it names teaches the thin version to whoever copies it — WP-1.5b's
/// author being the reader in question (docs/decisions.md D-257).
fn composed_win(game: &GameState, threats: &ThreatState, side: Player) -> bool {
    if game.outcome().is_decided() {
        return false;
    }
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
