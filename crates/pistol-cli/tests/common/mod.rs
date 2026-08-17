//! Shared test scaffolding: the committed configs, an engine built from one, and
//! a way to say lines to it.
//!
//! # Why the weights path is rewritten here
//!
//! A config names its weights file relative to the working directory, which is
//! what `tools/config_check.sh` and the engine binary both rely on (they run from
//! the repository root, and docs/decisions.md D-21 keeps `Config::validate` from
//! touching the filesystem at all). A cargo test's working directory is its own
//! package, and a test cannot change it — the process is shared with every other
//! test in the binary. So the absolute path is stated here, once, rather than
//! depending on where cargo was invoked from.
#![allow(dead_code)] // each test binary uses a subset of these helpers.

pub mod sha256;

use std::path::PathBuf;

use pistol_cli::Session;
use pistol_engine::{Config, Engine, Pistol};

/// The repository root, from this package's location.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root is two directories up from this package")
}

/// A path under the repository root.
pub fn repo(relative: &str) -> PathBuf {
    repo_root().join(relative)
}

/// The committed instrument-mode config: the one every strength claim uses.
pub const INSTRUMENT: &str = "configs/instrument_v0.toml";
/// The committed gate config: narrow on purpose, and fast enough for a test.
pub const GATE: &str = "configs/gate_v0.toml";
/// The committed play-mode config: the one that honours a wall-clock budget.
pub const PLAY: &str = "configs/play_v0.toml";

/// One of the committed configs, loaded and validated, with its weights path
/// made absolute.
pub fn committed(relative: &str) -> Config {
    let path = repo(relative);
    let mut config = Config::load(&path).unwrap_or_else(|error| {
        panic!("the committed config {} must load: {error}", path.display())
    });
    config.eval.weights_file = repo_root().join(&config.eval.weights_file);
    config
}

/// An engine built from one of the committed configs.
pub fn engine(relative: &str) -> Pistol {
    Pistol::from_config(committed(relative))
        .unwrap_or_else(|error| panic!("the committed config {relative} must build: {error}"))
}

/// Say these lines to an engine, and collect every line it answers with.
pub fn talk(engine: &mut dyn Engine, lines: &[&str]) -> Vec<String> {
    let mut session = Session::new(engine);
    let mut answers = Vec::new();
    for line in lines {
        session.line(line, &mut |answer| answers.push(answer.to_string()));
    }
    answers
}

/// The one answer whose first word is `word`, or a panic naming what was said.
pub fn only_line(answers: &[String], word: &str) -> String {
    let mut found = answers
        .iter()
        .filter(|line| line.split_whitespace().next() == Some(word));
    let Some(line) = found.next() else {
        panic!("no `{word}` line among:\n{}", answers.join("\n"));
    };
    assert!(
        found.next().is_none(),
        "more than one `{word}` line among:\n{}",
        answers.join("\n")
    );
    line.clone()
}

/// Whether any answer starts with `word`.
pub fn has_line(answers: &[String], word: &str) -> bool {
    answers
        .iter()
        .any(|line| line.split_whitespace().next() == Some(word))
}
