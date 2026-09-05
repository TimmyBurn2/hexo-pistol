# OPTION MATRIX — what to do about §B's cap calibration, revision 1.

**Written after `wp22_cap_prereg.md` STOPped at three FAILed reviews.** The
operator returned the decision with an instruction to reason it out and take the
best option. This matrix is the vehicle CLAUDE.md requires for that: options,
costs, failure modes, recommendation — attacked by a fresh-context
DECISION-RED-TEAM **before** selection.

**Every number is marked MEASURED or ESTIMATED (D-291).** The MEASURED ones come
from `artifacts/wp22_cap_dryrun_v2/`, 100 corpus positions in the corpus's own
move order at `--nodes 50000`, all three caps over the identical sample.

## §1 The four facts that decide this, three of them measured after the STOP

**F1 — THE FLOOR IS NOT THE BINDING CONSTRAINT AT ANY CAP.** D-537 asks for 28
win-proving firings on disjoint positions. Keys per position, MEASURED: **0.86 /
0.17 / 0.19** at 2048 / 8192 / 16384. Positions needed for 28, ESTIMATED by
division: **33 / 165 / 147**. With a 3x margin, wall clock ESTIMATED from the
arms' own seconds: **5.6 / 33.2 / 35.4 minutes**.

> **The calibration costs 1 h 40 m (MEASURED per-arm seconds × 500 positions) to
> choose a cap for a run that takes between six and thirty-five minutes.**

**F2 — THE CAPS ARE COMPLEMENTARY, NOT ORDERED. This was measured after the STOP
and no review saw it.** Over the identical 100 positions, the roots each cap
proves a win on (MEASURED):

| | proving roots | roots it alone finds |
|---|---|---|
| 2048 | 8 | **[7, 98]** — neither larger cap proves these |
| 8192 | 8 | — |
| 16384 | 9 | **[16, 52, 74, 91]** — 2048 proves none of these |
| **union** | **12** | |

**No cap dominates.** D-563's structural-foreclosure argument is CONFIRMED — the
large cap reaches four roots the small one cannot — and the converse is also
true, because at a large cap the 50 000-node budget is consumed by fewer, deeper
calls and the search never reaches positions the small cap fires at.

**F3 — THE QUANTITY THE REGISTERED RULE MAXIMISES IS CLUSTERING, NOT COVERAGE.**
MEASURED: cap 2048's 86 keys come from **8** roots — 10.75 keys per root —
against 2.1 at both larger caps. `w/s` therefore separates the arms 6x
(0.2522 / 0.0422 / 0.0396, MEASURED) while the ROOT rate does not separate them
at all (8 / 8 / 9). The rule reads the inflated quantity.

**F4 — THE SMALL CAP TRUNCATES MOST OF ITS OWN WORK.** MEASURED: attacker
invocations that hit the cap without proving — **71.2 % at 2048**, 48.7 % at
8192, 38.6 % at 16384. The small cap's advantage is volume, and most of that
volume is cut off mid-proof.

## §2 The options

### A — Adopt cap 2048 by ruling; do not run the calibration

**Cost**: zero. **What it buys**: the census can be registered immediately.

**Failure modes**: it takes the arm that wins on the clustering-inflated
quantity (F3) and silently accepts F2's blind spot — the four roots only a
larger cap reaches. It also inherits the wall-clock exposure: `w/s`'s denominator
was measured on a box carrying another project's arena runs.

**Kill condition**: F2. A ruling that picks one cap when no cap dominates is a
ruling made on the wrong question.

### B — Run the calibration as a CONFIRMATION with a discriminating condition

**Cost**: 1 h 40 m (MEASURED-derived). **What it buys**: a measured margin on 5x
the sample.

**Failure modes**: it spends 1 h 40 m confirming a choice between three options
where **the choice does not bind** (F1), and it confirms it on the quantity F3
says is the wrong one. The rev-4 review's bootstrap says the outcome is 2048 in
100.00 % of 20 000 replicates, so the confirmation's information content is near
zero.

**Kill condition**: F1. Confirming a decision that costs nothing to get wrong is
not worth 1 h 40 m.

### C — Redesign the calibration around a better question

**Cost**: a new registration, a new review cycle (three rounds have already cost
this session most of its length), plus the run.

**Failure modes**: the question it would ask — which cap gives the best coverage
— is **already answered by F2**, from data in hand, for free. A redesign would
re-measure at larger n what 100 positions already show qualitatively.

**Kill condition**: it buys precision on a question whose answer (no cap
dominates) is not close.

### D — RETIRE the cap calibration; register the census directly, at 2048, reporting the columns that make the cap question answerable as a by-product

**Cost**: census only. ESTIMATED **5.6 min** at n = 98 for a 3x margin; register
n = 200 for headroom, ESTIMATED **11 min**.

**What it buys**: the floor, cleared, now. The cap question stops being a
separate package and becomes three columns in the census's own report —
truncation rate, keys-per-root, and the att_visits distribution — so a successor
can see what a larger cap would have added instead of arguing about it.

**Failure modes**: single-cap coverage. It never reaches F2's four
larger-cap-only roots, and the census's own columns can only *suggest* their
existence, not find them.

**Kill condition**: if detector round 3 needs coverage rather than a count, a
single-cap census under-serves it and the shortfall is invisible in the count.

### E — RETIRE the calibration; run the census at BOTH 2048 and 16384 over one sample, reporting each arm and their union

**Cost**: ESTIMATED **27 min** at n = 200 for both arms together — still a
sixth of the calibration's own cost.

**What it buys**: everything D buys, plus F2's complementarity turned into
coverage. MEASURED at n = 100 the union is **12 roots against 8 and 9**, so the
second arm is worth roughly a third more coverage for roughly twice the minutes
of a very cheap run.

**Failure modes**: two instruments. D-537's count must be attributable, so the
**primary count is the single-instrument cap-2048 figure** and the union is
reported beside it as coverage evidence, never summed into the floor's
denominator without saying so. If a successor reads the union as the floor's
count, the floor has been met by two instruments and D-537 does not license that.

**Kill condition**: if the operator rules that D-537's count must come from
exactly one instrument and that reporting a union invites misreading, E collapses
to D.

## §3 Recommendation

**E, with D's reporting discipline and D's primary count.**

**The reasoning, in the order the facts force it.** F1 removes the calibration's
justification entirely: it is a 1 h 40 m instrument for a decision worth at most
thirty minutes of run time, and CLAUDE.md's own proportionality — *"a
pre-registration states what its governed run COSTS … so the proportion between
the document and the run is visible on the document's own face"* — is the rule
that says so. That kills B and C.

Between A, D and E: A and D both pick a single cap, and F2 says no single cap
dominates. E costs 16 minutes more than D (ESTIMATED) and recovers a third more
proving roots (MEASURED at n = 100). On a run measured in minutes that is the
cheapest coverage this package can buy.

**The primary count stays single-instrument** so D-537's denominator is
attributable, which is E's own named failure mode answered inside the option
rather than left as a risk.

**What I am NOT recommending, and why it matters**: I am not recommending that
the cap question be *settled*. F2 says it should not be — the honest answer is
that the two caps see different positions, and the census's job is to record
that rather than to choose.

## §4 What would flip the recommendation

- **F2 fails to replicate at n = 200.** The complementarity is measured on 100
  positions and 12 roots; it is the load-bearing fact and it is not large.
- **The operator rules D-537's count must be one instrument and one arm**, in
  which case E collapses to D by its own kill condition.
- **The census's truncation column comes back near zero at 2048**, which would
  say the small cap is not losing proofs to its cap after all and would weaken
  F4 and the case for a second arm.
