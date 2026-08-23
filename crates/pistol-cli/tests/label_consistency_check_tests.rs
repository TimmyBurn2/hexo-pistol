//! `tools/label_consistency_check.sh` — CI gate 15 of 15, the WP-1.5b carve
//! documents' self-state (docs/decisions.md D-338 row R4;
//! tools/SHELL_CHECKLIST.md item 10).
//!
//! # What is being guarded, and why a gate rather than care
//!
//! `docs/experiments/matrix_META1_REDTEAM.md` M2: both landed claim inventories
//! ship a headline count of their own table that their own table falsifies —
//! fifty-four rows under a stated thirty-four, eleven failing rows under a
//! stated six — uncaught by every round including the reviewer who read the
//! earlier one closely. Inside this gate's subject the same form appears twice
//! over: a summand line's arithmetic against its own stated total and against
//! the section heading above it, and a group-count line against the names it
//! introduces. A document that misdescribes its own count is
//! docs/decisions.md D-335's generator (2).
//!
//! # The head/foot u-rev label check this gate once ran is retired
//!
//! `docs/decisions.md` D-311's appended amendment: revision identity for these
//! six documents is the commit SHA, not an in-document label — `e42ca88`
//! appended 14 lines to `U4_soundness_instrument.md` without bumping its head
//! label, so `u-rev 9` came to name two different texts, the exact ambiguity a
//! label exists to prevent, produced by the commit that amended D-311 to
//! reaffirm the rule the label had just broken. This suite no longer builds
//! label-bearing fixtures or exercises the retired check; what a document says
//! about its own revision history is struck under D-346 rather than checked
//! here.
//!
//! # RULE9-JUSTIFICATION: one gate, one scratch repository, one fixture builder.
//! Every test drives the same shipped script over the same scratch git
//! repository holding the same six-document fixture set, and each verdict is one
//! planted defect against that one builder. Splitting the file splits the
//! builder, and two builders that drift apart is the defect this gate is for.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// The six documents the shipped gate names, restated rather than imported:
/// this file is a CHECK on the script, and agreeing with it by construction
/// proves nothing.
const DOCS: [&str; 6] = [
    "U1_gate_supersession",
    "U2_node_protocol",
    "U3_tier_t",
    "U4_soundness_instrument",
    "WPQ_seed",
    "section_owner_table",
];

/// The gate's non-vacuity floors, also restated: it refuses a run that finds
/// fewer than two summand lines, five summand rows, or four group counts,
/// because a smaller number is its extraction drifting off its subject rather
/// than the documents going clean.
const SUMMAND_FLOOR: usize = 2;
const ROW_FLOOR: usize = 5;
const GROUP_FLOOR: usize = 4;

fn scratch_repo(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    std::fs::create_dir_all(root.join("docs/experiments")).expect("a docs directory");
    std::fs::copy(
        repo("tools/label_consistency_check.sh"),
        root.join("tools/label_consistency_check.sh"),
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
        .arg(root.join("tools/label_consistency_check.sh"))
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

/// Assert the gate's exit code in a message that says what the OTHER codes would
/// have meant (tools/SHELL_CHECKLIST.md item 12 obligation 3). `!success()` is
/// satisfied by 1, by 2, by 127 and by a signal death alike, so a suite written
/// that way reports a RUN VOID as a regression.
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — every document agrees with itself",
        Some(1) => {
            "1 — a document's stated count disagrees with what it counts, or an extraction found nothing; an ANSWER, and it is no"
        }
        Some(2) => "2 — RUN VOID: no answer was taken, which is NOT a bad count",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\n{}",
        said(ran)
    );
}

/// One carve document: a title and a body. The gate no longer reads any
/// revision label, so the fixture carries none.
fn document(body: &str) -> String {
    format!("# T\n\n{body}\n")
}

/// The two counted forms, carrying the gate's floors exactly: two summand lines
/// under headings that state the same totals, and four backtick-quoted groups.
fn counted_body() -> String {
    [
        "## 4. §11 — the 5 test rows",
        "",
        "**U2 (2):** `alpha`, `beta`.",
        "",
        "**U3 (1):** `gamma`.",
        "",
        "**U4 (1):** `delta`.",
        "",
        "**SEED (1):** `epsilon`.",
        "",
        "2 + 1 + 1 + 1 = **5**. The map.",
        "",
        "## 5. §15 — the 15 ADR items",
        "",
        "| Owner | Items |",
        "|---|---|",
        "| **U1** | 1, 2, 3 |",
        "| **U2** | 4, 5, 6, 7 |",
        "| **U3** | 8, 9, 10 |",
        "| **U4** | 11, 12, 13 |",
        "| **SEED** | 14, 15 |",
        "",
        "3 + 4 + 3 + 3 + 2 = **15**. The other map.",
    ]
    .join("\n")
}

/// The shipped shape: six consistent documents, the counted forms in the owner
/// table exactly as the real one carries them.
fn shipped_shape() -> Vec<(String, String)> {
    DOCS.iter()
        .map(|name| {
            let body = if *name == "section_owner_table" {
                counted_body()
            } else {
                String::from("Normative content.")
            };
            ((*name).to_string(), document(&body))
        })
        .collect()
}

