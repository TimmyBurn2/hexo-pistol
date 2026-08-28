mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// The cap the gate enforces, restated rather than imported: this file is an
/// INPUT to the script, and agreeing with it by construction proves nothing.
const SOFT_CAP: usize = 300;

/// A scratch git repository with the gate at `<root>/tools/`.
///
/// One directory down, exactly as it sits in the real tree: the script roots
/// itself at `dirname/..`, so a copy at the scratch's top level would check the
/// scratch's PARENT (D-203's own finding, in the sibling gate's test).
fn scratch_repo(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    std::fs::copy(
        repo("tools/file_justification_check.sh"),
        root.join("tools/file_justification_check.sh"),
    )
    .expect("the gate copies");
    git(&root, &["init", "-q"]);
    root
}

fn git(root: &Path, args: &[&str]) {
    let ran = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git runs");
    assert!(
        ran.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn gate(root: &Path) -> Output {
    Command::new("bash")
        .arg(root.join("tools/file_justification_check.sh"))
        .current_dir(root)
        .output()
        .expect("the gate runs")
}

/// A file over the cap with no justification comment, in the given comment
/// spelling.
fn over_cap(root: &Path, name: &str, comment: &str) {
    let body: String = std::iter::repeat_n(format!("{comment} filler\n"), SOFT_CAP + 1).collect();
    std::fs::write(root.join(name), body).expect("the over-cap file writes");
}

#[test]
fn the_justification_gate_reads_the_index_and_not_the_working_tree() {
    let root = scratch_repo("justification-index");

    over_cap(&root, "big.rs", "//");
    git(&root, &["add", "big.rs"]);
    // The decoy: what commits is the INDEX, and the worktree copy is now two
    // lines. Before the fix this is what the gate measured.
    std::fs::write(root.join("big.rs"), "// nothing to see here\n").expect("the decoy writes");

    let ran = gate(&root);
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(
        !ran.status.success(),
        "the STAGED bytes are the ones rule 9 applies to:\n{out}"
    );
    assert!(
        out.contains("big.rs: over the cap with no RULE9-JUSTIFICATION: comment"),
        "and the gate names the file and the reason:\n{out}"
    );

    // Control: with the blob out of the index the same repository passes, so the
    // failure above is the detection and not the scaffolding refusing everything.
    git(&root, &["rm", "-qf", "--cached", "big.rs"]);
    let clean = gate(&root);
    assert!(
        clean.status.success(),
        "a repository tracking nothing over the cap passes: {}",
        String::from_utf8_lossy(&clean.stderr)
    );
}

#[test]
fn the_justification_gate_counts_the_shell_scripts_its_summary_says_it_counts() {
    // `.sh` joined the file set in D-234 because the only two files over the cap
    // were shell scripts. Nothing checked that the ENUMERATION reached them: a
    // mutant dropping `*.sh` from it left the summary still saying `.rs/.sh`
    // while counting only `.rs`, and every over-cap script went unseen.
    let root = scratch_repo("justification-shell");

    over_cap(&root, "big.sh", "#");
    git(&root, &["add", "big.sh"]);
    let ran = gate(&root);
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(
        !ran.status.success(),
        "an over-cap shell script is inside this gate's file set:\n{out}"
    );
    assert!(
        out.contains("big.sh: over the cap with no RULE9-JUSTIFICATION: comment"),
        "and the gate names it:\n{out}"
    );

    // And the summary's count is the loop's own, over both extensions.
    git(&root, &["rm", "-qf", "--cached", "big.sh"]);
    std::fs::write(root.join("small.rs"), "fn main() {}\n").expect("a small file writes");
    std::fs::write(root.join("small.sh"), "#!/usr/bin/env bash\n").expect("a small file writes");
    std::fs::write(root.join("notes.txt"), "not in the file set\n").expect("a decoy writes");
    git(&root, &["add", "small.rs", "small.sh", "notes.txt"]);
    let clean = gate(&root);
    let out = String::from_utf8_lossy(&clean.stdout);
    assert!(
        clean.status.success(),
        "nothing here is over the cap: {}",
        String::from_utf8_lossy(&clean.stderr)
    );
    assert!(
        out.contains("file_justification_check: 2 tracked .rs/.sh files, 0 over the cap"),
        "the summary counts one `.rs` and one `.sh` and not the `.txt`:\n{out}"
    );
}
