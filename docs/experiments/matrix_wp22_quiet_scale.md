# WP-2.2 Phase 1 — OPTION MATRIX: how the quiet terms are fixed, revision 5.

Governing revision: the commit that carries this file, which also carries
`tools/texel/options.py` — the instrument that PRINTS every table below and that
`test_texel.py` drives. Revision 3's §7 named two scratchpad harnesses that had
been left unrunnable by a signature change, one of them still computing a table
this document had withdrawn.

**Rounds 1 through 4 all FAILED** (13 findings, then 6, then 3, then 6 MAJOR).
This is the **second and last** revision of the operator's grant (D-631).

**What is and is not claimed about provenance**, because revision 4 claimed more
than was true and round 4 caught it: **§4's option table is printed in full by
`tools/texel/options.py`**, a committed instrument driven by `test_texel.py`, and
its output is receipted. **§2's and §3's numbers are NOT** — they come from
probes under gitignored `artifacts/`, receipted by digest and named in §7, and
one of them (the patched-engine tempo measurement) is a reviewer's and is
attributed as such.

Reports: `matrix_wp22_quiet_scale_REDTEAM.md`, `_round2.md`, `_round3.md`,
`_round4.md`.

## §1 The decision

R6 (D-622, corrected by D-626) splits eval v0's five weights into TACTICAL
(`w4`, `w5`) and QUIET (`w1..w3`) and fits only the quiet ones.

1. *What sets the tactical entries.* **Closed by measurement.** On the filtered
   rows the tactical regressors are identically zero, so the objective's
   curvature in `w4` and `w5` is exactly zero and no feasible point is preferred
   to any other. They are carried verbatim from the committed table; dominance
   survives as a registered CHECK, and §4's last two columns report it per
   option — `options.py` prints them, which is what round 2's m-6 asked for and
   what revision 4's rewrite silently dropped again.
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

| # | the pin | table | play-change | val MSE | resid var | quiet ρ | ALL ρ | **`w3/w4`** | **`Σ/w4`** | `w4>Σ` |
|---|---|---|---|---|---|---|---|---|---|---|
| — | *committed, not fitted* | `[2, 12, 60, 300, 1500]` | — | 724 743 | 663 078 | 0.3873 | 0.2088 | **0.2000** | **0.2467** | ✓ |
| **T** | **`w3 = 60` AND `Σ = 74`** | **`[4, 10, 60, 300, 1500]`** | **29.8 %** | **728 957** | 661 257 | 0.3743 | 0.2122 | **0.2000** | **0.2467** | ✓ |
| J | `w3 = 60` | `[5, 34, 60, 300, 1500]` | 48.0 % | 762 720 | 635 707 | 0.4015 | 0.2666 | **0.2000** | 0.3300 | ✓ |
| S | `Σ = 74` | `[4, 32, 38, 300, 1500]` | 52.4 % | 729 303 | 640 056 | 0.3890 | 0.2455 | 0.1267 | **0.2467** | ✓ |
| P1 | `w1 = 2` | `[2, 35, 64, 300, 1500]` | 46.8 % | 755 424 | 640 639 | 0.4006 | 0.2575 | 0.2133 | 0.3367 | ✓ |
| P2 | `w2 = 12` | `[6, 12, 82, 300, 1500]` | 42.7 % | 761 711 | 654 532 | 0.3669 | 0.2343 | 0.2733 | 0.3333 | ✓ |
| K | *unpinned* | `[5, 33, 67, 300, 1500]` | 45.2 % | 769 834 | 636 034 | 0.4045 | 0.2691 | 0.2233 | 0.3500 | ✓ |
| N | no SPRT | — | — | — | — | — | — | — | — | — |

**The two bold columns are what the decision is about.** The filtered rows carry
**no evidence about ANY** quiet-to-tactical exchange rate — the tactical
regressors are identically zero on all 45 271 of them — so a pin holds one rate
and moves the others. **Every SINGLE pin moves at least one**: J holds `w3/w4`
and moves `Σ/w4` from 0.2467 to 0.3300; S does exactly the reverse.