/// Write the fixture set and STAGE it — the gate reads the index, so an unstaged
/// file is invisible to it.
fn write_docs(root: &Path, docs: &[(String, String)]) {
    for (name, text) in docs {
        std::fs::write(root.join(format!("docs/experiments/{name}.md")), text)
            .expect("a doc writes");
    }
    git(root, &["add", "-A"]);
}

/// Replace one document in the fixture set, by name.
fn with_doc(docs: &mut [(String, String)], name: &str, text: String) {
    let slot = docs
        .iter_mut()
        .find(|(n, _)| n == name)
        .expect("the fixture set names that document");
    slot.1 = text;
}

/// THE CONTROL. Without it every refusal below is satisfied by a gate that
/// refuses everything.
#[test]
fn six_documents_that_agree_with_themselves_are_accepted() {
    let root = scratch_repo("label-control");
    write_docs(&root, &shipped_shape());
    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 0, "the shipped shape passes");
    assert!(
        out.contains("every document agrees with itself"),
        "and says so:\n{out}"
    );
    assert!(
        out.contains(&format!(
            "{SUMMAND_FLOOR} summand line(s), {ROW_FLOOR} summand row(s), {GROUP_FLOOR} group count(s)"
        )),
        "and reports the counted forms it actually checked, at the floors it refuses below:\n{out}"
    );
    assert!(
        out.contains("6 document(s) read"),
        "the run counts the subject it read:\n{out}"
    );
}

/// A stated group count against the names the group enumerates — M2's class,
/// inside this gate's subject.
#[test]
fn a_group_that_states_more_names_than_it_enumerates_is_refused() {
    let root = scratch_repo("label-group");
    let mut docs = shipped_shape();
    let body = counted_body().replace("**U2 (2):**", "**U2 (34):**");
    with_doc(&mut docs, "section_owner_table", document(&body));
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a group count its own list falsifies is a refusal");
    assert!(
        out.contains("group U2 states 34 and enumerates 2"),
        "stating both numbers:\n{out}"
    );
    assert!(
        out.contains("docs/experiments/section_owner_table.md"),
        "and naming the file:\n{out}"
    );
}

/// The summand line's own arithmetic, and the heading that states its total.
#[test]
fn a_summand_line_that_does_not_total_its_stated_number_is_refused() {
    let root = scratch_repo("label-summand");
    let mut docs = shipped_shape();
    let body = counted_body().replace("3 + 4 + 3 + 3 + 2 = **15**", "3 + 5 + 3 + 3 + 2 = **15**");
    with_doc(&mut docs, "section_owner_table", document(&body));
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "arithmetic that does not hold is a refusal");
    assert!(
        out.contains("the summands total 16 and the line states 15"),
        "stating both numbers:\n{out}"
    );
}

/// A heading that states a count its section's own total contradicts.
#[test]
fn a_heading_count_the_total_below_it_contradicts_is_refused() {
    let root = scratch_repo("label-heading");
    let mut docs = shipped_shape();
    let body = counted_body().replace(
        "## 5. §15 — the 15 ADR items",
        "## 5. §15 — the 19 ADR items",
    );
    with_doc(&mut docs, "section_owner_table", document(&body));
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a heading disagreeing with its total is a refusal");
    assert!(
        out.contains("the heading states 19 and the stated total is 15"),
        "stating both numbers:\n{out}"
    );
}

