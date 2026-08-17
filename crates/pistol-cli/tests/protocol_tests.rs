//! The line protocol: what each verb does, and what every refusal says.
//!
//! Every test here drives a real engine through [`pistol_cli::Session`], which is
//! the same code the binary runs (docs/decisions.md D-5). Positions are small and
//! budgets are shallow, because these tests are about the protocol rather than
//! about the search.

mod common;

use common::{GATE, INSTRUMENT, PLAY, committed, engine, has_line, only_line, talk};
use pistol_engine::{Budget, Config, Engine, EngineError, Pistol};

/// The `error <NamedError>` word of the one refusal in these answers.
fn refusal(answers: &[String]) -> String {
    let line = only_line(answers, "error");
    line.split_whitespace()
        .nth(1)
        .unwrap_or_else(|| panic!("an error line names its error: {line}"))
        .trim_end_matches(':')
        .to_string()
}

#[test]
fn protocol_handshake_answers_id_lines_then_pistolok() {
    let mut engine = engine(INSTRUMENT);
    let answers = talk(&mut engine, &["pistol"]);

    assert_eq!(
        answers.last().map(String::as_str),
        Some("pistolok"),
        "the handshake ends with pistolok: {answers:?}"
    );
    assert!(answers.iter().any(|line| line == "id name pistol"));
    assert!(answers.iter().any(|line| line == "id protocol v0"));
    assert!(answers.iter().any(|line| line == "id mode instrument"));
    assert!(
        answers
            .iter()
            .any(|line| line == "id budgets depth_turns nodes"),
        "instrument mode names the two reproducible budgets: {answers:?}"
    );
    assert!(
        answers.iter().all(|line| !line.contains('\n')),
        "every answer is one line: {answers:?}"
    );
}

#[test]
fn protocol_rejects_malformed_lines_and_stays_alive() {
    let mut engine = engine(GATE);
    for line in [
        "",                                           // not a verb
        "   ",                                        // still not a verb
        "ucinewgame",                                 // not one of ours
        "pistol extra",                               // takes no arguments
        "quit now",                                   // likewise
        "position",                                   // neither form
        "position sideways b:0,0",                    // no such form
        "position start moves",                       // a keyword with nothing after it
        "position start moves 0,0 1,0/",              // a turn token that is not one
        "position set b:0,0",                         // sections are all required
        "position set b: 0,0 w: tomove:b phase:0",    // a stone must follow its prefix
        "position set w: b:0,0 tomove:b phase:0",     // sections come in order
        "position set b:0,0 w: tomove:black phase:0", // b or w, not the word
        "position set b:0,0 w: tomove:b phase:2",     // 0 or 1
        "go 4",                                       // no such budget
        "go depth_turns",                             // no amount
        "go depth_turns four",                        // not an amount
        "go depth_turns 1 2",                         // one budget
        "go movetime 10",                             // instrument mode refuses wall clock
    ] {
        let answers = talk(&mut engine, &[line]);
        assert_eq!(
            answers.len(),
            1,
            "`{line}` gets exactly one answer: {answers:?}"
        );
        assert!(
            answers[0].starts_with("error "),
            "`{line}` is refused: {}",
            answers[0]
        );
    }
    // Still alive, and still able to work.
    let answers = talk(&mut engine, &["position start", "go depth_turns 1"]);
    assert!(has_line(&answers, "bestmove"), "{answers:?}");
}

#[test]
fn a_refusal_names_the_input_without_echoing_it() {
    // A refusal quotes what it refused, and whoever is on the other end of the
    // pipe chose that input: ten megabytes of nonsense must not come back as ten
    // megabytes (docs/decisions.md D-88).
    let mut engine = engine(GATE);
    let huge = "y".repeat(2_000_000);
    let answers = talk(&mut engine, &[&huge]);
    assert_eq!(answers.len(), 1, "one refusal");
    assert!(
        answers[0].chars().count() <= pistol_cli::report::MAX_REFUSAL_CHARS + 1,
        "a refusal is bounded, got {} characters",
        answers[0].chars().count()
    );
    assert!(answers[0].starts_with("error Protocol: unknown verb `yyy"));
    // And a control byte never reaches the pipe unescaped.
    let answers = talk(&mut engine, &["a\u{0}b"]);
    assert_eq!(answers.len(), 1);
    assert!(
        !answers[0].chars().any(|c| c.is_control()),
        "no control byte in {:?}",
        answers[0]
    );
}

#[test]
fn protocol_go_without_a_budget_is_budget_missing() {
    // Not a protocol complaint: a budget is always explicit, and this is the
    // named error that says so (docs/decisions.md D-4).
    let mut engine = engine(GATE);
    let answers = talk(&mut engine, &["position start", "go"]);
    assert_eq!(refusal(&answers), "BudgetMissing");
}

#[test]
fn go_rejects_zero_and_out_of_range_budget_amounts() {
    let mut engine = engine(GATE);
    for line in ["go depth_turns 0", "go nodes 0", "go depth_turns 65"] {
        let answers = talk(&mut engine, &["position start", line]);
        assert_eq!(
            refusal(&answers),
            "Config",
            "`{line}` is a value the engine will not honour: {answers:?}"
        );
        assert!(
            only_line(&answers, "error").contains("budget."),
            "the refusal names the budget key: {answers:?}"
        );
    }
}

