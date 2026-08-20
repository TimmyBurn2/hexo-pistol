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

/// Remove scratch directories left behind by EARLIER test processes.
///
/// Nothing ever cleaned these. A `Drop` guard is the wrong shape — a failing
/// test's output is the first thing anybody wants to look at — and a test binary
/// has no reliable teardown hook, so the directories simply accumulated: on the
/// machine this was found on, thousands of them, on a `/tmp` that is RAM, which
/// is the very hazard `tools/bench_delta.sh`'s own comment warns about
/// (docs/decisions.md D-234). Swept here instead: once per process, at the front
/// of the only function that makes one, and only for entries old enough that no
/// live run can own one. Errors are ignored throughout — a sweep that failed a
/// test would be worse than the litter it removes.
fn sweep_stale_scratch() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        // Generous: the longest `cargo test --workspace` on this project is
        // minutes, so nothing this old belongs to a process that is still going.
        const STALE_AFTER: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);
        let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) else {
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            // Both suites' scratch names begin with this, and nothing else in a
            // temp directory has any business doing so.
            if !name.to_string_lossy().starts_with("pistol-") {
                continue;
            }
            let stale = entry
                .metadata()
                .ok()
                .filter(std::fs::Metadata::is_dir)
                .and_then(|meta| meta.modified().ok())
                .and_then(|when| when.elapsed().ok())
                .is_some_and(|age| age > STALE_AFTER);
            if stale {
                let _ = std::fs::remove_dir_all(entry.path());
            }
        }
    });
}

/// A fresh, empty directory for a test to write into, named after the test.
///
/// Under the system temp directory rather than the repository, so nothing a test
/// writes can reach the git index — an output that leaked into the tree would be
/// an artifact nobody committed on purpose (CLAUDE.md rule 8).
pub fn scratch(name: &str) -> PathBuf {
    sweep_stale_scratch();
    // Tests in one binary share a process and run in parallel, so the process id
    // alone is not unique: two calls with the same name would race, one removing
    // the directory the other is writing into.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("pistol-{}-{name}-{serial}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path)
        .unwrap_or_else(|error| panic!("cannot create {}: {error}", path.display()));
    path
}

/// Write `text` to a file in a fresh scratch directory and return its path.
pub fn scratch_file(name: &str, file: &str, text: &str) -> PathBuf {
    let path = scratch(name).join(file);
    std::fs::write(&path, text)
        .unwrap_or_else(|error| panic!("cannot write {}: {error}", path.display()));
    path
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
