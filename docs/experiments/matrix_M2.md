# MATRIX M2 — the safety-net candidate cap

**REVISION 2.** Revision 1 (`5b2c972`) was attacked by a fresh-context
DECISION-RED-TEAM before selection, as CLAUDE.md requires, and its
recommendation **FELL**: `docs/experiments/matrix_M2_REDTEAM.md` (`c075bcc`,
sha256 `4e81a5c8…`), VERDICT **FALLS**, on three independent BLOCKING grounds
plus a fourth. **That report is not edited by this revision and nothing in it is
softened.** This revision does what the report says the field owed: it carries
the two missing rows with MEASURED cells, corrects every claim the report
refuted, and re-derives the recommendation — which is now a different row.

**WP-1.5d. Still authored before any option is selected.** `docs/decisions.md`
D-310 excised the quiet stage and its widening schedule from WP-1.5b; D-315
scheduled them and recorded that **M2 has never been authored in the form its
candidate takes** — `W-E` occurs zero times at `ec8f7fb`, was supplied by the
DECISION-RED-TEAM that killed `W-A`, and was never an option among options.

**THE DECISION.** On a BATCHED or BATCHED-lost row where Tier T comes back
empty, the candidate set today is the WHOLE quiet ball. What, if anything,
replaces it? Nothing else about the node protocol is in scope: Tier F, Tier T,
the FILTERED row's cover union and the WIN-NOW row are untouched by every option
below.

**WHAT CHANGED FROM REVISION 1, IN ONE TABLE**, so a reader can check the
adjudication rather than take it:

| revision 1 said | revision 2 says | why |
|---|---|---|
| the field has seven rows | eleven, adding **W-K0** (cap ply 0 only) and the **pool-prefilter** class in two instances | RED-TEAM B1, B2 |
| "a top-K cap never reduces the cost of a safety-net node" | true of a cap placed AFTER `delta_rank`; a cap placed BEFORE it removes the eval pass, MEASURED 11.9–32.5× | RED-TEAM B2 |
| "only a ROOT-INCLUSIVE cap moves F6" | an option moves F6 iff it removes the root's fanout OR cheapens the root's children; a post-rank truncation outside ply 0 does neither | RED-TEAM B1, B2 |
| the incumbent "replaces an unsearched answer with a searched one" | **false**: `Provenance::PartialRoot` is the exact depth-1 argmax over 957 of 3 564 root children | RED-TEAM B3 |
| the book's numbers, at `except-ply0` | re-taken at every scope; 25.8 % becomes 0.37 %–61.5 % depending on the scope, and the denominator was wrong too | RED-TEAM B4, M5 |
| "what it prunes is provably not tactical" | withdrawn; F1 proves a node-local claim and nothing about the subtree | RED-TEAM M8 |
| `quiet_top_k` never mentioned | F12: it exists in twelve committed configs with the OPPOSITE semantics and has no off-value | RED-TEAM M6 |
| completed depth decides the field | **it cannot**: F10, the deepest seat in the whole field is the degenerate one | RED-TEAM M9 |
| the recommendation is **W-K1** | the recommendation is **W-K2** | §5 |

**Every numeric claim is marked MEASURED or ESTIMATED (D-291).** Revision 1's
receipts stand — the red team re-derived F4, F5, F7, F8, F9, §2 and §H cell for
cell — and this revision adds
`artifacts/wp15d_m2_evidence_v2.txt` (sha256 `455aef9e…`) with its complete
driver at `artifacts/wp15d_m2_evidence_instrument_v2.txt` (sha256 `f73608dd…`).
Revision 1's are `artifacts/wp15d_m2_evidence_v1.txt` (sha256 `db8a8793…`) and
`artifacts/wp15d_m2_evidence_instrument_v1.txt` (sha256 `081c928a…`). All four
were taken in a detached worktree at `fbc8e62` with its own `CARGO_TARGET_DIR`;
no scratch edit entered the live tree. The census cells are
`artifacts/wp19_design_census_v1.txt`, taken by WP-1.5c.

---

## 0. THE FACTS THE FIELD IS SCORED AGAINST

**F1 — the safety net is a branch, and both tiers are empty inside it.**
`crates/pistol-search/src/staged.rs:222-236`, quoted **whole** this time —
revision 1 elided the comment, which is the only place in the tree that names
the knob F12 is about (RED-TEAM M6):

```rust
    let mut tier_t = tier_t_union(threats, us, params);
    if tier_t.is_empty() {
        // The safety net. See the module doc: at the game's earliest plies no
        // window anywhere has reached a live count, so Tier T is provably
        // empty too, and this is the branch that keeps the search from
        // reporting no move at all. The same ball `candidate_cells`'s `Staged`
        // arm answers with, uncapped — `quiet_top_k` is stage Q's own knob and
        // this D-scope does not arm stage Q.
        tier_t = within_radius(board, params.quiet_radius);
        out.used_quiet_safety_net = true;
    }
    delta_rank(&mut tier_t, eval, us);
    out.cells = tier_t;
    out.forced = 0;
```

