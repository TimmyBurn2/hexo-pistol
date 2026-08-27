//! What the search is built with, and where those numbers come from.
//!
//! Every value here is stated by whoever constructs the search. There is no
//! `Default`, no field default and no code path that invents one (CLAUDE.md
//! rule 1): the engine reads them from a validated config and hands them over,
//! and a test states them in its own body. So this must not compile:
//!
//! ```compile_fail
//! let _ = pistol_search::SearchParams::default();
//! ```
//!
//! # Not a rule
//!
//! [`CandidatePolicy`] narrows what the search *looks at*. It says nothing
//! about what is legal — that is rule 5's radius-8 region, which lives in
//! pistol-core, is a different concept, and is never compared with this one
//! (CLAUDE.md rule 2, docs/decisions.md D-20). The policy is applied by
//! intersecting with the region, never by reasoning about which radius is
//! larger.

/// How the search chooses which cells to consider.
///
/// Two kinds. It is an enum rather than a bare number so that Stage 1's
/// threat-first generator is an added variant rather than a changed signature
/// (docs/ROADMAP.md).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidatePolicy {
    /// Every empty, legally placeable cell within `radius` of some stone.
    Radius {
        /// Hex distance from the nearest stone. At least 1; a policy that
        /// reaches nowhere would offer the search no move at all, which
        /// [`crate::Searcher::new`] refuses by name rather than discovering at
        /// the first node.
        radius: u32,
    },
    /// Threat-first staged pair generation (docs/decisions.md D-310;
    /// docs/ROADMAP.md WP-1.5b): the node protocol in [`crate::staged`],
    /// selecting Tier F (forced) and Tier T (`LAW-SUPPORT`-qualified) at every
    /// node in place of the radius ball. Stage Q, the quiet tier beyond Tier T,
    /// is deferred (`docs/experiments/WPQ_seed.md`, D-315) — this D-scope ships
    /// stages F and T only.
    Staged(StagedParams),
}

/// The parameters `CandidatePolicy::Staged` carries.
///
/// Deliberately narrower than the config document's `[search.candidate_policy]`
/// table (`U3_tier_t.md` §10), which also states `quiet_top_k` and
/// `widen_schedule` — those two govern stage Q's widening schedule, which this
/// D-scope does not implement (§1 above; `WPQ_seed.md`). Carrying them here
/// unused would be dead weight on the search's own hot-path type for a
/// mechanism that does not run; they are validated at the config layer
/// (`pistol-engine`) for schema completeness and go no further. **This is an
/// IMPL-time reading of an OPEN question the design left the architect's**
/// (`U3_tier_t.md` §U3-Z: "whether the D-scope shipped surface keeps those two
/// keys at all is OPEN") — WP-1.5c is where stage Q, and therefore where these
/// two keys' consumption, is arms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagedParams {
    /// Hex distance the fallback's quiet ball reaches (docs/decisions.md
    /// U2-Z item 8: "the fallback under Staged reuses the `quiet_radius`
    /// ball"). At least 1, same bound as [`CandidatePolicy::Radius`]'s radius.
    pub quiet_radius: u32,
    /// `LAW-SUPPORT`'s threshold for the side to move's own qualifying
    /// windows: 2 or 3 (`U3_tier_t.md` §6.1, the THRESHOLD reading — own
    /// windows qualify at count `>= tier_t_own_count`).
    pub tier_t_own_count: u8,
    /// `LAW-SUPPORT`'s threshold for the opponent's qualifying windows: 2 or 3.
    pub tier_t_opponent_count: u8,
    /// How many further whole turns a threat-only quiescence extension may
    /// grant at a horizon (`crate::quiescence`, WP-1.6,
    /// docs/wp16_quiescence_design.md §6). `0` disables the extension; the
    /// horizon's free checks (win-now, `LAW-OVERLOAD`) run regardless.
    pub q_depth_turns: u32,
    /// Which of `crate::quiescence`'s two gate triggers may grant an
    /// extension (docs/decisions.md D-396).
    pub q_triggers: QTriggers,
    /// WP-1.7's three ordering-heuristic gates
    /// (docs/experiments/wp17_design.md §6): killers, history and
    /// countermove reorder the staged candidate set's UNFORCED range. Each
    /// is `false` in every committed config until an SPRT says otherwise;
    /// there is no `Default` and no code-side default (CLAUDE.md rule 1) —
    /// a value exists because a config or a test stated it.
    pub ordering: OrderingHeuristics,
}

