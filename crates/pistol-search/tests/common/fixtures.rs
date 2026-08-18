//! The oracle's positions: where they come from, and why they are these.
//!
//! Four sources, and the point of having four is that they fail differently.
//!
//! - **tactical-v0**, read in place from pistol-cli's sha-pinned fixture. Twenty
//!   positions somebody reasoned about: mates in one, must-blocks, mates in
//!   three, quiet.
//! - **the perft oracle's positions**, read in place from pistol-core's
//!   sha-pinned fixture. Openings and clouds chosen to stress the *legal region*
//!   — a lone stone, a tight cluster, two lobes joined by a bridge, a win the
//!   mover can take, and one already-decided position that both the search and
//!   the reference must refuse.
//! - **seeded playouts**: positions nobody designed ([`super::playouts`]).
//! - **two built positions** that carry the mate distances this suite could not
//!   otherwise reach, [`COMPACT_MATE_IN_3`] and the position one turn inside it.
//!
//! # Why the SHAs are not re-pinned here
//!
//! A fixture is pinned by SHA where it carries GOLDEN VALUES, so that editing
//! the file without editing the expectation is a red test (docs/decisions.md
//! D-37). This suite carries no golden values: its expectation is *agreement
//! between two implementations*, which is invariant under any edit to the
//! position set. A third std-only SHA-256 in the workspace would pin a file
//! whose contents cannot change this suite's verdict. Drift is still caught, by
//! the owning crates' own pins, in the same `cargo test --workspace` run.
//!
//! The coupling this does create is a cross-crate read: moving either fixture
//! file breaks a test in a crate that does not own it, with no compile-time
//! signal. That is stated here and in docs/decisions.md D-122 rather than left
//! to be discovered.

use std::fs;
use std::path::PathBuf;

use pistol_core::{Coord, GameState, Player};

use super::fixture_text::{parse_perft, parse_tactical};
use super::{playouts, position};

/// A position the oracle runs, and the name a failure reports.
pub struct Fixture {
    /// The case name, as its source file spells it.
    pub name: String,
    /// The position, replayed through the rules on the way in.
    pub state: GameState,
}