Tier F is provably empty on every row `batched` is called from (that function's
doc, `staged.rs:209-213`) and Tier T is empty by the branch condition. **No cap
placed inside this branch can exclude a cell Tier F or Tier T would have emitted
AT THAT NODE**, and that is a GUARD (the `if`), not a construction. The
node-local scope of the claim is stated here because revision 1 spent it as a
subtree claim and RED-TEAM M8 is right that it is not one: a cut cell is a move
never played, so the whole subtree under it — including a forced win three turns
later — is never searched. That is what a forward prune IS, and it is priced in
each row rather than argued away.

**F2 — CORRECTED. The per-node cost of this row is independent of the cap only
where the cap is placed after the ranking.** `candidates.rs:42-61` builds the
ball as a `BTreeSet` over `stones × ball_offsets` and filters by legality;
`staged.rs:287-295`'s `delta_rank` calls `Eval::delta` on **every cell of the
vector it is handed**, and `staged.rs:230, 233` hands it the whole ball. A
truncation placed BETWEEN those two lines removes the eval term. **MEASURED**
(RED-TEAM R0, 200 reps per cell, `staged_candidates` timed directly): a
safety-net node costs

| | width | whole | pool truncated to 8 first | ratio |
|---|---|---|---|---|
| 99 stones, r = 3 | 3 564 | 2 657.9 µs | 118.9 µs | **22.4×** |
| 51 stones, r = 3 | 1 836 | 1 202.8 µs | 43.2 µs | 27.8× |
| 21 stones, r = 3 | 756 | 434.7 µs | 13.4 µs | 32.5× |
| 99 stones, r = 2 | 1 782 | 1 414.9 µs | 119.4 µs | 11.9× |

So ~95 % of a wide safety-net node is the eval pass. **This is what makes the
pool-prefilter class a real option and not a footnote**, and revision 1 priced
that class by ESTIMATE when the measurement took 1.45 s of test time — a D-291
finding against revision 1, recorded rather than absorbed.

**F3 — the root's zone restriction and its fail-open run at ply 0 only.**
`pvs.rs:336-347`:

```rust
if let Some(zone) = self.root_restrict.as_ref().filter(|_| ply == 0) {
    let forced_intact = set.cells[..set.forced]
        .iter()
        .all(|cell| zone.binary_search(cell).is_ok());
    if forced_intact {
        let unrestricted = set.cells.clone();
        set.cells.retain(|cell| zone.binary_search(cell).is_ok());
        if set.cells.is_empty() {
            set.cells = unrestricted;
        }
    }
}
```

On a safety-net row `set.forced == 0`, so `forced_intact` is vacuously true.
**An option that caps at ply 0 makes the comment at `pvs.rs:329-335` false as
written** — "FAILS OPEN to the unrestricted set" becomes "fails open to the
capped set". **This is not an open choice, it is the correctness hole the
immediately preceding package named and fixed** (RED-TEAM M10):
`sessions/WP-1.9/2026-08-29-WP19-STOPPED.md` records it, and revision 2 of that
design excluded ply 0 under `root_restrict` with the re-review confirming the fix
"right and sufficient". `configs/gate_staged_solver_v0.toml` and
`configs/bench_wp18c_solver_on.toml` arm the solver, so the seat is not
hypothetical. Every ply-0-capping row below carries this as a cost, not as a
question.

**F4 — a fail-low widening trigger cannot bind on this recursion.** Re-verified
at HEAD: the root opens at `visit(depth_plies, -INFINITY, INFINITY, 0)`
(`pvs.rs:153`) and `best_score` starts at `-INFINITY` (`pvs.rs:368`), so
`best_score <= original_alpha` is unsatisfiable once any child returns a score;
a non-PV child opens at `(alpha, alpha + 1)` (`pvs.rs:498, 500`), so any
`score > alpha` sets `alpha >= beta` and breaks (`pvs.rs:427-429`). MEASURED by
the seed's own instrument: 0 root widenings in 101 root iterations, 0 of
2 022 904 non-PV interior nodes truncated. Independently confirmed sound by the
red team.

**F5 — the class's rate and width. MEASURED**
(`artifacts/wp19_design_census_v1.txt`):

| corpus | safety-net rows / batched rows | rate | mean width | share of all batched cells |
|---|---|---|---|---|
| `bench_positions_v1` (24 pos, 15/35 stones) | 66 / 108 662 | **0.000607** | **78.12** | 0.0015 |
| `spread_v1` (4 pos, 11/21/51/99 stones) | 2 468 / 4 015 | **0.6147** | **1 218.33** | **0.9966** |

**F6 — CORRECTED, and the correction matters more than the number.** The debt is
live at HEAD: MEASURED at `fbc8e62` through the shipped `target/release/pistol`
and `tools/bench_block.sh`, `go movetime 500` on `spread_v1` completes
`depth_turns` **1 / 0 / 0 / 0** at 11 / 21 / 51 / 99 stones under BOTH
`configs/play_v0.toml` and `configs/play_staged_v0.toml`. **But
completed-depth-0 is not an unsearched answer.** `Provenance::Fallback` is this
engine's unsearched answer (`search.rs:410-416, 434`); what these positions
return is `Provenance::PartialRoot`, and `Run::salvage`'s own doc
(`pvs.rs:174-186`) says a ply-0 promotion only ever happens on a COMPLETED child
subtree, so **the line is turn-whole and the score exact**. MEASURED this
revision (`RA/SPREAD/mt500/*/nocap`, root counters added to `pvs.rs`):

