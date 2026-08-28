// RULE9-JUSTIFICATION: the table, its epoch filter and the replacement law
// are one invariant — a stale epoch reads as absent precisely so the
// proven-retention law can hold within an epoch, and gate (d)'s value
// cross-check at two table sizes pins the whole file as one mechanism.
use pistol_core::Key128;

use crate::pn::{Value, value_of};
use crate::zone::ZoneP;

/// One stored node.
#[derive(Debug, Clone)]
pub struct Entry {
    /// The position the numbers belong to.
    pub key: Key128,
    /// The search epoch this entry belongs to; entries from earlier
    /// epochs are stale and read as absent. This is what lets one table
    /// serve many solves without an O(entries) clear between them — the
    /// clear cost is the entire reason a naive per-solve table made gate
    /// (c)'s sigma sweep infeasible at 145k solves.
    pub epoch: u32,
    /// The best-known proof number. `0` means proven.
    pub pn: u64,
    /// The best-known disproof number. `0` means disproven.
    pub dn: u64,
    /// The zone memoised for proven nodes, `None` otherwise.
    pub zone: Option<ZoneP>,
    /// The search pass that stored it.
    pub generation: u32,
}

impl Entry {
    /// The three-valued reading.
    pub fn value(&self) -> Value {
        value_of(self.pn, self.dn)
    }

    fn is_proven(&self) -> bool {
        self.pn == 0 || self.dn == 0
    }
}

/// The table. Create with a power-of-two entry count (the config validates
/// this); `SolverTT::new` panics otherwise, loudly, because a silently
/// masked index would corrupt every lookup after it.
pub struct SolverTT {
    preferred: Vec<Option<Entry>>,
    replace: Vec<Option<Entry>>,
    mask: usize,
}

impl SolverTT {
    /// # Panics
    ///
    /// If `entries` is not a power of two ≥ 2 — the index mask cannot be
    /// built for anything else, and a wrong mask is silent corruption.
    pub fn new(entries: usize) -> SolverTT {
        if entries < 2 || !entries.is_power_of_two() {
            panic!("pistol-solver invariant TT_SIZE: {entries} is not a power-of-two table size");
        }
        SolverTT {
            preferred: vec![None; entries],
            replace: vec![None; entries],
            mask: entries - 1,
        }
    }

