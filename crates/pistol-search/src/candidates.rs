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
///
/// # `CandidatePolicy::Staged`
///
/// Answers about the **quiet ball alone** — `within_radius(board,
/// quiet_radius)` — and not about the node protocol's actual per-node
/// candidate set, which [`crate::staged::staged_candidates`] computes and this
/// function cannot express (it takes no threat state). This is deliberate and
/// named (`U2_node_protocol.md` §5.35, U2-Z item 21): the two callers that
/// reach this function under `Staged` — [`crate::fallback::fallback_turn`]
/// (U2-Z item 8) and [`crate::search::Searcher::check_root`]'s no-candidates
/// check — both need only a bounded, threat-state-free reachability answer,
/// never the search's real per-node set.
pub fn candidate_cells(board: &Board, policy: CandidatePolicy) -> Vec<Coord> {
    match policy {
        CandidatePolicy::Radius { radius } => within_radius(board, radius),
        CandidatePolicy::Staged(params) => within_radius(board, params.quiet_radius),
    }
}

/// The union of the radius-`radius` balls around the stones, intersected with
/// the cells rule 5 admits.
///
/// `pub(crate)`, not private: [`crate::staged`] reuses it twice — for the
/// fallback ball `CandidatePolicy::Staged`'s own arm above delegates to, and
/// for the quiet-ball safety net a BATCHED node falls back to when Tier F ∪
/// Tier T is empty (see that module's doc for why the D-scope needs one).
pub(crate) fn within_radius(board: &Board, radius: u32) -> Vec<Coord> {
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
    // `Searcher::new` refuses a radius this conversion cannot make, so a failure
    // here is a caller that reached the generator without passing that gate —
    // an internal invariant, named rather than silently substituted.
    let reach = i16::try_from(radius).unwrap_or_else(|_| {
        panic!(
            "RADIUS_UNREPRESENTABLE: candidate radius {radius} exceeds {}",
            i16::MAX
        )
    });
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
