//! How far the platform actually let a stone land from the stones already down.
//!
//! The histogram this suite pins exists to answer one question with a number:
//! the HeXO platform's legal radius is 8 (docs/decisions.md D-101), and a
//! second, independent note claims 6 (docs/research/sealbot_deep_dive.md SB-65).
//! WP-1.2a's zero-violations replay (D-149) could never separate the two —
//! radius-6-legal games are a strict subset of radius-8-legal games, so a clean
//! replay under 8 is what BOTH hypotheses predict. Only an observed placement
//! beyond 6 distinguishes them, and that is what this measures.
//!
//! # The distinction this suite exists to keep honest
//!
//! Each stone is measured against every stone earlier in the record's own flat
//! `moves` array — an order-sensitive quantity, where legality is not (a turn is
//! legal iff SOME ordering of its stones is, D-6/D-51). So only part of the
//! measurement can refute anything:
//!
//! - a stone that is the LAST of its own turn has every turn-mate already inside
//!   its minimum, so reordering can only remove stones from the board at the
//!   moment it is placed. Its measured distance is a lower bound over every
//!   ordering — **this** is the class that refutes a radius;
//! - a stone that is not the last of its turn has a partner LATER in the array,
//!   which its minimum never saw, and that partner can bridge to it.
//!
//! # RULE9-JUSTIFICATION: one measurement, one fixture, one reference (CLAUDE.md
//! rule 9).
//!
//! Every test here asserts the same histogram over the same two committed
//! fixtures, through the same three helpers — `corpus`, the hand-computed
//! literals, and `reference`, a second implementation the suite exists to
//! disagree with. Splitting it would either duplicate that reference per file,
//! which defeats the point of having an independent one, or hoist it into a
//! shared module where no single suite owns it. The order-independence tests
//! are not a separate concern either: they are the SAME measurement read in the
//! only column that licenses a conclusion, and separating the sound reading
//! from the raw one across files is precisely the split that let the reviewed
//! revision state a false claim beside a correct number.
//!
//! `an_earlier_stone_in_the_pair_can_be_rescued_by_its_later_partner` pins the
//! second class with the real corpus counterexample that a REVIEW round found
//! after an earlier revision of this suite asserted the whole tail refuted
//! radius 6. That claim was false and the test that would have caught it did not
//! exist; it does now.

mod common;

use common::repo;
use pistol_cli::corpus::distance::PlacementDistances;
use pistol_cli::corpus::stats::Stats;
use pistol_cli::corpus::verdict::Replayed;
use pistol_cli::corpus::{read, record::Record, replay};
use pistol_cli::sha256::sha256_hex;
use pistol_core::Coord;
use std::path::PathBuf;

/// The SHA-256 of `tests/fixtures/corpus_distance_v1.jsonl`, pinned the way its
/// sibling synthetic corpus is: the hand-computed expectations below are only
/// hand-computed for these exact bytes.
const DISTANCE_FIXTURE_SHA256: &str =
    "8ea96b1ff67a38398b87ee47804e80852a60151066ab0e82112aab55ef98aaa2";

fn fixture(name: &str) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(name)
}

/// A committed corpus fixture, read.
fn corpus(name: &str) -> Vec<Record> {
    let path = fixture(name);
    let text = std::fs::read_to_string(&path).expect("the fixture is committed");
    read(&path, &text).expect("it is well formed")
}

/// A distance histogram as the reference builds one: distance, then count.
type Histogram = Vec<(u32, usize)>;

