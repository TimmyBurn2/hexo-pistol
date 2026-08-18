//! `random-openings` — a seeded, synthetic opening book, from a committed
//! config and nothing else.
//!
//! The book the arena runs its SPRT over (docs/decisions.md D-175). It is a
//! sibling of [`crate::corpus`] and not a mode of it: that tool's every path
//! begins by reading an external corpus of human games and identifying it by
//! digest, and this one has no input but a committed TOML document. What the
//! two share is the fixture *form* — [`crate::corpus::emit`]'s header, `#
//! param` and `# derived` lines, and the in-band body digest (D-147, D-148) —
//! which is a serialization discipline rather than anything about a corpus.
//!
//! - [`config`] — the four parameters, and every way a document is refused.
//!   Validation runs in [`generate`] as well as in the parser, because both
//!   this function and the config's fields are public and a validator on one of
//!   two doors is not one (docs/decisions.md D-181).
//! - [`rng`] — SplitMix64, pinned against its published stream.
//! - [`document`] — what the emitted file says.
//! - [`error`] — the named refusals.
//!
//! # What a generated opening is
//!
//! Game rule 3 fixes the shape: turn 1 is one stone at the origin, and every
//! later turn is two stones by the mover. So a `k_stones = 5` opening is P1 at
//! the origin, then P2's two stones, then P1's two — and the colours are not a
//! choice this module makes, they fall out of asking pistol-core whose stone
//! comes next.
//!
//! Rules truth stays in pistol-core (CLAUDE.md rule 2). Nothing here decides
//! whether a placement is legal: every stone goes down through
//! [`GameState::place`], and rule 5 is *asked* about each one at its own point
//! in the sequence before it is placed. Inside `max_radius` of the origin that
//! question can only be answered yes — the origin holds the first stone and
//! `max_radius` is far under `LEGAL_RADIUS` — and it is asked anyway, because a
//! generator that assumed it would go on writing a book on the day the
//! assumption stopped holding (rule 3).

pub mod config;
pub mod document;
pub mod error;
pub mod rng;

use std::collections::BTreeSet;

use pistol_core::{Coord, GameState, Player, Turn, canonical_form};
use pistol_engine::PositionSpec;

use config::RandomOpeningsConfig;
use error::RandomOpeningsError;
use rng::SplitMix64;

/// The name of the file this tool writes. The tool's, never the operator's: a
/// book that could be written under any name is a book whose pin means nothing.
pub const FILE_NAME: &str = "random_openings_v1.txt";

/// How many candidates in a row may duplicate an opening already in the book
/// before the run is refused.
///
/// A pool that cannot supply the book must fail rather than spin, and it must
/// fail rather than come up short: a shorter book is a sample size nobody chose
/// (CLAUDE.md rule 6). This number can only ever decide *whether* a run
/// refuses, never what a successful run wrote, so it is not a tunable and does
/// not belong in the config.
pub const CONSECUTIVE_COLLISION_LIMIT: usize = 4096;

/// One generated opening.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opening {
    /// The turns, in play order.
    pub moves: Vec<Turn>,
    /// The `position` verb's tail — the line as it is written.
    pub tail: String,
}

/// A generated book, with the numbers its header has to state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    /// The openings, in generation order.
    pub openings: Vec<Opening>,
    /// How many cells the sampling ball held.
    pub ball_cells: usize,
    /// How many candidates were drawn, kept and discarded together.
    pub candidates_drawn: usize,
    /// How many were discarded as a position already in the book, up to a
    /// lattice symmetry.
    pub symmetry_collisions: usize,
}

impl Book {
    /// The lines, in order — what a test compares when it means "the same book".
    pub fn tails(&self) -> Vec<&str> {
        self.openings
            .iter()
            .map(|opening| opening.tail.as_str())
            .collect()
    }
}

/// The cells within hex distance `max_radius` of the origin, ascending.
///
/// Crate-private, and the visibility is load bearing: the work is quadratic in
/// the radius, and what keeps it bounded is that every caller has already run
/// [`config::RandomOpeningsConfig::validate`], whose ceiling check comes before
/// its call to this function. That is why [`generate`] revalidates rather than
/// trusting its argument — the ceiling is not a property of the type.
///
/// Ascending by `Coord`'s own `(q, r)` order, which is the order the protocol
/// canonicalizes pairs in and the search tie-breaks on (docs/decisions.md D-5,
/// D-7) — a fixed order, because an index into this list is what a drawn word
/// becomes and a list whose order could drift is a book that could drift with
/// it. The distance is pistol-core's; this asks the question rather than
/// answering it (CLAUDE.md rule 2).
pub(crate) fn ball(max_radius: u32) -> Vec<Coord> {
    // Not a clamp. A radius past the ceiling is refused by validation before it
    // can arrive here, so a value that does not fit an `i16` means the ceiling
    // moved or a caller skipped the validator, and either is a broken invariant
    // rather than a number to round down (CLAUDE.md rule 3).
    let span = i32::from(i16::try_from(max_radius).unwrap_or_else(|_| {
        unreachable!(
            "a radius of {max_radius} reached ball(); MAX_RADIUS_CEILING is {}",
            config::MAX_RADIUS_CEILING
        )
    }));
    let mut cells = Vec::new();
    for q in -span..=span {
        for r in -span..=span {
            let cell = Coord::new(q as i16, r as i16);
            if Coord::ORIGIN.distance(cell) <= max_radius {
                cells.push(cell);
            }
        }
    }
    cells
}

