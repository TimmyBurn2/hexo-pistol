//! Reading a tactical fixture: the strict loader for the format
//! [`crate::fixtures`] describes.
//!
//! Strict on purpose. A directive it does not know, a case without a config, a
//! position, a budget or an expectation, a repeated name, a threshold larger than
//! the suite — each is a named refusal citing the line, because a fixture that is
//! quietly half-read reports a pass for cases nobody ran (CLAUDE.md rule 3).

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use pistol_core::{Coord, Turn};
use pistol_engine::{Budget, PositionSpec};

use crate::count::plain_count;
use crate::fixtures::{
    BUDGET, CASE, CONFIG, Case, EXPECT, Expectation, FixtureError, POSITION, REQUIRE, Suite,
};

/// Read and parse a fixture.
pub fn load(path: &Path) -> Result<Suite, FixtureError> {
    let text = fs::read_to_string(path).map_err(|io| FixtureError {
        path: path.to_path_buf(),
        line: None,
        why: format!("cannot read: {io}"),
    })?;
    parse(&text, path)
}

/// A case being read: every part optional until the case ends, so that a part
/// stated twice and a part never stated are both visible rather than being
/// overwritten by the last line to mention them.
#[derive(Debug)]
struct OpenCase {
    name: String,
    line: usize,
    config: Option<PathBuf>,
    position: Option<PositionSpec>,
    budget: Option<Budget>,
    expect: Vec<Expectation>,
}

/// Parse a fixture whose text is already in hand.
pub fn parse(text: &str, path: &Path) -> Result<Suite, FixtureError> {
    let mut required: Option<usize> = None;
    let mut cases: Vec<Case> = Vec::new();
    let mut open: Option<OpenCase> = None;

    for (number, directive, rest) in directives(text) {
        let fail = |why: String| FixtureError {
            path: path.to_path_buf(),
            line: Some(number),
            why,
        };
        match directive {
            REQUIRE => {
                if required.is_some() {
                    return Err(fail(format!("`{REQUIRE}` is stated twice")));
                }
                let count: usize = plain_count(rest)
                    .map_err(|why| fail(format!("`{rest}` is not a case count: {why}")))?;
                if count == 0 {
                    return Err(fail(format!(
                        "`{REQUIRE} 0` is a threshold no run can fail; a suite that demands \
                         nothing is not a gate"
                    )));
                }
                required = Some(count);
            }
            CASE => {
                if let Some(case) = open.take() {
                    cases.push(finish(case, path)?);
                }
                if rest.is_empty() {
                    return Err(fail(format!("`{CASE}` needs a name")));
                }
                if cases.iter().any(|case| case.name == rest) {
                    return Err(fail(format!("a second case is named `{rest}`")));
                }
                open = Some(OpenCase {
                    name: rest.to_string(),
                    line: number,
                    config: None,
                    position: None,
                    budget: None,
                    expect: Vec::new(),
                });
            }
            CONFIG | POSITION | BUDGET | EXPECT => {
                let case = open
                    .as_mut()
                    .ok_or_else(|| fail(format!("`{directive}` before any `{CASE}`")))?;
                read_into(case, directive, rest).map_err(fail)?;
            }
            other => return Err(fail(format!("unknown directive `{other}`"))),
        }
    }
    if let Some(case) = open.take() {
        cases.push(finish(case, path)?);
    }

    let Some(required) = required else {
        return Err(FixtureError {
            path: path.to_path_buf(),
            line: None,
            why: format!(
                "no `{REQUIRE}` line: the pass threshold is pre-registered in the fixture, not \
                 chosen by whoever runs it"
            ),
        });
    };
    if cases.is_empty() {
        return Err(FixtureError {
            path: path.to_path_buf(),
            line: None,
            why: String::from("the fixture holds no cases at all"),
        });
    }
    if required > cases.len() {
        return Err(FixtureError {
            path: path.to_path_buf(),
            line: None,
            why: format!(
                "`{REQUIRE} {required}` asks for more passes than the {} cases stated",
                cases.len()
            ),
        });
    }
    Ok(Suite { required, cases })
}

