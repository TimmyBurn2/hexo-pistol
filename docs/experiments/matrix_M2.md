# MATRIX M2 — the safety-net candidate cap

**WP-1.5d. Authored before any option is selected and before any line of the
mechanism is written.** `docs/decisions.md` D-310 excised the quiet stage and its
widening schedule from WP-1.5b; D-315 scheduled them into WP-1.5c and recorded
that **M2 has never been authored in the form its candidate takes** — `W-E`
occurs zero times at `ec8f7fb`, was supplied by the DECISION-RED-TEAM that killed
`W-A`, and was never an option among options. This is that authoring. It is a
fresh matrix and not a recovery.

**THE DECISION, STATED SO A REVIEWER CAN CHECK WHAT IS AND IS NOT BEING DECIDED.**
On a BATCHED or BATCHED-lost row where Tier T comes back empty, the candidate set
today is the WHOLE quiet ball. What, if anything, replaces it? Nothing else about
the node protocol is in scope: Tier F, Tier T, the FILTERED row's cover union and
the WIN-NOW row are untouched by every option below, and the option that touches
them is not in the field because it is not a safety-net option.

**Every numeric claim below is marked MEASURED or ESTIMATED (D-291).** Where a
number could have been measured in seconds it was, and the receipt is named. The
instrument for every MEASURED cell taken this session is
`artifacts/wp15d_m2_evidence_v1.txt` (sha256 `db8a8793…`), whose complete driver
— counter module, `pvs.rs` patch and both test drivers — is
`artifacts/wp15d_m2_evidence_instrument_v1.txt` (sha256 `081c928a…`), stored
whole because a stash does not capture untracked files (WP-1.5c m11). Both were
taken in a detached worktree at `fbc8e62` with its own `CARGO_TARGET_DIR`; no
scratch edit entered the live tree. The census cells are
`artifacts/wp19_design_census_v1.txt`, taken by WP-1.5c.

---

## 0. THE FACTS THE FIELD IS SCORED AGAINST

Nine facts. Each is either quoted at `file:line` or MEASURED with a named
receipt, because a dispatcher's claim about a mechanism is unverified until the
executing session quotes it (D-474).

**F1 — the safety net is a branch, and both tiers are empty inside it.**
`crates/pistol-search/src/staged.rs:222-236`:

```rust
let mut tier_t = tier_t_union(threats, us, params);
if tier_t.is_empty() {
    tier_t = within_radius(board, params.quiet_radius);
    out.used_quiet_safety_net = true;
}
delta_rank(&mut tier_t, eval, us);
out.cells = tier_t;
out.forced = 0;
```

Tier F is provably empty on every row `batched` is called from — that function's
own doc at `staged.rs:209-213` — and Tier T is empty by the branch condition. So
**no cap placed inside this branch can exclude a cell Tier F or Tier T would have
emitted**, and that is a GUARD (the `if`), not a construction. The distinction
matters: WP-1.5c's REVIEW-design m1 found "by construction and not by a guard"
wrong for exactly this shape.

**F2 — the per-node cost of this row is independent of any cap.**
`candidates.rs:42-61` builds the ball as a `BTreeSet` over `stones × ball_offsets`
and then filters by legality; `staged.rs:287-295`'s `delta_rank` calls
`Eval::delta` on **every cell of the vector it is handed**. The safety net hands
it the whole ball (`staged.rs:230, 233`). So a safety-net node costs
Θ(|stones|·|offsets| + |ball|·eval) whatever K is. **A top-K cap never reduces the
cost of a safety-net node; it reduces how many nodes pay it.** This is WP-1.5c's
own `delta_rank` lesson, restated where it bears on this decision.

**F3 — the root's zone restriction and its fail-open run at ply 0 only.**
`pvs.rs:336-347`, quoted:

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

