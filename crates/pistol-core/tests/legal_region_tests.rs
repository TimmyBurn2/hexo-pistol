//! The legal region (game rule 5), and the first stone (game rule 3).
//!
//! The region is the **union** of the radius-8 balls around the stones. Every
//! test here is written so that it would fail against the plausible wrong
//! implementations: distance from the first stone, distance from the last
//! stone, distance from the centroid, a bounding box, or a union with an
//! off-by-one where two balls overlap.

use pistol_core::{Board, Coord, CoreError, GameState, LEGAL_RADIUS, Player};

/// Cells the implementation calls legal region, over a box that reaches at
/// least one cell beyond the far side of every ball.
fn region_over_box(board: &Board, centres: &[Coord]) -> Vec<Coord> {
    let pad = i16::try_from(LEGAL_RADIUS).expect("the rule radius fits a coordinate") + 2;
    let min_q = centres.iter().map(|c| c.q).min().expect("some centre") - pad;
    let max_q = centres.iter().map(|c| c.q).max().expect("some centre") + pad;
    let min_r = centres.iter().map(|c| c.r).min().expect("some centre") - pad;
    let max_r = centres.iter().map(|c| c.r).max().expect("some centre") + pad;

    let mut cells = Vec::new();
    for q in min_q..=max_q {
        for r in min_r..=max_r {
            let cell = Coord::new(q, r);
            if board.in_legal_region(cell) {
                cells.push(cell);
            }
        }
    }
    cells
}

/// The oracle, said the way the rule says it.
fn union_of_balls(centres: &[Coord], cell: Coord) -> bool {
    centres
        .iter()
        .any(|centre| centre.distance(cell) <= LEGAL_RADIUS)
}

fn board_with(stones: &[(Coord, Player)]) -> Board {
    let mut board = Board::empty();
    for &(at, player) in stones {
        board.apply(at, player).expect("distinct fixture cells");
    }
    board
}

#[test]
fn legal_region_is_radius8_union() {
    // One stone: the closed form for a hex ball of radius R is 1 + 3R(R+1),
    // which is 217 at R = 8. Stated here as the pin it is — an implementation
    // that excluded the centre would give 216 and must not be able to have the
    // expected number "corrected" to match it afterwards.
    let single = board_with(&[(Coord::ORIGIN, Player::P1)]);
    let centres = [Coord::ORIGIN];
    let region = region_over_box(&single, &centres);
    assert_eq!(region.len(), 1 + 3 * 8 * 9, "cells within radius 8");
    assert_eq!(region.len(), 217);
    assert!(
        single.in_legal_region(Coord::ORIGIN),
        "the stone's own cell"
    );
    // ... and the cell under the stone is in the region but not placeable, so
    // the placement count is one lower.
    let placeable = region
        .iter()
        .filter(|&&cell| single.is_legal_placement(cell))
        .count();
    assert_eq!(placeable, 216);

    // Two clouds a hundred cells apart: the region is two disjoint balls, so an
    // implementation measuring from any single point — first stone, last stone,
    // centroid — gets this wrong.
    let far = Coord::new(100, -50);
    assert_eq!(Coord::ORIGIN.distance(far), 100);
    let two_cloud = board_with(&[(Coord::ORIGIN, Player::P1), (far, Player::P2)]);
    let centres = [Coord::ORIGIN, far];
    let region = region_over_box(&two_cloud, &centres);
    for &cell in &region {
        assert!(
            union_of_balls(&centres, cell),
            "{cell} is not within {LEGAL_RADIUS} of either stone"
        );
    }
    assert_eq!(region.len(), 2 * 217, "two disjoint balls");
    assert!(
        two_cloud.in_legal_region(Coord::new(108, -50)),
        "8 from far"
    );
    assert!(
        !two_cloud.in_legal_region(Coord::new(109, -50)),
        "9 from far"
    );
    assert!(
        !two_cloud.in_legal_region(Coord::new(50, -25)),
        "between them"
    );

    // Overlapping balls: the union is strictly smaller than the sum, and every
    // cell in the box agrees with the oracle — which is what catches an
    // off-by-one in the overlap that a disjoint case cannot see.
    let near = Coord::new(5, 0);
    let overlapping = board_with(&[(Coord::ORIGIN, Player::P1), (near, Player::P2)]);
    let centres = [Coord::ORIGIN, near];
    let pad = i16::try_from(LEGAL_RADIUS).expect("fits") + 2;
    let mut union_count = 0;
    for q in -pad..=(near.q + pad) {
        for r in (near.r - pad)..=pad {
            let cell = Coord::new(q, r);
            assert_eq!(
                overlapping.in_legal_region(cell),
                union_of_balls(&centres, cell),
                "{cell}"
            );
            if union_of_balls(&centres, cell) {
                union_count += 1;
            }
        }
    }
    assert!(
        (218..2 * 217).contains(&union_count),
        "overlapping balls: {union_count} cells"
    );
}

