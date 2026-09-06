# DECISION-RED-TEAM — `matrix_wp22_quiet_scale.md` revision 2 (round 2 of three)

**Named revision**: `cbd18b8` (`dev`). It matched HEAD at the start and at the end; the working
tree was empty of changes at both. One temporary worktree was created under `/home/tom` and
removed; the live tree was never modified.

**Instruments the reviewer built**, none in the repo: an independent extractor taking positions
from the manifest's canonical `key_full` stone list and enumerating windows by grouping stones
onto lines and sliding a length-6 two-pointer window, keeping per-side counts over all 89 805
positions; an **exact-rational** solver for every fit including the intercept model and four
normalisations; an exact minimum-hitting-set blockability probe; a play-change probe drawing
**uniformly over all 74 672 eval rows across all sixteen tranches**; and **a patched engine in a
detached worktree whose `HandcraftedV0::value` gains a mover-relative constant from an env var**.

**Self-check before any number**: the reviewer's extractor agrees with the engine's own
`HandcraftedV0::value` on **44 819 positions — every one of the 39 195 holding a 4-, 5- or
6-window on either side, plus a stride-9 draw of the remainder — 0 disagreements.**

## Re-derivation ledger (load-bearing rows; all reproduced except where marked)

| # | claim | reproduced |
|---|---|---|
| 1-8 | 89 805 / 74 672 / 15 133 / 45 271 (60.63 %) / 39 507 / 5 764; sign counts; 3 886 and **11**; offset **+247.1061**; mean g1/g2/g3 | **YES** |
| 9-10 | no-intercept `[1.1823, 22.5268, 9.0015]` non-monotone; **intercept `[4.9404, 32.8870, 66.9980]`, c = +366.19553, monotone** | **YES**, exact rationals |
| 11-14 | A, H, F, J real solves; **all 42 cells** of §4's table; bias/variance; the rounding shifts | **YES** |
| 15-16 | blockability 0 / 29 401 / 15 135 / 14 266 / **0 needing ≥3**; 5 339 mate rows pass — 5 307 `mate_in` at 3/5/7 **plus 32 `mated_in` at 4 and 6** | **YES** |
| 17 | §3(a) digest identity, sixteen tranches | **YES** |
| 18 | play-change at the reviewer's own draw (499 positions, all sixteen tranches): 48.70 / 11.62 / 35.27 / 12.22 / 63.13 / **41.68 %** | **YES** |
| 19-20 | §7's "instruments at the commit that lands this revision"; "the design's §9 carries all three" | **NO — M-1** |
| 21 | J is what the registered instrument computes | **NO — it prints `[5, 34, 60, 300, 1500]`; M-2** |
| 23 | §7's "90 s for the play-change probe" | **NO — 56.6 s for ONE of six pairs** |

## MAJOR findings

**M-1. The instruments are not in the tree at the revision the document names, and §7 says the
opposite in terms.** At `cbd18b8`, `verify_against_engine.py` is not tracked and `fit.py` has none
of `select`, `committed_table`, `constrained_min`, `schema_constraints` or
`tempo_normal_equations` that the receipted probes call. `git show cbd18b8:…wp22_phase1_design.md`
is still headed revision 2 and **contains no "§9" at all**. This is round 1's M-7 re-created one
step to the left: revision 1 pointed at a §7 that carried nothing; revision 2 points at a §9 that
does not exist.

**M-2. The registered table is not the table the registered instrument computes, and they play
differently on one position in seven.** The staged `fit.py` pins `w3` at 60 INSIDE the regression
and prints `[5, 34, 60, 300, 1500]`. The matrix's J, `[4, 29, 60, 300, 1500]`, comes from fitting
all three quiet weights freely beside the intercept and RESCALING to `w3 = 60` — a minimiser of
nothing, while §4's own header says each table is "the exact constrained minimiser". Measured at
the registered seat over 250 positions, the two choose a different move on **36 of 250 (14.4 %)**.

**M-3. §5's "Steps 1-4 admit exactly one table" is false, and §6 of the same document concedes
it.** Every normalisation against a committed number satisfies all four steps: `w1 = 2` gives
`[2, 13, 27]`, `w2 = 12` gives `[2, 12, 24]`, `Σ = 74` gives `[3, 23, 47]`, all schema-feasible and
materially different engines (23.2 % and 16.4 % play-change against J). Round 1's M-3 undischarged.

