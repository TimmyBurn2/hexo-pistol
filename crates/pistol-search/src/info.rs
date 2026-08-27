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

use crate::staged::StagedRow;

/// The node protocol's stage-share counters (docs/decisions.md U2-M item 2).
///
/// All zero under `CandidatePolicy::Radius`, where the staged dispatch never
/// runs. **The line protocol does not carry these** — `report.rs` renders an
/// explicit field list, so no protocol output changes; the rates are read
/// through a committed harness in the `pistol-search` test tree that calls
/// `Searcher::search` directly, the same shape `wp15b_census.rs` reads
/// (`crates/pistol-solver/tests/`) rather than through a printed line.
///
/// Every field is a WHOLE-SEARCH total, like [`SearchInfo::nodes`]: written
/// from the same point, on every [`SearchInfo`] construction path including
/// both salvage ones, so a counter never silently reads zero on a path that
/// visited real nodes.
///
/// Stage Q's own quantities — the widening rate per node class and the TT
/// entries the truncation rule declines to store — DEFER with stage Q
/// (`WPQ_seed.md` §7.2) and are not here; this D-scope's nearest counted
/// proxy is [`StageCounters::batched_quiet_safety_net`], which is not a
/// widening-rate quantity and is documented as such at its own field.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StageCounters {
    /// Nodes that took the WIN-NOW row: Tier F fired (`crate::staged`'s F
    /// firing rate).
    pub win_now: u64,
    /// Nodes that took the FILTERED row.
    pub filtered: u64,
    /// Nodes that took the BATCHED row (`Cover::NothingToBlock`): Tier T
    /// fired, or the quiet-ball safety net did in its place.
    pub batched: u64,
    /// Of [`StageCounters::batched`] and [`StageCounters::batched_lost`]
    /// combined, how many had Tier T itself empty and used the quiet-ball
    /// safety net (`crate::staged`'s module doc) instead — NOT stage Q's own
    /// widening-rate quantity, which this D-scope does not implement; the
    /// nearest counted proxy for "how often the T-only D-scope had nothing
    /// to offer beyond the raw ball."
    pub batched_quiet_safety_net: u64,
    /// Nodes that took the BATCHED-lost row (`Cover::Impossible` at a PV
    /// node or the root): the position is lost, but the line must still be
    /// searched to prove the score.
    pub batched_lost: u64,
    /// Nodes where `blocking_covers` answered `Impossible` — the union of
    /// [`StageCounters::overload_return`] and [`StageCounters::batched_lost`].
    pub cover_impossible: u64,
    /// Nodes that took `LAW-OVERLOAD`'s early return: no child expanded.
    pub overload_return: u64,
    /// `crate::quiescence`'s own totals (WP-1.6,
    /// docs/wp16_quiescence_design.md §8) — written only by `Run::quiescence`
    /// and its own helpers, never by `staged_candidates`' dispatch, so these
    /// are structurally disjoint from every field above.
    ///
    /// Every node `Run::quiescence` (or its ply-2 continuation) visits, both
    /// plies of every granted turn, every chain link.
    pub qnodes: u64,
    /// The gate's trigger (a) fired (win-now).
    pub q_win_now: u64,
    /// The gate's or ply-2's `LAW-OVERLOAD` shortcut fired
    /// (`Cover::Impossible`), combined — the WP's own analysis can split
    /// gate-vs-ply-2 by reading [`StageCounters::qnodes`] alongside it if
    /// needed.
    pub q_overload_return: u64,
    /// The gate's trigger (b) fired and an extension was granted.
    pub q_extend_defense: u64,
    /// The gate's trigger (c) fired and an extension was granted.
    pub q_extend_offense: u64,
    /// The gate was reached and neither trigger fired.
    pub q_stand_pat_no_trigger: u64,
    /// A trigger fired but the quiescence budget was already spent.
    pub q_stand_pat_cap: u64,
}

impl StageCounters {
    /// Record one node's [`StagedRow`] verdict.
    pub(crate) fn record(&mut self, row: StagedRow) {
        match row {
            StagedRow::WinNow => self.win_now += 1,
            StagedRow::Filtered => self.filtered += 1,
            StagedRow::Batched => self.batched += 1,
            StagedRow::BatchedLost => {
                self.batched_lost += 1;
                self.cover_impossible += 1;
            }
            StagedRow::OverloadReturn => {
                self.overload_return += 1;
                self.cover_impossible += 1;
            }
        }
    }

    /// Record that a BATCHED or BATCHED-lost row just recorded used the
    /// quiet-ball safety net (`StagedSet::used_quiet_safety_net`) rather than
    /// a non-empty Tier T. Called separately from [`StageCounters::record`]
    /// because the safety net is this D-scope's own IMPL choice and not a
    /// `StagedRow` of the node protocol itself (`crate::staged`'s doc).
    pub(crate) fn record_quiet_safety_net(&mut self) {
        self.batched_quiet_safety_net += 1;
    }
}

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
    /// The node protocol's stage-share counters (docs/decisions.md U2-M item
    /// 2), whole-search totals like [`SearchInfo::nodes`]. All zero under
    /// `CandidatePolicy::Radius`.
    pub stages: StageCounters,
    /// SEARCH nodes this search — the first of the two counters the
    /// instrument prints separately every turn a solver call spent anything
    /// (the commissioning dispatch's own wording; design wp18b §3). Zero
    /// difference from `nodes` whenever the gate is off.
    pub search_nodes: u64,
    /// Solver nodes spent on the search path this search (design wp18b §3:
    /// counted against the same budget as `nodes`, printed separately).
    /// ONE OF THE TWO INDEPENDENT COUNTERS — `nodes` is their derived sum
    /// at report time, and the sum test compares the two writers. Zero
    /// whenever the gate is off.
    pub solver_nodes: u64,
    /// How many solver calls returned `NoWinUnderZone` (design wp18b §8:
    /// loud, never swallowed — a counter, not a silent drop).
    pub solver_refusals: u32,
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
    /// The root solver proof answered before any deepening (design wp18b
    /// §2 D3): the move is the PROOF's first move, the score is the proof's
    /// mate distance, `depth_turns` is the proof's depth in turns.
    SolverProof,
    /// A wall-clock abort before any iteration completed: the pre-deepening
    /// fallback. The score is the root static evaluation — or a mate score,
    /// when the fallback's instant-win check proved the turn wins (rule 4).
    Fallback,
}
