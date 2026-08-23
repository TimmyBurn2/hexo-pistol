//! `tools/decision_key_check.sh` — CI gate 15 of 16, the decision log's own
//! integrity (docs/decisions.md D-279, D-284; tools/SHELL_CHECKLIST.md item 10).
//!
//! # What is being guarded, and why a gate rather than care
//!
//! `D-276` and `D-277` were each appended TWICE, in one commit, with DIFFERENT
//! TEXT. The mechanism was mundane — a two-step commit whose second step failed
//! and was re-run — and nothing anywhere checked. Every ADR reference in this
//! repository is by number, so a key denoting two texts breaks the one property
//! the log is cited for.
//!
//! # The exemption is part of the specification and so it is part of the control
//!
//! D-279 refused to DELETE the repeated copies, on the log's own "lines are
//! never edited or deleted" rule, and resolved the ambiguity by ruling that the
//! second copy of each is operative. So the file the gate guards contains two
//! repeated keys BY DECISION, and a gate spelled exactly as D-279 registered it
//! is red on arrival (D-284). What the gate enforces instead is: no repeat
//! outside that closed, named pair — checked in BOTH directions, so an exemption
//! whose subject is gone is refused too and the list cannot become the place
//! repeats go.
//!
//! Which is why the CONTROL below seeds a document holding exactly those two
//! repeats: the shipped gate's passing case is the shipped file's shape.
//!
//! # RULE9-JUSTIFICATION: one gate, one scratch repository.
//! Every test drives the same shipped script over the same scratch git
//! repository builder, and the three verdicts — clean, an unexpected repeat, a
//! stale exemption — are one claim about one comparison that cannot be split
//! without splitting the builder with it.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// The two keys the shipped gate exempts, restated rather than imported: this
/// file is a CHECK on the script, and agreeing with it by construction proves
/// nothing.
const GRANDFATHERED: [&str; 2] = ["D-276", "D-277"];

/// A scratch git repository with the gate at `<root>/tools/`.
///
/// One directory down, exactly as it sits in the real tree: the script roots
/// itself at `dirname/..`, so a copy at the scratch's top level would check the
/// scratch's PARENT.
fn scratch_repo(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    std::fs::create_dir_all(root.join("docs")).expect("a docs directory");
    std::fs::copy(
        repo("tools/decision_key_check.sh"),
        root.join("tools/decision_key_check.sh"),
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
        .arg(root.join("tools/decision_key_check.sh"))
        .current_dir(root)
        .output()
        .expect("the gate runs")
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// A decision log holding `lines`, written and STAGED — the gate reads the
/// index, so an unstaged file is invisible to it.
fn write_log(root: &Path, lines: &[String]) {
    let body: String = lines.iter().map(|line| format!("{line}\n")).collect();
    std::fs::write(root.join("docs/decisions.md"), body).expect("the log writes");
    git(root, &["add", "docs/decisions.md"]);
}

/// A log of the shape the shipped one has: keys in order, with the two
/// grandfathered ones appearing twice with different text.
fn shipped_shape() -> Vec<String> {
    let mut lines = vec![String::from("# Decisions"), String::new()];
    for n in 270..280 {
        lines.push(format!("D-{n}: a choice — a reason — what flips it"));
    }
    for key in GRANDFATHERED {
        lines.push(format!("{key}: the second copy, whose text differs"));
    }
    lines
}

/// Assert the gate's exit code, in a message that says what the OTHER codes
/// would have meant.
///
/// `tools/SHELL_CHECKLIST.md` item 12 obligation 3. `!status.success()` is
/// satisfied by 1, by 2, by 127 and by a signal death alike, so a suite written
/// that way cannot see a RUN VOID reported as a regression — which is the whole
/// of the item. Lifted from `solver_link_check_tests.rs`, which this diff got
/// right, into the suite it did not.
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — every key denotes one text",
        Some(1) => "1 — a key repeats, or the extraction is wrong; an ANSWER, and it is no",
        Some(2) => "2 — RUN VOID: no answer was taken, which is NOT a duplicate key",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\n{}",
        said(ran)
    );
}

/// THE CONTROL. Without it every refusal below is satisfied by a gate that
/// refuses everything.
#[test]
fn a_log_whose_only_repeats_are_the_grandfathered_pair_is_accepted() {
    let root = scratch_repo("keys-control");
    write_log(&root, &shipped_shape());
    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 0, "the shipped shape passes");
    assert!(
        out.contains("no repeat outside the exemption"),
        "and says so:\n{out}"
    );
    // The exemption is PRINTED, so it cannot sit unread in the source.
    for key in GRANDFATHERED {
        assert!(out.contains(key), "the exemption names {key}:\n{out}");
    }
}

/// THE SEEDED VIOLATION — the defect D-279 recorded, reproduced. A pass that
/// could not have been a failure is not a pass.
#[test]
fn a_repeated_key_outside_the_exemption_is_refused_and_named() {
    let root = scratch_repo("keys-seeded");
    let mut lines = shipped_shape();
    lines.push(String::from(
        "D-273: appended a second time, with other text",
    ));
    write_log(&root, &lines);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a repeated key is a refusal");
    assert!(
        out.contains("D-273 2"),
        "and the refusal NAMES the key rather than printing a count:\n{out}"
    );
    assert!(
        !out.contains("D-276 3"),
        "and does not blame the grandfathered pair:\n{out}"
    );
}

/// THE OTHER DIRECTION. An exemption whose subject is gone is a list outliving
/// what it was for, which is how an exemption becomes the place repeats go.
#[test]
fn an_exemption_with_nothing_left_to_exempt_is_refused() {
    let root = scratch_repo("keys-stale");
    let mut lines = shipped_shape();
    lines.retain(|line| !line.starts_with("D-277: the second copy"));
    write_log(&root, &lines);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a stale exemption is a refusal");
    assert!(
        out.contains("exempted at a count it does not have: D-277 2"),
        "named for what it is:\n{out}"
    );
}

