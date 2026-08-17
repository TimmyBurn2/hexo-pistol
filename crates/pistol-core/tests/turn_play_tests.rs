//! Making and taking back a whole turn: the transition the search will run a
//! million times a second, and every way it refuses.
//!
//! Two properties matter and are pinned here. A turn that is made and taken
//! back leaves *nothing* behind — not a stone, not a phase, not an outcome —
//! because a search that drifts by one bit per node is a search whose deep
//! results are fiction. And a turn that is refused leaves nothing behind
//! either: every refusal is atomic, so a caller that handles the error and
//! plays on is not standing on half a turn (CLAUDE.md rule 3).

mod common;

use common::perft_positions::perft_case;
use pistol_core::error::{
    EITHER_STONE_ALREADY_WINS, PAIR_NOT_CANONICAL, PAIR_OF_ONE_CELL, PAIR_ON_THE_FIRST_TURN,
    SINGLE_THAT_DOES_NOT_WIN,
};
use pistol_core::{Coord, CoreError, GameState, Outcome, Player, Turn, generate_turns};

/// How many of a position's turns the roundtrip walks. They are spread evenly
/// over the generated order, so the sample covers the rim of the region, the
/// cells beside the stones, and the turns rule 4 truncates alike.
const ROUNDTRIP_SAMPLE: usize = 40;

/// The position of a fixture case, replayed through the state machine.
fn fixture_position(name: &str) -> GameState {
    let case = perft_case(name);
    GameState::from_plies(&case.plies).expect("a legal fixture game")
}

/// A cell, from the token the fixtures and the protocol both use.
fn cell(token: &str) -> Coord {
    token.parse().expect("a stone token")
}

#[test]
fn make_unmake_roundtrips_board_state() {
    let positions = [
        GameState::new_game(),
        fixture_position("one_stone_at_the_origin"),
        fixture_position("tight_cluster_at_a_turn_boundary"),
        fixture_position("two_lobes_joined_by_a_bridge"),
        fixture_position("a_win_the_mover_can_take"),
    ];
    for state in positions {
        let turns = generate_turns(&state).expect("a turn boundary");
        let stride = turns.len().div_ceil(ROUNDTRIP_SAMPLE).max(1);
        for turn in turns.iter().copied().step_by(stride) {
            roundtrip(&state, turn);
        }
    }

    // The two turns rule 4 makes special, named rather than sampled: the
    // truncated winning stone, and the pair whose second stone wins.
    let state = fixture_position("a_win_the_mover_can_take");
    roundtrip(&state, Turn::Single(cell("5,0")));
    roundtrip(
        &state,
        Turn::pair(cell("0,-1"), cell("5,0")).expect("two distinct cells"),
    );
}

/// Make a turn, check it did what it says, take it back, check nothing is left.
fn roundtrip(state: &GameState, turn: Turn) {
    let mut position = state.clone();
    let outcome = position
        .make_turn(turn)
        .unwrap_or_else(|error| panic!("{turn}: {error}"));

    let mover = state.to_move();
    let expected_stones = state.board().stone_count() + turn.stone_count() as usize;
    assert_eq!(position.board().stone_count(), expected_stones, "{turn}");
    assert_eq!(position.board().get(turn.first()), Some(mover), "{turn}");
    if let Some(second) = turn.second() {
        assert_eq!(position.board().get(second), Some(mover), "{turn}");
    }
    assert_eq!(position.outcome(), outcome, "{turn}: the reported outcome");
    match outcome {
        Outcome::Ongoing => {
            assert_eq!(position.to_move(), mover.opponent(), "{turn}");
            assert_eq!(position.turn(), state.turn() + 1, "{turn}");
        }
        Outcome::Win { winner, turn: on } => {
            assert_eq!(winner, mover, "{turn}");
            assert_eq!(on, state.turn(), "{turn}: sudden death is scored in turns");
            assert_eq!(position.stones_owed(), 0, "{turn}");
        }
    }

    let unmade = position
        .unmake_turn()
        .unwrap_or_else(|error| panic!("{turn}: {error}"));
    assert_eq!(unmade, turn, "unmake reported a different turn");
    assert_eq!(&position, state, "{turn}: the position did not come back");
}

