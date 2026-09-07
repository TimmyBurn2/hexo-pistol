# The hex threat enum — derivation memo, revision 1. Written before any compute.

**Governing revision**: `0585e25` (`dev`), the revision every file, line and
count quoted below was read at (D-692). **NOTHING HERE IS MEASURED.** Every
number in this document is either a quotation from a receipted artifact, a
DERIVED count with its arithmetic on the page, or a bound. The class count `k`,
the code count, the observation census, the purity terms and the stabilisation
length are OUTPUTS of §6's instrument and appear in §7 only after it has run —
a derivation memo that already knew its answer would be the shape D-483 exists
to forbid.

**What this document is for.** `docs/research/eval_families_2026-09.md` §7 lists
`R-A4-CLASS` as *"the only row that is covering (§0.1) and densely evidenced at
once"* and states its kill in the same breath: *"the hex enum cannot be derived
without importing 4-axis thresholds, which `threat_calculus_v1.md`'s scope rule
forbids."* §8 of the same document records that the enum *"does not exist"*.
This memo defines it so that it can be COMPUTED rather than written, and D-706
is the standing line that binds the definition.

---

## §1 Premises, quoted (D-477)

An option matrix's axis is a premise and is quoted at `file:line` like any
other. So is a derivation's.

### 1.1 The scope rule — the kill condition, verbatim

`docs/research/threat_calculus_v1.md:12-13`:

> **Scope rule.** Every published *number* cited below was established on 4-axis
> square boards unless explicitly marked hex; algorithms/structures transfer,
> numbers do not.

**This is the condition the enum has to survive**, and it is why the enum is an
equivalence relation rather than a list of named shapes. A named class set has a
provenance; an equivalence class of a computed tuple has members and nothing
else. The two candidate imports are both on the record and both refused:
Rapfi's 16-member `Pattern` enum (`eval_families_2026-09.md` §A4, *"a 4-bit
`Pattern` enum with exactly 16 members"*, 4 axes) and the survey figure *"A
survey of nine classical line engines found **none using more than 13 threat
classes**"* (§1). Neither number appears in §5's tuple, and `k` is an output.

**AND THE ARITHMETIC THAT MOTIVATED THE ROW IS ITSELF AN IMPORT, WHICH THIS
MEMO SAYS ON ITS OWN FACE.** `eval_families_2026-09.md` §A4 derives *"The hex
analogue is `C(18,3) = 816` codes"*, and `C(18,3) = C(16+2,3)` — the 816 is
Rapfi's `k = 16` carried across the axis count. It is a DERIVED consequence of
an imported `k` and it is not a premise of this memo. The row's parameter count
is `C(k+2, 3)` at the `k` §6 computes, whatever that turns out to be, and if
that number is not 816 then §7 says so and the matrix prices the row on it.

### 1.2 The calculus definitions the tuple is built from

All at `docs/research/threat_calculus_v1.md`, revision v1.0, read at `0585e25`.

| ID | line | what the tuple takes from it |
|---|---|---|
| `DEF-WINDOW` | `:28` | *"6 consecutive cells on one axis. **Open** for side X iff no opponent stone in it. Overline monotonicity ⇒ 6-windows suffice for all theory."* The unit the tuple counts over, and the live/dead predicate. |
| `LEM-AXES` | `:37` | *"3 axes/cell (vs 4 square)"* — why a cell's code is a 3-multiset and not a 4-tuple. |
| `LEM-MONO` | `:40` | *"stones are never removed; an own extra stone never hurts"* — why the completion cost is monotone and why a class boundary cannot be crossed by removing a stone. |
| `LAW-SUPPORT` | `:68-71` | *"A forced win in k own turns requires an open window already holding ≥ 6−2k own stones."* The stones-still-needed arithmetic, read per STONE rather than per turn. |
| `RULE-EXACT` | `:64-66` | *"t is computed exactly … never read from a pattern table and never derived by weight algebra. Patterns may order moves; only exact t decides truth."* The discipline: every component is computed from the position, none is looked up. |
| `LAW-DECOMP` | `:87-90` | *"Regions with disjoint stars: t is additive … **The same additivity is FALSE for static evaluation — never sum regional eval as if independent.**"* Recorded in §8 as the standing limitation of the whole additive-codebook family, NOT as support for it. |
| `THM-WINDOW` | `:152-155` | *"length-6 occupancy tables cannot classify live vs dead fours (needs 8-cell context; fives need 7) … The 'length-11' Gomoku convention is k=5 lore — **re-derive the minimal sufficient hex length by enumeration.**"* The owed enumeration §6 discharges. |
| `E-PHASE` | `:156-157` | *"win threshold differs by intra-turn phase (phase 2: plan ≤1 wins); tempo terms condition on (side, phase)."* Why rule-4 relevance is a named component and why phase conditioning is left OUT of scope (§9). |
| `PROTO-NODE` step 1 | `:128` | *"**Win-now check.** A completing stone ends the turn instantly (second stone unplayed)."* Game rule 4, in the calculus's own words. |

