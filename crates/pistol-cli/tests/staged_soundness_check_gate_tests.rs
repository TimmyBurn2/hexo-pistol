//! `tools/staged_soundness_check.sh` part 1 — CI gate 11 of 16
//! (`tools/ci.sh:130`), the D-316 soundness gate's own artifact-resolution
//! ladder (`docs/decisions.md` D-361; WP-1.5b Phase 4's MAJOR 4 finding,
//! `docs/experiments/wp15b_IMPL_REVIEW_REDTEAM.md`).
//!
//! # Why this file exists at all
//!
//! D-361 exempted this script from `tools/SHELL_CHECKLIST.md` item 10 on the
//! reasoning that `tools/ci.sh` is its precedent. The Phase 4 review found the
//! actual precedent is `tools/tactical_check.sh`, whose artifact-resolution
//! ladder this script's part 1 copies nearly line for line — and that script
//! DOES carry a driving test
//! (`crates/pistol-cli/tests/tactical_check_gate_tests.rs`), added after a
//! stale binary at a redirected `CARGO_TARGET_DIR` let a gate print
//! `20 of 20` and exit 0 for an engine that had just been rebuilt failing the
//! suite (D-250). This file is that same test, pointed at this script.
//!
//! # Scope
//!
//! Part 1 only. Parts 2-4 are `cargo test -p pistol-search --test ...`
//! invocations of suites this scratch tree does not vendor, and they are
//! never reached in either scenario below: the control run's stub answers
//! part 1's `selftest` truthfully and part 1 passes; the regression's stub
//! answers falsely and part 1 refuses under `set -e` before part 2 starts,
//! which the "not blamed on parts 2-4" assertion checks for by name.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

const PASSES: &str = r#"fn main() {
    println!("selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce");
}
"#;

const FAILS: &str = r#"fn main() {
    println!("selftest: 19 of 20 cases solved (required 20), 0 failed to reproduce");
    std::process::exit(1);
}
"#;

const MANIFEST: &str = r#"[workspace]

[package]
name = "pistol-staged-soundness-stub"
version = "0.0.0"
edition = "2021"

[[bin]]
name = "pistol"
path = "src/main.rs"
"#;

fn stub(root: &Path, source: &str) {
    std::fs::write(root.join("src/main.rs"), source).expect("the stub engine writes");
}

fn scratch_tree(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    for dir in ["tools", "src", "crates/pistol-cli/tests/fixtures"] {
        std::fs::create_dir_all(root.join(dir)).expect("the scratch tree is created");
    }
    for script in ["staged_soundness_check.sh", "scratch_preflight.sh"] {
        std::fs::copy(
            repo(&format!("tools/{script}")),
            root.join("tools").join(script),
        )
        .unwrap_or_else(|error| panic!("the shipped {script} copies: {error}"));
    }
    std::fs::write(
        root.join("crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"),
        "# the gate stats this path and hands it to the engine; the stub ignores it\n",
    )
    .expect("the stub fixture writes");
    std::fs::write(root.join("Cargo.toml"), MANIFEST).expect("the stub manifest writes");
    stub(&root, PASSES);
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

fn soundness_check(root: &Path, target_dir: Option<&Path>) -> Output {
    let mut command = Command::new("bash");
    command
        .arg(root.join("tools/staged_soundness_check.sh"))
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
fn staged_soundness_check_part1_runs_the_binary_cargo_built_and_not_a_stale_one_at_the_default_path()
 {
    let root = scratch_tree("staged-soundness-check-stale");

    // THE CONTROL RUN reaches part 1's own summary line. It cannot reach
    // "all four parts passed" — this scratch tree vendors no pistol-search
    // test suite for parts 2-4 to run against — so the assertion below is
    // narrowly about part 1's own verdict, not the whole script's.
    let control = soundness_check(&root, None);
    let seen = said(&control);
    assert!(
        seen.contains("selftest: 20 of 20 cases solved"),
        "part 1 prints the suite's own summary line on the control run:\n{seen}"
    );

    let stale = root.join("target/release/pistol");
    assert!(
        stale.is_file(),
        "the control build lands at the path the tactical_check.sh defect hardcoded: {}",
        stale.display()
    );
    let stale_says = Command::new(&stale)
        .args(["selftest", "--fixtures", "ignored"])
        .output()
        .expect("the stale binary runs");
    assert!(
        String::from_utf8_lossy(&stale_says.stdout).contains("20 of 20"),
        "and the stale binary still claims the suite passes, which is what makes it a trap"
    );

    // THE REGRESSION: the engine now fails its own suite, and cargo is told
    // to build somewhere else. Part 1 must report the binary it BUILT, and
    // `set -e` must stop the script before parts 2-4 ever run.
    stub(&root, FAILS);
    let alt = scratch("staged-soundness-check-stale-alt");
    let ran = soundness_check(&root, Some(&alt));
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
        out.contains("selftest: 19 of 20 cases solved"),
        "the gate ran the binary cargo BUILT, not the stale one at ./target/release/pistol:\n{out}"
    );
    assert!(
        !ran.status.success(),
        "and a suite the built engine fails is a refusal, not a completed run:\n{out}"
    );
    assert!(
        out.contains("did not meet its pre-registered threshold"),
        "named by the reason THE TACTICAL SUITE UNDER STAGED actually gave:\n{out}"
    );
    assert!(
        !out.contains("2/4:") && !out.contains("all four parts passed"),
        "part 1's own failure stops the script under `set -e`; parts 2-4 never run:\n{out}"
    );
}
