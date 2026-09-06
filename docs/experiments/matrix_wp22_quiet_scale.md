# WP-2.2 Phase 1 — OPTION MATRIX: how the quiet terms are fixed, revision 3.

Governing revision: the commit that carries this file, **which also carries the
instruments §7 names and the design's §9** — revision 2 said that of a commit
that did not yet exist, which was round 2's M-1 and is the reason this line
reads as it does.

**Round 1 FAILED this matrix with eight MAJOR; round 2 FAILED revision 2 with
six.** Round 2 found six of round 1's eight genuinely discharged and two
RE-CREATED one step to the left, plus two new defects: the registered table was
not what the registered instrument computes, and the ADR line governing what may
be registered forbade the selected option. **This is the third and last round of
the grant, and it is REMEDIES ONLY**: every change below discharges a numbered
finding and introduces no new argument.

Reports: `matrix_wp22_quiet_scale_REDTEAM.md` (round 1),
`matrix_wp22_quiet_scale_REDTEAM_round2.md` (round 2). Measured numbers are
receipted in `artifacts/wp22_phase1_quiet/` and cited by digest in §7 (round 2's
m-9). Diagnostics GATE NOTHING (D-614).

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

Each table is the exact constrained minimiser of its own model, rounded.
**The play-change column decides whether an SPRT can measure anything at all**:
two tables choosing the same move everywhere make a self-match, where no
likelihood ratio is defined (D-156). Its draw is recorded in §7.

| # | how the quiet entries are fixed | table | play-change | quiet ρ | `w4 > Σ` |
|---|---|---|---|---|---|
| — | *committed, not fitted* | `[2, 12, 60, 300, 1500]` | — | 0.3873 | 300 > 74 |
| A | free absolute scale, no intercept | `[1, 19, 20, 300, 1500]` | 49.6 % | 0.3850 | 300 > 40 |
| B | normalise `w3 = 60`, no intercept | `[1, 12, 60, 300, 1500]` | 9.3 % | 0.3773 | 300 > 73 |
| C | normalise `Σw = 74`, no intercept | `[1, 21, 52, 300, 1500]` | 32.7 % | 0.3972 | 300 > 74 |
| F | committed SHAPE, one fitted scalar | `[1, 8, 38, 300, 1500]` | 14.1 % | 0.3857 | 300 > 47 |
| H | free-scale shape re-expressed at `w3 = 60` | `[4, 57, 60, 300, 1500]` | 65.7 % | 0.3868 | 300 > 121 |
| **J** | **intercept fitted and discarded, `w3` pinned INSIDE the solve** | **`[5, 34, 60, 300, 1500]`** | **48.0 %** | 0.4015 | 300 > 99 |
| S | no SPRT; report the fit as a finding | — | — | — | — |

**J's table is corrected from revision 2's `[4, 29, 60, 300, 1500]`** (round 2's
M-2). Revision 2 registered a table obtained by fitting all three quiet weights
freely beside the intercept and then RESCALING the result to `w3 = 60`. That
rescale is the minimiser of nothing, which is the defect §4's own header
forbids; **the shipped instrument pins `w3` inside the regression and prints
`[5, 34, 60, 300, 1500]`**, and the two differ on 14.4 % of positions at the
registered seat. The registered table is now the one the instrument computes.

**Option D — fit the tactical entries under dominance constraints — is not a
row**: its gradient is measured at exactly zero.

