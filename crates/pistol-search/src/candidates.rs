//! Which cells the search considers: the candidate policy, intersected with the
//! rule-5 legal region.
//!
//! The intersection is computed, never reasoned about. A cell is a candidate iff
//! the policy offers it **and** `pistol_core` says a stone may go there — the
//! two radii are never compared, and no shortcut is taken on the grounds that
//! one is smaller than the other (CLAUDE.md rule 2, docs/decisions.md D-20).
//! That also means a policy radius wider than the legal region simply yields the
//! legal region, without this module knowing why.
//!
//! # Order
//!
//! Ascending `(q, r)`, each cell once. That is the order the whole engine breaks
//! ties in (docs/decisions.md D-5, D-7), and move ordering leans on it: a stable
//! sort by score leaves equal-scoring cells in coordinate order, so the final
//! tie-break costs nothing and cannot drift between runs (CLAUDE.md rule 4).

use std::collections::BTreeSet;

use pistol_core::{Board, Coord, legal_placements};

use crate::params::CandidatePolicy;

/// Every cell the policy offers that a stone may legally go on, ascending.
///
/// An empty board is not a special case so much as the honest reading of a
/// proximity policy: there is no stone to be near, so the policy restricts
/// nothing and rule 3 decides alone — the origin, and nothing else.
///
/// A radius of zero reaches only the stones themselves, which are occupied, so
/// it yields nothing. [`crate::Searcher::new`] refuses that radius by name; this
/// function answers the question it was asked.
pub fn candidate_cells(board: &Board, policy: CandidatePolicy) -> Vec<Coord> {
    match policy {
        CandidatePolicy::Radius { radius } => within_radius(board, radius),
    }
}

/// The union of the radius-`radius` balls around the stones, intersected with
/// the cells rule 5 admits.
fn within_radius(board: &Board, radius: u32) -> Vec<Coord> {
    if board.is_empty() {
        return legal_placements(board);
    }
    let offsets = ball_offsets(radius);
    let mut cells = BTreeSet::new();
    for (stone, _) in board.stones() {
        for &delta in &offsets {
            // A ball cell off the addressable lattice is not a cell
            // (docs/decisions.md D-47).
            if let Some(cell) = stone.checked_offset(delta) {
                cells.insert(cell);
            }
        }
    }
    cells
        .into_iter()
        .filter(|&cell| board.is_legal_placement(cell))
        .collect()
}

/// The offsets of a ball of the given radius, centre included, ascending.
///
/// This is the policy's own geometry, built out of [`Coord::distance`]. It is
/// not a restatement of the rules' region: that one is enumerated in
/// pistol-core at its own pinned radius, and this one is enumerated here at
/// whatever radius the operator configured.
fn ball_offsets(radius: u32) -> Vec<Coord> {
    let reach = i16::try_from(radius).unwrap_or(i16::MAX);
    let mut offsets = Vec::new();
    for dq in -reach..=reach {
        for dr in -reach..=reach {
            let delta = Coord::new(dq, dr);
            if Coord::ORIGIN.distance(delta) <= radius {
                offsets.push(delta);
            }
        }
    }
    offsets
}
