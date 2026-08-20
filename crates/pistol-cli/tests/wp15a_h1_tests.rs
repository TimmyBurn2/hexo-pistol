//! `tools/wp15a_h1.sh` — WP-1.5a's H1 instrument
//! (docs/experiments/wp15a_prereg.md; tools/SHELL_CHECKLIST.md item 10).
//!
//! # Why this file exists at all
//!
//! THIS INSTRUMENT WAS 320 LINES INSIDE A PRE-REGISTRATION, AND FOUR CONSECUTIVE
//! REVIEW ROUNDS EACH FOUND A NEW BLOCKING DEFECT IN IT: an EXIT trap that
//! rewrote every verdict to `1`; a guard that announced its own conclusion about
//! a `git` invocation that never answered; a `diff -u` whose `mktemp` paths made
//! a genuine ABORT fail the document's own replication rule; an adjudicator that
//! read a `[dev-dependencies]` entry as a refutation. Every one was found by an
//! agent running the document by hand, because nothing else could run it — which
//! is item 10's diagnosis and D-231's precedent verbatim.
//!
//! # Why the workspace is synthetic and the snapshot is a stub
//!
//! The claims here are about CONTROL FLOW — which inputs reach a verdict, which
//! reach a refusal, and whether the refusal names the right reason. A real run
//! costs three release builds of the engine and a 24-position snapshot; a
//! two-crate scratch workspace and a deterministic stub reach every one of the
//! same branches in seconds. The script takes the subject crate, the package and
//! binary to build, and the snapshot's repository-relative path as BINDINGS, so
//! nothing here patches anything, and a test that used the real names could not
//! tell a script that honoured its arguments from one that ignored them.
//!
//! # The control is the point
//!
//! `a_clean_workspace_reaches_confirmed` exists so that every refusal below is
//! evidence about a REFUSAL and not about a script that refuses everything —
//! item 10's named failure mode.
//!
//! # RULE9-JUSTIFICATION: one instrument's verdicts, over one hypothesis.
//! Every test is the same claim — that this script reaches CONFIRMED, ABORT or
//! RUN VOID for the right reason and never dies saying nothing — and each needs
//! the same scratch repository, the same stub snapshot and the same bindings.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Output;

use common::{repo, scratch};

/// A two-crate workspace in a git repository with two commits, a stub snapshot
/// instrument, and a baseline record that matches the baseline build.
struct Fixture {
    root: PathBuf,
    base_rev: String,
    landing: String,
    base_record: PathBuf,
    sidecar: PathBuf,
}

fn git(root: &Path, args: &[&str]) -> Output {
    let ran = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git runs");
    assert!(
        ran.status.success(),
        "git {args:?} in {}: {}",
        root.display(),
        String::from_utf8_lossy(&ran.stderr)
    );
    ran
}

fn git_out(root: &Path, args: &[&str]) -> String {
    String::from_utf8_lossy(&git(root, args).stdout)
        .trim()
        .to_owned()
}

/// The stub snapshot: deterministic behaviour lines, plus the two lines `inv()`
/// excludes, plus a digest of a file it reads FROM ITS OWN ROOT. That last part
/// is what lets a test see whether the instrument read the clone's inputs or the
/// working tree's.
const STUB_SNAPSHOT: &str = r##"#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
BIN=""; OUT=""
while [ "$#" -gt 0 ]; do
	case "$1" in
	--binary) BIN="$2"; shift 2 ;;
	--out) OUT="$2"; shift 2 ;;
	*) echo "stub_snapshot: unknown argument $1" >&2; exit 2 ;;
	esac
done
[ -n "$BIN" ] && [ -n "$OUT" ] || { echo "stub_snapshot: --binary and --out are required" >&2; exit 2; }
{
	echo "baseline_snapshot 1"
	echo "revision $(git rev-parse HEAD)"
	echo "binary_sha256 $(sha256sum -- "$BIN" | cut -d' ' -f1)"
	echo "config configs/stub.toml $(sha256sum -- configs/stub.toml | cut -d' ' -f1)"
	for i in $(seq 1 55); do echo "position $i nodes 1000 score cp 0 bestmove a$i"; done
	echo "# timing"
	echo "timing tree $([ -z "$(git status --porcelain)" ] && echo clean || echo dirty)"
} > "$OUT"
echo "stub_snapshot: wrote $OUT" >&2
"##;

