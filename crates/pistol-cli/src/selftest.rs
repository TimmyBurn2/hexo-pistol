//! The in-process gate: every fixture case, searched twice, against what it must
//! find.
//!
//! Two questions, and they are answered separately because they fail for
//! different reasons and carry different consequences:
//!
//! - **Determinism.** Two runs of the same position under the same reproducible
//!   budget must agree on the move, the node count, the score, the depth and the
//!   whole principal variation. That is a law, not a percentage: one
//!   disagreement fails the gate (CLAUDE.md rule 4, docs/decisions.md D-7). The
//!   two runs are deliberately unalike — one from a freshly built engine, one
//!   from an engine that has already played other games and been told `newgame`
//!   — because the failure this catches in a single process is state that bleeds
//!   from one search into the next.
//! - **Tactics.** How many cases the engine actually solves, against the
//!   threshold the fixture pre-registered before the suite was first run
//!   (CLAUDE.md §Process). The threshold is read from the fixture, never from a
//!   flag, and this module has no opinion about what it should be.
//!
//! The cross-process half of the determinism gate is `tools/determinism.sh`,
//! which runs the compiled binary twice and diffs the transcripts. This half
//! cannot see what that one sees (two processes really are two processes) and
//! that one cannot see what this one does (a single process's carried state), so
//! both exist.

use std::fmt;
use std::path::{Path, PathBuf};

use pistol_core::Turn;
use pistol_engine::{Config, Engine, EngineError, Pistol, SearchOutcome};

use crate::fixtures::{Case, Suite};

/// What one case did.
#[derive(Debug, Clone)]
pub struct CaseReport {
    /// The case's name, from the fixture.
    pub name: String,
    /// Expectations the answer did not meet.
    pub tactical: Vec<String>,
    /// Ways the two runs disagreed. Any entry fails the gate.
    pub determinism: Vec<String>,
    /// The move the engine played.
    pub best: Turn,
    /// Nodes the whole search spent.
    pub nodes: u64,
    /// The last depth it completed, in turns.
    pub depth_turns: u32,
    /// The score of that depth, from the mover's point of view.
    pub score: i32,
    /// The line the score was proved on.
    pub pv: Vec<Turn>,
}

impl CaseReport {
    /// Whether this case is a pass: it met its expectations and reproduced.
    pub fn passed(&self) -> bool {
        self.tactical.is_empty() && self.determinism.is_empty()
    }
}

/// What the suite did.
#[derive(Debug, Clone)]
pub struct Report {
    /// The threshold the fixture pre-registered.
    pub required: usize,
    /// One report per case, in fixture order.
    pub cases: Vec<CaseReport>,
}

impl Report {
    /// How many cases met every expectation.
    pub fn tactical_passes(&self) -> usize {
        self.cases
            .iter()
            .filter(|case| case.tactical.is_empty())
            .count()
    }

    /// How many cases failed to reproduce. Any at all fails the gate.
    pub fn determinism_failures(&self) -> usize {
        self.cases
            .iter()
            .filter(|case| !case.determinism.is_empty())
            .count()
    }

    /// Whether the gate holds: nothing failed to reproduce, and at least the
    /// pre-registered number of cases solved.
    pub fn holds(&self) -> bool {
        self.determinism_failures() == 0 && self.tactical_passes() >= self.required
    }
}

impl fmt::Display for Report {
    /// One line per case, then the verdict. Written so that a failing gate says
    /// which case failed and how, without anyone having to re-run it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for case in &self.cases {
            let mark = if case.passed() { "ok  " } else { "FAIL" };
            writeln!(
                f,
                "{mark} {}  bestmove {} depth_turns {} nodes {} score {}",
                case.name, case.best, case.depth_turns, case.nodes, case.score
            )?;
            for why in case.tactical.iter().chain(case.determinism.iter()) {
                writeln!(f, "       {why}")?;
            }
            if !case.passed() {
                // The line the score was proved on: what a failing case was
                // actually thinking, without anyone having to re-run it.
                writeln!(f, "       pv {}", line_of_turns(&case.pv))?;
            }
        }
        write!(
            f,
            "selftest: {} of {} cases solved (required {}), {} failed to reproduce",
            self.tactical_passes(),
            self.cases.len(),
            self.required,
            self.determinism_failures(),
        )
    }
}

/// A case that could not be run at all.
///
/// Not a failed case: a position that does not replay, a budget the engine
/// refuses, or a config the suite names and the caller did not load means the
/// fixture or the config is broken, and reporting it as a tactical miss would be
/// the quiet kind of wrong (CLAUDE.md rule 3). It names the case and the fixture
/// line, because a twenty-case suite that says only "illegal position" leaves an
/// operator to guess which one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelftestError {
    /// The case that could not be run.
    pub case: String,
    /// The fixture line it starts on.
    pub line: usize,
    /// What the engine said.
    pub error: EngineError,
}

