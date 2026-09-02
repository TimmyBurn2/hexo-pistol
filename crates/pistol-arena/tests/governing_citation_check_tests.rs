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
