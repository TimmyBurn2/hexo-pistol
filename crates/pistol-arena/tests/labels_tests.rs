mod common;

use std::path::PathBuf;
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};

const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;

/// A self-play report, its capture, and the report's own text.
struct Staged {
    report: PathBuf,
    capture: PathBuf,
    text: String,
}

fn staged(scratch: &Scratch, tag: &str) -> Staged {
    let openings = scratch.write(&format!("openings-{tag}.txt"), &openings_prefix(OPENINGS));
    let config = scratch.stub_config(&format!("engine-{tag}.toml"), "honest");
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
    let ran: Ran = run(scratch, &spec, tag);
    let text = ran.report().to_string();
    let report = scratch.write(&format!("report-{tag}-copy.txt"), &text);
    let capture = scratch.path(&format!("capture-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--capture")
        .arg(&report)
        .arg("--out")
        .arg(&capture)
        .arg("--label-nodes")
        .arg("5000")
        .output()
        .expect("the arena binary runs");
    assert!(
        capture.exists(),
        "the capture stage failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Staged {
        report,
        capture,
        text,
    }
}

fn label(scratch: &Scratch, staged: &Staged, tag: &str) -> (Output, PathBuf) {
    let out = scratch.path(&format!("corpus-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--labels")
        .arg(&staged.capture)
        .arg("--report")
        .arg(&staged.report)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    (output, out)
}

fn rows(text: &str) -> Vec<Vec<String>> {
    let body = pistol_cli::corpus::emit::body_of(text).expect("a corpus carries a body digest");
    body.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split('\t').map(str::to_string).collect())
        .collect()
}

fn corpus_of(scratch: &Scratch, tag: &str) -> (String, Staged) {
    let staged = staged(scratch, tag);
    let (output, out) = label(scratch, &staged, tag);
    assert!(
        out.exists(),
        "the labels stage failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    (
        std::fs::read_to_string(&out).expect("the corpus is readable"),
        staged,
    )
}

#[test]
fn every_capture_record_produces_one_corpus_record_in_order() {
    let scratch = Scratch::new("labels-order");
    let (corpus, staged) = corpus_of(&scratch, "order");
    let capture_text = std::fs::read_to_string(&staged.capture).expect("the capture is readable");
    let capture = pistol_arena::capture::read(&capture_text).expect("the capture reads");
    let rows = rows(&corpus);
    assert_eq!(rows.len(), capture.records.len());
    for (row, record) in rows.iter().zip(capture.records.iter()) {
        assert_eq!(row[0], record.game.to_string());
        assert_eq!(row[1], record.turns_played.to_string());
    }
}

#[test]
fn a_cp_score_becomes_an_eval_column_and_not_a_cp_one() {
    let scratch = Scratch::new("labels-score");
    let (corpus, _) = corpus_of(&scratch, "score");
    for row in rows(&corpus) {
        assert!(
            matches!(row[7].as_str(), "eval" | "mate_in" | "mated_in"),
            "a score kind outside the three reached the corpus: {}",
            row[7]
        );
        assert_ne!(row[7], "cp", "the wire's own word reached the corpus");
    }
}

#[test]
fn a_totals_line_without_solver_fields_yields_all_nodes_as_search_nodes() {
    let scratch = Scratch::new("labels-nodes");
    let (corpus, staged) = corpus_of(&scratch, "nodes");
    let capture_text = std::fs::read_to_string(&staged.capture).expect("readable");
    let capture = pistol_arena::capture::read(&capture_text).expect("reads");
    for (row, record) in rows(&corpus).iter().zip(capture.records.iter()) {
        let words: Vec<&str> = record
            .totals
            .strip_prefix("info totals ")
            .expect("a totals line")
            .split_whitespace()
            .collect();
        if words.contains(&"solver_nodes") {
            continue;
        }
        let nodes = words[words.iter().position(|w| *w == "nodes").expect("nodes") + 1];
        assert_eq!(row[11], nodes, "search_nodes is not the line's own nodes");
        assert_eq!(row[12], "0", "solver_nodes is not zero with the gate off");
    }
}

#[test]
fn side_to_move_comes_from_pistol_core_and_not_from_turn_parity() {
    let scratch = Scratch::new("labels-tomove");
    let (corpus, staged) = corpus_of(&scratch, "tomove");
    let transcript = pistol_arena::transcript::read(
        &staged.text,
        pistol_cli::sha256::sha256_hex(staged.text.as_bytes()),
    )
    .expect("the report reads");
    for row in rows(&corpus) {
        let game: usize = row[0].parse().expect("a game index");
        let k: usize = row[1].parse().expect("a turn count");
        let mut state = pistol_core::GameState::new_game();
        for turn in &transcript.games[game].moves[..k] {
            state.make_turn(*turn).expect("legal");
        }
        assert_eq!(
            row[6],
            state.to_move().name(),
            "game {game} turn {k}: the corpus disagrees with pistol-core about the mover"
        );
    }
}

#[test]
fn the_position_reached_by_the_whole_book_and_nothing_else_is_flagged_book() {
    let scratch = Scratch::new("labels-book-in");
    let (corpus, staged) = corpus_of(&scratch, "bookin");
    let transcript = pistol_arena::transcript::read(
        &staged.text,
        pistol_cli::sha256::sha256_hex(staged.text.as_bytes()),
    )
    .expect("the report reads");
    let boundary = transcript.opening_turns.to_string();
    let found = rows(&corpus)
        .into_iter()
        .find(|row| row[0] == "0" && row[1] == boundary)
        .expect("the boundary position was labelled");
    assert_eq!(
        found[13], "yes",
        "the position reached by the whole book and nothing else is not flagged book"
    );
}

#[test]
fn the_first_position_reached_by_an_engines_own_choice_is_flagged_not_book() {
    let scratch = Scratch::new("labels-book-out");
    let (corpus, staged) = corpus_of(&scratch, "bookout");
    let transcript = pistol_arena::transcript::read(
        &staged.text,
        pistol_cli::sha256::sha256_hex(staged.text.as_bytes()),
    )
    .expect("the report reads");
    let after = (transcript.opening_turns + 1).to_string();
    let found = rows(&corpus)
        .into_iter()
        .find(|row| row[0] == "0" && row[1] == after)
        .expect("the first engine-chosen position was labelled");
    assert_eq!(
        found[13], "no",
        "a position an engine chose to reach is flagged book"
    );
}

#[test]
fn a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus() {
    let scratch = Scratch::new("labels-outcome");
    let (corpus, _) = corpus_of(&scratch, "outcome");
    for row in rows(&corpus) {
        assert!(matches!(row[14].as_str(), "p1_win" | "p2_win" | "capped"));
        assert!(matches!(row[15].as_str(), "normal" | "forfeit"));
    }
}

#[test]
fn the_turn_zero_record_writes_a_dash_for_its_three_empty_columns() {
    let scratch = Scratch::new("labels-zero");
    let (corpus, _) = corpus_of(&scratch, "zero");
    let zero = rows(&corpus)
        .into_iter()
        .find(|row| row[1] == "0")
        .expect("a turn-zero record");
    assert_eq!(zero[2], "-", "moves");
    assert_eq!(zero[3], "-", "key_seq");
    assert_eq!(zero[5], "-", "key_full");
    assert_ne!(zero[4], "-", "key_pos needs no sentinel");
}

#[test]
fn a_corpus_file_round_trips_through_its_own_loader_field_by_field() {
    let scratch = Scratch::new("labels-trip");
    let (corpus, _) = corpus_of(&scratch, "trip");
    let read = pistol_arena::labels_file::read(&corpus).expect("its own loader reads it");
    let rows = rows(&corpus);
    assert_eq!(read.records.len(), rows.len());
    for (record, row) in read.records.iter().zip(rows.iter()) {
        assert_eq!(record.game.to_string(), row[0]);
        assert_eq!(record.moves, row[2]);
        assert_eq!(record.key_seq, row[3]);
        assert_eq!(record.key_pos, row[4]);
        assert_eq!(record.key_full, row[5]);
        assert_eq!(record.to_move, row[6]);
        assert_eq!(record.score_kind, row[7]);
        assert_eq!(record.score_value.to_string(), row[8]);
        assert_eq!(record.best, row[9]);
        assert_eq!(record.depth_turns.to_string(), row[10]);
        assert_eq!(record.search_nodes.to_string(), row[11]);
        assert_eq!(record.solver_nodes.to_string(), row[12]);
    }
}

#[test]
fn a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name() {
    let scratch = Scratch::new("labels-keyfull");
    let (corpus, _) = corpus_of(&scratch, "keyfull");
    let broken = rebuild(&corpus, |row| {
        if row[1] != "0" {
            row[5] = String::from("0,0 1,0");
        }
    });
    let error = pistol_arena::labels_file::read(&broken).expect_err("bare cells are refused");
    assert!(error.to_string().contains("key_full"), "{error}");
}

#[test]
fn a_key_seq_field_that_is_not_turn_tokens_is_refused_by_name() {
    let scratch = Scratch::new("labels-keyseq");
    let (corpus, _) = corpus_of(&scratch, "keyseq");
    let broken = rebuild(&corpus, |row| {
        if row[1] != "0" {
            row[3] = String::from("not-a-turn");
        }
    });
    let error = pistol_arena::labels_file::read(&broken).expect_err("junk is refused");
    assert!(error.to_string().contains("key_seq"), "{error}");
}

#[test]
fn a_score_kind_outside_the_three_is_refused_by_name() {
    let scratch = Scratch::new("labels-kind");
    let (corpus, _) = corpus_of(&scratch, "kind");
    let broken = rebuild(&corpus, |row| row[7] = String::from("cp"));
    let error = pistol_arena::labels_file::read(&broken).expect_err("`cp` is refused");
    assert!(error.to_string().contains("score kind"), "{error}");
}

#[test]
fn a_negative_mate_value_is_refused_by_name() {
    let scratch = Scratch::new("labels-mate");
    let (corpus, _) = corpus_of(&scratch, "mate");
    let broken = rebuild(&corpus, |row| {
        row[7] = String::from("mate_in");
        row[8] = String::from("-3");
    });
    let error = pistol_arena::labels_file::read(&broken).expect_err("a negative mate is refused");
    assert!(error.to_string().contains("turn count"), "{error}");
}

#[test]
fn a_corpus_missing_one_of_its_four_meaning_params_is_refused_by_name() {
    let scratch = Scratch::new("labels-params");
    let (corpus, _) = corpus_of(&scratch, "params");
    for key in ["score_units", "score_sign", "mate_counts", "depth_meaning"] {
        let stripped: String = corpus
            .lines()
            .filter(|line| !line.starts_with(&format!("# param {key} ")))
            .map(|line| format!("{line}\n"))
            .collect();
        let error = pistol_arena::labels_file::read(&stripped)
            .expect_err("a corpus missing a meaning param must be refused");
        assert!(
            error.to_string().contains(key),
            "the refusal did not name `{key}`: {error}"
        );
    }
}

#[test]
fn a_corpus_whose_body_digest_is_wrong_is_refused_by_name() {
    let scratch = Scratch::new("labels-digest");
    let (corpus, _) = corpus_of(&scratch, "digest");
    let tampered = format!(
        "{corpus}0\t0\t-\t-\t{}\t-\tp1\teval\t0\t0,0\t1\t1\t0\tyes\tcapped\tnormal\n",
        "0".repeat(32)
    );
    let error = pistol_arena::labels_file::read(&tampered).expect_err("a tampered body is refused");
    assert!(error.to_string().contains("digests to"), "{error}");
}

#[test]
fn a_rerun_over_one_capture_and_report_is_byte_identical() {
    let scratch = Scratch::new("labels-rerun");
    let staged = staged(&scratch, "rerun");
    let (_, one) = label(&scratch, &staged, "rerun-a");
    let (_, two) = label(&scratch, &staged, "rerun-b");
    assert_eq!(
        std::fs::read_to_string(&one).expect("one"),
        std::fs::read_to_string(&two).expect("two")
    );
}

#[test]
fn a_capture_whose_source_digest_is_not_the_reports_is_refused_by_name() {
    let scratch = Scratch::new("labels-bind");
    let staged_a = staged(&scratch, "binda");
    let staged_b = staged(&scratch, "bindb");
    let out = scratch.path("corpus-crossed.txt");
    let output = Command::new(ARENA)
        .arg("--labels")
        .arg(&staged_a.capture)
        .arg("--report")
        .arg(&staged_b.report)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(
        !out.exists(),
        "a crossed capture and report produced a corpus"
    );
    assert!(
        stderr.contains("taken from a report digesting"),
        "the refusal did not name the binding: {stderr}"
    );
}

#[test]
fn a_report_whose_result_contradicts_its_moves_refuses_the_run_by_name() {
    // The capture is taken from the FLIPPED report, so the digest binding and
    // the identity check both pass and the outcome cross-check is the thing
    // under test. Flipping the report AFTER capturing it would be refused two
    // checks earlier, and an assertion that accepted either refusal would pin
    // nothing.
    let scratch = Scratch::new("labels-outcome-check");
    let openings = scratch.write("openings-check.txt", &openings_prefix(OPENINGS));
    let config = scratch.stub_config("engine-check.toml", "honest");
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
    let ran = run(&scratch, &spec, "check");
    let flipped = ran.report().replace("result capped", "result p1_win");
    assert_ne!(
        flipped,
        ran.report(),
        "this fixture recorded no capped game"
    );
    let report = scratch.write("report-flipped.txt", &flipped);
    let capture = scratch.path("capture-flipped.txt");
    let captured = Command::new(ARENA)
        .arg("--capture")
        .arg(&report)
        .arg("--out")
        .arg(&capture)
        .arg("--label-nodes")
        .arg("5000")
        .output()
        .expect("the arena binary runs");
    assert!(
        capture.exists(),
        "the capture stage refused the flipped report, so this test cannot reach the outcome \
         check: {}",
        String::from_utf8_lossy(&captured.stderr)
    );
    let out = scratch.path("corpus-flipped.txt");
    let output = Command::new(ARENA)
        .arg("--labels")
        .arg(&capture)
        .arg("--report")
        .arg(&report)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!out.exists(), "a contradicted report produced a corpus");
    assert!(
        stderr.contains("its moves reach"),
        "the refusal did not name the outcome disagreement: {stderr}"
    );
}

