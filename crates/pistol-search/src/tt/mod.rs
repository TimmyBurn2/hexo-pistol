//! The transposition table: what the search already knows about a position.
//!
//! The table is indexed by the low bits of the 128-bit position key and verifies
//! with the high 64, which is ample for a heuristic table — the solver's
//! proof-carrying table is a different one and keeps all 128
//! (docs/decisions.md D-8, D-57). One probe reads one bucket of
//! [`BUCKET_ENTRIES`] packed entries and nothing else.
//!
//! # Determinism
//!
//! Replacement is a pure function of the bucket's contents and the current
//! generation: the shallowest entry of the oldest generation goes, ties broken
//! by slot order. No clock, no random victim, no thread (CLAUDE.md rule 4,
//! docs/decisions.md D-7). Two identical store sequences leave identical tables,
//! which `tt_replacement_prefers_depth_and_is_the_same_every_run` pins.
//!
//! # Sizing
//!
//! The bucket count is the largest power of two whose buckets fit inside
//! `tt_bytes`, because the index is a mask. A bucket is 96 bytes and a byte
//! budget is a power of two, so the two never divide evenly and the table takes
//! three quarters of the budget in the worst case; [`Table::bytes`] reports what
//! it took. That is a ceiling honoured, not a stated value rounded — the config
//! rule that `search.tt_bytes` is a power of two stays exactly what
//! docs/decisions.md D-19 made it, and the number of entries this yields is the
//! same one a padded 32-byte entry would have given (D-75).

pub mod entry;

use std::fmt;

use pistol_core::Key128;

use crate::error::SearchError;
use crate::score;
use entry::{EMPTY, Entry, GENERATIONS};

pub use entry::{Bound, ENTRY_BYTES, Record, TT_FIELD_OUT_OF_RANGE};

/// Entries per bucket. A probe reads one bucket and nothing else.
pub const BUCKET_ENTRIES: usize = 4;

/// Bytes per bucket.
pub const BUCKET_BYTES: u64 = (ENTRY_BYTES * BUCKET_ENTRIES) as u64;

/// The transposition table.
pub struct Table {
    buckets: Vec<[Entry; BUCKET_ENTRIES]>,
    /// `buckets.len() - 1`; the count is a power of two so the index is a mask.
    mask: u64,
    generation: u8,
    used: u64,
}

impl fmt::Debug for Table {
    /// The shape and the fill, never the contents: a derived `Debug` on a
    /// quarter-gigabyte table is not a diagnostic, it is a denial of service on
    /// whoever printed it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Table")
            .field("buckets", &self.buckets.len())
            .field("bytes", &self.bytes())
            .field("generation", &self.generation)
            .field("hashfull_permille", &self.hashfull_permille())
            .finish()
    }
}

impl Table {
    /// Build a table that fits inside `tt_bytes`.
    ///
    /// Refuses a budget too small to hold a single bucket, naming the config key
    /// an operator edits. It does not restate the engine's power-of-two rule:
    /// that one is config policy (docs/decisions.md D-19), and this constructor
    /// answers only for what it can build.
    pub fn new(tt_bytes: u64) -> Result<Table, SearchError> {
        if tt_bytes < BUCKET_BYTES {
            return Err(SearchError::params(
                "search.tt_bytes",
                format!("must hold at least one {BUCKET_BYTES}-byte bucket, got {tt_bytes}"),
            ));
        }
        let count = previous_power_of_two(tt_bytes / BUCKET_BYTES);
        let count = usize::try_from(count).map_err(|_| {
            SearchError::params(
                "search.tt_bytes",
                format!("{tt_bytes} bytes is more table than this machine can address"),
            )
        })?;
        Ok(Table {
            buckets: vec![[EMPTY; BUCKET_ENTRIES]; count],
            mask: count as u64 - 1,
            generation: 0,
            used: 0,
        })
    }

    /// How many buckets the table holds. A power of two.
    pub fn buckets(&self) -> usize {
        self.buckets.len()
    }

    /// How many bytes the table actually took, which is never more than it was
    /// given.
    pub fn bytes(&self) -> u64 {
        self.buckets.len() as u64 * BUCKET_BYTES
    }

    /// Entries in use, in parts per thousand.
    pub fn hashfull_permille(&self) -> u32 {
        let entries = self.buckets.len() as u64 * BUCKET_ENTRIES as u64;
        u32::try_from(self.used * 1000 / entries).unwrap_or(1000)
    }

    /// Forget everything. This is what a new game does: a table carried across
    /// games would let one search's node count depend on another's
    /// (docs/decisions.md D-7).
    pub fn clear(&mut self) {
        self.buckets.fill([EMPTY; BUCKET_ENTRIES]);
        self.generation = 0;
        self.used = 0;
    }

    /// Begin a new search. Entries from earlier searches stay, and are the first
    /// to be evicted.
    pub fn new_generation(&mut self) {
        self.generation = (self.generation + 1) % GENERATIONS;
    }

    /// What the table knows about this position, with any mate distance
    /// re-based onto the root.
    pub fn probe(&self, key: Key128, turns_from_root: u32) -> Option<Record> {
        let bucket = &self.buckets[self.index(key)];
        let entry = bucket.iter().find(|entry| entry.matches(key.high()))?;
        let mut record = entry.record();
        record.score = score::from_table(record.score, turns_from_root);
        Some(record)
    }

    /// Write what this search learned about a position.
    pub fn store(&mut self, key: Key128, turns_from_root: u32, record: Record) {
        let packed = Entry::packed(
            key.high(),
            record,
            score::to_table(record.score, turns_from_root),
            self.generation,
        );
        let index = self.index(key);
        let slot = self.victim(index, key.high());
        if self.buckets[index][slot].is_empty() {
            self.used += 1;
        }
        self.buckets[index][slot] = packed;
    }

    /// Which slot of a bucket this key belongs in.
    ///
    /// The position's own entry if it already has one; otherwise the shallowest
    /// entry of the oldest generation, ties broken by slot order. An empty slot
    /// wins that comparison on its own terms — depth zero — so it needs no
    /// special case.
    fn victim(&self, index: usize, verification: u64) -> usize {
        let bucket = &self.buckets[index];
        if let Some(slot) = bucket.iter().position(|entry| entry.matches(verification)) {
            return slot;
        }
        let mut victim = 0;
        for slot in 1..BUCKET_ENTRIES {
            if self.rank(bucket[slot]) < self.rank(bucket[victim]) {
                victim = slot;
            }
        }
        victim
    }

    /// How much an entry is worth keeping: this generation first, then depth.
    /// Smallest goes.
    fn rank(&self, entry: Entry) -> (u8, u8) {
        (
            u8::from(entry.age() == self.generation),
            entry.depth_plies(),
        )
    }

    fn index(&self, key: Key128) -> usize {
        (key.low() & self.mask) as usize
    }
}

/// The largest power of two at most `value`, which is at least one because the
/// caller has already refused a budget below one bucket.
fn previous_power_of_two(value: u64) -> u64 {
    1u64 << (u64::BITS - 1 - value.leading_zeros())
}
