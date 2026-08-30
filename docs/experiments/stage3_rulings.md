# Stage-3 detector — the seven rulings, applied

**What this document is.** `docs/experiments/stage3_detector_premise_memo.md` §8
lists seven rulings the premise STOP left owed, each one an ADR line. The
overnight dispatch that resumes the package decides ruling 1 verbatim and
supplies a decision procedure for rulings 2-7. This document applies them,
names the ground for each, and is the section that OWNS the resumed package's
target. Every other document points here rather than restating it (D-423).

**Rulings 2-7 are decided by the dispatch's own procedure**, applied per ruling,
first fit wins:

> (a) the conservative default the memo/closure itself recommends;
> (b) the option preserving all registered numbers and adding no scope;
> (c) deferral to a named licensed-not-scheduled package.

**This is not a design document, so D-483 does not forbid its numbers.** Ruling 1
is a measured quantity by construction — the dispatch requires it be *re-derived
at HEAD from the artifacts, never inherited* — and every figure below is produced
post-implementation by a registered instrument in one run and cited from that
run's artifact (§9). The DESIGN that follows this document carries none of them.

---

## 0. One line

**The target is a per-search SOLVER CALL BUDGET, and re-derived at HEAD it is
about ONE capped call per search, not the ~2 the dispatch's parenthetical
carried: 1.10 (corpus band 15) / 0.47 (band 35) / 0.91 (trigger-rich)
capped-call equivalents, each a LOWER bound.** Pass-rate targets are retired;
the budget is allocated by precision ranking over trigger points.

---

## 1. Ruling 1 — THE TARGET. Architect-decided; applied verbatim.

**The dispatch's words, quoted because they are the ruling and not a gloss of
it:**

> the resumed detector designs against a PER-SEARCH SOLVER CALL BUDGET (~2 calls
> per search, RE-DERIVED at HEAD by this session from the artifacts, never
> inherited), allocated by PRECISION RANKING over trigger points — spend the
> budget where a proof is most likely; the expensive calls are the ones that
> return NoWin. Pass-rate targets are retired.

### 1.1 The re-derivation, at HEAD, by a second instrument

`tools/stage3_call_budget_derive.py` reads the same four `tools/bench_block.sh`
artifacts `tools/stage3_premise_derive.py` reads, by an independently written
parser, and states the budget in **capped-call equivalents** at the ON seat's
committed `per_call_node_cap` of 2048 (`configs/bench_wp18c_solver_on.toml`).
Artifact: `artifacts/stage3_call_budget_v1.txt`.

| band | bound | now | **BUDGET at the bound** |
|---|---|---|---|
| CORPUS band 15 | ≥ 0.50 | 23.99 capped calls | **1.10** |
| CORPUS band 35 | ≥ 0.50 | 19.47 capped calls | **0.47** |
| TRIGGER-RICH | ≥ 0.25 | 13.43 capped calls | **0.91** |

**The cross-check is external and it holds.** The same run reproduces, from the
same bytes by different code, every intermediate the committed derivation
recorded in `artifacts/stage3_premise_nps_derivation_v1.txt`: `a` = 2.2750 /
2.6586 / 3.3659 µs per search node, `c` = 54.5509 / 121.2553 / 180.6448 µs per
solver visit, `u_now` = 17.2598 / 11.6412 / 5.3307, and the rate factors
**379.3x / 507.7x / 88.3x**. A parser that attributed the wrong column — the
defect class this cross-check exists to exclude — could not land on all twelve.

### 1.2 What the re-derivation MOVED, stated because the dispatch required it

The dispatch's parenthetical says *~2 calls per search*. That figure is the
memo's §3.4 arithmetic at the **WP-1.8c-era** nps (2.11 capped calls), and it is
correct there. **At HEAD it is 1.10**, because the same WP-1.9/WP-1.9b speedups
that halved every band's ratio also halve the budget: the search got 1.75x-2.02x
faster and the solver seat did not, so the same bracket now leaves the solver
about half the share it left in August. The dispatch anticipated exactly this by
forbidding the inherited number, and this is the value the resumed package is
designed against.

### 1.3 The budget is a LOWER bound, and `K` is why

