# WP-2.2 — HANDOFF: what is settled, what is open, and the calls that are owed

**For whoever picks this up.** This document is the whole state. It is written to
be decided from, not only read: §6 states the open calls with the numbers that
settle them. Every figure here is measured and receipted unless marked
ESTIMATED.

## §1 ONE LINE

**Phase 1 produced no Elo number and three findings worth more than one would
have been: the SPRT that was going to produce it could not have concluded (power
0.0015 at the registered bounds); the corpus cannot identify the eval terms the
search already resolves; and — the one that decides what to do next — the phase
reduces to ONE integer whose two candidate estimators disagree about its
DIRECTION. Everything else — §0, §R, the R3 anchor, Phase 2's premise — closed.**

## §2 What the dispatch asked, and what closed

| item | state |
|---|---|
| §0 CI green, worktree removed | **CLOSED.** 21/21 gates at `ae211f0`, `EXIT=0`, tree frozen for the run. The worktree was already gone — derived four ways, not assumed |
| §R rulings R6-R9 as D-lines | **CLOSED.** D-622-D-625 |
| D-616 retired everywhere with receipt | **CLOSED.** `wp22_d616_sweep_receipt.md`; retired where it INSTRUCTS, left where it RECORDS |
| R3 seat-swap anchor | **CLOSED.** D-612 answered, `anchor_v6_seatswap_finding.md` |
| Phase 2 premise memo | **CLOSED.** `wp22_phase2_premise.md` |
| Phase 1 design PASS | **NOT ACHIEVED.** Five revisions, nine review rounds, no PASS |
| Phase 1 SPRT verdict | **NOT RUN, and deliberately so** — see §5 |

## §3 The two findings that outlive the package

**(a) The registered SPRT could not have concluded** (D-628). At `elo0 0` /
`elo1 10`, α=β=0.05, the test needs **3 554 pairs** in expectation; the
registration capped it at **400**. Measured with the project's own shipped
simulator on this engine pair's own pentanomial, power was **0.0015**. The
instrument (`crates/pistol-arena/examples/sprt_power.rs`) had been in the tree
since the book_v2 registration and was never run. Cross-checked against three of
this project's own recorded crossings.

**(b) The corpus cannot identify the tactical terms** (D-621, D-622, D-626,
D-629). Over all 89 805 deduped positions the mover-relative `g5` is positive in
**0**; `g4` is positive in **1 593**, every one a `mate_in` row the fit's own
`score_kind` clause removes. So the two entries are pinned for two DIFFERENT
reasons — one structural, one this phase's own filter — and Phase 2 inherits the
split: the learned family evaluates quiet structure, the search evaluates
tactics.

**(c) The phase reduces to one integer and the estimators disagree about its
sign** (D-633). After R6's pins and the two exchange-rate pins, `w1` is the only
free quiet entry (`w2 = 14 − w1`, `w3 = 60`). Measured on the fit's own split,
under **the objective the engine actually runs** — no constant term, because the
schema has none:

| `w1` | table | train MSE | val MSE |
|---|---|---|---|
| **1** | `[1, 13, 60, 300, 1500]` | **714 843** | **724 169** |
| 2 | `[2, 12, 60, 300, 1500]` — **committed** | 715 454 | 724 743 |
| 4 | `[4, 10, 60, 300, 1500]` — **the registered candidate** | 719 751 | 728 957 |

Monotone in `w1`. **The deployed objective wants 1; the tempo-fitted estimator
wants 4; the committed table is 2.** Whether "fit the tempo term and discard it"
is the right estimator is the entire content of this phase, it was settled by
argument and never by measurement, and it decides the SIGN of the change an SPRT
would be asked to detect. The registered candidate's own dry run leans
**−28.12 nelo**, agreeing with the deployed objective rather than with the fit.

**And a fourth, smaller but sharp**: the ROADMAP's registered **length-11**
codebook is not supported by this corpus — 62 370 cells, **61.9 % seen fewer
than ten times**, median 6, still climbing. Length 7 saturates at 1 990 cells
with no cell under ten.

