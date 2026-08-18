# WP-1.3 — pre-registration (revision 2)

Filled from the operator's WP-1.3 runbook and committed **before the first game
and before the first profile**. Every blank the runbook left is filled here, and
so are five the runbook did not have (§7). Nothing below moves once Run 3a's
first game is played: an inconclusive result is a result, and a threshold changed
after the numbers are in is not a smaller experiment, it is no experiment.

**This is revision 2, and revision 1 was never run.** It supersedes the
pre-registration committed at `23f14f0` after three fresh-context reviews
attacked it — two REVIEW-design rounds and a RED-TEAM round — exactly as §0 of
that document invited. No game and no profile was run under revision 1; the
review window it opened is what this revision came through, and it closes at Run
3a's first game. Changing anything after that means a new pre-registration and a
re-run, never an edit.

- **Measured revision:** `eb6ea93` for §1 and for §2b; `23f14f0` for §2a's
  radius-2 columns and for §2c's per-arm measurements, because
  `configs/instrument_r2_v0.toml` did not exist before that commit. Revision 1's
  header claimed all of §1 and §2 for `eb6ea93` and was wrong on that point.
- **Named revision for review:** the commit that carries this file —
  `git log -1 --format=%H -- docs/experiments/wp13_prereg.md`.
- **Date:** 2026-08-18.
- **Machine:** AMD Ryzen 7 3700X, 8 cores / 16 threads, 46 GB RAM, Linux
  7.1.8-arch1-3. Four workers hold two engines each at `tt_bytes = 268435456`,
  so about 2 GiB resident against 19 GiB available — checked, because an
  OOM-killed child abandons a run rather than forfeiting it.
- **Build:** `cargo build --release --locked` — the committed `[profile.release]`
  with `overflow-checks = true` (D-127). That IS the deployment build.
- **Recorded as:** D-186, amended by D-188. The book this run plays is D-187.

| run | order | arena config | engines |
|---|---|---|---|
| 3a — fairness + determinism gate | **first** | `configs/arena_wp13_fair_random.toml` | `instrument_v0` both seats |
| 1 — r2 vs r3 | second | `configs/arena_wp13_r2_vs_r3.toml` | `instrument_r2_v0` (slot A) vs `instrument_v0` |
| 3b — fairness, corpus book | third | `configs/arena_wp13_fair_corpus.toml` | `instrument_v0` both seats |
| 2 — flamegraph | fourth | none; §5 is the driver | `instrument_v0` (radius 3), pinned |

---

## 0. What the reviews changed

Recorded because a pre-registration that quietly replaced its predecessor would
be worth less than one that never existed.

1. **The design was underpowered and said the opposite.** At the 500-opening book
   revision 1 named, `alpha = beta = 0.05` achieved α = 0.030 and **power 0.569**
   against its own alternative. The book is extended to 2000 openings (D-187) and
   the same bounds now achieve α = 0.049 and power 0.945 (§3).
2. **`640000/elo1²` is not a power formula.** Revision 1's central derivation read
   an expected-sample heuristic as one. The corrected reasoning is in §3; D-188
   amends D-186, which carried it into the append-only log.
3. **A crossing could fire on ten pairs.** §3 now sets a 100-pair floor on the H1
   *action*.
4. **D-174 was miscited.** Revision 1 said four workers were "the ceiling D-174's
   RED-TEAM round cleared". D-174 lists the concurrency surface under "NOT
   reached, and therefore not cleared". WP-1.3's own RED-TEAM round then cleared
   1, 2, 4 and 8 workers over the full corpus book through a real early stop, and
   that is what §3 now cites. Nothing in this repository had run the arena above
   two workers before it.
5. **A promised reporting field does not exist.** Revision 1 said the eval weight
   table's digest stood in for Deliverable 4's net hash. Nothing digests it, and
   two engines differing only in that file produce byte-identical provenance
   (§3, §9).
6. **The profile's H1 target is inlined away**, and revision 1's fallback
   double-counted with H2 (§5).
