mod common;

// RULE9-JUSTIFICATION: one suite per instrument, and this one binds every
// number the census reports plus the nine mutants two reviews found alive;
// splitting it would let a reader believe one half certifies the whole.

use pistol_cli::corpus::census::{
    Census, Keys, Kind, OFF_ADDRESSABLE_LATTICE, flatten, keys_of, keys_of_all, keys_or_refusal,
    uncoloured,
};
use pistol_cli::corpus::record::Record;
use pistol_core::{
    Coord, Player, Symmetry,
    symmetry::{transform, transform_sequence},
};

/// A game of `n` stones walking a straight line, so a synthetic corpus can be
/// built without a fixture file.
fn game(hash: &str, moves: &[(i16, i16)]) -> Record {
    Record {
        game_hash: hash.to_string(),
        moves: moves.iter().map(|&(q, r)| Coord::new(q, r)).collect(),
        winner: Player::P1,
        elo: [Some(1500), Some(1500)],
    }
}

/// Eleven stones: the turn-6 boundary the early key is cut at.
const OPENING: [(i16, i16); 11] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (0, 1),
    (0, 2),
    (3, 0),
    (4, 0),
    (0, 3),
    (0, 4),
    (5, 0),
    (6, 0),
];

fn rotated(moves: &[(i16, i16)], symmetry: Symmetry) -> Vec<(i16, i16)> {
    moves
        .iter()
        .map(|&(q, r)| {
            let image = symmetry.apply(Coord::new(q, r));
            (image.q, image.r)
        })
        .collect()
}

fn sequence_census(records: &[Record]) -> Census {
    let keys = keys_of_all(records);
    Census::build(
        Kind::Sequence,
        &keys,
        |k: &Keys| Some(k.sequence.clone()),
        |k: &Keys, s| transform_sequence(&k.turns, s),
    )
}

fn early_census(records: &[Record]) -> Census {
    let keys = keys_of_all(records);
    Census::build(
        Kind::EarlyPosition,
        &keys,
        |k: &Keys| k.early_position.clone(),
        |k: &Keys, s| transform(&k.early_stones.clone().unwrap_or_default(), s),
    )
}

fn final_census(records: &[Record]) -> Census {
    let keys = keys_of_all(records);
    Census::build(
        Kind::FinalPosition,
        &keys,
        |k: &Keys| Some(k.final_position.clone()),
        |k: &Keys, s| transform(&k.stones, s),
    )
}

#[test]
fn an_injected_symmetry_duplicate_is_detected_as_the_same_game() {
    let turned = rotated(&OPENING, Symmetry::ALL[2]);
    let records = vec![
        game("aaaa000000000001", &OPENING),
        game("aaaa000000000002", &turned),
    ];
    let census = sequence_census(&records);
    assert_eq!(
        census.classes.len(),
        1,
        "a game and its rotation are the same game up to a symmetry"
    );
    assert_eq!(census.classes[0].members, vec![0, 1]);
    assert!(
        !census.classes[0].identical_under_identity,
        "they are equal only under a non-trivial element"
    );
    assert!(
        !census.classes[0].elements.is_empty(),
        "the carrying element is named, not left empty"
    );
}

#[test]
fn an_injected_divergent_pair_shares_an_opening_and_is_not_the_same_game() {
    let mut diverged: Vec<(i16, i16)> = OPENING.to_vec();
    diverged.extend([(0, 5), (0, 6)]);
    let mut other: Vec<(i16, i16)> = OPENING.to_vec();
    other.extend([(-1, 0), (-2, 0)]);

    let records = vec![
        game("bbbb000000000001", &diverged),
        game("bbbb000000000002", &other),
    ];
    assert!(
        sequence_census(&records).classes.is_empty(),
        "games that diverge after a shared opening are not the same game"
    );
    let early = early_census(&records);
    assert_eq!(early.classes.len(), 1, "they do share the opening");
    assert!(
        early.classes[0].identical_under_identity,
        "the shared opening is shared in the same orientation"
    );
}

