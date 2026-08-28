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
