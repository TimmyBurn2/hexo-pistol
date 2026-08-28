mod common;

use common::assert_pinned;
use common::games::{GOLDEN_GAMES_FILE, GameVerdict, golden_games, parse_games};
use pistol_core::{CoreError, GameState, Outcome, Phase, PlyOutcome};

/// The SHA-256 of `tests/fixtures/golden_games_v1.txt`.
const GOLDEN_GAMES_SHA256: &str =
    "8cdc714ab722b5a39166fc27cfe20c1afdc89c07099c3966789a3c785a51293a";

#[test]
fn golden_games_fixture_matches_its_pinned_sha256() {
    assert_pinned(GOLDEN_GAMES_FILE, GOLDEN_GAMES_SHA256);
}

#[test]
fn golden_games_replay_legally_and_reach_their_recorded_verdict() {
    for game in golden_games() {
        let name = &game.name;
        let (prefix, last) = game.split_last();

        // Everything but the last stone replays legally, and reaches the last
        // stone with the game still open.
        let mut state = GameState::from_plies(prefix)
            .unwrap_or_else(|error| panic!("case `{name}` (line {}): {error}", game.line));
        assert_eq!(
            state.outcome(),
            Outcome::Ongoing,
            "case `{name}`: decided before the last stone"
        );
        let stones_before = state.board().stone_count();
        assert_eq!(stones_before, prefix.len(), "case `{name}`: stones lost");

        match game.expect {
            GameVerdict::Ongoing { turn, phase } => {
                state
                    .place(last)
                    .unwrap_or_else(|error| panic!("case `{name}`: {error}"));
                assert_eq!(state.outcome(), Outcome::Ongoing, "case `{name}`");
                assert_eq!(state.turn(), turn, "case `{name}`: turn after the last ply");
                assert_eq!(state.phase(), phase, "case `{name}`: phase");
            }
            GameVerdict::Win {
                winner,
                turn,
                phase,
            } => {
                // The phase the winning stone is placed at is the whole point of
                // the case, so it is checked before the stone goes down.
                assert_eq!(state.turn(), turn, "case `{name}`: the winning turn");
                assert_eq!(
                    state.phase(),
                    phase,
                    "case `{name}`: the phase the winning stone is placed at"
                );

                assert_eq!(
                    state.place(last),
                    Ok(PlyOutcome::Win { winner, turn }),
                    "case `{name}`"
                );
                assert_eq!(state.outcome(), Outcome::Win { winner, turn });
                assert_eq!(
                    state.turn(),
                    turn,
                    "case `{name}`: the turn does not advance"
                );
                assert_eq!(state.stones_owed(), 0, "case `{name}`");
                assert_eq!(state.board().stone_count(), stones_before + 1);

                // Rule 4: nothing follows a completed line — and when the winning
                // stone was the turn's first, that is exactly the second stone
                // never being played.
                let elsewhere = last.step(pistol_core::Axis::ConstQ, 1);
                assert_eq!(
                    state.place(elsewhere),
                    Err(CoreError::GameDecided { winner, turn }),
                    "case `{name}`: a stone after the win"
                );
                assert_eq!(state.board().stone_count(), stones_before + 1);
            }
        }
    }
}

#[test]
fn golden_games_cover_both_phases_and_both_players() {
    let games = golden_games();
    assert_eq!(games.len(), 5, "the fixture lost or gained cases");

    let wins: Vec<GameVerdict> = games
        .iter()
        .map(|game| game.expect)
        .filter(|verdict| matches!(verdict, GameVerdict::Win { .. }))
        .collect();
    assert_eq!(wins.len(), 3);

    // Rule 4's truncation case, and its counterpart, both present.
    assert!(
        wins.iter().any(|verdict| matches!(
            verdict,
            GameVerdict::Win {
                phase: Phase::First,
                ..
            }
        )),
        "no first-stone win: rule 4's truncation is unpinned"
    );
    assert!(
        wins.iter().any(|verdict| matches!(
            verdict,
            GameVerdict::Win {
                phase: Phase::Second,
                ..
            }
        )),
        "no second-stone win"
    );
    let winners: Vec<_> = wins
        .iter()
        .filter_map(|verdict| match verdict {
            GameVerdict::Win { winner, .. } => Some(*winner),
            GameVerdict::Ongoing { .. } => None,
        })
        .collect();
    assert!(
        winners.contains(&pistol_core::Player::P1) && winners.contains(&pistol_core::Player::P2),
        "both players need a win case: {winners:?}"
    );
}

#[test]
#[should_panic(expected = "unknown directive")]
fn golden_game_loader_refuses_a_line_it_does_not_understand() {
    parse_games("case a\nplies 0,0\nresult win\nexpect ongoing turn 2 phase first\n");
}

#[test]
#[should_panic(expected = "not one of the two forms")]
fn golden_game_loader_refuses_a_verdict_it_cannot_read() {
    parse_games("case a\nplies 0,0\nexpect p1 wins eventually\n");
}

#[test]
#[should_panic(expected = "has no plies")]
fn golden_game_loader_refuses_a_case_with_no_moves() {
    parse_games("case a\nexpect ongoing turn 2 phase first\n");
}