impl fmt::Display for SelftestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "case `{}` (fixture line {}) cannot be run: {}",
            self.case, self.line, self.error
        )
    }
}

impl std::error::Error for SelftestError {}

/// Run the whole suite.
///
/// Each case names the config it is a claim about, and `configs` supplies the
/// loaded document for each path the suite mentions. Cases are run in fixture
/// order, and the carried engine of a given config sees that config's cases in
/// that order — so a case's second answer is produced by an engine with a history
/// behind it, which is the point.
///
/// A case that cannot be *run* — a config the suite names but the caller did not
/// load, a position that does not replay, a budget the engine refuses — is an
/// error rather than a failed case: the fixture or the config is broken, and
/// reporting it as a tactical miss would be the quiet kind of wrong (CLAUDE.md
/// rule 3).
pub fn run(configs: &[(PathBuf, Config)], suite: &Suite) -> Result<Report, SelftestError> {
    let mut carried: Vec<(PathBuf, Pistol)> = Vec::new();
    for (path, config) in configs {
        let engine = Pistol::from_config(config.clone()).map_err(|error| SelftestError {
            case: format!("<the engine for {}>", path.display()),
            line: 0,
            error,
        })?;
        carried.push((path.clone(), engine));
    }
    let mut cases = Vec::with_capacity(suite.cases.len());

    for case in &suite.cases {
        let blame = |error: EngineError| SelftestError {
            case: case.name.clone(),
            line: case.line,
            error,
        };
        let config = named(configs, &case.config).map_err(blame)?;
        let mut fresh = Pistol::from_config(config.clone()).map_err(blame)?;
        let first = search(&mut fresh, case).map_err(blame)?;

        let engine = carried
            .iter_mut()
            .find(|(path, _)| *path == case.config)
            .map(|(_, engine)| engine)
            .expect("every config the suite names has a carried engine");
        engine.new_game();
        let second = search(engine, case).map_err(blame)?;

        cases.push(CaseReport {
            name: case.name.clone(),
            tactical: unmet(case, &first),
            determinism: disagreements(&first, &second),
            best: first.best,
            nodes: first.info.nodes,
            depth_turns: first.info.depth_turns,
            score: first.info.score,
            pv: first.info.pv.clone(),
        });
    }

    Ok(Report {
        required: suite.required,
        cases,
    })
}

/// The loaded config a case names.
fn named<'c>(configs: &'c [(PathBuf, Config)], path: &Path) -> Result<&'c Config, EngineError> {
    configs
        .iter()
        .find(|(named, _)| named == path)
        .map(|(_, config)| config)
        .ok_or_else(|| {
            EngineError::config(
                "config",
                format!(
                    "the fixture names {} and it was not loaded: every config a case names has \
                     to be read before the suite runs",
                    path.display()
                ),
            )
        })
}

/// One case on one engine.
fn search(engine: &mut dyn Engine, case: &Case) -> Result<SearchOutcome, EngineError> {
    engine.set_position(&case.position)?;
    engine.go(case.budget)
}

/// The case's expectations that the answer did not meet.
fn unmet(case: &Case, outcome: &SearchOutcome) -> Vec<String> {
    case.expect
        .iter()
        .filter_map(|expectation| expectation.unmet(outcome.best, outcome.info.score))
        .collect()
}

/// Everything two runs of one position must agree on, and did not.
///
/// Time and nodes per second are deliberately absent: they measure the machine,
/// not the search (docs/decisions.md D-7).
fn disagreements(first: &SearchOutcome, second: &SearchOutcome) -> Vec<String> {
    let mut found = Vec::new();
    let mut differ = |what: &str, left: String, right: String| {
        if left != right {
            found.push(format!(
                "a fresh engine and one that has played disagree on {what}: {left} vs {right}"
            ));
        }
    };
    differ("bestmove", first.best.to_string(), second.best.to_string());
    differ(
        "nodes",
        first.info.nodes.to_string(),
        second.info.nodes.to_string(),
    );
    differ(
        "depth_turns",
        first.info.depth_turns.to_string(),
        second.info.depth_turns.to_string(),
    );
    differ(
        "score",
        first.info.score.to_string(),
        second.info.score.to_string(),
    );
    differ("pv", line_of(first), line_of(second));
    differ(
        "hashfull",
        first.info.hashfull_permille.to_string(),
        second.info.hashfull_permille.to_string(),
    );
    found
}

/// A principal variation as one string, for comparing and for reporting.
fn line_of(outcome: &SearchOutcome) -> String {
    line_of_turns(&outcome.info.pv)
}

/// The same, for a line already in hand.
fn line_of_turns(pv: &[Turn]) -> String {
    pv.iter()
        .map(Turn::to_string)
        .collect::<Vec<String>>()
        .join(" ")
}
