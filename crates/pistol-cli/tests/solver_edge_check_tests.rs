//! `tools/solver_edge_check.sh` — the `p = 0` adjudicator for WP-1.5a's H1
//! (docs/experiments/wp15a_prereg.md §2; tools/SHELL_CHECKLIST.md item 10).
//!
//! # Why this file exists at all
//!
//! THE TWO ADJUDICATORS BEFORE THIS ONE WERE BOTH WRONG, AND NEITHER WAS
//! TESTABLE. Both lived inside a pre-registration's prose, so both were driven
//! only by reviewers running the document by hand, and four consecutive review
//! rounds each found a new defect that way:
//!
//!   - a substring count over `*Cargo.toml` refused a tree whose only mention of
//!     the crate was a COMMENT, and passed a tree with a real
//!     `[dependencies."pistol-solver"]` edge that cargo resolves;
//!   - `cargo tree -i`'s EXIT STATUS read a `[dev-dependencies]` entry — the one
//!     the oracle's test-tree home depends on — as a refutation of `p = 0`,
//!     because a dev edge, a build edge and an off-target edge all exit 0 while
//!     `--edges normal` leaves the printed tree empty.
//!
//! That is D-231's lesson exactly (`bench_delta.sh` produced this project's
//! official perf verdict with zero tests until a review round forced some) and
//! it is `SHELL_CHECKLIST` item 10's rule: a `tools/` script that produces a
//! recorded number carries at least one test in a suite CI runs, driving the
//! SHIPPED script, with a control run so a pass cannot come from a gate that
//! refuses everything.
//!
//! # Why the workspaces are synthetic
//!
//! The script takes a workspace root and a crate name as ARGUMENTS, which is
//! what makes it testable at all: the real repository has exactly one answer and
//! it is the answer already believed, so a suite that only ran there would
//! certify nothing. These workspaces hold two crates of a few lines each and no
//! external dependencies, so every arm resolves in milliseconds and compiles
//! nothing. The crate name is `thecrate`, deliberately NOT `pistol-solver`: a
//! test that agreed with the script about the name would stop being able to see
//! a script that hard-coded it.
//!
//! # And one test does run against the real repository
//!
//! `the_shipped_workspace_has_no_normal_edge_on_the_solver` makes `p = 0` a
//! standing CI invariant rather than something one pre-registration checked
//! once. It is the assertion H1 rests on, and it now fails in CI on the commit
//! that breaks it instead of in a governed run months later.
//!
//! # RULE9-JUSTIFICATION: one script's readings, over one claim.
//! Every test here is the same claim — that this adjudicator separates a normal
//! dependency edge from a dev edge, a build edge, an absent crate and an
//! unreadable workspace, and names which — and all of them need the same
//! two-crate scratch workspace builder.

mod common;

use std::path::{Path, PathBuf};
use std::process::Output;

use common::{repo, repo_root, scratch};

/// The crate the scratch workspaces ask about. NOT `pistol-solver`: the script
/// is supposed to take the name as an argument, and a test that used the real
/// name could not tell a script that ignored the argument from one that honoured
/// it.
const SUBJECT: &str = "thecrate";

/// Run the shipped script against a workspace root and a crate name.
fn edge_check(root: &Path, crate_name: &str) -> Output {
    edge_check_env(root, crate_name, &[])
}

/// The same, with named environment bindings added to the script's environment.
/// The record's byte-invariance is a claim about what a CALLER'S environment can
/// do to it, so the tests that make that claim need a way to set one.
fn edge_check_env(root: &Path, crate_name: &str, env: &[(&str, &str)]) -> Output {
    let mut command = std::process::Command::new("bash");
    command
        .arg(repo("tools/solver_edge_check.sh"))
        .arg(root)
        .arg(crate_name);
    for (key, value) in env {
        command.env(key, value);
    }
    command.output().expect("bash runs the shipped script")
}

