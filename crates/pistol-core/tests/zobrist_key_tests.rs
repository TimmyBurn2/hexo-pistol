//! The key function: what a cell, a side and a phase are named, and why those
//! names are the same everywhere.
//!
//! The board is unbounded, so there is no table of random numbers to load — a
//! key is computed from `(q, r, colour, ZOBRIST_SEED)` (docs/decisions.md D-8,
//! D-57). That makes the function itself the thing to pin: it has to be pure,
//! and it has to give the same 128 bits in this run, in a second process, and
//! in a build made next year (CLAUDE.md rule 4). Two runs of an engine that
//! disagreed about what a position is called would disagree about what it is
//! worth, through a transposition table that answered questions nobody asked.
//!
//! The vectors here were computed from the construction D-57 describes,
//! independently of the implementation they check, so they are an oracle for it
//! rather than a photograph of it. The SplitMix64 they were computed with was
//! itself checked against Vigna's published stream for state zero
//! (`0xE220A8397B1DCDAF`, `0x6E789E6AA1B965F4`, `0x06C45D188009454F`,
//! `0xF88BB8A8724C81EC`).
//!
//! The key a whole *position* carries is `zobrist_position_tests.rs`.

use std::collections::BTreeSet;

use pistol_core::{
    Board, Color, Coord, Key128, Phase, ZOBRIST_SEED, cell_key, phase_key, side_key,
};

/// A spread of cells and colours: the origin, both signs, and the corners of
/// the addressable lattice, where the coordinate encoding is most likely to be
/// wrong.
fn sample_cells() -> Vec<(Coord, Color)> {
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
        cells.push((Coord::new(q, r), Color::Black));
        cells.push((Coord::new(q, r), Color::White));
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
        .map(|&(at, color)| cell_key(at, color))
        .collect();

    for round in 0..4 {
        for (index, &(at, color)) in cells.iter().enumerate().rev() {
            assert_eq!(
                cell_key(at, color),
                first[index],
                "cell_key({at}, {color}) changed on round {round}"
            );
        }
    }

    let mut board = Board::empty();
    for &(at, color) in cells
        .iter()
        // `unsigned_abs`, because `i16::MIN` has no absolute value in range.
        .filter(|&&(at, color)| color == Color::Black && at.q.unsigned_abs() < 8)
    {
        board.apply(at, color).expect("distinct cells");
    }
    for (index, &(at, color)) in cells.iter().enumerate() {
        assert_eq!(
            cell_key(at, color),
            first[index],
            "cell_key({at}, {color}) moved with the board, and it must not"
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
    let black = Color::Black;
    let white = Color::White;
    // The far corner of the lattice, where the encoding has to be two's
    // complement and not a sign-magnitude spelling of the same cell.
    let corner = Coord::new(i16::MIN, i16::MAX);
    let cells: [(Coord, Color, Key128); 6] = [
        (
            Coord::new(0, 0),
            black,
            pinned(0xF757_2DD1_3142_8A37, 0x1E8F_F245_11EF_2DF6),
        ),
        (
            Coord::new(0, 0),
            white,
            pinned(0xE7DA_6DC7_A1D8_96FA, 0x65CF_8095_BB4A_F015),
        ),
        (
            Coord::new(1, 0),
            black,
            pinned(0xACD2_0D77_215B_4B23, 0xE6F9_89D1_5393_70F7),
        ),
        (
            Coord::new(0, 1),
            black,
            pinned(0xBA7D_038B_8B09_E1F7, 0xEE38_730B_86CB_A54A),
        ),
        (
            Coord::new(-1, -1),
            white,
            pinned(0x6AC6_533F_019F_AD6C, 0xD35B_B1E6_BD3A_35EC),
        ),
        (
            corner,
            black,
            pinned(0xA228_9916_D7D3_933D, 0xB167_C38C_5DCA_2FF6),
        ),
    ];
    for (at, color, expected) in cells {
        assert_eq!(cell_key(at, color), expected, "cell_key({at}, {color})");
    }

    assert_eq!(
        side_key(black),
        Key128::from_parts(0xA730_B797_5016_47FA, 0x7985_C786_FBA1_571E)
    );
    assert_eq!(
        side_key(white),
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
            for color in [Color::Black, Color::White] {
                let at = Coord::new(q, r);
                record(cell_key(at, color), format!("cell_key({at}, {color})"));
            }
        }
    }
    // The context keys live in the same space, so they are checked against the
    // cells and not merely against each other.
    for color in [Color::Black, Color::White] {
        record(side_key(color), format!("side_key({color})"));
    }
    for phase in [Phase::First, Phase::Second] {
        record(phase_key(phase), format!("phase_key({phase:?})"));
    }
}
