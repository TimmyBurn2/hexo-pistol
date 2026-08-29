# WP-1.9 — design: stage Q, the quiet candidate tier and its `quiet_top_k` cap

**Revision 2.** It closes the five BLOCKING and six MAJOR findings of
revision 1's fresh-context REVIEW-design (`sessions/WP-1.9/wp19_design_REVIEW.md`,
sha256 `32c8172d…`, against revision 1 at `0d38d3d`, **VERDICT: FAIL**). This is
the one fix round the dispatch allows; this revision governs no run until the
re-review passes it. Revision 1 was written at `11c102e`.

**THE REVIEW CHANGED THIS WP'S EXPECTED TERMINAL STATE, AND THIS REVISION SAYS
SO ON ITS FACE RATHER THAN LETTING A BENCH DISCOVER IT.** The reviewer did not
only read: it implemented revision 1's §2 semantics in a detached worktree, built
four release variants, and ran §8's own instrument. **Stage Q as this WP scopes it
costs an nps ratio of 0.32–0.48 at every point of §7's grid, and building the
quiet pool alone — emitted set unchanged — already costs 0.729/0.603.** Revision
1's bracket `0.55 .. 0.95` is unreachable by a correct implementation. §8 is
therefore rewritten as a REPLICATION with a threshold inherited from precedent,
not as a discovery with a threshold derived from a prediction; §0.4 states the
consequence for the WP's arc.

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

**A fifth citation, stronger than the four above, which revision 1 missed and
the review supplied** — `docs/experiments/U3_tier_t.md` §10: *"The first batch is
`quiet_top_k` quiet cells. Tier F and Tier T are always emitted whole and are not
counted against it."* That is the registered semantics in one sentence, and it
settles §2.1 and §2.2 without argument.

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
   caps *the radius-2 ball* at 15 (root 20); pistol's committed staged policy
   already emits **31.2**. (Revision 1 put the ball at 78.1 here; REVIEW-design
   m8 is right that 78.121 is measured over the **66 safety-net rows** — sparse
   early positions — not over the population being compared. On non-safety-net
   batched rows the ball is the quiet pool 63.557 plus the in-ball part of Tier T,
   **≈ 85**. The direction survives; the number revision 1 quoted was not the one
   its sentence claimed.) The
   "pistol at r=3 searches thousands" sentence that framing rests on is a
   **RADIUS-3** number, superseded by WP-1.5b's staged policy (D-386). The
   4–6× nps gap the anchor measured therefore coexists with a 31-cell width,
   so width is not its dominant term.

**Revision 1 claimed here that this WP pays D-310's strength debt. THAT CLAIM IS
WITHDRAWN** — see §3, and §0.4 for the measurement that forces it. `WPQ_seed.md`
defines that debt on the class where *"Tier F and Tier T are both EMPTY"*, which
is the safety-net row §3 declines to touch, and stage Q is MEASURED inert there.
What this WP is worth is stated without that borrowed justification: it arms the
quiet tier behind a gate, and it establishes **by end-to-end measurement rather
than by argument** what the tier as specified costs — which is what makes the
follow-up package's scope decidable instead of speculative, and which is exactly
the WP-1.8b→1.8c arc's own shape (D-465). The hypothesis it tests is *a wider,
shallower, better-advised search is stronger*, not *a narrower search is faster*;
§8 registers the threshold that decides whether that hypothesis ever reaches an
SPRT.


### 0.4 WHAT THE REVIEW MEASURED, AND WHAT IT DOES TO THIS WP'S ARC

Revision 1's REVIEW-design implemented §2's semantics and benched them. Its
numbers are **MEASURED**, at revision 1 (`0d38d3d`), in a detached worktree,
release, single thread, `configs/instrument_staged_v0.toml`, `go nodes 50000`,
both bands of `bench_positions_v1.txt`, 5 reps, per-position median,
band-aggregate nps = Σ nodes / Σ median-time — §8's own instrument. IQR gate
clean throughout (largest observed 2.8 % against the 10 % convention).

| seat | early nps | late nps | ratio early | ratio late |
|---|---|---|---|---|
| gate OFF | 253 628 | 210 623 | 1.000 | 1.000 |
| pool built only, **emitted set unchanged** | 184 640 | 126 874 | **0.729** | **0.603** |
| stage Q, K = 2 | 121 418 | 82 255 | **0.479** | **0.391** |
| stage Q, K = 16 | 103 084 | 89 636 | **0.406** | **0.426** |
| stage Q, K = 32 | 85 140 | 67 203 | **0.336** | **0.319** |