On a safety-net row `set.forced == 0`, so `forced_intact` is vacuously true. Any
option that caps at ply 0 changes what `unrestricted` means — the fail-open then
restores the CAPPED set, not the ball — and that is a named cost in the rows
below, not a footnote.

**F4 — a fail-low widening trigger cannot bind on this recursion.** Re-verified
at HEAD, with the line numbers WPQ_seed §7.1's citations have since drifted from:
the root opens at `visit(depth_plies, -INFINITY, INFINITY, 0)` (`pvs.rs:153`) and
`best_score` starts at `-INFINITY` (`pvs.rs:368`), so `best_score <=
original_alpha` is unsatisfiable once any child returns a score, and **the root
can never widen**; a non-PV child opens at `(alpha, alpha + 1)`
(`pvs.rs:498, 500`), so any `score > alpha` sets `alpha >= beta` and breaks
(`pvs.rs:427-429`), and **a non-PV node either fails high early or exhausts its
batch having failed low** — after which a widening trigger widens it to full
width. MEASURED by the seed's own instrument: 0 root widenings in 101 root
iterations, 0 of 2,022,904 non-PV interior nodes truncated.

**F5 — the class's rate and width. MEASURED**
(`artifacts/wp19_design_census_v1.txt`, WP-1.5c's census at `41b56fae` over
`11c102e`):

| corpus | safety-net rows / batched rows | rate | mean width | share of all batched cells |
|---|---|---|---|---|
| `bench_positions_v1` (24 pos, 15/35 stones) | 66 / 108 662 | **0.000607** | **78.12** | 5 156 / 3 391 125 = 0.0015 |
| `spread_v1` (4 pos, 11/21/51/99 stones) | 2 468 / 4 015 | **0.6147** | **1 218.33** | 3 006 848 / 3 017 096 = **0.9966** |

**F6 — the debt is live at HEAD, re-measured rather than inherited.** MEASURED
this session at `fbc8e62` through the shipped `target/release/pistol` and
`tools/bench_block.sh`, `go movetime 500` on `spread_v1`: completed
`depth_turns` **1 / 0 / 0 / 0** at 11 / 21 / 51 / 99 stones under
`configs/play_v0.toml` (radius 3) **and** under `configs/play_staged_v0.toml`
(staged, `quiet_radius = 3`). At 21 stones and above the engine returns a move no
completed iteration produced. That is `WPQ_seed.md`'s WP-1.4 baseline, unchanged
by everything that has landed since, and it is what this WP is for.

**F7 — only a ROOT-INCLUSIVE cap moves F6. MEASURED**, `movetime 500`,
`quiet_radius 3`, completed `depth_turns` at 11 / 21 / 51 / 99 stones:

| K | cap at every ply | cap at every ply but 0 | cap at non-PV nodes only |
|---|---|---|---|
| **0 (no cap)** | **1 / 0 / 0 / 0** | — | — |
| 4 | **5 / 5 / 4 / 3** | 1 / 1 / 0 / 0 | 1 / 1 / 0 / 0 |
| 8 | **4 / 4 / 3 / 3** | 1 / 1 / 0 / 0 | 1 / 1 / 0 / 0 |
| 16 | **4 / 3 / 3 / 2** | 1 / 1 / 0 / 0 | 1 / 1 / 0 / 0 |
| 32 | **3 / 3 / 2 / 1** | 1 / 1 / 0 / 0 | 1 / 1 / 0 / 0 |
| 64 | 3 / 2 / 2 / 1 | 1 / 0 / 0 / 0 | 1 / 1 / 0 / 0 |
| 128 | 1 / 2 / 1 / 1 | 1 / 0 / 0 / 0 | 1 / 0 / 0 / 0 |

**This refutes the arithmetic the option field would otherwise have been built
on**, and it is why it was measured before the field was scored. The prediction
was that capping the non-root plies alone takes the 99-stone first iteration from
ball² to ball·K and therefore restores a completed iteration. It does not: F2 is
why. With the root uncapped, the first iteration still expands ~3 578 ply-1 nodes
and **each one pays the whole ball's construction and ranking whatever K is**. At
K = 4 the 99-stone search reaches 15 279 nodes in 500 ms — 30 k nps against the
uncapped seat's 403 k — and completes nothing.

