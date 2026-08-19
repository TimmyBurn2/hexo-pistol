//! D-110's oracle: `HandcraftedV0::delta`'s fast path may disagree with the
//! apply/value/undo roundtrip about NOTHING — not a value, not a panic.
//!
//! Two oracles, on purpose. The PRIMARY is the literal D-110 equation,
//! `apply(c, p); value(p); undo(c, p)`, written out on the SAME eval — no
//! trait dispatch sits between it and the definition, so nothing can mask it.
//! The SECONDARY is [`DefaultDelta`], a wrapper whose `delta` IS the trait's
//! provided default running as real code, which is what pins that the default
//! body itself computes the roundtrip. `DefaultDelta` must never gain a
//! `delta` override: a forwarding override would silently compare the fast
//! path against itself, and every assertion here would go vacuously green
//! ("DefaultDelta gains a forwarding delta" is a named mutant for the
//! mutation round).
//!
//! The adversarial constructions pin the places the fast path could diverge
//! and a random playout would rarely visit: the clamp's start-region x
//! end-region matrix (a fast path computing `clamp(p1_score) + diff` instead
//! of `clamp(p1_score + diff)` dies on the start-saturated cases), windows
//! becoming mixed, the i16 lattice edge where a cell has fewer than
//! [`WINDOWS_PER_CELL`] windows, and the desync parity on a full window.
//!
//! A probe mid-turn is a census-listed READ, not a node answer: ordering
//! scores hypothetical stones at phase-1 nodes too, and D-111's invariant —
//! no static value returned as a node's answer mid-turn — is about answers,
//! which a comparison key never is. The playout below probes at every ply
//! parity for exactly that reason.
//!
//! # RULE9-JUSTIFICATION: one oracle over one equivalence claim (CLAUDE.md
//! rule 9).
//!
//! Every test here asserts the same D-110 equation against the same pair of
//! oracles, through the same probe harness (`assert_probe_matches`, the
//! region machinery, `DefaultDelta` with its never-override warning).
//! Splitting the suite would either duplicate that harness per file or hoist
//! it into the shared scaffolding no other suite uses, and would separate the
//! clamp-matrix constructions from the region helpers whose semantics make
//! them readable. The suite shrinks when D-110's oracle discipline moves into
//! a shared reference for the Stage-2 codebook override, which will want the
//! same harness and is when hoisting pays.

mod common;

use common::committed_weights;
use common::playouts::{Rng, random_ply};
use pistol_core::{Coord, GameState, Player, WIN_LEN};
use pistol_eval::{
    EVAL_DESYNC, EVAL_MAX, Eval, HandcraftedV0, WINDOWS_PER_CELL, Weights, windows_through,
};

/// The literal D-110 equation on the same eval — the unmaskable oracle.
fn roundtrip(eval: &mut HandcraftedV0, at: Coord, player: Player) -> i32 {
    eval.apply(at, player);
    let value = eval.value(player);
    eval.undo(at, player);
    value
}

/// An eval whose `delta` is the trait's PROVIDED DEFAULT: it delegates the
/// three required methods and deliberately does not override the fourth.
///
/// Never give this type a `delta` impl. Its whole reason to exist is that its
/// `delta` resolves to the default body; a forwarding override would turn the
/// fast-vs-default comparisons below into fast-vs-fast.
struct DefaultDelta(HandcraftedV0);

impl Eval for DefaultDelta {
    fn apply(&mut self, at: Coord, player: Player) {
        self.0.apply(at, player);
    }

    fn undo(&mut self, at: Coord, player: Player) {
        self.0.undo(at, player);
    }

    fn value(&self, side_to_move: Player) -> i32 {
        self.0.value(side_to_move)
    }
}

