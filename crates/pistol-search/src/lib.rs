//! `pistol-search` — the classical search.
//!
//! This crate will own principal variation search with iterative deepening, the
//! transposition table, move ordering, threat-only quiescence, budget handling
//! and `SearchInfo`. No MCTS.
//!
//! Two invariants constrain everything added here:
//!
//! - the determinism law (CLAUDE.md rule 4) — in instrument mode nothing
//!   nondeterministic may influence move choice, so no unseeded hash iteration
//!   on a choice path, no time-based tie-breaks, no thread races;
//! - the search candidate policy is config, never a literal, and it is a
//!   different concept from the rules' radius-8 legal region.
//!
//! WP-01 is workspace scaffold, config, errors and CI; it writes no search.
