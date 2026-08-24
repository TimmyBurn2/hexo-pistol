//! The composition root: one engine, built from one config.
//!
//! Everything this workspace has is wired together here and nowhere else — the
//! rules from pistol-core, an evaluation backend from pistol-eval, the search
//! from pistol-search — so that a change of backend, of candidate policy or of
//! table size is a change to a document rather than to code (CLAUDE.md rule 1).
//!
//! Two translations live here, and they live here on purpose:
//!
//! - **config to parts.** Which eval backend exists, and what a `Searcher` is
//!   built with. A value that reaches the search came from a validated document.
//! - **`Budget` to `Stop`.** Which budgets a mode may use is operator-facing
//!   policy and stays in the engine (docs/decisions.md D-22); *stopping* is the
//!   search's. The translation is not a copy: a wall-clock budget becomes the
//!   instant it expires at, so the one clock read on the way into a search
//!   happens here and the search holds no path that computes a deadline of its
//!   own (D-73, CLAUDE.md rule 4).

use std::time::{Duration, Instant};

use pistol_core::GameState;
use pistol_eval::{Eval, HandcraftedV0, Weights};
use pistol_search::{
    CandidatePolicy as SearchCandidatePolicy, SearchError, SearchInfo, SearchOutcome, SearchParams,
    Searcher, Stop,
};

use crate::budget::Budget;
use crate::config::{CandidatePolicy, Config, EngineMode, EvalBackend, TieBreak};
use crate::engine::Engine;
use crate::error::EngineError;
use crate::position::PositionSpec;

/// pistol, assembled.
pub struct Pistol {
    config: Config,
    searcher: Searcher,
    state: GameState,
}

impl Pistol {
    /// Build an engine from a configuration.
    ///
    /// The config is validated again here even though [`Config::load`] already
    /// did: validation is pure and idempotent, and a caller that parsed with
    /// `parse_unvalidated` and skipped it would otherwise reach a search with
    /// values nobody checked (docs/decisions.md D-17).
    ///
    /// The weights file is opened now, and a missing or malformed one is a named
    /// [`EngineError::Config`] against `eval.weights_file` — the loud load-time
    /// half of D-21. Relative paths resolve against the process's working
    /// directory, exactly as the operator wrote them.
    pub fn from_config(config: Config) -> Result<Pistol, EngineError> {
        config.validate()?;
        check_consumed(&config)?;
        let eval = build_eval(&config)?;
        let params = SearchParams {
            tt_bytes: config.search.tt_bytes,
            candidate_policy: search_policy(&config.search.candidate_policy),
        };
        let searcher = Searcher::new(params, eval).map_err(from_search)?;
        Ok(Pistol {
            config,
            searcher,
            state: GameState::new_game(),
        })
    }

    /// The configuration this engine was built from.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// How many bytes the transposition table took, which is at most
    /// `search.tt_bytes` and generally less (docs/decisions.md D-75).
    pub fn table_bytes(&self) -> u64 {
        self.searcher.table_bytes()
    }
}

impl Engine for Pistol {
    fn mode(&self) -> EngineMode {
        self.config.engine.mode
    }

    fn state(&self) -> &GameState {
        &self.state
    }

    fn new_game(&mut self) {
        self.state = GameState::new_game();
        self.searcher.clear();
    }

    fn set_position(&mut self, spec: &PositionSpec) -> Result<(), EngineError> {
        // Replayed first, and only then adopted: a refused position leaves the
        // engine standing where it was, rather than half way into a document it
        // rejected (CLAUDE.md rule 3).
        self.state = spec.replay()?;
        Ok(())
    }

    fn go_reporting(
        &mut self,
        budget: Budget,
        report: &mut dyn FnMut(&SearchInfo),
    ) -> Result<SearchOutcome, EngineError> {
        let budget = Budget::resolve(Some(budget), self.config.engine.mode)?;
        let stop = stop_for(budget)?;
        self.searcher
            .search(&self.state, stop, report)
            .map_err(from_search)
    }
}

/// Every key this build cannot honour, refused before a search is built.
///
/// A config key that no code reads is an instruction that silently did nothing,
/// which is the failure mode CLAUDE.md rule 3 exists to prevent. Two keys need
/// saying so explicitly:
///
/// - `instrument.threads` — this build searches single-threaded whatever the mode
///   says. `Config::validate` already pins it to 1 in instrument mode (rule 4);
///   in play mode a larger number would simply be ignored, so it is refused with
///   a reason that names the stage it arrives in.
/// - `instrument.tie_break` — the search's move ordering already breaks ties by
///   coordinate (docs/decisions.md D-7), so the one variant needs no wiring. The
///   exhaustive match is what makes a *second* variant a compile error here
///   rather than a document that loads and is ignored.
fn check_consumed(config: &Config) -> Result<(), EngineError> {
    if config.instrument.threads != 1 {
        return Err(EngineError::config(
            "instrument.threads",
            format!(
                "this build searches single-threaded in every mode; Lazy SMP arrives in \
                 Stage 4 (docs/ROADMAP.md) — must be 1, got {}",
                config.instrument.threads
            ),
        ));
    }
    match config.instrument.tie_break {
        TieBreak::Lexicographic => Ok(()),
    }
}