7. **Run 2's engine config was never named** (§5), and Run 3 had no forfeit
   clause (§6).
8. Plus corrections to the wall-clock reading, the first-player claim, the
   `hang_timeout_ms` basis, the depth asymmetry, the "pooled"/"inclusive"
   definitions, `ball_offsets`, an off-by-one (14 218 pairs, not 14 217), and the run order.

## 1. Run 0 — smoke. COMPLETE, no anomaly.

Not a measurement. `tools/arena_smoke.sh`, from the gate's own log output
(D-185):

```
arena_smoke: ok — 8 games over 4 openings, distinct-n 4, verdict inconclusive_degenerate,
arena_smoke:      three runs agree on the verdict block at 1 and 2 workers
```

Ten-opening paired self-match of the committed config, one worker, at 25 000
nodes: `n 20`, `distinct-n 10`, all ten pairs 1-1, both sides on identical node
counts, `inconclusive_degenerate`. Every one of those is the value two identical
deterministic engines must produce. **This run is unaffected by D-187's extension
of the book**, because the ten openings it played are the book's first ten and
those bytes did not change.

## 2. The calibration probe — NOT part of any sample

### 2a. Per-move cost, `bench_positions_v1.txt`, median of 4 positions per band

| budget | r3, 15 st | r3, 35 st | r2, 15 st | r2, 35 st |
|---|---|---|---|---|
| 25 000 | 172 ms | 119 ms | 482 ms | 400 ms |
| **50 000** | 1477 ms | 275 ms | 1163 ms | 1838 ms |
| 100 000 | 3413 ms | 5560 ms | 2094 ms | 3478 ms |
| 200 000 | 7338 ms | 11135 ms | 3160 ms | 6538 ms |

**The ordering is not monotone in either direction**, and §3's second interval
reading depends on knowing that: at 50 000 nodes radius 2 costs 6.7× radius 3's
wall-clock in the 35-stone band, while at 100 000 and 200 000 it is uniformly
cheaper. Equal nodes is not equal time, and which way it falls is a property of
which iteration completes.

### 2b. Whole games, ten openings, self-match, one worker

| budget | wall/game | per side | deepest reached | game length | capped | first player |
|---|---|---|---|---|---|---|
| 25 000 | 0.59 s | 0.26 s | 1 turn in 20/20 games | 7–13 turns | 0 | 8 of 20 |
| 50 000 | 4.9 s | 2.4 s | 2 turns in 5/20 games | 7–13 turns | 0 | 8 of 20 |
| 100 000 | 48.4 s | ~24 s | 2 turns in 20/20 games | 10–29 turns | 0 | 12 of 20 |

### 2c. What else the probe measured

1. **The budget is a cliff, not a dial.** Doubling nodes costs ~8× the time,
   because what the extra nodes buy is completing the next turn-iteration, and an
   interrupted iteration is discarded.
2. **A fixed-node budget has a floor** — D-74's deliberate design, since
   `search.rs:150` passes `abortable = depth_turns > 1`. Measured over the whole
   24-position fixture **at this run's budget**: the 50 176-node cap binds
   identically on both arms in 22 of 24 positions; radius 3 exceeds it in two
   (104 448 and 54 272 nodes) and radius 2 in none.
3. **The same nodes buy very different depth.** Radius 3 completes a second
   turn-iteration in **6 of 24** positions, radius 2 in **17 of 24**. A depth-1
   iteration searches the mover's own turn only, so in most positions radius 3
   chooses without seeing a reply. This is the mechanism under test — a narrower
   candidate set is more depth per node — and not a confound, but it is a
   contrast and §4 states it rather than pooling it.
4. **The cap is the next multiple of 1024** (`NODE_CHECK_INTERVAL`), so "50 000
   nodes" is 50 176.
5. **nps collapses from ~215 000 to 10 000–50 000 when a search reaches depth 2**,
   i.e. much of the time is not counted as nodes — a prior consistent with
   D-114's H1. **§5's thresholds were not moved because of it.**
