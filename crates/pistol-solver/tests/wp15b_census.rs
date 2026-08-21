//! WP-1.5b's population census: the instrument behind every number in
//! `docs/experiments/wp15b_design.md` §6.2, §6.3 and §12.4.
//!
//! # Why this is committed rather than run from a worktree
//!
//! CLAUDE.md: "An artefact that produces a registered number — a `tools/`
//! script, a scratchpad harness, or a command block the document prints — is
//! named in the pre-registration WITH ITS REVISION, and a change to it reopens
//! the review exactly as an amendment to the document does." Revisions 1 to 3 of
//! the design ran this from a throwaway worktree and cited it as if it were
//! named; a REVIEW-design found that no such artefact existed at any revision
//! (`git ls-files` names only the design document). This file is that artefact.
//!
//! It is the opposite case from `common::plans`, which D-287 keeps test-tree-only
//! BECAUSE it records no number. This one records numbers, so D-287's own clause
//! — "promotion is a future ADR owed the day anything records from it" — is what
//! the design's ADR line answers.
//!
//! # `#[ignore]`, and why that is not a way of hiding it
//!
//! The census is a MEASUREMENT and not a gate: it asserts no threshold and can
//! fail no run, so putting it in CI would spend time to check nothing. What CI
//! does check is `wp15b_census_reproduces_the_registered_populations` below,
//! which is NOT ignored: it re-derives the handful of numbers the design's
//! matrices actually rest on and compares them against the values TYPED OUT here
//! by hand from the document. That is D-259's discipline — a derived number
//! checked only against its own generator agrees with a broken generator — and
//! it is what makes an edit to either side a red test rather than a silent
//! drift.
//!
//! # RULE9-JUSTIFICATION: the instrument and the numbers it is registered against
//! belong in one file (CLAUDE.md rule 9).
//!
//! Splitting the sampler from the pinning test would put the registered values
//! in a different file from the code that derives them, which is the exact
//! arrangement D-259 forbids for a derived fixture: a number checked only
//! against its own generator agrees with a broken generator, and the check is
//! only worth anything while an edit to either side is visible in one diff. The
//! three sampling regimes are likewise one decision — which population a
//! reported number came from — and separating them would let a caller mix them,
//! which is the defect a REVIEW-design found in the design's own cost column.
//!
//! Run the full census with:
//!   cargo test -p pistol-solver --release --test wp15b_census -- --ignored --nocapture

mod common;

use std::collections::BTreeSet;

use common::playouts::{Rng, random_ply};
use pistol_core::window::Window;
use pistol_core::{Board, Coord, GameState, Phase, Player};
use pistol_solver::{Cover, HitBudget, LiveCount, StonesLeft, ThreatState};

/// The registered corpus: the same 24 positions `tools/baseline_snapshot.sh`
/// measures, read from the sha-pinned fixture rather than restated.
const CORPUS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pistol-cli/tests/fixtures/bench_positions_v1.txt"
);

/// The design's committed quiet cut, so the staged-set column is the set the
/// config actually produces and not a number chosen here.
const QUIET_TOP_K: usize = 16;

/// Which live-window counts a Tier-T option qualifies, per side. The THRESHOLD
/// reading of `docs/experiments/wp15b_design.md` §6.1: a count of `n` means
/// "at least `n` own stones", which is `LiveCount` at `n..=3` UNIONED with the
/// hot set, since `LiveCount` is closed at `{Two, Three}` (D-255) and cannot
/// express `>= 4` on its own.
#[derive(Clone, Copy)]
struct TierOption {
    name: &'static str,
    own: u8,
    opponent: u8,
}

const OPTIONS: [TierOption; 3] = [
    TierOption {
        name: "A",
        own: 3,
        opponent: 3,
    },
    TierOption {
        name: "B",
        own: 2,
        opponent: 2,
    },
    TierOption {
        name: "C",
        own: 2,
        opponent: 3,
    },
];

