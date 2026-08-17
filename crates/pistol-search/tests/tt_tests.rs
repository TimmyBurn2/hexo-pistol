//! The transposition table, and the one conversion that makes it safe to share
//! a mate score between two positions at different distances from the root.
//!
//! A mate score is a *distance*, so it means something different at every node.
//! Storing one without converting it is the classic transposition-table bug:
//! the engine announces a mate in three that is really a mate in seven, or
//! prefers a longer win over a shorter one. The conversion is in turns here,
//! which is the unit rule 4 scores sudden death in and the unit both plies of a
//! turn share (docs/decisions.md D-9, D-72).

mod common;

use common::SMALL_TT;
use pistol_core::{Coord, Key128};
use pistol_eval::EVAL_MAX;
use pistol_search::score::{self, mate_in};
use pistol_search::tt::{Bound, Record, Table};

fn key(n: u64) -> Key128 {
    // Two halves that differ, so a table that indexed by the wrong one, or
    // verified with the wrong one, would not accidentally agree.
    Key128::from_parts(
        n.wrapping_mul(0x9E37_79B9_7F4A_7C15),
        n ^ 0xFFFF_0000_FFFF_0000,
    )
}

fn record(score: i32, depth_plies: u32) -> Record {
    Record {
        depth_plies,
        score,
        static_eval: 42,
        bound: Bound::Exact,
        best: Coord::new(1, -2),
    }
}

#[test]
fn tt_mate_score_depth_adjustment_roundtrips() {
    let distances = [0u32, 1, 2, 3, 7, 30];

    // The conversion is invertible for every score the search can hold, at
    // every distance it can hold one at. A node `k` turns from the root can only
    // report a win that completes later than turn `k`, so the mate scores are
    // built from the distance rather than listed against it.
    for turns_from_root in distances {
        let mut scores = vec![0, 17, -350, EVAL_MAX, -EVAL_MAX];
        for further in [1u32, 2, 5] {
            scores.push(mate_in(turns_from_root + further));
            scores.push(-mate_in(turns_from_root + further));
        }
        for score in scores {
            assert_eq!(
                score::from_table(score::to_table(score, turns_from_root), turns_from_root),
                score,
                "score {score} at {turns_from_root} turns from the root did not round trip"
            );
        }
    }

    // A static score is a value, not a distance, so it is not adjusted at all.
    assert_eq!(score::to_table(1234, 5), 1234);
    assert_eq!(score::from_table(-1234, 5), -1234);

    // A mate score is stored as the distance from the node that stored it...
    assert_eq!(score::to_table(mate_in(5), 2), mate_in(3));
    assert_eq!(score::to_table(-mate_in(5), 2), -mate_in(3));
    // ...and read back as the distance from the node that asked.
    assert_eq!(score::from_table(mate_in(3), 4), mate_in(7));
    assert_eq!(score::from_table(-mate_in(3), 4), -mate_in(7));

    // Through the table itself: a win five turns from a node two turns from the
    // root is a win seven turns from a node four turns from the root.
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    table.store(key(1), 2, record(mate_in(5), 6));
    let hit = table.probe(key(1), 4).expect("the entry just stored");
    assert_eq!(
        hit.score,
        mate_in(7),
        "the table did not re-base the distance"
    );
    assert_eq!(hit.depth_plies, 6);
    assert_eq!(hit.best, Coord::new(1, -2));

    // Read back from the distance it was stored at, it is unchanged.
    let hit = table.probe(key(1), 2).expect("the entry just stored");
    assert_eq!(hit.score, mate_in(5));
}

#[test]
fn tt_probe_misses_on_a_key_it_never_stored() {
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    table.store(key(1), 0, record(100, 4));
    assert!(
        table.probe(key(2), 0).is_none(),
        "a key never stored is a miss"
    );
}

#[test]
fn tt_never_takes_more_than_the_bytes_it_was_given() {
    for tt_bytes in [1u64 << 20, 1 << 22, 3 << 20, (1 << 20) + 1] {
        let table = Table::new(tt_bytes).expect("at least one bucket fits");
        assert!(
            table.bytes() <= tt_bytes,
            "a table given {tt_bytes} bytes took {}",
            table.bytes()
        );
        assert!(table.bytes() > tt_bytes / 2, "and it took most of them");
    }
}

#[test]
fn tt_refuses_a_size_that_cannot_hold_a_bucket() {
    let error = Table::new(8).expect_err("eight bytes is not a transposition table");
    assert!(
        error.to_string().contains("search.tt_bytes"),
        "the refusal must name the key an operator edits: {error}"
    );
}

#[test]
fn tt_clear_forgets_everything() {
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    // Enough entries to register in parts per thousand: one entry in a table of
    // tens of thousands is honestly zero permille.
    let entries = table.buckets() as u64 * 4;
    for n in 0..entries / 100 {
        table.store(key(n), 0, record(100, 4));
    }
    assert!(table.hashfull_permille() > 0, "stored entries are in use");
    table.clear();
    assert!(
        table.probe(key(1), 0).is_none(),
        "a cleared table remembers nothing"
    );
    assert_eq!(table.hashfull_permille(), 0);
}

#[test]
fn tt_replacement_prefers_depth_and_is_the_same_every_run() {
    // Five keys that land in one bucket of four, stored deepest first. The
    // shallow arrivals must not evict the deep entry, and two runs of the same
    // sequence must evict exactly the same slot (CLAUDE.md rule 4).
    let build = || {
        let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
        let bucket_stride = table.buckets() as u64;
        for n in 0..5u64 {
            // Same bucket index, different verification words.
            let colliding = Key128::from_parts(bucket_stride * n, 0xABCD_0000 + n);
            table.store(colliding, 0, record(100 + n as i32, 9 - n as u32));
        }
        table
    };
    let first = build();
    let second = build();

    let deepest = Key128::from_parts(0, 0xABCD_0000);
    assert!(
        first.probe(deepest, 0).is_some(),
        "the deepest entry must survive shallower arrivals"
    );
    for n in 0..5u64 {
        let colliding = Key128::from_parts(first.buckets() as u64 * n, 0xABCD_0000 + n);
        assert_eq!(
            first.probe(colliding, 0).map(|hit| hit.score),
            second.probe(colliding, 0).map(|hit| hit.score),
            "two identical store sequences must leave identical tables"
        );
    }
}