#[test]
fn a_class_is_described_on_the_stones_its_key_was_built_from() {
    // The regression: an early-position class described by its members' WHOLE
    // games compares boards no key looked at, and reports a class with a
    // carrying element as having none.
    let turned = rotated(&OPENING, Symmetry::ALL[1]);
    let mut left: Vec<(i16, i16)> = OPENING.to_vec();
    left.extend([(0, 5), (0, 6)]);
    let mut right: Vec<(i16, i16)> = turned.clone();
    right.extend([(9, 9), (10, 10)]);

    let records = vec![
        game("cccc000000000001", &left),
        game("cccc000000000002", &right),
    ];
    let early = early_census(&records);
    assert_eq!(early.classes.len(), 1, "the openings are one class");
    assert!(
        !early.classes[0].elements.is_empty(),
        "the element carrying one opening onto the other is reported"
    );
    assert!(
        sequence_census(&records).classes.is_empty(),
        "the whole games are not the same game"
    );
}

#[test]
fn discarding_colour_merges_openings_that_colour_keeps_apart() {
    // Same cells, different owners: this is the WP-P1 gate's comparison, which
    // was uncoloured because who owns a stone was a separate unknown (D-437).
    let stones: Vec<(Coord, Player)> = OPENING
        .iter()
        .map(|&(q, r)| (Coord::new(q, r), Player::P1))
        .collect();
    let recoloured: Vec<(Coord, Player)> = OPENING
        .iter()
        .enumerate()
        .map(|(index, &(q, r))| {
            let player = if index % 2 == 0 {
                Player::P1
            } else {
                Player::P2
            };
            (Coord::new(q, r), player)
        })
        .collect();
    assert_ne!(
        transform(&stones, Symmetry::IDENTITY),
        transform(&recoloured, Symmetry::IDENTITY),
        "the colourings differ"
    );
    assert_eq!(
        uncoloured(&stones),
        uncoloured(&recoloured),
        "discarding colour merges them"
    );
    assert_eq!(
        flatten(&recoloured).len(),
        recoloured.len(),
        "flattening keeps every stone"
    );
}

#[test]
fn the_census_is_deterministic_over_two_runs() {
    let mut records = Vec::new();
    for (index, symmetry) in Symmetry::ALL.iter().enumerate() {
        let moves = rotated(&OPENING, *symmetry);
        records.push(game(&format!("dddd00000000000{index:x}"), &moves));
    }
    let first = sequence_census(&records);
    let second = sequence_census(&records);
    assert_eq!(first, second, "the same corpus gives the same census");
    assert_eq!(
        format!("{first}"),
        format!("{second}"),
        "and the same rendering"
    );
}

#[test]
fn a_game_shorter_than_the_early_cut_carries_no_early_key() {
    let short = keys_of(&[Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)])
        .expect("three stones group into turns");
    assert!(
        short.early_position.is_none(),
        "a game that never reached the cut is counted out, not bucketed"
    );
    let full = keys_of(
        &OPENING
            .iter()
            .map(|&(q, r)| Coord::new(q, r))
            .collect::<Vec<_>>(),
    )
    .expect("eleven stones group into turns");
    assert!(full.early_position.is_some());
    assert_eq!(full.early_stones.expect("present").len(), 11);
}

/// A game whose coloured board is fixed by a 180-degree turn, so its board and
/// its move sequence disagree about whether two games are the same.
const STABILIZED: [(i16, i16); 9] = [
    (0, 0),
    (3, 1),
    (4, 2),
    (1, 0),
    (0, 2),
    (-3, -1),
    (-4, -2),
    (-1, 0),
    (0, -2),
];

#[test]
fn a_sequence_class_is_not_called_an_exact_duplicate_when_only_a_symmetry_relates_it() {
    // The regression both WP-P1b reviews found independently: the Sequence
    // census was described with BOARDS, so a pair whose boards coincide under
    // rot3 but whose move orders differ was reported `identity-equal true` —
    // asserting an exact duplicate where the truth is an orientation duplicate,
    // which inverts the discrimination the census exists to make.
    let turned = rotated(&STABILIZED, Symmetry::ALL[3]);
    let records = vec![
        game("eeee000000000001", &STABILIZED),
        game("eeee000000000002", &turned),
    ];
    let census = sequence_census(&records);
    assert_eq!(census.classes.len(), 1, "they are one game up to rot3");
    assert!(
        !census.classes[0].identical_under_identity,
        "their move sequences differ, so this is not an exact duplicate"
    );
    assert_eq!(
        census.classes[0].elements,
        vec![Symmetry::ALL[3]],
        "and the element that carries one onto the other is named"
    );
}