### 1.3 The query sites, quoted at `d83ac01` and re-quoted at HEAD

`crates/pistol-solver/src/query.rs:81-142`. **MEASURED at `0585e25`**:
`diff <(git show d83ac01:crates/pistol-solver/src/query.rs) crates/pistol-solver/src/query.rs`
returns nothing — the file is BYTE-IDENTICAL at `d83ac01` and at HEAD, so the
re-quotation is the same quotation and the range still means what it meant.
Line 81 is `pub enum LiveCount {` and line 142 is
`    pub fn live_windows_at_count(&self, side: Player, count: LiveCount) -> &[Window] {`
at both revisions.

What the range holds, and it is exactly the vocabulary the tuple re-uses:

- `LiveCount` (`:80-86`) — *"Exactly two own stones, no opponent stone"* /
  *"Exactly three own stones, no opponent stone"*: a LIVE window at an exact own
  count. The tuple's `open` component is the same live predicate, counted rather
  than classified.
- `NearHot` (`:88-99`) — *"Hot is `own >= 4`, so exactly one count — three — is
  one stone away from it"*: the one-stone-short reading. The tuple's `min`
  component generalises it to any distance.
- `WinWitness` (`:101-123`) — `OnePly` *"One stone completes a window already
  holding five. Valid at either budget."* and `Pair` *"Two stones complete a
  window holding four. Valid ONLY at `StonesLeft::Two`"*. The tuple's rule-4
  component is `OnePly`'s condition, and `Pair`'s is `min = 2`.
- `hot_windows` / `win_in_one_ply_windows` / `completed_windows` (`:127`, `:132`, `:137`) —
  the three shipped classes, all of them thresholds on the same two quantities
  the tuple carries continuously.

**THE POINT OF QUOTING THE SITE IS THAT THE TUPLE IMPORTS NOTHING FROM IT
EITHER.** `Class::Hot`'s `own >= 4` is a THRESHOLD, and a threshold is what
D-706 forbids taking on faith; the tuple carries `min` and `open` at full
resolution and lets the equivalence do the classifying. That the shipped
solver's three classes are recoverable as unions of tuple classes is a property
§6 reports (§6.5), not an assumption.

### 1.4 The corpus, and the receipts that hold its counts

- **89 805 deduped positions.** MEASURED at `0585e25`:
  `/usr/bin/grep -vc "^#" artifacts/arc3r_sweep_deduped_manifest.txt` → `89805`;
  `sha256sum` of that manifest is
  `00f61780cc1654958696786051dbd8de7d1bbab3f5d49153b1694770caf35968`, which is
  the digest `docs/book_v3_ledger.md` quotes in its disjointness receipt.
- **74 672 `eval` rows; 45 271 quiet (60.63 %)** — `wp22_phase2_premise.md` §2,
  under R6's filter (D-622): *"no length-6 window holds four or more stones of
  one side and none of the other"*.
- **3 487 distinct games, 25.75 positions each, median 23** —
  `training_pipeline_2026-09.md` §1, instrument
  `artifacts/research_2026-09/game_count.txt`.
- **The research receipt verifies.** MEASURED at `0585e25`: `sha256sum -c
  artifacts/research_2026-09/RECEIPT_research_2026-09.sha256` → **7 of 7 OK**,
  and the receipt's own digest is
  `13fe5712dafa94e2916267800f7b4162db380e27b03a16e00fd97f0981916dab`, equal to
  the figure `training_pipeline_2026-09.md` §9 states.
