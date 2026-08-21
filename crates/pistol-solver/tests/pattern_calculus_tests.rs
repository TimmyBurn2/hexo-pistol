//! V-P3.2 and V-P3.5 — the calculus's §5 pattern table and the two enumerated
//! counterexamples to weight addition, as sha-pinned fixtures with EXACT t
//! (docs/decisions.md D-287).
//!
//! docs/research/threat_calculus_v1.md §11 registers both:
//!
//!   - **V-P3.2** — pattern fixtures = the §5 table; mutation-gate the threat
//!     oracle, and the Gap Trap must be caught;
//!   - **V-P3.5** — the algebra counterexamples as regressions, because
//!     RULE-EXACT says t is COMPUTED and never read off a weight table or
//!     derived by addition.
//!
//! # What pins what
//!
//! [`SECTION_FIVE`] is the calculus's own table, TYPED OUT BY HAND from the
//! document. `tests/fixtures/pattern_v0.txt` is machine-derived from
//! `common::plans`. The two are compared. A derived fixture checked only against
//! its own generator agrees with a broken generator; the hand-written table is
//! the referent that does not share the suspect input.
//!
//! # Three readings, and the ceiling that makes the third necessary
//!
//! `common::plans` (R3) computes t exactly, with no ceiling.
//! `common::reference` (R1) and the shipped `ThreatState` both answer through
//! [`HitBudget`], which is closed at two, so they report t as 0, 1, 2 or
//! "above two" — a LADDER, spelled out in [`TByLadder`]. Every record is
//! checked on all three, and the two ALG- records are the reason the exact one
//! exists: their t is 3, and no budget-shaped query can say so.
//!
//! # RULE9-JUSTIFICATION: one pack, one suite (CLAUDE.md rule 9).
//!
//! Every test reads the same sha-pinned file through the same loader and the
//! same three oracles. The named-behaviour tests below are each a sentence of
//! §5 that a table-shaped oracle gets wrong, and separating them from the
//! whole-table comparison they are read against would leave each of them
//! arguing from a position nothing else in the suite pins.

mod common;

use common::patterns::{
    PATTERN_FIXTURE_FILE, PatternCase, pattern_case, pattern_cases, render_case,
};
use common::plans::{Plan, along, plan_family, plans_by_window, support, threat_number};
use common::reference::Reference;
use common::{assert_pinned, fixture_text, play};
use pistol_core::{Axis, Coord, Player};
use pistol_solver::HitBudget;

/// The SHA-256 of `tests/fixtures/pattern_v0.txt`.
const PATTERN_V0_SHA256: &str = "32ead66b86a77deb83e45f07b2efcbfb5bd5ab7ed7be1c649d3424a65f38e426";

/// docs/research/threat_calculus_v1.md §5 and the two §4 counterexamples, typed
/// out by hand: the record's id and the threat number the DOCUMENT states.
///
/// `PAT-4IFF` is a biconditional, so it is two records: the document's "t = 2
/// **iff** two-deep empty on BOTH sides; else t <= 1". The shallow branch is
/// tabulated at the value the document's `else` admits, and the test asserts the
/// inequality rather than the equality for that one row.
const SECTION_FIVE: [(&str, usize); 12] = [
    ("PAT-O5", 2),
    ("PAT-O4", 2),
    ("PAT-GAP", 2),
    ("PAT-C4", 1),
    ("PAT-4IFF-TWO-DEEP", 2),
    ("PAT-4IFF-SHALLOW", 1),
    ("PAT-RHOMBUS", 0),
    ("PAT-O3", 0),
    // LAW-OVERLOAD's addition floor, enumerated: "crossing fours t=3 != 4;
    // same-line double t=3 != 4".
    ("ALG-CROSS-BOTH", 3),
    ("ALG-SAMELINE-RIGHT", 2),
    ("ALG-SAMELINE-BOTH", 3),
    // The left half of the same-line double is the open four, same stones.
    ("PAT-O4", 2),
];

/// What a budget-shaped oracle can say about t.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TByLadder {
    /// t is exactly this, and this is 0, 1 or 2.
    Exactly(usize),
    /// t is above two; how far above, no budget can ask.
    AboveTwo,
}

