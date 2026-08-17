//! The load-bearing property of the whole crate: the value carried
//! incrementally is the value a recompute would give, at every position, on the
//! way down and on the way back up.
//!
//! The search applies and takes back one stone per ply and never recomputes
//! (docs/decisions.md D-11, D-41). So a drift of one window anywhere on either
//! path is a wrong evaluation at every node below it, and nothing else in this
//! crate would notice.

mod common;

use common::playouts::{Rng, random_ply};
use common::reference::value_from_scratch;
use common::{built, committed_weights};
use pistol_core::{Coord, GameState, Player};
use pistol_eval::{Eval, HandcraftedV0, Weights};

/// Both sides' readings of a position, which is what a caller can observe.
fn both_sides(eval: &HandcraftedV0) -> (i32, i32) {
    (eval.value(Player::P1), eval.value(Player::P2))
}

/// Assert the incremental value equals a from-scratch recompute, for both sides.
///
/// The recompute runs once and P2's reading is its negation: the recompute is
/// what makes this test cost anything, and running it twice would pay for the
/// antisymmetry that `eval_antisymmetric_under_player_swap` already pins.
fn assert_matches_recompute(eval: &HandcraftedV0, game: &GameState, weights: &Weights, at: &str) {
    let expected = value_from_scratch(game.board(), weights, Player::P1);
    assert_eq!(
        both_sides(eval),
        (expected, -expected),
        "{at}: the carried value drifted from a recompute"
    );
}

#[test]
fn eval_incremental_matches_from_scratch_on_random_playouts() {
    // Eight playouts of up to 150 plies: over a thousand applied and taken-back
    // stones, each of them checked against a recompute both ways.
    const PLAYOUTS: u64 = 8;
    const PLIES: usize = 150;

    let weights = committed_weights();
    let mut steps = 0usize;
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut eval = HandcraftedV0::new(weights.clone());
        let mut played: Vec<(Coord, Player)> = Vec::new();
        let mut values = vec![both_sides(&eval)];

        // A playout that stumbles into a completed line stops there: a decided
        // game accepts no further stone (rule 4). The eval still saw the winning
        // stone, and still has to be right about it.
        while played.len() < PLIES && !game.outcome().is_decided() {
            let at = random_ply(game.board(), &mut rng);
            let mover = game.to_move();
            game.place(at).expect("a sampled legal cell");
            eval.apply(at, mover);
            steps += 1;
            played.push((at, mover));
            assert_matches_recompute(
                &eval,
                &game,
                &weights,
                &format!("playout {seed} ply {} ({at})", played.len()),
            );
            values.push(both_sides(&eval));
        }

        while let Some((at, mover)) = played.pop() {
            eval.undo(at, mover);
            game.undo().expect("the stone just placed");
            assert_eq!(
                both_sides(&eval),
                values[played.len()],
                "playout {seed}: the value after {} plies differs on the way back",
                played.len()
            );
            assert_matches_recompute(
                &eval,
                &game,
                &weights,
                &format!("playout {seed} undoing to ply {}", played.len()),
            );
        }

        assert_eq!(
            eval,
            HandcraftedV0::new(weights.clone()),
            "playout {seed} did not unwind to a fresh eval"
        );
    }

    assert!(
        steps >= 1000,
        "{steps} plies is too thin a sample for an incremental invariant"
    );
}

#[test]
fn eval_apply_undo_roundtrip() {
    let weights = committed_weights();
    // Stones that share windows on all three axes, so taking one back has to
    // repair counts rather than merely drop entries.
    let stones = [
        (Coord::new(0, 0), Player::P1),
        (Coord::new(0, 1), Player::P1),
        (Coord::new(1, 0), Player::P2),
        (Coord::new(0, 2), Player::P1),
        (Coord::new(2, -1), Player::P2),
        (Coord::new(0, 3), Player::P2),
        (Coord::new(-1, 1), Player::P1),
    ];

    let fresh = HandcraftedV0::new(weights.clone());
    let mut eval = fresh.clone();
    let mut values = vec![both_sides(&eval)];
    for &(at, player) in &stones {
        eval.apply(at, player);
        values.push(both_sides(&eval));
    }
    let full = eval.clone();
    assert_ne!(full, fresh, "the position must be worth something");

    // Last stone first: the order a search takes a line back in.
    for (index, &(at, player)) in stones.iter().enumerate().rev() {
        eval.undo(at, player);
        assert_eq!(
            both_sides(&eval),
            values[index],
            "taking back {at} did not restore the value it was applied to"
        );
    }
    assert_eq!(
        eval, fresh,
        "an unwound eval must be indistinguishable from a fresh one, \
         emptied windows included"
    );

    // And in an order no search would use. The value is a function of the stones
    // on the board, not of the path that put them there, so a rotated take-back
    // has to arrive at the same empty state.
    let mut eval = full.clone();
    for &(at, player) in stones.iter().skip(3).chain(stones.iter().take(3)) {
        eval.undo(at, player);
    }
    assert_eq!(eval, fresh, "take-back order must not matter");
}

#[test]
fn eval_is_independent_of_the_order_stones_were_applied() {
    let weights = committed_weights();
    let stones = [
        (Coord::new(0, 0), Player::P1),
        (Coord::new(1, -1), Player::P2),
        (Coord::new(0, 1), Player::P1),
        (Coord::new(2, -2), Player::P2),
        (Coord::new(0, 2), Player::P1),
    ];
    let mut reversed = stones;
    reversed.reverse();

    let (_, forwards) = built(&weights, &stones);
    let (_, backwards) = built(&weights, &reversed);
    assert_eq!(
        forwards, backwards,
        "two orders of the same stones are one position"
    );
}
