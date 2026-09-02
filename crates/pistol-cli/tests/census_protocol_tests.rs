mod common;

use common::{engine, talk};
use pistol_core::{Coord, GameState, Turn, canonical_key};

/// The armed seat. Committed, already a CI gate seat, and armed for the reason
/// this suite needs: by `docs/experiments/wp20b_design.md` F3 a census row
/// cannot exist on a gate-off seat, so a test that needs one and takes an OFF
/// seat passes while measuring nothing.
const ARMED: &str = "configs/gate_staged_solver_v0.toml";
/// The gate-OFF seat. Exactly one test below wants it: the ABSENCE of rows is
/// what that one pins.
const UNARMED: &str = "configs/instrument_v0.toml";

/// The budget of the shared armed run, and it has THREE jobs.
///
/// It must complete more than one depth — a single-depth search cannot witness
/// a per-depth re-emission. It must fire at the ROOT, whose key is the one this
/// suite can derive an external referent for. And it must fire IN-TREE, because
/// the in-tree site is a second, separate piece of code and a suite that reads
/// only root rows leaves it unread — which a REVIEW measured, by making the
/// in-tree site emit `GameState::key` (the matrix's option A, the identity F2
/// rules out) and watching 45 tests pass.
///
/// MEASURED at this seat on the position below: `depth_turns 2` completes two
/// depths and fires ONCE, at the root — which is why it is not the budget.
/// `nodes 4000` completes two depths and fires five times, four of them
/// in-tree, for 4 695 nodes.
const BUDGET: &str = "go nodes 4000";
/// One completed depth, for the two runs that need only a firing to have
/// happened. 607 nodes, MEASURED.
const CHEAP: &str = "go depth_turns 1";

/// `bench_positions_v1.txt`'s first band-15 entry, stone for stone — the
/// position `trigger_census_cover_tests` reads, so the wire is exercised on a
/// COMMITTED workload rather than on one chosen to make a property hold. Its
/// root fires the trigger with the two detector directions differing, which is
/// what lets one row pin that they are separate fields.
const BENCH_B15_FIRST: &[&str] = &[
    "0,0", "-1,1", "1,0", "0,1", "0,2", "-1,0", "1,-1", "0,-1", "1,-2", "0,-2", "0,3", "-1,-1",
    "1,1", "-1,2", "-1,3",
];

/// A `position start moves …` line for a cell sequence: one stone on turn 1,
/// pairs after it. Built through [`Turn`] so the pair tokens carry the one
/// spelling docs/decisions.md D-5 canonicalises them to.
fn position_line(cells: &[&str]) -> String {
    let coords: Vec<Coord> = cells
        .iter()
        .map(|cell| cell.parse().expect("a coordinate"))
        .collect();
    let (first, rest) = coords.split_first().expect("a game has a first stone");
    let mut line = format!("position start moves {}", Turn::single(*first));
    for pair in rest.chunks(2) {
        let turn = match pair {
            [a, b] => Turn::pair(*a, *b).expect("two distinct cells are a turn"),
            [a] => Turn::single(*a),
            _ => unreachable!("chunks(2) yields one or two"),
        };
        line.push(' ');
        line.push_str(&turn.to_string());
    }
    line
}

/// What the engine says to one `go` on the named seat, standing on the fixture.
fn answers(config: &str, go: &str) -> Vec<String> {
    let mut engine = engine(config);
    talk(&mut engine, &[&position_line(BENCH_B15_FIRST), go])
}

/// THE ONE ARMED CENSUS RUN, shared by every test below that reads a row.
///
/// One workload, many readings. An armed search costs seconds of solver time
/// per node in a test build, and one run per assertion would be a gate nobody
/// runs — so the run happens once and each test asserts one property of it.
fn armed_census() -> &'static [String] {
    static ONCE: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| answers(ARMED, &format!("{BUDGET} census")))
}

#[test]
fn the_shared_run_fires_at_the_root_and_in_the_tree() {
    // THE PREMISE EVERY SEATED TEST BELOW STANDS ON, asserted rather than
    // assumed. The census has two firing sites and they are separate code; a
    // workload that reaches only one of them leaves the other unread, and every
    // test that walks these rows then pins half of what it claims to.
    let rows = rows(armed_census());
    let depths: Vec<&str> = rows
        .iter()
        .map(|row| value(row, "turns_from_root"))
        .collect();
    assert!(
        depths.contains(&"0"),
        "the shared run never fired at the ROOT: {depths:?}"
    );
    assert!(
        depths.iter().any(|d| *d != "0"),
        "the shared run never fired IN-TREE, so `pvs.rs`'s site is read by nothing: {depths:?}"
    );
}

/// Every census row among some answers.
fn rows(answers: &[String]) -> Vec<&String> {
    answers
        .iter()
        .filter(|line| line.starts_with("info census "))
        .collect()
}

/// The field NAMES of one census row, in the order they appear.
fn names(row: &str) -> Vec<&str> {
    row.split_whitespace().skip(2).step_by(2).collect()
}

