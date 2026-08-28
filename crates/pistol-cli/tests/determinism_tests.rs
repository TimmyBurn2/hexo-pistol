mod common;

use std::process::{Command, Stdio};

use common::{GATE, repo, repo_root};
use pistol_cli::fixture_loader;
use pistol_cli::report::{NPS_FIELD, TIME_FIELD};

/// The budget both runs are given. Reproducible, and small enough that a debug
/// build finishes: the point is agreement, not depth.
const BUDGET: &str = "go nodes 2048";

/// The fields two runs may differ on. Everything else is compared.
///
/// Taken from the module that writes them rather than spelled again here: the
/// field names are the protocol's contract, and a rename has to break one thing,
/// not silently pass a test that was normalizing a name nobody writes any more.
/// `tools/determinism.sh` has to restate them in a `sed`, which is the one copy
/// no Rust import can remove.
const MEASURED_FIELDS: [&str; 2] = [NPS_FIELD, TIME_FIELD];

#[test]
fn determinism_two_process_runs_identical() {
    let positions = positions();
    let mut script = String::new();
    for position in &positions {
        script.push_str(&format!("newgame\nposition {position}\n{BUDGET}\n"));
    }
    script.push_str("quit\n");

    let first = run(&script);
    let second = run(&script);

    // A refused run would make two transcripts agree and prove nothing, so what
    // was produced is checked before it is compared.
    assert!(
        !first.iter().any(|line| line.starts_with("error ")),
        "the engine refused something: {first:#?}"
    );
    assert_eq!(
        first
            .iter()
            .filter(|line| line.starts_with("bestmove "))
            .count(),
        positions.len(),
        "one answer per position: {first:#?}"
    );
    assert_eq!(
        first
            .iter()
            .filter(|line| line.starts_with("info totals depth_turns "))
            .count(),
        positions.len(),
        "one totals line per position: {first:#?}"
    );

    assert_eq!(
        first.len(),
        second.len(),
        "two runs of the same input answer with the same number of lines"
    );
    for (index, (left, right)) in first.iter().zip(second.iter()).enumerate() {
        assert_eq!(
            left, right,
            "two processes disagreed on line {index} of the transcript"
        );
    }
}

/// The positions this test uses, from the sha-pinned fixture.
///
/// Not all of them: the cases whose answer comes from a search that runs to its
/// budget, plus one whose answer is a mate found at the first depth, so that both
/// the interrupted path and the early-exit path are compared. The rest are covered
/// by `tools/determinism.sh`, which runs every position in release.
fn positions() -> Vec<String> {
    let suite = fixture_loader::load(&repo("crates/pistol-cli/tests/fixtures/tactical_v0.txt"))
        .expect("the committed tactical fixture must load");
    let chosen: Vec<String> = suite
        .cases
        .iter()
        .filter(|case| {
            case.name.starts_with("quiet")
                || case.name.starts_with("must_block")
                || case.name == "mate_in_3_double_three_becomes_double_four"
        })
        .map(|case| case.position.to_string())
        .collect();
    assert!(
        chosen.len() >= 6,
        "the fixture should still hold the cases this test names, found {}",
        chosen.len()
    );
    chosen
}

/// One run of the compiled binary, as its normalized answer lines.
fn run(script: &str) -> Vec<String> {
    let binary = env!("CARGO_BIN_EXE_pistol");
    // The working directory is the repository root, because a config names its
    // weights file relative to the working directory and `cargo test` runs in the
    // package directory (see `common`).
    let child = Command::new(binary)
        .arg("--config")
        .arg(GATE)
        .current_dir(repo_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("cannot run {binary}: {error}"));

    use std::io::Write;
    let mut child = child;
    child
        .stdin
        .as_mut()
        .expect("a piped stdin")
        .write_all(script.as_bytes())
        .expect("the engine reads its input");
    let output = child.wait_with_output().expect("the engine exits");
    assert!(
        output.status.success(),
        "the engine exited with {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("the protocol is text")
        .lines()
        .map(normalize)
        .collect()
}

/// One line with the machine's measurements dropped.
///
/// `nps` and `time` measure how fast this run happened to be; the move, the node
/// count, the score, the depth, the table occupancy and the whole principal
/// variation do not, and all of those are compared.
fn normalize(line: &str) -> String {
    let mut kept: Vec<&str> = Vec::new();
    let mut words = line.split_whitespace();
    while let Some(word) = words.next() {
        if MEASURED_FIELDS.contains(&word) {
            words.next();
            continue;
        }
        kept.push(word);
    }
    kept.join(" ")
}