`calls = solver_nodes / cap` assumes a fired call spends the whole cap. A call
that returns earlier spends less, so the true call count is at or above every
figure in §1.1 and the true budget is at or below **1.10 / 0.47 / 0.91**. `K` —
mean visits per fired call — has no counter at HEAD, which is ruling 2's whole
subject; it is named here and never divided out silently (D-508, memo §3.4).

**Band 35's budget is below one.** At the bracket's own bound the solver may be
called on about **one search in two** on that band — the band the bracket
requires alongside band 15, and the harder of the two. A design that budgets one
call per search satisfies band 15 and trigger-rich and misses band 35, and a
design must say which it is doing.

### 1.4 What "precision ranking" binds, and what it does not

The ruling replaces a THRESHOLD with an ALLOCATION. A pass-rate detector answers
*does this trigger point deserve a call*; a ranked allocator answers *which of
the trigger points this search reached deserve the budget*. The dispatch names
the economics: **the expensive calls are the ones that return NoWin**, so the
budget is spent where a proof is most likely and a call that would return NoWin
is the call the ranking must push out.

It does NOT settle the ranking's mechanism, its state, or its cost — those are
the option matrix's business (ruling 3) and the design's. It does settle the
AXIS the matrix's rows are compared on, which D-477 makes a premise: **rows are
ranked by the fraction of the budget they spend on calls that return a PROOF**,
and every row states what provable wins its ranking pushes out of budget.

---

## 2. Ruling 2 — THE MISSING COUNTER. Branch (a).

**Decision: `solver_calls` lands FIRST, as its own small unit with its own
review, before the option matrix is authored.**

**Ground — (a), the conservative default the memo itself recommends.** The memo
gives the recommendation twice and in its own voice: D-465 called the counter
*"the first thing the next package should do"*, and §3.6 calls the ranking
measurement *"a second, independent reason to land D-465's counter first"*.
Three separate obligations in this arc consume it and none can be discharged
without it — §3.4's `K` (ruling 1.3's lower bound becomes a measurement),
§3.6's option ranking (the matrix's rows are compared on a quantity nothing
counts today), and §5's recall denominator (ruling 4).

**Scope, held to the minimum that discharges those three**: a per-search count
of solver calls fired, reported on the line protocol beside the existing
`search_nodes`/`solver_nodes`. The dispatch's §3 already requires
`ranked/budgeted/fired/proof-found` counters on the line protocol for the
detector itself; this unit lands the one that exists independently of any
detector, so the matrix can be ranked before the detector is designed.

**What flips it:** nothing in this arc. If the unit's own review finds the
counter cannot be added without changing search behaviour, that is a finding
about the engine and reopens ruling 3's field rather than this ruling.

---

## 3. Ruling 3 — THE FIELD. Branch (b), and the dispatch supplies the authority.

**Decision: the field is the dispatch's own six rows, and it is re-opened
against the corrected target by the operator rather than by a session.**

> (a) tightened calculus-class trigger; (b) pattern-level must-block/open-four
> detection; (c) bounded VCDT-only probe as pre-filter; (d) two-tier detector ->
> certifier; (e) precision-ranked budget allocator over the current trigger
> (rank all firing points, call top-budget); (f) null.

**Ground — (b), the option preserving all registered numbers and adding no
scope.** The memo's own words are that *"a session cannot re-open a field on its
own authority; the field is chosen before the matrix and is the operator's"*.
The dispatch IS the operator, and it enumerates the field. Rows (a)-(d) are the
dispatched field unchanged; (e) and (f) are the operator's additions and are what
the corrected target calls for — (e) because ruling 1 replaces a threshold with
an allocation, (f) because a field without a null row cannot record that the
target is unreachable.

**No number moves and no scope is added**: the field is a list of mechanisms,
and D-374's registered numbers are untouched by it.

**What flips it:** a DECISION-RED-TEAM finding that a row is not a mechanism or
that the six do not span the space, which adds a row before selection rather
than after.

---

## 4. Ruling 4 — THE FIXTURE AND THE RECALL GATE. Branch (a).

**Decision: the value fixture is restated exactly as the memo's §5 repair states
it, and the recall gate is redefined per ruling 1 as a RANKING gate with the
denominator §5 finding 3 asks for.**

**The fixture, as repaired** — the five positions the shipped engine proves at a
cap it deploys, and the two the record cannot support as value rows:

| position | role |
|---|---|
| `g001-t44-p2` | VALUE — winner conversion |
| `g001-t46-p2` | VALUE — winner conversion |
| `g002-t12-p2` | VALUE — loser-win, at cap ≥ 4096 |
| `g002-t39-p1` | VALUE — winner conversion |
| `g002-t41-p1` | VALUE — winner conversion |
| `g001-t42-p2` | CALL-RECALL ONLY — the M4 flip, proven at 10,726 visits with NO node cap, against deployed caps of 2048 and 512 |
| `g002-t10-p2` | CALL-RECALL ONLY — a v0-policy proof the committed `one_free_stone` policy reproduces at no cap on the ladder |

**Ground — (a).** The memo authored the repair, gave its three findings, and had
all 21 of its cells checked against the artifacts by the premise red team.
Branch (a) is a first fit and (b)/(c) are not reached.

**The gate, redefined.** The dispatch replaces *"still receives a solver call"*
with **"the positions holding real proofs must RANK INSIDE the call budget,
pinned per position"**. That is the repair §5 finding 3 demanded: a gate over
seven hot positions could not tell whether the surviving fraction was the right
fraction, and a ranking gate can, because ranking has a denominator by
construction — every trigger point the search reached in that position's own
governed search.

**The two CALL-RECALL-ONLY rows are gated on ranking, never on proving.** No
configuration the engine deploys proves them, so a gate asking for a proof there
would ship red on correct code — the D-481 defect class, pre-empted. What they
are asked is that the detector RANK them inside budget; whether the solver then
returns a proof at the deployed cap is not this gate's question.

**What flips it:** a measurement showing a listed VALUE position is not proven at
a deployed cap at HEAD, which moves that row to CALL-RECALL-ONLY and is a change
to this table rather than to the gate.

---

## 5. Ruling 5 — `book_v2`. Branch (a), with the dispatch's own scope.

**Decision: `book_v2` is its own package (the dispatch's §2), not a first action;
its size is registered with grounds covering the STANDING SPRT worst-case n.**

**Ground — (a), the closure's own recommendation.** The closure's §5 is titled
*"`book_v2` was not generated, and the reason is a finding"* and concludes it
*"is a work package with its own design and REVIEW-impl … not a first action"*.
The dispatch scheduled it as one.

**Whose SPRT the size covers, answered because the memo asked**: the standing
worst-case n across every pre-registration that may draw from it — the resumed
detector's own SPRT (this arc's §3) and the **licensed-not-scheduled** WP-1.5d
±21.5 resolution run, which D-505 already binds to `book_v2` under a new
pre-registration. Both are named; only the first is scheduled.

**The hazard the closure named is carried into the package rather than restated
here**: the output name is a compile-time constant, so the naive regeneration
overwrites `random_openings_v1.txt`. `random_openings_v1.txt` is never
overwritten, and the six test files pinning the v1 filename keep pinning v1.

**What flips it:** nothing in this arc; the ruling is spent when `book_v2` is
committed with its generation receipt, at which point D-505 is superseded by the
generation record.

---

## 6. Ruling 6 — `t`, THE DETECTOR'S OWN PER-NODE COST. Branch (a).

**Decision: the detector's per-node cost is a registered design constraint on
the resumed package — designed against and MEASURED for at its own bench —
rather than a property discovered at that bench.**

**Ground — (a).** The memo bounds the INCUMBENT trigger's `t` by inspection at
O(1) slice access and finds it negligible, so no measurement is owed for the
floors; what it says is owed is that the threshold *"binds the DETECTOR"*. This
ruling records that, which is what §8's ruling 6 asks in its own words.

**Where the number lives, and why not here.** The threshold is
`t_max = a(1-R)/R`, whose value at HEAD is in
`artifacts/stage3_premise_nps_derivation_v1.txt` and is quoted by no design
document (D-483). The DESIGN states the constraint as a mechanism claim — *every
term the detector adds at a search node is O(1) in the position's size, quoted at
`file:line`* — and the BENCH pre-registration states the bracket and the abort,
with `t_max` quoted from that artifact as the ceiling the measured per-node cost
must sit far below. A design that needed the number to be written down in it
would be the D-483 breach; a bench that did not register it would be the
discovered-at-the-bench failure this ruling exists to prevent.

**What flips it:** a detector whose per-node test is not O(1) by inspection, which
makes `t` a measurement the design owes before its bench rather than a constraint
the bench confirms.

