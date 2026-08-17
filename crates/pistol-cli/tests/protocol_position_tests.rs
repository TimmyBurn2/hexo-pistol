//! The `position` verb: both forms, what they land on, and what they refuse.
//!
//! Split from `protocol_tests` for size (CLAUDE.md rule 9). The engine-level tests
//! of the same value live in pistol-engine's `position_tests`; these drive it
//! through the protocol, which is where an operator meets it.

mod common;

use common::{GATE, engine, only_line, talk};
use pistol_engine::{Engine, PositionSpec};

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
fn protocol_position_moves_roundtrip() {
    // A move list is the canonical encoding of a position (docs/decisions.md
    // D-6). Two things have to hold: the tail the engine accepts is the tail it
    // would write, and the position it lands on is the one the turns describe.
    let tail = "start moves 0,0 1,0/2,0 -1,1/0,1";
    let spec: PositionSpec = tail.parse().expect("a well-formed move list");
    assert_eq!(
        spec.to_string(),
        tail,
        "a position the parser accepts is spelled back the same way"
    );

    let mut engine = engine(GATE);
    let answers = talk(&mut engine, &[&format!("position {tail}")]);
    assert!(
        answers.is_empty(),
        "a position that was taken says nothing: {answers:?}"
    );

    let state = engine.state();
    assert_eq!(state.turn(), 4, "three turns were played");
    assert_eq!(state.to_move(), pistol_core::Player::P2);
    assert_eq!(state.phase(), pistol_core::Phase::First);
    assert_eq!(
        state.board().stone_count(),
        5,
        "one stone, then two, then two"
    );
    for (cell, player) in [
        ("0,0", pistol_core::Player::P1),
        ("1,0", pistol_core::Player::P2),
        ("2,0", pistol_core::Player::P2),
        ("-1,1", pistol_core::Player::P1),
        ("0,1", pistol_core::Player::P1),
    ] {
        let at = cell.parse::<pistol_core::Coord>().expect("a stone token");
        assert_eq!(state.board().get(at), Some(player), "{cell} holds {player}");
    }
}

#[test]
fn protocol_set_position_with_phase_roundtrip() {
    // The `set` form exists to say what a move list cannot: a position in the
    // middle of a turn. The mover's already-placed stone is the last one listed
    // for that side (docs/decisions.md D-6).
    let tail = "set p1:0,0 1,0 p2:0,1 1,1 tomove:p1 phase:1";
    let spec: PositionSpec = tail.parse().expect("a well-formed stone list");
    assert_eq!(spec.to_string(), tail);

    let mut engine = engine(GATE);
    let answers = talk(&mut engine, &[&format!("position {tail}")]);
    assert!(
        answers.is_empty(),
        "a position that was taken says nothing: {answers:?}"
    );

    let state = engine.state();
    assert_eq!(state.to_move(), pistol_core::Player::P1);
    assert_eq!(
        state.phase(),
        pistol_core::Phase::Second,
        "phase:1 means one stone of this turn is already down"
    );
    assert_eq!(state.turn(), 3);
    assert_eq!(state.stones_owed(), 1, "the mover owes the second stone");
    assert_eq!(
        state.board().get("1,0".parse().unwrap()),
        Some(pistol_core::Player::P1),
        "p1's last listed stone is the one already placed this turn"
    );
}

#[test]
fn protocol_set_position_accepts_a_side_with_no_stones() {
    // After turn 1 P2 holds nothing, and the form has to be able to say so.
    let tail = "set p1:0,0 p2: tomove:p2 phase:0";
    let spec: PositionSpec = tail.parse().expect("an empty p2 list is a list");
    assert_eq!(spec.to_string(), tail);

    let mut engine = engine(GATE);
    assert!(talk(&mut engine, &[&format!("position {tail}")]).is_empty());
    assert_eq!(engine.state().board().stone_count(), 1);
    assert_eq!(engine.state().to_move(), pistol_core::Player::P2);
}

