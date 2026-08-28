use pistol_core::{Coord, Phase, Player};

use super::{directives, fixture_text, parse_coords};

/// The fixture's name under `tests/fixtures/`.
pub const GOLDEN_GAMES_FILE: &str = "golden_games_v1.txt";

/// What the fixture says the last ply did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVerdict {
    /// The game is undecided; the state after the last ply is this.
    Ongoing {
        /// Whose turn it now is, by number.
        turn: u32,
        /// How far into that turn the mover is.
        phase: Phase,
    },
    /// The last ply completed a run.
    Win {
        /// Who placed it.
        winner: Player,
        /// The turn it is scored on.
        turn: u32,
        /// The phase the winning stone was placed AT — `First` means it was
        /// that turn's first stone, so the second is never played (rule 4).
        phase: Phase,
    },
}

/// One golden game.
#[derive(Debug, Clone)]
pub struct GoldenGame {
    /// The case name, as written in the fixture.
    pub name: String,
    /// Every stone, in play order.
    pub plies: Vec<Coord>,
    /// What the last ply did.
    pub expect: GameVerdict,
    /// The fixture line the case starts on, for failure messages.
    pub line: usize,
}

impl GoldenGame {
    /// The game up to but not including the last ply, and that ply.
    pub fn split_last(&self) -> (&[Coord], Coord) {
        let (last, prefix) = self
            .plies
            .split_last()
            .unwrap_or_else(|| panic!("case `{}` has no plies", self.name));
        (prefix, *last)
    }
}

/// Every golden game, in fixture order.
pub fn golden_games() -> Vec<GoldenGame> {
    parse_games(&fixture_text(GOLDEN_GAMES_FILE))
}

/// One golden game by name. Panics if the fixture has no such case — a test
/// that silently found nothing to run is worse than a failing one.
pub fn golden_game(name: &str) -> GoldenGame {
    golden_games()
        .into_iter()
        .find(|game| game.name == name)
        .unwrap_or_else(|| panic!("the golden-game fixture has no case `{name}`"))
}

/// Parse the format described in the fixture's own header.
pub fn parse_games(text: &str) -> Vec<GoldenGame> {
    let mut games: Vec<GoldenGame> = Vec::new();
    let mut open: Option<Partial> = None;

    for (line, directive, rest) in directives(text) {
        if directive == "case" {
            if let Some(partial) = open.take() {
                games.push(partial.finish());
            }
            assert!(!rest.is_empty(), "line {line}: `case` needs a name");
            assert!(
                !games.iter().any(|game: &GoldenGame| game.name == rest),
                "line {line}: a second case is named `{rest}`, and `golden_game` looks cases up \
                 by name"
            );
            open = Some(Partial::new(rest.to_string(), line));
            continue;
        }
        let partial = open
            .as_mut()
            .unwrap_or_else(|| panic!("line {line}: `{directive}` before any `case`"));
        match directive {
            "plies" => {
                assert!(!rest.is_empty(), "line {line}: `plies` with no stones");
                partial.plies.extend(parse_coords(rest, line));
            }
            "expect" => {
                let verdict = parse_verdict(rest, line);
                assert!(
                    partial.expect.replace(verdict).is_none(),
                    "line {line}: `expect` given twice for case `{}`",
                    partial.name
                );
            }
            other => panic!("line {line}: unknown directive `{other}`"),
        }
    }
    if let Some(partial) = open.take() {
        games.push(partial.finish());
    }
    assert!(!games.is_empty(), "the fixture holds no games at all");
    games
}

/// `ongoing turn <n> phase <first|second>`, or
/// `win <player> on-turn <n> as <first|second>-stone`.
fn parse_verdict(rest: &str, line: usize) -> GameVerdict {
    let words: Vec<&str> = rest.split_whitespace().collect();
    match words.as_slice() {
        ["ongoing", "turn", turn, "phase", phase] => GameVerdict::Ongoing {
            turn: parse_turn(turn, line),
            phase: parse_phase(phase, line),
        },
        ["win", player, "on-turn", turn, "as", stone] => {
            let phase = stone
                .strip_suffix("-stone")
                .unwrap_or_else(|| panic!("line {line}: `as` takes first-stone or second-stone"));
            GameVerdict::Win {
                winner: parse_player(player, line),
                turn: parse_turn(turn, line),
                phase: parse_phase(phase, line),
            }
        }
        _ => panic!("line {line}: `expect` is not one of the two forms: `{rest}`"),
    }
}

fn parse_turn(word: &str, line: usize) -> u32 {
    assert!(
        !word.starts_with('+'),
        "line {line}: a turn number is written without a `+`"
    );
    word.parse()
        .unwrap_or_else(|_| panic!("line {line}: `{word}` is not a turn number"))
}

fn parse_phase(word: &str, line: usize) -> Phase {
    match word {
        "first" => Phase::First,
        "second" => Phase::Second,
        other => panic!("line {line}: a phase is first or second, got `{other}`"),
    }
}

fn parse_player(word: &str, line: usize) -> Player {
    match word {
        "p1" => Player::P1,
        "p2" => Player::P2,
        other => panic!("line {line}: a player is p1 or p2, got `{other}`"),
    }
}

/// A case being read.
struct Partial {
    name: String,
    line: usize,
    plies: Vec<Coord>,
    expect: Option<GameVerdict>,
}

impl Partial {
    fn new(name: String, line: usize) -> Self {
        Partial {
            name,
            line,
            plies: Vec::new(),
            expect: None,
        }
    }

    fn finish(self) -> GoldenGame {
        let name = self.name;
        let line = self.line;
        assert!(
            !self.plies.is_empty(),
            "case `{name}` (line {line}) has no plies"
        );
        let expect = self
            .expect
            .unwrap_or_else(|| panic!("case `{name}` (line {line}) has no `expect`"));
        GoldenGame {
            name,
            plies: self.plies,
            expect,
            line,
        }
    }
}
