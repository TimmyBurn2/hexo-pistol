//! df-pn: Nagai's depth-first proof-number search with the Pawlewicz-Lew
//! thresholds and the 1+ε trick (docs/experiments/wp18a_design.md §4).
//!
//! # The loop semantics the paper's first-child derivation does not state
//!
//! The paper derives its formulas for the first child at one visit; a df-pn
//! node LOOPS. Each iteration of the loop below, in order (the order is
//! load-bearing for the subtraction argument):
//!
//! 1. re-derive `p`/`d` from the children's CURRENT table numbers;
//! 2. TERMINATE if the node is definitive (`p == 0` / `d == 0`) or if
//!    `p >= pt` or `d >= dt`;
//! 3. re-select the minimum child — least by ENUMERATION ORDER on ties
//!    (the policy's own order: arm A's v0 order first, arm B's
//!    raiser-major/free-ascending order appended — design wp18b_m4 §2,
//!    which is why the enumeration order is a determinism decision and
//!    not an implementation detail; it is NOT global canonical-pair order
//!    under the widened policy);
//! 4. recompute `p2`/`d2` against the CURRENT ordering;
//! 5. descend with the formulas evaluated against current values.
//!
//! Because step 2 guaranteed `p < pt ∧ d < dt`, the subtractions in step 5
//! cannot underflow: `rest ≤ d < dt`, so `dt − rest ≥ 1`, and the result is
//! a threshold at least `1` — never a wrap.
//!
//! # The formulas, quoted from Pawlewicz & Lew §2.2
//!
//! OR node, first child: `pt1 = min(pt, p2 + 1), dt1 = dt − d + d1`.
//! AND node, first child: `pt1 = pt − p + p1, dt1 = min(dt, d2 + 1)`.
//! With 1+ε (§3.2): `pt1 = min(pt, ⌈p2 (1 + ε)⌉)` in an OR node, and the
//! AND/DN form by the paper's own symmetry. `dt − d + d1` is implemented as
//! `dt − rest` with `rest` the OTHER children's sum — the same number, since
//! `d = rest + d1`.
//!
//! # Seesaw
//!
//! One counter per solve: a recursive call returns on a threshold miss and
//! the parent's next descent selects a different child. Printed per solve,
//! measured at the gates, no threshold (that number is WP-1.8c's input).

// RULE9-JUSTIFICATION: this module holds one algorithm whose parts are
// read as a whole — the node loop, the child-table arithmetic, the
// threshold formulas the design quotes, the proof DAG and the zone
// construction at proof time all name each other, and the §7 oracle gates
// verify exactly this file's invariants as one cross-check. Splitting the
// search from the zones would put the AT-1/DT-1 propagation the gates
// compare against the verifier on the far side of a module boundary from
// the loop that computes it.
use pistol_core::{GameState, Key128, Player, Turn};

use crate::pn::{Epsilon, INF, Value, saturating_sum, value_of};
use crate::policy::{self, turn_cells};
use crate::state::ThreatState;
use crate::tt::{Entry, SolverTT};
use crate::zone::{ZoneP, ep1_contribution, t31_contribution};
use crate::{StonesLeft, WinWitness};

/// What proved a node, for the witness tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofKind {
    /// An OR node that wins this turn. The completing stones are the
    /// witness.
    OrWinLeaf { witness: WinWitness },
    /// An AND node the attacker wins without expansion: LAW-OVERLOAD, three
    /// or more plans no two-stone defense can hit.
    AndOverloadLeaf,
    /// An OR node proven by one policy move.
    OrStep { witness: Turn },
    /// An AND node proven over its whole blocking-pair child set; the set is
    /// re-derived at emission.
    AndStep,
}

/// One proven node and its zone.
#[derive(Debug, Clone)]
pub struct ProofNode {
    /// The position's key.
    pub key: Key128,
    /// What proved it.
    pub kind: ProofKind,
    /// The relevance zone, built bottom-up at proof time.
    pub zone: ZoneP,
}

/// The proof DAG: every node proven during this solve, keyed by position.
/// Ordered by key, never iterated on a choice path (D-7).
#[derive(Debug, Default)]
pub struct ProofDag {
    nodes: std::collections::BTreeMap<Key128, ProofNode>,
}