6. **The worst single search at this budget is 5.1 s**, over the whole fixture,
   single-threaded. Revision 1 said 2.9 s, which was the worst of the eight-position
   probe rather than of the fixture.
7. **The first-player rate is NOT known to move with the budget.** Revision 1
   called the 8/20, 8/20, 12/20 sequence "measured". It is not: those are ten
   *distinct* games each (both games of a pair are the same game), the three rows
   are paired on the same openings, and no split of a net-2 shift over ten paired
   openings reaches p < 0.5. The caution survives — a first-player rate must be
   quoted with its budget — but the observation does not.

## 3. Run 1 — r2 vs r3, fixed-node SPRT

**Book.** `random_openings_v1.txt`, the primary SPRT book (**D-175**, extended to
2000 openings by D-187 — the runbook's "A-2 ADR" does not exist). Whole-file
digest `895a05ed…`, in-band body digest `7b1b3a99…`, verified by the arena
before a game is played (D-148, D-183).

**Budget.** Fixed **nodes = 50 000** (§2). **Engines.** A =
`instrument_r2_v0.toml` (radius 2), B = `instrument_v0.toml` (radius 3). The two
differ in exactly one value, verified by a comment-stripped diff.

**THE SLOT ASSIGNMENT IS THE REVERSE OF THE RUNBOOK'S LETTERING, DELIBERATELY.**
The arena's statistic is **engine A's** score, so `elo1 > 0` states "A is
stronger than B" and can state nothing else. All three reviewers traced this
independently — `record::score_a` → `tally.wins_a` → `Sample::of_pairs` →
`t_hat` → `crossing` — and RED-TEAM confirmed it empirically by running r2 in
slot A and getting `nelo_pair +148`, `llr_pair +1.12`, toward H1. The
pre-registered direction binds: **H1 = "radius 2 is stronger than radius 3"**, so
radius 2 sits in `engine_a`. (Revision 1 cited `score::game_sample` for this; that
is the *diagnostic* unit. The verdict runs through `score::pair_sample`. Both are
A-oriented.)

**Hypotheses.** `elo0 = 0`, **`elo1 = 25`** normalized Elo, `alpha = beta = 0.05`,
cap **2000 pairs = 4000 games** (`openings_take = 2000`; the game cap is derived
as twice it, D-157).

*Why 25, corrected.* Revision 1 argued from the report's `games ≈ 640000/elo1²`
solved at the book size. **That expression is not a power formula** — it sizes the
sample at which the LLR's *expected* value first reaches a boundary, where the
run has crossed roughly half the time. Solving it at the book size therefore
chose a ~50 %-power design by construction, and the measurement agreed: 0.569.
What survives is that 3–5 is badly matched to any sample this project can draw,
though not as badly as revision 1 implied: at elo1 = 5 and this cap, H1 is
reachable — about 1580 pairs at a truth of +25 nElo — but a true tie can never
be concluded, since its drift totals −0.41 against a −2.94 bound and would need
14 218 pairs. A design that can accept H1 and can never accept H0 is not a test.
25 is the scale at which this instrument can both test and estimate. **The cap is what had to move, not the alternative.**

*The achieved operating characteristics*, simulated over 40 000 runs per truth on
this crate's own `Sample` and `crossing`, at a 0.5 decisive-pair fraction:

| truth | P(H1) | P(H0) | P(inconclusive) | mean pairs | mean games |
|---|---|---|---|---|---|
| 0 nElo | 0.049 | 0.944 | 0.007 | 521 | 1042 |
| **+25 nElo** | **0.945** | 0.048 | 0.007 | 520 | 1039 |
| −25 nElo | 0.049 | 0.945 | 0.006 | 518 | 1037 |
| +50 nElo | 1.000 | 0.000 | 0.000 | 191 | 382 |

α and β are now delivered rather than declared. The four-fold cap costs almost
nothing because the test stops when it has an answer: the expected run is ~520
pairs either way, and the cap binds only where it is needed. At the cap, `ci95`
is **±10.8 nElo** (±21.5 at 500), H1 needs an observed +16.1 and H0 an observed
+9.0, and the drift boundary at 569 pairs is now *inside* the cap.

**The 100-pair floor on the H1 action.** At `elo1 = 25` the smallest H1 crossing
is **10 pairs** — 9 swept pairs and one 1-1 — which would be a config change on
20 games; D-156 declined a `min_pairs_before_stop` when the floor was 33 pairs at
elo1 = 4–5, and that reasoning does not survive the larger alternative. So: **an
H1 crossing on fewer than 100 pairs is reported and is NOT acted on.** It happens
in 1.8 % of runs at truth +25 and 0.12 % at a true tie. Confirmation cannot be a
re-run — the engines are deterministic, so the identical config reproduces the
identical result — and the arena takes a prefix with no offset knob, so the
pre-registered confirmatory sample is **`openings_v1.txt` at the same budget**, a
disjoint 1591-opening sample; the config changes only if that run also crosses
H1. H0 crossings have no floor: they leave the incumbent in place.

**Verdict level: pair.** D-154 makes it law and `conclusion.rs` hard-codes
`verdict_unit pair`. **Workers: 4** — cleared by WP-1.3's own RED-TEAM round (see
§0.4), not by D-174.

