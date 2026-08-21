//! The WP-1.5b mutation ledger's two BUILT witnesses, pinned as positions a
//! legal game reaches.
//!
//! `docs/experiments/U4_soundness_instrument.md` §8.4 registers eight mutations
//! and names the position each dies on. Two of them — M4 and M6 — had no corpus
//! witness and were BUILT, and the revision-7 REVIEW-design found (MAJOR 8) that
//! both built positions were `ThreatState`-level constructions with stone counts
//! rule 3 makes unreachable: turn 1 is ONE stone and every later turn is TWO by
//! the mover, so P1 always holds an ODD number and P2 holds one more or one
//! fewer. M4's witness held P1 = 8 and M6's held P2 = 15.
//!
//! §8.4's claim was "VERIFIED on the shipped solver", which was true of
//! `ThreatState::apply` driven directly and is not a verification of anything the
//! SEARCH can reach. A mutation that dies only on a position no game produces is
//! a mutation the gate never kills. So the two witnesses are rebuilt here and
//! REPLAYED THROUGH THE RULES: `common::play` drives every ply through
//! `GameState`, which is the referee (CLAUDE.md rule 2 — geometry, legality and
//! win detection live in pistol-core and no other crate re-implements them), and
//! an illegal ply, an out-of-region placement or an early win panics there
//! rather than here.
//!
//! This file measures the witnesses instead of asserting them, which is D-295's
//! own finding applied to the ledger that quotes it.

mod common;

use common::play;
use pistol_core::{Coord, GameState, Player};
use pistol_solver::{Cover, HitBudget, MinimalCover, StonesLeft, ThreatState, WinWitness};

fn c(q: i16, r: i16) -> Coord {
    Coord::new(q, r)
}

/// Rule 3's turn structure, as a ply list: P1 opens with one stone, and every
/// turn after that is two stones by the side to move.
///
/// The interleave is stated here rather than taken from a helper because the
/// PARITY it produces is the thing under test: if a witness's stone counts do
/// not fit this structure there is no ply list at all, and `play` never gets a
/// chance to complain.
fn replay(p1: &[Coord], p2: &[Coord]) -> (GameState, ThreatState) {
    assert_eq!(p1[0], Coord::ORIGIN, "turn 1 is one stone, the origin (rule 3)");
    assert_eq!(p1.len() % 2, 1, "p1 holds an odd number of stones, got {}", p1.len());
    assert!(
        p2.len() == p1.len() + 1 || p2.len() + 1 == p1.len(),
        "p2 holds one more or one fewer stone than p1, got {} against {}",
        p2.len(),
        p1.len()
    );

    let mut plies = vec![p1[0]];
    let (mut i, mut j) = (1usize, 0usize);
    while i < p1.len() || j < p2.len() {
        for _ in 0..2 {
            if j < p2.len() {
                plies.push(p2[j]);
                j += 1;
            }
        }
        for _ in 0..2 {
            if i < p1.len() {
                plies.push(p1[i]);
                i += 1;
            }
        }
    }
    play(&plies)
}

/// M4 — "minimum-cardinality covers instead of inclusion-minimal".
///
/// The mutant is an identity unless a 1-cover COEXISTS with a minimal 2-cover,
/// which is what this position holds: P1 owns two four-stone arms sharing an
/// empty corner, each arm sealed at both far ends so exactly one window per arm
/// is hot. The corner hits both windows alone; the two far empties hit one each.
/// Minimum-cardinality keeps `One` and drops the `Two`, and the drop is the
/// mutation.
#[test]
fn the_m4_witness_is_a_position_a_legal_game_reaches_and_separates_the_two_cover_notions() {
    let p1 = [
        c(0, 0), c(1, 0), c(2, 0), c(3, 0),
        c(-1, 1), c(-1, 2), c(-1, 3), c(-1, 4),
        c(0, 7),
    ];
    let p2 = [
        c(-2, 0), c(5, 0), c(-1, -1), c(-1, 6),
        c(4, -4), c(5, -4), c(-4, 4), c(-5, 5),
    ];

    let (game, threats) = replay(&p1, &p2);

    assert_eq!(game.to_move(), Player::P2, "the defender is the side to move");
    assert_eq!(StonesLeft::from_state(&game), Some(StonesLeft::Two));
    assert_eq!(
        threats.can_win_this_turn(Player::P2, StonesLeft::Two),
        None,
        "the FILTERED row is selected only where the mover cannot win now"
    );
    assert_eq!(
        threats.blocking_covers(Player::P2, HitBudget::Two),
        Cover::Minimal(vec![
            MinimalCover::One(c(-1, 0)),
            MinimalCover::Two { first: c(-1, 5), second: c(4, 0) },
        ]),
        "a one-cell cover coexisting with a minimal two-cell one is what the \
         mutant collapses"
    );
}

/// M6 — "the overload return drops its `can_win_this_turn` guard".
///
/// The mutant fires where the mover can win NOW and the opponent holds an
/// unblockable double threat: guarded, the mover wins; unguarded, the return
/// condemns a mover who was about to win. P1 holds one five-run sealed at one
/// end, so exactly one cell completes it; P2 holds three disjoint five-runs
/// eight apart, so no window is shared and no two cells block all three.
#[test]
fn the_m6_witness_is_a_position_a_legal_game_reaches_and_holds_win_now_beside_an_unblockable_double()
{
    let mut p1 = vec![c(0, 0), c(1, 0), c(2, 0), c(3, 0), c(4, 0)];
    p1.extend([c(-1, 8), c(-1, 16), c(-1, 24)]);
    p1.extend([c(0, 4), c(3, 4), c(0, 12), c(3, 12), c(0, 20), c(3, 20), c(7, 4)]);

    let mut p2 = vec![c(-1, 0)];
    for row in [8i16, 16, 24] {
        for q in 0..5i16 {
            p2.push(c(q, row));
        }
    }

    let (game, threats) = replay(&p1, &p2);

    assert_eq!(game.to_move(), Player::P1, "the mover is the side that can win now");
    assert_eq!(StonesLeft::from_state(&game), Some(StonesLeft::Two));

    let witness = threats
        .can_win_this_turn(Player::P1, StonesLeft::Two)
        .expect("the mover can win this turn");
    match witness {
        WinWitness::OnePly { at, .. } => assert_eq!(at, c(5, 0), "one cell completes the run"),
        other => panic!("the guard's subject is a one-ply win, got {other:?}"),
    }

    assert!(
        threats.unblockable_double_threat(Player::P2, HitBudget::Two),
        "the opponent's three disjoint threats are what the unguarded return would \
         read as a loss"
    );
}
