//! `tools/tactical_check.sh` — CI gate 8 of 13 (`tools/ci.sh:63`), the sha-pinned
//! tactical fixture at its pre-registered threshold.
//!
//! # Why this file exists at all
//!
//! NOTHING IN THIS REPOSITORY DROVE THIS SCRIPT. Its stdout carries `selftest: 20
//! of 20 cases solved (required 20), 0 failed to reproduce` and its exit status IS
//! the gate verdict, which is a recorded number under `tools/SHELL_CHECKLIST.md`'s
//! coverage rule however the number is arrived at — the rule asks what a script
//! RECORDS, not whether it computed the digits itself (docs/decisions.md D-240,
//! D-250). It went untested until a `tools/`-scoped review found the class the
//! checklist opens with in it: EXIT-0-WRONG-ANSWER.
//!
//! # The defect these tests pin
//!
//! The gate built with `cargo build --release --bin pistol` and then ran the
//! hardcoded `./target/release/pistol`. Cargo's target directory is redirectable
//! — `CARGO_TARGET_DIR`, `[build] target-dir`, and `[build] target`, which moves
//! the artifact into a per-triple subdirectory — so with a STALE binary at the
//! hardcoded path the gate ran the stale one, printed `20 of 20` and exited 0 for
//! an engine that had just been built failing the suite; and with nothing there,
//! bash's 127 flowed into the suite's own refusal and blamed the ENGINE for a
//! tactical regression that never happened.
//!
//! # Why they run in a SCRATCH TREE with a STUB engine
//!
//! The claim is about the script's choice of BINARY, not about the engine: it
//! needs two binaries that disagree about the suite, which the real engine cannot
//! be asked for, and it needs a `cargo build` whose target directory the test
//! controls. Pointing either at the live checkout would have a test redirect the
//! builds of the repository it is being reviewed in. The scratch tree holds a
//! zero-dependency crate whose `--bin pistol` is the stub, plus a copy of the
//! SHIPPED script and the one fixture path it stats.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// A stub `pistol` that answers `selftest` the way the gate reads it: the
/// summary line on stdout, the verdict in the exit status.
const PASSES: &str = r#"fn main() {
    println!("selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce");
}
"#;

/// The same stub with a tactical regression in it — the reviewer's injected
/// `selftest.rs::holds` break, reduced to what the gate can observe.
const FAILS: &str = r#"fn main() {
    println!("selftest: 19 of 20 cases solved (required 20), 0 failed to reproduce");
    std::process::exit(1);
}
"#;

/// A crate whose one binary is named `pistol`, so the shipped script's own
/// `cargo build --release --locked --quiet --bin pistol` is what runs.
const MANIFEST: &str = r#"[workspace]

[package]
name = "pistol-tactical-check-stub"
version = "0.0.0"
edition = "2021"

[[bin]]
name = "pistol"
path = "src/main.rs"
"#;

/// Write the stub's source, replacing whatever was there.
fn stub(root: &Path, source: &str) {
    std::fs::write(root.join("src/main.rs"), source).expect("the stub engine writes");
}