#[test]
fn protocol_rejects_illegal_move_with_named_error() {
    let mut engine = engine(GATE);

    // Rule 3: the first stone of the game is the origin.
    let answers = talk(&mut engine, &["position start moves 3,4"]);
    assert_eq!(refusal(&answers), "IllegalMove");
    assert!(
        only_line(&answers, "error").contains("0,0"),
        "the refusal says where the first stone goes: {answers:?}"
    );

    // Rule 5: a stone must be within hex-distance 8 of an existing stone.
    let answers = talk(&mut engine, &["position start moves 0,0 40,0/41,0"]);
    assert_eq!(refusal(&answers), "IllegalMove");
    assert!(only_line(&answers, "error").contains("turn 2"));

    // A cell cannot hold two stones.
    let answers = talk(&mut engine, &["position start moves 0,0 0,0/1,0"]);
    assert_eq!(refusal(&answers), "IllegalMove");

    // Turn 1 places one stone (rule 3), so a pair there is not a turn.
    let answers = talk(&mut engine, &["position start moves 0,0/1,0"]);
    assert_eq!(refusal(&answers), "IllegalMove");

    // The engine is still alive, and still where it was.
    assert!(talk(&mut engine, &["position start moves 0,0"]).is_empty());
    assert_eq!(engine.state().board().stone_count(), 1);
}

#[test]
fn protocol_rejects_a_stone_list_that_is_not_a_position() {
    let mut engine = engine(GATE);

    // A stone list is a claim about a whole position, so its refusals are
    // IllegalPosition and not IllegalMove: the operator named no turn.
    for tail in [
        // Stone counts that fit no turn structure (rule 3).
        "set p1:0,0 1,0 p2:0,1 tomove:p1 phase:0",
        // A stone out of reach when it was played (rule 5).
        "set p1:0,0 40,0 p2:0,1 1,1 tomove:p1 phase:0",
        // The stones say P2 is to move; the document says P1.
        "set p1:0,0 1,0 2,0 p2:0,1 1,1 0,2 tomove:p1 phase:0",
        // A first stone that is not the origin (rule 3).
        "set p1:5,5 p2: tomove:p2 phase:0",
    ] {
        let answers = talk(&mut engine, &[&format!("position {tail}")]);
        assert_eq!(
            refusal(&answers),
            "IllegalPosition",
            "`{tail}` is not a position: {answers:?}"
        );
    }
}

#[test]
fn protocol_rejects_already_won_set_position() {
    // A won position is terminal: there is no move to ask for, so the engine
    // refuses to stand on one at all (rule 4).
    let mut engine = engine(GATE);
    // P1 completed six with the first stone of turn 7, so the second stone of
    // that turn was never played (rule 4) and the stone counts are 6 and 6.
    let tail = "set p1:0,0 1,0 2,0 3,0 4,0 5,0 p2:0,3 1,3 2,3 0,5 1,5 2,5 tomove:p1 phase:0";
    let answers = talk(&mut engine, &[&format!("position {tail}")]);
    assert_eq!(refusal(&answers), "IllegalPosition");
    let line = only_line(&answers, "error");
    assert!(
        line.contains("p1") && line.contains("terminal"),
        "the refusal says who won and why that ends it: {line}"
    );
}

#[test]
fn protocol_rejects_move_list_ending_in_a_win() {
    // The same rule through the other form: the last turn of a finished game
    // leaves a position nobody can be asked for a move in.
    let mut engine = engine(GATE);
    let answers = talk(
        &mut engine,
        &["position start moves 0,0 0,3/1,3 1,0/2,0 0,5/2,3 3,0/4,0 1,5/2,5 5,0"],
    );
    assert_eq!(refusal(&answers), "IllegalPosition");
}

#[test]
fn protocol_rejects_a_move_after_the_game_was_decided() {
    // A turn played into an already-decided game is a rejected MOVE, and the
    // refusal names the turn it belongs to.
    let mut engine = engine(GATE);
    let answers = talk(
        &mut engine,
        &["position start moves 0,0 0,3/1,3 1,0/2,0 0,5/2,3 3,0/4,0 1,5/2,5 5,0 6,3/7,3"],
    );
    assert_eq!(refusal(&answers), "IllegalMove");
    let line = only_line(&answers, "error");
    assert!(
        line.contains("already won") && line.contains("6,3/7,3"),
        "the refusal names the turn that was refused and why: {line}"
    );
}
