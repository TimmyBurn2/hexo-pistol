# Stage-3 detector — the seven rulings, applied

**Revision 2**, after a fresh-context RED TEAM returned **STANDS WITH
CORRECTIONS** on revision 1 (`2f8f836`) — 3 BLOCKING, 11 MAJOR, 14 MINOR, every
finding applied or answered in §10. It reproduced revision 1's arithmetic
exactly with an independently written parser and found no transcription error;
what it broke was what the numbers were said to be, which is where the premise
memo's own red team found every defect too.

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

**The target is a per-search SOLVER BUDGET, and it is not two calls, and it is
not one: at HEAD the bracket leaves the solver about 4.4 % / 2.2 % / 5.7 % of a
search's nodes — at least 0.53 / 0.23 / 0.44 trigger FIRINGS per search, against
12.0 / 9.7 / 6.7 today.** Every band's budget is **below one firing per
search**. Pass-rate targets are retired; the budget is allocated by precision
ranking over trigger points.

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

| the unit | the line where it is consumed | what it means there |
|---|---|---|
| a **firing** | `crates/pistol-search/src/pvs.rs:265-272` | the predicate a detector gates: one trigger point, admitted or not |
| an **invocation** | `pvs.rs:609` (`solve`) and `pvs.rs:630` (`solve_defender`) | one direction of one firing. A firing makes the attacker call and then, **unless it proved a win**, the defender call — so a firing is up to TWO invocations |
| the **cap** | `pvs.rs:592` — `self.solver.as_ref()?.1.per_call_node_cap` | the ceiling on ONE invocation. A firing therefore costs up to `2 × cap` |
| the **visit** | `pvs.rs:610` and `pvs.rs:631`, both `solver_nodes = …saturating_add(result.nodes)`, stopped on at `pvs.rs:140-142` and `:681` | what the BUDGET is denominated in. `total_nodes = search_nodes + solver_nodes` |