- **The census this memo's instrument re-points**, from `eval_families_2026-09.md`
  §0.2, folded columns, over the 45 271-row quiet population:

  | L | folded cells | median obs/cell | cells < 10 obs | cells covering 90 % |
  |---|---|---|---|---|
  | 7 | 1 029 | 1 282 | 0 (0.0 %) | 190 |
  | 8 | 2 920 | 273 | 24 (0.8 %) | 293 |
  | 9 | 8 374 | 56 | 1 079 (12.9 %) | 434 |
  | 11 | 38 983 | 7 | 22 142 (56.8 %) | 810 |

  **That census counts WINDOWS by `(axis, start)`. This memo's does not** — §4
  explains why a class enum is indexed by CELL, and §6.3 records that the two
  populations are different and are not compared cell for cell.

### 1.5 The game rules the tuple depends on, quoted from CLAUDE.md

> 2. Win: ≥6 own stones contiguous along one axis. Overlines (7+) win.
> 4. A win completes the instant any single placed stone forms ≥6; the turn's
>    second stone is then not played.
> 5. Legal placement: within hex-distance 8 of an existing stone.

Rule 2 is why the window is 6 long and why overline monotonicity licenses
`DEF-WINDOW`'s *"6-windows suffice for all theory"*. Rule 4 is the third tuple
component. Rule 5 bounds the alternative cell population §6.3 reports beside the
one it scores.

---

## §2 What the enum IS, in one paragraph

A **class** is an equivalence class of length-`L` single-axis patterns under a
PROPERTY TUPLE. The tuple's components are counts and minima over the set of
6-windows through one cell, each computed from `DEF-WINDOW`'s own open/dead
predicate. Two patterns are in the same class iff their tuples are equal. The
enum is therefore the QUOTIENT of the pattern space by the tuple, its members
have no names, and `k(L)` — the number of classes at length `L` — is an output
of enumerating the tuple over every pattern, never a number this memo chooses.

---

## §3 The pattern space

Fix a cell `c` and an axis `a ∈ {(0,1), (1,0), (1,−1)}` — the three directions
`Axis::ALL` fixes, mirrored at `tools/texel/features.py:14` as
`AXES = ((0, 1), (1, 0), (1, -1))`.

For odd `L = 2m + 1`, the **pattern** `P_L(c, a)` is the sequence of `L` cells
`c + (i − m)·a` for `i = 0 … L−1`, each read MOVER-RELATIVE as one of
`{empty, own, opp}` — `own` is the side to move. `c` sits at index `m`.

**THE CENTRE CELL IS EMPTY AND IS NOT ENCODED, AND THE REASON IS THIS GAME'S AND
NOT RAPFI'S.** Only EMPTY cells are classified. The tuple asks *what a stone
placed at `c` buys*, which is the only question a cell-indexed table can ask,
and rule 4's component is literally *"does one placed stone complete a six"* —
a question with no meaning at an occupied cell. So `c`'s own reading is a
constant (`empty`) and carries no information, and the code has `L − 1` ternary
positions rather than `L`. `eval_families_2026-09.md` §A4 records that Rapfi
does the same — *"the centre cell is never encoded, being the candidate
placement"* — and that is a CONFIRMATION read off a 4-axis engine, not the
ground: the ground is that an occupied cell has no completion question.

**EVEN `L` HAS NO CENTRE AND IS DEFINED AS A PAIR.** For even `L` there is no
cell at the middle, so `P_L(c, a)` is taken as the unordered PAIR of the two
alignments — `c` at index `L/2 − 1` and `c` at index `L/2` — and the class is
the multiset of the two tuples. This is stated rather than avoided because §6
computes `L = 8` and because a single alignment would break §5.4's reversal
invariance, which is the one property the fold in
`eval_families_2026-09.md` §0.4 rests on.

---

## §4 Why the index is a CELL and not a window

`R-A4-CLASS` is scored *"on the 3-axis multiset"* with `C(k+2, 3)` parameters,
and `C(k+2, 3)` is the count of size-3 multisets over `k` classes. A cell has
exactly one length-`L` pattern per axis and therefore exactly three classes; a
window has one axis and one class. **`C(k+2, 3)` is a per-CELL object and the
row is only well posed per cell.**

