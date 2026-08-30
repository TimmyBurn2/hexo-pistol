# Stage-3 detector — the seven rulings, applied

**Revision 3.** Revision 1 (`2f8f836`) took a fresh-context RED TEAM —
**STANDS WITH CORRECTIONS**, 3 BLOCKING / 11 MAJOR / 14 MINOR. Revision 2
(`ba8e6b2`) took a scoped re-review and **FAILED** it: 25 of the 28 findings
closed, but 1 new BLOCKING, 2 new MAJOR and 6 new MINOR. This revision is the
third and last round the operator's grant allows (D-509) and is scoped to that
review's enumerated remedies.

**The new BLOCKING was that the headline "below one firing per search" is an
UPPER-bound claim while every count in §1.3 is a LOWER bound — true only if a
solver invocation spends its whole cap, which nothing had measured.** It is now
measured, and §1.3 is rebuilt on the measurement. The arithmetic has never been
in doubt: three independent parsers reproduce it to the last printed digit.

**What this document is.** `docs/experiments/stage3_detector_premise_memo.md` §8
lists seven rulings the premise STOP left owed, each one an ADR line. The
governing dispatch — transcribed to
`docs/experiments/stage3_overnight_dispatch.md` — decides ruling 1 verbatim and
supplies a decision procedure for rulings 2-7. This document applies them and
names the ground for each. **The ADR lines are D-509 through D-516**; this
document is where their reasoning lives and they are what carry them.

**Rulings 2-7 are decided by the dispatch's own procedure** (`§0.3` of the
transcript), applied per ruling, first fit wins:

> (a) the conservative default the memo/closure itself recommends;
> (b) the option preserving all registered numbers and adding no scope;
> (c) deferral to a named licensed-not-scheduled package.

**This is not a design document, so D-483 does not forbid its numbers.** Ruling 1
is a measured quantity by construction — the dispatch requires it be *re-derived
at HEAD from the artifacts, never inherited* — and every figure below is produced
by a registered instrument in one run and cited from that run's artifact (§9).
The DESIGN that follows this document carries none of them.

---

## 0. One line

**The target is a per-search SOLVER BUDGET. The bracket leaves the solver about
4.4 % / 2.2 % / 5.7 % of a search's nodes, and MEASURED at HEAD that is
0.82–0.85 / 0.28–0.29 / 0.58–0.60 trigger FIRINGS per search against 18.3 / 11.8
/ 9.1 today — a reduction of 21.7x / 41.1x / 15.1x.** Every band's budget is
**below one firing per search**, and that is now a measurement rather than an
assumption: the visits an invocation actually spends were counted. Pass-rate
targets are retired; the budget is allocated by precision ranking over trigger
points.

**THE FACTOR IS THE FIGURE TO CARRY, and it is the one quantity that does not
move.** The absolute counts on both sides of it depend on what an invocation
costs; their RATIO does not, and 21.7x / 41.1x / 15.1x is what the original
derivation's own *absolute solver-visit factor* column already said.

---

## 1. Ruling 1 — THE TARGET. Architect-decided; applied verbatim.

**The dispatch's words**, quoted from
`docs/experiments/stage3_overnight_dispatch.md` §0.2 because they are the ruling
and not a gloss of it:

> the resumed detector designs against a PER-SEARCH SOLVER CALL BUDGET (~2 calls
> per search, RE-DERIVED at HEAD by this session from the artifacts, never
> inherited), allocated by PRECISION RANKING over trigger points — spend the
> budget where a proof is most likely; the expensive calls are the ones that
> return NoWin. Pass-rate targets are retired.

### 1.1 THE AXIS, quoted where its unit is CONSUMED (D-477)

A budget stated in "calls" is not a quantity until three questions are answered,
and the answers are in the code, not in a preference:

**The citations name the STATEMENT and not only a line**, because a line number
is invalidated by the next commit to the file and revision 2's were invalidated
by the commit that landed the counters — the document whose title is *quoted
where its unit is consumed* stopped quoting it anywhere.