impl TByLadder {
    /// The ladder an exact t sits on.
    fn of(exact: usize) -> TByLadder {
        if exact <= 2 {
            TByLadder::Exactly(exact)
        } else {
            TByLadder::AboveTwo
        }
    }

    /// Read the ladder off a `min_hitting_set_exceeds`-shaped predicate.
    ///
    /// MONOTONICITY IS ASSERTED, not assumed: "exceeds one" while "does not
    /// exceed zero" is an oracle contradicting itself, and a ladder read off it
    /// would silently pick one of the two answers.
    fn read(exceeds: impl Fn(HitBudget) -> bool, what: &str) -> TByLadder {
        let (zero, one, two) = (
            exceeds(HitBudget::Zero),
            exceeds(HitBudget::One),
            exceeds(HitBudget::Two),
        );
        assert!(
            zero >= one && one >= two,
            "{what}: exceeds is monotone in the budget; got zero {zero} one {one} two {two}"
        );
        match (zero, one, two) {
            (false, _, _) => TByLadder::Exactly(0),
            (true, false, _) => TByLadder::Exactly(1),
            (true, true, false) => TByLadder::Exactly(2),
            (true, true, true) => TByLadder::AboveTwo,
        }
    }
}

#[test]
fn pattern_v0_fixture_matches_its_pinned_sha256() {
    assert_pinned(PATTERN_FIXTURE_FILE, PATTERN_V0_SHA256);
}

/// THE DERIVATION GUARD (docs/decisions.md D-259's shape). Every `expect` row is
/// what `common::plans` prints for that ply list, so an edited expectation is a
/// red test rather than a re-hash, and an edited `plies` line re-derives and is
/// caught by whichever answer it moved.
#[test]
fn pattern_v0_is_what_the_definitions_print() {
    let mut rendered = String::new();
    for case in pattern_cases() {
        let (game, _) = play(&case.plies);
        let board = game.board();
        rendered.push_str(&render_case(&PatternCase {
            name: case.name.clone(),
            line: case.line,
            plies: case.plies.clone(),
            side: case.side,
            support: support(board, case.side),
            plans: plan_family(board, case.side),
            t: threat_number(&plan_family(board, case.side)),
        }));
        rendered.push('\n');
    }
    let text = fixture_text(PATTERN_FIXTURE_FILE);
    let on_disk: Vec<&str> = text
        .lines()
        .map(|line| line.split('#').next().unwrap_or("").trim_end())
        .filter(|line| !line.trim().is_empty())
        .collect();
    let derived: Vec<&str> = rendered
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    for (index, (derived_line, disk_line)) in derived.iter().zip(&on_disk).enumerate() {
        assert_eq!(
            derived_line,
            disk_line,
            "record line {}: the definitions print one thing and {PATTERN_FIXTURE_FILE} says \
             another; the file is derived, so change the plies, not the expectation",
            index + 1
        );
    }
    assert_eq!(
        derived.len(),
        on_disk.len(),
        "{PATTERN_FIXTURE_FILE} has {} derivable lines and the definitions printed {}",
        on_disk.len(),
        derived.len()
    );
}

/// V-P3.2's whole claim: the pack states the §5 table and nothing else, and the
/// numbers are the document's.
#[test]
fn every_calculus_section_five_pattern_has_its_tabulated_threat_number() {
    let mut named: Vec<&str> = SECTION_FIVE.iter().map(|(name, _)| *name).collect();
    named.sort_unstable();
    named.dedup();
    let mut present: Vec<String> = pattern_cases().into_iter().map(|case| case.name).collect();
    present.sort();
    assert_eq!(
        present, named,
        "the pack and the hand-written §5 table name different records; a pattern the \
         document tabulates and this file does not hold is a row nothing checks"
    );

    for (name, tabulated) in SECTION_FIVE {
        let case = pattern_case(name);
        if name == "PAT-4IFF-SHALLOW" {
            // The document states this branch as an inequality, `t <= 1`.
            assert!(
                case.t <= 1,
                "{name}: the calculus admits t <= 1 on the shallow branch, the pack says {}",
                case.t
            );
            continue;
        }
        assert_eq!(
            case.t, tabulated,
            "{name}: docs/research/threat_calculus_v1.md tabulates t = {tabulated}"
        );
    }
}

