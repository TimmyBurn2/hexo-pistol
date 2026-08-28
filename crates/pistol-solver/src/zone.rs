// RULE9-JUSTIFICATION: the zone sequence, its two contribution sources
// (EP-1's active-segment grading, T3-1's threat cells) and the grading
// index math are one arithmetic — the design's §3 semantics and the
// gate-(b) verifier both read the same add_graded order mapping, and a
// split would duplicate the k-mapping the oracle cross-check pins.
use std::collections::BTreeSet;

use pistol_core::window::{Window, windows_through};
use pistol_core::{Board, Coord, Player, legal_placements};

use crate::state::ThreatState;

/// How many orders the sequence carries. The config's `zone_orders` is
/// validated against this constant: v0 implements exactly three, and a
/// different registered number is a named error rather than a silent
/// reinterpretation.
pub const ZONE_ORDERS: usize = 3;

/// The farthest a zone cell can sit from the legal region that produced it:
/// a window's cells lie within `WINDOW_LEN − 1` steps of any cell of the
/// window that intersects the region.
const ZONE_REACH: u32 = 5;

/// A relevance zone sequence `Z_1 ⊆ Z_2 ⊆ Z_3`.
///
/// The subset invariant is maintained by every operation (unions of
/// conforming sequences conform; every graded addition adds to a suffix of
/// the orders) and asserted by [`ZoneP::assert_invariants`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ZoneP {
    orders: [BTreeSet<Coord>; ZONE_ORDERS],
}

impl ZoneP {
    /// The empty sequence.
    pub fn new() -> ZoneP {
        ZoneP {
            orders: Default::default(),
        }
    }

    /// `Z_i` (0-indexed: order 0 is the paper's `Z_1`).
    pub fn order(&self, index: usize) -> &BTreeSet<Coord> {
        &self.orders[index]
    }

    /// Cells in every order — AT-1's move cells, T3-1's threat cells.
    pub fn add_all_orders(&mut self, cells: impl IntoIterator<Item = Coord>) {
        for at in cells {
            for order in &mut self.orders {
                order.insert(at);
            }
        }
    }

    /// EP-1's grading: a k-empty defender active segment's empties enter
    /// `Z_k..Z_3` — k = 1 to orders 0.., k = 2 to orders 1.., k ≥ 3 to
    /// order 2 only.
    pub fn add_graded(&mut self, cells: impl IntoIterator<Item = Coord>, k: u32) {
        let first = match k {
            1 => 0,
            2 => 1,
            _ => ZONE_ORDERS.saturating_sub(1),
        };
        for at in cells {
            for order in &mut self.orders[first..] {
                order.insert(at);
            }
        }
    }

    /// DT-1's propagation: elementwise union.
    pub fn union_with(&mut self, other: &ZoneP) {
        for (own, theirs) in self.orders.iter_mut().zip(&other.orders) {
            own.extend(theirs.iter().copied());
        }
    }

    /// Whether every cell of every order is in the corresponding order of
    /// `other` — the containment the fail-loud post-solve invariant checks.
    pub fn is_subset_of(&self, other: &ZoneP) -> bool {
        self.orders
            .iter()
            .zip(&other.orders)
            .all(|(own, theirs)| own.is_subset(theirs))
    }

    /// Every cell in the widest order, sorted.
    pub fn all_cells(&self) -> Vec<Coord> {
        self.orders[ZONE_ORDERS - 1].iter().copied().collect()
    }

    /// Whether the cell lies within `ZONE_REACH` of some stone on `board` —
    /// the per-node tripwire half of the post-solve invariant.
    pub fn cell_within_reach(&self, at: Coord, board: &Board) -> bool {
        board
            .stones()
            .any(|(stone, _)| hex_distance(at, stone) <= LEGAL_RADIUS_PLUS_REACH)
    }

    /// Assert the sequence invariants: order i ⊆ order i+1. Called by the
    /// solver after construction and by the tests.
    ///
    /// # Panics
    ///
    /// Named, if the subset chain is broken.
    pub fn assert_invariants(&self) {
        for pair in self.orders.windows(2) {
            if !pair[0].is_subset(&pair[1]) {
                panic!("pistol-solver invariant ZONE_ORDER_BROKEN: Z_i is not a subset of Z_i+1");
            }
        }
    }
}

/// The paper's ZONE_REACH plus rule 5's radius: the maximum distance from a
/// stone to a zone cell that region-bounded EP-1 scan can produce.
const LEGAL_RADIUS_PLUS_REACH: u32 = 8 + ZONE_REACH;

/// Axial hex distance: (|dq| + |dr| + |dq+dr|) / 2.
fn hex_distance(a: Coord, b: Coord) -> u32 {
    let dq = i32::from(a.q) - i32::from(b.q);
    let dr = i32::from(a.r) - i32::from(b.r);
    let ds = dq + dr;
    (dq.unsigned_abs() + dr.unsigned_abs() + ds.unsigned_abs()) / 2
}

