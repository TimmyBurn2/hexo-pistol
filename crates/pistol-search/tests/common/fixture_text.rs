//! Strict readers for the two fixture files this suite borrows.
//!
//! Both files are owned by other crates and pinned by them; these readers take
//! only the positions and ignore the expectations, which belong to the suites
//! that state them. "Ignore" is narrow: every directive the format defines is
//! recognised, and anything else is a panic naming the line. A fixture that is
//! quietly half-read reports a pass for cases nobody ran (docs/decisions.md
//! D-37's argument, applied to a reader rather than to a pin).

use pistol_core::{Coord, GameState, Player};

use super::position;

/// The tactical fixture's positions, in file order.
///
/// The format is documented in the file's own header; what matters here is the
/// `position set p1:… p2:… tomove:… phase:…` line, whose stones are listed in
/// PLAY ORDER, which is what makes the position one a legal game reaches
/// (docs/decisions.md D-6, D-42).
pub fn parse_tactical(text: &str, source: &str) -> Vec<(String, GameState)> {
    let mut cases = Vec::new();
    let mut name: Option<String> = None;
    let mut state: Option<GameState> = None;

    for (number, line) in directives(text) {
        let at = || format!("{source}:{number}");
        let mut words = line.split_whitespace();
        let directive = words.next().expect("a directive line is not blank");
        match directive {
            "require" => {
                words
                    .next()
                    .and_then(|count| count.parse::<u32>().ok())
                    .unwrap_or_else(|| panic!("{}: `require` wants a count: {line}", at()));
            }
            "case" => {
                flush(&mut cases, &mut name, &mut state, &at());
                name = Some(
                    words
                        .next()
                        .unwrap_or_else(|| panic!("{}: `case` wants a name", at()))
                        .to_string(),
                );
            }
            "position" => {
                assert!(name.is_some(), "{}: a position outside a case", at());
                assert!(state.is_none(), "{}: a second position in one case", at());
                state = Some(parse_set(words.collect::<Vec<_>>().as_slice(), &at()));
            }
            "config" | "budget" | "expect" => {}
            other => panic!("{}: unknown directive `{other}`: {line}", at()),
        }
    }
    flush(&mut cases, &mut name, &mut state, source);
    cases
}

/// The perft fixture's positions, in file order — including the decided one.
pub fn parse_perft(text: &str, source: &str) -> Vec<(String, GameState)> {
    let mut cases = Vec::new();
    let mut name: Option<String> = None;
    let mut plies: Vec<Coord> = Vec::new();

    for (number, line) in directives(text) {
        let at = || format!("{source}:{number}");
        let mut words = line.split_whitespace();
        let directive = words.next().expect("a directive line is not blank");
        match directive {
            "case" => {
                flush_plies(&mut cases, &mut name, &plies, &at());
                plies.clear();
                name = Some(
                    words
                        .next()
                        .unwrap_or_else(|| panic!("{}: `case` wants a name", at()))
                        .to_string(),
                );
            }
            "plies" => {
                assert!(name.is_some(), "{}: plies outside a case", at());
                plies.extend(words.map(|token| coord(token, &at())));
            }
            "expect" => {}
            other => panic!("{}: unknown directive `{other}`: {line}", at()),
        }
    }
    flush_plies(&mut cases, &mut name, &plies, source);
    cases
}

/// Close a perft case, refusing one that stated no stones.
///
/// `GameState::from_plies(&[])` is the empty board and is perfectly `Ok`, so a
/// case whose `plies` line was dropped would otherwise be replayed as a brand
/// new game under the missing case's name — ongoing, at a turn boundary, and
/// therefore passing every assertion in this suite. That is exactly the
/// quietly-half-read fixture this module exists to refuse, and it is likelier
/// here than anywhere because the file belongs to another crate
/// (docs/decisions.md D-122).
fn flush_plies(
    cases: &mut Vec<(String, GameState)>,
    name: &mut Option<String>,
    plies: &[Coord],
    at: &str,
) {
    let Some(name) = name.take() else {
        return;
    };
    assert!(
        !plies.is_empty(),
        "{at}: case `{name}` states no plies, and the empty board is not a case"
    );
    cases.push((name, replay(plies, at)));
}

/// The non-blank, non-comment lines of a fixture, with their line numbers.
fn directives(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()))
        .filter(|(_, line)| !line.is_empty() && !line.starts_with('#'))
}

/// `set p1:… p2:… tomove:… phase:…` as the position it names.
fn parse_set(words: &[&str], at: &str) -> GameState {
    assert_eq!(
        words.first().copied(),
        Some("set"),
        "{at}: the oracle reads the `set` form only"
    );
    let (mut p1, mut p2) = (Vec::new(), Vec::new());
    let (mut to_move, mut phase) = (None, None);
    let mut key: Option<&str> = None;

    for &word in &words[1..] {
        let value = match word.split_once(':') {
            Some((field, rest)) => {
                key = Some(field);
                if rest.is_empty() {
                    continue;
                }
                rest
            }
            None => word,
        };
        match key.unwrap_or_else(|| panic!("{at}: `{word}` belongs to no field")) {
            "p1" => p1.push(coord(value, at)),
            "p2" => p2.push(coord(value, at)),
            // A repeated `p1:`/`p2:` key continues its list, which is the
            // format's own semantics. A repeated scalar is a contradiction, and
            // last-one-wins would silently pick a side of it.
            "tomove" => to_move = Some(once("tomove", to_move, player(value, at), at)),
            "phase" => phase = Some(once("phase", phase, value.to_string(), at)),
            other => panic!("{at}: unknown field `{other}`"),
        }
    }

    assert_eq!(
        phase.as_deref(),
        Some("0"),
        "{at}: the oracle searches turn boundaries, and every tactical case states phase 0"
    );
    position(
        &p1,
        &p2,
        to_move.unwrap_or_else(|| panic!("{at}: no `tomove`")),
    )
}

fn flush(
    cases: &mut Vec<(String, GameState)>,
    name: &mut Option<String>,
    state: &mut Option<GameState>,
    at: &str,
) {
    let Some(name) = name.take() else {
        return;
    };
    let state = state
        .take()
        .unwrap_or_else(|| panic!("{at}: case `{name}` has no position"));
    cases.push((name, state));
}

fn replay(plies: &[Coord], at: &str) -> GameState {
    GameState::from_plies(plies)
        .unwrap_or_else(|error| panic!("{at}: the ply list is not a legal game: {error}"))
}

/// A field that may be stated once, stated once.
fn once<T>(field: &str, held: Option<T>, value: T, at: &str) -> T {
    assert!(
        held.is_none(),
        "{at}: `{field}` is stated twice, and this reader will not pick one of them"
    );
    value
}

fn coord(token: &str, at: &str) -> Coord {
    token
        .parse()
        .unwrap_or_else(|error| panic!("{at}: `{token}` is not a cell: {error}"))
}

fn player(token: &str, at: &str) -> Player {
    match token {
        "p1" => Player::P1,
        "p2" => Player::P2,
        other => panic!("{at}: `{other}` is not a player"),
    }
}