fn plies_of(line: &str) -> Vec<Coord> {
    let body = line.split_once(" #").map_or(line, |(head, _)| head);
    body.strip_prefix("start moves ")
        .expect("a corpus line is a `start moves` tail")
        .split_whitespace()
        .flat_map(|token| token.split('/'))
        .map(|cell| cell.parse::<Coord>().expect("a corpus coordinate"))
        .collect()
}

fn corpus() -> Vec<Vec<Coord>> {
    let text = std::fs::read_to_string(CORPUS).expect("the sha-pinned bench corpus");
    text.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("start moves "))
        .map(plies_of)
        .collect()
}

/// The empty cells of `window`, read from the masks because `empty_cells` is
/// crate-private by D-261 and this is another crate's test tree.
fn empties(state: &ThreatState, window: Window) -> impl Iterator<Item = Coord> {
    let masks = state.masks(window);
    let occupied = masks.p1 | masks.p2;
    window
        .cells()
        .into_iter()
        .enumerate()
        .filter(move |(index, _)| occupied & (1u8 << index) == 0)
        .map(|(_, cell)| cell)
}

/// Every empty cell of `side`'s live windows holding EXACTLY `count` stones.
///
/// Not what the design commits — it commits the threshold reading below — but the
/// number that killed revision 1 of the design's Tier-T option was the DIFFERENCE
/// between the two readings, and a number cited in a document needs an instrument
/// whether or not it is the adopted one.
fn tier_cells_exact(state: &ThreatState, side: Player, count: u8) -> BTreeSet<Coord> {
    let class = if count == 2 {
        LiveCount::Two
    } else {
        LiveCount::Three
    };
    let mut out = BTreeSet::new();
    for &window in state.live_windows_at_count(side, class) {
        out.extend(empties(state, window));
    }
    out
}

/// Every empty cell of `side`'s live windows holding AT LEAST `count` stones —
/// the threshold reading, hot windows included.
fn tier_cells(state: &ThreatState, side: Player, count: u8) -> BTreeSet<Coord> {
    let mut out = BTreeSet::new();
    let mut windows: Vec<Window> = Vec::new();
    if count <= 2 {
        windows.extend(state.live_windows_at_count(side, LiveCount::Two));
    }
    if count <= 3 {
        windows.extend(state.live_windows_at_count(side, LiveCount::Three));
    }
    // `>= 4` is the hot set, which `LiveCount` cannot name (D-255).
    windows.extend(state.hot_windows(side));
    for window in windows {
        out.extend(empties(state, window));
    }
    out
}

/// The radius-`radius` ball intersected with the rules' own legality answer,
/// asked one cell at a time (D-77: the two radii are never compared).
fn ball(board: &Board, radius: u32) -> BTreeSet<Coord> {
    let reach = i16::try_from(radius).expect("a representable radius");
    let mut cells = BTreeSet::new();
    for (stone, _) in board.stones() {
        for dq in -reach..=reach {
            for dr in -reach..=reach {
                let delta = Coord::new(dq, dr);
                if Coord::ORIGIN.distance(delta) <= radius
                    && let Some(cell) = stone.checked_offset(delta)
                    && board.is_legal_placement(cell)
                {
                    cells.insert(cell);
                }
            }
        }
    }
    cells
}

/// One position's census row.
struct Row {
    hot_us: usize,
    hot_them: usize,
    live2_us: usize,
    live3_us: usize,
    live2_them: usize,
    live3_them: usize,
    ball2: usize,
    filtered: bool,
    impossible: bool,
    win_now: bool,
    /// The emitted candidate count under each option, in `OPTIONS` order.
    staged: [usize; 3],
    /// Tier T alone under each option, ungated by the filter.
    tier_t: [usize; 3],
    /// The same under the EXACT-count reading the design does NOT commit — the
    /// difference between the two is what killed revision 1's Tier-T option.
    tier_t_exact: [usize; 3],
    /// Tier-T cells lying outside the radius-2 ball, under each option.
    outside_ball: [usize; 3],
}