fn write(path: &Path, text: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("the parent directory");
    }
    std::fs::write(path, text).unwrap_or_else(|e| panic!("cannot write {}: {e}", path.display()));
}

fn executable(path: &Path, text: &str) {
    write(path, text);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
            .expect("the stub is executable");
    }
}

/// Build the fixture. `subject_section` places the app's dependency on the
/// subject crate — `None` is the honest case, in which nothing links it.
fn fixture(name: &str, subject_section: Option<&str>) -> Fixture {
    fixture_with(name, subject_section, false)
}

/// `used`: the app CALLS the subject, so the subject's content change reaches
/// codegen and the two builds genuinely differ.
fn fixture_with(name: &str, subject_section: Option<&str>, used: bool) -> Fixture {
    let root = scratch(name).join("repo");
    std::fs::create_dir_all(&root).expect("the repository directory");
    write(
        &root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/subject\", \"crates/app\"]\nresolver = \"2\"\n",
    );
    write(
        &root.join("crates/subject/Cargo.toml"),
        "[package]\nname = \"subject\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    write(
        &root.join("crates/subject/src/lib.rs"),
        if used {
            "pub fn f() -> u64 { 1 }\n"
        } else {
            "// baseline\n"
        },
    );
    let edge = match subject_section {
        Some(section) => format!("\n[{section}]\nsubject = {{ path = \"../subject\" }}\n"),
        None => String::new(),
    };
    write(
        &root.join("crates/app/Cargo.toml"),
        &format!("[package]\nname = \"app\"\nversion = \"0.0.1\"\nedition = \"2021\"\n{edge}"),
    );
    write(
        &root.join("crates/app/src/main.rs"),
        if used {
            "fn main() { println!(\"{}\", subject::f()); }\n"
        } else {
            "fn main() {}\n"
        },
    );
    write(&root.join("configs/stub.toml"), "# the committed config\n");
    executable(&root.join("tools/stub_snapshot.sh"), STUB_SNAPSHOT);
    write(&root.join(".gitignore"), "/target\n");

    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "t@example.invalid"]);
    git(&root, &["config", "user.name", "T"]);
    let lock = std::process::Command::new("cargo")
        .current_dir(&root)
        .args(["generate-lockfile", "-q"])
        .output()
        .expect("cargo runs");
    assert!(lock.status.success(), "cargo generate-lockfile");
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "baseline"]);
    let base_rev = git_out(&root, &["rev-parse", "HEAD"]);

    // The baseline record must carry the digest of the binary the BASELINE
    // revision builds, because the instrument attests it by rebuilding.
    let base_bin = build(&root);
    let scratch_dir = scratch(&format!("{name}-artifacts"));
    let base_record = scratch_dir.join("baseline.txt");
    let taken = std::process::Command::new("bash")
        .current_dir(&root)
        .arg(root.join("tools/stub_snapshot.sh"))
        .args(["--binary", &base_bin, "--out"])
        .arg(&base_record)
        .output()
        .expect("the stub runs");
    assert!(
        taken.status.success(),
        "stub snapshot: {}",
        String::from_utf8_lossy(&taken.stderr)
    );
    let sidecar = scratch_dir.join("baseline.toolchain.txt");
    let rustc = String::from_utf8_lossy(
        &std::process::Command::new("rustc")
            .arg("-vV")
            .output()
            .expect("rustc runs")
            .stdout,
    )
    .lines()
    .next()
    .expect("rustc -vV has a first line")
    .to_owned();
    let cargo_version = String::from_utf8_lossy(
        &std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .expect("cargo runs")
            .stdout,
    )
    .trim()
    .to_owned();
    write(&sidecar, &format!("{rustc}\n{cargo_version}\n"));

    // The landing commit: the subject changes, nothing else does. When the app
    // CALLS it, the change reaches codegen and the two builds genuinely differ,
    // which is what the contradiction branch needs.
    write(
        &root.join("crates/subject/src/lib.rs"),
        if used {
            "pub fn f() -> u64 { 987_654_321 }\n"
        } else {
            "// landing\npub fn f() {}\n"
        },
    );
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "landing"]);
    let landing = git_out(&root, &["rev-parse", "HEAD"]);

    Fixture {
        root,
        base_rev,
        landing,
        base_record,
        sidecar,
    }
}