| the unit | the statement that consumes it, in `crates/pistol-search/` | what it means there |
|---|---|---|
| a **firing** | `pvs.rs` — the hot test in `solver_verdict` (`if !mover_hot && !opponent_hot { return None; }`), reached through `visit`'s `self.solver.is_some() && … solver_verdict()` gate; and `search.rs`'s `root_triggers(&mut self.position, wiring.trigger)` | the predicate a detector gates: one trigger point, admitted or not |
| an **invocation** | `pvs.rs` — `solver.solve(&state_view, cap)` and `solver.solve_defender(&state_view, cap)` | one direction of one firing. A firing makes the attacker call and then, **unless it proved a win**, the defender call — so a firing is up to TWO invocations. **MEASURED at 2.000 / 1.993 / 1.939 invocations per firing** |
| the **cap** | `pvs.rs` — `let cap = self.solver.as_ref()?.1.per_call_node_cap;` | the ceiling on ONE invocation, not on a firing |
| the **visit** | `pvs.rs` — `self.solver_nodes = self.solver_nodes.saturating_add(result.nodes)` at both call sites; read as `self.search_nodes + self.solver_nodes` in `Run::total_nodes`, which every stop check and every report reads | what the BUDGET is denominated in |

**The memo said this and called it load-bearing** (§2.2: *"A FIRING IS NOT A
CALL, and the distinction is load-bearing in §3"*), and revision 1 of this
document lost it — the D-477 defect recurring one unit further down, inside the
ruling that supersedes the memo written to prosecute it. The red team recovered
it from the data as well as from the code: of the 220 ON record rows in the two
governed artifacts, 60 are exact multiples of the cap and **every one of those
quotients is even** — 0 (ten rows, where the trigger never fired and the seat
printed no solver field at all), 18, 22 and 24, and not one odd. That is the
signature of invocations arriving in pairs, and the census has since **counted**
them: 2.000 / 1.993 / 1.939 invocations per firing, the shortfall from two being
exactly the firings whose attacker direction proved and skipped the defender.

### 1.2 What the bracket actually FIXES, and what every other figure borrows

The bracket is an nps bound, and inverting it gives `u*`, the solver visits per
search node it permits. The quantity that follows from `u*` alone is a
**FRACTION** — the share of a search's nodes the solver may take:

| band | bound | `u_now` | `u*` | rate factor | **SHARE — the bracket-intrinsic figure** |
|---|---|---|---|---|---|
| CORPUS band 15 | ≥ 0.50 | 17.2598 | 0.04550 | 379.3x | **4.352 %** |
| CORPUS band 35 | ≥ 0.50 | 11.6412 | 0.02293 | 507.7x | **2.242 %** |
| TRIGGER-RICH | ≥ 0.25 | 5.3307 | 0.06040 | 88.3x | **5.696 %** |

Everything below is that share presented in a unit, and each unit borrows
something the bracket does not supply:

- **visits per search** = share × `T`, and `T` is the **bench's** `nodes 50000`
  budget, not the bracket's. A 0.5 s deployment turn at band 15's OFF nps is
  about 220,000 nodes — 4.4x the bench's — so the visit figure does not transfer
  to the deployment turn without being re-multiplied.
- **invocations** = visits / `cap`, and `cap = 2048` is the **bench seat's**,
  whose own config says of itself *"NOT an SPRT arm and never a committed engine
  config"* (`configs/bench_wp18c_solver_on.toml:15-17`). At the determinism
  seat's committed cap of 512 the same visit budget is four times the
  invocations.
- **firings** = invocations / 2, by §1.1's structural factor.

### 1.3 The budget, MEASURED

`T_on` is the ON seat's own per-position total and `T_off` is the OFF seat's.
They differ because a solver call absorbs its whole node count at once, so the
ON seat overshoots its 50,000-node budget by more (1,808 nodes on band 15). **At
the bound the detector has gated nearly every firing, so it sits essentially at
the OFF seat and `T_off` is the nearer end.** Both are stated; the range is the
honest form.

**WHAT AN INVOCATION ACTUALLY COSTS — `K` — IS NOW COUNTED, AND IT IS NOT THE
CAP.** Revision 2 priced every invocation at the cap and had no choice: nothing
counted the visits. The counters D-510 landed do
(`artifacts/stage3_census_analysis_v1.txt`, both committed bench fixtures at
`nodes 50000`, cap 2048):

| band | **K, visits per invocation** | invocations per firing | **one firing costs** |
|---|---|---|---|
| CORPUS band 15 | **1339.1** | 2.000 | **2678.2 visits** |
| CORPUS band 35 | **1704.6** | 1.993 | **3397.3 visits** |
| TRIGGER-RICH | **1601.9** | 1.939 | **3106.1 visits** |

`K` is **below** the cap on every band, which by §1.2's own direction means the
budget affords MORE firings than pricing at the cap suggested — and it is still
below one:

| band | **BUDGET, visits** | **BUDGET, firings** | **NOW, firings/search** | **FACTOR** |
|---|---|---|---|---|
| CORPUS band 15 | 2183.6 … 2262.3 | **0.815 … 0.845** | **18.33** | **21.7x** |
| CORPUS band 35 | 937.6 … 970.5 | **0.276 … 0.286** | **11.75** | **41.1x** |
| TRIGGER-RICH | 1799.4 … 1860.5 | **0.579 … 0.599** | **9.05** | **15.1x** |

**THE FACTOR IS INVARIANT UNDER `K` AND THE COUNTS ARE NOT, WHICH IS WHY THE
FACTOR IS THE FIGURE TO CARRY.** A cheaper invocation raises the affordable
count and raises the observed count by the same ratio — the census's 18.33
firings per search is exactly 2048/1339.1 = 1.53 times the 12.00 that pricing at
the cap gave. Both moved; 21.7x did not. And 21.7x / 41.1x / 15.1x is what the
original derivation's own *absolute solver-visit factor* column already said
(21.7 / 41.1 / 14.8), reached from the other direction.

**`t = 0` throughout, and that is the FAVOURABLE assumption.** The ON seat
evaluates its trigger predicate at every search node and the OFF seat never
does, so a real detector's own per-node cost shrinks `u*` and with it every
figure above. Priced by the same run: at `t = 0.50` µs/node — a fifth of one
search node — band 15's visit budget falls by about a fifth. This is the second
reason the figures are not final, and ruling 6 is where it binds.

### 1.4 THE THREE THINGS A DESIGNER MUST TAKE FROM THIS

1. **Every band's budget is below ONE FIRING PER SEARCH, measured.** The
   detector must reach a state where the solver is consulted on roughly **five
   searches in six** (band 15: 0.83 of a firing per search), **two in seven**
   (band 35), **three in five** (trigger-rich). Stated as the reduction it is:
   **21.7x / 41.1x / 15.1x fewer firings than today.**
2. **THE ROOT'S OWN FIRING IS A LARGE SHARE OF A SMALL BUDGET, and it is now
   measured rather than bounded.** The search fires the trigger at the root
   before any deepening iteration (`crates/pistol-search/src/search.rs`, the
   `root_triggers` gate ahead of the deepening loop) and SEEDS the tree's
   counter with what it spent (`run.solver_nodes = root_solver_nodes`), so those
   visits are inside the same budget. The census counted it: the root fires in
   **3 of 12 / 5 of 12 / 12 of 20** searches, and costs a mean **446 / 1150 /
   1455 visits per search** against budgets of **2184 / 938 / 1799**. **On band
   35 the root's average cost alone exceeds the whole budget**, and on
   trigger-rich it is four fifths of it. A design that leaves the root ungated
   has spent most of the budget before its first node — on the band where the
   budget is tightest, all of it.
3. **The cap is a design variable again.** D-465 measured the cap dead as a
   lever on the RATIO. It is not dead as a lever on the COUNT the ratio affords:
   the visit budget is fixed by the bracket and a firing costs `2 × K`, with `K`
   itself bounded by the cap. Those are different claims about different
   quantities and neither refutes the other.

### 1.5 What the re-derivation MOVED, stated because the dispatch required it

The dispatch's parenthetical says *~2 calls per search*. That is the memo's §3.4
arithmetic at the **WP-1.8c-era** nps (2.11 capped calls, band 15), and it is
correct there. At HEAD the same computation gives **0.82–0.85 firings** on band 15 — the same
WP-1.9/WP-1.9b speedups that halved every band's ratio also halve the share,
because the search got 1.75x–2.02x faster and the solver seat did not.

**AND THE SECOND ROUTE THAT ONCE AGREED NOW DISAGREES, WHICH IS ITSELF A
FINDING.** D-508 grounded "the two" on two independent routes: the bracket
inversion, and a **deployment-wall** derivation that knows nothing of the
bracket — `configs/bench_wp18c_solver_on.toml:9-13` derives the cap 2048 from
the 0.5 s turn as *"two capped calls"*. WP-1.9/WP-1.9b did not touch that route:
the solver seat got *slower* (0.953 / 0.945 / 0.914). So at HEAD the bracket
route says about 1.7 invocations per search (0.85 firings at 2.000 invocations
each) and the deployment route still says ~2.

**The disagreement is favourable and it has narrowed**: with `K` measured the two
routes are within about 20 % of each other rather than a factor of two, and the
0.5 s turn still affords slightly more than the bracket permits — so the binding
constraint is the bracket and not the clock. The config's derivation is not stale for its own
purpose — it sized a cap against a wall, and that wall has not moved — and it is
not re-derived here.

### 1.6 What "precision ranking" binds, and what it does not

The ruling replaces a THRESHOLD with an ALLOCATION. A pass-rate detector answers
*does this trigger point deserve a firing*; a ranked allocator answers *which of
the trigger points this search reached deserve the budget*. The dispatch names
the economics: **the expensive calls are the ones that return NoWin**, so the
budget goes where a proof is most likely and a firing that would return NoWin is
what the ranking must push out.

It does NOT settle the ranking's mechanism, its state, or its cost — those are
the option matrix's business (ruling 3) and the design's. It does settle the
AXIS the matrix's rows are compared on, which D-477 makes a premise: **rows are
ranked by the share of the budget they spend on invocations that return a
PROOF**, and every row states what provable wins its ranking pushes out.

---

## 2. Ruling 2 — THE MISSING COUNTER. Branch (a). **D-510.**

**Decision: the solver call counters land FIRST, as their own small unit with
their own review, before the option matrix is authored.**

**Ground — (a), the conservative default the memo itself recommends.** D-465
called the counter *"the first thing the next package should do"*, and §3.6 of
the memo calls the ranking measurement *"a second, independent reason to land
D-465's counter first"*. Four obligations in this arc consume it and none can be
discharged without it: §3.4's `K` (§1.3's lower bound becomes a measurement),
§3.6's option ranking, §5's recall denominator (ruling 4), and **§1.4's root
question, which the red team's own analysis could bound but not answer**.

