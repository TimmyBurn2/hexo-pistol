# WP-1.3 — pre-registration

Filled from the operator's WP-1.3 runbook and committed **before the first game
and before the first profile**. Every blank the runbook left is filled here, and
so are four the runbook did not have (§7). Nothing below moves afterwards: an
inconclusive result is a result, and a threshold changed after the numbers are in
is not a smaller experiment, it is no experiment. Changing anything here means a
new pre-registration commit and a re-run, never an edit to this one.

**The review window is open until the first game of Run 1.** This document is
committed before its own REVIEW-design round, because the runbook requires it
committed before any game and a review cannot attack what is not written down.
Until that first game a fresh-context reviewer may still overturn any choice
below — which is precisely what happened to WP-1.2b's pre-registered verdict unit
(D-154) — and the overturning is a new pre-registration commit. From the first
game on, nothing here moves.

- **Measured revision:** `eb6ea932098e22ab29957d2b1798471169256d25` (`eb6ea93`),
  branch `dev`, tree clean. That is the engine, the fixtures and the tooling every
  number in §1 and §2 was produced with, and the tree the run configs were written
  against.
- **Named revision for review:** the commit that adds this file and the four run
  configs, directly on top of `eb6ea93` — `git log -1 --format=%H --
  docs/experiments/wp13_prereg.md`. A reviewer states that SHA and whether it
  still matches HEAD (CLAUDE.md, Process).
- **Date:** 2026-08-18.
- **Machine:** AMD Ryzen 7 3700X, 8 cores / 16 threads, Linux 7.1.8-arch1-3.
  Every number in §2 and every projection in §3 is this machine's.
- **Build:** `cargo build --release --locked`, i.e. the committed `[profile.release]`
  with `overflow-checks = true` (docs/decisions.md D-127). That IS the deployment
  build; no run below uses `release-checked` or a hand-edited profile.
- **Recorded as:** docs/decisions.md D-186.

Run configs are committed with this document, because a pre-registration whose
numbers live only in prose is re-typed into TOML afterwards, which is the gap it
exists to close:

| run | arena config | engines |
|---|---|---|
| 1 — r2 vs r3 | `configs/arena_wp13_r2_vs_r3.toml` | `configs/instrument_r2_v0.toml` vs `configs/instrument_v0.toml` |
| 3a — fairness, primary book | `configs/arena_wp13_fair_random.toml` | `configs/instrument_v0.toml` both seats |
| 3b — fairness, corpus book | `configs/arena_wp13_fair_corpus.toml` | `configs/instrument_v0.toml` both seats |

Run 2 has no arena config: it is a profile, and its driver is in §5.

---

## 1. Run 0 — smoke. COMPLETE, no anomaly, WP-1.3 proceeds.

Not a measurement. It answers one question — does the tool work on this hardware
— and the runbook makes any anomaly a full stop.

**`tools/arena_smoke.sh`**, from its own log output (docs/decisions.md D-185, not
an exit status):

```
arena_smoke: ok — 8 games over 4 openings, distinct-n 4, verdict inconclusive_degenerate,
arena_smoke:      three runs agree on the verdict block at 1 and 2 workers
```

**Ten-opening paired self-match of the committed config, one worker**, at 25 000
nodes (the budget was not yet chosen; this run is what began choosing it):

```
n 20  distinct-n 10  (10 duplicate games)
10 W / 10 L / 0 capped for a  (capped fraction 0.000)
pair outcomes  p0 0 p1 0 p2 10 p3 0 p4 0  (10 pairs)
LLR pair  none (degenerate sample)
compute a: 1149403 nodes, 5245 ms, 52 searches, deepest 1 turns
compute b: 1149403 nodes, 5242 ms, 52 searches, deepest 1 turns
wall 11753 ms at 1 workers
VERDICT inconclusive_degenerate
```

Every one of those is the value two identical deterministic engines must
produce: both games of a pair are the same game, so `distinct_n` is half of `n`,
every pair is 1-1, and no likelihood ratio is defined (docs/decisions.md D-156).
Both sides spent the identical node count. **No anomaly. Run 1 is unblocked.**

## 2. The calibration probe — NOT part of any sample

The runbook requires the node budget to be picked by a probe and the probe to be
recorded. It is recorded here in full, and no game or sample from it enters any
result. It ran at `eb6ea93`, release, one worker.