/// Assert fast path == literal roundtrip == provided default, for one probe,
/// and that the probe left the fast-path eval indistinguishable from before.
///
/// `twin` must hold exactly the stones `eval` holds; the assertion at the top
/// is what makes that a checked precondition rather than a hope.
fn assert_probe_matches(
    eval: &mut HandcraftedV0,
    twin: &mut DefaultDelta,
    at: Coord,
    at_step: &str,
) {
    assert_eq!(
        eval, &twin.0,
        "{at_step}: the twin drifted from the eval before any probe"
    );
    for player in [Player::P1, Player::P2] {
        let before = eval.clone();
        let fast = eval.delta(at, player);
        assert_eq!(
            eval, &before,
            "{at_step}: delta({at}, {player}) left a trace on the eval"
        );
        let literal = roundtrip(eval, at, player);
        assert_eq!(
            fast, literal,
            "{at_step}: delta({at}, {player}) disagrees with the literal \
             apply/value/undo roundtrip"
        );
        let default = twin.delta(at, player);
        assert_eq!(
            fast, default,
            "{at_step}: delta({at}, {player}) disagrees with the provided default"
        );
    }
}

/// Where a P1-relative sum sits against the eval band, read off a clamped
/// P1-relative value. `EVAL_MAX` itself is taken as saturated, which every
/// construction below keeps unambiguous by staying well clear of the exact
/// boundary.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Region {
    Below,
    Inside,
    Above,
}

fn region_of(p1_value: i32) -> Region {
    if p1_value >= EVAL_MAX {
        Region::Above
    } else if p1_value <= -EVAL_MAX {
        Region::Below
    } else {
        Region::Inside
    }
}

/// A weight table whose five-count entry sits just under the band, so that a
/// single window's death or birth moves the sum by nearly a whole band width
/// — the lever every clamp-matrix construction below uses.
fn steep_weights() -> Weights {
    Weights::parse(
        r#"
schema_version = 1
backend = "handcrafted_v0"

[table]
1 = 2
2 = 12
3 = 60
4 = 300
5 = 15999
"#,
    )
    .expect("the steep table is valid")
}

/// `len` stones of `player` in consecutive cells along constant r, from
/// `(q, r)` — applied to both evals.
fn place_run(
    eval: &mut HandcraftedV0,
    twin: &mut DefaultDelta,
    q: i16,
    r: i16,
    len: i16,
    player: Player,
) {
    for step in 0..len {
        let at = Coord::new(q + step, r);
        eval.apply(at, player);
        twin.apply(at, player);
    }
}

fn fresh_pair(weights: &Weights) -> (HandcraftedV0, DefaultDelta) {
    (
        HandcraftedV0::new(weights.clone()),
        DefaultDelta(HandcraftedV0::new(weights.clone())),
    )
}

/// One clamp-matrix case: build the stones, check the start region, probe,
/// check the probe agreed with both oracles and ended in the stated region.
fn assert_clamp_case(
    stones: &[(i16, i16, i16, Player)],
    probe: Coord,
    probe_player: Player,
    start: Region,
    end: Region,
    case: &str,
) {
    let weights = steep_weights();
    let (mut eval, mut twin) = fresh_pair(&weights);
    for &(q, r, len, player) in stones {
        place_run(&mut eval, &mut twin, q, r, len, player);
    }
    assert_eq!(
        region_of(eval.value(Player::P1)),
        start,
        "{case}: the construction does not start in the stated region"
    );
    assert_probe_matches(&mut eval, &mut twin, probe, case);
    // The end region is read off the probe's own P1-relative answer: the
    // roundtrip returns the value TO the prober, so P2's answer is negated.
    let answer = eval.delta(probe, probe_player);
    let p1_relative = match probe_player {
        Player::P1 => answer,
        Player::P2 => -answer,
    };
    assert_eq!(
        region_of(p1_relative),
        end,
        "{case}: the probe does not end in the stated region"
    );
}

/// D-110's playout half: >= 1000 random playout steps, the fast path against
/// both oracles on every one, at every ply parity, for both players.
#[test]
fn delta_matches_the_roundtrip_over_seeded_playouts() {
    const PLAYOUTS: u64 = 8;
    const PLIES: usize = 150;
    const REQUIRED_STEPS: usize = 1000;

    let weights = committed_weights();
    let mut steps = 0usize;
    for seed in 1..=PLAYOUTS {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let (mut eval, mut twin) = fresh_pair(&weights);

        while game.board().stone_count() < PLIES && !game.outcome().is_decided() {
            // Probe the cell the playout is ABOUT to fill — a legal, empty,
            // stone-adjacent cell, exactly the shape ordering probes — plus an
            // independently sampled second cell, for both players each.
            let next = random_ply(game.board(), &mut rng);
            let step = format!("seed {seed}, stone {}", game.board().stone_count());
            assert_probe_matches(&mut eval, &mut twin, next, &step);
            let other = random_ply(game.board(), &mut rng);
            if other != next {
                assert_probe_matches(&mut eval, &mut twin, other, &step);
            }
            steps += 1;

            let mover = game.to_move();
            game.place(next).expect("a sampled legal cell");
            eval.apply(next, mover);
            twin.apply(next, mover);
        }
    }
    assert!(
        steps >= REQUIRED_STEPS,
        "the oracle must see at least {REQUIRED_STEPS} playout steps, saw {steps} \
         — a shrunken playout is a shrunken oracle, not a pass"
    );
}