**Scope, held to the minimum that discharges those four**, and shaped by §1.1's
axis: `firings`, `invocations`, `proofs` and `root_nodes`, reported on the line
protocol beside the existing `search_nodes`/`solver_nodes`. `firings` and
`invocations` are separate counters because §1.1 makes them different units;
`root_nodes` because §1.4 item 2 is a question about the root's own firing that
no aggregate answers.

**What flips it:** nothing in this arc. If the unit's own review finds the
counters cannot be added without changing search behaviour, that is a finding
about the engine and reopens ruling 3's field rather than this ruling.

---

## 3. Ruling 3 — THE FIELD. The operator decided it; the procedure was not reached. **D-511.**

**Decision: the field is the dispatch's own six rows, and it is a FLOOR rather
than a closed set — the dispatch says "at least".**

> options ranked by precision economics, **at least**: (a) tightened
> calculus-class trigger; (b) pattern-level must-block/open-four detection;
> (c) bounded VCDT-only probe as pre-filter; (d) two-tier detector ->
> certifier; (e) precision-ranked budget allocator over the current trigger
> (rank all firing points, call top-budget); (f) null.
>
> — `docs/experiments/stage3_overnight_dispatch.md` §3

**The two words revision 2 dropped are the ones that matter.** A field recorded
as closed is D-500's own failure mode — WP-1.9 stopped on an incomplete field —
and D-511's flip clause already admits a row. The matrix is built over these
six and a red team may add to them before selection.