### 2a. Per-move cost, `bench_positions_v1.txt`, median of 4 positions per band

| budget | r3, 15 stones | r3, 35 stones | r2, 15 stones | r2, 35 stones |
|---|---|---|---|---|
| 25 000 | 172 ms | 119 ms | 482 ms | 400 ms |
| 50 000 | 1477 ms | 275 ms | 1163 ms | 1838 ms |
| 100 000 | 3413 ms | 5560 ms | 2094 ms | 3478 ms |
| 200 000 | 7338 ms | 11135 ms | 3160 ms | 6538 ms |

### 2b. Whole games, ten openings of the primary book, self-match, one worker

| budget | wall/game | per side/game | deepest depth reached | game length | capped | first player |
|---|---|---|---|---|---|---|
| 25 000 | 0.59 s | 0.26 s | 1 turn in 20 of 20 games | 7–13 turns | 0 | 8 of 20 |
| 50 000 | 4.9 s | 2.4 s | 2 turns in 5 of 20 games | 7–13 turns | 0 | 8 of 20 |
| 100 000 | 48.4 s | ~24 s | 2 turns in 20 of 20 games | 10–29 turns | 0 | 12 of 20 |

The 100 000 row shared the machine with the 50 000 row for about 97 s of its
969 s, so it is a slight over-estimate. It is far enough from the criterion that
the contention does not change what it decides.

### 2c. What the probe found, beyond the budget

1. **The budget is a cliff, not a dial.** Doubling nodes costs 8× the time,
   because what the extra nodes buy is *completing the next turn-iteration*, and
   an iteration the budget interrupts is discarded (`search.rs`).
2. **A fixed-node budget has a floor**, and it is D-74's deliberate design rather
   than a defect: `search.rs:150` passes `abortable = depth_turns > 1`, so the
   first deepening iteration cannot be interrupted and a position whose depth-1
   search costs more than the budget spends more. Measured: 104 448 nodes against
   a 25 000 budget. Node-matching is therefore approximate, and the per-side
   `compute` lines are what make the real spend visible.
3. **The cap is the first multiple of 1024 at or past the budget** — the pinned
   `NODE_CHECK_INTERVAL` (`stop.rs`) — so "50 000 nodes" is 50 176 in the report.
4. **nps collapses from ~215 000 to 10 000–50 000 exactly when a search reaches
   depth 2**, i.e. a large share of the time is not counted as nodes. That is a
   prior consistent with D-114's H1. **H1's threshold in §5 was NOT moved because
   of it** and stands at the runbook's recommended value; a profile is what
   adjudicates D-114, not this table.
5. **The first-player win rate moved with the budget** (8/20, 8/20, 12/20). Run 3
   therefore measures a property of the instrument, not of the game, and its ADR
   lines must say so.

## 3. Run 1 — r2 vs r3, fixed-node SPRT

**Book.** `crates/pistol-cli/tests/fixtures/random_openings_v1.txt`, the primary
SPRT book. *The runbook cites "A-2 ADR" for this; there is no A-numbered ADR in
this project — the ruling is **D-175**, and D-183 pins the cross-crate load. This
document cites D-175.* The arena verifies the file against its own in-band body
digest before a game is played (D-148).

**Budget.** Fixed **nodes = 50 000**. Chosen from §2: 2.4 s per side per game
meets the runbook's single-digit-seconds-per-side criterion with room, and a
second iteration completes in some games rather than in none of them (25 000) or
at 10× the cost (100 000). Projected run: ~25 min at 4 workers, from §2b's 4.9 s per game — an order of
magnitude rather than a promise, since §2b measured radius 3 against itself and
this run pairs it with radius 2, whose per-move cost differs.

**Engines.** A = `configs/instrument_r2_v0.toml` (radius 2). B =
`configs/instrument_v0.toml` (radius 3, the committed config). The two documents
differ in exactly one value — verified by diffing them with comments stripped:
`radius = 3` against `radius = 2`, and nothing else.

**THE SLOT ASSIGNMENT IS THE REVERSE OF THE RUNBOOK'S LETTERING, DELIBERATELY.**
The runbook says "A = committed config (radius 3), B = ... radius 2" with
"H1 = B (r2) is stronger". The arena's statistic is **engine A's** score —
`score::game_sample` builds the sample from `wins_a / capped / losses_a` — so
`elo1 > 0` states "A is stronger than B" and can state nothing else. Had radius 3
gone in slot A, the tool would have tested the opposite hypothesis to the one the
runbook pre-registered, and the error would have shown up only as a sign nobody
questioned. **The pre-registered direction is what binds: H1 = "radius 2 is
stronger than radius 3", and the radius-2 config therefore sits in `engine_a`.**

