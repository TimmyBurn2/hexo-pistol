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
