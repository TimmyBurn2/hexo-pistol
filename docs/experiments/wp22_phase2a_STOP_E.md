# STAGE E STOPS AND SPLITS — four rounds, one recurring defect, and a corrected result the stop does not take away.

**What stopped**: Stage E, the hex threat enum, at its review gate, with D-709's
four-round grant exhausted.
**Governing revision of this record**: `4fd23c2` (`dev`).

## The four rounds

| round | subject | verdict |
|---|---|---|
| 1 — REVIEW-design, fresh context | memo revision 1, stash `ecfc2d8e` | **FAIL** — 3 BLOCKING, 7 MAJOR, 8 MINOR (`hex_threat_enum_v1_REVIEW.md`) |
| 2 — scoped confirmation BY BEHAVIOUR | revision 2, `54eb3ba` | **FAIL** — 1 new BLOCKING, 5 new MAJOR; B-3 still landing (`hex_threat_enum_v1_CONFIRM.md`) |
| 3 — review | revision 3, `6238a05` | **FAIL** — 26 of 28 prior findings CLOSED, 4 new MAJOR (`hex_threat_enum_v1_ROUND3.md`) |
| 4 — review | revision 4, `c7a3ee1` | **FAIL** — 1 new BLOCKING, 4 new MAJOR (`hex_threat_enum_v1_ROUND4.md`) |

D-709: *"a fourth failure is STOP and split with no fifth round self-granted."*
It is not self-granted here.

## THE ONE DEFECT, AND IT IS ONE DEFECT FOUR TIMES

Every round found the same thing wearing a different face: **a referent that was
not matched on the property driving its score.**

| round | the referent | how it was unmatched | what it produced |
|---|---|---|---|
| 1, M-3 | none — no criterion was registered at all | — | nothing could fail |
| 2, N-1 | a random permutation over the whole code space | it is a FLOOR: clearing it says only "not noise" | a quotient of stone counts cleared it at 1.77x, so the criterion could not fire |
| 3, P-1 | the stone-count quotient, compared UNNESTED | `ω²` is class-count monotone under position-clustered labels, and its df correction removes ~1 % of the measured entitlement | the FAIL side sound, the PASS side not — so the four METs were not evidence |
| 4, NEW-1 | the nested null, permuted over the whole code space | **MEASURED: the null's join had ~4x the cells of the join it refereed** — 194 against 49 at `L = 7` T4, 5 267 against 1 323 at `L = 11` T4 | false negatives: the advantage the nested test exists to remove was moved onto the null's side |

**FOUR ROUNDS, FOUR REFERENTS, ONE PROPERTY UNCHECKED EACH TIME.** No round found
an arithmetic error: **four independent implementations of the tuple — the
author's and three reviewers' — agree with zero disagreements**, and round 4's
own full-corpus walk reproduced every census receipt cell for cell.

## THE CORRECTNESS FIX, BECAUSE A WRONG ANSWER IS NEVER OVERRULED

CLAUDE.md: *"a finding that names a way the code can produce a wrong answer is
never overruled, only fixed."* Round 4's BLOCKING names one, so it is fixed at
the code before this stop is recorded.

`tools/hex_enum/lengths.py` gains `permuted_within_counts`, which permutes the
class assignment **inside each stone-count stratum**, so the null join's cell
count and per-cell code multiplicities are exactly the real join's. **MEASURED**:
the ratio of null-join cells to real-join cells goes from **3.96 / 3.08 / 3.98 /
4.03** to **1.00 at every rung and length**. `test_census.py` pins both halves —
that the null matches the join's cell count, and that it is not the identity.

## THE CORRECTED RESULT, AND IT REVERSES §7.4c

Re-run over the whole 45 271-row quiet population at four lengths with the
matched null and **twelve** replicates. Receipt
`artifacts/wp22_phase2a/census_r5/`, `sha256sum -c` clean, digest
`7948e47a9e80ffc7c83d50079df8c14835a45d38c21bf272d96490b02d808fbe`.

**The nested test: does `count × class` explain more than `count` alone, beyond
what the same refinement by a matched value-free partition earns?**

| L | rung | increment | null mean (12) | null max | ratio, mean | ratio, max |
|---|---|---|---|---|---|---|
| 7 | T4 | +0.000642 | +0.000241 | +0.000375 | **2.67** | **1.71** |
| 9 | T4 | +0.001902 | +0.001011 | +0.001159 | **1.88** | **1.64** |
| 9 | T2 | +0.001524 | +0.000479 | +0.000569 | **3.18** | **2.68** |
| 11 | T4 | +0.002743 | +0.001724 | +0.001844 | **1.59** | **1.49** |
| 11 | T2 | +0.001576 | +0.000485 | +0.000627 | **3.25** | **2.51** |
| 13 | T2 | +0.001962 | +0.000599 | +0.000790 | **3.27** | **2.48** |