**Hypotheses.** `elo0 = 0`, **`elo1 = 25`** normalized Elo, `alpha = beta = 0.05`.
Direction as above.

*Why 25 and not the runbook's recommended 5.* The book supplies 500 openings =
500 pairs = 1000 games, and that is the hard cap (the game cap is derived as
twice `openings_take`, D-157). Two independent calculations agree on what a
sample that size can resolve:

- The research report's own guidance, `games ≈ 640000/elo1²`, solves to
  elo1 = 25.3 at 1000 games. At elo1 = 5 it wants 25 600 games — 12 800 openings,
  25.6× this book.
- The interval the arena will print. `conclusion.rs::confidence` is
  `1.96 / (NELO_TO_T · √2 · √pairs)`, **a function of the pair count alone** — the
  observed scores never enter it, because the estimate is already standardized.
  At 500 pairs that is **±21.5 normalized Elo**.

At the pair-level GSPRT this crate implements, with `LLR = n·(t₁·t̂ − t₁²/2)` and
a Wald bound of ±2.9444:

| elo1 | accepts H1 at observed | accepts H0 at observed | pairs at a true tie |
|---|---|---|---|
| 5 | +73.6 nElo | −68.6 nElo | 14 217 (28× the book) |
| **25** | **+26.7 nElo** | **−1.7 nElo** | **569** |
| 30 | +26.8 nElo | +3.2 nElo | 395 |

elo1 = 5 would print a verdict about a 5-Elo difference beside an interval four
times wider, and inside this book could conclude only on a ~70-Elo effect. At 25
the verdict and the interval tell the same story. **Rejected alternatives**, so
that this is re-openable rather than re-argued: (a) elo1 = 5 as recommended,
rejected on the arithmetic above; (b) elo1 = 30, marginally better powered —
it reaches an H0 verdict at a true tie inside the book — rejected only because 25
is the value the report's own formula and the reported interval both land on, and
a bound derived from the instrument is easier to defend than one tuned for its
own power; (c) regenerate the book at ~12 800 openings so elo1 = 5 is powered,
rejected as a work package of its own (new fixture, config, regeneration test and
ADR under D-175/D-177) plus ~3.5 days of games at 4 workers, which is a decision
for after this run rather than before it.

**Verdict level.** **Pair.** D-154 already makes this law and the arena hard-codes
`verdict_unit pair`; the game-level LLR is printed beside it as a diagnostic and
is not the verdict. Written here because the runbook asked for it in writing.

**Workers.** 4 — the ceiling D-174's RED-TEAM round cleared. Above it the
concurrency surface is explicitly not cleared, and a strength claim is not where
to explore it. The verdict block is worker-invariant for a run that completes
(D-161), so this costs the result nothing.

**Game cap if inconclusive.** **1000 games = 500 pairs = the whole book**, set as
`openings_take = 500`. This is the largest sample the primary book can supply;
the runbook's `~640000/elo1²` guidance is unreachable here at any elo1 below 25
and is what §3's elo1 choice answers instead.

**Outcome handling, pre-committed.**

- **H1 accepted** → radius 2 becomes the committed config, by one config commit
  and one ADR line. `configs/instrument_r2_v0.toml` stays as the recorded losing
  arm so the experiment can be re-run.
- **H0 accepted** → radius 3 stays.
- **Inconclusive at the cap** → radius 3 stays; the incumbent wins ties. The ADR
  line still carries `nelo_pair ± ci95`, which makes it a *bounded* null: "the
  candidate radius is worth less than about ±22 normalized Elo" is a finding, and
  a useful one given Stage 1's threat-first generator supersedes the radius policy
  (docs/ROADMAP.md WP-1.5).
- **`inconclusive_degenerate`** → not expected between two different
  configurations; if it happens, the run is reported and the cause is found before
  anything is concluded.
- **`invalid_forfeit`** → the run is not a measurement (D-158). It is reported,
  not discarded, and it is re-run only after the forfeit's cause is fixed; the
  report's `verdict_if_clean` line is read as diagnosis, never as the verdict.

