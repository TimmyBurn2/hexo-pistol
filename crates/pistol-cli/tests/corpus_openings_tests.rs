//! Choosing the openings: the rating band, the canonical dedupe, the
//! representative rule, the emission order, and the balance evidence.
//!
//! Each rule is exercised against the committed synthetic corpus, which is built
//! to make it bite — a mirrored pair for the dedupe, an unrated game for the
//! rated clause, a mismatched pair for the ceiling, and classes more than one
//! game reaches so the balance counts have something to count.
//!
//! Every assertion here pins a CONSEQUENCE rather than a constant against
//! itself: an earlier round of these tests compared `elo_gap() <= ELO_GAP_CEILING`,
//! which holds whatever the constant holds, and the mutation that widened the
//! ceiling to its maximum left the suite green (docs/decisions.md D-152).

mod common;

use common::repo;
use pistol_cli::corpus::openings::{Candidate, ELO_GAP_CEILING, K_TURNS, OPENING_STONES};
use pistol_cli::corpus::verdict::Replayed;
use pistol_cli::corpus::{openings, read, record::Record, replay};
use pistol_core::{Coord, Player, canonical_form};
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
fn openings_dedupe_by_position_key() {
    // The synthetic corpus carries a mirrored pair: one game and the reflection
    // of the whole game, so their openings are the same shape reflected. Both
    // sides of the pair carry the same ratings, so the floor keeps or drops them
    // together and the only thing that can collapse them is the canonical form.
    let (records, replays) = synthetic();
    let candidates = candidates(&records, &replays);
    let selection = openings::select(&candidates).expect("the fixture has candidates");

    assert!(
        selection.openings.len() < selection.distinct_positions,
        "the mirrored pair must collapse: {} canonical from {} distinct",
        selection.openings.len(),
        selection.distinct_positions
    );

    // And no two emitted openings are images of one another.
    let mut canonicals: Vec<Vec<(Coord, Player)>> = selection
        .openings
        .iter()
        .map(|opening| {
            let state = opening
                .position
                .replay()
                .expect("an emitted opening replays");
            let mut stones: Vec<(Coord, Player)> = state.board().stones().collect();
            stones.sort_unstable();
            canonical_form(&stones)
        })
        .collect();
    let before = canonicals.len();
    canonicals.sort();
    canonicals.dedup();
    assert_eq!(
        canonicals.len(),
        before,
        "two emitted openings share a canonical form, so one of them is the other reflected"
    );
}

#[test]
fn openings_selection_is_deterministic_under_priority_rule() {
    let (records, replays) = synthetic();
    let first = openings::select(&candidates(&records, &replays)).expect("candidates");

    // The same corpus read in a different order must give the same answer: the
    // rules are a total order, so nothing depends on which game arrived first.
    let mut shuffled: Vec<usize> = (0..records.len()).collect();
    shuffled.reverse();
    let reordered: Vec<Candidate<'_>> = shuffled
        .iter()
        .filter(|&&index| replays[index].verdict.is_eligible())
        .map(|&index| Candidate {
            record: &records[index],
            turns: &replays[index].turns,
        })
        .collect();
    let second = openings::select(&reordered).expect("candidates");

    assert_eq!(first.openings, second.openings);
    assert_eq!(first.floor, second.floor);
    assert_eq!(first.candidates, second.candidates);
}

#[test]
fn openings_come_out_in_game_hash_order_so_a_prefix_is_a_sample() {
    // Not in rating order. There is no cap, so a runner takes a prefix, and a
    // prefix of a rating-sorted pool is the extreme tail (docs/decisions.md D-143).
    let (records, replays) = synthetic();
    let selection = openings::select(&candidates(&records, &replays)).expect("candidates");
    let hashes: Vec<&str> = selection
        .openings
        .iter()
        .map(|opening| opening.game_hash.as_str())
        .collect();
    let mut sorted = hashes.clone();
    sorted.sort_unstable();
    assert_eq!(hashes, sorted, "openings are emitted in game-hash order");
}

