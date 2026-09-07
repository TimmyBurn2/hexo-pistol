mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::{repo, repo_root, scratch};

/// Assert the gate's exit code, in a message that says what the other code
/// would have meant (`tools/SHELL_CHECKLIST.md` item 12 obligation 3).
///
/// This gate DOES have a void class, and an earlier revision of this file said
/// it did not: it builds five validators, and `cargo run` conflates "the
/// document is bad" with "the binary would not compile" in one exit 1.
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — every document named parsed and validated",
        Some(1) => "1 — one or more were rejected, or there was nothing to check",
        Some(2) => "2 — RUN VOID: a validator would not build, so no document was judged",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn gate(paths: &[&Path]) -> Output {
    Command::new("bash")
        .arg(repo("tools/config_check.sh"))
        .args(paths)
        .output()
        .expect("the shipped script starts")
}

fn summary(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stdout)
        .lines()
        .find(|line| line.starts_with("config_check: ") && line.contains("engine config(s)"))
        .unwrap_or_else(|| {
            panic!(
                "no summary line in:\n{}",
                String::from_utf8_lossy(&ran.stdout)
            )
        })
        .to_owned()
}

/// THE CONTROL. Every refusal below is satisfied by a gate that refuses
/// everything, which is `tools/SHELL_CHECKLIST.md` item 10's named failure
/// mode, so the committed set has to pass first.
///
/// The count is cross-checked against an enumeration this test does itself,
/// rather than against a literal: a literal would churn on every config added,
/// and — worse — a gate that silently stopped reaching a whole directory would
/// still match whatever literal was last written down.
#[test]
fn every_committed_document_loads_and_the_summary_counts_them_all() {
    let ran = Command::new("bash")
        .arg(repo("tools/config_check.sh"))
        .output()
        .expect("the shipped script starts");
    assert_code(&ran, 0, "the committed documents are part of the contract");

    let mut tracked = 0;
    let mut stack = vec![repo_root().join("configs")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("configs/ is readable") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "toml") {
                tracked += 1;
            }
        }
    }

    let line = summary(&ran);
    let counted: usize = line
        .split_whitespace()
        .filter_map(|word| word.parse::<usize>().ok())
        .sum();
    assert_eq!(
        counted, tracked,
        "the gate's five buckets must add up to every .toml under configs/, which this test \
         enumerated independently: {line}"
    );
}

/// The kind routing, which is the only thing the summary's five numbers are.
///
/// A document goes to a different validator per its NAME, and a mutant dropping
/// one arm of that `case` sends its documents to the engine validator instead —
/// where an arena config is not a schema error but a wholly different document
/// that would be rejected for the wrong reason, or accepted for one.
#[test]
fn each_document_kind_reaches_the_validator_its_name_selects() {
    let root = repo_root();
    let one_of_each = [
        root.join("configs/instrument_v0.toml"),
        root.join("configs/eval_v0_weights.toml"),
        root.join("configs/arena_smoke_v0.toml"),
        root.join("configs/random_openings_v1.toml"),
        root.join("configs/solver_v0.toml"),
    ];
    let paths: Vec<&Path> = one_of_each.iter().map(AsRef::as_ref).collect();
    let ran = gate(&paths);
    assert_code(&ran, 0, "one committed document of each kind");
    assert_eq!(
        summary(&ran),
        "config_check: 1 engine config(s), 1 weight table(s), 1 arena config(s), \
         1 book config(s), 1 solver config(s)",
        "one of each kind lands in its own bucket"
    );
}