impl ProofDag {
    /// Record a proven node. The FIRST proof of a position wins — the search
    /// is deterministic, so a re-proof takes the same witness.
    pub fn record(&mut self, node: ProofNode) {
        self.nodes.entry(node.key).or_insert(node);
    }

    /// A proven node's record, if this solve proved it.
    pub fn get(&self, key: Key128) -> Option<&ProofNode> {
        self.nodes.get(&key)
    }

    /// The zone recorded for a proven node, if any.
    pub fn zone_of(&self, key: Key128) -> Option<&ZoneP> {
        self.nodes.get(&key).map(|node| &node.zone)
    }

    /// How many nodes this solve proved.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether nothing was proven at all.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// What the search counted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SearchStats {
    /// df-pn node visits.
    pub nodes: u64,
    /// Threshold-miss returns after which the parent switched subtrees.
    pub seesaw: u64,
}

/// The search over one root, to completion.
pub struct Search<'a> {
    attacker: Player,
    epsilon: Epsilon,
    attacker_policy: crate::config::AttackerPolicy,
    generation: u32,
    /// The solve's epoch: stamped on every entry this search stores, and
    /// the only epoch its lookups accept.
    pub epoch: u32,
    tt: &'a mut SolverTT,
    dag: &'a mut ProofDag,
    stats: &'a mut SearchStats,
}

/// A named outcome for the case the policy game outgrows the INF sentinel:
/// neither provable nor disprovable within it. Not an answer.
pub const STALLED: &str =
    "SOLVER_STALLED: neither provable nor disprovable within the INF sentinel";