**The hotspot revision 1 registered was wrong by ~4×, and the correction is
arithmetic on this design's own two numbers.** `delta_rank`
(`staged.rs:287-295`) scores **every cell of the vector it is handed**; to emit
*the top-K of the quiet pool by delta* the pool must be scored in full and only
then truncated. K caps what is EMITTED and SEARCHED, never what is SCORED. So
the per-batched-node roundtrip count goes 31.208 → 31.208 + **63.557** = 94.765
(**×3.04**), not 31.208 + 16 (×1.512). Against D-192's own 76.27 % share,
Amdahl gives `0.2373 + 0.7627 × 3.04 = 2.556×` time, **nps ≈ 0.391** — which is
what the table measures. **And a second hotspot revision 1 registered nowhere**:
building the pool needs `candidates::within_radius` at every batched node, today
called on 66 of 108 662 corpus batched rows (§0's 0.000607) and under stage Q on
99.94 % of them. The isolation build above pays *only* that and already costs
0.729/0.603.

**MEASURED, the WP as scoped is inert on the class `WPQ_seed.md` calls the debt.**
Stage Q at K = 16 on `spread_v1.txt` at `go nodes 50000`:

| stones | OFF nps | ON nps | ratio | OFF depth | ON depth |
|---|---|---|---|---|---|
| 11 | 437 570 | 449 996 | 1.028 | 1 | 1 |
| 21 | 446 754 | 422 329 | 0.945 | 1 | 1 |
| 51 | 419 535 | 408 594 | 0.974 | 1 | 1 |
| 99 | 404 350 | 372 588 | 0.921 | 1 | 1 |

**THE CONSEQUENCE, STATED HERE SO NO LATER SECTION HAS TO DISCOVER IT.** This
WP's expected terminal state is **a rule-5 bracket abort and no SPRT** — the
WP-1.8c shape (D-465), where a registered bracket fired and the run made
conditional on it was not taken. §8 is written as a replication with a
precedent-inherited threshold for exactly that reason: a pre-registration whose
answer is already known is not a measurement (`docs/process.md`, closing
sentence), and the honest response is to say so rather than to re-register a
number chosen to be survivable.

**AND THE SEALBOT-SHAPED WIDTH IS UNREACHABLE IN THIS WP BY CONSTRUCTION**, which
is the review's own observation and belongs here: under §2.1 the emitted set can
never fall **below Tier T** (corpus mean 31.208), while sealbot's cap is 15 total,
root 20 (`sealbot_notes.md:60-64`). No K on any grid reaches it. The only place a
sealbot-shaped width could come from is the safety-net cap §3 declines — which is
also, per `WPQ_seed.md`'s own words, where D-310's debt is measured. §3 now says
that plainly instead of around it.

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

"Stages F and T output never truncated" is satisfied by a **row dispatch**, and
saying so precisely matters — revision 1 said "by construction and not by a
guard" and REVIEW-design m1 is right that this was wrong. Appending to an empty
unforced range is not a no-op, it is an append, and on the FILTERED row it would
violate §5.3's *"the cover union, and nothing below it"*. **The actual
protection is that stage Q's code lives inside `batched()`, which
`staged_candidates` calls on two rows only (`staged.rs:139-153`)** — a guard, and
one §9's `the_forced_rows_emit_the_same_set_with_the_gate_on` therefore defends
something. On the batched rows Tier
F is provably empty (`staged.rs`'s own `batched` doc) and **Tier T is emitted
whole, ahead of every quiet cell, and is never truncated**.

### 2.3 Ply scope: every ply, with ONE named exclusion that is a correctness fix

Stage Q applies at every ply, **except ply 0 when `Run::root_restrict` is
`Some`**, where it is not applied at all. The exclusion is not a preference. It
closes a defect revision 1 did not see and REVIEW-design found:

`pvs.rs:336-346` restricts the root's candidates to a winning defender proof's
zone and **fails open** if the restriction would empty the set:

```rust
let forced_intact = set.cells[..set.forced].iter().all(|c| zone.binary_search(c).is_ok());
if forced_intact {
    let unrestricted = set.cells.clone();
    set.cells.retain(|c| zone.binary_search(c).is_ok());
    if set.cells.is_empty() { set.cells = unrestricted; }   // the fail-open
}
```

On a BATCHED row `set.forced == 0`, so `forced_intact` is vacuously true. Take a
root with Tier T non-empty and `TierT ∩ zone = ∅`. **Gate OFF:** `retain` empties
the set, the fail-open fires, the root searches all of Tier T. **Gate ON:** the
set is `TierT ++ topK(quiet)`; if any admitted quiet cell lies in the zone the
set is not empty, the fail-open does **not** fire, and the root searches
`topK(quiet) ∩ zone` **with every Tier-T cell removed**. Adding cells has
defeated a safety valve and produced a strictly smaller search at ply 0 — W-E's
own structural defect (i), at the node where the move is chosen and where D-124
says no oracle catches a mistake.

Reachability, stated rather than hand-waved: `root_restrict` is set only by a
winning defender proof (`search.rs:283-296`), which needs
`[solver] on_search_path = true` — `false` in every committed config but
`gate_staged_solver_v0.toml` and `bench_wp18c_solver_on.toml`. With
`stage_q = false` everywhere the pairing is inert today. **It is excluded anyway,
because the WP proposes an ON determinism seat and an ON SPRT seat and nothing
else forbids the pairing**, and because §3 and §9 spend the superset property and
a property with a known counterexample cannot be spent.

**The superset claim, restated with that path named and no longer as a
universal:** at every node where stage Q applies, the emitted set is a superset
of the gate-off set — Tier T whole, then quiet cells appended — **and ply 0 under
`root_restrict` is excluded from stage Q precisely so that this holds wherever
stage Q runs.** That is what lets §9 drop W-E's TT truncation rule: no stored
bound is ever computed over a subset of what the gate-off search would have seen.

**The remaining ply-scope axis is NOT settled here, and this design says so
rather than settling it by argument in a paragraph.** REVIEW-design MAJOR 3 is
correct that {K at every ply} versus {full pool at root/PV, K elsewhere} versus
{non-PV only} is a named decision with more than one viable option, which
CLAUDE.md settles by an OPTION MATRIX attacked by a fresh-context
DECISION-RED-TEAM before selection — the same rule §3 invokes to decline the
safety-net cap, and applying it to the declined option but not the adopted one
would be the inconsistency the review names. The dispatch forbids a red team, so
the matrix cannot be run in this WP. **The axis is therefore DEFERRED to the same
follow-up package that owns the safety-net cap and M2** (§10 item 6), and what
this WP ships is the option that is correct-by-exclusion above. **This defers no
conclusion**: §0.4 registers that the WP's expected terminal state is a bracket
abort with no SPRT, and under an abort the ply-scope choice licenses nothing
anyone may conclude — D-424's own test for whether a distinction is one.

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

## 3. THE SAFETY NET IS NOT CAPPED BY THIS WP — A RULE-BOUND REFUSAL, AND IT COSTS THIS WP THE DEBT IT CLAIMED TO PAY

When Tier T is empty the batched row falls back to the whole quiet ball
(`staged.rs:222-231`). Capping *that* set is the one place stage Q would remove a
cell the incumbent searches — a **forward prune of the LMR family**, which
`WPQ_seed.md` §7.2 names as such. §0 measures what it would buy: on SPREAD,
61.5 % of batched rows at mean width **1218.3** would fall to K, on precisely the
D-95 / WP-1.4 class whose committed baseline is *completed depth 0 at
`movetime 500`*.

**It is refused here, and the refusal is rule-bound.** CLAUDE.md: a named
decision with more than one viable option is settled by an OPTION MATRIX attacked
by a fresh-context DECISION-RED-TEAM **before** selection, and an option adopted
without one is the same breach as silent architecture drift. `WPQ_seed.md`'s own
M2 debt note records that this matrix **has never been authored in the form its
adopted candidate takes** — a fresh matrix, not a recovery — and this session's
dispatch forbids a red team outright ("No red team"). A forward prune cannot be
selected under those two facts together.

**WHAT THAT REFUSAL COSTS, STATED AGAINST THE SEED'S OWN DEFINITION RATHER THAN
AROUND IT.** REVIEW-design MAJOR 6 is correct and revision 1 argued past it.
`WPQ_seed.md` defines D-310's debt on exactly the class §3 declines:

> "**BASELINE MEASURED** … completed depth 1 / 0 / 0 / 0 at 11 / 21 / 51 / 99
> stones … On this class no length-6 window holds two stones of either colour, so
> Tier F and Tier T are both EMPTY and Staged reduces to Tier Q batching over the
> same ball — the cleanest possible test of the cut alone, with every threat
> mechanism inert. … **That is the debt this stage exists to move.**"

"Tier F and Tier T both EMPTY" **is** the safety-net row. So the excised axis the
seed names is the half this WP refuses, and the half it implements is the one the
seed treats as residual. **MEASURED (§0.4): stage Q at K = 16 moves that class by
nothing** — nps 0.921–1.028, completed depth unchanged at 1 on all four
positions.

**So §0's claim that this package pays D-310's debt is WITHDRAWN.** It does not.
What this WP does is: arm the quiet tier behind a gate, measure its cost
end-to-end, and establish by measurement that the tier as specified is
unaffordable at current eval cost — which is what makes the follow-up package's
scope decidable instead of speculative. D-310's flip clause is **not** discharged
by this WP and §10 records that rather than letting a later reader infer it from
a closure summary.

## 4. The gate

A new boolean `stage_q` in `[search.candidate_policy]`, `false` in **every**
committed config — the WP-1.7 precedent (`killers`/`history`/`countermove`),
same shape, same reason. `false` reads `quiet_top_k` not at all and the search
is byte-identical to `11c102e` by construction; `true` arms §2.2.

**A gate is required and `quiet_top_k = 0` cannot serve as one.** Under §2.1's
semantics a LARGE K means "admit the whole quiet tier", so the two configs whose
headers disable the cut with a large K — `tactical_staged_v0.toml` (1024) and
`gate_staged_v0.toml` (128) — would become **maximally widened** the moment the
knob were read, silently changing gate 8's `require 20` derivation and gate 11's
cases. **Three configs, not two** (REVIEW-design m7): `gate_staged_heuristics_v0.toml`
carries the same disabling `quiet_top_k = 128` at `quiet_radius = 1`, and it is
`tools/determinism.sh`'s third seat. Those headers were written forward, for a stage Q that did not
exist, and arming the knob without a gate would make them live at the worst
possible value. The gate is what keeps them inert.

`widen_schedule` stays validated-and-unread. It is the second knob, and it is
out of scope by the dispatch.

---

## 5. Where the code changes

The list is the enumerated set, not a glob: REVIEW-design MAJOR 5 found revision
1's `configs/*staged*.toml` wrong in **both** directions, and its file list short
by five. Verified this session with
`/usr/bin/grep -l 'kind = "staged"' configs/*.toml` against `ls configs/*staged*.toml`.

| file | change |
|---|---|
| `crates/pistol-engine/src/config.rs` | `stage_q: bool` on `CandidatePolicy::Staged` |
| `crates/pistol-engine/src/validate.rs:81-92` | destructures all ten fields with no `..` — **will not compile** without the new one; also where a `stage_q = true` document's cross-field rule lives |
| `crates/pistol-engine/src/instance.rs:191-200` | stop discarding `quiet_top_k`; pass it and `stage_q` through |
| `crates/pistol-search/src/params.rs` | `StagedParams` gains `stage_q: bool`, `quiet_top_k: u64` |
| `crates/pistol-search/src/search.rs:111-148` | `Searcher::new`'s own bound on `quiet_top_k` — `U3_tier_t.md` §10 requires validation in the engine validator **and again here**, because a `SearchParams` can be built in code and never pass through a document (rule 1) |
| `crates/pistol-search/src/staged.rs` | `batched()` appends the ranked, truncated quiet pool |
| `crates/pistol-search/src/pvs.rs` | the ply-0 `root_restrict` exclusion of §2.3 |
| `crates/pistol-search/src/info.rs` | `StageCounters` gains `quiet_rows`, `quiet_cells_admitted`, `quiet_pool_cells` |
| `crates/pistol-cli/src/bin/pistol.rs:154-159` | destructures; the handshake id gains `stage_q` |
| `crates/pistol-engine/tests/common/mod.rs:62` | embeds a staged TOML |
| `crates/pistol-engine/tests/config_validate_tests.rs:186` | destructures |
| `crates/pistol-engine/tests/config_schema_tests.rs` | the precedent commit `f745d90` touched it for this identical shape of change |
| `tools/determinism.sh:65-75` | the fifth seat (§6), reviewed against `tools/SHELL_CHECKLIST.md` per `docs/process.md`'s tools/ coverage rule |
| `crates/pistol-search/tests/wp19_calibration.rs` | NEW — §7's instrument, with its own governing revision |
| `configs/gate_staged_stageq_v0.toml` | NEW — §6's determinism seat, `gate_staged_v0.toml` + `stage_q = true`, `quiet_top_k = 8` |

**The twelve `kind = "staged"` documents, all gaining `stage_q = false`** —
`bench_wp18c_solver_off.toml`, `bench_wp18c_solver_on.toml`,
`gate_staged_heuristics_v0.toml`, `gate_staged_solver_v0.toml`,
`gate_staged_v0.toml`, `instrument_staged_h_v0.toml`,
`instrument_staged_q_defensive_and_offensive_v0.toml`,
`instrument_staged_q_defensive_only_v0.toml`, `instrument_staged_v0.toml`,
**`instrument_v0.toml`**, `play_staged_v0.toml`, `tactical_staged_v0.toml`.

Two traps the glob would have sprung, both named because they are silent:
the four `arena_*staged*.toml` documents match the glob and have **no
`[search.candidate_policy]` table at all**, so adding the key is a schema error
under `deny_unknown_fields`; and `instrument_v0.toml` plus both
`bench_wp18c_solver_*.toml` are `kind = "staged"` and do **not** match the glob,
so missing them makes every load of them fail. **`configs/instrument_v0.toml`
being staged is worth saying out loud**: the "radius incumbent" of the WP-1.5b
SPRT no longer exists as a radius document.

**`SCHEMA_VERSION` does not move, and that is a sentence rather than a silence**
(REVIEW-design m10): adding a required key to a `deny_unknown_fields` document
invalidates every existing one, which is D-16's case, but the precedent this
design invokes settles it the other way — `f745d90` added three required bools
(`killers`/`history`/`countermove`) and left `schema_version = 3`. This WP adds
one required bool and does the same.

**NEW: `configs/instrument_staged_stageq_v0.toml`** — `instrument_staged_v0.toml`
with `stage_q = true` and the §7 K. Named `stageq`, not `q`:
`instrument_staged_q_defensive_only_v0.toml` already spends `_q_` on WP-1.6's
QUIESCENCE gate, and two different `q`s in one config family is D-249's defect at
file scale.

## 6. Determinism (rule 4, D-7)

Stage Q adds no time source, no thread and no hash iteration to a choice path.
The quiet pool is a `BTreeSet` drain (ascending), ranked by a stable sort on
`Eval::delta`, and truncated by index.

`tools/determinism.sh` gains a **FIFTH** seat, not a third — it already carries
four (`SEATS=(radius staged staged-heuristics staged-solver)`, lines 65-75), which
revision 1 miscounted (REVIEW-design m2). The seat is
**`stage-q configs/gate_staged_stageq_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt`**,
a NEW gate document: `gate_staged_v0.toml` with `stage_q = true` and
**`quiet_top_k = 8`**. It does NOT reuse the gate family's committed
`quiet_top_k = 128`, because at `quiet_radius = 1` that value is §4's own
"maximally widened" case and the seat would then run the most expensive
configuration in the tree at the standing gate budgets (`depth_turns 4`,
`nodes 200000`). **Cost: ESTIMATED 2-4x the `staged` seat's CI time at K = 8**,
from §0.4's measured 2-3x at `quiet_radius = 2`; if the measured seat exceeds
that it takes the solver seat's own precedent (line 74) and carries its own
reduced budgets, which is a reproducibility gate's legitimate shape.

---

## 7. CALIBRATION — registered here, BEFORE the sweep is run

- **Grid:** K ∈ {2, 4, 8, 16, 32}. 16 is `U3_tier_t.md` §10's registered value and
  the census's `QUIET_TOP_K`; the grid brackets it ×4 either way.
- **INSTRUMENT — a harness in the `pistol-search` test tree, `crates/pistol-search/tests/wp19_calibration.rs`, with its OWN GOVERNING REVISION named in the artifact it writes** (`docs/process.md`, "Instrument governing revision"). Revision 1 registered `target/release/pistol` and REVIEW-design BLOCKING 3 found it **cannot** produce one of the four registered outputs: `StageCounters` are not on the line protocol — `info.rs` says so in terms and `report.rs` renders a closed field list. A harness calling `Searcher::search` directly reads them, the shape `wp15b_census.rs` already uses.
- **Reported per K, and at gate-off:** per-position completed `depth_turns`; per-position wall **time-to-depth** to the gate-off seat's completed depth; per-position nps; corpus-aggregate nps; and from the counters, `quiet_rows`, `quiet_cells_admitted`, `quiet_pool_cells`, with **the minimum and the 10th percentile of the per-row pool size, not only the mean** — revision 1 inferred "every grid point binds" from a mean, which is not a criterion any defect could falsify.
- **THE SELECTION RULE, STATED BEFORE THE SWEEP RUNS AND NOT RE-READ AFTER IT (D-374).** Keyed on **time-to-depth**, which D-374's standing lesson makes primary across seats with different candidate policies (*"nps is not a like-for-like unit (nodes differ); ttd registers as primary, nps as context"*), and which revision 1 had no gate on at all:

  > **The largest K on the grid whose corpus-aggregate time-to-depth ratio
  > against the gate-off seat is ≥ 0.5 in BOTH bands. If no grid point reaches
  > 0.5 in both bands, NO K IS SELECTED and §8's abort clause is what the WP
  > acts on.**

  Revision 1's rule — "largest K losing at most one completed depth at any
  position" — is **withdrawn**: REVIEW-design MAJOR 1 measured that it selects
  the grid maximum K = 32 (12 of 24 positions lose exactly one depth, none lose
  two), at a measured nps of 0.336/0.319, because completed depth at a fixed
  50 000-node budget is too coarse an integer to discriminate — to lose *two*
  depths a position must fall 3 → 1, needing roughly another order of magnitude.
  A rule satisfied by every grid point registers the appearance of a constraint
  and constrains nothing. Revision 1's unmarked claim that *"the strength bet is
  monotone in K"* is likewise withdrawn: it was neither MEASURED nor ESTIMATED,
  and it was the whole justification for "largest-not-smallest".
- **COST, on this document's own face** (`docs/process.md`): 5 grid points + 1
  gate-off seat × 24 positions × 5 reps = **720 searches at 50 000 nodes**.
  MEASURED anchor: §0's own runs took 118–196 ms per 50 000-node search at
  gate-off and REVIEW-design measures the ON seats 2–3× slower, so **ESTIMATED
  25–50 minutes wall on one core**, in-process (no per-invocation process
  startup, unlike §8). Operator attention: one launch, one read.
- The table lands as `artifacts/wp19_calibration_v1.txt`, sha-recorded, and the
  chosen K and the rule above are QUOTED — not paraphrased — into the prereg.

## 8. RULE-5 BENCH — A REPLICATION WITH A PRECEDENT-INHERITED THRESHOLD, NOT A DISCOVERY

**Why this section is not shaped like WP-1.7 §7.** Its answer is already known:
§0.4 records a fresh-context reviewer implementing §2's semantics and measuring
0.32–0.48 at every grid point with this section's own instrument. `docs/process.md`
closes on the point — *"neither catches a run whose answer is already known before
it is taken — that defect is judged, not checked"*. Registering a fresh bracket now
would be choosing a number knowing which side of it the data falls on, which is
D-374's forbidden move wearing a pre-registration's clothes. So the threshold below
is **inherited verbatim from precedent**, not derived from a prediction, and the
run below is a **replication at the implementation revision**.

- **Pre-registered hotspots — BOTH of them, corrected per §0.4:**
  1. `staged::delta_rank`'s `Eval::delta` roundtrip, paid over the **whole quiet
     pool** (§0's mean 63.557), not over K. D-192 measured that roundtrip at
     76.27 % of profiled ordering stacks.
  2. `candidates::within_radius`, one call per batched node — today 0.000607 of
     corpus batched rows, under stage Q 99.94 % of them. **MEASURED in isolation
     at 0.729/0.603 with the emitted set unchanged** (§0.4).
- **THE THRESHOLD, INHERITED FROM `wp18b_design.md` §7 as WP-1.8c inherited it**
  (D-465: *"its bracket and abort clause inherited from `wp18b_design.md` §7
  verbatim and quoted rather than paraphrased"*). That section's corpus clause,
  quoted rather than paraphrased:

  > *"**Bracket**: band-aggregate nps ratio ON/OFF ≥ 0.5 in both bands on the
  > CORPUS fixture (the regression axis: gate-on must not halve ordinary
  > throughput)"*

  **TWO UNITS ARE REGISTERED, NOT ONE, AND THE DIFFERENCE IS NAMED RATHER THAN
  SILENTLY SPENT.** The inherited bound is on **nps** and this WP takes it at its
  own value and for its own stated reason — a candidate-policy change must not
  halve ordinary throughput at the 0.5 s design point. But D-374's standing lesson
  is that *"across seats with different candidate policies, nps is not a
  like-for-like unit (nodes differ); ttd registers as primary, nps as context"*,
  and revision 1 gated on nps while demoting time-to-depth to non-gating
  (REVIEW-design MAJOR 4). **The measurement shows that confound is live here**:
  K = 2 benched *worse* in the late band (0.391) than K = 16 (0.426) although it
  emits strictly fewer cells at every node, because the two seats' 50 000 nodes
  sit at different completed depths. So: **the ON seat is a candidate for h1 only
  if BOTH the time-to-depth ratio (primary, D-374) AND the band-aggregate nps
  ratio (inherited, wp18b §7) are ≥ 0.5 in BOTH corpus bands.** Either falling
  below 0.5 is the abort.
- **VERDICT SPACE — TOTAL, every ratio mapping to exactly one registered reading**
  (revision 1 left `[0.40, 0.55)` and `> 0.95` unregistered and the measured point
  landed in the first gap — REVIEW-design BLOCKING 2):

  | measured, corpus | registered reading |
  |---|---|
  | **ttd ≥ 0.5 in both bands AND nps ≥ 0.5 in both bands** | within bracket. The ON seat is a candidate; the WP proceeds to the prereg and the SPRT at §7's K |
  | **either unit < 0.5 in either band** | **ABORT.** The ON seat is not a candidate for h1 **regardless of what any SPRT would say** — D-465's registered consequence, inherited with the threshold. No SPRT is run, the committed configs do not move, and the WP closes as a measured finding (rule 5: a measured structural floor is a finding, not a failure) |

  There is no third region: the two arms partition every pair of ratios, which is
  the property revision 1 dropped when it inherited WP-1.7's command block without
  WP-1.7's *"Verdict space — TOTAL, so no reading is chosen after the numbers"*.

- **THE "STAGE Q NEVER BIT" CRITERION, and the defect class it excludes.**
  `WPQ_seed.md` §7.2 registered this defect class by name (*"whose defect class is
  'the cap never bites'"*) with a non-zero count that had to be reported; revision
  1 added the counters and registered no criterion on them. **Registered here: on
  the ON seat, `quiet_rows` > 0 AND `quiet_cells_admitted == K × quiet_rows` AND
  the per-row pool minimum ≥ K.** A ratio at or above 0.95 with those unmet means
  the gate did not arm, K did not cut, or the pool was empty — an instrument
  finding, not a measurement of stage Q, and the run is void rather than read.
  These are read off §7's harness, not off the line protocol (BLOCKING 3).
- **IQR gate:** per position, IQR of its 5 per-rep times ≤ 10 % of that position's
  median (D-215/D-362). A position exceeding it WITHHOLDS the verdict and is
  re-measured, both configs, before any ratio is read.
- **THE DRY RUN — RECORDED, WITH ITS INPUT, ITS CRITERION AND ITS DEFECT CLASS**
  (`docs/process.md`; revision 1 registered none — BLOCKING 4).
  **DONE, this session**, receipt `artifacts/wp19_bench_dryrun_v1.txt` (sha256
  `24d45875…`).
  - *Input:* two hand-authored `start moves` entries in `bench_positions_v1.txt`'s
    own line form, 5 and 7 stones — same KIND as the registered workload, differing
    only in identity, never the workload itself.
  - *Criterion:* the two entries must produce DIFFERENT `info totals` lines, and no
    `^error ` line may appear on either stream.
  - *Defect class excluded:* a refused or mis-parsed `position` line measuring the
    empty board instead of the entry. Output shape, plausible magnitude, exit
    status and agreement between the two configs all **survive** that defect, so
    none of them is a criterion; the externally derived referent is the entry's own
    stone count, which the engine never sees as a number and cannot launder.
  - **IT FOUND ONE, AND THE COMMAND BLOCK IS FIXED HERE.** The block WP-1.7 §7
    registered — which revision 1 adopted "verbatim" — **silently substitutes the
    empty board for any entry the engine refuses**: the `error` goes to *stdout*,
    the engine exits **0**, `sed -n 's/^info totals //p'` drops it, and the
    following `go` measures the empty board and emits a well-formed totals line.
    MEASURED: the malformed stand-in returned byte-identical totals for a 5-stone
    and a 7-stone entry with a pv beginning at the origin. This is
    `SHELL_CHECKLIST` item 3's EXIT-0-WRONG-ANSWER class and D-464's class.
    **The guard registered here:** capture stdout *and* stderr, refuse the entry
    by name on any `^error ` line, and abort the bench rather than aggregate it.
  - **What it does NOT imply, checked rather than assumed:** the defect is LATENT
    on the fixtures used — all 24 `bench_positions_v1.txt` entries and all 4
    `spread_v1.txt` entries load, 0 refused, driven through the shipped engine at
    a 1-node budget (D-464's own rule). **No landed number is in question, D-431
    included.**
  - **AND THE SECOND FIXTURE'S GRAMMAR IS NOT THE FIRST'S** (BLOCKING 4):
    `spread_v1.txt` lines already carry a `position ` prefix and are interleaved
    with bare `stones N` lines, so the WP-1.7 block emits
    `position position start moves …` and takes `error Protocol:` on every entry,
    exiting 0 with no data. The registered `spread_v1.txt` pass strips the prefix
    and skips `stones` lines; **MEASURED, 4 loaded / 0 refused** with that
    extraction.
- **The `spread_v1.txt` pass is reported and is NOT a gate**, because §0.4
  measures stage Q inert there (nps 0.921–1.028, depth unchanged) — reported so a
  reader does not assume the D-95 class moved.
- **COST, on this document's own face:** 24 positions × 5 reps × 2 configs = 240
  sequential engine invocations, plus 4 × 5 × 2 = 40 for the spread pass.
  MEASURED anchors: 118–196 ms per gate-off search (§0), ON 2–3× that (§0.4);
  plus ~280 × ~30 ms process startup (D-236). **ESTIMATED 8–18 minutes wall on one
  core.** Operator attention: one launch, one read.

## 9. Tests, and the mutants each must kill

| test | pins |
|---|---|
| `staged_stage_q_off_reproduces_the_pinned_expectations` | **cross-REVISION identity, which a two-config comparison inside one binary cannot pin** (REVIEW-design m4): a staged counterpart of `instrument_behavior_byte_identical_pre_post`, driving a sha-pinned expectation fixture beside `crates/pistol-cli/tests/fixtures/instrument_golden_v1.txt`. Revision 1's "both determinism fixtures" was also ill-formed — `tactical_v0.txt` is driven under `configs/gate_v0.toml`, a `kind = "radius"` document with no `stage_q` to set |
| `the_quiet_cap_admits_exactly_k_cells` | at a batched node with pool > K: emitted width == `\|TierT\| + K` |
| `k_and_k_plus_one_differ_by_exactly_the_next_ranked_quiet_cell` | the boundary, as a set difference — **the off-by-one mutant dies here.** The mutation is named in terms the implementation admits: `Vec::truncate(k)` → `truncate(k + 1)`. Revision 1 registered "cap boundary `>` for `>=`", which the chosen mechanism contains no comparison to mutate — a mutation that cannot be applied is not a receipt (REVIEW-design m5) |
| `tier_t_is_emitted_whole_and_ahead_of_every_quiet_cell` | `cells[..\|TierT\|]` is exactly Tier T — **the "truncation applied to stage T" mutant dies here** |
| `the_forced_rows_emit_the_same_set_with_the_gate_on` | WIN-NOW and FILTERED sets identical ON and OFF — **"truncation applied to a forced row" dies here** |
| `the_quiet_cap_is_stable_across_runs` | two runs, identical emitted sets; and equal-delta quiet cells keep ascending order — **the order-dependent tie-break mutant dies at the determinism seat** |
| `the_safety_net_is_never_capped` | Tier T empty ⇒ emitted width == ball width, gate ON — §3's refusal, seated |
| `a_staged_config_omitting_stage_q_is_refused` | rule 1, on **the key this WP adds**. Revision 1 registered `a_stage_q_config_is_refused_without_quiet_top_k`, which is green before the WP starts — `quiet_top_k` is already mandatory (`config.rs:224`, no serde default; `validate.rs:94-97`) — so it tested a pre-existing property (REVIEW-design m6) |
| `a_searcher_built_in_code_is_refused_at_quiet_top_k_zero` | rule 1 at the SECOND validator: `Searcher::new`, because a `SearchParams` can be built in code and never pass through a document (`U3_tier_t.md` §10) |
| `stage_q_does_not_apply_at_ply_zero_under_a_root_restriction` | §2.3's exclusion: with `root_restrict` `Some` and `TierT ∩ zone = ∅`, the root searches all of Tier T with the gate ON exactly as it does with the gate OFF — **the mutant that removes the exclusion dies here**, and it is the one mutant in this list guarding a WRONG ANSWER rather than a wrong count |

**Mutation receipts** (separate worktree, D-469 export before removal): the three
the dispatch names — cap boundary `>` for `>=`; tie-break made order-dependent;
truncation applied to the Tier-T prefix — plus **two** this design adds:
**truncation applied to the safety-net set**, which must die at
`the_safety_net_is_never_capped` because §3's refusal is a designed rule and an
undefended designed rule is a hedge; and **the ply-0 `root_restrict` exclusion
removed**, which must die at `stage_q_does_not_apply_at_ply_zero_under_a_root_restriction`.
Five mutants, each with a named gate.

## 10. ADR lines this design records

1. Stage Q is a quiet tier ADDED beyond Tier T; K counts quiet cells
   (`U3_tier_t.md` §10, quoted); the dispatch's narrowing framing is refuted with
   §0's numbers, and the dispatch is archived at `sessions/WP-1.9/wp19_DISPATCH.md`
   so the adjudication is checkable.
2. Stage Q as specified is UNAFFORDABLE at eval v0, MEASURED end-to-end
   (§0.4: 0.32–0.48 at every grid point; pool construction alone 0.729/0.603),
   and the dominant term is the `Eval::delta` roundtrip over the WHOLE quiet pool
   plus one `within_radius` per batched node — neither of which revision 1
   registered. The cost-reduction package is SCHEDULED, and D-465's WP-1.8b→c
   arc is its precedent and its shape.
3. The safety net is NOT capped, why the refusal is rule-bound (no matrix, no red
   team), and — withdrawn from revision 1 — **this WP therefore does NOT pay
   D-310's strength debt**, because `WPQ_seed.md` defines that debt on exactly the
   safety-net class, and stage Q is MEASURED inert there. D-310's flip clause
   stands undischarged.
4. Ply 0 under `root_restrict` is EXCLUDED from stage Q, because adding cells
   defeats `pvs.rs`'s fail-open and produces a strictly smaller root search — a
   wrong-answer path, found by REVIEW-design, not a preference.
5. The remaining ply-scope axis {every ply | root/PV exempt | non-PV only} is
   DEFERRED with M2 and the safety-net cap to the follow-up package, since it is
   matrix-and-red-team work the dispatch forbids and, under §8's expected abort,
   licenses no conclusion (D-424's test).
6. The `stage_q` gate exists because a large `quiet_top_k` inverts meaning between
   the two readings and would silently arm `tactical_staged_v0.toml` (1024) and
   `gate_staged_v0.toml` (128) at their most widening value.
7. **D-356's IMPL-time reading is REVERSED, and the reversal is recorded rather
   than performed silently** (REVIEW-design m3): `params.rs:47-57` states why
   `quiet_top_k` was deliberately kept OFF `StagedParams` — *"Carrying them here
   unused would be dead weight on the search's own hot-path type for a mechanism
   that does not run … This is an IMPL-time reading of an OPEN question the design
   left the architect's"*. The mechanism now runs, so the reading no longer holds;
   §5 puts the field on the type and this line is what stops that being drift.
8. The WP-1.9 / WP-1.5c designation collision, for the operator to settle.
9. The WP-1.7 §7 bench command block silently substitutes the empty board for a
   refused entry (stdout, exit 0, dropped by `sed`); the guard, the
   `spread_v1.txt` grammar fix, and the MEASURED check that no landed number is
   affected.

## 11. Out of scope

`widen_schedule` and every widening mechanism; dominance pruning; the
safety-net cap (§3); eval; solver; TT; ordering heuristics (WP-1.7 closed h0);
any second tunable.
