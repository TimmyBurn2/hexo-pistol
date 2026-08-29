# WP-1.9 — design: stage Q, the quiet candidate tier and its `quiet_top_k` cap

**Revision 1. UNREVIEWED AT DISPATCH; this revision governs no run until a
fresh-context REVIEW-design passes it.** Written at `11c102e` on `dev`.

**Designation collision, recorded rather than left to be rediscovered.**
`docs/ROADMAP.md:260` already spends **WP-1.9** on *eval window-map storage*
(D-225, renumbered onto that designation by D-249). D-471 and this session's
dispatch spend **WP-1.9** on the stage-Q cap. Two work packages, one
designation — the exact defect D-249 exists to have fixed once. By CONTENT
this package is **WP-1.5c** (`docs/ROADMAP.md:124`, D-315: "the quiet stage,
its widening schedule, and dominance pruning"). This document keeps the
dispatch's label because D-471 is the operator's own newer line, and asks for
one ADR line settling which designation survives. **Nothing in this design
depends on the answer.**

---

## 0. THE FINDING THAT MOVES THIS WP'S PREMISE, MEASURED BEFORE ANYTHING WAS WRITTEN

The dispatch's Scope and its Expected-effect paragraph describe two different
mechanisms with opposite signs, and the repository's own bytes plus one
measurement settle which is which.

**Stage Q is a quiet tier ADDED BEYOND Tier T, not a cap cut into it.** The
repository states this in four independent committed places:

- `crates/pistol-search/src/params.rs:40` — "Stage Q, **the quiet tier beyond
  Tier T**, is deferred".
- `crates/pistol-engine/src/config.rs:224-226` — `quiet_top_k` is "the first
  batch's **quiet-cell** count"; `widen_schedule` is "cumulative **quiet-cell**
  batch boundaries after the first".
- `crates/pistol-solver/tests/wp15b_census.rs:213-215`, a gate-run test —
  the emitted set on a batched row is modelled as
  `t.len() + quiet.min(QUIET_TOP_K)` where `quiet = |ball \ TierT|`: **Tier T
  whole, plus at most K quiet cells.**
- `configs/tactical_staged_v0.toml:12-18` and `configs/gate_staged_v0.toml:12-15`
  — both DISABLE the cut by setting `quiet_top_k` **above the whole ball**
  (1024 against a bound of `18*17 = 306`; 128 against `6*17 = 102`), so that
  "no legitimate cell is ever cut". K is sized against the **ball**, never
  against Tier T.

So arming stage Q **widens** the candidate set. It does not narrow it.

**MEASURED this session** (`artifacts/wp19_design_census_v1.txt`, sha256
`37486417…`; instrument revision `41b56fae` — a `git stash create` over
`11c102e` carrying a throwaway test plus seven scratch counters, reverted
after the run; committed `instrument_staged_v0.toml` policy values,
`Stop::Nodes(50_000)`, release, single thread):

| | CORPUS (`bench_positions_v1`, 24 pos) | SPREAD (`spread_v1`, 4 pos) |
|---|---|---|
| row shares | filtered 0.2679, batched 0.7312, win-now 0.0000, overload 0.0009 | batched 1.0000 |
| emitted width, BATCHED rows | **mean 31.208**, max 93 | mean 751.456, max 1791 |
| emitted width, FILTERED rows | mean 1.912 | — |
| quiet-ball safety-net rate | **66 / 108 662 = 0.000607** | 2468 / 4015 = 0.614695 |
| safety-net width | mean 78.121 | mean 1218.334 |
| quiet pool `\|ball \ TierT\|`, non-safety-net batched rows | **mean 63.557** | mean 1097.613 |
| stage Q at K=16 moves batched mean | **31.208 → 47.198 (×1.512)** | 751.456 → 757.621 (×1.008) |

Three consequences, each of which the dispatch's Expected-effect paragraph
gets backwards, and each stated here so no reader has to discover it from the
bench:

1. **Direction is NEGATIVE for nps and for depth at equal nodes on the corpus.**
   The batched row is 73% of dispatches and it grows ×1.51. `nps` ON/OFF will
   be **below 1**, and completed depth at 50 000 nodes will fall or hold. The
   bench bracket in §8 is registered with that sign, before measuring.
2. **D-192's hotspot GROWS.** `staged::delta_rank` pays one `Eval::delta` per
   emitted cell; stage Q adds K more per batched node. The dispatch names
   D-192's 76% ordering hotspot "as the cost being cut"; it is the cost being
   **increased**. Nothing in this WP cuts it.
3. **The sealbot-shaped lever is much smaller than the framing assumes, and is
   now measured rather than assumed.** `docs/research/sealbot_notes.md:56-66`
   caps *the radius-2 ball* at 15 (root 20); pistol's radius-2 ball measures
   **78.1** cells and its committed staged policy already emits **31.2**. The
   "pistol at r=3 searches thousands" sentence that framing rests on is a
   **RADIUS-3** number, superseded by WP-1.5b's staged policy (D-386). The
   4–6× nps gap the anchor measured therefore coexists with a 31-cell width,
   so width is not its dominant term.

**This WP is still worth running, for the reason the ROADMAP already gives and
not for the reason the dispatch gives.** `docs/ROADMAP.md:124-138` and D-315:
WP-1.5b's SPRT delta shrank by exactly the axis D-310 excised, this is the
only package where that debt can be paid, and **D-310 flips if it is never
scheduled**. The hypothesis this WP tests is *a wider, shallower, better-
advised search is stronger* — not *a narrower search is faster*. §7's
expectation is written accordingly.

---

## 1. The two facts the dispatch asked this design to quote

- **The knob exists in config.** `crates/pistol-engine/src/config.rs:224`,
  `quiet_top_k: u64` inside `CandidatePolicy::Staged`; validated at
  `crates/pistol-engine/src/validate.rs:94-97` (`must be at least 1`) and
  cross-validated against `widen_schedule` at `validate.rs:104-121`; echoed in
  the handshake id at `crates/pistol-cli/src/bin/pistol.rs:159`.
- **The search does not read it.** It is destructured and DISCARDED at
  `crates/pistol-engine/src/instance.rs:200` (`quiet_top_k: _`), and
  `crates/pistol-search/src/params.rs:47-58` says so in `StagedParams`'s own
  doc — "Deliberately narrower than the config document's
  `[search.candidate_policy]` table … those two govern stage Q's widening
  schedule, which this D-scope does not implement". `git grep quiet_top_k --
  crates/pistol-search/src` returns two COMMENTS
  (`params.rs:48`, `staged.rs:228`) and no code.

---

## 2. Semantics — the four questions the dispatch put to this design

### 2.1 K counts CELLS, and specifically QUIET cells

`StagedSet::cells` is a `Vec<Coord>`; a turn is two sequential same-side plies
with the intra-turn phase in the key (**D-9**), so a *pair* is never a unit the
candidate set holds — it is produced by the recursion, one ply at a time, with
the alpha-beta cutoff between the two plies as the main pruning lever. There is
nothing at this seam for K to count but cells. **K counts cells of the QUIET
POOL** — `ball \ TierT` — not cells of the emitted set: that is what "the first
batch's quiet-cell count" says at `config.rs:226`, what `wp15b_census.rs:213`
computes, and what makes both "cut disabled" configs' ball-sized bounds correct.

### 2.2 Where the cap applies: the BATCHED and BATCHED-lost rows, and only there

The node protocol's rows split exactly along "is anything forced":

| row | `forced` | stage Q |
|---|---|---|
| WIN-NOW | `== cells.len()` | **exempt by construction** — the whole set is Tier F |
| FILTERED | `== cells.len()` | **exempt by construction** — the whole set is the cover union |
| BATCHED / BATCHED-lost | `== 0` | Tier T ∪ top-K(`ball \ TierT`) |
| OverloadReturn | — | no set is emitted |

"Stages F and T output never truncated" is therefore satisfied *by
construction* and not by a guard: on the two forced rows the unforced range is
empty, and stage Q appends to the unforced range only. On the batched rows Tier
F is provably empty (`staged.rs`'s own `batched` doc) and **Tier T is emitted
whole, ahead of every quiet cell, and is never truncated**.

### 2.3 Ply scope: EVERY ply, root and PV included

`WPQ_seed.md` §7.2's option W-E exempts the root and every PV node from the
schedule. **That exemption does not transfer, and the reason is that W-E is a
CUT and this is an ADDITION.** W-E's two structural defects are (i) a capped
root cannot play a candidate it never generated and cannot widen, and (ii) a PV
node that truncates stores `Bound::Exact` over a SUBSET, which is a lower bound
only. Both are defects of searching **less** than the incumbent. Under §2.2's
rule the emitted set is a **superset** of today's at every batched node where
Tier T is non-empty — which §0 measures at **99.94%** of corpus batched rows —
so neither defect is reachable: the root can play strictly more, and every
stored bound is computed over the same set the search is now defined against.
Exempting the root and PV nodes would instead mean handing them the **entire**
quiet ball (mean 78.1 cells on the corpus, 1218 on spread) at exactly the most
expensive nodes, which is a cost with no soundness argument buying it.

### 2.4 Tie-break: `delta_rank`'s existing stable sort, unchanged

The quiet pool is built by `candidates::within_radius`, which returns cells
**ascending** (it drains a `BTreeSet`). `staged::delta_rank` sorts by
`Reverse(Eval::delta)` with `sort_by_key`, which is stable, so equal-scoring
cells keep ascending coordinate order — D-5/D-7's tie-break, already in the
tree, reused rather than restated. Stage Q ranks the quiet pool with the same
function and truncates the ranked list, so the K-th and (K+1)-th cells are
separated by a total order that has no time, no thread and no hash iteration in
it. Rule 4 holds by construction, and §9 seats it anyway.

---

## 3. THE SAFETY NET IS NOT CAPPED BY THIS WP, AND THAT IS A RULE-BOUND REFUSAL

When Tier T is empty the batched row falls back to the whole quiet ball
(`staged.rs:222-231`). Capping *that* set is the one place stage Q would
remove a cell the incumbent searches — a **forward prune of the LMR family**,
which `WPQ_seed.md` §7.2 names as such. §0 measures what it would buy: on
SPREAD, 61.5% of batched rows at mean width **1218.3** would fall to K, a ~40×
narrowing on precisely the D-95 / WP-1.4 class whose committed baseline is
*completed depth 0 at `movetime 500`*.

**It is still refused here.** CLAUDE.md: a named decision with more than one
viable option is settled by an OPTION MATRIX attacked by a fresh-context
DECISION-RED-TEAM **before** selection, and an option adopted without one is
the same breach as silent architecture drift. `WPQ_seed.md`'s own M2 debt note
records that the widening-schedule matrix **has never been authored in the form
its adopted candidate takes** — it is a fresh matrix, not a recovery — and this
session's dispatch forbids a red team outright ("No red team"). A forward prune
cannot be selected under those two facts together. Leaving the safety net alone
also makes this WP's diff a **pure superset** everywhere, which is what lets
§2.3 dispense with W-E's exemptions and lets §9 drop the TT truncation rule
entirely.

**Scheduled, not merely licensed** (§10 records the ADR line): the safety-net
cap, its matrix, its DECISION-RED-TEAM and its own SPRT are the next package,
and it now starts from measured numbers instead of from the seed's prose.

---

## 4. The gate

A new boolean `stage_q` in `[search.candidate_policy]`, `false` in **every**
committed config — the WP-1.7 precedent (`killers`/`history`/`countermove`),
same shape, same reason. `false` reads `quiet_top_k` not at all and the search
is byte-identical to `11c102e` by construction; `true` arms §2.2.

**A gate is required and `quiet_top_k = 0` cannot serve as one.** Under §2.1's
semantics a LARGE K means "admit the whole quiet tier", so the two configs whose
headers disable the cut with a large K — `tactical_staged_v0.toml` (1024) and
`gate_staged_v0.toml` (128) — would become **maximally widened** the moment the
knob were read, silently changing gate 8's `require 20` derivation and gate
11's cases. Those headers were written forward, for a stage Q that did not
exist, and arming the knob without a gate would make them live at the worst
possible value. The gate is what keeps them inert.

`widen_schedule` stays validated-and-unread. It is the second knob, and it is
out of scope by the dispatch.

---

## 5. Where the code changes

| file | change |
|---|---|
| `crates/pistol-engine/src/config.rs` | `stage_q: bool` field on `CandidatePolicy::Staged` |
| `crates/pistol-engine/src/instance.rs` | stop discarding `quiet_top_k`; pass it and `stage_q` through |
| `crates/pistol-search/src/params.rs` | `StagedParams` gains `stage_q: bool` and `quiet_top_k: u64` |
| `crates/pistol-search/src/staged.rs` | `batched()` appends the ranked, truncated quiet pool when `stage_q` and Tier T is non-empty |
| `crates/pistol-search/src/info.rs` | `StageCounters` gains `quiet_cells_admitted: u64` and `quiet_rows: u64` — the calibration and the bench both read mean admitted width from them, and a counter is what stops §7 quoting an estimate |
| `crates/pistol-cli/src/bin/pistol.rs` | the handshake id already prints `quiet_top_k`; it gains `stage_q` |
| every `configs/*staged*.toml` | `stage_q = false` |
| `configs/instrument_staged_stageq_v0.toml` | NEW — the SPRT seat: `instrument_staged_v0.toml` with `stage_q = true` and the calibrated K. **Named `stageq`, not `q`**: `configs/instrument_staged_q_defensive_only_v0.toml` already spends `_q_` on WP-1.6's QUIESCENCE gate and two different `q`s in one config family is the D-249 defect at file scale |

---

## 6. Determinism (rule 4, D-7)

Stage Q adds no time source, no thread and no hash iteration to a choice path.
The quiet pool is a `BTreeSet` drain (ascending), ranked by a stable sort on
`Eval::delta`, and truncated by index. `tools/determinism.sh` runs the ON seat
as its own third seat, the shape gate 9 already uses for
`gate_staged_heuristics_v0.toml`.

---

## 7. CALIBRATION — registered here, BEFORE the sweep is run

- **Grid:** K ∈ {2, 4, 8, 16, 32}. 16 is `U3_tier_t.md` §10's registered value
  and the census's `QUIET_TOP_K`; the grid brackets it by ×4 either way. The
  pool exceeds 32 at essentially every batched node (§0: mean 63.6), so every
  grid point binds.
- **Instrument:** `target/release/pistol` at the impl revision, `go nodes 50000`,
  over all 24 entries of `bench_positions_v1.txt`, once per K plus once at
  gate-off. Reported per K: per-position completed `depth_turns`, per-position
  nps, corpus-aggregate nps, and mean admitted quiet width from §5's counters.
- **THE SELECTION RULE, STATED BEFORE THE SWEEP RUNS AND NOT RE-READ AFTER IT
  (D-374):** **the LARGEST K on the grid such that no one of the 24 positions
  loses more than one completed `depth_turns` against the gate-off seat. If no
  grid point satisfies that, K = 2.** Largest-not-smallest because the axis
  under test is *how much quiet advice the search can afford*, and the strength
  bet is monotone in K while the cost is monotone in K too; one completed depth
  at any single position is the most this WP is willing to spend for it.
- The table lands as `artifacts/wp19_calibration_v1.txt`, sha-recorded, and the
  chosen K and this rule are QUOTED — not paraphrased — into the prereg.

## 8. RULE-5 BENCH — registered here, BEFORE measuring

- **Pre-registered hotspot:** `staged::delta_rank`'s `Eval::delta` roundtrip,
  paid once per emitted cell (D-192 measured that roundtrip at 76.27% of
  profiled ordering stacks). Stage Q adds K cells per batched node to it, and
  batched rows are 73.12% of dispatches (§0). **This is a cost this WP ADDS;
  the expected gain is not here, it is in the SPRT.**
- **Instrument:** the WP-1.7 §7 command block verbatim, both bands of
  `bench_positions_v1.txt` (≤17 stones early, else late), 5 reps,
  per-band aggregate nps = Σ nodes / Σ median-time-per-position, ON =
  `configs/instrument_staged_stageq_v0.toml` at the §7 K, OFF =
  `configs/instrument_staged_v0.toml`. Plus one non-gating pass over
  `spread_v1.txt`, reported because §0 predicts stage Q is nearly inert there
  (×1.008) and a reader will otherwise assume the D-95 class moved.
- **EXPECTED BRACKET, and its sign is the one §0 measured, not the one the
  dispatch assumed: nps ratio ON/OFF in `0.55 .. 0.95` in both corpus bands.**
  ESTIMATED from §0's ×1.512 batched-width growth weighted by the 0.7312
  batched share, against a search whose per-node cost is not purely linear in
  emitted width (the TT and the alpha-beta cutoff between the two intra-turn
  plies both damp it).
- **ABORT THRESHOLD: ratio < 0.40 in EITHER corpus band.** Below that the
  widening is too expensive to be worth an SPRT at any K on the grid; the
  numbers are recorded as a finding, the committed config does not move, and
  **no SPRT is run** — the WP-1.8c precedent (D-465), where a registered
  bracket aborted and the run that was conditional on it was not taken.
  A measured structural floor is a finding, not a failure (rule 5).
- **IQR gate:** per position, the IQR of its 5 per-rep times ≤ 10% of that
  position's median (D-215/D-362). A position exceeding it WITHHOLDS the
  verdict and is re-measured, both configs, before any ratio is read.
- **Time-to-depth** is reported for both seats per rule 5 and is not a gate
  here; §7's selection rule has already spent it.

## 9. Tests, and the mutants each must kill

| test | pins |
|---|---|
| `gate_off_is_byte_identical_to_the_committed_search` | both determinism fixtures, ON-config-with-`stage_q=false` vs committed: identical bestmove AND node count |
| `the_quiet_cap_admits_exactly_k_cells` | at a batched node with pool > K: emitted width == `\|TierT\| + K` |
| `k_and_k_plus_one_differ_by_exactly_the_next_ranked_quiet_cell` | the boundary, as a set difference — **the off-by-one mutant dies here** |
| `tier_t_is_emitted_whole_and_ahead_of_every_quiet_cell` | `cells[..\|TierT\|]` is exactly Tier T — **the "truncation applied to stage T" mutant dies here** |
| `the_forced_rows_emit_the_same_set_with_the_gate_on` | WIN-NOW and FILTERED sets identical ON and OFF — **"truncation applied to a forced row" dies here** |
| `the_quiet_cap_is_stable_across_runs` | two runs, identical emitted sets; and equal-delta quiet cells keep ascending order — **the order-dependent tie-break mutant dies at the determinism seat** |
| `the_safety_net_is_never_capped` | Tier T empty ⇒ emitted width == ball width, gate ON — §3's refusal, seated |
| `a_stage_q_config_is_refused_without_quiet_top_k` | rule 1: no code-side default |

**Mutation receipts** (separate worktree, D-469 export before removal): the
three the dispatch names — cap boundary `>` for `>=`; tie-break made
order-dependent; truncation applied to the Tier-T prefix — plus a fourth this
design adds: **truncation applied to the safety-net set**, which must die at
`the_safety_net_is_never_capped`, because §3's refusal is a designed rule and
an undefended designed rule is a hedge.

## 10. ADR lines this design records

1. Stage Q is a quiet tier ADDED beyond Tier T; K counts quiet cells; the
   dispatch's narrowing framing is refuted with §0's numbers.
2. The safety net is NOT capped, and why the refusal is rule-bound rather than
   cautious; the cap is SCHEDULED as its own matrix-plus-red-team package.
3. Root and PV are NOT exempt, and why W-E's exemption does not transfer.
4. The `stage_q` gate exists because a large `quiet_top_k` inverts meaning
   between the two readings and would silently arm two gate configs.
5. The WP-1.9 / WP-1.5c designation collision, for the operator to settle.

## 11. Out of scope

`widen_schedule` and every widening mechanism; dominance pruning; the
safety-net cap (§3); eval; solver; TT; ordering heuristics (WP-1.7 closed h0);
any second tunable.
