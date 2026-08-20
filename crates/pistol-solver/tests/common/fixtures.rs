//! The golden threat fixture: fourteen positions, every query's answer for both
//! sides, sha-pinned (CLAUDE.md rule 7).
//!
//! # The grammar
//!
//! One record per case, line-oriented, `#` to end of line is a comment, the key
//! is the leading token and the value is the rest — the idiom of
//! `tactical_v0.txt` and of the arena's report (docs/decisions.md D-139, D-147,
//! D-161).
//!
//! ```text
//! case    <name>
//! plies   <q,r> ...                       # play order; legality comes from core
//! expect  <side> hot          <axis>@<q,r> ...   | -
//! expect  <side> win1         ...
//! expect  <side> completed    ...
//! expect  <side> live3        ...
//! expect  <side> live2        ...
//! expect  <side> threat_cells <q,r> ...          | -
//! expect  <side> raise_cells  <q,r> ...          | -
//! expect  <side> cover  <1|2> nothing | impossible | {<q,r>} {<q,r> <q,r>} ...
//! expect  <side> canwin <1|2> none | oneply <q,r> <axis>@<q,r>
//!                                              | pair <q,r> <q,r> <axis>@<q,r>
//! expect  state  to_move <side> phase <First|Second> stones_owed <n>
//!                outcome ongoing | outcome win <side> turn <n>
//! end
//! ```
//!
//! **`cover` and `canwin` are read from the side that OWNS the threat.** A
//! `cover` row under `p1` is `blocking_covers(P2, budget)`: the covers of P1's
//! hot windows, which is what P2 would have to play. Keeping every row of a
//! record about one side's threats is what makes the record readable; the
//! defender is always the other one.
//!
//! A parser that refuses an unknown key and accepts a missing one lets a fixture
//! pass by omission, so this one refuses both, by name.
//!
//! # RULE9-JUSTIFICATION: one grammar, one reader (CLAUDE.md rule 9).
//!
//! The record type, the required-key set and the value parsers are one
//! statement of one grammar. Splitting them puts the list of keys a record must
//! carry in a different file from the parser that reads them, which is exactly
//! the drift the missing-key refusal exists to catch. It shrinks when a second
//! fixture class arrives and the record-scanning skeleton becomes worth
//! sharing.

use std::collections::BTreeMap;

use pistol_core::window::Window;
use pistol_core::{Coord, Outcome, Phase, Player};
use pistol_solver::{Cover, MinimalCover, WinWitness};

use super::{fixture_text, parse_coord, parse_coords, parse_player, parse_window};

/// The fixture's name under `tests/fixtures/`.
pub const THREAT_FIXTURE_FILE: &str = "threat_v0.txt";

/// Every expectation about one side.
#[derive(Debug, Clone)]
pub struct SideExpectation {
    /// `hot_windows`.
    pub hot: Vec<Window>,
    /// `win_in_one_ply_windows`.
    pub win1: Vec<Window>,
    /// `completed_windows`.
    pub completed: Vec<Window>,
    /// `live_windows_at_count(.., Three)`.
    pub live3: Vec<Window>,
    /// `live_windows_at_count(.., Two)`.
    pub live2: Vec<Window>,
    /// `threat_cells`.
    pub threat_cells: Vec<Coord>,
    /// `cells_raising_to_hot(.., Three)`.
    pub raise_cells: Vec<Coord>,
    /// `blocking_covers(other side, One)` then `(.., Two)`.
    pub cover: [Cover; 2],
    /// `can_win_this_turn(side, One)` then `(.., Two)`.
    pub canwin: [Option<WinWitness>; 2],
}

/// What the position itself is, so a fixture cannot silently be about a
/// different turn or phase than it was written for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateExpectation {
    /// Whose stone is next.
    pub to_move: Player,
    /// How far into the turn.
    pub phase: Phase,
    /// What the turn still owes.
    pub stones_owed: u32,
    /// Ongoing, or won by whom on which turn.
    pub outcome: Outcome,
}

/// One golden case.
#[derive(Debug, Clone)]
pub struct ThreatCase {
    /// The case name, which is its identity.
    pub name: String,
    /// The line the record starts on, for failure messages.
    pub line: usize,
    /// The plies, in play order.
    pub plies: Vec<Coord>,
    /// P1's expectations, then P2's.
    pub sides: [SideExpectation; 2],
    /// The position's own facts.
    pub state: StateExpectation,
}

impl ThreatCase {
    /// The expectations for `side`.
    pub fn side(&self, side: Player) -> &SideExpectation {
        match side {
            Player::P1 => &self.sides[0],
            Player::P2 => &self.sides[1],
        }
    }
}

/// Every case in the golden fixture, in file order.
pub fn threat_cases() -> Vec<ThreatCase> {
    parse_cases(&fixture_text(THREAT_FIXTURE_FILE))
}

/// The case with this name; panics if the fixture does not carry it, because a
/// test that silently skips its own position is worse than no test.
pub fn threat_case(name: &str) -> ThreatCase {
    threat_cases()
        .into_iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("{THREAT_FIXTURE_FILE} has no case `{name}`"))
}