impl<'a> Search<'a> {
    /// One search over one root. The table and the DAG are the caller's so a
    /// solver can keep them across its passes.
    pub fn new(
        attacker: Player,
        epsilon: Epsilon,
        attacker_policy: crate::config::AttackerPolicy,
        tt: &'a mut SolverTT,
        dag: &'a mut ProofDag,
        stats: &'a mut SearchStats,
    ) -> Search<'a> {
        Search {
            attacker,
            epsilon,
            attacker_policy,
            generation: 0,
            epoch: 0,
            tt,
            dag,
            stats,
        }
    }

    /// Run to a definitive root value, refusing to spin: a pass that proves
    /// nothing and disproves nothing raises the stall outcome by name.
    pub fn solve_root(&mut self, state: &mut GameState, threat: &mut ThreatState) -> Value {
        // A node count alone cannot detect a stall — `dfpn` increments it
        // on every entry, so a root that returns the same numbers forever
        // still "makes progress". The guard is the ROOT'S OWN ANSWER: a
        // pass that returns the same (pn, dn) as the previous pass has
        // learned nothing the next pass could use, and re-searching with
        // the same thresholds would walk the same tree — that is the spin
        // the design refuses to allow.
        let mut previous: Option<(u64, u64)> = None;
        loop {
            let (pn, dn) = self.dfpn(state, threat, INF, INF);
            match value_of(pn, dn) {
                Value::Proven => return Value::Proven,
                Value::Disproven => return Value::Disproven,
                Value::Unknown => {
                    if previous == Some((pn, dn)) {
                        panic!("{STALLED}");
                    }
                    previous = Some((pn, dn));
                    self.generation += 1;
                }
            }
        }
    }

    /// The df-pn descent. Returns when the node's numbers reach a threshold
    /// or turn definitive; the caller stores what it learns.
    fn dfpn(
        &mut self,
        state: &mut GameState,
        threat: &mut ThreatState,
        pt: u64,
        dt: u64,
    ) -> (u64, u64) {
        self.stats.nodes += 1;
        let key = state.key();
        if let Some(entry) = self.tt.lookup(key, self.epoch) {
            if entry.pn == 0 {
                return (0, INF);
            }
            if entry.dn == 0 {
                return (INF, 0);
            }
            if entry.pn >= pt || entry.dn >= dt {
                return (entry.pn, entry.dn);
            }
        }
        if state.to_move() == self.attacker {
            self.dfpn_or(state, threat, key, pt, dt)
        } else {
            self.dfpn_and(state, threat, key, pt, dt)
        }
    }

    fn dfpn_or(
        &mut self,
        state: &mut GameState,
        threat: &mut ThreatState,
        key: Key128,
        pt: u64,
        dt: u64,
    ) -> (u64, u64) {
        // §2.1: a win this turn is a leaf proof.
        if let Some(witness) = threat.can_win_this_turn(self.attacker, StonesLeft::Two) {
            let zone = self.leaf_zone(state, threat, witness_cells(&witness));
            self.tt.store(Entry {
                key,
                epoch: self.epoch,
                pn: 0,
                dn: INF,
                zone: Some(zone.clone()),
                generation: self.generation,
            });
            self.dag.record(ProofNode {
                key,
                kind: ProofKind::OrWinLeaf { witness },
                zone,
            });
            return (0, INF);
        }
        // §2.2-3: the policy moves; none at all is a leaf disproof.
        let mut moves = Vec::new();
        policy::threat_pairs(
            state,
            threat,
            self.attacker,
            self.attacker_policy,
            &mut moves,
        );
        if moves.is_empty() {
            self.tt.store(Entry {
                key,
                epoch: self.epoch,
                pn: INF,
                dn: 0,
                zone: None,
                generation: self.generation,
            });
            return (INF, 0);
        }
        let children = self.child_keys(state, threat, &moves);
        let mut last: Option<Turn> = None;
        loop {
            // Step 1: current numbers from the table.
            let mut p = INF;
            let mut d = 0u64;
            let mut p2 = INF;
            let mut selected = None;
            for (turn, child_key) in &children {
                let (cpn, cdn) = self.child_numbers(*child_key);
                if cpn < p {
                    p2 = p;
                    p = cpn;
                    selected = Some((*turn, *child_key, cdn));
                } else if cpn < p2 {
                    p2 = cpn;
                }
                d = saturating_sum([d, cdn]);
            }
            // Step 2: definitive or threshold termination. `selected` is
            // None exactly when every child is disproven (all pn at INF) —
            // the node is disproven, and the d == 0 arm below is the one
            // that answers it, so the empty selection is answered HERE
            // rather than by an unwrap that cannot reach its error.
            if selected.is_none() || d == 0 {
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: INF,
                    dn: 0,
                    zone: None,
                    generation: self.generation,
                });
                return (INF, 0);
            }
            let Some((turn, child_key, d1)) = selected else {
                unreachable!("the empty selection was answered above")
            };
            if p == 0 {
                let zone = self.or_step_zone(state, threat, turn, child_key);
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: 0,
                    dn: INF,
                    zone: Some(zone.clone()),
                    generation: self.generation,
                });
                self.dag.record(ProofNode {
                    key,
                    kind: ProofKind::OrStep { witness: turn },
                    zone,
                });
                return (0, INF);
            }
            if d == 0 {
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: INF,
                    dn: 0,
                    zone: None,
                    generation: self.generation,
                });
                return (INF, 0);
            }
            if p >= pt || d >= dt {
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: p,
                    dn: d,
                    zone: None,
                    generation: self.generation,
                });
                return (p, d);
            }
            // Step 3-4: the selected child and the current second minimum.
            if last.is_some_and(|previous| previous != turn) {
                self.stats.seesaw += 1;
            }
            last = Some(turn);
            // Step 5: the Pawlewicz-Lew thresholds. `rest` is the other
            // children's dn sum; `d < dt` held at step 2, so `rest ≤ d < dt`
            // and `dt − rest` cannot underflow — it equals the paper's
            // `dt − d + d1`, since `d = rest + d1`.
            let rest = d - d1;
            let dt1 = dt - rest;
            let pt1 = pt.min(self.epsilon.loosen(p2));
            apply_turn(state, threat, turn);
            let returned = self.dfpn(state, threat, pt1, dt1);
            undo_turn(state, threat, turn);
            self.merge_store(child_key, returned, None);
        }
    }

    fn dfpn_and(
        &mut self,
        state: &mut GameState,
        threat: &mut ThreatState,
        key: Key128,
        pt: u64,
        dt: u64,
    ) -> (u64, u64) {
        // §2 defender step 1: the race is the defender's to lose.
        let defender = self.attacker.opponent();
        if threat
            .can_win_this_turn(defender, StonesLeft::Two)
            .is_some()
        {
            self.tt.store(Entry {
                key,
                epoch: self.epoch,
                pn: INF,
                dn: 0,
                zone: None,
                generation: self.generation,
            });
            return (INF, 0);
        }
        let hot = threat.hot_windows(self.attacker);
        assert!(!hot.is_empty(), "{}", policy::NO_PLAN_ASSERT);
        // §2 defender step 3: LAW-OVERLOAD, no expansion.
        if policy::overload(threat, self.attacker) {
            let mut zone = t31_contribution(threat, self.attacker);
            zone.union_with(&ep1_contribution(state.board(), self.attacker));
            self.tt.store(Entry {
                key,
                epoch: self.epoch,
                pn: 0,
                dn: INF,
                zone: Some(zone.clone()),
                generation: self.generation,
            });
            self.dag.record(ProofNode {
                key,
                kind: ProofKind::AndOverloadLeaf,
                zone,
            });
            return (0, INF);
        }
        // §2 defender step 4: the blocking pairs.
        let mut children = Vec::new();
        policy::blocking_pairs(state, threat, self.attacker, &mut children);
        assert!(
            !children.is_empty(),
            "pistol-solver invariant SOLVER_UNCOVERED_PLAN: t <= 2 admits a cover, so a blocking pair exists"
        );
        let child_keys = self.child_keys(state, threat, &children);
        let mut last: Option<Turn> = None;
        loop {
            // AND is OR's dual: p sums, d takes the minimum.
            let mut p = 0u64;
            let mut d = INF;
            let mut d2 = INF;
            let mut selected = None;
            for (turn, child_key) in &child_keys {
                let (cpn, cdn) = self.child_numbers(*child_key);
                if cdn < d {
                    d2 = d;
                    d = cdn;
                    selected = Some((*turn, *child_key, cpn));
                } else if cdn < d2 {
                    d2 = cdn;
                }
                p = saturating_sum([p, cpn]);
            }
            // The AND dual: `selected` is None exactly when every child is
            // proven — the node is proven, answered here before the unwrap.
            if selected.is_none() || p == 0 {
                let zone = self.and_step_zone(state, threat, &child_keys);
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: 0,
                    dn: INF,
                    zone: Some(zone.clone()),
                    generation: self.generation,
                });
                self.dag.record(ProofNode {
                    key,
                    kind: ProofKind::AndStep,
                    zone,
                });
                return (0, INF);
            }
            let Some((turn, child_key, p1)) = selected else {
                unreachable!("the empty selection was answered above")
            };
            if d == 0 {
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: INF,
                    dn: 0,
                    zone: None,
                    generation: self.generation,
                });
                return (INF, 0);
            }
            if p == 0 {
                let zone = self.and_step_zone(state, threat, &child_keys);
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: 0,
                    dn: INF,
                    zone: Some(zone.clone()),
                    generation: self.generation,
                });
                self.dag.record(ProofNode {
                    key,
                    kind: ProofKind::AndStep,
                    zone,
                });
                return (0, INF);
            }
            if p >= pt || d >= dt {
                self.tt.store(Entry {
                    key,
                    epoch: self.epoch,
                    pn: p,
                    dn: d,
                    zone: None,
                    generation: self.generation,
                });
                return (p, d);
            }
            if last.is_some_and(|previous| previous != turn) {
                self.stats.seesaw += 1;
            }
            last = Some(turn);
            // The AND dual of step 5: `pt − rest` with `rest` the other
            // children's pn sum, and the loosened dn threshold.
            let rest = p - p1;
            let pt1 = pt - rest;
            let dt1 = dt.min(self.epsilon.loosen(d2));
            apply_turn(state, threat, turn);
            let returned = self.dfpn(state, threat, pt1, dt1);
            undo_turn(state, threat, turn);
            self.merge_store(child_key, returned, None);
        }
    }

    /// The (turn, child-key) pairs for a node's moves, by applying each turn
    /// once. Keys are stable; the loop re-reads only the table.
    fn child_keys(
        &mut self,
        state: &mut GameState,
        threat: &mut ThreatState,
        moves: &[Turn],
    ) -> Vec<(Turn, Key128)> {
        moves
            .iter()
            .map(|&turn| {
                apply_turn(state, threat, turn);
                let key = state.key();
                undo_turn(state, threat, turn);
                (turn, key)
            })
            .collect()
    }

    /// A child's current numbers: the table's, or the unsolved-leaf
    /// initialisation `1/1` (Pawlewicz-Lew §2.2: "In case of failure we
    /// initialize the node's PN and DN the same way as we do it for a
    /// leaf").
    fn child_numbers(&self, key: Key128) -> (u64, u64) {
        match self.tt.lookup(key, self.epoch) {
            Some(entry) => (entry.pn, entry.dn),
            None => (1, 1),
        }
    }

    /// Store a child's returned numbers, merging monotonically: a definitive
    /// return overwrites, otherwise components only grow — stored values stay
    /// valid lower bounds, which is what makes re-searches safe.
    fn merge_store(&mut self, key: Key128, returned: (u64, u64), zone: Option<ZoneP>) {
        let (rpn, rdn) = returned;
        let merged = match self.tt.lookup(key, self.epoch) {
            Some(existing) => {
                let (pn, dn) = if rpn == 0 {
                    (0, INF)
                } else if rdn == 0 {
                    (INF, 0)
                } else {
                    (existing.pn.max(rpn), existing.dn.max(rdn))
                };
                let zone = existing.zone.clone().or(zone);
                Entry {
                    key,
                    epoch: self.epoch,
                    pn,
                    dn,
                    zone,
                    generation: self.generation,
                }
            }
            None => Entry {
                key,
                epoch: self.epoch,
                pn: rpn,
                dn: rdn,
                zone,
                generation: self.generation,
            },
        };
        self.tt.store(merged);
    }

    /// A win leaf's zone: the completing stones in every order, plus the
    /// node's EP-1.
    fn leaf_zone(
        &self,
        state: &GameState,
        threat: &ThreatState,
        cells: [pistol_core::Coord; 2],
    ) -> ZoneP {
        let mut zone = ZoneP::new();
        zone.add_all_orders(cells);
        zone.union_with(&ep1_contribution(state.board(), self.attacker));
        let _ = threat;
        zone
    }

    /// An OR step's zone: the proving child's zone, the move's cells, the
    /// node's EP-1 — AT-1.
    fn or_step_zone(
        &mut self,
        state: &mut GameState,
        threat: &mut ThreatState,
        turn: Turn,
        child_key: Key128,
    ) -> ZoneP {
        let mut zone = self
            .dag
            .zone_of(child_key)
            .cloned()
            .unwrap_or_else(|| panic!("pistol-solver invariant SOLVER_CHILD_ZONE: the proving child was recorded before its parent"));
        zone.add_all_orders(turn_cells(&turn));
        zone.union_with(&ep1_contribution(state.board(), self.attacker));
        zone.assert_invariants();
        let _ = threat;
        zone
    }

    /// An AND step's zone: the union over every blocking pair's child zone,
    /// plus T3-1 and EP-1 — DT-1.
    fn and_step_zone(
        &mut self,
        state: &mut GameState,
        threat: &mut ThreatState,
        children: &[(Turn, Key128)],
    ) -> ZoneP {
        let mut zone = ZoneP::new();
        for (_, child_key) in children {
            zone.union_with(
                &self
                    .dag
                    .zone_of(*child_key)
                    .cloned()
                    .unwrap_or_else(|| panic!("pistol-solver invariant SOLVER_CHILD_ZONE: every AND child was proven before its parent")),
            );
        }
        zone.union_with(&t31_contribution(threat, self.attacker));
        zone.union_with(&ep1_contribution(state.board(), self.attacker));
        zone.assert_invariants();
        zone
    }
}

