//! The verifier's own readings of the position: plan families, threat
//! moves, blocking pairs, the defender's race check, and EP-1 — all from
//! the board, all independent of `ThreatState` and of the solver's policy
//! module (design §7b).
//!
//! `r3` holds the reference VALUES; this module holds the pieces the
//! verifier and the gate-(c) walker share. They live apart from `r3`
//! because the value reference and the tree verifier are separate
//! instruments that happen to need the same board readings, and merging
//! them would make the verifier's re-derivation a restatement of the
//! reference's internals instead of an independent one.

use std::collections::BTreeSet;

use pistol_core::window::{Window, windows_through};
use pistol_core::{Board, Coord, GameState, Player, Turn};

use pistol_solver::ZoneP;

/// Whether the defender completes six this turn — the race check, computed
/// from the board by the five-own/four-own window scan.
pub fn defender_wins_now(state: &GameState, attacker: Player) -> bool {
    let defender = attacker.opponent();
    let board = state.board();
    let mut windows = hot_windows(board, defender);
    windows.sort_unstable();
    for window in &windows {
        let own = own_count(window, board, defender);
        if own >= 5 {
            return true;
        }
    }
    // A four-own window completes with the turn's two stones.
    windows
        .iter()
        .any(|window| own_count(window, board, defender) == 4)
}

/// The attacker's policy moves, from the board: `C`-pairs that create a
/// hot window. Mirrors `policy::threat_pairs` with none of its machinery.
pub fn threat_moves(state: &GameState, attacker: Player) -> Vec<Turn> {
    let board = state.board();
    let mut candidates = candidate_cells(board, attacker);
    candidates.sort_unstable();
    candidates.dedup();
    let mut moves = Vec::new();
    for (index, &first) in candidates.iter().enumerate() {
        for &second in &candidates[index + 1..] {
            let turn = Turn::pair(first, second).expect("candidates are distinct");
            let mut child = state.clone();
            if child.make_turn(turn).is_err() {
                continue;
            }
            if !hot_windows(child.board(), attacker).is_empty() {
                moves.push(turn);
            }
        }
    }
    moves
}

/// The blocking pairs, from the board: every legal turn whose cells hit
/// every plan family. Mirrors `policy::blocking_pairs` with none of its
/// machinery.
pub fn blocking_pairs(state: &GameState, attacker: Player) -> Vec<Turn> {
    let families = plan_families(state.board(), attacker);
    let mut out = Vec::new();
    for turn in pistol_core::generate_turns(state).expect("an AND node is a legal position") {
        let cells = turn_cells(&turn);
        let covers = families.iter().all(|family| {
            family
                .iter()
                .any(|at| cells.contains(at) && !state.board().is_occupied(*at))
        });
        if covers {
            out.push(turn);
        }
    }
    out
}

/// The attacker's hot-window empties.
pub fn threat_cells(state: &GameState, attacker: Player, out: &mut Vec<Coord>) {
    out.clear();
    for family in plan_families(state.board(), attacker) {
        out.extend(family);
    }
    out.sort_unstable();
    out.dedup();
}

/// EP-1, from the board: the graded empties of every defender active
/// segment intersecting the legal region. Mirrors `zone::ep1_contribution`
/// with none of its machinery.
pub fn ep1(board: &Board, attacker: Player) -> ZoneP {
    let mut zone = ZoneP::new();
    let mut seen: BTreeSet<Window> = BTreeSet::new();
    let mut scan = |at: Coord, zone: &mut ZoneP| {
        for window in windows_through(at) {
            if !seen.insert(window) {
                continue;
            }
            let mut empties = Vec::new();
            let mut defender = 0u32;
            for index in 0..6u8 {
                match board.get(window.cell(index)) {
                    None => empties.push(window.cell(index)),
                    Some(player) if player == attacker => {
                        empties.clear();
                        break;
                    }
                    Some(_) => defender += 1,
                }
            }
            if !empties.is_empty() {
                zone.add_graded(empties, 6 - defender);
            }
        }
    };
    for (at, _) in board.stones() {
        scan(at, &mut zone);
    }
    for at in pistol_core::legal_placements(board) {
        scan(at, &mut zone);
    }
    zone
}

/// The plan families: the empty-cell sets of the attacker's hot windows,
/// least by `(axis, start)` first (the BTreeSet's own order).
pub fn plan_families(board: &Board, attacker: Player) -> Vec<Vec<Coord>> {
    let mut windows: BTreeSet<Window> = BTreeSet::new();
    for (at, _) in board.stones() {
        for window in windows_through(at) {
            if is_hot(window, board, attacker) {
                windows.insert(window);
            }
        }
    }
    windows
        .into_iter()
        .map(|window| {
            window
                .cells()
                .into_iter()
                .filter(|cell| !board.is_occupied(*cell))
                .collect()
        })
        .collect()
}

fn is_hot(window: Window, board: &Board, attacker: Player) -> bool {
    let mut own = 0u32;
    for index in 0..6u8 {
        match board.get(window.cell(index)) {
            Some(player) if player == attacker => own += 1,
            Some(_) => return false,
            None => {}
        }
    }
    own >= 4
}

fn hot_windows(board: &Board, side: Player) -> Vec<Window> {
    let mut windows: BTreeSet<Window> = BTreeSet::new();
    for (at, _) in board.stones() {
        for window in windows_through(at) {
            if is_hot(window, board, side) {
                windows.insert(window);
            }
        }
    }
    windows.into_iter().collect()
}

fn own_count(window: &Window, board: &Board, side: Player) -> u32 {
    window
        .cells()
        .into_iter()
        .filter(|at| board.get(*at) == Some(side))
        .count() as u32
}

/// The attacker's candidate cells `C`, from the board.
fn candidate_cells(board: &Board, side: Player) -> Vec<Coord> {
    let mut cells = Vec::new();
    for (at, _) in board.stones() {
        for window in windows_through(at) {
            let mut own = 0u32;
            let mut live = true;
            for index in 0..6u8 {
                match board.get(window.cell(index)) {
                    Some(player) if player == side => own += 1,
                    Some(_) => {
                        live = false;
                        break;
                    }
                    None => {}
                }
            }
            if live && own >= 2 {
                cells.extend(
                    window
                        .cells()
                        .into_iter()
                        .filter(|at| !board.is_occupied(*at)),
                );
            }
        }
    }
    cells
}

/// Apply a turn to a state (the test tree's own wrapper, so the verifier
/// never touches the solver's crate-private apply).
pub fn r3_apply(state: &mut GameState, turn: &Turn) {
    state.make_turn(*turn).expect("a verified turn is legal");
}

/// Take a turn back.
pub fn r3_undo(state: &mut GameState, turn: &Turn) {
    state
        .unmake_turn()
        .expect("the turn being taken back is the last one");
    let _ = turn;
}

pub fn turn_cells(turn: &Turn) -> [Coord; 2] {
    match turn {
        Turn::Single(at) => [*at, *at],
        Turn::Pair(first, second) => [*first, *second],
    }
}