/// Every `expect` key a record must carry, exactly once.
fn required_keys() -> Vec<String> {
    let mut keys = vec![String::from("state")];
    for side in ["p1", "p2"] {
        for what in [
            "hot",
            "win1",
            "completed",
            "live3",
            "live2",
            "threat_cells",
            "raise_cells",
        ] {
            keys.push(format!("{side} {what}"));
        }
        for budget in ["1", "2"] {
            keys.push(format!("{side} cover {budget}"));
            keys.push(format!("{side} canwin {budget}"));
        }
    }
    keys
}

/// One `expect` value and the line it was written on.
type Stated = (usize, String);

/// A record being read: its name, its first line, its plies, and what it has
/// stated so far.
type OpenCase = (String, usize, Vec<Coord>, BTreeMap<String, Stated>);

fn parse_cases(text: &str) -> Vec<ThreatCase> {
    let mut cases = Vec::new();
    let mut open: Option<OpenCase> = None;
    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        let content = match raw.split_once('#') {
            Some((before, _)) => before.trim(),
            None => raw.trim(),
        };
        if content.is_empty() {
            continue;
        }
        let (key, rest) = match content.split_once(char::is_whitespace) {
            Some((key, rest)) => (key, rest.trim()),
            None => (content, ""),
        };
        match key {
            "case" => {
                assert!(
                    open.is_none(),
                    "line {line}: `case` inside an unclosed case"
                );
                assert!(!rest.is_empty(), "line {line}: `case` needs a name");
                open = Some((rest.to_string(), line, Vec::new(), BTreeMap::new()));
            }
            "plies" => {
                let (_, _, plies, _) = open
                    .as_mut()
                    .unwrap_or_else(|| panic!("line {line}: `plies` outside a case"));
                assert!(plies.is_empty(), "line {line}: a case has one `plies` line");
                *plies = parse_coords(rest, line);
            }
            "expect" => {
                let (name, _, _, seen) = open
                    .as_mut()
                    .unwrap_or_else(|| panic!("line {line}: `expect` outside a case"));
                let (subject, value) = split_expect_key(rest, line);
                if let Some((earlier, _)) = seen.insert(subject.clone(), (line, value)) {
                    panic!(
                        "line {line}: case `{name}` states `{subject}` twice (also line {earlier})"
                    );
                }
            }
            "end" => {
                let (name, start, plies, seen) = open
                    .take()
                    .unwrap_or_else(|| panic!("line {line}: `end` outside a case"));
                cases.push(finish(name, start, plies, seen, line));
            }
            other => panic!("line {line}: unknown directive `{other}`"),
        }
    }
    assert!(open.is_none(), "the last case has no `end`");
    assert!(!cases.is_empty(), "{THREAT_FIXTURE_FILE} holds no case");
    cases
}

/// `<side> <what> [budget] <value>` — the key is everything but the value.
fn split_expect_key(rest: &str, line: usize) -> (String, String) {
    let mut tokens = rest.split_whitespace();
    let head = tokens
        .next()
        .unwrap_or_else(|| panic!("line {line}: `expect` needs a subject"));
    if head == "state" {
        return (String::from("state"), tokens.collect::<Vec<_>>().join(" "));
    }
    parse_player(head, line);
    let what = tokens
        .next()
        .unwrap_or_else(|| panic!("line {line}: `expect {head}` names nothing"));
    let key = match what {
        "cover" | "canwin" => {
            let budget = tokens
                .next()
                .unwrap_or_else(|| panic!("line {line}: `expect {head} {what}` needs a budget"));
            assert!(
                budget == "1" || budget == "2",
                "line {line}: `{budget}` is not a budget this fixture spells"
            );
            format!("{head} {what} {budget}")
        }
        "hot" | "win1" | "completed" | "live3" | "live2" | "threat_cells" | "raise_cells" => {
            format!("{head} {what}")
        }
        other => panic!("line {line}: unknown expectation `{other}`"),
    };
    (key, tokens.collect::<Vec<_>>().join(" "))
}

