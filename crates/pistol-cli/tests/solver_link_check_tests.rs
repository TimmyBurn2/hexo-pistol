mod common;

use std::path::{Path, PathBuf};
use std::process::Output;

use common::{repo, repo_root, scratch};

const SUBJECT: &str = "crates/subject";

fn link_check(root: &Path, crate_path: &str) -> Output {
    std::process::Command::new("bash")
        .arg(repo("tools/solver_link_check.sh"))
        .arg(root)
        .arg(crate_path)
        .output()
        .expect("bash runs the shipped script")
}

fn write(path: &Path, text: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("the parent directory");
    }
    std::fs::write(path, text).unwrap_or_else(|e| panic!("cannot write {}: {e}", path.display()));
}

/// How the binary crate reaches — or fails to reach — the subject.
enum Reach {
    /// No relationship at all.
    None,
    /// A normal dependency edge, declared and used.
    Dependency,
    /// A dev-dependency: declared, and reaching no shipped binary.
    DevDependency,
    /// `include_str!` across crate directories, with NO manifest edge — the
    /// route a dependency-graph check cannot see.
    IncludeStr,
}

fn workspace(name: &str, reach: Reach) -> PathBuf {
    let root = scratch(name).join("ws");
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
        "pub fn f() -> u64 {\n    7\n}\n",
    );
    let (manifest_extra, main) = match reach {
        Reach::None => (String::new(), "fn main() {}\n".to_owned()),
        Reach::Dependency => (
            "\n[dependencies]\nsubject = { path = \"../subject\" }\n".to_owned(),
            "fn main() { println!(\"{}\", subject::f()); }\n".to_owned(),
        ),
        Reach::DevDependency => (
            "\n[dev-dependencies]\nsubject = { path = \"../subject\" }\n".to_owned(),
            "fn main() {}\n".to_owned(),
        ),
        Reach::IncludeStr => (
            String::new(),
            "const S: &str = include_str!(\"../../subject/src/lib.rs\");\n\
             fn main() { println!(\"{}\", S.len()); }\n"
                .to_owned(),
        ),
    };
    write(
        &root.join("crates/app/Cargo.toml"),
        &format!(
            "[package]\nname = \"app\"\nversion = \"0.0.1\"\nedition = \"2021\"{manifest_extra}"
        ),
    );
    write(&root.join("crates/app/src/main.rs"), &main);
    let ran = std::process::Command::new("cargo")
        .current_dir(&root)
        .args(["generate-lockfile", "-q"])
        .output()
        .expect("cargo runs");
    assert!(ran.status.success(), "cargo generate-lockfile");
    root
}

/// Assert the gate's exit code, in a message that says what the OTHER codes
/// would have meant.
///
/// `tools/SHELL_CHECKLIST.md` item 12: 0 is "no", 1 is "yes, and here are the
/// files", 2 is "no answer was taken". A bare `assert_eq!(code, Some(0))`
/// reports a VOID as a regression, and that is not hypothetical — a full
/// RAM-backed `/tmp` made this gate exit 2 with `cannot build the workspace's
/// binaries`, and what a reader saw was a red solver-link gate (D-281, D-285).
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — no source under the subject reaches any shipped binary",
        Some(1) => "1 — it does, and the files are named",
        Some(2) => "2 — RUN VOID: the gate could not take an answer, which is NOT a regression",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn out(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stdout).into_owned()
}
fn err(ran: &Output) -> String {
    String::from_utf8_lossy(&ran.stderr).into_owned()
}

/// THE CONTROL. Without it every refusal below is satisfied by a gate that
/// refuses everything — item 10's named failure mode.
#[test]
fn a_workspace_where_nothing_reaches_the_subject_is_accepted() {
    let root = workspace("link-none", Reach::None);
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "nothing reaches the subject\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(out(&ran).contains("NO source under"), "{}", out(&ran));
}

#[test]
fn a_normal_dependency_that_is_used_reaches_the_binary_and_is_named() {
    let root = workspace("link-dep", Reach::Dependency);
    let ran = link_check(&root, SUBJECT);
    assert_eq!(ran.status.code(), Some(1), "stdout: {}", out(&ran));
    assert!(
        out(&ran).contains("subject/src/lib.rs"),
        "the refusal names the file, not a digest: {}",
        out(&ran)
    );
}

