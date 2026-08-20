//! The oracle: the incremental state against a from-scratch reference, at every
//! ply of seeded playouts, plus the invariants no fixture can express.
//!
//! # The regime is MECHANIZED, not cited
//!
//! Seeds `1..=PLAYOUTS` with `Rng::new(seed)`, cap [`PLIES`], plies coloured by
//! `GameState::place` and never by ply parity — and **one `random_ply` draw per
//! ply, and that draw is the ply**. The schedule is part of the registration
//! because it is not free: drawing twice and playing the first advances the
//! generator between plies, changes every trajectory, and drops the unblockable
//! count far below the floor below, so an implementer copying that loop shape
//! would write a test that fails on a correct implementation (docs/decisions.md
//! D-256).
//!
//! The floors are asserted rather than trusted: a shrunken playout is a shrunken
//! oracle, not a pass. They are floors and not targets — the regime produces
//! comfortably more than each — so a change that quietly narrows the coverage
//! fails here instead of passing thinly.
//!
//! # RULE9-JUSTIFICATION: one playout regime, one suite (CLAUDE.md rule 9).
//!
//! Every test here drives the same registered regime — same seeds, same cap,
//! same one-draw-per-ply schedule — and the whole-surface comparison is a single
//! claim that cannot be cut in half without comparing half a state. Splitting
//! the file would duplicate the loop per part and let the parts drift apart on
//! the one thing that must not drift, which is the schedule. The desync pins sit
//! here rather than beside the fixtures because they are about the apply/undo
//! contract this suite exercises and no fixture can express.

mod common;

use std::collections::BTreeMap;

use common::playouts::{Rng, random_ply};
use common::reference::Reference;
use common::{cell_list, window_list};
use pistol_core::window::Window;
use pistol_core::{Coord, GameState, Player};
use pistol_solver::{
    Cover, HitBudget, LiveCount, MinimalCover, NearHot, StonesLeft, THREAT_DESYNC, ThreatState,
};

/// Playouts, seeded `1..=PLAYOUTS`.
const PLAYOUTS: u64 = 12;
/// The stone cap on one playout.
const PLIES: usize = 150;

/// Plies the oracle must see. The crate's own floor, in the style of the eval's.
const REQUIRED_STEPS: usize = 1000;
/// `(ply, side)` pairs where that side holds a hot window.
const REQUIRED_HOT_POSITIONS: usize = 400;
/// `(ply, side)` pairs where that side's hot windows admit no two-cell cover.
const REQUIRED_UNBLOCKABLE: usize = 100;
/// `(ply, side)` pairs whose minimal covers include a CROSS-WINDOW pair — the
/// case a flat cell list cannot carry, and the one worth counting separately.
const REQUIRED_CROSS_WINDOW: usize = 100;
/// CELLS on which the legality premise is checked — not `(ply, side)` pairs.
///
/// Its own floor, because it counts its own thing: the legality test walks every
/// empty of every hot window and so sees several cells per hot side-position.
/// It borrowed [`REQUIRED_HOT_POSITIONS`] before, which is a floor on a
/// different unit and sits about a fifth of the way to what this one measures —
/// so the check could have shrunk by four fifths and still passed. The regime
/// produces 3330; this is about half, the ratio the four floors above already
/// sit at (docs/decisions.md D-256).
const REQUIRED_HOT_CELLS: usize = 1600;

/// The census one playout regime produces.
#[derive(Debug, Default, Clone, Copy)]
struct Census {
    steps: usize,
    hot: usize,
    unblockable: usize,
    cross_window: usize,
}

#[test]
fn threat_incremental_matches_reference_on_random_playouts() {
    let mut census = Census::default();
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut threats = ThreatState::new();
        while game.board().stone_count() < PLIES && !game.outcome().is_decided() {
            // ONE draw, and it is the ply.
            let next = random_ply(game.board(), &mut rng);
            let mover = game.to_move();
            game.place(next).expect("a sampled legal cell");
            threats.apply(next, mover);
            census.steps += 1;
            compare(
                &game,
                &threats,
                &format!("seed {seed} ply {}", census.steps),
                &mut census,
            );
        }
    }
    assert!(
        census.steps >= REQUIRED_STEPS,
        "the oracle must see at least {REQUIRED_STEPS} playout steps, saw {} — a shrunken \
         playout is a shrunken oracle, not a pass",
        census.steps
    );
    assert!(
        census.hot >= REQUIRED_HOT_POSITIONS,
        "at least {REQUIRED_HOT_POSITIONS} hot side-positions, saw {}",
        census.hot
    );
    assert!(
        census.unblockable >= REQUIRED_UNBLOCKABLE,
        "at least {REQUIRED_UNBLOCKABLE} unblockable side-positions, saw {}",
        census.unblockable
    );
    assert!(
        census.cross_window >= REQUIRED_CROSS_WINDOW,
        "at least {REQUIRED_CROSS_WINDOW} cross-window cover instances, saw {}",
        census.cross_window
    );
    eprintln!(
        "oracle census: {} plies, {} hot side-positions, {} unblockable, {} cross-window",
        census.steps, census.hot, census.unblockable, census.cross_window
    );
}