/// What the staged generator emits at this position, per
/// `docs/experiments/wp15b_design.md` §5.3.
fn census(state: &GameState, threats: &ThreatState) -> Option<Row> {
    if state.outcome().is_decided() || state.phase() != Phase::First {
        return None;
    }
    let us = state.to_move();
    let them = us.opponent();
    let left = StonesLeft::from_state(state)?;
    let budget = HitBudget::from(left);

    let win_now = threats.can_win_this_turn(us, left).is_some();
    let cover = threats.blocking_covers(us, budget);
    let filtered = matches!(cover, Cover::Minimal(_));
    let impossible = matches!(cover, Cover::Impossible);
    let pool = ball(state.board(), 2);

    let mut staged = [0usize; 3];
    let mut tier_t = [0usize; 3];
    let mut tier_t_exact = [0usize; 3];
    let mut outside_ball = [0usize; 3];
    for (slot, option) in OPTIONS.iter().enumerate() {
        let mut t = tier_cells(threats, us, option.own);
        t.extend(tier_cells(threats, them, option.opponent));
        tier_t[slot] = t.len();
        let mut e = tier_cells_exact(threats, us, option.own);
        e.extend(tier_cells_exact(threats, them, option.opponent));
        tier_t_exact[slot] = e.len();
        outside_ball[slot] = t.iter().filter(|cell| !pool.contains(cell)).count();

        staged[slot] = if win_now {
            // §5.3's WIN-NOW row: exactly the win-now class, nothing else.
            let mut cells: BTreeSet<Coord> = BTreeSet::new();
            let mut buf = Vec::new();
            threats.win_in_one_ply_cells(us, &mut buf);
            cells.extend(buf.iter().copied());
            if left == StonesLeft::Two {
                for &window in threats.hot_windows(us) {
                    if threats.masks(window).own_count(us) == 4 {
                        cells.extend(empties(threats, window));
                    }
                }
            }
            cells.len()
        } else if filtered {
            // §5.3's `Minimal` row: the cover union, and nothing below it.
            cover.cells().len()
        } else {
            // The batched rows. Tier T whole, then the quiet cut.
            let quiet = pool.iter().filter(|cell| !t.contains(cell)).count();
            t.len() + quiet.min(QUIET_TOP_K)
        };
    }

    Some(Row {
        hot_us: threats.hot_windows(us).len(),
        hot_them: threats.hot_windows(them).len(),
        live2_us: threats.live_windows_at_count(us, LiveCount::Two).len(),
        live3_us: threats.live_windows_at_count(us, LiveCount::Three).len(),
        live2_them: threats.live_windows_at_count(them, LiveCount::Two).len(),
        live3_them: threats.live_windows_at_count(them, LiveCount::Three).len(),
        ball2: pool.len(),
        filtered,
        impossible,
        win_now,
        staged,
        tier_t,
        tier_t_exact,
        outside_ball,
    })
}

fn replay(plies: &[Coord]) -> (GameState, ThreatState) {
    let mut game = GameState::new_game();
    let mut threats = ThreatState::new();
    for (index, &at) in plies.iter().enumerate() {
        let mover = game.to_move();
        game.place(at)
            .unwrap_or_else(|error| panic!("ply {index} at {at}: {error}"));
        threats.apply(at, mover);
    }
    (game, threats)
}

/// The three sampling regimes the design reports, named here so a reader knows
/// which one produced a number.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Regime {
    /// The 24 corpus roots, as they are.
    Roots,
    /// Each root carried up to three turns deeper by draws from the RADIUS-2
    /// ball — the policy the search actually uses. This is the REPORTED regime.
    DeepenRadius2,
    /// The same, drawn from the radius-8 legal region. SUPERSEDED: it inflates
    /// the ball by the sampler rather than by depth, and it is retained only so
    /// the design's sampler-sensitivity number stays re-derivable.
    DeepenRadius8,
    /// Long uniform playouts from an empty board, for the tail the corpus does
    /// not reach. Not a search distribution and not offered as one; it is the
    /// regime that shows what the staged set does when the board fills up.
    Playouts,
}

