use pistol_core::{Coord, Player};

use super::plans::{Plan, plan_list};
use super::{parse_coords, parse_player};

/// The pattern pack, under `tests/fixtures/`.
pub const PATTERN_FIXTURE_FILE: &str = "pattern_v0.txt";

/// One record: a named pattern, the position, and what the definitions say.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCase {
    /// The calculus's own id — `PAT-GAP`, `ALG-CROSS-BOTH`.
    pub name: String,
    /// The line the record opened on, for a failure that can be found.
    pub line: usize,
    /// The stones, in play order.
    pub plies: Vec<Coord>,
    /// Whose threats the expectations are about.
    pub side: Player,
    /// DEF-SUPPORT: the largest own-stone count over `side`'s open windows.
    pub support: u32,
    /// DEF-PLAN: the plan family, deduplicated and sorted.
    pub plans: Vec<Plan>,
    /// DEF-T: the exact minimum hitting set over that family.
    pub t: usize,
}

/// How a record is written back out — the derivation guard compares this text
/// with the file's own bytes.
pub fn render_case(case: &PatternCase) -> String {
    let side = match case.side {
        Player::P1 => "p1",
        Player::P2 => "p2",
    };
    let plies: Vec<String> = case.plies.iter().map(Coord::to_string).collect();
    format!(
        "case    {}\nplies   {}\nside    {side}\nexpect  support {}\nexpect  plans   {}\n\
         expect  t       {}\nend\n",
        case.name,
        plies.join(" "),
        case.support,
        plan_list(&case.plans),
        case.t
    )
}

/// Every record in the pack, in file order.
pub fn pattern_cases() -> Vec<PatternCase> {
    parse_cases(&super::fixture_text(PATTERN_FIXTURE_FILE))
}

/// One record by name.
pub fn pattern_case(name: &str) -> PatternCase {
    pattern_cases()
        .into_iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("{PATTERN_FIXTURE_FILE} has no record named {name}"))
}

/// What a record is part-way through being.
#[derive(Default)]
struct Partial {
    name: String,
    line: usize,
    plies: Option<Vec<Coord>>,
    side: Option<Player>,
    support: Option<u32>,
    plans: Option<Vec<Plan>>,
    t: Option<usize>,
}

fn parse_cases(text: &str) -> Vec<PatternCase> {
    let mut cases: Vec<PatternCase> = Vec::new();
    let mut open: Option<Partial> = None;
    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        let body = raw.split('#').next().unwrap_or("").trim();
        if body.is_empty() {
            continue;
        }
        let (key, rest) = match body.split_once(char::is_whitespace) {
            Some((key, rest)) => (key, rest.trim()),
            None => (body, ""),
        };
        match key {
            "case" => {
                assert!(
                    open.is_none(),
                    "line {line}: `case {rest}` opens while a record is still open"
                );
                assert!(!rest.is_empty(), "line {line}: a case needs a name");
                open = Some(Partial {
                    name: rest.to_owned(),
                    line,
                    ..Partial::default()
                });
            }
            "plies" | "side" | "expect" => {
                let partial = open
                    .as_mut()
                    .unwrap_or_else(|| panic!("line {line}: `{key}` outside a record"));
                fill(partial, key, rest, line);
            }
            "end" => {
                let partial = open
                    .take()
                    .unwrap_or_else(|| panic!("line {line}: `end` closes nothing"));
                cases.push(finish(partial, line));
            }
            other => panic!("line {line}: `{other}` is not a key this loader knows"),
        }
    }
    assert!(
        open.is_none(),
        "{PATTERN_FIXTURE_FILE} ends with a record left open"
    );
    assert!(!cases.is_empty(), "{PATTERN_FIXTURE_FILE} holds no records");
    cases
}

fn fill(partial: &mut Partial, key: &str, rest: &str, line: usize) {
    match key {
        "plies" => set(&mut partial.plies, parse_coords(rest, line), "plies", line),
        "side" => set(&mut partial.side, parse_player(rest, line), "side", line),
        "expect" => {
            let (what, value) = rest
                .split_once(char::is_whitespace)
                .unwrap_or_else(|| panic!("line {line}: `expect {rest}` states no value"));
            let value = value.trim();
            match what {
                "support" => set(&mut partial.support, number(value, line), "support", line),
                "plans" => set(
                    &mut partial.plans,
                    parse_plans(value, line),
                    "expect plans",
                    line,
                ),
                "t" => set(
                    &mut partial.t,
                    number(value, line) as usize,
                    "expect t",
                    line,
                ),
                other => panic!("line {line}: `expect {other}` is not an expectation kind"),
            }
        }
        other => unreachable!("line {line}: `{other}` reached fill"),
    }
}

fn set<T>(slot: &mut Option<T>, value: T, what: &str, line: usize) {
    assert!(slot.is_none(), "line {line}: `{what}` is stated twice");
    *slot = Some(value);
}

fn number(text: &str, line: usize) -> u32 {
    text.parse()
        .unwrap_or_else(|_| panic!("line {line}: `{text}` is not a count"))
}

/// `-` for none, otherwise `{q,r}` and `{q,r q,r}` tokens.
fn parse_plans(text: &str, line: usize) -> Vec<Plan> {
    if text == "-" {
        return Vec::new();
    }
    let mut plans = Vec::new();
    for token in text.split_inclusive('}') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let inner = token
            .strip_prefix('{')
            .and_then(|rest| rest.strip_suffix('}'))
            .unwrap_or_else(|| panic!("line {line}: `{token}` is not a `{{q,r ...}}` plan"));
        let cells = parse_coords(inner, line);
        assert!(
            !cells.is_empty() && cells.len() <= 2,
            "line {line}: a plan holds one or two cells, `{token}` holds {}",
            cells.len()
        );
        plans.push(cells);
    }
    plans
}

fn finish(partial: Partial, line: usize) -> PatternCase {
    let missing = |what: &str| -> ! {
        panic!(
            "line {line}: record `{}` (opened at line {}) states no `{what}`",
            partial.name, partial.line
        )
    };
    PatternCase {
        plies: partial.plies.clone().unwrap_or_else(|| missing("plies")),
        side: partial.side.unwrap_or_else(|| missing("side")),
        support: partial.support.unwrap_or_else(|| missing("expect support")),
        plans: partial
            .plans
            .clone()
            .unwrap_or_else(|| missing("expect plans")),
        t: partial.t.unwrap_or_else(|| missing("expect t")),
        name: partial.name,
        line: partial.line,
    }
}