**F8 — at the reproducible budget the same ordering holds, and nps is not the
unit. MEASURED**, `Stop::Nodes(50_000)`, `quiet_radius 2`, `spread_v1`:

| seat | depth 11/21/51/99 | nodes | wall ms (99 stones) |
|---|---|---|---|
| no cap | 1 / 1 / 1 / 1 | 58 368 / 201 728 / 1 149 952 / **4 283 392** | 10 648 |
| every ply, K = 4 | **5 / 4 / 5 / 5** | 50 176 each | **907** |
| every ply, K = 8 | 4 / 4 / 4 / 4 | 50 176 each | 869 |
| every ply, K = 32 | 3 / 2 / 2 / 2 | 50 176 each | 809 |
| every ply but 0, K = 4 | 2 / 2 / 2 / 1 | 50 176 each | **16 441** |

The uncapped seat's node counts are D-74's own behaviour — the first iteration is
not interruptible under a reproducible stop — and they are why **nps is not a
like-for-like unit across these seats** (D-374's standing lesson, at 4.28 M mostly
leaf nodes against 50 k mostly interior ones). Completed depth at a fixed budget
is the channel this class is read on.

**F9 — the cap is inert where the class is rare. MEASURED**, `bench_positions_v1`
at `Stop::Nodes(50_000)`, `quiet_radius 2`, cap at every ply but 0:

| K | Σ nodes | completed depth, 24 positions | Σ wall ms | ratio to K = 0 |
|---|---|---|---|---|
| 0 | 1 104 026 | `3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3` | 4 825 | 1.000 |
| 8 | 1 104 026 | identical, position for position | 4 836 | 1.002 |
| 32 | 1 104 026 | identical, position for position | 4 801 | 0.995 |

Node counts identical and depths identical: on the corpus this decision changes
nothing measurable, which is the cost side of every row below and the reason the
corpus bench is a no-regression check rather than a gain channel.

---

## 1. THE FIELD

Seven options. The ply scope is part of an option's mechanism rather than a
second axis, because F7 measures the scope as the term that decides whether an
option works at all — a field that factored it out would score six rows on a
question the measurement has already answered.

### W-N — NO CAP (the null option, and the incumbent)

- **Mechanism.** None. `staged.rs:222-236` unchanged.
- **What it can wrongly exclude.** Nothing.
- **Cost shape.** MEASURED, F6: completed depth 0 at 21 stones and above at
  `movetime 500`, so on this class the engine plays a move no iteration produced.
  MEASURED, F8: 4 283 392 nodes and 10.6 s for one completed turn of depth at 99
  stones under a 50 000-node budget.
- **Debt it touches.** None. D-310's excised-axis debt and the D-95 / WP-1.4
  depth debt both stay open, and D-310's flip clause stays armed.
- **Kill condition.** Any option that raises completed depth on `spread_v1` at
  `movetime 500` while leaving the corpus bench inside its registered bracket.
  MEASURED: F7's first column does this at every K on the grid.

### W-CFG — SHRINK `quiet_radius` (config only, zero code)

- **Mechanism.** None in code. Under `Staged`, `quiet_radius` reaches only two
  callers — `candidates::candidate_cells`'s `Staged` arm (`candidates.rs:31`,
  serving `fallback_turn` and `check_root`) and the safety net itself
  (`staged.rs:230`) — so lowering it IS a safety-net narrowing, available to the
  operator today with no gate and no ADR.
- **What it can wrongly exclude.** Every cell of the outer rings, at every node
  and both tiers' rows alike, including the fallback answer and `check_root`'s
  reachability question.
