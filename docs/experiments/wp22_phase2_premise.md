# WP-2.2 Phase 2 — premise memo (D-477), revision 1.

Governing revision: `0998a44` (`dev`). Every claim below is a quotation, a
`file:line`, or a MEASURED number with the instrument named. The instrument for
§3 is `artifacts/wp22_phase2_premise/probe_patterns.py`, receipted beside its
output.

**What a premise memo is for**: to state what is true before a matrix is
written, so the matrix's rows are scored against the corpus rather than against
a plan. D-621 and R6 (D-622) already foreclose one whole family of rows; §3
below forecloses another, and it does so with a measurement that cost ninety
seconds.

## §1 What Phase 2 inherits, and it is a division of labour

R6 (D-622): *"Phase 2 inherits the split: the learned family evaluates quiet
structure; the search evaluates tactics."*

So the learned family is **not** asked to learn what a four- or five-window is
worth. D-621 established why — game rule 4 completes a win the instant a stone
forms six, and a corpus position is recorded at a turn boundary, so the mover
never holds a live five-window and the mover-relative feature takes one sign
only. **A matrix row that proposes to learn tactical value from search labels is
killed by D-621 on its face**, and this memo does not re-argue it.

## §2 The population, MEASURED

R6's filter — no length-6 window holds four or more stones of one side and none
of the other — keeps **45 271** of the corpus's 74 672 `eval` rows, **60.63 %**.
That is the population any Phase 2 family trained on this corpus learns from,
and it is the denominator every count in §3 is over.

Instrument: `tools/texel/fit.py`'s own `select`, whose per-clause populations are
printed by the shipped script.

## §3 THE FINDING — observations per pattern cell, and the ROADMAP's registered window length is not supported

`docs/ROADMAP.md`'s Stage-2 entry registers *"3 directional maps, length-11
axial windows"*. **Measured over the quiet population, a length-11 codebook is
not learnable from this corpus.**

The instrument walks every quiet position, enumerates every axial window of
length `L` holding at least one stone (deduped per position by axis and start),
encodes it as a ternary code oriented to the side to move, and counts.

| L | distinct cells | observations | mean / cell | **median / cell** | **cells seen < 10** | cells covering 90 % |
|---|---|---|---|---|---|---|
| 7 | **1 990** | 11 874 459 | 5 967 | **660** | **0 (0.0 %)** | 368 (18.5 %) |
| 9 | 16 145 | 14 247 128 | 882 | 30 | 3 721 (23.0 %) | 852 (5.3 %) |
| 11 | **62 370** | 16 613 729 | 266 | **6** | **38 585 (61.9 %)** | 1 599 (2.6 %) |

**Length 7 is SATURATED and length 11 is not close.** The growth curves, over the
same walk:

```
L7   5000:1983  10000:1990  15000:1990 … 45000:1990     flat from 10 000 on
L9   5000:11125 10000:13211 …            45000:16137    still climbing
L11  5000:24555 10000:33925 …            45000:62222    still climbing, steeply
```

L7 reaches 1 990 of the 2 186 codes a length-7 ternary window can take and stops
moving. L11's curve has no flat stretch at all, so its cell count is a **count of
distinct things accumulated**, which D-619 records cannot be extrapolated from a
sample — and is not extrapolated here: 62 370 is what 45 271 positions actually
produced, and the true support is larger.

**Why the medians are what decides it.** A per-cell weight fitted from six
observations is not fitted; it is memorised. At L11, 61.9 % of cells carry fewer
than ten observations and 16.0 % carry exactly one. At L7 no cell carries fewer
than ten and half carry more than six hundred.

**One caveat, stated because it cuts the other way.** The saturation at L7 is
partly R6's filter doing its job: the filter removes exactly the positions that
would generate the densest patterns, so these counts describe the QUIET
population and not the game. A family that also evaluated tactical positions
would see patterns this census cannot.

## §4 The store API — the seam already exists and Phase 2 does not design one

`crates/pistol-eval/src/eval.rs` defines the `Eval` trait, and its contract is
what a codebook family has to satisfy, unchanged:

- `apply(at, player)` / `undo(at, player)`, called *"at the same seam the board
  and the zobrist key are updated"* (D-41), and *"The two are inverses, and the
  value depends only on the *set* of stones applied"*.
- `value(side_to_move)`, side-relative, magnitude never exceeding `EVAL_MAX`.
- `delta(at, player)`, whose default body IS the apply/value/undo roundtrip, so
  a backend that does not override it computes what the search computed before
  the method existed — *"a performance path, never a correctness"* one.
- *"Integer arithmetic only, no interior randomness, no dependence on iteration
  order"* — CLAUDE.md rule 4, and the reason a learned family is quantised
  rather than floating.

**The trait is object safe on purpose**, and the engine *"picks a backend from
config at construction"*. So the whole of Phase 2's engine-side integration is a
new `impl Eval` plus a `backend` value the config schema accepts. **Nothing in
the store API is Phase 2's to decide**, which is one row a matrix does not need.

## §5 The budget

The deployment budget is CLAUDE.md's: *"a strong move consistently within 0.5 s
(stretch 0.1-0.3 s) on a single workstation; online evaluation on CPU."* The
optimization arc left the engine **1.294x faster** end to end
(`docs/experiments/opt_arc_CLOSURE.md`), measured with byte-identical output.

**The nps figure a matrix row costs against is registered as OWED rather than
quoted here**: the arc's closure reports RATIOS, and a ratio is not a budget. It
is measured at the standing instrument seat and recorded in §5 of this memo's
next revision, before any matrix row claims a cost. Quoting a remembered
absolute here is the estimate D-291 calls a finding.

## §6 What is NOT established here

- **Nothing about which family wins.** That is the matrix's, attacked by a
  fresh DECISION-RED-TEAM before selection.
- **Nothing about Phase 1's SPRT.** Phase 2 proceeds on either verdict (R7,
  D-623); h0 is its premise and h1 changes only which table the comparison
  starts from.
- **Nothing about a corpus recorded off the turn boundary.** D-621's flip clause
  names it, and it would change §1 rather than §3. Generating one is a package,
  not a paragraph.
- **Nothing about the human corpus.** `docs/ROADMAP.md` blocks D-434's Stage-2
  holdout until a population-grade corpus supersedes the artifact-grade one, and
  D-453 licenses no statement about the platform's players.