**Ground: the operator, directly — not branch (a), (b) or (c).** The memo's own
words are that *"a session cannot re-open a field on its own authority; the
field is chosen before the matrix and is the operator's"*. The dispatch IS the
operator and it enumerates the field, so the (a)/(b)/(c) procedure is not
reached here any more than it is for ruling 1. Revision 1 labelled this branch
(b) and claimed the rows added no scope; **both were wrong** — a six-row matrix
costs more to author and red-team than a four-row one, and the claim is
withdrawn.

**THE LETTERS DO NOT RECONCILE, AND THE DISCREPANCY IS RECORDED RATHER THAN
RESOLVED.** The memo's §8 ruling 3 and the closure's §7 both describe the
dispatched field as **"(a)-(e)"**; the transcribed dispatch enumerates six. This
document does not adjudicate which enumeration the earlier documents meant. What
it fixes is that **the field this arc's matrix is built over is the six rows
above**, quoted from the text that governs the arc.

**What flips it:** a DECISION-RED-TEAM finding that a row is not a mechanism or
that the six do not span the space, which adds a row before selection.

---

## 4. Ruling 4 — THE FIXTURE AND THE RECALL GATE. Branch (a). **D-512.**

**Decision: the value fixture is restated exactly as the memo's §5 repair states
it, and the recall gate is TWO gates — a per-position ranking gate, and the
governed-search denominator §5 finding 3 actually asked for.**

