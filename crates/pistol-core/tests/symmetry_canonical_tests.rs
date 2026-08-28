use std::collections::BTreeSet;

use pistol_core::symmetry::transform;
use pistol_core::{Coord, Player, Symmetry, canonical_form};

#[test]
fn canonical_form_is_the_same_for_every_image_of_a_position() {
    // The promise the corpus dedupe rests on: twelve spellings, one answer.
    let position = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(2, -1), Player::P2),
        (Coord::new(1, 3), Player::P1),
        (Coord::new(-2, 0), Player::P2),
    ];
    let expected = canonical_form(&position);
    for symmetry in Symmetry::ALL {
        let image = transform(&position, symmetry);
        assert_eq!(
            canonical_form(&image),
            expected,
            "the image of the position under {symmetry} has a different canonical form"
        );
    }
}

#[test]
fn canonical_form_ignores_the_order_stones_are_given_in() {
    let played = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(2, -1), Player::P2),
        (Coord::new(1, 3), Player::P1),
    ];
    let mut shuffled = played.clone();
    shuffled.reverse();
    assert_eq!(canonical_form(&played), canonical_form(&shuffled));
}

#[test]
fn canonical_form_keeps_colour_so_a_colour_swap_is_a_different_position() {
    let position = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(1, 0), Player::P2),
    ];
    let swapped = vec![
        (Coord::new(0, 0), Player::P2),
        (Coord::new(1, 0), Player::P1),
    ];
    assert_ne!(
        canonical_form(&position),
        canonical_form(&swapped),
        "a colour swap changes whose position it is, so it is not a symmetry of it"
    );
}

#[test]
fn a_position_with_no_symmetry_has_twelve_distinct_images() {
    let position = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(3, -1), Player::P2),
        (Coord::new(1, 4), Player::P1),
    ];
    let images: BTreeSet<Vec<(Coord, Player)>> = Symmetry::ALL
        .into_iter()
        .map(|symmetry| transform(&position, symmetry))
        .collect();
    assert_eq!(images.len(), 12);
    assert!(images.contains(&canonical_form(&position)));
}

#[test]
fn the_canonical_form_is_one_of_the_images() {
    let position = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(-3, 2), Player::P2),
        (Coord::new(4, 4), Player::P1),
    ];
    let canonical = canonical_form(&position);
    assert!(
        Symmetry::ALL
            .into_iter()
            .any(|symmetry| transform(&position, symmetry) == canonical),
        "the canonical form is not an image of the position it came from"
    );
}

#[test]
fn the_canonical_form_is_the_least_image_not_merely_some_image() {
    // Pinned because nothing else does: taking the maximum instead would satisfy
    // every invariance property and partition positions identically, so the
    // suite would stay green while the doc's stated choice quietly changed. That
    // becomes load-bearing the moment a canonical value reaches a sha-pinned
    // fixture (CLAUDE.md rule 7).
    let position = vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(3, -1), Player::P2),
        (Coord::new(1, 4), Player::P1),
    ];
    let canonical = canonical_form(&position);
    for symmetry in Symmetry::ALL {
        assert!(
            canonical <= transform(&position, symmetry),
            "the image under {symmetry} sorts below the canonical form"
        );
    }
}
