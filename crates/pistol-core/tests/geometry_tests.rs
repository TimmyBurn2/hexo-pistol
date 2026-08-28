use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, VecDeque};

use pistol_core::coord::COORD_OVERFLOW;
use pistol_core::{Axis, Coord, NEIGHBOUR_DIRECTIONS};

/// The reference: cube distance is the largest absolute cube-coordinate
/// difference. The implementation uses the axial half-sum form instead, so the
/// two are independent statements of the same quantity.
fn reference_distance(from: Coord, to: Coord) -> u32 {
    let dx = i64::from(to.q) - i64::from(from.q);
    let dz = i64::from(to.r) - i64::from(from.r);
    let dy = -dx - dz;
    let largest = dx.abs().max(dy.abs()).max(dz.abs());
    u32::try_from(largest).expect("a distance is not negative")
}

#[test]
fn hex_distance_matches_reference_formula() {
    for from_q in -6..=6 {
        for from_r in -6..=6 {
            for to_q in -6..=6 {
                for to_r in -6..=6 {
                    let from = Coord::new(from_q, from_r);
                    let to = Coord::new(to_q, to_r);
                    assert_eq!(
                        from.distance(to),
                        reference_distance(from, to),
                        "distance {from} -> {to}"
                    );
                    assert_eq!(from.distance(to), to.distance(from), "{from} <-> {to}");
                }
            }
        }
    }

    // The extremes, explicitly: the differences do not fit in an i16, so an
    // implementation that subtracts before widening gets these wrong and
    // everything near the origin right.
    let extremes = [
        (
            Coord::new(i16::MIN, i16::MIN),
            Coord::new(i16::MAX, i16::MAX),
        ),
        (
            Coord::new(i16::MIN, i16::MAX),
            Coord::new(i16::MAX, i16::MIN),
        ),
        (Coord::new(i16::MIN, 0), Coord::new(i16::MAX, 0)),
        (Coord::new(0, i16::MIN), Coord::new(0, i16::MAX)),
        (Coord::new(i16::MAX, i16::MIN), Coord::ORIGIN),
    ];
    for (from, to) in extremes {
        assert_eq!(
            from.distance(to),
            reference_distance(from, to),
            "distance {from} -> {to}"
        );
    }

    assert_eq!(Coord::ORIGIN.distance(Coord::ORIGIN), 0);
}

/// The second oracle: distance is the number of single-cell steps, so a
/// breadth-first walk over the six neighbours has to agree with it.
#[test]
fn hex_distance_equals_the_number_of_single_cell_steps() {
    let mut steps: BTreeMap<Coord, u32> = BTreeMap::new();
    let mut queue: VecDeque<Coord> = VecDeque::new();
    steps.insert(Coord::ORIGIN, 0);
    queue.push_back(Coord::ORIGIN);

    const HORIZON: u32 = 5;
    while let Some(cell) = queue.pop_front() {
        let depth = steps[&cell];
        if depth == HORIZON {
            continue;
        }
        for neighbour in cell.neighbours() {
            if let Entry::Vacant(slot) = steps.entry(neighbour) {
                slot.insert(depth + 1);
                queue.push_back(neighbour);
            }
        }
    }

    for (&cell, &depth) in &steps {
        assert_eq!(
            Coord::ORIGIN.distance(cell),
            depth,
            "{cell} is {depth} steps out"
        );
    }
    // 1 + 3R(R+1) cells within R steps, so the walk has to have found them all.
    assert_eq!(
        steps.len(),
        1 + 3 * (HORIZON as usize) * (HORIZON as usize + 1)
    );
}

#[test]
fn axis_directions_are_the_three_hex_lines() {
    assert_eq!(Axis::ALL, [Axis::ConstQ, Axis::ConstR, Axis::ConstS]);
    assert_eq!(Axis::ConstQ.direction(), Coord::new(0, 1));
    assert_eq!(Axis::ConstR.direction(), Coord::new(1, 0));
    assert_eq!(Axis::ConstS.direction(), Coord::new(1, -1));

    // Each axis holds the coordinate it is named for constant, in both
    // directions, however far it is walked.
    let start = Coord::new(-3, 7);
    for steps in -20..=20 {
        assert_eq!(start.step(Axis::ConstQ, steps).q, start.q);
        assert_eq!(start.step(Axis::ConstR, steps).r, start.r);
        assert_eq!(start.step(Axis::ConstS, steps).s(), start.s());
        // ... and moves every other one, so no two axes are the same line.
        if steps != 0 {
            assert_ne!(start.step(Axis::ConstQ, steps).r, start.r);
            assert_ne!(start.step(Axis::ConstR, steps).q, start.q);
            assert_ne!(start.step(Axis::ConstS, steps).q, start.q);
        }
    }

    // Three axes, not six half-directions: no axis is another's opposite.
    for axis in Axis::ALL {
        for other in Axis::ALL {
            if axis != other {
                assert_ne!(axis.direction(), other.direction().negated());
            }
        }
        assert_eq!(Coord::ORIGIN.step(axis, 1), axis.direction());
        assert_eq!(Coord::ORIGIN.distance(axis.direction()), 1);
    }
}

