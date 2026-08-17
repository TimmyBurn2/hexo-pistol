//! Win detection at the completing stone (game rules 2 and 4).
//!
//! Six or more contiguous own stones along one of the three axes wins.
//! Overlines win. Five does not. A run does not bend, does not jump a gap, and
//! does not pass through an opponent's stone. Each of those is a separate way
//! for an implementation to be wrong, so each gets its own test — and the
//! six-in-a-row tests place the completing stone at every position in the run,
//! because a scan that only looks forward passes the end-stone case and fails
//! every other one.

use pistol_core::win::{run_through, winning_run};
use pistol_core::{Axis, Board, Coord, Player, WIN_LEN, wins_at};

/// `count` stones of `player` along `axis`, starting at `start`.
fn line(start: Coord, axis: Axis, count: i16) -> Vec<Coord> {
    (0..count).map(|step| start.step(axis, step)).collect()
}

/// A board holding exactly these stones, all one player.
fn board_of(cells: &[Coord], player: Player) -> Board {
    let mut board = Board::empty();
    for &cell in cells {
        board.apply(cell, player).expect("distinct cells");
    }
    board
}

/// Whether any stone on the board completes a run — the whole-board question,
/// asked in the test tree because the crate deliberately does not offer it on
/// the hot path (docs/decisions.md D-36).
fn any_stone_wins(board: &Board) -> bool {
    board.stones().any(|(at, _)| wins_at(board, at))
}

#[test]
fn win_detects_exact_six_on_each_of_three_axes() {
    for axis in Axis::ALL {
        let start = Coord::new(-2, 3);
        let six = line(start, axis, 6);
        assert_eq!(six.len(), WIN_LEN as usize);

        for completing in 0..six.len() {
            // The board the instant before the last stone: five in a row, no win
            // anywhere on it.
            let mut before: Vec<Coord> = six.clone();
            let last = before.remove(completing);
            let mut board = board_of(&before, Player::P1);
            assert!(
                !any_stone_wins(&board),
                "{axis:?}: five stones already win, without {last}"
            );

            board.apply(last, Player::P1).expect("empty cell");
            assert!(
                wins_at(&board, last),
                "{axis:?}: {last} completes six and must win"
            );

            let run = winning_run(&board, last).expect("a winning run");
            assert_eq!(run.axis, axis);
            assert_eq!(run.len, 6);
            assert_eq!(run.player, Player::P1);
            assert_eq!(run.start, six[0], "the run starts at its lower end");
            assert_eq!(run.start, *six.iter().min().expect("six stones"));
            assert_eq!(run_through(&board, last, axis), 6);
        }
    }
}

#[test]
fn win_detects_overline_seven() {
    for axis in Axis::ALL {
        let start = Coord::new(4, -9);
        let seven = line(start, axis, 7);

        for completing in 0..seven.len() {
            let mut before = seven.clone();
            let last = before.remove(completing);
            let mut board = board_of(&before, Player::P1);
            let already = any_stone_wins(&board);
            board.apply(last, Player::P1).expect("empty cell");

            assert!(
                wins_at(&board, last),
                "{axis:?}: seven in a row wins, overlines included ({last})"
            );
            let run = winning_run(&board, last).expect("a winning run");
            assert_eq!(run.len, 7, "the run is seven long, not truncated to six");
            assert_eq!(run.axis, axis);

            // Removing a stone from the middle of seven leaves 3 + 3, which does
            // not win; removing an end leaves six, which does. Both are stated so
            // the case above cannot be passing for the wrong reason.
            let interior = completing != 0 && completing != seven.len() - 1;
            assert_eq!(already, !interior, "{axis:?}: before placing {last}");
        }
    }
}

#[test]
fn five_in_row_is_not_win() {
    for axis in Axis::ALL {
        let five = line(Coord::new(1, 1), axis, 5);
        let board = board_of(&five, Player::P2);
        assert!(!any_stone_wins(&board), "{axis:?}: five is not six");
        for &cell in &five {
            assert_eq!(run_through(&board, cell, axis), 5);
            assert!(winning_run(&board, cell).is_none());
        }
    }
}

#[test]
fn stone_filling_a_gap_between_two_runs_wins() {
    // Five own stones on each side of a hole. The stone in the hole has to look
    // both ways to see the eleven-long run it just made.
    for axis in Axis::ALL {
        let hole = Coord::new(-6, 2);
        let mut cells: Vec<Coord> = (1..=5)
            .flat_map(|k| [hole.step(axis, k), hole.step(axis, -k)])
            .collect();
        let mut board = board_of(&cells, Player::P1);
        assert!(!any_stone_wins(&board), "{axis:?}: two fives, no six");

        board.apply(hole, Player::P1).expect("the hole is empty");
        assert!(wins_at(&board, hole), "{axis:?}: the bridging stone wins");
        assert_eq!(run_through(&board, hole, axis), 11);
        cells.push(hole);
        assert_eq!(board.stone_count(), cells.len());
    }
}

