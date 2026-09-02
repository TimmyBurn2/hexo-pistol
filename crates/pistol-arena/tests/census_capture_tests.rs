mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};
use pistol_engine::CensusRequest;

const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;
const LABEL_NODES: &str = "5000";

/// A self-play run of one stub behaviour — the only shape a capture can be
/// taken from, for the reason `capture_tests` gives.
fn self_play(scratch: &Scratch, behave: &str, tag: &str) -> Ran {
    let openings = scratch.write(&format!("openings-{tag}.txt"), &openings_prefix(OPENINGS));
    let config = scratch.stub_config(&format!("engine-{tag}.toml"), behave);
    let spec = ConfigSpec {
        openings: &openings,
        take: OPENINGS,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: 1,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "nodes",
        budget_value: 5_000,
        binary_a: STUB,
        config_a: &config,
        binary_b: STUB,
        config_b: &config,
    };
    run(scratch, &spec, tag)
}

/// Ask the binary to capture one report, with or without the census.
fn capture(
    scratch: &Scratch,
    report: &Path,
    tag: &str,
    census: bool,
) -> (Output, PathBuf, PathBuf) {
    let out = scratch.path(&format!("capture-{tag}.txt"));
    let mut command = Command::new(ARENA);
    command
        .arg("--capture")
        .arg(report)
        .arg("--out")
        .arg(&out)
        .arg("--label-nodes")
        .arg(LABEL_NODES);
    if census {
        command.arg("--census");
    }
    let output = command.output().expect("the arena binary runs");
    let census_out = scratch.path(&format!("capture-{tag}.census.txt"));
    (output, out, census_out)
}

fn report_at(scratch: &Scratch, ran: &Ran, tag: &str) -> PathBuf {
    scratch.write(&format!("report-{tag}-copy.txt"), ran.report())
}

