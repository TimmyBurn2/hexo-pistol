//! A stated position: its token grammar, and the replay that turns it into a
//! game.
//!
//! Two forms, one door (docs/decisions.md D-6, D-42): whatever a caller states is
//! replayed through the rules in pistol-core, so every refusal here is a rules
//! refusal wearing the engine's name for it.

use pistol_core::{Color, Coord, Phase, Turn};
use pistol_engine::{EngineError, PositionSpec};

/// The tail as the engine spells it back.
fn spelled(tail: &str) -> String {
    tail.parse::<PositionSpec>()
        .unwrap_or_else(|error| panic!("`{tail}` should parse: {error}"))
        .to_string()
}

/// The position a tail describes, or a panic.
fn replayed(tail: &str) -> pistol_core::GameState {
    tail.parse::<PositionSpec>()
        .unwrap_or_else(|error| panic!("`{tail}` should parse: {error}"))
        .replay()
        .unwrap_or_else(|error| panic!("`{tail}` should replay: {error}"))
}

/// The refusal a tail earns.
fn refused(tail: &str) -> EngineError {
    tail.parse::<PositionSpec>()
        .unwrap_or_else(|error| panic!("`{tail}` should parse: {error}"))
        .replay()
        .err()
        .unwrap_or_else(|| panic!("`{tail}` should have been refused"))
}

fn cell(token: &str) -> Coord {
    token.parse().expect("a stone token")
}

#[test]
fn position_tokens_round_trip_for_every_form() {
    for tail in [
        "start",
        "start moves 0,0",
        "start moves 0,0 1,0/2,0",
        "set b: w: tomove:b phase:0",
        "set b:0,0 w: tomove:w phase:0",
        "set b:0,0 1,0 w:0,1 1,1 tomove:b phase:1",
        "set b:0,0 -1,0 w:0,-1 1,-1 tomove:b phase:1",
    ] {
        assert_eq!(spelled(tail), tail, "one position, one spelling");
    }
}

#[test]
fn position_tokens_refuse_what_they_could_have_repaired() {
    // The parser is strict for the same reason the stone and turn tokens are
    // (docs/decisions.md D-46, D-56): two lines that differ must not mean the
    // same position.
    for tail in [
        "",                                      // no form
        "sideways b:0,0",                        // no such form
        "start extra",                           // the move list needs its keyword
        "start moves",                           // and something after it
        "start moves 1,0/0,0",                   // an uncanonical pair token
        "start moves 0,0/0,0",                   // a pair of one cell
        "start moves 0,0/1,0/2,0",               // one separator
        "set",                                   // every section is required
        "set b:0,0 w: tomove:b",                 // including the phase
        "set b:0,0 w: phase:0 tomove:b",         // in order
        "set w: b:0,0 tomove:b phase:0",         // likewise
        "set b:0,0 b:1,0 w: tomove:b phase:0",   // once each
        "set b: 0,0 w: tomove:b phase:0",        // a stone follows its prefix
        "set b:0,0 w: tomove: b phase:0",        // a value is attached
        "set b:0,0 w: tomove:black phase:0",     // `b` or `w`
        "set b:0,0 w: tomove:b phase:2",         // `0` or `1`
        "set b:0,0 w: tomove:b phase:0 extra:1", // no fifth section
        "set b:007,0 w: tomove:b phase:0",       // canonical stone tokens only
    ] {
        assert!(
            tail.parse::<PositionSpec>().is_err(),
            "`{tail}` should not parse"
        );
    }
}

#[test]
fn a_move_list_replays_into_the_position_it_describes() {
    let state = replayed("start moves 0,0 1,0/2,0 -1,1/0,1");
    assert_eq!(state.turn(), 4);
    assert_eq!(state.to_move(), Color::White);
    assert_eq!(state.phase(), Phase::First);
    assert_eq!(state.board().stone_count(), 5);
    assert_eq!(state.board().get(cell("2,0")), Some(Color::White));
    assert_eq!(state.board().get(cell("0,1")), Some(Color::Black));
}