/// The measurement again, written the obvious way, for the suite to disagree
/// with. Deliberately not the implementation's shape: a flat scan of every
/// earlier stone, no accumulator, no map, and the turn structure spelled out
/// rather than walked.
fn reference(records: &[Record]) -> (Histogram, Histogram) {
    let mut all: Histogram = Vec::new();
    let mut order_independent: Histogram = Vec::new();
    let bump = |counts: &mut Histogram, key: u32| match counts
        .iter_mut()
        .find(|(distance, _)| *distance == key)
    {
        Some((_, count)) => *count += 1,
        None => counts.push((key, 1)),
    };
    for record in records {
        let stones = record.moves.len();
        for (index, stone) in record.moves.iter().enumerate().skip(1) {
            let nearest = record.moves[..index]
                .iter()
                .map(|earlier| earlier.distance(*stone))
                .min()
                .expect("a stone after the first has an earlier stone");
            bump(&mut all, nearest);
            // Stone 0 is turn 1's single; pairs run (1,2), (3,4), … so an even
            // index is the second of its pair, and a final odd index is a turn
            // the record cut short. Both are the last stone of their turn.
            if index % 2 == 0 || index + 1 == stones {
                bump(&mut order_independent, nearest);
            }
        }
    }
    all.sort_unstable();
    order_independent.sort_unstable();
    (all, order_independent)
}

#[test]
fn the_distance_fixture_matches_its_pinned_sha256() {
    let bytes = std::fs::read(fixture("corpus_distance_v1.jsonl")).expect("it is committed");
    assert_eq!(sha256_hex(&bytes), DISTANCE_FIXTURE_SHA256);
}

#[test]
fn the_rendered_block_lists_every_distance_in_range_including_the_zero_rows() {
    // The rendered block is what an operator reads and what the ADR's numbers
    // were transcribed from, and a REVIEW round found NOTHING asserted a
    // character of it: mutants that skipped zero rows, collapsed the printed
    // range and inflated the total all survived the whole suite. Parsed by
    // whitespace rather than compared as one string, so column widths stay free
    // to change while the content does not.
    let records = corpus("corpus_distance_v1.jsonl");
    let rendered = PlacementDistances::of(&records).to_string();
    let row = |line: &str| -> Vec<String> { line.split_whitespace().map(str::to_string).collect() };
    let lines: Vec<Vec<String>> = rendered.lines().map(row).collect();

    assert_eq!(
        lines[0],
        ["distance", "count", "of", "which", "order-independent"]
    );
    // One row per distance from the smallest observed to the largest. The 5 and
    // 6 rows are the point: the fixture observes neither, and a listing that
    // skipped them would hide a gap the module doc promises to show.
    assert_eq!(
        lines[1..9],
        [
            ["1", "3", "1"],
            ["2", "1", "1"],
            ["3", "1", "1"],
            ["4", "1", "0"],
            ["5", "0", "0"],
            ["6", "0", "0"],
            ["7", "2", "1"],
            ["8", "2", "1"],
        ]
    );
    assert_eq!(lines[9], ["stones", "measured", "10"]);
    assert_eq!(lines[10], ["order-independent", "5"]);
    // The refuting headline itself, so the number the record quotes is printed
    // rather than hand-summed off the rows above.
    assert_eq!(lines[11][..4], ["order-independent", "beyond", "6", "2"]);
    assert_eq!(lines[12], ["MAX", "distance", "8"]);
    assert_eq!(lines.len(), 13, "no row beyond the ones named above");
}

#[test]
fn every_game_read_is_counted_including_the_ineligible_ones() {
    // D-218's explicit design claim, which a REVIEW round found unbound
    // workspace-wide: counting only eligible games left every test green,
    // because the human corpus happens to have zero exclusions. The question
    // this histogram answers is what the PLATFORM accepted, so a game
    // pistol-core would refuse is still evidence. Every game in this fixture is
    // undecided and therefore ineligible, so an eligibility filter would report
    // an empty histogram.
    let records = corpus("corpus_distance_v1.jsonl");
    let replays: Vec<Replayed> = records.iter().map(replay::replay).collect();
    let stats = Stats::gather("digest-not-under-test".to_string(), &records, &replays);

    assert_eq!(stats.games_read, 3);
    assert_eq!(stats.excluded(), 3, "no game in this fixture is eligible");
    assert_eq!(
        stats.placements.total(),
        10,
        "every stone is still counted, from games none of which are eligible"
    );
    assert_eq!(stats.placements.max(), Some(8));
}

