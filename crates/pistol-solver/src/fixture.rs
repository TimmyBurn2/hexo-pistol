//! The solver fixture loader (the `pattern_v0.txt` discipline).
//!
//! Strict on purpose: an unknown directive, a missing expectation, an
//! unparseable cell, a case that does not reach an ongoing two-stone
//! position for the mover — each is a named error, because a golden file
//! that is quietly half-read reports a pass for cases nobody ran (rule 3).

use std::fmt;

use pistol_core::{Coord, GameState};

/// A malformed fixture, named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureError {
    /// An unknown directive.
    UnknownDirective { line: usize, what: String },
    /// A well-known directive in the wrong place, or missing at the end.
    Structure { line: usize, what: String },
    /// A cell that is not `q,r`.
    BadCell { line: usize, what: String },
    /// The game the plies spell is not one the solver takes.
    BadPosition { name: String, what: String },
    /// The fixture holds no cases at all.
    Empty,
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixtureError::UnknownDirective { line, what } => {
                write!(f, "fixture line {line}: unknown directive {what:?}")
            }
            FixtureError::Structure { line, what } => write!(f, "fixture line {line}: {what}"),
            FixtureError::BadCell { line, what } => {
                write!(f, "fixture line {line}: not a cell: {what:?}")
            }
            FixtureError::BadPosition { name, what } => {
                write!(f, "fixture case {name:?}: {what}")
            }
            FixtureError::Empty => write!(f, "fixture holds no cases"),
        }
    }
}

/// What a case expects the solver to answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expectation {
    /// The attacker forces the win.
    Win,
    /// The attacker does not.
    NoWin,
}

/// One fixture case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureCase {
    /// The case's name.
    pub name: String,
    /// The game prefix that reaches the position.
    pub plies: Vec<Coord>,
    /// The registered expectation.
    pub expect: Expectation,
}

impl FixtureCase {
    /// The position the plies reach.
    ///
    /// # Errors
    ///
    /// [`FixtureError::BadPosition`] when the plies are not a legal game
    /// prefix ending at an ongoing `Phase::First` position owing two stones.
    pub fn position(&self) -> Result<GameState, FixtureError> {
        let state =
            GameState::from_plies(&self.plies).map_err(|error| FixtureError::BadPosition {
                name: self.name.clone(),
                what: format!("the plies are not a legal game prefix ({error})"),
            })?;
        if state.outcome().is_decided() {
            return Err(FixtureError::BadPosition {
                name: self.name.clone(),
                what: "the game is already decided".into(),
            });
        }
        if state.phase() != pistol_core::Phase::First || state.stones_owed() != 2 {
            return Err(FixtureError::BadPosition {
                name: self.name.clone(),
                what: "the position is not at a turn boundary owing two stones".into(),
            });
        }
        Ok(state)
    }
}

/// Load and validate a fixture file.
pub fn load(text: &str) -> Result<Vec<FixtureCase>, FixtureError> {
    let mut cases = Vec::new();
    let mut name: Option<String> = None;
    let mut plies: Option<Vec<Coord>> = None;
    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut words = trimmed.split_whitespace();
        let Some(directive) = words.next() else {
            continue;
        };
        match directive {
            "case" => {
                finish(&mut name, &mut plies, &mut cases, line)?;
                let what = words.next().map(str::to_owned).unwrap_or_default();
                if what.is_empty() {
                    return Err(FixtureError::Structure {
                        line,
                        what: "case needs a name".into(),
                    });
                }
                name = Some(what);
            }
            "plies" => {
                if name.is_none() || plies.is_some() {
                    return Err(FixtureError::Structure {
                        line,
                        what: "plies without a case, or twice in one case".into(),
                    });
                }
                let mut cells = Vec::new();
                for word in words {
                    let cell = Coord::from_str_word(word).ok_or_else(|| FixtureError::BadCell {
                        line,
                        what: word.to_owned(),
                    })?;
                    cells.push(cell);
                }
                plies = Some(cells);
            }
            "expect" => {
                let (Some(case_name), Some(case_plies)) = (name.take(), plies.take()) else {
                    return Err(FixtureError::Structure {
                        line,
                        what: "expect without a case's plies".into(),
                    });
                };
                let expect = match words.next() {
                    Some("win") => Expectation::Win,
                    Some("nowin") => Expectation::NoWin,
                    other => {
                        return Err(FixtureError::Structure {
                            line,
                            what: format!("unknown expectation {other:?}"),
                        });
                    }
                };
                cases.push(FixtureCase {
                    name: case_name,
                    plies: case_plies,
                    expect,
                });
            }
            other => {
                return Err(FixtureError::UnknownDirective {
                    line,
                    what: other.to_owned(),
                });
            }
        }
    }
    finish(&mut name, &mut plies, &mut cases, text.lines().count())?;
    if cases.is_empty() {
        return Err(FixtureError::Empty);
    }
    for case in &cases {
        case.position()?;
    }
    Ok(cases)
}

fn finish(
    name: &mut Option<String>,
    plies: &mut Option<Vec<Coord>>,
    _cases: &mut Vec<FixtureCase>,
    line: usize,
) -> Result<(), FixtureError> {
    if name.is_some() || plies.is_some() {
        return Err(FixtureError::Structure {
            line,
            what: "a case ended without its expectation".into(),
        });
    }
    Ok(())
}

trait CoordParse {
    fn from_str_word(word: &str) -> Option<Coord>;
}

impl CoordParse for Coord {
    fn from_str_word(word: &str) -> Option<Coord> {
        word.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_a_well_formed_case() {
        let cases = load("case a\nplies 0,0 0,8 0,9\nexpect win\n").unwrap();
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].name, "a");
        assert_eq!(cases[0].expect, Expectation::Win);
        assert_eq!(cases[0].plies.len(), 3);
        assert!(cases[0].position().is_ok());
    }

    #[test]
    fn an_unknown_directive_is_refused() {
        let error = load("case a\nplies 0,0\nexpect win\nbogus x\n").unwrap_err();
        assert_eq!(
            error,
            FixtureError::UnknownDirective {
                line: 4,
                what: "bogus".into()
            }
        );
    }

    #[test]
    fn a_case_without_its_expectation_is_refused() {
        let error = load("case a\nplies 0,0 0,8 0,9\n").unwrap_err();
        assert!(matches!(error, FixtureError::Structure { .. }));
    }

    #[test]
    fn an_empty_fixture_is_refused() {
        assert_eq!(load("# nothing\n").unwrap_err(), FixtureError::Empty);
    }

    #[test]
    fn a_decided_position_is_refused() {
        // Six in a row for P1: the game is decided before any solver runs.
        let error = load(
            "case won\nplies 0,0 0,8 0,9 1,0 1,8 1,9 2,0 2,8 2,9 3,0 3,8 3,9 4,0 4,8 4,9 5,0\nexpect win\n",
        )
        .unwrap_err();
        assert!(matches!(error, FixtureError::BadPosition { .. }));
    }
}