| position | root width | root children completed | what the answer is |
|---|---|---|---|
| 99 stones, r = 3 | 3 564 | **957** | the exact depth-1 argmax over 26.9 % of the root ball |
| 51 stones, r = 3 | 1 836 | 530 | over 28.9 % |
| 21 stones, r = 3 | 756 | 337 | over 44.6 % |
| 99 stones, r = 2 | 1 782 | 642 | over 36.0 % |

(RED-TEAM B3 measured 956 at the first of these; the ±1 is the deadline's own
granularity, and the two runs are independent.) **So the debt is "one turn deep
over a fraction of the root", not "no search at all"**, and any row that
narrows the root is trading 957 candidates at depth 1 against K candidates at
greater depth. Which side of that is stronger is exactly what D-124 says no
oracle answers.

**F7 — CORRECTED. What moves F6 is not "a root-inclusive cap"; it is either
removing the root's fanout or cheapening the root's children.** MEASURED,
`movetime 500`, completed `depth_turns` at 11 / 21 / 51 / 99 stones:

| row | r = 3 | r = 2 |
|---|---|---|
| W-N incumbent | 1 / 0 / 0 / 0 | 1 / 1 / 0 / 0 |
| W-K2 `except-ply0`, K = 8 | 1 / 1 / 0 / 0 | 2 / 1 / 1 / 0 |
| W-K3 `non-pv-only`, K = 8 | 1 / 1 / 0 / 0 | — |
| **W-K0 `ply0-only`, K = 8** | **2 / 2 / 2 / 2** | **2 / 2 / 2 / 2** |
| **W-K1 `every-ply`, K = 8** | **4 / 4 / 3 / 3** | 4 / 4 / 4 / 3 |
| **W-PFR recency M = 2, every ply** | — | **2 / 2 / 2 / 2** |
| **W-PFR recency M = 2, `except-ply0`** | — | **2 / 2 / 1 / 1** |
| **W-PFO ring-order M = 2, every ply** | — | **5 / 6 / 5 / 5** |

Two rows refute revision 1's headline. **W-K0** removes the root's fanout and
nothing else — MEASURED, it caps 3 of 49–76 safety-net rows per search — and
pays the debt at all four stone counts. **W-PFR at `except-ply0`** leaves the
root's set whole and still lifts 51 and 99 stones off zero, because F2's eval
term is what its children were paying. Revision 1's sentence was true of the
rows it had and false of the field.

**F8 — at the reproducible budget the same ordering holds, and nps is not the
unit. MEASURED**, `Stop::Nodes(50_000)`, `quiet_radius 2`, `spread_v1`:

| seat | depth 11/21/51/99 | nodes | wall ms (99 stones) |
|---|---|---|---|
| no cap | 1 / 1 / 1 / 1 | 58 368 / 201 728 / 1 149 952 / **4 283 392** | 10 648 |
| every ply, K = 4 | 5 / 4 / 5 / 5 | 50 176 each | **907** |
| every ply, K = 8 | 4 / 4 / 4 / 4 | 50 176 each | 869 |
| every ply but 0, K = 4 | 2 / 2 / 2 / 1 | 50 176 each | **16 441** |

