use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use pistol_core::{Coord, Player, Turn, canonical_form};

/// Whether a capture run memoises, within itself, the answer to a `position`
/// line it has already asked (docs/experiments/wp21_label_cache_design.md).
///
/// A MODE rather than a map: the memo is built inside `capture::run` and dies
/// with it, so no caller can hand one run another run's answers
/// (docs/decisions.md D-576).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelCache {
    /// Every asked prefix is asked of the engine.
    Off,
    /// A prefix whose `position` line this run has already asked takes the
    /// answer it already has.
    On,
}

/// What one capture run counted, cached or not.
///
/// `asks` is a COUNTER at the call to the engine, never derived from the
/// records or the memo: derived, it would be a function of the capture file,
/// which is identical cached or not, and a dead cache would read as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CaptureCounts {
    /// How many times the engine was asked.
    pub asks: u64,
    /// How many records were written.
    pub records: u64,
    /// `records - asks`.
    pub hits: u64,
    /// Misses whose sorted stone list an earlier miss already had — what a
    /// transposition-folding key would have merged.
    pub key_pos_collisions: u64,
    /// Misses whose canonical form an earlier miss already had — what a
    /// symmetry-folding key would have merged. A stone-set collision is a
    /// canonical one too, so this is never below `key_pos_collisions`.
    pub key_full_collisions: u64,
    /// The summed wall of the two folds, so their cost is MEASURED on every run
    /// rather than estimated (docs/decisions.md D-586).
    pub fold_ms: u64,
}

impl CaptureCounts {
    /// The one stdout line that records a run's mode.
    ///
    /// The capture file carries no trace of the mode by design — that is what
    /// the byte-identity criterion compares — so this line and the run log's
    /// verbatim command are the only record that a run was cached.
    pub fn line(&self, mode: LabelCache) -> String {
        match mode {
            LabelCache::Off => format!(
                "arena: label cache off: asks {} records {}",
                self.asks, self.records
            ),
            LabelCache::On => format!(
                "arena: label cache on: asks {} records {} hits {} key_pos_collisions {} \
                 key_full_collisions {} fold_ms {}",
                self.asks,
                self.records,
                self.hits,
                self.key_pos_collisions,
                self.key_full_collisions,
                self.fold_ms
            ),
        }
    }
}

/// The memo: the answers this run has, keyed on the `position` line's exact
/// bytes, and the two coarser-key sets the counters read.
///
/// Crate-private so its map's type crosses no public signature.
pub(crate) struct Memo {
    mode: LabelCache,
    answers: BTreeMap<String, (String, String)>,
    key_pos: BTreeSet<Vec<(Coord, Player)>>,
    key_full: BTreeSet<Vec<(Coord, Player)>>,
    counts: CaptureCounts,
    fold: Duration,
}

impl Memo {
    pub(crate) fn new(mode: LabelCache) -> Memo {
        Memo {
            mode,
            answers: BTreeMap::new(),
            key_pos: BTreeSet::new(),
            key_full: BTreeSet::new(),
            counts: CaptureCounts::default(),
            fold: Duration::ZERO,
        }
    }

    /// The pair an identical `position` line already produced, if the mode
    /// keeps one. `Off` never has one, which is what makes `Off` the pass that
    /// exists today.
    pub(crate) fn lookup(&self, position: &str) -> Option<(String, String)> {
        match self.mode {
            LabelCache::Off => None,
            LabelCache::On => self.answers.get(position).cloned(),
        }
    }

    /// Counted immediately before the call to the engine.
    pub(crate) fn asked(&mut self) {
        self.counts.asks += 1;
    }

    /// Counted once per record written, hit or miss.
    pub(crate) fn recorded(&mut self) {
        self.counts.records += 1;
    }

    /// A miss's answer, after `normalise`, under its `position`; and the two
    /// folds of the prefix that produced it, for the counters.
    ///
    /// Under `Off` nothing is kept and nothing is folded: the counters exist to
    /// read a CACHED tranche against the book's own floor, and the uncached
    /// pass pays nothing for them.
    pub(crate) fn insert(&mut self, position: &str, prefix: &[Turn], pair: (String, String)) {
        if self.mode == LabelCache::Off {
            return;
        }
        self.answers.insert(position.to_string(), pair);
        let started = Instant::now();
        let mut stones = stones_of(prefix);
        stones.sort_unstable();
        if !self.key_pos.insert(stones.clone()) {
            self.counts.key_pos_collisions += 1;
        }
        if !self.key_full.insert(canonical_form(&stones)) {
            self.counts.key_full_collisions += 1;
        }
        self.fold += started.elapsed();
    }

    /// The counts, with the derived fields filled in.
    pub(crate) fn into_counts(self) -> CaptureCounts {
        CaptureCounts {
            hits: self.counts.records.saturating_sub(self.counts.asks),
            fold_ms: u64::try_from(self.fold.as_millis()).unwrap_or(u64::MAX),
            ..self.counts
        }
    }
}

/// The stones a prefix places: turn `i`'s cells under player `i mod 2`.
///
/// Built from the turns and not from a replay, because `asked_prefixes` has
/// already proved the game legal and a second replay per miss would be the
/// quadratic cost the design declines (docs/decisions.md D-586).
fn stones_of(prefix: &[Turn]) -> Vec<(Coord, Player)> {
    let mut stones = Vec::with_capacity(prefix.len() * 2);
    for (index, turn) in prefix.iter().enumerate() {
        let player = if index.is_multiple_of(2) {
            Player::P1
        } else {
            Player::P2
        };
        stones.push((turn.first(), player));
        if let Some(second) = turn.second() {
            stones.push((second, player));
        }
    }
    stones
}