/// The evaluation backend the config names, with its weights loaded.
///
/// The match is exhaustive over [`EvalBackend`] so that adding a backend is a
/// compile error here rather than a document that loads and evaluates with the
/// wrong table.
fn build_eval(config: &Config) -> Result<Box<dyn Eval>, EngineError> {
    let path = &config.eval.weights_file;
    match config.eval.backend {
        EvalBackend::HandcraftedV0 => {
            let weights = Weights::load(path)
                .map_err(|error| EngineError::config("eval.weights_file", error.to_string()))?;
            Ok(Box::new(HandcraftedV0::new(weights)))
        }
    }
}

/// The search's candidate policy, from the config's.
///
/// Two enums, and they are deliberately different types: one is a document's
/// vocabulary and the other is a search's. The radii are never compared with
/// the rules' radius-8 legal region (CLAUDE.md rule 2, docs/decisions.md
/// D-20). Under `Staged`, `quiet_top_k` and `widen_schedule` are validated at
/// the config layer (`validate.rs`) for schema completeness against
/// `U3_tier_t.md` §10 and go no further — `pistol_search::StagedParams`
/// deliberately does not carry them, because this D-scope's search does not
/// arm stage Q's widening schedule (docs/decisions.md D-353).
fn search_policy(policy: &CandidatePolicy) -> SearchCandidatePolicy {
    match policy {
        CandidatePolicy::Radius { radius } => SearchCandidatePolicy::Radius { radius: *radius },
        CandidatePolicy::Staged {
            quiet_radius,
            tier_t_own_count,
            tier_t_opponent_count,
            q_depth_turns,
            quiet_top_k: _,
            widen_schedule: _,
        } => SearchCandidatePolicy::Staged(pistol_search::StagedParams {
            quiet_radius: *quiet_radius,
            tier_t_own_count: *tier_t_own_count,
            tier_t_opponent_count: *tier_t_opponent_count,
            q_depth_turns: *q_depth_turns,
        }),
    }
}

/// The stop condition a budget becomes.
fn stop_for(budget: Budget) -> Result<Stop, EngineError> {
    Ok(match budget {
        Budget::DepthTurns(turns) => Stop::DepthTurns(turns),
        Budget::Nodes(nodes) => Stop::Nodes(nodes),
        Budget::MovetimeMs(millis) => {
            let deadline = Instant::now()
                .checked_add(Duration::from_millis(millis))
                .ok_or_else(|| {
                    EngineError::config(
                        budget.key(),
                        format!("{millis} ms from now is not a time this machine can represent"),
                    )
                })?;
            Stop::Deadline(deadline)
        }
    })
}

/// A search refusal, as the engine reports it.
///
/// Each one lands on the variant an operator can act on: a parameter names the
/// config key that set it (docs/decisions.md D-10, D-24), a depth past the
/// horizon names the budget key that asked for it, a candidate policy that
/// offers nothing names `search.candidate_policy` itself — policy-agnostic,
/// since `SearchError::NoCandidates` carries the whole policy and not a bare
/// radius (docs/decisions.md D-353, U2-Z item 8) — and a root half way
/// through a turn is a position with nothing to search rather than an
/// illegal one.
///
/// A decided root is the one case that cannot be reached: `set_position` refuses
/// a won position and a new game is not one, so the search seeing a decided root
/// would mean this crate let one through. It is reported as the internal
/// invariant it would be, which keeps the reachable error set honest.
fn from_search(error: SearchError) -> EngineError {
    match error {
        SearchError::Params { key, why } => EngineError::config(key, why),
        other @ SearchError::TurnInProgress { .. } => {
            EngineError::not_searchable(other.to_string())
        }
        other @ SearchError::GameDecided { .. } => EngineError::internal(format!(
            "the engine was standing on a decided position, which set_position refuses: {other}"
        )),
        other @ SearchError::DepthOutOfRange { .. } => {
            EngineError::config("budget.depth_turns", other.to_string())
        }
        other @ SearchError::NoCandidates { .. } => {
            EngineError::config("search.candidate_policy", other.to_string())
        }
    }
}