#[test]
fn instrument_mode_rejects_movetime_budget() {
    // Wall clock cannot be reproduced, and instrument mode is where every
    // strength claim comes from (docs/decisions.md D-22).
    let mut engine = engine(INSTRUMENT);
    let answers = talk(&mut engine, &["position start", "go movetime 10"]);
    assert_eq!(refusal(&answers), "InstrumentBudgetUnsupported");

    // The same refusal through the trait, without the protocol in between.
    let mut direct = Pistol::from_config(committed(INSTRUMENT)).expect("the committed config");
    assert_eq!(
        direct.go(Budget::MovetimeMs(10)),
        Err(EngineError::InstrumentBudgetUnsupported)
    );

    // And the handshake says so in advance: `movetime` is not among the budgets
    // this mode names.
    let handshake = talk(&mut direct, &["pistol"]);
    let budgets = handshake
        .iter()
        .find(|line| line.starts_with("id budgets "))
        .unwrap_or_else(|| panic!("the handshake names its budgets: {handshake:?}"));
    assert!(!budgets.contains("movetime"), "{budgets}");
}

#[test]
fn go_works_under_all_three_budgets_in_play_mode() {
    // Play mode honours every budget kind, wall clock included.
    let mut engine = engine(PLAY);
    for budget in ["depth_turns 1", "nodes 1024", "movetime 50"] {
        let answers = talk(
            &mut engine,
            &[
                "newgame",
                "position start moves 0,0 1,0/2,0",
                &format!("go {budget}"),
            ],
        );
        assert!(
            !answers.iter().any(|line| line.starts_with("error ")),
            "`go {budget}` is honoured in play mode: {answers:?}"
        );
        let best = only_line(&answers, "bestmove");
        assert!(
            best.split_whitespace().count() == 2,
            "one turn token follows bestmove: {best}"
        );
        let totals = answers
            .iter()
            .find(|line| line.starts_with("info totals "))
            .unwrap_or_else(|| panic!("a totals line precedes bestmove: {answers:?}"));
        assert!(totals.contains(" nodes "), "{totals}");
    }
    assert_eq!(
        committed(PLAY).engine.mode,
        pistol_engine::EngineMode::Play,
        "the committed play config is the one this test is about"
    );
}

#[test]
fn protocol_reports_one_info_line_per_completed_depth_then_a_totals_line() {
    let mut engine = engine(GATE);
    let answers = talk(
        &mut engine,
        &[
            "position set b:0,0 2,0 0,2 2,2 1,4 w:1,0 0,1 1,1 2,1 0,3 1,3 tomove:b phase:0",
            "go depth_turns 3",
        ],
    );
    let depths: Vec<&String> = answers
        .iter()
        .filter(|line| line.starts_with("info depth_turns "))
        .collect();
    assert_eq!(
        depths.len(),
        3,
        "one report per completed depth: {answers:?}"
    );
    for (index, line) in depths.iter().enumerate() {
        assert!(
            line.starts_with(&format!("info depth_turns {}", index + 1)),
            "the reports come in order: {line}"
        );
        assert!(
            line.contains(" pv "),
            "every report carries its line: {line}"
        );
    }
    // The totals line repeats the last depth with the whole search's cost
    // (docs/decisions.md D-80), and `bestmove` is last.
    assert_eq!(
        answers
            .iter()
            .filter(|line| line.starts_with("info totals "))
            .count(),
        1,
        "exactly one totals line: {answers:?}"
    );
    assert!(
        answers[answers.len() - 2].starts_with("info totals depth_turns 3"),
        "the totals line comes just before bestmove: {answers:?}"
    );
    assert!(answers.last().unwrap().starts_with("bestmove "));
}

#[test]
fn newgame_forgets_the_position_and_the_game() {
    let mut engine = engine(GATE);
    talk(&mut engine, &["position start moves 0,0 1,0/2,0"]);
    assert_eq!(engine.state().board().stone_count(), 3);
    talk(&mut engine, &["newgame"]);
    assert!(
        engine.state().board().is_empty(),
        "a new game has no stones"
    );
    assert_eq!(engine.state().turn(), 1);
}

#[test]
fn quit_stops_reading_and_nothing_else() {
    let mut engine = engine(GATE);
    let mut session = pistol_cli::Session::new(&mut engine);
    let mut answers = Vec::new();
    let flow = session.line("quit", &mut |line| answers.push(line.to_string()));
    assert_eq!(flow, pistol_cli::Flow::Quit);
    assert!(answers.is_empty(), "quit says nothing: {answers:?}");
}

#[test]
fn a_config_that_names_a_missing_weights_file_is_refused_by_name() {
    // The loud load-time half of docs/decisions.md D-21: config validation stays
    // pure, and the engine is what refuses a weights file that is not there.
    let mut config = committed(GATE);
    config.eval.weights_file = common::repo("configs/does_not_exist.toml");
    match Pistol::from_config(config).err() {
        Some(EngineError::Config { key, why }) => {
            assert_eq!(key, "eval.weights_file");
            assert!(why.contains("does_not_exist"), "{why}");
        }
        other => panic!("expected a named config refusal, got {other:?}"),
    }
}

#[test]
fn a_config_this_build_cannot_honour_is_refused_at_construction() {
    // A key no code reads is an instruction that silently did nothing. This build
    // searches single-threaded in every mode, so it says so rather than ignoring
    // the number (CLAUDE.md rule 3).
    let text = std::fs::read_to_string(common::repo(PLAY)).expect("the committed play config");
    let config = Config::parse_unvalidated(&text.replace("threads = 1", "threads = 4"))
        .expect("the edit keeps the document well formed");
    match Pistol::from_config(config).err() {
        Some(EngineError::Config { key, why }) => {
            assert_eq!(key, "instrument.threads");
            assert!(why.contains("single-threaded"), "{why}");
        }
        other => panic!("expected a named config refusal, got {other:?}"),
    }
}
