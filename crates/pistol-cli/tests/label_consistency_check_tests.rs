//! `tools/label_consistency_check.sh` — CI gate 15 of 15, the WP-1.5b carve
//! documents' self-state (docs/decisions.md D-338 row R4;
//! tools/SHELL_CHECKLIST.md item 10).
//!
//! # What is being guarded, and why a gate rather than care
//!
//! `docs/experiments/matrix_META1_REDTEAM.md` M3 found two live head/foot
//! disagreements in the six frozen carve documents with a one-line shell loop,
//! in under a second, in a defect class the matrix under attack asserted only a
//! fresh reviewer's hand-built claim inventory could reach. One of the two was
//! manufactured in the commit AFTER the identical defect was reported as
//! BLOCKING in a sibling document. A document that misdescribes its own state is
//! docs/decisions.md D-335's generator (2).
//!
//! # The loop that found two missed a third, and that is what this suite pins
//!
//! That loop read a `tail -3`. Three of the six documents wrap their closing
//! italic paragraph over several lines, and `U4_soundness_instrument.md` closes
//! with a single very long line behind a `---` rule — on U4 the loop printed a
//! truncated `foot=u-rev` with no number, read as noise rather than as a
//! refusal, while U4 was head `u-rev 8` against foot `u-rev 7` the whole time.
//! So `a_closing_block_wrapped_over_several_lines_still_resolves` and
//! `a_single_long_closing_line_behind_a_rule_still_resolves` are not incidental
//! coverage: they are the two shapes the instrument this gate replaces was blind
//! to, and a fixed-depth tail cannot pass both.
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
/// fewer than two summand lines or fewer than four group counts, because a
/// smaller number is its extraction drifting off its subject rather than the
/// documents going clean.
const SUMMAND_FLOOR: usize = 2;
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
            "1 — a document disagrees with itself, or an extraction found nothing; an ANSWER, and it is no"
        }
        Some(2) => "2 — RUN VOID: no answer was taken, which is NOT a stale label",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\n{}",
        said(ran)
    );
}

/// One carve document: a head label, a body, and a closing italic block.
///
/// `foot` is the WHOLE closing block, so a test can wrap it, run it long, or
/// make it name two u-revs — the three shapes the extraction has to survive.
fn document(head: u32, body: &str, foot: &str) -> String {
    format!("# T\n\n**u-rev {head}.** A carve.\n\n{body}\n\n---\n\n*T, {foot}*\n")
}

/// The two counted forms, carrying the gate's floors exactly: two summand lines
/// under headings that state the same totals, and four backtick-quoted groups.
fn counted_body() -> String {
    String::from(
        "## 4. §11 — the 5 test rows\n\n\
         **U2 (2):** `alpha`, `beta`.\n\n\
         **U3 (1):** `gamma`.\n\n\
         **U4 (1):** `delta`.\n\n\
         **SEED (1):** `epsilon`.\n\n\
         2 + 1 + 1 + 1 = **5**. The map.\n\n\
         ## 5. §12 — the 7 cost rows\n\n\
         3 + 4 = **7**. The other map.\n",
    )
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
            (
                (*name).to_string(),
                document(4, &body, "u-rev 4. The label alone."),
            )
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
            "{SUMMAND_FLOOR} summand line(s), {GROUP_FLOOR} group count(s)"
        )),
        "and reports the counted forms it actually checked, at the floors it refuses below:\n{out}"
    );
    // The subject is PRINTED, so it cannot sit unread in the source.
    for name in DOCS {
        assert!(out.contains(name), "the run names {name}:\n{out}");
    }
}