**M-4. The diagnostics do not confirm the derivation, they complete it, and the licensing sentence
is measurably backwards.** Quiet ρ is scale-free but flat (0.3873 for the committed shape at ×0.5,
×1.0, ×3.0, ×6.0; 0.4027-0.4067 within J's family, maximised by `Σ = 74`). **Residual variance is
exactly the scale statistic** — rescaling the committed shape alone moves it 719 668 → 651 200, a
68 000 swing against the 24 818 by which J is said to beat committed. ALL-ρ rises monotonically
with scale (0.1499 → 0.3121). **A pure ×1.75 rescale of the committed table, with no fit at all,
reaches ALL-ρ 0.2573 and changes play on 31.2 %.**

**M-5. J's play-change is not receipted, and the receipt that exists cannot be reproduced from what
it records.** `PROBES.md` carries A, B, C, F, H and no J, and records outputs without the `stride`
that sets the entire scope: at stride 1-10 the draw never leaves tranche 1.

**M-6. D-623's binding consequence forbids J, and the matrix never confronts it.** D-623:
*"the registered target must be a table the corpus produced end to end … which is why the option
matrix's selection turns on data-derivedness rather than on any diagnostic."* J imports `w3 = 60`;
it is precisely the hybrid D-623 names. Under hard rule 10 this must be amended, not read past —
and the ground for amending exists in round 1's own M-1.

## MINOR findings

- **m-1.** `round_to_schema` still projects silently at the FLOOR, inside a routine whose docstring
  says "REFUSING rather than clamping"; the only refusal added covers the ceiling, which on the
  registered path is **unreachable by construction** (`60 >= 300` never).
- **m-2.** `normal_equations`' guard tests `a[i][i] == 0.0` (degeneracy); D-621's finding is *no
  sign variation*, which leaves the diagonal at 132 953 and 4 692 over the eval rows. The guard
  cannot detect the condition it cites.
- **m-3.** `constrained_min`'s post-hoc optimality check passes vacuously on the registered problem
  (the answer is interior, slacks +3.85 / +27.81 / +25.34), and its detection floor is 3.66e-03
  against a 1e-6 step. What IS fixed: the singular set is counted and printed, and `solve`'s
  implicit scale-relative pivoting is the right shape of fix for round 1's m-2.
- **m-4.** §4's "each is the exact constrained minimiser" is false for B and C, whose real solves
  violate `w1 >= 1`; both round to the same shipped tables, so no number moves.
- **m-5.** The rounding table omits B and C.
- **m-6.** The dominance CHECK is still never reported (round 1's m-4, undischarged). It passes
  everywhere: J `300 > 93`, H `300 > 121`, A `300 > 40`.
- **m-7.** "90 s for the play-change probe. MEASURED." — one pair takes 56.6 s and there are six.
- **m-8.** D-626's "distances 3, 5 and 7" describes 5 307 of the 5 339 passers; 32 are `mated_in`
  at 4 and 6.
- **m-9.** D-483: §3(b)'s decomposition and §4's tables are cited from no artifact by digest.
- **m-10.** The stated governing revision is the PARENT of the commit that lands the document.

## QUESTIONS

- **Q-1.** What licenses a second arm at all under D-614, when the diagnostic naming it is a scale
  statistic?
- **Q-2.** D-626's corrected ground is a reason not to FIT `w4`/`w5`, not a reason to DROP the
  rows. Fitting on all 74 672 with the tactical contribution as a known offset uses 65 % more data;
  it is foreclosed by D-622, and the matrix should say that is why it is absent.
- **Q-3.** The registered limit hedges that the cancellation is "very good and not perfect".
  Measured, it is **exact**. Is a hedge no measurement supports worth carrying?
- **Q-4.** What does the play-change column license that "not a self-match" does not?

## What the reviewer attacked and it SURVIVED

1. **Step 3 is right, and stronger than the document claims — the attack pressed hardest, and it
   failed completely.** A patched engine at `cbd18b8` taking a mover-relative constant from
   `PISTOL_RT_TEMPO`: control passes; constants **1, 10, 100, 366, 1000** change **0 of 200**
   bestmoves under the committed table, and **1, 366, −366, 5000** change **0 of 120** under J —
   and **the `info` node counts and depths are identical in all 448 records**, so the alpha-beta
   tree is literally unchanged. `q_depth_turns = 0`, `extension_budget = 0`,
   `lmr_min_depth_turns = 0`, `aspiration_delta = 0`, `root_reorder = false`, heuristics off, and
   **no futility, razoring or null-move anywhere in `crates/pistol-search`**. Terminal nodes do not
   break it: `MATE_THRESHOLD = 29 000` against `EVAL_MAX = 16 000`.
2. §3(a) reproduces exactly; round 1's M-1 is conceded correctly and the digest is the right
   instrument for it.
3. The §3(b) withdrawal is correct, in exact rationals.
4. §2 and D-626 reproduce at the reviewer's own scope with a different algorithm. **And the
   corrected ground licenses exactly D-622's filter and no other**: dropping on `g4`/`g5`/`g6`
   != 0` keeps the same 45 271 rows, 0 disagreement, because the mover owns a 4+ window in 0 of
   74 672 rows. That coextension is a measured fact of this corpus, not a definition.
5. Every count and every diagnostic cell reproduces, under both the no-ties Spearman and a
   tie-corrected one, which changes no ordering.
6. The extractor is right: 44 819 positions, 0 disagreements.
7. The play-change column reproduces at a scope it never had, J's 40.7 % included (41.68 %).
8. §1's kill of option D is sound.
9. Round 1's M-8 is discharged.

## VERDICT

**The matrix must be revised again before any option is selected.** Round 1's M-1, M-2, M-4, M-5,
M-6 and M-8 are genuinely discharged and could not be broken. **M-3 and M-7 are not discharged and
each has been re-created one step to the left.** Two new defects are worse than either: the
registered table is not what the registered instrument computes, and the ADR line governing what
may be registered forbids the selected option and is never mentioned.

**Option S is not the right answer.** Nothing measured says the run would be uninformative; what is
measured says it is **not yet registrable**.

**If forced to select, still J — but for a reason the document does not give, and it is one
sentence**: *`w3` is the coordinate adjacent to the pinned tactical block, so pinning it at the
committed value is the only normalisation that leaves the un-evidenced quiet-to-tactical ratio
exactly where the committed table has it, which makes the J-vs-committed SPRT a contrast in the
shape the corpus determined and in nothing else.* Under `w1 = 2` or `Σ = 74` the top quiet entry
moves to 27 or 47 against a pinned `w4 = 300`, so the run would vary the one quantity step 4 says
the corpus is silent about. That argument singles out J, survives M-3 and M-4, and needs no
diagnostic.