/// The value of one named field of a census row.
fn value<'a>(row: &'a str, name: &str) -> &'a str {
    let words: Vec<&'a str> = row.split_whitespace().collect();
    let at = words
        .iter()
        .position(|word| *word == name)
        .unwrap_or_else(|| panic!("no `{name}` field in `{row}`"));
    words
        .get(at + 1)
        .unwrap_or_else(|| panic!("`{name}` is the last word of `{row}`"))
}

/// The canonical key of the fixture position, derived from pistol-core alone.
///
/// The externally derived referent this suite is judged against: it shares no
/// code path with the search that wrote the row.
fn fixture_key() -> String {
    let mut state = GameState::new_game();
    for cell in BENCH_B15_FIRST {
        state
            .place(cell.parse().expect("a coordinate"))
            .expect("a legal stone");
    }
    canonical_key(&state.board().stones().collect::<Vec<_>>()).to_string()
}

#[test]
fn a_census_row_carries_the_canonical_key_of_the_position_it_fired_at() {
    let rows = rows(armed_census());
    assert!(!rows.is_empty(), "this position fires the trigger");
    let root = rows
        .iter()
        .find(|row| value(row, "turns_from_root") == "0")
        .expect("the root fires the trigger at this position");
    assert_eq!(
        value(root, "key"),
        fixture_key(),
        "the root's row carries the key of the position the `position` line named"
    );
    for row in &rows {
        let key = value(row, "key");
        assert_eq!(key.len(), 32, "a 128-bit key is 32 hex digits: `{row}`");
        assert!(
            key.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
            "a key is lower-case hex: `{row}`"
        );
    }
}

/// A game reaching exactly [`BENCH_B15_FIRST`]'s position by a different route.
///
/// P2's turn 2 and turn 4 exchange one stone each — `1,0` and `-1,0` swap turns
/// — so both sides hold the same cells in the end and no stone changes colour,
/// which is what makes this a TRANSPOSITION rather than a different position.
/// A key over the GAME tells the two apart; the census's key may not.
const B15_TRANSPOSED: &[&str] = &[
    "0,0", "-1,1", "-1,0", "0,1", "0,2", "1,0", "1,-1", "0,-1", "1,-2", "0,-2", "0,3", "-1,-1",
    "1,1", "-1,2", "-1,3",
];

#[test]
fn two_move_orders_reaching_one_position_share_a_census_key_on_the_wire() {
    // THE FIXTURE THE DISPATCH ASKS FOR, read at the seam that matters. The
    // pure-function statement lives in pistol-core; this one pins that the row
    // the ENGINE writes is keyed on the position and not on the game that
    // reached it — which is what an identity read from `played()` in play order
    // would get wrong, at a site no pistol-core test can see.
    let one = answers(ARMED, &format!("{CHEAP} census"));
    let mut engine = engine(ARMED);
    let other = talk(
        &mut engine,
        &[&position_line(B15_TRANSPOSED), &format!("{CHEAP} census")],
    );
    let root = |said: &[String]| {
        rows(said)
            .into_iter()
            .find(|row| value(row, "turns_from_root") == "0")
            .map(|row| value(row, "key").to_string())
            .expect("the root fires the trigger at this position")
    };
    assert_eq!(
        root(&one),
        root(&other),
        "two games reaching one position wrote two different census keys"
    );
    assert_eq!(root(&one), fixture_key());
}

#[test]
fn without_the_token_a_go_line_writes_no_census_byte() {
    let said = answers(ARMED, BUDGET);
    assert!(
        said.iter().any(|line| line.starts_with("info totals ")),
        "the search still ran and still answered: {said:?}"
    );
    assert!(
        !said.iter().any(|line| line.contains("census")),
        "the word `census` reached the wire off the token: {said:?}"
    );
}

#[test]
fn a_search_with_the_solver_gate_off_produces_no_census_row_under_the_token() {
    // A LIMITATION, pinned so a successor meets it in the suite rather than
    // after a months-long sweep: the census fires where the solver does, and
    // the seat the production sweep runs has the gate off (wp20b_design.md F3).
    let said = answers(UNARMED, &format!("{BUDGET} census"));
    assert!(
        said.iter().any(|line| line.starts_with("info totals ")),
        "the search still ran and still answered: {said:?}"
    );
    assert!(
        rows(&said).is_empty(),
        "an unarmed seat produced a census row, so the census reaches past the solver wiring"
    );
}

#[test]
fn a_third_word_that_is_not_the_census_token_is_refused_naming_it() {
    let said = answers(ARMED, "go nodes 2000 censuss");
    let refusal = said
        .iter()
        .find(|line| line.starts_with("error "))
        .unwrap_or_else(|| panic!("no refusal among {said:?}"));
    assert!(
        refusal.contains("censuss"),
        "the refusal names the third word: `{refusal}`"
    );
    assert!(
        refusal.contains("`go` takes one budget, and"),
        "a third word keeps the refusal it has always had: `{refusal}`"
    );
}