**Outcome handling, pre-committed.**

- **H1 accepted on ≥ 100 pairs** → radius 2 becomes the committed config, by one
  config commit and one ADR line — **after all four runs are complete** (§8).
- **H1 accepted on < 100 pairs** → reported as a signal; the confirmatory run
  above decides.
- **H0 accepted** → radius 3 stays. The ADR line carries `nelo_pair ± ci95`
  *and* states what H0 does and does not exclude: at this cap it means "radius 2
  is not ≥ 25 nElo better", which is weaker than it sounds.
- **Inconclusive at the cap** → radius 3 stays; the incumbent wins ties. The ADR
  line carries the interval, making it a bounded null.
- **`inconclusive_degenerate`** → not expected between two different
  configurations; reported, and the cause found before anything is concluded.
- **`invalid_forfeit`** → not a measurement (D-158). Reported, not discarded, and
  re-run only after the cause is fixed. **`nelo_pair`, `ci95` and `llr_pair` on
  such a report include the forfeited pairs**; only `verdict_if_clean` excludes
  them, so the estimate printed above that line is the polluted one.
- **An abandoned run** — a hang, an engine exit or a handshake failure — writes
  `arena_report_aborted` with **no verdict line at all**, and exits 1. It is not a
  measurement and its completed prefix is not a result: reading one would be
  optional stopping on a timing-correlated rule. It is re-run in full.

**Reporting fields** (Deliverable 4). ADR line carries the starred ones.

| field | source |
|---|---|
| engine hashes | `engine_id`, per-side binary and config digests, `experiment_sha256`\* |
| net hash | **none, and none stands in.** The eval is `handcrafted_v0`; **nothing in the report identifies `eval_v0_weights.toml` by content** — see §9.1. Provenance is the repository revision plus the directory the run started in |
| budget mode + value\* | `budget nodes 50000` |
| hardware, threads | this header; `threads = 1` per engine, `n_workers 4` |
| book\* | `openings_file` + `openings_body_sha256` |
| n, distinct-n\* | `counts n … distinct_n …` (an over-count and a bound, D-163) |
| W/D/L\* | `wins_a`, `capped`, `losses_a` — "D" is *capped*; there are no draws |
| pentanomial\*, **decisive pairs**\* | `pentanomial p0 … p4`; the decisive count is `p0+p1+p3+p4` and is quoted beside the estimate — see §4 |
| Elo ± CI\*, normalized Elo\* | `nelo_pair … ci95 …`, in **normalized** Elo, never converted to logistic |
| LLR\* | `llr_pair last` (verdict), `llr_game last` (diagnostic) |
| first-player rate | Run 3 |
| per-side compute\* | `compute`: nodes, ms, searches, deepest — **plus the per-game `depth_a`/`depth_b` distribution**, since run-level `deepest` is a maximum |

**Two pre-registered readings of the interval.**

