use crate::coord::Coord;
use crate::error::{
    CoreError, EITHER_STONE_ALREADY_WINS, PAIR_NOT_CANONICAL, PAIR_OF_ONE_CELL,
    PAIR_ON_THE_FIRST_TURN, SINGLE_THAT_DOES_NOT_WIN,
};
use crate::rules::TURN_STONES;
use crate::state::{GameState, HISTORY_DESYNC};
use crate::turn::{Outcome, Phase, PlyOutcome, Turn, canonical_pair};

/// Named invariant: a turn that owes two stones ended on its first without
/// completing a line.
pub const TURN_STRUCTURE_DESYNC: &str = "TURN_STRUCTURE_DESYNC";

/// Why one ordering of a pair did not play it.
enum Refusal {
    /// A cell was refused, by name: occupied, or outside the legal region.
    Cell(CoreError),
    /// The first stone completed a line, so this ordering plays a turn of one
    /// stone rather than this pair (rule 4).
    Truncates,
}

impl GameState {
    /// Play a whole turn.
    ///
    /// Refuses, by name: a turn played into a decided game or into a position
    /// in the middle of one ([`CoreError::GameDecided`],
    /// [`CoreError::TurnInProgress`]); a pair that is malformed, that does not
    /// fit the stone count this turn owes, or whose every ordering is cut short
    /// by rule 4 ([`CoreError::IllegalTurn`]); and a cell that is occupied or
    /// outside the legal region ([`CoreError::OccupiedCell`],
    /// [`CoreError::OutsideLegalRegion`], [`CoreError::FirstStoneNotAtOrigin`]).
    pub fn make_turn(&mut self, turn: Turn) -> Result<Outcome, CoreError> {
        if let Outcome::Win { winner, turn } = self.outcome() {
            return Err(CoreError::GameDecided { winner, turn });
        }
        if self.phase() != Phase::First {
            return Err(CoreError::TurnInProgress { turn: self.turn() });
        }
        match turn {
            Turn::Single(at) => self.make_single(turn, at),
            Turn::Pair(first, second) => self.make_pair(turn, first, second),
        }
    }

    /// Take back the whole of the last turn: both its stones, or the one stone
    /// a turn of one left behind.
    ///
    /// This is the exact inverse of [`GameState::make_turn`] — the position,
    /// the side to move, the turn number and the outcome are the ones the turn
    /// was played from — and it returns the turn it took back, canonically
    /// spelled. The only refusals are [`CoreError::NothingToUndo`] and
    /// [`CoreError::TurnInProgress`], the latter because a position that owes
    /// one stone is not at a turn boundary and taking back "the last turn"
    /// there would mean unplaying a stone the mover has not finished playing.
    ///
    /// # Panics
    ///
    /// With [`HISTORY_DESYNC`] if the history holds a second stone with no
    /// first stone before it.
    pub fn unmake_turn(&mut self) -> Result<Turn, CoreError> {
        if self.phase() != Phase::First {
            return Err(CoreError::TurnInProgress { turn: self.turn() });
        }
        let last = self.undo()?;
        // The phase now says what that stone was: a turn's first stone leaves a
        // turn boundary behind it, a second stone leaves the turn half played.
        if self.phase() == Phase::First {
            return Ok(Turn::Single(last));
        }
        let first = self.undo().unwrap_or_else(|error| {
            panic!(
                "pistol-core invariant {HISTORY_DESYNC}: {last} was the second stone of turn {}, \
                 and the history has no first stone before it: {error}",
                self.turn()
            )
        });
        assert!(
            self.phase() == Phase::First,
            "pistol-core invariant {HISTORY_DESYNC}: taking back both stones of turn {} did not \
             land on a turn boundary",
            self.turn()
        );
        Ok(canonical_pair(first, last))
    }

    /// Play a turn of one stone: turn 1's stone, or a stone that wins (rules 3
    /// and 4).
    fn make_single(&mut self, turn: Turn, at: Coord) -> Result<Outcome, CoreError> {
        match self.place(at)? {
            PlyOutcome::Win { .. } | PlyOutcome::TurnComplete => Ok(self.outcome()),
            PlyOutcome::TurnContinues => {
                // The turn owes a second stone and this one did not end it.
                self.take_back(at);
                Err(CoreError::IllegalTurn {
                    turn,
                    why: SINGLE_THAT_DOES_NOT_WIN,
                })
            }
        }
    }

    /// Play a turn of two stones, in whichever order the rules allow.
    fn make_pair(&mut self, turn: Turn, first: Coord, second: Coord) -> Result<Outcome, CoreError> {
        if first == second {
            return Err(CoreError::IllegalTurn {
                turn,
                why: PAIR_OF_ONE_CELL,
            });
        }
        if second < first {
            return Err(CoreError::IllegalTurn {
                turn,
                why: PAIR_NOT_CANONICAL,
            });
        }
        if self.stones_owed() < TURN_STONES {
            return Err(CoreError::IllegalTurn {
                turn,
                why: PAIR_ON_THE_FIRST_TURN,
            });
        }
        let canonical = match self.try_ordering(first, second) {
            Ok(outcome) => return Ok(outcome),
            Err(refusal) => refusal,
        };
        let reversed = match self.try_ordering(second, first) {
            Ok(outcome) => return Ok(outcome),
            Err(refusal) => refusal,
        };
        // Neither ordering plays this turn. A cell that was refused by name is
        // the more useful answer, and the canonical ordering's is preferred so
        // that the same turn is always refused the same way.
        Err(match (canonical, reversed) {
            (Refusal::Cell(error), _) | (Refusal::Truncates, Refusal::Cell(error)) => error,
            (Refusal::Truncates, Refusal::Truncates) => CoreError::IllegalTurn {
                turn,
                why: EITHER_STONE_ALREADY_WINS,
            },
        })
    }

    /// Try to play the pair in this order, leaving the position untouched if it
    /// does not play.
    fn try_ordering(&mut self, first: Coord, second: Coord) -> Result<Outcome, Refusal> {
        match self.place(first) {
            Err(error) => Err(Refusal::Cell(error)),
            Ok(PlyOutcome::Win { .. }) => {
                // Rule 4 ends the turn here, so this ordering plays a turn of
                // one stone, which is a different turn from the pair asked for.
                self.take_back(first);
                Err(Refusal::Truncates)
            }
            Ok(PlyOutcome::TurnComplete) => panic!(
                "pistol-core invariant {TURN_STRUCTURE_DESYNC}: turn {} owed two stones and the \
                 stone on {first} completed it without completing a line",
                self.turn()
            ),
            Ok(PlyOutcome::TurnContinues) => match self.place(second) {
                Ok(_) => Ok(self.outcome()),
                Err(error) => {
                    self.take_back(first);
                    Err(Refusal::Cell(error))
                }
            },
        }
    }

    /// Take back a stone this call has just placed.
    ///
    /// # Panics
    ///
    /// With [`HISTORY_DESYNC`] if it is not there to take back, which would
    /// mean the history and the board disagree about a stone placed a moment
    /// ago.
    fn take_back(&mut self, at: Coord) {
        self.undo().unwrap_or_else(|error| {
            panic!(
                "pistol-core invariant {HISTORY_DESYNC}: the stone just placed on {at} cannot be \
                 taken back: {error}"
            )
        });
    }
}
