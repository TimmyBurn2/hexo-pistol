use pistol_core::{Board, Coord, CoreError, Player};

#[test]
fn a_stone_can_be_placed_read_back_and_taken_back() {
    let mut board = Board::empty();
    assert!(board.is_empty());
    assert_eq!(board.stone_count(), 0);
    assert_eq!(board.get(Coord::ORIGIN), None);

    for (at, player) in [(Coord::ORIGIN, Player::P1), (Coord::new(-3, 7), Player::P2)] {
        board.apply(at, player).expect("an empty cell");
        assert_eq!(board.get(at), Some(player), "the player that was placed");
        assert!(board.is_occupied(at));
        assert!(!board.is_empty());

        assert_eq!(board.undo(at), Ok(player), "the player that comes back");
        assert_eq!(board.get(at), None);
        assert!(!board.is_occupied(at));
    }
    assert!(board.is_empty(), "every stone was taken back");
}

#[test]
fn applying_to_an_occupied_cell_is_refused_and_changes_nothing() {
    let mut board = Board::empty();
    board.apply(Coord::ORIGIN, Player::P1).expect("empty cell");

    for player in [Player::P2, Player::P1] {
        assert_eq!(
            board.apply(Coord::ORIGIN, player),
            Err(CoreError::OccupiedCell { at: Coord::ORIGIN }),
            "no player may overwrite a stone"
        );
        assert_eq!(
            board.get(Coord::ORIGIN),
            Some(Player::P1),
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

    board.apply(Coord::ORIGIN, Player::P2).expect("empty cell");
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
        (Coord::new(0, 0), Player::P1),
        (Coord::new(1, 0), Player::P2),
        (Coord::new(-2, 5), Player::P1),
        (Coord::new(9, -9), Player::P2),
    ];
    let mut board = Board::empty();
    for &(at, player) in &stones {
        board.apply(at, player).expect("distinct cells");
    }
    let before = board.clone();

    let probe = Coord::new(4, 4);
    board.apply(probe, Player::P1).expect("empty cell");
    assert_eq!(board.undo(probe), Ok(Player::P1));
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
        let player = if index % 2 == 0 {
            Player::P1
        } else {
            Player::P2
        };
        board.apply(at, player).expect("distinct cells");
    }

    let read: Vec<Coord> = board.stones().map(|(at, _)| at).collect();
    let mut sorted = placed.to_vec();
    sorted.sort();
    assert_eq!(read, sorted, "ascending by q, then r — not insertion order");
    assert_ne!(read, placed.to_vec(), "the fixture was not already sorted");

    // Reading twice gives the same order, and so does reading after a stone has
    // been taken back and put down again.
    assert_eq!(read, board.stones().map(|(at, _)| at).collect::<Vec<_>>());
    let player = board.undo(placed[0]).expect("a placed stone");
    board.apply(placed[0], player).expect("now empty again");
    assert_eq!(read, board.stones().map(|(at, _)| at).collect::<Vec<_>>());
}

#[test]
fn players_are_opposites_and_name_themselves() {
    assert_eq!(Player::P1.opponent(), Player::P2);
    assert_eq!(Player::P2.opponent(), Player::P1);
    assert_eq!(Player::P1.opponent().opponent(), Player::P1);
    assert_eq!(Player::P1.name(), "p1");
    assert_eq!(Player::P2.name(), "p2");
    assert_eq!(Player::P1.to_string(), "p1");
}
