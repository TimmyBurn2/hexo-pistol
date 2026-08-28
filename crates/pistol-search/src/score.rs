use pistol_eval::EVAL_MAX;

/// A completed line, scored at zero distance. No position ever holds this
/// score: a win is always at least one turn away from the node that reports it.
pub const MATE: i32 = 30_000;

/// Wider than any score, so an alpha-beta window can start open on both sides
/// without a special case for "no bound yet".
pub const INFINITY: i32 = MATE + 1;

/// The widest mate distance a score can carry, in turns.
///
/// It is a band, not a limit on play: it bounds how deep a *search* can be and
/// still express a mate distance, and the search's own horizon
/// ([`crate::search::MAX_DEPTH_TURNS`]) is far inside it.
pub const MAX_MATE_TURNS: u32 = 1_000;

/// At or above this magnitude, a score is a mate distance rather than a value.
pub const MATE_THRESHOLD: i32 = MATE - MAX_MATE_TURNS as i32;

/// Named invariant: a mate distance outside the band the score encoding holds.
pub const MATE_DISTANCE_OUT_OF_BAND: &str = "MATE_DISTANCE_OUT_OF_BAND";

// The two bands may not meet: a saturated static evaluation must still read as
// a value, or the search would announce a mate it never found.
const _: () = assert!(
    EVAL_MAX < MATE_THRESHOLD,
    "the static evaluation band must sit strictly below the mate band"
);

/// The score of a win that completes `turns` turns from here.
///
/// # Panics
///
/// With [`MATE_DISTANCE_OUT_OF_BAND`] if the distance is zero — no node is its
/// own win — or wider than [`MAX_MATE_TURNS`].
pub fn mate_in(turns: u32) -> i32 {
    assert!(
        (1..=MAX_MATE_TURNS).contains(&turns),
        "pistol-search invariant {MATE_DISTANCE_OUT_OF_BAND}: {turns} turns is not a mate \
         distance this build can express (1..={MAX_MATE_TURNS})"
    );
    MATE - turns as i32
}

/// Whether this score is a mate distance rather than a static value.
pub fn is_mate(score: i32) -> bool {
    score.abs() >= MATE_THRESHOLD
}

/// What a score says, in the vocabulary the engine reports
/// (docs/decisions.md D-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreKind {
    /// A static evaluation, positive for the side to move.
    Eval(i32),
    /// The side to move completes a line this many turns from now.
    MateIn(u16),
    /// The opponent does.
    MatedIn(u16),
}

/// Read a raw score as one of the three things it can be.
pub fn classify(score: i32) -> ScoreKind {
    if score >= MATE_THRESHOLD {
        ScoreKind::MateIn(distance(MATE - score))
    } else if score <= -MATE_THRESHOLD {
        ScoreKind::MatedIn(distance(MATE + score))
    } else {
        ScoreKind::Eval(score)
    }
}

/// A mate distance as the `u16` the reported score kind carries.
fn distance(turns: i32) -> u16 {
    u16::try_from(turns).unwrap_or_else(|_| {
        panic!(
            "pistol-search invariant {MATE_DISTANCE_OUT_OF_BAND}: {turns} turns is not a \
             reportable mate distance"
        )
    })
}

/// Re-base a root-relative score onto the node that is storing it.
///
/// A static value is not a distance and passes through untouched.
pub fn to_table(score: i32, turns_from_root: u32) -> i32 {
    shift(score, turns_from_root as i32)
}

/// Re-base a stored, node-relative score onto the root.
pub fn from_table(score: i32, turns_from_root: u32) -> i32 {
    shift(score, -(turns_from_root as i32))
}

/// Move a mate score `by` turns closer to (positive) or further from (negative)
/// the node that holds it, leaving a static value alone.
fn shift(score: i32, by: i32) -> i32 {
    if score >= MATE_THRESHOLD {
        score + by
    } else if score <= -MATE_THRESHOLD {
        score - by
    } else {
        score
    }
}