/// The shipped state, the from-scratch reference and the exact computation all
/// agree on every record — which is the condition V-P3.2 asks for.
#[test]
fn the_shipped_state_and_the_reference_agree_with_the_exact_threat_number() {
    for case in pattern_cases() {
        let (game, threats) = play(&case.plies);
        let board = game.board();
        let side = case.side;

        // R3, exact and unbounded, against the fixture.
        let exact = threat_number(&plan_family(board, side));
        assert_eq!(exact, case.t, "{}: the exact t moved", case.name);

        // R1, the from-scratch reference, through the budget ladder.
        let reference = Reference::from_board(board);
        let hot = reference.hot(side);
        let by_reference = TByLadder::read(
            |budget| reference.min_hitting_set_exceeds(budget, &hot),
            &format!("{} / reference", case.name),
        );

        // The shipped incremental state, same ladder.
        let by_state = TByLadder::read(
            |budget| threats.min_hitting_set_exceeds(budget, threats.hot_windows(side)),
            &format!("{} / ThreatState", case.name),
        );

        assert_eq!(
            by_reference,
            TByLadder::of(case.t),
            "{}: the from-scratch reference disagrees with the exact t",
            case.name
        );
        assert_eq!(
            by_state, by_reference,
            "{}: the shipped state and the from-scratch reference disagree",
            case.name
        );
    }
}

/// THE GAP TRAP (PAT-GAP, was T7). `X.XXXX` open right holds a five-window
/// whose plan is the SINGLETON at the gap — a cell the defender MUST play — and
/// hitting it is not enough, because the four-window past the gap is still
/// live. A named-shape taxonomy that reads the position as one broken five
/// scores it t=1 and is wrong by a stone.
///
/// THIS IS THE MUTATION GATE V-P3.2 REGISTERS. The mutation checked is the
/// oracle's hot-window predicate narrowed from `own >= 4` to `own == 4`, which
/// is exactly "miss the singleton plan": the five-window leaves the family and
/// the gap singleton with it. Under that mutation the family below loses
/// `{1,0}` and this test goes red. It is checked HERE and not only on t,
/// because t is 2 either way — the singleton is contained in two other plans of
/// this shape, so a test that watched only the number would not have moved.
#[test]
fn gap_trap_singleton_plan_detected() {
    let case = pattern_case("PAT-GAP");
    let (game, threats) = play(&case.plies);
    let board = game.board();
    let gap = Coord::new(1, 0);

    let family = plan_family(board, case.side);
    assert!(
        family.contains(&vec![gap]),
        "the five-window's SINGLETON plan at the gap is missing from the family: {family:?}"
    );

    // The shipped state sees the same window: it is the one holding five own
    // stones and one empty, and that empty is the gap.
    let win_windows = threats.win_in_one_ply_windows(case.side);
    assert_eq!(
        win_windows.len(),
        1,
        "one window holds five own stones and a gap; the state reports {win_windows:?}"
    );
    let mut cells: Vec<Coord> = Vec::new();
    threats.win_in_one_ply_cells(case.side, &mut cells);
    assert_eq!(
        cells,
        vec![gap],
        "the shipped state's own singleton is the gap cell"
    );

    // AND THE FIVE-WINDOW IS IN THE SHIPPED HOT SET. `win_in_one_ply` and `hot`
    // are two sets and the second is what the cover arithmetic reads, so a
    // narrowing of the hot predicate from `own >= 4` to `own == 4` drops the
    // singleton out of the family while leaving the win set untouched. On this
    // shape it does not move t — the singleton is contained in two other plans
    // — so nothing that watched only the number would have seen it.
    assert!(
        threats.hot_windows(case.side).contains(&win_windows[0]),
        "the five-window is a plan too: hot is own >= 4, not own == 4"
    );

    // And the trap itself: hitting the gap does not answer the position.
    assert_eq!(case.t, 2, "the Gap Trap is t=2, not the t=1 a table reads");
    let after_the_gap: Vec<Plan> = family
        .iter()
        .filter(|plan| !plan.contains(&gap))
        .cloned()
        .collect();
    assert_eq!(
        after_the_gap,
        vec![vec![Coord::new(6, 0), Coord::new(7, 0)]],
        "a plan survives the forced block, which is why t is not 1"
    );
}