**The fixture, as repaired:**

| position | role |
|---|---|
| `g001-t44-p2` | VALUE — winner conversion |
| `g001-t46-p2` | VALUE — winner conversion |
| `g002-t12-p2` | VALUE — loser-win, at cap ≥ 4096 |
| `g002-t39-p1` | VALUE — winner conversion |
| `g002-t41-p1` | VALUE — winner conversion |
| `g001-t42-p2` | CALL-RECALL ONLY — the M4 flip, proven at 10,726 visits with **no node cap at all**, against deployed caps of 2048 and 512 |
| `g002-t10-p2` | CALL-RECALL ONLY — a v0-policy proof that **no cap on the M4 ladder reproduces**: under the committed `one_free_stone` policy, probe v2 at 60 s and no node cap returned `wall-cap` |

**Ground — (a).** The memo authored the repair, gave its three findings, and had
all 21 of its cells checked against the artifacts by the premise red team.

**The gate, and both halves of it.** The dispatch replaces *"still receives a
solver call"* with **"the positions holding real proofs must RANK INSIDE the
call budget, pinned per position"**. That is one gate and it is per-position.
**It is not the denominator the memo asked for**, and revision 1 claimed it was
by quoting finding 3 up to the clause that says otherwise:

> Whatever the resumed package's recall instrument is, it needs a denominator:
> **the proofs the ON seat actually finds during a governed search, not seven
> positions a probe once returned `win` on.**

So the gate is **two** gates, and the second is the memo's:

