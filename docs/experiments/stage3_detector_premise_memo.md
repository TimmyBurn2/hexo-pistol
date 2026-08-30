# Stage-3 scoped detector — PREMISE MEMO (the premise gate)

**Verdict: PREMISE STOP before the option matrix. No matrix was authored, no
option was selected, no design was written, no engine code was touched.**

**Revision 2**, after a fresh-context PREMISE RED-TEAM returned **STANDS WITH
CORRECTIONS** on revision 1 — three BLOCKING, five MAJOR, six MINOR, every one
in the argument and none in the conclusion. §9 records what it changed and what
it could not break. This is the document's one fix round (D-481).

The premise that fails and forces the stop is **P2**. The package's axis is
CALL COUNT; the bracket's unit is nps under a SHARED node budget; D-477 names
that pair — *nodes against calls* — as a premise defect by name. A gated call
does not shrink the search, it *funds* it.

Correcting it moves the package's stated target from **~6x** to a **floor** of

- **190.0x / 250.9x / 35.8x** (corpus band 15 / band 35 / trigger-rich) at the
  WP-1.8c-era nps the bracket was registered against — between **32x and 42x**
  what was dispatched (190.0/6.0 and 250.9/6.0); and
- **379.3x / 507.7x / 88.3x** at HEAD, **MEASURED this session** on the two
  committed seats re-run unchanged (§3.5) — between **63x and 85x** what was
  dispatched (379.3/6.0 and 507.7/6.0).

The second row is the operative one, and the gap between the rows is itself the
finding: the OFF seat got 1.75x-2.02x faster across WP-1.9 and WP-1.9b while the
gate-on seat got *slightly slower*, because a df-pn visit does not go through
`pistol-eval`. **The bracket aborts harder today than when it was registered.**

A field of options arranged against the stated target would not be the field.
This is the disposition the dispatch itself names: *"a premise STOP is the
process succeeding."*

**Revision this memo is taken at:** `21e05f8` (`dev`, the WP-1.9b closure HEAD).

**Where the numbers come from.** Every number is quoted from a registered
instrument's artifact or is arithmetic over such lines, with the arithmetic
re-executable: `tools/stage3_premise_derive.py`, output
`artifacts/stage3_premise_derivation_v2.txt`. Where a quantity is a floor, an
assumption or a hypothetical, the word appears.

---

## 1. The four premises, adjudicated

| # | The dispatch's premise | Verdict |
|---|---|---|
| P1 | the current solver trigger and both call sites, quoted at `file:line` | **HOLDS** |
| **P2** | **the 1.8c bench's cost decomposition showing CALL COUNT as the binding term, from artifact lines and not summary prose** | **FAILS, two ways — and P2 ALONE forces the stop** |
| P3 | the per-call cost state after 1.8c's fast paths | **HOLDS** |
| P4 | the anchor-probe positions that constitute the value fixture | **FAILS as named**, repaired in §5 |

---

## 2. P1 — the trigger and both call sites. HOLDS.

### 2.1 The trigger is one variant, and it is a disjunction over both sides

`crates/pistol-search/src/params.rs:156-164`:

> ```rust
> /// What fires a solver call at a node (design wp18b §2 D1): the calculus
> /// ID names the pattern class, and v0 wires exactly one.
> #[derive(Debug, Clone, Copy, PartialEq, Eq)]
> pub enum SolverTrigger {
>     /// PAT-O4+ on either side: any hot window (an open four or better)
>     /// held by the mover or the opponent, read off the staged policy's
>     /// own `ThreatState`.
>     AnyOpenFour,
> }
> ```

HOT, at `crates/pistol-solver/src/query.rs:126-129`:

> ```rust
> /// `side`'s HOT windows: live, four or more own stones (D-243).
> pub fn hot_windows(&self, side: Player) -> &[Window] {
> ```

So the trigger fires wherever **either** side holds a live six-window with four
or more of its own stones — not an open four in the Connect6 sense, and not
conditioned on whose turn it is.

### 2.2 Call site 1 — the in-tree trigger

`crates/pistol-search/src/pvs.rs:265-272`:

> ```rust
> if ply > 0
>     && self.position.state().phase() == Phase::First
>     && self.solver.is_some()
>     && matches!(self.policy, CandidatePolicy::Staged(_))
>     && let Some(verdict) = self.solver_verdict()
> {
>     return verdict;
> }
> ```

and the predicate, `crates/pistol-search/src/pvs.rs:597-600`:

> ```rust
> let mover_hot = !threats.hot_windows(mover).is_empty();
> let opponent_hot = !threats.hot_windows(opponent).is_empty();
> if !mover_hot && !opponent_hot {
>     return None;
> }
> ```