/// RULE-EXACT, as a regression: t is COMPUTED, never summed. Both enumerated
/// counterexamples, each stated as "the parts say 4 and the whole says 3".
///
/// The crossing case takes its two parts from ONE position, split by axis, so
/// no second position has to be trusted to be the same fours. The same-line
/// case cannot be split that way — its five-windows span both halves, which is
/// the mechanism — so its parts are the two records that hold each half alone.
#[test]
fn threat_number_is_never_additive() {
    // Crossing fours: an open four on ConstR and one on ConstQ meeting at an
    // empty cell that lies in a plan of each.
    let crossing = pattern_case("ALG-CROSS-BOTH");
    let (game, _) = play(&crossing.plies);
    let by_window = plans_by_window(game.board(), crossing.side);
    let on_r = along(Axis::ConstR, &by_window);
    let on_q = along(Axis::ConstQ, &by_window);
    let on_s = along(Axis::ConstS, &by_window);
    assert!(on_s.is_empty(), "the crossing is two axes, not three");
    let (part_r, part_q) = (threat_number(&on_r), threat_number(&on_q));
    assert_eq!(
        (part_r, part_q),
        (2, 2),
        "each four alone is a t=2 open four"
    );
    assert_eq!(
        crossing.t, 3,
        "LEM-CROSS's addition floor is 2+2-1 = 3, and the exact answer meets it"
    );
    assert_ne!(
        crossing.t,
        part_r + part_q,
        "weight addition reads {} for crossing fours; the exact t is {}",
        part_r + part_q,
        crossing.t
    );

    // Same-line double: two open fours on one line sharing the gap between them.
    let whole = pattern_case("ALG-SAMELINE-BOTH");
    let left = pattern_case("PAT-O4");
    let right = pattern_case("ALG-SAMELINE-RIGHT");
    assert_eq!(
        (left.t, right.t),
        (2, 2),
        "each half alone is a t=2 open four"
    );
    assert_eq!(whole.t, 3, "the same-line double is t=3");
    assert_ne!(
        whole.t,
        left.t + right.t,
        "weight addition reads {} for the same-line double; the exact t is {}",
        left.t + right.t,
        whole.t
    );

    // And the left half really is this position's left half: same P1 stones.
    let (whole_game, _) = play(&whole.plies);
    let (left_game, _) = play(&left.plies);
    let mine = |game: &pistol_core::GameState| -> Vec<Coord> {
        let mut cells: Vec<Coord> = game
            .board()
            .stones()
            .filter(|&(_, who)| who == Player::P1)
            .map(|(at, _)| at)
            .filter(|at| at.r == 0 && at.q <= 3)
            .collect();
        cells.sort_unstable();
        cells
    };
    assert_eq!(
        mine(&whole_game),
        mine(&left_game),
        "PAT-O4 is quoted as the left half, so it must hold the same stones there"
    );
}

/// PAT-RHOMBUS: a four-stone diamond has at most two collinear on every axis, so
/// no window holds four and there is NO PLAN AT ALL. The community weight layer
/// scores it W3 (REJ-WSC); its menace is multi-turn generation, not weight.
///
/// The support row is what makes this record falsifiable: `plans -` and `t 0`
/// are true of an empty board too.
#[test]
fn rhombus_has_empty_plan_family() {
    let case = pattern_case("PAT-RHOMBUS");
    let (game, threats) = play(&case.plies);
    let board = game.board();

    assert!(
        plan_family(board, case.side).is_empty(),
        "the rhombus has no plan"
    );
    assert_eq!(case.t, 0, "no plan family, nothing to hit");
    assert_eq!(
        support(board, case.side),
        2,
        "and the reason is the shape: at most two collinear on every axis"
    );
    assert!(
        threats.hot_windows(case.side).is_empty(),
        "the shipped state agrees there is no hot window"
    );

    // The stones ARE there and they ARE a rhombus — otherwise every row above
    // is true of a position with no stones in it.
    let mut mine: Vec<Coord> = board
        .stones()
        .filter(|&(_, who)| who == case.side)
        .map(|(at, _)| at)
        .collect();
    mine.sort_unstable();
    assert_eq!(
        mine,
        vec![
            Coord::new(0, 0),
            Coord::new(0, 1),
            Coord::new(1, 0),
            Coord::new(1, 1)
        ],
        "the diamond this record is about"
    );
}

