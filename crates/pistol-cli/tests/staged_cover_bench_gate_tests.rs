mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// A stub that answers the `pistol` handshake with an `id` line and every
/// `go` command with a well-formed `totals` line — enough for
/// `staged_cover_bench.sh`'s own awk parser and IQR gate, and constant so the
/// bench's own noise gate never fires.
const ANSWERS: &str = r#"
use std::io::{BufRead, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_default();
        if line == "pistol" {
            writeln!(out, "id name stub").unwrap();
        } else if line.starts_with("go ") {
            writeln!(out, "info totals nodes 1000 time 10").unwrap();
        } else if line == "quit" {
            break;
        }
        out.flush().unwrap();
    }
}
"#;

/// The same stub with the `pistol` handshake dropped — silence where an
/// `id ` line belongs, the shape a rebuilt-but-incompatible engine takes.
const NO_HANDSHAKE: &str = r#"
use std::io::{BufRead, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_default();
        if line.starts_with("go ") {
            writeln!(out, "info totals nodes 1000 time 10").unwrap();
        } else if line == "quit" {
            break;
        }
        out.flush().unwrap();
    }
}
"#;

const MANIFEST: &str = r#"[workspace]

[package]
name = "pistol-cover-bench-stub"
version = "0.0.0"
edition = "2021"

[[bin]]
name = "pistol"
path = "src/main.rs"
"#;

/// A two-entry fixture, one per band (`EARLY_MAX` is 17 in the shipped
/// script): the shell script itself parses this file's `stones` field, so
/// unlike `tactical_check_gate_tests.rs`'s stub fixture (which the ENGINE
/// reads and the stub ignores), this one's shape is load-bearing.
const FIXTURE: &str = "start moves 0,0 # src stub stones 1\nstart moves 0,0 # src stub stones 40\n";

fn stub(root: &Path, source: &str) {
    std::fs::write(root.join("src/main.rs"), source).expect("the stub engine writes");
}

fn scratch_tree(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    for dir in [
        "tools",
        "src",
        "configs",
        "crates/pistol-cli/tests/fixtures",
    ] {
        std::fs::create_dir_all(root.join(dir)).expect("the scratch tree is created");
    }
    for script in ["staged_cover_bench.sh", "scratch_preflight.sh"] {
        std::fs::copy(
            repo(&format!("tools/{script}")),
            root.join("tools").join(script),
        )
        .unwrap_or_else(|error| panic!("the shipped {script} copies: {error}"));
    }
    std::fs::write(
        root.join("configs/instrument_staged_v0.toml"),
        "# the gate stats this path and hands it to the engine; the stub ignores it\n",
    )
    .expect("the stub config writes");
    std::fs::write(
        root.join("crates/pistol-cli/tests/fixtures/bench_positions_v1.txt"),
        FIXTURE,
    )
    .expect("the stub fixture writes");
    std::fs::write(root.join("Cargo.toml"), MANIFEST).expect("the stub manifest writes");
    stub(&root, ANSWERS);
    let ran = Command::new(env!("CARGO"))
        .current_dir(&root)
        .args(["generate-lockfile", "--offline", "-q"])
        .output()
        .expect("cargo runs");
    assert!(
        ran.status.success(),
        "the stub crate takes a lockfile: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    root
}

/// Run the SHIPPED script in `root`, at the pre-registered `REPS` floor —
/// same target-directory pinning `tactical_check_gate_tests.rs` uses, for the
/// same reason.
fn cover_bench(root: &Path, target_dir: Option<&Path>) -> Output {
    let mut command = Command::new("bash");
    command
        .arg(root.join("tools/staged_cover_bench.sh"))
        .arg("5")
        .current_dir(root)
        .env_remove("CARGO_BUILD_TARGET")
        .env("TMPDIR", root.parent().expect("scratch has a parent"));
    match target_dir {
        Some(dir) => command.env("CARGO_TARGET_DIR", dir),
        None => command.env("CARGO_TARGET_DIR", root.join("target")),
    };
    command.output().expect("the gate script runs")
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

#[test]
fn staged_cover_bench_runs_the_binary_cargo_built_and_not_a_stale_one_at_the_default_path() {
    let root = scratch_tree("staged-cover-bench-stale");

    // THE CONTROL RUN: nothing is wrong, and the bench has to complete —
    // otherwise the refusal below could come from a script that refuses
    // everything (`tools/SHELL_CHECKLIST.md`'s coverage rule names this
    // pass-for-the-wrong-reason trap).
    let control = cover_bench(&root, None);
    let seen = said(&control);
    assert!(
        control.status.success(),
        "the bench completes when nothing is wrong:\n{seen}"
    );
    assert!(
        seen.contains("staged_cover_bench: done"),
        "and it prints its own completion line:\n{seen}"
    );

    let stale = root.join("target/release/pistol");
    assert!(
        stale.is_file(),
        "the control build lands at the path the tactical_check.sh defect hardcoded: {}",
        stale.display()
    );

    // THE REGRESSION: cargo is told to build somewhere else, and the freshly
    // built engine no longer answers the handshake. The bench must report
    // THAT engine's refusal, not silently trust the stale one at the default
    // path (which still answers the handshake perfectly well).
    stub(&root, NO_HANDSHAKE);
    let alt = scratch("staged-cover-bench-stale-alt");
    let ran = cover_bench(&root, Some(&alt));
    let out = said(&ran);
    assert!(
        alt.join("release/pistol").is_file(),
        "cargo built into the redirected directory:\n{out}"
    );
    assert!(
        stale.is_file(),
        "and the stale binary is still sitting at the default path:\n{out}"
    );
    assert!(
        !ran.status.success(),
        "an engine that fails the handshake is a refusal, not a completed bench:\n{out}"
    );
    assert!(
        out.contains("answered no `id ` lines"),
        "named by the reason the handshake actually gave, not a stale pass:\n{out}"
    );
}

#[test]
fn staged_cover_bench_refuses_a_reps_argument_with_a_leading_zero() {
    let root = scratch_tree("staged-cover-bench-reps-spelling");
    let mut command = Command::new("bash");
    command
        .arg(root.join("tools/staged_cover_bench.sh"))
        .arg("010")
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .env("TMPDIR", root.parent().expect("scratch has a parent"));
    let ran = command.output().expect("the gate script runs");
    let out = said(&ran);
    assert!(
        !ran.status.success(),
        "REPS=010 is octal 8 to a numeric test but is not the plain decimal spelling this script now requires:\n{out}"
    );
    assert!(
        out.contains("REPS must be spelled as a plain decimal integer"),
        "named by the spelling refusal, not a generic parse error:\n{out}"
    );
}