1. **`ci95` is anti-conservative at an early stop.** Measured: coverage 0.868 at
   the sequential stop against 0.978 for runs reaching the cap — about eight
   points. A crossing run's ADR line says so. It also prints a *wider* interval
   than ±10.8, because the interval is a function of the pairs actually played.
2. **A fixed-node result is not a fixed-time result, and the direction is not
   predictable in advance.** Revision 1 asserted that an r2 node-win would be a
   time-win too; §2a refutes that at this very budget. **The direction is read off
   the run's own per-side `compute` ms lines, in whichever way they fall, and
   nothing is asserted here.**

## 4. What the pre-registration does not claim

- **A normalized-Elo bar is set by the run's own tie rate, and that is the axis
  this experiment is fragile on.** Revision 1 said 1-1 pairs make `ci95`
  overstate precision. That is wrong and is withdrawn: measured coverage of
  `nelo_pair ± ci95` is 0.946–0.957 across decisive fractions from 1.0 down to
  0.02, and if anything the interval is conservative when decisive pairs are
  sparse. What *is* true is that the same conditional edge maps to wildly
  different normalized Elo depending on how often the arms differ at all — a
  radius 2 winning 60 % of the openings where the two differ registers ~50 nElo
  if they differ everywhere and ~11 nElo if they differ on 5 % of openings. At the
  extreme, **494 pairs 1-1 plus six openings swept by A crosses H1** (`llr_pair
  +3.0186`). That is arithmetically correct for a normalized-Elo SPRT and it is
  why the decisive-pair count is a required ADR field: the verdict is not wrong,
  but its practical weight is not readable without it.
- **The arms differ often enough for the run not to be vacuous** — measured, they
  choose different moves on 9 of 24 bench positions and 7 of 12 in the 15-stone
  band, which is the regime a 7–13-turn game occupies.
- **The instrument is shallow, asymmetrically.** §2c.3: radius 3 completes a
  second iteration in 6 of 24 positions against radius 2's 17. This experiment
  compares two candidate policies at *that* instrument and does not predict their
  order at Stage 1 depths.
- **`distinct_n` is an over-count and a bound, not a census** (D-163).

## 5. Run 2 — flamegraph, adjudicating D-114's H1 and H2

**Engine config: `configs/instrument_v0.toml` as it stands at THIS commit —
radius 3 — regardless of Run 1's outcome.** Named because revision 1 named
everything else and left this blank, while its own H1 action would have rewritten
"the committed config" underneath it. **Build:** release, committed profile,
`overflow-checks` on, with `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` in the
environment — not a profile edit, and it changes no codegen. **Workload:**
instrument mode over `bench_positions_v1.txt`, both bands — 12 positions at 15
stones and 12 in the 35-stone band, of which eleven hold 35 and one holds 31
(D-146) — at **50 000 nodes**.

**Driver.** `bench` is unimplemented and stays so (D-14): for each position,
`newgame`, `position <tail>`, `go nodes 50000`, one `perf record -F 999
--call-graph dwarf` per band.

**Adjudication, pre-registered.** **Pooled** means the raw sample counts of the
two `perf.data` files are summed; **inclusive** means the fraction of samples
whose stack contains the frame. Per-band numbers are reported beside the pooled
figure; pooling is fixed now so neither band can be chosen afterwards.

- **H1 — ordering evals (D-76) is CONFIRMED at ≥ 20 % of pooled samples.**
  Revision 1 named `Position::static_score_after`; **that symbol does not exist in
  this build** (`nm -C target/release/pistol | grep -c static_score_after` → 0),
  and revision 1's fallback — everything inclusive under `ordering::order` —
  swept in the `scored` vector's allocation and the sort, which is what H2
  separately counts, making H1 near-self-confirming. The target is instead the
  three `Eval` callees, which sit behind `Box<dyn Eval>` (`position.rs:27`) and
  therefore **cannot be inlined**: `<pistol_eval::handcrafted::HandcraftedV0 as
  pistol_eval::eval::Eval>::{apply, value, undo}`, counted only on stacks passing
  through `pistol_search::ordering::order`. DWARF call graphs separate those from
  the same callees reached via `Position::place` under `pvs::visit`. **H1's figure
  excludes the sort and the allocations H2 counts.**