#[test]
fn openings_respect_the_rating_band_and_the_computed_floor() {
    let (records, replays) = synthetic();
    let selection = openings::select(&candidates(&records, &replays)).expect("candidates");
    for opening in &selection.openings {
        let record = records
            .iter()
            .find(|record| record.game_hash == opening.game_hash)
            .expect("every opening names a game in the corpus");
        assert!(
            record.elo_gap().expect("a rated game") <= ELO_GAP_CEILING,
            "game {} has a rating gap over the ceiling",
            record.game_hash
        );
        assert!(
            record.min_elo().expect("a rated game") >= selection.floor,
            "game {} is below the computed floor",
            record.game_hash
        );
    }
    assert!(
        records.iter().any(|record| record.min_elo().is_none()),
        "the fixture must carry an unrated game, so the rated clause has something to exclude"
    );
    assert!(
        !selection.openings.iter().any(|opening| {
            records
                .iter()
                .find(|record| record.game_hash == opening.game_hash)
                .is_some_and(|record| record.min_elo().is_none())
        }),
        "an unrated game must not reach the openings"
    );
}

#[test]
fn the_rating_floor_is_the_lower_median_of_the_candidates() {
    // Pinned as arithmetic, not as a number: the floor travels to another corpus
    // and a hard-coded rating would not.
    let mut odd = vec![10u16, 30, 20];
    assert_eq!(openings::lower_median(&mut odd), Some(20));
    let mut even = vec![40u16, 10, 30, 20];
    assert_eq!(
        openings::lower_median(&mut even),
        Some(20),
        "the LOWER median: index (n-1)/2, with no averaging"
    );
}

#[test]
fn every_opening_is_seven_stones_and_four_turns() {
    let (records, replays) = synthetic();
    for opening in openings::select(&candidates(&records, &replays))
        .expect("candidates")
        .openings
    {
        let state = opening.position.replay().expect("an opening replays");
        assert_eq!(state.board().stones().count(), OPENING_STONES);
        assert_eq!(state.turn(), K_TURNS as u32 + 1);
    }
}

#[test]
fn the_rating_ceiling_excludes_a_mismatched_game() {
    // The band's value is pinned as a literal, and the consequence is asserted
    // rather than the constant: an earlier version of this test compared
    // `elo_gap() <= ELO_GAP_CEILING`, which is true whatever the constant holds,
    // so widening the ceiling to u16::MAX left the suite green.
    assert_eq!(ELO_GAP_CEILING, 100);
    let (records, replays) = synthetic();
    let mismatched = records
        .iter()
        .find(|record| record.elo_gap().is_some_and(|gap| gap > ELO_GAP_CEILING))
        .expect("the fixture carries a game whose players were far apart");
    let selection = openings::select(&candidates(&records, &replays)).expect("candidates exist");
    assert!(
        !selection
            .openings
            .iter()
            .any(|opening| opening.game_hash == mismatched.game_hash),
        "game {} has a rating gap of {:?} and must not reach the openings",
        mismatched.game_hash,
        mismatched.elo_gap()
    );
    assert!(
        !selection.openings.is_empty(),
        "and the ceiling must not have excluded everything"
    );
}

#[test]
fn a_corpus_with_no_candidate_games_is_refused_by_name() {
    // Reachable from a schema-valid corpus, so it is a refusal and not a broken
    // invariant: an unrated corpus, or one whose players are all mismatched,
    // used to abort the process and throw away the stats block with it.
    let (records, replays) = synthetic();
    let none: Vec<Candidate<'_>> = Vec::new();
    assert_eq!(
        openings::select(&none).err(),
        Some(openings::NO_CANDIDATE_GAMES),
        "an empty candidate set is a named refusal"
    );
    // And the ordinary corpus still selects, so the guard is not just always on.
    assert!(openings::select(&candidates(&records, &replays)).is_ok());
}
