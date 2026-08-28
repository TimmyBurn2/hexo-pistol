use std::fmt;
use std::str::FromStr;

use pistol_core::Turn;

use crate::position::PositionSpec;
use crate::position_set_token::{parse_set, write_set};

/// The move-list form's first word.
pub const START_FORM: &str = "start";
/// The keyword the move list follows.
pub const MOVES_KEYWORD: &str = "moves";
/// The stone-list form's first word.
pub const SET_FORM: &str = "set";

/// A stated position that is not one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePositionError {
    /// The tail as given, without the verb.
    pub tail: String,
    /// What is wrong with it.
    pub why: String,
}

impl fmt::Display for ParsePositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad position {:?}: {}", self.tail, self.why)
    }
}

impl std::error::Error for ParsePositionError {}

impl fmt::Display for PositionSpec {
    /// The tail the `position` verb carries, canonically spelled.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSpec::Start { moves } if moves.is_empty() => f.write_str(START_FORM),
            PositionSpec::Start { moves } => {
                write!(f, "{START_FORM} {MOVES_KEYWORD}")?;
                for turn in moves {
                    write!(f, " {turn}")?;
                }
                Ok(())
            }
            PositionSpec::Set {
                p1,
                p2,
                to_move,
                phase,
            } => {
                write!(f, "{SET_FORM} ")?;
                write_set(f, p1, p2, *to_move, *phase)
            }
        }
    }
}

impl FromStr for PositionSpec {
    type Err = ParsePositionError;

    fn from_str(tail: &str) -> Result<Self, Self::Err> {
        let reject = |why: String| ParsePositionError {
            tail: tail.to_string(),
            why,
        };
        let words: Vec<&str> = tail.split_whitespace().collect();
        match words.split_first() {
            Some((&START_FORM, rest)) => parse_start(rest).map_err(reject),
            Some((&SET_FORM, rest)) => parse_set(rest).map_err(reject),
            Some((other, _)) => Err(reject(format!(
                "expected `{START_FORM}` or `{SET_FORM}`, got `{other}`"
            ))),
            None => Err(reject(format!(
                "expected `{START_FORM}` or `{SET_FORM}`, got nothing"
            ))),
        }
    }
}

/// `start`, or `start moves <turn> <turn> ...`.
///
/// The turn tokens are pistol-core's own, refusals included: an uncanonical pair
/// is rejected rather than reordered (docs/decisions.md D-56).
fn parse_start(words: &[&str]) -> Result<PositionSpec, String> {
    let moves = match words.split_first() {
        None => Vec::new(),
        Some((&MOVES_KEYWORD, [])) => {
            return Err(format!("`{MOVES_KEYWORD}` with no turns after it"));
        }
        Some((&MOVES_KEYWORD, tokens)) => tokens
            .iter()
            .map(|token| token.parse::<Turn>().map_err(|error| error.why))
            .collect::<Result<Vec<Turn>, String>>()?,
        Some((other, _)) => {
            return Err(format!("expected `{MOVES_KEYWORD}`, got `{other}`"));
        }
    };
    Ok(PositionSpec::Start { moves })
}
