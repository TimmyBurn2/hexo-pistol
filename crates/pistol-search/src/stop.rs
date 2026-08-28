use std::time::Instant;

/// How many nodes pass between two tests of the stop condition.
///
/// A power of two so the test is a mask, and large enough that the check costs
/// nothing next to the node itself.
///
/// # The stopping point, and its two exceptions
///
/// The node condition is tested every interval of SEARCH nodes, at a node
/// boundary, which makes an UNGATED search's stopping point exact and
/// reproducible: given `n` nodes it stops on `n.next_multiple_of(INTERVAL)`, on
/// every machine and in every run (CLAUDE.md rule 4). The interval is a pinned
/// constant of that contract, not a tunable — a configurable granularity would
/// make two runs at the same node budget incomparable.
///
/// **A GATED SEAT HAS ITS OWN BOUND** (WP-1.8c, superseding docs/decisions.md
/// D-441's overshoot claim). With the solver on the search path its nodes are
/// absorbed in whole calls, so the stopping point is not a multiple of
/// anything: such a search stops within one interval of search nodes plus one
/// visit's own two capped solver calls of its budget. D-441 registered the
/// overshoot as bounded by a single capped call and it was not bounded at all —
/// the exact-multiple test was made on the DERIVED total, which a call moves by
/// thousands, so it stepped over the multiples and a gated seat MEASURED a mean
/// 156,313 nodes against a 50,000 budget. `crate::pvs`'s `should_stop` carries
/// the fix and the reasoning.
///
/// **BOTH SENTENCES ABOVE ARE ABOUT DEPTH >= 2.** The first iteration of
/// iterative deepening is not abortable — there is no completed answer to fall
/// back on yet — so no budget check is consulted during it, and a search whose
/// depth-1 iteration alone outruns its budget overshoots by whatever that
/// iteration costs. That is D-74's granularity rather than the gated seat's:
/// MEASURED at a 1,000-node budget, five of twenty late-game positions exceed
/// the gated bound and two of the five make no solver call at all; at the
/// 50,000-node instrument budget none of the twenty does.
///
/// The DEADLINE condition is not masked at all (WP-1.4, docs/decisions.md
/// D-207): it is tested at every abortable node, and inside the move-ordering
/// scoring loop every `ORDER_CHECK_INTERVAL` cells, because a mask tuned for
/// node budgets would let a whole interval of nodes — each with an ordering
/// pass — run past the clock, which is D-95's magnitude class.
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
    /// abortable node and inside the ordering pass, never masked (see
    /// [`NODE_CHECK_INTERVAL`]). **Not** reproducible: the engine refuses to build one of
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
