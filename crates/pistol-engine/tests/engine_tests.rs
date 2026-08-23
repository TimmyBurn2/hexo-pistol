//! The `Engine` seam: what each verb promises, and what it refuses.
//!
//! These tests use the trait, not the protocol. The protocol's own tests live in
//! pistol-cli and drive the same engine through text; if the two ever disagree,
//! the protocol is the one that is wrong (docs/decisions.md D-5).
//!
//! # RULE9-JUSTIFICATION: one seam, over every verb it exposes (CLAUDE.md rule 9).
//!
//! `new_game`, `set_position` and `go`/`go_reporting` are read and asserted
//! together because a caller of `Engine` uses them together — a test isolating
//! one verb from the position and mode it runs under would either rebuild that
//! context per file or share it through a module boundary the trait itself
//! does not have. It grows again only if `Engine` gains a verb.

mod common;

use common::{VALID, VALID_STAGED, buildable, replacing};
use pistol_core::{Coord, Phase, Player, Turn};
use pistol_engine::{Budget, Engine, EngineError, EngineMode, Pistol, PositionSpec, ScoreKind};

/// An engine from a document.
fn engine(document: &str) -> Pistol {
    Pistol::from_config(buildable(document))
        .unwrap_or_else(|error| panic!("this document should build an engine: {error}"))
}

/// The instrument-mode engine every test starts from, with a candidate radius of
/// 1 so that a test costs milliseconds rather than seconds. Stated here rather
/// than taken from a committed config, because a test states its own values
/// (CLAUDE.md rule 1).
fn instrument() -> Pistol {
    engine(&replacing("radius = 3", "radius = 1"))
}

/// The same, in play mode.
fn play() -> Pistol {
    engine(&replacing("radius = 3", "radius = 1").replace("\"instrument\"", "\"play\""))
}

/// An instrument-mode engine under `CandidatePolicy::Staged`
/// (`U3_tier_t.md` §10), for the one test that checks the config layer's
/// mapping reaches a working search.
fn staged() -> Pistol {
    engine(VALID_STAGED)
}

/// A position with a mate in one for the side to move: five in a row, one end
/// blocked, so exactly one cell completes six.
fn mate_in_one() -> PositionSpec {
    "set p1:0,0 1,0 2,0 3,0 4,0 p2:-1,0 1,3 2,3 3,3 1,5 2,5 tomove:p1 phase:0"
        .parse()
        .expect("a well-formed stone list")
}

#[test]
fn a_new_engine_stands_on_the_initial_position() {
    let engine = instrument();
    assert_eq!(engine.mode(), EngineMode::Instrument);
    assert!(engine.state().board().is_empty());
    assert_eq!(engine.state().turn(), 1);
    assert_eq!(engine.state().to_move(), Player::P1);
    assert_eq!(engine.state().phase(), Phase::First);
}

#[test]
fn go_answers_with_the_move_and_the_evidence() {
    let mut engine = instrument();
    engine
        .set_position(&mate_in_one())
        .expect("a legal position");
    let outcome = engine
        .go(Budget::DepthTurns(1))
        .expect("an ongoing position");

    assert_eq!(
        outcome.best,
        Turn::Single(Coord::new(5, 0)),
        "the one cell that completes six ends the turn (rule 4)"
    );
    assert_eq!(
        pistol_engine::classify(outcome.info.score),
        ScoreKind::MateIn(1)
    );
    assert_eq!(outcome.info.pv.first(), Some(&outcome.best));
    assert_eq!(outcome.info.depth_turns, 1);
    assert!(outcome.info.nodes >= 1, "a search visits nodes");
}