The uncapped seat's node counts are D-74's own behaviour. **nps is not a
like-for-like unit across these seats** (D-374's standing lesson): 4.28 M mostly
leaf nodes against 50 k mostly interior ones. Replicated by the red team at the
committed `tt_bytes = 268 435 456` — **every seat byte-identical** to the
receipt's `1<<20` — so the table size is not carrying any of this.

**F9 — the cap is inert where the class is rare. MEASURED**,
`bench_positions_v1` at `Stop::Nodes(50_000)`, `quiet_radius 2`:

| K | Σ nodes | completed depth, 24 positions | Σ wall ms |
|---|---|---|---|
| 0 | 1 104 026 | `3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3` | 4 825 |
| 8, every ply but 0 | 1 104 026 | identical, position for position | 4 836 |
| 32, every ply but 0 | 1 104 026 | identical, position for position | 4 801 |

and it transfers to **every** scope, which revision 1 asserted and the red team
verified: at `every-ply` the corpus has **zero** safety-net rows at ply 0
(`sn_rows − sn_capped = 0`), so the scopes walk the same tree (their R2). The
corpus bench is therefore a no-regression check and never a gain channel.

**F10 — NEW, AND IT DISQUALIFIES THE CHANNEL REVISION 1 DECIDED ON.** Completed
depth on `spread_v1` is monotonically improved by narrowing, without bound, so it
ranks options by how hard they narrow. MEASURED, and the demonstration is the
whole field's own worst row: **W-PFO — the ring-order prefilter, whose key is
ascending `(q, r)` — is the DEEPEST seat measured anywhere in this document (5 /
6 / 5 / 5 at M = 2)** and it plays `-4,1/-2,1` at 11, 51 and 99 stones alike,
i.e. beside the origin regardless of where the game is. Depth cannot decide this
field. What the channel CAN carry is F6's **binary** defect — completed depth 0,
no iteration finished — and that is the only way it is read below.

**F11 — NEW. The transposition store is sound under every UNIFORM ply scope and
unsound under the non-PV one, MEASURED.** A capped node stores a `Record`
(`pvs.rs:449-467`) ranging over the capped set, and the probe (`pvs.rs:245-256`)
takes a cutoff on `!is_pv` alone; the bound is sound for a later visitor iff the
same key always emits the same set. A probe hashing set MEMBERSHIP at the point
`visit` will store over it — after the cap, after `promote_table_move`, after the
root restriction — with all three ordering heuristics ON:

| scope | observations | disagreements |
|---|---|---|
| 0 `every-ply`, 1 `except-ply0`, 3 `ply0-only` (K ∈ {8, 32}, three fixtures) | **3 329 995** | **0** |
| 2 `non-pv-only` (same) | 1 254 229 | **821** |

The reason is structural and worth stating: the stone count is a function of the
key and only increases down the tree, so ply 0's key can never recur below it,
and a scope that is a function of ply alone therefore emits one set per key. A
scope that is a function of `is_pv` does not — the same key is visited PV (full
ball) and non-PV (top-K) inside one search, and `Bound::Lower` from the superset
is not a valid lower bound for the subset. **That is `WPQ_seed.md` §7.1(3)'s
poisoned-entry class arriving exactly as it describes**, and it is the ground
that kills W-K3, replacing revision 1's weaker one. (Independently measured by
the red team at scopes 0 and 2; extended to 1 and 3 here. Revision 1's own first
attempt at this probe was VOID — an order-sensitive digest reporting every
table-move promotion as a disagreement — and the void run is kept in the receipt
so the correction is visible.)

**F12 — NEW. The knob this decision would use already exists, in twelve
committed documents, meaning the opposite thing.** `/usr/bin/grep -rn
"quiet_top_k" configs/ | LC_ALL=C sort` returns twelve documents;
`crates/pistol-engine/src/validate.rs:94-97` refuses `quiet_top_k == 0`, so **the
schema has no off-value**; `validate.rs:104-121` cross-validates
`widen_schedule` against it; and `docs/experiments/U3_tier_t.md` §10 fixes the
semantics as *"the first batch is `quiet_top_k` quiet cells; Tier F and Tier T
are always emitted whole and are not counted against it"* — **a quiet tier ADDED
beyond Tier T, the opposite direction from every row below**, which is the
finding that stopped WP-1.5c and which D-474 names by name.
`configs/instrument_staged_v0.toml:10-15` further asserts that "the cut BINDS
here". **Any selected option owes an answer**: a third knob is two schema places
for one default (hard rule 1, D-423), and re-purposing these two silently changes
what twelve committed headers assert. §5 answers it.

---

## 1. THE FIELD

Eleven options. The ply scope stays part of an option's mechanism rather than a
second axis — but for the opposite reason revision 1 gave: F7 and F11 both make
the scope decide *what the row is*, and F10 removes the channel on which a
factored-out scope could have been scored independently.

### W-N — NO CAP (the null option, and the incumbent)

- **Mechanism.** None.
- **Wrongly excludes.** Nothing.
- **Cost.** MEASURED F6: completed depth 0 at 21 stones and above at
  `movetime 500`, the answer a depth-1 argmax over 27–45 % of the root ball.
  MEASURED F8: 4 283 392 nodes and 10.6 s for one turn of depth at 99 stones.
  MEASURED §3: 121 of the SPRT book's 2 000 openings reach completed depth ≥ 3.
- **Debt.** Pays none; D-310's flip clause stays armed.
- **Kill.** Any row that lifts the book's depth distribution while holding the
  corpus. MEASURED, several do.

### W-CFG — SHRINK `quiet_radius` (config only, zero code)

- **Mechanism.** None in code. Under `Staged`, `quiet_radius` reaches only
  `candidates.rs:31` and `staged.rs:230`, so lowering it IS a safety-net
  narrowing, available today with no gate and no ADR.
- **Wrongly excludes.** Every outer-ring cell, at every row, including the
  fallback answer and `check_root`'s reachability question.
- **Cost.** **It scales the ball; it does not bound it.** MEASURED §2: at its
  floor `quiet_radius = 1` the 99-stone position still completes ZERO
  iterations. Mean widths at 99 stones: 596.0 (r=1), 1 789.2 (r=2), 3 579.3
  (r=3).
- **Kill.** MEASURED, its own floor. It cannot reach the debt it is proposed for.

### W-K0 — SCORE-RANKED TOP-K, PLY 0 ONLY *(new in revision 2, RED-TEAM B1)*

- **Mechanism.** Inside F1's branch, keep the first K of the delta ranking, at
  ply 0 and nowhere else.
