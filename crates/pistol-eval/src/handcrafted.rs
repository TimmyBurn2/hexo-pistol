//! `handcrafted_v0`: three axes, length-six windows, one integer table.
//!
//! The whole evaluation is one sum. Every window that holds a stone contributes
//! the table entry for how many stones the owning player holds in it, positive
//! for P1 and negative for P2; a window holding both players is *dead* and
//! contributes nothing, because neither side can complete it (docs/decisions.md
//! D-11).
//!
//! The sum is carried, not computed. Placing one stone changes exactly the
//! windows through that cell — at most [`WINDOWS_PER_CELL`](crate::WINDOWS_PER_CELL)
//! of them — so an
//! update is eighteen entries in an ordered map, and taking the stone back is the
//! same eighteen the other way. The board is unbounded and only a window holding
//! a stone can score, so a window exists in the map exactly while it holds one:
//! there is nothing to iterate that is not there, and no bookkeeping over empty
//! space (docs/decisions.md D-62).
//!
//! This is the cheapest evaluation that produces sane play at depth four to six.
//! Sophistication is Stage 2's job, under this same trait.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;

use pistol_core::{Coord, Player};

use crate::eval::{EVAL_MAX, Eval};
use crate::weights::Weights;
use crate::window::{WINDOW_LEN, Window, windows_through};

/// [`WINDOW_LEN`] as a stone count, which is what a full window holds.
const WINDOW_LEN_STONES: u8 = WINDOW_LEN as u8;

const _: () = assert!(
    WINDOW_LEN_STONES as u32 == WINDOW_LEN,
    "a window's cells must be countable in a byte"
);

/// Named invariant: the eval was told about a stone that contradicts what it
/// already holds — a cell applied twice, or a stone taken back that was never
/// applied.
///
/// The token appears verbatim in the panic message, so a test can pin it and a
/// log can be grepped for it. Reaching it means the caller's board and this
/// eval have drifted apart, which is a bug in pistol and not an answer to a
/// question anyone asked (CLAUDE.md rule 3, and the same argument as
/// docs/decisions.md D-45).
pub const EVAL_DESYNC: &str = "EVAL_DESYNC";

/// How many stones of each player a window holds.
///
/// A window has [`WINDOW_LEN`] cells, so both counts fit in a byte many times
/// over and their sum never exceeds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Counts {
    p1: u8,
    p2: u8,
}

impl Counts {
    /// How many stones of `player` this window holds.
    fn of(self, player: Player) -> u8 {
        match player {
            Player::P1 => self.p1,
            Player::P2 => self.p2,
        }
    }

    /// How many stones it holds in total.
    fn total(self) -> u8 {
        self.p1 + self.p2
    }

    /// Whether it holds none at all, and so is not worth an entry.
    fn is_empty(self) -> bool {
        self.total() == 0
    }

    /// Add one stone of `player`.
    fn add(&mut self, player: Player) {
        match player {
            Player::P1 => self.p1 += 1,
            Player::P2 => self.p2 += 1,
        }
    }

    /// Take one stone of `player` back.
    fn remove(&mut self, player: Player) {
        match player {
            Player::P1 => self.p1 -= 1,
            Player::P2 => self.p2 -= 1,
        }
    }
}

/// The handcrafted line-window evaluation.
///
/// Equality is over the whole carried state, not just the score: two evals are
/// equal exactly when they hold the same stones. That is what lets a test assert
/// that an unwound eval is indistinguishable from a fresh one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandcraftedV0 {
    weights: Weights,
    /// The windows holding at least one stone, and what they hold. Ordered, so
    /// nothing in this crate can make a value depend on iteration order.
    windows: BTreeMap<Window, Counts>,
    /// The sum of every window's contribution, P1-relative.
    ///
    /// Widened past the eval band on purpose: a position with thousands of
    /// stones sums far past [`EVAL_MAX`] before [`HandcraftedV0::value`] clamps
    /// it, and an intermediate that overflowed would be a wrong answer rather
    /// than a saturated one.
    p1_score: i64,
}

impl HandcraftedV0 {
    /// An evaluation of the empty position, reading `weights`.
    pub fn new(weights: Weights) -> Self {
        HandcraftedV0 {
            weights,
            windows: BTreeMap::new(),
            p1_score: 0,
        }
    }

    /// The table this evaluation reads.
    pub fn weights(&self) -> &Weights {
        &self.weights
    }

    /// What one window contributes to P1: the owner's table entry, signed,
    /// and nothing at all if both players are in it (the window is dead) or
    /// neither is.
    fn contribution(&self, counts: Counts) -> i64 {
        match (counts.p1, counts.p2) {
            (0, 0) => 0,
            (p1, 0) => i64::from(self.weights.window_value(p1)),
            (0, p2) => -i64::from(self.weights.window_value(p2)),
            _ => 0,
        }
    }
}

