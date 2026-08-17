//! The stones themselves: putting one down, taking it back, and reading them
//! out in a fixed order.
//!
//! `Board::apply`/`undo` is the per-stone seam that WP-04's incremental zobrist
//! and the incremental eval will hook (docs/decisions.md D-41), so a silent
//! overwrite or a wrong colour handed back would desynchronize state that has
//! no other way to notice. That is what these tests exist to make impossible.

use pistol_core::{Board, Color, Coord, CoreError};

#[test]
fn a_stone_can_be_placed_read_back_and_taken_back() {
    let mut board = Board::empty();
    assert!(board.is_empty());
    assert_eq!(board.stone_count(), 0);
    assert_eq!(board.get(Coord::ORIGIN), None);

    for (at, color) in [
        (Coord::ORIGIN, Color::Black),
        (Coord::new(-3, 7), Color::White),
    ] {
        board.apply(at, color).expect("an empty cell");
        assert_eq!(board.get(at), Some(color), "the colour that was placed");
        assert!(board.is_occupied(at));
        assert!(!board.is_empty());

        assert_eq!(board.undo(at), Ok(color), "the colour that comes back");
        assert_eq!(board.get(at), None);
        assert!(!board.is_occupied(at));
    }
    assert!(board.is_empty(), "every stone was taken back");
}

#[test]
fn applying_to_an_occupied_cell_is_refused_and_changes_nothing() {
    let mut board = Board::empty();
    board
        .apply(Coord::ORIGIN, Color::Black)
        .expect("empty cell");

    for color in [Color::White, Color::Black] {
        assert_eq!(
            board.apply(Coord::ORIGIN, color),
            Err(CoreError::OccupiedCell { at: Coord::ORIGIN }),
            "no colour may overwrite a stone"
        );
        assert_eq!(
            board.get(Coord::ORIGIN),
            Some(Color::Black),
            "the stone that was there is still there, unchanged"
        );
        assert_eq!(board.stone_count(), 1);
    }
}

#[test]
fn undoing_an_empty_cell_is_refused_by_name() {
    let mut board = Board::empty();
    assert_eq!(
        board.undo(Coord::ORIGIN),
        Err(CoreError::UnoccupiedCell { at: Coord::ORIGIN })
    );

    board
        .apply(Coord::ORIGIN, Color::White)
        .expect("empty cell");
    board.undo(Coord::ORIGIN).expect("the stone just placed");
    assert_eq!(
        board.undo(Coord::ORIGIN),
        Err(CoreError::UnoccupiedCell { at: Coord::ORIGIN }),
        "twice taken back is once too many"
    );
}

#[test]
fn apply_undo_restores_the_board_exactly() {
    let stones = [
        (Coord::new(0, 0), Color::Black),
        (Coord::new(1, 0), Color::White),
        (Coord::new(-2, 5), Color::Black),
        (Coord::new(9, -9), Color::White),
    ];
    let mut board = Board::empty();
    for &(at, color) in &stones {
        board.apply(at, color).expect("distinct cells");
    }
    let before = board.clone();

    let probe = Coord::new(4, 4);
    board.apply(probe, Color::Black).expect("empty cell");
    assert_eq!(board.undo(probe), Ok(Color::Black));
    assert_eq!(board, before, "a round trip is not a rewrite");
    assert_eq!(
        board.stones().collect::<Vec<_>>(),
        before.stones().collect::<Vec<_>>()
    );
}

#[test]
fn stones_are_read_out_in_ascending_coordinate_order() {
    // The order is the contract, not an accident of the container: it is what
    // makes two runs over the same position agree (CLAUDE.md rule 4).
    let placed = [
        Coord::new(3, -1),
        Coord::new(-4, 8),
        Coord::new(3, -2),
        Coord::new(0, 0),
        Coord::new(-4, 7),
    ];
    let mut board = Board::empty();
    for (index, &at) in placed.iter().enumerate() {
        let color = if index % 2 == 0 {
            Color::Black
        } else {
            Color::White
        };
        board.apply(at, color).expect("distinct cells");
    }

    let read: Vec<Coord> = board.stones().map(|(at, _)| at).collect();
    let mut sorted = placed.to_vec();
    sorted.sort();
    assert_eq!(read, sorted, "ascending by q, then r — not insertion order");
    assert_ne!(read, placed.to_vec(), "the fixture was not already sorted");

    // Reading twice gives the same order, and so does reading after a stone has
    // been taken back and put down again.
    assert_eq!(read, board.stones().map(|(at, _)| at).collect::<Vec<_>>());
    let colour = board.undo(placed[0]).expect("a placed stone");
    board.apply(placed[0], colour).expect("now empty again");
    assert_eq!(read, board.stones().map(|(at, _)| at).collect::<Vec<_>>());
}

#[test]
fn colours_are_opposites_and_name_themselves() {
    assert_eq!(Color::Black.opponent(), Color::White);
    assert_eq!(Color::White.opponent(), Color::Black);
    assert_eq!(Color::Black.opponent().opponent(), Color::Black);
    assert_eq!(Color::Black.name(), "black");
    assert_eq!(Color::White.name(), "white");
    assert_eq!(Color::Black.to_string(), "black");
}
