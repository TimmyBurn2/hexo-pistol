use std::collections::BTreeSet;

use pistol_core::{Coord, Player, canonical_form};
use pistol_engine::PositionSpec;

use super::error::RandomOpeningsError;
use super::{Book, Opening};

/// One position's identity, folded by transposition and by lattice symmetry.
///
/// The same fold `generate` dedupes with and the same one the corpus's
/// `key_full` column carries, so a key here is comparable with both.
pub type Key = Vec<(Coord, Player)>;

/// The canonical form of every opening a rendered book states.
///
/// Reads the DOCUMENT, not the generator's own `Book`, because a check that
/// shared the generator's state would agree with it by construction.
///
/// # Errors
/// A line the `position` verb cannot take, or one the rules refuse on replay.
pub fn keys_of_document(source: &str, text: &str) -> Result<BTreeSet<Key>, RandomOpeningsError> {
    let mut keys = BTreeSet::new();
    for (index, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        keys.insert(key_of_tail(source, index + 1, line)?);
    }
    Ok(keys)
}

/// The canonical form of one `position` tail.
///
/// # Errors
/// A tail that does not parse, or does not replay under the rules.
pub fn key_of_tail(source: &str, line: usize, tail: &str) -> Result<Key, RandomOpeningsError> {
    let spec: PositionSpec = tail
        .parse()
        .map_err(|why| RandomOpeningsError::Unreadable {
            source: source.to_string(),
            line,
            why: format!("{why}"),
        })?;
    let state = spec
        .replay()
        .map_err(|why| RandomOpeningsError::Unreadable {
            source: source.to_string(),
            line,
            why: format!("{why}"),
        })?;
    Ok(canonical_form(&state.played().collect::<Vec<_>>()))
}

/// Drop every opening whose canonical form is already in `excluded`, and refuse
/// unless exactly `wanted` survive.
///
/// The survivors keep GENERATION ORDER, so a prefix of the filtered book is
/// still a prefix of the draw — which is what lets a window of it be a sample
/// (docs/decisions.md D-143's argument).
///
/// # Errors
/// [`RandomOpeningsError::SurvivorCount`] when the count is not `wanted`,
/// naming the `n_openings` that would land on it; the book is never truncated
/// to fit.
pub fn retain_disjoint(
    drawn: Book,
    excluded: &BTreeSet<Key>,
    wanted: usize,
    n_openings: usize,
) -> Result<(Book, usize), RandomOpeningsError> {
    let ball_cells = drawn.ball_cells;
    let candidates_drawn = drawn.candidates_drawn;
    let symmetry_collisions = drawn.symmetry_collisions;
    let mut kept: Vec<Opening> = Vec::with_capacity(wanted);
    let mut rejected = 0usize;
    for (index, opening) in drawn.openings.into_iter().enumerate() {
        let key = key_of_tail("a generated opening", index + 1, &opening.tail)?;
        if excluded.contains(&key) {
            rejected += 1;
            continue;
        }
        kept.push(opening);
    }
    if kept.len() != wanted {
        // Signed, deliberately: the correction is negative when the filter left
        // MORE than was asked for, and computing it in `usize` would wrap to an
        // astronomical suggestion instead of a smaller book.
        let correction = i64::try_from(wanted).unwrap_or(i64::MAX)
            - i64::try_from(kept.len()).unwrap_or(i64::MAX);
        return Err(RandomOpeningsError::SurvivorCount {
            wanted,
            survived: kept.len(),
            suggestion: i64::try_from(n_openings).unwrap_or(i64::MAX) + correction,
        });
    }
    Ok((
        Book {
            openings: kept,
            ball_cells,
            candidates_drawn,
            symmetry_collisions,
        },
        rejected,
    ))
}
