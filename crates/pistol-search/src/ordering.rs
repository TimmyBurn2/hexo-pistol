//! Which candidate the search tries first.
//!
//! Alpha-beta's whole yield depends on this: a node whose best ply is tried
//! first cuts after one child, and a node whose best ply is tried last searches
//! everything. Stage 0 orders on two things and no more — killers, history and
//! countermoves arrive in Stage 1 with the arena that can measure them
//! (docs/ROADMAP.md):
//!
//! 1. the table's move for this position, if it has one;
//! 2. every other candidate by what the evaluation makes of a stone there.
//!
//! The static score is the [`Eval`](pistol_eval::Eval) backend's own reading —
//! the search does not carry a second opinion about what a pattern is worth
//! (docs/decisions.md D-76). A cell that completes a line saturates the eval
//! band, so a winning ply sorts to the front without the ordering knowing what a
//! win is.
//!
//! # Determinism
//!
//! The sort is **stable** and the candidates arrive in ascending `(q, r)`, so
//! equal scores stay in coordinate order and the lexicographic tie-break
//! (docs/decisions.md D-5, D-7) costs nothing and cannot drift. Nothing here
//! reads a clock, a hasher, or a node count (CLAUDE.md rule 4).

use std::cmp::Reverse;

use pistol_core::Coord;

use crate::position::Position;

/// Sort `cells` into the order the search will try them.
///
/// `cells` must be ascending and duplicate-free, which is what
/// [`crate::candidate_cells`] emits.
pub fn order(position: &mut Position, cells: &mut Vec<Coord>, table_move: Option<Coord>) {
    let mut scored: Vec<(i32, Coord)> = cells
        .iter()
        .map(|&at| (position.static_score_after(at), at))
        .collect();
    // Descending by score. `sort_by` is stable, so equal scores keep the
    // ascending coordinate order they arrived in.
    scored.sort_by_key(|&(score, _)| Reverse(score));

    cells.clear();
    cells.extend(scored.into_iter().map(|(_, at)| at));

    // The table's move goes first, and only if it is a move here: a verification
    // word is 64 bits, so a probe can in principle answer about a different
    // position, and a cell that is not a candidate would be a stone the search
    // has no business placing.
    if let Some(best) = table_move
        && let Some(found) = cells.iter().position(|&at| at == best)
    {
        // Rotate rather than swap: the cells behind it keep the order the sort
        // put them in, so the table's move costs nothing but its own place.
        cells[..=found].rotate_right(1);
    }
}
