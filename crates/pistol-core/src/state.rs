//! The turn structure: whose turn it is, and how far into it we are.
//!
//! A turn is two stones by the same side — except the first, which is one
//! (rule 3) — so a turn is represented as two sequential same-side plies with
//! an intra-turn phase (docs/decisions.md D-9). Everything the outside world
//! counts is in **turns**: the turn number here is the number sudden death is
//! scored in (rule 4), and the search's depth and mate distance are the same
//! unit (D-3).
//!
//! Rule 4 falls out of the transition rather than being special-cased on top of
//! it: a stone that completes a line decides the game, and the state then
//! accepts no further stone, so the second stone of a winning turn is not
//! played — it is unreachable, not skipped.
//!
//! The vocabulary this machine produces — [`Phase`], [`Outcome`],
//! [`PlyOutcome`], and the reasons there is neither a draw nor a stalemate —
//! lives in [`crate::turn`].
//!
//! # Counting
//!
//! The turn counter is `u32` and counts absolute turns from [`FIRST_TURN`]. It
//! is not the `u16` of `MateIn`/`MatedIn` (docs/decisions.md D-3): that one is
//! a *distance* in turns from the root of a search, which is bounded by the
//! search depth, while this one is a position's own history and is bounded only
//! by how long a game gets.

use crate::board::{Board, Player};
use crate::coord::Coord;
use crate::error::CoreError;
use crate::rules::{FIRST_TURN, TURN_STONES, stones_in_turn};
use crate::turn::{Outcome, Phase, PlyOutcome};
use crate::win::wins_at;
use crate::zobrist::{Key128, context_key};

/// Named invariant: the turn counter left the `u32` range.
pub const TURN_OVERFLOW: &str = "TURN_OVERFLOW";

/// Named invariant: the board and the ply history disagree about the last
/// stone played.
pub const HISTORY_DESYNC: &str = "HISTORY_DESYNC";

/// One placement, and the state it was made from.
///
/// Undo restores the recorded state rather than recomputing it: the search
/// applies and takes back one ply at a time (docs/decisions.md D-9, D-38), and
/// recomputing the transition backwards would be a second implementation of
/// the rules living next to the first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ply {
    at: Coord,
    mover: Player,
    phase: Phase,
    turn: u32,
}

/// A game in progress: the stones, whose turn it is, and how far into it.
///
/// Every field is private and the board is handed out by shared reference
/// only. There is deliberately no `board_mut`: the turn, the phase and the
/// outcome are derived from the stones, and a caller that could put a stone on
/// the board directly would desynchronize them silently, which is the failure
/// mode CLAUDE.md rule 3 exists to prevent. So this must not compile:
///
/// ```compile_fail
/// use pistol_core::{Board, Coord, GameState};
/// let mut state = GameState::new_game();
/// let board: &mut Board = state.board();
/// board.apply(Coord::ORIGIN, pistol_core::Player::P1).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    board: Board,
    to_move: Player,
    phase: Phase,
    turn: u32,
    outcome: Outcome,
    history: Vec<Ply>,
}

impl GameState {
    /// A new game: an empty board, P1 to move, turn one, one stone owed.
    ///
    /// Named for the protocol verb it serves (docs/decisions.md D-2, D-5).
    pub fn new_game() -> Self {
        GameState {
            board: Board::empty(),
            to_move: Player::P1,
            phase: Phase::First,
            turn: FIRST_TURN,
            outcome: Outcome::Ongoing,
            history: Vec::new(),
        }
    }

    /// Replay a game from its move list, in play order.
    ///
    /// The move list is the canonical encoding of a position
    /// (docs/decisions.md D-6), and this is the only way to reach a position
    /// other than by playing to it. Replaying through [`GameState::place`] is
    /// the point: legality and the turn and phase structure fall out of the one
    /// implementation of the rules rather than being checked a second time by
    /// whoever loads a fixture.
    ///
    /// A stone played *after* the game was decided is refused by name. The stone
    /// that decides it is not: [`GameState::place`] tests the outcome on entry, so
    /// the winning ply returns `Ok` and this returns a **decided** state. D-6's
    /// already-won position is therefore not refused here, and a caller that must
    /// not stand on one asks [`GameState::outcome`] — which is what the engine
    /// does, in both directions of its `position` verb (docs/decisions.md D-84).
    pub fn from_plies(plies: &[Coord]) -> Result<GameState, CoreError> {
        let mut state = GameState::new_game();
        for &at in plies {
            state.place(at)?;
        }
        Ok(state)
    }

    /// The stones. Shared reference only, by design — see the type's
    /// documentation.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// The side that places the next stone.
    ///
    /// Only meaningful while the game is ongoing; after a win the state freezes
    /// on the completing stone, and this reads as the winner.
    pub fn to_move(&self) -> Player {
        self.to_move
    }