/// THE ROUTE THE DEPENDENCY-GRAPH GATE CANNOT SEE. Measured against
/// `tools/solver_edge_check.sh` on the same input: the graph exits 0, this
/// exits 1. That difference is why both gates exist.
#[test]
fn an_include_str_with_no_manifest_edge_reaches_the_binary() {
    let root = workspace("link-include", Reach::IncludeStr);
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "include_str! reaches codegen with no dependency edge\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(
        out(&ran).contains("subject/src/lib.rs"),
        "and the file is named: {}",
        out(&ran)
    );

    // The same input, through the graph gate: it cannot see this.
    let graph = std::process::Command::new("bash")
        .arg(repo("tools/solver_edge_check.sh"))
        .arg(&root)
        .arg("subject")
        .output()
        .expect("bash runs the graph gate");
    assert_eq!(
        graph.status.code(),
        Some(0),
        "the graph gate reports no normal edge, which is why it is not the whole check"
    );
}

/// A dev-dependency reaches no shipped binary, and must not fire.
#[test]
fn a_dev_dependency_does_not_reach_a_shipped_binary() {
    let root = workspace("link-dev", Reach::DevDependency);
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "a dev-dependency is not a shipped-binary input\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
}

/// THE GATE'S OWN BLIND SPOT, AND IT REFUSES RATHER THAN GUESSING. dep-info does
/// not record what a build script READ, so a workspace with one gets "I cannot
/// tell" (2) and never "no" (0).
#[test]
fn a_workspace_with_a_build_script_is_refused_rather_than_answered() {
    let root = workspace("link-buildrs", Reach::None);
    write(&root.join("crates/app/build.rs"), "fn main() {}\n");
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a build script voids the answer\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(
        err(&ran).contains("declares a build script"),
        "the refusal names its own blind spot: {}",
        err(&ran)
    );
}

#[test]
fn an_absolute_crate_path_is_refused() {
    let root = workspace("link-abs", Reach::None);
    let absolute = root.join("crates/subject").display().to_string();
    let ran = link_check(&root, &absolute);
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("must be repository-relative"),
        "{}",
        err(&ran)
    );
}

#[test]
fn a_crate_path_that_does_not_exist_is_refused() {
    let root = workspace("link-missing", Reach::None);
    let ran = link_check(&root, "crates/no-such-crate");
    assert_eq!(ran.status.code(), Some(2), "stdout: {}", out(&ran));
    assert!(
        err(&ran).contains("no such crate directory"),
        "{}",
        err(&ran)
    );
}

/// THE BLIND-SPOT REFUSAL MUST NOT DEPEND ON A FILENAME. `find -name build.rs`
/// does not see a script named by the manifest's `build =` key, and a review
/// drove exactly that past the guard: a `build = "custom_build.rs"` that baked
/// the subject into the binary reached EXIT 0 through the check written to
/// refuse it. Cargo reports a `custom-build` target for both spellings.
#[test]
fn a_build_script_named_by_the_manifest_key_is_refused_like_any_other() {
    let root = workspace("link-buildkey", Reach::None);
    let manifest = root.join("crates/app/Cargo.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest reads");
    write(
        &manifest,
        &text.replace(
            "edition = \"2021\"",
            "edition = \"2021\"\nbuild = \"custom_build.rs\"",
        ),
    );
    write(&root.join("crates/app/custom_build.rs"), "fn main() {}\n");
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a build script under any name voids the answer\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(
        err(&ran).contains("declares a build script"),
        "the refusal names its blind spot: {}",
        err(&ran)
    );
}

/// A SYMLINKED SOURCE MUST NOT BE RESOLVED OUT OF THE CRATE IT LIVES IN.
/// `realpath -m` follows links, so a review made `crates/subject/src/lib.rs` a
/// symlink to `crates/shared/real.rs`, dep-info recorded the path under the
/// crate, canonicalisation resolved it away, and the gate answered EXIT 0 on a
/// tree whose binary observed the subject. `-ms` normalises `..` without
/// following links.
#[test]
fn a_symlinked_source_under_the_crate_still_counts_as_the_crate() {
    let root = workspace("link-symlink", Reach::IncludeStr);
    write(
        &root.join("crates/shared/real.rs"),
        "pub fn f() -> u64 {\n    7\n}\n",
    );
    let link = root.join("crates/subject/src/lib.rs");
    std::fs::remove_file(&link).expect("the real file is removed");
    #[cfg(unix)]
    std::os::unix::fs::symlink("../../shared/real.rs", &link).expect("the symlink is created");
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a symlinked source under the crate still reaches the binary\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
}

