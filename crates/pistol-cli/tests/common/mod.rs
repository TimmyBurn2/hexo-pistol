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

/// The prefix EVERY scratch directory in this workspace's test suites carries,
/// and the only thing the sweep below is allowed to remove.
///
/// It is not `pistol-`. That prefix is the WORKSPACE'S OWN NAMING SCHEME: every
/// crate directory — `pistol-core`, `pistol-cli`, `pistol-eval`, `pistol-search`
/// — starts with it, so a `TMPDIR` pointed anywhere near a checkout made the
/// sweep delete source directories it never created, from a test that PASSED
/// (REPRODUCED: one `cargo test -p pistol-cli` removed four of them). A sweep
/// that removes by prefix must own the prefix, and `pistol-testscratch-` is a
/// name nothing but these suites writes.
///
/// `crates/pistol-arena/tests/common/mod.rs` spells the same prefix: the two
/// suites share one temp directory, and this sweep is the only thing that
/// removes what the arena's `Drop` guard could not (a killed or aborting test
/// binary runs no destructor).
pub const SCRATCH_PREFIX: &str = "pistol-testscratch-";

/// How old a scratch directory must be before the sweep may remove it. Generous:
/// the longest `cargo test --workspace` on this project is minutes, so nothing
/// this old belongs to a process that is still going.
pub const SCRATCH_STALE_AFTER: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);

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
    ONCE.call_once(|| sweep_scratch_in(&std::env::temp_dir(), SCRATCH_STALE_AFTER));
}

/// The sweep itself, over a named directory: the destructive half, separated so
/// a test can watch it spare what it must spare. Nothing else may call it — the
/// `Once` above is what keeps it to one pass per process.
pub fn sweep_scratch_in(dir: &std::path::Path, stale_after: std::time::Duration) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        if !name.to_string_lossy().starts_with(SCRATCH_PREFIX) {
            continue;
        }
        let stale = entry
            .metadata()
            .ok()
            .filter(std::fs::Metadata::is_dir)
            .and_then(|meta| meta.modified().ok())
            .and_then(|when| when.elapsed().ok())
            .is_some_and(|age| age > stale_after);
        if stale {
            let _ = std::fs::remove_dir_all(entry.path());
        }
    }
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
    let path = std::env::temp_dir().join(format!(
        "{SCRATCH_PREFIX}{}-{name}-{serial}",
        std::process::id()
    ));
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