1. **The ranking gate** (the dispatch's): on each of the seven positions, the
   position's own trigger points are ranked and the proof-bearing one must fall
   inside the budget. Pinned per position.
2. **The recall denominator** (the memo's): over a governed search on the bench
   fixtures, the fraction of the proofs the ungated ON seat FINDS that the
   detector still admits. Ruling 2's `proofs` counter is what makes this
   measurable, and it is measured at the detector's own bench.

**The two CALL-RECALL-ONLY rows are gated on ranking, never on proving.** No
configuration the engine deploys proves them, so a gate asking for a proof there
would ship red on correct code — the D-481 defect class, pre-empted.

**What flips it:** a measurement showing a listed VALUE position is not proven at
a deployed cap at HEAD, which moves that row to CALL-RECALL-ONLY.

---

## 5. Ruling 5 — `book_v2`. Branch (a), with the dispatch's own scope. **D-513.**

**Decision: `book_v2` is its own package (`docs/experiments/stage3_overnight_dispatch.md`
§2), not a first action; its size is registered with grounds covering the
standing SPRT worst-case n.**

**Ground — (a), the closure's own recommendation.** The closure's §5 is titled
*"`book_v2` was not generated, and the reason is a finding"* and concludes it
*"is a work package with its own design and REVIEW-impl … not a first action"*.

**Whose SPRT the size covers**: the resumed detector's own SPRT (scheduled) and
the **licensed-not-scheduled** WP-1.5d ±21.5 resolution run, which D-505 already
binds to `book_v2` under a new pre-registration. The grounds are measured and
registered in `docs/experiments/book_v2_registration.md`; the consumed ranges
are `docs/book_v2_ledger.md`.

**What flips it:** nothing in this arc; the ruling is spent when `book_v2` is
committed with its generation receipt.

---

## 6. Ruling 6 — THE DETECTOR'S OWN PER-NODE COST. Branch (a). **D-514.**

**Decision: the detector's per-node cost is a registered design constraint —
designed against and MEASURED for at its own bench — and the ceiling it is
registered against is derived from the BUDGET's sensitivity, not from `t_max`.**

**Ground — (a).** The memo bounds the INCUMBENT trigger's `t` by inspection at
O(1) slice access and finds it negligible; what it says is owed is that the
threshold *"binds the DETECTOR"*.

**WHY NOT `t_max`, and this is revision 1's error corrected.** `t_max = a(1−R)/R`
is where the bound goes unreachable **by a detector that gates every firing** —
2.275 µs/node on band 15 at HEAD. A detector must do more than that: it must
gate to `u*` **and still afford the budget**. §1.3's sensitivity shows band 15's
budget falling 0.55 → 0.44 firings at `t = 0.50`, a factor 4.5 *"far below"*
`t_max`. **A ceiling nothing can breach is prose that constrains nothing —
D-424's own test — while reading as though it constrains.**

**The registered ceiling is therefore a BUDGET-EROSION bound**: the detector's
own per-node test must not cost more than the `t` at which band 15's budget
falls below **90 %** of its `t = 0` value. **The instrument prints it**, so the
ceiling is a figure with a run behind it rather than one this document computed:
`t_90` = **0.2368 µs/node** on band 15, 0.2713 on band 35, 1.0643 on
trigger-rich — about a tenth of one search node on the two corpus bands
(`artifacts/stage3_call_budget_v3.txt`). The design states the constraint as a
mechanism claim — *every term the detector adds at a search node is O(1) in the
position's size, quoted at `file:line`* — carrying no number (D-483); the bench
pre-registration quotes `t_90` from the artifact.

**What flips it:** a detector whose per-node test is not O(1) by inspection,
which makes `t` a measurement the design owes before its bench.

---

## 7. Ruling 7 — D-504's nps-JUMP LIMB. Branch (a). **D-515.**

**Decision: the standalone re-measurement does NOT discharge it. D-504 is
discharged by the detector's OWN registered bench, exactly as that line says,
and this arc's bench states that it is doing so.**

**Ground — (a), the conservative default the closure itself recommends.** The
closure says in its own voice *"This does NOT discharge D-504"*. Revision 1
labelled this branch (b) and then observed that (a) agrees; under a first-fit
procedure that makes it (a).

**D-504's own words, quoted exactly**: the detector's registered bench
re-measures the solver seat under post-WP-1.9 nps *"as part of its OWN bracket,
rather than as a standalone re-bench"*, and its grounds are that a standalone
re-bench *"answers no actionable question"*. Reading a standalone run as the
discharge would rewrite a registered line after the run that might benefit from
it.

**D-504's flip clause is checked rather than assumed.** It flips *"if the
detector package is abandoned or re-scoped away from call-count reduction"*.
Ruling 1 re-scopes the target from a rate to a BUDGET — still call-count
reduction, and more literally so than the pass-rate framing it replaces. **The
clause does not fire.**

**What the §3.5 run is, then**: corroborating evidence that the bracket aborts
harder at HEAD. **The Stage-2-exit limb and D-428's quiescence re-test are
untouched, and D-505 has not flipped.**

---

## 8. What the seven rulings do NOT settle

- **Which option wins.** Ruling 3 fixes the field; the matrix and its
  DECISION-RED-TEAM choose from it.
- **Whether the target is reachable.** Row (f), the null, is in the field
  precisely so "it is not" is a result the field can express.
- **Whether the roadmap flips.** D-471's clause fires on a bracket miss at a
  registered kill point or an SPRT `h0`; neither has occurred, because there is
  still no detector. **The closure assigned the further question — whether the
  corrected target should flip the roadmap anyway — to ruling 3, and ruling 3
  does not answer it.** It is recorded here as owed and open: the field, not the
  target, is what ruling 3 settles, and only the matrix's own verdict can say
  whether the field has a reachable row. **If the matrix's DECISION-RED-TEAM
  selects (f), that IS the answer, and the flip follows.**

---

## 9. The instrument, its dry run, and its receipt

**The instrument**: `tools/stage3_call_budget_derive.py`, at the revision this
document is committed at. A change to it reopens §1 exactly as an amendment
would (`docs/process.md`, instrument governing revision).

**THE ORDER IS STATED PLAINLY, BECAUSE §9 IS NOT A PRE-REGISTRATION AND SHOULD
NOT READ AS ONE.** The instrument was authored, dry-run, run and registered in
one pass, and its first fresh-context review is
`artifacts/stage3_rulings_redteam_v1.md` — which came after. What limits the
risk is that every referent below is an arithmetic identity over committed
bytes, with no threshold that could have been moved; what does not excuse it is
the shape, which is the one D-479/D-481 clustered five findings on.

**The dry run, on input of the same kind and different identity** — the
two-entry one-rep slice sweeps `artifacts/stage3_premise_dryrun_{off,on}_v1.txt`:

| criterion | referent | verdict |
|---|---|---|
| `u_now` on the ON slice | BY HAND from the two record lines: 97574 / 4552 = 21.435413 | **MET** (`u_now 21.4354`) |
| `c`, the residual | BY HAND: (4216000 − 1.913266·4552) / 97574 = 43.11897 | **MET** (`c 43.1190`) |
| the SHARE | BY HAND: u\* = 0.048693 ⇒ 0.048693/1.048693 = 4.643 % | **MET** (`SHARE 4.643%`) |
| a leg passed under the wrong seat flag | — | **MET**, receipted — `FAIL: … is an on-seat sweep passed under --off`, exit 1 |
| an unreadable input | — | **MET**, receipted — `RUN VOID: cannot read …`, exit 2 |
| an input that is not a completed sweep | — | **MET**, receipted — `FAIL: … has no \`bench_block: done:\` line`, exit 1 |
| a missing or zero `--cap`, and an unknown option | — | **MET**, receipted — three distinct named refusals, exit 1 each |
| determinism | the same argv twice | **MET** — byte-identical |

**Two referents revision 1 claimed were discriminating are NOT, and are dropped
rather than defended.** `T` computed from `search+solver` and from `nodes` are
the same number by construction (`pvs.rs:140-142`), and on the OFF seat `nodes`
*is* `search_nodes` because the split prints only when `solver_nodes > 0`
(`crates/pistol-cli/src/report.rs:62-68`) — so neither could falsify "read the
wrong column", which `docs/process.md` says is not a criterion. The `u_now`,
`c` and SHARE referents are discriminating and carry the row set.

**What the dry run still cannot test, said rather than left silent**: both slices
hold only `stones 15` rows, so a banding defect produces identical arithmetic.
Banding is excluded by neither instrument, and it is a registered convention
rather than a computation — WP-1.8c's own: trigger-rich is one band, a corpus
entry is band 15 iff its `stones` annotation reads 15, every other count band 35.

**The second instrument, and the word "external".** Both instruments read the
same four files, so their agreement is a SECOND-INSTRUMENT check and not the
external referent `docs/process.md` reserves the word for. It is recorded as
what it is: `tools/stage3_premise_derive.py`'s committed artifact
`artifacts/stage3_premise_nps_derivation_v1.txt` states `a`, `c`, `u_now` and
the rate factors, and this run reproduces all of them. **No agreement criterion
was registered before either ran**, so the agreement is corroboration and not a
discharge — and the red team's own independently written parser, which shares no
code with either, reproduced every figure to the last printed digit.

**Receipt**: `artifacts/stage3_call_budget_v3.txt`, exit 0. The artifact records
its own argv and the sha256 of all four inputs, so it can be tied to what it
read without re-running, **and it now carries the six refusal legs too** — a
criterion recorded MET with no artifact behind it is a claim, which is what
revision 2's own review said of three of them and what this file no longer does.
`artifacts/stage3_census_analysis_v1.txt` is where §1.3's `K` comes from.
**`artifacts/` is gitignored, so every digest is anchored in a committed
document**: `docs/experiments/overnight_export_receipt.md`.
`stage3_call_budget_v1.txt` and `_v2.txt` are revisions 1 and 2's runs and are
**SUPERSEDED**; they are listed there as such, because the review reports quote
their figures and a superseded artifact that vanishes makes a report unreadable.

---

## 10. The two review rounds, and what they changed

**Round 1** — fresh-context RED TEAM against `2f8f836`, brief: falsify it.
**STANDS WITH CORRECTIONS**, 3 BLOCKING / 11 MAJOR / 14 MINOR. It reproduced
every figure with an independently written parser and re-ran the committed
instrument to a byte-identical artifact.

| finding | what it was | where it landed |
|---|---|---|
| BLOCKING 1 | the bound's sign asserted both ways in one paragraph | §1.3 states one direction |
| BLOCKING 2 | "capped call" is an INVOCATION; the ranking spends FIRINGS | §1.1 is new and quotes the consuming statements; every figure is on the firing axis |
| BLOCKING 3 | the artifact had no committed digest anchor | `docs/experiments/overnight_export_receipt.md` |
| MAJOR 1 | `T` is not invariant under gating | both ends printed as a range |
| MAJOR 2 | every figure assumes `t = 0`, unstated | §1.3 states it; the instrument prices it |
| MAJOR 3 | ruling 6's `t_max` ceiling does not bind | §6 registers a budget-erosion ceiling the instrument prints |
| MAJOR 4 | ruling 4 did not supply finding 3's denominator | §4 is two gates |
| MAJOR 5 | the governing dispatch existed at no tracked path | transcribed |
| MAJOR 6 | ruling 3's letters contradict the memo and closure | §3 records the discrepancy rather than resolving it |
| MAJOR 7 | two documents claimed to own the target | the ROADMAP points here |
| MAJOR 8 | seven decisions, zero ADR lines | D-509 through D-516 |
| MAJOR 9 | the deployment-wall route was severed in silence | §1.5 |
| MAJOR 10 | the budget is not bracket-intrinsic; the fraction is | §1.2 leads with the share |
| MAJOR 11 | instrument, registration and result in one commit | §9 says so plainly |
| MINOR 1-14 | citations, branch labels, two vacuous dry-run referents, three unreceipted criteria, an untestable banding, a missing seat guard, "external" misused, a mean read as a constant | applied, except the two round 2 re-opened below |

**Round 2** — scoped re-review against `ba8e6b2`. **FAIL**: 25 of the 28 closed,
1 new BLOCKING, 2 new MAJOR, 6 new MINOR, and two round-1 MINORs not closed
while §10 claimed all were. What this revision did with each:

| finding | what it was | what changed |
|---|---|---|
| NEW BLOCKING 1 | "below one firing" is an UPPER bound while every count is a LOWER bound — true only at `K = cap`, which nothing had measured | **`K` is now MEASURED** (1339.1 / 1704.6 / 1601.9 visits per invocation) and §1.3 is rebuilt on it. The headline survives as a measurement, and §0 and §1.3 now lead with the FACTOR, which is invariant under `K` |
| NEW MAJOR 1 | `search.rs:325` is the wrong line for the seeding claim | §1.4 item 2 cites the STATEMENT — `run.solver_nodes = root_solver_nodes` — and §1.1 does the same for every unit, because a line number is invalidated by the next commit and these were |
| NEW MAJOR 2 | the dispatch says "at least: (a)…(f)"; §3 and D-511 recorded a closed six | §3 restores the words and states the field is a FLOOR |
| MINOR (r1) 7 | three dry-run criteria recorded MET with no artifact | the artifact carries all six refusal legs |
| MINOR (r1) 14 | §10's remedy for the mean-vs-median point was a non-sequitur | **it was not fixed, and this row says so rather than claiming it was.** What replaces it is better: the census counts firings directly, so §1.3's `NOW` column is a count and not a mean of a ratio |
| MINOR | the parity parenthetical omitted quotient 0 | all four quotients stated, with what quotient 0 means |
| MINOR | §1.1 cited the node gate where the firing predicate is | the statement is quoted instead |
| MINOR | ruling 6's ceiling was printed by no artifact | the instrument prints `t_90` |
| MINOR | the ROADMAP's "stated ONCE" is falsified by D-516 | reworded: the ADR log RECORDS, this document OWNS |

**The strongest attack that did not land**, recorded because a later reader will
try it: *the call-budget step inherits the circularity the memo concedes in
§3.3*. It does not. The identity's vacuity is about **corroboration** — the check
"the model reproduces the recorded ratio" cannot fail — and not about
**inversion**, which consumes `a` and `c` as two separately measured rates and
depends on `c` essentially. The budget step adds nothing on top: it consumes
`u*`, `T` and `cap`, and neither `T` nor `cap` touches `c`.
