# WP-2.2 Phase 1 — the review record for rounds 4 and 5

**Why this file exists.** Round 4's three reports existed in no commit on any ref;
round 5's final reviewer found that and named it as the failure mode CLAUDE.md's
Process section cites by example (*"WP-1.8c's four review reports survive only in
a transcript"*, D-469). Six reports are consolidated here rather than lost. The
MAJOR findings, the verdicts and the load-bearing measurements are reproduced;
the re-derivation ledgers are not, and every figure below was reproduced by this
session against the tree before it was written down.

**All six verdicts: FAIL.** Across nine rounds no reviewer ever found a shipped
number wrong. What failed, every time, was what the documents licensed.

## Round 4 — DECISION-RED-TEAM on matrix revision 4

**MAJOR-1** §5's ground is refuted by the authority it cites and by a missing
option. R6's own text quantifies the boundary as **the sum** (D-622: *"exceeds
the sum of everything below it"*), so the "adjacent boundary" argument selects
`Σ = 74`, not `w3 = 60`; and J moves `w1/w4` ×2.5, `w2/w4` ×2.8 and `Σ/w4` +34 %,
so *"a contrast in the shape and nothing else"* is false. A two-pin table
`[4, 10, 60, 300, 1500]` holds **both** rates.
**MAJOR-2** §7 named `measure8.py`/`measure9.py` as the instruments for §4's and
§5's tables; both abort, and one computes the withdrawn `[4, 29, 60]`.
**MAJOR-3** §5's rescale anchors: two of three are not rescales, and "ALL-ρ rises
with scale" is false of a rescale.
**MAJOR-4** §1 cited a dominance report §4 no longer contained.
**MAJOR-5** §6 scored a dead option set and reused the letter `S`.
**MAJOR-6** the revision's own condition — "every number is printed by a
committed instrument" — was false of most of the document.

## Round 4 — REVIEW-design on design revision 4

**MAJOR 1** h1 was described as *"at least 50 normalized Elo better"*; measured,
h1 is the modal outcome at a true effect of **30** (0.5856), and the h0 crossing
threshold (**+16.11** observed nelo at n = 400) was absent — so the run would
report h0 for a candidate up to ~+16 nelo *better*.
**MAJOR 2** the power figures recorded no invocation, had no receipt, and two
alpha cells held the `h0` rate from the `truth = elo1` run.
**MAJOR 3** the governed arena config still carried the arm-carrying ground for
`turn_cap`.
**MAJOR 4** §4 asserted a rounding theorem the code documents as false.
**MAJOR 5** §2's `g5` mechanism (*"cannot be recorded at a turn boundary at
all"*) is false: it happens **11** times.
**MAJOR 6** the dry-run criterion was preserved by its own defect class —
`pistol.rs` says the identity digest is *"a SECOND read of the file … not a
digest of the bytes the eval parsed"* — and the falsifying referent (a
**non-degenerate pentanomial**) was already receipted.

## Round 4 — REVIEW-impl, round 2

**44 mutants seeded, 23 survived.**
**MAJOR-1** the row equilibration in `solve()` is INERT: it drives constraint
columns from 1.0 to 1.4e-10, and the scale tolerance came from the 1e-12
threshold.
**MAJOR-2** `fit()` had no test; shipping the no-intercept CONTRAST, fitting on
the VALIDATION slice, and skipping `round_to_schema` all survived — and
hand-editing the committed candidate document to any other legal table passed all
four gates.
**MAJOR-3** the `to_move` guard had no test; deleting it restored the hazard.
**MAJOR-4** the oracle's sample rule was computed and never applied on the
suite's path.
**MAJOR-5** `options.py`'s test asserted only that the pin holds — which a
rescale also satisfies — so a rescale mutant produced the withdrawn table with
the test green.

## Round 5 — DECISION-RED-TEAM on matrix revision 5

**Eight MAJOR. The decisive one, MAJOR-1**: the ground admits **six** two-pin
options, all holding exactly two of the four exchange rates; the matrix scored
one. Measured:

| pins | table | val MSE | quiet ρ | play-change |
|---|---|---|---|---|
| `w1, w2` | `[2, 12, 80, 300, 1500]` | 743 570 | 0.3701 | **20.2 %** |
| `w1, w3` | `[2, 35, 60, 300, 1500]` | 750 301 | 0.3997 | 48.8 % |
| `w2, w3` | `[6, 12, 60, 300, 1500]` | 738 017 | 0.3615 | 41.9 % |
| **`w1, Σ`** | **`[2, 33, 39, 300, 1500]`** | **724 710** | 0.3902 | 53.6 % |
| `w2, Σ` | `[5, 12, 57, 300, 1500]` | 730 549 | 0.3717 | 33.9 % |
| `w3, Σ` (**registered**) | `[4, 10, 60, 300, 1500]` | 728 957 | 0.3743 | 29.8 % |
| — committed | `[2, 12, 60, 300, 1500]` | 724 743 | 0.3873 | — |