    /// How far into the current turn the mover is.
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// The current turn, counting from [`FIRST_TURN`]. After a win, the turn
    /// the win completed on — the number sudden death is scored in.
    pub fn turn(&self) -> u32 {
        self.turn
    }

    /// Whether the game is still going, and who won if it is not.
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    /// The position's zobrist key: its stones, whose move it is, and how far
    /// into the turn they are (docs/decisions.md D-8).
    ///
    /// The stones' half is carried by the board and costs nothing to read; the
    /// side and the phase are XORed in here, from the machine's own fields, so
    /// there is no fourth copy of them to drift (D-58).
    ///
    /// The turn *number* is deliberately absent, and nothing is lost: for an
    /// ongoing game the stone count fixes the turn, the phase and the mover
    /// together, so two positions this key cannot tell apart are the same
    /// position. The outcome is absent for the same reason — a completed run is
    /// a property of the stones.
    pub fn key(&self) -> Key128 {
        self.board.stones_key() ^ context_key(self.to_move, self.phase)
    }

    /// How many stones this turn still owes: two on any turn but the first,
    /// less the one already placed if the mover is at [`Phase::Second`], and
    /// none at all once the game is decided (rule 4).
    pub fn stones_owed(&self) -> u32 {
        if self.outcome.is_decided() {
            return 0;
        }
        stones_in_turn(self.turn) - self.phase.index()
    }

    /// Every stone in the order it was played. This move list is the canonical
    /// encoding of a position (docs/decisions.md D-6).
    pub fn played(&self) -> impl Iterator<Item = (Coord, Player)> + '_ {
        self.history.iter().map(|ply| (ply.at, ply.mover))
    }

    /// Place the next stone of this turn.
    ///
    /// Refuses, by name: a stone after the game is decided, a stone on an
    /// occupied cell, and a stone outside the legal region — which on an empty
    /// board is the origin alone (rules 3, 4, 5).
    pub fn place(&mut self, at: Coord) -> Result<PlyOutcome, CoreError> {
        if let Outcome::Win { winner, turn } = self.outcome {
            return Err(CoreError::GameDecided { winner, turn });
        }
        self.board.check_placement(at)?;

        let mover = self.to_move;
        self.board.apply(at, mover)?;
        self.history.push(Ply {
            at,
            mover,
            phase: self.phase,
            turn: self.turn,
        });

        if wins_at(&self.board, at) {
            // Rule 4: the turn ends on this stone. The state freezes here — the
            // turn number is the one the win is scored on, and nothing may be
            // placed from a decided game, so the second stone has no way to be
            // played.
            self.outcome = Outcome::Win {
                winner: mover,
                turn: self.turn,
            };
            self.phase = Phase::First;
            return Ok(PlyOutcome::Win {
                winner: mover,
                turn: self.turn,
            });
        }

        if self.phase == Phase::First && stones_in_turn(self.turn) == TURN_STONES {
            self.phase = Phase::Second;
            return Ok(PlyOutcome::TurnContinues);
        }

        self.turn = match self.turn.checked_add(1) {
            Some(turn) => turn,
            None => panic!("pistol-core invariant {TURN_OVERFLOW}: turn counter exhausted u32"),
        };
        self.to_move = mover.opponent();
        self.phase = Phase::First;
        Ok(PlyOutcome::TurnComplete)
    }

    /// Take back the last stone, restoring the exact state it was placed from —
    /// side to move, phase, turn, and outcome, across a turn boundary and
    /// across the stone that ended the game alike.
    ///
    /// The only refusal is [`CoreError::NothingToUndo`]. If the board disagrees
    /// with the history about what the last stone was, that is this crate
    /// having broken its own invariant rather than a question anyone asked, and
    /// it panics with [`HISTORY_DESYNC`] — the one free cross-check that the
    /// two representations of the same game still agree.
    ///
    /// # Panics
    ///
    /// With [`HISTORY_DESYNC`] if the board does not hold the recorded stone.
    pub fn undo(&mut self) -> Result<Coord, CoreError> {
        // Peeked, not popped: the board comes off first, so no failure can
        // leave the state half-taken-back.
        let Some(&ply) = self.history.last() else {
            return Err(CoreError::NothingToUndo);
        };
        let removed = match self.board.undo(ply.at) {
            Ok(player) => player,
            Err(error) => panic!(
                "pistol-core invariant {HISTORY_DESYNC}: the history records a {} stone on {}, \
                 and the board says: {error}",
                ply.mover, ply.at
            ),
        };
        assert!(
            removed == ply.mover,
            "pistol-core invariant {HISTORY_DESYNC}: the history records a {} stone on {}, but \
             the board held a {removed} one",
            ply.mover,
            ply.at
        );
        self.history.pop();
        self.to_move = ply.mover;
        self.phase = ply.phase;
        self.turn = ply.turn;
        // A stone is only ever placed from an ongoing game, so the state it was
        // placed from was ongoing.
        self.outcome = Outcome::Ongoing;
        Ok(ply.at)
    }
}
