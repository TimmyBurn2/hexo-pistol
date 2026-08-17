//! The pinned rule constants.
//!
//! Every value here is a **game rule**, not a tunable. None of it is
//! configurable, none of it has a code-side default to override, and no other
//! crate re-states any of it (CLAUDE.md rule 2). A search knob that happens to
//! be a distance — the candidate radius — lives in the engine config and is
//! never compared with [`LEGAL_RADIUS`] (docs/decisions.md D-20).

/// Stones in a contiguous own run that win the game.
///
/// The test is `run >= WIN_LEN`, never `run == WIN_LEN`: an overline of seven or
/// more wins exactly as a six does (rule 2). There are no bans and no
/// exact-six variant.
pub const WIN_LEN: u32 = 6;

/// Hex distance within which a stone may be placed (rule 5).
///
/// A cell is in the legal region iff it lies within this distance of *some*
/// stone already on the board — the union of the radius-[`LEGAL_RADIUS`] balls,
/// not the ball around any one stone. See [`crate::Board::in_legal_region`].
pub const LEGAL_RADIUS: u32 = 8;

/// The number the first turn of a game carries. Turns count from one, and the
/// win-distance accounting of the whole engine is in turns (docs/decisions.md
/// D-3, D-9).
pub const FIRST_TURN: u32 = 1;

/// Stones placed on the first turn: one (rule 3).
pub const FIRST_TURN_STONES: u32 = 1;

/// Stones placed on every turn after the first: two (rule 3).
///
/// A turn that completes a line stops early — the second stone is then never
/// played (rule 4) — so this is what a turn *owes*, not what it always spends.
pub const TURN_STONES: u32 = 2;

/// How many stones the given turn owes, before rule 4 truncates it.
pub const fn stones_in_turn(turn: u32) -> u32 {
    if turn == FIRST_TURN {
        FIRST_TURN_STONES
    } else {
        TURN_STONES
    }
}
