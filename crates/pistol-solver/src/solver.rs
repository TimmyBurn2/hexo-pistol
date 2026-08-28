// RULE9-JUSTIFICATION: the entry, the witness emission walk, the digest
// and the win-depth walk are one traversal of the proof DAG — the walk's
// per-node zone tripwire is the design §3 containment invariant stated
// over the same nodes the emission produces, the depth reads the same
// emitted tree the digest fingerprints, and separating them would let the
// certificate and its checks drift.
use pistol_core::{GameState, Key128, Player, Turn};

use crate::dfpn::{ProofDag, ProofKind, Search, SearchStats};
use crate::pn::{Epsilon, Value};
use crate::policy::{self, turn_cells};
use crate::state::ThreatState;
use crate::tt::SolverTT;
use crate::zone::ZoneP;

/// The precondition the solver states and refuses by name on violation
/// (CLAUDE.md rule 3: wrong-kind input raises a named error).
pub const WRONG_POSITION: &str =
    "SOLVER_WRONG_POSITION: the solver takes an ongoing position at Phase::First owing two stones";

/// The solver's answer for a position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveOutcome {
    /// The attacker forces a rule-2 win under the policy; the witness tree
    /// is attached.
    Win(Box<ProofTree>),
    /// The attacker cannot force a win under the policy.
    NoWin,
    /// A win was found but its zone violates the containment invariant —
    /// refused loudly rather than certified. Unreachable at the v0 config by
    /// construction; the outcome exists so the invariant has a failure mode.
    NoWinUnderZone,
    /// The visit cap truncated the search before any definitive value
    /// (design wp18b §2a). NOT a refutation: the caller treats it as "no
    /// verdict here", never as `NoWin`.
    Unknown,
}

/// Everything one solve produced.
#[derive(Debug, Clone)]
pub struct SolveResult {
    /// The value.
    pub outcome: SolveOutcome,
    /// df-pn node visits.
    pub nodes: u64,
    /// Threshold-miss returns after which the parent switched subtrees.
    pub seesaw: u64,
}

/// One emitted node of the witness tree. Transpositions collapse: a position
/// proven once appears once, its children referencing further nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmittedNode {
    /// The position's key.
    pub key: Key128,
    /// What proved it.
    pub kind: ProofKind,
    /// The solver's zone for it.
    pub zone: ZoneP,
    /// For `OrStep`: the one child (after the witness move). For `AndStep`:
    /// every blocking pair and its child.
    pub children: Vec<(Turn, Key128)>,
}

/// The emitted witness tree: nodes in deterministic POST-ORDER walk order
/// (children before the parent that proved them), so the ROOT is
/// `nodes.last()` — key by `self.root`, never by position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofTree {
    /// The root position's key.
    pub root: Key128,
    /// Every node, walk order.
    pub nodes: Vec<EmittedNode>,
}

impl ProofTree {
    /// A deterministic digest of the tree: nodes, kinds, witness cells,
    /// children and zones folded through splitmix64. Equal trees under equal
    /// configs give equal digests; it is a fingerprint, not a checksum.
    pub fn digest(&self) -> u64 {
        let mut z = 0x243F_6A88_85A3_08D3u64;
        for node in &self.nodes {
            z = mix(z ^ node.key.low()).wrapping_add(node.key.high());
            z = mix(z ^ kind_tag(&node.kind));
            for (turn, child) in &node.children {
                for at in turn_cells(turn) {
                    z = mix(z ^ ((at.q as u64) << 16 | (at.r as u64) as u16 as u64));
                }
                z = mix(z ^ child.low());
            }
            for order in 0..crate::zone::ZONE_ORDERS {
                z = mix(z ^ (node.zone.order(order).len() as u64));
                for at in node.zone.order(order) {
                    z = mix(z ^ ((at.q as u64) << 16 | (at.r as u64) as u16 as u64));
                }
            }
        }
        mix(z)
    }