/// A two-crate workspace: `thecrate` (a library) and `user` (a binary), where
/// `user`'s dependency on `thecrate` is declared under `section` — or not at all
/// when `section` is `None`, which is the CONTROL.
fn workspace(name: &str, section: Option<&str>) -> PathBuf {
    let root = scratch(name).join("ws");
    std::fs::create_dir_all(root.join("crates/thecrate/src")).expect("the subject crate's tree");
    std::fs::create_dir_all(root.join("crates/user/src")).expect("the dependent crate's tree");
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/thecrate\", \"crates/user\"]\nresolver = \"2\"\n",
    )
    .expect("the workspace manifest");
    std::fs::write(
        root.join("crates/thecrate/Cargo.toml"),
        "[package]\nname = \"thecrate\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    )
    .expect("the subject manifest");
    std::fs::write(root.join("crates/thecrate/src/lib.rs"), "\n").expect("the subject source");
    let edge = match section {
        Some(section) => {
            format!("\n[{section}]\nthecrate = {{ path = \"../thecrate\" }}\n")
        }
        None => String::new(),
    };
    std::fs::write(
        root.join("crates/user/Cargo.toml"),
        format!("[package]\nname = \"user\"\nversion = \"0.0.1\"\nedition = \"2021\"\n{edge}"),
    )
    .expect("the dependent manifest");
    std::fs::write(root.join("crates/user/src/main.rs"), "fn main() {}\n")
        .expect("the dependent source");
    lock(&root);
    root
}

/// The script runs `cargo tree --locked`, so a workspace without a lockfile
/// cannot be resolved. Generating one is setup, not a claim, so a failure here
/// panics rather than becoming a confusing assertion further down.
fn lock(root: &Path) {
    let ran = std::process::Command::new("cargo")
        .current_dir(root)
        .args(["generate-lockfile", "-q"])
        .output()
        .expect("cargo runs");
    assert!(
        ran.status.success(),
        "cargo generate-lockfile in {}: {}",
        root.display(),
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn stdout(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stdout).into_owned()
}

fn stderr(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stderr).into_owned()
}

/// THE CONTROL. Without it every assertion below is satisfied by a script that
/// refuses everything, which is the failure mode item 10 names by name.
#[test]
fn a_workspace_with_no_edge_at_all_is_accepted() {
    let root = workspace("edge-none", None);
    let ran = edge_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "a workspace with no dependency on {SUBJECT} must be accepted\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
    assert!(
        stdout(&ran).contains("NO normal reverse-dependency"),
        "the acceptance is named, not merely an exit code: {}",
        stdout(&ran)
    );
}

#[test]
fn a_normal_dependency_edge_is_refused_and_the_dependent_is_named() {
    let root = workspace("edge-normal", Some("dependencies"));
    let ran = edge_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a normal dependency edge must be refused\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
    assert!(
        stdout(&ran).contains("user"),
        "the refusal prints the tree, which NAMES the dependent: {}",
        stdout(&ran)
    );
}

/// The reading `cargo tree -i`'s exit status got wrong, and the reason this
/// script reads its stdout instead. A dev-dependency is how the solver's own
/// oracle would be reached from a test tree, and it reaches no shipped binary.
#[test]
fn a_dev_dependency_edge_is_accepted() {
    let root = workspace("edge-dev", Some("dev-dependencies"));
    let ran = edge_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "a dev-dependency reaches no shipped binary and must be accepted\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
}

#[test]
fn a_build_dependency_edge_is_accepted_by_the_normal_edge_reading() {
    let root = workspace("edge-build", Some("build-dependencies"));
    let ran = edge_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "`--edges normal` excludes build edges, and the reading follows it\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
}

/// A crate the workspace does not have is NOT the same observation as a crate
/// with no dependents, and conflating them is how a typo becomes good news.
#[test]
fn a_crate_absent_from_the_workspace_is_a_refusal_and_not_an_acceptance() {
    let root = workspace("edge-absent", None);
    let ran = edge_check(&root, "no-such-crate");
    assert_eq!(
        ran.status.code(),
        Some(2),
        "an absent crate voids the run rather than answering it\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
    assert!(
        stderr(&ran).contains("not a package in the workspace"),
        "the refusal says which question went unanswered: {}",
        stderr(&ran)
    );
}

/// "cargo could not answer" and "there is no edge" are two reasons, and the
/// readability probe exists so one status cannot mean both.
#[test]
fn an_unreadable_workspace_voids_the_run_by_its_own_reason() {
    let root = workspace("edge-broken", None);
    std::fs::write(
        root.join("crates/user/Cargo.toml"),
        "this is not toml [[[\n",
    )
    .expect("the manifest is broken on purpose");
    let ran = edge_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "an unresolvable workspace voids the run\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
    assert!(
        stderr(&ran).contains("cannot resolve the workspace"),
        "the refusal names resolution, not the crate: {}",
        stderr(&ran)
    );
}

#[test]
fn a_missing_workspace_root_is_named_rather_than_assumed() {
    let root = scratch("edge-missing").join("nowhere");
    let ran = edge_check(&root, SUBJECT);
    assert_eq!(ran.status.code(), Some(2), "a missing root voids the run");
    assert!(
        stderr(&ran).contains("no such workspace root"),
        "the refusal names the missing root: {}",
        stderr(&ran)
    );
}

#[test]
fn a_crate_name_that_would_be_read_as_an_option_is_refused() {
    let root = workspace("edge-dash", None);
    let ran = edge_check(&root, "--workspace");
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a leading dash is refused before it reaches cargo"
    );
    assert!(
        stderr(&ran).contains("starts with a dash"),
        "the refusal says why: {}",
        stderr(&ran)
    );
}

