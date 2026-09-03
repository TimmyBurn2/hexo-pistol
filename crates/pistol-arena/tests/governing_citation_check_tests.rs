mod common;

use std::process::{Command, Output};

use common::repo;

/// The shipped gate script, driven as a program (docs/process.md's tools/
/// coverage rule: the gate runs this, so the test runs this).
fn check() -> Output {
    Command::new("bash")
        .arg(repo().join("tools/governing_citation_check.sh"))
        .current_dir(repo())
        .output()
        .expect("bash runs the gate script")
}

fn said(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn every_governing_document_the_gate_names_reproduces_its_citations() {
    let output = check();
    assert!(
        output.status.success(),
        "the gate is red at this revision. {}",
        said(&output)
    );
}

/// The list is the whole point: a gate over a glob would report the seventy
/// records whose citations rotted as the tree moved, and a record is not a
/// governing document.
#[test]
fn the_gate_names_the_documents_it_governs_rather_than_globbing() {
    let output = check();
    let said = said(&output);
    for named in [
        "docs/experiments/wp21_prereg.md",
        "docs/experiments/wp21_throughput_prereg.md",
        "docs/experiments/matrix_label_cache_key.md",
    ] {
        assert!(
            said.contains(named),
            "the gate did not check {named}. {said}"
        );
    }
}

/// A `--proposes` entry that outlives the document proposing it is a finding,
/// so the gate must actually pass the declarations through rather than silently
/// widening the check.
#[test]
fn the_proposed_paths_are_declared_and_counted() {
    let output = check();
    let said = said(&output);
    assert!(
        said.contains("proposed path(s)"),
        "the gate did not report its --proposes declarations. {said}"
    );
}

// ---------------------------------------------------------------------------
// THE GATE'S SECOND HALF: a `<doc>.md revision N` names the revision that
// document's own title line holds (docs/decisions.md D-599..D-602). The class
// was caught five times by fresh reviewers before anything mechanical looked
// for it, and the checker above cannot see it — it matches paths, and the
// citation that kept going stale is a bare basename.
// ---------------------------------------------------------------------------

/// Drive the SHIPPED revision checker over fixtures of the test's own making,
/// so the case a green tree cannot exercise — a stale citation — is exercised.
fn revisions(args: &[&std::path::Path], exempt: &[&std::path::Path]) -> Output {
    let mut command = Command::new("python3");
    command.arg(repo().join("tools/revision_citation_check.py"));
    for path in exempt {
        command.arg("--exempt").arg(path);
    }
    for path in args {
        command.arg(path);
    }
    command
        .current_dir(repo())
        .output()
        .expect("python3 runs the revision checker")
}

/// The three exit codes spelled out, so a failure says what the code the test
/// did NOT get would have meant (tools/SHELL_CHECKLIST.md item 12).
fn meaning(output: &Output) -> String {
    format!(
        "exit {:?} — 0 is `every cited revision is the cited document's own`, 1 is `one is \
         stale`, 2 is `RUN VOID, no answer was taken`.\n{}",
        output.status.code(),
        said(output)
    )
}

/// A document whose first line states `revision <n>`.
fn doc(scratch: &common::Scratch, name: &str, revision: u32, body: &str) -> std::path::PathBuf {
    scratch.write(
        name,
        &format!("# a fixture document, revision {revision}.\n\n{body}\n"),
    )
}

#[test]
fn a_citation_of_a_revision_the_cited_document_does_not_hold_is_refused_by_name() {
    // THE CONTROL AND THE DEFECT IN ONE TEST: the same pair of documents, once
    // citing the revision the tree holds and once citing the one before it. A
    // checker that refused everything would pass the first half; one that
    // refused nothing would pass the second.
    let scratch = common::Scratch::new("revcite-stale");
    let cited = doc(&scratch, "cited.md", 10, "nothing here cites anything.");
    let fresh = doc(
        &scratch,
        "fresh.md",
        4,
        "GOVERNING: `cited.md` revision 10.",
    );
    let stale = doc(&scratch, "stale.md", 4, "GOVERNING: `cited.md` revision 9.");

    let good = revisions(&[&cited, &fresh], &[]);
    assert_eq!(
        good.status.code(),
        Some(0),
        "a citation naming the cited document's own revision was refused. {}",
        meaning(&good)
    );

    let bad = revisions(&[&cited, &stale], &[]);
    assert_eq!(
        bad.status.code(),
        Some(1),
        "a citation one revision stale was accepted, which is the defect this \
         check exists for. {}",
        meaning(&bad)
    );
    let told = said(&bad);
    assert!(
        told.contains("revision 9") && told.contains("revision 10"),
        "the refusal names neither the citation nor the truth. {told}"
    );
}

#[test]
fn a_document_whose_citations_are_exempt_is_not_checked_and_says_so() {
    // A document whose review gate has closed keeps the citations it closed
    // with (D-589, D-600). The exemption is named, so a reader sees which
    // document was skipped rather than inferring it from silence.
    let scratch = common::Scratch::new("revcite-exempt");
    let cited = doc(&scratch, "cited.md", 10, "nothing.");
    let stale = doc(&scratch, "stale.md", 4, "GOVERNING: `cited.md` revision 9.");
    let output = revisions(&[&cited, &stale], &[&stale]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "an exempt document's stale citation was still refused. {}",
        meaning(&output)
    );
    assert!(
        said(&output).contains("EXEMPT"),
        "the run did not say which document it skipped. {}",
        said(&output)
    );
}

#[test]
fn a_cited_document_that_states_no_revision_is_a_void_and_not_a_refusal() {
    // ITEM 12'S OWN DISTINCTION. "The cited document states no revision" is not
    // "the citation is stale": no answer can be taken, and a reader who saw a
    // refusal would go looking for a wrong number that does not exist.
    let scratch = common::Scratch::new("revcite-void");
    let untitled = scratch.write("untitled.md", "# a fixture with no revision.\n\nbody.\n");
    let citing = doc(&scratch, "citing.md", 2, "see `untitled.md` revision 3.");
    let output = revisions(&[&citing, &untitled], &[]);
    assert_eq!(
        output.status.code(),
        Some(2),
        "a cited document stating no revision must be a VOID (exit 2), not a \
         refusal (exit 1): no answer was taken. {}",
        meaning(&output)
    );
    assert!(
        said(&output).contains("RUN VOID") && said(&output).contains("NOT a failure"),
        "the void did not say it was not a failure. {}",
        said(&output)
    );
}

#[test]
fn the_gate_runs_the_revision_check_over_the_documents_it_governs() {
    // The wiring, not the checker: a check that never runs on the gate is the
    // gap docs/experiments/wp20m_DESIGN_STOP.md recorded and nothing acted on
    // for two arcs (D-582, D-583).
    let output = check();
    let told = said(&output);
    assert!(
        told.contains("REVISION_CITATION_CHECK_DONE"),
        "the gate did not run the revision check. {told}"
    );
    assert!(
        told.contains("revision citation(s) checked"),
        "the gate ran it but reported no count. {told}"
    );
}