**The memo said this and called it load-bearing** (§2.2: *"A FIRING IS NOT A
CALL, and the distinction is load-bearing in §3"*), and revision 1 of this
document lost it — the D-477 defect recurring one unit further down, inside the
ruling that supersedes the memo written to prosecute it. The red team recovered
it from the data as well as from the code: of the 220 ON record rows in the two
governed artifacts, 60 are exact multiples of the cap and **every one of those
quotients is even** (18, 22, 24; not one odd), which is the signature of
invocations arriving in pairs.

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

### 1.3 The budget, on every axis, at both ends of `T`

`T_on` is the ON seat's own per-position total and `T_off` is the OFF seat's.
They differ because a solver call absorbs its whole node count at once, so the
ON seat overshoots its 50,000-node budget by more (1,808 nodes on band 15). **At
the bound the detector has gated nearly every firing, so it sits essentially at
the OFF seat and `T_off` is the nearer end.** Both are stated; the range is the
honest form.

| band | NOW (firings) | **BUDGET, visits** | **≥ invocations** | **≥ FIRINGS** |
|---|---|---|---|---|
| CORPUS band 15 | 12.00 | 2183.6 … 2262.3 | 1.07 … 1.10 | **0.53 … 0.55** |
| CORPUS band 35 | 9.73 | 937.6 … 970.5 | 0.46 … 0.47 | **0.23 … 0.24** |
| TRIGGER-RICH | 6.72 | 1799.4 … 1860.5 | 0.88 … 0.91 | **0.44 … 0.45** |

**THE DIRECTION IS ONE WAY, AND REVISION 1 STATED IT BACKWARDS.** `invocations =
visits / cap` prices an invocation at the cap; an invocation that returns
earlier costs LESS, so the same visit budget affords **MORE** invocations and
more firings, never fewer. **Every count above is a LOWER bound**, and `K` — the
mean visits per invocation, which no counter measured when this was derived — can
only raise it. What is *not* a bound is the visit column, which is exact given
the share and the total it is taken of. **Carry the visit figure; the counts are
a presentation of it.**

**`t = 0` throughout, and that is the FAVOURABLE assumption.** The ON seat
evaluates its trigger predicate at every search node and the OFF seat never
does, so a real detector's own per-node cost shrinks `u*` and with it every
figure above. Priced by the same run: at `t = 0.50` µs/node — a fifth of one
search node — band 15's budget falls from 0.55 to 0.44 firings, band 35's from
0.24 to 0.19. This is the second reason the figures are not final, and ruling 6
is where it binds.

### 1.4 THE THREE THINGS A DESIGNER MUST TAKE FROM THIS

1. **Every band's budget is below ONE FIRING PER SEARCH.** Not two calls, not
   one. The detector must reach a state where the solver is consulted on roughly
   **one search in two** (band 15), **one in four** (band 35). That is a
   different brief again from the one revision 1 wrote, and the difference is
   the firing/invocation factor of two.
2. **THE ROOT'S OWN FIRING CAN EXCEED THE WHOLE BUDGET.** The search fires the
   trigger at the root before any deepening iteration
   (`crates/pistol-search/src/search.rs:250-299`, gated by `root_triggers` at
   `:515`) and SEEDS the tree's counter with what it spent
   (`search.rs:325`) — so those visits are inside the same budget. One root
   firing that caps both directions costs `2 × 2048 = 4096` visits against a
   budget of 2184 / 938 / 1799. **A design that leaves the root ungated has
   spent the budget before the first node.** How often the root actually fires
   is measurable and was not measured when this was written; it is the first
   thing ruling 2's counter answers.
3. **The cap is a design variable again.** D-465 measured the cap dead as a
   lever on the RATIO. It is not dead as a lever on the COUNT the ratio affords:
   the visit budget is fixed by the bracket and `count = visits / cap`, so
   halving the cap doubles the affordable firings. Those are different claims
   about different quantities and neither refutes the other.

### 1.5 What the re-derivation MOVED, stated because the dispatch required it

The dispatch's parenthetical says *~2 calls per search*. That is the memo's §3.4
arithmetic at the **WP-1.8c-era** nps (2.11 capped calls, band 15), and it is
correct there. At HEAD the same computation gives **1.07–1.10 invocations, or
0.53–0.55 firings** — the same WP-1.9/WP-1.9b speedups that halved every band's
ratio also halve the share, because the search got 1.75x–2.02x faster and the
solver seat did not.

**AND THE SECOND ROUTE THAT ONCE AGREED NOW DISAGREES, WHICH IS ITSELF A
FINDING.** D-508 grounded "the two" on two independent routes: the bracket
inversion, and a **deployment-wall** derivation that knows nothing of the
bracket — `configs/bench_wp18c_solver_on.toml:9-13` derives the cap 2048 from
the 0.5 s turn as *"two capped calls"*. WP-1.9/WP-1.9b did not touch that route:
the solver seat got *slower* (0.953 / 0.945 / 0.914). So at HEAD the bracket
route says ~1.1 invocations and the deployment route still says ~2.

**The disagreement is favourable and is stated as such**: the 0.5 s turn can
afford about twice what the bracket permits, so the binding constraint is the
bracket and not the clock. The config's derivation is not stale for its own
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

**Decision: the field is the dispatch's own SIX rows, stated as six without a
claim about which are new.**

> (a) tightened calculus-class trigger; (b) pattern-level must-block/open-four
> detection; (c) bounded VCDT-only probe as pre-filter; (d) two-tier detector ->
> certifier; (e) precision-ranked budget allocator over the current trigger
> (rank all firing points, call top-budget); (f) null.
>
> — `docs/experiments/stage3_overnight_dispatch.md` §3

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
falls below **90 %** of its `t = 0` value. The bench pre-registration states that
`t` as a number quoted from the run that produced it, and the design states the
constraint as a mechanism claim — *every term the detector adds at a search node
is O(1) in the position's size, quoted at `file:line`* — carrying no number
(D-483).

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
| a leg passed under the wrong seat flag | — | **MET** — `FAIL: … is an on-seat sweep passed under --off`, exit 1 |
| an unreadable input | — | **MET** — `RUN VOID: cannot read …`, exit 2 |
| an input that is not a completed sweep | — | **MET** — `FAIL: … has no \`bench_block: done:\` line`, exit 1 |
| a missing or zero `--cap` | — | **MET** — `FAIL: --cap is required …`, exit 1 |
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

**Receipt**: `artifacts/stage3_call_budget_v2.txt`, exit 0. The artifact records
its own argv and the sha256 of all four inputs, so it can be tied to what it
read without re-running. **`artifacts/` is gitignored, so its digest is anchored
in a committed document**: `docs/experiments/overnight_export_receipt.md`, which
is this arc's continuation of the receipt the closure's §10 anchored.
`artifacts/stage3_call_budget_v1.txt` is revision 1's run and is **SUPERSEDED**;
it is listed there as such.

---

## 10. The red team, and what it changed

Dispatched fresh-context against `2f8f836`, brief: falsify it. **STANDS WITH
CORRECTIONS.** It reproduced every figure with an independently written parser
and re-ran the committed instrument to a byte-identical artifact.

| finding | what it was | where it landed |
|---|---|---|
| BLOCKING 1 | the bound's sign asserted both ways in one paragraph, and the design instruction rested on the wrong one | §1.3 states one direction; the instrument's own closing gloss is corrected; the "one search in two" contrast is rebuilt on the firing axis where it is true |
| BLOCKING 2 | "capped call" is an INVOCATION; the ranking spends FIRINGS; a firing costs up to 2 × cap | §1.1 is new and quotes the four consuming lines; every figure is stated on both axes; the headline moved from "about one call" to "below one firing" |
| BLOCKING 3 | the artifact had no committed digest anchor and §9 claimed it did | `docs/experiments/overnight_export_receipt.md` opened and anchored; the instrument now prints its own inputs' digests |
| MAJOR 1 | `T` is not invariant under gating — 1,808 nodes vanish on band 15 | both ends printed and stated as a range; the invariance claim deleted from the instrument |
| MAJOR 2 | every figure assumes `t = 0`, the FAVOURABLE assumption, unstated | §1.3 states it and the instrument prices it |
| MAJOR 3 | ruling 6's `t_max` ceiling does not bind | §6 registers a budget-erosion ceiling instead |
| MAJOR 4 | ruling 4 did not supply finding 3's denominator and truncated the clause that says so | §4 is two gates; the quotation is restored in full |
| MAJOR 5 | the governing dispatch existed at no tracked path | transcribed to `docs/experiments/stage3_overnight_dispatch.md`, provenance stated |
| MAJOR 6 | ruling 3's letters contradict the memo and closure | §3 states six rows and records the discrepancy rather than resolving it |
| MAJOR 7 | "every other document points here" was false and two documents claimed ownership | the ROADMAP now points here; §0 no longer claims to own the memo's sections |
| MAJOR 8 | seven decisions, zero ADR lines | D-509 through D-516 land with this revision |
| MAJOR 9 | the deployment-wall route was severed in silence | §1.5 states it, and states that the disagreement is favourable |
| MAJOR 10 | the budget is not bracket-intrinsic; the fraction is | §1.2 leads with the share and names what each unit borrows |
| MAJOR 11 | instrument, registration and result in one commit | §9 says so plainly |
| MINOR 1-14 | a mis-attributed D-504 quotation, two branch mislabels, an ambiguous fixture row, two vacuous dry-run referents, three unreceipted criteria, an untestable banding, a missing seat guard, "external" misused, "all twelve" overstated, a half-true attribution, a dropped closure assignment, a mean read as a constant | all applied; the seat guard and the input digests are instrument changes, and the mean-vs-median point is why §1.3 carries a range |

**The strongest attack that did not land**, recorded because a later reader will
try it: *the call-budget step inherits the circularity the memo concedes in
§3.3*. It does not. The identity's vacuity is about **corroboration** — the check
"the model reproduces the recorded ratio" cannot fail — and not about
**inversion**, which consumes `a` and `c` as two separately measured rates and
depends on `c` essentially (`∂u*/∂c ≠ 0`). The budget step adds nothing on top:
it consumes `u*`, `T` and `cap`, and neither `T` nor `cap` touches `c`.