## §4 What is landed and trustworthy

- **The candidate** `[4, 10, 60, 300, 1500]`, computed by `tools/texel/fit.py`,
  reproduced by three independent reviewers with their own extractors, window
  algorithms and exact rational arithmetic. It changes play on **29.8 %** of
  positions at the standing seat, so it is not a self-match.
- **The oracle**: `tools/texel/verify_against_engine.py` at **41 215 positions**,
  three weight tables, two of them saturating the clamp, **0 disagreements** —
  82x the sample it replaces, re-run at full scale by two reviewers.
- **The instruments** `features.py`, `extract.py`, `fit.py`, `options.py`,
  `verify_against_engine.py`, all driven by `test_texel.py` (gate 18).
- **The receipts**: `artifacts/wp22_phase1_quiet/` (22 files, `sha256sum -c`
  clean), `artifacts/wp22_phase2_premise/`, `artifacts/wp22_census_governed/`.
- **Twelve ADR lines**: D-621 through D-632.

## §5 Why the run is not launched

Measured, at the only bounds 400 openings can decide (`elo1 = 50`, the smallest
grid value reaching power 0.90):

| true effect | P(h1) | P(h0) | P(inconclusive) |
|---|---|---|---|
| **0** — the registered expectation | 0.0656 | **0.9149** | 0.0196 |
| 30 | 0.5856 | 0.2978 | 0.1167 |
| 50 | 0.9335 | 0.0454 | 0.0211 |

Under the design's own registered expectation the run returns **h0 with p =
0.91**, and h0 there licenses only *"not 50 normalized Elo better"* — a
proposition the document already states it believes. The second most likely
outcome is a false positive. **It would cost 400 of the 1 000 openings D-568
reserves, on which two other packages have standing claims.** The ledger row was
therefore withdrawn and the governed arena config removed; the reservation is
whole.

**R7's actual question** — *"does this corpus move Elo at all"*, i.e.
`elo1 = 10` — needs **8 000 pairs** (measured: power 0.9045), hence a `book_v3`
of about **8 500 openings** by `book_v2_registration.md` §4's own rule. At the
measured 2.046 s per opening that is about **4.5 hours** of machine time. It is a
package, not a paragraph: `BookVersion` has no `V3` variant.

## §6 THE OPEN CALLS

**Call 0 — the one that comes first, and it is cheap.** *Which estimator?* The
phase is one integer with six admissible values. Fitting a mover-relative tempo
term and discarding it says 4; the objective the engine minimises at run time
says 1; the committed value is 2. **A single SPRT between `w1 = 1` and `w1 = 4`
settles which estimator produces the better player, needs no new corpus work, and
is the only comparison in this package whose two arms differ by more than
noise-level reasoning.** Everything below is downstream of it.

**Call 1 — the selection is NOT settled, and the matrix should not be read as
having settled it.** The final red team enumerated the option set the selection's
own ground admits: **six** two-pin tables, all holding exactly two of the four
un-evidenced exchange rates. `[2, 12, 80]` changes 20.2 % of moves against the
registered table's 29.8 %, so it wins on the ground's own minimality reading;
`[2, 33, 39]` is the **only table this package ever produced that beats the
committed table on validation MSE** (724 710 against 724 743). The registered
`[4, 10, 60]` is neither. **Treat the candidate as one of six equally principled
options, not as derived.**

**Call 2 — what Phase 1 closes on.** Four options now, the third of which is the
final reviewer's and was missed by every earlier round:

| | buys | costs |
|---|---|---|
| **(a) Close on the findings** | D-628, D-621/622, D-633, Phase 2's premise, at zero further cost | no Elo measurement of the v0 weights |
| **(b) `book_v3` (~8 500 openings) and answer R7** | the only route to *"does this corpus move Elo at all"* at power 0.90 | a package (`BookVersion` has no `V3`), plus ~4.5 h — **and it does not resolve Call 1** |
| **(c) A screening run on `book_v1`** — 2 000 openings, **unseen by this fit** (the corpus came from v2's `13..3499`), so it spends **zero** reserved openings | `nelo_pair ± 10.8` in ~68 min; enough to say whether a `book_v3` package is worth building | D-505 retires v1 for GOVERNED use; this would be an estimate, not a verdict — the same reading under which this design already draws two v1 slices |
| (d) Spend 400 of the reservation at `elo1 = 50` | a screening test for a large effect | 400 irreplaceable openings; h0 with p ≈ 0.93, licensing a proposition already believed |

**(c) is the best value on the table and no earlier round found it.** If a
governed verdict is wanted, size it at the **whole** reservation rather than 400:
the design's own rule at 1 000 pairs returns `elo1 = 30`, power 0.9189, and h0
then requires an observed `nelo_pair ≤ +9.08` instead of `≤ +16.11`.

**Call 3 — Q-2, now asked four times and never answered.** `docs/ROADMAP.md`
places SPSA/Texel in **Stage 4** while carrying this phase in **Stage 2**, and
`configs/eval_v0_weights.toml`'s own header says *"SPSA/Texel replaces these in
Stage 4"*. **No ADR moves it.** One of the two is wrong.

**Call 4 — is `tools/texel/` trusted for Phase 2's trainer?** The last reviewer
answered it precisely: **yes for the arithmetic, no for the instruments, and the
gap between them is the risk.** Three sessions have reproduced this pipeline's
numbers at scopes it never took — a different data path, a different window
algorithm, exact rational arithmetic, an independent minimum-hitting-set, a
patched engine, a from-scratch extractor reading the axes out of the Rust source
— and **not one shipped number has ever been wrong**. `features.py` and
`extract.py` are well defended: 13 of 13 mutants across them die.

**What is not defended is the layer of checks above them.** The final sweep was
73 seeded / 22 surviving. Two of those were correctness bugs and are fixed here:
the oracle refused the registered workload because a guard was written from a
finding's sentence rather than run (**D-635**), and the candidate seat could be
pointed at the committed table with all seven gates green — a self-match.
**Three remain open and are named, not hidden**: `tempo_constraints` omits
`w3 >= w2 + 1` so `round_to_schema`'s refusal is load-bearing rather than
redundant (68 of 268 committed triples reach it); the candidate's five digits are
pinned by no mechanism in the document, the design or the matrix; and
`options.py`'s minimiser check covers three of six branches, missing the
recommended row's. Read `wp22_phase1_impl_REVIEW.md` and
`wp22_phase1_review_rounds_4_5.md` before building on this.

## §7 What a successor must not rediscover

- **Compute an instrument's POWER before registering it.** `sprt.rs` works in
  NORMALIZED Elo; `elo1 = 10` there is not 10 Elo. Pairs needed is
  `2·ln(19)/t1²` with `t1 = elo1·ln(10)/800·√2`. `sprt_power.rs` measures it.
- **Run the shipped tool and diff its output against the document** before
  registering any number. This package registered a table its own instrument did
  not compute — twice.
- **A fix discharges a finding's PROPERTY, not its sentence.** Every one of the
  five failed rounds re-created a finding one step to the left.
- `find` and `grep` are shimmed in the agent shell; use `/usr/bin/` or `git`.
- `/tmp` is a 24 GiB tmpfs that gets cleared; put run logs under `artifacts/`.

## §8 The review record

Nine rounds, all committed so none survives only in a transcript:

| gate | rounds | reports |
|---|---|---|
| OPTION MATRIX | 5 | `matrix_wp22_quiet_scale_REDTEAM{,_round2,_round3,_round4}.md` + the final |
| REVIEW-design | 3 | `wp22_phase1_design_REVIEW.md`, `_rev3_REVIEW.md` + the final |
| REVIEW-impl | 3 | `wp22_phase1_impl_REVIEW.md` + the final |

**Not one shipped number was ever wrong.** What failed, every time, was what the
documents licensed.