#[test]
fn a_corpus_column_that_is_empty_or_carries_a_tab_refuses_the_run_by_name() {
    // `best` is the tail of a line an ENGINE wrote, so an empty column is
    // reachable rather than hypothetical — and the loader refuses one, so a
    // transform without this guard writes a corpus it cannot read back.
    let mut record = pistol_arena::labels_file::CorpusRecord {
        game: 2,
        turns_played: 4,
        moves: String::from("0,0"),
        key_seq: String::from("0,0"),
        key_pos: "0".repeat(32),
        key_full: String::from("0,0:p1"),
        to_move: String::from("p2"),
        score_kind: String::from("eval"),
        score_value: 0,
        best: String::new(),
        depth_turns: 1,
        search_nodes: 1,
        solver_nodes: 0,
        book: false,
        result: String::from("capped"),
        end: String::from("normal"),
    };
    let error = pistol_arena::labels::writable(&record).expect_err("an empty column is refused");
    assert!(
        error.to_string().contains("`best` column is empty")
            && error.to_string().contains("game 2"),
        "the refusal did not name the column and the record: {error}"
    );
    record.best = String::from("0,0\tbestmove");
    let error = pistol_arena::labels::writable(&record).expect_err("a TAB is refused");
    assert!(error.to_string().contains("TAB"), "{error}");
}

