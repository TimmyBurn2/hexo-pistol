# STAGE E's TWO-ROUND STOP — the record, superseded as a disposition.

> **SUPERSEDED BY D-709 AND BY ROUND 3's RESULT — READ THIS AS A RECORD, NOT AS
> A DISPOSITION.** This document was written when the dispatch's one-fix-round
> cap fired at two failures. The operator then granted *"up to 4 rounds"*
> (D-709), Stage E reopened at round 3, and **N-1's remedy was built and run**:
> §6.5's criterion is now stated against a STONE-COUNT quotient rather than a
> random permutation, and `hex_threat_enum_v1.md` §7.4 reports it **FIRING at 12
> of 16 cells — every affordable rung at every length**.
>
> **THE OUTCOME IS THE SAME AND ITS STANDING IS NOT.** `R-A4-CLASS` is still
> unpriced, but now because a criterion that COULD fail DID, on the full corpus,
> rather than because a round cap ran out. Everything below stands as the record
> of what two rounds found; its "what a resume needs" list is what round 3
> executed.

**Which stage stopped**: E, the hex threat enum, at its confirmation gate.
**Governing revision of this record**: `0f0cb44` plus the commits it names.

## The cap, and it was registered before the work started

The dispatch: *"E2 REVIEW-design … One fix round, confirmed by behaviour. Second
FAIL = STOP E, and the matrix runs without A4-CLASS (default fork applied,
recorded)."* Both failures happened, in that order, and the fork is applied.

| gate | revision | verdict |
|---|---|---|
| REVIEW-design, fresh context | memo revision 1, stash `ecfc2d8e` | **FAIL** — 3 BLOCKING, 7 MAJOR, 8 MINOR (`hex_threat_enum_v1_REVIEW.md`) |
| the ONE fix round | memo revision 2, `54eb3ba` | 15 of 18 findings closed |
| scoped confirmation BY BEHAVIOUR, fresh context | `54eb3ba` | **FAIL** — 1 new BLOCKING, 5 new MAJOR; B-3 still lands (`hex_threat_enum_v1_CONFIRM.md`) |

Both reviewers wrote their own implementation of the tuple from the memo's prose
and enumerated exhaustively. **Neither found an arithmetic error anywhere in the
enum**, and the second says so in terms: *"No arithmetic error anywhere in the
enum."* What failed both times was what the document LICENSED.

## THE FINDING, and it is worth more than the row

**§6.5's purity criterion — registered before the run, as `docs/process.md`
demands — is passed at 1.77x by a quotient that discards everything the calculus
names.** The confirming reviewer replaced the class table with `(own stones in
the pattern, opp stones in the pattern)` and drove the SHIPPED `census.py`:

| partition | classes | `ω²` | worst of 3 matched nulls | ratio | criterion |
|---|---|---|---|---|---|
| the enum, T4, `L = 7` | 16 | 0.005031 | 0.001958 | 2.57x | MET |
| **count-only quotient** | 23 | **0.004840** | 0.002740 | **1.77x** | **MET** |

**The value-free quotient reaches 96 % of the full tuple's `ω²`.** It cannot see
openness, completion cost, or game rule 4 — the memo's own third tuple
component — and it merges both of §5.5's `t = 1` / `t = 2` witness pairs. So:

- **On this corpus, through this statistic, at this unit, threat structure beyond
  stone counting is worth 4 %.** That is the single most transferable number
  Stage E produced and it does not depend on the enum surviving.
- **The criterion could not fire rather than declining to.** `docs/process.md`:
  *"A criterion that is a property the named defect class PRESERVES … passes
  vacuously and is not a criterion; it must be one that defect could falsify."*
  Measured: the named defect preserves it.
- **And it had already reached a matrix.** `matrix_wp22_phase2_eval.md` revision
  1 stated `R-A4-CLASS`'s kill as *"MET at all sixteen cells, so not fired"* and
  ranked the row partly on it. Revision 2 withdraws that reading in §2.5 and
  drops the row from the priced field.

## The other findings that name a wrong answer, recorded and NOT repaired

The fix-round cap is spent. A third revision is the argument-instead-of-
measurement pattern D-630 named, and these are recorded so a successor does not
have to rediscover them.

| # | finding | the measurement |
|---|---|---|
| **N-2** | §5.5's merge count is computed under a different reading of `DEF-PLAN` than the one it prints | under the printed definition (*"open windows at 4 or 5 own stones"*) the merge is **32 classes / 6 099 patterns**, not the published 27 / 5 348; the same paragraph's max-`t` distribution reproduces **only** under the printed reading |
| **N-3** | §5.5's *"not something a single-axis class can hold"* is false | adding single-axis exact `t` as a seventh component removes the merge **entirely**: `k` 357 → 393, `C(k+2,3)` 7 647 059 → 10 193 765. The true statement is about the POSITION's `t`, a different object |
| **N-4** | B-3 STILL LANDS: T1's *"expressible only if 1, 2, 3 and 4 stay apart"* is false | the coarsest partition expressing both `LAW-SUPPORT` boundaries is `{≤2 \| 3–4 \| ≥5 \| dead}`, giving `k = 16` and `C(18,3) = **816**` — the imported figure, out of the rung's own cited law. What keeps the shipped clip off it is an unjustified extra split |
| **N-5** | T3's clip reasons from a bound on `t` to a clip on a count of WINDOWS | the clip merges **7 807** `t`-inconsistent patterns against the full tuple's 6 099 — 28 % more |
| **N-7** | the merge curve is tagged DERIVED and is an exhaustive measurement, produced by no named instrument | `refinement()` returns three integers and is called only at `step = 2`; the fanouts are right and are measured |
| **N-8 … N-12** | the null is matched on the code space and not on the observed population; the criterion binds one unit while the instrument prints two; residual D-706 mentions; stale prose; §7 landed while the confirmation was outstanding | recorded |

