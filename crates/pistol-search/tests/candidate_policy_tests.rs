mod common;

use std::collections::BTreeSet;

use common::{line, position, spectators};
use pistol_core::{Axis, Board, Coord, LEGAL_RADIUS, Player};
use pistol_search::{CandidatePolicy, candidate_cells};

/// Every empty, legally placeable cell within `radius` of some stone, found by
/// scanning a box that is wide enough to contain the answer.
///
/// This shares no code with the generator: it asks the board one cell at a time
/// (`Board::check_placement` is rule 5's own answer) and measures distance from
/// each stone directly.
fn by_scan(board: &Board, radius: u32) -> BTreeSet<Coord> {
    let reach = i16::try_from(radius.max(LEGAL_RADIUS)).expect("test radii are small");
    let stones: Vec<Coord> = board.stones().map(|(at, _)| at).collect();
    let (min_q, max_q) = extent(stones.iter().map(|at| at.q), reach);
    let (min_r, max_r) = extent(stones.iter().map(|at| at.r), reach);

    let mut cells = BTreeSet::new();
    for q in min_q..=max_q {
        for r in min_r..=max_r {
            let cell = Coord::new(q, r);
            let near = stones.iter().any(|&stone| stone.distance(cell) <= radius);
            if near && board.is_legal_placement(cell) {
                cells.insert(cell);
            }
        }
    }
    cells
}

fn extent(values: impl Iterator<Item = i16>, reach: i16) -> (i16, i16) {
    let values: Vec<i16> = values.collect();
    let low = *values.iter().min().expect("a position has stones") - reach;
    let high = *values.iter().max().expect("a position has stones") + reach;
    (low, high)
}

/// Four P1 stones on one axis and P2 sitting well clear of them.
fn two_clusters() -> pistol_core::GameState {
    let p1 = line(Coord::ORIGIN, Axis::ConstR, 5);
    let p2 = spectators(Coord::new(0, 6), Axis::ConstR, 6);
    position(&p1, &p2, Player::P1)
}

#[test]
fn candidate_radius_comes_from_config_not_constant() {
    let state = two_clusters();
    let board = state.board();

    let near: BTreeSet<Coord> = candidate_cells(board, CandidatePolicy::Radius { radius: 1 })
        .into_iter()
        .collect();
    let far: BTreeSet<Coord> = candidate_cells(board, CandidatePolicy::Radius { radius: 2 })
        .into_iter()
        .collect();

    // Each radius yields exactly what an independent scan says it should.
    assert_eq!(near, by_scan(board, 1), "radius 1 candidates");
    assert_eq!(far, by_scan(board, 2), "radius 2 candidates");

    // And the two differ: the radius is read, not ignored in favour of a
    // constant. A strict superset is the strongest form of "different" here —
    // it also rules out a generator that reads the radius but drops cells.
    assert!(
        near.is_subset(&far) && near.len() < far.len(),
        "radius 2 must reach strictly further than radius 1: {} vs {} cells",
        near.len(),
        far.len()
    );

    // Nothing in either set is a cell the rules refuse.
    for cell in &far {
        assert!(
            board.is_legal_placement(*cell),
            "{cell} is not a legal placement"
        );
    }
}

#[test]
fn candidate_cells_are_the_legal_region_when_the_radius_reaches_past_it() {
    // A radius wider than the rules' region cannot widen the region: the policy
    // narrows what the search looks at and never decides what is legal
    // (docs/decisions.md D-20). The two radii are never compared in the
    // generator; this is the test that would notice if one were substituted for
    // the other.
    let state = two_clusters();
    let board = state.board();
    let wide = candidate_cells(board, CandidatePolicy::Radius { radius: 32 });

    assert_eq!(
        wide.into_iter().collect::<BTreeSet<Coord>>(),
        by_scan(board, LEGAL_RADIUS),
        "a radius past the legal region yields the legal region and no more"
    );
}

#[test]
fn candidate_cells_on_an_empty_board_are_the_origin_alone() {
    // There is no stone to be near, so a proximity policy restricts nothing and
    // rule 3 decides: the first stone goes on the origin.
    let board = Board::empty();
    for radius in [1, 3, 8, 32] {
        assert_eq!(
            candidate_cells(&board, CandidatePolicy::Radius { radius }),
            vec![Coord::ORIGIN],
            "radius {radius} on an empty board"
        );
    }
}

#[test]
fn candidate_cells_are_ascending_and_unoccupied() {
    let state = two_clusters();
    let cells = candidate_cells(state.board(), CandidatePolicy::Radius { radius: 3 });
    assert!(
        cells.windows(2).all(|pair| pair[0] < pair[1]),
        "candidates are emitted in ascending (q, r) order, once each"
    );
    assert!(
        cells.iter().all(|&at| !state.board().is_occupied(at)),
        "an occupied cell is not a candidate"
    );
}

/// A radius this crate cannot honour is refused by name at construction, at
/// both ends of the range.
///
/// The engine validates its own document before a `Searcher` is ever built, but
/// `SearchParams` is public and a bench, an arena or a test constructs one
/// directly — so a radius the generator cannot express has to be refused here or
/// nowhere. It used to be repaired instead: the generator clamped it to the
/// widest ball a coordinate can hold and searched that, which is the silent
/// fallback CLAUDE.md rule 3 forbids.
#[test]
fn a_radius_this_search_cannot_honour_is_refused_by_name() {
    for radius in [0, u32::from(i16::MAX as u16) + 1, u32::MAX] {
        let refused = pistol_search::Searcher::new(
            common::params(radius, common::SMALL_TT),
            Box::new(pistol_eval::HandcraftedV0::new(common::committed_weights())),
        );
        let Err(error) = refused else {
            panic!("radius {radius} cannot be honoured, so it must be refused");
        };
        let said = error.to_string();
        assert!(
            said.contains("candidate_policy.radius"),
            "radius {radius} must name the key an operator edits, got: {said}"
        );
    }
}