#[test]
fn distance_histogram_matches_hand_computed_fixture() {
    // corpus_distance_v1.jsonl, computed by hand from the axial distance
    // `(|dq| + |dr| + |dq+dr|) / 2`. Turn 1 is one stone; every later turn is
    // two, so the turns are [0], [1,2], [3,4] and the LAST stone of each turn is
    // at an even index (or a final odd index, where the record stops early).
    //
    // game d15a…a1, moves (0,0) (1,0) (8,0) (0,1) (4,0)
    //   (1,0): nearest (0,0)                     -> 1, first of its turn
    //   (8,0): (0,0)=8, (1,0)=7                  -> 7, LAST of its turn
    //   (0,1): (0,0)=1                           -> 1, first of its turn
    //   (4,0): (0,0)=4, (1,0)=3, (8,0)=4, (0,1)=4
    //                                            -> 3, LAST of its turn
    // game d15a…b2, moves (0,0) (0,8) (-8,0) (2,2) (1,1)
    //   (0,8):  (0,0)=8                          -> 8, first of its turn
    //   (-8,0): (0,0)=8, (0,8)=16                -> 8, LAST of its turn
    //   (2,2):  (0,0)=4, (0,8)=6, (-8,0)=12      -> 4, first of its turn
    //   (1,1):  (0,0)=2, (0,8)=7, (-8,0)=10, (2,2)=2
    //                                            -> 2, LAST of its turn
    // game d15a…c3, moves (0,0) (7,-1) (6,-1)
    //   (7,-1): (0,0)=7                          -> 7, first of its turn
    //   (6,-1): (0,0)=6, (7,-1)=1                -> 1, LAST of its turn
    //
    // so all: 1 three times, 2 once, 3 once, 4 once, 7 twice, 8 twice — ten
    // stones measured out of thirteen placed, three games contributing a first
    // stone each. Distances 5 and 6 are absent on purpose: the histogram must
    // show a gap as a gap rather than closing it.
    let records = corpus("corpus_distance_v1.jsonl");
    let measured = PlacementDistances::of(&records);

    assert_eq!(
        measured.counts().collect::<Vec<_>>(),
        vec![(1, 3), (2, 1), (3, 1), (4, 1), (7, 2), (8, 2)],
        "the hand-computed histogram"
    );
    assert_eq!(measured.max(), Some(8), "the hand-computed maximum");
    assert_eq!(measured.total(), 10, "stones measured");
    assert_eq!(measured.count(5), 0, "an unobserved distance counts zero");
    assert_eq!(measured.count(6), 0, "an unobserved distance counts zero");

    // Only the last stone of each turn: one 7 (game a1), one 8 (game b2), plus
    // the 3, the 2 and game c3's 1.
    assert_eq!(
        measured.order_independent_counts().collect::<Vec<_>>(),
        vec![(1, 1), (2, 1), (3, 1), (7, 1), (8, 1)],
        "the hand-computed order-independent histogram"
    );
    assert_eq!(measured.order_independent_total(), 5);
}

#[test]
fn only_order_independent_far_stones_refute_a_radius() {
    // The claim the whole adjudication rests on, and the arithmetic that makes
    // it a smaller number than the raw tail: of the four stones beyond distance
    // 6 in the fixture, only two are the last stone of their turn.
    let records = corpus("corpus_distance_v1.jsonl");
    let measured = PlacementDistances::of(&records);

    let raw_tail = measured.count(7) + measured.count(8);
    assert_eq!(raw_tail, 4, "stones beyond 6 by the raw measurement");
    assert_eq!(
        measured.unrescuable_beyond(6),
        2,
        "of which only the order-independent ones refute radius 6"
    );
    assert!(
        measured.unrescuable_beyond(6) < raw_tail,
        "the refuting count must be the SMALLER one: an implementation returning \
         the raw tail would overclaim exactly the way the reviewed revision did"
    );
    // Nothing in the fixture is beyond 8, so radius 8 is not refuted by it.
    assert_eq!(measured.unrescuable_beyond(8), 0);
    assert_eq!(measured.unrescuable_beyond(7), 1);
}

