//! The bounded fallback a wall-clock search secures before deepening (WP-1.4):
//! deterministic given the position, legal by the rules' own replay, and the
//! answer the whole search returns when the deadline expires before anything
//! completed.
//!
//! Wall-clock searches are not reproducible — the INSTANT an abort lands is the
//! machine's — so what these tests pin is the property that survives that: the
//! fallback itself is a pure function of (position, policy), and at an
//! already-expired deadline the search's answer IS the fallback, every time,
//! with the report saying honestly that no depth completed.

mod common;

use std::time::{Duration, Instant};

use common::{position, searcher};
use pistol_core::{Coord, GameState, Player, Turn};
use pistol_search::score::{ScoreKind, classify};
use pistol_search::{CandidatePolicy, FallbackAnswer, Provenance, Stop, fallback_turn};

/// An instant that is already spent — or now, which a `>=` deadline test also
/// reads as spent by the time the first check runs.
fn expired() -> Instant {
    let now = Instant::now();
    now.checked_sub(Duration::from_millis(1)).unwrap_or(now)
}

/// A quiet middlegame: nobody threatens anything, so the fallback's win scans
/// find nothing and the answer is the deterministic ordering's first pair.
fn quiet_middlegame() -> GameState {
    position(
        &[Coord::new(0, 0), Coord::new(2, 0), Coord::new(4, 0)],
        &[
            Coord::new(0, 3),
            Coord::new(2, 3),
            Coord::new(4, 3),
            Coord::new(6, 3),
        ],
        Player::P1,
    )
}

#[test]
fn fallback_answer_is_deterministic_given_position() {
    let state = quiet_middlegame();
    let policy = CandidatePolicy::Radius { radius: 2 };

    // A pure function of the position: two calls, one answer.
    let first = fallback_turn(&state, policy);
    let second = fallback_turn(&state, policy);
    assert_eq!(first, second, "the fallback is a function of the position");

    // Legal by the rules' own judgment, not this crate's.
    state
        .clone()
        .make_turn(first.turn())
        .expect("the fallback turn is legal from the position it was computed for");

    // At an already-expired deadline the whole search answers with exactly this
    // turn — twice, through two separate searchers, because play-mode
    // wall-clock varies and the SET of possible answers at this interrupt point
    // must still be this single fixed turn.
    for _ in 0..2 {
        let mut searcher = searcher(2);
        let outcome = searcher
            .search(&state, Stop::Deadline(expired()), &mut |_| {})
            .expect("an expired deadline still answers");
        assert_eq!(
            outcome.best,
            first.turn(),
            "an interrupt before any completed depth answers the fallback"
        );
        assert_eq!(outcome.provenance, Provenance::Fallback);
        assert_eq!(
            outcome.info.depth_turns, 0,
            "no depth completed, and the report says so"
        );
        assert_eq!(
            outcome.info.pv,
            vec![first.turn()],
            "the evidence is the fallback turn itself"
        );
        assert!(outcome.info.nodes >= 1, "the aborted work is still billed");
    }
}

/// The instant-win half of the fallback: a single stone that completes six is
/// the whole turn (rule 4 — the second stone is then not played), found even at
/// a deadline that expired before the search placed a single node's stone.
#[test]
fn fallback_plays_the_instant_win_as_a_single_stone() {
    // P1 has five in a row, open at (-1,0) and (5,0); P2's stones threaten
    // nothing and block neither end.
    let state = position(
        &[
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
            Coord::new(4, 0),
        ],
        &[
            Coord::new(0, 5),
            Coord::new(1, 5),
            Coord::new(2, 5),
            Coord::new(0, 6),
            Coord::new(1, 6),
            Coord::new(2, 6),
        ],
        Player::P1,
    );
    let policy = CandidatePolicy::Radius { radius: 2 };

    let answer = fallback_turn(&state, policy);
    // (-1,0) precedes (5,0) in the ascending scan; both complete six.
    assert_eq!(
        answer,
        FallbackAnswer::WinsThisTurn(Turn::single(Coord::new(-1, 0))),
        "the first completing stone of the deterministic ordering wins the turn alone"
    );

    let mut searcher = searcher(2);
    let outcome = searcher
        .search(&state, Stop::Deadline(expired()), &mut |_| {})
        .expect("an expired deadline still answers");
    assert_eq!(outcome.best, Turn::single(Coord::new(-1, 0)));
    assert_eq!(outcome.provenance, Provenance::Fallback);
    assert_eq!(
        classify(outcome.info.score),
        ScoreKind::MateIn(1),
        "an instant win the fallback proved is a mate in one turn (docs/decisions.md D-3)"
    );
}

/// The second-stone half: a win no single stone completes, whose completing
/// pair BEGINS with the ascending-first candidate — built that way on the
/// decision-red-team's finding that a gap anywhere else would make this test
/// assert nothing about the second win scan.
#[test]
fn fallback_carries_a_second_stone_win_in_its_pair() {
    // P1 holds (0,0),(1,0),(2,0),(3,0): four in a row, no five anywhere, so no
    // single stone completes six. The ascending-first candidate at radius 2 is
    // (-2,0); once it is down, (-1,0) completes (-2..3,0) — six.
    let state = position(
        &[
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
            Coord::new(0, 4),
        ],
        &[
            Coord::new(0, 8),
            Coord::new(1, 8),
            Coord::new(2, 8),
            Coord::new(0, 9),
            Coord::new(1, 9),
            Coord::new(2, 9),
        ],
        Player::P1,
    );
    let policy = CandidatePolicy::Radius { radius: 2 };

    let answer = fallback_turn(&state, policy);
    let expected =
        Turn::pair(Coord::new(-2, 0), Coord::new(-1, 0)).expect("two distinct cells pair");
    assert_eq!(
        answer,
        FallbackAnswer::WinsThisTurn(expected),
        "the pair carries the completing second stone rather than the first empty cell"
    );

    // And the rules agree it wins: replaying the pair ends the game.
    let mut replay = state.clone();
    let outcome = replay.make_turn(expected).expect("the pair is legal");
    assert!(
        matches!(
            outcome,
            pistol_core::Outcome::Win {
                winner: Player::P1,
                ..
            }
        ),
        "the fallback's claimed win is the rules' win too, got {outcome:?}"
    );
}

/// Rule 3's one-stone turn: on the empty board the fallback is the origin, and
/// an expired deadline on turn 1 still answers it.
#[test]
fn fallback_on_turn_one_is_a_single_stone() {
    let state = GameState::new_game();
    let policy = CandidatePolicy::Radius { radius: 2 };
    assert_eq!(
        fallback_turn(&state, policy),
        FallbackAnswer::Ordinary(Turn::single(Coord::ORIGIN)),
        "turn 1 owes one stone and the empty board admits only the origin"
    );

    let mut searcher = searcher(2);
    let outcome = searcher
        .search(&state, Stop::Deadline(expired()), &mut |_| {})
        .expect("an expired deadline still answers on turn 1");
    assert_eq!(outcome.best, Turn::single(Coord::ORIGIN));
}