#[test]
fn an_empty_stone_list_is_the_initial_position() {
    let state = replayed("set b: w: tomove:b phase:0");
    assert!(state.board().is_empty());
    assert_eq!(state.turn(), 1);
    assert_eq!(state.stones_owed(), 1, "turn 1 places one stone (rule 3)");
    assert_eq!(replayed("start"), state, "the two forms agree about it");
}

#[test]
fn a_stone_list_carries_a_turn_in_progress() {
    // What the move list cannot say. The mover's already-placed stone is the last
    // one listed for that side (docs/decisions.md D-6), which falls out of the
    // list being in play order rather than being a rule of its own.
    let state = replayed("set b:0,0 1,0 w:0,1 1,1 tomove:b phase:1");
    assert_eq!(state.phase(), Phase::Second);
    assert_eq!(state.to_move(), Color::Black);
    assert_eq!(state.stones_owed(), 1);
    assert_eq!(state.board().get(cell("1,0")), Some(Color::Black));
}

#[test]
fn set_position_accepts_a_pair_whose_only_legal_order_is_reversed() {
    // A pair is legal iff SOME ordering of its two placements is (D-6), and D-52
    // constructs exactly this: the far cell is reachable only through the ball the
    // near one opens. The near cell here is the LARGER of the two in the canonical
    // order, so the canonical ordering is refused and the reverse is what plays —
    // which is `make_turn`'s rule (D-51) and not something this form re-decides.
    let state = replayed("set b:0,0 w:-16,0 -8,0 tomove:b phase:0");
    assert_eq!(state.board().get(cell("-16,0")), Some(Color::White));
    assert_eq!(state.board().get(cell("-8,0")), Some(Color::White));

    // The stone list is never sorted or canonicalized, so the other order of the
    // same two cells is the same position.
    let other = replayed("set b:0,0 w:-8,0 -16,0 tomove:b phase:0");
    assert_eq!(state.key(), other.key(), "the same position either way");

    // And the move list says it with one canonical turn token.
    let moves = replayed("start moves 0,0 -16,0/-8,0");
    assert_eq!(moves.key(), state.key());
    assert_eq!(
        moves.played().collect::<Vec<(Coord, Color)>>(),
        state.played().collect::<Vec<(Coord, Color)>>(),
        "one turn leaves one ply history, whichever form stated it (D-51)"
    );
}

#[test]
fn a_stone_list_that_fits_no_turn_structure_is_refused() {
    // Rule 3 fixes the counts: one stone on turn 1, two on every turn after. The
    // refusal names both counts and what was left over.
    for tail in [
        "set b:0,0 1,0 w:0,1 tomove:b phase:0",
        "set b:0,0 w:0,1 1,1 2,1 tomove:b phase:0",
        "set b: w:0,0 tomove:w phase:0",
    ] {
        match refused(tail) {
            EngineError::IllegalPosition { why } => {
                assert!(
                    why.contains("turn structure") || why.contains("left over"),
                    "`{tail}`: {why}"
                );
            }
            other => panic!("`{tail}`: expected IllegalPosition, got {other}"),
        }
    }
}

#[test]
fn a_stone_list_that_contradicts_its_own_header_is_refused() {
    // `tomove` and `phase` are checked, never trusted: the stones alone fix them,
    // and a document that says otherwise is refused rather than repaired.
    for tail in [
        "set b:0,0 1,0 2,0 w:0,1 1,1 0,2 tomove:b phase:0",
        "set b:0,0 w: tomove:b phase:0",
        "set b:0,0 1,0 w:0,1 1,1 tomove:b phase:0",
        "set b:0,0 1,0 w:0,1 1,1 tomove:w phase:1",
    ] {
        match refused(tail) {
            EngineError::IllegalPosition { why } => {
                assert!(
                    why.contains("to move at phase") || why.contains("left over"),
                    "`{tail}`: {why}"
                );
            }
            other => panic!("`{tail}`: expected IllegalPosition, got {other}"),
        }
    }
}

