//! The principal variation: the line the search believes both sides will play.
//!
//! It is collected as it is found, in a triangular table — each node keeps the
//! best line beneath it, and a node that improves on alpha takes its child's
//! line and puts its own ply in front. Reading it back out of the transposition
//! table afterwards would be cheaper and would also be a guess: entries get
//! overwritten, and the line that comes back can be a different line from the
//! one the score was proved on.
//!
//! # Plies in, turns out
//!
//! The table holds plies, because that is what the search moves in. What a
//! caller sees is [`Turn`]s, because that is what the rules and the protocol
//! are written in (docs/decisions.md D-5, D-9). [`turns_from_plies`] converts by
//! **replaying** the line through `GameState`, so the grouping is the rules'
//! own: a first stone that completes a line ends its turn (rule 4) and is a
//! `Single`, and everything else pairs up.

use pistol_core::{Coord, GameState, PlyOutcome, Turn};

/// Named invariant: a principal variation that is not playable from the root,
/// or that ends in the middle of a turn.
pub const PV_NOT_PLAYABLE: &str = "PV_NOT_PLAYABLE";

/// The lines under every ply of the current search.
pub struct PvTable {
    lines: Vec<Coord>,
    lengths: Vec<usize>,
    stride: usize,
}

impl PvTable {
    /// A table for a search that will not exceed `max_ply` plies.
    pub fn new(max_ply: usize) -> PvTable {
        PvTable {
            lines: vec![Coord::ORIGIN; max_ply * max_ply],
            lengths: vec![0; max_ply + 1],
            stride: max_ply,
        }
    }

    /// Forget the line under `ply`. Every node does this on entry, so a node
    /// that fails low leaves no stale line behind for its parent to adopt.
    pub fn clear(&mut self, ply: usize) {
        self.lengths[ply] = 0;
    }

    /// Make `at` the first ply of the line under `ply`, followed by whatever the
    /// child found.
    pub fn promote(&mut self, ply: usize, at: Coord) {
        let child = self.lengths[ply + 1];
        assert!(
            child < self.stride,
            "pistol-search invariant {PV_NOT_PLAYABLE}: a line of {} plies does not fit the table",
            child + 1
        );
        let base = ply * self.stride;
        let child_base = (ply + 1) * self.stride;
        self.lines
            .copy_within(child_base..child_base + child, base + 1);
        self.lines[base] = at;
        self.lengths[ply] = child + 1;
    }

    /// The line under `ply`.
    pub fn line(&self, ply: usize) -> &[Coord] {
        let base = ply * self.stride;
        &self.lines[base..base + self.lengths[ply]]
    }
}

/// Group a line of plies into the turns it plays, by replaying it.
///
/// # Panics
///
/// With [`PV_NOT_PLAYABLE`] if a ply is not legal from the position the ones
/// before it reach, or if the line stops with a turn half played. Both would
/// mean the search reported a line it did not search.
pub fn turns_from_plies(root: &GameState, plies: &[Coord]) -> Vec<Turn> {
    let mut state = root.clone();
    let mut turns = Vec::new();
    let mut pending: Option<Coord> = None;

    for &at in plies {
        let outcome = state.place(at).unwrap_or_else(|error| {
            panic!("pistol-search invariant {PV_NOT_PLAYABLE}: replaying {at}: {error}")
        });
        match outcome {
            PlyOutcome::TurnContinues => pending = Some(at),
            PlyOutcome::TurnComplete | PlyOutcome::Win { .. } => {
                turns.push(match pending.take() {
                    Some(first) => Turn::pair(first, at).unwrap_or_else(|error| {
                        panic!("pistol-search invariant {PV_NOT_PLAYABLE}: {error}")
                    }),
                    None => Turn::Single(at),
                });
            }
        }
    }

    assert!(
        pending.is_none(),
        "pistol-search invariant {PV_NOT_PLAYABLE}: the line ends with turn {} half played",
        state.turn()
    );
    turns
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A line a node abandoned is not adopted by the node above it.
    ///
    /// [`PvTable::clear`] drops the *length* and leaves the coordinates where
    /// they are — zeroing the row would be work at every node for a buffer
    /// nobody reads past its length. So the length is the whole guard: every
    /// node clears on entry, and a node that then fails low leaves a zero there
    /// rather than whatever a previous sibling's subtree wrote. Without it a
    /// parent's [`PvTable::promote`] copies the stale cells into its own line,
    /// and the search reports a variation it never searched — which surfaces at
    /// the root, as a `PV_NOT_PLAYABLE` panic or, worse, as a plausible line
    /// that is simply not the one the score was proved on.
    ///
    /// This is a private-invariant guard: `PvTable` is `pub(crate)`, and the
    /// staleness is invisible from outside because the stale line is often
    /// still legal (docs/decisions.md D-115).
    ///
    /// Mutation checked: making `clear` a no-op makes this test red.
    #[test]
    fn a_cleared_line_is_not_adopted_by_the_parent_that_promotes_over_it() {
        let (a, b, c) = (Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0));
        let mut pv = PvTable::new(4);

        // A sibling subtree found a two-ply line under ply 1.
        pv.promote(2, c);
        pv.promote(1, b);
        assert_eq!(pv.line(1), [b, c], "the line under ply 1 is what was found");

        // Ply 1 is re-entered for the next sibling and clears on entry; this
        // one fails low, so it never promotes anything.
        pv.clear(1);
        assert_eq!(
            pv.line(1),
            [] as [Coord; 0],
            "a node that has not improved on alpha has no line, whatever the buffer still holds"
        );

        // The parent improves on alpha and takes its child's line — which is
        // empty, so its own ply is the whole line.
        pv.promote(0, a);
        assert_eq!(
            pv.line(0),
            [a],
            "the parent adopts the empty line its child left, not the one a previous sibling wrote"
        );
    }
}