    /// The entry for `key` from epoch `epoch`, if the table still holds a
    /// live one. Stale entries read as absent, which is the whole
    /// correctness argument for never clearing.
    ///
    /// Both slots are checked; on the (legal) case where both hold `key`,
    /// the fresher generation wins — a pure function of the contents, so the
    /// same table state always yields the same answer.
    pub fn lookup(&self, key: Key128, epoch: u32) -> Option<&Entry> {
        let index = Self::index(key, self.mask);
        fn live(entry: &Option<Entry>, key: Key128, epoch: u32) -> Option<&Entry> {
            entry.as_ref().filter(|e| e.key == key && e.epoch == epoch)
        }
        match (
            live(&self.preferred[index], key, epoch),
            live(&self.replace[index], key, epoch),
        ) {
            (Some(a), Some(b)) => Some(if a.generation >= b.generation { a } else { b }),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    /// Store `entry` under its key, honouring the replacement law.
    ///
    /// ONE DOCUMENTED EXCEPTION to the law's letter: the same-key refresh
    /// path below can overwrite a PROVEN entry from a STALE epoch with an
    /// unproven one from the live epoch. The law's substance is untouched —
    /// the stale entry already reads as absent to every lookup, so nothing
    /// the search can see has been lost — but a reader matching the law's
    /// wording alone would expect the check here, and this comment is why
    /// it is not.
    ///
    /// The two slots divide the labour: the PREFERRED slot holds the entry
    /// worth keeping (same key, empty slot, or an unproven entry an at
    /// least equally fresh one displaces), and the REPLACE slot is the
    /// always-replace spill that keeps a colliding second key findable —
    /// which is the whole reason there are two. A refresh updates whichever
    /// slot holds the key; a key nobody holds goes to an empty slot first;
    /// only when both slots hold OTHER keys does displacement decide, and
    /// the proven-never-replaced-by-unproven law outranks both.
    pub fn store(&mut self, entry: Entry) {
        let index = Self::index(entry.key, self.mask);
        if let Some(existing) = &mut self.preferred[index]
            && existing.key == entry.key
        {
            *existing = entry;
            return;
        }
        if let Some(existing) = &mut self.replace[index]
            && existing.key == entry.key
        {
            *existing = entry;
            return;
        }
        if self.preferred[index].is_none() {
            self.preferred[index] = Some(entry);
            return;
        }
        if self.replace[index].is_none() {
            self.replace[index] = Some(entry);
            return;
        }
        // Both slots hold other keys. The replace slot is the spill: it
        // takes the newcomer unless the law protects what it holds.
        if Self::may_replace(&self.replace[index], &entry) {
            self.replace[index] = Some(entry);
            return;
        }
        if Self::may_replace(&self.preferred[index], &entry) {
            self.preferred[index] = Some(entry);
        }
        // Neither slot may take it: the entry is dropped. That is eviction
        // working, not an error.
    }

    /// The law, and the epoch qualification the body carries: WITHIN AN EPOCH
    /// proven entries are never replaced by unproven ones and the preferred
    /// slot keeps the fresher generation. Across epochs the occupant is stale,
    /// invisible to `lookup`, and always replaceable (WP-1.8c §4c).
    fn may_replace(slot: &Option<Entry>, entry: &Entry) -> bool {
        match slot {
            None => true,
            Some(existing) => {
                if existing.epoch > entry.epoch {
                    return false;
                }
                // A STALE occupant is always replaceable, and this clause is
                // the whole of WP-1.8c §4c. `lookup` filters on the epoch, so a
                // stale entry is invisible to every read; protecting it under
                // the proven-retention law or the generation test therefore
                // shields nothing and only shrinks the live table, which is why
                // a later solve through the same `Solver` could turn a 714-node
                // win into an `Unknown` at a cap of 4,096. The law's substance
                // is untouched: within an epoch, proven still beats unproven.
                if existing.epoch < entry.epoch {
                    return true;
                }
                if existing.is_proven() && !entry.is_proven() {
                    return false;
                }
                entry.generation >= existing.generation
            }
        }
    }

    fn index(key: Key128, mask: usize) -> usize {
        // splitmix64 over the key's low half, fixed constants, written down.
        let mut z = key.low().wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        (z ^ (z >> 31)) as usize & mask
    }
}

#[cfg(test)]
mod tests {
    /// WP-1.8c §4c, the mechanism: a STALE occupant does not hold a slot
    /// against a live newcomer, however proven or however fresh it was.
    ///
    /// Before the epoch clause, `may_replace` refused a displacement when the
    /// occupant `is_proven()` and the newcomer was not — with no epoch guard —
    /// and refused again on the generation test. A stale PROVEN entry was then
    /// invisible to every `lookup` and unevictable by every displacement, so a
    /// `Solver`'s table filled with dead weight as it served solve after solve.
    /// MEASURED on the anchor positions: three of 85 non-answers recovered, and
    /// one position that answered `win` at 714 nodes alone had answered
    /// `unknown` at 4,096 in company.
    #[test]
    fn a_stale_occupant_never_blocks_a_live_entry() {
        let mut table = SolverTT::new(2);
        // EVERY slot of the smallest table filled by a PROVEN entry of a past
        // epoch at the freshest generation there is — the strongest occupant
        // the old law could produce. Enough keys that no slot is left empty
        // whichever index each hashes to: an empty slot is taken before
        // `may_replace` is ever consulted, and a test that left one would pass
        // without exercising the law at all (which is how this test first
        // failed to kill its own mutant).
        for low in 1u64..=16 {
            table.store(Entry {
                key: key(low),
                epoch: 1,
                pn: 0,
                dn: crate::pn::INF,
                zone: None,
                generation: u32::MAX,
            });
        }
        assert!(
            table
                .preferred
                .iter()
                .zip(&table.replace)
                .all(|(first, second)| first.is_some() && second.is_some()),
            "the table is full, so the newcomer below must DISPLACE rather than \
             find an empty slot"
        );
        // A live, UNPROVEN, generation-zero newcomer: everything the old law
        // refused a displacement for.
        let newcomer = key(17);
        table.store(Entry {
            key: newcomer,
            epoch: 2,
            pn: 1,
            dn: 1,
            zone: None,
            generation: 0,
        });
        assert!(
            table.lookup(newcomer, 2).is_some(),
            "a live entry displaces a stale one whatever the stale one holds"
        );
        // And the law itself is untouched WITHIN an epoch.
        let held = key(64);
        table.store(Entry {
            key: held,
            epoch: 2,
            pn: 0,
            dn: crate::pn::INF,
            zone: None,
            generation: 5,
        });
        for low in 65u64..=80 {
            table.store(Entry {
                key: key(low),
                epoch: 2,
                pn: 1,
                dn: 1,
                generation: 0,
                zone: None,
            });
        }
        assert!(
            table.lookup(held, 2).is_some(),
            "within an epoch a proven entry is still never replaced by an unproven one"
        );
    }

    use super::*;

    fn key(low: u64) -> Key128 {
        Key128::from_parts(low, 0)
    }

    fn unproven(low: u64, generation: u32) -> Entry {
        Entry {
            key: key(low),
            epoch: 7,
            pn: 3,
            dn: 5,
            zone: None,
            generation,
        }
    }

    fn proven(low: u64, generation: u32) -> Entry {
        Entry {
            key: key(low),
            epoch: 7,
            pn: 0,
            dn: crate::pn::INF,
            zone: Some(ZoneP::new()),
            generation,
        }
    }

    #[test]
    fn stores_and_finds_by_key() {
        let mut tt = SolverTT::new(4);
        tt.store(unproven(1, 0));
        tt.store(proven(2, 0));
        assert_eq!(tt.lookup(key(1), 7).unwrap().pn, 3);
        assert_eq!(tt.lookup(key(2), 7).unwrap().pn, 0);
        assert!(tt.lookup(key(3), 7).is_none());
    }

    #[test]
    fn colliding_keys_both_stay_findable() {
        // Two keys with the same index: preferred holds one, replace the
        // other.
        let mut tt = SolverTT::new(2);
        let a = unproven(0, 0);
        let b = unproven(u64::MAX - 1, 1); // same index under mask 1
        assert_eq!(SolverTT::index(a.key, 1), SolverTT::index(b.key, 1));
        tt.store(a);
        tt.store(b);
        assert_eq!(tt.lookup(key(0), 7).unwrap().generation, 0);
        assert_eq!(
            tt.lookup(Key128::from_parts(u64::MAX - 1, 0), 7)
                .map(|e| e.generation),
            Some(1)
        );
    }

    #[test]
    fn proven_entries_survive_unproven_pressure() {
        let mut tt = SolverTT::new(2);
        tt.store(proven(5, 1));
        // Flood the same index with unproven entries of later generations.
        for generation in 2..50u32 {
            tt.store(Entry {
                key: Key128::from_parts(u64::from(generation) * 2 + 1, 7),
                epoch: 7,
                pn: 1,
                dn: 1,
                zone: None,
                generation,
            });
        }
        assert!(
            tt.lookup(key(5), 7).is_some_and(|e| e.pn == 0),
            "a proven entry is never replaced by an unproven one"
        );
    }

    #[test]
    fn refresh_updates_in_place_without_losing_the_slot_neighbour() {
        let mut tt = SolverTT::new(2);
        let index = SolverTT::index(key(9), 1);
        tt.store(unproven(9, 0));
        tt.store(Entry {
            key: Key128::from_parts(77, 7),
            epoch: 7,
            pn: 2,
            dn: 2,
            zone: None,
            generation: 0,
        });
        // A refresh of key 9 must not disturb key 77's slot.
        let refreshed = Entry {
            key: key(9),
            epoch: 7,
            pn: 8,
            dn: 8,
            zone: None,
            generation: 1,
        };
        tt.store(refreshed);
        assert_eq!(tt.lookup(key(9), 7).unwrap().pn, 8);
        assert!(tt.lookup(Key128::from_parts(77, 7), 7).is_some());
        let _ = index;
    }

    #[test]
    #[should_panic(expected = "TT_SIZE")]
    fn non_power_of_two_sizes_are_refused() {
        SolverTT::new(3);
    }

    #[test]
    fn stale_epochs_read_as_absent() {
        let mut tt = SolverTT::new(2);
        tt.store(Entry {
            key: key(1),
            epoch: 1,
            pn: 3,
            dn: 5,
            zone: None,
            generation: 0,
        });
        tt.store(Entry {
            key: key(1),
            epoch: 2,
            pn: 9,
            dn: 9,
            zone: None,
            generation: 0,
        });
        assert!(tt.lookup(key(1), 1).is_none(), "epoch 1's entry is stale");
        assert_eq!(tt.lookup(key(1), 2).unwrap().pn, 9);
    }
}