/// The clamp matrix: every reachable (start region, end region) pair over
/// {below, inside, above}, both signs. The start-saturated cases are the ones
/// that kill a fast path clamping in the wrong place — `clamp(p1_score) +
/// diff` agrees with the truth everywhere the start is inside the band.
#[test]
fn delta_matches_where_the_band_saturates() {
    let p1 = Player::P1;
    let p2 = Player::P2;
    let far = Coord::new(200, 200);

    // A five-run's two five-count windows are worth 2 x 15999 alone, so one
    // run saturates a band of 16000; a four-run stays around +1100.
    let five = |q, r, player| (q, r, 5i16, player);
    let four = |q, r, player| (q, r, 4i16, player);

    // inside -> inside: a quiet probe far from a quiet position.
    assert_clamp_case(
        &[four(0, 0, p1)],
        far,
        p1,
        Region::Inside,
        Region::Inside,
        "inside->inside",
    );
    // inside -> above: the stone that completes a five-run.
    assert_clamp_case(
        &[four(0, 0, p1)],
        Coord::new(4, 0),
        p1,
        Region::Inside,
        Region::Above,
        "inside->above",
    );
    // inside -> below: the mirror, in P2's favour.
    assert_clamp_case(
        &[four(0, 0, p2)],
        Coord::new(4, 0),
        p2,
        Region::Inside,
        Region::Below,
        "inside->below",
    );
    // above -> above: a quiet probe far from a saturated position.
    assert_clamp_case(
        &[five(0, 0, p1)],
        far,
        p1,
        Region::Above,
        Region::Above,
        "above->above",
    );
    // above -> inside: a P2 stone in the middle kills both five-count windows,
    // pulling a saturated sum back inside — the case a wrong-place clamp dies on.
    assert_clamp_case(
        &[five(0, 0, p1)],
        Coord::new(2, 0),
        p2,
        Region::Above,
        Region::Inside,
        "above->inside",
    );
    // above -> below: two P1 five-runs with one gap cell make every one of the
    // six windows through the gap five-count (~96k of P1 value dies in one
    // probe), while two isolated P2 five-runs hold the START just above the
    // band instead of far above it.
    assert_clamp_case(
        &[
            five(0, 0, p1),
            five(6, 0, p1),
            five(0, 50, p2),
            five(0, 100, p2),
        ],
        Coord::new(5, 0),
        p2,
        Region::Above,
        Region::Below,
        "above->below",
    );
    // below -> below, below -> inside, below -> above: the colour mirrors.
    assert_clamp_case(
        &[five(0, 0, p2)],
        far,
        p2,
        Region::Below,
        Region::Below,
        "below->below",
    );
    assert_clamp_case(
        &[five(0, 0, p2)],
        Coord::new(2, 0),
        p1,
        Region::Below,
        Region::Inside,
        "below->inside",
    );
    assert_clamp_case(
        &[
            five(0, 0, p2),
            five(6, 0, p2),
            five(0, 50, p1),
            five(0, 100, p1),
        ],
        Coord::new(5, 0),
        p1,
        Region::Below,
        Region::Above,
        "below->above",
    );
}

