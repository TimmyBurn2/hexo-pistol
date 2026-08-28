use pistol_core::{GameState, Player, Turn, generate_turns};

use crate::config::AttackerPolicy;

use crate::state::ThreatState;

/// Asserted by the solver at every AND node: the attacker's last move left a
/// plan, so the node has one to defend against.
pub const NO_PLAN_ASSERT: &str =
    "SOLVER_NO_PLAN: an AND node with no attacker plan is unreachable by construction";

/// The attacker's candidate cells `C`: the empty cells of the attacker's live
/// windows with own ≥ 2 (LAW-SUPPORT k=2) — which, live meaning no defender
/// stone, includes the hot windows' empties at own ≥ 4.
///
/// Every `C`-cell lies in a window that holds an attacker stone, so every
/// `C`-pair is a legal turn under rule 5 without partner-reachability.
pub fn candidate_cells(threat: &ThreatState, attacker: Player, out: &mut Vec<pistol_core::Coord>) {
    // Three separate buffers: every query CLEARS the buffer it is handed
    // (query.rs's own contract), so reusing one across the three calls would
    // drop everything but the last call's cells — a defect this comment
    // exists to keep from coming back.
    out.clear();
    threat.live_cells_at_count(attacker, crate::LiveCount::Two, out);
    let mut threes = Vec::new();
    threat.live_cells_at_count(attacker, crate::LiveCount::Three, &mut threes);
    let mut hot = Vec::new();
    threat.threat_cells(attacker, &mut hot);
    out.extend(threes);
    out.extend(hot);
    out.sort_unstable();
    out.dedup();
}