- **Cost shape.** **It scales the ball; it does not bound it.** An isolated stone
  contributes 6 cells at radius 1 and 18 at radius 2, so width still grows
  linearly in the stone count and the class is unbounded at any radius ≥ 1.
  MEASURED at radius 3 the 99-stone ball is 3 578 cells (F7's `pool_mean`); the
  radius axis is measured in §2 rather than argued.
- **Debt it touches.** Partially the D-95 class, at the price of narrowing every
  other row's fallback.
- **Kill condition.** If the radius sweep leaves completed depth at 0 on the
  51- and 99-stone positions, the option cannot pay the debt it is proposed for.

### W-K1 — SCORE-RANKED TOP-K, AT EVERY PLY

- **Mechanism.** Inside F1's branch, after `delta_rank`, keep the first K.
  One truncation, no new ranking, no new pass over the ball.
- **What it can wrongly exclude.** (i) At the root, any cell outside the top K of
  the static ranking — D-124's blindness at the node where the move is chosen.
  (ii) The opponent's development cells at live count **exactly two**, which
  Tier T's threshold-3 opponent reading does not qualify and which are therefore
  the only threat-shaped class present on a safety-net row at all. MEASURED over
  the SPRT book's own 2 000 openings at K = 32: 2 425 555 of 9 384 074 such cells
  removed (**25.8 %**). Neither is immediately fatal — a count-two window needs
  four more stones, two whole turns — but both are real and neither is argued
  away here.
  (iii) Under `root_restrict` (F3) the fail-open restores the CAPPED set rather
  than the ball, so a root whose zone intersects the ball but not the top K
  searches a strictly smaller set than the gate-off engine would.
- **Cost shape.** MEASURED, F7 and F8: the only option in the field that moves
  F6, at every K on the grid, and the smallest K is the best. MEASURED, F9: inert
  on the corpus.
- **Debt it touches.** D-95 / WP-1.4's depth debt directly; D-310's excised-axis
  debt on `WPQ_seed.md`'s own definition of it (*"Tier F and Tier T are both
  EMPTY"*).
- **Kill condition.** A corpus bench outside its registered bracket, or a
  calibration in which no K both raises `spread_v1` depth and holds the corpus.

### W-K2 — SCORE-RANKED TOP-K, EVERY PLY BUT 0

- **Mechanism.** W-K1 plus a `ply > 0` guard.
- **What it can wrongly exclude.** Only W-K1's (ii). The root keeps full width,
  so D-124's blindness is not introduced where the move is chosen and F3's
  fail-open is untouched — its `unrestricted` is still the ball.
- **Cost shape.** MEASURED, F7: **it does not move F6** — completed depth stays
  0 at 51 and 99 stones at every K on the grid, and F2 says why. MEASURED, F8: it
  makes wall time-to-fixed-nodes WORSE than the incumbent at 99 stones (16 441 ms
  against 10 648 ms), because the nodes it visits are interior ones that each pay
  the whole ball.
- **Debt it touches.** MEASURED it does not pay the D-95 debt. It does raise
  completed depth on the SPRT book's own openings (§3).
- **Kill condition.** F7. It is the option whose safety this WP would most like
  to have and the measurement declines to give it.

### W-K3 — SCORE-RANKED TOP-K, NON-PV NODES ONLY

- **Mechanism.** W-K1 under `!is_pv`, which `staged_candidates` already receives
  (`staged.rs:119`). The root and every PV node keep full width, which is
  `WPQ_seed.md` §7.2's own W-E scope applied to this row.
- **What it can wrongly exclude.** W-K2's set, minus the PV spine.
- **Cost shape.** MEASURED, F7: indistinguishable from W-K2 on the class, and
  for W-K2's reason. Additionally, a PVS re-search converts a capped node into a
  full-width one (`pvs.rs:503-507`: a scan landing inside the window is re-run at
  the full window), so its effective width is a function of the ordering rather
  than of K — MEASURED, `emit_mean` runs 4.40 to 26.39 at K = 4 rather than 4.00.
- **Debt it touches.** As W-K2: none of the D-95 class.
- **Kill condition.** F7, and the width instability above, which makes its own
  parameter not the thing that decides its width.

### W-P — PROXIMITY-RANKED TOP-K

- **Mechanism.** Rank the ball by hex distance to the nearest stone and truncate,
  ranking only the survivors by `Eval::delta` — the one option whose ranking cost
  is **not** Θ(|ball|·eval), because a ring index is geometry and needs no eval.
- **What it can wrongly exclude.** Everything in the outer rings, which is what
  it is for; and — the structural objection — on the class it targets the
  proximity key is nearly constant. On `spread_v1` every stone is exactly
  distance 8 from its predecessor, so at `quiet_radius` r every ball cell lies in
  ring 1..r of SOME stone and the key takes r distinct values over ~3 578 cells.
  The tie-break then carries the whole decision, and a lexicographic `(q, r)`
  tie-break concentrates the emitted set at one end of the line — a positional
  bias with no game meaning.
- **Cost shape.** ESTIMATED: saves |ball| − K `Eval::delta` calls per safety-net
  node, which F2 makes the larger half of that node's cost; costs a distance pass
  the ball construction does not already do. **Not measured**, and marked so: the
  structural objection above is what keeps it out of the recommendation, not the
  cost.
- **Debt it touches.** The same as W-K1's if the tie-break can be made to mean
  something.
- **Kill condition.** The degeneracy: if the proximity key takes r values on the
  class, W-P is W-K1 with a worse tie-break wherever the eval discriminates at
  all, and a coordinate-order prune wherever it does not.

### W-2R — TWO-RADIUS: FULL NEAR RING, CAPPED FAR RING

- **Mechanism.** Emit `ball(r_near)` whole, then the top K of
  `ball(quiet_radius) \ ball(r_near)`.
- **What it can wrongly exclude.** Far-ring cells below K.
- **Cost shape.** **It cannot bound the class it targets**, for W-CFG's reason
  with the near ring in place of the whole ball: `ball(1)` is itself 6 cells per
  isolated stone, so at 99 stones the uncapped part alone is ~594 cells and the
  emitted width is 594 + K where W-K1's is K. It also keeps F2's whole cost.
- **Debt it touches.** The D-95 class partially, by a factor that shrinks as the
  stone count grows — which is the wrong direction.
- **Kill condition.** The unbounded near ring.

### W-W — STAGED WIDENING ON FAIL-LOW (`WPQ_seed.md`'s W-E, applied to this row)

- **Mechanism.** Build the ranked ball once, search the first batch, enter the
  next only on a fail-low, last schedule entry finite.
- **What it can wrongly exclude.** The tail past the last finite entry, on nodes
  that never failed low.
- **Cost shape.** **F4 is the kill and it is structural, not empirical:** the
  root can never widen and a non-PV node either fails high early or fails low and
  is then widened to full width. The schedule therefore does not bind, and what
  it adds is bookkeeping plus a TT truncation rule the seed itself specifies to
  cover a case that F4 says does not arise on this recursion.
- **Debt it touches.** None, because it does not bind.
- **Kill condition.** F4, and it is already met. This row is authored because
  `WPQ_seed.md` records W-E as the adopted-looking text and D-315 records that it
  was never an option among options; the field is what makes it one, and the
  field is where it falls.

### W-A — ADAPTIVE K (K a function of the ball size)

- **Mechanism.** `K = max(K_min, |ball| / f)`, a second tunable.
- **What it can wrongly exclude.** W-K1's set, at a K that varies with the
  position.
- **Cost shape.** One more calibration axis, and a K that grows exactly where F7
  measures a smaller K to be better (at 99 stones K = 4 completes depth 3 where
  K = 128 completes depth 1).
- **Debt it touches.** W-K1's.
- **Kill condition.** MEASURED, F7: a single fixed K already spans 11 to 99
  stones — K = 4 gives 5 / 5 / 4 / 3 — so the adaptive term buys nothing the
  fixed one does not, at the price of a second registered parameter.

---

## 2. THE RADIUS AXIS, MEASURED RATHER THAN ARGUED

W-CFG is the option a red team supplies if the matrix does not, because it is
available today and costs no code. It is priced on the same class, the same
clock and the same instrument as every code option. MEASURED (§E of the
receipt), `movetime 500`, completed `depth_turns` at 11 / 21 / 51 / 99 stones:

| `quiet_radius` | no cap | every ply, K = 8 | every ply, K = 32 | every ply but 0, K = 8 |
|---|---|---|---|---|
| 1 | **2 / 1 / 1 / 0** | 4 / 4 / 4 / 4 | 3 / 3 / 3 / 3 | 3 / 2 / 1 / 1 |
| 2 | **1 / 1 / 0 / 0** | 4 / 4 / 4 / 3 | 3 / 3 / 3 / 2 | 2 / 1 / 1 / 0 |
| 3 | **1 / 0 / 0 / 0** | 4 / 4 / 3 / 3 | 3 / 3 / 2 / 1 | 1 / 1 / 0 / 0 |

**W-CFG's kill condition fires.** At its floor — `quiet_radius = 1`, the smallest
value `Searcher::new` accepts — the 99-stone position still completes **zero**
iterations. The radius scales the ball (MEASURED mean widths at 99 stones: 596.0
at r = 1, 1 789.2 at r = 2, 3 578.5 at r = 3) and the ball still grows linearly
in the stone count, so no radius bounds the class. The option is real and it is
free, and it does not reach the debt.

Two further readings the table settles rather than leaves:

- **The cap dominates the radius, and they compose.** At r = 1 with no cap the
  99-stone position is at depth 0; at r = 3 WITH a K = 8 root-inclusive cap it is
  at depth 3. Every capped cell in the table beats every uncapped cell at the
  same radius.
- **The ply scope survives the radius change.** The `every ply but 0` column is
  worse than the `every ply` column at every radius, and at r = 3 it is back to
  the incumbent's own 1 / 1 / 0 / 0. F7 is not an artefact of `quiet_radius = 3`.

---

## 3. WHAT THE GOVERNED RUN WOULD SEE

The dispatch requires this to be measured from the book's own openings rather
than assumed, because a change inert on the governed sample makes its SPRT
predictably insensitive and the honest expectation would then be `h0`-or-
inconclusive with the value living on the spread class in bench.

**MEASURED, the book's own openings** (§D of the receipt):
`random_openings_v1.txt`, all **2 000** openings, `Stop::Nodes(50_000)`,
`quiet_radius 2`, cap every ply but 0:

| seat | safety-net rows / nodes | completed-depth histogram over the 2 000 |
|---|---|---|
| no cap | 1 936 431 / 100 352 000 = **1.93 %** | d1 10, d2 1 869, d3 113, d4 8 |
| K = 32 | 1 214 170 / 100 352 000 = 1.21 % | d2 1 318, d3 **612**, d4 **70** |

Openings reaching completed depth ≥ 3 go from **121 / 2 000 (6.1 %)** to
**682 / 2 000 (34.1 %)**. The class is not exotic on this book: at five stones no
length-six window holds two own stones, which is exactly `WPQ_seed.md`'s
definition of it.

**MEASURED, on trajectories the governed run itself would walk** (§H): 25 games
from the book's first 25 openings, turn cap 40, both sides the UNCAPPED engine at
`Stop::Nodes(50_000)`, `quiet_radius 2` — and at every turn the capped engine
(K = 8, every ply) asked the same question from the same position:

- **631 searches**, 16 games decided before the turn cap.
- **120 searches (19.0 %) visited at least one safety-net row**; 55 690 such rows
  in total.
- **58 searches (9.19 %) would have been answered with a DIFFERENT turn.**
- `diverged_and_bearing == diverged == 58`: **every divergence is on a
  safety-net-bearing search**, which is the check that the cap is the whole cause
  and the measurement is not reading noise.

**So the SPRT is not predictably insensitive.** A change that moves the played
turn on 9.2 % of a governed game's searches is inside what a paired SPRT over
this book can see. That is a statement about SENSITIVITY and not about
direction: whether those 58 divergences are better turns is the SPRT's to say
and nothing here anticipates it.

---

## 4. RECOMMENDATION — **W-K1**, and what it costs to recommend it

**W-K1: score-ranked top-K of the quiet ball, at every ply, inside F1's
`tier_t.is_empty()` branch, behind a gate that is `false` in every committed
config.**

Five grounds, each one a cell above rather than an argument:

1. **It is the only option in the field that moves the debt.** F7 and §2:
   W-K2, W-K3, W-CFG and W-2R all leave the 99-stone position at completed depth
   0 at `movetime 500`; W-K1 reaches 3 at K = 8 and 5 at K = 4. W-W does not bind
   at all (F4).
2. **The reason the others fail is measured and structural, not incidental.**
   F2: a safety-net node's cost is Θ(|ball|) whatever K is, so an option that
   leaves the root uncapped leaves ~|ball| nodes each paying it — and F8 measures
   that seat taking 16 441 ms for the same 50 176 nodes the root-capped seat
   takes 907 ms for.
3. **It is inert where the class is rare.** F9: identical node counts, identical
   completed depths position-for-position, wall ratio 1.002 / 0.995 on the
   corpus.
4. **It is visible to the SPRT.** §3: 9.19 % of governed-shape searches change
   their answer, every one of them on a safety-net-bearing search.
5. **What it prunes is provably not tactical.** F1: both tiers are empty inside
   the branch, so no forced defence, no win-in-one and no cover cell can be cut.

**THE COST OF THE RECOMMENDATION, STATED AND NOT DISCOUNTED.** W-K1 caps the
ROOT, and `WPQ_seed.md` §7.2's own W-E says the root is never capped —
`docs/decisions.md` D-124 says no oracle catches a mistake there. Three things
separate this from the W-A that fell, and a reader should weigh them rather than
take the recommendation on the table alone:

- W-A capped the general candidate set at the root, where Tier F's wins and the
  cover union's forced defences live. W-K1 caps only a row on which F1 proves
  both tiers empty.
- MEASURED F6: on this class the incumbent's root completes **no iteration** and
  answers `Provenance::PartialRoot` — the best move found inside an aborted
  iteration. W-K1 replaces an unsearched answer with a searched one; it does not
  narrow a root that was working.
- The exclusion is not free even so. MEASURED: at K = 32 over the book's 2 000
  openings the cap removes **2 425 555 of 9 384 074 (25.8 %)** of the opponent's
  live-count-two cells — the one threat-shaped class a safety-net row can hold.

**And three things the recommendation does NOT settle**, listed so the red team
attacks them as open rather than as hidden:

- **K itself.** No value is selected here. K is a calibration whose rule,
  direction and treatment of undefined positions belong to the design and are
  registered before the calibration runs — WP-1.5c's re-review FAIL is that
  paragraph's checklist. F7's advisory table is monotone in the wrong direction
  for a naive rule (smaller K is always deeper), so a rule that maximises depth
  selects the grid minimum, which is the mirror image of the defect that failed
  WP-1.5c.
- **The `root_restrict` interaction (F3).** W-K1 caps ply 0, so the fail-open
  restores the capped set. The design must say what happens there and pin it.
- **Whether the table move is promoted before or after the truncation.**
  `pvs.rs:328` promotes within `cells[forced..]`; which side of the truncation it
  falls on changes whether the emitted set is a pure function of the position.

---

*M2. Authored before selection. Nothing here is selected; the DECISION-RED-TEAM
attacks this document, and the selection record is a separate file.*