fn build(root: &Path) -> String {
    let ran = std::process::Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "--release",
            "--locked",
            "-p",
            "app",
            "--bin",
            "app",
            "--message-format=json-render-diagnostics",
        ])
        .output()
        .expect("cargo runs");
    assert!(
        ran.status.success(),
        "cargo build in {}: {}",
        root.display(),
        String::from_utf8_lossy(&ran.stderr)
    );
    String::from_utf8_lossy(&ran.stdout)
        .lines()
        .filter_map(|line| line.split("\"executable\":\"").nth(1))
        .filter_map(|rest| rest.split('"').next())
        .next_back()
        .expect("cargo names the executable it produced")
        .to_owned()
}

fn sha256(path: &Path) -> String {
    let ran = std::process::Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum runs");
    String::from_utf8_lossy(&ran.stdout)
        .split_whitespace()
        .next()
        .expect("sha256sum prints a digest")
        .to_owned()
}

impl Fixture {
    /// Run the shipped instrument, with every binding overridable.
    fn run(&self, overrides: &[(&str, &str)]) -> Output {
        let mut env: BTreeMap<&str, String> = BTreeMap::new();
        env.insert("REPO", self.root.display().to_string());
        env.insert("BASE", self.base_record.display().to_string());
        env.insert("BASE_SHA", sha256(&self.base_record));
        env.insert("BASE_REV", self.base_rev.clone());
        env.insert("BASE_TC", self.sidecar.display().to_string());
        env.insert("BASE_TC_SHA", sha256(&self.sidecar));
        env.insert("LANDING", self.landing.clone());
        env.insert("SUBJECT_CRATE", "subject".into());
        env.insert("SUBJECT_PATH", "crates/subject".into());
        env.insert("BUILD_PKG", "app".into());
        env.insert("BUILD_BIN", "app".into());
        env.insert("SNAPSHOT_REL", "tools/stub_snapshot.sh".into());
        env.insert(
            "EDGE_CHECK",
            repo("tools/solver_edge_check.sh").display().to_string(),
        );
        for (key, value) in overrides {
            env.insert(key, (*value).to_owned());
        }
        let mut command = std::process::Command::new("bash");
        command.arg(repo("tools/wp15a_h1.sh"));
        command.env_clear();
        for key in ["PATH", "HOME", "USER", "CARGO_HOME", "RUSTUP_HOME"] {
            if let Ok(value) = std::env::var(key) {
                command.env(key, value);
            }
        }
        for (key, value) in &env {
            command.env(key, value);
        }
        command.output().expect("bash runs the shipped instrument")
    }
}

fn out(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stdout).into_owned()
}
fn err(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stderr).into_owned()
}

/// THE CONTROL. Without it every refusal below is satisfied by a script that
/// refuses everything.
#[test]
fn a_clean_workspace_reaches_confirmed() {
    let f = fixture("h1-control", None);
    let ran = f.run(&[]);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "the honest case must CONFIRM\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(out(&ran).contains("H1-a CONFIRMED"), "{}", out(&ran));
    assert!(out(&ran).contains("H1-b CONFIRMED"), "{}", out(&ran));
}

/// A linked subject refutes `p = 0` — and BOTH instruments' readings must reach
/// the record before either is acted on. Ordering the edge check to exit first
/// made the registered agreement criterion unevaluable in exactly the branch
/// where the two instruments can disagree, which is a refusal that should fire
/// and cannot.
#[test]
fn a_normal_dependency_aborts_with_both_instruments_readings_in_the_record() {
    let f = fixture("h1-edge", Some("dependencies"));
    let ran = f.run(&[]);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a linked subject refutes p = 0\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(
        out(&ran).contains("p = 0 REFUTED"),
        "the graph's reading is recorded: {}",
        out(&ran)
    );
    assert!(
        out(&ran).contains("H1-a reading"),
        "and H1-a's reading is taken anyway, so a disagreement could be seen: {}",
        out(&ran)
    );
    assert!(
        err(&ran).contains("adjudicates nothing here"),
        "and the abort says H1-a's reading does NOT adjudicate here, because a linked crate \
         whose diff is dead code is still linked: {}",
        err(&ran)
    );
}