fn sample(regime: Regime) -> Vec<Row> {
    let mut rows = Vec::new();
    if regime == Regime::Playouts {
        for seed in 1..=24u64 {
            let mut rng = Rng::new(seed);
            let mut game = GameState::new_game();
            let mut threats = ThreatState::new();
            for _ in 0..80 {
                if game.outcome().is_decided() {
                    break;
                }
                let at = random_ply(game.board(), &mut rng);
                let mover = game.to_move();
                game.place(at).expect("a sampled ply is legal");
                threats.apply(at, mover);
                rows.extend(census(&game, &threats));
            }
        }
        return rows;
    }
    for plies in corpus() {
        let (game, threats) = replay(&plies);
        if regime == Regime::Roots {
            rows.extend(census(&game, &threats));
            continue;
        }
        for seed in 1..=8u64 {
            let mut rng = Rng::new(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ plies.len() as u64);
            let mut game = game.clone();
            let mut threats = threats.clone();
            for _ in 0..6 {
                if game.outcome().is_decided() {
                    break;
                }
                let at = match regime {
                    Regime::DeepenRadius8 => random_ply(game.board(), &mut rng),
                    _ => {
                        let pool: Vec<Coord> = ball(game.board(), 2).into_iter().collect();
                        pool[rng.below(pool.len())]
                    }
                };
                let mover = game.to_move();
                game.place(at).expect("a sampled ply is legal");
                threats.apply(at, mover);
                rows.extend(census(&game, &threats));
            }
        }
    }
    rows
}

fn mean(rows: &[Row], of: impl Fn(&Row) -> f64) -> f64 {
    if rows.is_empty() {
        return 0.0;
    }
    rows.iter().map(&of).sum::<f64>() / rows.len() as f64
}

/// The canonical markdown the design document must carry verbatim.
///
/// ONE renderer, so a number exists in one place. `wp15b_design.md` carries this
/// text between the two markers below and restates none of it elsewhere; the
/// test underneath compares the two byte for byte, which is D-259's discipline
/// applied to a design table rather than to a fixture — an edited number is a red
/// test rather than a drift a reviewer has to find.
///
/// Nine times across four revisions of that document, a repair moved a number
/// here and left a copy of it somewhere else. That is why this exists.
pub const BEGIN: &str =
    "<!-- BEGIN CENSUS TABLE — rendered by crates/pistol-solver/tests/wp15b_census.rs -->";
pub const END: &str = "<!-- END CENSUS TABLE -->";

fn cell(rows: &[Row], of: impl Fn(&Row) -> f64) -> String {
    format!("{:.4}", mean(rows, of))
}

/// One named sampling regime and the rows it produced. A named type because the
/// tuple is passed as a slice and clippy's `type_complexity` is right that
/// `&[(&str, Vec<Row>)]` reads as noise at a call site.
type Regimes<'a> = [(&'a str, Vec<Row>)];

