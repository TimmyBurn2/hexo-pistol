//! `pistol-search` — the classical search.
//!
//! Principal variation search with iterative deepening, a transposition table,
//! move ordering and budget handling. No MCTS, and in Stage 0 no quiescence, no
//! threat generation and none of the reductions the research report parks in
//! later stages.
//!
//! Two invariants constrain everything here:
//!
//! - the determinism law (CLAUDE.md rule 4) — in instrument mode nothing
//!   nondeterministic may influence move choice, so no unseeded hash iteration
//!   on a choice path, no time-based tie-breaks, no thread races;
//! - the search candidate policy is config, never a literal, and it is a
//!   different concept from the rules' radius-8 legal region
//!   ([`candidate_cells`], docs/decisions.md D-20).
//!
//! # Units
//!
//! Depth is measured internally in **plies** — a turn is two same-side plies
//! with the phase bit in the position key — and externally in **turns**
//! (docs/decisions.md D-9). Everything a caller sees is turns: the budget, the
//! reported depth, the mate distance, and the principal variation.

pub mod candidates;
pub mod error;
pub mod fallback;
pub mod info;
pub mod params;
pub mod score;
pub mod search;
// Public per docs/decisions.md D-353 (`U2_node_protocol.md` §5.35, U2-Z item
// 17): the one entry point the differential gate's expensive half drives from
// an integration test. A permanent surface commitment.
pub mod staged;
pub mod stop;
pub mod tt;

// The recursion and what it walks. Private because they are how the search
// works rather than what it offers: a caller holds a `Searcher` and a `Stop`,
// and everything below is free to change behind them.
pub(crate) mod ordering;
pub(crate) mod position;
pub(crate) mod pv;
pub(crate) mod pvs;
// Threat-only quiescence at the horizon (WP-1.6). Internal for the same
// reason `pvs` is: it is how `Run::visit`'s horizon works, not a surface a
// caller reaches directly.
pub(crate) mod quiescence;

pub use candidates::candidate_cells;
pub use error::SearchError;
pub use fallback::{FallbackAnswer, fallback_turn};
pub use info::{Provenance, SearchInfo, SearchOutcome, StageCounters};
pub use params::{CandidatePolicy, QTriggers, SearchParams, StagedParams};
pub use score::{MATE, ScoreKind};
pub use search::{MAX_DEPTH_TURNS, Searcher};
pub use stop::{NODE_CHECK_INTERVAL, Stop};
