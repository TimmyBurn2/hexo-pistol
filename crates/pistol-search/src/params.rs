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
/// One kind today. It is an enum rather than a bare number so that Stage 1's
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
