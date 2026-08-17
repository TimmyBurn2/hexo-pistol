//! The `Engine` trait — the one seam the future API layer wraps.
//!
//! Three verbs, and they are the three docs/decisions.md D-2 pinned:
//! [`Engine::new_game`], [`Engine::set_position`] and [`Engine::go`]. The trait
//! is **synchronous and has no stop verb**: at a deployment budget of half a
//! second a stop verb buys nothing and would force async plumbing through the
//! one seam everything else adapts. Cancellation arrives with Lazy SMP in
//! Stage 4, as an additive trait extension and an additive protocol verb.
//!
//! The line protocol in pistol-cli mirrors this trait one to one (D-5), and
//! pistol-api stays empty until the API layer is specified (CLAUDE.md rule 11).
//! Anything a future transport needs, it needs from here.
//!
//! # Reporting
//!
//! [`Engine::go`] answers with a [`SearchOutcome`]: the move, and the evidence.
//! A return value cannot carry a *stream*, and the protocol prints one `info`
//! line per completed depth, so [`Engine::go_reporting`] is the same call with a
//! sink for those reports and `go` is a provided method over it. The split keeps
//! D-2's signature exactly as pinned while giving the protocol the seam it
//! needs; a caller that does not want the stream writes `go` and pays nothing.

use pistol_core::GameState;
use pistol_search::{SearchInfo, SearchOutcome};

use crate::budget::Budget;
use crate::config::EngineMode;
use crate::error::EngineError;
use crate::position::PositionSpec;

/// An engine that can be asked for a move.
///
/// Object safe on purpose: the protocol layer holds one of these without being
/// generic over it, which is what makes a second implementation — a stub for a
/// test, a remote engine for the arena — a drop-in rather than a rewrite.
pub trait Engine {
    /// How this engine trades reproducibility against strength.
    ///
    /// The protocol reports it, and it decides which budgets are honoured:
    /// instrument mode refuses a wall-clock budget by name
    /// (docs/decisions.md D-22). The enforcement lives in [`Engine::go`]'s
    /// implementation, not in the caller — a protocol that policed it would be
    /// a second copy of the policy.
    fn mode(&self) -> EngineMode;

    /// The position the engine is standing on.
    ///
    /// A shared borrow of the rules' own state, so a driver checks the game it
    /// is running against pistol-core rather than against a second opinion
    /// (CLAUDE.md rule 2).
    fn state(&self) -> &GameState;

    /// Start a new game: the initial position, and nothing remembered from the
    /// last one.
    ///
    /// Infallible, and total. Everything a new game needs to forget is
    /// forgotten here, which is what the determinism law requires of it: two
    /// searches of the same position, one of them after a different game and a
    /// `new_game`, must agree node for node (CLAUDE.md rule 4,
    /// docs/decisions.md D-7).
    fn new_game(&mut self);

    /// Stand on the position `spec` describes.
    ///
    /// The spec is replayed through the rules, so every refusal is a rules
    /// refusal ([`PositionSpec::replay`]). A position that is already won is
    /// refused: it is terminal, and there is no move to ask for.
    fn set_position(&mut self, spec: &PositionSpec) -> Result<(), EngineError>;

    /// Search the current position under `budget` and answer with a move.
    ///
    /// The budget is always explicit — there is no default budget anywhere in
    /// this workspace (CLAUDE.md rule 1) — and a budget this engine's mode
    /// cannot honour is refused by name rather than substituted.
    fn go(&mut self, budget: Budget) -> Result<SearchOutcome, EngineError> {
        self.go_reporting(budget, &mut |_| {})
    }

    /// [`Engine::go`], reporting once per completed depth.
    ///
    /// `report` sees the same [`SearchInfo`] the search produces at each
    /// completed iteration. The outcome's own report carries the last completed
    /// depth's line and score with the whole search's totals, which is not the
    /// same thing as the last report the sink saw (docs/decisions.md D-80).
    fn go_reporting(
        &mut self,
        budget: Budget,
        report: &mut dyn FnMut(&SearchInfo),
    ) -> Result<SearchOutcome, EngineError>;
}