/// A row label and the field it reads. Named for the same reason `Regimes` is.
type Quantity = (&'static str, fn(&Row) -> f64);

fn render(regimes: &Regimes<'_>) -> String {
    let mut out = String::new();
    out.push_str("| quantity |");
    for (name, _) in regimes {
        out.push_str(&format!(" {name} |"));
    }
    out.push_str("\n|---|");
    for _ in regimes {
        out.push_str("---|");
    }
    out.push('\n');

    let quantities: [Quantity; 8] = [
        ("own hot, mean", |r| r.hot_us as f64),
        ("opponent hot, mean", |r| r.hot_them as f64),
        ("live-2 own", |r| r.live2_us as f64),
        ("live-2 opponent", |r| r.live2_them as f64),
        ("live-3 own", |r| r.live3_us as f64),
        ("live-3 opponent", |r| r.live3_them as f64),
        ("radius-2 ball", |r| r.ball2 as f64),
        ("cover union when FILTERED", |r| {
            if r.filtered {
                r.staged[2] as f64
            } else {
                f64::NAN
            }
        }),
    ];
    for (label, of) in quantities {
        if label.starts_with("cover union") {
            out.push_str(&format!("| {label} |"));
            for (_, rows) in regimes {
                let f: Vec<&Row> = rows.iter().filter(|r| r.filtered).collect();
                let mean = if f.is_empty() {
                    0.0
                } else {
                    f.iter().map(|r| r.staged[2] as f64).sum::<f64>() / f.len() as f64
                };
                out.push_str(&format!(" {mean:.4} |"));
            }
            out.push('\n');
            continue;
        }
        out.push_str(&format!("| {label} |"));
        for (_, rows) in regimes {
            out.push_str(&format!(" {} |", cell(rows, of)));
        }
        out.push('\n');
    }

    for (label, pick) in [
        ("WIN-NOW row", 3usize),
        ("FILTERED row (`Cover::Minimal`)", 0),
        ("`Cover::Impossible`", 1),
        ("BATCHED nodes", 2),
    ] {
        out.push_str(&format!("| {label} | "));
        let mut first = true;
        for (_, rows) in regimes {
            if !first {
                out.push_str(" | ");
            }
            first = false;
            let n = rows.len().max(1) as f64;
            let v = match pick {
                0 => rows.iter().filter(|r| r.filtered).count() as f64,
                1 => rows.iter().filter(|r| r.impossible).count() as f64,
                3 => rows.iter().filter(|r| r.win_now).count() as f64,
                _ => rows.iter().filter(|r| !r.filtered && !r.win_now).count() as f64,
            };
            out.push_str(&format!("{:.1} %", 100.0 * v / n));
        }
        out.push_str(" |\n");
    }

    for (slot, option) in OPTIONS.iter().enumerate() {
        out.push_str(&format!(
            "| option {} — Tier T (threshold, ADOPTED) | ",
            option.name
        ));
        let mut first = true;
        for (_, rows) in regimes {
            if !first {
                out.push_str(" | ");
            }
            first = false;
            out.push_str(&cell(rows, |r| r.tier_t[slot] as f64));
        }
        out.push_str(" |\n");
        out.push_str(&format!(
            "| option {} — Tier T (exact, NOT adopted) | ",
            option.name
        ));
        let mut first = true;
        for (_, rows) in regimes {
            if !first {
                out.push_str(" | ");
            }
            first = false;
            out.push_str(&cell(rows, |r| r.tier_t_exact[slot] as f64));
        }
        out.push_str(" |\n");
        out.push_str(&format!(
            "| option {} — staged, BATCHED only | ",
            option.name
        ));
        let mut first = true;
        for (_, rows) in regimes {
            if !first {
                out.push_str(" | ");
            }
            first = false;
            let open: Vec<&Row> = rows.iter().filter(|r| !r.filtered && !r.win_now).collect();
            let (staged, ball) = if open.is_empty() {
                (0.0, 0.0)
            } else {
                (
                    open.iter().map(|r| r.staged[slot] as f64).sum::<f64>() / open.len() as f64,
                    open.iter().map(|r| r.ball2 as f64).sum::<f64>() / open.len() as f64,
                )
            };
            out.push_str(&format!(
                "{staged:.2} = {:.2}x",
                if staged > 0.0 { ball / staged } else { 0.0 }
            ));
        }
        out.push_str(" |\n");
        out.push_str(&format!(
            "| option {} — Tier T outside the r2 ball | ",
            option.name
        ));
        let mut first = true;
        for (_, rows) in regimes {
            if !first {
                out.push_str(" | ");
            }
            first = false;
            out.push_str(&cell(rows, |r| r.outside_ball[slot] as f64));
        }
        out.push_str(" |\n");
    }
    out
}

fn report(label: &str, rows: &[Row]) {
    let ball2 = mean(rows, |r| r.ball2 as f64);
    println!("\n== {label}  (n = {})", rows.len());
    println!(
        "   hot us {:.4} / them {:.4} | live2 us {:.2} them {:.2} | live3 us {:.2} them {:.2}",
        mean(rows, |r| r.hot_us as f64),
        mean(rows, |r| r.hot_them as f64),
        mean(rows, |r| r.live2_us as f64),
        mean(rows, |r| r.live2_them as f64),
        mean(rows, |r| r.live3_us as f64),
        mean(rows, |r| r.live3_them as f64),
    );
    let n = rows.len().max(1) as f64;
    println!(
        "   radius-2 ball {ball2:.2} | filter fires {:.1}% | impossible {:.1}%",
        100.0 * rows.iter().filter(|r| r.filtered).count() as f64 / n,
        100.0 * rows.iter().filter(|r| r.impossible).count() as f64 / n,
    );
    // THE COLUMN IS A MIXTURE, and the two populations are reported apart.
    // A `Minimal` or win-now node emits a forced set of two or three cells; the
    // batched nodes are the ones `quiet_top_k` and `widen_schedule` are sized
    // against, and quoting only the blended mean overstates the reduction on the
    // population the knobs actually govern (a REVIEW-design finding).
    let open: Vec<&Row> = rows.iter().filter(|r| !r.filtered && !r.win_now).collect();
    let open_ball = if open.is_empty() {
        0.0
    } else {
        open.iter().map(|r| r.ball2 as f64).sum::<f64>() / open.len() as f64
    };
    println!(
        "   batched nodes: {} of {} ({:.1}%), their ball {:.2}",
        open.len(),
        rows.len(),
        100.0 * open.len() as f64 / n,
        open_ball
    );
    for (slot, option) in OPTIONS.iter().enumerate() {
        let staged = mean(rows, |r| r.staged[slot] as f64);
        let open_staged = if open.is_empty() {
            0.0
        } else {
            open.iter().map(|r| r.staged[slot] as f64).sum::<f64>() / open.len() as f64
        };
        println!(
            "   option {}: tierT {:.4}  staged(all) {:.2} = {:.2}x  staged(batched only) {:.2} = {:.2}x  outside-ball {:.2}",
            option.name,
            mean(rows, |r| r.tier_t[slot] as f64),
            staged,
            if staged > 0.0 { ball2 / staged } else { 0.0 },
            open_staged,
            if open_staged > 0.0 {
                open_ball / open_staged
            } else {
                0.0
            },
            mean(rows, |r| r.outside_ball[slot] as f64),
        );
    }
}

#[test]
#[ignore = "a measurement, not a gate; run with --ignored --nocapture"]
fn wp15b_census() {
    report("corpus roots", &sample(Regime::Roots));
    report(
        "+1..3 turns, radius-2 draw (REPORTED)",
        &sample(Regime::DeepenRadius2),
    );
    report(
        "+1..3 turns, radius-8 draw (SUPERSEDED)",
        &sample(Regime::DeepenRadius8),
    );
    report("uniform playouts to 80 plies", &sample(Regime::Playouts));
}

/// THE DESIGN DOCUMENT CARRIES THIS INSTRUMENT'S OUTPUT VERBATIM.
///
/// Nine times across four revisions, a repair moved a number in one section of
/// `docs/experiments/wp15b_design.md` and left a copy of it in another. The
/// document now restates no population number outside the block below, and this
/// test is what makes that true rather than intended.
#[test]
fn the_design_document_carries_this_censuss_table_verbatim() {
    let table = render(&[
        ("corpus roots", sample(Regime::Roots)),
        (
            "+1..3 turns, r2 draw (REPORTED)",
            sample(Regime::DeepenRadius2),
        ),
        (
            "+1..3 turns, r8 draw (SUPERSEDED)",
            sample(Regime::DeepenRadius8),
        ),
        ("playouts", sample(Regime::Playouts)),
    ]);
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/experiments/wp15b_design.md"
    );
    let doc = std::fs::read_to_string(path).expect("the design document");
    let start = doc
        .find(BEGIN)
        .unwrap_or_else(|| panic!("the design document carries no `{BEGIN}` marker"))
        + BEGIN.len();
    let end = doc
        .find(END)
        .unwrap_or_else(|| panic!("the design document carries no `{END}` marker"));
    assert!(end > start, "the census markers are in the wrong order");
    // EXACTLY ONE PAIR. A reviewer appended a second BEGIN..END block carrying
    // `option C — Tier T | 999.9999` and this test stayed green, because
    // `find` takes the first pair and everything after `END` was unchecked.
    assert_eq!(
        doc.matches(BEGIN).count(),
        1,
        "the design document carries more than one census block; the pin checks the first"
    );
    assert_eq!(
        doc.matches(END).count(),
        1,
        "more than one census END marker"
    );
    let carried = doc[start..end].trim();
    assert_eq!(
        carried,
        table.trim(),
        "\n\nthe design document's census table has drifted from the instrument.\n\
         Replace the block between the two markers with:\n\n{table}"
    );

    // AND NO NUMBER FROM THE BLOCK IS RESTATED OUTSIDE IT. §6.2 claims this in
    // words; without the check the claim was false at eight sites and the pin was
    // green under a corruption of every one of them.
    let outside = format!("{}{}", &doc[..start - BEGIN.len()], &doc[end + END.len()..]);
    let mut restated: Vec<&str> = Vec::new();
    for line in table.lines() {
        for field in line.split('|').map(str::trim) {
            // Only the four-decimal renderings are unambiguous enough to grep
            // for; a bare "16" or "2.17x" occurs in prose for other reasons.
            if field.len() >= 6
                && field.contains('.')
                && field.split('.').nth(1).is_some_and(|d| d.len() == 4)
                && field.chars().all(|c| c.is_ascii_digit() || c == '.')
                && outside.contains(field)
            {
                restated.push(field);
            }
        }
    }
    restated.sort_unstable();
    restated.dedup();
    assert!(
        restated.is_empty(),
        "these census figures are restated OUTSIDE the pinned block, where nothing checks \
         them — cite the block instead: {restated:?}"
    );
}

