//! The solver oracle gates (design §7): the differential against R3', the
//! proof-tree re-verification, the RZ property over the σ class, and the
//! TT cross-check. Each gate prints its own PASS/FAIL line; any failure
//! fails the target.
//!
//! The fixture is sha-pinned by its own test; all four gates are
//! `#[ignore]`d here and run in release only, through
//! `tools/solver_oracle_check.sh` with `--ignored` — the tactical gate's
//! split (gate (c) alone is minutes in release and unbounded in debug).

// RULE9-JUSTIFICATION: the four oracle gates are one instrument — they
// share the fixture loaders, the sigma class and the perturb construction,
// and the design's §7 adjudicates them as one gate leg with one exit
// story. Splitting per gate would replicate the loaders and the sigma
// sweep harness four times.
use common::fixture_loader;
use common::r3::Reference;
use common::r3_zone;
use common::verifier::{Verdict, verify};
use pistol_core::{Coord, GameState, Player, Turn};
use pistol_solver::fixture::Expectation;
use pistol_solver::{SolveOutcome, Solver, ZoneP};

mod common;

const FIXTURE: &str = "solver_v0.txt";

/// The gates' parameters come from the committed config — the same file,
/// the same parser, the same values the shipped binary reads (rule 1). A
/// future edit to configs/solver_v0.toml moves the gates with it instead of
/// leaving them on stale literals.
fn gate_params() -> pistol_solver::SolverParams {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../configs/solver_v0.toml");
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("the committed config must read: {error}"));
    let file = pistol_solver::SolverConfigFile::parse(&text)
        .unwrap_or_else(|what| panic!("the committed config must parse: {what}"));
    file.validate()
        .unwrap_or_else(|what| panic!("the committed config must validate: {what:?}"))
}

fn solver() -> Solver {
    let params = gate_params();
    Solver::new(params.epsilon, params.tt_entries)
}

fn tiny_solver() -> Solver {
    let params = gate_params();
    // The 32-entry table is the gate's OWN registered instrument knob, not
    // a solver tunable: the cross-check's subject.
    Solver::new(params.epsilon, 32)
}