#[test]
fn a_stone_the_rules_refuse_names_the_ply_it_was_played_on() {
    // Rule 5: a stone goes within hex-distance 8 of an existing stone. On the
    // stone-list path the operator named no turn, so the refusal is about the
    // position and names the ply instead of inventing a turn index.
    match refused("set b:0,0 40,0 w:0,1 1,1 tomove:b phase:0") {
        EngineError::IllegalPosition { why } => {
            assert!(why.contains("stone 4"), "{why}");
            assert!(why.contains("play order"), "{why}");
            assert!(why.contains("40,0"), "{why}");
        }
        other => panic!("expected IllegalPosition, got {other}"),
    }
    // Rule 3: the first stone is the origin, and that is not rule 5's refusal.
    match refused("set b:5,5 w: tomove:w phase:0") {
        EngineError::IllegalPosition { why } => assert!(why.contains("0,0"), "{why}"),
        other => panic!("expected IllegalPosition, got {other}"),
    }
}

#[test]
fn a_rejected_turn_in_a_move_list_names_the_turn_it_belongs_to() {
    let spec = PositionSpec::Start {
        moves: vec![
            Turn::Single(Coord::ORIGIN),
            Turn::pair(cell("40,0"), cell("41,0")).expect("two distinct cells"),
        ],
    };
    match spec.replay().expect_err("out of reach") {
        EngineError::IllegalMove { turn, why } => {
            assert_eq!(turn, 2, "the second turn is the one that was refused");
            assert!(why.contains("40,0/41,0"), "{why}");
        }
        other => panic!("expected IllegalMove, got {other}"),
    }
}

#[test]
fn a_decided_position_is_refused_by_both_forms() {
    // A won position is terminal, so the engine will not stand on one at all: it
    // has no move to be asked for (rule 4). `from_plies` alone would accept it —
    // it refuses a stone AFTER the win, not the winning stone — so this is an
    // explicit check after the replay rather than a consequence of it.
    let won_by_stones = "set b:0,0 1,0 2,0 3,0 4,0 5,0 w:0,3 1,3 2,3 0,5 1,5 2,5 tomove:b phase:0";
    let won_by_moves = "start moves 0,0 0,3/1,3 1,0/2,0 0,5/2,3 3,0/4,0 1,5/2,5 5,0";
    for tail in [won_by_stones, won_by_moves] {
        match refused(tail) {
            EngineError::IllegalPosition { why } => {
                assert!(why.contains("black") && why.contains("turn 7"), "{why}");
            }
            other => panic!("`{tail}`: expected IllegalPosition, got {other}"),
        }
    }
}

#[test]
fn a_win_by_the_second_stone_of_a_turn_is_refused_too() {
    // Black holds four in a row with one end blocked, and the last turn's two
    // stones complete six: the first does not win and the second does. Rule 4
    // leaves that turn an ordinary pair rather than a truncated one, and the
    // position it reaches is just as decided.
    let tail = "start moves 0,0 -1,0/0,3 1,0/2,0 1,3/2,3 3,-2/3,0 0,5/1,5 4,0/5,0";
    match refused(tail) {
        EngineError::IllegalPosition { why } => assert!(why.contains("turn 7"), "{why}"),
        other => panic!("expected IllegalPosition, got {other}"),
    }
    // A pair whose two cells EACH complete a line on their own is a different
    // refusal, and the rules layer names it: rule 4 ends the turn on whichever
    // stone goes down first, so neither ordering plays that pair at all.
    let both_win = "start moves 0,0 0,3/1,3 1,0/2,0 0,5/2,3 3,0/4,0 1,5/2,5 -1,0/5,0";
    match refused(both_win) {
        EngineError::IllegalMove { turn, why } => {
            assert_eq!(turn, 7);
            assert!(why.contains("completes a line on its own"), "{why}");
        }
        other => panic!("expected IllegalMove, got {other}"),
    }
}