- **H2 — per-node allocation is CONFIRMED at ≥ 10 %** of pooled samples in
  allocator frames reached from `pistol_search::candidates::candidate_cells`
  (which allocates per node via `pistol_search::candidates::within_radius` and
  `pistol_search::candidates::ball_offsets` — **not** `pistol_core::movegen::
  ball_offsets`, which is the rules' radius-8 generator and a different function
  on the other side of the distinction rule 2 forbids conflating) or from
  `ordering::order`'s `scored` vector.
- **H1 confirmed** unlocks `Eval::delta` (D-110) as its own WP with D-110's oracle
  test and a rule-5 bench. **H2 confirmed** gets its own pre-registered bench.
  **Neither** → record the top three and stop. A third hypothesis is a new D-line
  and a new profile (D-114).

**`kernel.perf_event_paranoid` is 2 here**, permitting user-space samples only.
The profile is attempted at 1; if the operator declines the `sysctl`, H2 is a
**lower bound**, since allocation cost landing in kernel page-fault handling is
invisible. The writeup states which happened, and the total sample count per band.

**Artifacts** go to the workbench directory outside the repository (§8).

## 6. Run 3 — fairness (a measurement, not a test)

Self-play of the committed config at 50 000 nodes, 4 workers, both books.

**Run 3a runs FIRST and is this package's determinism gate.** A self-match is
*forced* to `inconclusive_degenerate` by the determinism law: both games of a
pair are the same game with the seats relabelled, so every pair scores 1-1 and no
LLR is defined (D-156). **A non-degenerate Run 3a is a FULL STOP** — rule 4 would
be broken, and with it the pair-level verdict unit Run 1 rests on (D-154). It is
a strictly stronger gate than Run 0's four-opening smoke: same book, budget, turn
cap and worker count as Run 1. No early stop is possible on a degenerate sample,
so all 4000 games are played.

Reported per book: `first_player_wins k of n decided`, `distinct_n`,
`capped_fraction`, per-side compute, and a **Wilson 95 % interval on DISTINCT
games** — the estimator named now, not later: **±2.2 pp** at 2000 distinct games
(3a), **±2.5 pp** at 1591 (3b), both at p = ½. One ADR line per book.

Three clauses those lines carry whatever the numbers are:

1. **The rate is a property of the instrument** and is quoted with its budget.
   §2c.7 withdraws revision 1's claim that the rate was *measured* to move with
   the budget, but the caution stands.
2. **The two books do not estimate the same quantity and are never averaged.** 3a
   samples openings uniformly; 3b's are the ones humans reached, and D-145
   measured those lopsided (0.10–0.91 across classes with ≥10 games, 26 of 61
   outside 0.35–0.65). 3b is the first player's advantage *conditioned on a
   human-reached opening*.
3. **The sample is the distinct games**, not the games played.

**A forfeit invalidates the number and the number still prints.** `score::tally`
counts any non-capped result as `decided`, and a forfeit is a decided result, so
a forfeited run reports a first-player rate composed of protocol failures on the
very line this run exists for; only the `forfeits` count and the `invalid_forfeit`
token, on neighbouring lines, say so. Run 3 takes Run 1's forfeit rule: not a
measurement, reported rather than discarded, re-run after the cause is found.
Related: `distinct_n` counts all games while `decided` is the rate's denominator;
the two coincide only while `capped_fraction` is 0, and if the cap ever binds
partially the Wilson denominator is stated explicitly in the writeup.

Also expected and not an error: **Run 3's `llr_game` will read far past its
boundary** while the verdict is correctly `inconclusive_degenerate` — the file
marks the unit only via `verdict_unit pair`, and D-154 makes the game-level LLR a
diagnostic.

## 7. Blanks the runbook did not have

