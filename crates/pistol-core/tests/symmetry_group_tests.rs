use std::collections::BTreeSet;

use pistol_core::symmetry::transform;
use pistol_core::{Axis, Coord, Player, Symmetry};

/// A spread of cells including the origin, the six neighbours, and cells that
/// lie on no axis through the origin.
fn sample_cells() -> Vec<Coord> {
    let mut cells = vec![Coord::ORIGIN];
    for q in -4i16..=4 {
        for r in -4i16..=4 {
            cells.push(Coord::new(q, r));
        }
    }
    cells
}

#[test]
fn the_group_has_twelve_distinct_elements() {
    // Distinct as transforms, not merely as values: two spellings that moved
    // every cell the same way would be one symmetry written twice.
    let cells = sample_cells();
    let mut images: BTreeSet<Vec<Coord>> = BTreeSet::new();
    for symmetry in Symmetry::ALL {
        images.insert(cells.iter().map(|&c| symmetry.apply(c)).collect());
    }
    assert_eq!(
        images.len(),
        12,
        "the point group of the hex lattice has order 12"
    );
}

#[test]
fn the_group_is_closed_under_composition() {
    let cells = sample_cells();
    let action =
        |symmetry: Symmetry| -> Vec<Coord> { cells.iter().map(|&c| symmetry.apply(c)).collect() };
    let known: BTreeSet<Vec<Coord>> = Symmetry::ALL.into_iter().map(action).collect();
    for first in Symmetry::ALL {
        for second in Symmetry::ALL {
            let composed: Vec<Coord> = cells
                .iter()
                .map(|&c| second.apply(first.apply(c)))
                .collect();
            assert!(
                known.contains(&composed),
                "{first} then {second} is not one of the twelve"
            );
        }
    }
}

#[test]
fn every_symmetry_has_an_inverse_in_the_group() {
    let cells = sample_cells();
    for first in Symmetry::ALL {
        assert!(
            Symmetry::ALL
                .into_iter()
                .any(|second| cells.iter().all(|&c| second.apply(first.apply(c)) == c)),
            "{first} has no inverse among the twelve"
        );
    }
}

#[test]
fn every_symmetry_preserves_hex_distance() {
    // The defining property: these are the isometries of the lattice that fix
    // the origin. A "symmetry" that moved two cells closer together would
    // change which cells are within LEGAL_RADIUS of each other (rule 5) and
    // which stones form a run (rule 2).
    let cells = sample_cells();
    for symmetry in Symmetry::ALL {
        for &a in &cells {
            for &b in &cells {
                assert_eq!(
                    symmetry.apply(a).distance(symmetry.apply(b)),
                    a.distance(b),
                    "{symmetry} changed the distance between {a} and {b}"
                );
            }
        }
    }
}

#[test]
fn every_symmetry_maps_the_three_axes_onto_the_three_axes() {
    // Rule 2 counts runs along three axes. A transform that took a line off the
    // three would turn a win into a non-win, so a symmetry has to permute them.
    let directions: BTreeSet<Coord> = Axis::ALL
        .into_iter()
        .flat_map(|axis| [axis.direction(), axis.direction().negated()])
        .collect();
    for symmetry in Symmetry::ALL {
        for &direction in &directions {
            let image = symmetry.apply(direction);
            assert!(
                directions.contains(&image),
                "{symmetry} took the axis direction {direction} to {image}, off the three axes"
            );
        }
    }
}

#[test]
fn the_rotation_has_order_six_and_the_reflection_is_an_involution() {
    let rotation = Symmetry::ALL[1];
    let reflection = Symmetry::ALL[6];
    for &cell in &sample_cells() {
        if cell == Coord::ORIGIN {
            continue; // fixed by everything; it says nothing about the order.
        }
        let mut walked = cell;
        for turn in 1..6 {
            walked = rotation.apply(walked);
            assert_ne!(
                walked, cell,
                "the rotation returned to {cell} after {turn} sixths"
            );
        }
        assert_eq!(
            rotation.apply(walked),
            cell,
            "six sixth-turns is not the identity at {cell}"
        );
        assert_eq!(
            reflection.apply(reflection.apply(cell)),
            cell,
            "the reflection is not an involution at {cell}"
        );
    }
}

