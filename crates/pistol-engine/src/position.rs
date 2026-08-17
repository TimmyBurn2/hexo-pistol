//! The position as the outside world states it, and the one way it becomes a
//! game.
//!
//! Two forms, and the reason there are two is docs/decisions.md D-6: the move
//! list is the canonical encoding of a position — the board is unbounded, so
//! there is no FEN analogue — and [`PositionSpec::Set`] exists for fixtures,
//! which need to say things a move list cannot, namely a position in the middle
//! of a turn.
//!
//! Both forms are turned into a position by **replaying** them through the
//! rules ([`PositionSpec::replay`]). That is D-42: legality, the turn structure,
//! rule 4's truncation and rule 5's legal region all fall out of the one
//! implementation of the rules in pistol-core, rather than being checked a
//! second time here by whoever loads a fixture. This module states no rule of
//! its own (CLAUDE.md rule 2); what it knows is how a *document* is laid out.
//!
//! # A won position is not a position the engine holds
//!
//! Either form is refused with [`EngineError::IllegalPosition`] if the game it
//! describes is already decided. The work package pins that for the `set` form
//! ("a set position already containing a >=6 run"), and it holds for the move
//! list for the same reason: a won position is terminal, so asking this engine
//! to stand on one is asking it for a move that does not exist (rule 4).

use pistol_core::{Coord, CoreError, GameState, Outcome, Phase, Player, Turn};

use crate::error::EngineError;

/// A position, as a caller states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionSpec {
    /// The game from the beginning, as the turns that were played. The
    /// canonical encoding (docs/decisions.md D-6); it always names a position
    /// at a turn boundary, because a turn is the unit it counts in.
    Start {
        /// The turns, in play order.
        moves: Vec<Turn>,
    },
    /// The stones each side holds, **in play order**, plus whose turn it is and
    /// how far into it they are.
    ///
    /// Play order is what makes this form replayable: rule 5's legal region
    /// grows as stones arrive, so whether a stone was legal depends on which
    /// stones preceded it, and a heap of stones cannot say. It is also what
    /// makes D-6's rule about the phase — "the mover's already-placed stone is
    /// the last listed stone of the side to move" — fall out rather than being a
    /// second rule this module enforces: at [`Phase::Second`] the mover's last
    /// listed stone *is* the last stone played.
    ///
    /// Order *within* a turn is not a claim, because a turn is an unordered pair
    /// (docs/decisions.md D-49): the two stones of one turn are replayed by
    /// [`GameState::make_turn`], which tries the canonical order and then the
    /// reverse (D-51). A pair that is legal in only one order — the case D-52
    /// constructs, where the second cell is reachable only through the ball the
    /// first one opens — is therefore accepted whichever way round it is
    /// written, and the ply history it leaves behind is the same one the move
    /// list would have left.
    ///
    /// The lists are never sorted or canonicalized here. The order given is the
    /// order replayed, so a refusal names the ply it happened on.
    Set {
        /// P1's stones, in play order.
        p1: Vec<Coord>,
        /// P2's stones, in play order.
        p2: Vec<Coord>,
        /// The side that places the next stone.
        to_move: Player,
        /// How far into the current turn that side is.
        phase: Phase,
    },
}

impl PositionSpec {
    /// The position this spec describes, or the named reason it is not one.
    ///
    /// Every refusal comes from the rules layer, mapped at this call site
    /// (docs/decisions.md D-31): a rejected turn in a move list is an
    /// [`EngineError::IllegalMove`] naming the turn it belongs to, and anything
    /// wrong with a stated stone list is an [`EngineError::IllegalPosition`],
    /// because a stone list is a claim about a whole position rather than about
    /// one move.
    pub fn replay(&self) -> Result<GameState, EngineError> {
        let state = match self {
            PositionSpec::Start { moves } => replay_moves(moves)?,
            PositionSpec::Set {
                p1,
                p2,
                to_move,
                phase,
            } => replay_stones(p1, p2, *to_move, *phase)?,
        };
        if let Outcome::Win { winner, turn } = state.outcome() {
            return Err(EngineError::illegal_position(format!(
                "{winner} completed a line on turn {turn}: a won position is terminal, so \
                 there is no move to ask this engine for (rule 4)"
            )));
        }
        Ok(state)
    }
}

/// Play a move list, one whole turn at a time.
fn replay_moves(moves: &[Turn]) -> Result<GameState, EngineError> {
    let mut state = GameState::new_game();
    for &turn in moves {
        let number = state.turn();
        state
            .make_turn(turn)
            .map_err(|error| illegal_move(error, number, turn))?;
    }
    Ok(state)
}

