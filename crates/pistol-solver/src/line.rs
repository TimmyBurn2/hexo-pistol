use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::{BuildHasherDefault, Hasher};

use pistol_core::window::{WINDOW_LEN, Window, windows_through_indexed};
use pistol_core::{Axis, Coord, Player};

use crate::table::WindowMasks;

/// How many consecutive positions of one line a chunk holds.
const CHUNK_LEN: i32 = 64;

/// One chunk of one line: a bit per cell per side, bit `i` standing for
/// position `index * CHUNK_LEN + i` along the axis direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Chunk {
    p1: u64,
    p2: u64,
}

impl Chunk {
    fn is_vacant(self) -> bool {
        self.p1 == 0 && self.p2 == 0
    }

    fn side_mut(&mut self, side: Player) -> &mut u64 {
        match side {
            Player::P1 => &mut self.p1,
            Player::P2 => &mut self.p2,
        }
    }
}

/// Where a cell sits on one of its three lines: the line's invariant
/// coordinate, and the cell's position along the axis direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LinePos {
    pub(crate) line: i32,
    pub(crate) pos: i32,
}

impl LinePos {
    /// `at` on its `axis` line. The direction vectors are `(0,1)`, `(1,0)`
    /// and `(1,-1)` (`Axis::direction`), so the position is the coordinate
    /// that increases along the axis and the line is the one that does not.
    pub(crate) fn of(axis: Axis, at: Coord) -> LinePos {
        let (q, r) = (i32::from(at.q), i32::from(at.r));
        match axis {
            Axis::ConstQ => LinePos { line: q, pos: r },
            Axis::ConstR => LinePos { line: r, pos: q },
            Axis::ConstS => LinePos {
                line: q + r,
                pos: q,
            },
        }
    }

    /// The inverse of [`LinePos::of`], for a position a chunk bit names.
    fn cell(self, axis: Axis) -> Coord {
        let (q, r) = match axis {
            Axis::ConstQ => (self.line, self.pos),
            Axis::ConstR => (self.pos, self.line),
            Axis::ConstS => (self.pos, self.line - self.pos),
        };
        let coordinate = |value: i32| {
            i16::try_from(value)
                .expect("a chunk bit names a cell that was placed, which is addressable")
        };
        Coord::new(coordinate(q), coordinate(r))
    }
}

/// A chunk's key: axis, line and chunk index, each biased into its own
/// unsigned field. `q + r` spans `[-65536, 65534]` over the lattice and a
/// chunk index `[-512, 511]`, and a run read one chunk past either end reaches
/// `-513` and `512`; the biases below keep every field non-negative, and the
/// fields — 24 bits of line above 16 of chunk, the axis above both — disjoint.
pub(crate) fn pack(axis: Axis, line: i32, chunk: i32) -> u64 {
    let axis = match axis {
        Axis::ConstQ => 0u64,
        Axis::ConstR => 1,
        Axis::ConstS => 2,
    };
    let line = u64::try_from(line + (1 << 17)).expect("a line index lies within the lattice");
    let chunk = u64::try_from(chunk + (1 << 10)).expect("a chunk index lies within the lattice");
    (axis << 40) | (line << 16) | chunk
}

/// The inverse of [`pack`], exactly.
///
/// # Panics
///
/// If `key` was not produced by [`pack`]: a key naming no axis is a bug in
/// this crate rather than an answer to anyone's question (CLAUDE.md rule 3).
pub(crate) fn unpack(key: u64) -> (Axis, i32, i32) {
    let axis = match key >> 40 {
        0 => Axis::ConstQ,
        1 => Axis::ConstR,
        2 => Axis::ConstS,
        other => panic!("{other} names no axis: {key:#x} did not come from pack()"),
    };
    let line =
        i32::try_from((key >> 16) & 0xFF_FFFF).expect("a line field fits in i32") - (1 << 17);
    let chunk = i32::try_from(key & 0xFFFF).expect("a chunk field fits in i32") - (1 << 10);
    (axis, line, chunk)
}

/// splitmix64's finalizer, as a `Hasher`: fixed constants, no seed state,
/// nothing per-process (CLAUDE.md rule 4).
///
/// The alternative this workspace measured — a multiply-only FxHash-style
/// mix — collapses to `n * K` from a zero state, and since the low bits of a
/// product depend only on the low bits of the operand while a packed key's low
/// bits are one coordinate, keys sharing it land in one bucket
/// (docs/decisions.md D-254). splitmix64 mixes every input bit.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SplitMix64 {
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
    /// Byte input. Unreached by [`LineStore`], whose key is a `u64`, and
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