#[test]
fn the_identity_moves_nothing() {
    for &cell in &sample_cells() {
        assert_eq!(Symmetry::IDENTITY.apply(cell), cell);
    }
}

#[test]
fn the_origin_alone_is_fixed_by_every_symmetry() {
    let position = vec![(Coord::ORIGIN, Player::P1)];
    for symmetry in Symmetry::ALL {
        assert_eq!(
            transform(&position, symmetry),
            position,
            "{symmetry} moved the origin"
        );
    }
}

#[test]
fn a_symmetry_preserves_whether_a_position_is_won() {
    // The bridge from geometry to rule 2: transforming a board must not create
    // or destroy a run of six. Pinned on a real six-in-a-row along each axis.
    for axis in Axis::ALL {
        let run: Vec<(Coord, Player)> = (0..6i16)
            .map(|step| (Coord::ORIGIN.step(axis, step), Player::P1))
            .collect();
        for symmetry in Symmetry::ALL {
            let image = transform(&run, symmetry);
            let cells: BTreeSet<Coord> = image.iter().map(|&(cell, _)| cell).collect();
            let found = Axis::ALL.into_iter().any(|other| {
                cells.iter().any(|&start| {
                    (0..6).all(|step| {
                        start
                            .checked_step(other, step)
                            .is_some_and(|cell| cells.contains(&cell))
                    })
                })
            });
            assert!(
                found,
                "{symmetry} took a six-run along {axis:?} to something with no six-run"
            );
        }
    }
}

#[test]
fn the_derived_ordering_of_a_symmetry_is_the_order_of_all() {
    // The type's documentation says so, and the guarantee is a field order, so
    // it is the kind of promise a later edit breaks silently.
    let mut sorted = Symmetry::ALL;
    sorted.sort();
    assert_eq!(sorted, Symmetry::ALL);
}

#[test]
fn a_symmetry_of_a_cell_off_the_addressable_lattice_is_refused_not_wrapped() {
    // `checked_apply` answers `Some` exactly within hex distance i16::MAX of the
    // origin. Just inside, every one of the twelve answers.
    for cell in [
        Coord::new(i16::MAX, 0),
        Coord::new(0, i16::MAX),
        Coord::new(16383, 16384),
        Coord::new(i16::MAX, -i16::MAX),
    ] {
        for symmetry in Symmetry::ALL {
            assert!(
                symmetry.checked_apply(cell).is_some(),
                "{symmetry} refused {cell}, which is within reach"
            );
        }
    }
    // Just outside, some of them do not — and the pure transposition never
    // fails, because swapping two i16 is always representable.
    let beyond = Coord::new(16384, 16384);
    assert!(Symmetry::ALL[1].checked_apply(beyond).is_none());
    assert!(Symmetry::ALL[6].checked_apply(beyond).is_some());
    assert!(
        Symmetry::ALL[6]
            .checked_apply(Coord::new(i16::MIN, 0))
            .is_some()
    );
}

#[test]
#[should_panic(expected = "COORD_OVERFLOW")]
fn applying_a_symmetry_past_the_addressable_lattice_panics_by_name() {
    // A named invariant with no test can be downgraded to a wrapping `+` with
    // the suite still green, which is why every other named panic in this crate
    // is pinned this way.
    let _ = Symmetry::ALL[1].apply(Coord::new(16384, 16384));
}

#[test]
#[should_panic(expected = "COORD_OVERFLOW")]
fn a_rotation_that_would_negate_the_least_coordinate_panics_by_name() {
    let _ = Symmetry::ALL[2].apply(Coord::new(i16::MIN, 0));
}