#[test]
fn a_fourth_word_on_a_go_line_is_refused_naming_the_fourth_word() {
    // The EXACT text, pinned: a refusal that named the token instead would
    // send a driver to fix a word that is not the one that broke the line.
    let said = answers(ARMED, "go nodes 2000 census extra");
    let refusal = said
        .iter()
        .find(|line| line.starts_with("error "))
        .unwrap_or_else(|| panic!("no refusal among {said:?}"));
    assert!(
        refusal.contains(
            "`go` takes one budget and at most the `census` token, and `extra` follows them"
        ),
        "`{refusal}`"
    );
}

#[test]
fn the_census_line_field_order_is_the_documented_one() {
    for row in rows(armed_census()) {
        assert_eq!(
            names(row),
            vec![
                "key",
                "turns_from_root",
                "mover_hot",
                "opponent_hot",
                "mover_win_in_one_ply",
                "opponent_win_in_one_ply",
                "mover_live_three",
                "opponent_live_three",
                "cover",
                "cover_count",
                "attacker_visits",
                "attacker_proved",
                "defender_visits",
                "defender_proved",
            ],
            "`{row}`"
        );
        assert_eq!(
            row.split_whitespace().count(),
            2 + 14 * 2,
            "every value is ONE word, so the line holds exactly two per field: `{row}`"
        );
    }
}

#[test]
fn a_census_row_spells_an_unasked_defender_rather_than_omitting_it() {
    for row in rows(armed_census()) {
        assert!(
            names(row).contains(&"defender_visits") && names(row).contains(&"defender_proved"),
            "both defender fields are present on every row: `{row}`"
        );
        assert_eq!(
            value(row, "defender_visits") == "-",
            value(row, "defender_proved") == "-",
            "the defender pair is absent together or present together: `{row}`"
        );
    }
}

#[test]
fn a_census_row_spells_each_direction_as_its_own_field() {
    // docs/decisions.md D-512 and D-535: the two directions are different
    // quantities on both axes, and a row shipping a sum would put back a
    // denominator correction that had to be made twice.
    let rows = rows(armed_census());
    let root = rows
        .iter()
        .find(|row| value(row, "turns_from_root") == "0")
        .expect("the root fires the trigger at this position");
    assert_ne!(
        value(root, "mover_hot"),
        value(root, "opponent_hot"),
        "this fixture's root is hot for ONE side, which is what tells the two apart: `{root}`"
    );
    assert_ne!(
        value(root, "attacker_visits"),
        value(root, "defender_visits"),
        "the two solver directions spent different visits here: `{root}`"
    );
}

#[test]
fn a_firing_produces_one_census_line_however_many_depths_completed() {
    // The per-depth re-emission this pins against would multiply the wire by
    // the iteration count, and an internal row set that is right cannot see it.
    let said = armed_census();
    let depths = said
        .iter()
        .filter(|line| line.starts_with("info depth_turns "))
        .count();
    assert!(
        depths >= 2,
        "the search completed more than one depth: {depths}"
    );
    let firings: u64 = said
        .iter()
        .find(|line| line.starts_with("info totals "))
        .map(|line| value(line, "solver_firings").parse().expect("a count"))
        .expect("the armed seat prints its call counters");
    assert_eq!(
        rows(said).len() as u64,
        firings,
        "one line per firing, over {depths} completed depths"
    );
}

#[test]
fn the_census_block_is_emitted_after_the_last_depth_and_before_the_totals_line() {
    let said = armed_census();
    let kind = |line: &String| {
        if line.starts_with("info totals ") {
            2
        } else if line.starts_with("info census ") {
            1
        } else if line.starts_with("info ") {
            0
        } else {
            3
        }
    };
    let order: Vec<u8> = said.iter().map(kind).collect();
    assert!(
        order.windows(2).all(|pair| pair[0] <= pair[1]),
        "per-depth reports, then the census block, then totals, then bestmove: {said:?}"
    );
    assert!(order.contains(&1), "there is a census block at all");
    assert_eq!(order.last(), Some(&3), "bestmove is last");
}

#[test]
fn a_census_go_takes_its_rows_before_it_disarms() {
    // Ordering, pinned by its consequence: `take_trigger_census` panics once
    // the collector has been stopped, so a disarm ordered first ends this in a
    // panic rather than in a wrong answer.
    assert!(
        !rows(armed_census()).is_empty(),
        "a census `go` came back with no rows, which is also what a stop-before-take looks like \
         if it did not panic"
    );
}

#[test]
fn a_plain_go_after_a_census_go_computes_no_key_and_emits_no_line() {
    // The disarm. A plain `go` answers with no rows either way — the rows only
    // reach a `SearchOutcome` when they were asked for — so the FOLD is the
    // whole observable difference and the counter is what sees it.
    let mut instance = engine(ARMED);
    let position = position_line(BENCH_B15_FIRST);
    let said = talk(&mut instance, &[&position, &format!("{CHEAP} census")]);
    let folds = instance.census_fold_entries();
    assert_eq!(
        folds,
        rows(&said).len() as u64,
        "the census `go` folded once per row"
    );
    assert!(folds > 0, "the census `go` folded at all");
    let plain = talk(&mut instance, &[&position, CHEAP]);
    assert!(rows(&plain).is_empty());
    assert_eq!(
        instance.census_fold_entries(),
        folds,
        "the `go` after a census `go` was still paying the fold, so the disarm never ran"
    );
}
