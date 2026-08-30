use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};

use pistol_core::{Axis, Coord, Player};

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

/// A [`Window`] packed into a `u64` key.
///
/// The three fields occupy disjoint bit ranges, so the packing is INJECTIVE over
/// every addressable window — which is the property this map depends on, since a
/// collision would merge two windows' counts into one entry and evaluate a
/// position wrongly with no panic and no symptom.
///
/// It is additionally order-preserving with respect to [`Window`]'s derived
/// `Ord`: `(q ^ 0x8000)` is the standard order-preserving map from `i16` to
/// `u16`, so the key orders by `(axis, q, r)` exactly as the window does. A hash
/// map does not need that, and nothing here relies on it; it is pinned so the key
/// stays reusable if an ordered container ever returns (docs/decisions.md D-225).
#[inline]
fn window_key(window: Window) -> u64 {
    let axis = match window.axis {
        Axis::ConstQ => 0u64,
        Axis::ConstR => 1,
        Axis::ConstS => 2,
    };
    let q = u64::from((window.start.q as u16) ^ 0x8000);
    let r = u64::from((window.start.r as u16) ^ 0x8000);
    axis << 32 | q << 16 | r
}

/// A multiply-xor hash over the one key shape this map holds.
///
/// SEEDLESS BY CONSTRUCTION — there is no `RandomState` and nothing derived from
/// the environment — which is what keeps a hashed store clear of the determinism
/// law: two runs of the same position hash identically on any machine
/// (CLAUDE.md rule 4, docs/decisions.md D-32, D-498).
#[derive(Default)]
struct WindowHasher(u64);

impl Hasher for WindowHasher {
    #[inline]
    fn write_u64(&mut self, value: u64) {
        // The keys are dense and structured — neighbouring windows differ in the
        // low bits — so the top of a Fibonacci-ratio multiply is folded down to
        // where the table reads.
        let mixed = value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        self.0 = mixed ^ (mixed >> 32);
    }

