//! What a search reports: once per completed depth, and once at the end.
//!
//! Every depth in **turns**, every score in the band [`crate::score`] describes,
//! every move a [`Turn`]. Plies are an implementation detail of the recursion
//! and do not appear here (docs/decisions.md D-9).
//!
//! # What is comparable between two runs
//!
//! Everything except [`SearchInfo::time_ms`] and [`SearchInfo::nps`]. Those two
//! are measurements of the machine, not of the search: the determinism law is
//! about what the search *chose* and how much work it did, so two runs of the
//! same position under the same reproducible budget must agree on the move, the
//! node count, the principal variation and the score, and are free to disagree
//! about how long it took (CLAUDE.md rule 4, docs/decisions.md D-7).

use pistol_core::Turn;

/// One report from the search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchInfo {
    /// The depth this report is for, in turns — always a depth that was
    /// actually COMPLETED. Zero only in a returned outcome under a wall-clock
    /// abort that no iteration survived, where the answer is the fallback or
    /// the aborted iteration's completed root prefix; partial-iteration work is
    /// never attributed to a completed depth (WP-1.4).
    pub depth_turns: u32,
    /// The deepest any line reached, in turns. Equal to `depth_turns` in a
    /// completed iteration — Stage 0 has no extension that passes the horizon —
    /// and larger in the returned outcome when the budget interrupted a deeper
    /// iteration, which reached further before it was abandoned.
    pub seldepth_turns: u32,
    /// Nodes visited since the search began, leaves included.
    pub nodes: u64,
    /// Nodes per second over the whole search. A measurement, never an input.
    pub nps: u64,
    /// Wall-clock milliseconds since the search began. A measurement.
    pub time_ms: u64,
    /// The line the search believes will be played, starting with the move it
    /// would make. Empty only if there was nothing to search.
    pub pv: Vec<Turn>,
    /// The score of the position from the point of view of the side to move at
    /// the root; read it with [`crate::score::classify`].
    pub score: i32,
    /// How full the transposition table is, in parts per thousand.
    pub hashfull_permille: u32,
}

/// What a search returns: the move, and the report that goes with it.
///
/// `best` is always `info.pv[0]`. It is named separately because it is the
/// answer to the question that was asked, and the report is the evidence
/// (docs/decisions.md D-2).
///
/// Under a reproducible stop the report is the last completed depth's — its
/// line, its score, its depth — with the **totals** for the whole search in
/// `nodes`, `time_ms`, `nps`, `seldepth_turns` and `hashfull_permille`. An
/// iteration the budget interrupted is discarded as an answer but not as work:
/// a node budget is a promise about what the search spends, and per-side
/// compute is a reporting requirement (CLAUDE.md rule 6). So the final `nodes`
/// is generally larger than the one in the last report the callback saw.
///
/// Under a wall-clock stop the answer may come from deeper than the last
/// completed depth — the aborted iteration's completed root prefix, or the
/// pre-deepening fallback when nothing completed at all — and `provenance`
/// says which, because a score whose kind cannot be read from the data is the
/// silent widening CLAUDE.md rule 10 forbids. `depth_turns` still counts only
/// completed depths: the depth field understates, never overstates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchOutcome {
    /// The turn the engine would play.
    pub best: Turn,
    /// The report: the answer's line and score, the completed depth, the
    /// whole search's totals.
    pub info: SearchInfo,
    /// Where the answer came from.
    pub provenance: Provenance,
}

/// Where a search's answer came from — closed, and telling a consumer exactly
/// how to read `SearchInfo::score` beside `SearchInfo::depth_turns`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// The last completed iteration's move: the score is exact at
    /// `depth_turns`. The only provenance a reproducible stop can produce, and
    /// therefore the only one a strength claim ever quotes (CLAUDE.md rule 6).
    CompletedDepth,
    /// A wall-clock abort's salvage: the move was fully searched at one turn
    /// DEEPER than `depth_turns` inside the aborted iteration, its score exact
    /// for that move there — a lower bound on the position, not its value.
    PartialRoot,
    /// A wall-clock abort before any iteration completed: the pre-deepening
    /// fallback. The score is the root static evaluation — or a mate score,
    /// when the fallback's instant-win check proved the turn wins (rule 4).
    Fallback,
}
