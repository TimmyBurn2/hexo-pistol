//! Every way this tool refuses, each with its own name.
//!
//! One enum rather than a string, because the tests match on the variant: a
//! refusal identified by its message is a refusal that changes identity when
//! someone improves the wording (CLAUDE.md rule 3, and the lesson D-152 records
//! about assertions that hold a constant against itself).

use std::fmt;
use std::path::PathBuf;

/// A document or a run this build refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RandomOpeningsError {
    /// The config file could not be read.
    Read {
        /// The path given.
        path: PathBuf,
        /// The operating system's reason.
        why: String,
    },
    /// The document is not TOML, or does not fit the schema.
    Schema {
        /// What the parser said, key path included where it recovered one.
        why: String,
    },
    /// The document is written against another version of this schema.
    SchemaVersion {
        /// The version the document states.
        found: u32,
        /// The version this build speaks.
        expected: u32,
    },
    /// An even stone count: a position between the two stones of one turn.
    MidTurnStoneCount {
        /// The count asked for.
        k_stones: usize,
    },
    /// A turn boundary this generator is not specified at.
    UnsupportedStoneCount {
        /// The count asked for.
        k_stones: usize,
        /// The counts it does generate.
        supported: &'static [usize],
    },
    /// The generation ball cannot hold a whole position.
    BallTooSmall {
        /// The radius asked for.
        max_radius: u32,
        /// How many cells that ball holds.
        cells: usize,
        /// How many stones a position needs.
        k_stones: usize,
    },
    /// A radius past the typo ceiling.
    RadiusPastCeiling {
        /// The radius asked for.
        max_radius: u32,
        /// The ceiling.
        ceiling: u32,
    },
    /// A book size past the typo ceiling, or of no openings at all.
    CountPastCeiling {
        /// The count asked for.
        n_openings: usize,
        /// The ceiling.
        ceiling: usize,
    },
    /// The pool of distinct openings ran dry before the book was full.
    Exhausted {
        /// How many distinct openings were produced.
        produced: usize,
        /// How many were asked for.
        wanted: usize,
        /// How many candidates in a row were duplicates before giving up.
        consecutive: usize,
    },
    /// A drawn stone landed outside the legal region game rule 5 allows.
    ///
    /// Its own variant, and not folded into [`RandomOpeningsError::IllegalPlacement`],
    /// because the two have different remedies: this one says the GENERATION
    /// radius reached past the LEGAL radius and the document should ask for
    /// less, and the other says the rules refused a stone for a reason this
    /// tool did not anticipate. pistol-core makes the same split for the same
    /// reason (`Board::check_placement` distinguishes rule 3 from rule 5).
    ///
    /// Reachable: `max_radius` has its own typo ceiling and may exceed
    /// `LEGAL_RADIUS`, at which point a drawn stone can be beyond eight of
    /// every stone on the board. It is never silently redrawn — a resample here
    /// would quietly turn `max_radius` into something other than what it says.
    OutsideLegalRegion {
        /// The ply it happened on, 1-based.
        ply: usize,
        /// The cell that was drawn.
        cell: String,
    },
    /// The rules refused a drawn stone for any other reason.
    IllegalPlacement {
        /// The ply it happened on, 1-based.
        ply: usize,
        /// What the rules said.
        why: String,
    },
    /// A generated position did not survive its own round trip.
    Roundtrip {
        /// The line that was written.
        tail: String,
        /// What went wrong reading it back.
        why: String,
    },
    /// The unbiased index sampler rejected too many words in a row.
    ///
    /// Rejection sampling discards the words that would bias the modulus. The
    /// discarded fraction is under one part in `2^49` at any radius this
    /// schema allows, so this cannot happen; it is a named refusal because a
    /// loop with no exit is not a thing this project ships.
    RejectionRunaway {
        /// The bound being sampled under.
        bound: u64,
        /// How many words were rejected in a row.
        tries: usize,
    },
    /// An output could not be written.
    Write {
        /// The path.
        path: PathBuf,
        /// The operating system's reason.
        why: String,
    },
}

impl fmt::Display for RandomOpeningsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomOpeningsError::Read { path, why } => {
                write!(f, "cannot read {}: {why}", path.display())
            }
            RandomOpeningsError::Schema { why } => write!(f, "{why}"),
            RandomOpeningsError::SchemaVersion { found, expected } => write!(
                f,
                "this document states schema_version {found} and this build speaks {expected}"
            ),
            RandomOpeningsError::MidTurnStoneCount { k_stones } => write!(
                f,
                "k_stones = {k_stones} is a MID-TURN position: a turn places two stones (rule 3), \
                 so an even count stops between them and is not a position any game is at. The \
                 turn boundaries are the odd counts"
            ),
            RandomOpeningsError::UnsupportedStoneCount {
                k_stones,
                supported,
            } => write!(
                f,
                "k_stones = {k_stones} is a turn boundary this generator is not specified at; it \
                 generates {supported:?}"
            ),
            RandomOpeningsError::BallTooSmall {
                max_radius,
                cells,
                k_stones,
            } => write!(
                f,
                "a radius-{max_radius} ball around the origin holds {cells} cell(s) and a \
                 position needs {k_stones}"
            ),
            RandomOpeningsError::RadiusPastCeiling {
                max_radius,
                ceiling,
            } => write!(
                f,
                "max_radius = {max_radius} is past this tool's typo ceiling of {ceiling}; it is a \
                 generation knob and is not game rule 5's LEGAL_RADIUS"
            ),
            RandomOpeningsError::CountPastCeiling {
                n_openings,
                ceiling,
            } => write!(
                f,
                "n_openings = {n_openings} is outside 1..={ceiling}, this tool's typo ceiling"
            ),
            RandomOpeningsError::Exhausted {
                produced,
                wanted,
                consecutive,
            } => write!(
                f,
                "the pool ran dry at {produced} of {wanted} openings: {consecutive} candidates in \
                 a row were positions already in the book, up to a lattice symmetry. Widen \
                 max_radius or ask for fewer — a short book is not written, because a sample \
                 size nobody chose is not a measurement"
            ),
            RandomOpeningsError::OutsideLegalRegion { ply, cell } => write!(
                f,
                "stone {ply} of a generated position was drawn at {cell}, outside the legal \
                 region the stones before it opened (game rule 5). max_radius reached past \
                 LEGAL_RADIUS: the two are different numbers and this tool does not reconcile \
                 them by redrawing"
            ),
            RandomOpeningsError::IllegalPlacement { ply, why } => write!(
                f,
                "stone {ply} of a generated position is not a legal placement: {why}"
            ),
            RandomOpeningsError::Roundtrip { tail, why } => {
                write!(f, "the line {tail:?} does not read back as itself: {why}")
            }
            RandomOpeningsError::RejectionRunaway { bound, tries } => write!(
                f,
                "the index sampler rejected {tries} words in a row under bound {bound}, which the \
                 arithmetic says cannot happen"
            ),
            RandomOpeningsError::Write { path, why } => {
                write!(f, "cannot write {}: {why}", path.display())
            }
        }
    }
}

impl std::error::Error for RandomOpeningsError {}