/// Generate the book this config describes.
///
/// The loop is: draw a candidate, place it through the rules, fold it by its
/// canonical form, and keep it if the book has not already got it. A duplicate
/// discards the WHOLE candidate and the next one is drawn from the next words
/// of the same stream — a resample rule that is a function of the draws made so
/// far and of nothing else, which is what makes the run reproducible.
pub fn generate(config: &RandomOpeningsConfig) -> Result<Book, RandomOpeningsError> {
    // Revalidated, not trusted. This is `pub` and every field of the config is
    // `pub`, so the struct-literal door exists whether or not anyone uses it
    // today, and behind it are the bounds that keep this loop finite and the
    // stone count inside D-175's arithmetic.
    config.validate()?;
    let wanted = config.generate.n_openings;
    let k_stones = config.generate.k_stones;
    let cells = ball(config.generate.max_radius);
    let mut rng = SplitMix64::from_seed(config.generate.seed);

    let mut seen: BTreeSet<Vec<(Coord, Player)>> = BTreeSet::new();
    let mut openings: Vec<Opening> = Vec::with_capacity(wanted);
    let mut candidates_drawn = 0;
    let mut symmetry_collisions = 0;
    let mut consecutive = 0;

    while openings.len() < wanted {
        if consecutive >= CONSECUTIVE_COLLISION_LIMIT {
            return Err(RandomOpeningsError::Exhausted {
                produced: openings.len(),
                wanted,
                consecutive,
            });
        }
        candidates_drawn += 1;
        let plies = draw(&mut rng, &cells, k_stones)?;
        let state = place_all(&plies)?;
        if !seen.insert(canonical_form(&state.played().collect::<Vec<_>>())) {
            symmetry_collisions += 1;
            consecutive += 1;
            continue;
        }
        consecutive = 0;
        openings.push(spell(&plies, &state)?);
    }

    Ok(Book {
        openings,
        ball_cells: cells.len(),
        candidates_drawn,
        symmetry_collisions,
    })
}

/// One candidate's stones, in play order, starting at the origin.
///
/// Each later stone is a uniform draw over the cells of the ball that this
/// candidate has not used yet. Drawing from the free cells rather than from the
/// whole ball and redrawing on a hit is the same distribution and has no loop
/// in it: exactly one word is consumed per stone, so where the stream stands
/// after a candidate does not depend on how the candidate came out.
fn draw(
    rng: &mut SplitMix64,
    cells: &[Coord],
    k_stones: usize,
) -> Result<Vec<Coord>, RandomOpeningsError> {
    let mut free: Vec<Coord> = cells.to_vec();
    let mut plies = Vec::with_capacity(k_stones);
    // Rule 3: turn 1 is one stone at the origin, so it is placed and never
    // drawn. It leaves the free list because a cell holds one stone.
    plies.push(Coord::ORIGIN);
    free.retain(|&cell| cell != Coord::ORIGIN);
    while plies.len() < k_stones {
        let index = rng.below(free.len() as u64)? as usize;
        plies.push(free.remove(index));
    }
    Ok(plies)
}

/// Place a candidate's stones, asking rule 5 about each one before it goes down.
fn place_all(plies: &[Coord]) -> Result<GameState, RandomOpeningsError> {
    let mut state = GameState::new_game();
    for (index, &cell) in plies.iter().enumerate() {
        if !state.board().in_legal_region(cell) {
            return Err(RandomOpeningsError::OutsideLegalRegion {
                ply: index + 1,
                cell: cell.to_string(),
            });
        }
        state
            .place(cell)
            .map_err(|error| RandomOpeningsError::IllegalPlacement {
                ply: index + 1,
                why: error.to_string(),
            })?;
    }
    Ok(state)
}

/// The line a candidate is written as, checked against the position it came
/// from.
///
/// The round trip is done here rather than left to a test over the finished
/// file, so that a spelling which does not read back as itself is a refusal at
/// the moment it is produced instead of bytes on disk that a later gate has to
/// catch (CLAUDE.md rule 3).
fn spell(plies: &[Coord], state: &GameState) -> Result<Opening, RandomOpeningsError> {
    let mut moves = vec![Turn::Single(plies[0])];
    for pair in plies[1..].chunks(2) {
        let [first, second] = pair else {
            return Err(RandomOpeningsError::Roundtrip {
                tail: format!("{pair:?}"),
                why: "a turn after the first places two stones (game rule 3)".to_string(),
            });
        };
        moves.push(Turn::pair(*first, *second).map_err(|error| {
            RandomOpeningsError::Roundtrip {
                tail: format!("{first}/{second}"),
                why: error.to_string(),
            }
        })?);
    }
    let spec = PositionSpec::Start {
        moves: moves.clone(),
    };
    let tail = spec.to_string();
    let reread: PositionSpec = tail
        .parse()
        .map_err(|error| RandomOpeningsError::Roundtrip {
            tail: tail.clone(),
            why: format!("{error}"),
        })?;
    if reread != spec {
        return Err(RandomOpeningsError::Roundtrip {
            tail: tail.clone(),
            why: "it parses as a different move list".to_string(),
        });
    }
    let replayed = reread
        .replay()
        .map_err(|error| RandomOpeningsError::Roundtrip {
            tail: tail.clone(),
            why: format!("{error}"),
        })?;
    if replayed.key() != state.key() {
        return Err(RandomOpeningsError::Roundtrip {
            tail: tail.clone(),
            why: "it replays to a different position".to_string(),
        });
    }
    Ok(Opening { moves, tail })
}
