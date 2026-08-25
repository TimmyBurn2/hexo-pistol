//! R3' — the brute-force reference for the policy game (design §7a).
//!
//! D-68's pattern, fourth instance after the eval, the search and the
//! threat queries: a second implementation sharing NOTHING with the solver
//! but pistol-core — no df-pn, no thresholds, no TT, no zones, and none of
//! the solver's §2 defender-side shortcuts. Every legal defender turn from
//! `generate_turns` is applied and the child evaluated; the classifications
//! (defender-wins-first, LAW-OVERLOAD, blocking) are never used, which is
//! what makes them falsifiable here.
//!
//! The attacker side IS policy-restricted — that restriction is the game
//! being solved — but implemented independently: its own window scan over
//! the board, not `ThreatState`.
//!
//! Short-circuiting is value-preserving: OR stops at the first `Win` child,
//! AND at the first `NoWin` child. Memoised by position key so a shared
//! subtree is evaluated once — the memo is a `BTreeMap`, never iterated on
//! a choice path (D-7).

use std::collections::BTreeMap;

use pistol_core::window::{Window, windows_through};
use pistol_core::{Coord, GameState, Key128, Player, Turn};

/// The reference's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefValue {
    /// The attacker forces the win under the policy.
    Win,
    /// It does not.
    NoWin,
}

/// One invocation: memo table inside, fresh per position (the caller owns
/// the lifetime).
pub struct Reference {
    attacker: Player,
    memo: BTreeMap<Key128, RefValue>,
}

impl Reference {
    /// Solve `state` for the policy game.
    ///
    /// # Panics
    ///
    /// If the position is not an undecided `Phase::First` position owing
    /// two stones — the same precondition the solver asserts, stated here
    /// by the same panic text so a fixture feeding one a different position
    /// than the other fails loudly.
    pub fn solve(state: &GameState) -> RefValue {
        assert_precondition(state);
        let attacker = state.to_move();
        let mut reference = Reference {
            attacker,
            memo: BTreeMap::new(),
        };
        let mut work = state.clone();
        reference.value(&mut work)
    }

    fn value(&mut self, state: &mut GameState) -> RefValue {
        let key = state.key();
        if let Some(&known) = self.memo.get(&key) {
            return known;
        }
        let value = if state.to_move() == self.attacker {
            self.or_value(state)
        } else {
            self.and_value(state)
        };
        self.memo.insert(key, value);
        value
    }

    fn or_value(&mut self, state: &mut GameState) -> RefValue {
        // §2.1, independently: a completing stone ends the turn.
        if let Some(turn) = self.win_now(state) {
            let mut applied = state.clone();
            applied.make_turn(turn).expect("a completing turn is legal");
            if applied.outcome().is_decided() {
                return RefValue::Win;
            }
            unreachable!("a win-now turn decides the game");
        }
        // §2.2-3, independently: the threat pairs, from this module's own
        // window scan.
        let moves = self.threat_moves(state);
        if moves.is_empty() {
            return RefValue::NoWin;
        }
        for turn in moves {
            let mut child = state.clone();
            child.make_turn(turn).expect("a policy turn is legal");
            if child.outcome().is_decided() {
                // The policy never completes six (the win check above
                // fires first); a decided child here is a defect.
                unreachable!("a policy turn decided the game without being a win-now turn");
            }
            if self.value(&mut child) == RefValue::Win {
                return RefValue::Win;
            }
        }
        RefValue::NoWin
    }

    fn and_value(&mut self, state: &mut GameState) -> RefValue {
        // EVERY legal defender turn, no shortcuts.
        let turns = pistol_core::generate_turns(state)
            .expect("an AND node is an undecided position at Phase::First");
        for turn in turns {
            let mut child = state.clone();
            child.make_turn(turn).expect("a legal turn applies");
            if let pistol_core::Outcome::Win { winner, .. } = child.outcome() {
                if winner != self.attacker {
                    // The defender completed six: the race is lost.
                    return RefValue::NoWin;
                }
                unreachable!("the attacker cannot complete six on the defender's turn");
            }
            if self.value(&mut child) == RefValue::NoWin {
                return RefValue::NoWin;
            }
        }
        RefValue::Win
    }

