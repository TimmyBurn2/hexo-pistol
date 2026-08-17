//! The brute-force reference generator: the movegen oracle (CLAUDE.md rule 7,
//! docs/decisions.md D-12).
//!
//! This is a second, independent reading of rules 3, 4 and 5, written the slow
//! obvious way so that it has no bug in common with the fast one. It shares no
//! code with `pistol_core::movegen` beyond the types every reading of the rules
//! must agree on — [`Board`] and [`Coord`] as containers, [`wins_at`] because
//! win detection is rules truth that lives in one place (CLAUDE.md rule 2), and
//! [`LEGAL_RADIUS`] because restating `8` here would make the test tree a
//! second source of a pinned rule constant rather than a second implementation
//! of the rule.
//!
//! Where the real generator enumerates the balls around the stones and reasons
//! about which end of a pair may be played first, this one:
//!
//! 1. sweeps a bounding box big enough to contain anything reachable;
//! 2. asks, cell by cell, whether a stone may go there — by scanning every
//!    stone on the board and measuring, which is rule 5 said literally;
//! 3. enumerates **ordered** placements, first stone then second, replaying the
//!    legality question on the board the first stone has already changed;
//! 4. truncates a first stone that completes a line to a turn of one (rule 4);
//! 5. drops the ordering by collecting into a `BTreeSet` of canonical turns.
//!
//! It also keeps its own game state — board, side, turn number, decided — and
//! replays a move list by its own grouping of plies into turns (1, then 2, then
//! 2, …), so that a fixture position is reached twice by two implementations
//! and compared.
//!
//! The box is padded by `2 * LEGAL_RADIUS + 1` around the stones. A second
//! stone is within `LEGAL_RADIUS` of a first, which is within `LEGAL_RADIUS` of
//! a stone, and hex distance bounds both axial components (`dist >= |dq|` and
//! `dist >= |dr|`), so nothing legal can fall outside it.

use std::collections::BTreeSet;

use pistol_core::{Board, Coord, FIRST_TURN, LEGAL_RADIUS, Player, stones_in_turn, wins_at};

/// A turn as the reference spells it: its own type, canonicalized by its own
/// comparison, so that agreement with `pistol_core::Turn` is a result and not a
/// shared assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RefTurn {
    /// One stone: turn 1, or a stone that completes a line.
    Single(Coord),
    /// Two distinct cells, smaller first.
    Pair(Coord, Coord),
}

impl RefTurn {
    /// The canonical spelling of a turn over two distinct cells.
    fn pair(a: Coord, b: Coord) -> RefTurn {
        assert!(a != b, "a pair is two distinct cells");
        if a < b {
            RefTurn::Pair(a, b)
        } else {
            RefTurn::Pair(b, a)
        }
    }
}

/// A game the reference plays by its own reading of the rules.
#[derive(Debug, Clone)]
pub struct RefGame {
    board: Board,
    to_move: Player,
    turn: u32,
    decided: bool,
}

impl RefGame {
    /// A new game: no stones, P1 to move, turn one.
    pub fn new() -> RefGame {
        RefGame {
            board: Board::empty(),
            to_move: Player::P1,
            turn: FIRST_TURN,
            decided: false,
        }
    }

    /// Replay a move list, grouping plies into turns the way rule 3 does: one
    /// stone on turn 1, two on every later turn, and none after a stone
    /// completes a line (rule 4).
    pub fn from_plies(plies: &[Coord]) -> RefGame {
        let mut game = RefGame::new();
        let mut owed = stones_in_turn(game.turn);
        for (index, &at) in plies.iter().enumerate() {
            assert!(
                !game.decided,
                "ply {index} ({at}) follows a completed line: the game is over"
            );
            assert!(
                legal_for(&game.board, at),
                "ply {index} ({at}) is not a legal placement"
            );
            game.board.apply(at, game.to_move).expect("an empty cell");
            owed -= 1;
            if wins_at(&game.board, at) {
                // Rule 4: the turn ends here, on the turn it is scored on, and
                // a second stone owed is never played.
                game.decided = true;
            } else if owed == 0 {
                game.advance();
                owed = stones_in_turn(game.turn);
            }
        }
        assert!(
            game.decided || owed == stones_in_turn(game.turn),
            "the move list ends in the middle of turn {}",
            game.turn
        );
        game
    }

    /// The stones, for comparison with the position the engine's own state
    /// machine reached from the same move list.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// The turn number, counting from [`FIRST_TURN`].
    pub fn turn(&self) -> u32 {
        self.turn
    }

    /// Whether a line has been completed.
    pub fn is_decided(&self) -> bool {
        self.decided
    }