/// Mixed-window transitions: a stone that DEADENS windows (pure-owner windows
/// gaining the other colour), a stone landing in already-dead windows, and a
/// stone extending pure windows, each against both oracles.
#[test]
fn delta_matches_when_a_window_becomes_mixed() {
    let weights = committed_weights();
    let (mut eval, mut twin) = fresh_pair(&weights);

    // Pure P1 windows around a three-run.
    place_run(&mut eval, &mut twin, 0, 0, 3, Player::P1);
    // A P2 probe inside the run's windows: every pure window it touches dies.
    assert_probe_matches(
        &mut eval,
        &mut twin,
        Coord::new(3, 0),
        "P2 deadens pure-P1 windows",
    );
    // Make some windows actually mixed, then probe INTO the dead zone: the
    // diff through dead windows is zero and must stay zero.
    let killer = Coord::new(1, 1); // shares ConstQ/ConstS windows with the run
    eval.apply(killer, Player::P2);
    twin.apply(killer, Player::P2);
    assert_probe_matches(
        &mut eval,
        &mut twin,
        Coord::new(1, 2),
        "probing an already-mixed zone",
    );
    // And a pure extension away from the mixing, for the positive diff.
    assert_probe_matches(
        &mut eval,
        &mut twin,
        Coord::new(-1, 0),
        "P1 extends its own pure windows",
    );
}

/// At the i16 lattice edge a cell sits in FEWER than [`WINDOWS_PER_CELL`]
/// windows, because windows that would run off the addressable lattice do not
/// exist (`Window::new` says None). Both paths consume the same enumeration,
/// and this pins it where the enumeration actually shrinks.
#[test]
fn delta_matches_at_the_lattice_edge() {
    let weights = committed_weights();

    for corner in [
        Coord::new(i16::MAX - 1, 0),
        Coord::new(i16::MIN + 1, 0),
        Coord::new(0, i16::MAX - 1),
        Coord::new(i16::MAX - 1, i16::MIN + 1),
    ] {
        assert!(
            windows_through(corner).count() < WINDOWS_PER_CELL,
            "{corner} is not actually on the edge — the case would pin nothing"
        );
        let (mut eval, mut twin) = fresh_pair(&weights);
        // A neighbour along constant r, still addressable, so the probe's
        // surviving windows hold a stone and the diffs are not all trivial.
        let neighbour = Coord::new(corner.q.saturating_add(-1).max(i16::MIN + 1), corner.r);
        eval.apply(neighbour, Player::P1);
        twin.apply(neighbour, Player::P1);
        assert_probe_matches(&mut eval, &mut twin, corner, "lattice edge");
    }
}

/// The M1 pin: a delta probe leaves the eval EQUAL to what it was — whole
/// carried state, not just the score — even where the sum is saturated and
/// where the probe cell's windows were never in the map.
#[test]
fn delta_leaves_the_eval_indistinguishable() {
    let weights = steep_weights();
    let (mut eval, mut twin) = fresh_pair(&weights);
    place_run(&mut eval, &mut twin, 0, 0, 5, Player::P1);
    let before = eval.clone();
    // A cell whose windows are all absent from the map: the roundtrip would
    // insert and remove them; the fast path must create nothing.
    eval.delta(Coord::new(500, 500), Player::P2);
    // A cell inside the run's windows, both players.
    eval.delta(Coord::new(2, 0), Player::P2);
    eval.delta(Coord::new(5, 0), Player::P1);
    assert_eq!(eval, before, "a probe left a trace in the carried state");
}

/// Desync parity: probing a cell whose window is already full panics with the
/// same named token `apply` would panic with — the fast path must never turn
/// a broken invariant into a number.
///
/// The eval is NOT reused after the caught panic: the default panics
/// mid-mutation and a read-only fast path panics with state untouched, so
/// post-panic the two are legitimately different objects (D-214).
#[test]
fn delta_desyncs_on_a_full_window_like_apply_would() {
    let weights = committed_weights();
    let full = i16::try_from(WIN_LEN).expect("a window's length fits a step count");

    for prober in [Player::P1, Player::P2] {
        // Six stones fill the window starting at (0,0) along constant r.
        // Mixed on purpose: fullness is about TOTAL stones, not ownership.
        let mut eval = HandcraftedV0::new(weights.clone());
        for step in 0..full {
            let owner = if step % 2 == 0 {
                Player::P1
            } else {
                Player::P2
            };
            eval.apply(Coord::new(step, 0), owner);
        }
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            eval.delta(Coord::new(0, 0), prober)
        }))
        .expect_err("a probe into a full window must refuse, not answer");
        let message = panic
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
            .expect("the panic carries a message");
        assert!(
            message.contains(EVAL_DESYNC),
            "the refusal must carry the {EVAL_DESYNC} token, got: {message}"
        );
    }
}
