use pistol_core::Coord;

use super::{directives, fixture_text, parse_coords};

/// The fixture's name under `tests/fixtures/`.
pub const PERFT_POSITIONS_FILE: &str = "perft_positions_v1.txt";

/// One depth, and the number of distinct turn sequences of that depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerftExpectation {
    /// The depth, in turns.
    pub depth_turns: u32,
    /// How many sequences of that many turns the position has.
    pub turns: u64,
    /// The fixture line, for failure messages.
    pub line: usize,
}

/// One perft position.
#[derive(Debug, Clone)]
pub struct PerftCase {
    /// The case name, as written in the fixture.
    pub name: String,
    /// The game that reaches the position, in play order.
    pub plies: Vec<Coord>,
    /// Every depth the fixture states a count for, in fixture order.
    pub expect: Vec<PerftExpectation>,
    /// The fixture line the case starts on, for failure messages.
    pub line: usize,
}

/// Every perft position, in fixture order.
pub fn perft_cases() -> Vec<PerftCase> {
    parse_cases(&fixture_text(PERFT_POSITIONS_FILE))
}

/// One perft position by name. Panics if the fixture has no such case — a test
/// that silently found nothing to run is worse than a failing one.
pub fn perft_case(name: &str) -> PerftCase {
    perft_cases()
        .into_iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("the perft fixture has no case `{name}`"))
}

/// Parse the format described in the fixture's own header.
pub fn parse_cases(text: &str) -> Vec<PerftCase> {
    let mut cases: Vec<PerftCase> = Vec::new();
    let mut open: Option<PerftCase> = None;

    for (line, directive, rest) in directives(text) {
        if directive == "case" {
            if let Some(case) = open.take() {
                cases.push(finish(case));
            }
            assert!(!rest.is_empty(), "line {line}: `case` needs a name");
            assert!(
                !cases.iter().any(|case| case.name == rest),
                "line {line}: a second case is named `{rest}`"
            );
            open = Some(PerftCase {
                name: rest.to_string(),
                plies: Vec::new(),
                expect: Vec::new(),
                line,
            });
            continue;
        }
        let case = open
            .as_mut()
            .unwrap_or_else(|| panic!("line {line}: `{directive}` before any `case`"));
        match directive {
            "plies" => {
                assert!(!rest.is_empty(), "line {line}: `plies` with no stones");
                case.plies.extend(parse_coords(rest, line));
            }
            "expect" => {
                let expectation = parse_expectation(rest, line);
                assert!(
                    !case
                        .expect
                        .iter()
                        .any(|other| other.depth_turns == expectation.depth_turns),
                    "line {line}: case `{}` states depth {} twice",
                    case.name,
                    expectation.depth_turns
                );
                case.expect.push(expectation);
            }
            other => panic!("line {line}: unknown directive `{other}`"),
        }
    }
    if let Some(case) = open.take() {
        cases.push(finish(case));
    }
    assert!(!cases.is_empty(), "the fixture holds no cases at all");
    cases
}

/// `expect depth <n> turns <n>`.
fn parse_expectation(rest: &str, line: usize) -> PerftExpectation {
    let words: Vec<&str> = rest.split_whitespace().collect();
    let ["depth", depth, "turns", turns] = words.as_slice() else {
        panic!("line {line}: `expect` reads `depth <n> turns <n>`, got `{rest}`");
    };
    for word in [depth, turns] {
        assert!(
            !word.starts_with('+') && !word.starts_with('-'),
            "line {line}: `{word}` is written as a plain count"
        );
    }
    PerftExpectation {
        depth_turns: depth
            .parse()
            .unwrap_or_else(|_| panic!("line {line}: `{depth}` is not a depth in turns")),
        turns: turns
            .parse()
            .unwrap_or_else(|_| panic!("line {line}: `{turns}` is not a turn count")),
        line,
    }
}

/// A case is only complete once it says what it expects — a position with no
/// stated count is a case nobody checks.
fn finish(case: PerftCase) -> PerftCase {
    assert!(
        !case.plies.is_empty(),
        "case `{}` (line {}) has no plies",
        case.name,
        case.line
    );
    assert!(
        !case.expect.is_empty(),
        "case `{}` (line {}) states no expected count",
        case.name,
        case.line
    );
    case
}
