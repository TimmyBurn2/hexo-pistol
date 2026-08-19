//! The bounded answer a wall-clock search secures before it deepens.
//!
//! A deadline can land before the first iteration finishes — its cost grows
//! with the candidate count, which the opponent partly controls
//! (docs/decisions.md D-95) — so a search that must answer within the budget
//! needs a move whose cost is bounded and paid up front. This module computes
//! it: an instant-win check over the candidate set, then the first candidates
//! of the deterministic (ascending `(q, r)`) ordering, replayed through
//! `GameState` so legality is the rules' own answer and never an inference
//! (CLAUDE.md rule 2).
//!
//! # Bounded, and what the bound is
//!
//! Two candidate generations and two O(|candidates|) win scans, no evaluation
//! calls, no clock reads, no table probes. The cost grows linearly with the
//! candidate count and with nothing else; the measured numbers live in the
//! WP-1.4 decision line together with the epsilon they must fit under.
//!
//! # Pure
//!
//! A function of (position, candidate policy) only. Two calls on the same
//! position return the same turn, on any machine, at any wall-clock moment —
//! which is what lets a test pin the fallback answer even though the instant a
//! deadline interrupts a real search cannot be pinned.

use pistol_core::{Coord, GameState, PlyOutcome, Turn};

use crate::candidates::candidate_cells;
use crate::params::CandidatePolicy;

/// Named invariant: the fallback was asked for a move on a position where the
/// candidate policy offers none. The search refuses that root by name
/// (`SearchError::NoCandidates`) before the fallback runs, and a policy that
/// runs dry half way through a turn is the same impossibility
/// `NO_CANDIDATES_MID_TURN` names for the recursion.
pub const FALLBACK_NO_CANDIDATES: &str = "FALLBACK_NO_CANDIDATES";

/// Named invariant: the candidate policy offered a cell the rules refuse.
pub const FALLBACK_CANDIDATE_ILLEGAL: &str = "FALLBACK_CANDIDATE_ILLEGAL";

/// The fallback answer, and what it knows about itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackAnswer {
    /// A stone placed this turn completes six or more: the turn wins now, so
    /// its score is a mate in one turn whichever stone completes it
    /// (docs/decisions.md D-3, rule 4).
    WinsThisTurn(Turn),
    /// No stone this scan reached wins this turn; the turn is the first legal
    /// candidates of the deterministic ordering, unevaluated.
    Ordinary(Turn),
}

impl FallbackAnswer {
    /// The turn to play, whatever it knows.
    pub fn turn(self) -> Turn {
        match self {
            FallbackAnswer::WinsThisTurn(turn) | FallbackAnswer::Ordinary(turn) => turn,
        }
    }
}

/// The bounded fallback for `state`, which must be ongoing and at a turn
/// boundary — exactly what the search has already checked when it calls this.
///
/// 1. Every candidate is tried for an instant win; the first (ascending) that
///    completes a line is the whole turn (rule 4: the second stone is then not
///    played).
/// 2. A one-stone turn (turn 1) is the first candidate.
/// 3. Otherwise the first candidate goes down, the candidates of the resulting
///    half-turn position are scanned for a completing second stone, and the
///    turn is that pair — or the pair with the first candidate of the
///    half-turn position when no second stone completes.
///
/// # Panics
///
/// With [`FALLBACK_NO_CANDIDATES`] if the policy offers no cell (the search
/// refuses that position by name before deepening, so reaching it here is a
/// caller that skipped the check), and with [`FALLBACK_CANDIDATE_ILLEGAL`] if
/// the policy offers a cell the rules refuse.
pub fn fallback_turn(state: &GameState, policy: CandidatePolicy) -> FallbackAnswer {
    let cells = candidate_cells(state.board(), policy);
    assert!(
        !cells.is_empty(),
        "pistol-search invariant {FALLBACK_NO_CANDIDATES}: the candidate policy offers no cell \
         at turn {}, which the search refuses by name before the fallback runs",
        state.turn()
    );

    let mut scratch = state.clone();
    if let Some(winner) = winning_placement(&mut scratch, &cells) {
        return FallbackAnswer::WinsThisTurn(Turn::single(winner));
    }

    let first = cells[0];
    if matches!(place(&mut scratch, first), PlyOutcome::TurnComplete) {
        // Rule 3: this turn owes exactly one stone.
        return FallbackAnswer::Ordinary(Turn::single(first));
    }

    // The mover still owes a stone. The half-turn position's candidates
    // include cells around `first` itself, so the set cannot be empty for any
    // policy the search accepts (docs/decisions.md D-104's argument); an empty
    // set here is the same broken policy `NO_CANDIDATES_MID_TURN` names.
    let seconds = candidate_cells(scratch.board(), policy);
    assert!(
        !seconds.is_empty(),
        "pistol-search invariant {FALLBACK_NO_CANDIDATES}: the candidate policy ran dry half \
         way through turn {}, where the mover still owes a stone",
        scratch.turn()
    );
    let (second, wins) = match winning_placement(&mut scratch, &seconds) {
        // `second` completes a line once `first` is down: the pair wins this
        // turn, with the completing stone placed second (rule 4 leaves it an
        // ordinary two-stone turn, docs/decisions.md D-96).
        Some(winner) => (winner, true),
        None => (seconds[0], false),
    };
    let turn = Turn::pair(first, second).unwrap_or_else(|error| {
        panic!(
            "pistol-search invariant {FALLBACK_CANDIDATE_ILLEGAL}: {first} and {second} do not \
             pair: {error}"
        )
    });
    if wins {
        return FallbackAnswer::WinsThisTurn(turn);
    }
    FallbackAnswer::Ordinary(turn)
}

/// The first (ascending) cell whose placement completes a line right now, with
/// the position handed back exactly as it was.
fn winning_placement(scratch: &mut GameState, cells: &[Coord]) -> Option<Coord> {
    cells.iter().find(|&&at| wins_after(scratch, at)).copied()
}

/// Whether placing `at` completes a line, tried and taken back.
fn wins_after(scratch: &mut GameState, at: Coord) -> bool {
    let outcome = place(scratch, at);
    undo(scratch);
    matches!(outcome, PlyOutcome::Win { .. })
}

/// Place a cell the policy offered, panicking by name if the rules refuse it.
fn place(scratch: &mut GameState, at: Coord) -> PlyOutcome {
    scratch.place(at).unwrap_or_else(|error| {
        panic!(
            "pistol-search invariant {FALLBACK_CANDIDATE_ILLEGAL}: the candidate policy offered \
             {at}, and the rules say: {error}"
        )
    })
}

/// Take back the stone this module just placed.
fn undo(scratch: &mut GameState) {
    scratch.undo().unwrap_or_else(|error| {
        panic!("pistol-search invariant {FALLBACK_CANDIDATE_ILLEGAL}: taking back: {error}")
    });
}