    /// The witness strategy's win depth in TURNS: the longest attacker
    /// line the proof certifies, counted in the ATTACKER's own turns
    /// (sudden death is scored in turns, rule 4, and the defender's blocking
    /// turns are not the attacker's). An [`ProofKind::OrWinLeaf`] completes
    /// on its own attacker turn; an [`ProofKind::OrStep`] spends one; an
    /// [`ProofKind::AndOverloadLeaf`] makes the attacker's COMPLETING turn
    /// inevitable without expanding it (LAW-OVERLOAD: the surviving plan,
    /// size ≤ 2, is placed next turn); an [`ProofKind::AndStep`] is the
    /// defender's turn and adds nothing.
    ///
    /// Max over root-to-leaf paths of the emitted DAG — transpositions
    /// collapse, so a node can carry two parents; the walk is memoized and
    /// keyed, never hash-ordered (D-7). A child key absent from the node map
    /// is a bug, not an answer: it panics, because an emitted tree is
    /// complete by construction.
    pub fn win_depth_turns(&self) -> u32 {
        let by_key: std::collections::BTreeMap<Key128, &EmittedNode> =
            self.nodes.iter().map(|node| (node.key, node)).collect();
        let mut memo: std::collections::BTreeMap<Key128, u32> = std::collections::BTreeMap::new();
        self.depth_of(self.root, &by_key, &mut memo)
    }

    fn depth_of(
        &self,
        key: Key128,
        by_key: &std::collections::BTreeMap<Key128, &EmittedNode>,
        memo: &mut std::collections::BTreeMap<Key128, u32>,
    ) -> u32 {
        if let Some(seen) = memo.get(&key) {
            return *seen;
        }
        let node = by_key.get(&key).expect("emitted trees are complete");
        let own = match &node.kind {
            ProofKind::OrWinLeaf { .. } | ProofKind::OrStep { .. } | ProofKind::AndOverloadLeaf => {
                1
            }
            ProofKind::AndStep => 0,
        };
        let deepest = node
            .children
            .iter()
            .map(|(_, child)| self.depth_of(*child, by_key, memo))
            .max()
            .unwrap_or(0);
        let value = own + deepest;
        memo.insert(key, value);
        value
    }
}

fn kind_tag(kind: &ProofKind) -> u64 {
    match kind {
        ProofKind::OrWinLeaf { .. } => 1,
        ProofKind::AndOverloadLeaf => 2,
        ProofKind::OrStep { .. } => 3,
        ProofKind::AndStep => 4,
    }
}

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The unbounded cap: what the WP-1.8a gates' registered runs and the
/// probe instruments pass (an explicit constant, not a default — rule 1).
pub const UNCAPPED: u64 = u64::MAX;

/// A solver: configuration plus ONE table, reused across solves.
pub struct Solver {
    epsilon: Epsilon,
    attacker_policy: crate::config::AttackerPolicy,
    tt_entries: usize,
    /// ONE table, reused across solves. Entries carry the epoch of the
    /// solve that wrote them and read as absent for every later solve, so
    /// nothing crosses positions — the isolation of a fresh table without
    /// the O(entries) clear that made a per-solve table cost more than the
    /// search itself on small positions (gate (c)'s sigma sweep needs
    /// ~145k solves).
    table: SolverTT,
    epoch: u32,
}

impl Solver {
    /// From validated parameters (the config's `validate`).
    pub fn new(
        epsilon: Epsilon,
        tt_entries: usize,
        attacker_policy: crate::config::AttackerPolicy,
    ) -> Solver {
        Solver {
            epsilon,
            attacker_policy,
            tt_entries,
            table: SolverTT::new(tt_entries),
            epoch: 0,
        }
    }

    /// Wholesale reset — what a new game does (design wp18b §1): the table
    /// is rebuilt and the epoch restarted. Memory hygiene and
    /// defence-in-depth; epoch isolation already makes earlier solves read
    /// as absent, so nothing observable rides on this call (stated once,
    /// honestly, in the design's §1).
    pub fn reset(&mut self) {
        self.table = SolverTT::new(self.tt_entries);
        self.epoch = 0;
    }

    /// Solve `state` for the policy game.
    ///
    /// # Panics
    ///
    /// With [`WRONG_POSITION`] when the position is not an ongoing
    /// `Phase::First` position owing two stones, and with the df-pn module's
    /// own named invariants otherwise. Panics are this crate's fail-loud
    /// channel for states its own callers must not reach.
    pub fn solve(&mut self, state: &GameState, node_cap: u64) -> SolveResult {
        self.solve_attacking(state, state.to_move(), node_cap)
    }

    /// The defender direction (design wp18b §2 D2): does the NON-mover
    /// force a policy-game win against the mover's best defense? A thin
    /// wrapper — the attacker is the opponent, the SAME `solve_root`, and
    /// df-pn's own to-move dispatch lands it in the existing AND path
    /// from the first node. Zero df-pn changes.
    pub fn solve_defender(&mut self, state: &GameState, node_cap: u64) -> SolveResult {
        self.solve_attacking(state, state.to_move().opponent(), node_cap)
    }