/// EP-1's contribution at one position: the graded empties of every defender
/// active segment intersecting the legal region.
///
/// A defender active segment is a 6-window holding no attacker stones (Wu &
/// Lin §III-A); its unoccupied squares are graded by their count. Windows are
/// found through the region's cells — every stone and every empty legal
/// placement — so a window anywhere in the region is found exactly once.
pub fn ep1_contribution(board: &Board, attacker: Player) -> ZoneP {
    let mut zone = ZoneP::new();
    let mut seen: BTreeSet<Window> = BTreeSet::new();
    let mut scan_through = |at: Coord, zone: &mut ZoneP| {
        for window in windows_through(at) {
            if !seen.insert(window) {
                continue;
            }
            let mut empties = Vec::new();
            let mut defender = 0u32;
            for index in 0..6u8 {
                match board.get(window.cell(index)) {
                    None => empties.push(window.cell(index)),
                    Some(player) if player == attacker => {
                        // An attacker stone kills the segment: not active for
                        // the defender.
                        empties.clear();
                        break;
                    }
                    Some(_) => defender += 1,
                }
            }
            if !empties.is_empty() {
                zone.add_graded(empties, 6 - defender);
            }
        }
    };
    for (at, _) in board.stones() {
        scan_through(at, &mut zone);
    }
    for at in legal_placements(board) {
        scan_through(at, &mut zone);
    }
    zone
}

/// T3-1's contribution at one position: the attacker's hot-window empties, in
/// every order.
pub fn t31_contribution(threat: &ThreatState, attacker: Player) -> ZoneP {
    let mut cells = Vec::new();
    threat.threat_cells(attacker, &mut cells);
    let mut zone = ZoneP::new();
    zone.add_all_orders(cells);
    zone
}

#[cfg(test)]
mod tests {
    use super::*;

    use pistol_core::GameState;

    /// Place a whole test game and return the state. Panics on any illegality:
    /// a test position that cannot be reached is a broken test, not a skip.
    fn game(plies: &[Coord]) -> GameState {
        let mut state = GameState::new_game();
        for &at in plies {
            state.place(at).expect("test game is legal");
        }
        assert_eq!(
            state.outcome(),
            pistol_core::Outcome::Ongoing,
            "test game must not be decided"
        );
        state
    }

    #[test]
    fn all_orders_reaches_every_order() {
        let mut zone = ZoneP::new();
        zone.add_all_orders([Coord::new(0, 0)]);
        for order in 0..ZONE_ORDERS {
            assert!(zone.order(order).contains(&Coord::new(0, 0)));
        }
    }

    #[test]
    fn grading_follows_the_paper() {
        let mut zone = ZoneP::new();
        // k = 1 enters every order; k = 2 from order 1; k = 5 collapses to
        // the last order only.
        zone.add_graded([Coord::new(1, 0)], 1);
        zone.add_graded([Coord::new(2, 0)], 2);
        zone.add_graded([Coord::new(3, 0), Coord::new(3, 1)], 5);
        assert!(zone.order(0).contains(&Coord::new(1, 0)));
        assert!(!zone.order(0).contains(&Coord::new(2, 0)));
        assert!(zone.order(1).contains(&Coord::new(2, 0)));
        assert!(zone.order(1).contains(&Coord::new(1, 0)));
        assert!(!zone.order(0).contains(&Coord::new(3, 0)));
        assert!(!zone.order(1).contains(&Coord::new(3, 0)));
        assert!(zone.order(2).contains(&Coord::new(3, 0)));
        zone.assert_invariants();
    }

    #[test]
    fn union_is_elementwise_and_keeps_the_chain() {
        let mut a = ZoneP::new();
        a.add_graded([Coord::new(0, 0)], 1);
        let mut b = ZoneP::new();
        b.add_graded([Coord::new(5, 5)], 2);
        a.union_with(&b);
        assert!(a.order(0).contains(&Coord::new(0, 0)));
        assert!(!a.order(0).contains(&Coord::new(5, 5)));
        assert!(a.order(1).contains(&Coord::new(5, 5)));
        assert!(a.is_subset_of(&a.clone()));
        a.assert_invariants();
    }

    /// The EP-1 grading, one line, three defender formations at k = 1, 2, 3.
    ///
    /// Attacker P1 holds stones off the r=2 line (the origin among them);
    /// the defender P2 holds (1,2)..(5,2) (five-own), (8,2)..(11,2)
    /// (four-own) and (14,2)..(16,2) (three-own) on it. No run reaches
    /// six, so the game stays ongoing. 23 plies, P1 to move.
    const EP1_GAME: [Coord; 23] = [
        Coord::new(0, 0), // P1 turn 1 (the origin, rule 3)
        Coord::new(1, 2), // P2
        Coord::new(2, 2),
        Coord::new(0, 4), // P1
        Coord::new(0, 5),
        Coord::new(3, 2), // P2
        Coord::new(4, 2),
        Coord::new(2, 4), // P1
        Coord::new(2, 5),
        Coord::new(5, 2), // P2
        Coord::new(8, 2),
        Coord::new(4, 4), // P1
        Coord::new(4, 5),
        Coord::new(9, 2), // P2
        Coord::new(10, 2),
        Coord::new(6, 4), // P1
        Coord::new(6, 5),
        Coord::new(11, 2), // P2
        Coord::new(14, 2),
        Coord::new(8, 4), // P1
        Coord::new(8, 5),
        Coord::new(15, 2), // P2
        Coord::new(16, 2),
    ];

