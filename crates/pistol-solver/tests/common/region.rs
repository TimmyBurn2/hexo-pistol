use std::collections::BTreeMap;

use pistol_core::window::{WINDOW_LEN, Window};
use pistol_core::{Axis, Board, Coord};

use super::reference::RefWindow;

/// Every window whose start lies in the stones' bounding box grown by
/// [`WINDOW_LEN`], with what it holds — INCLUDING the ones holding nothing,
/// which is the half R1 cannot produce.
///
/// The box is wide enough by construction: a window holding a stone starts at
/// most `WINDOW_LEN - 1` steps back along its axis from that stone, and one step
/// moves each coordinate by at most one.
pub fn region_scan(board: &Board) -> BTreeMap<Window, RefWindow> {
    let mut scanned = BTreeMap::new();
    let Some((low, high)) = bounds(board) else {
        return scanned;
    };
    let grow = i32::try_from(WINDOW_LEN).expect("six fits");
    for q in (i32::from(low.q) - grow)..=(i32::from(high.q) + grow) {
        for r in (i32::from(low.r) - grow)..=(i32::from(high.r) + grow) {
            let (Ok(q), Ok(r)) = (i16::try_from(q), i16::try_from(r)) else {
                continue;
            };
            for axis in Axis::ALL {
                let Some(window) = Window::new(axis, Coord::new(q, r)) else {
                    continue;
                };
                scanned.insert(window, RefWindow::read(window, board));
            }
        }
    }
    scanned
}

/// The corners of the smallest axis-aligned `(q, r)` box holding every stone.
fn bounds(board: &Board) -> Option<(Coord, Coord)> {
    let mut stones = board.stones().map(|(at, _)| at);
    let first = stones.next()?;
    let (mut low, mut high) = (first, first);
    for at in stones {
        low = Coord::new(low.q.min(at.q), low.r.min(at.r));
        high = Coord::new(high.q.max(at.q), high.r.max(at.r));
    }
    Some((low, high))
}