**A FIRING IS NOT A CALL, and the distinction is load-bearing in §3.** One
firing makes one attacker call (`pvs.rs:609`, `solver.solve`) and then, only if
that did not prove, one defender call (`pvs.rs:630`, `solver.solve_defender`).
Each is separately capped at `per_call_node_cap` (`pvs.rs:592`), so a firing
costs up to **2 x cap**, not cap. At a mover-hot-only node the defender answers
in one visit — the code says so at `pvs.rs:626-628` and the bench shows it, a
`solver_nodes 2049` row being one capped call plus that single visit.

### 2.3 Call site 2 — the root gate

`crates/pistol-search/src/search.rs:250-256`:

> ```rust
> if let Some(wiring) = self.params.solver
>     && self.solver.is_some()
>     && state.stones_owed() == 2
>     && state.phase() == Phase::First
>     && root_triggers(&mut self.position, wiring.trigger)
> {
> ```

`root_triggers` is the same predicate spelled again
(`crates/pistol-search/src/search.rs:515-524`).

**P1 holds.** Both call sites exist where the dispatch says, both are gated by
the same trigger, and the trigger is one named enum variant a detector can be
placed in front of without touching solver machinery.

---

## 3. P2 — call count as the binding term. FAILS.

### 3.1 First failure: the axis is not a quantity this engine can report

The dispatch asks for the decomposition *"showing call count as the binding term
(artifact lines, not summary prose)"*. **No artifact line carries a call count,
because no counter does:**

```
$ git grep -c "solver_calls" -- crates/ tools/ configs/
(no matches)
```

The search keeps two counters and both are node counters
(`crates/pistol-search/src/info.rs:166-175`); the protocol prints those same two
(`crates/pistol-cli/src/report.rs:62-66`). D-465 states the position and marks
it:

> **The binding term is the number of solver CALLS times the per-call FIXED
> cost** (INFERRED, and marked so: there is no `solver_calls` counter, and adding
> one is the first thing the next package should do).

**The sharpest form of the failure, which is not merely that a quotation is
missing.** The quantity `u` — solver visits per search node — IS measurable and
is measured throughout §3. Its **factorisation** into (firings per search node)
x (visits per firing) is **not**, because no firing counter exists. So the
dispatch's target, stated in call count, is not a quantity the engine can report
at all; only `u` is. Nor is the factorisation inferable from the node totals:
over the 158 `solver_nodes` values in `artifacts/wp18c_bench_v1.txt`, 30 are
exact multiples of 2048, 8 are 2048k+1, and 100 are neither, so the
per-position totals do not divide into a firing count.

### 3.2 Second failure, and it is the one that decides the package: the axis is not the unit

D-477 names this class, including this exact pair of units:

> where the axis is a unit (plies against turns, cells against pairs, **nodes
> against calls**) the quotation names the line where that unit is CONSUMED, not
> where it is defined.

The line where the unit is consumed is the shared budget —
`crates/pistol-search/src/pvs.rs:140-142`:

> ```rust
> pub fn total_nodes(&self) -> u64 {
>     self.search_nodes + self.solver_nodes
> }
> ```

and `crates/pistol-search/src/pvs.rs:681`:

> ```rust
> self.aborted = self.stop.is_spent(self.total_nodes());
> ```

**A solver call and a search visit come out of one budget.** A call the detector
gates therefore does not shrink the search — it *funds* it. The search runs
further, and further means more nodes at which the trigger fires. The call count
is not a free parameter; it is a fixed point.

**THE BACKFILL NEEDS NO MODEL AND NO PRE-FIX DATA. The OFF seat IS the
gate-everything limit of any detector**, and both seats are in the same
post-budget-fix registered bench (`artifacts/wp18c_bench_v1.txt`, per position):

| band | OFF search | ON search | ON solver | ON total |
|---|---|---|---|---|
| CORPUS 15 | 50,176.0 | 2,846.9 | 49,137.1 | 51,984.0 |
| CORPUS 35 | 41,826.2 | 3,424.7 | 39,867.1 | 43,291.8 |
| TRIGGER-RICH | 31,590.7 | 5,159.7 | 27,504.7 | 32,664.3 |

Gating every call turns each second column into the first: on band 15, 47,329
nodes come back to the **search**. That is the whole mechanism, at its
endpoints, with no fitted parameter.

**The cap sweep says the same thing and is quoted with its caveat, because
revision 1 quoted it without.** `artifacts/wp18c_cap_sweep_onseat_v1.txt`
predates the D-463 budget fix, which its own governing design flags
(`docs/experiments/wp18c_design.md:948-951`):

> **Caveat, stated because it cuts the other way and is still not enough**: the
> sweep was taken before §4d's budget fix, so every ON row overshoots its budget.

It does overshoot, and the overshoot grows with the cap — 0.96x the budget at
cap 32, 3.13x at cap 2048, that last being D-463's own MEASURED mean of 156,313
nodes against 50,000. So the raw counts are not comparable at fixed budget, and
revision 1's reading of them (*"solver nodes fell 6.6x while search nodes rose
6.7x"*) was in the wrong frame. **At fixed budget the same two rows say solver
2.03x DOWN and search 21.91x UP** (shares 0.97635 → 0.48188 and 0.02365 →
0.51812). Both corrections cut against the naive reading and **for** this
finding: the backfill is larger and the solver-node saving smaller than the raw
counts suggest.