    fn solve_attacking(
        &mut self,
        state: &GameState,
        attacker: Player,
        node_cap: u64,
    ) -> SolveResult {
        if state.outcome().is_decided()
            || state.phase() != pistol_core::Phase::First
            || state.stones_owed() != 2
        {
            panic!("{WRONG_POSITION}");
        }
        let mut work = state.clone();
        let mut threat = ThreatState::new();
        for (at, player) in work.board().stones() {
            threat.apply(at, player);
        }
        // A new epoch per solve: the reused table's earlier entries go
        // stale the moment this increments, all at once, with no clearing.
        self.epoch = self.epoch.wrapping_add(1);
        let mut dag = ProofDag::default();
        let mut stats = SearchStats::default();
        let value = {
            let mut search = Search::new(
                attacker,
                self.epsilon,
                self.attacker_policy,
                node_cap,
                &mut self.table,
                &mut dag,
                &mut stats,
            );
            search.epoch = self.epoch;
            search.solve_root(&mut work, &mut threat)
        };
        match value {
            Value::Disproven => SolveResult {
                outcome: SolveOutcome::NoWin,
                nodes: stats.nodes,
                seesaw: stats.seesaw,
            },
            Value::Unknown => SolveResult {
                outcome: SolveOutcome::Unknown,
                nodes: stats.nodes,
                seesaw: stats.seesaw,
            },
            Value::Proven => {
                let (tree, zone_ok) = emit(&mut work, &mut threat, attacker, state.key(), &dag);
                if let Some(tree) = tree {
                    if zone_ok {
                        SolveResult {
                            outcome: SolveOutcome::Win(Box::new(tree)),
                            nodes: stats.nodes,
                            seesaw: stats.seesaw,
                        }
                    } else {
                        SolveResult {
                            outcome: SolveOutcome::NoWinUnderZone,
                            nodes: stats.nodes,
                            seesaw: stats.seesaw,
                        }
                    }
                } else {
                    // The root is proven but the DAG lost it — impossible
                    // without a defect, and named rather than guessed at.
                    panic!(
                        "pistol-solver invariant SOLVER_ROOT_UNPROVEN: the root value is Win but the proof DAG has no root node"
                    );
                }
            }
        }
    }
}

/// Walk the proof DAG from the root, replaying the game to re-derive child
/// keys, and emit the witness tree in deterministic order. The second return
/// is the containment tripwire: whether every node's zone cells lie within
/// reach (13) of SOME proof node's stones — the full stone union of the
/// completed walk, checked once after the walk so the verdict cannot depend
/// on DFS visit order (the red-team B-1 fix).
fn emit(
    state: &mut GameState,
    threat: &mut ThreatState,
    attacker: Player,
    root: Key128,
    dag: &ProofDag,
) -> (Option<ProofTree>, bool) {
    let mut walk = Walk {
        dag,
        out: Vec::new(),
        seen: std::collections::BTreeMap::new(),
        zone_ok: true,
        stones: std::collections::BTreeSet::new(),
    };
    emit_node(state, threat, attacker, root, &mut walk);
    if walk.out.is_empty() {
        return (None, walk.zone_ok);
    }
    // The tripwire, per the design's registered invariant: every zone cell
    // of every node within reach (13) of SOME proof node's stones — the
    // completed walk's full union, order-independent by construction.
    for node in &walk.out {
        for at in node.zone.all_cells() {
            let near = walk
                .stones
                .iter()
                .any(|&stone| hex_distance(at, stone) <= 13);
            if !near {
                walk.zone_ok = false;
            }
        }
    }
    (
        Some(ProofTree {
            root,
            nodes: walk.out,
        }),
        walk.zone_ok,
    )
}

/// The walk's mutable state, bundled so the recursion carries one argument
/// where eight would invite ordering mistakes.
struct Walk<'a> {
    dag: &'a ProofDag,
    out: Vec<EmittedNode>,
    seen: std::collections::BTreeMap<Key128, usize>,
    zone_ok: bool,
    /// Every stone on every proof board the walk visits — the "some proof
    /// node" the design's containment invariant names. Checking a node's
    /// zone against ONLY that node's stones would be a stronger claim than
    /// the registered one and would falsely refuse a depth-2 proof whose
    /// descendant zone cells sit beyond 13 of the ancestor's stones.
    stones: std::collections::BTreeSet<pistol_core::Coord>,
}