#[test]
fn legal_region_ignores_stone_player() {
    // Rule 5 says "an existing stone", not "an own stone".
    let p1 = board_with(&[(Coord::ORIGIN, Player::P1)]);
    let p2 = board_with(&[(Coord::ORIGIN, Player::P2)]);
    for q in -12..=12 {
        for r in -12..=12 {
            let cell = Coord::new(q, r);
            assert_eq!(p1.in_legal_region(cell), p2.in_legal_region(cell), "{cell}");
        }
    }
}

#[test]
fn empty_board_legal_region_is_the_origin_only() {
    let empty = Board::empty();
    for q in -12..=12 {
        for r in -12..=12 {
            let cell = Coord::new(q, r);
            assert_eq!(
                empty.in_legal_region(cell),
                cell == Coord::ORIGIN,
                "{cell} on an empty board"
            );
        }
    }

    // And the refusal names rule 3, not rule 5: an empty board has no stone for
    // a distance to be measured from.
    assert_eq!(
        empty.check_placement(Coord::new(3, -2)),
        Err(CoreError::FirstStoneNotAtOrigin {
            at: Coord::new(3, -2)
        })
    );
    assert_eq!(empty.check_placement(Coord::ORIGIN), Ok(()));
}

#[test]
fn occupied_cell_is_not_a_legal_placement() {
    let board = board_with(&[(Coord::ORIGIN, Player::P1)]);
    assert!(board.in_legal_region(Coord::ORIGIN), "still in the region");
    assert!(!board.is_legal_placement(Coord::ORIGIN));
    assert_eq!(
        board.check_placement(Coord::ORIGIN),
        Err(CoreError::OccupiedCell { at: Coord::ORIGIN })
    );

    // Two rules, two names: outside the region reads differently from taken.
    let outside = Coord::new(9, 0);
    assert_eq!(Coord::ORIGIN.distance(outside), 9);
    assert_eq!(
        board.check_placement(outside),
        Err(CoreError::OutsideLegalRegion { at: outside })
    );
}

#[test]
fn second_stone_may_anchor_on_the_first_stone_of_the_same_turn() {
    // The region grows as the turn is played, not at the end of it: the second
    // stone of a turn sees the first one. This is what makes a pair legal in
    // one order and not the other (docs/decisions.md D-6).
    let mut game = GameState::new_game();
    game.place(Coord::ORIGIN)
        .expect("first stone on the origin");

    let anchor = Coord::new(8, 0);
    let far = Coord::new(16, 0);
    assert_eq!(Coord::ORIGIN.distance(anchor), 8);
    assert_eq!(Coord::ORIGIN.distance(far), 16);
    assert_eq!(anchor.distance(far), 8);

    game.place(anchor).expect("8 from the origin is legal");
    game.place(far)
        .expect("8 from the stone just placed is legal");
    assert_eq!(game.board().stone_count(), 3);
}

#[test]
fn pair_legal_in_only_one_order_is_rejected_in_the_other() {
    let mut game = GameState::new_game();
    game.place(Coord::ORIGIN)
        .expect("first stone on the origin");

    let far = Coord::new(16, 0);
    assert_eq!(
        game.place(far),
        Err(CoreError::OutsideLegalRegion { at: far }),
        "16 from the only stone on the board"
    );
    // The refusal changed nothing: the same pair still plays in the other order.
    assert_eq!(game.board().stone_count(), 1);
    game.place(Coord::new(8, 0)).expect("the anchor first");
    game.place(far).expect("then the far stone");
}
