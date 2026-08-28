mod common;

use common::repo;
use pistol_cli::corpus::bench::{BANDS, Band, PER_BAND};
use pistol_cli::corpus::openings::Candidate;
use pistol_cli::corpus::verdict::Replayed;
use pistol_cli::corpus::{bench, read, record::Record, replay};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(name)
}

/// The synthetic corpus, read and replayed.
fn synthetic() -> (Vec<Record>, Vec<Replayed>) {
    let path = fixture("corpus_synthetic_v1.jsonl");
    let text = std::fs::read_to_string(&path).expect("the synthetic corpus is committed");
    let records = read(&path, &text).expect("it is well formed");
    let replays = records.iter().map(replay::replay).collect();
    (records, replays)
}

/// Candidates over the synthetic corpus, in corpus order.
fn candidates<'a>(records: &'a [Record], replays: &'a [Replayed]) -> Vec<Candidate<'a>> {
    records
        .iter()
        .zip(replays)
        .filter(|(_, replayed)| replayed.verdict.is_eligible())
        .map(|(record, replayed)| Candidate {
            record,
            turns: &replayed.turns,
        })
        .collect()
}

#[test]
fn bench_positions_come_from_distinct_games_in_band() {
    let (records, replays) = synthetic();
    let chosen = bench::select(&candidates(&records, &replays));
    assert!(
        !chosen.is_empty(),
        "the fixture is long enough to fill a band"
    );

    let mut hashes: Vec<&str> = chosen
        .iter()
        .map(|position| position.game_hash.as_str())
        .collect();
    let before = hashes.len();
    hashes.sort_unstable();
    hashes.dedup();
    assert_eq!(
        hashes.len(),
        before,
        "no game contributes twice, within a band or across the two"
    );

    for band in BANDS {
        let in_band: Vec<_> = chosen
            .iter()
            .filter(|position| position.centre == band.centre)
            .collect();
        assert!(
            in_band.len() <= PER_BAND,
            "band {} took more than {PER_BAND}",
            band.centre
        );
        for position in in_band {
            assert!(
                position.stones <= band.centre && position.stones + band.width >= band.centre,
                "band {} produced a {}-stone position",
                band.centre,
                position.stones
            );
            assert!(
                !position.stones.is_multiple_of(2),
                "a turn boundary carries an odd stone count, not {}",
                position.stones
            );
            let state = position
                .position
                .replay()
                .expect("a bench position replays");
            assert_eq!(state.board().stones().count(), position.stones);
        }
    }
}

#[test]
fn a_band_takes_the_largest_odd_count_it_can_reach() {
    // The rule spelled as it behaves: the upper half of a band never fires, and
    // a game that ends early falls back down the band rather than out of it.
    let band = Band {
        centre: 35,
        width: 5,
    };
    assert_eq!(
        band.count_for(100),
        Some(35),
        "a long game reaches the centre"
    );
    assert_eq!(band.count_for(36), Some(35));
    assert_eq!(
        band.count_for(35),
        Some(33),
        "strictly fewer than the game's own stones"
    );
    assert_eq!(band.count_for(34), Some(33));
    assert_eq!(band.count_for(33), Some(31));
    assert_eq!(band.count_for(32), Some(31));
    assert_eq!(
        band.count_for(31),
        None,
        "31 stones can only offer 29, below the band"
    );
    assert_eq!(band.count_for(0), None);
    assert_eq!(Band::turn_for(35), 18, "turn t carries 2t-1 stones");
    assert_eq!(Band::turn_for(15), 8);
}

#[test]
fn both_bench_bands_take_the_full_twelve_when_the_corpus_can_supply_them() {
    assert_eq!(PER_BAND, 12);
    let (records, replays) = synthetic();
    let chosen = bench::select(&candidates(&records, &replays));
    for band in BANDS {
        let taken = chosen
            .iter()
            .filter(|position| position.centre == band.centre)
            .count();
        assert_eq!(
            taken, PER_BAND,
            "band {} took {taken}, not the full {PER_BAND}",
            band.centre
        );
    }
}