impl Eval for HandcraftedV0 {
    fn apply(&mut self, at: Coord, player: Player) {
        for window in windows_through(at) {
            // The map borrow ends with the block, so the score update below can
            // read the weights on the same `self`.
            let (before, after) = {
                let counts = self.windows.entry(window).or_default();
                let before = *counts;
                if before.total() >= WINDOW_LEN_STONES {
                    desync(format_args!(
                        "{player} stone on {at} would make {} stones in the {WINDOW_LEN}-cell \
                         window at {} along {:?}",
                        u32::from(before.total()) + 1,
                        window.start,
                        window.axis
                    ));
                }
                counts.add(player);
                (before, *counts)
            };
            self.p1_score += self.contribution(after) - self.contribution(before);
        }
    }

    fn undo(&mut self, at: Coord, player: Player) {
        for window in windows_through(at) {
            let (before, after) = {
                let Entry::Occupied(mut slot) = self.windows.entry(window) else {
                    desync(format_args!(
                        "{player} stone taken off {at}, but the window at {} along {:?} holds \
                         nothing",
                        window.start, window.axis
                    ));
                };
                let before = *slot.get();
                if before.of(player) == 0 {
                    desync(format_args!(
                        "{player} stone taken off {at}, but the window at {} along {:?} holds no \
                         {player} stone",
                        window.start, window.axis
                    ));
                }
                slot.get_mut().remove(player);
                let after = *slot.get();
                // An emptied window leaves no entry behind: a window holding
                // nothing scores nothing, and there are infinitely many of those.
                if after.is_empty() {
                    slot.remove();
                }
                (before, after)
            };
            self.p1_score += self.contribution(after) - self.contribution(before);
        }
    }

    fn value(&self, side_to_move: Player) -> i32 {
        let band = i64::from(EVAL_MAX);
        let clamped = i32::try_from(self.p1_score.clamp(-band, band))
            .expect("the clamp keeps the score inside the eval band");
        match side_to_move {
            Player::P1 => clamped,
            Player::P2 => -clamped,
        }
    }

    /// The fast path move ordering runs on: the roundtrip's answer WITHOUT the
    /// roundtrip — no map surgery, no entry inserted or removed, nothing to
    /// undo (docs/decisions.md D-110, licensed by D-192's H1 at 76.27%).
    ///
    /// # Equivalence, term by term (the oracle test pins this; D-214)
    ///
    /// [`Eval::apply`] folds `contribution(after) - contribution(before)` into
    /// `p1_score` for every window of `windows_through(at)`; `value(player)`
    /// then clamps the sum and negates for P2; [`Eval::undo`] reverses. This
    /// body sums the same terms over the same iterator into a local `diff` and
    /// clamps `p1_score + diff` once — the same single end-of-path clamp
    /// `value` performs, there being no intermediate saturation in either
    /// path. Two premises make the sums equal rather than merely similar:
    ///
    /// - **Distinctness.** `windows_through` yields pairwise-distinct windows
    ///   (pinned in `eval_window_tests`): `apply` reads each `before` from a
    ///   map its earlier iterations already mutated, this body reads every
    ///   `before` from the unmutated map, and the two agree only because no
    ///   window repeats within one pass.
    /// - **Associativity, unconditionally.** Stepwise and summed accumulation
    ///   differ only under intermediate overflow, and the sum sits far from
    ///   i64's edge: |p1_score| <= stones x [`WINDOWS_PER_CELL`]
    ///   (crate::WINDOWS_PER_CELL) x [`EVAL_MAX`] — even the full i16 lattice
    ///   is ~1.2e15, so neither path can trap where the other doesn't.
    ///
    /// The `before` values are equal too: `entry().or_default()` reads the
    /// same counts `get().copied().unwrap_or_default()` reads, and the empty
    /// entry `apply` inserts is removed again by `undo`, so the roundtrip
    /// leaves no residue for this body to miss.
    ///
    /// The full-window check mirrors `apply`'s, so an impossible stone panics
    /// with the same [`EVAL_DESYNC`] token on the same first window instead of
    /// returning a number the default would refuse — the token is shared, the
    /// post-panic state deliberately not (this body has mutated nothing).
    fn delta(&mut self, at: Coord, player: Player) -> i32 {
        let mut diff = 0i64;
        for window in windows_through(at) {
            let before = self.windows.get(&window).copied().unwrap_or_default();
            if before.total() >= WINDOW_LEN_STONES {
                desync(format_args!(
                    "a hypothetical {player} stone on {at} would make {} stones in the \
                     {WINDOW_LEN}-cell window at {} along {:?}",
                    u32::from(before.total()) + 1,
                    window.start,
                    window.axis
                ));
            }
            let mut after = before;
            after.add(player);
            diff += self.contribution(after) - self.contribution(before);
        }
        let band = i64::from(EVAL_MAX);
        let clamped = i32::try_from((self.p1_score + diff).clamp(-band, band))
            .expect("the clamp keeps the score inside the eval band");
        match player {
            Player::P1 => clamped,
            Player::P2 => -clamped,
        }
    }
}

/// Report a caller whose stones contradict what this eval holds, loudly.
#[cold]
#[inline(never)]
fn desync(detail: fmt::Arguments<'_>) -> ! {
    panic!("pistol-eval invariant {EVAL_DESYNC}: {detail}");
}