    fn write(&mut self, bytes: &[u8]) {
        panic!(
            "pistol-eval invariant {EVAL_DESYNC}: the window map hashed {} bytes rather than a \
             u64 key",
            bytes.len()
        );
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

/// The windows holding at least one stone, and what they hold.
///
/// Keyed by [`window_key`] and hashed by [`WindowHasher`], and it lives in this
/// file rather than behind a module boundary for a MEASURED reason — the entry in
/// `docs/rule9_justifications.md` names the run.
///
/// Nothing iterates it on a value path. `HashMap`'s own `PartialEq` compares by
/// length and per-key lookup rather than by iteration order, so a map a game grew
/// and unwound compares equal to a fresh one though the two differ in capacity
/// and in the order they would iterate (docs/decisions.md D-498).
type WindowMap = HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>;

/// The handcrafted line-window evaluation.
///
/// Equality is over the whole carried state, not just the score: two evals are
/// equal exactly when they hold the same stones. That is what lets a test assert
/// that an unwound eval is indistinguishable from a fresh one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandcraftedV0 {
    weights: Weights,
    /// The windows holding at least one stone, and what they hold. Never
    /// iterated on a value path, which is what keeps iteration order out of
    /// every answer this crate gives.
    windows: WindowMap,
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
            windows: WindowMap::default(),
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
                let counts = self.windows.entry(window_key(window)).or_default();
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
            // ONE probe, not two: the entry resolves the slot once and every
            // edit below goes through it, where a lookup then a store would hash
            // each window twice on a path a search walks for every stone it
            // unwinds. Measured, and the reason this is not spelled as a get and
            // a set. NO TEST GUARDS THIS SHAPE — a two-probe rewrite passes the
            // whole suite, and only the rule-5 bench catches it.
            let (before, after) = {
                let Entry::Occupied(mut slot) = self.windows.entry(window_key(window)) else {
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
                // nothing scores nothing, and there are infinitely many of
                // those. It is also what makes an absent window and an emptied
                // one ONE observation to this map, which `delta`'s equivalence
                // rests on and which the desync check above fires against.
                if after.total() == 0 {
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
    ///   i64's edge: |p1_score| <= stones x [`crate::WINDOWS_PER_CELL`] x
    ///   [`EVAL_MAX`] — even the full i16 lattice is ~1.2e15, so neither path
    ///   can trap where the other doesn't.
    ///
    /// The `before` values are equal too: the entry `apply` reads through
    /// answers the same counts the lookup here answers — an absent window and an
    /// emptied one are ONE observation to this map, because `undo` removes an
    /// entry it empties — and the empty entry `apply` inserts is removed again
    /// by `undo`, so the roundtrip leaves no residue for this body to miss. This
    /// sentence names the operations because D-214 accepted the equivalence in
    /// terms of the two it replaced, and an argument whose sites no longer exist
    /// is not checkable.
    ///
    /// The full-window check mirrors `apply`'s, so an impossible stone panics
    /// with the same [`EVAL_DESYNC`] token on the same first window instead of
    /// returning a number the default would refuse — the token is shared, the
    /// post-panic state deliberately not (this body has mutated nothing).
    fn delta(&mut self, at: Coord, player: Player) -> i32 {
        let mut diff = 0i64;
        for window in windows_through(at) {
            let before = self
                .windows
                .get(&window_key(window))
                .copied()
                .unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::WINDOWS_PER_CELL;

    use super::*;

    /// The coordinates a sweep visits: both ends of the lattice, the sign
    /// boundary, and a small interior run.
    const SWEEP: [i16; 9] = [
        i16::MIN,
        i16::MIN + 1,
        -257,
        -1,
        0,
        1,
        257,
        i16::MAX - 1,
        i16::MAX,
    ];

    fn swept_windows() -> Vec<Window> {
        let mut windows = Vec::new();
        for axis in Axis::ALL {
            for q in SWEEP {
                for r in SWEEP {
                    windows.push(Window {
                        axis,
                        start: Coord::new(q, r),
                    });
                }
            }
        }
        windows
    }

    /// A weight table for the tests below, parsed rather than loaded: these
    /// tests are about what the store does under `apply` and `undo`, and reading
    /// a path would make them about the deployment layout as well.
    ///
    /// The values DELIBERATELY differ from `configs/eval_v0_weights.toml`, whose
    /// table is operator-confirmed: nothing here reads a weight, and a second
    /// unmarked copy of confirmed numbers inside `src/` is one a retune would
    /// silently leave behind (the same reason `eval_delta_tests`' own fixture
    /// differs).
    fn test_weights() -> Weights {
        Weights::parse(
            "schema_version = 1\n\
             backend = \"handcrafted_v0\"\n\
             [table]\n\
             1 = 1\n\
             2 = 3\n\
             3 = 7\n\
             4 = 15\n\
             5 = 31\n",
        )
        .expect("the table above is well formed")
    }

    #[test]
    fn a_packed_key_never_collides_for_two_distinct_windows() {
        let windows = swept_windows();
        let keys: BTreeSet<u64> = windows.iter().map(|&w| window_key(w)).collect();
        assert_eq!(
            keys.len(),
            windows.len(),
            "two distinct windows share a key, which would merge their counts"
        );
    }

    #[test]
    fn a_packed_key_orders_windows_the_way_the_window_type_does() {
        let windows = swept_windows();
        for &a in &windows {
            for &b in &windows {
                assert_eq!(
                    window_key(a) < window_key(b),
                    a < b,
                    "the key orders {a:?} against {b:?} differently from the window"
                );
            }
        }
    }

    #[test]
    fn the_window_hasher_answers_a_fixed_digest_for_a_fixed_key() {
        // GOLDEN, and golden on purpose: an in-process "two fresh hashers
        // agree" check passes for a hasher seeded once per process from a
        // clock or the environment, so it would pin nothing. These digests
        // are the seedless function's own output, and a seed of any kind
        // moves them.
        const GOLDEN: [(u64, u64); 4] = [
            (0x0000_0000_0000_0000, 0x0000_0000_0000_0000),
            (0x0000_0000_0000_0001, 0x9e37_79b9_e17d_05ac),
            (0xffff_ffff_ffff_ffff, 0x61c8_8646_e17d_05ad),
            (0x9e37_79b9_7f4a_7c15, 0xdf44_2d22_110c_749b),
        ];
        for (key, expected) in GOLDEN {
            let mut hasher = WindowHasher::default();
            hasher.write_u64(key);
            assert_eq!(
                hasher.finish(),
                expected,
                "the hasher moved for key {key:#018x}, so it is not the seedless function"
            );
        }
    }

    #[test]
    #[should_panic(expected = "EVAL_DESYNC")]
    fn the_window_hasher_refuses_a_key_that_is_not_a_u64() {
        WindowHasher::default().write(&[0, 1, 2]);
    }

    #[test]
    fn an_emptied_window_leaves_no_entry_behind() {
        // Driven through the shipped `apply` and `undo` rather than through the
        // map directly: the emptied-entry rule lives in `undo`, so a test that
        // inserted and removed entries itself would pin its own arithmetic
        // instead of the rule that `delta`'s equivalence argument rests on.
        let mut eval = HandcraftedV0::new(test_weights());
        let at = Coord::new(3, -4);
        eval.apply(at, Player::P1);
        assert_eq!(
            eval.windows.len(),
            WINDOWS_PER_CELL,
            "one stone should have entered every window through its cell"
        );
        eval.undo(at, Player::P1);
        assert!(
            eval.windows.is_empty(),
            "an emptied window kept its entry, so an absent window and an emptied one are two \
             observations to this map rather than one"
        );
    }

    #[test]
    fn the_window_map_holds_its_peak_capacity_after_every_entry_is_gone() {
        // THE NON-VACUOUS FOOTPRINT PROPERTY. `capacity() >= len()` is a std
        // invariant and asserting it pins nothing; what this store actually
        // does — and what the selection record was measured against — is
        // decline to shrink, so its footprint is bounded by the historical PEAK
        // and never by the live count. A map that started shrinking on removal
        // would fail here.
        let mut eval = HandcraftedV0::new(test_weights());
        assert_eq!(eval.windows.capacity(), 0, "a fresh map allocates nothing");

        let mut stones = Vec::new();
        for q in 0..8i16 {
            for r in 0..8i16 {
                let at = Coord::new(q, r);
                let player = if (q + r) % 2 == 0 {
                    Player::P1
                } else {
                    Player::P2
                };
                eval.apply(at, player);
                stones.push((at, player));
            }
        }
        // The peak is checked against an INDEPENDENTLY DERIVED count rather
        // than a floor: the same cells enumerated into a set say exactly how
        // many distinct windows the sweep must have touched, so a sweep that
        // quietly lost entries fails here instead of clearing a bound by
        // twentyfold. The predecessor asserted a literal `64 * 64`; this asks
        // the enumeration rather than a number that has to be maintained.
        let expected: BTreeSet<u64> = stones
            .iter()
            .flat_map(|&(at, _)| windows_through(at))
            .map(window_key)
            .collect();
        let live = eval.windows.len();
        assert_eq!(
            live,
            expected.len(),
            "the sweep touched {} distinct windows and the map holds {live}",
            expected.len()
        );

        while let Some((at, player)) = stones.pop() {
            eval.undo(at, player);
        }
        assert!(eval.windows.is_empty(), "every entry was removed");
        // NOT `capacity() == peak`: `capacity()` answers how many more fit
        // before a reallocation, and it drops as removals leave tombstones
        // behind. The property that matters is that the ALLOCATION does not
        // shrink to fit — an emptied map still has room for everything it ever
        // held, so the footprint is bounded by the peak and never by `len`. The
        // bound in bytes is that capacity times one `(u64, Counts)` pair plus a
        // control byte: derived, and never asserted as a number.
        assert!(
            eval.windows.capacity() >= live,
            "an emptied map kept room for only {} of the {live} windows it held, so it shrank to \
             fit and a peak-derived bound no longer holds",
            eval.windows.capacity()
        );
    }
}