/// The payload lines of a rendered fixture.
fn body(text: &str) -> Vec<String> {
    pistol_cli::corpus::emit::body_of(text)
        .expect("a census file carries a body digest")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

#[test]
fn a_capture_run_with_the_census_flag_writes_the_rows_it_was_sent() {
    // THE SINK. Every other test in this workspace drives the ENGINE, and the
    // engine is correct in the mutant this kills: the rows are written to the
    // wire and dropped downstream, at exit 0, with a capture that looks right
    // (docs/experiments/wp20b_design.md §3.1).
    let scratch = Scratch::new("census-sink");
    let ran = self_play(&scratch, "census_rows", "sink");
    let report = report_at(&scratch, &ran, "sink");
    let (output, out, census_out) = capture(&scratch, &report, "sink", true);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(out.exists(), "the capture itself was refused: {stderr}");
    assert!(
        census_out.exists(),
        "a --census capture wrote no census file: {stderr}"
    );
    let text = std::fs::read_to_string(&census_out).expect("the census file reads");
    let rows = body(&text);
    assert!(
        !rows.is_empty(),
        "the census file holds no row, so the sink kept nothing"
    );
    for row in &rows {
        assert!(
            row.starts_with("info census key "),
            "a census row reached the file as the engine wrote it: `{row}`"
        );
    }
    let records = std::fs::read_to_string(&out).expect("the capture reads");
    let asked = pistol_cli::corpus::emit::body_of(&records)
        .expect("a capture carries a body digest")
        .lines()
        .filter(|line| !line.is_empty())
        .count();
    assert_eq!(
        rows.len(),
        asked * 3,
        "the instrument writes three rows per ask, and every ask's rows are in the file"
    );
}

#[test]
fn the_census_file_carries_a_body_digest_over_the_rows_it_holds() {
    let scratch = Scratch::new("census-digest");
    let ran = self_play(&scratch, "census_rows", "digest");
    let report = report_at(&scratch, &ran, "digest");
    let (_, _, census_out) = capture(&scratch, &report, "digest", true);
    let text = std::fs::read_to_string(&census_out).expect("the census file reads");
    let claimed =
        pistol_cli::corpus::emit::claimed_body_digest(&text).expect("the header claims a digest");
    let payload = pistol_cli::corpus::emit::body_of(&text).expect("the file has a body");
    assert_eq!(
        claimed,
        pistol_cli::sha256::sha256_hex(payload.as_bytes()),
        "the census file's header names bytes it does not hold"
    );
}

#[test]
fn the_headers_row_count_is_the_number_of_rows_the_file_holds() {
    // PINNED ON A NON-EMPTY CENSUS, and that is the whole finding. Its only
    // other reader asserts `# derived rows 0` on the EMPTY census, which a
    // mutant writing a constant zero satisfies — and that mutant ships a
    // hundred-row census claiming `rows 0` at exit 0, over the line the code's
    // own comment calls the only signal a tranche has.
    let scratch = Scratch::new("census-rowcount");
    let ran = self_play(&scratch, "census_rows", "rowcount");
    let report = report_at(&scratch, &ran, "rowcount");
    let (_, _, census_out) = capture(&scratch, &report, "rowcount", true);
    let text = std::fs::read_to_string(&census_out).expect("the census file reads");
    let held = body(&text).len();
    assert!(held > 0, "this fixture writes rows");
    let claimed = text
        .lines()
        .find_map(|line| line.strip_prefix("# derived rows "))
        .expect("the header states a row count")
        .trim()
        .parse::<usize>()
        .expect("a count");
    assert_eq!(
        claimed, held,
        "the header claims {claimed} rows over a body of {held}"
    );
}

#[test]
fn the_census_manifest_row_names_the_digest_the_file_itself_claims() {
    // THE ROW AND THE FILE ARE WRITTEN BY TWO CALLS, and a manifest row is what
    // rule 8 and docs/decisions.md D-469 lean on when the artifact is large,
    // uncommitted and sha-indexed. Nothing else compares the two.
    let scratch = Scratch::new("census-manifest");
    let ran = self_play(&scratch, "census_rows", "manifest");
    let report = report_at(&scratch, &ran, "manifest");
    let (output, _, census_out) = capture(&scratch, &report, "manifest", true);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let row = stdout
        .lines()
        .find(|line| line.starts_with("census_manifest "))
        .unwrap_or_else(|| panic!("no census manifest row among:\n{stdout}"));
    let words: Vec<&str> = row.split_whitespace().collect();
    let at = words
        .iter()
        .position(|word| *word == "body_sha256")
        .expect("the row names a body digest");
    let claimed_by_row = words[at + 1];
    let text = std::fs::read_to_string(&census_out).expect("the census file reads");
    let claimed_by_file =
        pistol_cli::corpus::emit::claimed_body_digest(&text).expect("the header claims a digest");
    assert_eq!(claimed_by_row, claimed_by_file);
    let payload = pistol_cli::corpus::emit::body_of(&text).expect("the file has a body");
    assert_eq!(
        claimed_by_row,
        pistol_cli::sha256::sha256_hex(payload.as_bytes()),
        "the row and the header agree with each other and not with the bytes"
    );
}

#[test]
fn a_capture_without_the_flag_writes_no_census_file_at_all() {
    let scratch = Scratch::new("census-absent");
    let ran = self_play(&scratch, "honest", "absent");
    let report = report_at(&scratch, &ran, "absent");
    let (output, out, census_out) = capture(&scratch, &report, "absent", false);
    assert!(
        out.exists(),
        "the capture was refused: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !census_out.exists(),
        "a capture that asked for no census wrote one anyway"
    );
}

#[test]
fn a_census_capture_of_an_engine_that_cannot_serve_one_is_refused_by_name() {
    // The stub REFUSES the token rather than answering with no rows, and the
    // refusal ends the run: a capture that quietly wrote an empty census would
    // be indistinguishable from one whose engine never fired a trigger.
    let scratch = Scratch::new("census-refused");
    let ran = self_play(&scratch, "honest", "refused");
    let report = report_at(&scratch, &ran, "refused");
    let (output, out, census_out) = capture(&scratch, &report, "refused", true);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(
        !out.exists(),
        "an engine that refused the token produced a capture"
    );
    assert!(
        !census_out.exists(),
        "an engine that refused the token produced a census file"
    );
    assert!(
        stderr.contains("CensusUnsupported"),
        "the refusal did not name itself: {stderr}"
    );
}

#[test]
fn the_census_refusal_reaches_the_wire_as_one_readable_line() {
    // The refusal a driver SHOWS AN OPERATOR, pinned as text. It went out once
    // with eighteen spaces in the middle of it — a line continuation the
    // formatter had folded into literal whitespace — and nothing saw it,
    // because the only assertion was that the name appeared.
    let scratch = Scratch::new("census-refusal-text");
    let config = scratch.stub_config("engine-refusal.toml", "honest");
    let mut child = std::process::Command::new(STUB)
        .arg("--config")
        .arg(&config)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("the stub runs");
    use std::io::Write as _;
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"position start\ngo nodes 100 census\nquit\n")
        .expect("the stub reads its input");
    let output = child.wait_with_output().expect("the stub answers");
    let said = String::from_utf8_lossy(&output.stdout).to_string();
    let refusal = said
        .lines()
        .find(|line| line.starts_with("error "))
        .unwrap_or_else(|| panic!("no refusal among:\n{said}"));
    assert_eq!(
        refusal,
        "error CensusUnsupported: `arena-stub-engine` cannot produce a trigger census, and \
         answering with no rows would read as a search whose trigger never fired"
    );
    assert!(
        !refusal.contains("  "),
        "a refusal with a run of spaces in it is a folded line continuation: `{refusal}`"
    );
}

