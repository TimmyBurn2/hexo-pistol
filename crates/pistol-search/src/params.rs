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
}