#[test]
fn the_owner_of_each_stone_follows_the_turn_structure() {
    // Binds `owner_of`: with every stone owned by P1 the coloured censuses and
    // the coloured-versus-uncoloured contrast would still run and still report.
    let keys = keys_of(
        &OPENING
            .iter()
            .map(|&(q, r)| Coord::new(q, r))
            .collect::<Vec<_>>(),
    )
    .expect("groups");
    let owners: Vec<Player> = keys.stones.iter().map(|&(_, player)| player).collect();
    assert_eq!(
        owners,
        vec![
            Player::P1,
            Player::P2,
            Player::P2,
            Player::P1,
            Player::P1,
            Player::P2,
            Player::P2,
            Player::P1,
            Player::P1,
            Player::P2,
            Player::P2,
        ],
        "turn 1 is one stone for P1, then two per turn alternating (rule 3)"
    );
}

#[test]
fn the_final_position_key_is_canonicalized() {
    // Binds the final-position key: without canonicalization two games that end
    // on the same board in different orientations would not collide, and the
    // 0-classes result would be vacuous.
    let mut left: Vec<(i16, i16)> = OPENING.to_vec();
    left.push((7, 0));
    let right = rotated(&left, Symmetry::ALL[4]);
    let records = vec![
        game("ffff000000000001", &left),
        game("ffff000000000002", &right),
    ];
    let census = final_census(&records);
    assert_eq!(
        census.classes.len(),
        1,
        "the same final board in two orientations is one class"
    );
    assert!(!census.classes[0].identical_under_identity);
}

#[test]
fn a_reflection_only_duplicate_is_detected() {
    // The suite's other duplicate is a pure rotation, so a reflection-blind
    // canonicalization would pass it. `Symmetry::ALL[6]` is refl-rot0.
    let reflected = rotated(&OPENING, Symmetry::ALL[6]);
    let records = vec![
        game("aaaa00000000000a", &OPENING),
        game("aaaa00000000000b", &reflected),
    ];
    let census = sequence_census(&records);
    assert_eq!(census.classes.len(), 1, "a mirrored game is the same game");
    assert_eq!(census.classes[0].elements, vec![Symmetry::ALL[6]]);
}

#[test]
fn the_reported_totals_and_rendering_are_bound_to_the_classes() {
    // Binds `keyed`, `colliding_games`, `size_distribution` and the Display
    // block — the rendered block every reported number is transcribed from.
    // D-219 records this exact gap being paid for once already in this module.
    let mut records = vec![game("bbbb00000000000a", &OPENING)];
    for (index, symmetry) in Symmetry::ALL.iter().enumerate().take(3).skip(1) {
        records.push(game(
            &format!("bbbb00000000001{index}"),
            &rotated(&OPENING, *symmetry),
        ));
    }
    let mut other: Vec<(i16, i16)> = OPENING.to_vec();
    other[10] = (-6, 0);
    records.push(game("cccc00000000000a", &other));
    records.push(game("cccc00000000000b", &rotated(&other, Symmetry::ALL[7])));

    let census = sequence_census(&records);
    assert_eq!(census.keyed, 5, "every game carried a key");
    assert_eq!(census.classes.len(), 2);
    assert_eq!(census.colliding_games(), 5);
    let sizes = census.size_distribution();
    assert_eq!(sizes.get(&3), Some(&1), "one class of three");
    assert_eq!(sizes.get(&2), Some(&1), "one class of two");
    assert_eq!(sizes.len(), 2, "and no other size");

    let rendered = format!("{census}");
    assert!(rendered.contains("games keyed             5"), "{rendered}");
    assert!(rendered.contains("classes of size >= 2    2"), "{rendered}");
    assert!(rendered.contains("games in such a class   5"), "{rendered}");
    assert!(
        rendered.contains("classes needing a non-trivial element        2"),
        "{rendered}"
    );

    assert!(
        census.classes[0].members[0] < census.classes[1].members[0],
        "classes are ordered by their least member, so the report is stable"
    );
}

#[test]
fn a_game_off_the_addressable_lattice_is_refused_by_name_and_not_by_panic() {
    // The reader's guard is the i16 range; a rotation's domain is narrower, so
    // a schema-valid game can reach an overflow that `Symmetry::apply` would
    // panic on. Rule 3 wants a named refusal.
    let far = [Coord::new(0, 0), Coord::new(20000, 20000), Coord::new(1, 0)];
    assert_eq!(
        keys_or_refusal(&far, 6),
        Err(OFF_ADDRESSABLE_LATTICE),
        "named, not a panic"
    );
    assert!(keys_of(&far).is_none());
}