---

## 7. Ruling 7 — D-504's nps-JUMP LIMB. Branch (b).

**Decision: the standalone re-measurement does NOT discharge it. D-504 is
discharged by the detector's OWN registered bench, exactly as that line says,
and this arc's §3 bench states that it is doing so.**

**Ground — (b), the option preserving all registered numbers and adding no
scope.** D-504's text discharges the limb *"inside the detector's own bracket"*
and gives its grounds — a standalone re-bench *"answers no actionable question"*.
Reading a standalone run as the discharge would rewrite a registered line after
the run that might benefit from it, which is the after-the-numbers move the
process forbids. The closure already declined to claim it
(*"This does NOT discharge D-504"*), so branch (b) and branch (a) agree.

**D-504's own flip clause is checked rather than assumed.** It flips *"if the
detector package is abandoned or re-scoped away from call-count reduction"*.
Ruling 1 re-scopes the target from a call-count RATIO to a call-count BUDGET —
still call-count reduction, and more literally so than the pass-rate framing it
replaces. **The clause does not fire and the limb stays where D-504 put it.**

**What the §3.5 run is, then**: corroborating evidence that the bracket aborts
harder at HEAD, quotable as that and not as a discharge. **The Stage-2-exit limb
and D-428's quiescence re-test are untouched, and D-505 has not flipped.**

**What flips it:** the detector package being abandoned, at which point D-504's
own clause fires and the nps limb reverts to owing a standalone re-bench — which
is D-504 working, not this ruling being reopened.

---

## 8. What the seven rulings do NOT settle

- **Which option wins.** Ruling 3 fixes the field; the matrix and its
  DECISION-RED-TEAM choose from it.
- **Whether the target is reachable.** Ruling 1 states it. Row (f), the null, is
  in the field precisely so "it is not" is a result the field can express.
- **Whether the roadmap flips.** D-471's clause fires on a bracket miss at a
  registered kill point or an SPRT `h0`. Neither has occurred: there is still no
  detector. The closure's §7 said so and nothing here changes it.

---

## 9. The instrument, its dry run, and its receipt

**The instrument**: `tools/stage3_call_budget_derive.py`, at the revision this
document is committed at. A change to it reopens this document's §1 exactly as an
amendment would (`docs/process.md`, instrument governing revision).

**The dry run, on input of the same kind and different identity** — the two-entry
one-rep slice sweeps `artifacts/stage3_premise_dryrun_{off,on}_v1.txt`, which are
`bench_block` artifacts of the same shape as the governed four and were taken as
the premise run's own dry run:

| criterion | referent | observed | verdict |
|---|---|---|---|
| `a` on the OFF slice | BY HAND from the two record lines: (109+83)·1000 / (50176·2) = 1.913266 | `a 1.9133 us/node` | **MET** |
| `u_now` on the ON slice | BY HAND: 97574 / 4552 = 21.435413 | `u_now 21.4354` | **MET** |
| `c`, the residual | BY HAND: (4216000 − 1.913266·4552) / 97574 = 43.11897 | `c 43.1190 us/visit` | **MET** |
| budget | BY HAND: T = 51063.0, u\* = 0.048693, V\* = 2370.9, /2048 = 1.158 | `1.16 capped calls` | **MET** |
| an unreadable input | — | `RUN VOID: cannot read …`, exit **2** | **MET** |
| an input with no record lines | — | `FAIL: … holds no \`bench_block: record entry\` line`, exit **1** | **MET** |
| a missing or zero `--cap` | — | `FAIL: --cap …`, exit **1** | **MET** |

**The defect class the by-hand referents exclude** is the one that matters for a
parser over a line format with five integer fields: **attribution** — reading
`nodes` where `search_nodes` is meant, or summing the wrong seat. Every referent
above is computed from the artifact's bytes by a route that does not run the
script, which is what `docs/process.md` asks for first.

**The banding is not independent and is not claimed to be.** WP-1.8c's registered
convention — trigger-rich is one band; a corpus entry is band 15 iff its own
`stones` annotation reads 15, every other count band 35 — is a convention, not a
computation, and both instruments adopt it. Independence is claimed for the
parsing and the arithmetic only.

**Receipt**: `artifacts/stage3_call_budget_v1.txt`, exit 0, digest recorded in
the arc's export receipt.