**Reporting fields**, per the research report's Deliverable 4, and where each one
comes from. The ADR line carries the starred ones; the full report file carries
all of them.

| Deliverable 4 field | source |
|---|---|
| engine hashes | `engine_id` / per-side binary and config digests; `experiment_sha256`\* |
| net hash | none at Stage 0 — the eval is `handcrafted_v0`; the committed weight table's digest stands in |
| budget mode + value\* | `budget nodes 50000` |
| hardware, thread count | this document's header; `threads = 1` per engine, `n_workers 4` |
| book\* | `openings_file` + `openings_body_sha256` |
| n, n distinct\* | `counts n … distinct_n …` |
| W/D/L\* | `wins_a`, `capped`, `losses_a` — "D" is *capped*, not a draw; there are no draws (game rule 6) |
| pentanomial\* | `pentanomial p0 … p4` |
| Elo ± CI\*, normalized Elo\* | `nelo_pair … ci95 …`, reported in **normalized** Elo and NOT converted to logistic Elo — the two coincide only at σ = ½ |
| LLR\* | `llr_pair last` (verdict) and `llr_game last` (diagnostic) |
| first-player win rate | `first_player_wins k of n decided` (Run 3 is the run that exists for it) |
| per-side compute\* | `compute` lines: nodes, ms, searches, deepest |

**Two pre-registered readings of the interval**, so neither can be discovered
afterwards:

1. **`ci95` is anti-conservative at an early stop.** Optional stopping selects on
   the estimate, so a run that crosses a boundary prints an optimistic interval;
   it is honest only at the cap. A crossing run's ADR line says so.
2. **A fixed-node result is not a fixed-time result.** At 50 000 nodes r3 and r2
   do not cost the same wall-clock (§2a), and the per-side `compute` lines are
   what show it. If r2 wins on nodes it wins on time too; if r3 wins on nodes, the
   time picture is the opposite and the ADR line states both.

## 4. What the pre-registration does not claim

- **1-1 pairs carry no signal.** D-145 measured human openings lopsided, and under
  determinism a decided opening yields a fixed 1-1 pair. If `pentanomial p2`
  dominates, the effective sample is below 500 pairs even though `ci95` — which
  is a function of the pair count alone — does not shrink to say so. The
  pentanomial line is what reveals it and the ADR line quotes it.
- **`distinct_n` is an over-count and a bound, not a census** (D-163).
- **The instrument is shallow.** At this budget most searches complete one turn of
  depth and some complete two. This experiment compares two candidate policies at
  *that* instrument; it does not predict their order at Stage 1 depths, and
  nothing here licenses that extrapolation.

## 5. Run 2 — flamegraph, adjudicating D-114's H1 and H2

**Build.** Release, the committed profile, `overflow-checks` on — the deployment
build, stated because the writeup must. DWARF line tables are added with
`CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` in the environment: it is not a
profile edit and does not change codegen, only what the samples can be attributed
to.

**Workload.** Instrument mode, fixed-node runs over
`crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`, **both bands** — 12
positions in the 15-stone band and 12 in the 35-stone band, of which eleven hold
35 stones and one holds 31, because a band does not always reach its centre
(D-146). D-114 requires two stone counts because candidate cost grows with how
far apart the stones sit, at **50 000 nodes**, the
Run-1 value.

**Driver.** `bench` is unimplemented and stays so (D-14), so the workload is
driven through the line protocol: for each position, `newgame`, `position <tail>`,
`go nodes 50000`, one `perf record` per band. `perf record -F 999 --call-graph
dwarf`, and the writeup reports the total sample count so the percentages have an
*n*.

**Adjudication rule, pre-registered, on the POOLED two-band sample** (per-band
numbers are reported beside it; pooling is stated now so that neither band can be
chosen after the fact):

- **H1 — ordering evals (D-76)** is CONFIRMED at **≥ 20 %** of samples
  attributable to the eval apply/undo roundtrip in move ordering. The target is
  named exactly rather than described: `pistol_search::position::Position::
  static_score_after`, which *is* D-76's `apply` → `value` → `undo`, called once
  per candidate from `ordering::order`. Measured inclusively; if it is inlined
  into `order`, the inclusive figure under `order` is the attribution and the
  writeup says so. **This threshold is the runbook's recommendation and was not
  moved by §2c's nps observation.**