#[test]
fn a_capture_whose_bestmove_carries_no_turn_refuses_the_run_by_name() {
    // Drives the CALL, not the guard: a test that only calls `writable` pins the
    // function and leaves "the guard not called" alive — which is the defect
    // this arc's design round was failed for and which a mutation run caught
    // here too.
    //
    // `bestmove ` with a trailing space is what an engine can write and what the
    // capture carries verbatim; the transform strips the prefix and is left with
    // an empty `best` column, which its own loader refuses.
    let scratch = Scratch::new("labels-empty-best");
    let staged = staged(&scratch, "emptybest");
    let capture_text = std::fs::read_to_string(&staged.capture).expect("readable");
    let header: String = capture_text
        .lines()
        .take_while(|line| !line.starts_with("# body_sha256 "))
        .map(|line| format!("{line}\n"))
        .collect();
    let body: String = pistol_cli::corpus::emit::body_of(&capture_text)
        .expect("a body")
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(at, line)| {
            let mut fields: Vec<&str> = line.split('\t').collect();
            if at == 0 {
                fields[4] = "bestmove ";
            }
            format!("{}\n", fields.join("\t"))
        })
        .collect();
    let rewritten = format!(
        "{header}# body_sha256 {}\n{body}",
        pistol_cli::sha256::sha256_hex(body.as_bytes())
    );
    let capture = scratch.write("capture-emptybest.txt", &rewritten);
    let out = scratch.path("corpus-emptybest.txt");
    let output = Command::new(ARENA)
        .arg("--labels")
        .arg(&capture)
        .arg("--report")
        .arg(&staged.report)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!out.exists(), "an empty `best` column produced a corpus");
    assert!(
        stderr.contains("`best` column is empty"),
        "the refusal did not name the empty column: {stderr}"
    );
}