/// `p = 0` AS A STANDING CI INVARIANT. WP-1.5a's whole hypothesis is that
/// nothing in this workspace links the solver; until now that was checked by one
/// pre-registration, once. Here it fails on the commit that breaks it.
#[test]
fn the_shipped_workspace_has_no_normal_edge_on_the_solver() {
    let ran = edge_check(&repo_root(), "pistol-solver");
    assert_eq!(
        ran.status.code(),
        Some(0),
        "p = 0: nothing in this workspace may take a normal dependency on pistol-solver\n\
         stdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
}

/// `pwd -P`, AND THE RECORD IS WHAT IS UNDER TEST. The script substitutes the
/// workspace root out of the printed tree so that two replications of a
/// registered run compare byte for byte; cargo prints PHYSICAL paths, so a
/// caller standing on a symlinked root gave bash's LOGICAL `pwd` a string that
/// matched none of them and the per-run absolute path went into the record
/// intact. That is EXIT-0-WRONG-ANSWER in the printed half: the verdict was
/// right, the bytes were wrong, and nothing was red.
///
/// REPRODUCED against the shipped script before the repair: with `pwd`, this
/// case printed `/tmp/pistol-testscratch-…/ws/crates/user` and no `<workspace>`.
#[test]
fn a_symlinked_workspace_root_is_still_substituted_out_of_the_printed_tree() {
    let root = workspace("edge-symlink", Some("dependencies"));
    let real = root
        .parent()
        .expect("the workspace sits inside its scratch directory");
    let link = real.with_file_name(format!(
        "{}-link",
        real.file_name()
            .expect("the scratch directory has a name")
            .to_string_lossy()
    ));
    let _ = std::fs::remove_file(&link);
    std::os::unix::fs::symlink(real, &link).expect("a symlink to the scratch directory");

    let ran = edge_check_env(&link.join("ws"), SUBJECT, &[]);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "1 is the answer 'there is an edge'; 2 would mean no answer was taken and \
         there would be no tree to inspect\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
    let printed = stdout(&ran);
    // THE CONTROL for this test: the tree was actually taken and printed, so the
    // absence asserted below is a substitution rather than an empty record.
    assert!(
        printed.contains("<workspace>/crates/user"),
        "the dependent is named under the substituted root: {printed}"
    );
    let physical = real
        .canonicalize()
        .expect("the scratch directory resolves")
        .to_string_lossy()
        .into_owned();
    assert!(
        !printed.contains(&physical),
        "the physical scratch path leaked into the record through a logical `pwd`: \
         {physical} in {printed}"
    );
}

/// `--color never`, FOR THE SAME RECORD. `CARGO_TERM_COLOR=always` in the
/// caller's environment makes cargo emit SGR escapes around the tree glyphs even
/// when its output is a pipe, and the script captures that output and prints it.
/// Escapes are invisible in a terminal and are bytes in a file, so a record
/// compared byte for byte across replications differs for a reason no reader can
/// see.
///
/// REPRODUCED against the shipped script before the repair: this case printed
/// `\x1b[2m` around the tree glyph and exited 1 all the same.
#[test]
fn a_colour_forcing_environment_leaves_no_escape_sequence_in_the_printed_tree() {
    let root = workspace("edge-colour", Some("dependencies"));
    let ran = edge_check_env(&root, SUBJECT, &[("CARGO_TERM_COLOR", "always")]);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "1 is the answer 'there is an edge'\nstdout: {}\nstderr: {}",
        stdout(&ran),
        stderr(&ran)
    );
    let printed = stdout(&ran);
    // THE CONTROL, as above: a record with no tree in it has no escapes either.
    assert!(
        printed.contains("<workspace>/crates/user"),
        "the dependent is named, so there is a tree to be coloured: {printed}"
    );
    assert!(
        !printed.contains('\u{1b}'),
        "an SGR escape reached the record: {printed:?}"
    );
}