/// NON-VACUITY, ENFORCED BY THE GATE ITSELF. A criterion the named defect cannot
/// falsify is not a criterion (CLAUDE.md), so a subject that has lost the
/// counted forms is a refusal rather than a silent green.
#[test]
fn a_subject_set_that_lost_its_counted_forms_is_refused_not_passed() {
    let root = scratch_repo("label-vacuous");
    let mut docs = shipped_shape();
    with_doc(
        &mut docs,
        "section_owner_table",
        document("Normative content, and no counted form at all."),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(
        &ran,
        1,
        "an extraction with nothing left to check is a refusal",
    );
    assert!(
        out.contains("found 0 lines to check") || out.contains("found 0 groups to check"),
        "naming which form it lost:\n{out}"
    );
    assert!(
        out.contains("drifting off its subject"),
        "and saying that is what a smaller number means:\n{out}"
    );
}

/// A document the subject list names but the index does not hold is a REFUSAL,
/// never a silent skip (tools/SHELL_CHECKLIST.md item 5).
#[test]
fn a_subject_document_missing_from_the_index_is_refused_not_skipped() {
    let root = scratch_repo("label-missing");
    let mut docs = shipped_shape();
    docs.retain(|(name, _)| name != "WPQ_seed");
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a missing subject is a refusal");
    assert!(
        out.contains("docs/experiments/WPQ_seed.md is not in the git index"),
        "named, rather than skipped:\n{out}"
    );
}

/// THE INDEX IS WHAT COMMITS (tools/SHELL_CHECKLIST.md item 5). Stage the
/// violation, overwrite the worktree copy with something clean, and a gate that
/// opened the PATH would pass it while the real bytes go to HEAD.
#[test]
fn the_staged_bytes_are_read_and_not_the_worktree_copy() {
    let root = scratch_repo("label-index");
    let mut docs = shipped_shape();
    let bad_body = counted_body().replace("**U2 (2):**", "**U2 (34):**");
    with_doc(&mut docs, "section_owner_table", document(&bad_body));
    write_docs(&root, &docs);
    // The worktree now says something clean; the index still holds the bad count.
    std::fs::write(
        root.join("docs/experiments/section_owner_table.md"),
        document(&counted_body()),
    )
    .expect("the worktree copy is overwritten");

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(
        &ran,
        1,
        "the STAGED bad count is what the gate answers about",
    );
    assert!(
        out.contains("group U2 states 34 and enumerates 2"),
        "reading the index and not the worktree:\n{out}"
    );
}

/// Arguments are not silently ignored (docs/decisions.md D-251 MINOR-3).
#[test]
fn an_argument_is_refused_rather_than_run_as_the_default() {
    let root = scratch_repo("label-args");
    write_docs(&root, &shipped_shape());
    let ran = Command::new("bash")
        .arg(root.join("tools/label_consistency_check.sh"))
        .arg("--all")
        .current_dir(&root)
        .output()
        .expect("the gate runs");
    let out = said(&ran);
    assert_code(&ran, 1, "an unexpected argument is a refusal");
    assert!(
        out.contains("takes no arguments and was given: --all"),
        "quoted back:\n{out}"
    );
}

/// THE MULTI-WORD LABEL — REVIEW-impl MAJOR-3, reproduced and pinned. The
/// extraction's own character class admits a space in a group label, and a
/// positionally-read record then absorbed half of it: a CORRECT two-word group
/// was refused with `group New states Plan and enumerates 4 4`. The label is the
/// record's LAST field, so it may hold spaces.
#[test]
fn a_correct_group_with_a_two_word_label_is_accepted_and_not_garbled() {
    let root = scratch_repo("label-two-word");
    let mut docs = shipped_shape();
    let body = counted_body().replace("**U3 (1):**", "**New Plan (1):**");
    with_doc(&mut docs, "section_owner_table", document(&body));
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 0, "a correct two-word label is not a defect");
    assert!(
        !out.contains("group New states"),
        "and the record's fields are not misaligned by the space:\n{out}"
    );
}

/// And the same label, genuinely wrong, is still named in full.
#[test]
fn a_two_word_label_that_miscounts_is_named_in_full() {
    let root = scratch_repo("label-two-word-bad");
    let mut docs = shipped_shape();
    let body = counted_body().replace("**U3 (1):**", "**New Plan (7):**");
    with_doc(&mut docs, "section_owner_table", document(&body));
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a miscounted group is a refusal");
    assert!(
        out.contains("group New Plan states 7 and enumerates 1"),
        "with the whole label and both numbers:\n{out}"
    );
}

/// THE SUMMANDS AGAINST THE TABLE — REVIEW-impl MAJOR-2, reproduced and pinned.
/// A table row that lost an item leaves the arithmetic AND the heading
/// untouched, so checking those two against each other is a property the named
/// defect PRESERVES — which CLAUDE.md says is not a criterion at all. This is the
/// only check that reaches `section_owner_table.md` §7, whose owners' items live
/// in table CELLS rather than in a backtick group.
#[test]
fn a_table_row_that_lost_an_item_is_refused_though_the_arithmetic_still_holds() {
    let root = scratch_repo("label-summand-row");
    let mut docs = shipped_shape();
    let body = counted_body().replace("| **U2** | 4, 5, 6, 7 |", "| **U2** | 4, 5, 6 |");
    with_doc(&mut docs, "section_owner_table", document(&body));
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a row disagreeing with its summand is a refusal");
    assert!(
        out.contains("summand 2 is 4 and the table row above enumerates 3"),
        "naming which summand and both numbers:\n{out}"
    );
    assert!(
        !out.contains("the summands total"),
        "and the arithmetic is untouched, which is the point:\n{out}"
    );
}

/// A directory that is not a repository is "I could not look", not "a document
/// is stale" (tools/SHELL_CHECKLIST.md item 12).
#[test]
fn outside_a_git_repository_the_run_is_void_and_not_a_failure() {
    let root = scratch("label-void").join("repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    std::fs::copy(
        repo("tools/label_consistency_check.sh"),
        root.join("tools/label_consistency_check.sh"),
    )
    .expect("the gate copies");
    // No `git init`: nothing here is a repository.
    let ran = Command::new("bash")
        .arg(root.join("tools/label_consistency_check.sh"))
        .current_dir(&root)
        .env("GIT_CEILING_DIRECTORIES", root.parent().expect("a parent"))
        .output()
        .expect("the gate runs");
    let out = said(&ran);
    assert_code(&ran, 2, "no repository is a VOID");
    assert!(out.contains("RUN VOID"), "spelled as one:\n{out}");
}