/// The loader, driven as the program that ships it
/// (tools/SHELL_CHECKLIST.md item 10).
const CORPUS_CHECK: &str = env!("CARGO_BIN_EXE_corpus-check");

/// The three codes spelled out, so a failure message says what the code the
/// test did NOT get would have meant (tools/SHELL_CHECKLIST.md item 12).
fn checked(paths: &[&std::path::Path]) -> (Option<i32>, String, String) {
    let output = Command::new(CORPUS_CHECK)
        .args(paths)
        .output()
        .expect("the corpus loader runs");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

/// THE CONTROL. A loader that refused everything would pass every refusal case
/// below and answer nothing.
#[test]
fn a_corpus_this_build_wrote_loads_through_the_reader_that_ships_with_the_writer() {
    let scratch = Scratch::new("labels-load-ok");
    let (corpus, _) = corpus_of(&scratch, "loadok");
    let path = scratch.write("corpus-loadok-copy.txt", &corpus);
    let (code, stdout, stderr) = checked(&[&path]);
    assert_eq!(
        code,
        Some(0),
        "a corpus this build wrote was not loadable by this build; 1 would have meant a          grammar refusal and 2 that no answer was taken.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let rows = rows(&corpus).len();
    assert!(
        stdout.contains(&format!("ok, {rows} record(s)")),
        "the loader did not say what it read ({rows} records): {stdout}"
    );
}

#[test]
fn a_corpus_carrying_an_injected_malformed_record_is_refused_loudly() {
    let scratch = Scratch::new("labels-load-bad");
    let (corpus, _) = corpus_of(&scratch, "loadbad");
    // A `key_pos` that is not thirty-two hex digits: reachable only by editing
    // the file, which is exactly the injection this check exists to refuse.
    let broken = rebuild(&corpus, |row| row[4] = String::from("not-a-key"));
    let path = scratch.write("corpus-loadbad.txt", &broken);
    let (code, stdout, stderr) = checked(&[&path]);
    assert_eq!(
        code,
        Some(1),
        "an injected malformed record was not refused; 0 would have meant it loaded and 2 \
         that no answer was taken.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("REFUSED") && stderr.contains("key_pos"),
        "the refusal did not name the column: {stderr}"
    );
}

#[test]
fn a_corpus_summarises_the_columns_a_pre_registration_reads_a_number_off() {
    // The summary line is what fixes the pre-registration's LABEL budget
    // (docs/experiments/wp20_pilot_prereg.md RULE-2), so it is a recorded number
    // and carries a test driving the shipped program
    // (tools/SHELL_CHECKLIST.md item 10).
    let scratch = Scratch::new("labels-summary");
    let (corpus, _) = corpus_of(&scratch, "summary");
    let path = scratch.write("corpus-summary-copy.txt", &corpus);
    let (code, stdout, stderr) = checked(&[&path]);
    assert_eq!(code, Some(0), "the control corpus did not load: {stderr}");

    // The referent is the corpus's own column, read here and NOT from the
    // program's output: a summary checked against itself is not checked.
    let rows = rows(&corpus);
    let mut depths: Vec<i64> = rows
        .iter()
        .map(|row| row[10].parse().expect("a depth column"))
        .collect();
    depths.sort_unstable();
    let n = depths.len();
    let expected = if n % 2 == 1 {
        depths[n / 2] as f64
    } else {
        (depths[n / 2 - 1] + depths[n / 2]) as f64 / 2.0
    };
    assert!(
        stdout.contains(&format!("depth_turns median {expected:.1} ")),
        "the summary's median is not the one the corpus's own column holds          ({expected:.1}): {stdout}"
    );
    assert!(
        stdout.contains(&format!("max {}", depths[n - 1])),
        "the summary's maximum is not the column's: {stdout}"
    );
    // The closed-set columns' spread, which is what tells a reader whether a
    // green load exercised one arm of a guard or both.
    let ends: std::collections::BTreeSet<&String> = rows.iter().map(|row| &row[15]).collect();
    assert!(
        stdout.contains(&format!("end {} (", ends.len())),
        "the summary did not say how many values `end` takes ({}): {stdout}",
        ends.len()
    );
}

#[test]
fn a_corpus_whose_body_digest_is_wrong_is_refused_by_the_shipped_loader() {
    // Drives the BINARY. Its sibling above pins the same refusal by calling
    // `labels_file::read` directly, which pins the function and not the call
    // (docs/decisions.md D-553) — and the pre-registration's own injection runs
    // the program, so the program is what must be shown to refuse.
    let scratch = Scratch::new("labels-digest-run");
    let (corpus, _) = corpus_of(&scratch, "digestrun");
    let tampered = format!(
        "{corpus}0\t0\t-\t-\t{}\t-\tp1\teval\t0\t0,0\t1\t1\t0\tyes\tcapped\tnormal\n",
        "0".repeat(32)
    );
    let path = scratch.write("corpus-digestrun.txt", &tampered);
    let (code, stdout, stderr) = checked(&[&path]);
    assert_eq!(
        code,
        Some(1),
        "an appended record with a stale digest was not refused; 0 would have meant it \
         loaded and 2 that no answer was taken.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("digests to"),
        "the refusal did not name the digest disagreement: {stderr}"
    );
}

#[test]
fn a_corpus_path_that_is_not_a_file_is_a_void_and_not_a_refusal() {
    let scratch = Scratch::new("labels-load-void");
    let missing = scratch.path("no-such-corpus.txt");
    let (code, stdout, stderr) = checked(&[&missing]);
    assert_eq!(
        code,
        Some(2),
        "an unreadable path was not a VOID; reading it as a grammar refusal is the defect \
         item 12 exists for.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("RUN VOID") && stderr.contains("NOT a refusal"),
        "the void did not say it was not a refusal: {stderr}"
    );
}

#[test]
fn a_corpus_path_carrying_a_control_character_is_a_void_before_it_is_printed() {
    // The item-9 boundary guard, driven at its CALL. A newline is legal in a
    // Linux filename, so this input is reachable rather than hypothetical, and a
    // path interpolated into the receipt this program prints would INJECT LINES
    // into a document somebody parses (tools/SHELL_CHECKLIST.md item 9). The
    // guard landed in a round whose own document invokes D-553 by name and it
    // landed with no test; this is that test.
    let scratch = Scratch::new("labels-load-ctrl");
    let (corpus, _) = corpus_of(&scratch, "loadctrl");
    let path = scratch.write("corpus\nwith-a-newline.txt", &corpus);
    let (code, stdout, stderr) = checked(&[&path]);
    assert_eq!(
        code,
        Some(2),
        "a path carrying a control character was not a VOID; 0 would have meant it was \
         read and printed.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("control character") && stderr.contains("NOT a refusal"),
        "the void did not name the boundary it guards: {stderr}"
    );
    assert!(
        !stdout.contains("record(s)"),
        "the guarded path reached the printed receipt anyway: {stdout}"
    );
}

#[test]
fn the_summary_reports_every_closed_set_column_the_loader_checks() {
    // The usage block promises "each of the closed-set columns", and
    // `labels_file`'s loader checks four: to_move, book, result, end.
    let scratch = Scratch::new("labels-tokensets");
    let (corpus, _) = corpus_of(&scratch, "tokensets");
    let path = scratch.write("corpus-tokensets-copy.txt", &corpus);
    let (code, stdout, stderr) = checked(&[&path]);
    assert_eq!(code, Some(0), "the control corpus did not load: {stderr}");
    for column in ["to_move", "book", "result", "end"] {
        assert!(
            stdout.contains(&format!("{column} ")),
            "the summary omits the `{column}` token set: {stdout}"
        );
    }
}

#[test]
fn naming_no_corpus_at_all_is_a_void() {
    let (code, stdout, stderr) = checked(&[]);
    assert_eq!(
        code,
        Some(2),
        "naming nothing was not a VOID.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// A capture rewritten through `edit`, re-digested honestly, on disk.
///
/// The header sits OUTSIDE the body digest (`emit::body_of` starts the body at
/// the `# body_sha256 ` line), so a header value a doctored capture claims
/// survives every digest check and reaches the transform's own bindings. That is
/// what makes those bindings testable rather than decorative.
fn doctored_capture(
    scratch: &Scratch,
    staged: &Staged,
    name: &str,
    edit: impl Fn(&mut Vec<String>, &mut Vec<String>),
) -> PathBuf {
    let text = std::fs::read_to_string(&staged.capture).expect("the capture is readable");
    let mut header: Vec<String> = text
        .lines()
        .take_while(|line| !line.starts_with("# body_sha256 "))
        .map(str::to_string)
        .collect();
    let mut body: Vec<String> = pistol_cli::corpus::emit::body_of(&text)
        .expect("a capture carries a body digest")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();
    edit(&mut header, &mut body);
    let rendered: String = body.iter().map(|line| format!("{line}\n")).collect();
    let whole = format!(
        "{}\n# body_sha256 {}\n{rendered}",
        header.join("\n"),
        pistol_cli::sha256::sha256_hex(rendered.as_bytes())
    );
    scratch.write(name, &whole)
}

/// Run `--labels` over an exact capture path and hand back `(stderr, wrote)`.
fn labels_at(scratch: &Scratch, capture: &PathBuf, report: &PathBuf, tag: &str) -> (String, bool) {
    let out = scratch.path(&format!("corpus-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--labels")
        .arg(capture)
        .arg("--report")
        .arg(report)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    (
        String::from_utf8_lossy(&output.stderr).to_string(),
        out.exists(),
    )
}

#[test]
fn a_capture_whose_header_identity_is_not_the_one_its_inputs_produce_is_refused_by_name() {
    // The identity binding, driven at its CALL: the header is outside the body
    // digest, so a claimed `capture_sha256` reaches the transform untouched and
    // only this check stands between it and a corpus attributed to a run that
    // never happened (docs/decisions.md D-553).
    let scratch = Scratch::new("labels-identity");
    let staged = staged(&scratch, "identity");
    let capture = doctored_capture(&scratch, &staged, "capture-identity.txt", |header, _| {
        for line in header.iter_mut() {
            if line.starts_with("# derived capture_sha256 ") {
                *line = format!("# derived capture_sha256 {}", "0".repeat(64));
            }
        }
    });
    let (stderr, wrote) = labels_at(&scratch, &capture, &staged.report, "identity");
    assert!(
        !wrote,
        "a capture claiming a foreign identity produced a corpus"
    );
    assert!(
        stderr.contains("its own inputs produce"),
        "the refusal did not name the identity disagreement: {stderr}"
    );
}

#[test]
fn a_capture_that_holds_no_record_of_one_of_the_reports_games_is_refused_by_name() {
    // A corpus over SOME of a report's games is a corpus over a sample nobody
    // registered. Driven at the call: every earlier check passes, because the
    // records that remain are the report's own.
    let scratch = Scratch::new("labels-missing-game");
    let staged = staged(&scratch, "missinggame");
    let capture = doctored_capture(&scratch, &staged, "capture-missing.txt", |_, body| {
        body.retain(|line| !line.starts_with("1\t"));
    });
    let (stderr, wrote) = labels_at(&scratch, &capture, &staged.report, "missinggame");
    assert!(!wrote, "a capture missing a whole game produced a corpus");
    assert!(
        stderr.contains("has no record in the capture"),
        "the refusal did not name the game the capture skipped: {stderr}"
    );
}

#[test]
fn a_captured_position_that_is_not_the_reports_own_prefix_is_refused_by_name() {
    // The record-grain half of the capture-to-report binding: the digests still
    // agree and the identity still holds, so this check is the only thing that
    // can see a record whose position is not the one its own turn count names.
    let scratch = Scratch::new("labels-prefix");
    let staged = staged(&scratch, "prefix");
    let capture = doctored_capture(&scratch, &staged, "capture-prefix.txt", |_, body| {
        for line in body.iter_mut() {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields[1] == "0" {
                let mut fields: Vec<String> = fields.iter().map(|f| (*f).to_string()).collect();
                fields[2] = String::from("position start moves 0,0/1,1");
                *line = fields.join("\t");
            }
        }
    });
    let (stderr, wrote) = labels_at(&scratch, &capture, &staged.report, "prefix");
    assert!(
        !wrote,
        "a record whose position is not its own prefix produced a corpus"
    );
    assert!(
        stderr.contains("is not the report's own prefix"),
        "the refusal did not name the position disagreement: {stderr}"
    );
}

#[test]
fn a_corpus_missing_its_opening_turns_param_is_refused_by_name() {
    let scratch = Scratch::new("labels-openingturns");
    let (corpus, _) = corpus_of(&scratch, "openingturns");
    let stripped: String = corpus
        .lines()
        .filter(|line| !line.starts_with("# param opening_turns "))
        .map(|line| format!("{line}\n"))
        .collect();
    let error = pistol_arena::labels_file::read(&stripped)
        .expect_err("a corpus missing `opening_turns` must be refused");
    assert!(error.to_string().contains("opening_turns"), "{error}");
}

#[test]
fn a_labels_run_prints_a_corpus_manifest_row_naming_its_digests() {
    let scratch = Scratch::new("labels-manifest");
    let staged = staged(&scratch, "manifest");
    let (output, out) = label(&scratch, &staged, "manifest");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let corpus = std::fs::read_to_string(&out).expect("the corpus is readable");
    let body = pistol_cli::corpus::emit::claimed_body_digest(&corpus).expect("a body digest");
    assert!(
        stdout.contains("corpus_manifest ") && stdout.contains(body),
        "no manifest row naming the corpus digest was printed: {stdout}"
    );
}

#[test]
fn two_transposed_positions_are_two_records_sharing_a_key_full() {
    // Two move ORDERS reaching one stone set: `key_pos` and `key_full` fold them,
    // `key_seq` does not, and the corpus deduplicates nothing.
    // A pair is an UNORDERED set and `2,0/1,0` is refused as uncanonical
    // (docs/decisions.md D-56), so a transposition here is two of one player's
    // TURNS swapped rather than the two cells of one turn.
    let one: Vec<pistol_core::Turn> = ["0,0", "0,5/2,5", "1,0/2,0", "4,5/6,5", "3,0/4,0"]
        .iter()
        .map(|t| t.parse().expect("a turn"))
        .collect();
    let two: Vec<pistol_core::Turn> = ["0,0", "0,5/2,5", "3,0/4,0", "4,5/6,5", "1,0/2,0"]
        .iter()
        .map(|t| t.parse().expect("a turn"))
        .collect();
    assert_ne!(
        one, two,
        "the two sequences must differ to be a transposition"
    );
    let key = |moves: &[pistol_core::Turn]| {
        let mut state = pistol_core::GameState::new_game();
        for turn in moves {
            state.make_turn(*turn).expect("legal");
        }
        let stones: Vec<(pistol_core::Coord, pistol_core::Player)> =
            state.board().stones().collect();
        (
            state.key().to_string(),
            pistol_core::canonical_form(&stones),
        )
    };
    let (pos_one, full_one) = key(&one);
    let (pos_two, full_two) = key(&two);
    assert_eq!(pos_one, pos_two, "two orders of one turn are one position");
    assert_eq!(full_one, full_two, "`key_full` folds transposition");

    // And the corpus itself deduplicates nothing: every capture record becomes a
    // record, so two positions sharing a key are two rows.
    let scratch = Scratch::new("labels-dedupe");
    let (corpus, _) = corpus_of(&scratch, "dedupe");
    let read = pistol_arena::labels_file::read(&corpus).expect("the corpus reads");
    let mut keys: Vec<&str> = read
        .records
        .iter()
        .map(|record| record.key_full.as_str())
        .collect();
    let total = keys.len();
    keys.sort_unstable();
    keys.dedup();
    assert!(
        total >= keys.len(),
        "the corpus holds fewer records than distinct keys, which is impossible"
    );
    assert_eq!(
        total,
        read.records.len(),
        "the corpus deduplicated records on write"
    );
}

/// Rewrite every body record through `edit`, re-digesting honestly so the only
/// thing under test is the field that changed.
fn rebuild(corpus: &str, edit: impl Fn(&mut Vec<String>)) -> String {
    let header: String = corpus
        .lines()
        .take_while(|line| !line.starts_with("# body_sha256 "))
        .map(|line| format!("{line}\n"))
        .collect();
    let mut body = String::new();
    for row in rows(corpus) {
        let mut row = row;
        edit(&mut row);
        body.push_str(&row.join("\t"));
        body.push('\n');
    }
    format!(
        "{header}# body_sha256 {}\n{body}",
        pistol_cli::sha256::sha256_hex(body.as_bytes())
    )
}