#[test]
fn stone_filling_a_gap_that_leaves_five_does_not_win() {
    for axis in Axis::ALL {
        let hole = Coord::new(0, 0);
        let cells = [
            hole.step(axis, -2),
            hole.step(axis, -1),
            hole.step(axis, 1),
            hole.step(axis, 2),
        ];
        let mut board = board_of(&cells, Player::P1);
        board.apply(hole, Player::P1).expect("the hole is empty");
        assert!(
            !wins_at(&board, hole),
            "{axis:?}: two plus one plus two is five"
        );
        assert_eq!(run_through(&board, hole, axis), 5);
    }
}

#[test]
fn a_run_stops_at_an_opponent_stone() {
    for axis in Axis::ALL {
        let start = Coord::ORIGIN;
        let mut board = board_of(&line(start, axis, 3), Player::P1);
        let blocker = start.step(axis, 3);
        board.apply(blocker, Player::P2).expect("empty cell");
        for cell in line(start.step(axis, 4), axis, 3) {
            board.apply(cell, Player::P1).expect("empty cell");
        }

        assert!(!any_stone_wins(&board), "{axis:?}: three, block, three");
        assert_eq!(run_through(&board, start, axis), 3);
        assert_eq!(run_through(&board, blocker, axis), 1);
        assert_eq!(run_through(&board, start.step(axis, 6), axis), 3);
    }
}

#[test]
fn a_run_does_not_bend_between_axes() {
    // Six stones in one connected chain that turns a corner: adjacency is not
    // the rule, collinearity is.
    let corner = Coord::ORIGIN;
    let mut cells = line(corner, Axis::ConstR, 3);
    cells.extend(line(
        corner.step(Axis::ConstR, 2).step(Axis::ConstQ, 1),
        Axis::ConstQ,
        3,
    ));
    let board = board_of(&cells, Player::P1);

    assert_eq!(board.stone_count(), 6);
    assert!(!any_stone_wins(&board), "a bent chain of six is not a run");
}

#[test]
fn a_run_does_not_count_the_parallel_line_beside_it() {
    let mut cells = line(Coord::ORIGIN, Axis::ConstR, 5);
    cells.extend(line(Coord::new(0, 1), Axis::ConstR, 5));
    let board = board_of(&cells, Player::P1);
    assert_eq!(board.stone_count(), 10);
    assert!(!any_stone_wins(&board), "two parallel fives are not a six");
}

#[test]
fn winning_run_reports_the_first_axis_when_a_stone_completes_two() {
    // One stone at the crossing of two lines of six. Which run is reported is a
    // choice, and it is fixed: the first in `Axis::ALL` order (CLAUDE.md
    // rule 4).
    let crossing = Coord::ORIGIN;
    let mut cells: Vec<Coord> = Vec::new();
    for axis in [Axis::ConstQ, Axis::ConstR] {
        for step in -2..=3 {
            if step != 0 {
                cells.push(crossing.step(axis, step));
            }
        }
    }
    let mut board = board_of(&cells, Player::P1);
    assert!(!any_stone_wins(&board), "neither line is six without it");

    board.apply(crossing, Player::P1).expect("empty cell");
    assert!(wins_at(&board, crossing));
    assert_eq!(run_through(&board, crossing, Axis::ConstQ), 6);
    assert_eq!(run_through(&board, crossing, Axis::ConstR), 6);

    let run = winning_run(&board, crossing).expect("a winning run");
    assert_eq!(run.axis, Axis::ConstQ, "the first axis in ALL order");
    assert_eq!(run.start, crossing.step(Axis::ConstQ, -2));
}

#[test]
#[should_panic(expected = "WIN_CHECK_ON_EMPTY_CELL")]
fn asking_whether_an_empty_cell_wins_is_a_named_invariant_panic() {
    let board = board_of(&[Coord::ORIGIN], Player::P1);
    let _ = wins_at(&board, Coord::new(1, 0));
}

/// A run longer than a step count can express.
///
/// `Board::apply` builds synthetic positions no game reaches — that is what it
/// is for (docs/decisions.md D-35) — and a line of this lattice holds more
/// cells than an `i16` can count steps. Reporting the run must not become a
/// panic just because the run got long.
#[test]
fn a_run_longer_than_a_step_count_still_reports_its_start() {
    let mut board = Board::empty();
    let length: i32 = 32_769;
    for step in 0..length {
        let r = i16::try_from(-step).expect("inside the lattice");
        board
            .apply(Coord::new(0, r), Player::P1)
            .expect("distinct cells");
    }
    let probe = Coord::ORIGIN;

    assert!(wins_at(&board, probe));
    assert_eq!(
        run_through(&board, probe, Axis::ConstQ),
        u32::try_from(length).expect("a positive length")
    );
    let run = winning_run(&board, probe).expect("a winning run");
    assert_eq!(run.axis, Axis::ConstQ);
    assert_eq!(run.len, u32::try_from(length).expect("a positive length"));
    assert_eq!(
        run.start,
        Coord::new(0, i16::try_from(-(length - 1)).expect("inside the lattice")),
        "the far end of the run, carried out of the scan"
    );
}