    /// §2.1, independently: a single-cell completion via this module's own
    /// scan (a five-own live window) or a two-cell completion of a
    /// four-own one.
    fn win_now(&self, state: &GameState) -> Option<Turn> {
        let board = state.board();
        let attacker = self.attacker;
        let mut windows = live_windows(board, attacker);
        windows.sort_unstable();
        // One-ply first, deterministically least cell.
        for (window, own) in &windows {
            if *own == 5 {
                let at = (0..6u8)
                    .map(|index| window.cell(index))
                    .find(|&at| !board.is_occupied(at))
                    .expect("a live five-own window has one empty");
                return Some(Turn::Single(at));
            }
        }
        for (window, own) in &windows {
            if *own == 4 {
                let empties: Vec<Coord> = (0..6u8)
                    .map(|index| window.cell(index))
                    .filter(|at| !board.is_occupied(*at))
                    .collect();
                let (first, second) = (empties[0].min(empties[1]), empties[0].max(empties[1]));
                return Some(Turn::pair(first, second).expect("the two empties are distinct"));
            }
        }
        None
    }

    /// §2.2, independently: pairs of `C`-cells that create a hot window.
    fn threat_moves(&self, state: &GameState) -> Vec<Turn> {
        let board = state.board();
        let attacker = self.attacker;
        let mut candidates = candidate_cells(board, attacker);
        candidates.sort_unstable();
        candidates.dedup();
        let mut moves = Vec::new();
        for (index, &first) in candidates.iter().enumerate() {
            for &second in &candidates[index + 1..] {
                // Apply on a clone: no ThreatState here, by construction.
                let mut child = state.clone();
                let turn = Turn::pair(first, second).expect("candidates are distinct");
                if child.make_turn(turn).is_err() {
                    continue;
                }
                let after = child.board();
                if live_windows(after, attacker)
                    .iter()
                    .any(|(_, own)| *own >= 4)
                {
                    moves.push(turn);
                }
            }
        }
        moves
    }
}

/// Live windows for `side` (own ≥ 1, opponent 0) with their own counts —
/// the reference's own reading, every cell straight off the board.
fn live_windows(board: &pistol_core::Board, side: Player) -> Vec<(Window, u32)> {
    let mut found: BTreeMap<Window, (u32, bool)> = BTreeMap::new();
    for (at, _) in board.stones() {
        for window in windows_through(at) {
            found.entry(window).or_insert_with(|| {
                let mut own = 0u32;
                let mut live = true;
                for index in 0..6u8 {
                    match board.get(window.cell(index)) {
                        Some(player) if player == side => own += 1,
                        Some(_) => live = false,
                        None => {}
                    }
                }
                (own, live)
            });
        }
    }
    found
        .into_iter()
        .filter(|(_, (own, live))| *live && *own >= 1)
        .map(|(window, (own, _))| (window, own))
        .collect()
}

/// The attacker's candidate cells `C`: empties of live windows with own
/// ≥ 2, from this module's own scan.
fn candidate_cells(board: &pistol_core::Board, side: Player) -> Vec<Coord> {
    let mut cells = Vec::new();
    for (window, own) in live_windows(board, side) {
        if own >= 2 {
            cells.extend(
                (0..6u8)
                    .map(|index| window.cell(index))
                    .filter(|&at| !board.is_occupied(at)),
            );
        }
    }
    cells
}

fn assert_precondition(state: &GameState) {
    assert!(
        !state.outcome().is_decided()
            && state.phase() == pistol_core::Phase::First
            && state.stones_owed() == 2,
        "pistol-solver reference R3' precondition: an ongoing two-stone position"
    );
}