### 3.3 What the bracket therefore demands

With `a` = µs per search node (OFF seat), `c` = µs per solver visit (ON seat),
`u` = solver visits per search node:

```
nps_OFF = 1/a ;  nps_ON = (S+V)/(S.a + V.c) = (1+u)/(a + u.c)
ratio(u) = a(1+u) / (a + u.c)      inverting:  u = a(1-R)/(R.c - a)
```

**This is an IDENTITY, not a fit, and revision 1 was wrong to present its
agreement with the record as corroboration.** `c` is the residual
`(T_on - S.a)/V`, so `a + u.c` collapses to `T_on/S` and the whole expression
collapses to `nps_ON/nps_OFF` for **any** `c`. What checking it against the
record does exclude is narrow and still worth having: the recorded ratios are
ratios of per-rep MEDIANS while these are sums over every rep, so agreement to
< 0.0006 says the reps are homogeneous and the summation has no arithmetic slip.
It says nothing whatever about `c`.

| row | a (µs/search node) | c (µs/solver visit) | u | ratio | RECORDED |
|---|---|---|---|---|---|
| CORPUS band 15 | 4.0119 | 52.181 | 17.2598 | 0.08098 | 0.0809 |
| CORPUS band 35 | 4.8650 | 114.585 | 11.6412 | 0.04594 | 0.0458 |
| TRIGGER-RICH | 6.8011 | 164.278 | 5.3307 | 0.04879 | 0.0488 |

**What the inversion actually rests on is that `c` survives a 190x fall in `u`,
and the evidence for that is the cap sweep — used for this and not for the
backfill.** Across its four rows `u` moves 0.9301 → 41.2915, a **44.4x** range
on one fixture, while the residual `c` moves 84.18 → 102.65, about ±10 %. The
`u` the bracket needs, 0.09085, is **one** order of magnitude below the sweep's
floor, not four. Separately: `c` is a per-**visit** rate, so survivor selection
by a detector changes the visits-per-*call* `K`, not `c`.

**Inverted at the registered bounds** (≥ 0.50 both corpus bands, ≥ 0.25
trigger-rich):

| row | bound | u now | u needed | **rate factor** | absolute solver-VISIT factor |
|---|---|---|---|---|---|
| CORPUS band 15 | 0.50 | 17.2598 | 0.09085 | **190.0x** | 11.3x |
| CORPUS band 35 | 0.50 | 11.6412 | 0.04640 | **250.9x** | 20.8x |
| TRIGGER-RICH | 0.25 | 5.3307 | 0.14885 | **35.8x** | 6.5x |

**THE OMITTED TERM, whose sign is known and adverse — so every factor above is a
FLOOR.** The OFF seat holds no solver, so it never evaluates the trigger:
`self.solver.is_some()` short-circuits before `solver_verdict()`
(`crates/pistol-search/src/pvs.rs:265-269`; the wiring is `None` on that seat,
`crates/pistol-engine/src/instance.rs:150-152`). The ON seat pays the predicate
— `staged_context()` and two `hot_windows()` — at **every** search node. That
cost, call it `t` µs/node, is absent from `a` and is absorbed by the residual
`c`. The honest model is `ratio = a(1+u) / ((a+t) + u.c)`, inverting to
`u = (a(1-R) - R.t)/(R.c - a)`:

| t µs/node | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| 0.00 | 190.0x | 250.9x | 35.8x |
| 0.50 | 217.0x | 279.6x | 36.7x |
| 2.00 | 378.8x | 426.1x | 39.7x |
| 4.00 | 64163.9x | 1411.2x | 44.5x |
| 8.00 | **unreachable by any detector** | **unreachable by any detector** | 58.9x |

**There is a threshold worth its own sentence, and then a reason it is not
reached.** The bound is unreachable by ANY detector — including one that gates
every single call — once `t ≥ a(1-R)/R`, which is **4.012 µs/node on band 15 and
4.865 on band 35: in both cases exactly the cost of ONE SEARCH NODE.** If
evaluating the trigger predicate cost as much as searching a node, no detector
would reach the 0.5 bound and the package would be over before a matrix.

**It does not, and the bound comes from inspection rather than a measurement,
which is the D-474 form.** Every term the ON seat pays and the OFF seat does not
is O(1), quoted:

| term | `file:line` | what it is |
|---|---|---|
| `matches!(self.policy, Staged(_))` | `pvs.rs:268` | a discriminant test |
| `self.solver.as_ref()?.1.per_call_node_cap` | `pvs.rs:592` | a field read |
| `turns_from_root()` | `pvs.rs:569-571` | `self.position.state().turn() - self.root_turn` — a subtraction |
| `staged_context()` | `position.rs:156-164` | an `Option::as_ref` and a tuple |
| two `hot_windows()` | `query.rs:127-129` → `state.rs:101-103` → `sets.rs:158-160` | `&self.sets[class.slot()]` — a slice index, *"sorted BY CONSTRUCTION, not by sorting here"* |