**T is the row revision 4 did not carry, and round 4 supplied it.** Pinning
`w3 = 60` **and** `Σ = 74` together holds BOTH rates at their committed values
and leaves one degree of freedom for the corpus to speak in — the split of the
committed quiet total between the one- and two-stone entries, which it moves
from `2 : 12` to `4 : 10`. It is the constrained minimiser under its pins, it is
strictly interior, it changes exactly the two integers §1 allows, and at 29.8 %
play-change it is nowhere near a self-match.

**Option D — fitting the tactical entries under dominance constraints — is not a
row**: its gradient is measured at exactly zero.

## §5 Recommendation — T, on D-627's own principle applied fully

**Revision 4's ground is withdrawn, and it was wrong in both halves.** It
selected J because `w3` sits on the quiet side of R6's split, calling that *"the
exchange rate the division of labour is about — how much quiet structure a four
outweighs"*. Round 4 showed two things and both reproduce here:

- **R6's own text quantifies that boundary as the SUM, not the top entry.**
  D-622 records the operator's words: *"each tactical class exceeds **the sum of
  everything below it** by a registered gap"*, which §1 carries as the dominance
  check `w4 > w1 + w2 + w3`. Revision 4's own gloss — "how much quiet structure a
  four outweighs" — **describes `Σ/w4` while naming `w3/w4`**. On the authority
  it cited, the argument selects S.
- **"One of the two had to be" is false.** T holds both.

**THE GROUND, and it is already in the log rather than invented here.** D-627,
replacing the clause it deleted, says the selection *"turns on which
normalisation leaves the quantity the corpus is SILENT about where it already
was"*. The corpus is silent about every quiet-to-tactical exchange rate. **T
leaves two of them where they were; J and S each leave one; K leaves none.**
Applied fully rather than stopped at the first pin, D-627's principle selects T,
and it needs no appeal to which rate is "the" balance — the point is that the
run should vary as little as possible of what the data cannot speak to.

**What T's experiment asks, and it is well posed.** Holding the committed scale
and the committed quiet total, does the corpus want the one-stone/two-stone split
different from `2 : 12`? It answers `4 : 10`. **A J-vs-committed or S-vs-committed
SPRT would confound that answer with a change in a ratio the corpus has no
opinion about; a T-vs-committed SPRT does not.**

**The cost of asking the well-posed question, registered rather than discovered.**
T is the smallest change in the family and changes 29.8 % of moves against J's
48.0 %. A smaller behavioural change is, other things equal, a smaller Elo
effect, and against the design's registered alternative of 50 normalized Elo
that makes h0 more likely for T than for J. **That is a property of asking the
narrow question, not a reason to ask the wide one**: choosing the arm that
changes play most because it is likelier to cross is selecting on the outcome.

**What no measurement here supports.** The diagnostics separate nothing: quiet ρ
spans 0.3669-0.4045 across the family, and residual variance and ALL-rows ρ move
with the quiet-to-tactical balance, which is the decision's own axis rather than
an independent judge of it. **They are printed because a matrix with no measured
content is the estimate D-291 calls a finding, and they are used for nothing.**
Revision 4 carried a bullet of rescale anchors in support of that claim; round 4
measured that two of its three anchors were not rescales at all and that its
monotonicity claim was false of a rescale. **The bullet is DELETED rather than
corrected** — §5 already reaches its conclusion without it, so the numbers did no
work and were the only wrong ones in the document.

**The rivals, recorded so a successor knows the choice was contested.** S is the
pin R6's registered check names and has the family's second-lowest validation
MSE; J holds the rate at the split's own boundary and changes play most; K is
the only row importing no committed quiet number at all. **None is selected, and
the reason is D-627's principle and nothing else.**