#[test]
fn a_census_row_on_a_capture_that_asked_for_none_refuses_the_run_by_name() {
    // OFF THE TOKEN THE BLOCK DOES NOT EXIST (design §4), so a row arriving on
    // a census-off capture is a protocol deviation by the engine. It used to be
    // classified, pushed into a vector and dropped — a swallowed deviation,
    // which is the shape rule 3 is about.
    let scratch = Scratch::new("census-unasked");
    let ran = self_play(&scratch, "census_rows_unasked", "unasked");
    let report = report_at(&scratch, &ran, "unasked");
    let (output, out, census_out) = capture(&scratch, &report, "unasked", false);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!out.exists(), "an unasked census row produced a capture");
    assert!(!census_out.exists());
    assert!(
        stderr.contains("did not ask for one"),
        "the refusal did not name what it refused: {stderr}"
    );
}

#[test]
fn a_census_row_carrying_a_tab_refuses_the_run_by_name() {
    // The arity guard every other captured field passes. No engine in this tree
    // can emit a TAB, which is why it is CHECKED rather than assumed: §3
    // requires the two seats to attest ONE engine, not to be `pistol`.
    let scratch = Scratch::new("census-tab");
    let ran = self_play(&scratch, "census_tab", "tab");
    let report = report_at(&scratch, &ran, "tab");
    let (output, out, census_out) = capture(&scratch, &report, "tab", true);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!out.exists(), "a tabbed census row produced a capture");
    assert!(
        !census_out.exists(),
        "a tabbed census row produced a census file"
    );
    assert!(
        stderr.contains("census row carries a TAB"),
        "the refusal did not name what it refused: {stderr}"
    );
}

#[test]
fn an_engine_that_honours_the_token_and_fires_nothing_writes_a_census_of_zero_rows() {
    // F3's FAILURE MODE, WEARING A RECEIPT — and this pins that it is legible.
    // Every committed arena config points both seats at a gate-off engine
    // config, so the realistic first production `--census` run spends the whole
    // capture and writes a census whose body is empty. That is a legitimate
    // answer — the trigger never fired — and the design asks for no refusal, so
    // the `rows 0` line is the ONLY signal a tranche has. It is asserted here so
    // it cannot quietly stop being written.
    let scratch = Scratch::new("census-empty");
    let ran = self_play(&scratch, "census_none", "empty");
    let report = report_at(&scratch, &ran, "empty");
    let (output, out, census_out) = capture(&scratch, &report, "empty", true);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(out.exists(), "the capture was refused: {stderr}");
    assert!(
        census_out.exists(),
        "a --census capture that fired nothing wrote no census file at all: {stderr}"
    );
    let text = std::fs::read_to_string(&census_out).expect("the census file reads");
    assert!(
        text.contains("# derived rows 0"),
        "the header states the count a reader has to believe a tranche by: {text}"
    );
    assert!(body(&text).is_empty(), "an empty census has an empty body");
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("captured 0 census row(s)"),
        "the run says on stdout what it wrote"
    );
}