Not one of them walks a structure. **`ThreatState` itself is maintained on BOTH
seats** — `staged_context()` is on the ordinary staged path at `pvs.rs:304` and
`:383`, and both bench seats run `kind = "staged"` — so the ON-only cost is two
slice-length reads and a handful of scalar operations, tens of nanoseconds. At
`t = 0.03` µs the band-15 factor moves 190.0x → 191.4x; the threshold is two to
three orders of magnitude away.

**So the omitted term is real, its sign is adverse, and its magnitude is
negligible: the floors stand as stated.** The sensitivity table earns its place
by fixing the direction and by naming a threshold that a future detector — which
would add its OWN per-node test on top of this one — must stay under.

### 3.4 The two factors, and where the dispatch's "~6x" came from

A detector is a per-node predicate, so what a design sets is the **rate**: the
fraction of nodes at which the present trigger fires that still get a call. The
last column above is the absolute **solver-visit** factor at fixed budget — it
is deliberately *not* called a call-count factor, because
`calls = V/K` and `K` (visits per fired call) cancels only if it is invariant
under gating, which is precisely what a selective detector breaks: keep the hard
calls and `K` rises, keep the cheap proofs and it falls. `K` is unmeasurable at
HEAD (§3.1). **K-invariance is therefore a named assumption wherever a call
count appears in this memo, and it is never divided out silently** — the defect
revision 1 committed in the very table prosecuting a unit substitution.

Under that assumption, and stated in capped calls per search on band 15:

```
now:    solver 49,137 / search  2,847 nodes  =  23.99 capped calls at cap 2048
target: solver  4,330 / search 47,654 nodes  =   2.11 capped calls
pass rate  1 per 119 search nodes  ->  1 per 22,541      =  190.0x
call count       23.99             ->        2.11        =   11.3x
```

**The bracket demands about TWO capped solver calls per search instead of about
TWENTY-FOUR** — and because the twenty-two dropped calls hand ~45,000 nodes back
to the search, the detector's pass rate must fall 190x, not 11x and not 6x.

**INDEPENDENT CORROBORATION OF THE "TWO", BY A ROUTE THAT KNOWS NOTHING ABOUT
THE BRACKET.** `configs/bench_wp18c_solver_on.toml:9-13` derives the cap from
the deployment turn:

> at the MEASURED in-search cost of ~102 microseconds per solver visit, **two
> capped calls at 2048** are about what a 0.5 s turn can absorb

Two. The bracket inversion, from an nps ratio and knowing nothing of the 0.5 s
design point, lands on 2.11.

**The strongest defence of the dispatched "~6x", stated before it is answered.**
`0.5/0.0809 = 6.18` is a *correct, measured* statement: the ON seat's nps must
rise about 6x to clear band 15. Under that reading "~6x" is not an error at all
and only the words "call-count" glued to it are. The defence fails on its own
terms — a designer handed *cut call count ~6x* builds a detector at a pass rate
of 1/6 where 1/190 is required, and the gap between "the bracket wants a 6x nps
improvement" and "the detector must cut calls 6x" is exactly D-477's *nodes
against calls*. It also fails a second way: band 35's shortfall is
`0.5/0.0458 = 10.9x`, so even as an nps shortfall "~6x" names only the easier of
two bands the bracket requires **both** of.

**Two governing documents carried the same figure**, and quoting them needs a
word about revisions because one of them has since moved. At the revision this
memo opened on, `21e05f8`, `docs/ROADMAP.md:183-185` read:

> targeting the measured ~6x call-count cut the WP-1.8c bracket demands

and D-471 (`docs/decisions.md:1010`) states it as the package's target in the
same words. **D-471 is not edited and never will be** — the ADR log is
append-only, so D-508 supersedes it by name rather than amending it, and the
"~6x" stays legible there as what was believed. The ROADMAP sentence IS edited,
because the ROADMAP is a live plan and not a log: at the closure revision it
points here instead of restating a figure, which is D-423 applied to a number
three documents were inheriting from each other. D-503 carries the same phrase
for the same reason and is likewise left standing.

### 3.5 The same inversion at TODAY's nps — MEASURED, not estimated

Registered before it was taken in
`docs/experiments/stage3_premise_nps_registration.md`; the two committed WP-1.8c
seats re-run unchanged at `21e05f8` on `tools/bench_block.sh`, engine sha256
`e0eb1b19…` — **the digest WP-1.9b's closure recorded for its shipped landing**,
so the binary under measurement is the one that package shipped. Four legs, 5
reps each, `nodes 50000`, **0 refused** on all four
(`bench_block: done: … 0 refused`, exit 0 each).

