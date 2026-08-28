use std::collections::BTreeSet;

use pistol_core::{
    Board, Coord, Key128, Phase, Player, ZOBRIST_SEED, cell_key, phase_key, side_key,
};

/// A spread of cells and players: the origin, both signs, and the corners of
/// the addressable lattice, where the coordinate encoding is most likely to be
/// wrong.
fn sample_cells() -> Vec<(Coord, Player)> {
    let mut cells = Vec::new();
    for &(q, r) in &[
        (0, 0),
        (1, 0),
        (0, 1),
        (-1, -1),
        (7, -7),
        (i16::MIN, i16::MAX),
        (i16::MAX, i16::MIN),
    ] {
        cells.push((Coord::new(q, r), Player::P1));
        cells.push((Coord::new(q, r), Player::P2));
    }
    cells
}

#[test]
fn zobrist_key_is_pure_function_of_cell() {
    // There is no cache to clear (docs/decisions.md D-59): a key is computed
    // every time, so "cold" and "warm" are the same call, and what a cache
    // would have had to preserve is what is asserted here — repeated calls, in
    // any order, under any board state, give one answer.
    let cells = sample_cells();
    let first: Vec<Key128> = cells
        .iter()
        .map(|&(at, player)| cell_key(at, player))
        .collect();

    for round in 0..4 {
        for (index, &(at, player)) in cells.iter().enumerate().rev() {
            assert_eq!(
                cell_key(at, player),
                first[index],
                "cell_key({at}, {player}) changed on round {round}"
            );
        }
    }

    let mut board = Board::empty();
    for &(at, player) in cells
        .iter()
        // `unsigned_abs`, because `i16::MIN` has no absolute value in range.
        .filter(|&&(at, player)| player == Player::P1 && at.q.unsigned_abs() < 8)
    {
        board.apply(at, player).expect("distinct cells");
    }
    for (index, &(at, player)) in cells.iter().enumerate() {
        assert_eq!(
            cell_key(at, player),
            first[index],
            "cell_key({at}, {player}) moved with the board, and it must not"
        );
    }
}

#[test]
fn zobrist_keys_match_the_published_construction() {
    assert_eq!(
        ZOBRIST_SEED, 0x7069_7374_6F6C_0001,
        "the seed is pinned: changing it renames every position in every book, \
         net and match log ever produced"
    );

    let pinned = Key128::from_parts;
    let p1 = Player::P1;
    let p2 = Player::P2;
    // The far corner of the lattice, where the encoding has to be two's
    // complement and not a sign-magnitude spelling of the same cell.
    let corner = Coord::new(i16::MIN, i16::MAX);
    let cells: [(Coord, Player, Key128); 6] = [
        (
            Coord::new(0, 0),
            p1,
            pinned(0xF757_2DD1_3142_8A37, 0x1E8F_F245_11EF_2DF6),
        ),
        (
            Coord::new(0, 0),
            p2,
            pinned(0xE7DA_6DC7_A1D8_96FA, 0x65CF_8095_BB4A_F015),
        ),
        (
            Coord::new(1, 0),
            p1,
            pinned(0xACD2_0D77_215B_4B23, 0xE6F9_89D1_5393_70F7),
        ),
        (
            Coord::new(0, 1),
            p1,
            pinned(0xBA7D_038B_8B09_E1F7, 0xEE38_730B_86CB_A54A),
        ),
        (
            Coord::new(-1, -1),
            p2,
            pinned(0x6AC6_533F_019F_AD6C, 0xD35B_B1E6_BD3A_35EC),
        ),
        (
            corner,
            p1,
            pinned(0xA228_9916_D7D3_933D, 0xB167_C38C_5DCA_2FF6),
        ),
    ];
    for (at, player, expected) in cells {
        assert_eq!(cell_key(at, player), expected, "cell_key({at}, {player})");
    }

    assert_eq!(
        side_key(p1),
        Key128::from_parts(0xA730_B797_5016_47FA, 0x7985_C786_FBA1_571E)
    );
    assert_eq!(
        side_key(p2),
        Key128::from_parts(0xA4A0_8BCC_BCA2_EF3F, 0x5D09_4415_EFAF_5B6A)
    );
    assert_eq!(
        phase_key(Phase::First),
        Key128::from_parts(0xAC1F_7EF1_42BA_A52D, 0x50BE_6542_FF23_0003)
    );
    assert_eq!(
        phase_key(Phase::Second),
        Key128::from_parts(0x6C3B_374D_6588_788B, 0xF460_DEDD_546F_424E)
    );
}

#[test]
fn zobrist_distinct_inputs_have_distinct_keys() {
    // Each half is checked on its own and not only the whole: the search TT
    // indexes from the low bits and verifies with the high 64
    // (docs/decisions.md D-8), so a construction that collided within a half
    // would defeat the verification it exists to provide.
    let mut whole = BTreeSet::new();
    let mut lows = BTreeSet::new();
    let mut highs = BTreeSet::new();
    let mut record = |key: Key128, what: String| {
        assert!(whole.insert(key), "{what} collides in all 128 bits");
        assert!(lows.insert(key.low()), "{what} collides in the low 64");
        assert!(highs.insert(key.high()), "{what} collides in the high 64");
    };

    for q in -12i16..=12 {
        for r in -12i16..=12 {
            for player in [Player::P1, Player::P2] {
                let at = Coord::new(q, r);
                record(cell_key(at, player), format!("cell_key({at}, {player})"));
            }
        }
    }
    // The context keys live in the same space, so they are checked against the
    // cells and not merely against each other.
    for player in [Player::P1, Player::P2] {
        record(side_key(player), format!("side_key({player})"));
    }
    for phase in [Phase::First, Phase::Second] {
        record(phase_key(phase), format!("phase_key({phase:?})"));
    }
}
