mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};

const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;
const LABEL_NODES: &str = "5000";

/// A SELF-PLAY run: one binary, one engine config, two labels — which is the
/// only shape a capture can be taken from, and the shape `validate` forces by
/// refusing identical labels rather than identical engines.
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

/// A run of two DIFFERENT engines, which a capture must refuse.
fn two_engines(scratch: &Scratch, tag: &str) -> Ran {
    let openings = scratch.write(&format!("openings-{tag}.txt"), &openings_prefix(OPENINGS));
    let config_a = scratch.stub_config(&format!("a-{tag}.toml"), "honest");
    let config_b = scratch.stub_config(&format!("b-{tag}.toml"), "honest_last");
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
        config_a: &config_a,
        binary_b: STUB,
        config_b: &config_b,
    };
    run(scratch, &spec, tag)
}

/// Ask the binary to capture one report.
fn capture(scratch: &Scratch, report: &Path, tag: &str) -> (Output, PathBuf) {
    let out = scratch.path(&format!("capture-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--capture")
        .arg(report)
        .arg("--out")
        .arg(&out)
        .arg("--label-nodes")
        .arg(LABEL_NODES)
        .output()
        .expect("the arena binary runs");
    (output, out)
}

/// The report a capture is taken from, on disk.
fn report_at(scratch: &Scratch, ran: &Ran, tag: &str) -> PathBuf {
    scratch.write(&format!("report-{tag}-copy.txt"), ran.report())
}

/// The body lines of a rendered capture.
fn records(text: &str) -> Vec<Vec<String>> {
    let body = pistol_cli::corpus::emit::body_of(text).expect("a capture carries a body digest");
    body.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split('\t').map(str::to_string).collect())
        .collect()
}

#[test]
fn a_self_play_report_whose_seats_carry_distinct_labels_is_accepted() {
    let scratch = Scratch::new("capture-accept");
    let ran = self_play(&scratch, "honest", "accept");
    let report = report_at(&scratch, &ran, "accept");
    let (output, out) = capture(&scratch, &report, "accept");
    assert!(
        out.exists(),
        "a self-play report was refused: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_report_whose_seats_attest_different_engines_is_refused_by_name() {
    let scratch = Scratch::new("capture-two-engines");
    let ran = two_engines(&scratch, "twoeng");
    let report = report_at(&scratch, &ran, "twoeng");
    let (output, out) = capture(&scratch, &report, "twoeng");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!out.exists(), "a two-engine report produced a capture");
    assert!(
        stderr.contains("two seats attest different engines") && stderr.contains("config_sha256"),
        "the refusal did not name the field that differed: {stderr}"
    );
}

#[test]
fn a_capture_over_a_report_whose_budget_is_not_nodes_is_refused_by_name() {
    let scratch = Scratch::new("capture-depth-budget");
    let openings = scratch.write("openings-depth.txt", &openings_prefix(OPENINGS));
    let config = scratch.stub_config("engine-depth.toml", "honest");
    let spec = ConfigSpec {
        openings: &openings,
        take: OPENINGS,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: 1,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "depth_turns",
        budget_value: 1,
        binary_a: STUB,
        config_a: &config,
        binary_b: STUB,
        config_b: &config,
    };
    let ran = run(&scratch, &spec, "depth");
    let report = report_at(&scratch, &ran, "depth");
    let (output, out) = capture(&scratch, &report, "depth");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!out.exists(), "a depth_turns report produced a capture");
    assert!(
        stderr.contains("only a `nodes` budget"),
        "the refusal did not name the budget kind: {stderr}"
    );
}