**Criterion N — node identity — HOLDS, on every leg, and it is what licenses
reading `k` as a speed ratio at all.** All 24 corpus entries and all 20
trigger-rich entries reproduce, at HEAD, the exact `nodes` counts WP-1.8c
recorded — 0 differences of 44 — and so do the ON seat's `search_nodes` and
`solver_nodes` totals (170,815 / 2,948,225 on band 15, identical to the sums in
§3.3's table). The seats gained `safety_net_top_k` since WP-1.8c
(`e4bb5bf`), and its committed value 0 is measured inert rather than asserted so.
The two revisions search the same trees; only the clock moved.

| band | OFF nps | IQR | ON nps | IQR | ratio | bound | verdict |
|---|---|---|---|---|---|---|---|
| CORPUS 15 | 439,819 | 0.51 % | 19,319 | 0.49 % | **0.0439** | ≥ 0.50 | **ABORTS** |
| CORPUS 35 | 375,684 | 0.83 % | 8,953 | 0.68 % | **0.0238** | ≥ 0.50 | **ABORTS** |
| TRIGGER-RICH | 297,184 | 0.49 % | 6,557 | 0.36 % | **0.0221** | ≥ 0.25 | **ABORTS** |

IQR gate CLEAN on all six medians — every one under 0.9 % of its median against
the 10 % convention (D-215/D-362).

**`k`, both terms of every ratio named.** OFF seat, HEAD against WP-1.8c's
recorded medians: 439,819/250,776 = **1.754**; 375,684/206,975 = **1.815**;
297,184/147,036 = **2.021**. **ON seat: 19,319/20,278 = 0.953; 8,953/9,478 =
0.945; 6,557/7,173 = 0.914.**

**That second row is the memo's structural claim, measured.** The search got
1.75x to 2.02x faster; the gate-on seat got *slightly slower*, because its wall
is df-pn visits and a df-pn visit does not go through `pistol-eval`. The residual
`c` moved the same way — 52.18 → 54.55, 114.59 → 121.26, 164.28 → 180.65 — up,
not down. **So the bracket is HARDER at HEAD than it was at WP-1.8c**, and every
band's ratio roughly halved: 0.0809 → 0.0439, 0.0458 → 0.0238, 0.0488 → 0.0221.

**The target at today's revision, by the same code path that read WP-1.8c's rows**
(`artifacts/stage3_premise_nps_derivation_v1.txt`):

| row | bound | u now | u needed | **rate factor** | absolute solver-VISIT factor |
|---|---|---|---|---|---|
| CORPUS band 15 | 0.50 | 17.2598 | 0.04550 | **379.3x** | 21.7x |
| CORPUS band 35 | 0.50 | 11.6412 | 0.02293 | **507.7x** | 41.1x |
| TRIGGER-RICH | 0.25 | 5.3307 | 0.06040 | **88.3x** | 14.8x |

**So the operative number for the resumed package is a floor of ~379x / ~508x /
~88x, not ~6x — between 63x and 85x what was dispatched** (379.3/6.0 and
507.7/6.0). §3.3's 1.8c-era figures are kept because they are the ones the
bracket was registered against; these are the ones a detector would have to
beat.

**The registered reading applies, and it is the one this run expected:** *"the
bracket still aborts at HEAD — recorded as the WP-1.8 arc's abort re-measured
under post-WP-1.9b nps."* The other branch, a pass, would have been a STOP for
re-reading; it did not occur. **This run does not discharge D-504** — that line
discharges the nps-jump limb *"inside the detector's own bracket"* and there is
no detector — see ruling 7.

### 3.6 What P2's failure does NOT say

It does not say the detector is impossible. `u_needed` of 0.09085 is reachable
in principle: it is about two capped calls per search where there are now
twenty-four. It says the target is **a detector that passes roughly one node in
190 of those the present trigger fires at**, not one in six — a different design
brief, which admits a different field.

Revision 1 went further and asserted that option (a) *"is very unlikely to
reach it"* and that option (e) *"gains a much stronger case"*. **Both were
judgements dressed as findings and are withdrawn.** What replaces them is the
cheap, well-posed measurement they were gesturing at, which is worth more than
either assertion:

> For each candidate narrowing of the trigger, what FRACTION of the present
> trigger's firings does it keep, on the two committed bench fixtures?

One instrumented run, once a firing counter exists, ranks the entire option
field on the only axis that now matters — and it is a second, independent reason
to land D-465's counter first (ruling 2).

**One premise of the 190x, named rather than buried.** The rate factor equals the
detector's pass rate only if firings per search node is invariant under gating.
It is not obviously so: a call that **proves** returns a mate score and collapses
a subtree (`crates/pistol-search/src/pvs.rs:612-617`), so gating a proving call
also forces the search to expand a subtree the proof would have cut. The sign is
ambiguous and the magnitude is small — proofs are a few of the ~24 calls — but
the assumption is stated, not assumed.

---

## 4. P3 — the per-call cost after 1.8c's fast paths. HOLDS.

`artifacts/wp18c_leg_ladder_v1.txt`, the shipped stage (T4, `9eb1245b`, the leg
carrying the TT fix) and the one before it (T3, `654abd43`):

```
STAGE T4 corpus TOTAL cases 24 nodes 72794  us  6933609 us_per_visit 95.25
STAGE T4 probe  TOTAL cases 85 nodes 256292 us 23671708 us_per_visit 92.36
STAGE T3 corpus TOTAL cases 24 nodes 72794  us  6682387 us_per_visit 91.80
STAGE T3 probe  TOTAL cases 85 nodes 263640 us 22413374 us_per_visit 85.02
```

**P3 holds.** Two qualifications, both of which revision 1 got loose.

**The 25-30x is not read off this ladder.** The ladder's own T0 endpoints
(2,904.08 corpus / 2,200.84 probe) are the **timer-instrumented** run; D-461's
registered endpoints are the **timer-free, uncontended** 2,630.11 / 1,975.53 at
29.71x / 24.18x. A ratio whose two terms come from different instruments is the
D-479 class, so this memo quotes D-461's pair for the factor and the ladder only
for the shipped per-visit cost.

**"`c` is flat" is TRUE ACROSS THE CAP AND FALSE ACROSS THE BANDS**, and
revision 1 said it without the qualifier. `docs/experiments/wp18c_design.md:443-446`
establishes the first — *"The per-visit wall is FLAT across the ladder
(82.6-91.2 µs on ANCHOR), so `c` is not a number that moves with the cap"* — and
the four cap-sweep rows show `c` moving only 84.18 → 102.65 over a 44.4x range
in `u`. That is the flatness the inversion needs and it is the one that is
evidenced. Across the three BANDS the residual `c` spans 52.18 to 164.28, a
3.15x spread, and 52.18 sits **below every per-visit cost measured anywhere in
this project** (T4's 95.25 / 92.36; D-461's timer-free 88.53 / 81.70). That
spread is consistent with the residual absorbing per-firing and per-node costs a
true per-visit rate would not carry — §3.3's `t`, and a per-firing set-up
amortised over `K` visits — and **separating them needs the counter that does
not exist.** It is recorded as unresolved, not explained away.

---

## 5. P4 — the value fixture. FAILS as named; repaired here.

The dispatch names *"every win the solver proved on the anchor positions (the
loser-wins g2 t10/t12, the winner conversions, the M4 flip at t42)"*. Under the
**committed** attacker policy (`one_free_stone`, the M4 widening), that set does
not exist as described.

