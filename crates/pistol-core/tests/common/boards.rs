//! The golden-board fixture: positions, the stone played last, and the verdict
//! that stone must get (game rules 2 and 4).

use pistol_core::{Board, Coord, Player};

use super::{directives, fixture_text, parse_coord, parse_coords};

/// The fixture's name under `tests/fixtures/`.
pub const GOLDEN_BOARDS_FILE: &str = "golden_boards_v1.txt";

/// One golden board.
#[derive(Debug, Clone)]
pub struct GoldenCase {
    /// The case name, as written in the fixture.
    pub name: String,
    /// Whether the last stone completes a run of six or more.
    pub expect_win: bool,
    /// Every stone, in the order the fixture lists them.
    pub stones: Vec<(Coord, Player)>,
    /// The stone the verdict is about.
    pub last: Coord,
    /// Its player.
    pub last_player: Player,
    /// The fixture line the case starts on, for failure messages.
    pub line: usize,
}

impl GoldenCase {
    /// The position as written.
    pub fn board(&self) -> Board {
        self.board_from(self.stones.iter().copied())
    }

    /// The position without the last stone — the board as it was the instant
    /// before the verdict.
    pub fn board_without_last(&self) -> Board {
        self.board_from(
            self.stones
                .iter()
                .copied()
                .filter(|&(at, _)| at != self.last),
        )
    }

    fn board_from(&self, stones: impl Iterator<Item = (Coord, Player)>) -> Board {
        let mut board = Board::empty();
        for (at, player) in stones {
            board.apply(at, player).unwrap_or_else(|error| {
                panic!("case `{}` (line {}): {error}", self.name, self.line)
            });
        }
        board
    }
}

/// Every golden board, in fixture order.
pub fn golden_cases() -> Vec<GoldenCase> {
    parse_boards(&fixture_text(GOLDEN_BOARDS_FILE))
}

/// Parse the format described in the fixture's own header.
pub fn parse_boards(text: &str) -> Vec<GoldenCase> {
    let mut cases: Vec<GoldenCase> = Vec::new();
    let mut open: Option<Partial> = None;

    for (line, directive, rest) in directives(text) {
        if directive == "case" {
            if let Some(partial) = open.take() {
                cases.push(partial.finish());
            }
            assert!(!rest.is_empty(), "line {line}: `case` needs a name");
            assert!(
                !cases.iter().any(|case: &GoldenCase| case.name == rest),
                "line {line}: a second case is named `{rest}`"
            );
            open = Some(Partial::new(rest.to_string(), line));
            continue;
        }
        let partial = open
            .as_mut()
            .unwrap_or_else(|| panic!("line {line}: `{directive}` before any `case`"));
        partial.directive(directive, rest, line);
    }
    if let Some(partial) = open.take() {
        cases.push(partial.finish());
    }
    assert!(!cases.is_empty(), "the fixture holds no cases at all");
    cases
}

/// A case being read.
struct Partial {
    name: String,
    line: usize,
    expect_win: Option<bool>,
    stones: Vec<(Coord, Player)>,
    last: Option<(Coord, Player)>,
}

impl Partial {
    fn new(name: String, line: usize) -> Self {
        Partial {
            name,
            line,
            expect_win: None,
            stones: Vec::new(),
            last: None,
        }
    }

    fn directive(&mut self, directive: &str, rest: &str, line: usize) {
        match directive {
            "expect" => {
                let verdict = match rest {
                    "win" => true,
                    "no-win" => false,
                    other => panic!("line {line}: `expect` takes win or no-win, got `{other}`"),
                };
                assert!(
                    self.expect_win.replace(verdict).is_none(),
                    "line {line}: `expect` given twice for case `{}`",
                    self.name
                );
            }
            "p1" | "p2" => {
                let player = if directive == "p1" {
                    Player::P1
                } else {
                    Player::P2
                };
                assert!(
                    !rest.is_empty(),
                    "line {line}: `{directive}` with no stones"
                );
                for at in parse_coords(rest, line) {
                    assert!(
                        !self.stones.iter().any(|&(other, _)| other == at),
                        "line {line}: cell {at} listed twice in case `{}`",
                        self.name
                    );
                    self.stones.push((at, player));
                }
            }
            "last" => {
                let (player_word, token) = rest
                    .split_once(char::is_whitespace)
                    .unwrap_or_else(|| panic!("line {line}: `last` takes a player and a cell"));
                let player = match player_word.trim() {
                    "p1" => Player::P1,
                    "p2" => Player::P2,
                    other => panic!("line {line}: `last` takes p1 or p2, got `{other}`"),
                };
                let at = parse_coord(token.trim(), line);
                assert!(
                    self.last.replace((at, player)).is_none(),
                    "line {line}: `last` given twice for case `{}`",
                    self.name
                );
            }
            other => panic!("line {line}: unknown directive `{other}`"),
        }
    }

    fn finish(self) -> GoldenCase {
        let name = self.name;
        let line = self.line;
        let expect_win = self
            .expect_win
            .unwrap_or_else(|| panic!("case `{name}` (line {line}) has no `expect`"));
        let (last, last_player) = self
            .last
            .unwrap_or_else(|| panic!("case `{name}` (line {line}) has no `last`"));
        assert!(
            self.stones.contains(&(last, last_player)),
            "case `{name}` (line {line}): the last stone {last_player} {last} is not among that \
             player's stones"
        );
        GoldenCase {
            name,
            expect_win,
            stones: self.stones,
            last,
            last_player,
            line,
        }
    }
}