**Rounding, all options** (round 2's m-5 added B and C):

| | real solve | rounded | `w2/w1` real → integer |
|---|---|---|---|
| A | `[1.2538, 18.9098, 19.9098]` | `[1, 19, 20]` | 15.08 → 19.00 (+26 %) |
| B | `[1.0000, 11.7850, 60.0000]` | `[1, 12, 60]` | 11.79 → 12.00 (+2 %) |
| C | `[1.0000, 20.7671, 52.2329]` | `[1, 21, 52]` | 20.77 → 21.00 (+1 %) |
| F | `[1.2608, 7.5646, 37.8231]` | `[1, 8, 38]` | 6.00 → 8.00 (**+33 %**) |
| H | `[3.7785, 56.9864, 60.0000]` | `[4, 57, 60]` | 15.08 → 14.25 (−5 %) |
| **J** | `[4.8530, 33.6615, 60.0000]` | `[5, 34, 60]` | 6.93 → 6.80 (−2 %) |

F's real solve carries the committed shape exactly by construction and the table
that would ship does not, so "committed shape, one fitted scalar" is false of
what F would run.

## §5 Recommendation — J, on the only ground that survives round 2

**Revision 2 claimed steps 1-4 admitted exactly one table. They do not**
(round 2's M-3), and the claim is withdrawn. Applying the same steps under other
normalisations of the same intercept model gives, measured:

| normalisation | table | **`w3/w4`** | quiet ρ |
|---|---|---|---|
| **`w3 = 60`** | **`[5, 34, 60, 300, 1500]`** | **0.200** | 0.4015 |
| `w1 = 2` | `[2, 35, 64, 300, 1500]` | 0.213 | 0.4006 |
| `w2 = 12` | `[6, 12, 82, 300, 1500]` | 0.273 | 0.3669 |
| *committed* | `[2, 12, 60, 300, 1500]` | **0.200** | 0.3873 |

**The ground is the `w3/w4` column and nothing else.** `w3` is the coordinate
ADJACENT to the pinned tactical block, so pinning it at the committed value is
the only normalisation of the family that leaves the quiet-to-tactical ratio
exactly where the committed table has it. Under any other pin the top quiet
entry moves against a pinned `w4 = 300`, so the run would vary **the one
quantity the filtered rows are measured to be silent about** — the tactical
regressors are identically zero there — and a verdict could not say whether it
had measured the shape or the balance. **J-vs-committed is a contrast in the
shape the corpus determined and in nothing else.** No diagnostic enters this.

**Revision 2's "the two components a scale choice cannot manufacture" is
DELETED** (round 2's M-4), because it is measurably backwards:

- **Quiet ρ is scale-free but flat across the family** — 0.3873 at every rescale
  of the committed shape from ×0.5 to ×6.0, and 0.3669-0.4015 across the
  normalisations above. It separates nothing.
- **Residual variance and ALL-rows ρ are SCALE statistics.** Rescaling the
  committed shape with no fit at all moves residual variance 719 668 → 651 200 →
  713 891 across ×0.05, ×1.75, ×3.0, and ALL-ρ rises monotonically with scale,
  0.1236 → 0.2905. A pure ×1.75 rescale reaches ALL-ρ 0.2573 and changes play on
  31.2 %, with nothing fitted.

So the diagnostics support no option over another and are reported here only
because a matrix with no measured content is the estimate D-291 calls a finding.

**The pre-named second arm is DELETED** (round 2's Q-1). Revision 2 named H on
ALL-rows ρ, which is the scale statistic above; naming a second arm on it is the
move D-614 forbids, one arm later. **If J returns h0, that is the finding R7
registers and Phase 2 proceeds on it.**

**Why not the others.** A: §3(b) measures its whole gain as offset absorption,
and D-627 removes the clause that would have preferred it. B: one integer from
the committed table. C and F: their pins move `w3/w4` or their shipped table
does not carry the shape they are described by. H: the same rescale-of-nothing
defect J's own row was corrected for. S: R7 makes the SPRT what turns the
expectation into a measurement, and nothing measured says the run is
uninformative.

## §6 Costs and failure modes

| # | cost | failure mode |
|---|---|---|
| **J** | **one SPRT**, registered and MEASURED in the design's §9 | the intercept is estimated from the same labels, so "uncontaminated" means free of the OFFSET and not free of the committed table — §3(a) reaches J as much as any row; and pinning `w3` imports one committed number, which is a choice no evidence in this corpus can make |
| A | one SPRT | ships a table whose gain is a constant the feature set cannot hold |
| B | one SPRT | the least powerful comparison in the field, for a reserved holdout slice |
| C, F, H | one SPRT | each varies the quantity §5 says the corpus is silent about, or ships a shape it is not described by |
| S | none | forgoes the only measurement that answers R7 |

## §7 Instruments, receipts, and what is ESTIMATED

- `tools/texel/{features,extract,fit,verify_against_engine,test_texel}.py` **at
  this document's own commit**, which carries them. Round 2's M-1 found revision
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