/// P1's stones in the built mate-in-3: a three on `ConstR` and a three on
/// `ConstQ` sharing the origin.
pub const MATE_IN_3_P1: [Coord; 5] = [
    Coord::new(0, 0),
    Coord::new(1, 0),
    Coord::new(2, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
];

/// P2's stones in the built mate-in-3, packed against P1's cluster and touching
/// neither winning line.
pub const MATE_IN_3_P2: [Coord; 6] = [
    Coord::new(-1, 1),
    Coord::new(-1, 2),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(2, 1),
    Coord::new(2, 2),
];

/// The winner's stones in the built mated-in-2: the double four itself, an L of
/// seven cells sharing the origin.
pub const MATED_IN_2_WINNER: [Coord; 7] = [
    Coord::new(0, 0),
    Coord::new(1, 0),
    Coord::new(2, 0),
    Coord::new(3, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(0, 3),
];

/// The mover's stones in the built mated-in-2, packed into the L's corner so
/// that they touch neither four and add as few candidates as possible.
pub const MATED_IN_2_MOVER: [Coord; 6] = [
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(1, 3),
    Coord::new(2, 1),
    Coord::new(2, 2),
    Coord::new(3, 1),
];

/// Everything the oracle may search: every fixture that is ongoing and at a
/// turn boundary.
pub fn searchable() -> Vec<Fixture> {
    all()
        .into_iter()
        .filter(|fixture| !fixture.state.outcome().is_decided())
        .collect()
}

/// Every fixture, decided ones included.
pub fn all() -> Vec<Fixture> {
    let mut fixtures = tactical_v0();
    fixtures.extend(perft_midgame());
    fixtures.extend(playouts());
    fixtures.extend(built());
    fixtures.extend(opening());
    fixtures
}

/// The position before anything has been played.
///
/// It is here for one branch and it is the only position that reaches it: turn 1
/// owes ONE stone (rule 3), so it is where `search.rs`'s `plies_for` stops being
/// twice the depth. That sum is the whole of the engine's depth arithmetic, and
/// the reference deliberately does not share it — it reads the turn structure off
/// `PlyOutcome` instead — so the two agreeing here is the only differential check
/// of rule 3's asymmetry there is. Every other fixture starts at turn 2 or later.
///
/// It is also nearly free: the policy offers one cell on an empty board however
/// wide its radius (docs/decisions.md D-77), so the tree stays small enough to
/// carry all three depths.
pub fn opening() -> Vec<Fixture> {
    vec![Fixture {
        name: "empty_board_turn_1".to_string(),
        state: GameState::new_game(),
    }]
}

/// The twenty sha-pinned tactical positions.
pub fn tactical_v0() -> Vec<Fixture> {
    let path = workspace().join("crates/pistol-cli/tests/fixtures/tactical_v0.txt");
    named_cases(parse_tactical(&read(&path), &path.display().to_string()))
}

/// The perft oracle's five positions, the decided one included: that both the
/// search and the reference refuse it is itself an agreement worth having.
pub fn perft_midgame() -> Vec<Fixture> {
    let path = workspace().join("crates/pistol-core/tests/fixtures/perft_positions_v1.txt");
    named_cases(parse_perft(&read(&path), &path.display().to_string()))
}

/// Four positions nobody designed. The turn counts are small and stated: a
/// full-width reference pays the candidate count squared per turn, so a
/// playout's size is a runtime decision, not an arbitrary one.
pub fn playouts() -> Vec<Fixture> {
    [
        // ODD seeds only. `Rng::new` nudges a zero state by OR-ing the low bit,
        // so an even seed aliases onto the odd one above it: 0x5EED_0002 and
        // 0x5EED_0003 were one generator and produced one position under two
        // names, which the distinctness check in
        // `oracle_fixtures_are_positions_the_search_can_be_asked_about` now
        // refuses. The nudge is three crates' shared convention and is not this
        // suite's to change.
        (0x5EED_0001, 3),
        (0x5EED_0003, 4),
        (0x5EED_0005, 4),
        (0x5EED_0007, 5),
    ]
    .into_iter()
    .map(|(seed, turns): (u64, u32)| Fixture {
        name: format!("playout_seed_{seed:x}_turns_{turns}"),
        state: playouts::playout(seed, turns),
    })
    .collect()
}

/// The two positions built for the mate distances the borrowed fixtures cannot
/// afford.
///
/// Both are as small as their value allows, because a full-width reference pays
/// the candidate count to the power of twice the depth. `compact_mate_in_3` is
/// eleven stones, the fewest a forced mate in three can have: the mover needs a
/// double four after its turn, which needs seven own stones, so five before it —
/// and P1's stone count is always odd, which fixes P2's at six. Fifteen
/// candidates at radius 1 is the packing floor for eleven stones.
/// `compact_mated_in_2` is the same idea with the double four already made and
/// its owner NOT to move: thirteen stones, sixteen candidates, and the only
/// position in the suite whose value is an EVEN mate distance — which is what
/// D-72 says a loss always is, and the one place a sign error in the negamax
/// flip or in the table's re-basing would show.
///
/// Both were checked at radius 3 as well as radius 1, so each distance is a
/// game fact rather than an artefact of a narrow policy — the same check the
/// tactical fixture's header records for its own mate-in-3 cases.
pub fn built() -> Vec<Fixture> {
    vec![
        Fixture {
            name: "compact_mate_in_3".to_string(),
            state: position(&MATE_IN_3_P1, &MATE_IN_3_P2, Player::P1),
        },
        Fixture {
            name: "compact_mated_in_2".to_string(),
            state: position(&MATED_IN_2_WINNER, &MATED_IN_2_MOVER, Player::P2),
        },
    ]
}

/// A fixture by name, for the tests that are about one position.
pub fn named(name: &str) -> Fixture {
    all()
        .into_iter()
        .find(|fixture| fixture.name == name)
        .unwrap_or_else(|| panic!("no oracle fixture named {name}"))
}

fn named_cases(cases: Vec<(String, GameState)>) -> Vec<Fixture> {
    cases
        .into_iter()
        .map(|(name, state)| Fixture { name, state })
        .collect()
}

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &PathBuf) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("the oracle reads {}: {error}", path.display()))
}