/// The completing stones of a win witness.
fn witness_cells(witness: &WinWitness) -> [pistol_core::Coord; 2] {
    match *witness {
        WinWitness::OnePly { at, .. } => [at, at],
        WinWitness::Pair { first, second, .. } => [first, second],
    }
}

/// Apply a whole turn to the game and the threat state together.
pub(crate) fn apply_turn(state: &mut GameState, threat: &mut ThreatState, turn: Turn) {
    let mover = state.to_move();
    state.make_turn(turn).unwrap_or_else(|error| {
        panic!("pistol-solver invariant SOLVER_ILLEGAL_TURN: the policy only generates legal turns ({error})")
    });
    let [first, second] = turn_cells(&turn);
    threat.apply(first, mover);
    if turn.stone_count() == 2 {
        threat.apply(second, mover);
    }
}

/// Take a whole turn back.
pub(crate) fn undo_turn(state: &mut GameState, threat: &mut ThreatState, turn: Turn) {
    let mover = state.to_move().opponent();
    state
        .unmake_turn()
        .unwrap_or_else(|error| panic!("pistol-solver invariant SOLVER_UNDO: {error}"));
    let [first, second] = turn_cells(&turn);
    if turn.stone_count() == 2 {
        threat.undo(second, mover);
    }
    threat.undo(first, mover);
}
