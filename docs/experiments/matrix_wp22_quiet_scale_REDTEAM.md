# DECISION-RED-TEAM — `matrix_wp22_quiet_scale.md` revision 1

**Document attacked**: `matrix_wp22_quiet_scale.md`, revision 1, uncommitted at the time.
**Named revision**: `ae211f0` (`dev`). It matched HEAD at the start and the working tree was clean.
It did NOT match HEAD at the end (`0998a441`), and the reviewer records that **R6 landed as D-622
mid-review**, so the findings against the filter bear on a landed ADR line — D-617's own class.

**Instruments the reviewer built**, all outside the repo, no `cargo`, live tree untouched: an
independent extractor taking the position from the manifest's canonical `key_full` stone list
rather than replaying the corpus `moves`, enumerating windows in line coordinates rather than
by 18 keys per stone; and an **exact rational** active-set QP over `fractions.Fraction`.
Self-checks before any number: features agree with `tools/texel/features.py` on 6 908
stride-sampled positions, 0 disagreements; the `key_full` path agrees with the `moves`-replay
path on 8 165 positions, 0 disagreements.

## Re-derivation ledger (abridged to the load-bearing rows; every row reproduced)

| # | claim | reproduced |
|---|---|---|
| 1-4 | 89 805 deduped; 74 672 eval / 15 133 mate; **45 271 quiet (60.63 %)**; 39 507 / 5 764 split | YES |
| 5 | `g4`, `g5` identically 0 on every filtered row | YES — *and vacuously: the filter is defined that way* |
| 6-7 | sign counts; `t` = 6.36 / 31.70 / 5.55; eigenvalues, **cond 90.53** | YES |
| 7b | *reviewer's extra scope*: cluster-robust by game (3 487 clusters) `t` = 4.73 / 23.98 / 4.19; HC0 5.72 / 28.43 / 4.99 | **identification survives** |
| 8 | unconstrained quiet solve, in exact rationals → `1.1823, 22.5268, 9.0015` | YES |
| 9 | correlations `+0.1430, −0.0756, +0.2699` | YES |
| 10 | **every §3 row**, exact QP plus own MSE/ρ | YES, every cell |
| 11 | B is one integer from committed; its diagnostics move 0.0728 % | YES |
| 13 | 3.4 % / 95.7 % attribution | YES (3.7 % on a sharper reading) |
| 14 | `g5 > 0` in **0 of 89 805** | YES |
| 16 | `configs/eval_v0_weights.toml:44-48` for the Stage-0 quote | **NO — line 43** |
| 17 | *"design decides and states"* as R6's text | **NO — absent from the tree** |
| 18 | *"The design's §7 carries it"* (SPRT cost) | **NO — §7 carries no bounds, no cost, no dry run** |

## MAJOR findings

**M-1. §5's answer to its own strongest objection is circular, and the circle is digest-verifiable.**
All sixteen tranche reports carry `weights_sha256 41ef549666d787bf…`, equal to
`sha256sum configs/eval_v0_weights.toml`. Every label was produced by a 400 000-node search whose
leaf evaluator read the committed table. *"pistol-eval's own integer units"* is the committed
table's unit. The A/B/C/F distinction is not "data-derived vs hybrid" but "the committed table
enters once" vs "twice". **Fix**: state the provenance with the digest, withdraw "DATA-DERIVED end
to end", re-argue or re-select.

**M-2. Option A's entire diagnostic gain is absorption of a constant the feature set cannot
express; its residual VARIANCE is worse than the committed table's.** Mean label +70.3, mean
committed static eval −176.8, offset **+247.1**. Decomposed on validation: committed bias²
61 665 / var 663 078; A bias² 35 995 / var **668 947**. More than 100 % of A's MSE gain is
mean-offset absorption, which is why A's rank correlations FALL on both slices. The reviewer also
measured that the label/eval scale changes SIGN across depth strata (0.96, −0.74, 1.94, −2.03),
so "a unit" it is not. **Fix**: report the decomposition and re-argue A against the variance column.

**M-3. Shape and scale are not separable — the schema's gaps are ABSOLUTE.** Solving with `w3`
pinned at 10, 20, 40, 60, 120 gives shapes that renormalise to `[14.75, 54, 60]`, `[3.73, 57, 60]`,
`[1.53, 24.08, 60]`, `[1.00, 11.78, 60]`, `[0.50, 1.00, 60]`. B returns the committed table
*because* it pinned the committed scale. "The shape finding" and "the scale decision" are one
decision presented as two.

**M-4. The option set is incomplete.** Option **H** — the fitted shape re-expressed at the
committed balance, `[4, 57, 60, 300, 1500]` — pins a RATIO rather than a value, costs what A
costs, and beats every listed row on ALL-rows ρ (0.2789) and on residual variance (650 765).
*(Option D is correctly killed, but only inside the filter.)*