`[2, 12, 80]` beats the registered table on the ground's own minimality reading,
and `[2, 33, 39]` is the only table this package ever produced that beats the
committed table on validation MSE. **D-632 — written by this session — states the
rule this breaks**, and `options.py`, built to make such claims checkable, covered
one row of six.

Also: **MAJOR-2** §7's `OPTIONS.txt` digest names no file in the tree;
**MAJOR-3** §3(b)'s `35 995`/`668 947` are option A's, not the no-intercept
fit's, and are quoted inside D-627; **MAJOR-4** the selected option's
play-change is in no receipt; **MAJOR-5** the `w5>Σ` half of R6's check is
reported nowhere, and two `options.py` mutants pass gate 18; **MAJOR-6** §7
asserts a citation remedy in the same line where it is absent; **MAJOR-7**
D-627's replacement clause names `w3/w4` and selects J by name, unamended;
**MAJOR-8** *"a T-vs-committed SPRT does not confound"* is false — T moves
`w1/w4` ×2.0.

## Round 5 — REVIEW-design on design revision 5

**Ten MAJOR. The decisive one, MAJOR 10** — and it is the finding that outlives
the package (**D-633**): the phase reduces to one integer, and under the
objective the engine actually runs the fit is **monotone in `w1` and prefers
`w1 = 1`**, where the tempo-fitted estimator says 4 and the committed table is 2.
**The two estimators disagree in direction.**

**MAJOR 2**: the power that withdraws the run was measured on the **withdrawn
candidate's** pentanomial (`2,3,8,7,4`); the registered dry run gives
`3,4,13,1,3` and leans **−28.12 nelo**. On the correct shape the achieved alpha
is 0.0471, *below* nominal — so one of the two stated grounds for not launching
evaporates while the other stands.
**MAJOR 4**: the decision is derived at a take of 400 the same section calls
unfixed, and omits the 1 000-pair option the ledger holds (`elo1 = 30`, power
0.9189, h0 requiring `≤ +9.08` instead of `≤ +16.11`).
**MAJOR 5**: the test is one-sided, so h0 is also what a large regression
returns (0.9977 at truth −30) — and the registered dry run leans negative.
**MAJOR 6**: D-621, D-629 and `docs/ROADMAP.md` all still carry the five-window
mechanism §2 measures to be false.
**MAJOR 8**: the ledger un-spends a committed range against its own written rule
with no ADR.
**MAJOR 9**: the matrix cites a round-4 report that exists in no commit.
Plus MAJOR 1, 3, 7 as recorded in D-634 and above.

## Round 5 — REVIEW-impl, round 3

**Eight MAJOR; 73 mutants seeded, 51 killed, 22 survived.**

**MAJOR-1 — a correctness bug the fix round introduced.** The shipped oracle
**exited 1 on the registered workload**: the fold guard added for round 2's
MAJOR-4 required distinct keys, where the registered sample deliberately holds
2 369 duplicates. The test written with it pinned the inversion. Recorded as
**D-635** and fixed — the guard now compares the draw against what the predicate
passed, and the oracle returns 41 215 positions / 0 disagreements again.

**MAJOR-2** pointing the candidate seat's `weights_file` at the committed table
passed **all seven gates**, making the registered SPRT a self-match. Fixed: a
seat-pair assertion in the Rust suite, mutation-verified.

**MAJOR-3** `tempo_constraints` omits `w3 >= w2 + 1`, so `constrained_min` does
NOT solve over the whole schema and the implementer's "near-equivalent"
judgement for the `round_to_schema` skip was wrong: **68 of 268 admissible
committed triples reach the flatness refusal**, and without it `fit()` would
print a table `weights.rs` cannot load.

**MAJOR-4** the candidate's digits are pinned by nothing mechanical in three
directions — the document (four wrong tables pass every gate), the design, and
the matrix.

**MAJOR-6** two claims added at revision 5 attribute the **superseded** dry
run's pentanomial to the committed one. Re-measured: the selection rule's
answers do not move (under 0.005), so no conclusion depends on it. Fixed.

**MAJOR-7** the `1.4e-10` figure does not reproduce: it was measured on a
synthetic system written for the probe, not on the one `constrained_min` builds
(whose `max|A|` is 2.0e7, giving 4.9e-08). Deleted from both the design and
`fit.py` rather than re-measured, with the unscoped range beside it.

**MAJOR-5** (the wrong `OPTIONS.txt` digest) and **MAJOR-8** (the minimiser check
covering three of six branches) — the first was found independently by this
session and recorded as D-634(3); the second stands open.

**FOR THE DECIDER, in the reviewer's own summary**: the arithmetic is trustworthy
and has been reproduced at scopes it never took by three sessions of reviewers;
what is not trustworthy is the layer of checks above it, *"which is the worst
shape for something a second fit would inherit, because a second fit will be
judged by exactly these checks."*
