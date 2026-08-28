mod common;

use common::repo;
use pistol_cli::random_openings::config::RandomOpeningsConfig;
use pistol_cli::random_openings::{self, FILE_NAME};
use pistol_core::{Coord, GameState, Player, Turn};
use pistol_engine::PositionSpec;

/// The committed config: the parameters `random_openings_v1.txt` was made with.
fn committed_config() -> RandomOpeningsConfig {
    RandomOpeningsConfig::load(&repo("configs/random_openings_v1.toml"))
        .expect("the committed config loads")
}

/// A document with the committed shape and these three fields changed.
fn config(k_stones: usize, n_openings: usize, max_radius: u32, seed: u64) -> RandomOpeningsConfig {
    RandomOpeningsConfig::parse(&format!(
        "schema_version = 1\n[generate]\nk_stones = {k_stones}\nn_openings = {n_openings}\n\
         max_radius = {max_radius}\nseed = {seed}\n"
    ))
    .expect("a well-formed document")
}

/// The payload lines of the committed book.
fn committed_lines() -> Vec<String> {
    let text = std::fs::read_to_string(repo("crates/pistol-cli/tests/fixtures").join(FILE_NAME))
        .expect("the book is committed");
    pistol_cli::corpus::emit::body_of(&text)
        .expect("a rendered fixture has a body")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// The position a payload line names, replayed through the rules.
fn replay(line: &str) -> GameState {
    let spec: PositionSpec = line
        .parse()
        .unwrap_or_else(|error| panic!("{line:?} does not parse: {error}"));
    spec.replay()
        .unwrap_or_else(|error| panic!("{line:?} does not replay: {error}"))
}

/// The stones of a position, in play order.
fn stones(state: &GameState) -> Vec<(Coord, Player)> {
    state.played().collect()
}

#[test]
fn random_openings_stone_counts_follow_turn_structure() {
    // Rule 3 is the whole shape of a generated opening: one stone on turn 1,
    // two on every turn after. At k = 5 that is turn 1 P1, turn 2 P2, turn 3 P1
    // — so P1 holds three stones and P2 two, the position sits at a turn
    // boundary, and it is P2 to move.
    let config = committed_config();
    assert_eq!(
        config.generate.k_stones, 5,
        "this test reads the k = 5 book"
    );
    for line in committed_lines() {
        let state = replay(&line);
        let held = stones(&state);
        assert_eq!(held.len(), 5, "{line:?} holds k stones");
        assert_eq!(
            held.iter().filter(|(_, side)| *side == Player::P1).count(),
            3,
            "{line:?}: P1 played turn 1 and turn 3"
        );
        assert_eq!(
            held.iter().filter(|(_, side)| *side == Player::P2).count(),
            2,
            "{line:?}: P2 played turn 2"
        );
        assert_eq!(state.turn(), 4, "{line:?} is three turns in");
        assert_eq!(state.phase(), pistol_core::Phase::First, "{line:?}");
        assert_eq!(state.to_move(), Player::P2, "{line:?}");
    }
}

#[test]
fn random_openings_at_three_stones_stop_after_the_reply() {
    // The other supported boundary, generated rather than committed: k = 3 is
    // P1's opening stone and P2's whole reply, and nothing else.
    let book = random_openings::generate(&config(3, 40, 5, 7)).expect("a k = 3 book generates");
    for opening in &book.openings {
        let state = replay(&opening.tail);
        let held = stones(&state);
        assert_eq!(held.len(), 3, "{:?}", opening.tail);
        assert_eq!(state.turn(), 3, "{:?} is two turns in", opening.tail);
        assert_eq!(state.to_move(), Player::P1, "{:?}", opening.tail);
    }
}

#[test]
fn random_openings_first_stone_is_origin() {
    // Rule 3 puts turn 1 at the origin without loss of generality, and the
    // generator does not sample it: it is placed. pistol-core would refuse any
    // other first stone, so what this pins is that the tool never asks it to —
    // over freshly generated openings as well as the committed ones, since the
    // committed bytes are already held by their own digest and a fixture-only
    // assertion would be pinning the file rather than the generator.
    let fresh = random_openings::generate(&config(5, 30, 5, 808)).expect("generates");
    let lines: Vec<String> = committed_lines()
        .into_iter()
        .chain(fresh.tails().into_iter().map(str::to_string))
        .collect();
    for line in lines {
        let spec: PositionSpec = line.parse().expect("a payload line parses");
        let PositionSpec::Start { moves } = &spec else {
            panic!("{line:?} is a move list");
        };
        assert_eq!(
            moves.first().copied(),
            Some(Turn::Single(Coord::ORIGIN)),
            "{line:?} opens on the origin"
        );
    }
}

#[test]
fn random_openings_respect_generation_radius() {
    // `max_radius` is a GENERATION knob and is not rule 5's LEGAL_RADIUS
    // (docs/decisions.md D-177): it bounds the distance from the ORIGIN, where
    // rule 5 bounds the distance from the nearest stone.
    //
    // Asserted at THREE radii, generated here rather than only read off the
    // committed file, because one radius cannot tell a generator that honours
    // the knob from one that samples some fixed disc of its own. Both halves
    // matter at each: every stone inside the ball, and the ball's own edge
    // reached — the first alone would pass for a generator that quietly sampled
    // smaller, and the second alone for one that sampled wider.
    let check = |radius: u32, tails: Vec<String>| {
        let mut reached = false;
        for tail in &tails {
            for (cell, _) in stones(&replay(tail)) {
                let distance = Coord::ORIGIN.distance(cell);
                assert!(
                    distance <= radius,
                    "{tail:?}: {cell} is {distance} from the origin, past max_radius {radius}"
                );
                reached |= distance == radius;
            }
        }
        assert!(reached, "some stone sits on the max_radius {radius} ring");
    };

    check(committed_config().generate.max_radius, committed_lines());
    for radius in [3, 8] {
        let book = random_openings::generate(&config(5, 60, radius, 5))
            .unwrap_or_else(|error| panic!("a radius-{radius} book generates: {error}"));
        check(
            radius,
            book.tails().into_iter().map(str::to_string).collect(),
        );
    }
}

#[test]
fn a_shorter_book_is_a_prefix_of_the_committed_one() {
    // What makes raising `n_openings` in place an EXTENSION of the sample rather
    // than a replacement of it, checked rather than argued. D-177 emits in
    // generation order precisely so that a prefix of the file is already a
    // sample, and D-187 leaned on that to grow the book from 500 to 2000 by
    // editing the committed config instead of writing a second one: if this
    // property did not hold, that edit would have silently discarded the 500
    // openings every earlier reading of the book was about.
    //
    // The seed, the stone count and the radius are the committed document's, so
    // the only difference between the two runs is how many candidates were asked
    // for. Drawing fewer must not change the ones drawn.
    let committed = committed_config();
    let shorter = config(
        committed.generate.k_stones,
        500,
        committed.generate.max_radius,
        committed.generate.seed,
    );
    let book = random_openings::generate(&shorter).expect("a 500-opening book generates");
    let lines = committed_lines();
    assert!(
        lines.len() > book.openings.len(),
        "the committed book is the longer of the two, or this test compares it with itself"
    );
    for (index, opening) in book.openings.iter().enumerate() {
        assert_eq!(
            opening.tail, lines[index],
            "opening {index} of a shorter run differs from the committed book's"
        );
    }
}
