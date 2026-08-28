use std::fmt;
use std::ops::{BitXor, BitXorAssign};

use crate::board::{Board, Player};
use crate::coord::Coord;
use crate::turn::Phase;

/// The seed every key in this engine descends from.
///
/// It is `"pistol"` in ASCII followed by the key-format version. Changing it
/// renames every position in every book, net and match log ever produced, which
/// is why it is a pinned constant with its own decision line (docs/decisions.md
/// D-57) and not configuration.
pub const ZOBRIST_SEED: u64 = 0x7069_7374_6F6C_0001;

/// A 128-bit position key.
///
/// The two halves are named rather than merged into a `u128` because they are
/// used apart: the search TT indexes from the low bits and stores the high 64 as
/// its verification word (docs/decisions.md D-8).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key128 {
    low: u64,
    high: u64,
}

impl Key128 {
    /// The identity of the XOR: the key of no stones and no context.
    pub const ZERO: Key128 = Key128::from_parts(0, 0);

    /// A key from its two halves.
    pub const fn from_parts(low: u64, high: u64) -> Key128 {
        Key128 { low, high }
    }

    /// The low half — what a transposition table indexes by.
    pub const fn low(self) -> u64 {
        self.low
    }

    /// The high half — what a transposition table verifies with.
    pub const fn high(self) -> u64 {
        self.high
    }

    /// The XOR of two keys, as a `const fn`, so that composite keys can be
    /// built at compile time. [`BitXor`] is the same operation.
    pub const fn xor(self, other: Key128) -> Key128 {
        Key128 {
            low: self.low ^ other.low,
            high: self.high ^ other.high,
        }
    }
}

impl BitXor for Key128 {
    type Output = Key128;

    fn bitxor(self, other: Key128) -> Key128 {
        self.xor(other)
    }
}

impl BitXorAssign for Key128 {
    fn bitxor_assign(&mut self, other: Key128) {
        *self = self.xor(other);
    }
}

impl fmt::Display for Key128 {
    /// The 32 hex digits of the key, high half first — one reading order for a
    /// log, a fixture and an assertion message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}{:016x}", self.high, self.low)
    }
}

impl fmt::Debug for Key128 {
    /// The derived form would print two decimal `u64`s, which no reader can
    /// compare against a pinned vector or a TT dump.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key128({self})")
    }
}

/// The key of a `player` stone on `at`.
pub const fn cell_key(at: Coord, player: Player) -> Key128 {
    // The coordinates go in as their two's-complement bit patterns, which is
    // what makes the word injective over the whole addressable lattice.
    let payload =
        (player_index(player) << 32) | ((at.r as u16 as u64) << 16) | (at.q as u16 as u64);
    key_of(CELL_TAG, payload)
}

/// The key of `player` being the side to move.
pub const fn side_key(player: Player) -> Key128 {
    key_of(SIDE_TAG, player_index(player))
}

/// The key of the mover standing at `phase` of the current turn.
///
/// Both phases have a key, rather than one phase being the absence of one:
/// [`Phase::First`] is a state a position can be *set* to (docs/decisions.md
/// D-6), not a default, and a key that says nothing about it could not be told
/// apart from a key that had not been given one.
pub const fn phase_key(phase: Phase) -> Key128 {
    key_of(PHASE_TAG, phase.index() as u64)
}

/// The whole key of a position, recomputed from its stones.
///
/// This is the reference the incrementally carried key is checked against, and
/// the way a position assembled from outside gets its first key. It is linear in
/// the stones on the board; the search never calls it — it carries
/// [`crate::GameState::key`] instead, which is what the round-trip test pins it
/// against.
pub fn from_scratch_key(board: &Board, to_move: Player, phase: Phase) -> Key128 {
    let mut key = context_key(to_move, phase);
    for (at, player) in board.stones() {
        key ^= cell_key(at, player);
    }
    key
}

/// The part of a key that is not stones: whose move it is, and how far into the
/// turn they are.
///
/// Both are the state machine's own fields, so composing them on read is exact
/// by construction — there is no third copy of the side and the phase to fall
/// out of step with the two the machine already keeps (docs/decisions.md D-58).
pub(crate) const fn context_key(to_move: Player, phase: Phase) -> Key128 {
    side_key(to_move).xor(phase_key(phase))
}

/// Which family a word names. The tag is the top byte, so the families cannot
/// overlap however wide a payload grows.
const TAG_SHIFT: u32 = 56;
const CELL_TAG: u64 = 1;
const SIDE_TAG: u64 = 2;
const PHASE_TAG: u64 = 3;

/// P1 is 0 and P2 is 1 — an encoding for the key, deliberately local:
/// nothing outside this module gets a number for a player from here.
const fn player_index(player: Player) -> u64 {
    match player {
        Player::P1 => 0,
        Player::P2 => 1,
    }
}

/// The two streams of one word.
const fn key_of(tag: u64, payload: u64) -> Key128 {
    let word = (tag << TAG_SHIFT) | payload;
    Key128 {
        low: splitmix64(word ^ LOW_STREAM),
        high: splitmix64(word ^ HIGH_STREAM),
    }
}

/// The two stream seeds. The second is the first put through the mixer, so both
/// descend from [`ZOBRIST_SEED`] and neither is a hand-picked second constant.
const LOW_STREAM: u64 = ZOBRIST_SEED;
const HIGH_STREAM: u64 = splitmix64(ZOBRIST_SEED);

/// SplitMix64 (Vigna), as published: the state increment, then the two
/// multiply-xorshift rounds and the final shift.
///
/// It is a bijection on `u64`, which is the property the construction leans on —
/// distinct words cannot share a stream output, so two stones can never share
/// half a key.
const fn splitmix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
