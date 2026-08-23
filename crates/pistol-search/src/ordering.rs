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
//! (docs/decisions.md D-76). Since D-214 the reading arrives through
//! [`Eval::delta`](pistol_eval::Eval::delta) — the same number the D-76
//! apply/value/undo roundtrip produced, without mutating the eval — because
//! that roundtrip was measured at 76.27% of profiled stacks under this
//! module's `order` (D-192). A cell that completes a line saturates the eval
//! band, so a winning ply sorts to the front without the ordering knowing what
//! a win is.
//!
//! # Determinism
//!
//! The sort is **stable** and the candidates arrive in ascending `(q, r)`, so
//! equal scores stay in coordinate order and the lexicographic tie-break
//! (docs/decisions.md D-5, D-7) costs nothing and cannot drift. Nothing here
//! reads a clock, a hasher, or a node count (CLAUDE.md rule 4).

use std::cmp::Reverse;
use std::time::Instant;

use pistol_core::Coord;

use crate::position::Position;

/// How many candidates are scored between two deadline checks.
///
/// The scoring loop is the longest uninterruptible stretch inside one node —
/// its length is the candidate count, which grows with how spread out the
/// stones are, which the opponent partly controls (docs/decisions.md D-95). So
/// under a wall-clock stop the loop checks the deadline at this stride, and the
/// worst-case tail of a node is this many static-score probes rather than
/// a whole ordering pass. A constant of the overshoot bound, not a tunable.
pub const ORDER_CHECK_INTERVAL: usize = 64;

/// How an ordering pass ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderOutcome {
    /// `cells` is sorted and ready to search.
    Completed,
    /// The deadline passed mid-scoring; `cells` is in an unspecified order and
    /// must not be searched — the caller aborts the node.
    DeadlinePassed,
}

/// Sort `cells` into the order the search will try them.
///
/// `cells` must be ascending and duplicate-free, which is what
/// [`crate::candidate_cells`] emits.
///
/// `deadline` is `Some` only under a wall-clock stop in an abortable
/// iteration: instrument mode and every reproducible budget pass `None`, and
/// with `None` this function performs **zero clock reads** — the determinism
/// law's no-clock-on-a-choice-path rule, kept by construction (CLAUDE.md
/// rule 4, docs/decisions.md D-73).
pub fn order(
    position: &mut Position,
    cells: &mut Vec<Coord>,
    table_move: Option<Coord>,
    deadline: Option<Instant>,
) -> OrderOutcome {
    let mut scored: Vec<(i32, Coord)> = Vec::with_capacity(cells.len());
    for (index, &at) in cells.iter().enumerate() {
        if let Some(expires) = deadline
            && index % ORDER_CHECK_INTERVAL == 0
            && index > 0
            && Instant::now() >= expires
        {
            return OrderOutcome::DeadlinePassed;
        }
        scored.push((position.static_score_after(at), at));
    }
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
    OrderOutcome::Completed
}

#[cfg(test)]
mod tests {
    use pistol_core::Player;
    use pistol_eval::Eval;

    use super::*;
    use crate::position::Position;

    /// An evaluation that scores a stone by its `q` and nothing else, so a test
    /// can state the order it expects rather than derive it from the pattern
    /// tables. What is under test here is the ordering, not the eval.
    struct ByQ {
        sum: i32,
    }

    impl ByQ {
        fn boxed() -> Box<dyn Eval> {
            Box::new(ByQ { sum: 0 })
        }
    }

    impl Eval for ByQ {
        fn apply(&mut self, at: Coord, player: Player) {
            self.sum += sign(player) * i32::from(at.q);
        }

        fn undo(&mut self, at: Coord, player: Player) {
            self.sum -= sign(player) * i32::from(at.q);
        }

        fn value(&self, side_to_move: Player) -> i32 {
            sign(side_to_move) * self.sum
        }
    }

    fn sign(player: Player) -> i32 {
        match player {
            Player::P1 => 1,
            Player::P2 => -1,
        }
    }

    /// The table's move is promoted only if it is one of the candidates.
    ///
    /// A 64-bit verification word can in principle answer about a different
    /// position (docs/decisions.md D-8), so `best` is not guaranteed to be a
    /// cell this node may play. Without the membership test the search would
    /// place a stone the candidate policy never offered, which `visit` refuses
    /// by name as `CANDIDATE_ILLEGAL` — a panic one level down, naming the
    /// policy, for something the ordering did.
    ///
    /// This is a private-invariant guard and lives here because `order` is
    /// `pub(crate)` and the drop is invisible from outside: a search that
    /// promoted a foreign cell would fail somewhere else entirely
    /// (docs/decisions.md D-115).
    ///
    /// Mutation checked: replacing the `position(..)`/`rotate_right` pair with
    /// an unconditional `cells.insert(0, best)` makes this test red.
    #[test]
    fn a_table_move_that_is_not_a_candidate_is_dropped_rather_than_played() {
        let mut position = Position::new(ByQ::boxed(), false);
        let candidates = [Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)];
        // By `ByQ`, q ascending is score ascending, so the sorted order is the
        // reverse of the ascending input `order` requires.
        let sorted = vec![Coord::new(2, 0), Coord::new(1, 0), Coord::new(0, 0)];

        // A cell that is nowhere in the set leaves it exactly as the sort left
        // it, and above all does not join it.
        let mut cells = candidates.to_vec();
        order(&mut position, &mut cells, Some(Coord::new(9, 9)), None);
        assert_eq!(
            cells, sorted,
            "a table move that is not a candidate must neither be played nor displace one"
        );

        // The control, without which this test would also pass on an `order`
        // that had stopped promoting anything at all: the same table move, when
        // it IS a candidate, goes first.
        let mut cells = candidates.to_vec();
        order(&mut position, &mut cells, Some(Coord::new(0, 0)), None);
        assert_eq!(
            cells,
            vec![Coord::new(0, 0), Coord::new(2, 0), Coord::new(1, 0)],
            "a table move that is a candidate is tried first, and the rest keep the sort's order"
        );
    }
}
