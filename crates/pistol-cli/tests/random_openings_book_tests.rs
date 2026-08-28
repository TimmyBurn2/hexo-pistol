mod common;

use common::repo;
use pistol_cli::random_openings::config::RandomOpeningsConfig;
use pistol_cli::random_openings::error::RandomOpeningsError;
use pistol_cli::random_openings::{self, FILE_NAME};
use pistol_core::{Coord, GameState, Player, canonical_form};
use pistol_engine::PositionSpec;

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
fn random_openings_deterministic_under_seed() {
    // The determinism law (CLAUDE.md rule 4) applied to a generator: the seed
    // and the parameters fix the bytes. The second half is what keeps the first
    // from being vacuous — a generator that ignored its seed would pass the
    // equality and fail the difference.
    let same = config(5, 50, 5, 12345);
    let first = random_openings::generate(&same).expect("generates");
    let second = random_openings::generate(&same).expect("generates again");
    assert_eq!(
        first.tails(),
        second.tails(),
        "the same seed and parameters give the same book"
    );

    let other = random_openings::generate(&config(5, 50, 5, 12346)).expect("generates");
    assert_ne!(
        first.tails(),
        other.tails(),
        "a different seed gives a different book"
    );
}

#[test]
fn random_openings_deduped_under_canonical_key() {
    // The same key WP-1.2a folds openings by (docs/decisions.md D-137), which
    // is also the key the arena's openings READER refuses duplicate openings
    // by. It is deliberately NOT the arena's distinct-GAME key: D-163 keys a
    // game on `canonical_sequence`, because a position's canonical form would
    // merge two different games that reached the same stones. Two openings one
    // reflection apart are ONE opening, and a book holding both would report a
    // distinct-n it does not have.
    let mut seen: Vec<Vec<(Coord, Player)>> = Vec::new();
    for line in committed_lines() {
        seen.push(canonical_form(&stones(&replay(&line))));
    }
    let total = seen.len();
    seen.sort();
    seen.dedup();
    assert_eq!(seen.len(), total, "no two openings are a symmetry apart");

    // And the resample path itself, which the committed parameters never take:
    // a radius-2 ball is small enough that draws collide, so this run exercises
    // the branch that discards a candidate and draws the next one.
    let crowded = random_openings::generate(&config(5, 200, 2, 99)).expect("a crowded book");
    assert!(
        crowded.symmetry_collisions > 0,
        "a radius-2 ball collides, so the resample rule ran: {crowded:?}"
    );
    let mut keys: Vec<Vec<(Coord, Player)>> = crowded
        .openings
        .iter()
        .map(|opening| canonical_form(&stones(&replay(&opening.tail))))
        .collect();
    let produced = keys.len();
    assert_eq!(produced, 200, "the book is the size it was asked for");
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), produced, "and every one of them is distinct");
}

#[test]
fn random_openings_all_roundtrip_through_position_verb() {
    // The round trip the fixture form rests on, driven where an operator meets
    // it: every line goes down the pipe as the `position` verb's tail, the
    // engine takes it in silence, and the position it lands on is the one the
    // line describes.
    let mut engine = common::engine(common::GATE);
    for line in committed_lines() {
        let answers = common::talk(&mut engine, &[&format!("position {line}")]);
        assert!(
            answers.is_empty(),
            "a position that was taken says nothing: {line:?} -> {answers:?}"
        );
        let spec: PositionSpec = line.parse().expect("a payload line parses");
        assert_eq!(
            spec.to_string(),
            line,
            "a line the parser accepts is spelled back the same way"
        );
        assert_eq!(
            pistol_engine::Engine::state(&engine).key(),
            spec.replay().expect("replays").key(),
            "{line:?} lands on the position it names"
        );
    }
}

#[test]
fn random_openings_cannot_hold_a_mate_in_one_turn_at_this_k() {
    // Why there is no balance filter, as arithmetic rather than as a judgement
    // (docs/decisions.md D-175). A turn places two stones, so a mate in one
    // TURN needs four own stones already in a six-window; a mate in one PLY
    // needs five. The largest holding in this book is three, so neither exists
    // in any position in it, and there is nothing for a filter to remove.
    for line in committed_lines() {
        let held = stones(&replay(&line));
        for side in [Player::P1, Player::P2] {
            let count = held.iter().filter(|(_, owner)| *owner == side).count();
            assert!(count <= 3, "{line:?}: {side} holds {count} stones");
        }
    }
}

#[test]
fn random_openings_refuse_a_pool_too_small_for_the_book() {
    // A radius-1 ball holds seven cells, so there are only a few dozen
    // five-stone positions in it and nothing like five hundred symmetry
    // classes. The generator refuses with the measurement in hand rather than
    // returning a shorter book: a book that quietly came up short would be a
    // sample size nobody chose (CLAUDE.md rule 3, rule 6).
    let error =
        random_openings::generate(&config(5, 500, 1, 1)).expect_err("the pool is too small");
    assert!(
        matches!(error, RandomOpeningsError::Exhausted { .. }),
        "an exhausted pool is named, got {error}"
    );
}

#[test]
fn random_openings_refuse_a_stone_the_legal_region_does_not_reach() {
    // The rule-5 check at every placement is not decoration, and this is the
    // reachable case that proves it. `max_radius` is a generation knob with its
    // own typo ceiling, so it CAN be set wider than game rule 5's LEGAL_RADIUS
    // — and a stone drawn out there is beyond eight of the origin and of every
    // other stone, which the rules refuse.
    //
    // It refuses rather than redrawing, and that is the point: a generator that
    // quietly resampled would make `max_radius` mean "somewhere inside this,
    // mostly", and reconciling the two radii by hand is exactly the conflation
    // CLAUDE.md rule 2 forbids. The rules answer, and the tool stops.
    let error = random_openings::generate(&config(5, 10, 40, 3))
        .expect_err("a radius-40 draw lands outside the legal region");
    assert!(
        matches!(error, RandomOpeningsError::OutsideLegalRegion { .. }),
        "rule 5 is named on its own, not folded into a generic placement refusal: {error}"
    );
}