## THE ONE THING THAT WAS FIXED, because correctness is never overruled

**N-6: CI gate 19 could not see a constant `ω²`.** Four mutants of
`tools/hex_enum/report.py` survived the whole suite — `between` always zero,
`ω²` always 1.0, `η²` and `ω²` both 0.5, and the mean correction dropped from
`within` — and `total = within + between` made the law-of-total-variance check
compare a number to itself, which is exactly the empty-check defect the memo had
just deleted from its solver arm.

CLAUDE.md: *"a finding that names a way the code can produce a wrong answer is
never overruled, only fixed"*. So it is fixed, at the code:

- `Moments.terms()` computes the total **without the partition**,
  `Σx² − N·grand²`, and **raises by name** when the two disagree.
- `test_hex_enum.py` gains a unit test of the statistic itself, with a control: a
  partition whose class means differ must earn a positive between term and a
  positive `ω²`, and a partition whose class means are equal must earn **zero**.

**MEASURED, the four mutants against the fix** (`artifacts/wp22_phase2a/n6_mutants/`,
receipt `d7e33861129a51bcd492a271dec468cc54056f2e384dd0076e785124102016d7`), run
in a scratch copy of `tools/` and never in the live tree:

```
baseline (unmutated): exit 0
M1 between always zero                     exit 1  DEAD
M2 omega2 always 1.0 (the computed line)   exit 1  DEAD
M3 eta2 and omega2 both 0.5                exit 1  DEAD
M4 the mean correction dropped from within exit 1  DEAD
restored: exit 0
```

**AND THE HARNESS'S OWN FIRST RUN CAUGHT A DEFECT IN ITSELF, recorded because it
is the same class**: M2 mutated the initialiser `omega2 = 0.0` rather than the
computed line, and a later assignment overwrote it — the mutant reported ALIVE
for a reason that was the harness's, not the fix's. Re-pointed at the computed
line it dies. A mutant that does not reach the code it names is a mutant that
tested nothing (D-650's shape, in Python rather than in `mv`).

## WHAT SURVIVES STAGE E AND IS NOT WITHDRAWN

The enum's arithmetic, independently re-derived twice:

- `k = 25 / 98 / 357 / 357` at `L = 7, 9, 11, 13`, and the whole ladder at
  `L = 11` (357 / 121 / 47 / 36).
- **`k(13) = k(11)`, and over the corpus `L = 13` sees one more class and one
  more code than `L = 11` for 18 % more window traffic per stone.** The
  covering length is 11 and nothing beyond it pays.
- Reversal invariance (0 of 59 049), swap equivariance (17 fixed classes, 187
  orbits), `r4`'s redundancy as SETS, the `|W_L| = min(6, L−5)` count, the fold
  ceilings, the window-containment counts, `C(1371,3)`, the ladder-is-a-chain
  property, both `t` witnesses, and **max single-axis exact `DEF-T` = 2 over all
  177 147 lines** — every one reproduced exactly under an independent
  implementation.
- **The even-`L` refusal is closed cleanly and at the code**, reaching all seven
  entry points, and it costs the matrix nothing.
- The census itself: 45 271 quiet rows, 8 174 025 scored cells at `L = 11`,
  observation distributions and growth curves at four lengths.

## WHAT A RESUME NEEDS, and it is three things

1. **A statistic that can discriminate.** `ω²` over position-level labels at the
   window unit cannot separate the enum from stone counting. A resume needs
   either a different referent (per-position residual against a stone-count
   baseline, so the baseline is subtracted rather than compared against), or a
   different label population, or the acknowledgement that this corpus cannot
   answer the question at all.
2. **§5.5's number republished under one stated reading of `DEF-PLAN`**, with the
   denominator that reading gives, and N-3's repair priced rather than declared
   impossible.
3. **T1's and T3's clips restated as choices**, since neither is determined by
   the law it cites — or the coarsest faithful clips adopted and `k = 16` /
   `C(18,3) = 816` reported as the ENUMERATED answer with the derivation that
   makes it not an import.

## WHERE THINGS LAND

**No code goes to `phase2a-stopped` and the reason is D-701's, measured again
here.** Stage E's stopped unit is a DOCUMENT — the derivation memo — inside a
package whose code is a passing CI gate that two later documents cite by receipt.
`tools/hex_enum/` produced every number in `matrix_wp22_phase2_eval.md` §2, §3
and §10, it is gate 19, and its one defect was a correctness defect that is fixed
rather than branched. Moving it off `dev` would break the closure that cites it,
which is the exact shape D-701 recorded and D-703 made the rule. **The banner on
the memo and this document are what carry the STOP**, and both are on `dev` where
the next reader finds them (D-703).

**No process is alive.** `ps` shows no census, no build and no agent; `git
worktree list` shows the main tree alone.