/// A document that does not load is refused, and the refusal names the field.
///
/// Paired with its own control — the same document without the added key — so a
/// pass cannot come from a gate that rejects every path handed to it.
#[test]
fn a_document_that_does_not_load_is_refused_by_name_and_a_clean_copy_is_not() {
    let dir = scratch("config_check_unknown_field");
    let committed = std::fs::read_to_string(repo("configs/instrument_v0.toml"))
        .expect("the committed instrument config is readable");

    let clean = dir.join("clean.toml");
    std::fs::write(&clean, &committed).expect("the control writes");
    assert_code(
        &gate(&[&clean]),
        0,
        "a byte-for-byte copy of a committed document still loads",
    );

    let broken = dir.join("broken.toml");
    std::fs::write(
        &broken,
        committed.replace(
            "[search.candidate_policy]",
            "bogus_key = 1\n[search.candidate_policy]",
        ),
    )
    .expect("the subject writes");
    let ran = gate(&[&broken]);
    assert_code(&ran, 1, "an unknown key is refused: hard rule 1");
    let said =
        String::from_utf8_lossy(&ran.stdout).into_owned() + &String::from_utf8_lossy(&ran.stderr);
    assert!(
        said.contains("bogus_key"),
        "and the refusal names the field it could not place:\n{said}"
    );
}

/// `--check-weights-file` is the arm that reads the tree rather than the
/// document, and nothing else in this gate does.
#[test]
fn an_engine_config_naming_a_weights_file_that_is_not_there_is_refused() {
    let dir = scratch("config_check_missing_weights");
    let committed = std::fs::read_to_string(repo("configs/instrument_v0.toml"))
        .expect("the committed instrument config is readable");
    // NOT `*_weights.toml`: the kind routing is by BASENAME, so a name ending
    // that way sends an engine config to the weight-table validator and this
    // test would assert against the wrong refusal.
    let path = dir.join("engine_naming_no_table.toml");
    std::fs::write(
        &path,
        committed.replace(
            "configs/eval_v0_weights.toml",
            "configs/eval_v0_weights_that_nothing_tracks.toml",
        ),
    )
    .expect("the subject writes");

    let ran = gate(&[&path]);
    assert_code(&ran, 1, "a weights file the tree does not hold is refused");
    let said =
        String::from_utf8_lossy(&ran.stdout).into_owned() + &String::from_utf8_lossy(&ran.stderr);
    assert!(
        said.contains("eval_v0_weights_that_nothing_tracks.toml"),
        "and the refusal quotes the path it could not read:\n{said}"
    );
}

/// A validator that will not BUILD is a void, not a rejected document.
///
/// `cargo run` returns one exit code for both, so before the builds were split
/// out a full disk answered this gate's question in cargo's vocabulary and a
/// reader went looking for a broken config (tools/SHELL_CHECKLIST.md item 12,
/// docs/decisions.md D-281).
#[test]
fn a_validator_that_will_not_build_is_a_void_and_not_a_rejected_document() {
    let bin = scratch("config_check_stub_cargo").join("bin");
    std::fs::create_dir_all(&bin).expect("the stub directory");
    let stub = bin.join("cargo");
    std::fs::write(
        &stub,
        "#!/bin/sh\necho 'error: could not compile `pistol-engine`' >&2\nexit 101\n",
    )
    .expect("the stub is written");
    {
        use std::os::unix::fs::PermissionsExt;
        let mut mode = std::fs::metadata(&stub)
            .expect("the stub exists")
            .permissions();
        mode.set_mode(0o755);
        std::fs::set_permissions(&stub, mode).expect("the stub is executable");
    }
    let inherited = std::env::var("PATH").unwrap_or_default();
    let ran = Command::new("bash")
        .arg(repo("tools/config_check.sh"))
        .arg(repo("configs/instrument_v0.toml"))
        .env("PATH", format!("{}:{inherited}", bin.display()))
        .output()
        .expect("the shipped script starts");
    assert_code(
        &ran,
        2,
        "a validator that will not build judges no document",
    );
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(
        said.contains("config_check: RUN VOID"),
        "and the gate names it as a void in its own words:\n{said}"
    );
    assert!(
        said.contains("no document is implicated"),
        "and says the document is not what failed:\n{said}"
    );
}