/// A tree holding the shipped script, a stub crate for it to build, and the one
/// fixture path it stats — with a lockfile, because the script builds `--locked`.
fn scratch_tree(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    for dir in ["tools", "src", "crates/pistol-cli/tests/fixtures"] {
        std::fs::create_dir_all(root.join(dir)).expect("the scratch tree is created");
    }
    std::fs::copy(
        repo("tools/tactical_check.sh"),
        root.join("tools/tactical_check.sh"),
    )
    .expect("the shipped gate copies");
    std::fs::write(root.join("Cargo.toml"), MANIFEST).expect("the stub manifest writes");
    std::fs::write(
        root.join("crates/pistol-cli/tests/fixtures/tactical_v0.txt"),
        "# the gate stats this path and hands it to the engine; the stub ignores it\n",
    )
    .expect("the stub fixture writes");
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

/// Run the SHIPPED script in `root`, saying where cargo is to build rather than
/// inheriting it — which environment variable is set is the whole subject here.
///
/// The default case NAMES `root/target` rather than removing the variable.
/// `env_remove` covers the ENVIRONMENT layer only, and a user-level
/// `~/.cargo/config.toml` carrying `[build] target-dir` is a different layer: it
/// would move the control build out from under this test for a reason unrelated
/// to the code under test, on exactly the operator setup the fix under test
/// promises to respect. Measured here: an environment `CARGO_TARGET_DIR` beats a
/// config-file `[build] target-dir`, so naming it pins both layers.
fn tactical_check(root: &Path, target_dir: Option<&Path>) -> Output {
    let mut command = Command::new("bash");
    command
        .arg(root.join("tools/tactical_check.sh"))
        .current_dir(root)
        .env_remove("CARGO_BUILD_TARGET");
    match target_dir {
        Some(dir) => command.env("CARGO_TARGET_DIR", dir),
        None => command.env("CARGO_TARGET_DIR", root.join("target")),
    };
    command.output().expect("the gate script runs")
}

/// Everything the run said, in one string: the gate prints its verdict on stdout
/// and its refusals on stderr, and a failure wants both.
fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

#[test]
fn tactical_check_runs_the_binary_cargo_built_and_not_a_stale_one_at_the_default_path() {
    let root = scratch_tree("tactical-check-stale");

    // THE CONTROL RUN. Nothing is wrong, and the gate has to PASS — otherwise the
    // refusal below could come from a gate that refuses everything, which is the
    // pass-for-the-wrong-reason `tools/SHELL_CHECKLIST.md`'s coverage rule names.
    let control = tactical_check(&root, None);
    let seen = said(&control);
    assert!(
        control.status.success(),
        "the gate passes when nothing is wrong:\n{seen}"
    );
    assert!(
        seen.contains("selftest: 20 of 20 cases solved"),
        "and it prints the suite's own summary line:\n{seen}"
    );

    // The control's own binary is now the STALE one, at exactly the path the gate
    // used to hardcode. If it is not there the test would pass for the wrong
    // reason — there would be no staleness left to be fooled by.
    let stale = root.join("target/release/pistol");
    assert!(
        stale.is_file(),
        "the control build lands at the path the defect hardcoded: {}",
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

    // THE REGRESSION: the engine now fails its own suite, and cargo is told to
    // build somewhere else. The gate must report the binary it BUILT.
    stub(&root, FAILS);
    let alt = scratch("tactical-check-stale-alt");
    let ran = tactical_check(&root, Some(&alt));
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
        "and a suite the built engine fails is a refusal, not an exit-0 pass:\n{out}"
    );
    assert!(
        out.contains("did not meet its pre-registered threshold"),
        "named by the reason the suite actually gave:\n{out}"
    );
}

#[cfg(unix)]
#[test]
fn tactical_check_refuses_a_built_artifact_that_carries_no_execute_bit() {
    use std::os::unix::fs::PermissionsExt;

    let root = scratch_tree("tactical-check-noexec");
    let control = tactical_check(&root, None);
    assert!(
        control.status.success(),
        "the control run passes:\n{}",
        said(&control)
    );

    // The fourth case `command -v` admits and exec then answers with 126 — a
    // half-extracted toolchain, a `chmod` accident, a shim dropped without `+x`.
    // Cargo reports a fresh artifact without re-linking it, so the mode survives.
    let built = root.join("target/release/pistol");
    std::fs::set_permissions(&built, std::fs::Permissions::from_mode(0o644))
        .expect("the execute bit clears");

    let ran = tactical_check(&root, None);
    let out = said(&ran);
    assert!(
        !ran.status.success(),
        "a binary that cannot be exec'd is a refusal:\n{out}"
    );
    assert!(
        out.contains("is not executable"),
        "named for what it is:\n{out}"
    );
    // ONE REFUSAL PER REASON (item 8): not the build's, and not the suite's.
    assert!(
        !out.contains("does not build"),
        "and not blamed on the build, which succeeded:\n{out}"
    );
    assert!(
        !out.contains("did not meet its pre-registered threshold"),
        "and not blamed on the engine, which never ran:\n{out}"
    );
}
