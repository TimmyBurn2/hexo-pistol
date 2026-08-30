use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::{BuildHasherDefault, Hasher};

use pistol_core::Axis;

use crate::window::Window;

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
pub(crate) fn window_key(window: Window) -> u64 {
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
pub(crate) struct WindowHasher(u64);

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
            "pistol-eval invariant {}: the window map hashed {} bytes rather than a u64 key",
            crate::handcrafted::EVAL_DESYNC,
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
/// Equality is the whole point of the newtype's `PartialEq`: `HashMap` compares
/// by length and per-key lookup, never by iteration order, so a map grown by a
/// game and unwound compares equal to a fresh one even though the two differ in
/// capacity and in the order they would iterate (docs/decisions.md D-498).
///
/// Nothing iterates this map on a value path. The three operations below are all
/// there are, and all three are point lookups.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct WindowMap<V> {
    entries: HashMap<u64, V, BuildHasherDefault<WindowHasher>>,
}

impl<V: Copy + Default + PartialEq> WindowMap<V> {
    /// What `window` holds, or the default if it holds nothing.
    #[inline]
    pub(crate) fn get(&self, window: Window) -> V {
        self.entries
            .get(&window_key(window))
            .copied()
            .unwrap_or_default()
    }

    /// The entry for `window`, inserted as the default if it is absent.
    #[inline]
    pub(crate) fn entry_or_default(&mut self, window: Window) -> &mut V {
        self.entries.entry(window_key(window)).or_default()
    }

    /// Replace what `window` holds, removing the entry when nothing is left.
    ///
    /// An emptied window leaves no entry behind: a window holding nothing scores
    /// nothing, and there are infinitely many of those. Preserving that is what
    /// keeps an unwound map equal to a fresh one, and it is why an absent window
    /// and an empty one are the same observation to [`WindowMap::get`].
    #[inline]
    pub(crate) fn set(&mut self, window: Window, value: V) {
        match self.entries.entry(window_key(window)) {
            Entry::Occupied(mut slot) => {
                if value == V::default() {
                    slot.remove();
                } else {
                    slot.insert(value);
                }
            }
            Entry::Vacant(slot) => {
                if value != V::default() {
                    slot.insert(value);
                }
            }
        }
    }

    /// How many windows hold anything.
    ///
    /// Test-only, with the footprint bound: the eval itself never asks, and a
    /// method the shipped path does not call is dead weight outside one.
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    /// How many buckets the table has allocated.
    ///
    /// Test-only, and the reason the footprint bound is a derivation rather
    /// than a quoted number.
    #[cfg(test)]
    pub(crate) fn capacity(&self) -> usize {
        self.entries.capacity()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use pistol_core::Coord;

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
                    (a.axis, a.start) < (b.axis, b.start),
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
    fn the_window_map_footprint_is_bounded_by_its_capacity_and_its_entry_size() {
        // The derivation, not a quoted number: hashbrown lays out one (key,
        // value) pair plus one control byte per bucket, and holds at most
        // 7/8 of its buckets live.
        const PAIR: usize = std::mem::size_of::<(u64, (u8, u8))>();
        let mut map: WindowMap<(u8, u8)> = WindowMap::default();
        assert_eq!(
            map.capacity() * (PAIR + 1),
            0,
            "a fresh map allocates nothing"
        );

        for q in 0..64i16 {
            for r in 0..64i16 {
                map.set(
                    Window {
                        axis: Axis::ConstQ,
                        start: Coord::new(q, r),
                    },
                    (1, 0),
                );
            }
        }
        let live = map.len();
        let bound = map.capacity() * (PAIR + 1);
        assert_eq!(
            live,
            64 * 64,
            "the sweep should have inserted every cell once"
        );
        assert!(
            map.capacity() >= live,
            "capacity below the live count is not a table"
        );
        assert!(
            bound <= live * 8 * (PAIR + 1),
            "footprint {bound} B exceeds the 8x-of-live ceiling the 7/8 load factor implies"
        );
    }

    #[test]
    fn an_emptied_window_leaves_no_entry_behind() {
        let mut map: WindowMap<(u8, u8)> = WindowMap::default();
        let window = Window {
            axis: Axis::ConstR,
            start: Coord::new(3, -4),
        };
        map.set(window, (1, 0));
        assert_eq!(map.len(), 1);
        map.set(window, (0, 0));
        assert_eq!(map.len(), 0, "an emptied window kept its entry");
        assert_eq!(
            map,
            WindowMap::default(),
            "an emptied map is not a fresh one"
        );
    }
}
