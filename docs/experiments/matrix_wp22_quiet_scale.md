# WP-2.2 Phase 1 — OPTION MATRIX: how the quiet terms are fixed, revision 4.

Governing revision: the commit that carries this file, which also carries
`tools/texel/options.py` — the instrument that PRINTS every table below and that
`test_texel.py` drives. Revision 3's §7 named two scratchpad harnesses that had
been left unrunnable by a signature change, one of them still computing a table
this document had withdrawn.

**Rounds 1, 2 and 3 all FAILED** (13 findings, then 6, then 3 MAJOR). Under the
operator's grant (D-631) this is a fourth revision, and its condition is that it
be clean: **every number below is printed by a committed instrument, and the
claim round 3 killed is DELETED rather than refined.**

Reports: `matrix_wp22_quiet_scale_REDTEAM.md`, `_round2.md`, `_round3.md`.

## §1 The decision

R6 (D-622, corrected by D-626) splits eval v0's five weights into TACTICAL
(`w4`, `w5`) and QUIET (`w1..w3`) and fits only the quiet ones.

1. *What sets the tactical entries.* **Closed by measurement.** On the filtered
   rows the tactical regressors are identically zero, so the objective's
   curvature in `w4` and `w5` is exactly zero and no feasible point is preferred
   to any other. They are carried verbatim from the committed table; dominance
   survives as a registered CHECK, reported per option in §4 (round 2's m-6).
2. *What sets the quiet entries.* **This is the decision.**

**Why one option is absent, said rather than left to be noticed** (round 2's
Q-2): fitting `w1..w3` on all 74 672 `eval` rows with the tactical contribution
entered as a KNOWN offset would use 65 % more data and would implement R6's
*"pinned at committed values"* literally. It is not a row because D-622 fixes
the population — *"WHAT DOES NOT CHANGE: the ruling, the predicate, the
population"* — so it would need an amendment, not a matrix row.

## §2 What the filter does — the label was wrong and the ground is better

Over all 29 401 dropped `eval` rows, by minimum hitting set over each opponent
window's empty cells: the MOVER owns a live four-or-more window in **0**, the
opponent in all 29 401, one stone kills every one of them in **15 135** and two
in **14 266**, and **the number needing three or more — which a turn of two
stones cannot answer — is ZERO**. Not one dropped row is a forced loss.
Meanwhile **5 339 mate rows PASS the filter**: 5 307 `mate_in` at distances 3, 5
and 7, and **32 `mated_in` at distances 4 and 6** (round 2's m-8).

**The sound ground is ONE-SIDEDNESS**: over all 74 672 `eval` rows `g4` is
positive in **0** and negative in 29 165, and `g5` is positive in **0** and
negative in 2 009. D-626 records this.

**And the corrected ground licenses exactly this filter and no other** — round 2
checked what this document had not: dropping rows on `g4 != 0 or g5 != 0 or
g6 != 0` keeps **the same 45 271 rows, 0 disagreement**, because the mover owns
a four-or-more window in 0 of 74 672 rows so a two-sided tie cannot occur. That
coextension is a measured fact of this corpus and not a definition.

## §3 The two findings that decide the options

### (a) The label's unit IS the committed table — verified by digest

All sixteen tranche reports carry `weights_sha256 41ef549666d787bf…`, equal to
`sha256sum configs/eval_v0_weights.toml`. **Every label was produced by a
400 000-node search whose leaf evaluator read the committed table.** The
absolute scale therefore carries no information the committed table did not
already have. **D-627 deletes D-623's consequence clause on this ground**
(round 2's M-6): "a table the corpus produced end to end" describes nothing on
this corpus, so it cannot separate the options.

### (b) The feature set is short a term worth +247, and a free scale absorbs it

On the fitted population: mean LABEL **+70.3**, mean committed STATIC EVAL
**−176.8**, offset **+247.1**; mean `g1` −9.698, `g2` −3.697, `g3` −1.884. The
v0 model has no constant term, so a no-intercept fit absorbs that only by moving
the weights: on the validation slice its bias² falls 61 665 → 35 995 while
residual variance RISES 663 078 → 668 947.

**It reaches the SHAPE too.** Without an intercept the solve is
`[1.1823, 22.5268, 9.0015]`, non-monotone; with a mover-relative intercept it is
`[4.9404, 32.8870, 66.9980]`, monotone, `c = +366.20`, no constraint binding.
**Revision 1's "the corpus wants a non-monotone table" was an artifact of the
missing intercept and stays withdrawn.**

**The tempo term cannot change a move, and that is MEASURED rather than argued**
(round 2's Q-3 removed the hedge revision 2 carried). Round 2 built a patched
engine whose `HandcraftedV0::value` takes a mover-relative constant from an
environment variable and ran it at the registered seat: constants 1, 10, 100,
366, 1000 and 5000 change **0 of 200** bestmoves under the committed table and
**0 of 120** under the candidate, and **the `info` node counts and depths are
identical in all 448 records** — the alpha-beta tree is unchanged, not merely
the answer. The committed instrument config arms no quiescence, no extension, no
reduction and no aspiration window, `crates/pistol-search` carries no futility,
razoring or null-move margin, and `MATE_THRESHOLD` 29 000 against `EVAL_MAX`
16 000 keeps a shifted eval out of the mate band.

## §4 The options

**§3(b) supersedes the no-intercept family.** Revision 3's table scored six
tables fitted WITHOUT the tempo term; §3(b) measures that such a fit absorbs a
+247-unit offset into the weights, so every one of them is contaminated in the
way that section describes. They are not re-scored here. What remains is the
family §4's model admits: the intercept model under one pin, each row a
constrained MINIMISER under its own pin rather than a rescale of a free solve.

Printed by `tools/texel/options.py`; diagnostics are validation-slice and GATE
NOTHING (D-614); play-change is 248 corpus positions at `go nodes 50000`,
drawn every 300th `eval` row so the draw spans all sixteen tranches.

| # | the pin | table | play-change | val MSE | resid var | quiet ρ | ALL ρ | **`w3/w4`** | **`Σ/w4`** |
|---|---|---|---|---|---|---|---|---|---|
| — | *committed, not fitted* | `[2, 12, 60, 300, 1500]` | — | 724 743 | 663 078 | 0.3873 | 0.2088 | **0.2000** | **0.2467** |
| **J** | **`w3 = 60`** | **`[5, 34, 60, 300, 1500]`** | **48.0 %** | 762 720 | 635 707 | 0.4015 | 0.2666 | **0.2000** | 0.3300 |
| P1 | `w1 = 2` | `[2, 35, 64, 300, 1500]` | 46.8 % | 755 424 | 640 639 | 0.4006 | 0.2575 | 0.2133 | 0.3367 |
| P2 | `w2 = 12` | `[6, 12, 82, 300, 1500]` | 42.7 % | 761 711 | 654 532 | 0.3669 | 0.2343 | 0.2733 | 0.3333 |
| S | `Σ = 74` | `[4, 32, 38, 300, 1500]` | 52.4 % | 729 303 | 640 056 | 0.3890 | 0.2455 | 0.1267 | **0.2467** |
| K | *unpinned* | `[5, 33, 67, 300, 1500]` | 45.2 % | 769 834 | 636 034 | 0.4045 | 0.2691 | 0.2233 | 0.3500 |
| — | no SPRT | — | — | — | — | — | — | — | — |

**The two right-hand columns are what the decision is about, and they are why
revision 3 failed.** The filtered rows carry **no evidence about ANY**
quiet-to-tactical exchange rate — the tactical regressors are identically zero
on all 45 271 of them — so every pin holds one rate and moves the others.
**J holds `w3/w4` at the committed 0.2000 and moves `Σ/w4` from 0.2467 to
0.3300; S does exactly the reverse.** No row holds both, and the only table that
holds every rate is the committed one.

**Option D — fitting the tactical entries under dominance constraints — is not a
row**: its gradient is measured at exactly zero.

## §5 Recommendation — J, AS A CHOICE AND NOT A DERIVATION

**Revision 3 claimed the pin was the ONLY normalisation holding the
quiet-to-tactical balance, and that the run would contrast "the shape and
nothing else". Both are DELETED.** Round 3 measured that J moves `w1/w4` by
×2.5, `w2/w4` by ×2.8 and `Σ/w4` by 34 %, and supplied the rival row S, absent
from revision 3's table, that holds the aggregate exactly. The claim was false
against a column this document itself now prints. **It is deleted rather than
refined, which is D-424's own remedy, and nothing is put in its place that
claims more.**

**What is true, and it is a limit rather than a ground**: the pin has to go
somewhere; every place it can go moves a quantity the corpus is silent about;
and no measurement here separates the candidates. The diagnostics do not —
quiet ρ spans 0.3669 to 0.4045 across the family and §5's own bullets below
measure two of the three columns to be scale statistics. Play-change does not —
every row changes 42.7 % to 52.4 % of moves, so every row is equally far from a
self-match. **The selection is underdetermined by the data, and saying so is the
finding this section carries.**

**Why J is chosen anyway, stated as a reason for a choice.** R6's split runs
between the quiet entries and the tactical ones, and `w3` is the entry on the
quiet side of that boundary. `w3/w4` is therefore the exchange rate the division
of labour is *about* — how much quiet structure a four outweighs — and holding
it at the committed value is what makes a J-vs-committed comparison a contrast
in the quiet SHAPE at the boundary the split names. **This is an argument from
R6's structure, not from the data, and the data does not support it or refute
it.**

**The strongest rival is S, and its reason is recorded so a successor knows the
choice was contested.** `Σ(quiet)/w4` is what a position's TOTAL quiet value
trades against a tactical term, which is at least as natural a reading of "the
balance" as the marginal one. S also has the lowest validation MSE of the family
and the bias closest to the committed table's. **It is not selected, and the
honest reason is that one of the two had to be and the boundary argument is the
one tied to R6's own text.**

**No second arm is pre-named.** Revision 2 named one on a diagnostic that round
2 measured to be a scale statistic; naming another on a diagnostic would be the
same move again, and naming one on a reason would need its own openings, which
`docs/book_v2_ledger.md` does not have to spare.

**On the diagnostics, stated because §4 prints them.** Quiet ρ is scale-free but
flat: 0.3873 for the committed shape at every rescale from ×0.5 to ×6.0, and
0.3669-0.4045 across this family. Residual variance and ALL-rows ρ are SCALE
statistics — rescaling the committed shape with no fit at all moves residual
variance 719 668 → 651 200 → 713 891 across ×0.05, ×1.75 and ×3.0, and ALL-ρ
rises monotonically with scale. **They support no option over another**, and
revision 2's claim that two of them are "components a scale choice cannot
manufacture" stays deleted.

## §6 Costs and failure modes

| # | cost | failure mode |
|---|---|---|
| **J** | **one SPRT**, registered and MEASURED in the design's §9 | the intercept is estimated from the same labels, so "uncontaminated" means free of the OFFSET and not free of the committed table — §3(a) reaches J as much as any row; and pinning `w3` imports one committed number, which is a choice no evidence in this corpus can make |
| A | one SPRT | ships a table whose gain is a constant the feature set cannot hold |
| B | one SPRT | the least powerful comparison in the field, for a reserved holdout slice |
| C, F, H | one SPRT | each varies the quantity §5 says the corpus is silent about, or ships a shape it is not described by |
| S | none | forgoes the only measurement that answers R7 |

## §7 Instruments, receipts, and what is ESTIMATED

- `tools/texel/features.py`, `tools/texel/extract.py`, `tools/texel/fit.py`,
  `tools/texel/options.py`, `tools/texel/verify_against_engine.py` and
  `tools/texel/test_texel.py` **at this document's own commit**. Written out
  rather than brace-expanded: the citation gate does not recognise `{a,b}.py` as
  a path, so the brace form put this document on the GOVERNING list with **zero
  citations checked** — an addition that looked like coverage and was inert.
- **`tools/texel/options.py` prints §4's table**, is driven by `test_texel.py`,
  and reports a pin whose answer the schema cannot hold as `INFEASIBLE` rather
  than crashing. Revision 3 named two harnesses for these tables that **abort**
  against the shipped `fit.py`, one of them computing a WITHDRAWN table — the
  third consecutive revision of that class, and this is what closes it. Round 2's M-1 found revision
  2 claiming this of a commit that did not exist; it is true of this one.
- **The play-change probe's exact command and draw** (round 2's M-5), which
  revision 2 recorded as an output only:
  `python3 probe_disagree.py 250 300 "go nodes 50000" <committed cfg> <option cfg>`
  — every 300th `eval` row, so the draw spans all sixteen tranches rather than
  sitting inside tranche 1 as a small stride would. **Every option's figure,
  J included, is in `artifacts/wp22_phase1_quiet/PLAYCHANGE.txt`** with the
  command above printed beside it — and so is the WITHDRAWN `[4, 29, 60]`, at
  40.7 %, because a receipt that showed only the surviving table would hide the
  correction round 2 forced.
- **Receipts**: `artifacts/wp22_phase1_quiet/RECEIPT.sha256` covers the probes,
  their outputs, the oracle run and the dry run. §3(b)'s decomposition and §4's
  and §5's tables are produced by `measure8.py` and `measure9.py`, whose outputs
  are receipted there (round 2's m-9).
- **Cost, MEASURED**: 14 s to walk sixteen corpora; under a second to solve;
  **56.6 s per option pair** for the play-change probe — revision 2 said "90 s
  for the play-change probe" of six pairs, which round 2 flagged as a mismark
  under D-291 and this corrects.
- **The SPRT's wall time is ESTIMATED at 14 minutes** for 400 openings, from a
  MEASURED 2.04 s per opening in the design's registered dry run.
