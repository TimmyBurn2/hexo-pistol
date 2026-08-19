//! How far the platform actually let a stone land from the stones already down.
//!
//! The histogram this suite pins exists to answer one question with a number:
//! the HeXO platform's legal radius is 8 (docs/decisions.md D-101), and a
//! second, independent note claims 6 (docs/research/sealbot_deep_dive.md SB-65).
//! WP-1.2a's zero-violations replay (D-149) could never separate the two —
//! radius-6 games are a strict subset of radius-8 legality, so a clean replay
//! under 8 is what BOTH hypotheses predict. Only an observed placement beyond 6
//! distinguishes them, and that is what this measures.
//!
//! # Why the recorded order is the right order
//!
//! Each stone is measured against every stone earlier in the record's own flat
//! `moves` array — not against the position at the start of its turn. That is
//! the conservative direction for a refutation and the reason is worth writing
//! down: if a stone's nearest earlier stone sits at distance `d >= 7`, then its
//! distance to the pre-turn stones is also `>= 7` AND its distance to its own
//! pair partner is `>= 7`, so no ordering of that pair places it within 6 of
//! anything. A `d >= 7` observation therefore refutes radius 6 whichever way the
//! platform ordered the turn (a pair is legal iff SOME ordering is — D-6, D-51).
//! The converse does not hold, which is fine: `d <= 6` was never evidence for
//! either hypothesis.

mod common;

use common::repo;
use pistol_cli::corpus::distance::PlacementDistances;
use pistol_cli::corpus::{read, record::Record};
use pistol_core::Coord;
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(name)
}

/// A committed corpus fixture, read.
fn corpus(name: &str) -> Vec<Record> {
    let path = fixture(name);
    let text = std::fs::read_to_string(&path).expect("the fixture is committed");
    read(&path, &text).expect("it is well formed")
}

/// The measurement again, written the obvious way, for the suite to disagree
/// with. Deliberately not the implementation's shape: a flat scan of every
/// earlier stone, no accumulator, no map.
fn reference(records: &[Record]) -> Vec<(u32, usize)> {
    let mut counts: Vec<(u32, usize)> = Vec::new();
    for record in records {
        for (index, stone) in record.moves.iter().enumerate().skip(1) {
            let nearest = record.moves[..index]
                .iter()
                .map(|earlier| earlier.distance(*stone))
                .min()
                .expect("a stone after the first has an earlier stone");
            match counts.iter_mut().find(|(distance, _)| *distance == nearest) {
                Some((_, count)) => *count += 1,
                None => counts.push((nearest, 1)),
            }
        }
    }
    counts.sort_unstable();
    counts
}

#[test]
fn distance_histogram_matches_hand_computed_fixture() {
    // corpus_distance_v1.jsonl, computed by hand from the axial distance
    // `(|dq| + |dr| + |dq+dr|) / 2`:
    //
    // game d15a…a1, moves (0,0) (1,0) (8,0) (0,1) (4,0)
    //   (1,0): nearest (0,0)            -> 1
    //   (8,0): (0,0)=8, (1,0)=7         -> 7
    //   (0,1): (0,0)=1                  -> 1
    //   (4,0): (0,0)=4, (1,0)=3, (8,0)=4, (0,1)=4
    //                                   -> 3
    // game d15a…b2, moves (0,0) (0,8) (-8,0) (2,2) (1,1)
    //   (0,8):  (0,0)=8                 -> 8
    //   (-8,0): (0,0)=8, (0,8)=16       -> 8
    //   (2,2):  (0,0)=4, (0,8)=6, (-8,0)=12
    //                                   -> 4
    //   (1,1):  (0,0)=2, (0,8)=7, (-8,0)=10, (2,2)=2
    //                                   -> 2
    //
    // so: 1 twice, 2 once, 3 once, 4 once, 7 once, 8 twice — eight stones
    // measured out of ten placed, two games contributing a first stone each.
    // Distances 5 and 6 are absent on purpose: the histogram must show a gap as
    // a gap rather than closing it.
    let records = corpus("corpus_distance_v1.jsonl");
    let measured = PlacementDistances::of(&records);

    assert_eq!(
        measured.counts().collect::<Vec<_>>(),
        vec![(1, 2), (2, 1), (3, 1), (4, 1), (7, 1), (8, 2)],
        "the hand-computed histogram"
    );
    assert_eq!(measured.max(), Some(8), "the hand-computed maximum");
    assert_eq!(measured.total(), 8, "stones measured");
    assert_eq!(measured.count(5), 0, "an unobserved distance counts zero");
    assert_eq!(measured.count(6), 0, "an unobserved distance counts zero");
}

#[test]
fn histogram_deterministic_across_runs() {
    // CLAUDE.md rule 4. Two gathers over the same records must agree on every
    // count AND on the order they are reported in, which is what an unseeded
    // hash map would break: std's `RandomState` gives two maps built in one
    // process different key orders, so this comparison genuinely sees it.
    let records = corpus("corpus_synthetic_v1.jsonl");
    let first = PlacementDistances::of(&records);
    let second = PlacementDistances::of(&records);

    assert_eq!(
        first.counts().collect::<Vec<_>>(),
        second.counts().collect::<Vec<_>>(),
        "same corpus, same histogram in the same order"
    );
    assert_eq!(first.to_string(), second.to_string(), "same rendering");
    assert_eq!(first.max(), second.max());
    assert_eq!(first.total(), second.total());

    let reported: Vec<u32> = first.counts().map(|(distance, _)| distance).collect();
    let mut ascending = reported.clone();
    ascending.sort_unstable();
    assert_eq!(reported, ascending, "distances are reported ascending");
}

#[test]
fn histogram_agrees_with_an_independent_scan_of_the_synthetic_corpus() {
    // The hand-computed case is five stones a game; this is the same claim over
    // a corpus nobody can check by eye, against a second implementation that
    // shares no code with the first.
    let records = corpus("corpus_synthetic_v1.jsonl");
    let measured = PlacementDistances::of(&records);
    let expected = reference(&records);

    assert_eq!(measured.counts().collect::<Vec<_>>(), expected);
    assert_eq!(
        measured.total(),
        expected.iter().map(|(_, count)| count).sum::<usize>()
    );
    assert_eq!(
        measured.max(),
        expected.last().map(|(distance, _)| *distance)
    );
}

#[test]
fn a_lone_first_stone_is_measured_zero_times() {
    // The first stone of a game has nothing to be near, so it is not a
    // measurement of anything — an implementation counting it against the
    // origin would report a distance nobody placed.
    let mut measured = PlacementDistances::new();
    measured.add_game(&[Coord::new(0, 0)]);

    assert_eq!(measured.total(), 0);
    assert_eq!(measured.max(), None);
    assert_eq!(measured.counts().collect::<Vec<_>>(), Vec::new());
}

#[test]
fn nearest_is_the_minimum_over_every_earlier_stone_not_the_previous_one() {
    // The distinction the whole measurement rests on: a stone landing beside an
    // old stone is near, however far the stone before it was.
    let mut measured = PlacementDistances::new();
    measured.add_game(&[
        Coord::new(0, 0),
        Coord::new(8, 0),  // 8 from the origin
        Coord::new(-1, 0), // 1 from the origin, 9 from the stone before it
    ]);

    assert_eq!(measured.counts().collect::<Vec<_>>(), vec![(1, 1), (8, 1)]);
    assert_eq!(measured.max(), Some(8));
}