/// `instance.rs::search_policy`'s `Staged` arm reaches a working search: the
/// config layer's mapping from `pistol_engine::config::CandidatePolicy::Staged`
/// to `pistol_search::CandidatePolicy::Staged(StagedParams)` is exercised
/// end-to-end, not only unit-checked (docs/decisions.md D-353).
#[test]
fn a_staged_engine_finds_the_same_mate_a_radius_engine_does() {
    let mut engine = staged();
    engine
        .set_position(&mate_in_one())
        .expect("a legal position");
    let outcome = engine
        .go(Budget::DepthTurns(1))
        .expect("an ongoing position");

    assert_eq!(
        outcome.best,
        Turn::Single(Coord::new(5, 0)),
        "the one cell that completes six ends the turn (rule 4), under Staged exactly as \
         under Radius"
    );
    assert_eq!(
        pistol_engine::classify(outcome.info.score),
        ScoreKind::MateIn(1)
    );
    assert!(
        outcome.info.stages.win_now >= 1,
        "the staged node protocol's own counters must have fired: {:?}",
        outcome.info.stages
    );
}

#[test]
fn go_reporting_reports_once_per_completed_depth() {
    let mut engine = instrument();
    engine
        .set_position(
            &"set p1:0,0 2,0 0,2 2,2 1,4 p2:1,0 0,1 1,1 2,1 0,3 1,3 tomove:p1 phase:0"
                .parse()
                .expect("a quiet position"),
        )
        .expect("a legal position");

    let mut depths = Vec::new();
    let outcome = engine
        .go_reporting(Budget::DepthTurns(3), &mut |info| {
            depths.push((info.depth_turns, info.nodes));
        })
        .expect("an ongoing position");

    assert_eq!(
        depths.iter().map(|(depth, _)| *depth).collect::<Vec<u32>>(),
        vec![1, 2, 3],
        "one report per completed depth, in order"
    );
    assert!(
        depths.windows(2).all(|pair| pair[0].1 <= pair[1].1),
        "the node count only grows: {depths:?}"
    );
    // The outcome carries the last completed depth's line with the whole search's
    // totals (docs/decisions.md D-80).
    assert_eq!(outcome.info.depth_turns, 3);
    assert_eq!(outcome.info.nodes, depths.last().expect("three reports").1);
}

#[test]
fn go_honours_each_budget_kind_its_mode_allows() {
    let mut engine = play();
    let position: PositionSpec = "start moves 0,0 1,0/2,0".parse().expect("a move list");
    for budget in [
        Budget::DepthTurns(2),
        Budget::Nodes(2048),
        Budget::MovetimeMs(50),
    ] {
        engine.new_game();
        engine.set_position(&position).expect("a legal position");
        let outcome = engine
            .go(budget)
            .unwrap_or_else(|error| panic!("play mode honours {budget:?}: {error}"));
        assert!(
            outcome.info.depth_turns >= 1,
            "a completed depth backs the answer: {outcome:?}"
        );
        assert_eq!(outcome.info.pv.first(), Some(&outcome.best));
    }
}

#[test]
fn instrument_mode_refuses_a_wall_clock_budget_by_name() {
    let mut engine = instrument();
    assert_eq!(
        engine.go(Budget::MovetimeMs(10)),
        Err(EngineError::InstrumentBudgetUnsupported),
        "wall clock cannot be reproduced (docs/decisions.md D-22)"
    );
    // And the same engine still works under the budgets it does allow.
    assert!(engine.go(Budget::DepthTurns(1)).is_ok());
}

#[test]
fn a_zero_or_out_of_range_budget_is_a_named_refusal() {
    let mut engine = instrument();
    for budget in [
        Budget::DepthTurns(0),
        Budget::Nodes(0),
        Budget::DepthTurns(65),
    ] {
        match engine.go(budget).err() {
            Some(EngineError::Config { key, .. }) => {
                assert!(key.starts_with("budget."), "{key}");
            }
            other => panic!("{budget:?} should be refused by name, got {other:?}"),
        }
    }
}

