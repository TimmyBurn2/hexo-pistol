use std::collections::BTreeSet;

use pistol_core::{Axis, Board, Coord, Player, WIN_LEN};
use pistol_eval::{EVAL_MAX, Weights};

/// The window length the v0 backend reads: the win length (rules 2 and D-11).
/// Six fits an `i16` step count many times over.
const LEN: i16 = WIN_LEN as i16;

/// The value of `board` to `side_to_move`, from the windows through its stones.
pub fn value_from_scratch(board: &Board, weights: &Weights, side_to_move: Player) -> i32 {
    let mut windows = BTreeSet::new();
    for (stone, _) in board.stones() {
        for axis in Axis::ALL {
            for back in 0..LEN {
                if let Some(start) = stone.checked_step(axis, -back) {
                    windows.insert((axis, start));
                }
            }
        }
    }
    let score = windows
        .into_iter()
        .filter_map(|(axis, start)| window_score(board, weights, axis, start))
        .sum();
    signed(score, side_to_move)
}

/// The value of `board` to `side_to_move`, from a scan of a region that
/// contains every window that could hold a stone.
pub fn value_by_region_scan(board: &Board, weights: &Weights, side_to_move: Player) -> i32 {
    let mut cells = board.stones().map(|(at, _)| at);
    let Some(first) = cells.next() else {
        // No stones, no window with anything in it. An unbounded board has
        // infinitely many empty windows and they score nothing.
        return 0;
    };
    let (mut low, mut high) = (first, first);
    for at in cells {
        low = Coord::new(low.q.min(at.q), low.r.min(at.r));
        high = Coord::new(high.q.max(at.q), high.r.max(at.r));
    }
    // A window that holds a stone starts within LEN - 1 steps of it, and one
    // step moves each component by at most one, so a pad of LEN is generous.
    let mut score = 0i64;
    for q in low.q.saturating_sub(LEN)..=high.q.saturating_add(LEN) {
        for r in low.r.saturating_sub(LEN)..=high.r.saturating_add(LEN) {
            for axis in Axis::ALL {
                score += window_score(board, weights, axis, Coord::new(q, r)).unwrap_or(0);
            }
        }
    }
    signed(score, side_to_move)
}

/// What one window is worth, signed for whoever owns the stones in it, or
/// `None` if the window runs off the addressable lattice and so is not a window.
///
/// This is D-11 said literally: a window holding both players is dead, and any
/// other window holding stones is worth the table entry for how many it holds,
/// positive for P1 and negative for P2.
fn window_score(board: &Board, weights: &Weights, axis: Axis, start: Coord) -> Option<i64> {
    let mut p1 = 0u8;
    let mut p2 = 0u8;
    for step in 0..LEN {
        match board.get(start.checked_step(axis, step)?) {
            Some(Player::P1) => p1 += 1,
            Some(Player::P2) => p2 += 1,
            None => {}
        }
    }
    let score = match (p1, p2) {
        (_, 0) => i64::from(weights.window_value(p1)),
        (0, _) => -i64::from(weights.window_value(p2)),
        _ => 0,
    };
    Some(score)
}

/// A P1-relative score as the side to move reads it, clamped into the eval
/// band (docs/decisions.md D-3).
pub fn signed(score: i64, side_to_move: Player) -> i32 {
    let band = i64::from(EVAL_MAX);
    let clamped = i32::try_from(score.clamp(-band, band)).expect("the clamp keeps it in range");
    match side_to_move {
        Player::P1 => clamped,
        Player::P2 => -clamped,
    }
}