/// THE CRITERION'S CONTRADICTION BRANCH, in the ONE direction where the two
/// instruments genuinely constrain each other: a crate absent from the resolved
/// graph cannot reach codegen, so a moved binary means one of them is lying. It
/// is exercised by making the edge check lie in that direction, which is the only
/// honest way to reach a branch whose whole purpose is to catch an instrument
/// that has stopped telling the truth.
///
/// The OTHER direction is deliberately not a contradiction, and this project
/// learned that from a test rather than a reviewer: an unused dependency edge
/// gives `edge = 1` with two bit-identical binaries, because H1-a asks whether a
/// CONTENT CHANGE reaches codegen and the graph asks whether the crate is LINKED.
/// A biconditional would have voided that run.
#[test]
fn a_moved_binary_with_no_edge_in_the_graph_voids_the_run() {
    // The app CALLS the subject, so its landing content reaches codegen and the
    // two builds differ. An edge check reporting NO dependent then contradicts
    // the binaries, and the run must take no verdict.
    let f = fixture_with("h1-contradict", Some("dependencies"), true);
    let liar = scratch("h1-contradict-liar").join("liar.sh");
    executable(
        &liar,
        "#!/usr/bin/env bash\necho \"liar: reporting no dependent\"\nexit 0\n",
    );
    let ran = f.run(&[("EDGE_CHECK", &liar.display().to_string())]);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a contradiction is RUN VOID, never a verdict\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(
        err(&ran).contains("CONTRADICT"),
        "the refusal names the contradiction: {}",
        err(&ran)
    );
}

/// The control for the branch above: with the app CALLING the subject and the
/// edge check telling the truth, the run is an honest ABORT rather than a
/// contradiction — so that branch is reached by the LIE and not by the fixture.
#[test]
fn a_used_subject_with_a_truthful_edge_check_aborts_rather_than_contradicting() {
    let f = fixture_with("h1-used-honest", Some("dependencies"), true);
    let ran = f.run(&[]);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "an honest linked-and-used subject is an ABORT\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(out(&ran).contains("H1-a reading differs"), "{}", out(&ran));
}

/// A dev-dependency reaches no shipped binary. Revision 10's adjudicator called
/// this a refutation; the instrument must not.
#[test]
fn a_dev_dependency_on_the_subject_still_confirms() {
    let f = fixture("h1-dev", Some("dev-dependencies"));
    let ran = f.run(&[]);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "a dev-dependency is not a refutation\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
}

/// THE B10-3 REGRESSION TEST. Tampering with the instrument's data inputs in the
/// WORKING TREE, invisibly to `git status`, must not reach the verdict: the
/// snapshot runs inside the pristine clone, where every input is committed
/// content at LANDING.
#[test]
fn tampering_with_the_instruments_config_in_the_worktree_does_not_reach_the_verdict() {
    let f = fixture("h1-worktree-config", None);
    git(
        &f.root,
        &["update-index", "--assume-unchanged", "configs/stub.toml"],
    );
    write(
        &f.root.join("configs/stub.toml"),
        "# TAMPERED IN THE WORKTREE\n",
    );
    assert_eq!(
        git_out(&f.root, &["status", "--porcelain"]),
        "",
        "the tamper is invisible to git status, which is the whole hazard"
    );
    let ran = f.run(&[]);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "the clone's committed config is what the instrument reads\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
}

#[test]
fn a_moved_behaviour_line_aborts_with_a_named_refusal() {
    let f = fixture("h1-moved", None);
    let text = std::fs::read_to_string(&f.base_record).expect("the record reads");
    let mutated = text.replace("bestmove a7", "bestmove ZZ");
    assert_ne!(text, mutated, "the mutation applies");
    std::fs::write(&f.base_record, &mutated).expect("the record writes");
    let sha = sha256(&f.base_record);
    let ran = f.run(&[("BASE_SHA", &sha)]);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a moved behaviour line is an ABORT\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(err(&ran).contains("H1-b FAILED"), "named: {}", err(&ran));
    assert!(
        out(&ran).contains("--- baseline") && out(&ran).contains("+++ candidate"),
        "the diff carries fixed labels, not mktemp paths: {}",
        out(&ran)
    );
}