#[test]
fn an_earlier_stone_in_the_pair_can_be_rescued_by_its_later_partner() {
    // The REVIEW finding, pinned. Shape taken from real corpus game
    // dff648bcbc1833d0 index 1: a stone measuring distance 7 from everything
    // before it, whose own partner comes LATER in the record and sits 6 from the
    // board and 1 from it. Under radius 6 the order (partner, stone) is legal,
    // so this stone refutes nothing — and an implementation counting it would
    // claim a refutation the game does not support.
    let mut measured = PlacementDistances::new();
    measured.add_game(&[
        Coord::new(0, 0),
        Coord::new(7, -1), // 7 from the board, but first of its turn
        Coord::new(6, -1), // 6 from the board, 1 from its partner
    ]);

    assert_eq!(measured.count(7), 1, "the far stone is measured");
    assert_eq!(
        measured.order_independent_count(7),
        0,
        "but it is not order-independent, so it proves nothing"
    );
    assert_eq!(
        measured.unrescuable_beyond(6),
        0,
        "this game does not refute radius 6"
    );
    // The partner is the last stone of the turn and is measured against the far
    // stone, which is why it reads 1 rather than 6.
    assert_eq!(measured.order_independent_count(1), 1);
}

#[test]
fn a_turn_the_record_cut_short_leaves_its_stone_order_independent() {
    // Rule 4's truncation: the winning stone is a turn's first, and the second
    // is never played. A stone with no turn-mate at all has nothing that could
    // bridge it, so its measured distance is exact rather than an upper bound.
    let mut measured = PlacementDistances::new();
    measured.add_game(&[
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(10, 0), // turn 3's first and only stone
    ]);

    assert_eq!(measured.count(8), 1, "8 from (2,0), the nearest stone");
    assert_eq!(
        measured.order_independent_count(8),
        1,
        "no partner exists, so no reordering can rescue it"
    );
    assert_eq!(measured.unrescuable_beyond(6), 1);
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
    assert_eq!(
        first.order_independent_counts().collect::<Vec<_>>(),
        second.order_independent_counts().collect::<Vec<_>>(),
    );
    assert_eq!(first.to_string(), second.to_string(), "same rendering");
    assert_eq!(first.max(), second.max());
    assert_eq!(first.total(), second.total());

    for reported in [
        first
            .counts()
            .map(|(distance, _)| distance)
            .collect::<Vec<_>>(),
        first
            .order_independent_counts()
            .map(|(distance, _)| distance)
            .collect::<Vec<_>>(),
    ] {
        let mut ascending = reported.clone();
        ascending.sort_unstable();
        assert_eq!(reported, ascending, "distances are reported ascending");
    }
}

#[test]
fn histogram_agrees_with_an_independent_scan_of_the_synthetic_corpus() {
    // The hand-computed case is a handful of stones a game; this is the same
    // claim over a corpus nobody can check by eye, against a second
    // implementation that shares no code with the first.
    let records = corpus("corpus_synthetic_v1.jsonl");
    let measured = PlacementDistances::of(&records);
    let (all, order_independent) = reference(&records);

    assert_eq!(measured.counts().collect::<Vec<_>>(), all);
    assert_eq!(
        measured.order_independent_counts().collect::<Vec<_>>(),
        order_independent
    );
    assert_eq!(
        measured.total(),
        all.iter().map(|(_, count)| count).sum::<usize>()
    );
    assert_eq!(measured.max(), all.last().map(|(distance, _)| *distance));
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
    assert_eq!(
        measured.to_string(),
        "  no stone was measured",
        "the empty rendering must not claim anything about game lengths"
    );
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