#[test]
fn the_asked_set_is_every_legal_turn_boundary() {
    let scratch = Scratch::new("capture-asked-set");
    let ran = self_play(&scratch, "honest", "asked");
    let report = report_at(&scratch, &ran, "asked");
    let (_, out) = capture(&scratch, &report, "asked");
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    let rows = records(&text);

    let source_sha = pistol_cli::sha256::sha256_hex(ran.report().as_bytes());
    let transcript =
        pistol_arena::transcript::read(ran.report(), source_sha).expect("the report reads back");
    let expected: usize = transcript
        .games
        .iter()
        .map(|game| {
            pistol_arena::capture::asked_prefixes(game)
                .expect("a recorded game is legal")
                .len()
        })
        .sum();
    assert_eq!(
        rows.len(),
        expected,
        "the capture holds a different number of records than the asked set"
    );
    for game in &transcript.games {
        let mine: Vec<usize> = rows
            .iter()
            .filter(|row| row[0] == game.index.to_string())
            .map(|row| row[1].parse().expect("a turn count"))
            .collect();
        assert_eq!(
            mine,
            pistol_arena::capture::asked_prefixes(game).expect("legal"),
            "game {} asked a different prefix set",
            game.index
        );
    }
}

#[test]
fn the_initial_position_is_asked_without_a_moves_keyword() {
    let scratch = Scratch::new("capture-initial");
    let ran = self_play(&scratch, "honest", "initial");
    let report = report_at(&scratch, &ran, "initial");
    let (_, out) = capture(&scratch, &report, "initial");
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    for row in records(&text) {
        if row[1] == "0" {
            assert_eq!(
                row[2], "position start",
                "the turn-zero record was not asked as bare `position start`"
            );
        }
    }
}

#[test]
fn a_captured_totals_line_keeps_every_field_but_nps_and_time() {
    let scratch = Scratch::new("capture-normalise");
    let ran = self_play(&scratch, "honest", "norm");
    let report = report_at(&scratch, &ran, "norm");
    let (_, out) = capture(&scratch, &report, "norm");
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    for row in records(&text) {
        let totals = &row[3];
        assert!(
            !totals.contains(" nps ") && !totals.contains(" time "),
            "a captured totals line still carries a wall-clock field: {totals}"
        );
        for field in [
            "depth_turns",
            "seldepth",
            "nodes",
            "hashfull",
            "score",
            "pv",
        ] {
            assert!(
                totals.contains(field),
                "the normalisation removed `{field}`: {totals}"
            );
        }
    }
}

#[test]
fn a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote() {
    let scratch = Scratch::new("capture-bestmove");
    let ran = self_play(&scratch, "honest", "best");
    let report = report_at(&scratch, &ran, "best");
    let (_, out) = capture(&scratch, &report, "best");
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    for row in records(&text) {
        let best = &row[4];
        assert!(
            best.starts_with("bestmove ") && best.matches(' ').count() == 1,
            "the bestmove line was re-rendered rather than captured: {best}"
        );
    }
}

