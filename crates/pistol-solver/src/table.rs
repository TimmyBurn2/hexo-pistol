//! The per-window store: what each side holds in every window that holds a
//! stone.
//!
//! # The record is a MASK and not a pair of counts
//!
//! Six occupancy bits per side. The counts are `count_ones()` and the *empty
//! cells* fall out as `!(p1 | p2)`, with no board access at all. A record of
//! two counts cannot say WHICH cells are empty, so a state carrying only counts
//! would have to hold or be handed a `&Board` on every query path — a second
//! source of occupancy beside the one it is already maintaining, which is
//! exactly the drift `EVAL_DESYNC` exists to catch in the eval. The per-stone
//! update costs the same either way: one bit set or cleared per window instead
//! of one increment.
//!
//! # The store
//!
//! A `std::HashMap` keyed by an order-preserving packed `u64`, hashed by
//! splitmix64 with written-down constants and no seed state (docs/decisions.md
//! D-225, D-254). It is legal for this to be hashed *because the table is never
//! iterated on a choice path*: every query answers out of the maintained sorted
//! sets in [`crate::sets`], and the only enumeration of the table is
//! [`WindowTable::snapshot`], which sorts (CLAUDE.md rule 4).
//!
//! An entry exists exactly while the window holds at least one stone, and
//! [`WindowTable::set`] prunes the moment it empties. That is D-62's rule for
//! the eval's own map, and it is what makes a state that has been unwound
//! EQUAL to a fresh one rather than merely equal in every answer.

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasherDefault, Hasher};

use pistol_core::window::{WINDOW_LEN, Window};
use pistol_core::{Axis, Coord, Player};

/// Every cell of a window, as a bit per cell.
pub const FULL_MASK: u8 = (1u8 << WINDOW_LEN) - 1;

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

    /// Whether the window holds no stone at all — the pruning condition.
    pub fn is_vacant(self) -> bool {
        self.p1 == 0 && self.p2 == 0
    }

    /// This record with `side`'s bit at `index` set or cleared.
    pub fn with(self, side: Player, index: u8, occupied: bool) -> WindowMasks {
        let bit = 1u8 << index;
        let mut masks = self;
        let slot = match side {
            Player::P1 => &mut masks.p1,
            Player::P2 => &mut masks.p2,
        };
        if occupied {
            *slot |= bit;
        } else {
            *slot &= !bit;
        }
        masks
    }
}

/// The empty cells of `window`, in window order, which is also `(q, r)`
/// lexicographic order: every axis direction — `(0,1)`, `(1,0)`, `(1,-1)` —
/// increases `q` or, where `q` is constant, `r`.
pub fn empty_cells(window: Window, masks: WindowMasks) -> impl Iterator<Item = Coord> {
    let empties = masks.empties();
    (0..WINDOW_LEN as u8)
        .filter(move |index| empties & (1u8 << index) != 0)
        .map(move |index| window.cell(index))
}

/// A window as an integer key that ORDERS the same way the window does.
///
/// `(axis, start.q, start.r)`, each biased into an unsigned field, so that
/// `pack(a) < pack(b)` exactly when `a < b`. Order preservation is not needed by
/// the hash — it is needed by [`WindowTable::snapshot`], which unpacks — and it
/// is what makes the key a lossless renaming of the window rather than a digest.
pub fn pack(window: Window) -> u64 {
    let axis = match window.axis {
        Axis::ConstQ => 0u64,
        Axis::ConstR => 1,
        Axis::ConstS => 2,
    };
    let q = (i32::from(window.start.q) - i32::from(i16::MIN)) as u64;
    let r = (i32::from(window.start.r) - i32::from(i16::MIN)) as u64;
    (axis << 32) | (q << 16) | r
}

/// The window a key names. The inverse of [`pack`], exactly.
///
/// # Panics
///
/// If `key` was not produced by [`pack`]: a key naming no axis is a bug in this
/// crate rather than an answer to anyone's question (CLAUDE.md rule 3).
pub fn unpack(key: u64) -> Window {
    let axis = match key >> 32 {
        0 => Axis::ConstQ,
        1 => Axis::ConstR,
        2 => Axis::ConstS,
        other => panic!("{other} names no axis: {key:#x} did not come from pack()"),
    };
    let field = |shift: u32| -> i16 {
        let biased = i32::try_from((key >> shift) & 0xFFFF).expect("sixteen bits fit in i32");
        i16::try_from(biased + i32::from(i16::MIN)).expect("a biased coordinate fits back")
    };
    let start = Coord::new(field(16), field(0));
    Window { axis, start }
}

/// splitmix64's finalizer, as a `Hasher`: fixed constants, no seed state,
/// nothing per-process (CLAUDE.md rule 4).
///
/// The alternative this workspace measured — a multiply-only FxHash-style
/// mix — collapses to `n * K` from a zero state, and since the low bits of a
/// product depend only on the low bits of the operand while a packed key's low
/// sixteen bits are the biased `r`, every window sharing an `r` lands in the
/// same bucket: 27 distinct buckets and 81 per bucket over a realistic key set,
/// at every table size (docs/decisions.md D-254).
#[derive(Debug, Clone, Copy, Default)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn mix(value: u64) -> u64 {
        let mut z = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

impl Hasher for SplitMix64 {
    /// Byte input. Unreached by [`WindowTable`], whose key is a `u64`, and
    /// written correctly rather than left to panic so that it is a hasher for
    /// whatever a future caller hands it.
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = SplitMix64::mix(self.state ^ u64::from(byte));
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.state = SplitMix64::mix(self.state ^ value);
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

/// The window table: packed key to masks, with the pruning rule.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowTable {
    entries: HashMap<u64, WindowMasks, BuildHasherDefault<SplitMix64>>,
}

impl WindowTable {
    /// What `window` holds. A window with no entry holds nothing, which is the
    /// same answer as an entry of zeroes and is why there is no `Option` here.
    pub fn masks(&self, window: Window) -> WindowMasks {
        self.entries.get(&pack(window)).copied().unwrap_or_default()
    }

    /// Record what `window` holds, pruning the entry the moment it holds
    /// nothing (D-62).
    pub fn set(&mut self, window: Window, masks: WindowMasks) {
        let key = pack(window);
        if masks.is_vacant() {
            self.entries.remove(&key);
        } else {
            self.entries.insert(key, masks);
        }
    }

    /// How many windows hold a stone.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no window holds a stone.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The whole table, sorted by window.
    ///
    /// This is the only enumeration of the table there is, it allocates, and it
    /// is for oracles and diagnostics. Nothing on a choice path may call it:
    /// the queries answer out of the maintained sets, which are sorted by
    /// construction, and that split is what licenses a hashed table at all.
    pub fn snapshot(&self) -> BTreeMap<Window, WindowMasks> {
        self.entries
            .iter()
            .map(|(&key, &masks)| (unpack(key), masks))
            .collect()
    }
}
