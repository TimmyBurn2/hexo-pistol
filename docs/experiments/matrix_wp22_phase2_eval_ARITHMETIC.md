# `matrix_wp22_phase2_eval.md` — the arithmetic, re-derived

**Governing revision**: `0c2f519` (`dev`) — matrix revision 3.

**Why this file exists.** `docs/process.md`'s re-derivation clause is addressed
to the REVIEWER, and its reason is that *"the author is reaching for a label for
something already believed"*. It does not follow that the author should skip the
check — only that the author's own check is not the reviewer's. This is the
author's, published so the reviewer can see which cells were checked and pick
different ones.

**Instrument**: one script re-deriving every arithmetic claim in the matrix from
the census, pilot and power artifacts rather than from the matrix's own text.

- **Against revision 1: 29 checks, 28 reproduced, 1 MISMATCH — and it was real.**
- **Against revision 3, after the red team: 19 checks over every number the
  corrections touched, 0 mismatches.**

**AND THE PASS ABOVE IS WORTH LESS THAN THE RED TEAM'S FAIL, WHICH IS THE POINT
OF PUBLISHING BOTH.** This file checked revision 1's ARITHMETIC and found one
error. The DECISION-RED-TEAM checked what revision 1 and 2 CLAIMED and found four
BLOCKING and eleven MAJOR — including three numbers this file had happily
verified as arithmetic while they described the wrong thing: `531 548` (a real
median of a real run that the receipt did not contain), `31.77 %` (a real line of
a real profile that was one third of the eval), and the `1.2`-versus-`333`
density comparison (two correct numbers in two different currencies).
`docs/process.md` is right that the author and the reviewer are the same kind of
agent and the difference is not diligence.

## Revision 1's mismatch, and it was corrected then

`R-C-SPSA`'s opening cost. The matrix said `ceil_to_500(60 500) = 61 000`;
`ceil_to_500` rounds UP to a multiple of 500 and **60 500 already is one**, so
the answer is **60 500**. The three numbers it feeds — the openings, the ratio
to `book_v3` and the wall time — are corrected to **60 500**, **7.1x** and
**34.4 h**. The kill is unchanged: no committed book funds it and D-644 bars the
one book that could.

**The class is worth naming.** `ceil_to_500(P + 500)` returns `P + 500`
unchanged whenever `P` is a multiple of 500, which is exactly the shape every
round figure in this project takes — `book_v3`'s own `ceil_to_500(8000 + 500) =
8500` is the same identity, correctly. The error was in applying the function by
eye to a number that needed no rounding.

## The 28 that reproduced

| claim | re-derived | matrix |
|---|---|---|
| A4 T4 observations per nominal parameter | 1.0689 | 1.07 |
| A4 T3 | 27.01 | 27.0 |
| A4 T2 | 443.66 | 444 |
| A4 T1 | 968.9 | 969 |
| A4 T4 per observed code | 75.63 | 75.6 |
| A4 T2 per observed code | 5 332.0 | 5 332 |
| scored cells per position, L11 | 180.56 | 180.6 |
| scored cells per position, L7 | 111.14 | 111.1 |
| A2 positions per generator parameter | 3.197 | 3.2 |
| SPSA pairs from 120 000 games | 60 000 | 60 000 |
| SPSA openings against `book_v3` | 7.176x | 7.1x |
| SPSA wall at 2.046 s/opening | 34.67 h | 34.4 h (at the corrected 60 500) |
| `book_v3` wall at 2.046 s/opening | 4.831 h | 4.83 h |
| L11 T4 `ω²` as a share of the raw ceiling | 0.6225 | 62 % |
| L11 nps bracket, low | 341 617.3 | 341 617 |
| L11 nps bracket, high | 546 003.4 | 546 003 |
| R-H-EXT bracket, low | 403 390.8 | 403 391 |
| R-H-EXT bracket, high | 499 791.3 | 499 791 |
| L13 window traffic over L11 | 1.1818x | 18.2 % more |
| seed spread, K = 8 | 5.5795 % | 5.58 % |
| seed spread, K = 32 | 6.9142 % | 6.91 % |
| seed spread, K = 64 | 6.9167 % | 6.92 % |
| val MSE gain, K = 8 to K = 64 | 5.606 % | 5.6 % |
| purity ratio L11 T2 | 3.5809x | 3.58x |
| purity ratio L11 T4 | 2.7714x | 2.77x |
| purity ratio L9 T2 | 3.5885x | 3.59x |
| purity ratio L13 T3 | 2.0030x | 2.00x |

## The SPRT closed form, checked against the project's own figures rather than itself

`expected = 2·ln(19)/t1²`, `worst = ln(19)²/t1²`, `t1 = Δ · ln(10)/800 · √2`.

- Δ = 10 expected → **3 554**, which is D-628's stated figure.
- Δ = 10 worst → **5 233**, which is fishtest's `T = 1046535/Δ²` at Δ = 10 exactly
  (`1046535/100/2 = 5 233` pairs).
- `ceil_to_500(8000 + 500)` → **8 500**, which is `book_v3`'s committed size.

Three independent anchors, none of them this matrix's own.

## What this file does not check

It checks ARITHMETIC. It does not check that a number was measured on the
population it is said to describe, which is `docs/process.md`'s named defect and
is the reviewer's to catch — *"a claim CHECKED AGAINST THE WRONG POPULATION"*.
The two places that defect would live here are §1.6's power figures, which are
tilted from an instrument-seat pentanomial and applied to a play-seat question,
and §4's parameter counts, which are stated at two different population units in
two different rows. **Both are named on the matrix's own face**; neither is
discharged by this file.


## Revision 3's checks, after the red team

| claim | re-derived | matrix |
|---|---|---|
| the eval's profile share | 31.77 + 6.64 + 6.29 = **44.70** | 44.70 % |
| L8 codebook floor / ceiling | 365 760.2 / 621 921.3 | 365 760 / 621 921 |
| L11 codebook floor / ceiling | 296 958.9 / 549 732.5 | 296 959 / 549 733 |
| R-H-EXT floor / ceiling | 365 760.2 / 529 255.0 | 365 760 / 529 255 |
| SPSA openings, wall | 60 500, 34.38 h | 60 500, 34.4 h |
| `book_v3` wall | 4.831 h | 4.83 h |
| criterion ratios, T4, L = 7 / 9 / 11 / 13 | 1.0136 / 1.1077 / 1.1246 / 0.8336 | 1.014 / 1.108 / 1.125 / 0.834 |
| criterion ratios, T2, L = 7 / 9 / 11 | 0.7932 / 0.6732 / 0.6373 | 0.793 / 0.673 / 0.637 |
| T4 observations per nominal parameter | 1.0689 | 1.07 |
| the code-unit cell §2.5 disowns | 2.2691 | 2.27 |

**What this file still does not check** is unchanged and is the reviewer's:
whether a number was measured on the population it is said to describe. Revision
3 adds one place that defect could live and did not before — §4's currency
table, which asserts which unit each row's density is in.
