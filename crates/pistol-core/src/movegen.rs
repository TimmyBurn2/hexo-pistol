use std::collections::BTreeSet;

use crate::board::{Board, Player};
use crate::coord::Coord;
use crate::error::CoreError;
use crate::rules::{LEGAL_RADIUS, TURN_STONES};
use crate::state::GameState;
use crate::turn::{Phase, Turn, canonical_pair};
use crate::win::wins_at;

/// Named invariant: the scratch board a hypothetical stone is tested on
/// disagreed with the position it was cloned from.
pub const SCRATCH_DESYNC: &str = "SCRATCH_DESYNC";

/// The rule radius as a coordinate offset. Checked at compile time rather than
/// asserted at run time: it is a constant, and a constant that does not fit is
/// a build error, not a diagnostic.
const RADIUS: i16 = {
    assert!(LEGAL_RADIUS <= i16::MAX as u32);
    LEGAL_RADIUS as i16
};

/// Every turn the mover may play from `state`.
///
/// A decided game has no turns — the game is over, and that is an answer, not a
/// refusal (rule 4). A position in the middle of a turn is
/// [`CoreError::TurnInProgress`]: it owes one stone, which is a ply, and
/// [`GameState::place`] is what places it.
pub fn generate_turns(state: &GameState) -> Result<Vec<Turn>, CoreError> {
    if state.outcome().is_decided() {
        return Ok(Vec::new());
    }
    if state.phase() != Phase::First {
        return Err(CoreError::TurnInProgress { turn: state.turn() });
    }

    let board = state.board();
    let region = region_cells(board);
    let empty: Vec<Coord> = region
        .iter()
        .copied()
        .filter(|&cell| !board.is_occupied(cell))
        .collect();

    // Turn 1 owes one stone (rule 3), and so does no other turn at its first
    // phase; a single stone there is the whole turn, winning or not.
    if state.stones_owed() < TURN_STONES {
        return Ok(empty.into_iter().map(Turn::Single).collect());
    }

    let offsets = ball_offsets();
    let wins = winning_cells(board, &empty, state.to_move());
    let mut turns = Vec::new();
    for (index, &first) in empty.iter().enumerate() {
        if wins[index] {
            // Rule 4: the turn ends on this stone, so it is a turn of one and
            // is not half of any pair.
            turns.push(Turn::Single(first));
            continue;
        }
        for (other, &second) in empty.iter().enumerate() {
            // A pair of two base-region cells is reachable from either end. It
            // is emitted from the smaller — unless the other end wins, whose
            // own enumeration above emits nothing for it.
            if other == index || (other < index && !wins[other]) {
                continue;
            }
            turns.push(canonical_pair(first, second));
        }
        for &delta in &offsets {
            // The cells the first stone opens up. A cell outside the base
            // region holds no stone (a stone's own cell is in the region), so
            // there is no occupancy test to make here.
            let Some(second) = first.checked_offset(delta) else {
                continue;
            };
            if region.binary_search(&second).is_err() {
                turns.push(canonical_pair(first, second));
            }
        }
    }
    Ok(turns)
}

/// Every cell a stone may be placed on right now: the legal region, less the
/// cells that already hold a stone (rule 5), ascending.
///
/// This is the enumerate-the-balls form of [`Board::is_legal_placement`], which
/// probes one cell at a time. The two answer the same question and a test pins
/// that they agree.
pub fn legal_placements(board: &Board) -> Vec<Coord> {
    region_cells(board)
        .into_iter()
        .filter(|&cell| !board.is_occupied(cell))
        .collect()
}

/// The legal region, ascending — occupied cells included, since they are what
/// the region is measured from and a pair's second cell has to be tested
/// against the whole of it.
fn region_cells(board: &Board) -> Vec<Coord> {
    // An empty board has no stone to measure a distance from: the first stone
    // goes on the origin (rule 3, docs/decisions.md D-40).
    if board.is_empty() {
        return vec![Coord::ORIGIN];
    }
    let offsets = ball_offsets();
    let mut cells = BTreeSet::new();
    for (stone, _) in board.stones() {
        for &delta in &offsets {
            // A ball cell off the addressable lattice is not a cell, so it is
            // not in the region either (docs/decisions.md D-47).
            if let Some(cell) = stone.checked_offset(delta) {
                cells.insert(cell);
            }
        }
    }
    cells.into_iter().collect()
}

/// The offsets of a radius-[`LEGAL_RADIUS`] ball, ascending, centre included.
fn ball_offsets() -> Vec<Coord> {
    let mut offsets = Vec::new();
    for dq in -RADIUS..=RADIUS {
        for dr in -RADIUS..=RADIUS {
            let delta = Coord::new(dq, dr);
            if Coord::ORIGIN.distance(delta) <= LEGAL_RADIUS {
                offsets.push(delta);
            }
        }
    }
    offsets
}

/// For each cell, whether a `mover` stone placed there would complete a line.
///
/// The question is about a stone that is not on the board yet, and win
/// detection reads the player off the board by design (docs/decisions.md D-36),
/// so the stone goes down on a scratch copy and comes straight back off.
///
/// # Panics
///
/// With [`SCRATCH_DESYNC`] if the scratch board refuses a cell the position
/// says is empty, or hands back a stone that was never put there.
fn winning_cells(board: &Board, cells: &[Coord], mover: Player) -> Vec<bool> {
    let mut scratch = board.clone();
    cells
        .iter()
        .map(|&at| {
            if let Err(error) = scratch.apply(at, mover) {
                panic!(
                    "pistol-core invariant {SCRATCH_DESYNC}: {at} is an empty cell of the legal \
                     region, and the scratch board says: {error}"
                );
            }
            let won = wins_at(&scratch, at);
            if let Err(error) = scratch.undo(at) {
                panic!(
                    "pistol-core invariant {SCRATCH_DESYNC}: the stone just placed on {at} is \
                     gone, and the scratch board says: {error}"
                );
            }
            won
        })
        .collect()
}
