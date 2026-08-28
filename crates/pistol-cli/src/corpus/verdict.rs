use pistol_core::{Coord, Player, Turn};

/// One turn of a game, as the record spells it and as pistol-core names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupedTurn {
    /// The turn, canonically spelled — an unordered pair, or a single stone.
    pub turn: Turn,
    /// The stone the record played first. Not part of the turn's identity; the
    /// probe is the only thing that reads it.
    pub recorded_first: Coord,
    /// Where the record's first stone of this turn sits in the flat `moves`
    /// array — the coordinate an operator can grep the corpus with.
    pub move_index: usize,
}

/// Why a game is not eligible, or that it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Every turn played, the game ended decided, and the winner agrees.
    Eligible,
    /// pistol-core refused a turn.
    IllegalTurn {
        /// Which turn, counting from one.
        turn_number: u32,
        /// Where its first recorded stone is in the flat `moves` array.
        move_index: usize,
        /// The turn as pistol-core was asked to play it.
        turn: Turn,
        /// pistol-core's own refusal.
        why: String,
    },
    /// Stones continue after a turn completed a line (rule 4).
    PostWinContinuation {
        /// The flat index of the first stone that should not exist.
        move_index: usize,
    },
    /// The replay's winner is not the one the record states. This is the only
    /// check that catches a mis-grouped or interleaved export, in which the
    /// stones would be assigned to the wrong sides and *both* replays would
    /// happily accept a different but perfectly legal game.
    WinnerMismatch {
        /// Who pistol-core says won.
        replayed: Player,
        /// Who the record says won.
        recorded: Player,
    },
    /// The stones ran out with no line completed, though the corpus states it
    /// holds only decisive games.
    NotDecided,
}

impl Verdict {
    /// The one-word name this verdict is listed under in a fixture header.
    pub fn name(&self) -> &'static str {
        match self {
            Verdict::Eligible => "eligible",
            Verdict::IllegalTurn { .. } => "illegal-turn",
            Verdict::PostWinContinuation { .. } => "post-win-continuation",
            Verdict::WinnerMismatch { .. } => "winner-mismatch",
            Verdict::NotDecided => "not-decided",
        }
    }

    /// The flat move index this verdict points at, where it points at one.
    pub fn move_index(&self) -> Option<usize> {
        match self {
            Verdict::IllegalTurn { move_index, .. }
            | Verdict::PostWinContinuation { move_index } => Some(*move_index),
            _ => None,
        }
    }

    /// Whether the game may be curated from.
    pub fn is_eligible(&self) -> bool {
        matches!(self, Verdict::Eligible)
    }
}

/// What one game's replay found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replayed {
    /// Whether the game is eligible, or why not.
    pub verdict: Verdict,
    /// The turns, in play order. Empty where the regrouping itself failed.
    pub turns: Vec<GroupedTurn>,
    /// Turns whose recorded first stone was outside the legal region, and which
    /// the reverse ordering played.
    pub order_rescued: usize,
    /// Turns whose recorded first stone already completed a line, leaving the
    /// record's second stone after a decided game.
    pub stone_after_win: usize,
    /// The turn the game was decided on, where it was.
    pub decided_on_turn: Option<u32>,
}