/// THE PLANTED MISMATCH — the defect M3 recorded, reproduced, and the refusal
/// must NAME THE FILE rather than printing a count.
#[test]
fn a_foot_one_behind_the_head_is_refused_and_the_file_is_named() {
    let root = scratch_repo("label-stale");
    let mut docs = shipped_shape();
    with_doc(
        &mut docs,
        "U3_tier_t",
        document(4, "Normative content.", "u-rev 3. The label alone."),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a stale foot is a refusal");
    assert!(
        out.contains("docs/experiments/U3_tier_t.md: head u-rev 4"),
        "the refusal NAMES the file:\n{out}"
    );
    assert!(
        out.contains("foot u-rev 3"),
        "and states both labels:\n{out}"
    );
    assert!(
        !out.contains("U2_node_protocol.md: head"),
        "and does not blame the documents that agree:\n{out}"
    );
}

/// A foot that recounts what each u-rev did names several, and there is then no
/// fact of the matter about which is the document's own. That is a refusal, not
/// a tolerance, and the fold law is the repair it points at.
#[test]
fn a_foot_naming_two_u_revs_is_refused_as_unresolvable() {
    let root = scratch_repo("label-ambiguous");
    let mut docs = shipped_shape();
    with_doc(
        &mut docs,
        "U1_gate_supersession",
        document(
            4,
            "Normative content.",
            "u-rev 4. u-rev 3 was a carve, not a revision.",
        ),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "an unresolvable foot is a refusal");
    assert!(
        out.contains("docs/experiments/U1_gate_supersession.md: the closing block")
            && out.contains("names 2 u-rev labels"),
        "named for what it is:\n{out}"
    );
    assert!(
        out.contains("D-331"),
        "and points at the fold law as the repair:\n{out}"
    );
}

/// THE FIRST SHAPE THE REPLACED LOOP WAS BLIND TO. A `tail -3` reading this
/// document sees a middle line; a `tail -1` sees `restates none of it.*`.
#[test]
fn a_closing_block_wrapped_over_several_lines_still_resolves() {
    let root = scratch_repo("label-wrapped");
    let mut docs = shipped_shape();
    with_doc(
        &mut docs,
        "U2_node_protocol",
        document(
            4,
            "Normative content.",
            "u-rev 4. What each revision did is the head\nblock's, and this line\nrestates none of it.",
        ),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 0, "a wrapped closing block resolves");
    assert!(
        out.contains("U2_node_protocol.md") && out.contains("foot=u-rev 4"),
        "to the label on its FIRST line:\n{out}"
    );
}

/// THE SECOND SHAPE, and the one that hid U4: a single very long closing line,
/// mentioning `u-rev` again without a number. A greedy `sub(/^.*u-rev /, ...)`
/// yields an EMPTY label here — measured, during this gate's own build.
#[test]
fn a_single_long_closing_line_behind_a_rule_still_resolves() {
    let root = scratch_repo("label-longline");
    let mut docs = shipped_shape();
    let long = format!(
        "u-rev 4. What each u-rev of this unit did is the head block's and U4-Z's, \
         and this line restates neither. {} IMPL has not started.",
        "Further clauses of the same closing sentence. ".repeat(20)
    );
    with_doc(
        &mut docs,
        "U4_soundness_instrument",
        document(4, "Normative content.", &long),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 0, "a long single-line closing block resolves");
    assert!(
        out.contains("U4_soundness_instrument.md") && out.contains("foot=u-rev 4"),
        "to its FIRST u-rev and not to an empty label:\n{out}"
    );
}

/// A stated group count against the names the group enumerates — M2's class,
/// inside this gate's subject.
#[test]
fn a_group_that_states_more_names_than_it_enumerates_is_refused() {
    let root = scratch_repo("label-group");
    let mut docs = shipped_shape();
    let body = counted_body().replace("**U2 (2):**", "**U2 (34):**");
    with_doc(
        &mut docs,
        "section_owner_table",
        document(4, &body, "u-rev 4. The label alone."),
    );
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
    let body = counted_body().replace("3 + 4 = **7**", "3 + 5 = **7**");
    with_doc(
        &mut docs,
        "section_owner_table",
        document(4, &body, "u-rev 4. The label alone."),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "arithmetic that does not hold is a refusal");
    assert!(
        out.contains("the summands total 8 and the line states 7"),
        "stating both numbers:\n{out}"
    );
}

/// A heading that states a count its section's own total contradicts.
#[test]
fn a_heading_count_the_total_below_it_contradicts_is_refused() {
    let root = scratch_repo("label-heading");
    let mut docs = shipped_shape();
    let body = counted_body().replace("## 5. §12 — the 7 cost rows", "## 5. §12 — the 9 cost rows");
    with_doc(
        &mut docs,
        "section_owner_table",
        document(4, &body, "u-rev 4. The label alone."),
    );
    write_docs(&root, &docs);

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(&ran, 1, "a heading disagreeing with its total is a refusal");
    assert!(
        out.contains("the heading states 9 and the stated total is 7"),
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
        document(
            4,
            "Normative content, and no counted form at all.",
            "u-rev 4. The label alone.",
        ),
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
    with_doc(
        &mut docs,
        "U3_tier_t",
        document(4, "Normative content.", "u-rev 3. The label alone."),
    );
    write_docs(&root, &docs);
    // The worktree now says something clean; the index still holds the stale foot.
    std::fs::write(
        root.join("docs/experiments/U3_tier_t.md"),
        document(4, "Normative content.", "u-rev 4. The label alone."),
    )
    .expect("the worktree copy is overwritten");

    let ran = gate(&root);
    let out = said(&ran);
    assert_code(
        &ran,
        1,
        "the STAGED stale foot is what the gate answers about",
    );
    assert!(
        out.contains("docs/experiments/U3_tier_t.md: head u-rev 4") && out.contains("foot u-rev 3"),
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
