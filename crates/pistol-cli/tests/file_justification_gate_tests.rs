mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// The cap the gate enforces, restated rather than imported: this file is an
/// INPUT to the script, and agreeing with it by construction proves nothing.
const SOFT_CAP: usize = 300;

/// Where the gate reads every why from.
const REGISTRY: &str = "docs/rule9_justifications.md";

/// A scratch git repository with the gate at `<root>/tools/` and an empty
/// registry staged.
///
/// One directory down, exactly as it sits in the real tree: the script roots
/// itself at `dirname/..`, so a copy at the scratch's top level would check the
/// scratch's PARENT (D-203's own finding, in the sibling gate's test).
fn scratch_repo(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    std::fs::create_dir_all(root.join("docs")).expect("a docs directory");
    std::fs::copy(
        repo("tools/file_justification_check.sh"),
        root.join("tools/file_justification_check.sh"),
    )
    .expect("the gate copies");
    git(&root, &["init", "-q"]);
    registry(&root, "");
    root
}

/// Writes the registry and stages it, since the index is what the gate reads.
fn registry(root: &Path, entries: &str) {
    registry_bytes(root, &format!("prose\n{entries}"));
}

/// The same, byte for byte, for the cases where the trailing newline is the
/// subject rather than an incidental.
fn registry_bytes(root: &Path, body: &str) {
    std::fs::write(root.join(REGISTRY), body).expect("the registry writes");
    git(root, &["add", REGISTRY]);
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

fn text(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// A file over the cap, in the given comment spelling.
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
    let out = text(&ran);
    assert!(
        !ran.status.success(),
        "the STAGED bytes are the ones rule 9 applies to:\n{out}"
    );
    assert!(
        out.contains(&format!("big.rs: over the cap with no entry in {REGISTRY}")),
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
fn the_registry_is_read_from_the_index_too_and_a_why_there_is_what_clears_a_file() {
    let root = scratch_repo("justification-registry");

    over_cap(&root, "big.rs", "//");
    git(&root, &["add", "big.rs"]);
    registry(
        &root,
        "- `big.rs`: one recursion whose parts are not independent\n",
    );

    let ran = gate(&root);
    let out = text(&ran);
    assert!(
        ran.status.success(),
        "a registered file clears the cap:\n{out}"
    );
    assert!(
        out.contains("over the cap and registered: big.rs"),
        "and the gate says which file the entry cleared:\n{out}"
    );

    // The same decoy the file set gets: an UNSTAGED registry is not the one a
    // fresh clone would be judged on, so removing the entry from the worktree
    // alone must not change the verdict.
    std::fs::write(root.join(REGISTRY), "prose only\n").expect("the decoy writes");
    let still = gate(&root);
    assert!(
        still.status.success(),
        "the index still carries the entry:\n{}",
        text(&still)
    );

    // And staging that removal is what makes it bite.
    git(&root, &["add", REGISTRY]);
    let now = gate(&root);
    assert!(
        !now.status.success(),
        "with the entry gone from the index the file is unjustified again:\n{}",
        text(&now)
    );
}

#[test]
fn an_entry_that_argues_about_nothing_is_refused() {
    let root = scratch_repo("justification-stale");

    // Under the cap: rule 9 asks nothing of it, so a why for it is a claim that
    // does no work — the drift a single home is supposed to keep visible.
    std::fs::write(root.join("small.rs"), "fn main() {}\n").expect("a small file writes");
    git(&root, &["add", "small.rs"]);
    registry(
        &root,
        "- `small.rs`: a why for a file nobody asked one of\n",
    );
    let under = gate(&root);
    assert!(
        text(&under).contains("an entry for small.rs, which is under the cap"),
        "{}",
        text(&under)
    );
    assert!(!under.status.success());

    registry(&root, "- `gone.rs`: a why for a file that is not there\n");
    let untracked = gate(&root);
    assert!(
        text(&untracked).contains("an entry for gone.rs, which nothing tracks"),
        "{}",
        text(&untracked)
    );
    assert!(!untracked.status.success());
}

#[test]
fn two_whys_for_one_file_and_a_malformed_entry_are_both_refused() {
    let root = scratch_repo("justification-malformed");

    over_cap(&root, "big.rs", "//");
    git(&root, &["add", "big.rs"]);
    registry(
        &root,
        "- `big.rs`: the first why\n- `big.rs`: the second why\n",
    );
    let doubled = gate(&root);
    assert!(
        text(&doubled).contains("big.rs has two entries"),
        "{}",
        text(&doubled)
    );
    assert!(!doubled.status.success());

    // A near-miss entry must refuse rather than read as prose: silently skipped,
    // it would report the file it names as having no why at all, and its author
    // would be hunting the wrong defect (CLAUDE.md rule 3).
    registry(&root, "- big.rs: the backticks are missing\n");
    let malformed = gate(&root);
    assert!(
        text(&malformed).contains("an entry that is not of the form"),
        "{}",
        text(&malformed)
    );
    assert!(!malformed.status.success());
}

#[test]
fn a_registry_the_index_does_not_carry_is_a_named_refusal() {
    let root = scratch_repo("justification-no-registry");
    git(&root, &["rm", "-qf", "--cached", REGISTRY]);
    let ran = gate(&root);
    let out = text(&ran);
    assert!(!ran.status.success(), "{out}");
    assert!(
        out.contains(&format!("{REGISTRY} is not in the index")),
        "the gate refuses by name rather than passing a tree it never read:\n{out}"
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
    let out = text(&ran);
    assert!(
        !ran.status.success(),
        "an over-cap shell script is inside this gate's file set:\n{out}"
    );
    assert!(
        out.contains(&format!("big.sh: over the cap with no entry in {REGISTRY}")),
        "and the gate names it:\n{out}"
    );

    // And the summary's count is the loop's own, over both extensions.
    git(&root, &["rm", "-qf", "--cached", "big.sh"]);
    std::fs::write(root.join("small.rs"), "fn main() {}\n").expect("a small file writes");
    std::fs::write(root.join("small.sh"), "#!/usr/bin/env bash\n").expect("a small file writes");
    std::fs::write(root.join("notes.txt"), "not in the file set\n").expect("a decoy writes");
    git(&root, &["add", "small.rs", "small.sh", "notes.txt"]);
    let clean = gate(&root);
    let out = text(&clean);
    assert!(
        clean.status.success(),
        "nothing here is over the cap:\n{out}"
    );
    assert!(
        out.contains("file_justification_check: 2 tracked .rs/.sh files, 0 over the cap"),
        "the summary counts one `.rs` and one `.sh` and not the `.txt`:\n{out}"
    );
}

#[test]
fn the_last_entry_is_read_even_when_no_newline_terminates_it() {
    // A `read` loop without the `-n` guard drops an unterminated final line, and
    // the three refusals below then report a registry they were built to fail as
    // clean. Each is asserted at the END of the file, which is the only position
    // the defect reaches.
    let root = scratch_repo("justification-unterminated");
    over_cap(&root, "big.rs", "//");
    git(&root, &["add", "big.rs"]);

    let registered = "- `big.rs`: one recursion whose parts are not independent";
    for (tail, expected) in [
        (
            "- `gone.rs`: a why for a file that is not there",
            "which nothing tracks",
        ),
        ("- `big.rs`: a second why", "big.rs has two entries"),
        (
            "- big.rs: the backticks are missing",
            "an entry that is not of the form",
        ),
    ] {
        registry_bytes(&root, &format!("prose\n{registered}\n{tail}"));
        let ran = gate(&root);
        let out = text(&ran);
        assert!(
            !ran.status.success(),
            "an unterminated last line is still a line:\n{out}"
        );
        assert!(out.contains(expected), "{out}");
    }

    // Control: the same registry without the offending tail, also unterminated,
    // passes — so the refusals above are the detection and not the missing
    // newline refusing everything.
    registry_bytes(&root, &format!("prose\n{registered}"));
    let clean = gate(&root);
    assert!(clean.status.success(), "{}", text(&clean));
}

#[test]
fn a_why_that_counts_lines_and_a_why_that_says_nothing_are_both_refused() {
    // Rule 9 derives counts and never asserts them, and an entry with no why is
    // an entry that answers nothing. Both arms lived only in the script's own
    // seeded self-test, so a mutant of either failed every test here for the
    // wrong reason.
    let root = scratch_repo("justification-why-shape");
    over_cap(&root, "big.rs", "//");
    git(&root, &["add", "big.rs"]);

    registry(
        &root,
        "- `big.rs`: it is 348 lines and every one earns its place\n",
    );
    let counted = gate(&root);
    assert!(
        text(&counted).contains("states a line count, and counts are derived"),
        "{}",
        text(&counted)
    );
    assert!(!counted.status.success());

    registry(&root, "- `big.rs`: \n");
    let empty = gate(&root);
    assert!(text(&empty).contains("carries no why"), "{}", text(&empty));
    assert!(!empty.status.success());
}

#[test]
fn an_entry_shaped_like_a_stage_spec_is_refused_for_the_reason_it_deserves() {
    // `git cat-file -e ":0:big.rs"` resolves the stage rather than the path, so
    // the gate used to answer about a file the entry does not name.
    let root = scratch_repo("justification-stage-spec");
    over_cap(&root, "big.rs", "//");
    git(&root, &["add", "big.rs"]);
    registry(
        &root,
        "- `big.rs`: one recursion whose parts are not independent\n\
         - `0:big.rs`: a path spelled as a stage specifier\n",
    );
    let ran = gate(&root);
    let out = text(&ran);
    assert!(!ran.status.success(), "{out}");
    assert!(
        out.contains("an entry for 0:big.rs, which nothing tracks"),
        "the refusal names the reason that is true of the path as written:\n{out}"
    );
}
