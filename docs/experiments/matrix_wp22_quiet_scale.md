# WP-2.2 Phase 1 — OPTION MATRIX: how the quiet terms are fixed, revision 2.

Governing revision: `fe5c992` (`dev`). **Revision 1 was attacked by a
fresh-context DECISION-RED-TEAM and FAILED**, eight MAJOR
(`matrix_wp22_quiet_scale_REDTEAM.md`). Every one of its numbers reproduced at a
scope revision 1 never took, and the FAIL was about the ARGUMENT: the
recommendation was circular, its measured support ran the other way, the option
set was incomplete, and the filter did not do what it was said to do. This
revision concedes all four, adds two options, and reaches a different answer.

**Every number below is MEASURED unless marked ESTIMATED** (D-291), and the
instruments are named in §7. Diagnostics GATE NOTHING (D-614); §5 states in
terms which of them entered the selection and which are reported after it.

## §1 The decision

R6 (D-622) splits eval v0's five weights into TACTICAL (`w4`, `w5`) and QUIET
(`w1..w3`) and fits only the quiet ones. **Two questions then arise and only the
second is open.**

1. *What sets the tactical entries.* **Closed by measurement.** On the filtered
   rows the tactical regressors are identically zero — 0 of 45 271 — so the
   objective's curvature in `w4` and `w5` is exactly zero and no feasible point
   is preferred to any other. A dominance CONSTRAINT bounds them and cannot
   choose among the points it admits. **They are carried verbatim from the
   committed table**, and dominance survives as a registered CHECK. *(Attacked
   and upheld: the red team verified the curvature is zero, and noted correctly
   that it is zero by construction of the filter — which is why §2 restates what
   the filter does.)*
2. *What sets the quiet entries.* **This is the decision**, and revision 1 posed
   it too narrowly as "the scale".

**The clause revision 1 attributed to R6 is not R6's.** *"design decides and
states"* appears nowhere in D-622 or in the tree; it is from the dispatch's
"Phase 1, narrowed" bullet — *"tactical terms carried verbatim from the
committed table (or as constraints, design decides and states)"*. The
delegation is real and its source is the dispatch, not the ruling.

## §2 What the filter actually does — the label was wrong and the ground is better

R6's words are *"rows where neither side holds an in-window forced win"*, and
D-622 landed that. **Measured over all 29 401 dropped `eval` rows**, with a
minimum-hitting-set over every opponent four-or-five window's empty cells:

| | count |
|---|---|
| the MOVER owns a live four-or-more window | **0** |
| the OPPONENT owns one | 29 401 |
| the mover kills every one of them with **one** stone | 15 135 |
| with **two** — and a turn places two (rule 3) | 14 266 |
| **needing three or more, i.e. actually forced** | **0** |

**Not one dropped row is a forced loss.** The predicate is not "no forced win";
it is *"the mover is not facing a four"*. Independently confirmed by the red
team's own detector, which found **5 339 mate rows that PASS the filter** —
proven wins at distances 3, 5 and 7 with no four-window on either side — so the
fitted population is clean of forced wins because `score_kind != eval` removed
them, not because this clause caught anything.

**The sound ground is ONE-SIDEDNESS, and it is stronger than the stated one.**
Over all 74 672 `eval` rows the mover-relative `g4` is positive in **0** and
negative in 29 165; `g5` is positive in **0** and negative in 2 009. Both
tactical regressors carry the same defect D-621 found for `g5` alone: no sign
variation anywhere, so neither can report what such a window is worth to its
OWNER. Dropping those rows removes regressors that could only ever have been
fitted backwards. **D-626 corrects D-622's mechanism on this evidence; the
predicate and the ruling stand.**

## §3 The two findings that decide the options

### (a) The label's unit IS the committed table — verified by digest

Revision 1 defended a free absolute scale with the corpus header's
*"score_units eval is pistol-eval's own integer units"*. Measured:

```
all sixteen tranche reports:  weights_sha256 41ef549666d787bf…
sha256 configs/eval_v0_weights.toml: 41ef549666d787bf…
```

**Every label was produced by a 400 000-node search whose leaf evaluator read the
committed table, byte-identical to the one at HEAD.** So the unit is not
neutral; it is the committed table's own. **What this kills** is revision 1's
claim that matching labels in absolute units is a calibration rather than a free
choice. **What it does NOT kill** is the phase: the target is the committed eval
*plus 400 000 nodes of lookahead*, and distilling that back into the eval is a
coherent thing to want. It means the ABSOLUTE SCALE carries no information the
committed table did not already have.