fn emit_node(
    state: &mut GameState,
    threat: &mut ThreatState,
    attacker: Player,
    key: Key128,
    walk: &mut Walk<'_>,
) {
    if walk.seen.contains_key(&key) {
        return;
    }
    let Some(record) = walk.dag.get(key) else {
        panic!(
            "pistol-solver invariant SOLVER_DAG_GAP: the walk reached a position the search never recorded"
        );
    };
    walk.seen.insert(key, walk.out.len());
    for (stone, _) in state.board().stones() {
        walk.stones.insert(stone);
    }
    // No zone check here: the stones accumulated so far are a DFS-order-
    // dependent subset of the union, and a check against them refuses
    // legitimate trees (the red team's 9917-node reproducer). The one
    // registered check runs in emit(), after the walk completes.
    let mut children = Vec::new();
    match &record.kind {
        ProofKind::OrWinLeaf { .. } | ProofKind::AndOverloadLeaf => {}
        ProofKind::OrStep { witness } => {
            crate::dfpn::apply_turn(state, threat, *witness);
            let child_key = state.key();
            emit_node(state, threat, attacker, child_key, walk);
            crate::dfpn::undo_turn(state, threat, *witness);
            children.push((*witness, child_key));
        }
        ProofKind::AndStep => {
            let mut pairs = Vec::new();
            policy::blocking_pairs(state, threat, attacker, &mut pairs);
            for turn in pairs {
                crate::dfpn::apply_turn(state, threat, turn);
                let child_key = state.key();
                emit_node(state, threat, attacker, child_key, walk);
                crate::dfpn::undo_turn(state, threat, turn);
                children.push((turn, child_key));
            }
        }
    }
    walk.out.push(EmittedNode {
        key,
        kind: record.kind.clone(),
        zone: record.zone.clone(),
        children,
    });
}