/// PAT-4IFF, both branches, which is what the calculus's "iff" claims: a
/// contiguous four is t=2 EXACTLY WHEN the empty run on both sides is two deep.
/// The shallow branch differs from the deep one in one opponent stone, one cell
/// further out than the branch a table would look at.
#[test]
fn contiguous_four_is_two_only_when_both_sides_are_two_deep() {
    let deep = pattern_case("PAT-4IFF-TWO-DEEP");
    let shallow = pattern_case("PAT-4IFF-SHALLOW");
    assert_eq!(deep.t, 2, "two-deep on both sides");
    assert_eq!(shallow.t, 1, "one-deep on the left collapses it");
    assert_eq!(
        deep.support, shallow.support,
        "both are contiguous fours; the support does not tell them apart, which is why a \
         length-6 occupancy table cannot either (THM-WINDOW)"
    );

    // The shallow branch's whole family is hit by ONE cell, and that cell is in
    // both plans — the shape a t=2 reading would miss.
    let (game, _) = play(&shallow.plies);
    let family = plan_family(game.board(), shallow.side);
    let shared = Coord::new(4, 0);
    assert!(
        family.iter().all(|plan| plan.contains(&shared)),
        "one cell hits every plan of the shallow four: {family:?}"
    );
}

/// PAT-O5: "plans include both end singletons; both must be hit". Two singleton
/// plans that no single cell can meet is the whole reason an open five is t=2.
#[test]
fn open_five_plans_include_both_end_singletons() {
    let case = pattern_case("PAT-O5");
    let (game, _) = play(&case.plies);
    let family = plan_family(game.board(), case.side);
    let (left, right) = (Coord::new(-1, 0), Coord::new(5, 0));
    assert!(family.contains(&vec![left]), "the left end singleton");
    assert!(family.contains(&vec![right]), "the right end singleton");
    assert_eq!(
        case.t, 2,
        "two disjoint singletons cannot be hit by one cell"
    );
}

/// PAT-O3 GETS THE REFERENT PAT-RHOMBUS ALREADY HAD.
///
/// D-287 claims the all-negative class is closed by construction: "every record
/// carries a `support` row ... so PAT-RHOMBUS (support 2) and PAT-O3 (support 3),
/// whose every other answer is negative, are separated by a POSITIVE claim that
/// an answer-invariant edit cannot leave true", and the fixture header repeats
/// it. That was false for PAT-O3, and a RED-TEAM turned one record into the
/// other: PAT-O3's plies replaced by PAT-RHOMBUS's, re-derived through the
/// pack's own renderer and re-pinned, suite 9 of 9 GREEN — because `support`
/// sits on the DERIVED side of the file and so re-derives WITH the edit rather
/// than standing against it. Only the rhombus's 2 had an independent referent,
/// hard-coded in `rhombus_has_empty_plan_family`, which is exactly why one
/// direction died and the other did not.
///
/// It compounds, and that is why this is not bookkeeping: on the pristine pack,
/// widening the hot threshold dies on PAT-O3 ALONE; with the swap applied first,
/// the same mutation is green. The edit the pack claimed to prevent removed the
/// pack's only gate on the hot threshold's lower side.
#[test]
fn open_three_has_the_support_and_the_stones_this_record_is_about() {
    let case = pattern_case("PAT-O3");
    let (game, threats) = play(&case.plies);
    let board = game.board();

    // The POSITIVE claim, computed live rather than read off the file the edit
    // would have re-derived.
    assert_eq!(
        support(board, case.side),
        3,
        "an open three has three of its own on the axis; a rhombus has two, and \
         that difference is what separates the two all-negative records"
    );
    assert!(
        plan_family(board, case.side).is_empty(),
        "and it is still short of a plan family"
    );
    assert_eq!(case.t, 0, "no plan family, nothing to hit");
    assert!(
        threats.hot_windows(case.side).is_empty(),
        "the shipped state agrees there is no hot window"
    );

    // The stones ARE there and they ARE collinear — otherwise every row above is
    // true of a position holding a rhombus, which is the swap this closes.
    let mut mine: Vec<Coord> = board
        .stones()
        .filter(|&(_, who)| who == case.side)
        .map(|(at, _)| at)
        .collect();
    mine.sort_unstable();
    assert_eq!(
        mine,
        vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)],
        "three in a row on one axis, which no rhombus is"
    );
}

