use pistol_core::window::{WINDOW_LEN, Window};
use pistol_core::{Coord, Player};

/// Every cell of a window, as a bit per cell.
pub(crate) const FULL_MASK: u8 = (1u8 << WINDOW_LEN) - 1;

/// What one window holds: six bits per side, `1 << index` per cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindowMasks {
    /// P1's cells.
    pub p1: u8,
    /// P2's cells.
    pub p2: u8,
}

impl WindowMasks {
    /// `side`'s cells.
    pub fn own(self, side: Player) -> u8 {
        match side {
            Player::P1 => self.p1,
            Player::P2 => self.p2,
        }
    }

    /// The other side's cells.
    pub fn opp(self, side: Player) -> u8 {
        self.own(side.opponent())
    }

    /// `side`'s stone count in this window.
    pub fn own_count(self, side: Player) -> u32 {
        self.own(side).count_ones()
    }

    /// The other side's stone count in this window.
    pub fn opp_count(self, side: Player) -> u32 {
        self.opp(side).count_ones()
    }

    /// The cells nobody holds.
    pub fn empties(self) -> u8 {
        !(self.p1 | self.p2) & FULL_MASK
    }
}

/// The empty cells of `window`, in window order, which is also `(q, r)`
/// lexicographic order: every axis direction — `(0,1)`, `(1,0)`, `(1,-1)` —
/// increases `q` or, where `q` is constant, `r`.
pub(crate) fn empty_cells(window: Window, masks: WindowMasks) -> impl Iterator<Item = Coord> {
    let empties = masks.empties();
    (0..WINDOW_LEN as u8)
        .filter(move |index| empties & (1u8 << index) != 0)
        .map(move |index| window.cell(index))
}