/// Which of WP-1.7's three ordering heuristics run
/// (`docs/experiments/wp17_design.md`). Three independent gates, each
/// defaulting OFF in every committed config; `any()` is the one question the
/// search's hot path asks, so a fully-off policy costs one boolean OR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderingHeuristics {
    /// Per-ply killer slots: the completing stone (two single-cell slots)
    /// and the canonical pair (one slot, phase-First plies).
    pub killers: bool,
    /// The (mover, cell) cutoff score, aged by halving at each new search.
    pub history: bool,
    /// The opponent's last placed stone → the reply that refuted it.
    pub countermove: bool,
}

impl OrderingHeuristics {
    /// Whether any gate is on — the one question `pvs::visit` asks before it
    /// touches the heuristic tables at all.
    pub const fn any(self) -> bool {
        self.killers || self.history || self.countermove
    }
}

/// Which quiescence gate triggers may grant an extension
/// (`crate::quiescence`, D-396). Mirrors
/// `pistol_engine::config::QTriggers` — this crate does not depend on
/// `pistol-engine` (the crate map's composition direction is the other
/// way), so the two are separate types with the same two variants, the
/// same pattern `CandidatePolicy` itself already follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QTriggers {
    /// Trigger (b) only (`docs/wp16_quiescence_design.md` §3.2).
    DefensiveOnly,
    /// Trigger (b) and trigger (c) (`docs/wp16_quiescence_design.md` §3.2,
    /// §3.3) — the compound configuration D-395 measured.
    DefensiveAndOffensive,
}

/// Everything the search needs that is not the position.
///
/// Deliberately not `Copy`-by-habit-only: it is small, but it is the operator's
/// configuration, and it is passed once at construction rather than threaded
/// through the recursion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchParams {
    /// Transposition table size in bytes. The table indexes by masking, so this
    /// must be a power of two, and it is a ceiling: the table takes the largest
    /// power-of-two count of buckets that fits inside it and never rounds the
    /// stated number up (docs/decisions.md D-19).
    pub tt_bytes: u64,
    /// Which cells the search is allowed to consider.
    pub candidate_policy: CandidatePolicy,
    /// The solver-on-the-search-path wiring (design wp18b §2). `None` is
    /// the OFF gate — the committed default in every config until an SPRT
    /// says otherwise — and OFF constructs no solver at all, so a gate-off
    /// search is byte-identical to the pre-wiring search by construction.
    /// `Some(wiring)` is the gate ON, refused under a Radius-kind policy.
    pub solver: Option<SolverWiring>,
}

/// What fires a solver call at a node (design wp18b §2 D1): the calculus
/// ID names the pattern class, and v0 wires exactly one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverTrigger {
    /// PAT-O4+ on either side: any hot window (an open four or better)
    /// held by the mover or the opponent, read off the staged policy's
    /// own `ThreatState`.
    AnyOpenFour,
}

/// The wiring the search consumes (design wp18b §2 §5): the gate, the
/// per-call cap, the trigger, and the solver's own validated parameters
/// (carried as `pistol_solver::SolverParams` — validated once, at the
/// engine's config layer, by the solver's own validator; the search never
/// re-reads a literal, rule 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverWiring {
    /// The per-call visit cap (design wp18b §2a). An EXPLICIT value, never
    /// defaulted in code.
    pub per_call_node_cap: u64,
    /// Which nodes fire calls.
    pub trigger: SolverTrigger,
    /// The solver's own parameters, validated at the config layer.
    pub inner: pistol_solver::SolverParams,
}
