//! The tactical fixture: positions, what the engine is given, and what it must
//! find.
//!
//! The format's types live here and its loader in [`crate::fixture_loader`], which
//! is the same file split for size (CLAUDE.md rule 9) rather than a second
//! concept.
//!
//! One loader, in the library, read by both the `selftest` command and the tests
//! (CLAUDE.md rule 7 wants the suite sha-pinned; the pin fires in the test that
//! reads the file, docs/decisions.md D-37). Two loaders for one format would
//! drift, and the one in the binary would be the one nobody pinned.
//!
//! The format is line-oriented and the loader is strict: a directive it does not
//! know, a case without a position, a case without an expectation, a repeated
//! name — each is a named refusal citing the line. A fixture that is quietly
//! half-read reports a pass for cases nobody ran (CLAUDE.md rule 3).
//!
//! ```text
//! require <n>                 the pre-registered pass threshold, once
//! case <name>                 starts a case
//! config <path>               the engine config this case is a claim about
//! position <tail>             the position, as the `position` verb spells it
//! budget depth_turns <n> | budget nodes <n>
//! expect move <turn>          the engine plays exactly this turn
//! expect cell <q,r>           the turn it plays includes this cell
//! expect mate <n>             the score is a win in n turns
//! expect not-mated            the score is not a loss
//! ```

use std::fmt;
use std::path::PathBuf;

use pistol_core::{Coord, Turn};
use pistol_engine::{Budget, PositionSpec, ScoreKind, classify};

/// The pre-registered pass threshold.
pub const REQUIRE: &str = "require";
/// The engine config a case is a claim about.
pub const CONFIG: &str = "config";
/// The start of a case.
pub const CASE: &str = "case";
/// The case's position, as a `position` verb tail.
pub const POSITION: &str = "position";
/// The case's budget.
pub const BUDGET: &str = "budget";
/// One thing the engine must do.
pub const EXPECT: &str = "expect";

/// A loaded suite: its cases, and how many of them must pass.
#[derive(Debug, Clone)]
pub struct Suite {
    /// How many cases must pass, as pre-registered in the fixture header before
    /// the suite was first run (CLAUDE.md §Process: no post-hoc threshold
    /// moves).
    pub required: usize,
    /// The cases, in fixture order.
    pub cases: Vec<Case>,
}

impl Suite {
    /// Every config the suite names, in first-appearance order.
    pub fn configs(&self) -> Vec<PathBuf> {
        let mut seen: Vec<PathBuf> = Vec::new();
        for case in &self.cases {
            if !seen.contains(&case.config) {
                seen.push(case.config.clone());
            }
        }
        seen
    }
}

/// One tactical case.
#[derive(Debug, Clone)]
pub struct Case {
    /// The name, as written in the fixture.
    pub name: String,
    /// The fixture line the case starts on.
    pub line: usize,
    /// The engine config this case is a claim about, as written in the fixture
    /// and resolved against the working directory.
    ///
    /// Stated per case, and required: a result means nothing without the search
    /// that produced it, and the same position at a narrower candidate radius is
    /// a different claim (CLAUDE.md rule 6). There is no default — a case that
    /// named no config would be a claim about whatever the runner happened to
    /// pass (rule 1).
    pub config: PathBuf,
    /// The position to search.
    pub position: PositionSpec,
    /// What the engine is given to search it with.
    pub budget: Budget,
    /// Everything that must hold of the answer.
    pub expect: Vec<Expectation>,
}

/// One thing a case demands of the answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expectation {
    /// The engine plays exactly this turn. For a position with one winning move
    /// this is the strongest statement available; where several moves win, a
    /// case states [`Expectation::Cell`] or a score instead.
    Move(Turn),
    /// The turn the engine plays puts a stone on this cell — the form a block
    /// takes, where the second stone is free.
    Cell(Coord),
    /// The score says the side to move completes a line this many turns from
    /// here. Every turn from the root counts, both sides', so a win for the
    /// mover is an odd number (docs/decisions.md D-72).
    MateIn(u16),
    /// The score is not a loss: whatever the engine played, it was not already
    /// lost by force within the depth it reached.
    NotMated,
}

impl Expectation {
    /// Why this expectation was not met, if it was not.
    pub fn unmet(self, best: Turn, score: i32) -> Option<String> {
        match self {
            Expectation::Move(turn) if best != turn => {
                Some(format!("expected move {turn}, played {best}"))
            }
            Expectation::Cell(cell) if !plays(best, cell) => {
                Some(format!("expected a stone on {cell}, played {best}"))
            }
            Expectation::MateIn(turns) if classify(score) != ScoreKind::MateIn(turns) => {
                Some(format!(
                    "expected mate in {turns} turns, scored {:?}",
                    classify(score)
                ))
            }
            Expectation::NotMated if matches!(classify(score), ScoreKind::MatedIn(_)) => Some(
                format!("expected not to be lost, scored {:?}", classify(score)),
            ),
            _ => None,
        }
    }
}

/// Whether a turn puts a stone on a cell.
fn plays(turn: Turn, cell: Coord) -> bool {
    turn.first() == cell || turn.second() == Some(cell)
}

impl fmt::Display for Expectation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expectation::Move(turn) => write!(f, "move {turn}"),
            Expectation::Cell(cell) => write!(f, "cell {cell}"),
            Expectation::MateIn(turns) => write!(f, "mate {turns}"),
            Expectation::NotMated => f.write_str("not-mated"),
        }
    }
}

/// A fixture that could not be read as one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureError {
    /// The file it was read from.
    pub path: PathBuf,
    /// The line it went wrong on, if it went wrong on one.
    pub line: Option<usize>,
    /// What is wrong.
    pub why: String,
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}:{line}: {}", self.path.display(), self.why),
            None => write!(f, "{}: {}", self.path.display(), self.why),
        }
    }
}

impl std::error::Error for FixtureError {}