/// The ABORT path's stdout must be byte-identical across processes, because the
/// pre-registration voids a run whose replications disagree.
#[test]
fn the_abort_path_is_replication_stable() {
    let f = fixture("h1-replication", None);
    let text = std::fs::read_to_string(&f.base_record).expect("the record reads");
    std::fs::write(&f.base_record, text.replace("bestmove a7", "bestmove ZZ")).expect("writes");
    let sha = sha256(&f.base_record);
    let first = f.run(&[("BASE_SHA", &sha)]);
    let second = f.run(&[("BASE_SHA", &sha)]);
    assert_eq!(first.status.code(), Some(1));
    assert_eq!(second.status.code(), Some(1));
    assert_eq!(
        out(&first),
        out(&second),
        "two processes on the same ABORT must print the same bytes"
    );
}

#[test]
fn an_incomplete_baseline_record_voids_the_run_rather_than_aborting() {
    let f = fixture("h1-incomplete", None);
    let text = std::fs::read_to_string(&f.base_record).expect("the record reads");
    let broken = text.replacen("baseline_snapshot 1", "baseline_snapshot_incomplete 1", 1);
    std::fs::write(&f.base_record, &broken).expect("the record writes");
    let sha = sha256(&f.base_record);
    let ran = f.run(&[("BASE_SHA", &sha)]);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "an incomplete record is RUN VOID, not ABORT\nstderr: {}",
        err(&ran)
    );
    assert!(err(&ran).contains("not a COMPLETE"), "{}", err(&ran));
}

#[test]
fn a_landing_that_is_not_an_object_name_is_refused_before_any_git_call_can_accept_it() {
    let f = fixture("h1-branch", None);
    let ran = f.run(&[("LANDING", "master")]);
    assert_eq!(ran.status.code(), Some(2), "stderr: {}", err(&ran));
    assert!(
        err(&ran).contains("not a 40-hex object name"),
        "{}",
        err(&ran)
    );
}

#[test]
fn a_landing_that_is_not_an_ancestor_of_head_voids_the_run() {
    let f = fixture("h1-ancestor", None);
    let ran = f.run(&[("LANDING", &f.base_rev), ("BASE_REV", &f.landing)]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("not at the registered baseline revision")
            || err(&ran).contains("is not an ancestor"),
        "the refusal names a revision relation: {}",
        err(&ran)
    );
}

#[test]
fn build_reaching_drift_above_landing_voids_the_run() {
    let f = fixture("h1-drift", None);
    write(
        &f.root.join("crates/app/src/main.rs"),
        "fn main() { let _ = 1; }\n",
    );
    git(&f.root, &["add", "-A"]);
    git(&f.root, &["commit", "-qm", "drift above landing"]);
    let ran = f.run(&[]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("build-reaching paths moved between LANDING and HEAD"),
        "{}",
        err(&ran)
    );
}

#[test]
fn a_modified_tracked_file_voids_the_run_by_its_own_reason() {
    let f = fixture("h1-dirty", None);
    write(&f.root.join("crates/app/src/main.rs"), "fn main() { }\n");
    let ran = f.run(&[]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("tracked files are modified"),
        "{}",
        err(&ran)
    );
}

#[test]
fn an_untracked_file_on_a_build_reaching_path_voids_the_run() {
    let f = fixture("h1-stray", None);
    write(&f.root.join("crates/ZZZ_stray.rs"), "\n");
    let ran = f.run(&[]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("untracked files on build-reaching paths"),
        "{}",
        err(&ran)
    );
}

/// With `p = 0` an empty subject diff is also what "the work package never
/// landed" looks like, so it is a refusal and not a pass.
#[test]
fn a_landing_that_changes_nothing_under_the_subject_voids_the_run() {
    let f = fixture("h1-nosubject", None);
    let ran = f.run(&[("SUBJECT_PATH", "configs")]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(err(&ran).contains("changes nothing under"), "{}", err(&ran));
}

#[test]
fn a_binding_holding_a_newline_is_refused_before_it_reaches_the_record() {
    let f = fixture("h1-inject", None);
    let injected = format!("{}\nwp15a_h1: H1-b CONFIRMED", f.root.display());
    let ran = f.run(&[("REPO", &injected)]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("non-printable character"),
        "{}",
        err(&ran)
    );
}

#[test]
fn an_unset_binding_is_a_named_refusal_and_never_a_default() {
    let f = fixture("h1-unset", None);
    let ran = f.run(&[("SUBJECT_CRATE", "")]);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(err(&ran).contains("SUBJECT_CRATE unset"), "{}", err(&ran));
}