#[test]
fn a_census_file_of_a_later_grammar_is_not_the_same_run_as_this_one() {
    // A version written only as a `# param` is a promise nothing binds: without
    // the version in the identity, a census v2 carries the SAME digest as the
    // v1 it cannot be read as. `capture_tests` pins exactly this property for
    // the capture, and the census needs its own or it has none.
    let go = pistol_arena::capture::label_go_line(5_000, CensusRequest::On);
    let at = |version| pistol_arena::census_file::census_sha256("experiment", &go, version);
    assert_ne!(
        at(1),
        at(2),
        "the identity does not move with the grammar, so the version binds nothing"
    );
    assert_ne!(
        at(pistol_arena::census_file::CENSUS_FORMAT_VERSION),
        pistol_arena::capture::capture_sha256(
            "experiment",
            &go,
            pistol_arena::capture_file::CAPTURE_FORMAT_VERSION,
        ),
        "the census's identity is the capture's own"
    );
    assert_ne!(
        at(pistol_arena::census_file::CENSUS_FORMAT_VERSION),
        pistol_arena::census_file::census_sha256(
            "a different experiment",
            &go,
            pistol_arena::census_file::CENSUS_FORMAT_VERSION,
        ),
        "the census identity ignores the experiment it is of"
    );
}

#[test]
fn the_label_go_line_is_the_one_budget_section_spells_plus_the_token_when_asked() {
    // MOVED from `capture_tests` rather than deleted: it pinned an EQUALITY the
    // token breaks, and the honest replacement is that equality off the token
    // and the token appended on it.
    let spelled = pistol_arena::config::BudgetSection::Nodes { value: 5_000 }
        .go_line()
        .expect("a nodes budget spells a go line");
    assert_eq!(
        pistol_arena::capture::label_go_line(5_000, CensusRequest::Off),
        spelled
    );
    assert_eq!(
        pistol_arena::capture::label_go_line(5_000, CensusRequest::On),
        format!("{spelled} census")
    );
}

#[test]
fn a_census_capture_and_a_census_off_capture_are_different_instruments() {
    // The digest covers the `go` line, so the two are not the same run with a
    // logging flag: a tranche registered against one is not the other.
    let off = pistol_arena::capture::capture_sha256(
        "experiment",
        &pistol_arena::capture::label_go_line(5_000, CensusRequest::Off),
        pistol_arena::capture_file::CAPTURE_FORMAT_VERSION,
    );
    let on = pistol_arena::capture::capture_sha256(
        "experiment",
        &pistol_arena::capture::label_go_line(5_000, CensusRequest::On),
        pistol_arena::capture_file::CAPTURE_FORMAT_VERSION,
    );
    assert_ne!(off, on);
}

#[test]
fn a_census_row_is_kept_by_the_classifier_and_a_per_depth_report_is_not() {
    // The arm that has to sit AHEAD of the `info` catch-all. Read as a pure
    // function so the ordering is pinned without a run.
    let row = "info census key 00000000000000000000000000000000 turns_from_root 0 mover_hot 1 \
               opponent_hot 0 mover_win_in_one_ply 0 opponent_win_in_one_ply 0 mover_live_three \
               0 opponent_live_three 0 cover none cover_count 0 attacker_visits 1 \
               attacker_proved 0 defender_visits - defender_proved -";
    assert_eq!(
        pistol_arena::capture::classify(row),
        pistol_arena::capture::Step::Census(row.to_string())
    );
    assert_eq!(
        pistol_arena::capture::classify("info depth_turns 1 seldepth 1 nodes 1"),
        pistol_arena::capture::Step::Ignore
    );
}