/// The attacker's threat pairs: the policy's OR-node move set (design
/// wp18b_m4 §2 — the spec; `r3.rs` and `r3_zone.rs` mirror it).
///
/// Under `BothStonesRelevant` (v0): canonical pairs from `C` after which
/// the attacker owns at least one hot window (DEF-PLAN).
///
/// Under `OneFreeStone` (M4): v0's arm A VERBATIM and in v0's order, then
/// arm B appended — pairs `{c, f}` with `c` a raiser (alone it raises a
/// live three to hot) and `f` any empty legal-region cell NOT in `C`.
/// Arm B needs no DEF-PLAN check (the raiser alone creates the plan) and
/// no dedup: its free stone is never a `C`-cell while arm A's pairs are
/// always both-in-`C`, and a two-raiser pair `{c1, c2}` is arm A's to
/// emit (both cells are `C`-cells, and a raiser pair always creates a hot
/// window). This is the design §2 order-and-dedup spec, implemented once.
///
/// The win check has already absorbed every completing pair before this is
/// called (a hot window at the mover's turn is always completable, so
/// `can_win_this_turn` fires first), so no pair here completes six and no
/// pair is missing from `generate_turns`' set.
pub fn threat_pairs(
    state: &GameState,
    threat: &mut ThreatState,
    attacker: Player,
    policy: AttackerPolicy,
    out: &mut Vec<Turn>,
) {
    let mut candidates = Vec::new();
    candidate_cells(threat, attacker, &mut candidates);
    // ARM A'S DEF-PLAN FILTER, AS A PREDICATE (WP-1.8c leg 3). The set and the
    // ORDER are the ones the apply/undo filter emitted — the same `i < j` walk
    // over the same candidates — so df-pn's first-minimum tie-break sees the
    // same sequence. What changes is the test.
    //
    // A pair creates a hot window iff, after it, some live attacker window
    // holds four or more own stones. Own counts never fall, and attacker stones
    // never kill an attacker window's liveness, so from a live window at own k
    // the pair reaches four exactly two ways: k == 3 and the pair supplies one
    // of that window's empties (the `cells_raising_to_hot` class), or k == 2
    // and the pair supplies TWO of them. k <= 1 cannot reach four with two
    // stones. The classes NEST (`sets.rs`), so "hot" here is exactly
    // `own >= 4 && live` and those two routes are all of them.
    //
    // `hot_already` is the general statement for a position that is already
    // hot, where the committed filter passes every pair. A search node past
    // step 1 is never such a position, but this function is public and the
    // three-site agreement test drives it directly on fixture positions that
    // ARE — so the arm is exercised rather than dead.
    out.clear();
    let hot_already = !threat.hot_windows(attacker).is_empty();
    let mut raisers = Vec::new();
    threat.cells_raising_to_hot(attacker, crate::NearHot::Three, &mut raisers);
    raisers.sort_unstable();
    raisers.dedup();
    let mut joint: Vec<(pistol_core::Coord, pistol_core::Coord)> = Vec::new();
    if !hot_already {
        let board = state.board();
        for window in threat.live_windows_at_count(attacker, crate::LiveCount::Two) {
            // A live window holds no defender stone, so its non-own cells are
            // empty and there are exactly WINDOW_LEN - 2 of them.
            let empties: Vec<pistol_core::Coord> = (0..6u8)
                .map(|index| window.cell(index))
                .filter(|&at| !board.is_occupied(at))
                .collect();
            for (index, &first) in empties.iter().enumerate() {
                for &second in &empties[index + 1..] {
                    joint.push((first.min(second), first.max(second)));
                }
            }
        }
        joint.sort_unstable();
        joint.dedup();
    }
    for (index, &first) in candidates.iter().enumerate() {
        for &second in &candidates[index + 1..] {
            let creates = hot_already
                || raisers.binary_search(&first).is_ok()
                || raisers.binary_search(&second).is_ok()
                || joint
                    .binary_search(&(first.min(second), first.max(second)))
                    .is_ok();
            if creates {
                let turn = Turn::pair(first, second).expect("candidate cells are distinct");
                out.push(turn);
            }
        }
    }
    if policy == AttackerPolicy::BothStonesRelevant {
        return;
    }
    // Arm B: raisers ascending, free cells ascending (the design §2 order).
    // The raiser set is the one leg 3 already computed above — same query, same
    // sort, same dedup — so it is reused rather than asked for twice.
    if raisers.is_empty() {
        return;
    }
    // The free cell is any legal empty cell OUTSIDE C (see the doc comment
    // for why outside-C is the dedup-free spelling of the design's union).
    let in_c = |cell: pistol_core::Coord| candidates.binary_search(&cell).is_ok();
    let mut free: Vec<pistol_core::Coord> = pistol_core::legal_placements(state.board())
        .into_iter()
        .filter(|&cell| !in_c(cell))
        .collect();
    free.sort_unstable();
    free.dedup();
    for &raiser in &raisers {
        for &cell in &free {
            let turn = Turn::pair(raiser, cell).expect("a raiser and a legal cell are distinct");
            // Debug-build-only belt-and-braces: the raiser alone keeps a hot
            // window, so the pair preserves the AND-node plan assertion
            // (policy.rs's construction argument). Release builds skip the
            // PROBE COST the design §2 registered as avoided; the always-on
            // backstop is dfpn.rs's NO_PLAN_ASSERT at every AND node.
            #[cfg(debug_assertions)]
            {
                threat.apply(raiser, attacker);
                threat.apply(cell, attacker);
                assert!(
                    !threat.hot_windows(attacker).is_empty(),
                    "a raiser alone creates a hot window, so the pair keeps the plan assertion"
                );
                threat.undo(cell, attacker);
                threat.undo(raiser, attacker);
            }
            out.push(turn);
        }
    }
}

/// Whether the turn's cells jointly hit every attacker hot window's empty
/// set: a cover of the plan family. The blocking predicate (§2.4), computed
/// from the board directly — the specification `blocking_covers` is the fast
/// path for, and a unit test pins their agreement.
pub fn covers_plans(
    state: &GameState,
    threat: &ThreatState,
    attacker: Player,
    turn: &Turn,
) -> bool {
    let board = state.board();
    let cells = turn_cells(turn);
    for window in threat.hot_windows(attacker) {
        let hit = (0..6u8)
            .map(|index| window.cell(index))
            .any(|at| !board.is_occupied(at) && cells.contains(&at));
        if !hit {
            return false;
        }
    }
    true
}

