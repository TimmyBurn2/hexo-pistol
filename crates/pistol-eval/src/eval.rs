//! The evaluation contract: what every backend in this crate promises a search.
//!
//! The contract is **incremental**, and that is a design decision rather than an
//! optimization (docs/decisions.md D-11, D-61). The Stage-2 backend is a pattern
//! codebook net that is only affordable because it is updated per placed stone;
//! writing the trait that way now means swapping the backend later is not also a
//! search change.

use pistol_core::{Coord, Player};

/// The largest magnitude a static evaluation may report.
///
/// Above this sits the mate band: `MATE = 30_000` and `MateIn(t) = MATE - t`,
/// scored in turns (docs/decisions.md D-3). A win is the search's to report —
/// it knows the distance — so eval saturates here rather than ever crossing
/// into that range. Keeping the two bands apart is what lets a search compare a
/// static value against a mate score without a special case.
pub const EVAL_MAX: i32 = 16_000;

/// A position evaluator, updated one stone at a time.
///
/// # Contract
///
/// - [`Eval::apply`] and [`Eval::undo`] are called by whoever moves the stone,
///   at the same seam the board and the zobrist key are updated
///   (docs/decisions.md D-41). The player travels with the cell so that an
///   implementation never has to consult a board — that is the whole reason,
///   and NOT so that it can cross-check what it is told against what it already
///   holds: this crate's checks are opportunistic and cannot see a caller that
///   mispairs an apply with an undo, so pairing them is the caller's invariant
///   (docs/decisions.md D-70, and D-108 which withdrew the cross-check clause
///   this sentence used to make).
/// - The two are inverses, and the value depends only on the *set* of stones
///   applied — never on the order they arrived in, nor on the order they are
///   taken back in. A search relies on the first half; the second half is what
///   makes the first cheap to be sure of.
/// - [`Eval::value`] is **side-relative**: positive means good for
///   `side_to_move`, so a negamax search negates a child's value instead of
///   tracking whose number it is. Its magnitude never exceeds [`EVAL_MAX`].
/// - Integer arithmetic only, no interior randomness, no dependence on
///   iteration order: two runs of the same position give the same number, on
///   any machine (CLAUDE.md rule 4).
/// - Neither call fails. A cell that may not hold a stone is refused by
///   `pistol_core::GameState` long before an eval hears about it; being told
///   something impossible anyway is a broken invariant in pistol, and an
///   implementation panics with a named token rather than returning an error
///   nobody could handle (CLAUDE.md rule 3).
///
/// The trait is object safe on purpose: the engine picks a backend from config
/// at construction, and `Box<dyn Eval>` is the cheapest way to hold that choice
/// without making every search type generic over it.
pub trait Eval {
    /// Account for a stone of `player` arriving on `at`.
    fn apply(&mut self, at: Coord, player: Player);

    /// Account for the stone of `player` on `at` being taken back.
    fn undo(&mut self, at: Coord, player: Player);

    /// What the position is worth to `side_to_move`, in `-EVAL_MAX..=EVAL_MAX`.
    fn value(&self, side_to_move: Player) -> i32;
}