/// Everything, for both sides, at every stones-left and every budget.
fn compare(game: &GameState, threats: &ThreatState, at: &str, census: &mut Census) {
    let reference = Reference::from_board(game.board());

    // The full table, as masks, sorted.
    let carried: BTreeMap<Window, (u8, u8)> = threats
        .table_snapshot()
        .into_iter()
        .map(|(window, masks)| (window, (masks.p1, masks.p2)))
        .collect();
    let fresh: BTreeMap<Window, (u8, u8)> = reference
        .table()
        .iter()
        .map(|(&window, held)| (window, (held.p1, held.p2)))
        .collect();
    assert_eq!(carried, fresh, "{at}: the window table");

    let mut cells = Vec::new();
    for side in [Player::P1, Player::P2] {
        // All ten maintained sets.
        assert_eq!(
            window_list(threats.hot_windows(side)),
            window_list(&reference.hot(side)),
            "{at} {side}: hot"
        );
        assert_eq!(
            window_list(threats.win_in_one_ply_windows(side)),
            window_list(&reference.win_in_one_ply(side)),
            "{at} {side}: win1"
        );
        assert_eq!(
            window_list(threats.completed_windows(side)),
            window_list(&reference.completed(side)),
            "{at} {side}: completed"
        );
        for count in [LiveCount::Two, LiveCount::Three] {
            assert_eq!(
                window_list(threats.live_windows_at_count(side, count)),
                window_list(&reference.live_at(side, count)),
                "{at} {side}: live at {count:?}"
            );
        }
        // The cell answers.
        threats.threat_cells(side, &mut cells);
        assert_eq!(
            cell_list(&cells),
            cell_list(&reference.threat_cells(side)),
            "{at} {side}: threat cells"
        );
        threats.win_in_one_ply_cells(side, &mut cells);
        assert_eq!(
            cell_list(&cells),
            cell_list(&reference.win_in_one_ply_cells(side)),
            "{at} {side}: win-in-one-ply cells"
        );
        threats.cells_raising_to_hot(side, NearHot::Three, &mut cells);
        assert_eq!(
            cell_list(&cells),
            cell_list(&reference.cells_raising_to_hot(side, NearHot::Three)),
            "{at} {side}: cells raising to hot"
        );
        // The hitting-set predicate at every budget, the covers as SETS of
        // covers rather than as a cell union, and the guard at both budgets.
        for budget in [HitBudget::Zero, HitBudget::One, HitBudget::Two] {
            assert_eq!(
                threats.min_hitting_set_exceeds(budget, threats.hot_windows(side)),
                reference.min_hitting_set_exceeds(budget, &reference.hot(side)),
                "{at} {side}: min_hitting_set_exceeds at {budget:?}"
            );
            assert_eq!(
                threats.blocking_covers(side, budget),
                reference.blocking_covers(side, budget),
                "{at} {side}: blocking covers at {budget:?}"
            );
            assert_eq!(
                threats.unblockable_double_threat(side, budget),
                reference.unblockable_double_threat(side, budget),
                "{at} {side}: unblockable at {budget:?}"
            );
        }
        // The witness, INCLUDING the window it names.
        for left in [StonesLeft::One, StonesLeft::Two] {
            assert_eq!(
                threats.can_win_this_turn(side, left),
                reference.can_win_this_turn(side, left),
                "{at} {side}: can_win_this_turn at {left:?}"
            );
        }

        if !threats.hot_windows(side).is_empty() {
            census.hot += 1;
        }
        if threats.unblockable_double_threat(side, HitBudget::Two) {
            census.unblockable += 1;
        }
        if let Cover::Minimal(covers) = threats.blocking_covers(side.opponent(), HitBudget::Two)
            && covers
                .iter()
                .any(|cover| matches!(cover, MinimalCover::Two { .. }))
        {
            census.cross_window += 1;
        }
    }
}