**No second arm is pre-named.** Revision 2 named one on a diagnostic that round 2
measured to be a scale statistic; a second arm would also need openings
`docs/book_v2_ledger.md` does not have to spare.

## §6 Costs and failure modes

Revision 4 left this section untouched from revision 2, so it scored a dead
option set, omitted three live rows, and reused the letter `S` for the "no SPRT"
row while §4 and §5 used it for the `Σ = 74` pin. It is rewritten against the
seven rows §4 now prints, and "no SPRT" is **N**.

| # | cost | failure mode |
|---|---|---|
| **T** | **one SPRT**, registered and MEASURED in the design's §9 | the smallest change in the family, so the likeliest of the seven to return h0 at the registered alternative — and its two pins import two committed numbers, which is more imported arithmetic than any other row even though it is less imported *freedom* |
| J | one SPRT | holds `w3/w4` and moves `Σ/w4` by 34 %, so an h1 cannot be attributed to the quiet shape alone |
| S | one SPRT | holds `Σ/w4` and moves `w3/w4` from 0.2000 to 0.1267, with the same attribution problem mirrored |
| P1, P2 | one SPRT each | each holds the rate its own name fixes and moves both others; P2 additionally has the family's worst rank correlation |
| K | one SPRT | imports no committed quiet number and therefore moves every exchange rate, which is the largest confound of the seven |
| N | none | forgoes the run. **Not "the only measurement that answers R7"** — the design's §9.2 measures that the registered run does NOT answer R7 at all, and revision 4 still carried that phrase |

**The failure mode common to J, S, P1, P2 and K** is the one §4's bold columns
make visible: an h1 is attributable to the quiet shape only in so far as the
comparison holds the un-evidenced rates fixed, and only T holds both.

## §7 Instruments

- **`tools/texel/options.py` prints §4's table in full**, is driven by
  `test_texel.py`, and reports a pin whose answer the schema cannot hold as
  `INFEASIBLE` rather than crashing. Its output is
  `artifacts/wp22_phase1_quiet/OPTIONS.txt`, sha256 `32d815924de2897e…`.
- **The play-change figures** are
  `artifacts/wp22_phase1_quiet/PLAYCHANGE_intercept.txt`, sha256
  `89de3b17afa959ed…`, produced by
  `probe_disagree.py` with the command it prints — every 300th `eval` row, 248
  positions, all sixteen tranches, `go nodes 50000`.
  `PLAYCHANGE.txt` holds the earlier no-intercept family and the WITHDRAWN
  `[4, 29, 60]` beside it.
- **`measure8.py` and `measure9.py` ARE NOT INSTRUMENTS OF THIS DOCUMENT and the
  sentence saying they were is deleted.** Both abort against the shipped
  `fit.py`, and one of them computed the withdrawn table; round 3 found this and
  revision 4 left the false pointer standing beside the working one, which is the
  fourth instance of this package's own pattern. They remain in `artifacts/` as
  the record of what produced revision 2's numbers and produce nothing now.
- **§2's and §3's numbers come from `artifacts/wp22_phase1_quiet/`**, receipted
  by digest — `probe_perside.py`, `probe_block.py`, `probe_disagree.py` — and are
  NOT printed by a committed instrument. The tempo-cancellation measurement in
  §3(b) is a round-2 reviewer's, taken with a patched engine that is in no
  worktree that survives, and it is attributed to that report rather than
  claimed here.
- **`tools/texel/{features,extract,fit,verify_against_engine,test_texel}.py`**
  at this document's own commit, written out in §7's citation list so the gate
  recognises them as paths.
- **Cost, MEASURED**: 12.9 s to walk sixteen corpora; 1.0 s for `options.py`;
  56.6 s per option pair for the play-change probe. **The SPRT's wall time is
  ESTIMATED at 14 minutes** for 400 openings, from a MEASURED 2.046 s per
  opening in the design's registered dry run.