/// Replay two stone lists by asking the rules whose stone comes next.
///
/// The turn structure is not restated here (CLAUDE.md rule 2). The state machine
/// already knows whose turn it is and how many stones that turn owes, so the
/// replay reads those and takes the next stones from that side's list: rule 3's
/// one-then-two pattern is obeyed without this module holding a copy of it.
///
/// A side whose remaining stones are fewer than its turn owes is the position in
/// the middle of a turn (D-6), and the one leftover stone is played as the ply it
/// is. That can only happen at the end, which the exhaustion check below is what
/// enforces.
///
/// The stated `to_move` and `phase` are **checked, not trusted**: the stone lists
/// alone fix the whole structure, so a document that contradicts itself is
/// refused rather than repaired (CLAUDE.md rule 3). They are still required
/// tokens, because a fixture that states what it means is a fixture whose
/// meaning survives being edited.
fn replay_stones(
    p1: &[Coord],
    p2: &[Coord],
    to_move: Player,
    phase: Phase,
) -> Result<GameState, EngineError> {
    let lists = [p1, p2];
    let mut taken = [0usize, 0usize];
    let mut state = GameState::new_game();

    loop {
        if state.outcome().is_decided() {
            // A stone that completed a line ends the game (rule 4). Whether any
            // stones are left over is the exhaustion check's business; the
            // decided position itself is refused by the caller.
            break;
        }
        let side = state.to_move();
        let index = side_index(side);
        let remaining = &lists[index][taken[index]..];
        if remaining.is_empty() {
            break;
        }
        let ply = taken[0] + taken[1] + 1;
        let owed = state.stones_owed() as usize;
        if remaining.len() < owed {
            // Fewer stones than the turn owes: this is the mover's first stone of
            // a turn in progress, and it is a ply rather than a turn, so the
            // ply-level entry point is what plays it (D-50, D-71).
            state
                .place(remaining[0])
                .map_err(|error| stone_refusal(error, ply, side))?;
            taken[index] += 1;
            break;
        }
        let turn = whole_turn(remaining, owed, ply, side)?;
        state
            .make_turn(turn)
            .map_err(|error| stone_refusal(error, ply, side))?;
        taken[index] += owed;
    }

    if state.outcome().is_decided() {
        // The game ended inside the list. That is the reason this is not a
        // position the engine can hold, and it is a better answer than anything
        // the checks below could say about whose turn it is or what is left over:
        // after a win the state freezes on the winner, so those checks would
        // report a mover disagreement and blame the wrong thing. The caller names
        // the win (docs/decisions.md D-84).
        return Ok(state);
    }
    for (index, list) in lists.iter().enumerate() {
        let left = &list[taken[index]..];
        if let Some(&stone) = left.first() {
            let side = side_of(index);
            return Err(EngineError::illegal_position(format!(
                "{} of {side}'s stones are left over after replaying {} plies, starting with \
                 {stone}: p1: has {} and p2: has {}, which is not a turn structure any game \
                 reaches (rule 3: one stone on turn 1, two on every turn after)",
                left.len(),
                taken[0] + taken[1],
                p1.len(),
                p2.len(),
            )));
        }
    }
    if state.to_move() != to_move || state.phase() != phase {
        return Err(EngineError::illegal_position(format!(
            "those stones leave {} to move at phase {}, and the document says {to_move} at \
             phase {}",
            state.to_move(),
            state.phase().index(),
            phase.index(),
        )));
    }
    Ok(state)
}

/// The turn the next `owed` stones of a list make up.
fn whole_turn(
    remaining: &[Coord],
    owed: usize,
    ply: usize,
    side: Player,
) -> Result<Turn, EngineError> {
    match owed {
        1 => Ok(Turn::Single(remaining[0])),
        2 => {
            Turn::pair(remaining[0], remaining[1]).map_err(|error| stone_refusal(error, ply, side))
        }
        // A turn owes one stone or two (rule 3), and a decided game owes none —
        // which the loop has already broken out of before calling this. Reading a
        // second stone on the strength of that reasoning would be an index panic
        // instead of a named failure, so the impossible case says so.
        other => Err(EngineError::internal(format!(
            "the turn at stone {ply} of the replay owes {other} stones, which rule 3 does not \
             allow"
        ))),
    }
}

/// Which slot of the stone lists a side owns. A local convenience, not a rule.
const fn side_index(side: Player) -> usize {
    match side {
        Player::P1 => 0,
        Player::P2 => 1,
    }
}

/// The inverse of [`side_index`].
const fn side_of(index: usize) -> Player {
    match index {
        0 => Player::P1,
        _ => Player::P2,
    }
}

/// A rules refusal of a stated stone, as the engine reports it.
///
/// A stone list is a claim about a whole position rather than about one move, so
/// this is an [`EngineError::IllegalPosition`] and not an
/// [`EngineError::IllegalMove`]: the operator named no turn, and naming one back
/// at them would invent a turn index they never wrote. The ply the refusal
/// happened on is named instead, because "in play order" is what the form means
/// and the ply is where that order failed.
fn stone_refusal(error: CoreError, ply: usize, side: Player) -> EngineError {
    match error {
        CoreError::NothingToUndo | CoreError::UnoccupiedCell { .. } => EngineError::internal(
            format!("replaying stone {ply} raised an undo refusal: {error}"),
        ),
        // The replay asks the state machine whose stone comes next and how many
        // that turn owes, so it never plays a turn into a half-played one.
        CoreError::TurnInProgress { .. } => EngineError::internal(format!(
            "replaying stone {ply} found a turn already in progress: {error}"
        )),
        rules => EngineError::illegal_position(format!(
            "stone {ply} of the replay, {side}'s, listed in play order: {rules}"
        )),
    }
}

/// A rules refusal of a stated turn, as the engine reports it.
///
/// `number` is the turn the refused move belongs to, which is what
/// [`EngineError::IllegalMove`] carries (docs/decisions.md D-10); the turn
/// itself travels in the explanation, because "turn 4" and "4,0/5,0" answer two
/// different questions an operator has.
fn illegal_move(error: CoreError, number: u32, turn: Turn) -> EngineError {
    match error {
        CoreError::NothingToUndo | CoreError::UnoccupiedCell { .. } => EngineError::internal(
            format!("playing turn {turn} raised an undo refusal: {error}"),
        ),
        // `make_turn` from a turn boundary lands on a turn boundary or refuses,
        // and a replay only ever starts from one.
        CoreError::TurnInProgress { .. } => EngineError::internal(format!(
            "turn {number} was replayed from the middle of a turn: {error}"
        )),
        rules => EngineError::IllegalMove {
            turn: number,
            why: format!("{turn}: {rules}"),
        },
    }
}
