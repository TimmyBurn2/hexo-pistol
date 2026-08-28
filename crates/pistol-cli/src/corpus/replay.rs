use pistol_core::{Coord, CoreError, GameState, Outcome, PlyOutcome, Turn, stones_in_turn};

use super::record::Record;
use super::verdict::{GroupedTurn, Replayed, Verdict};

/// Named invariant: the recorded-order probe was refused in a way that cannot
/// happen for a turn pistol-core has just accepted.
pub const PROBE_REFUSAL_IMPOSSIBLE: &str = "PROBE_REFUSAL_IMPOSSIBLE";

/// Group a flat stone list into turns: one stone, then two, then two, …
///
/// A single stone left at the end is rule 4's truncation — the winning stone was
/// a turn's first, so the second was never played. It is emitted as
/// [`Turn::Single`], which makes pistol-core assert that the stone *wins*: a
/// truncated turn and an abandoned one look identical here and are told apart
/// there (`SINGLE_THAT_DOES_NOT_WIN`).
pub fn group_turns(moves: &[Coord]) -> Result<Vec<GroupedTurn>, Verdict> {
    let mut turns = Vec::new();
    let mut index = 0usize;
    let mut number = pistol_core::FIRST_TURN;
    while index < moves.len() {
        let owed = stones_in_turn(number) as usize;
        let taken = owed.min(moves.len() - index);
        let turn = if taken == 1 {
            Turn::Single(moves[index])
        } else {
            match Turn::pair(moves[index], moves[index + 1]) {
                Ok(turn) => turn,
                Err(error) => {
                    return Err(Verdict::IllegalTurn {
                        turn_number: number,
                        move_index: index,
                        turn: Turn::Pair(moves[index], moves[index + 1]),
                        why: error.to_string(),
                    });
                }
            }
        };
        turns.push(GroupedTurn {
            turn,
            recorded_first: moves[index],
            move_index: index,
        });
        index += taken;
        number += 1;
    }
    Ok(turns)
}

/// Replay one record all the way through, answering both questions in one pass.
pub fn replay(record: &Record) -> Replayed {
    let turns = match group_turns(&record.moves) {
        Ok(turns) => turns,
        Err(verdict) => {
            return Replayed {
                verdict,
                turns: Vec::new(),
                order_rescued: 0,
                stone_after_win: 0,
                decided_on_turn: None,
            };
        }
    };

    let mut state = GameState::new_game();
    let mut order_rescued = 0usize;
    let mut stone_after_win = 0usize;

    for grouped in &turns {
        if let Outcome::Win { turn, .. } = state.outcome() {
            return Replayed {
                verdict: Verdict::PostWinContinuation {
                    move_index: grouped.move_index,
                },
                turns,
                order_rescued,
                stone_after_win,
                decided_on_turn: Some(turn),
            };
        }
        let number = state.turn();
        let probe = probe_recorded_order(&mut state, grouped);
        if let Err(error) = state.make_turn(grouped.turn) {
            return Replayed {
                verdict: Verdict::IllegalTurn {
                    turn_number: number,
                    move_index: grouped.move_index,
                    turn: grouped.turn,
                    why: error.to_string(),
                },
                turns,
                order_rescued,
                stone_after_win,
                decided_on_turn: None,
            };
        }
        match probe {
            Probe::Ordinary => {}
            Probe::OrderRescued => order_rescued += 1,
            Probe::StoneAfterWin => stone_after_win += 1,
            Probe::Impossible(error) => panic!(
                "pistol-cli invariant {PROBE_REFUSAL_IMPOSSIBLE}: turn {number} of game {} played, \
                 but probing its recorded first stone {} was refused with: {error}",
                record.game_hash, grouped.recorded_first,
            ),
        }
    }

    let (verdict, decided_on_turn) = match state.outcome() {
        Outcome::Ongoing => (Verdict::NotDecided, None),
        Outcome::Win { winner, turn } if winner == record.winner => (Verdict::Eligible, Some(turn)),
        Outcome::Win { winner, turn } => (
            Verdict::WinnerMismatch {
                replayed: winner,
                recorded: record.winner,
            },
            Some(turn),
        ),
    };
    Replayed {
        verdict,
        turns,
        order_rescued,
        stone_after_win,
        decided_on_turn,
    }
}

/// What probing the record's own ordering found.
enum Probe {
    /// The recorded order starts the turn the ordinary way.
    Ordinary,
    /// The recorded first stone is outside the legal region.
    OrderRescued,
    /// The recorded first stone completes a line, so the record's second stone
    /// follows a decided game.
    StoneAfterWin,
    /// A refusal that cannot happen once the turn itself plays.
    Impossible(CoreError),
}

/// Place the record's first stone of this turn, classify, and take it back.
///
/// The position is left exactly as it was found: `undo` restores the state the
/// stone was placed from, which is what `pistol_core::play`'s own `take_back`
/// relies on.
fn probe_recorded_order(state: &mut GameState, grouped: &GroupedTurn) -> Probe {
    let outcome = match state.place(grouped.recorded_first) {
        Ok(outcome) => outcome,
        Err(CoreError::OutsideLegalRegion { .. }) => return Probe::OrderRescued,
        Err(error) => return Probe::Impossible(error),
    };
    let classification = match outcome {
        // A single-stone turn has no second stone to follow the win, so a win
        // here is rule 4 working, not a record continuing past one.
        PlyOutcome::Win { .. } if grouped.turn.second().is_some() => Probe::StoneAfterWin,
        _ => Probe::Ordinary,
    };
    state
        .undo()
        .unwrap_or_else(|error| panic!("the probe's own stone must come back: {error}"));
    classification
}

/// The position after the first `turns_wanted` turns, replayed the same way
/// eligibility replayed it.
///
/// # Panics
///
/// If a turn that replayed once does not replay again — the caller has already
/// established that this prefix plays.
pub fn position_after(turns: &[GroupedTurn], turns_wanted: usize) -> GameState {
    let mut state = GameState::new_game();
    for grouped in turns.iter().take(turns_wanted) {
        state.make_turn(grouped.turn).unwrap_or_else(|error| {
            panic!(
                "a prefix that replayed once must replay again: turn {} refused with {error}",
                grouped.turn
            )
        });
    }
    state
}