/// THE RIGHT HALF IS A DIFFERENT HALF FROM THE LEFT.
///
/// ALG-SAMELINE-RIGHT's whole role in V-P3.5 is TO BE THE RIGHT HALF of the
/// same-line double, and it was pinned by a single integer: `t = 2`. A RED-TEAM
/// swapped its plies for PAT-O4's — making the "right half" a copy of the LEFT
/// half — re-derived and re-pinned, and the suite stayed 9 of 9 green, with
/// `threat_number_is_never_additive` then asserting "each half alone is a t=2
/// open four" about the same half twice. PAT-O4 carries a stone-identity
/// assertion that would have caught this; its sibling, one guard away, did not.
#[test]
fn the_same_line_double_has_two_halves_and_they_are_not_the_same_half() {
    let left = pattern_case("PAT-O4");
    let right = pattern_case("ALG-SAMELINE-RIGHT");

    let (left_game, _) = play(&left.plies);
    let (right_game, _) = play(&right.plies);

    let own = |board: &pistol_core::Board, side: Player| -> Vec<Coord> {
        let mut found: Vec<Coord> = board
            .stones()
            .filter(|&(_, who)| who == side)
            .map(|(at, _)| at)
            .collect();
        found.sort_unstable();
        found
    };
    let left_stones = own(left_game.board(), left.side);
    let right_stones = own(right_game.board(), right.side);

    assert_ne!(
        left_stones, right_stones,
        "the two halves of the double are different positions; if they are the \
         same, the additivity counterexample is one half counted twice"
    );
    // And each half is exactly where its name says. Both carry the origin
    // stone, turn 1 being one stone at the origin by rule 3, so the halves are
    // told apart by the RUN and not by the whole stone set.
    assert_eq!(
        left_stones,
        vec![
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0)
        ],
        "the left half is the run at q = 0..3 on r = 0"
    );
    assert_eq!(
        right_stones,
        vec![
            Coord::new(0, 0),
            Coord::new(5, 0),
            Coord::new(6, 0),
            Coord::new(7, 0),
            Coord::new(8, 0)
        ],
        "the right half is the run at q = 5..8, which is what makes it the RIGHT \
         half rather than a second copy of the left"
    );
    assert_eq!(right.t, 2, "and it is a t=2 open four alone");
}

/// PAT-C4's §5 NOTE SAYS "SINGLE PLAN", AND NOTHING CHECKED IT.
///
/// The record was pinned by `t = 1` and by nothing else, and was swapped whole
/// for PAT-4IFF-SHALLOW's plies with the suite green — at which point its
/// `plans` row silently went from one plan to two and the pack held two
/// byte-identical positions under different names, with §5's "closed four ...
/// single plan" false on the record carrying it.
#[test]
fn a_closed_four_has_exactly_one_plan() {
    let case = pattern_case("PAT-C4");
    let (game, _) = play(&case.plies);
    let family = plan_family(game.board(), case.side);
    assert_eq!(
        family.len(),
        1,
        "a closed four is blocked on one side, so exactly one run can still \
         complete it — that is what makes it t=1: {family:?}"
    );
    assert_eq!(case.t, 1, "one plan, one cell to hit");
}