/// THE INDEX IS WHAT COMMITS (tools/SHELL_CHECKLIST.md item 5). Stage the
/// violation, overwrite the worktree copy with something clean, and a gate that
/// opened the PATH would pass it while the real bytes go to HEAD.
#[test]
fn the_key_gate_reads_the_index_and_not_the_working_tree() {
    let root = scratch_repo("keys-index");
    let mut lines = shipped_shape();
    lines.push(String::from("D-271: staged twice, with other text"));
    write_log(&root, &lines);

    // The worktree copy is now innocent. Only the index carries the repeat.
    let clean: String = shipped_shape()
        .iter()
        .map(|line| format!("{line}\n"))
        .collect();
    std::fs::write(root.join("docs/decisions.md"), clean).expect("the worktree copy writes");

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(
        &ran,
        1,
        "the staged repeat is what commits, so it is what the gate must see",
    );
    assert!(out.contains("D-271 2"), "{out}");
}

/// A `D-<n>` mentioned MID-LINE is prose, not a key. The extraction is anchored,
/// and this is the test that says the anchor is load-bearing.
#[test]
fn a_key_mentioned_mid_line_is_prose_and_not_a_second_appearance() {
    let root = scratch_repo("keys-anchor");
    let mut lines = shipped_shape();
    lines.push(String::from(
        "D-280: this line cites D-271 and D-272 in its body, which is what ADR lines do",
    ));
    write_log(&root, &lines);
    let ran = gate(&root);
    assert_code(&ran, 0, "a citation is not an appearance");
}

/// MINOR-3's class, not joined (docs/decisions.md D-251): a misspelled flag is a
/// refusal, not a silent run of the default.
#[test]
fn the_key_gate_refuses_arguments_rather_than_ignoring_them() {
    let root = scratch_repo("keys-args");
    write_log(&root, &shipped_shape());
    let ran = Command::new("bash")
        .arg(root.join("tools/decision_key_check.sh"))
        .arg("--doc")
        .arg("/nonexistent")
        .current_dir(&root)
        .output()
        .expect("the gate runs");
    let out = said(&ran);
    assert_code(&ran, 1, "arguments are not ignored");
    assert!(
        out.contains("takes no arguments"),
        "named for what it is:\n{out}"
    );
}

/// A log with no key at all is the EXTRACTION being wrong, not the file being
/// clean — the exit-0-wrong-answer this whole checklist is about.
#[test]
fn a_log_the_extraction_finds_no_key_in_is_refused_rather_than_passed() {
    let root = scratch_repo("keys-empty");
    write_log(
        &root,
        &[
            String::from("# Decisions"),
            String::from("nothing here starts with a key"),
        ],
    );
    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "no keys is a broken extraction, not a clean file");
    assert!(out.contains("the EXTRACTION is wrong"), "{out}");
}

/// A THIRD TEXT UNDER A GRANDFATHERED KEY IS NOT WHAT THE EXEMPTION BOUGHT.
///
/// `uniq -d` prints a repeated key ONCE however many times it occurs, so a set
/// comparison against an exempted KEY cannot tell the two copies D-279 ruled on
/// from a third one. Reproduced against the real log before the fix: a third
/// `D-276` with a third different text, staged, and the gate printed «no repeat
/// outside the exemption» and exited 0 — the property the gate exists for
/// broken, and reported as held. The exemption now carries the COUNT it was
/// granted, so a third copy is a different record and falls outside it.
#[test]
fn a_third_text_under_a_grandfathered_key_is_refused_rather_than_absorbed() {
    let root = scratch_repo("keys-third-copy");
    let mut lines = shipped_shape();
    lines.push(String::from(
        "D-276: a THIRD copy, whose text differs again",
    ));
    write_log(&root, &lines);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(
        &ran,
        1,
        "the exemption was granted for two copies, not for any number",
    );
    assert!(
        out.contains("D-276 3"),
        "and names the key WITH the count that exceeded its grant:\n{out}"
    );
}

/// AN ENVIRONMENT THAT REFUSES IS A RUN VOID, NOT A DUPLICATE KEY.
///
/// `tools/SHELL_CHECKLIST.md` item 12 obligation 1, applied to the gate that
/// landed in the same commits as the item. Absent git, a directory that is not a
/// repository and an unreadable blob are «I could not look», and spelling them 1
/// makes `ci: FAIL: decision key check` indistinguishable in a log from the
/// decision log actually carrying a repeat — the reading D-281 and D-285 record.
///
/// The gate resolves its own root from `BASH_SOURCE` and `cd`s there, so the
/// caller's working directory cannot reach this path: the script is COPIED out
/// of the repository, which is the only way its own root is not one.
#[test]
fn a_directory_that_is_not_a_repository_is_a_run_void_and_not_a_refusal() {
    let root = scratch("keys-not-a-repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    std::fs::create_dir_all(root.join("docs")).expect("a docs directory");
    std::fs::write(root.join("docs/decisions.md"), "D-1: a choice\n").expect("a log to not read");
    std::fs::copy(
        repo("tools/decision_key_check.sh"),
        root.join("tools/decision_key_check.sh"),
    )
    .expect("the shipped script copies");

    let ran = Command::new("bash")
        .arg(root.join("tools/decision_key_check.sh"))
        .output()
        .expect("bash runs the shipped script");
    assert_code(&ran, 2, "no git repository means no answer was taken");
    assert!(
        said(&ran).contains("RUN VOID"),
        "and the log says so in the gate's own vocabulary:\n{}",
        said(&ran)
    );
}