/// THE BREADTH CLAIM IS CHECKED, NOT ASSERTED. `cargo build --workspace --bins`
/// silently skips a bin whose `required-features` are unmet, so the gate would
/// print a narrower count and answer 0 with no hint it had narrowed.
#[test]
fn a_binary_behind_an_unmet_feature_is_refused_rather_than_silently_skipped() {
    let root = workspace("link-gated", Reach::None);
    let manifest = root.join("crates/app/Cargo.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest reads");
    write(
        &manifest,
        &format!(
            "{text}\n[features]\ngated = []\n\n[[bin]]\nname = \"app\"\npath = \"src/main.rs\"\n\n\
             [[bin]]\nname = \"gated-bin\"\npath = \"src/gated.rs\"\nrequired-features = [\"gated\"]\n"
        ),
    );
    write(&root.join("crates/app/src/gated.rs"), "fn main() {}\n");
    let ran = link_check(&root, SUBJECT);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a declared-but-unbuilt binary voids the answer\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
    assert!(
        err(&ran).contains("declares 2 binary targets"),
        "the refusal names the arithmetic: {}",
        err(&ran)
    );
}

// --- the three fixture classes D-281's pattern diagnosis names -----------------

/// Add a member that is NOT in `default-members` and whose binary reads the
/// subject.
///
/// KILLS S2 — dropping `--workspace` from the gate's build. Every fixture above
/// is a workspace whose default members ARE all its members, so `cargo build
/// --bins` and `cargo build --workspace --bins` do the same thing and no test
/// can tell the two invocations apart. Here they differ: without `--workspace`
/// the `extra` binary is never built, its dep-info never read, and the source
/// that reaches it never seen.
fn add_non_default_member(root: &Path) {
    write(
        &root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/subject\", \"crates/app\", \"crates/extra\"]\n\
         default-members = [\"crates/app\"]\nresolver = \"2\"\n",
    );
    write(
        &root.join("crates/extra/Cargo.toml"),
        "[package]\nname = \"extra\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    write(
        &root.join("crates/extra/src/main.rs"),
        "const S: &str = include_str!(\"../../subject/src/lib.rs\");\n\
         fn main() { println!(\"{}\", S.len()); }\n",
    );
    relock(root);
}

