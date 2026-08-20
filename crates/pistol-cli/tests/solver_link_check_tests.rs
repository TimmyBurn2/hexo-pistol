//! `tools/solver_link_check.sh` — does any source of one crate reach a shipped
//! binary? (docs/decisions.md D-276; tools/SHELL_CHECKLIST.md item 10.)
//!
//! # Why this gate exists and the two before it did not survive
//!
//! WP-1.5a's whole claim is that the threat generator adds nothing a shipped
//! binary can observe. Two instruments were built for it and a fresh context
//! broke each:
//!
//!   - a DEPENDENCY-GRAPH check (`tools/solver_edge_check.sh`) asks whether the
//!     crate is LINKED. A crate with no manifest edge at all still reaches a
//!     binary through `include!`/`include_str!`, and that was reproduced: the
//!     graph answered "no normal reverse-dependency, exit 0" on a tree where
//!     mutating the crate moved the binary's digest.
//!   - a TWO-BUILD DIGEST comparison has a measured FALSE CONFIRMED inside its
//!     own stated coverage: sharing one target directory — which is how it was
//!     written — a build script that reads the crate without declaring
//!     `rerun-if-changed` leaves a stale `OUT_DIR` artefact, the second build
//!     compiles nothing, and two identical digests are reported for a binary
//!     whose behaviour moved.
//!
//! This asks rustc's own bookkeeping instead: every binary cargo builds gets a
//! dep-info file listing every source that went into it. It costs no extra
//! build, covers all five shipped binaries rather than one, and NAMES THE FILE.
//!
//! # The two defects that were designed out, both found by building it
//!
//! rustc records paths AS WRITTEN, so a source reached from `src/bin/` appears
//! as `…/src/bin/../../../pistol-solver/src/lib.rs` and a plain substring match
//! returns ZERO hits on the file it exists to catch. Every entry is canonicalised
//! first. And an unscoped `target/*.d` glob picks up `libpistol_solver.d` — the
//! subject's own library — so the binary set is taken from cargo's own JSON.
//!
//! # RULE9-JUSTIFICATION: one gate's readings, over one claim.
//! Every test is the same claim — that this gate separates "reaches a shipped
//! binary" from "linked but unreached", from "not linked at all", and from "I
//! cannot tell" — and each needs the same synthetic workspace builder.

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

/// THE STANDING INVARIANT, over the real workspace, on every commit: no source
/// of the threat generator is an input to any of the five binaries this
/// workspace ships. This is what WP-1.5a's H1 claimed, checked continuously
/// rather than once in a governed run (D-276).
#[test]
fn no_solver_source_reaches_any_shipped_binary_of_this_workspace() {
    let ran = link_check(&repo_root(), "crates/pistol-solver");
    assert_eq!(
        ran.status.code(),
        Some(0),
        "no pistol-solver source may be an input to a shipped binary\nstdout: {}\nstderr: {}",
        out(&ran),
        err(&ran)
    );
}
