//! Turn generation: every turn the mover may play, and nothing else.
//!
//! A turn is an unordered pair of distinct cells (rule 3), and a pair is legal
//! iff **some** ordering of its two placements is legal under rule 5
//! (docs/decisions.md D-6) — the second stone may be reachable only through the
//! ball the first stone opens. Rule 4 cuts across that: a first stone that
//! completes a line ends the turn, so such a cell is generated as a
//! [`Turn::Single`] and never as half of a pair.
//!
//! Both facts are structural here rather than filtered afterwards. Write the
//! base region `R` (the union of the radius-[`LEGAL_RADIUS`] balls around the
//! stones on the board, rule 5) and let `E` be its empty cells. A first stone
//! must come from `E`. Given a first stone `f` that does not win, the second
//! may be any empty cell of `R ∪ ball(f)` other than `f`. So the turns are:
//!
//! - `Single(f)` for every `f` in `E` that completes a line;
//! - `{f, s}` for every non-winning `f` in `E` and every `s` in
//!   `(E \ {f}) ∪ (ball(f) \ R)`.
//!
//! A pair whose two cells are both in `E` is reachable from either of them, so
//! it would be emitted twice; it is emitted from the smaller cell only, unless
//! the other cell wins, in which case the other cell's own enumeration emits
//! nothing (rule 4) and this one has to. A pair with one cell outside `R` is
//! reachable only through the cell inside it, and a pair with both cells
//! outside `R` is not a turn at all, since neither stone could go down first.
//!
//! That last case is the one a "generate around the stones, then check each
//! cell" implementation gets wrong, and the brute-force reference in the test
//! tree — which enumerates ordered placements and dedupes — is what pins it
//! (CLAUDE.md rule 7).
//!
//! # Order
//!
//! Generation is deterministic and duplicate-free (CLAUDE.md rule 4): the base
//! region is a `BTreeSet` drained in ascending `(q, r)` order and the ball
//! offsets are a fixed ascending array, so the same position yields the same
//! `Vec<Turn>` in the same order in every run. The order is *not* sorted by
//! [`Turn`] — canonicalizing a pair can put its smaller cell second in emission
//! order — and no caller may depend on it being anything but stable: move
//! **ordering** is the search's job and arrives with it (Stage 1).
//!
//! # Not a candidate policy
//!
//! Everything here is rule 5, the game rule. The search's candidate policy —
//! which narrows this set to the cells worth looking at — is configuration and
//! lives in the engine, and the two radii are never compared (CLAUDE.md rule 2,
//! docs/decisions.md D-20). Perft counts what this module generates, so it
//! counts the full legal region.

use std::collections::BTreeSet;

use crate::board::{Board, Color};
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
/// detection reads the colour off the board by design (docs/decisions.md D-36),
/// so the stone goes down on a scratch copy and comes straight back off.
///
/// # Panics
///
/// With [`SCRATCH_DESYNC`] if the scratch board refuses a cell the position
/// says is empty, or hands back a stone that was never put there.
fn winning_cells(board: &Board, cells: &[Coord], mover: Color) -> Vec<bool> {
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
