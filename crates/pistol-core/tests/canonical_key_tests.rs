use std::collections::BTreeSet;

use pistol_core::symmetry::transform;
use pistol_core::{
    Coord, GameState, Key128, Player, Symmetry, Turn, canonical_form, canonical_key,
    canonical_sequence, cell_key,
};

/// The stones `cells` places, IN PLAY ORDER.
///
/// Play order and not the board's own iteration order, which is sorted: two
/// transpositions read off a sorted board are the SAME slice, and a fixture
/// that hands the same slice to the function twice cannot tell an
/// order-sensitive fold from an order-blind one. Reading the history is what
/// makes the transposition pair below two different inputs.
fn stones(cells: &[&str]) -> Vec<(Coord, Player)> {
    game(cells).played().collect()
}

/// The game `cells` plays: one stone on turn 1, pairs after it.
fn game(cells: &[&str]) -> GameState {
    let mut state = GameState::new_game();
    for cell in cells {
        let coord: Coord = cell.parse().expect("a coordinate");
        state.place(coord).expect("a legal stone");
    }
    state
}

/// The turns `cells` groups into, which is what `canonical_sequence` reads.
fn turns(cells: &[&str]) -> Vec<Turn> {
    let coords: Vec<Coord> = cells
        .iter()
        .map(|cell| cell.parse().expect("a coordinate"))
        .collect();
    let (first, rest) = coords.split_first().expect("a game has a first stone");
    let mut out = vec![Turn::single(*first)];
    for pair in rest.chunks(2) {
        out.push(match pair {
            [a, b] => Turn::pair(*a, *b).expect("two distinct cells are a turn"),
            [a] => Turn::single(*a),
            _ => unreachable!("chunks(2) yields one or two"),
        });
    }
    out
}

/// Two games whose turns differ and whose stone sets do not: turn 2 takes
/// `1,0 0,1` in one and `1,0 2,0` in the other, and turn 4 swaps the leftovers
/// back. A transposition in the strong sense — the SEQUENCE key separates them.
const ORDER_ONE: &[&str] = &["0,0", "1,0", "0,1", "-1,1", "1,1", "2,0", "3,0"];
const ORDER_OTHER: &[&str] = &["0,0", "1,0", "2,0", "-1,1", "1,1", "0,1", "3,0"];

#[test]
fn two_move_orders_reaching_one_position_share_a_census_key() {
    assert_ne!(
        stones(ORDER_ONE),
        stones(ORDER_OTHER),
        "the fixture is wrong: the two play orders must be DIFFERENT inputs, or an \
         order-sensitive fold would pass this"
    );
    let (mut sorted_one, mut sorted_other) = (stones(ORDER_ONE), stones(ORDER_OTHER));
    sorted_one.sort_unstable();
    sorted_other.sort_unstable();
    assert_eq!(
        sorted_one, sorted_other,
        "the fixture is wrong: the two games must reach the same stone SET"
    );
    assert_ne!(
        canonical_sequence(&turns(ORDER_ONE)),
        canonical_sequence(&turns(ORDER_OTHER)),
        "the fixture is wrong: a key over the GAME must tell these two apart, or the census \
         key folding them says nothing"
    );
    assert_eq!(
        canonical_key(&stones(ORDER_ONE)),
        canonical_key(&stones(ORDER_OTHER))
    );
}

#[test]
fn a_known_transposition_pair_counts_as_one_disjoint_position() {
    // The counting rule, not a program. Nothing in the census counts disjoint
    // positions — it emits rows and an analyst counts them — so what is pinned
    // here is that the two firings of this pair are ONE key, and the count
    // follows from the rule docs/decisions.md D-537 states.
    let distinct: BTreeSet<Key128> = [
        canonical_key(&stones(ORDER_ONE)),
        canonical_key(&stones(ORDER_OTHER)),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        distinct.len(),
        1,
        "the ruled count over this pair is ONE position, and two keys make it two"
    );
}

#[test]
fn a_position_and_its_mirror_image_share_a_census_key() {
    // The fold `GameState::key` does not do, which is why the census does not
    // carry that key.
    let position = stones(ORDER_ONE);
    let expected = canonical_key(&position);
    for symmetry in Symmetry::ALL {
        assert_eq!(
            canonical_key(&transform(&position, symmetry)),
            expected,
            "the image under {symmetry} is the same position and must carry the same key"
        );
    }
}

#[test]
fn two_positions_that_are_not_one_position_carry_different_census_keys() {
    // The other direction: a fold that merged these would UNDER-count, and an
    // under-count clears a floor early — the failure the floor exists to stop.
    let mut keys = BTreeSet::new();
    for cells in [
        &["0,0", "1,0", "0,1", "2,0", "1,-1"][..],
        &["0,0", "1,0", "0,1", "2,0", "3,0"][..],
        &["0,0", "1,0", "0,1", "2,0", "2,1"][..],
        &["0,0", "1,0", "0,1"][..],
    ] {
        assert!(
            keys.insert(canonical_key(&stones(cells))),
            "two positions with different canonical forms share a key: {cells:?}"
        );
    }
}

#[test]
fn a_colour_swap_is_a_different_position_to_the_census_too() {
    let position = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(1, 0), Player::P2),
    ];
    let swapped = vec![
        (Coord::new(0, 0), Player::P2),
        (Coord::new(1, 0), Player::P1),
    ];
    assert_ne!(canonical_key(&position), canonical_key(&swapped));
}

#[test]
fn the_key_is_the_fold_of_the_canonical_form_and_not_a_second_minimisation() {
    // Whatever `canonical_form` calls one position, this key calls one
    // position. A second minimisation could drift from it, which is the
    // fourth-notion-of-sameness hazard (docs/decisions.md D-137).
    let position = stones(&["0,0", "1,0", "0,1", "2,0", "1,-1", "2,-1"]);
    let folded = canonical_form(&position)
        .into_iter()
        .fold(Key128::ZERO, |key, (cell, player)| {
            key ^ cell_key(cell, player)
        });
    assert_eq!(canonical_key(&position), folded);
}

#[test]
fn every_image_of_every_prefix_of_a_game_folds_to_one_key() {
    // The invariant over a population rather than one hand-picked board: every
    // prefix of a long game, each under all twelve symmetries.
    let cells = [
        "0,0", "1,0", "0,1", "2,0", "1,-1", "2,-1", "-1,1", "0,2", "3,-1", "1,1", "-1,2", "2,1",
        "-2,2", "3,0", "-1,0", "0,-1", "4,-2", "1,2", "-2,1", "2,2",
    ];
    let mut state = GameState::new_game();
    let mut checked = 0usize;
    for cell in cells {
        let coord: Coord = cell.parse().expect("a coordinate");
        state.place(coord).expect("a legal stone");
        let position: Vec<(Coord, Player)> = state.board().stones().collect();
        let expected = canonical_key(&position);
        for symmetry in Symmetry::ALL {
            assert_eq!(canonical_key(&transform(&position, symmetry)), expected);
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        cells.len() * Symmetry::ALL.len(),
        "every prefix was read under every symmetry"
    );
}