/// Axial hex distance.
fn hex_distance(a: pistol_core::Coord, b: pistol_core::Coord) -> u32 {
    let dq = i32::from(a.q) - i32::from(b.q);
    let dr = i32::from(a.r) - i32::from(b.r);
    let ds = dq + dr;
    (dq.unsigned_abs() + dr.unsigned_abs() + ds.unsigned_abs()) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pn::Epsilon;
    use pistol_core::{Coord, Outcome, Turn};

    fn solver() -> Solver {
        Solver::new(
            Epsilon::new(1, 4).unwrap(),
            1024,
            crate::config::AttackerPolicy::BothStonesRelevant,
        )
    }

    fn game_of_turns(turns: &[Turn]) -> GameState {
        let mut state = GameState::new_game();
        let mut iter = turns.iter();
        let first = iter.next().expect("the first turn is P1's single stone");
        state.make_turn(*first).expect("turn 1");
        for turn in iter {
            state.make_turn(*turn).expect("test turn is legal");
        }
        assert_eq!(state.outcome(), Outcome::Ongoing);
        state
    }

    fn pair(cells: &[(i16, i16)]) -> Turn {
        let mut iter = cells.iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        Turn::pair(Coord::new(first.0, first.1), Coord::new(second.0, second.1)).unwrap()
    }

    /// An attacker open four on the mover's turn: the win is immediate
    /// (§2.1's leaf).
    #[test]
    fn an_open_four_for_the_mover_wins_now() {
        let state = game_of_turns(&[
            Turn::Single(Coord::new(0, 0)),
            pair(&[(7, 0), (8, 1)]),
            pair(&[(1, 0), (2, 0)]),
            pair(&[(7, 2), (8, 3)]),
            pair(&[(3, 0), (4, 0)]),
            pair(&[(7, 4), (8, 5)]),
        ]);
        assert_eq!(state.to_move(), pistol_core::Player::P1);
        let mut solver = solver();
        let result = solver.solve(&state, UNCAPPED);
        match &result.outcome {
            SolveOutcome::Win(tree) => {
                assert_eq!(result.nodes, 1, "a win-now leaf is one visit");
                assert_eq!(
                    tree.win_depth_turns(),
                    1,
                    "the win completes on the mover's own turn"
                );
            }
            SolveOutcome::NoWin => panic!("an open four for the mover is a win"),
            SolveOutcome::NoWinUnderZone => panic!("an immediate win has a trivial zone"),
            SolveOutcome::Unknown => panic!("an uncapped solve cannot return Unknown"),
        }
    }

    /// No live window at own >= 2 anywhere for the attacker — every pair of
    /// attacker stones sits more than a window apart — so the policy move
    /// set is empty and the root is a one-visit NoWin leaf.
    #[test]
    fn a_scattered_position_is_nowin() {
        let state = game_of_turns(&[
            Turn::Single(Coord::new(0, 0)),
            pair(&[(0, 8), (0, -8)]),
            pair(&[(8, 0), (-7, 7)]),
            pair(&[(1, 8), (1, -8)]),
        ]);
        assert_eq!(state.to_move(), pistol_core::Player::P1);
        let mut solver = solver();
        let result = solver.solve(&state, UNCAPPED);
        assert_eq!(result.outcome, SolveOutcome::NoWin);
        assert_eq!(result.nodes, 1, "an empty policy set is one visit");
    }

    /// A two-turn win: the attacker's canonical-first threat pair
    /// {(0,1),(0,2)} creates THREE disjoint plan families — the ConstR
    /// column cluster through (0,3),(0,4) reaches four own, the ConstQ
    /// three at r=1 reaches four, the ConstQ three at r=2 reaches four —
    /// so the defender's node is a LAW-OVERLOAD leaf and the root proves
    /// in a handful of visits.
    ///
    /// The attacker is P2, so the origin stone belongs to the DEFENDER and
    /// kills every column window below r=1 — that is what keeps {(0,1),
    /// (0,2)} the two smallest candidate cells. The defender's killer
    /// stones are spread across three columns and two diagonals so they
    /// hold no four of their own: a defender with a completable window at
    /// the AND node would win the race and the proof would not exist.
    #[test]
    fn a_pair_creating_three_disjoint_plans_wins() {
        let plies = [
            // P1 turn 1: the origin, a defender stone.
            Coord::new(0, 0),
            // P2: the ConstQ three at r=1.
            Coord::new(1, 1),
            Coord::new(2, 1),
            // P1: the r=1 and r=2 left killers.
            Coord::new(-1, 1),
            Coord::new(-1, 2),
            // P2: the ConstR column cluster.
            Coord::new(0, 3),
            Coord::new(0, 4),
            // P1: the c=1 and c=2 diagonal killers.
            Coord::new(-2, 3),
            Coord::new(-2, 4),
            // P2: the ConstQ three at r=2.
            Coord::new(2, 2),
            Coord::new(3, 2),
            // P1: the c=3 and c=4 diagonal killers.
            Coord::new(-2, 5),
            Coord::new(1, 3),
            // P2: the rows' right stones.
            Coord::new(5, 1),
            Coord::new(5, 2),
            // P1: the c=3 diagonal killer and a parity filler.
            Coord::new(-1, 4),
            Coord::new(-3, 3),
        ];
        let state = GameState::from_plies(&plies).expect("legal game");
        assert_eq!(state.to_move(), pistol_core::Player::P2);
        let attacker = pistol_core::Player::P2;
        let mut threat = crate::ThreatState::new();
        for (at, player) in state.board().stones() {
            threat.apply(at, player);
        }
        // No hot window on either side at the root: the win is neither
        // immediate nor racy.
        assert!(threat.hot_windows(attacker).is_empty());
        assert!(threat.hot_windows(attacker.opponent()).is_empty());
        let mut candidates = Vec::new();
        crate::policy::candidate_cells(&threat, attacker, &mut candidates);
        assert_eq!(
            &candidates[..2],
            &[Coord::new(0, 1), Coord::new(0, 2)],
            "the winning pair is the canonical-first threat pair"
        );
        let mut solver = solver();
        let result = solver.solve(&state, UNCAPPED);
        match &result.outcome {
            SolveOutcome::Win(tree) => {
                assert!(
                    result.nodes < 20,
                    "a two-turn overload proof is a handful of visits, got {}",
                    result.nodes
                );
                assert_ne!(tree.digest(), 0);
                assert_eq!(
                    tree.win_depth_turns(),
                    2,
                    "one attacker pair (OrStep) plus the overload leaf's completing \
                     turn is two attacker turns"
                );
            }
            other => panic!("three disjoint plans from one pair is a policy win: {other:?}"),
        }
    }

    #[test]
    #[should_panic(expected = "SOLVER_WRONG_POSITION")]
    fn a_mid_turn_position_is_refused() {
        let mut state = game_of_turns(&[Turn::Single(Coord::new(0, 0)), pair(&[(0, 6), (0, 7)])]);
        // Force a mid-turn state by placing one stone of P1's turn.
        state.place(Coord::new(1, 0)).expect("legal ply");
        let mut solver = solver();
        let _ = solver.solve(&state, UNCAPPED);
    }
}
