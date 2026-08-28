use pistol_core::symmetry::{canonical_sequence, transform_sequence};
use pistol_core::{Coord, Symmetry, Turn, canonical_form};

fn cell(q: i16, r: i16) -> Coord {
    Coord::new(q, r)
}

fn pair(a: (i16, i16), b: (i16, i16)) -> Turn {
    Turn::pair(cell(a.0, a.1), cell(b.0, b.1)).expect("two distinct cells")
}

/// A game: one stone, then pairs. Turn 1 is a single stone at the origin
/// (game rule 3), which is also the shape every real game has.
///
/// `game_one` and `game_two` differ only in the ORDER the first player plays
/// its two pairs — turns 3 and 5 are exchanged — so the two games reach the
/// same cells with the same colours while being different games. Colour is what
/// an earlier draft of this fixture got wrong: redistributing stones ACROSS
/// turns changes who owns them, and then the positions differ too and the test
/// proves nothing.
fn game_one() -> Vec<Turn> {
    vec![
        Turn::single(cell(0, 0)),
        pair((1, 0), (2, 0)),
        pair((0, 1), (0, 2)),
        pair((3, 0), (4, 0)),
        pair((0, 3), (0, 4)),
    ]
}

/// `game_one` with the first player's two pairs exchanged.
fn game_two() -> Vec<Turn> {
    vec![
        Turn::single(cell(0, 0)),
        pair((1, 0), (2, 0)),
        pair((0, 3), (0, 4)),
        pair((3, 0), (4, 0)),
        pair((0, 1), (0, 2)),
    ]
}

#[test]
fn two_games_reaching_the_same_stones_are_two_distinct_games() {
    // The trap this whole module exists for: keying a game on a POSITION's
    // canonical form merges these, and an arena would then under-report
    // distinct-n — the opposite of the error symmetry folding is for.
    let one = game_one();
    let two = game_two();
    assert_ne!(one, two, "the two fixtures are different games");

    let stones = |game: &[Turn]| {
        let mut out = Vec::new();
        let mut player = pistol_core::Player::P1;
        for turn in game {
            out.push((turn.first(), player));
            if let Some(second) = turn.second() {
                out.push((second, player));
            }
            player = match player {
                pistol_core::Player::P1 => pistol_core::Player::P2,
                pistol_core::Player::P2 => pistol_core::Player::P1,
            };
        }
        out
    };
    assert_eq!(
        canonical_form(&stones(&one)),
        canonical_form(&stones(&two)),
        "the two games reach the same position, which is why the position's \
         canonical form is the wrong key for a game"
    );
    assert_ne!(
        canonical_sequence(&one),
        canonical_sequence(&two),
        "and the sequence's canonical form keeps them apart"
    );
}

#[test]
fn every_image_of_a_game_has_the_same_canonical_sequence() {
    let game = game_one();
    let canonical = canonical_sequence(&game);
    for symmetry in Symmetry::ALL {
        let image = transform_sequence(&game, symmetry);
        assert_eq!(
            canonical_sequence(&image),
            canonical,
            "a game and its image under {symmetry:?} are the same game"
        );
    }
}

#[test]
fn a_transformed_pair_is_returned_in_canonical_order() {
    // The defect a hand-composed `apply` has: a symmetry does not preserve the
    // (q, r) order the pair token is pinned to (docs/decisions.md D-5), so an
    // image that skipped re-canonicalization would not be symmetry-invariant.
    // Asserted over every turn of every image, so this cannot pass by accident
    // on a pair that happened to stay ordered.
    let game = game_one();
    let mut saw_a_reordering = false;
    for symmetry in Symmetry::ALL {
        for (before, after) in game.iter().zip(transform_sequence(&game, symmetry)) {
            assert!(
                after.is_canonical(),
                "{before} under {symmetry:?} came back as {after}, which is not canonical"
            );
            if let (Turn::Pair(first, _), Turn::Pair(image_first, _)) = (*before, after)
                && symmetry.apply(first) != image_first
            {
                saw_a_reordering = true;
            }
        }
    }
    assert!(
        saw_a_reordering,
        "no symmetry in this fixture actually reordered a pair, so the assertion \
         above proved nothing; the fixture needs a pair the group reorders"
    );
}

#[test]
fn the_canonical_sequence_is_one_of_the_twelve_images() {
    let game = game_one();
    let canonical = canonical_sequence(&game);
    let images: Vec<Vec<Turn>> = Symmetry::ALL
        .iter()
        .map(|&symmetry| transform_sequence(&game, symmetry))
        .collect();
    assert!(
        images.contains(&canonical),
        "the canonical form is an image, not a construction"
    );
    assert_eq!(
        images.iter().min().expect("twelve images"),
        &canonical,
        "and it is the LEAST image, not merely some image"
    );
}

#[test]
fn a_single_stone_turn_survives_transformation_as_a_single() {
    // Rule 4's truncation is carried by the key, not merged into a pair: a game
    // won on its turn's first stone must not become indistinguishable from one
    // that played both.
    for symmetry in Symmetry::ALL {
        let image = transform_sequence(&[Turn::single(cell(3, -1))], symmetry);
        assert_eq!(image.len(), 1);
        assert_eq!(image[0].stone_count(), 1, "a single stone stays one stone");
    }
}
