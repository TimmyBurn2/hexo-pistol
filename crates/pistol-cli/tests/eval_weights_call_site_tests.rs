mod common;

use common::{INSTRUMENT, committed, scratch_file, talk};
use pistol_cli::report::{NPS_FIELD, TIME_FIELD};
use pistol_engine::{Config, Pistol};

/// Deep enough that the table's values reach the move choice, small enough for
/// a debug build.
const BUDGET: &str = "go depth_turns 3";

/// A position with lines of both colours, so several window counts are non-zero
/// and a change to the table can express itself.
const POSITION: &str = "position start moves 0,0 1,0/2,0 0,1/0,2 1,1/3,0";

/// The fields two runs may differ on, taken from the module that writes them.
const MEASURED_FIELDS: [&str; 2] = [NPS_FIELD, TIME_FIELD];

fn weights_document(table: [i32; 5], trailing_comment: &str) -> String {
    let mut text = String::from("schema_version = 1\nbackend = \"handcrafted_v0\"\n[table]\n");
    for (index, value) in table.iter().enumerate() {
        text.push_str(&format!("{} = {value}\n", index + 1));
    }
    text.push_str(trailing_comment);
    text
}

fn answers_with(weights: &std::path::Path) -> Vec<String> {
    let mut config: Config = committed(INSTRUMENT);
    config.eval.weights_file = weights.to_path_buf();
    let mut engine: Pistol = Pistol::from_config(config)
        .unwrap_or_else(|error| panic!("the config must build: {error}"));
    let said = talk(&mut engine, &["newgame", POSITION, BUDGET]);
    assert!(
        !said.iter().any(|line| line.starts_with("error ")),
        "the engine refused something: {said:#?}"
    );
    assert!(
        said.iter().any(|line| line.starts_with("bestmove ")),
        "no bestmove among: {said:#?}"
    );
    said.iter().map(|line| normalize(line)).collect()
}

/// One line with the machine's measurements dropped, so what is compared is the
/// search and not how fast this run happened to be.
fn normalize(line: &str) -> String {
    let mut kept: Vec<&str> = Vec::new();
    let mut words = line.split_whitespace();
    while let Some(word) = words.next() {
        if MEASURED_FIELDS.contains(&word) {
            words.next();
            continue;
        }
        kept.push(word);
    }
    kept.join(" ")
}

/// The property the dispatch's un-buildable "byte-identity when the key is
/// absent" clause was reaching for: an absent key is a named error (hard rule 1),
/// so what can be tested is that the loader reads VALUES and nothing else.
#[test]
fn a_weights_file_differing_only_in_presentation_leaves_search_output_identical() {
    let committed_table = [2, 12, 60, 300, 1500];
    let plain = scratch_file(
        "weights-plain",
        "eval_v0_weights.toml",
        &weights_document(committed_table, ""),
    );
    let dressed = scratch_file(
        "weights-dressed",
        "eval_v0_weights.toml",
        &weights_document(
            committed_table,
            "\n# a comment the loader must not read as a value\n\n",
        ),
    );
    assert_ne!(
        std::fs::read(&plain).unwrap(),
        std::fs::read(&dressed).unwrap(),
        "the two documents must differ as BYTES, or this test compares a file with itself"
    );
    assert_eq!(answers_with(&plain), answers_with(&dressed));
}

/// The call-site mutant that must die (docs/decisions.md D-553). A test that
/// cannot fail this way is testing nothing: it would pass just as well against
/// an engine that ignored `weights_file` entirely.
#[test]
fn one_perturbed_table_entry_changes_search_output() {
    let base = scratch_file(
        "weights-base",
        "eval_v0_weights.toml",
        &weights_document([2, 12, 60, 300, 1500], ""),
    );
    let perturbed = scratch_file(
        "weights-perturbed",
        "eval_v0_weights.toml",
        &weights_document([2, 12, 60, 300, 1499], ""),
    );
    assert_ne!(answers_with(&base), answers_with(&perturbed));
}

/// The table this phase's fit produced, driven at the call site: it is a legal
/// document and it CHANGES PLAY, which is what makes an SPRT against it a
/// measurement rather than a self-match.
#[test]
fn the_quiet_fit_table_is_loadable_and_changes_the_search() {
    let committed_weights = scratch_file(
        "weights-committed",
        "eval_v0_weights.toml",
        &weights_document([2, 12, 60, 300, 1500], ""),
    );
    let fitted = scratch_file(
        "weights-quiet-fit",
        "eval_v0_weights.toml",
        &weights_document([5, 34, 60, 300, 1500], ""),
    );
    assert_ne!(answers_with(&committed_weights), answers_with(&fitted));
}
