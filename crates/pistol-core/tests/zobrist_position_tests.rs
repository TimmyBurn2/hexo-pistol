//! The key a position carries: what it says about the position, and that it
//! never drifts from what the stones say.
//!
//! Two properties do the work the transposition table (WP-06) and the solver
//! will lean on. Turn order is *not* part of a position, so two orders that
//! reach the same stones reach the same key and the search sees one node rather
//! than two. Everything else that distinguishes two positions *is* in the key —
//! the side to move, and the intra-turn phase, because a turn with one of its
//! two stones down is not the node the completed turn is (docs/decisions.md
//! D-8, D-9).
//!
//! The key is carried incrementally through `place` and `undo`, one XOR each
//! way, and the search takes a position back at every node it leaves. So the
//! last test here is the load-bearing one: over randomized playouts, the
//! carried key equals the key recomputed from the stones at every ply, on the
//! way down and on the way back up.
//!
//! The key *function* those keys are built from is `zobrist_key_tests.rs`.

mod common;

use std::collections::BTreeSet;

use common::playouts::{Rng, random_ply};
use pistol_core::{Board, Coord, GameState, Phase, Player, from_scratch_key, phase_key, side_key};

/// A position no game reaches, built directly (docs/decisions.md D-35).
///
/// The side and phase tests need the *same* stones under different context,
/// which no legal game can offer: in an ongoing game the stone count fixes the
/// turn, the phase and the mover together. That is also why the key loses
/// nothing by leaving the turn number out.
fn synthetic_board() -> Board {
    let mut board = Board::empty();
    for (at, player) in [
        (Coord::ORIGIN, Player::P1),
        (Coord::new(1, 0), Player::P2),
        (Coord::new(0, 1), Player::P1),
        (Coord::new(-2, 3), Player::P2),
        (Coord::new(4, -4), Player::P1),
    ] {
        board.apply(at, player).expect("distinct cells");
    }
    board
}

#[test]
fn zobrist_transpositions_collide_by_construction() {
    // The same five stones by the same two sides, reached two ways: P2's
    // turn 2 and P1's turn 3 each played in the other order. Same position,
    // same side to move, same phase — one key.
    let one = GameState::from_plies(&[
        Coord::ORIGIN,
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(0, 1),
        Coord::new(0, 2),
    ])
    .expect("a legal game");
    let two = GameState::from_plies(&[
        Coord::ORIGIN,
        Coord::new(2, 0),
        Coord::new(1, 0),
        Coord::new(0, 2),
        Coord::new(0, 1),
    ])
    .expect("a legal game");

    assert_ne!(
        one.played().collect::<Vec<_>>(),
        two.played().collect::<Vec<_>>(),
        "the two games must differ in play order, or this test asserts nothing"
    );
    assert_eq!(one.board(), two.board());
    assert_eq!(one.to_move(), two.to_move());
    assert_eq!(one.phase(), two.phase());
    assert_eq!(one.key(), two.key(), "a transposition is one node");

    // And the collision is the intended one, not a key that ignores stones: one
    // stone moved, and the key moves with it.
    let elsewhere = GameState::from_plies(&[
        Coord::ORIGIN,
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(0, 1),
        Coord::new(0, 3),
    ])
    .expect("a legal game");
    assert_ne!(one.key(), elsewhere.key());
}

#[test]
fn zobrist_phase_bit_distinguishes_intra_turn_states() {
    let board = synthetic_board();
    let first = from_scratch_key(&board, Player::P1, Phase::First);
    let second = from_scratch_key(&board, Player::P1, Phase::Second);

    assert_ne!(
        first, second,
        "a half-played turn is not the node the completed turn is"
    );
    assert_eq!(
        first ^ second,
        phase_key(Phase::First) ^ phase_key(Phase::Second),
        "the phase is XORed in, not folded into the stones"
    );

    // And a real position in the middle of a turn carries the phase it is in.
    let mut game = GameState::from_plies(&[Coord::ORIGIN, Coord::new(1, 0)]).expect("a legal game");
    assert_eq!(game.phase(), Phase::Second, "p2 owes one more stone");
    assert_eq!(
        game.key(),
        from_scratch_key(game.board(), game.to_move(), Phase::Second)
    );
    assert_ne!(
        game.key(),
        from_scratch_key(game.board(), game.to_move(), Phase::First)
    );

    game.place(Coord::new(2, 0)).expect("the second stone");
    assert_eq!(game.phase(), Phase::First, "the turn is complete");
    assert_eq!(
        game.key(),
        from_scratch_key(game.board(), game.to_move(), Phase::First)
    );
}

#[test]
fn zobrist_side_to_move_distinguishes() {
    let board = synthetic_board();
    let p1 = from_scratch_key(&board, Player::P1, Phase::First);
    let p2 = from_scratch_key(&board, Player::P2, Phase::First);

    assert_ne!(p1, p2, "whose move it is, is part of the position");
    assert_eq!(
        p1 ^ p2,
        side_key(Player::P1) ^ side_key(Player::P2),
        "the side is XORed in, not folded into the stones"
    );

    let mut distinct = BTreeSet::new();
    for player in [Player::P1, Player::P2] {
        for phase in [Phase::First, Phase::Second] {
            assert!(
                distinct.insert(from_scratch_key(&board, player, phase)),
                "{player} at {phase:?} shares a key with another context"
            );
        }
    }

    // A real turn boundary hands the move over, and the key says so.
    let before = GameState::from_plies(&[Coord::ORIGIN, Coord::new(1, 0)]).expect("a legal game");
    let mut after = before.clone();
    after.place(Coord::new(2, 0)).expect("the second stone");
    assert_eq!(before.to_move(), Player::P2);
    assert_eq!(after.to_move(), Player::P1);
    assert_ne!(before.key(), after.key());
}

#[test]
fn zobrist_apply_undo_roundtrips_to_identical_key() {
    // Eight playouts of up to 150 plies: over a thousand applied and taken-back
    // stones, each of them checked against a recompute. A drift of one XOR
    // anywhere on either path fails this.
    const PLAYOUTS: u64 = 8;
    const PLIES: usize = 150;

    let mut steps = 0usize;
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut keys = vec![game.key()];

        // A playout that stumbles into a completed line stops there: a decided
        // game accepts no further stone (rule 4).
        while keys.len() <= PLIES && !game.outcome().is_decided() {
            let at = random_ply(game.board(), &mut rng);
            game.place(at).expect("a sampled legal cell");
            steps += 1;
            assert_eq!(
                game.key(),
                from_scratch_key(game.board(), game.to_move(), game.phase()),
                "playout {seed}: the carried key drifted at ply {} ({at})",
                keys.len()
            );
            keys.push(game.key());
        }

        for (ply, &expected) in keys.iter().enumerate().rev() {
            assert_eq!(
                game.key(),
                expected,
                "playout {seed}: the key after {ply} plies differs on the way back"
            );
            assert_eq!(
                game.key(),
                from_scratch_key(game.board(), game.to_move(), game.phase()),
                "playout {seed}: the carried key drifted undoing to ply {ply}"
            );
            if ply > 0 {
                game.undo().expect("the stone just placed");
            }
        }
        assert_eq!(game, GameState::new_game(), "playout {seed} did not unwind");
    }

    assert!(
        steps >= 1000,
        "{steps} plies is too thin a sample for an incremental invariant"
    );
}