#[test]
fn make_turn_refuses_a_turn_that_does_not_fit_the_position() {
    let quiet = cell("1,0");

    // Rule 3: turn 1 is one stone, and no pair is a turn there.
    let new_game = GameState::new_game();
    let pair = Turn::pair(Coord::ORIGIN, quiet).expect("two distinct cells");
    refused(
        &new_game,
        pair,
        CoreError::IllegalTurn {
            turn: pair,
            why: PAIR_ON_THE_FIRST_TURN,
        },
    );

    // Rule 3 the other way round: a turn that owes two stones is not finished
    // by one, unless rule 4 finishes it.
    let opened = fixture_position("one_stone_at_the_origin");
    refused(
        &opened,
        Turn::Single(quiet),
        CoreError::IllegalTurn {
            turn: Turn::Single(quiet),
            why: SINGLE_THAT_DOES_NOT_WIN,
        },
    );

    // A pair is two distinct cells, written smaller first. Both malformed
    // spellings are refused rather than repaired: a turn has one spelling
    // (docs/decisions.md D-5).
    let doubled = Turn::Pair(quiet, quiet);
    refused(
        &opened,
        doubled,
        CoreError::IllegalTurn {
            turn: doubled,
            why: PAIR_OF_ONE_CELL,
        },
    );
    assert_eq!(
        Turn::pair(quiet, quiet),
        Err(CoreError::IllegalTurn {
            turn: doubled,
            why: PAIR_OF_ONE_CELL
        }),
        "the constructor refuses it too"
    );
    let backwards = Turn::Pair(cell("2,0"), quiet);
    refused(
        &opened,
        backwards,
        CoreError::IllegalTurn {
            turn: backwards,
            why: PAIR_NOT_CANONICAL,
        },
    );

    // Rule 4: two cells that each complete a line make two turns of one stone,
    // and no pair — whichever went down first would end the turn.
    let winning = fixture_position("a_win_the_mover_can_take");
    let both_win = Turn::pair(cell("-1,0"), cell("5,0")).expect("two distinct cells");
    refused(
        &winning,
        both_win,
        CoreError::IllegalTurn {
            turn: both_win,
            why: EITHER_STONE_ALREADY_WINS,
        },
    );
}

#[test]
fn make_turn_refuses_a_cell_by_name() {
    let state = fixture_position("tight_cluster_at_a_turn_boundary");
    let taken = state
        .board()
        .stones()
        .next()
        .expect("a stone on the board")
        .0;
    let empty = cell("3,3");
    assert!(state.board().is_legal_placement(empty));

    // Rule 5's two cell-level refusals reach through a turn unchanged, naming
    // the cell rather than the turn.
    refused(
        &state,
        Turn::pair(taken, empty).expect("two distinct cells"),
        CoreError::OccupiedCell { at: taken },
    );
    let far = cell("40,0");
    refused(
        &state,
        Turn::Single(far),
        CoreError::OutsideLegalRegion { at: far },
    );
    refused(
        &state,
        Turn::pair(empty, far).expect("two distinct cells"),
        CoreError::OutsideLegalRegion { at: far },
    );

    // And a decided game takes no turn at all (rule 4).
    let decided = fixture_position("already_won_is_not_expanded");
    let Outcome::Win { winner, turn } = decided.outcome() else {
        panic!("the fixture case is a won game");
    };
    assert_eq!(winner, Player::P1);
    refused(
        &decided,
        Turn::Single(cell("6,0")),
        CoreError::GameDecided { winner, turn },
    );
}

#[test]
fn unmake_turn_refuses_a_game_with_no_turn_behind_it() {
    let mut state = GameState::new_game();
    assert_eq!(state.unmake_turn(), Err(CoreError::NothingToUndo));
    assert_eq!(state, GameState::new_game());

    // One turn back and no further: turn 1 is one stone, so the first turn is a
    // single and taking it back empties the board.
    assert_eq!(
        state.make_turn(Turn::Single(Coord::ORIGIN)),
        Ok(Outcome::Ongoing)
    );
    assert_eq!(state.unmake_turn(), Ok(Turn::Single(Coord::ORIGIN)));
    assert_eq!(state, GameState::new_game());
    assert_eq!(state.unmake_turn(), Err(CoreError::NothingToUndo));
}

/// A refusal names the reason and changes nothing.
fn refused(state: &GameState, turn: Turn, expected: CoreError) {
    let mut position = state.clone();
    assert_eq!(position.make_turn(turn), Err(expected), "{turn}");
    assert_eq!(
        &position, state,
        "{turn}: a refused turn changed the position"
    );
}