#[test]
fn neighbours_are_the_six_ring_cells_in_fixed_order() {
    assert_eq!(
        Coord::ORIGIN.neighbours(),
        [
            Coord::new(1, 0),
            Coord::new(1, -1),
            Coord::new(0, -1),
            Coord::new(-1, 0),
            Coord::new(-1, 1),
            Coord::new(0, 1),
        ]
    );

    let cell = Coord::new(11, -4);
    let ring = cell.neighbours();
    for (index, neighbour) in ring.iter().enumerate() {
        assert_eq!(cell.distance(*neighbour), 1, "{neighbour} is not adjacent");
        // Consecutive neighbours are adjacent to each other: the order walks
        // round the ring rather than jumping across it.
        let next = ring[(index + 1) % ring.len()];
        assert_eq!(neighbour.distance(next), 1, "{neighbour} -> {next}");
    }
    for (index, neighbour) in ring.iter().enumerate() {
        assert!(
            !ring[index + 1..].contains(neighbour),
            "{neighbour} appears twice"
        );
    }
}

#[test]
fn neighbour_directions_are_exactly_the_axis_directions_and_their_negations() {
    let mut expected: Vec<Coord> = Axis::ALL
        .iter()
        .flat_map(|axis| [axis.direction(), axis.direction().negated()])
        .collect();
    let mut actual: Vec<Coord> = NEIGHBOUR_DIRECTIONS.to_vec();
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected);
}

#[test]
fn coord_ordering_is_lexicographic_q_then_r() {
    let mut cells = vec![
        Coord::new(1, -5),
        Coord::new(0, 3),
        Coord::new(-1, 100),
        Coord::new(0, -2),
        Coord::new(1, -6),
    ];
    cells.sort();
    assert_eq!(
        cells,
        vec![
            Coord::new(-1, 100),
            Coord::new(0, -2),
            Coord::new(0, 3),
            Coord::new(1, -6),
            Coord::new(1, -5),
        ]
    );
    assert!(Coord::new(0, i16::MAX) < Coord::new(1, i16::MIN), "q first");
}

#[test]
#[should_panic(expected = "COORD_OVERFLOW")]
fn coord_arithmetic_panics_on_overflow() {
    // Not a debug assertion: the check is a `checked_add`, so this panics in a
    // release build too, where a bare `+` would silently wrap.
    let _ = Coord::new(i16::MAX, 0).offset(Coord::new(1, 0));
}

#[test]
#[should_panic(expected = "COORD_OVERFLOW")]
fn coord_step_past_the_lattice_panics() {
    let _ = Coord::new(0, i16::MAX - 2).step(Axis::ConstQ, 3);
}

#[test]
fn checked_arithmetic_reports_the_edge_instead_of_wrapping() {
    assert_eq!(
        Coord::new(i16::MAX, 0).checked_offset(Coord::new(1, 0)),
        None
    );
    assert_eq!(Coord::new(0, i16::MIN).checked_step(Axis::ConstQ, -1), None);
    assert_eq!(
        Coord::new(i16::MAX - 1, 0).checked_offset(Coord::new(1, 0)),
        Some(Coord::new(i16::MAX, 0))
    );
    assert_eq!(COORD_OVERFLOW, "COORD_OVERFLOW");
}

#[test]
fn a_step_is_judged_by_where_it_lands_not_by_how_far_it_reaches() {
    let from = Coord::new(i16::MAX, i16::MIN);
    assert_eq!(
        from.checked_step(Axis::ConstS, i16::MIN),
        Some(Coord::new(-1, 0)),
        "the destination is addressable, so it is not `None`"
    );
    assert_eq!(from.step(Axis::ConstS, i16::MIN), Coord::new(-1, 0));

    // Every extreme step count from every corner: `checked_step` says `Some`
    // exactly when the destination fits, and never merely because the walk was
    // long.
    for &q in &[i16::MIN, -1, 0, 1, i16::MAX] {
        for &r in &[i16::MIN, -1, 0, 1, i16::MAX] {
            let from = Coord::new(q, r);
            for axis in Axis::ALL {
                for steps in [i16::MIN, i16::MIN + 1, -1, 0, 1, i16::MAX] {
                    let direction = axis.direction();
                    let landed_q = i32::from(q) + i32::from(direction.q) * i32::from(steps);
                    let landed_r = i32::from(r) + i32::from(direction.r) * i32::from(steps);
                    let addressable =
                        i16::try_from(landed_q).is_ok() && i16::try_from(landed_r).is_ok();
                    assert_eq!(
                        from.checked_step(axis, steps).is_some(),
                        addressable,
                        "{from} stepped {steps} along {axis:?} lands on ({landed_q},{landed_r})"
                    );
                }
            }
        }
    }
}