- **Wrongly excludes.** The root's tail beyond rank K — **and nothing else**.
  MEASURED: 3 of 49–76 safety-net rows per spread search; 5 400 of 520 164 on
  the book (1.04 %); 16 762 of 4 530 244 opponent live-count-two cells
  (**0.37 %**). It is the smallest exclusion in the field that does anything.
  **But it is entirely at ply 0**, which is where D-124 says no oracle catches a
  mistake, where F6 measures the incumbent to have been searching 957 candidates,
  and where F3's fail-open comment becomes false.
- **Cost.** MEASURED F7: pays the debt at all four stone counts, 2/2/2/2, from a
  **completed** iteration. MEASURED §3: 121 → 644 of 2 000 book openings at depth
  ≥ 3, and 25 % FASTER in wall time at the same node budget (286 645 ms against
  381 324). MEASURED F11: TT-sound.
- **Debt.** The D-95 / WP-1.4 depth debt, fully.
- **Kill.** F3 and F6 together: it is a pure root prune, and its whole benefit is
  bought at the one node this project's own governing documents single out.

### W-K1 — SCORE-RANKED TOP-K, AT EVERY PLY *(revision 1's recommendation)*

- **Mechanism.** W-K0 ∪ W-K2. This is exact, not approximate: the scopes
  partition the plies.
- **Wrongly excludes.** Both rows' sets. MEASURED on the book at K = 8:
  61.5 % of opponent live-count-two cells (1 377 486 of 2 241 129), or 79.5 %
  against the pool denominator (RED-TEAM R7, 300 openings).
- **Cost.** MEASURED F7/F8: the largest depth gain anywhere in the eval-ranked
  family. MEASURED §3: **2 000 of 2 000** book openings at depth ≥ 3. MEASURED:
  16 % SLOWER in wall time at fixed nodes on the book (442 277 ms against
  381 324), because the ratio of ball-paying interior nodes to leaves inverts.
  MEASURED F11: TT-sound. Carries F3's correctness cost in full.
- **Debt.** Both.
- **Kill.** It is the union of two rows that serve two different objectives and
  carry very different risks, so neither an `h1` nor an `h0` on it can say which
  half earned or lost the verdict. §4 is that argument in full.

### W-K2 — SCORE-RANKED TOP-K, EVERY PLY BUT 0

- **Mechanism.** The truncation under a `ply > 0` guard.
- **Wrongly excludes.** Interior tails only. The root keeps full width, so D-124
  is not engaged at the node where the move is chosen and **F3's fail-open is
  untouched** — its `unrestricted` is still the ball, and the comment at
  `pvs.rs:329-335` stays true. MEASURED on the book at K = 8: 56.5 % of opponent
  live-count-two cells; against the pool denominator, §3.