/// The numbers the design's matrices actually rest on, TYPED OUT from
/// `docs/experiments/wp15b_design.md` and compared against what this instrument
/// derives. An edit to either side is a red test rather than a silent drift
/// (D-259's discipline).
///
/// Only the corpus-roots regime is pinned: it is deterministic with no sampler
/// in it, so it is the half a reader can check without trusting a seed.
#[test]
fn wp15b_census_reproduces_the_registered_populations() {
    let rows = sample(Regime::Roots);
    assert_eq!(rows.len(), 24, "the corpus is 24 turn-boundary positions");

    let close = |what: &str, got: f64, want: f64| {
        assert!(
            (got - want).abs() < 5e-3,
            "{what}: this instrument derives {got:.4}, the design registers {want:.4}"
        );
    };

    // §6.2's population table, corpus-roots column.
    close("own hot", mean(&rows, |r| r.hot_us as f64), 0.0417);
    close("opponent hot", mean(&rows, |r| r.hot_them as f64), 0.4583);
    close("live-2 own", mean(&rows, |r| r.live2_us as f64), 7.2083);
    close(
        "live-2 opponent",
        mean(&rows, |r| r.live2_them as f64),
        12.1667,
    );
    close("live-3 own", mean(&rows, |r| r.live3_us as f64), 0.75);
    close(
        "live-3 opponent",
        mean(&rows, |r| r.live3_them as f64),
        1.875,
    );
    close("radius-2 ball", mean(&rows, |r| r.ball2 as f64), 77.9583);

    // §6.3's Tier-T column, THRESHOLD reading. The defect that killed revision 1
    // was that the design's config clause spelled the EXACT reading while this
    // column was derived under the threshold one, so the exact values are what
    // this test exists to hold still.
    close(
        "tier T, option A",
        mean(&rows, |r| r.tier_t[0] as f64),
        6.125,
    );
    close(
        "tier T, option B",
        mean(&rows, |r| r.tier_t[1] as f64),
        46.5,
    );
    close(
        "tier T, option C",
        mean(&rows, |r| r.tier_t[2] as f64),
        23.2917,
    );

    // The filter rate, which is what makes the staged column a MIXTURE of two
    // node populations — the disclosure a REVIEW-design required.
    let filtered = rows.iter().filter(|r| r.filtered).count();
    let impossible = rows.iter().filter(|r| r.impossible).count();
    assert_eq!(
        filtered, 6,
        "the `Minimal` row is taken at 6 of the 24 roots"
    );
    assert_eq!(impossible, 1, "`Cover::Impossible` at exactly one root");
}