### (b) The feature set is short a term worth +247, and a free scale absorbs it

On the fitted population the mover is systematically behind in window count and
ahead in label — it is about to place two stones:

```
mean LABEL  +70.3      mean committed STATIC EVAL  -176.8      offset +247.1
mean g1 -9.698   mean g2 -3.697   mean g3 -1.884
```

The v0 model has **no constant term**, so a no-intercept fit can only absorb that
offset by moving the weights. Decomposed on the validation slice, the free-scale
fit's MSE gain is **entirely** offset absorption: bias² falls 61 665 → 35 995
while residual variance RISES 663 078 → 668 947.

**And the contamination reaches the SHAPE, which revision 1 reported as its
headline finding.** Fitting the offset explicitly, as a mover-relative intercept
the v0 schema has no term for:

```
without an intercept :  w = [ 1.1823, 22.5268,  9.0015]     NON-monotone
with one             :  w = [ 4.9404, 32.8870, 66.9980]     MONOTONE, c = +366.2
```

**Revision 1's "the corpus wants a three-window worth less than a two-window" is
an artifact of the missing intercept and is WITHDRAWN.** With the tempo term
fitted, the corpus wants a monotone table and no schema constraint binds at all.

**Why the intercept must not be shipped and must not be absorbed.** A
mover-relative constant is added to every sibling at a ply and negated at the
next, so under negamax at a fixed depth it cancels exactly between the moves
being compared — and `configs/instrument_v0.toml` carries `q_depth_turns = 0`,
`extension_budget = 0` and `lmr_min_depth_turns = 0`, so an iteration's leaves
sit at one ply. **A term that cannot change a move is the worst possible thing to
spend the weight vector on.**

## §4 The options

Each table is the exact constrained minimiser rounded to the schema, and each is
schema-feasible. **The play-change column is the one that decides whether an
SPRT can measure anything at all**: two tables that choose the same move
everywhere make a self-match, where no likelihood ratio is defined (D-156). It is
measured over 248 corpus positions at the registered seat, `go nodes 50000`.

| # | how the quiet entries are fixed | table | **play-change** | quiet ρ | **residual var** | ALL ρ | val MSE |
|---|---|---|---|---|---|---|---|
| — | *committed, not fitted* | `[2, 12, 60, 300, 1500]` | — | 0.3873 | 663 078 | 0.2088 | 724 743 |
| A | free absolute scale | `[1, 19, 20, 300, 1500]` | 49.6 % | 0.3850 | 668 947 | 0.1954 | **704 942** |
| B | normalise `w3 = 60`, re-solve | `[1, 12, 60, 300, 1500]` | 9.3 % | 0.3773 | 667 200 | 0.1989 | 724 216 |
| C | normalise `Σw = 74`, re-solve | `[1, 21, 52, 300, 1500]` | 32.7 % | 0.3972 | 654 717 | 0.2215 | 720 463 |
| F | committed SHAPE, one fitted scalar | `[1, 8, 38, 300, 1500]` | 14.1 % | 0.3857 | 681 720 | 0.1788 | 715 377 |
| H | free-scale SHAPE re-expressed at `w3 = 60` | `[4, 57, 60, 300, 1500]` | 65.7 % | 0.3868 | 650 765 | **0.2789** | 835 568 |
| **J** | **intercept fitted and discarded, shape at `w3 = 60`** | **`[4, 29, 60, 300, 1500]`** | **40.7 %** | **0.4063** | **638 260** | 0.2577 | 746 364 |
| S | no SPRT; report the fit as a finding | — | — | — | — | — | — |

**Option D — fit the tactical entries under dominance constraints — is not a row**:
§1(1) measures its gradient at exactly zero, so it is an unconstrained choice
wearing a constraint's name.

**Rounding, which revision 1 did not report** (red team M-6). The schema takes
integers, and at a small scale rounding moves the ratios that decide moves:

| | real solve | rounded | `w2/w1` real → integer |
|---|---|---|---|
| A | `[1.2538, 18.9098, 19.9098]` | `[1, 19, 20]` | 15.08 → 19.00 (**+26 %**) |
| F | `[1.2608, 7.5646, 37.8231]` | `[1, 8, 38]` | 6.00 → 8.00 (**+33 %**) |
| H | `[3.7785, 56.9864, 60.0000]` | `[4, 57, 60]` | 15.08 → 14.25 (−5 %) |
| **J** | `[4.4243, 29.4520, 60.0000]` | `[4, 29, 60]` | 6.66 → 7.25 (+9 %) |