/// Gate (a): the differential. Solver value == R3' value == the registered
/// expectation, on every case.
#[test]
#[ignore = "release-only: tools/solver_oracle_check.sh runs all four gates with --ignored (the tactical gate's split, D-54)"]
fn gate_a_differential_matches_the_brute_force_reference() {
    let cases = fixture_loader::load_solver_fixture(FIXTURE);
    let mut solver = solver();
    let mut failures = Vec::new();
    for case in &cases {
        let position = case.position().expect("the loader validated every case");
        let reference = Reference::solve(&position);
        let result = solver.solve(&position);
        let solver_value = match result.outcome {
            SolveOutcome::Win(_) => "win",
            // The registered semantics (§7a): NoWinUnderZone is a MISMATCH
            // that fails the gate, never a nowin — the laundering path the
            // red team named (a false win whose zone overflows printing
            // "nowin" on a nowin-registered case and passing) closes here.
            SolveOutcome::NoWin => "nowin",
            SolveOutcome::NoWinUnderZone => "nowin-under-zone",
        };
        let reference_value = match reference {
            common::r3::RefValue::Win => "win",
            common::r3::RefValue::NoWin => "nowin",
        };
        let expected = match case.expect {
            Expectation::Win => "win",
            Expectation::NoWin => "nowin",
        };
        if solver_value != reference_value || solver_value != expected {
            failures.push(format!(
                "{}: solver {solver_value}, reference {reference_value}, registered {expected}",
                case.name
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "gate (a) FAIL:\n{}",
        failures.join("\n")
    );
    println!("gate (a) PASS: {} cases agree with R3'", cases.len());
}

/// Gate (b): the proof-tree re-verification, on every Win case.
/// The gate's registered wall cap, in the test itself: exceeding it is the
/// named failure VERIFIER-OVERRUN, never a hang. Test-side wall clock only —
/// the solver's choice paths consult nothing but the position.
const VERIFIER_WALL: std::time::Duration = std::time::Duration::from_secs(30 * 60);
const SIGMA_WALL: std::time::Duration = std::time::Duration::from_secs(60 * 60);

#[test]
#[ignore = "release-only: tools/solver_oracle_check.sh runs all four gates with --ignored (the tactical gate's split, D-54)"]
fn gate_b_proof_trees_reverify_full_width() {
    // Bounded AND deep: the verifier is the design's instrument for deep
    // positions, where R3' is measured intractable.
    let mut cases = fixture_loader::load_solver_fixture(FIXTURE);
    cases.extend(fixture_loader::load_deep_fixture());
    let started = std::time::Instant::now();
    let mut solver = solver();
    let mut checked = 0;
    let mut failures = Vec::new();
    for case in &cases {
        if case.expect != Expectation::Win {
            continue;
        }
        let position = case.position().expect("the loader validated every case");
        let result = solver.solve(&position);
        let SolveOutcome::Win(tree) = result.outcome else {
            failures.push(format!("{}: expected a win, got none", case.name));
            continue;
        };
        match verify(&position, &tree) {
            Verdict::Verified { .. } => checked += 1,
            Verdict::Failed(what) => failures.push(format!("{}: {what}", case.name)),
        }
        if started.elapsed() > VERIFIER_WALL {
            panic!("VERIFIER-OVERRUN: gate (b) exceeded its 30-minute wall cap");
        }
    }
    assert!(
        failures.is_empty(),
        "gate (b) FAIL:\n{}",
        failures.join("\n")
    );
    assert!(checked > 0, "the fixture set has Win cases to verify");
    println!("gate (b) PASS: {checked} proof trees re-verified full-width");
}

/// Gate (c): the RZ property. Two legs per Win case — the sequence replay
/// and the adversarial value of `P+σ` — over the registered σ class.
///
/// IMPL-TIME AMENDMENT, recorded in the design: the registered |σ| = 1 and
/// |σ| = 3 samples require pre-placement stones beyond σ itself (a turn
/// places exactly two stones), and the design did not specify those stones'
/// zone class. The implemented class is |σ| = 2 — σ1 outside `Z_1`, σ2
/// outside `Z_2`, the pair one defender turn — which is constructible
/// exactly. |σ| = 1 and |σ| = 3 sampling is licensed-not-scheduled with a
/// filler policy to be specified.
#[test]
#[ignore = "release-only: tools/solver_oracle_check.sh runs all four gates with --ignored (the tactical gate's split, D-54)"]
fn gate_c_relevance_zone_property_holds() {
    // Bounded only: the sigma sweep multiplies each position's solve cost
    // by thousands, and with the deep decoys attached the leg exceeded its
    // registered 60-minute wall cap (62+ min CPU, killed, MEASURED). The
    // deep positions' zones remain covered by gate (b)'s cross-check; the
    // reduction — no sigma replay on the deep trees — is recorded in the
    // design's §9a amendment.
    let cases = fixture_loader::load_solver_fixture(FIXTURE);
    let mut solver = solver();
    let started = std::time::Instant::now();
    let mut checked = 0;
    let mut failures = Vec::new();
    let mut sigma_count = 0u64;
    let mut refused = 0u64;
    for case in &cases {
        if case.expect != Expectation::Win {
            continue;
        }
        let position = case.position().expect("the loader validated every case");
        let result = solver.solve(&position);
        let SolveOutcome::Win(tree) = result.outcome else {
            failures.push(format!("{}: expected a win, got none", case.name));
            continue;
        };
        let Verdict::Verified { zone } = verify(&position, &tree) else {
            failures.push(format!("{}: gate (b) must pass before (c) runs", case.name));
            continue;
        };
        // The proof tree's move cells: attacker fillers must dodge them.
        let mut proof_cells = std::collections::BTreeSet::new();
        for node in &tree.nodes {
            for (turn, _) in &node.children {
                for at in turn_cells(turn) {
                    proof_cells.insert(at);
                }
            }
            if let pistol_solver::ProofKind::OrWinLeaf { witness } = &node.kind {
                for at in witness_cells(witness) {
                    proof_cells.insert(at);
                }
            }
        }
        for sigma in sigma_class(&position, &zone) {
            let Some(perturbed) = perturb(&position, &sigma, &proof_cells) else {
                refused += 1;
                continue;
            };
            sigma_count += 1;
            if started.elapsed() > SIGMA_WALL {
                panic!(
                    "SIGMA-SAMPLE-OVERRUN: gate (c) exceeded its 60-minute wall cap at {sigma_count} placements"
                );
            }
            // (c1): the sequence replay in P+sigma.
            if let Err(what) = replay(&perturbed, &tree) {
                failures.push(format!("{} sigma {sigma:?}: replay: {what}", case.name));
            }
            // (c2): the solver's value on P+sigma is still Win.
            let result = solver.solve(&perturbed);
            if !matches!(result.outcome, SolveOutcome::Win(_)) {
                failures.push(format!(
                    "{} sigma {sigma:?}: the win does not survive the irrelevant placement",
                    case.name
                ));
            }
        }
        checked += 1;
    }
    assert!(
        failures.is_empty(),
        "gate (c) FAIL:\n{}",
        failures.join("\n")
    );
    assert!(checked > 0, "the fixture set has Win cases");
    println!(
        "gate (c) PASS: {checked} wins, {sigma_count} sigma placements replayed and revalued ({refused} refused on collision)"
    );
}
/// Gate (d): the TT cross-check. A 32-entry table returns the same VALUES
/// as the full one, under the registered 50x node cap. The wall cap is
/// named like (b)/(c)'s: a tiny-table solve that cannot return is a named
/// failure, never a silent hang.
const TT_CROSS_WALL: std::time::Duration = std::time::Duration::from_secs(10 * 60);

#[test]
#[ignore = "release-only: tools/solver_oracle_check.sh runs all four gates with --ignored (the tactical gate's split, D-54)"]
fn gate_d_tt_size_does_not_change_values() {
    // BOUNDED ONLY, MEASURED at closure. The red-team vacuity finding
    // (leaf-only bounded cases cannot differ at two table sizes) invited a
    // deep extension; the extension is infeasible in bounded time and was
    // withdrawn: NO deep case returns at a 32-entry table at all (the 8
    // original decoys had no verdict in 300 s; decoy-m0 none in 120 s at
    // every size up to 512 entries; against 0.1 s and ~1 s at the full
    // table — receipts in artifacts/wp18a_tt_knee_v1.txt). The registered
    // 50x node cap can only fire on a solve that RETURNS, so a deep leg
    // here is a hang with an unreachable detector. Gate (b) remains the
    // only multi-node instrument (§9a); the knee ladder's measurements
    // are WP-1.8c's input for any future re-extension at a knee-sized
    // table.
    let cases = fixture_loader::load_solver_fixture(FIXTURE);
    let started = std::time::Instant::now();
    let mut failures = Vec::new();
    for case in &cases {
        let position = case.position().expect("the loader validated every case");
        let mut full_solver = solver();
        let full = full_solver.solve(&position);
        let mut tiny_solver = tiny_solver();
        let tiny = tiny_solver.solve(&position);
        let full_value = matches!(full.outcome, SolveOutcome::Win(_));
        let tiny_value = matches!(tiny.outcome, SolveOutcome::Win(_));
        if full_value != tiny_value {
            failures.push(format!(
                "{}: full TT says {full_value:?}, 32-entry TT says {tiny_value:?}",
                case.name
            ));
            continue;
        }
        if tiny.nodes > 50 * full.nodes.max(1) {
            failures.push(format!(
                "{}: TT-NONTERMINATION: 32-entry run used {} nodes against {} full-run nodes",
                case.name, tiny.nodes, full.nodes
            ));
        }
        if started.elapsed() > TT_CROSS_WALL {
            panic!("TT-CROSS-OVERRUN: gate (d) exceeded its 10-minute wall cap");
        }
    }
    assert!(
        failures.is_empty(),
        "gate (d) FAIL:\n{}",
        failures.join("\n")
    );
    println!("gate (d) PASS: values agree at both table sizes");
}

/// The implemented σ class (see the gate (c) amendment note): ordered pairs
/// (σ1, σ2), ascending, σ1 outside `Z_1`, σ2 outside `Z_2`, both cells in
/// `legal_placements(P)`, capped at the first 5 000 in lexicographic order
/// over ascending pairs.
fn sigma_class(position: &GameState, zone: &ZoneP) -> Vec<Vec<Coord>> {
    let empties: Vec<Coord> = pistol_core::legal_placements(position.board())
        .into_iter()
        .filter(|at| !position.board().is_occupied(*at))
        .collect();
    let mut out = Vec::new();
    'pairs: for (i, &first) in empties.iter().enumerate() {
        if zone.order(0).contains(&first) {
            continue;
        }
        for &second in &empties[i + 1..] {
            if zone.order(1).contains(&second) {
                continue;
            }
            out.push(vec![first, second]);
            if out.len() >= 5000 {
                break 'pairs;
            }
        }
    }
    out
}
fn witness_cells(witness: &pistol_solver::WinWitness) -> [Coord; 2] {
    match *witness {
        pistol_solver::WinWitness::OnePly { at, .. } => [at, at],
        pistol_solver::WinWitness::Pair { first, second, .. } => [first, second],
    }
}

fn turn_cells(turn: &Turn) -> [Coord; 2] {
    match turn {
        Turn::Single(at) => [*at, *at],
        Turn::Pair(first, second) => [*first, *second],
    }
}

/// (c1): replay the witness tree in `P+σ`. Attacker moves must stay legal
/// and activating; defender tree-pairs are applied and a defender win is a
/// named failure; a σ-illegal tree pair is skipped (its line is (c2)'s).
fn replay(state: &GameState, tree: &pistol_solver::ProofTree) -> Result<(), String> {
    let mut state = state.clone();
    let attacker = state.to_move();
    let mut nodes = std::collections::BTreeMap::new();
    for node in &tree.nodes {
        nodes.insert(node.key, node);
    }
    replay_node(&mut state, attacker, tree.root, &nodes, 0)
        .map_err(|what| format!("{what} (depth guard ok)"))
}

fn replay_node(
    state: &mut GameState,
    attacker: Player,
    key: pistol_core::Key128,
    nodes: &std::collections::BTreeMap<pistol_core::Key128, &pistol_solver::EmittedNode>,
    depth: usize,
) -> Result<(), String> {
    if depth > 64 {
        return Err("the replay descended past the fixture class's depth cap".into());
    }
    let node = *nodes
        .get(&key)
        .ok_or_else(|| format!("the tree lacks the node {key:?}"))?;
    match &node.kind {
        pistol_solver::ProofKind::OrWinLeaf { witness } => {
            let turn = match *witness {
                pistol_solver::WinWitness::OnePly { at, .. } => Turn::Single(at),
                pistol_solver::WinWitness::Pair { first, second, .. } => {
                    Turn::pair(first, second).expect("witness cells are distinct")
                }
            };
            state
                .make_turn(turn)
                .map_err(|error| format!("the win leaf's turn no longer applies: {error}"))?;
            match state.outcome() {
                pistol_core::Outcome::Win { winner, .. } if winner == attacker => Ok(()),
                other => Err(format!("the win leaf no longer wins: {other:?}")),
            }
        }
        pistol_solver::ProofKind::AndOverloadLeaf => {
            if r3_zone::defender_wins_now(state, attacker) {
                return Err("DEFENDER-WIN-UNDER-SIGMA at an overload leaf".into());
            }
            let families = r3_zone::plan_families(state.board(), attacker);
            if families.len() < 3 {
                return Err("the overload leaf's plan families did not survive sigma".into());
            }
            Ok(())
        }
        pistol_solver::ProofKind::OrStep { witness } => {
            let mut applied = state.clone();
            applied
                .make_turn(*witness)
                .map_err(|error| format!("the OR step's move is illegal under sigma: {error}"))?;
            let child = node
                .children
                .first()
                .map(|(_, key)| *key)
                .ok_or("an OR step without its child")?;
            replay_node(&mut applied, attacker, child, nodes, depth + 1)?;
            Ok(())
        }
        pistol_solver::ProofKind::AndStep => {
            if r3_zone::defender_wins_now(state, attacker) {
                return Err("DEFENDER-WIN-UNDER-SIGMA at an AND node".into());
            }
            for (turn, child) in &node.children {
                let mut applied = state.clone();
                if applied.make_turn(*turn).is_err() {
                    // sigma occupies a free-stone cell: skipped and counted;
                    // the line is (c2)'s to adjudicate.
                    continue;
                }
                if let pistol_core::Outcome::Win { winner, .. } = applied.outcome() {
                    if winner != attacker {
                        return Err("DEFENDER-WIN-UNDER-SIGMA on a tree pair".into());
                    }
                    unreachable!("the attacker cannot win on the defender's turn");
                }
                replay_node(&mut applied, attacker, *child, nodes, depth + 1)?;
            }
            Ok(())
        }
    }
}

/// `P+σ`: the position with the σ stones pre-placed for the defender,
/// the attacker to move.
///
/// σ is injected as ONE EXTRA DEFENDER TURN inserted before the position's
/// final defender turn; an ATTACKER filler turn (whose two stones dodge the
/// proof tree's move cells, so every proof move stays legal, and whose
/// stones can only help the attacker — LEM-MONO) restores the alternation;
/// the defender's original final turn then plays its own stones. A σ whose
/// cells collide with the history is skipped and counted (the sample the
/// class loses is announced by the count, never silently narrowed).
fn perturb(
    root: &GameState,
    sigma: &[Coord],
    proof_cells: &std::collections::BTreeSet<Coord>,
) -> Option<GameState> {
    let plies: Vec<Coord> = root.played().map(|(at, _)| at).collect();
    // plies.len() is odd (turn 1 is one stone); the last two are the
    // defender's final turn.
    if plies.len() < 3 || sigma.len() != 2 {
        return None;
    }
    let cut = plies.len() - 2;
    let mut state = GameState::new_game();
    for &at in &plies[..cut] {
        state.place(at).expect("the original history replays");
    }
    // The defender's extra turn: σ1, σ2. A collision refuses the sample,
    // and so does a σ cell outside the TRUNCATED board's region: the
    // region at the insertion point is smaller than P's (the final defender
    // turn's stones are not down yet), and a σ cell legal only through
    // those stones cannot be pre-placed before them.
    for &at in sigma {
        if state.board().is_occupied(at) || !state.board().is_legal_placement(at) {
            return None;
        }
        state
            .place(at)
            .expect("sigma cells are empty and in-region");
    }
    // The attacker's filler turn: the two smallest legal empties that are
    // not proof cells and not occupied.
    let mut fillers = Vec::new();
    for at in pistol_core::legal_placements(state.board()) {
        if fillers.len() == 2 {
            break;
        }
        if !state.board().is_occupied(at) && !proof_cells.contains(&at) {
            fillers.push(at);
        }
    }
    assert_eq!(fillers.len(), 2, "the region always holds two spare cells");
    for at in fillers {
        state
            .place(at)
            .expect("the filler was chosen from the legal region");
    }
    // The defender's original final turn.
    for &at in &plies[cut..] {
        if state.board().is_occupied(at) {
            // The defender's own final stone collides with sigma: the
            // sample is refused rather than the position silently altered.
            return None;
        }
        state.place(at).expect("the final turn replays");
    }
    assert_eq!(state.to_move(), root.to_move());
    Some(state)
}
