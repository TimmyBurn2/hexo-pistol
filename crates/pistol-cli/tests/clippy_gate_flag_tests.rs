mod common;

use std::path::Path;
use std::process::Command;

use common::{repo, scratch};

/// The lint flags gate 4 actually runs with, READ OUT OF `tools/ci.sh`.
///
/// Not restated here. A test that spells its own copy of the flags passes
/// forever after somebody edits the gate, which is the drift `docs/process.md`
/// names: the check and the thing checked must not be two documents.
fn gate_four_flags() -> Vec<String> {
    let script = std::fs::read_to_string(repo("tools/ci.sh")).expect("tools/ci.sh is readable");
    let line = script
        .lines()
        .find(|line| line.starts_with("cargo clippy --workspace"))
        .unwrap_or_else(|| panic!("no gate-4 clippy command in tools/ci.sh:\n{script}"));
    let after = line
        .split_once(" -- ")
        .unwrap_or_else(|| panic!("gate 4's command passes no lint flags: {line}"))
        .1;
    let flags: Vec<String> = after
        .split_whitespace()
        .take_while(|word| *word != "||")
        .map(str::to_owned)
        .collect();
    assert!(
        !flags.is_empty(),
        "gate 4 must pass at least one lint flag: {line}"
    );
    // THE FLAGS ARE NOT THE WHOLE GATE, and a reviewer found the two edits that
    // prove it: dropping `--all-targets` stops the lint reaching tests and
    // examples, and replacing `|| fail "clippy"` with `|| true` makes the gate
    // decorative — both leave every assertion below green, because both live
    // OUTSIDE the post-`--` string. Neither is a lint flag, so neither belongs
    // in `flags`; they are checked here, on the same line, read once.
    assert!(
        line.contains(" --all-targets "),
        "gate 4 must lint tests and examples too, or the flags below reach less \
         than this gate claims: {line}"
    );
    assert!(
        line.contains("|| fail "),
        "gate 4 must FAIL the run when clippy refuses; a gate whose refusal is \
         swallowed is a gate that does not exist: {line}"
    );
    flags
}

/// A standalone crate, outside this workspace, holding `body`.
///
/// The package is named after the case, and `clippy` below pins the target
/// directory INSIDE it. Both matter and neither is tidiness: three crates all
/// called `subject` at one version are one cache entry to cargo, so under an
/// exported `CARGO_TARGET_DIR` — which is how mutation and verification work
/// runs — the first case's artifact answered for the other two and they passed
/// on code that was never compiled (REPRODUCED in this package's own mutation
/// baseline, which is what a purged baseline is for).
fn crate_with(name: &str, body: &str) -> std::path::PathBuf {
    let root = scratch(name).join("subject");
    std::fs::create_dir_all(root.join("src")).expect("the scratch crate tree");
    std::fs::write(
        root.join("Cargo.toml"),
        // `[workspace]` so cargo does not walk upward looking for one.
        format!(
            "[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n[workspace]\n"
        ),
    )
    .expect("the manifest writes");
    std::fs::write(root.join("src/lib.rs"), body).expect("the source writes");
    root
}

fn clippy(root: &Path, flags: &[String]) -> std::process::Output {
    Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--")
        .args(flags)
        .current_dir(root)
        // Never the ambient one: see `crate_with`.
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .expect("cargo clippy runs")
}

/// THE CONTROL. Every refusal below is satisfied by a flag set that refuses
/// everything, which is `tools/SHELL_CHECKLIST.md` item 10's named failure
/// mode.
#[test]
fn gate_fours_flags_accept_a_crate_with_nothing_wrong_with_it() {
    let flags = gate_four_flags();
    let root = crate_with(
        "clippy_flags_control",
        "pub fn answer() -> u32 {\n    7\n}\n",
    );
    let ran = clippy(&root, &flags);
    assert!(
        ran.status.success(),
        "the flag set must not refuse clean code, or every check below is vacuous:\n{}",
        String::from_utf8_lossy(&ran.stderr)
    );
}

/// A planted RUSTC warning fails the gate.
///
/// This is A-19's own defect as a test: an unused import is a rustc lint, not a
/// clippy one, so `-D clippy::all` alone let it through every gate in CI and an
/// audit found it instead (docs/decisions.md D-677). Dropping `-D warnings`
/// from `tools/ci.sh` makes this test go red, because the flags come from that
/// line.
#[test]
fn a_planted_rustc_warning_fails_gate_fours_flag_set() {
    let flags = gate_four_flags();
    let root = crate_with(
        "clippy_flags_rustc_warning",
        "use std::collections::BTreeMap;\n\npub fn answer() -> u32 {\n    7\n}\n",
    );
    let ran = clippy(&root, &flags);
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(
        !ran.status.success(),
        "an unused import must fail gate 4's flag set — it did not, so a rustc \
         warning can reach `main` again:\n{said}"
    );
    assert!(
        said.contains("unused import"),
        "and the refusal names the lint that fired:\n{said}"
    );
}

/// And the clippy half is still there, so a fix for the rustc half that
/// replaced the flags rather than adding to them goes red here.
#[test]
fn a_planted_clippy_lint_fails_gate_fours_flag_set() {
    let flags = gate_four_flags();
    let root = crate_with(
        "clippy_flags_clippy_lint",
        "pub fn answer(values: &[u32]) -> u32 {\n    let mut total = 0;\n    \
         for index in 0..values.len() {\n        total += values[index];\n    }\n    total\n}\n",
    );
    let ran = clippy(&root, &flags);
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(
        !ran.status.success(),
        "a clippy lint must still fail the flag set:\n{said}"
    );
    assert!(
        said.contains("clippy::needless_range_loop"),
        "and the refusal names the clippy lint that fired:\n{said}"
    );
}
