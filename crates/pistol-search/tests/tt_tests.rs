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
        from_quiescence: false,
    }
}

fn quiescence_record(score: i32) -> Record {
    Record {
        from_quiescence: true,
        ..record(score, 1)
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

#[test]
fn tt_probe_never_returns_a_quiescence_record() {
    // docs/wp16_quiescence_design.md §6 item 5: a full-width caller treats a
    // quiescence hit exactly as a miss.
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    table.store(key(1), 0, quiescence_record(100));
    assert!(
        table.probe(key(1), 0).is_none(),
        "a quiescence-flagged entry is never returned by probe"
    );
}

#[test]
fn tt_store_fills_an_empty_slot_with_a_quiescence_record() {
    // Even though probe never returns it, the store itself is real: the slot
    // is occupied, not silently dropped. One entry in a table of tens of
    // thousands is honestly zero permille (`tt_clear_forgets_everything`'s
    // own reasoning), so enough distinct keys are stored to register.
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    assert_eq!(table.hashfull_permille(), 0);
    let entries = table.buckets() as u64 * 4;
    for n in 0..entries / 100 {
        table.store(key(n), 0, quiescence_record(100));
    }
    assert!(
        table.hashfull_permille() > 0,
        "quiescence stores into empty slots occupy them"
    );
}

#[test]
fn tt_store_declines_a_quiescence_record_that_would_evict_a_full_width_entry() {
    // docs/wp16_quiescence_design.md §6 item 3: a quiescence store never
    // clobbers existing full-width data, even at the SAME key (where
    // `victim` would otherwise pick that exact slot) and even when a
    // different key's shallower arrival would ordinarily be evicted.
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    table.store(key(1), 0, record(100, 9));
    table.store(key(1), 0, quiescence_record(999));
    let hit = table
        .probe(key(1), 0)
        .expect("the full-width entry must survive");
    assert_eq!(
        hit.score, 100,
        "the quiescence store must have been declined"
    );
    assert_eq!(hit.depth_plies, 9);

    // Same story across a bucket collision: five full-width entries at
    // increasing depth fill a bucket, then a quiescence store at a colliding
    // key must not evict any of them (the ordinary `rank` comparison would
    // otherwise pick the shallowest full-width entry as the victim).
    let bucket_stride = table.buckets() as u64;
    let mut table = Table::new(SMALL_TT).expect("a one mebibyte table");
    for n in 0..4u64 {
        let colliding = Key128::from_parts(bucket_stride * n, 0xBEEF_0000 + n);
        table.store(colliding, 0, record(100 + n as i32, 1 + n as u32));
    }
    let shallowest = Key128::from_parts(0, 0xBEEF_0000);
    let new_key = Key128::from_parts(bucket_stride * 4, 0xBEEF_0004);
    table.store(new_key, 0, quiescence_record(999));
    assert!(
        table.probe(shallowest, 0).is_some(),
        "the shallowest full-width entry survives a quiescence store, unlike an ordinary one"
    );
}

#[test]
fn tt_store_lets_a_quiescence_record_replace_another_quiescence_record() {
    // A quiescence store is only ever declined against full-width data
    // (the test above); against another quiescence entry the ordinary
    // victim rule applies, so storing twice at the same key occupies
    // exactly one slot, not two — checked in bulk so the difference
    // registers in parts per thousand rather than rounding to zero.
    let mut twice = Table::new(SMALL_TT).expect("a one mebibyte table");
    let entries = twice.buckets() as u64 * 4;
    for n in 0..entries / 100 {
        twice.store(key(n), 0, quiescence_record(100));
        twice.store(key(n), 0, quiescence_record(200));
    }
    let mut once = Table::new(SMALL_TT).expect("a one mebibyte table");
    for n in 0..entries / 100 {
        once.store(key(n), 0, quiescence_record(200));
    }
    assert_eq!(
        twice.hashfull_permille(),
        once.hashfull_permille(),
        "a second quiescence store at an already-occupied quiescence key must replace it, not add a slot"
    );
    assert!(once.hashfull_permille() > 0, "the stores must have landed");
}

#[test]
fn table_refuses_a_size_this_machine_cannot_allocate_by_name() {
    // The engine's ceiling catches the typo class offline (`MAX_TT_BYTES`), but
    // how much memory a machine actually has is not a question config validation
    // is allowed to ask (docs/decisions.md D-21), so the constructor answers it.
    // The failure it replaces was `handle_alloc_error`: no name, no key, and a
    // core dump instead of a line (CLAUDE.md rule 3).
    let error = Table::new(1u64 << 60).expect_err("a table this large is refused, never aborted");
    let said = error.to_string();
    assert!(
        said.contains("search.tt_bytes"),
        "the refusal names the key an operator edits: {said}"
    );
}
