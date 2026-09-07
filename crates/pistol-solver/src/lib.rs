//! `pistol-solver` — forcing-sequence machinery: the threat state and the
//! twelve queries a forcing search asks of it, then TSS/DBS and the df-pn
//! family along the staged plan in docs/research/minimax_report.md.
//!
//! **Nothing in this crate decides a game.** The theorem it serves —
//! docs/decisions.md D-243 — is stones-remaining conditioned and side-to-move
//! conditioned, and both conditions live in the CALLER:
//! [`ThreatState::can_win_this_turn`] takes the stones left as an argument
//! rather than guessing, and [`ThreatState::unblockable_double_threat`] is a
//! statement about hitting sets that becomes one about the game only under the
//! two conditions its own doc names. A primitive folding them in would return a
//! mate score for the losing side in exactly the position where it matters.
//!
//! It composes `pistol-core` and re-implements no rule (CLAUDE.md rule 2), and
//! takes one dependency and no dev-dependencies.

pub mod config;
pub mod cover;
pub mod dfpn;
pub mod fixture;
pub mod pn;
pub mod policy;
pub mod query;
pub mod solver;
pub mod state;
pub mod tt;
pub mod zone;

mod line;
mod sets;
mod table;

pub use config::{
    AttackerPolicy, SOLVER_SCHEMA_VERSION, SUPPORTED_FREE_STONE_RADIUS, SUPPORTED_ZONE_ORDERS,
    SolverConfigError, SolverConfigFile, SolverParams, SolverSection,
};
pub use cover::{Cover, MinimalCover};
pub use dfpn::{ProofDag, ProofKind, ProofNode, SearchStats};
pub use fixture::{Expectation, FixtureCase, FixtureError};
pub use pn::{Epsilon, INF, Value, value_of};
pub use query::{HitBudget, LiveCount, NearHot, StonesLeft, WinWitness};
pub use solver::{
    EmittedNode, ProofTree, SolveOutcome, SolveResult, Solver, UNCAPPED, WRONG_POSITION,
};
pub use state::{THREAT_DESYNC, ThreatState};
pub use table::WindowMasks;
pub use zone::{ZONE_ORDERS, ZoneP, ep1_contribution, t31_contribution};