F's real solve carries the committed shape exactly by construction, and the table
that would ship does not — so "committed shape, one fitted scalar" is false of
what F would actually run.

## §5 Recommendation — J, and the derivation is not a diagnostic

**J is not the row that scored best; it is the row the two findings of §3
construct.** The derivation, in order, with no diagnostic in it:

1. §3(a): the absolute scale carries no information the committed table did not
   have. ⇒ **do not fit the scale.**
2. §3(b): the fitted population carries an offset the feature set cannot
   express, and a no-intercept fit absorbs it into the weights. ⇒ **fit the
   offset explicitly so it stops contaminating them.**
3. A mover-relative constant cannot change a move at a fixed depth. ⇒ **discard
   it rather than ship it**, and never let it be paid for out of the weights.
4. Nothing in the filtered rows speaks to the quiet-to-tactical balance. ⇒ **pin
   it where the only evidence leaves it**, at the committed `w3 = 60`.

Steps 1-4 admit exactly one table, and it is J. **Only then** are the
diagnostics read, and they agree: J has the best rank correlation on the fitted
population (0.4063) and the best residual variance (638 260) of every row here —
the two components a scale choice cannot manufacture. **They confirm the
derivation; they did not produce it**, which is the distinction D-614 turns on.

**J changes play on 40.7 % of positions**, so the SPRT has something to measure.

**What is registered as J's limit, rather than argued away.** The intercept is
estimated from the same contaminated labels, so "uncontaminated" means "free of
the offset", not "free of the committed table" — §3(a) applies to every row here
and to J as much as to A. And a mover-relative constant cancels *exactly* only
where every leaf of an iteration sits at one ply; a terminal win ends a line
early, so the cancellation is very good and not perfect.

**Why not H, which wins the ALL-rows rank correlation.** H is J's construction
with step 2 left out — the same rescaling applied to the offset-contaminated
solve. Choosing it over J on ALL-ρ would be choosing the contaminated arm because
a diagnostic preferred it, which is D-614's own case. **H is registered as the
pre-named second arm** if J returns h0, and it is named HERE, before the run.

**Why not A**, revision 1's recommendation: §3(b) measures that its entire MSE
gain is offset absorption and that its residual variance is worse than the
committed table's. It is the only fitted row that lowers BOTH rank correlations.

**Why not B**: it returns the committed table but for `w1`, changes play on 9.3 %
of positions, and its own diagnostics move by 0.07 %.

**Why not S**: R7 (D-623) makes the SPRT the thing that turns the expectation
into a measurement, and J is a table the corpus produced under a derivation that
survives the red team's two strongest findings.

## §6 Costs and failure modes

| # | cost | failure mode |
|---|---|---|
| A | one SPRT | ships a table whose gain is a constant the feature set cannot hold; the run measures the tempo term, not the structure |
| B | one SPRT | 9.3 % play-change on a one-integer change: the least powerful comparison in the field, for a reserved holdout slice |
| C | one SPRT | the normalisation is one of several with nothing distinguishing them, and `Σw = 74` is itself a committed number |
| F | one SPRT | the shipped table does not carry the shape F is described by (33 % ratio shift from rounding alone) |
| H | one SPRT | carries §3(b)'s contamination into the shape; wins a diagnostic and loses the derivation |
| **J** | **one SPRT**, cost registered in the design's §9 and MEASURED by its dry run | the intercept is estimated from the same labels; and pinning `w3 = 60` imports one committed number, which is a choice no evidence can make for us |
| S | none | forgoes the only measurement that answers R7 |

## §7 Instruments, and what is ESTIMATED

- `tools/texel/{features,extract,fit,verify_against_engine}.py` at the commit
  that lands this revision. Nothing above stands on a script outside the tree.
- The play-change probe and the blockability probe are receipted in
  `artifacts/wp22_phase1_quiet/` with the commands that produced them.
- **The offline work costs seconds**: 14 s to walk sixteen corpora, under a
  second to solve, 90 s for the play-change probe. MEASURED.
- **The SPRT's wall time is ESTIMATED nowhere here.** Revision 1 said "the
  design's §7 carries it" and that was FALSE against the tree: the design at
  that revision carried no bounds, no cost and no dry run. The design's §9 at the
  commit that lands this revision carries all three, and the dry run MEASURES the
  cost before the run is launched.