    /// Every turn the mover may play, ascending, by brute force.
    pub fn turns(&self) -> Vec<RefTurn> {
        if self.decided {
            return Vec::new();
        }
        let cells = self.search_box();
        let owed = stones_in_turn(self.turn);
        let mut turns = BTreeSet::new();
        for &first in &cells {
            if !legal_for(&self.board, first) {
                continue;
            }
            let mut after_first = self.board.clone();
            after_first
                .apply(first, self.to_move)
                .expect("an empty cell");
            if owed == 1 || wins_at(&after_first, first) {
                // Rule 3's one-stone turn, and rule 4's: a first stone that
                // completes a line ends the turn, so no pair contains it.
                turns.insert(RefTurn::Single(first));
                continue;
            }
            for &second in &cells {
                if second != first && legal_for(&after_first, second) {
                    turns.insert(RefTurn::pair(first, second));
                }
            }
        }
        turns.into_iter().collect()
    }

    /// The position after playing `turn`.
    pub fn child(&self, turn: RefTurn) -> RefGame {
        let mut next = self.clone();
        match turn {
            RefTurn::Single(at) => next.play_single(at),
            RefTurn::Pair(a, b) => next.play_pair(a, b),
        }
        next
    }

    /// How many distinct sequences of `depth` turns this position has.
    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let turns = self.turns();
        if depth == 1 {
            return turns.len() as u64;
        }
        turns
            .iter()
            .map(|&turn| self.child(turn).perft(depth - 1))
            .sum()
    }

    /// Play a turn of one stone: turn 1's, or one that completes a line.
    fn play_single(&mut self, at: Coord) {
        assert!(legal_for(&self.board, at), "{at} is not a legal placement");
        self.board.apply(at, self.to_move).expect("an empty cell");
        let won = wins_at(&self.board, at);
        assert!(
            won || stones_in_turn(self.turn) == 1,
            "{at} neither wins nor is turn 1's only stone, so it is not a whole turn"
        );
        self.decided = won;
        if !won {
            self.advance();
        }
    }

    /// Play a turn of two stones, in whichever order the rules allow.
    fn play_pair(&mut self, a: Coord, b: Coord) {
        assert!(
            stones_in_turn(self.turn) == 2,
            "turn {} places one stone",
            self.turn
        );
        for (first, second) in [(a, b), (b, a)] {
            if !legal_for(&self.board, first) {
                continue;
            }
            let mut board = self.board.clone();
            board.apply(first, self.to_move).expect("an empty cell");
            if wins_at(&board, first) {
                // This ordering plays a turn of one stone, not this pair.
                continue;
            }
            if !legal_for(&board, second) {
                continue;
            }
            board.apply(second, self.to_move).expect("an empty cell");
            self.decided = wins_at(&board, second);
            self.board = board;
            if !self.decided {
                self.advance();
            }
            return;
        }
        panic!("no ordering of the pair {a} {b} is legal");
    }

    /// Hand the turn over.
    fn advance(&mut self) {
        self.turn += 1;
        self.to_move = self.to_move.opponent();
    }

    /// Every cell that could possibly take a stone this turn, ascending.
    fn search_box(&self) -> Vec<Coord> {
        if self.board.is_empty() {
            return vec![Coord::ORIGIN];
        }
        let pad = i16::try_from(2 * LEGAL_RADIUS + 1).expect("the padded radius fits a coordinate");
        let stones: Vec<Coord> = self.board.stones().map(|(at, _)| at).collect();
        let min_q = stones.iter().map(|at| at.q).min().expect("some stone") - pad;
        let max_q = stones.iter().map(|at| at.q).max().expect("some stone") + pad;
        let min_r = stones.iter().map(|at| at.r).min().expect("some stone") - pad;
        let max_r = stones.iter().map(|at| at.r).max().expect("some stone") + pad;
        let mut cells = Vec::new();
        for q in min_q..=max_q {
            for r in min_r..=max_r {
                cells.push(Coord::new(q, r));
            }
        }
        cells
    }
}

/// Whether a stone may be placed on `cell`, rule 5 said literally: the cell is
/// empty and within [`LEGAL_RADIUS`] of some stone — measured against every
/// stone in turn, with no region, no cache and no shortcut. On an empty board
/// the first stone goes on the origin (rule 3).
fn legal_for(board: &Board, cell: Coord) -> bool {
    if board.is_occupied(cell) {
        return false;
    }
    if board.is_empty() {
        return cell == Coord::ORIGIN;
    }
    board
        .stones()
        .any(|(stone, _)| stone.distance(cell) <= LEGAL_RADIUS)
}