    #[test]
    fn ep1_grades_by_defender_count_in_one_line() {
        let state = game(&EP1_GAME);
        assert_eq!(state.to_move(), pistol_core::Player::P1);
        let zone = ep1_contribution(state.board(), pistol_core::Player::P1);
        zone.assert_invariants();
        // Five-own window q=0..=5 at r=2: its one empty (0,2) is k=1 ->
        // every order.
        assert!(
            zone.order(0).contains(&Coord::new(0, 2)),
            "k=1 empties reach Z_1"
        );
        // Four-own window q=8..=13 at r=2: its empties 12,13 are k=2 ->
        // orders 1.. only.
        assert!(
            !zone.order(0).contains(&Coord::new(12, 2)),
            "k=2 empties stay out of Z_1"
        );
        assert!(zone.order(1).contains(&Coord::new(12, 2)));
        assert!(zone.order(1).contains(&Coord::new(13, 2)));
        // Three-own window q=14..=19 at r=2: its empties 17,18,19 are k=3 ->
        // the last order only.
        for q in 17..=19 {
            assert!(
                !zone.order(1).contains(&Coord::new(q, 2)),
                "k=3 empty {q} must stay out of Z_2"
            );
            assert!(
                zone.order(2).contains(&Coord::new(q, 2)),
                "k=3 empty {q} must reach Z_3"
            );
        }
        // An empty window near the stones is an active segment with k=6 ->
        // Z_3.
        assert!(
            zone.order(2).contains(&Coord::new(0, 1)),
            "an empty window's cells reach Z_3"
        );
    }

    #[test]
    fn ep1_kills_segments_the_attacker_occupies() {
        // The same defender five-own formation, (1,2)..(5,2); the two boards
        // differ only in whether one of the attacker's pair stones sits on
        // the formation's k=1 empty (0,2).
        let open = [
            Coord::new(0, 0), // P1 turn 1
            Coord::new(1, 2), // P2
            Coord::new(2, 2),
            Coord::new(0, 4), // P1
            Coord::new(0, 5),
            Coord::new(3, 2), // P2
            Coord::new(4, 2),
            Coord::new(2, 4), // P1
            Coord::new(2, 5),
            Coord::new(5, 2), // P2
            Coord::new(0, 8),
        ];
        let killed = [
            Coord::new(0, 0), // P1 turn 1
            Coord::new(1, 2), // P2
            Coord::new(2, 2),
            Coord::new(0, 2), // P1: ON the k=1 empty, killing the window
            Coord::new(0, 4),
            Coord::new(3, 2), // P2
            Coord::new(4, 2),
            Coord::new(2, 4), // P1
            Coord::new(2, 5),
            Coord::new(5, 2), // P2
            Coord::new(0, 8),
        ];
        let open = game(&open);
        let killed = game(&killed);
        let zone_open = ep1_contribution(open.board(), pistol_core::Player::P1);
        let zone_killed = ep1_contribution(killed.board(), pistol_core::Player::P1);
        // With the window open, its single empty (0,2) grades k=1; the
        // sibling window (1,2)..(6,2) is open in both and grades its own
        // empty (6,2) k=1.
        assert!(zone_open.order(0).contains(&Coord::new(0, 2)));
        assert!(zone_killed.order(0).contains(&Coord::new(6, 2)));
        // With the attacker on (0,2), the window is not a defender active
        // segment: (0,2) reaches no Z_1 (its ConstR window holds the
        // attacker's origin stone too).
        assert!(!zone_killed.order(0).contains(&Coord::new(0, 2)));
        zone_open.assert_invariants();
        zone_killed.assert_invariants();
    }

    #[test]
    fn window_grading_is_axis_agnostic() {
        // The five-own shape along ConstR (q fixed, r varying) grades k=1
        // exactly as the ConstQ line does: three axes everywhere.
        let plies = [
            Coord::new(0, 0), // P1 turn 1
            Coord::new(2, 1), // P2
            Coord::new(2, 2),
            Coord::new(0, 4), // P1
            Coord::new(0, 5),
            Coord::new(2, 3), // P2
            Coord::new(2, 4),
            Coord::new(4, 4), // P1
            Coord::new(4, 5),
            Coord::new(2, 5), // P2
            Coord::new(0, 8),
        ];
        let state = game(&plies);
        let zone = ep1_contribution(state.board(), pistol_core::Player::P1);
        zone.assert_invariants();
        assert!(
            zone.order(0).contains(&Coord::new(2, 0)),
            "the ConstR five-own window's empty grades k=1"
        );
    }
}