/// One directive of an open case. Position and budget are stated once each; a
/// second one is a fixture that says two things and means neither.
fn read_into(case: &mut OpenCase, directive: &str, rest: &str) -> Result<(), String> {
    match directive {
        CONFIG => {
            if case.config.is_some() {
                return Err(format!("case `{}` states `{CONFIG}` twice", case.name));
            }
            if rest.is_empty() {
                return Err(format!("`{CONFIG}` needs a path"));
            }
            case.config = Some(PathBuf::from(rest));
        }
        POSITION => {
            if case.position.is_some() {
                return Err(format!("case `{}` states `{POSITION}` twice", case.name));
            }
            case.position = Some(PositionSpec::from_str(rest).map_err(|error| error.why)?);
        }
        BUDGET => {
            if case.budget.is_some() {
                return Err(format!("case `{}` states `{BUDGET}` twice", case.name));
            }
            case.budget = Some(parse_budget(rest)?);
        }
        EXPECT => case.expect.push(parse_expectation(rest)?),
        // The caller matches the directive before calling this, so that an
        // unknown one is reported as unknown rather than as belonging to no case.
        // This arm is what makes adding a directive there and forgetting it here
        // a rejection instead of a silent no-op.
        other => return Err(format!("unknown directive `{other}`")),
    }
    Ok(())
}

/// `budget depth_turns <n>` or `budget nodes <n>`.
///
/// A wall-clock budget is deliberately not accepted: a fixture whose answer
/// depends on how fast the machine is would be a fixture that pins nothing
/// (CLAUDE.md rule 4, docs/decisions.md D-4).
fn parse_budget(rest: &str) -> Result<Budget, String> {
    let words: Vec<&str> = rest.split_whitespace().collect();
    let amount = |text: &str| -> Result<u64, String> {
        plain_count::<u64>(text).map_err(|why| format!("`{text}` is not an amount: {why}"))
    };
    match words.as_slice() {
        ["depth_turns", turns] => {
            let turns = amount(turns)?;
            let turns =
                u32::try_from(turns).map_err(|_| format!("{turns} turns is not a depth"))?;
            if turns == 0 {
                return Err(String::from("a depth budget of 0 turns searches nothing"));
            }
            Ok(Budget::DepthTurns(turns))
        }
        ["nodes", nodes] => {
            let nodes = amount(nodes)?;
            if nodes == 0 {
                return Err(String::from("a node budget of 0 searches nothing"));
            }
            Ok(Budget::Nodes(nodes))
        }
        _ => Err(format!(
            "`{BUDGET}` reads `depth_turns <n>` or `nodes <n>`, got `{rest}`"
        )),
    }
}

/// One `expect` line.
fn parse_expectation(rest: &str) -> Result<Expectation, String> {
    let words: Vec<&str> = rest.split_whitespace().collect();
    match words.as_slice() {
        ["move", token] => token
            .parse::<Turn>()
            .map(Expectation::Move)
            .map_err(|error| error.why),
        ["cell", token] => token
            .parse::<Coord>()
            .map(Expectation::Cell)
            .map_err(|error| error.why.to_string()),
        ["mate", turns] => plain_count::<u16>(turns)
            .map(Expectation::MateIn)
            .map_err(|why| format!("`{turns}` is not a mate distance in turns: {why}")),
        ["not-mated"] => Ok(Expectation::NotMated),
        _ => Err(format!(
            "`{EXPECT}` reads `move <turn>`, `cell <q,r>`, `mate <n>` or `not-mated`, got `{rest}`"
        )),
    }
}

/// A case is complete once it has a position, a budget and something to check.
fn finish(case: OpenCase, path: &Path) -> Result<Case, FixtureError> {
    let fail = |why: String| FixtureError {
        path: path.to_path_buf(),
        line: Some(case.line),
        why,
    };
    let missing = |what: &str| {
        fail(format!(
            "case `{}` states no `{what}`: a case is a config, a position, what the engine is \
             given, and what it must find",
            case.name
        ))
    };
    let config = case.config.ok_or_else(|| missing(CONFIG))?;
    let position = case.position.ok_or_else(|| missing(POSITION))?;
    let budget = case.budget.ok_or_else(|| missing(BUDGET))?;
    if case.expect.is_empty() {
        return Err(fail(format!(
            "case `{}` states no `{EXPECT}`: a case nobody checks is not a case",
            case.name
        )));
    }
    Ok(Case {
        name: case.name,
        line: case.line,
        config,
        position,
        budget,
        expect: case.expect,
    })
}

/// The meaningful lines, as `(line number, directive, rest)`. Blank lines and
/// `#` comments are dropped; nothing else is.
fn directives(text: &str) -> Vec<(usize, &str, &str)> {
    text.lines()
        .enumerate()
        .filter_map(|(index, raw)| {
            let content = raw.trim();
            if content.is_empty() || content.starts_with('#') {
                return None;
            }
            let (directive, rest) = match content.split_once(char::is_whitespace) {
                Some((directive, rest)) => (directive, rest.trim()),
                None => (content, ""),
            };
            Some((index + 1, directive, rest))
        })
        .collect()
}