#[test]
fn every_label_go_is_preceded_by_a_newgame() {
    let scratch = Scratch::new("capture-newgame");
    let ran = self_play(&scratch, "demands_newgame_per_ask", "newgame");
    let report = report_at(&scratch, &ran, "newgame");
    let (output, out) = capture(&scratch, &report, "newgame");
    assert!(
        out.exists(),
        "an engine demanding a `newgame` per ask refused the capture, so the pass does not send \
         one per ask: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_rerun_over_one_report_is_byte_identical() {
    let scratch = Scratch::new("capture-rerun");
    let ran = self_play(&scratch, "honest", "rerun");
    let report = report_at(&scratch, &ran, "rerun");
    let (_, first) = capture(&scratch, &report, "rerun-a");
    let (_, second) = capture(&scratch, &report, "rerun-b");
    assert_eq!(
        std::fs::read_to_string(&first).expect("first"),
        std::fs::read_to_string(&second).expect("second"),
        "two captures of one report at one label budget differ"
    );
}

#[test]
fn a_capture_file_round_trips_through_its_own_loader_field_by_field() {
    let scratch = Scratch::new("capture-roundtrip");
    let ran = self_play(&scratch, "honest", "trip");
    let report = report_at(&scratch, &ran, "trip");
    let (_, out) = capture(&scratch, &report, "trip");
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    let read = pistol_arena::capture::read(&text).expect("its own loader reads it");
    let rows = records(&text);
    assert_eq!(read.records.len(), rows.len());
    for (record, row) in read.records.iter().zip(rows.iter()) {
        assert_eq!(record.game.to_string(), row[0]);
        assert_eq!(record.turns_played.to_string(), row[1]);
        assert_eq!(record.position, row[2]);
        assert_eq!(record.totals, row[3]);
        assert_eq!(record.bestmove, row[4]);
    }
}

#[test]
fn a_capture_whose_body_digest_is_wrong_is_refused_by_name() {
    let scratch = Scratch::new("capture-digest");
    let ran = self_play(&scratch, "honest", "digest");
    let report = report_at(&scratch, &ran, "digest");
    let (_, out) = capture(&scratch, &report, "digest");
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    let tampered = format!("{text}0\t0\tposition start\tinfo totals x\tbestmove 0,0\n");
    let error = pistol_arena::capture::read(&tampered).expect_err("a tampered body is refused");
    assert!(
        error.to_string().contains("digests to"),
        "the refusal did not name the digest: {error}"
    );
}

#[test]
fn a_capture_record_with_the_wrong_field_count_is_refused_by_name() {
    let broken = one_record_capture("0\t0\tposition start\tinfo totals depth_turns 1");
    let error = pistol_arena::capture::read(&broken).expect_err("a short record is refused");
    assert!(
        error.to_string().contains("TAB-separated field"),
        "the refusal did not name the arity: {error}"
    );
}

#[test]
fn a_capture_record_with_an_empty_field_is_refused_by_name() {
    let broken = one_record_capture("0\t0\t\tinfo totals depth_turns 1\tbestmove 0,0");
    let error = pistol_arena::capture::read(&broken).expect_err("an empty field is refused");
    assert!(
        error.to_string().contains("is empty"),
        "the refusal did not name the empty field: {error}"
    );
}

#[test]
fn a_capture_whose_format_version_is_unknown_is_refused_by_name() {
    let good = one_record_capture("0\t0\tposition start\tinfo totals depth_turns 1\tbestmove 0,0");
    let bumped = good.replace(
        "# param capture_format_version 1",
        "# param capture_format_version 2",
    );
    let error = pistol_arena::capture::read(&bumped).expect_err("an unknown version is refused");
    assert!(
        error.to_string().contains("capture format 2"),
        "the refusal did not name the version: {error}"
    );
}

/// A capture file with one body line, digested honestly.
fn one_record_capture(record: &str) -> String {
    let mut fixture = pistol_cli::corpus::emit::Fixture::new(&["a fixture capture"]);
    fixture.param("capture_format_version", 1);
    fixture.param("experiment_sha256", "e");
    fixture.param("source_sha256", "s");
    fixture.param("label_go", "go nodes 5000");
    fixture.derived("capture_sha256", "c");
    fixture.derived("games", 1);
    fixture.derived("records", 1);
    fixture.line(record);
    fixture.render()
}

#[test]
fn two_reports_of_one_experiment_share_a_capture_identity() {
    let one = pistol_arena::capture::capture_sha256("experiment", "go nodes 5000", 1);
    let two = pistol_arena::capture::capture_sha256("experiment", "go nodes 5000", 1);
    assert_eq!(one, two);
}

#[test]
fn two_captures_of_different_experiments_do_not_share_an_identity() {
    let one = pistol_arena::capture::capture_sha256("experiment-a", "go nodes 5000", 1);
    let two = pistol_arena::capture::capture_sha256("experiment-b", "go nodes 5000", 1);
    assert_ne!(one, two);
}

#[test]
fn a_capture_identity_moves_when_the_label_budget_moves() {
    let one = pistol_arena::capture::capture_sha256("experiment", "go nodes 5000", 1);
    let two = pistol_arena::capture::capture_sha256("experiment", "go nodes 6000", 1);
    assert_ne!(one, two);
}

#[test]
fn a_capture_identity_moves_when_the_format_version_moves() {
    let one = pistol_arena::capture::capture_sha256("experiment", "go nodes 5000", 1);
    let two = pistol_arena::capture::capture_sha256("experiment", "go nodes 5000", 2);
    assert_ne!(one, two);
}

#[test]
fn the_label_go_line_is_the_one_budget_section_spells() {
    assert_eq!(
        pistol_arena::capture::label_go_line(5_000),
        pistol_arena::config::BudgetSection::Nodes { value: 5_000 }
            .go_line()
            .expect("a nodes budget spells a go line")
    );
}

#[test]
fn a_label_node_count_spelled_a_way_this_program_will_not_echo_back_is_refused() {
    let scratch = Scratch::new("capture-spelling");
    let out = scratch.path("never.txt");
    let output = Command::new(ARENA)
        .arg("--capture")
        .arg(scratch.path("absent.txt"))
        .arg("--out")
        .arg(&out)
        .arg("--label-nodes")
        .arg("05000")
        .output()
        .expect("the arena binary runs");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(
        stderr.contains("will not echo back"),
        "a padded node count was accepted: {stderr}"
    );
}

#[test]
fn the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line() {
    let line = "info totals depth_turns 3 seldepth 3 nodes 90 search_nodes 60 solver_nodes 30 \
                solver_firings 2 solver_invocations 2 solver_proofs 1 solver_root_nodes 4 nps 900 \
                time 100 hashfull 12 score cp 7 pv 0,0";
    let out = pistol_arena::capture::normalise(line).expect("a solver-bearing line normalises");
    assert!(!out.contains(" nps ") && !out.contains(" time "));
    for field in [
        "search_nodes 60",
        "solver_nodes 30",
        "solver_firings 2",
        "solver_invocations 2",
        "solver_proofs 1",
        "solver_root_nodes 4",
        "hashfull 12",
        "score cp 7",
        "pv 0,0",
    ] {
        assert!(
            out.contains(field),
            "the normalisation removed `{field}`: {out}"
        );
    }
}

#[test]
fn two_totals_lines_differing_only_in_nps_and_time_normalise_equal() {
    let one = "info totals depth_turns 1 seldepth 1 nodes 4 nps 1 time 0 hashfull 0 score cp 0 \
               pv 0,0";
    let two = "info totals depth_turns 1 seldepth 1 nodes 4 nps 40000 time 9 hashfull 0 score cp \
               0 pv 0,0";
    assert_eq!(
        pistol_arena::capture::normalise(one).expect("one"),
        pistol_arena::capture::normalise(two).expect("two")
    );
}

#[test]
fn a_totals_line_with_no_score_at_all_is_captured_as_written() {
    let line = "info totals depth_turns 1 seldepth 1 nodes 4 nps 1 time 0 hashfull 0 pv 0,0";
    let out = pistol_arena::capture::normalise(line).expect("a score-less line is not refused");
    assert_eq!(
        out,
        "info totals depth_turns 1 seldepth 1 nodes 4 hashfull 0 pv 0,0"
    );
}

#[test]
fn a_capture_prints_a_manifest_row_naming_its_digests() {
    let scratch = Scratch::new("capture-manifest");
    let ran = self_play(&scratch, "honest", "manifest");
    let report = report_at(&scratch, &ran, "manifest");
    let (output, out) = capture(&scratch, &report, "manifest");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let text = std::fs::read_to_string(&out).expect("the capture is readable");
    let body = pistol_cli::corpus::emit::claimed_body_digest(&text).expect("a body digest");
    assert!(
        stdout.contains("capture_manifest ")
            && stdout.contains(body)
            && stdout.contains("experiment_sha256 "),
        "no manifest row naming the digests was printed: {stdout}"
    );
}
