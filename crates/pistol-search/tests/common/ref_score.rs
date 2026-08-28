use std::cmp::Ordering;

use pistol_search::score::{ScoreKind, classify};

/// A position's value to the side to move.
///
/// A distance counts EVERY turn from the ROOT, both sides' (docs/decisions.md
/// D-72), so it does not change as a value is handed up the tree; only the
/// point of view flips ([`RefScore::negate`]). A win for the side to move is
/// therefore always an odd distance and a loss always an even one, which is the
/// sanity check to apply to any mate this suite prints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefScore {
    /// The opponent completes a line this many turns from the root.
    LossInTurns(u32),
    /// A static evaluation, positive for the side to move.
    Value(i32),
    /// The side to move completes a line this many turns from the root.
    WinInTurns(u32),
}

impl RefScore {
    /// The same score seen from the other side of the board.
    pub fn negate(self) -> RefScore {
        match self {
            RefScore::LossInTurns(turns) => RefScore::WinInTurns(turns),
            RefScore::Value(value) => RefScore::Value(-value),
            RefScore::WinInTurns(turns) => RefScore::LossInTurns(turns),
        }
    }

    /// Whether this is a mate rather than a static value.
    pub fn is_mate(self) -> bool {
        !matches!(self, RefScore::Value(_))
    }

    fn band(self) -> u8 {
        match self {
            RefScore::LossInTurns(_) => 0,
            RefScore::Value(_) => 1,
            RefScore::WinInTurns(_) => 2,
        }
    }
}

impl Ord for RefScore {
    /// Any win beats any value and any value beats any loss — which is what the
    /// engine's band layout says too, where `EVAL_MAX` sits strictly below
    /// `MATE_THRESHOLD` under a `const` assertion in `score.rs`. Inside the mate
    /// bands a sooner win is better, and a later loss is better.
    fn cmp(&self, other: &RefScore) -> Ordering {
        match (self, other) {
            (RefScore::WinInTurns(mine), RefScore::WinInTurns(theirs)) => theirs.cmp(mine),
            (RefScore::LossInTurns(mine), RefScore::LossInTurns(theirs)) => mine.cmp(theirs),
            (RefScore::Value(mine), RefScore::Value(theirs)) => mine.cmp(theirs),
            _ => self.band().cmp(&other.band()),
        }
    }
}

impl PartialOrd for RefScore {
    fn partial_cmp(&self, other: &RefScore) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The engine's packed score, read in the reference's vocabulary.
///
/// It goes through `classify`, which is what every consumer of a score uses, so
/// the packing is one of the things this oracle certifies.
pub fn engine_score_as_reference(score: i32) -> RefScore {
    match classify(score) {
        ScoreKind::Eval(value) => RefScore::Value(value),
        ScoreKind::MateIn(turns) => RefScore::WinInTurns(u32::from(turns)),
        ScoreKind::MatedIn(turns) => RefScore::LossInTurns(u32::from(turns)),
    }
}
