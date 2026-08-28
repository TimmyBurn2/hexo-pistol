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
        // Asked for, not taken: the plain `vec!` form ends a table this machine
        // cannot hold with `handle_alloc_error`, which aborts the process — no
        // name, no key, no line an operator could act on, and a core dump where
        // CLAUDE.md rule 3 requires a refusal. How much memory a machine has is
        // not something config validation may ask (docs/decisions.md D-21), so
        // this is the seam that has to answer for it.
        let mut buckets: Vec<[Entry; BUCKET_ENTRIES]> = Vec::new();
        buckets.try_reserve_exact(count).map_err(|_| {
            SearchError::params(
                "search.tt_bytes",
                format!(
                    "{tt_bytes} bytes is more table than this machine can allocate ({count} \
                     buckets of {BUCKET_BYTES} bytes)"
                ),
            )
        })?;
        buckets.resize(count, [EMPTY; BUCKET_ENTRIES]);
        Ok(Table {
            buckets,
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
    ///
    /// A quiescence-regime entry (`Entry::from_quiescence`) is never
    /// returned: a full-width caller (the only caller `Table::probe` has —
    /// `crate::quiescence`'s own nodes never probe, docs/wp16_quiescence_design.md
    /// §6 item 5) treats a hit on one exactly as if this method had answered
    /// `None` — no cutoff, no move-ordering hint. Centralised here rather than
    /// at each call site so there is exactly one place this rule is stated.
    pub fn probe(&self, key: Key128, turns_from_root: u32) -> Option<Record> {
        let bucket = &self.buckets[self.index(key)];
        let entry = bucket
            .iter()
            .find(|entry| entry.matches(key.high()) && !entry.from_quiescence())?;
        let mut record = entry.record();
        record.score = score::from_table(record.score, turns_from_root);
        Some(record)
    }

    /// Write what this search learned about a position.
    ///
    /// A quiescence-regime record (`record.from_quiescence`) never evicts an
    /// existing full-width entry — it may fill an empty slot or replace an
    /// existing quiescence entry, using the ordinary victim rule below, but a
    /// store that would otherwise land on a non-empty, non-quiescence slot is
    /// silently declined instead (docs/wp16_quiescence_design.md §6 item 3,
    /// WP-1.6 D-390/D-392/D-393). A full-width store is never declined by
    /// this rule — it may evict anything, exactly as before this field
    /// existed.
    pub fn store(&mut self, key: Key128, turns_from_root: u32, record: Record) {
        let index = self.index(key);
        let slot = self.victim(index, key.high());
        let existing = self.buckets[index][slot];
        if record.from_quiescence && !existing.is_empty() && !existing.from_quiescence() {
            return;
        }
        let packed = Entry::packed(
            key.high(),
            record,
            score::to_table(record.score, turns_from_root),
            self.generation,
        );
        if existing.is_empty() {
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