#[test]
fn go_on_a_half_played_turn_says_there_is_nothing_to_search() {
    // A `phase:1` position is legal and is one the protocol can be handed
    // (docs/decisions.md D-6). The search counts and reports in turns, so it
    // starts at a turn boundary (D-50, D-71) — and says so rather than searching
    // half a turn or inventing a one-stone turn the rules do not have.
    let mut engine = instrument();
    engine
        .set_position(
            &"set p1:0,0 1,0 p2:0,1 1,1 tomove:p1 phase:1"
                .parse()
                .expect("a turn in progress"),
        )
        .expect("a legal position");
    match engine.go(Budget::DepthTurns(1)).err() {
        Some(EngineError::PositionNotSearchable { why }) => {
            assert!(why.contains("half played"), "{why}");
        }
        other => panic!("expected PositionNotSearchable, got {other:?}"),
    }
}

#[test]
fn new_game_forgets_the_position_and_everything_learned() {
    // Rule 4's determinism law in one process: the same position and budget twice,
    // once from a fresh engine and once from an engine that has played a different
    // game and been told `newgame`, must agree node for node (D-7).
    let position = mate_in_one();
    let other: PositionSpec = "start moves 0,0 1,0/2,0 -1,1/0,1"
        .parse()
        .expect("a move list");

    let mut fresh = instrument();
    fresh.set_position(&position).expect("a legal position");
    let first = fresh
        .go(Budget::DepthTurns(2))
        .expect("an ongoing position");

    let mut reused = instrument();
    reused.set_position(&other).expect("a legal position");
    reused
        .go(Budget::DepthTurns(2))
        .expect("an ongoing position");
    reused.new_game();
    assert!(
        reused.state().board().is_empty(),
        "a new game has no stones"
    );
    reused.set_position(&position).expect("a legal position");
    let second = reused
        .go(Budget::DepthTurns(2))
        .expect("an ongoing position");

    assert_eq!(first.best, second.best);
    assert_eq!(first.info.nodes, second.info.nodes);
    assert_eq!(first.info.pv, second.info.pv);
    assert_eq!(first.info.score, second.info.score);
    assert_eq!(first.info.hashfull_permille, second.info.hashfull_permille);
}

#[test]
fn a_refused_position_leaves_the_engine_where_it_was() {
    let mut engine = instrument();
    let good: PositionSpec = "start moves 0,0 1,0/2,0".parse().expect("a move list");
    engine.set_position(&good).expect("a legal position");
    let before = engine.state().key();

    let bad: PositionSpec = "set p1:0,0 1,0 p2:0,1 tomove:p1 phase:0"
        .parse()
        .expect("parses, but is no position");
    assert!(engine.set_position(&bad).is_err());
    assert_eq!(
        engine.state().key(),
        before,
        "a refused position changes nothing (CLAUDE.md rule 3)"
    );
    assert!(
        engine.go(Budget::DepthTurns(1)).is_ok(),
        "and still searches"
    );
}

#[test]
fn mode_token_matches_the_document() {
    // The `serde` rename and `EngineMode::token` are two spellings of one word,
    // and a rename cannot be read at run time. This is the pin that they agree.
    for (document, mode, token) in [
        (VALID.to_string(), EngineMode::Instrument, "instrument"),
        (
            replacing("\"instrument\"", "\"play\""),
            EngineMode::Play,
            "play",
        ),
    ] {
        let config = common::accepted(&document);
        assert_eq!(config.engine.mode, mode);
        assert_eq!(config.engine.mode.token(), token);
        assert!(
            document.contains(&format!("mode = \"{token}\"")),
            "the document spells it the same way"
        );
    }
}

#[test]
fn eval_backend_token_matches_the_document() {
    let config = common::accepted(VALID);
    assert_eq!(config.eval.backend.token(), "handcrafted_v0");
    assert!(VALID.contains("backend = \"handcrafted_v0\""));
}

#[test]
fn the_table_never_takes_more_than_the_config_allows() {
    // docs/decisions.md D-75: `tt_bytes` is a ceiling honoured, not a stated value
    // silently rounded up.
    let engine = instrument();
    let allowed = engine.config().search.tt_bytes;
    assert!(
        engine.table_bytes() <= allowed,
        "{} > {allowed}",
        engine.table_bytes()
    );
    assert!(engine.table_bytes() > 0);
}
