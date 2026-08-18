//! Who speaks for a canonical opening class, and what the corpus says about how
//! that opening turned out.
//!
//! Both are counted over the class rather than the game, and both are re-derived
//! here from the records rather than read back from the selector — an inverted
//! winner test would otherwise flip every `p1` column with nothing noticing, and
//! a representative rule inverted from min-elo-descending to ascending changed
//! the fixture while the suite stayed green (docs/decisions.md D-152).

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
fn a_class_is_represented_by_its_highest_rated_game() {
    // D-143's priority rule keeps exactly one job once the cap is gone: which
    // game speaks for a canonical class. Nothing tested it, and inverting the
    // comparison left the suite green while the fixture changed.
    let (records, replays) = synthetic();
    let candidates = candidates(&records, &replays);
    let selection = openings::select(&candidates).expect("candidates exist");

    for opening in &selection.openings {
        let canonical = canonical_of(opening);
        // Every eligible game reaching this same canonical opening.
        let mut members: Vec<(u16, &str)> = candidates
            .iter()
            .filter(|candidate| {
                candidate
                    .record
                    .min_elo()
                    .is_some_and(|elo| elo >= selection.floor)
                    && candidate
                        .record
                        .elo_gap()
                        .is_some_and(|gap| gap <= ELO_GAP_CEILING)
                    && candidate.record.moves.len() > OPENING_STONES
                    && canonical_at_k(candidate) == canonical
            })
            .map(|candidate| {
                (
                    candidate.record.min_elo().expect("rated"),
                    candidate.record.game_hash.as_str(),
                )
            })
            .collect();
        assert!(!members.is_empty());
        // Highest rating first, then the lower hash — the same total order the
        // selector claims to use.
        members.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
        assert_eq!(
            opening.game_hash, members[0].1,
            "the class was represented by {} (elo {}) instead of {} (elo {})",
            opening.game_hash, opening.min_elo, members[0].1, members[0].0
        );
    }
}

#[test]
fn the_balance_evidence_counts_first_player_wins_over_every_eligible_game() {
    // D-145's charter clause, and the column most able to be silently wrong:
    // inverting the winner test flips every `p1` count with no test noticing,
    // and narrowing the count to the rated band shrinks them all.
    let (records, replays) = synthetic();
    let candidates = candidates(&records, &replays);
    let selection = openings::select(&candidates).expect("candidates exist");

    assert!(
        selection
            .openings
            .iter()
            .any(|opening| opening.class_games > 1),
        "the fixture must hold a class more than one game reaches, or the counting is untested"
    );

    for opening in &selection.openings {
        // Counted over ALL eligible-replay games, not only the rated band: the
        // count is a fact about the opening, and narrowing it would throw away
        // evidence without making it truer.
        let canonical = canonical_of(opening);
        let members: Vec<&Candidate<'_>> = candidates
            .iter()
            .filter(|candidate| {
                candidate.record.moves.len() > OPENING_STONES
                    && canonical_at_k(candidate) == canonical
            })
            .collect();
        // Both numbers are re-derived here from the records rather than from the
        // selector, so an inverted winner test or a narrowed count fails.
        let expected_p1 = members
            .iter()
            .filter(|candidate| candidate.record.winner == Player::P1)
            .count();
        assert_eq!(
            opening.class_games,
            members.len(),
            "opening from {} miscounts its class",
            opening.game_hash
        );
        assert_eq!(
            opening.class_p1_wins, expected_p1,
            "opening from {} miscounts the first player's wins in its class",
            opening.game_hash
        );
        assert!(opening.class_games > 0);
    }
    assert!(
        selection
            .openings
            .iter()
            .any(|opening| opening.class_p1_wins < opening.class_games),
        "the fixture must contain an opening the second player sometimes won, or an inverted \
         p1 column would be indistinguishable from a correct one"
    );
}
/// The canonical form of an emitted opening's position.
fn canonical_of(opening: &openings::Opening) -> Vec<(Coord, Player)> {
    let state = opening.position.replay().expect("an opening replays");
    let mut stones: Vec<(Coord, Player)> = state.board().stones().collect();
    stones.sort_unstable();
    canonical_form(&stones)
}

/// The canonical form of a candidate's own K-turn opening.
fn canonical_at_k(candidate: &Candidate<'_>) -> Vec<(Coord, Player)> {
    let state = replay::position_after(candidate.turns, K_TURNS);
    let mut stones: Vec<(Coord, Player)> = state.board().stones().collect();
    stones.sort_unstable();
    canonical_form(&stones)
}