- **`turn_cap = 40`** (all runs). Capped games are not inert: a mass point at a
  half shrinks σ and, for a fixed decisive record, accelerates the LLR toward H1
  (D-157). 40 is three times the longest of sixty probe games, none of which
  capped — but that probe was radius-3 self-play on the primary book only, so
  neither the heterogeneous pairing of Run 1 nor 3b's four-turn corpus openings
  are measured. `capped_fraction` is reported so a run in which it bound cannot
  be read as one in which it did not.
- **`hang_timeout_ms = 120000`** (all runs). Liveness only (D-159), about 24× the
  worst single search over the whole fixture at this budget (5.1 s), single-threaded.
- **The slot assignment** (§3), which the runbook's lettering would have inverted.
- **The 100-pair floor** (§3).
- **The inert `[sprt]` blocks** in the Run-3 configs, complete because the schema
  admits no absent key (rule 1); no LLR is defined on a self-match sample.

## 8. Order, operating rules, and outputs

**Order: 3a → 1 → 3b → 2.** 3a first because it is the determinism gate Run 1's
verdict unit depends on. Run 2 is independent of Run 1's outcome by §5's pinned
config.

**The digests are taken once and the engines are respawned from disk for every
game** — `identity_of` runs before the first game; `schedule::one_game` re-execs
per game (D-164). A mid-run edit is therefore silent: RED-TEAM swapped
`radius = 2` for `radius = 1` eighteen seconds into a live run and got exit 0 and
a report attesting radius 2. So, for the duration of this package:

1. **No document under `configs/` is edited while any run is live.**
2. **`target/release/pistol` is not rebuilt while any run is live** — including
   Run 2's `CARGO_PROFILE_RELEASE_DEBUG` build, which writes a different binary to
   the same path both arena configs name. Run 2 runs last for this reason, and
   the plain `--release` binary is rebuilt before any re-run.
3. **Run 1's outcome is landed only after all four runs are complete**, because
   the H1 action edits `instrument_v0.toml`, which is engine B of Run 1 and *both
   seats* of Runs 3a and 3b.

**Outputs.** One report per run, written outside the repository to
`~/Work/pistol-wp13/`, named `wp13_run3a.matchlog`, `wp13_run1.matchlog`,
`wp13_run3b.matchlog`. The `.matchlog` extension is not decoration: `.gitignore`
already ignores it, whereas a report written as `.txt` or `.report` passes both
`.gitignore` and `tools/artifact_check.sh` and is committable — rule 8 breached
with no gate firing. Distinct names because `--out` refuses an existing file only
at dispatch: two runs started before either finishes both pass the check and one
silently destroys the other's report.

## 9. Known gaps this pre-registration does not close

1. **The eval weight table is not identified by content anywhere.**
   `identity_of` digests the binary and the engine config; nothing digests
   `eval_v0_weights.toml`, and the handshake emits only the backend name. RED-TEAM
   ran two workspaces differing solely in that file and got byte-identical
   `experiment_sha256` and every other digest, while `nelo_pair` moved 98 points
   and `first_player_wins` moved 25 points. The path is relative and resolves
   against the working directory, and CLAUDE.md's own mutation-testing rule means
   a second worktree will exist. **Mitigation for these runs: every run is started
   from the repository root of this checkout, and the ADR lines name the
   repository revision.** The fix — digesting the weights file, or emitting
   `id weights_sha256` from the handshake — is a code change and its own work
   package, and until it lands this is a real limit on what a report proves.
2. **No offset knob**, so a confirmatory disjoint sample from the same book is not
   expressible; §3's confirmatory run uses the other book instead.
3. **Not reached by any review round:** the flamegraph itself; the full 4000-game
   run with the real engine at four workers (RED-TEAM's scale test used the stub);
   `hang_timeout_ms` under real four-worker contention; D-174's remaining protocol
   items; and any run above 8 workers.

## 10. After WP-1.3

Results return to the planning session as ADR lines. Then WP-1.4 (movetime
ceiling, D-95) with the pistol-api stdio-shim spec ADR riding along, then WP-1.5
(threat core, carrying its D-124 visibility obligation).