fn finish(
    name: String,
    start: usize,
    plies: Vec<Coord>,
    seen: BTreeMap<String, Stated>,
    end_line: usize,
) -> ThreatCase {
    assert!(!plies.is_empty(), "case `{name}` has no `plies` line");
    let mut missing: Vec<String> = required_keys()
        .into_iter()
        .filter(|key| !seen.contains_key(key))
        .collect();
    missing.sort();
    assert!(
        missing.is_empty(),
        "case `{name}` (ends line {end_line}) is missing: {}",
        missing.join(", ")
    );
    let extra: Vec<&String> = seen
        .keys()
        .filter(|key| !required_keys().contains(key))
        .collect();
    assert!(
        extra.is_empty(),
        "case `{name}` states expectations nothing reads: {extra:?}"
    );
    let value = |key: &str| -> Stated { seen[key].clone() };
    let sides = [Player::P1, Player::P2].map(|side| {
        let tag = match side {
            Player::P1 => "p1",
            Player::P2 => "p2",
        };
        let windows = |what: &str| {
            let (line, text) = value(&format!("{tag} {what}"));
            parse_window_list(&text, line)
        };
        let cells = |what: &str| {
            let (line, text) = value(&format!("{tag} {what}"));
            parse_cell_list(&text, line)
        };
        SideExpectation {
            hot: windows("hot"),
            win1: windows("win1"),
            completed: windows("completed"),
            live3: windows("live3"),
            live2: windows("live2"),
            threat_cells: cells("threat_cells"),
            raise_cells: cells("raise_cells"),
            cover: ["1", "2"].map(|budget| {
                let (line, text) = value(&format!("{tag} cover {budget}"));
                parse_cover(&text, line)
            }),
            canwin: ["1", "2"].map(|budget| {
                let (line, text) = value(&format!("{tag} canwin {budget}"));
                parse_canwin(&text, line)
            }),
        }
    });
    let (state_line, state_text) = value("state");
    ThreatCase {
        name,
        line: start,
        plies,
        sides,
        state: parse_state(&state_text, state_line),
    }
}

fn parse_window_list(text: &str, line: usize) -> Vec<Window> {
    if text == "-" {
        return Vec::new();
    }
    assert!(
        !text.is_empty(),
        "line {line}: write `-` for an empty answer"
    );
    text.split_whitespace()
        .map(|token| parse_window(token, line))
        .collect()
}

fn parse_cell_list(text: &str, line: usize) -> Vec<Coord> {
    if text == "-" {
        return Vec::new();
    }
    assert!(
        !text.is_empty(),
        "line {line}: write `-` for an empty answer"
    );
    parse_coords(text, line)
}

fn parse_cover(text: &str, line: usize) -> Cover {
    match text {
        "nothing" => return Cover::NothingToBlock,
        "impossible" => return Cover::Impossible,
        "" => panic!("line {line}: a cover row states nothing"),
        _ => {}
    }
    let mut covers = Vec::new();
    let mut group: Vec<Coord> = Vec::new();
    let mut inside = false;
    for token in text.split_whitespace() {
        let opens = token.starts_with('{');
        let closes = token.ends_with('}');
        let body = token.trim_start_matches('{').trim_end_matches('}');
        if opens {
            assert!(!inside, "line {line}: `{{` inside a cover");
            inside = true;
        }
        assert!(inside, "line {line}: `{token}` is outside any cover");
        group.push(parse_coord(body, line));
        if closes {
            covers.push(match group.as_slice() {
                [only] => MinimalCover::One(*only),
                [first, second] => MinimalCover::Two {
                    first: *first,
                    second: *second,
                },
                other => panic!("line {line}: a cover of {} cells", other.len()),
            });
            group.clear();
            inside = false;
        }
    }
    assert!(!inside, "line {line}: a cover is not closed");
    assert!(
        !covers.is_empty(),
        "line {line}: write `impossible` or `nothing`, not an empty list"
    );
    Cover::Minimal(covers)
}

fn parse_canwin(text: &str, line: usize) -> Option<WinWitness> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    match tokens.as_slice() {
        ["none"] => None,
        ["oneply", at, window] => Some(WinWitness::OnePly {
            at: parse_coord(at, line),
            window: parse_window(window, line),
        }),
        ["pair", first, second, window] => Some(WinWitness::Pair {
            first: parse_coord(first, line),
            second: parse_coord(second, line),
            window: parse_window(window, line),
        }),
        other => panic!("line {line}: `{}` is not a witness", other.join(" ")),
    }
}

fn parse_state(text: &str, line: usize) -> StateExpectation {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    let expect = |index: usize, word: &str| {
        assert_eq!(
            tokens.get(index).copied(),
            Some(word),
            "line {line}: expected `{word}` as token {index} of the state row"
        );
    };
    expect(0, "to_move");
    expect(2, "phase");
    expect(4, "stones_owed");
    expect(6, "outcome");
    let to_move = parse_player(tokens[1], line);
    let phase = match tokens[3] {
        "First" => Phase::First,
        "Second" => Phase::Second,
        other => panic!("line {line}: `{other}` is not a phase"),
    };
    let stones_owed: u32 = tokens[5]
        .parse()
        .unwrap_or_else(|_| panic!("line {line}: `{}` is not a stone count", tokens[5]));
    let outcome = match tokens.get(7).copied() {
        Some("ongoing") => {
            assert_eq!(tokens.len(), 8, "line {line}: `ongoing` takes nothing more");
            Outcome::Ongoing
        }
        Some("win") => {
            assert_eq!(tokens.len(), 11, "line {line}: `win <side> turn <n>`");
            expect(9, "turn");
            Outcome::Win {
                winner: parse_player(tokens[8], line),
                turn: tokens[10]
                    .parse()
                    .unwrap_or_else(|_| panic!("line {line}: `{}` is not a turn", tokens[10])),
            }
        }
        other => panic!("line {line}: `{other:?}` is not an outcome"),
    };
    StateExpectation {
        to_move,
        phase,
        stones_owed,
        outcome,
    }
}