- **H2 — per-node allocation** is CONFIRMED at **≥ 10 %** of samples in allocator
  frames reached from `candidates::candidate_cells` (which allocates a fresh
  `Vec` per node, via `within_radius` and `ball_offsets`) or from
  `ordering::order` (which allocates its `scored` vector per node).
- **H1 confirmed** unlocks implementing `Eval::delta` per D-110 — its own work
  package, with D-110's oracle test and a Hard-Rule-5 bench. **H2 confirmed** gets
  its own pre-registered bench before any fix. **Neither confirmed** → record the
  actual top three and stop. No fix lands without a new pre-registration, and a
  third hypothesis is a new D-line and a new profile, never an addition to this
  one (D-114).

**One machine caveat, stated in advance.** `kernel.perf_event_paranoid` is 2 here,
which permits user-space samples only. The profile is attempted at 1 (one
`sysctl`, operator's call); if it is not lowered, the run is user-space-only, and
H2 is then a **lower bound**, because allocation cost that lands in kernel
page-fault handling is invisible. The writeup states which of the two happened.

**Artifacts.** SVGs and `perf.data` go to the operator's workbench directory
outside the repository and are never committed (CLAUDE.md rule 8). One ADR line
each for H1 and H2.

## 6. Run 3 — fairness (a measurement, not a test)

Self-play of the committed config, 50 000 nodes, 4 workers, **both books**, via
`configs/arena_wp13_fair_random.toml` (500 openings) and
`configs/arena_wp13_fair_corpus.toml` (1591 openings). Projected ~1.5 h at 4
workers for the pair of them.

Reported per book: `first_player_wins k of n decided`, `distinct_n`,
`capped_fraction`, per-side compute, and a **Wilson 95 % interval computed on
DISTINCT games** — the estimator is named now so it is not chosen after the
numbers: ±4.4 pp at 500 distinct games, ±2.5 pp at 1591 (both at p = ½). One ADR
line per book. The expected verdict is `inconclusive_degenerate`, which is the
correct answer for a self-match and not a failure (D-156).

Three clauses that go in those ADR lines whatever the numbers are:

1. **The rate is a property of the instrument.** §2b measured it moving with the
   budget — 8/20 at 25 000 and 50 000, 12/20 at 100 000. A first-player rate
   quoted without its budget is a claim nobody measured.
2. **The two books do not estimate the same quantity and are never averaged.**
   3a samples openings uniformly; 3b's openings are the ones humans reached, and
   D-145 measured those lopsided (first-player rate 0.10–0.91 across classes with
   ≥10 corpus games). 3b is the first player's advantage *conditioned on a
   human-reached opening*.
3. **The sample is the distinct games**, not the games played: identical
   deterministic engines make both games of a pair the same game.

## 7. Blanks the runbook did not have, filled here

The arena schema requires them, they are not statistically inert, and a
pre-registration that left them to run time would leave the two liveliest knobs
unpinned.

- **`turn_cap = 40`** (all three runs). Capped games are *not* inert: normalized
  Elo is variance-normalized, so a mass point at a half shrinks σ and, for a fixed
  decisive record, accelerates the LLR toward H1 (D-157, which reports the
  measurement and deliberately does not threshold on it). 40 is three times the
  longest game §2b saw, across 60 games at three budgets, none of which capped. It
  is expected to bind on nothing, and `capped_fraction` is reported so that a run
  in which it did bind cannot be read as one in which it did not.
- **`hang_timeout_ms = 120000`** (all three runs). Liveness only — it can end a
  run and can never produce a game result (D-159) — set at roughly forty times the
  worst single search the probe saw at this budget (2.9 s, single-threaded), which
  is the margin four contending workers and a late-game position are allowed to
  want.
- **The slot assignment**, §3. The runbook's A/B lettering would have inverted the
  hypothesis.
- **The `[sprt]` block in the two Run-3 configs**, which is inert and complete
  because the schema admits no absent key (CLAUDE.md rule 1). No likelihood ratio
  is defined on a self-match sample at all.

## 8. Order, and what happens after

Run 2's budget is Run 1's and is fixed here, so the three runs are independent and
may be run in any order; each writes its own report to the workbench directory
outside the repository. Results return to the planning session as ADR lines. Then
WP-1.4 (movetime ceiling, D-95) with the pistol-api stdio-shim spec ADR riding
along, then WP-1.5 (threat core, carrying its D-124 visibility obligation).