- **Cost.** MEASURED F7: it does NOT pay the D-95 debt — 51 and 99 stones stay at
  completed depth 0 at r = 3 — and F2 says why: it removes subtrees, not the
  eval pass its root's ~1 782 children each pay. It does not REGRESS the class
  either (r = 2: 2/1/1/0 against the incumbent's 1/1/0/0). MEASURED F8: 16 441 ms
  for 50 176 nodes at 99 stones, the worst wall time in the field on that
  fixture. MEASURED §3: 121 → 1 083 of 2 000 book openings at depth ≥ 3, wall
  time on the book unchanged (378 694 ms against 381 324). MEASURED F9: inert on
  the corpus. MEASURED F11: TT-sound.
- **Debt.** D-310's excised-axis debt on `WPQ_seed.md`'s own definition of it
  (*"Tier F and Tier T are both EMPTY"*), on the sample the SPRT walks. Not the
  D-95 depth debt.
- **Kill.** A corpus bench outside its registered bracket, or a calibration in
  which no K lifts the book's depth distribution while holding the corpus.

### W-K3 — SCORE-RANKED TOP-K, NON-PV NODES ONLY

- **Mechanism.** The truncation under `!is_pv`, which `staged_candidates`
  already receives (`staged.rs:119`) — `WPQ_seed.md` §7.2's W-E scope.
- **Wrongly excludes.** W-K2's set minus the PV spine.
- **Cost.** MEASURED F7: indistinguishable from W-K2 on the class. MEASURED F11:
  **821 membership disagreements in 1 254 229 observations** — the same key
  emitting the full ball at a PV visit and the top K at a non-PV visit inside one
  search, so a `Bound::Lower` proved over the superset is consumed as a lower
  bound for the subset.
- **Kill.** **F11, and it is a correctness ground rather than a performance one.**
  This is `WPQ_seed.md` §7.1(3) verbatim, and it is the reason W-E's own
  specification carried a TT truncation rule. Revision 1 killed this row on width
  instability, which RED-TEAM M15 showed was arithmetic from the exempt rows and
  not instability at all; the kill above is the real one.

### W-PFR — POOL PREFILTER BY RECENCY *(new in revision 2, RED-TEAM B2)*

- **Mechanism.** Build the quiet ball around the **last M stones played** rather
  than around every stone, then `delta_rank` that pool. The truncation is BEFORE
  the eval pass, so F2's term shrinks with it; the pool is Θ(M) balls rather than
  Θ(stones).
- **Wrongly excludes.** Everything outside the recent stones' reach — at every
  ply it applies to, the root included when its scope includes ply 0.
- **Cost.** MEASURED F7: at M = 2, every ply, r = 2 it pays the debt (2/2/2/2);
  at `except-ply0` it lifts 51 and 99 stones off zero (2/2/1/1) **with the root's
  set left whole**, which is the only row in the field that does both. MEASURED
  F2: 11.9–32.5× cheaper per node. **But MEASURED, its move choices on this
  fixture are locality artefacts**: at M = 2 it plays `773,1/774,0` at 99 stones,
  `389,1/390,0` at 51 and `149,1/150,0` at 21 — always beside the last stone,
  never anywhere else. And MEASURED, its advantage is a function of |ball|: on the
  book, where the ball is ~78 cells, the whole-seat wall times differ by under
  1 % (378 694 against 381 324 ms), so the term it removes is not the term the
  SPRT's own sample pays.
- **Debt.** The D-95 class, at the price of a NEW selection concept.
- **Kill.** Not killed. **It is recorded as LICENSED, NOT SELECTED**: it needs a
  tunable this decision does not otherwise introduce, its exclusion key is not
  the eval's judgement, and its measured advantage is concentrated where the SPRT
  cannot see it. §5 says what would promote it.

### W-PFO — POOL PREFILTER BY RING/`(q, r)` ORDER

- **Mechanism.** The whole ball truncated in its own ascending order.
- **Wrongly excludes.** Everything but one end of the board.
- **Cost.** MEASURED F10: the deepest seat in this document, and it plays beside
  the origin at 11, 51 and 99 stones alike.
- **Kill.** The degeneracy, MEASURED rather than argued. It is carried because it
  is the instance the red team measured, and because it is what makes F10
  demonstrable.

### W-2R — TWO-RADIUS: FULL NEAR RING, CAPPED FAR RING

- **Mechanism.** Emit `ball(r_near)` whole, then the top K of the rest.
- **Cost.** **It cannot bound the class it targets**: `ball(1)` is 6 cells per
  isolated stone, MEASURED 596.0 at 99 stones, so the uncapped part alone grows
  linearly in the stone count. It also keeps F2's whole cost.
- **Kill.** The unbounded near ring.

### W-W — STAGED WIDENING ON FAIL-LOW (`WPQ_seed.md`'s W-E on this row)

- **Mechanism.** Rank once, search the first batch, widen on a fail-low, last
  entry finite.
- **Cost.** **F4 is the kill and it is structural:** the root can never widen and
  a non-PV node either fails high early or fails low and is then widened to full
  width, so the schedule does not bind.
- **Kill.** F4, already met. The row exists because D-315 records W-E as never
  having been an option among options; the field is what makes it one.

### W-A — ADAPTIVE K (K a function of the ball size)

- **Mechanism.** `K = max(K_min, |ball| / f)`, a second tunable.
- **Cost.** A second calibration axis, and a K that grows exactly where the
  measurement says a smaller one is better.
- **Kill.** MEASURED F7: a single fixed K spans 11 to 99 stones. The adaptive
  term buys nothing the fixed one does not, at the price of a registered
  parameter — and F10 says the channel that would have justified it cannot rank.

---

## 2. THE RADIUS AXIS

W-CFG is the option a red team supplies if the matrix does not. MEASURED,
`movetime 500`, completed `depth_turns` at 11 / 21 / 51 / 99 stones:

| `quiet_radius` | no cap | every ply, K = 8 | every ply, K = 32 | every ply but 0, K = 8 |
|---|---|---|---|---|
| 1 | **2 / 1 / 1 / 0** | 4 / 4 / 4 / 4 | 3 / 3 / 3 / 3 | 3 / 2 / 1 / 1 |
| 2 | **1 / 1 / 0 / 0** | 4 / 4 / 4 / 3 | 3 / 3 / 3 / 2 | 2 / 1 / 1 / 0 |
| 3 | **1 / 0 / 0 / 0** | 4 / 4 / 3 / 3 | 3 / 3 / 2 / 1 | 1 / 1 / 0 / 0 |

**W-CFG's kill condition fires**: at `quiet_radius = 1`, the smallest value
`Searcher::new` accepts, the 99-stone position still completes zero iterations.
Revision 1 also claimed "every capped cell beats every uncapped cell at the same
radius"; six of the 36 comparisons are ties (RED-TEAM M14), and the true
statement is *at least as good, and strictly better in 30 of 36*.

---

## 3. WHAT THE GOVERNED RUN WOULD SEE — RE-TAKEN AT EVERY SCOPE

Revision 1 measured this at `except-ply0` and reported it as a property of
`every-ply` (RED-TEAM B4), against a denominator including cells the cap could
not have removed (RED-TEAM M5). Both are corrected. MEASURED,
`random_openings_v1.txt`, **all 2 000** openings, `Stop::Nodes(50_000)`,
`quiet_radius 2`, ordering heuristics off:

| seat | openings at completed depth ≥ 3 | safety-net rows capped | opponent count-two cells cut | Σ wall ms |
|---|---|---|---|---|
| **W-N incumbent** | **121 / 2 000 (6.1 %)** | 0 / 1 936 431 | 0 | 381 324 |
| **W-K0** ply0-only, K = 8 | **644 (32.2 %)** | 5 400 / 520 164 (**1.0 %**) | 16 762 / 4 530 244 (**0.37 %**) | **286 645** |
| **W-K2** except-ply0, K = 32 | 682 (34.1 %) | 1 209 126 / 1 214 170 | 2 425 555 / 9 384 074 (25.9 %) | 392 979 |
| **W-K2** except-ply0, K = 16 | 964 (48.2 %) | 795 860 / 801 213 | 2 917 284 / 6 611 517 (44.1 %) | 386 885 |
| **W-K2** except-ply0, K = 8 | **1 083 (54.2 %)** | 611 121 / 616 627 | 2 926 562 / 5 181 856 (**56.5 %**) | 378 694 |
| **W-K2** except-ply0, K = 4 | 1 280 (64.0 %) | 521 168 / 526 931 | 2 896 585 / 4 331 529 (66.9 %) | 384 301 |
| **W-K1** every-ply, K = 8 | **2 000 (100 %)** | 263 291 / 263 291 | 1 377 486 / 2 241 129 (**61.5 %**) | 442 277 |

**The denominator, corrected.** `opp2` above is every opponent live-count-two
cell on the board; a cell outside the quiet ball was never a candidate, so its
non-emission is not the cap's doing and the denominator the cap can be measured
against is the count-two cells IN the pool it truncates (RED-TEAM M5, whose
reading of revision 1's own gloss — *"the one threat-shaped class a safety-net
row can **hold**"* — is right). MEASURED this revision over the book's first
300 openings, same budget and radius:

| seat | cut / all cells | cut / cells IN the pool |
|---|---|---|
| W-K2 `except-ply0`, K = 4 | 0.6671 | 0.8560 |
| W-K2 `except-ply0`, K = 8 | 0.5679 | **0.7390** |
| W-K2 `except-ply0`, K = 16 | 0.4452 | 0.5862 |
| W-K2 `except-ply0`, K = 32 | 0.2567 | 0.3288 |
| W-K1 `every-ply`, K = 8 | 0.6083 | **0.7949** |
| W-K0 `ply0-only`, K = 8 | 0.0036 | **0.0044** |

The `every-ply` K = 8 row replicates RED-TEAM R7's own 300-opening cell
(0.6083 / 0.7949) to four figures on a separately written instrument, which is
what makes the other four rows readable. **The exclusion is materially larger
than revision 1 stated, on both the numerator's scope and the denominator's
membership: revision 1 quoted 25.8 %, and the selected row's number is 73.9 %.**

**The divergence rate, on BOTH trajectories** (RED-TEAM M19 is right that
revision 1 measured only the incumbent's). 25 games from the book, turn cap 40,
`Stop::Nodes(50_000)`, W-K2 at K = 8, the other seat asked the same question from
the same position at every turn:

| trajectory played | searches | safety-net-bearing | turn diverged |
|---|---|---|---|
| incumbent's own moves | 631 | 120 (19.0 %) | **42 (6.66 %)** |
| the capped engine's own moves | 654 | 108 (16.5 %) | **34 (5.20 %)** |

**The SPRT is not predictably insensitive, and that conclusion no longer rests on
one trajectory.** A change that moves the played turn on 5–7 % of a governed
game's searches is inside what a paired SPRT over this book can see. This is a
statement about SENSITIVITY and not direction. Revision 1 also offered
`diverged_and_bearing == diverged` as corroboration; **that equality is a theorem
of the instrument** — the cap is a no-op at every non-safety-net node, so the
trees are bit-identical when no such node is visited — and it is withdrawn
(RED-TEAM M7, `docs/process.md`'s own vacuous-criterion clause).

---

## 4. THE DECISION IS TWO DECISIONS, AND THAT IS THE FINDING

The scopes partition the plies, so **W-K1 = W-K0 ∪ W-K2 exactly**. Revision 1
recommended the union because it does everything. Measured, the two halves are
not two halves of one thing:

| | what it buys | what it excludes | where the risk is |
|---|---|---|---|
| **W-K0**, the root half | the D-95 debt entirely (F7: 0 → 2 at every stone count); 121 → 644 book openings at depth ≥ 3; 25 % faster wall | **0.37 %** of the risk class | ALL of it at ply 0 — D-124's blind spot, F6's 957 searched candidates, F3's false comment |
| **W-K2**, the interior half | nothing on the D-95 debt (F7); 121 → 1 083 book openings at depth ≥ 3; wall neutral | **56.5 %** of the risk class | interior tails only; the root is untouched |

They serve different objectives, their risks sit at different nodes, and each is
independently measurable. **Bundled, an `h1` cannot say which half earned it and
an `h0` cannot say which half lost it** — and this project's own rule is one
change, one commit, one SPRT. That is the ground on which the union is not
selected, and it is a stronger ground than either half's own defects.

---

## 5. RECOMMENDATION — **W-K2**, with the root half deferred as its own package

**W-K2: score-ranked top-K of the quiet ball, at every ply but ply 0, inside
F1's `tier_t.is_empty()` branch, behind a gate that is `false` in every committed
config.**

Six grounds, each a measured cell rather than an argument:

1. **It introduces no exclusion at the node where the move is chosen.** F3 stays
   true as written, D-124 is not engaged, and the correctness hole the
   immediately preceding package found and fixed by excluding ply 0 does not
   reappear. Every other row that pays the D-95 debt does engage it.
2. **Its effect is large on the sample the SPRT actually walks.** MEASURED §3:
   121 → 1 083 of 2 000 book openings at completed depth ≥ 3 at K = 8.
3. **It is visible to the SPRT on both trajectories.** MEASURED §3: 6.66 % and
   5.20 % turn divergence.
4. **It is inert where the class is rare.** MEASURED F9: identical node counts,
   identical completed depths position for position, corpus wall ratio 1.002 /
   0.995.
5. **It is sound in the transposition table, measured, not argued.** F11: 0
   membership disagreements at this scope, where W-K3's scope produces 821.
6. **What it prunes carries no forced obligation at the node where it is
   pruned.** F1's guard, stated at the strength F1 actually supports — not
   revision 1's "provably not tactical", which RED-TEAM M8 correctly refused.

**WHAT SELECTING W-K2 COSTS, STATED AND NOT DISCOUNTED.**

- **It does not pay the D-95 / WP-1.4 depth debt.** MEASURED F7: 51 and 99 stones
  stay at completed depth 0 at `movetime 500`, r = 3. The WP that closes that
  debt is the one that narrows the root on this row, and §4 is why that is a
  different package. This is recorded here so no closure summary can imply
  otherwise, and D-95's own flip clause stays armed.
- **It excludes 56.5 % of the opponent's live-count-two cells** at K = 8 on the
  book, and more against the pool denominator. That class is the only
  threat-shaped one a safety-net row can hold; a count-two window needs four more
  stones, two whole turns, which makes the exclusion survivable and not
  harmless.
- **It is the field's worst wall time on `spread_v1`** (F8: 16 441 ms for 50 176
  nodes at 99 stones), because it removes subtrees without removing F2's eval
  pass. The registered bench must read time-to-depth with its direction stated,
  and must not read nps as a like-for-like unit (D-374).

**THE ANSWER F12 DEMANDS.** W-K2 does **not** re-purpose `quiet_top_k` or
`widen_schedule`. Those two keys carry `U3_tier_t.md` §10's committed semantics —
a quiet tier ADDED beyond Tier T — in twelve documents, they are validated and
unread, and silently inverting them is the defect that stopped WP-1.5c. The
design introduces its own key with its own name and its own off-value, and states
in one place that the two existing keys remain validated-and-unread; a third
knob whose semantics are stated once is not two schema places for one default.

**WHAT IS DEFERRED, NAMED SO IT IS NOT LOST.**

- **W-K0, the root half.** MEASURED to pay the D-95 debt at every stone count and
  to be the cheapest row in the field by exclusion (0.37 %). It is deferred, not
  killed, and it owes its own package with its own SPRT arm — and, because F3
  makes the ply-0 fail-open comment false, its own correctness fix there.
- **W-PFR, the pool prefilter.** MEASURED 11.9–32.5× cheaper per node and the
  only row that lifts the spread class with the root's set left whole. Deferred
  because it introduces a selection key that is not the eval's judgement and a
  tunable this decision does not need, and because its advantage is measured to
  be concentrated where the SPRT cannot see it. **What would promote it:** a
  measured whole-engine cost finding that names the safety-net eval pass as a
  hotspot on a governed-shaped workload, which no rule-5 receipt currently does.

**THREE THINGS THIS RECOMMENDATION DOES NOT SETTLE**, listed so they are attacked
as open:

- **K itself.** No value is selected. Its calibration rule, the DIRECTION of
  every registered ratio, and the treatment of positions where a quantity is
  undefined are the design's, registered before the calibration runs;
  `sessions/WP-1.9/wp19_design_REVIEW_rev2.md` BLOCKING N1 (a)(b)(c) is that
  paragraph's checklist. **F10 is the trap**: the deciding channel is monotone in
  the direction of narrowing, so a rule that maximises depth selects the grid
  minimum — the mirror image of the defect that failed WP-1.5c — and §3's book
  table is monotone the same way (K = 4 beats K = 8 beats K = 16 beats K = 32 on
  depth and loses to all of them on exclusion).
- **Whether the table move is promoted before or after the truncation.**
  `pvs.rs:328` promotes within `cells[forced..]`; which side of the truncation it
  falls on decides whether the emitted set is a pure function of the position,
  and F11's soundness measurement was taken with the promotion AFTER.
- **The gate's own surface.** F12 says a new key is owed; where it lives, what
  its off-value is, and which of the twelve documents gain it are the design's.

---

*M2, revision 2. Nothing here is selected. The DECISION-RED-TEAM attacks this
revision before selection, as it attacked revision 1; the selection record is a
separate file, and `docs/experiments/matrix_M2_REDTEAM.md` is not edited by
either.*