/// The store: per-axis line bitboards, chunk key to the two sides' occupancy
/// along that run of the line, with the pruning rule.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct LineStore {
    chunks: HashMap<u64, Chunk, BuildHasherDefault<SplitMix64>>,
}

impl LineStore {
    fn chunk(&self, axis: Axis, line: i32, index: i32) -> Chunk {
        self.chunks
            .get(&pack(axis, line, index))
            .copied()
            .unwrap_or_default()
    }

    /// Both sides' bits over `count` consecutive positions of one line from
    /// `from`: bit `j` of each stands for position `from + j`. `count` is at
    /// most 64, so at most two chunks are read, and the second only when the
    /// run crosses a chunk boundary — `offset + count > 64` is false at
    /// `offset == 0`, so the shift by `64 - offset` is never a shift by 64.
    pub(crate) fn run(&self, axis: Axis, line: i32, from: i32, count: u32) -> (u64, u64) {
        debug_assert!(count <= 64, "a run is at most one chunk long");
        let index = from.div_euclid(CHUNK_LEN);
        let offset = from.rem_euclid(CHUNK_LEN) as u32;
        let low = self.chunk(axis, line, index);
        let mut p1 = low.p1 >> offset;
        let mut p2 = low.p2 >> offset;
        if offset + count > 64 {
            let high = self.chunk(axis, line, index + 1);
            p1 |= high.p1 << (64 - offset);
            p2 |= high.p2 << (64 - offset);
        }
        let keep = if count == 64 {
            u64::MAX
        } else {
            (1u64 << count) - 1
        };
        (p1 & keep, p2 & keep)
    }

    /// What `window` holds. A window whose line holds nothing reads as all
    /// zeroes, which is the same answer as a record of zeroes and is why there
    /// is no `Option` here.
    pub(crate) fn masks(&self, window: Window) -> WindowMasks {
        let at = LinePos::of(window.axis, window.start);
        let (p1, p2) = self.run(window.axis, at.line, at.pos, WINDOW_LEN);
        WindowMasks {
            p1: p1 as u8,
            p2: p2 as u8,
        }
    }

    /// Set `side`'s bit at `at` on its `axis` line. Returns the chunk's key and
    /// what it held before, which [`LineStore::restore`] takes back. A bit is
    /// never cleared here — the only way back is the log — and a chunk that
    /// gains a bit is never vacant, so no pruning question arises on this path.
    pub(crate) fn place(&mut self, axis: Axis, at: Coord, side: Player) -> (u64, Chunk) {
        let at = LinePos::of(axis, at);
        let key = pack(axis, at.line, at.pos.div_euclid(CHUNK_LEN));
        let bit = 1u64 << at.pos.rem_euclid(CHUNK_LEN);
        let before = self.chunks.get(&key).copied().unwrap_or_default();
        let mut after = before;
        *after.side_mut(side) |= bit;
        self.chunks.insert(key, after);
        (key, before)
    }

    /// Put a chunk back as [`LineStore::place`] found it, pruning the chunk the
    /// moment it holds nothing (D-62): a record that was vacant before the
    /// stone is removed rather than written back as zeroes, so an unwound store
    /// EQUALS a fresh one.
    pub(crate) fn restore(&mut self, key: u64, before: Chunk) {
        if before.is_vacant() {
            self.chunks.remove(&key);
        } else {
            self.chunks.insert(key, before);
        }
    }

    /// Whether no line holds a stone, which by the pruning rule is whether no
    /// window does.
    pub(crate) fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    /// Every addressable window holding a stone, sorted by window, with what
    /// it holds.
    ///
    /// This is the only enumeration of the store there is, it allocates, and it
    /// is for oracles and diagnostics. Nothing on a choice path may call it:
    /// the queries answer out of the maintained sets, which are sorted by
    /// construction, and that split is what licenses a hashed store at all.
    pub(crate) fn snapshot(&self) -> BTreeMap<Window, WindowMasks> {
        let mut windows = BTreeSet::new();
        for (&key, chunk) in &self.chunks {
            let (axis, line, index) = unpack(key);
            let mut bits = chunk.p1 | chunk.p2;
            while bits != 0 {
                let bit = bits.trailing_zeros() as i32;
                bits &= bits - 1;
                let cell = LinePos {
                    line,
                    pos: index * CHUNK_LEN + bit,
                }
                .cell(axis);
                windows.extend(
                    windows_through_indexed(cell)
                        .filter(|(window, _)| window.axis == axis)
                        .map(|(window, _)| window),
                );
            }
        }
        windows
            .into_iter()
            .map(|window| (window, self.masks(window)))
            .collect()
    }
}
