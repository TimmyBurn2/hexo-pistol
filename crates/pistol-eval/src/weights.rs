//! The handcrafted_v0 weight table, as committed configuration.
//!
//! One document, one backend, one table: what a length-six window is worth to
//! whoever owns the stones in it, by how many of the six cells they hold. Every
//! entry is required and none has a code-side default, so a missing entry is a
//! named error rather than a zero (CLAUDE.md rule 1, docs/decisions.md D-11).
//!
//! Two entries are deliberately *not* in the document:
//!
//! - **0** — an empty window scores nothing, and an unbounded board has
//!   infinitely many of them.
//! - **6** — six own stones in a window is a win, and a win's score is the
//!   search's mate band, never a number an operator tunes. The table's sixth
//!   entry is therefore [`DECIDED_WINDOW_VALUE`], derived from the eval band
//!   rather than configured (docs/decisions.md D-3, D-63).
//!
//! This is a different document kind from the engine config, with its own
//! `schema_version`, and `tools/config_check.sh` validates it with the
//! `validate_weights` example rather than as a config (docs/decisions.md D-64).

use std::fs;
use std::path::Path;

use pistol_core::WIN_LEN;
use serde::Deserialize;

use crate::error::{self, EvalError};
use crate::eval::EVAL_MAX;

/// The weights schema version this build understands.
pub const WEIGHTS_SCHEMA_VERSION: u32 = 1;

/// What a window of [`WIN_LEN`] own stones is worth to a static evaluation: the
/// top of the eval band, and no further. The position is won, but by how many
/// turns is the search's answer, not this one's.
pub const DECIDED_WINDOW_VALUE: i32 = EVAL_MAX;

/// Entries in the table: one per own-stone count, `0..=WIN_LEN`.
const TABLE_LEN: usize = WIN_LEN as usize + 1;

/// How many entries the document states: the counts a position can hold without
/// already being decided.
const STATED_COUNTS: usize = WIN_LEN as usize - 1;

const _: () = assert!(
    STATED_COUNTS == 5,
    "the document states one entry per own-stone count below WIN_LEN; a change \
     to WIN_LEN changes the document, and its schema version with it"
);

/// A loaded, validated weight table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Weights {
    /// Indexed by own-stone count: `[0, stated 1..=5, DECIDED_WINDOW_VALUE]`.
    table: [i32; TABLE_LEN],
}

impl Weights {
    /// Read, parse and validate a weight table.
    ///
    /// A file that is not there is [`EvalError::WeightsUnreadable`] — the loud
    /// load-time half of docs/decisions.md D-21, which config validation
    /// deliberately does not check. Relative paths resolve against the process's
    /// working directory, exactly as the operator wrote them.
    pub fn load(path: &Path) -> Result<Weights, EvalError> {
        let text = fs::read_to_string(path).map_err(|io| EvalError::WeightsUnreadable {
            path: path.to_path_buf(),
            why: io.to_string(),
        })?;
        Weights::parse(&text)
    }

    /// Parse and validate a weight table from a document.
    ///
    /// Two stages, for two kinds of error: the first parse reports syntax with a
    /// line and column, the second reports schema violations with the key path
    /// `serde_path_to_error` recovers.
    pub fn parse(text: &str) -> Result<Weights, EvalError> {
        let document: toml::Value = toml::from_str(text).map_err(error::from_toml_syntax)?;
        let document: Document =
            serde_path_to_error::deserialize(document).map_err(error::from_path_error)?;
        document.into_weights()
    }

    /// What one window holding `own_count` stones of one player, and none of the
    /// other, is worth to that player.
    ///
    /// # Panics
    ///
    /// If `own_count` exceeds [`WIN_LEN`], which a window of `WIN_LEN` cells
    /// cannot hold and the window bookkeeping cannot produce.
    pub fn window_value(&self, own_count: u8) -> i32 {
        self.table[usize::from(own_count)]
    }
}

/// The document as written.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    schema_version: u32,
    backend: Backend,
    table: Table,
}

/// Which `Eval` implementation a table is for.
///
/// One variant, and it earns its keep: a Stage-2 codebook file names a different
/// backend, and this is what refuses it instead of reading it as a v0 table.
#[derive(Debug, Deserialize)]
enum Backend {
    #[serde(rename = "handcrafted_v0")]
    HandcraftedV0,
}

/// `[table]` — one entry per own-stone count the document states.
///
/// The keys are the counts themselves, so the document reads the way D-11 states
/// the rule. The field names are spelled out because a Rust identifier cannot be
/// a digit; `serde` maps them back.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Table {
    #[serde(rename = "1")]
    one: i32,
    #[serde(rename = "2")]
    two: i32,
    #[serde(rename = "3")]
    three: i32,
    #[serde(rename = "4")]
    four: i32,
    #[serde(rename = "5")]
    five: i32,
}

impl Document {
    /// Apply the rules `serde` cannot express: the version, and the shape of the
    /// table. Nothing here repairs a value — a table is right or it is refused
    /// (CLAUDE.md rule 3).
    fn into_weights(self) -> Result<Weights, EvalError> {
        let Document {
            schema_version,
            backend,
            table,
        } = self;
        // Reading it is the check: a foreign backend never deserializes.
        match backend {
            Backend::HandcraftedV0 => {}
        }

        if schema_version != WEIGHTS_SCHEMA_VERSION {
            return Err(EvalError::weights(
                "schema_version",
                format!(
                    "this build reads weights schema version {WEIGHTS_SCHEMA_VERSION}, \
                     the document says {schema_version}"
                ),
            ));
        }

        // Bounds, not values (docs/decisions.md D-18, D-65): more own stones in
        // a window is never worth less, and a single window that alone saturates
        // the clamp would make every deeper distinction invisible.
        let stated = [table.one, table.two, table.three, table.four, table.five];
        let mut previous = 0i32;
        for (index, &value) in stated.iter().enumerate() {
            let count = index + 1;
            let key = format!("table.{count}");
            if value <= previous {
                let why = if count == 1 {
                    format!("a window holding one own stone must be worth at least 1, got {value}")
                } else {
                    format!(
                        "the table must strictly increase: {count} own stones at {value} is not \
                         worth more than {} at {previous}",
                        count - 1
                    )
                };
                return Err(EvalError::weights(key, why));
            }
            if value > EVAL_MAX {
                return Err(EvalError::weights(
                    key,
                    format!("must be at most the eval band's {EVAL_MAX}, got {value}"),
                ));
            }
            previous = value;
        }

        let mut full = [0i32; TABLE_LEN];
        full[1..=STATED_COUNTS].copy_from_slice(&stated);
        full[TABLE_LEN - 1] = DECIDED_WINDOW_VALUE;
        Ok(Weights { table: full })
    }
}