**Which cells.** A cell is SCORED iff it is empty and at least one of its three
length-`L` patterns holds a stone. Cells outside that set take the single
all-empty code on all three axes, which is a constant and carries no
information. This mirrors the committed census's own *"holding at least one
stone"* rule transposed from windows to cells. §6.3 reports the size of the
alternative population — every empty cell inside rule 5's radius-8 legal region
— beside it, because a deployed evaluator has to choose one and the matrix
should see both counts.

**AND A WINDOW NOT CONTAINING `c` IS NOT `c`'s BUSINESS.** The tuple ranges over
6-windows THROUGH `c` only. A 6-window lying inside `P_L(c, a)` but missing `c`
is scored at its own cells' codes, and counting it here would make one
structure's weight depend on how many cells happen to see it. This is a
containment convention and it is stated as one; it is not licensed by
`LAW-DECOMP`, which says the opposite thing about eval and is recorded in §8.

---

## §5 The tuple

Let `W_L(c, a)` be the set of 6-windows that contain `c` and lie wholly inside
`P_L(c, a)`. **DERIVED**, by counting starts: `|W_L| = min(6, L − 5)` for
`L ≥ 6`, so `|W_7| = 2`, `|W_8| = 3`, `|W_9| = 4`, `|W_11| = 6`, and
`|W_L| = 6` for every `L ≥ 11` — the six windows `c−5…c` through `c…c+5`. A
window through `c` spans at most `c−5 … c+5`, which is `2·6 − 1 = 11` cells;
that is `eval_families_2026-09.md` §0.1's covering bound, re-derived here from
`DEF-WINDOW` alone.

For each side `X ∈ {own, opp}` (mover-relative):