#[test]
fn threat_apply_undo_roundtrips() {
    // Whole-state equality, not agreement in every answer: undoing to an earlier
    // ply must leave the state INDISTINGUISHABLE from the one recorded there.
    // What makes that true is the pruning rule — an undone stone leaves no
    // (0,0) entry behind — and without it the table would grow with the search
    // PATH rather than with the position.
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut threats = ThreatState::new();
        let mut recorded: Vec<(Coord, Player, ThreatState)> = Vec::new();
        while game.board().stone_count() < PLIES && !game.outcome().is_decided() {
            let next = random_ply(game.board(), &mut rng);
            let mover = game.to_move();
            recorded.push((next, mover, threats.clone()));
            game.place(next).expect("a sampled legal cell");
            threats.apply(next, mover);
        }
        while let Some((at, mover, before)) = recorded.pop() {
            threats.undo(at, mover);
            assert_eq!(
                threats, before,
                "seed {seed}: undoing {mover} at {at} did not restore the state",
            );
        }
        assert_eq!(threats, ThreatState::new(), "seed {seed}: fully unwound");
        assert!(
            threats.is_empty(),
            "seed {seed}: no entry survives the unwind"
        );
    }
}

#[test]
fn window_map_ordering_is_unobservable() {
    // The table may be hashed BECAUSE nothing observes its order. Two states
    // built from the same stones in different insertion orders must answer
    // identically — window for window and cell for cell.
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut stones: Vec<(Coord, Player)> = Vec::new();
        while game.board().stone_count() < PLIES && !game.outcome().is_decided() {
            let next = random_ply(game.board(), &mut rng);
            let mover = game.to_move();
            game.place(next).expect("a sampled legal cell");
            stones.push((next, mover));
        }
        let mut forwards = ThreatState::new();
        for &(at, player) in &stones {
            forwards.apply(at, player);
        }
        let mut backwards = ThreatState::new();
        for &(at, player) in stones.iter().rev() {
            backwards.apply(at, player);
        }
        assert_eq!(
            forwards, backwards,
            "seed {seed}: insertion order changed the state itself"
        );
        let mut here = Vec::new();
        let mut there = Vec::new();
        for side in [Player::P1, Player::P2] {
            assert_eq!(
                window_list(forwards.hot_windows(side)),
                window_list(backwards.hot_windows(side))
            );
            forwards.threat_cells(side, &mut here);
            backwards.threat_cells(side, &mut there);
            assert_eq!(here, there, "seed {seed} {side}: threat cells");
            for budget in [HitBudget::Zero, HitBudget::One, HitBudget::Two] {
                assert_eq!(
                    forwards.blocking_covers(side, budget),
                    backwards.blocking_covers(side, budget)
                );
            }
        }
    }
}

#[test]
fn hot_window_empties_are_always_legal_placements() {
    // D-243's legality premise, which is what lets every query skip a legality
    // check: a hot window holds at least four of six own stones, so its empties
    // are within two steps of a stone already on the board and sit well inside
    // rule 5's radius-8 region. A premise a design DEPENDS on is pinned by the
    // design, not by the ADR that states it.
    let mut cells = Vec::new();
    let mut checked = 0usize;
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut threats = ThreatState::new();
        while game.board().stone_count() < PLIES && !game.outcome().is_decided() {
            let next = random_ply(game.board(), &mut rng);
            let mover = game.to_move();
            game.place(next).expect("a sampled legal cell");
            threats.apply(next, mover);
            for side in [Player::P1, Player::P2] {
                threats.threat_cells(side, &mut cells);
                for &cell in &cells {
                    assert!(
                        game.board().is_legal_placement(cell),
                        "seed {seed}: the hot-window empty {cell} is not a legal placement"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(
        checked >= REQUIRED_HOT_CELLS,
        "the premise must be checked on at least {REQUIRED_HOT_CELLS} hot-window empty CELLS, \
         saw {checked}"
    );
}

#[test]
#[should_panic(expected = "THREAT_DESYNC")]
fn applying_a_stone_twice_is_a_desync() {
    // Rule 3: being told about a stone that contradicts what this state holds
    // means a caller's board and this state have drifted apart.
    assert_eq!(THREAT_DESYNC, "THREAT_DESYNC");
    let mut threats = ThreatState::new();
    threats.apply(Coord::ORIGIN, Player::P1);
    threats.apply(Coord::ORIGIN, Player::P2);
}

#[test]
#[should_panic(expected = "THREAT_DESYNC")]
fn taking_back_a_stone_that_was_never_applied_is_a_desync() {
    let mut threats = ThreatState::new();
    threats.apply(Coord::ORIGIN, Player::P1);
    threats.undo(Coord::new(1, 0), Player::P1);
}

#[test]
#[should_panic(expected = "THREAT_DESYNC")]
fn taking_back_the_wrong_player_is_a_desync() {
    let mut threats = ThreatState::new();
    threats.apply(Coord::ORIGIN, Player::P1);
    threats.undo(Coord::ORIGIN, Player::P2);
}
