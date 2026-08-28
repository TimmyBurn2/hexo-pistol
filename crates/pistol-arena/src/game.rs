use pistol_core::{Outcome, Turn};

use crate::channel::Channel;
use crate::error::ArenaError;
use crate::exchange::{Answer, ask};
use crate::openings::{Opening, replayed};
use crate::record::{Compute, End, ForfeitReason, GameRecord, GameResult};

/// Everything one game needs that is the same for every game in a run.
pub struct Rules<'a> {
    /// The `go` line, already spelled.
    pub go_line: &'a str,
    /// Turns before the horizon ends the game.
    pub turn_cap: u32,
    /// The liveness watchdog, in milliseconds.
    pub hang_timeout_ms: u64,
}

/// Play one game and return its record.
///
/// `a_is_p1` decides the seating; `channels` are indexed by ENGINE (`0` is A),
/// which is how compute and forfeits are attributed.
pub fn play(
    opening: &Opening,
    a_is_p1: bool,
    index: usize,
    channels: &mut [Channel; 2],
    rules: &Rules<'_>,
) -> Result<GameRecord, ArenaError> {
    let mut state = replayed(opening);
    let mut moves = opening.moves.clone();
    let mut compute = [Compute::default(); 2];

    let finish = |result: GameResult,
                  end: End,
                  by: Option<usize>,
                  refusal: Option<String>,
                  moves: Vec<Turn>,
                  compute: [Compute; 2]| GameRecord {
        index,
        opening: opening.index,
        a_is_p1,
        result,
        end,
        forfeit_by: by,
        refusal,
        moves,
        compute,
    };

    loop {
        if state.turn() > rules.turn_cap {
            return Ok(finish(
                GameResult::Capped,
                End::Normal,
                None,
                None,
                moves,
                compute,
            ));
        }
        let mover_is_p1 = state.to_move() == pistol_core::Player::P1;
        let engine = seat_of(mover_is_p1, a_is_p1);
        let loser = GameResult::loser_of(mover_is_p1);

        let answer = ask(
            &mut channels[engine],
            &moves,
            rules,
            opening.index,
            state.turn(),
            &mut compute[engine],
        )?;
        let turn = match answer {
            Answer::Move(turn) => turn,
            Answer::Forfeit { reason, line } => {
                return Ok(finish(
                    loser,
                    End::Forfeit(reason),
                    Some(engine),
                    line,
                    moves,
                    compute,
                ));
            }
        };

        // pistol-core is the referee and the only judge of legality (rule 2).
        let outcome = match state.make_turn(turn) {
            Ok(outcome) => outcome,
            Err(error) => {
                return Ok(finish(
                    loser,
                    End::Forfeit(ForfeitReason::IllegalTurn),
                    Some(engine),
                    Some(format!("{turn}: {error}")),
                    moves,
                    compute,
                ));
            }
        };
        moves.push(turn);
        if let Outcome::Win { winner, .. } = outcome {
            let result = match winner {
                pistol_core::Player::P1 => GameResult::P1Win,
                pistol_core::Player::P2 => GameResult::P2Win,
            };
            return Ok(finish(result, End::Normal, None, None, moves, compute));
        }
    }
}

/// Which ENGINE holds the seat that moves now: `0` is A, which is how compute
/// and forfeits are attributed.
///
/// Seat 0 is the first player, and engine A holds it when `a_is_p1`. One
/// function rather than two lines spelled twice: the replay path asks the same
/// question of the same state, and a copy that agrees today is not an
/// inheritance (docs/decisions.md D-406's own finding, D-408's answer to it).
pub const fn seat_of(mover_is_p1: bool, a_is_p1: bool) -> usize {
    if mover_is_p1 == a_is_p1 { 0 } else { 1 }
}

impl GameResult {
    /// The result in which the side now to move LOSES. A forfeit's outcome.
    const fn loser_of(mover_is_p1: bool) -> GameResult {
        if mover_is_p1 {
            GameResult::P2Win
        } else {
            GameResult::P1Win
        }
    }
}