- **`open_X`** = `|{w ∈ W_L(c, a) : w holds no stone of the other side}|`.
  `DEF-WINDOW`'s *"Open for side X iff no opponent stone in it"*, counted. Range
  `0 … |W_L|`. **This is the "count of distinct completing windows, live vs
  dead" component**: a window that is not open for `X` can never be completed by
  `X` (`LAW-HIT`, `:45-46`: *"One defender stone in any empty cell of a window kills
  that window permanently"*), so `open_X` is exactly the count of `X`'s surviving
  completions through `c`.
- **`min_X`** = `min over open w of (6 − |X's stones in w|)`, and `∞` when
  `open_X = 0`. The exact number of further `X` stones needed to fill some
  window through `c`, `c` itself among them since `c` is empty. Range
  `1 … 6`, or `∞`. This is `LAW-SUPPORT`'s *"≥ 6−2k own stones"* read at one
  stone per unit instead of two per turn, and `RULE-EXACT` is why it is computed
  over the window set rather than read from a shape table.
- **`r4_X`** = `[min_X = 1]`. Game rule 4 / `PROTO-NODE` step 1: one placed
  stone completes six and the turn ends with its second stone unplayed.

**The class of `P_L(c, a)` is the 6-tuple**
`(min_own, open_own, r4_own, min_opp, open_opp, r4_opp)`.

### 5.1 `r4` IS REDUNDANT AND IS KEPT ANYWAY, AND THIS MEMO SAYS SO FIRST

`r4_X ⟺ min_X = 1` by definition, so the component induces **no refinement at
all**: the classes of the 6-tuple and of the 4-tuple without `r4` are the same
classes. §6 MEASURES that (it computes `k` both ways and they must be equal — a
disagreement is an instrument fault, not a finding). It is named in the tuple
because `E-PHASE` conditions the win threshold on the intra-turn phase and
`min = 1` is the only cost a phase-2 mover can pay; writing it down keeps the
rule-4 reading visible where a reader would otherwise have to re-derive it. **A
component that changes no class may not be used to claim the tuple is richer
than it is**, and §7 reports `k` once.

### 5.2 The tuple's range, DERIVED

`min_X ∈ {1,…,6, ∞}` — 7 values; `min_X = 6` is an all-empty open window and
`min_X ≥ 1` because `c` is empty and must itself be filled; `|X| = 6` cannot
occur in a window containing the empty `c`. `open_X ∈ {0,…,|W_L|}`. The two are
linked: `open_X = 0 ⟺ min_X = ∞`. So per side at `L ≥ 11` there are at most
`1 + 6·6 = 37` combinations, and at most `37² = 1369` joint ones. **That is a
BOUND and not a prediction**: most joint combinations are unrealisable, and
`k(11)` is whatever §6 enumerates.

### 5.3 `k` is an output, and `C(k+2, 3)` is CUBIC in it

`C(k+2, 3) = (k+2)(k+1)k / 6`. At `k = 16` it is 816; at `k = 40`, 11 480; at
`k = 100`, 171 700 (DERIVED, arithmetic above). **The row's parameter count is therefore extremely sensitive
to a quantity this memo deliberately does not choose**, which is why §6.4
computes a COARSENING LADDER beside the full tuple.

### 5.4 Reversal invariance, BY CONSTRUCTION

Reversing `P_L(c, a)` about `c` permutes `W_L(c, a)` and maps each window to a
window with the same per-side stone counts. Every component of the tuple is a
count or a minimum over that set, so the tuple is invariant and the class is
invariant. **This is a priori and needs no measurement** — where
`eval_families_2026-09.md` §0.4 had to MEASURE fold-invariance for raw codes
(0 of 2 200 folded images differ, 1 760 of 2 200 unfolded do), a class enum
inherits it from the tuple's shape. §6 nonetheless checks it as an instrument
self-check, because an invariance that is claimed and not checked is a claim.

### 5.5 What the tuple deliberately does NOT carry

- **A threshold.** `Class::Hot`'s `own >= 4`, `WinInOnePly`'s `= 5`, Rapfi's 16,
  the survey's 13: none appears. §6.5 reports which unions of tuple classes the
  three shipped solver classes are, as a check that the enum is at least as fine
  as the code the engine already runs.
- **A name.** No class is called a four, a three, a gap trap or a rhombus.
  `PAT-GAP` (`threat_calculus_v1.md:104`) is the reason: *"Absent from named-shape
  taxonomies; tables mis-score it"* — a name is where a taxonomy's blind spot
  lives, and the quotient has none.
- **The intra-turn phase**, and **the side-to-move's stone budget.** See §9.
- **Anything off the axis.** The tuple is single-axis by construction; the three
  axes meet only in the cell's multiset. `LEM-CROSS` (`:38`, *"two lines on distinct
  axes meet in ≤1 cell"*) is why that is the only place they can meet at a cell,
  and `LAW-DECOMP`'s eval warning (§8) is why the multiset is a limitation.

---

## §6 What the instrument computes, and the instrument is named with its revision

**Instrument**: `tools/hex_enum/enum.py` (the tuple, the class table, the
derivation checks) and `tools/hex_enum/census.py` (the corpus walk), driven by
`tools/hex_enum/test_hex_enum.py` through `tools/hex_enum_tests.sh`, at the
revision that lands them. `docs/process.md`'s *"Instrument governing revision"*
binds: a change to either reopens this memo's review.

### 6.1 `k(L)`, by exhaustive enumeration

For each `L`, enumerate all `3^(L−1)` patterns, compute the tuple, count
distinct tuples. `3^6 = 729` at `L = 7` through `3^12 = 531 441` at `L = 13`:
exhaustive, no sampling, no seed. Reported for `L ∈ {7, 8, 9, 11, 13}` as the
dispatch's headline set, and for the contiguous range `L = 6 … 14` because §6.6's
refinement test needs `k(L+1)` beside `k(L)`.

### 6.2 The code count

`C(k(L) + 2, 3)`, the size-3 multiset over the classes, DERIVED. Reported beside
the raw-code ceiling for the folded single-axis code with the centre dropped,
`(3^(L−1) + 3^⌈(L−1)/2⌉)/2` — `eval_families_2026-09.md` §0.2's form at `L−1`
ternary positions, without its `−1`, because that document's `−1` excludes the
all-empty WINDOW and a single axis of a scored CELL may legitimately be empty
(§4). So the compression the quotient buys is visible as a ratio.

### 6.3 Observations per code, on the quiet population

The R6 population — 45 271 quiet `eval` rows — walked position by position. For
each position: the scored cell set of §4, each cell's three axis classes, the
sorted triple as one code, one observation per (position, cell). Reported: total
observations, distinct codes seen, mean and median observations per code, the
count of codes covering 90 % of observations, and **cells under Buro's floors**
at all three of his published figures (`eval_families_2026-09.md` §A1):
`≤ 4` (*"the weight is set to 0"*), `< 20` (*"sufficiently high (say ≥ 20)"* for
a safe fit), and `< 75` (his generator's own keep rule). A growth curve every
5 000 positions, as the committed census prints one, so saturation is visible
rather than asserted.

**AND THE ALTERNATIVE POPULATION IS COUNTED BESIDE IT**: the number of empty
cells inside rule 5's radius-8 legal region, per position, so the matrix can see
what the scored set excludes. **These counts are NOT comparable cell-for-cell
with `eval_families_2026-09.md` §0.2**, whose unit is a window at `(axis, start)`
and whose population is windows holding a stone; the document already warns that
*"a centred-window codebook indexes the same space differently, so these counts
are indicative rather than exact for that variant"*, and this memo does not
compare them.

### 6.4 The coarsening ladder

Because `C(k+2, 3)` is cubic in `k` (§5.3), the instrument reports the same
statistics for four PROJECTIONS of the same tuple, each a strictly coarser
equivalence and none of them hand-written:

| rung | the equivalence | why it is a projection and not a new enum |
|---|---|---|
| **T4** | the full tuple | — |
| **T3** | `(min_own, open_own, min_opp, open_opp)` with `open` clipped to `{0, 1, ≥2}` | the live-count distinction `LiveCount` makes, at the resolution `LAW-OVERLOAD`'s `t ≥ 3` needs |
| **T2** | `(min_own, min_opp, [open_own ≥ 1], [open_opp ≥ 1])` | completion cost plus alive-or-dead — `DEF-WINDOW`'s predicate and `LAW-SUPPORT`'s arithmetic and nothing else |
| **T1** | `(min_own, min_opp)` with both clipped at `≥ 4` | the cost alone, at the resolution `LAW-SUPPORT` gives for `k ≤ 2` own turns |

Each rung is a function of T4, so the ladder is a chain of quotients and every
rung inherits §5.4's reversal invariance. **The ladder is not a selection**: §7
reports `k`, codes, observations and purity at each rung and recommends nothing.

### 6.5 CLASS PURITY, and both terms are named (D-479)

The label is the corpus row's `score_value` at `SCORE_KIND = eval`
(`tools/texel/extract.py:24-27` names the columns). For each observation the
label is its POSITION's label. Over all observations, with classes `C`:

- **WITHIN-class variance** `= Σ_C (n_C / N) · Var(label | C)` — the mean of the
  per-class variances, weighted by class size.
- **BETWEEN-class variance** `= Σ_C (n_C / N) · (mean_C − mean)²` — the variance
  of the class means, weighted by class size.
- Their sum is the total label variance (the law of total variance), which the
  instrument checks to machine precision as a self-check.
- `η² = between / total` is reported as a ratio, and it is **not** a fit quality
  and gates nothing (D-614).

Reported at the WINDOW unit — one observation per `(position, cell, axis)`,
classed by that axis's class — and at the CODE unit — one per `(position,
cell)`, classed by the 3-multiset — because the enum's own purity and the row's
purity are different questions.

**AND TWO REFERENTS, BECAUSE A PURITY NUMBER ALONE PASSES VACUOUSLY**
(`docs/process.md`, *"Criterion and defect class"*): a criterion the named
defect preserves is not a criterion. The named defect is *the quotient throws
away a distinction that carries value*. Two things it does not preserve:

1. **The un-quotiented ceiling** — `η²` for the RAW folded single-axis code at
   the same `L`. The quotient can only lose `η²`; the gap is what it costs.
2. **A random quotient of the same size** — every raw code assigned to one of
   `k` buckets by a fixed-seed hash, seed recorded. This is the externally
   derived referent: it shares the corpus and the `k` and shares nothing of the
   calculus, so a tuple whose `η²` does not beat it has separated nothing the
   corpus can see.

**THE HONEST CAVEAT, stated before the number exists.** Every observation of one
position carries that position's single label, so observations are massively
non-independent and `η²` is a statement about how much of the LABEL variance the
class marginal explains on this corpus — not a fit, not a bound on a fit, and
not comparable to a per-position `R²`. The effective `n` is closer to the 3 487
games than to the observation count (`training_pipeline_2026-09.md` §1). The
random-quotient referent is what makes the comparison readable in spite of that,
because it carries the same dependence structure.

### 6.6 `THM-WINDOW`'s owed enumeration, and what is DERIVED in it

The stabilisation length is the smallest `L` at which `k(L+1)` refines no class
of `k(L)` — that is, at which every pair of length-`(L+1)` patterns whose
`L`-cores are in one `L`-class are themselves in one `(L+1)`-class.

**THE ANSWER IS PARTLY DERIVED AND THE MEMO SAYS WHICH PART, BEFORE THE RUN.**
§5's `|W_L| = min(6, L − 5)` means the tuple's reach is exactly `c ± 5`: at
`L ≥ 11` the window set is complete and no further cell can enter any component,
so `k(L) = k(11)` for all `L ≥ 11` **by derivation**. The instrument's report at
`L = 12, 13, 14` is therefore a CONFIRMATION of the derivation and an instrument
self-check, not an independent measurement, and §7 will label it so.

**THE MEASURED HALF IS BELOW 11**, and it is the half that carries content: how
many `k(11)` classes each `k(L)` class merges for `L < 11`, and — the question
that actually decides a length — whether the merging costs `η²` on the corpus.
A short window is cheap and covering-deficient; the ladder in §6.4 and the
curve in `L` are the two axes the matrix prices `R-A4-CLASS` on.

**THE TUPLE'S LONGEST-RANGE PROPERTY NEEDS 11 CELLS AND NOTHING IS TRUNCATED
SILENTLY.** At `L < 11` the tuple is computed over the windows that FIT, which
is a different question from the one at `L = 11`, and the instrument reports the
two as different rows rather than as one row at two resolutions.

### 6.7 Instrument self-checks, run before any reported number

1. **Reversal.** Every pattern's class equals its reverse's class, at every `L`
   enumerated. §5.4 says this is a priori; the check is what makes it a fact
   about the code.
2. **`r4` redundancy.** `k` with and without the two `r4` components is equal
   (§5.1). A disagreement is an instrument fault.
3. **Law of total variance.** `within + between = total`, to machine precision
   (§6.5).
4. **Window count.** `|W_L| = min(6, L − 5)` as enumerated equals the DERIVED
   value (§5).
5. **Solver agreement.** For each shipped solver class — `Hot` (`own ≥ 4`
   live), `WinInOnePly` (`own = 5` live), `Completed` — the set of length-11
   patterns satisfying it is a union of tuple classes (§6.5's promise). A
   pattern-level counterexample is a finding against the tuple.

---

## §7 Outputs

**EMPTY UNTIL §6's INSTRUMENT HAS RUN.** Every number named in §6 lands here
with the digest of the artifact that produced it (D-483, D-483's *"cited from
that run's artifact by digest"*). **NO SELECTION.** This memo says what the enum
IS and what it measured; which row wins is the matrix's, and the matrix does not
select either (D-708).

---

## §8 The standing limitation, recorded rather than buried

`LAW-DECOMP` says, verbatim: *"The same additivity is FALSE for static
evaluation — never sum regional eval as if independent."* **Every row in family
A is an additive table, this one included**, and a per-cell code summed over
cells is exactly regional eval summed as if independent. This is not an argument
against `R-A4-CLASS` specifically — it binds `R-A1`, `R-A2`, `R-A3` and `R-A5`
identically, and `handcrafted_v0` too, which sums 18 window contributions per
stone — but it is the calculus's own statement that the family's functional form
is known-wrong, and a memo that cited `LAW-DECOMP` only for the part that helps
would be quoting selectively. `eval_families_2026-09.md` §2 records the same
thing from the other side: family B buys the interactions an additive table
cannot express, and pays incrementality for them.

---

## §9 What this memo does NOT establish

- **Nothing about which row wins**, which length is chosen, or which ladder rung
  is used. §7 reports; D-708 says selection is the architect's.
- **Nothing about the evaluator.** How a code's weight enters a score, whether
  the sum is over cells or windows, and what the accumulator's type is are
  Phase 2b's, behind the `Eval` trait the premise memo §4 already fixes.
- **Nothing about the intra-turn phase.** `E-PHASE` conditions win thresholds on
  the phase; conditioning the CODE on the phase would double `C(k+2, 3)`, and
  every corpus position is recorded at a turn boundary (D-621), so the corpus
  cannot price the conditioned variant at all. Named as out of scope, not as
  settled.
- **Nothing about tactical value.** D-621 and D-622 foreclose learning it from
  these labels; the enum is a QUIET-structure representation and the tactical
  terms stay pinned.
- **No strength claim, no Elo, no committed file.** Nothing here touches
  `configs/`.
