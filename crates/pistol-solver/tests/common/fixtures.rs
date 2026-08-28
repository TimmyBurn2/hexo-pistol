use std::collections::BTreeMap;

use pistol_core::window::Window;
use pistol_core::{Coord, Outcome, Phase, Player};
use pistol_solver::{Cover, MinimalCover, WinWitness};

use super::{
    cell_list, fixture_text, parse_coord, parse_coords, parse_player, parse_window, window_list,
    window_token,
};

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

impl SideExpectation {
    /// Whether every row for this side is one of the grammar's THREE NEGATIVE
    /// SPELLINGS — `-`, `nothing`, `none`.
    ///
    /// This is a property of the RECORD and not of the state, and it is the
    /// operational predicate for the one drift the derivation cannot see. The
    /// regeneration in `threat_v0_is_what_the_reference_prints` catches an
    /// edited `plies` line whenever the edit moves any derived answer; what it
    /// cannot catch is an ANSWER-INVARIANT edit, and answer-invariance is total
    /// exactly where there is no positive answer to move. A record every one of
    /// whose rows reads negative therefore has no protection from the
    /// derivation at all, and needs a row that states its own precondition
    /// (docs/decisions.md D-259, D-260, D-264).
    pub fn states_nothing(&self) -> bool {
        self.hot.is_empty()
            && self.win1.is_empty()
            && self.completed.is_empty()
            && self.live3.is_empty()
            && self.live2.is_empty()
            && self.threat_cells.is_empty()
            && self.raise_cells.is_empty()
            && self
                .cover
                .iter()
                .all(|cover| *cover == Cover::NothingToBlock)
            && self.canwin.iter().all(Option::is_none)
    }
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

/// One record, rendered in the fixture's own grammar — the inverse of
/// [`parse_cases`], and here for the reason the required-key set is here: one
/// grammar, one file.
///
/// It is not a convenience. `threat_v0_is_what_the_reference_prints` renders
/// every case from the from-scratch reference in [`super::reference`] and
/// compares the result with the pinned bytes, so the fixture's expectations are
/// DERIVED from R1 over the committed ply lists rather than merely hashed. An
/// edit to an expectation then has to be justified against the reference; a
/// re-hash alone no longer makes the suite green (docs/decisions.md D-259).
///
/// The layout is the file's: `expect ` then the key padded to [`KEY_WIDTH`]
/// then one space then the value, except the state row, whose key is followed
/// by two spaces because the row's own value starts with a key of its own.
pub fn render_case(case: &ThreatCase) -> String {
    let mut text = String::new();
    text.push_str(&format!("case {}\n", case.name));
    text.push_str(&format!("plies {}\n", cell_list(&case.plies)));
    for side in [Player::P1, Player::P2] {
        let tag = player_token(side);
        let want = case.side(side);
        let mut row = |what: &str, value: String| {
            text.push_str(&format!(
                "expect {:<KEY_WIDTH$} {value}\n",
                format!("{tag} {what}"),
                KEY_WIDTH = KEY_WIDTH
            ));
        };
        row("hot", window_list(&want.hot));
        row("win1", window_list(&want.win1));
        row("completed", window_list(&want.completed));
        row("live3", window_list(&want.live3));
        row("live2", window_list(&want.live2));
        row("threat_cells", cell_list(&want.threat_cells));
        row("raise_cells", cell_list(&want.raise_cells));
        for (index, budget) in ["1", "2"].into_iter().enumerate() {
            row(&format!("cover {budget}"), cover_text(&want.cover[index]));
        }
        for (index, budget) in ["1", "2"].into_iter().enumerate() {
            row(
                &format!("canwin {budget}"),
                canwin_text(&want.canwin[index]),
            );
        }
    }
    text.push_str(&format!("expect state  {}\n", state_text(&case.state)));
    text.push_str("end\n");
    text
}

/// The width the `expect` key is padded to, which is the widest key there is
/// (`p1 threat_cells`). Derived from the keys rather than written twice: a
/// column that drifts from the file is a diff nobody can read.
const KEY_WIDTH: usize = "p1 threat_cells".len();

/// `p1` or `p2` — the fixture's spelling, which is not [`Player`]'s `Display`.
fn player_token(side: Player) -> &'static str {
    match side {
        Player::P1 => "p1",
        Player::P2 => "p2",
    }
}

/// A cover row's value: the three spellings, and never an empty brace list.
fn cover_text(cover: &Cover) -> String {
    match cover {
        Cover::NothingToBlock => String::from("nothing"),
        Cover::Impossible => String::from("impossible"),
        Cover::Minimal(covers) => covers
            .iter()
            .map(|cover| match cover {
                MinimalCover::One(at) => format!("{{{at}}}"),
                MinimalCover::Two { first, second } => format!("{{{first} {second}}}"),
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

/// A canwin row's value, carrying the witness WINDOW as well as the cells.
fn canwin_text(witness: &Option<WinWitness>) -> String {
    match witness {
        None => String::from("none"),
        Some(WinWitness::OnePly { at, window }) => {
            format!("oneply {at} {}", window_token(*window))
        }
        Some(WinWitness::Pair {
            first,
            second,
            window,
        }) => format!("pair {first} {second} {}", window_token(*window)),
    }
}

/// The state row's value: the position's own facts, from `GameState` and not
/// from the reference.
fn state_text(state: &StateExpectation) -> String {
    let phase = match state.phase {
        Phase::First => "First",
        Phase::Second => "Second",
    };
    let outcome = match state.outcome {
        Outcome::Ongoing => String::from("ongoing"),
        Outcome::Win { winner, turn } => format!("win {} turn {turn}", player_token(winner)),
    };
    format!(
        "to_move {} phase {phase} stones_owed {} outcome {outcome}",
        player_token(state.to_move),
        state.stones_owed
    )
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
