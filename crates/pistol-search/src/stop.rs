//! When a search stops.
//!
//! This is the search's side of the engine's `Budget` (docs/decisions.md D-4):
//! the engine validates the operator's budget, refuses the ones its mode cannot
//! honour, and translates what is left into one of these. The translation is not
//! a copy — a duration becomes the [`Instant`] it expires at, because a search
//! that computed its own deadline would be reading the clock on a path that must
//! not exist in instrument mode (docs/decisions.md D-73).
//!
//! There is no "no limit". A search always has exactly one stop condition, and
//! an absent one is the engine's `BudgetMissing`, never a default here
//! (CLAUDE.md rule 1).
//!
//! # Granularity
//!
//! The node condition is tested every [`NODE_CHECK_INTERVAL`] nodes, at a node
//! boundary, which makes the stopping point exact and reproducible: a search
//! given `n` nodes stops on node `n.next_multiple_of(NODE_CHECK_INTERVAL)`, on
//! every machine and in every run (CLAUDE.md rule 4). The interval is a pinned
//! constant of that contract, not a tunable: a configurable granularity would
//! make two runs with different configs incomparable at the same node budget.
//!
//! The deadline condition is NOT masked (WP-1.4, docs/decisions.md D-207): it
//! is tested at every abortable node, and inside the move-ordering scoring
//! loop every `ORDER_CHECK_INTERVAL` cells, because a mask tuned for node
//! budgets would let up to [`NODE_CHECK_INTERVAL`] nodes — each with a whole
//! ordering pass — run past the clock, which is D-95's magnitude class. A
//! deadline is not reproducible anyway, so granularity buys it nothing.

use std::time::Instant;

/// How many nodes pass between two tests of the stop condition.
///
/// A power of two so the test is a mask, and large enough that the check costs
/// nothing next to the node itself.
pub const NODE_CHECK_INTERVAL: u64 = 1024;

const _: () = assert!(
    NODE_CHECK_INTERVAL.is_power_of_two(),
    "the node check is a mask"
);

/// What ends a search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// Complete this many turns of iterative deepening and stop. Reproducible.
    DepthTurns(u32),
    /// Stop at the first node check at or past this count. Reproducible.
    Nodes(u64),
    /// Stop at the first check at or past this instant — tested at every
    /// abortable node and inside the ordering pass, never masked (see the
    /// module doc). **Not** reproducible: the engine refuses to build one of
    /// these in instrument mode, and no other variant reads a clock.
    Deadline(Instant),
}

impl Stop {
    /// Whether two searches under this condition must agree, node for node.
    pub const fn is_reproducible(self) -> bool {
        match self {
            Stop::DepthTurns(_) | Stop::Nodes(_) => true,
            Stop::Deadline(_) => false,
        }
    }

    /// Whether a search that has visited `nodes` nodes has run out.
    ///
    /// Called only at the granularity above, and only from a node boundary.
    pub fn is_spent(self, nodes: u64) -> bool {
        match self {
            // Depth is not a running total; the iterative deepening loop is what
            // honours it, one completed iteration at a time.
            Stop::DepthTurns(_) => false,
            Stop::Nodes(cap) => nodes >= cap,
            Stop::Deadline(at) => Instant::now() >= at,
        }
    }
}