**M-5. The filter does not implement R6's predicate.** Over all 29 401 dropped eval rows, the
minimum hitting set over the opponent's four/five windows is 1 stone in 15 135 and 2 in 14 266 —
**100 % blockable with the turn's two stones, 0 forced**. Meanwhile **5 339 mate rows PASS the
filter** (distances 3, 5, 7), and a twenty-line double-threat detector fires on 200 of 200 sampled
quiet `mate_in 3` positions and 0 of 300 kept eval rows. The drop is 100 % one-sided (opponent-owned
in all 29 401) and shifts the mean label by +215 units between kept and dropped. **The one-sidedness
IS a good ground** — `g4 > 0` in 0 of 74 672 eval rows — but it is not the ground stated.

**M-6. Integer rounding at the chosen scale is a first-order confound of the ratios that decide
moves.** F's real solve carries the committed shape exactly (`w2/w1 = 6.00`) and the table that
would ship carries 8.00 — a 33 % shift from rounding alone. A's moves 26 %.

**M-7. Every option's cost is "one SPRT", and no SPRT is registrable at the governing revision.**
§6's pointer — *"The design's §7 carries it"* — is false against the tree.

**M-8. §1 quotes the operator ruling with four words the ruling does not contain**, and the
invented clause is the one that licenses the matrix. `git grep "design decides and states"` returns
nothing.

## MINOR findings

- **m-1.** Citation rot: `configs/eval_v0_weights.toml:44-48` → the quote is at **line 43**.
- **m-2.** `qp.py` is exact as used (agrees with exact rationals to 7e-15) but pivots on an
  ABSOLUTE threshold against a KKT matrix mixing `O(2e7)` and `O(1)`: scaling the same problem by
  `c ≥ 7e3` discards 14 of 15 active sets through a silent `continue` — verbatim the second half of
  the design review's M-4, reproduced in the script offered as its remedy. Hard rule 3. It also
  silently assumes `A` positive definite: fed the rank-3-of-5 problem it answers without complaint.
- **m-3.** "≈ 1/3 of committed" is true of `w3/w4` and false of `Σquiet/w4` (0.54).
- **m-4.** The registered dominance CHECK is never reported. Option A makes the table MORE
  tactically dominant than committed (`w4` vs `Σ(w1..w3)`: 4.05× → 7.50×).
- **m-5.** "1 000 reserved holdout openings" is unmarked MEASURED/ESTIMATED.
- **m-6.** Instruments without revisions: the probes are untracked.
- **m-7.** The matrix is not on `tools/governing_citation_check.sh`'s GOVERNING list.
- **m-8.** D-621's own count is imprecise on the population it names: **3 886** positions hold a
  five-window and **11** have the mover owning one (all both-sides). D-621's figures are exact on
  the `g5 ≠ 0` basis and its conclusion — `g5 > 0` in 0 rows — is exactly right.
- **m-9.** A's degradation is never stated: it is the only fitted row lowering BOTH ρ.

## QUESTIONS

- **Q-1.** Is pre-naming C the D-614 move, one arm later? The selector is still ρ.
- **Q-2.** Should the filter's operationalisation have been the matrix? Dropping only five-window
  rows keeps 72 663 eval rows and `g4` is still `+0 / −27 392` there, so the conclusion survives by
  a route the matrix does not state.
- **Q-3.** The labels are 400 000-node searches; the seat is 50 000, and the label/eval scale is
  unstable across the depth the 400k search reached.

## What the reviewer attacked and it SURVIVED

1. Every number in §1-§4 reproduces exactly at a scope the document never took — a different data
   path, a different window algorithm, exact rational arithmetic.
2. The identification claim survives clustering by game and HC0.
3. The attribution survives a sharper decomposition (3.7 % rather than 3.4 %).
4. The split's known per-position defect is measured **inert**: holding out whole games gives the
   same rounded table.
5. `qp.py` is exact as used, and the design review's M-4 headline defect is genuinely fixed.
6. The kill of option B reproduces.
7. D-621's load-bearing claim reproduces: `g5 > 0` in 0 of 89 805.
8. §1(1)'s kill of option D is sound within the filter.

## VERDICT

**The matrix must be revised before any option is selected.** Not because its arithmetic is wrong —
none of it is — but because the recommendation's argument is circular (M-1), its measured support
runs the other way (M-2), the option set is incomplete in the dimension the decision is about
(M-4), and the decision framed as one is two (M-3) sitting on a filter that does not do what it is
said to do (M-5).

**The strongest surviving attack, for whichever ADR line follows**: *the labels are the committed
table's own opinion backed up 400 000 nodes, so no fit to them in absolute units is a calibration
against anything but the committed table; and on the fitted population the v0 feature set is short
a constant worth +247 eval units, so the one free parameter the matrix is deciding is the parameter
that absorbs it.*
