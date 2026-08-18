//! Counting DISTINCT games (CLAUDE.md rule 6).
//!
//! The key is the canonical form of the ORDERED TURN SEQUENCE under the twelve
//! lattice symmetries, computed in pistol-core because it is lattice geometry
//! and geometry lives there (rule 2, docs/decisions.md D-137).
//!
//! It is deliberately not a position's canonical form. `canonical_form` folds a
//! stone multiset and says so — play order is not part of the identity it
//! computes — so keying a GAME on it would merge two different games that
//! reached the same stones, understating distinct-n. That is the opposite of
//! the error symmetry folding exists to prevent (docs/decisions.md D-163).
//!
//! # What it does not catch
//!
//! It has no false positives and it does have false negatives. D-7's final
//! tie-break is lexicographic by `(q, r)` and is therefore not
//! symmetry-invariant, so two mirrored openings usually do not produce mirrored
//! games — D-137 recorded the same thing. Games this misses are strongly
//! correlated samples rather than identical ones, so `distinct_n` OVER-counts
//! and the number is a bound rather than a census. Said here rather than left
//! for a reader to discover.
//!
//! With two deterministic engines at a fixed budget and identical
//! configurations, both games of a pair are the same game move for move, so
//! `distinct_n` collapses to the opening count. That is a finding the report is
//! required to surface, not a bug.

use std::collections::BTreeMap;

use pistol_core::{Turn, canonical_sequence};

use crate::record::GameRecord;

/// For each game, the index of the lowest-indexed game it duplicates.
///
/// `None` for a game that is the first of its kind. Input must be in index
/// order, which is what makes "lowest-indexed" well defined and makes the
/// answer independent of the order games happened to finish in.
pub fn duplicates(records: &[GameRecord]) -> Vec<Option<usize>> {
    let mut first_seen: BTreeMap<Vec<Turn>, usize> = BTreeMap::new();
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        let key = canonical_sequence(&record.moves);
        match first_seen.get(&key) {
            Some(&first) => out.push(Some(first)),
            None => {
                first_seen.insert(key, record.index);
                out.push(None);
            }
        }
    }
    out
}

/// How many games in this run were not duplicates of an earlier one.
pub fn distinct_count(duplicates: &[Option<usize>]) -> usize {
    duplicates.iter().filter(|slot| slot.is_none()).count()
}
