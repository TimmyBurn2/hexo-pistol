//! What the arena does when an engine stops behaving.
//!
//! Split from `run_tests.rs` on the line that matters for reading them: that
//! file is about a run where everything worked, and this one is about the four
//! ways it does not — a turn the rules refuse, a peer that stops speaking the
//! protocol, a peer that goes silent, and a peer a strength claim may not come
//! from. The refusals differ in KIND and the distinction is load-bearing: a
//! deterministic wrong answer is adjudicated, and silence is not
//! (docs/decisions.md D-159).

mod common;

use common::{Scratch, openings_prefix, run, self_match};

/// A cap that leaves room for a real game past the four-turn openings.
const TURN_CAP: u32 = 10;
/// Openings for the tests that play a full pair set.
const OPENINGS: usize = 4;

#[test]
fn arena_forfeits_engine_that_plays_illegal_turn() {
    let scratch = Scratch::new("illegal");
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    let illegal = scratch.stub_config("illegal.toml", "illegal");
    let mut spec = self_match(&openings, &honest_config, OPENINGS, TURN_CAP, 1);
    spec.config_b = &illegal;
    let ran = run(&scratch, &spec, "illegal");

    assert_eq!(
        ran.code(),
        1,
        "a run with a forfeit in it does not exit clean"
    );
    let games = ran.games();
    let forfeited: Vec<&&str> = games
        .iter()
        .filter(|game| game.contains("end forfeit"))
        .collect();
    assert!(
        !forfeited.is_empty(),
        "the illegal engine forfeited:\n{}",
        ran.report()
    );
    for game in &forfeited {
        assert!(
            game.contains("forfeit_by b"),
            "the report names the offender, not just the reason: {game}"
        );
        assert!(
            game.contains("reason illegal_turn"),
            "and names the reason: {game}"
        );
    }
    // The verbatim refusal is recorded, so the forfeit is diagnosable from the
    // report alone.
    assert!(
        ran.report()
            .lines()
            .any(|line| line.starts_with("refusal ")),
        "the offending turn and the rules' account of it are recorded"
    );
    // The verdict token itself says the run is not a measurement — a count on a
    // neighbouring line would read green to anything that greps for `verdict`.
    assert_eq!(ran.field("verdict"), "invalid_forfeit");
    assert_ne!(
        ran.field("verdict_if_clean"),
        "none",
        "and what it would have concluded is still stated, because a silent skip \
         is what rule 3 forbids"
    );
    // A forfeit is a LOSS, not a discarded game: engine b lost every game.
    let counts = ran.field("counts");
    assert!(
        counts.contains(&format!("wins_a {}", OPENINGS * 2)),
        "the forfeits scored as losses for b: {counts}"
    );
}

#[test]
fn arena_forfeits_engine_that_stops_speaking_the_protocol() {
    // Three deviations, one test, because they share a shape: the engine gave a
    // deterministic wrong answer to a deterministic input, which is adjudicable.
    let scratch = Scratch::new("protocol");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    for (behave, expected) in [
        ("garbage", "protocol_error"),
        ("bad_bestmove", "bad_bestmove"),
        ("exit", "engine_exited"),
    ] {
        let broken = scratch.stub_config(&format!("{behave}.toml"), behave);
        let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
        spec.config_b = &broken;
        let ran = run(&scratch, &spec, behave);
        assert_eq!(ran.code(), 1, "{behave} forfeits, so the run is not clean");
        assert!(
            ran.report().contains(&format!("reason {expected}")),
            "{behave} is reported as {expected}:\n{}",
            ran.report()
        );
        assert_eq!(ran.field("verdict"), "invalid_forfeit");
    }
}

#[test]
fn arena_aborts_run_when_engine_hangs() {
    // Silence is not an answer. A timeout that forfeited a game would make the
    // verdict a function of how fast this machine is, so it abandons the run
    // instead (CLAUDE.md rule 4, docs/decisions.md D-159).
    let scratch = Scratch::new("hang");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    let hanging = scratch.stub_config("hang.toml", "hang");
    let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
    spec.config_b = &hanging;
    spec.hang_ms = 400;
    let ran = run(&scratch, &spec, "hang");

    assert_eq!(ran.code(), 1, "an abandoned run does not exit clean");
    let report = ran.report();
    assert!(
        report.starts_with("arena_report_aborted "),
        "an abandoned run writes a report of a DIFFERENT kind, so nothing can read a \
         verdict off it:\n{report}"
    );
    assert!(
        !report.lines().any(|line| line.starts_with("verdict ")),
        "and it carries no verdict at all:\n{report}"
    );
    assert!(
        report.lines().any(|line| line.starts_with("aborted Hung")),
        "it names why:\n{report}"
    );
    assert!(
        String::from_utf8_lossy(&ran.output.stderr).contains("Hung"),
        "and says so on stderr"
    );
}

#[test]
fn arena_refuses_an_engine_a_strength_claim_may_not_come_from() {
    // Both refusals happen at the handshake the arena makes BEFORE any game, so
    // they exit 2 — "a document this build refuses" — and write no report at
    // all, rather than abandoning a run that had started. That is the point of
    // gathering each engine's identity up front: an engine a strength claim may
    // not come from should cost nothing, not a partial run
    // (docs/decisions.md D-162).
    let scratch = Scratch::new("refuse-engine");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    for (behave, expected) in [("play_mode", "instrument"), ("bad_protocol", "protocol")] {
        let broken = scratch.stub_config(&format!("{behave}.toml"), behave);
        let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
        spec.config_b = &broken;
        let ran = run(&scratch, &spec, behave);
        assert_eq!(
            ran.code(),
            2,
            "{behave} is refused before any game is played, not after"
        );
        assert!(
            ran.report.is_none(),
            "and nothing is written, because there is no run to report on"
        );
        let stderr = String::from_utf8_lossy(&ran.output.stderr);
        assert!(
            stderr.contains("Handshake"),
            "{behave} is a handshake refusal: {stderr}"
        );
        assert!(
            stderr.contains(expected),
            "{behave}'s refusal says what was wrong with it: {stderr}"
        );
    }
}

#[test]
fn arena_refuses_to_overwrite_an_existing_report() {
    let scratch = Scratch::new("overwrite");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let stub = scratch.stub_config("honest.toml", "honest");
    let spec = self_match(&openings, &stub, 1, TURN_CAP, 1);
    let config = scratch.write("arena.toml", &spec.render());
    let out = scratch.write("report.txt", "an earlier run's evidence\n");
    let output = std::process::Command::new(common::ARENA)
        .arg("--config")
        .arg(&config)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena runs");
    assert_eq!(output.status.code(), Some(2), "a refusal before any work");
    assert_eq!(
        std::fs::read_to_string(&out).expect("the file is still there"),
        "an earlier run's evidence\n",
        "and the earlier report is untouched"
    );
}