| position | probe v1 — v0 policy, 60 s wall, no node cap | probe v2 — M4, 60 s wall, no node cap | wp18c harness — M4 + TT fix, cap 4096 |
|---|---|---|---|
| g001-t42-p2 | `nowin` 955 | **`win` 10,726**, depth 3 | `unknown` 4,096 |
| g001-t44-p2 | `win` 86 | `win` 86 | `win` 86 |
| g001-t46-p2 | `win` 1 | `win` 1 | `win` 1 |
| g002-t10-p2 | **`win` 1,599**, depth 5 | `wall-cap` | `unknown` 4,096 |
| g002-t12-p2 | **`win` 397**, depth 4 | `wall-cap` | **`win` 3,904** |
| g002-t39-p1 | `win` 714 | `win` 714 | `win` 714 |
| g002-t41-p1 | `win` 1 | `win` 1 | `win` 1 |

Sources: `artifacts/wp18b_probe_v1_results.txt`,
`artifacts/wp18b_probe_v2_results.txt`, `artifacts/wp18c_ttlaw_anchor_v1.txt`
(post-TT-fix; its `TOTAL cases 85 nodes 256292` matches the ladder's T4 probe
row, so it is the shipped engine's answer). The red team checked all 21 cells
against those artifacts and found no error.

**Three findings.**

1. **Two named positions are proven by no configuration the engine deploys.**
   `g002-t10-p2` is a v0-policy proof M4 does not reproduce within any cap on
   the ladder; `g001-t42-p2` is an M4 proof taken with **no node cap at all** at
   10,726 visits, against deployed caps of 2048 (bench seat) and 512
   (determinism seat). WP-1.8c says it plainly
   (`docs/experiments/wp18c_design.md:450-461`): *"under the committed attacker
   policy, the search path cannot see the two loser-wins that motivated this
   WP."*

2. **The `14,673` figure has no artifact receipt, and its corroboration has none
   either.** `docs/experiments/wp18c_design.md:457`, quoted in full because
   revision 1 truncated it at the clause that answers it:

   > **MEASURED this session**, fast path, no wall cap: g002-t10-p2 wins at
   > **14,673 nodes** and g002-t12-p2 at **3,904** — 9.2x and 9.8x their v0
   > counts, **independently re-reproduced by REVIEW-design**.

   `3,904` is at `artifacts/wp18c_ttlaw_anchor_v1.txt:56`. `14,673` appears in
   no artifact: `/usr/bin/grep -rn "14673\|14,673" artifacts/ docs/ sessions/`
   returns that design line and two lines of
   `sessions/WP-1.8c/2026-08-28-WP18C-CLOSED-bracket-abort.md` (`:32`, `:132`),
   both restating it, and nothing under `artifacts/`. **And the corroboration
   did not survive either**: REVIEW-design's report is among the review reports
   D-469 records as lost with a removed worktree. This is that ADR's class,
   twice in one sentence.

3. **The recall gate as specified is weaker than it reads.** It asks that each
   listed position *"still receives a solver call"*. Every one of them is hot —
   that is why the present trigger fires there — so passing is close to free,
   while §3 says the detector must gate about 189 of every 190 firings. **A gate
   over seven hot positions cannot tell whether the surviving fraction is the
   right fraction.** Whatever the resumed package's recall instrument is, it
   needs a denominator: the proofs the ON seat actually finds during a governed
   search, not seven positions a probe once returned `win` on.

**The repair, which is why this is "fails as named" and not "fails":** state the
fixture as the five positions the shipped engine proves at a cap it deploys
(`g001-t44-p2`, `g001-t46-p2`, `g002-t12-p2` at cap ≥ 4096, `g002-t39-p1`,
`g002-t41-p1`); carry `g001-t42-p2` and `g002-t10-p2` as CALL-recall-only rows
with their status stated; and give the gate the denominator finding 3 asks for.

---

## 6. What was done, and what was deliberately not

**Done** — every §0 first action that does not depend on a premise:

- **D-504 and D-505 were already appended by WP-1.9b** (`docs/decisions.md:1076`,
  `:1078`), which is the condition §0.1 makes its instruction conditional on.
  The next free number is **D-508**.
- **`dev` green at the WP-1.9b closure HEAD** — §7.
- **The current-nps re-measurement** — §3.5, registered before it was taken in
  `docs/experiments/stage3_premise_nps_registration.md`.

**Not done — `book_v2`, and the reason is stronger than revision 1's.**
Revision 1 deferred it because its size had no grounds. The decisive reason is
that **it cannot be generated at all without a code change.** The output name is
a compile-time constant:

> `crates/pistol-cli/src/random_openings/mod.rs:17`
> ```rust
> pub const FILE_NAME: &str = "random_openings_v1.txt";
> ```

used by the binary (`crates/pistol-cli/src/bin/random-openings.rs:61-62`) and by
six test files — `git grep -n "FILE_NAME" -- crates/` returns 21 lines — and the
rendered header hard-codes the version and the name of the v1 pinning test
(`crates/pistol-cli/src/random_openings/document.rs:9-13`). So §0.2 is a work
package with its own design and its own REVIEW-impl (a version parameter, a
second pinning test, six test files that currently assume exactly one book), not
a first action discharged by running a generator with a new seed.

**AND THE NAIVE DISCHARGE DESTROYS THE RECORD — a hazard for whoever picks it
up.** `configs/random_openings_v1.toml` states the regeneration command as
`--out-dir crates/pistol-cli/tests/fixtures`, and `random-openings --help` states
*"An existing file is overwritten"*. Because the name is fixed at compile time,
running that command with a `_v2` config **overwrites `random_openings_v1.txt`**
— the book that is retired for governed use but still readable as the artifact
governing every closed SPRT verdict (D-505, D-491). CI would catch it, because
`random_openings_v1_is_what_this_build_produces` would go red — but only after
the committed bytes were clobbered in the working tree.

---

## 7. Gates at this revision

`tools/ci.sh` at `21e05f8`, run in a detached worktree (`git worktree add
--detach`, its own `target/`, the live tree untouched while it ran — WP-1.9b §8
hazard 1). The worktree is removed at closure, so the log is cited at the path
it was EXPORTED to and not the one it was written at (D-469):
`artifacts/stage3_ci_base_v1.txt`.

Read the way the repo requires — from the gate log's own lines, never a
wrapper's exit status:

- all nineteen `=== gate N/19:` lines present, `gate 1/19` through `gate 19/19`;
- final line **`ci: all gates passed`**;
- `/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"` returns **0**;
- gate 9 is `tools/determinism.sh` and closes `determinism: ok — 5 seat(s), no
  difference outside nps/time in any of them`.

**One CI run in this session is VOID and is not a failure.** The first launch
exported `CARGO_TARGET_DIR` around the whole script.
`crates/pistol-cli/tests/solver_link_check_tests.rs` builds its own scratch
cargo workspaces, and a shared target directory makes one fixture read another's
dep-info — the hazard CLAUDE.md's Environment section states in those words.
Eight of its nineteen tests failed at gate 3/19 and no gate after it adjudicated
anything. Re-launched without the export in the same worktree, where
`$ROOT/target` is already isolated by the worktree itself. The void log is kept
and exported as `artifacts/stage3_ci_VOID_targetdir_v1.txt`.

---

## 8. The rulings owed — each one an ADR line

1. **The target.** Is it restated as the rate the derivation gives — a **floor**
   of ≈ 379x (band 15), ≈ 508x (band 35), ≈ 88x (trigger-rich) at HEAD, with
   WP-1.8c's 190x / 250.9x / 35.8x kept as the figures the bracket was registered
   against? `docs/ROADMAP.md` already points here rather than restating a figure,
   and D-471 and D-503 keep theirs because the ADR log is append-only and D-508
   supersedes by name. What is owed is the ruling on WHICH figure the resumed
   package is designed against, stated once in the section that owns it.
2. **The missing counter.** D-465 named a `solver_calls` counter *"the first
   thing the next package should do"* and it still does not exist. Is it landed
   FIRST, as its own small unit with its own review? §3 does not need it — `u` is
   measured directly — but §3.4's `K`, §3.6's option ranking and §5's recall
   denominator all do.
3. **The field.** Do the dispatch's options (a)-(e) stand, or is the field
   re-opened against the corrected target? A session cannot re-open a field on
   its own authority; the field is chosen before the matrix and is the
   operator's.
4. **The fixture.** Is the value fixture restated per §5's repair, and is the
   recall gate given a denominator?
5. **`book_v2`'s grounds and its scope.** §6 shows it is a work package, not a
   first action. Does it get one, and whose SPRT does its size cover?
6. **`t`, as a constraint on the DETECTOR rather than a question about the
   incumbent.** §3.3 bounds the present trigger's per-node cost by inspection and
   finds it negligible, so no measurement is owed for the floors. What the
   threshold binds is the *detector*: whatever test it adds at every node must
   stay far below 4.012 µs/node on band 15, or it makes the bound unreachable by
   its own overhead. Is that recorded as a design constraint on the resumed
   package — a per-node budget the detector is designed against and measured
   for — rather than left to be discovered at its bench?
7. **D-504's nps-jump limb.** §3.5's run re-measures the 1.8c bracket at current
   nps with no detector. D-504 discharges the limb *"inside the detector's own
   bracket"*. Does a standalone re-measurement discharge it instead? This session
   does not claim it does. D-504's Stage-2-exit limb and D-428's quiescence
   re-test are untouched either way, and D-505 has not flipped.

---

## 9. The premise red team, and what it changed

Dispatched fresh-context against `1831dfbe8b70bd28efc92fb5bdd2097779be311a` (a
`git stash create` over `21e05f8`), with the brief of falsifying the STOP.
**Verdict: STANDS WITH CORRECTIONS.** It reproduced every number in revision 1's
derivation exactly with an independently written script and found no arithmetic
error — the defects were all in what the numbers were *said to be*.

| finding | what it was | where it landed |
|---|---|---|
| BLOCKING 1 | the "seven checks" is an algebraic identity; `c` cancels | §3.3 now says so and moves the load to the 44.4x-in-`u` constant-`c` evidence |
| BLOCKING 2 | the cap sweep is pre-D-463-fix and the frame is wrong | §3.2 quotes the caveat and gives the fixed-budget figures (2.03x / 21.91x); the backfill's weight moved to the model-free endpoints |
| BLOCKING 3 | the "absolute-count factor" is a visit factor; `K` does not cancel | column renamed; `K`-invariance named as an assumption in §3.4 |
| MAJOR 1 | `c` is a residual absorbing the ON-seat-only trigger cost `t` | §3.3 gains the sensitivity table and the `t ≥ one search node` threshold — and then BOUNDS `t` by inspection at O(1) slice access, so the floors stand and the threshold becomes a constraint on the detector (ruling 6) rather than an open measurement |
| MAJOR 2 | "`c` is flat" is stronger than the memo's own table | §4 splits flat-across-the-cap (evidenced) from flat-across-bands (false) |
| MAJOR 3 | `u_required`'s degenerate branch printed "UNREACHABLE" where the truth is "already met" | instrument fixed; the two branches now print opposite texts |
| MAJOR 4 | a quotation truncated at the clause answering it | §5 finding 2 quotes it in full, and records that the corroboration is itself lost under D-469 |
| MAJOR 5 | §7 cited a closure that did not exist | §7 now cites the gate log's own lines |
| MINOR 1-6 | denominators, two off-by-one citations, a timer-free/timer-instrumented mixup, an inflated premise count, a floor stated as a range, an unnamed invariance | all applied |

**The strongest attack that did not land**, recorded because it is the one a
later reader will try: *"the fit is circular, so the 190x is unfounded."* The
circularity is real and is now conceded in §3.3 — but the inversion never
consumes the fit. It consumes the exact identity `ratio = a(S+V)/T_on` plus one
assumption, that `T_on` stays affine in `(S, V)` as `u` falls. `c` would have to
fall to **0.180x** (band 15) or **0.107x** (band 35) of its derived value for
~6x to be right — a survivor cost of 9.42 µs/visit against the cheapest
per-visit figure measured anywhere in this project, 81.70 µs (D-461, timer-free).
Halving `c` still leaves 77.7x / 113.8x. And the one non-negligible omitted term
can only make the target harder. **190x is a floor.**