fn relock(root: &Path) {
    let ran = std::process::Command::new("cargo")
        .current_dir(root)
        .args(["generate-lockfile", "-q"])
        .output()
        .expect("cargo runs");
    assert!(
        ran.status.success(),
        "cargo generate-lockfile: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
}

#[test]
fn a_binary_in_a_non_default_workspace_member_is_still_examined() {
    let root = workspace("link-nondefault", Reach::None);
    add_non_default_member(&root);
    let ran = link_check(&root, SUBJECT);
    assert_code(
        &ran,
        1,
        "a member outside `default-members` still ships a binary, so `--workspace` is what \
         makes the answer cover it",
    );
    assert!(
        out(&ran).contains("subject/src/lib.rs"),
        "and the file is named: {}",
        out(&ran)
    );
    // The breadth arithmetic saw both, which is the half a narrowed build would
    // have reported without saying it had narrowed.
    assert!(
        out(&ran).contains("2 shipped binaries"),
        "both binaries were examined: {}",
        out(&ran)
    );
}

/// Add `crates/subject-x`, a crate whose directory name has the subject's as a
/// PREFIX, and make the binary read IT and not the subject.
///
/// KILLS S10 — the hit match losing its trailing slash. `case "$abs" in
/// "$CRATE_ABS"/*)` becomes `"$CRATE_ABS"*)`, and `…/crates/subject-x/src/lib.rs`
/// then matches `…/crates/subject`. Every fixture above holds one crate under
/// `crates/`, so no fixture could produce the collision.
fn add_prefix_sibling(root: &Path) {
    write(
        &root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/subject\", \"crates/subject-x\", \"crates/app\"]\n\
         resolver = \"2\"\n",
    );
    write(
        &root.join("crates/subject-x/Cargo.toml"),
        "[package]\nname = \"subject-x\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    write(
        &root.join("crates/subject-x/src/lib.rs"),
        "pub fn g() -> u64 {\n    9\n}\n",
    );
    write(
        &root.join("crates/app/src/main.rs"),
        "const S: &str = include_str!(\"../../subject-x/src/lib.rs\");\n\
         fn main() { println!(\"{}\", S.len()); }\n",
    );
    relock(root);
}

#[test]
fn a_sibling_crate_sharing_a_name_prefix_is_not_the_subject() {
    let root = workspace("link-prefix", Reach::None);
    add_prefix_sibling(&root);

    // The subject itself is untouched, and `crates/subject-x` is not it.
    let ran = link_check(&root, SUBJECT);
    assert_code(
        &ran,
        0,
        "`crates/subject-x` is a different crate; a match that dropped the separator would \
         report it as the subject",
    );

    // AND THE FIXTURE IS NOT VACUOUS: the sibling really does reach the binary,
    // which is what makes the exit 0 above a discrimination rather than a tree
    // where nothing reaches anything.
    let sibling = link_check(&root, "crates/subject-x");
    assert_code(&sibling, 1, "the sibling itself does reach the binary");
    assert!(
        out(&sibling).contains("subject-x/src/lib.rs"),
        "and is named: {}",
        out(&sibling)
    );
}

/// How a dep-info file is broken.
enum Corrupt {
    /// Zero bytes.
    Empty,
    /// A well-formed target and separator, and no inputs after it.
    NoInputs,
    /// A SECOND `": "` in the first line. Real dep-info has exactly one, which
    /// is why a first-separator split and a last-separator split agree on every
    /// honest line and disagree only here.
    SecondSeparator,
}

/// Break `app`'s dep-info as the gate reads it, and return the directory to put
/// in front of `PATH`.
///
/// MEASURED, and it is why this is a shim rather than a `std::fs::write`: CARGO
/// REWRITES `target/debug/app.d` ON EVERY BUILD, including one that recompiles
/// nothing — the file is copied out of the fingerprint directory each time. A
/// corruption written before the gate runs is gone by the time the gate looks
/// (checked directly: write `CORRUPT`, build again, read the honest first line
/// back). So the corruption is applied by a `cargo` shim, AFTER the real cargo
/// has run and before it returns, which is exactly the moment a real one would
/// arrive — a truncated write, a full filesystem, a concurrent process.
///
/// The shim forwards EVERY invocation to the real cargo, so `cargo metadata` —
/// which this gate also runs, and which must keep working — is untouched; only
/// a `build` triggers the overwrite.
fn corrupt_dep_info(root: &Path, how: Corrupt) -> PathBuf {
    let built = std::process::Command::new(env!("CARGO"))
        .current_dir(root)
        .args(["build", "--locked", "--workspace", "--bins", "-q"])
        .output()
        .expect("cargo runs");
    assert!(
        built.status.success(),
        "the fixture builds before it is broken: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    let exe = root.join("target/debug/app");
    assert!(exe.is_file(), "the fixture's binary is where cargo put it");
    let dep = root.join("target/debug/app.d");
    assert!(dep.is_file(), "and its dep-info is beside it");

    let text = match how {
        Corrupt::Empty => String::new(),
        Corrupt::NoInputs => format!("{}: \n", exe.display()),
        Corrupt::SecondSeparator => format!(
            "{}: {}: {}\n",
            exe.display(),
            root.join("crates/subject/src/lib.rs").display(),
            root.join("crates/app/src/main.rs").display()
        ),
    };
    let shim_dir = root.join("shim");
    let corrupt = shim_dir.join("corrupt.dep-info");
    write(&corrupt, &text);
    let shim = shim_dir.join("cargo");
    write(
        &shim,
        &format!(
            "#!/usr/bin/env bash\n\
             set -uo pipefail\n\
             \"{real}\" \"$@\"\n\
             status=$?\n\
             case \" $* \" in *\" build \"*) cp -- \"{corrupt}\" \"{dep}\" ;; esac\n\
             exit \"$status\"\n",
            real = env!("CARGO"),
            corrupt = corrupt.display(),
            dep = dep.display()
        ),
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&shim, std::fs::Permissions::from_mode(0o755))
            .expect("the shim is executable");
    }
    shim_dir
}

/// The gate, with `shim_dir` in front of `PATH`.
fn link_check_shimmed(root: &Path, crate_path: &str, shim_dir: &Path) -> Output {
    let path = std::env::var("PATH").unwrap_or_default();
    std::process::Command::new("bash")
        .arg(repo("tools/solver_link_check.sh"))
        .arg(root)
        .arg(crate_path)
        .env("PATH", format!("{}:{path}", shim_dir.display()))
        .output()
        .expect("bash runs the shipped script")
}

/// KILLS S5 — the `[ -s "$dep" ]` refusal. Without it an empty file parses to an
/// empty input list and the gate falls through to the per-binary floor, which
/// refuses for a DIFFERENT reason: same exit code, wrong diagnosis, and an
/// operator sent to look at the parse rather than at the file.
#[test]
fn an_empty_dep_info_file_is_refused_by_its_own_name() {
    let root = workspace("link-dep-empty", Reach::None);
    let shim = corrupt_dep_info(&root, Corrupt::Empty);
    let ran = link_check_shimmed(&root, SUBJECT, &shim);
    assert_code(&ran, 2, "an unreadable input list is a void, not a `no`");
    assert!(
        err(&ran).contains("no dep-info beside"),
        "refused for what it is — an empty file, not a failed parse: {}",
        err(&ran)
    );
}

/// KILLS A5 — the per-binary input floor. A dep-info that is non-empty but lists
/// nothing gets past the size check, and without the floor the gate reports
/// `0 source inputs` for that binary and answers 0 with no hint it read nothing.
#[test]
fn a_dep_info_listing_no_inputs_is_refused_rather_than_counted_as_zero() {
    let root = workspace("link-dep-noinputs", Reach::None);
    let shim = corrupt_dep_info(&root, Corrupt::NoInputs);
    let ran = link_check_shimmed(&root, SUBJECT, &shim);
    assert_code(&ran, 2, "a binary with no recorded inputs is a void");
    assert!(
        err(&ran).contains("listed no source inputs"),
        "and the refusal is the floor's: {}",
        err(&ran)
    );
}

/// KILLS S29 — `${line#*: }` becoming `${line##*: }`, which keeps only the LAST
/// input per binary.
///
/// The mutation is INVISIBLE on well-formed dep-info, because rustc writes
/// exactly one `": "` per first line (a path containing a space is escaped as
/// `\ `, so `": "` cannot occur inside one) — first-separator and
/// last-separator agree on every honest file, and that is why eleven tests left
/// this alive. A second separator separates them: splitting at the FIRST yields
/// an entry ending in `:` that names nothing on disk, which the gate now refuses
/// by name; splitting at the LAST silently drops the subject and answers 0.
#[test]
fn a_dep_info_first_line_with_a_second_separator_is_refused_rather_than_half_read() {
    let root = workspace("link-dep-separator", Reach::None);
    let shim = corrupt_dep_info(&root, Corrupt::SecondSeparator);
    let ran = link_check_shimmed(&root, SUBJECT, &shim);
    assert_code(
        &ran,
        2,
        "a first line this parser cannot split unambiguously is a void, and answering 0 \
         would be an answer about a file set nobody built",
    );
    assert!(
        err(&ran).contains("names nothing on disk"),
        "refused at the entry that is not a path: {}",
        err(&ran)
    );
}

// --- two more of D-281's survivors, closed with the fixtures they needed -------

/// KILLS S4 — dropping `--locked` from the build.
///
/// Every fixture above locks itself immediately after it is written, so
/// `--locked` and its absence do the same thing and no test could tell them
/// apart. Here the workspace gains a member and the lockfile is deliberately NOT
/// regenerated: `--locked` refuses to update it and the gate takes no answer,
/// while without `--locked` cargo silently relocks, builds the new binary and
/// answers about a dependency graph the committed lockfile does not describe —
/// which is the whole reason a gate that adjudicates builds `--locked`.
///
/// `cargo metadata --no-deps` runs first and does NOT resolve, so it leaves the
/// lockfile alone; measured, not assumed.
#[test]
fn a_stale_lockfile_voids_the_answer_rather_than_being_relocked() {
    let root = workspace("link-stale-lock", Reach::None);
    // A member cargo has never resolved, and no `generate-lockfile` after it.
    write(
        &root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/subject\", \"crates/app\", \"crates/extra\"]\n\
         resolver = \"2\"\n",
    );
    write(
        &root.join("crates/extra/Cargo.toml"),
        "[package]\nname = \"extra\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    write(&root.join("crates/extra/src/main.rs"), "fn main() {}\n");

    let ran = link_check(&root, SUBJECT);
    assert_code(
        &ran,
        2,
        "a lockfile that does not describe the workspace is a void; relocking it silently \
         would answer about a graph nobody committed",
    );
    assert!(
        err(&ran).contains("cannot build the workspace's binaries"),
        "and the build is where it stops: {}",
        err(&ran)
    );
}

/// KILLS S11 — the root-`Cargo.toml` refusal.
///
/// Removing it does not make the gate answer: `cargo metadata` fails a moment
/// later and the gate still voids. What moves is the DIAGNOSIS — "no Cargo.toml
/// at the workspace root" becomes "cannot read the workspace metadata", which
/// sends a reader to look at cargo rather than at the path they passed. One
/// refusal per reason (tools/SHELL_CHECKLIST.md item 8) is a claim about the
/// message, so the test is about the message.
#[test]
fn a_root_with_no_manifest_is_refused_by_the_manifest_and_not_by_cargo() {
    let root = scratch("link-no-manifest").join("not-a-workspace");
    std::fs::create_dir_all(root.join("crates/subject")).expect("the directory tree");
    let ran = link_check(&root, SUBJECT);
    assert_code(&ran, 2, "a directory that is not a workspace is a void");
    assert!(
        err(&ran).contains("no Cargo.toml at the workspace root"),
        "named at the path the caller passed, not at cargo's expense: {}",
        err(&ran)
    );
}

/// `docs/experiments/U1_gate_supersession.md` §4.4, OPTION (f)'s LINK HALF: A
/// DERIVED HIT-SET INVARIANT, not a live "reaches nothing" assertion.
///
/// WP-1.5b (docs/decisions.md D-310) makes `pistol-solver` a normal input to
/// every shipped binary — U1_gate_supersession.md §4.1 measured the resulting
/// transcript before the edge landed ("30 hits over 5 binaries") so this is
/// not a surprise. What survives as a standing invariant is narrower and
/// stronger than "reaches nothing": the set of `pistol-solver` files this gate
/// finds reaching a shipped binary is EXACTLY the crate's own `src/` file set —
/// an EXTERNALLY DERIVED referent (the filesystem, not the gate's own prior
/// output), which is what CLAUDE.md's process section asks a reviewer to look
/// for first. A stray file reaching a binary through something other than the
/// crate's own compiled sources (`U1_gate_supersession.md` §4.3's
/// `include_str!` construction, the residual class this gate exists to
/// catch) would show up as a hit line naming a path OUTSIDE this set; a
/// source file no longer compiled into any binary would be missing from the
/// hit set while still present on disk. Either is a red run.
#[test]
fn the_solver_hit_set_is_exactly_its_own_src_files_reaching_every_shipped_binary() {
    let ran = link_check(&repo_root(), "crates/pistol-solver");
    assert_code(
        &ran,
        1,
        "pistol-search's dependency on pistol-solver (docs/decisions.md D-310) makes this \
         crate's sources reach every shipped binary; a 0 here would mean the edge went missing",
    );
    let stdout = out(&ran);
    assert!(
        stdout.contains("solver_link_check: 10 shipped binaries,"),
        // The count is RE-DERIVED at every merge that adds a binary, never
        // carried from either side: six became seven twice over on two open
        // branches (this WP's `solver-cost`, the corpus work's own additions)
        // and a merge that kept either side's number would compile and assert
        // a false one — which is what this gate exists to refuse. Nine became
        // TEN when the corpus loader landed (`corpus-check`, autodiscovered
        // from `crates/pistol-arena/src/bin/`), and it reaches the solver by
        // the same route every other binary does.
        "this workspace ships ten binaries, machine-invariant across a run: {stdout}"
    );

    // The externally derived referent: the crate's own `src/` tree,
    // enumerated by this test and not read from the gate's output. Recursive
    // since the solver crate gained `src/bin/` with its selftest binary
    // (WP-1.8a): a flat read_dir would miss the bin target the hit set
    // legitimately reaches.
    let mut on_disk: Vec<String> = Vec::new();
    let mut queue = vec![repo("crates/pistol-solver/src")];
    while let Some(dir) = queue.pop() {
        for entry in std::fs::read_dir(&dir)
            .unwrap_or_else(|error| panic!("{} must read: {error}", dir.display()))
        {
            let entry = entry.expect("a directory entry reads");
            let path = entry.path();
            if path.is_dir() {
                queue.push(path);
                continue;
            }
            if entry.file_name().to_string_lossy().ends_with(".rs") {
                on_disk.push(
                    path.strip_prefix(repo_root())
                        .expect("under the repo root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    on_disk.sort();
    on_disk.dedup();
    assert!(
        !on_disk.is_empty(),
        "the referent itself must be non-empty, or this test proves nothing"
    );

    // The hit set the gate reports, canonicalised down to the repo-relative
    // subject path and deduplicated across the shipped binaries — a hit line
    // repeats per binary by design, which is not a second file.
    let mut hit: Vec<String> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("solver_link_check:   "))
        .filter_map(|line| line.split_once(" <- "))
        .map(|(path, _binary)| path)
        .filter_map(|path| path.split_once("crates/pistol-solver/"))
        .map(|(_prefix, rest)| format!("crates/pistol-solver/{rest}"))
        .collect();
    hit.sort();
    hit.dedup();

    assert_eq!(
        hit, on_disk,
        "the set of pistol-solver files reaching a shipped binary must be exactly its own \
         src/ files — nothing more (a stray non-source input) and nothing less (a source file \
         no binary actually compiles)"
    );
}

/// A RELATIVE DEP-INFO ENTRY IS ANCHORED TO THE ROOT THE GATE WAS GIVEN, NOT TO
/// THE CALLER'S WORKING DIRECTORY.
///
/// `build.dep-info-basedir` is a supported cargo config key, and with it set
/// cargo emits relative prerequisites. `realpath -ms` resolves those against the
/// PROCESS's cwd, and the entry loop runs wherever the caller stood — the two
/// `cargo` calls above it `cd` only inside their own subshells. Measured at
/// 6b03899 on this exact workspace: from a foreign cwd the gate answered exit 2
/// («names nothing on disk»), and from the workspace root it answered exit 1,
/// the correct answer — the same tree and the same arguments, three verdicts as
/// a function of `cd` (exit 0 with the WRONG answer before the S29 existence
/// check landed). This drives the gate from a directory that is neither.
///
/// Without a fixture that sets the key, an anchored resolve and a cwd-relative
/// one are indistinguishable on every well-formed line, which is the same shape
/// as the S29 mutant that survived eleven tests.
#[test]
fn a_relative_dep_info_entry_is_resolved_against_the_given_root() {
    let root = workspace("link-relative-depinfo", Reach::IncludeStr);
    write(
        &root.join(".cargo/config.toml"),
        "[build]\ndep-info-basedir = \".\"\n",
    );

    // Driven from a directory that is neither the workspace nor the repository,
    // so a cwd-relative resolve cannot accidentally be right.
    let elsewhere = scratch("link-relative-depinfo-cwd");
    let ran = std::process::Command::new("bash")
        .arg(repo("tools/solver_link_check.sh"))
        .arg(&root)
        .arg(SUBJECT)
        .current_dir(&elsewhere)
        .output()
        .expect("bash runs the shipped script");

    assert_code(
        &ran,
        1,
        "the subject reaches the binary through include_str!, and relative \
         dep-info does not change that",
    );
    let said = format!("{}{}", out(&ran), String::from_utf8_lossy(&ran.stderr));
    assert!(
        said.contains("REACHES a shipped binary"),
        "and the verdict is the reaching one, not a void:\n{said}"
    );
}