/// The defender's children at an AND node: every legal turn that covers the
/// plan family (§2.4). Single-stone turns do not appear — a winning cell for
/// the defender means the race check already disproved the node, and no
/// other turn places one stone.
pub fn blocking_pairs(
    state: &GameState,
    threat: &ThreatState,
    attacker: Player,
    out: &mut Vec<Turn>,
) {
    out.clear();
    // THE FAST PATH (WP-1.8b §7a's bench-abort fix, semantics-preserving):
    // a covering pair CONTAINS a minimal cover, and a pair is exactly two
    // cells, so the covering-pair set is CONSTRUCTED from the minimal
    // covers — each one-cell cover `a` with every legal partner `x`, each
    // two-cell cover as itself — instead of scanned out of ~10^5 legal
    // pairs with a per-pair window check (the spec form, ~26 ms per AND
    // visit MEASURED at the anchor's t38; this form is the same SET for a
    // small multiple of |minimal covers| x |legal| constructions).
    // `covers_plans` stays as the specification, and the pinned agreement
    // test below now drives BOTH directions of the equivalence.
    let covers = threat.blocking_covers(attacker.opponent(), crate::HitBudget::Two);
    let crate::Cover::Minimal(minimal_covers) = covers else {
        panic!(
            "pistol-solver invariant {COVER_CLASS_MISMATCH}: expected minimal covers, got {covers:?}"
        )
    };
    let legal = pistol_core::legal_placements(state.board());
    for minimal in minimal_covers {
        match minimal {
            crate::MinimalCover::One(at) => {
                for &other in &legal {
                    if other != at {
                        out.push(
                            Turn::pair(at, other).expect("a plan cell and a legal cell differ"),
                        );
                    }
                }
                // The first stone OPENS ITS OWN BALL (movegen's rule-5
                // reading): a partner outside the CURRENT region is a legal
                // second stone iff within LEGAL_RADIUS of `at` — the exact
                // pairs the spec scan emits through its `checked_offset`
                // arm, and the ones a region-only construction misses (the
                // 12-pair divergence the equivalence check caught).
                for dq in -8i16..=8 {
                    for dr in -8i16..=8 {
                        let hex =
                            (dq.unsigned_abs() + dr.unsigned_abs() + (dq + dr).unsigned_abs()) / 2;
                        if hex == 0 || hex > 8 {
                            continue;
                        }
                        if let Some(other) = at.checked_offset(pistol_core::Coord::new(dq, dr))
                            && !state.board().in_legal_region(other)
                        {
                            // A cell outside the region holds no stone (a
                            // stone's own cell is in the region), so there
                            // is no occupancy test to make.
                            out.push(
                                Turn::pair(at, other)
                                    .expect("a plan cell and an opened cell differ"),
                            );
                        }
                    }
                }
            }
            crate::MinimalCover::Two { first, second } => {
                out.push(Turn::pair(first, second).expect("the cover's cells are distinct"));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    // DEDUP, and the equivalence the fast path owes: a sorted, deduplicated
    // construction equals the spec form's scan. The debug build checks it
    // inline (the bench registered the release form); the unit test below
    // pins it permanently.
    #[cfg(debug_assertions)]
    {
        let mut spec = Vec::new();
        for turn in
            generate_turns(state).expect("an AND node is an undecided position at Phase::First")
        {
            if let Turn::Pair(..) = turn
                && covers_plans(state, threat, attacker, &turn)
            {
                spec.push(turn);
            }
        }
        spec.sort_unstable();
        spec.dedup();
        debug_assert!(
            out == &spec,
            "the fast path constructs the spec form's set exactly (|fast| {} vs |spec| {})",
            out.len(),
            spec.len()
        );
    }
}

const COVER_CLASS_MISMATCH: &str = "SOLVER_COVER_CLASS_MISMATCH";

/// Whether the defender can no longer block against the attacker's plans:
/// the exact minimum hitting set over the attacker's hot windows exceeds two
/// stones (LAW-OVERLOAD's arithmetic, RULE-EXACT).
pub fn overload(threat: &ThreatState, attacker: Player) -> bool {
    let windows = threat.hot_windows(attacker);
    !windows.is_empty() && threat.min_hitting_set_exceeds(crate::HitBudget::Two, windows)
}

/// The cells a turn places, canonical order. A single-stone turn's cell is
/// repeated, which is harmless: both entries name the one stone.
pub fn turn_cells(turn: &Turn) -> [pistol_core::Coord; 2] {
    match turn {
        Turn::Single(at) => [*at, *at],
        Turn::Pair(first, second) => [*first, *second],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pistol_core::Coord;

    /// Build a game from an explicit PLAY-ORDER turn list: the first entry
    /// is P1's single origin stone, then pairs in the order they were
    /// played. Colour assignment stays written where it is meant instead of
    /// inferred from ply parity.
    fn game_of_turns(turns: &[Turn]) -> GameState {
        let mut state = GameState::new_game();
        let mut iter = turns.iter();
        let first = iter.next().expect("the first turn is P1's single stone");
        assert!(matches!(first, Turn::Single(at) if *at == pistol_core::Coord::new(0, 0)));
        state
            .make_turn(*first)
            .expect("turn 1 is the origin single");
        for turn in iter {
            state.make_turn(*turn).expect("test turn is legal");
        }
        assert_eq!(state.outcome(), pistol_core::Outcome::Ongoing);
        state
    }

    fn pair(cells: &[(i16, i16)]) -> Turn {
        let mut iter = cells.iter();
        let first = iter.next().expect("a turn has a first stone");
        let second = iter.next().expect("a turn has a second stone");
        Turn::pair(
            pistol_core::Coord::new(first.0, first.1),
            pistol_core::Coord::new(second.0, second.1),
        )
        .expect("cells are distinct")
    }

    fn threat_of(state: &GameState) -> ThreatState {
        let mut threat = ThreatState::new();
        for (at, player) in state.board().stones() {
            threat.apply(at, player);
        }
        threat
    }

    /// Attacker P1 owns a live three (0,0),(1,0),(2,0) on the r=0 line;
    /// the defender's stones sit far off the line. P1 to move.
    fn live_three_position() -> GameState {
        game_of_turns(&[
            Turn::Single(pistol_core::Coord::new(0, 0)),
            pair(&[(6, 0), (6, 1)]),
            pair(&[(1, 0), (5, 4)]),
            pair(&[(6, 2), (6, 3)]),
            pair(&[(2, 0), (5, 5)]),
            pair(&[(5, 6), (5, 7)]),
        ])
    }

    #[test]
    fn candidate_cells_read_live_windows_at_own_two_or_more() {
        let state = live_three_position();
        assert_eq!(state.to_move(), Player::P1);
        let threat = threat_of(&state);
        let mut cells = Vec::new();
        candidate_cells(&threat, Player::P1, &mut cells);
        // The live three's empties are candidates (own 3 >= 2)...
        assert!(cells.contains(&Coord::new(3, 0)));
        assert!(cells.contains(&Coord::new(4, 0)));
        assert!(cells.contains(&Coord::new(5, 0)));
        assert!(cells.contains(&Coord::new(-1, 0)));
        // ...and the defender's column is not (own 0 for the attacker).
        assert!(!cells.contains(&Coord::new(6, 1)));
    }

    #[test]
    fn threat_pairs_create_hot_windows_and_stay_inside_the_candidates() {
        let state = live_three_position();
        let mut threat = threat_of(&state);
        let mut pairs = Vec::new();
        threat_pairs(
            &state,
            &mut threat,
            Player::P1,
            crate::config::AttackerPolicy::BothStonesRelevant,
            &mut pairs,
        );
        assert!(!pairs.is_empty(), "a live three admits threat pairs");
        for turn in &pairs {
            let [first, second] = turn_cells(turn);
            let mut candidates = Vec::new();
            candidate_cells(&threat, Player::P1, &mut candidates);
            assert!(candidates.contains(&first) && candidates.contains(&second));
            threat.apply(first, Player::P1);
            threat.apply(second, Player::P1);
            assert!(!threat.hot_windows(Player::P1).is_empty());
            threat.undo(second, Player::P1);
            threat.undo(first, Player::P1);
        }
        // (4,0)+(5,0) completes the window to five own: a hot window, and a
        // pair the enumeration must find. (3,0)+(4,0) leaves a five-own
        // window (0..5) and a four-own (2..7): hot too.
        assert!(pairs.contains(&pair(&[(4, 0), (5, 0)])));
        assert!(pairs.contains(&pair(&[(3, 0), (4, 0)])));
        // A pair whose cells do not jointly reach any hot window.
        assert!(!pairs.contains(&pair(&[(-1, 0), (6, 8)])));
    }

    /// The open-four shape: FIVE contiguous attacker stones (0,0)..(4,0).
    /// Four hot windows (q=-2..=1); the two five-own ones (q=-1, q=0)
    /// leave singleton disjoint families {(-1,0)} and {(5,0)}, so the
    /// minimum hitting set over all four is exactly 2 and the unique
    /// minimal cover is {(-1,0),(5,0)}. P2 to move.
    fn open_four_position() -> GameState {
        game_of_turns(&[
            Turn::Single(pistol_core::Coord::new(0, 0)),
            pair(&[(0, 6), (0, 7)]),
            pair(&[(1, 0), (2, 0)]),
            pair(&[(0, 8), (0, 9)]),
            pair(&[(3, 0), (4, 0)]),
        ])
    }

    #[test]
    fn blocking_pairs_are_covers_and_match_the_cover_arithmetic() {
        let state = open_four_position();
        assert_eq!(state.to_move(), Player::P2);
        let threat = threat_of(&state);
        // Four hot windows; the two five-own ones have disjoint singleton
        // empties, so no single cell hits every family.
        assert_eq!(threat.hot_windows(Player::P1).len(), 4);
        let mut pairs = Vec::new();
        blocking_pairs(&state, &threat, Player::P1, &mut pairs);
        assert_eq!(
            pairs,
            vec![pair(&[(-1, 0), (5, 0)])],
            "two disjoint singleton families admit exactly one cover"
        );
        // The cover arithmetic agrees: one minimal two-cell cover.
        assert_eq!(
            threat.blocking_covers(Player::P2, crate::HitBudget::Two),
            crate::Cover::Minimal(vec![crate::MinimalCover::Two {
                first: Coord::new(-1, 0),
                second: Coord::new(5, 0)
            }])
        );
    }

    #[test]
    fn one_plan_blocks_through_its_empties_and_the_free_stone() {
        // Attacker stones (0,0),(1,0),(2,0),(4,0): two hot windows —
        // (-1,0)..(4,0) with empties {(-1,0),(3,0)} and (0,0)..(5,0) with
        // empties {(3,0),(5,0)} — sharing the cell (3,0). t=1 via (3,0),
        // so the blocking pairs are every legal pair CONTAINING (3,0)
        // (block-plus-free-stone, the LAW-RIPOSTE range over the legal
        // region) plus the cross pair {(-1,0),(5,0)}.
        let state = game_of_turns(&[
            Turn::Single(pistol_core::Coord::new(0, 0)),
            pair(&[(0, 6), (0, 7)]),
            pair(&[(1, 0), (2, 0)]),
            pair(&[(0, 8), (0, 9)]),
            pair(&[(4, 0), (5, 6)]),
        ]);
        assert_eq!(state.to_move(), Player::P2);
        let threat = threat_of(&state);
        assert_eq!(threat.hot_windows(Player::P1).len(), 2);
        let mut pairs = Vec::new();
        blocking_pairs(&state, &threat, Player::P1, &mut pairs);
        // Block-plus-free-stone: (3,0) with a far free stone.
        assert!(pairs.contains(&pair(&[(3, 0), (6, 5)])));
        assert!(pairs.contains(&pair(&[(3, 0), (-1, 0)])));
        // The cross pair.
        assert!(pairs.contains(&pair(&[(-1, 0), (5, 0)])));
        // A pair blocking only one family is not blocking.
        assert!(!pairs.contains(&pair(&[(-1, 0), (6, 5)])));
        assert!(!pairs.contains(&pair(&[(5, 0), (6, 5)])));
        for turn in &pairs {
            assert!(covers_plans(&state, &threat, Player::P1, turn));
        }
    }

    /// The `hot_already` arm of arm A's predicate, which nothing else gates.
    ///
    /// It is the branch for a position where the attacker ALREADY holds a hot
    /// window, and there the committed apply/undo filter passed every pair —
    /// placing two more attacker stones cannot un-hot a window. A hot attacker
    /// window is also a win THIS turn, so `dfpn_or` answers such a position at
    /// step 1 and never reaches this function; the three-site agreement gate
    /// skips them for the same reason, and the arm exists so `threat_pairs`
    /// stays total on an input its callers do not produce. Dropping it is a
    /// registered mutant, and this is what kills it.
    #[test]
    fn a_hot_position_emits_every_candidate_pair() {
        let state = open_four_position();
        let attacker = Player::P1;
        let mut threat = threat_of(&state);
        assert!(
            !threat.hot_windows(attacker).is_empty(),
            "the open-four shape is hot for P1"
        );
        let mut candidates = Vec::new();
        candidate_cells(&threat, attacker, &mut candidates);
        let mut expected = Vec::new();
        for (index, &first) in candidates.iter().enumerate() {
            for &second in &candidates[index + 1..] {
                expected.push(Turn::pair(first, second).expect("candidates are distinct"));
            }
        }
        let mut emitted = Vec::new();
        threat_pairs(
            &state,
            &mut threat,
            attacker,
            crate::config::AttackerPolicy::BothStonesRelevant,
            &mut emitted,
        );
        assert_eq!(
            emitted, expected,
            "every candidate pair keeps a window that is already hot"
        );
    }

    #[test]
    fn overload_needs_plans_no_two_stones_can_hit() {
        // The open-four position: two singleton families, coverable by one
        // pair. No overload.
        let state = open_four_position();
        let threat = threat_of(&state);
        assert!(!overload(&threat, Player::P1));
    }
}