**THE ENUM ADDS AT ALL SIXTEEN CELLS, UNDER BOTH SUMMARIES**, by 1.36x to 3.50x.
So the max-versus-mean choice round 3 caught no longer decides anything, and the
`L = 7` T4 cell that carried the whole "UNDECIDED" verdict goes **0.93 → 1.71**.

**AND THE UNNESTED CRITERION IS UNCHANGED, WHICH IS THE THIRD REPLICATION OF IT.**
The `r5` run reproduces `r3`'s sixteen ratios exactly — `L = 7` T4 1.014, `L = 11`
T2 0.637, `L = 13` T4 0.834 — so the two measurements genuinely disagree and
neither is an artifact of the other's run.

## WHAT THE ROW IS, STATED PLAINLY

Round 4's reviewer answered the question this session put to it, and its answer
is adopted: §7.4c *"is not honest, but not in the usual way: it declines a good
result rather than hiding a bad one."* Three separable artifacts produced
"UNDECIDED" — the unmatched null, a `max` over an unregistered replicate count,
and "affordable" narrowed to "comfortably affordable" between §7.3 and §7.4c.
With all three removed:

- **`L = 9` T4 is AFFORDABLE** — 40.4 observations per nominal parameter, above
  the Buro `≥ 20` line this package cites — **MET** on the registered criterion
  (1.108), and **ADDS** on the corrected nested test (1.64 worst / 1.88 mean).
- **`L = 7` T4 is comfortably affordable** — 1 720 per nominal parameter — MET
  (1.014) and ADDS (1.71 / 2.67).
- **Every coarse rung still FAILS the registered criterion**, because the enum
  ALONE has fewer classes than stone counts alone; and every coarse rung ADDS on
  the nested test, because conditional on counts it carries real structure.

**SO `R-A4-CLASS` IS PRICED AND WEAK, NOT UNDECIDED.** It is a live row at
`L = 7` or `L = 9`, rung T4, and the honest characterisation of its strength is
that the structure it adds over stone counting is real and small.

## THE SPLIT (D-481), and it is round 4's own proposal

Round 4: *"The failure is not uniform: §1-§5, §6.1-§6.4 and §7.1-§7.3 are
finished. What has failed four times is one thing."*

| part | disposition |
|---|---|
| **§1 premises, §2 the definition, §3 the pattern space, §4 the index, §5 the tuple** | **SETTLED.** Four implementations agree; every quotation and line number reproduced at every round; the even-`L` refusal closed at the code across all seven entry points. |
| **§6.1-§6.4** — enumeration, code counts, observations, the ladder | **SETTLED**, with §6.4's twenty-clip table and the 816 coincidence named. |
| **§7.1-§7.3** — `k`, `THM-WINDOW`'s confirmation, the observation census | **SETTLED**, replicated three times (r3, r4, r5) with zero cell disagreements. |
| **§6.5 the criterion and §7.4 onward** | **GOES BACK AS ITS OWN PACKAGE.** Four rounds could not settle what referent this question needs, and the fourth round's own remedy needed a fifth correction. |

**WHAT THE SUCCESSOR PACKAGE OWES, and it is three things:**

1. **A referent matched on every property that drives the score**, not on one of
   them. The four failures are four properties nobody checked: existence, class
   count, class-count monotonicity under clustering, and join cell count.
2. **A registered summary rule** — mean or max over how many replicates — before
   the run, since round 3 measured that the choice changes a verdict.
3. **An instrument that is not `ω²` over position-level labels**, or an argument
   that it is fit for this question. Its effective `n` is nearer 3 487 games than
   8 million observations, which is the root of two of the four failures.

## WHERE THINGS LAND

**No code goes to `phase2a-stopped`** (D-703, and D-701's measured shape). Stage
E's stopped unit is a DOCUMENT; its code is CI gate 19, it produced every number
in `matrix_wp22_phase2_eval.md` §2, §3 and §10, and its defects were correctness
defects that are fixed rather than branched. Moving it off `dev` would break the
closure that cites it.

**The memo carries a STOPPED banner** naming what in it is settled and what is
superseded, and **§7.4b and §7.4c are superseded by this document's corrected
measurement** rather than rewritten, because the gate has no round left to review
a rewrite.

**No process is alive**; `git worktree list` shows the main tree alone.
