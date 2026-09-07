# The hex threat enum — derivation memo, revision 3.

> **ROUND 3 OF FOUR, UNDER THE OPERATOR GRANT D-709, AND THE STOP IS
> SUPERSEDED.** Revision 1 FAILED its REVIEW-design
> (`hex_threat_enum_v1_REVIEW.md`: 3 BLOCKING, 7 MAJOR, 8 MINOR); revision 2
> FAILED its scoped confirmation (`hex_threat_enum_v1_CONFIRM.md`: 1 new
> BLOCKING, 5 new MAJOR, B-3 still landing), and at the dispatch's one-fix-round
> cap that was STOP E (`wp22_phase2a_STOP_E.md`). The operator then granted *"up
> to 4 rounds"*, which D-709 reads as four per review gate, so **rounds 3 and 4
> are available and this is round 3. It is REMEDIES-ONLY (D-709), and it adds no
> scope.**
>
> **THE ONE REMEDY THAT IS NOT A CORRECTION OF WORDS**: §6.5's criterion was
> passed at 1.77x by a value-free quotient of stone counts, so it is REPLACED
> rather than reworded. The stone-count quotient is now the registered referent —
> the strongest partition that knows nothing the calculus names — the random
> permutation is demoted to a floor, and the raw code is labelled the identity it
> always was. **The criterion can now fail, and §7.4 reports whether it does.**
>
> `wp22_phase2a_STOP_E.md` is not deleted: it is the record of what two rounds
> found, and its finding list is this revision's input.

**Governing revision**: `c5123c1` (`dev`), the revision every file, line and count
quoted below was read at (D-692). **REVISION 2 IS THE ONE FIX ROUND** granted
against `hex_threat_enum_v1_REVIEW.md`'s FAIL (3 BLOCKING, 7 MAJOR, 8 MINOR at
stash `ecfc2d8e`); §10 records what each finding changed, and every remedy was
DERIVED and RUN before this revision was written (D-591), never transcribed from
the reviewer's sentence.

**NOTHING IN §7 IS MEASURED YET.** Every number in §1 through §6 is a quotation,
a DERIVED count with its arithmetic on the page, or a bound — except where a
paragraph is explicitly marked MEASURED because a finding could only be answered
by running something. The class count `k`, the observation census, the purity
terms and the merge curve land in §7 only after §6's instrument has run.

**What this document is for.** `docs/research/eval_families_2026-09.md` §7 lists
`R-A4-CLASS` as *"the only row that is covering (§0.1) and densely evidenced at
once"* and states its kill in the same breath: *"the hex enum cannot be derived
without importing 4-axis thresholds, which `threat_calculus_v1.md`'s scope rule
forbids."* §8 of the same document records that the enum *"does not exist"*.
This memo defines it so that it can be COMPUTED rather than written; D-706 binds
the definition, and §1.5 records what that ADR's provenance is and is not.

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
is `C(k+2, 3)` at the `k` §6 computes, whatever that turns out to be.

**THE FIRST REVIEW MEASURED THAT AN EARLIER REVISION RE-DERIVED 816 BY ANOTHER
ROUTE, AND THAT IS WHY §6.4's BOUNDARIES ARE NOW STATED AS THE CALCULUS'S OWN.**
Revision 1's coarsest ladder rung clipped the completion cost at "≥ 4", which
leaves four values a side and therefore **`k ≤ 16` and `C(18,3) = 816` before any
enumeration** — the imported figure arriving as a consequence of a chosen clip.
§6.4's rungs now take their boundaries from `LAW-SUPPORT`'s own `k = 1` and
`k = 2` readings and from `DEF-WINDOW`'s dead predicate, and from nothing else;
§5.5 records that a PROJECTION has boundaries where the TUPLE has none, so the
two claims do not collide.

### 1.2 The calculus definitions the tuple is built from

All at `docs/research/threat_calculus_v1.md`, revision v1.0, read at `c5123c1`.

| ID | line | what the tuple takes from it |
|---|---|---|
| `DEF-WINDOW` | `:28` | *"6 consecutive cells on one axis. **Open** for side X iff no opponent stone in it. Overline monotonicity ⇒ 6-windows suffice for all theory."* The unit the tuple counts over, and the live/dead predicate. |
| `DEF-PLAN` | `:29` | *"the empty-cell set (size ≤2) of an open window with ≥4 own stones"* — the object §5.5's `t` witness is computed over. |
| `DEF-T` | `:30` | *"exact **minimum hitting set** over plan family F"* — the quantity §5.5 measures the tuple against. |
| `LEM-AXES` | `:37` | *"3 axes/cell (vs 4 square)"* — why a cell's code is a 3-multiset and not a 4-tuple. |
| `LEM-CROSS` | `:38` | *"two lines on distinct axes meet in ≤1 cell"* — why the three axes meet only at the cell. |
| `LEM-MONO` | `:40` | *"stones are never removed; an own extra stone never hurts"* — why the completion cost is monotone. |
| `LAW-HIT` | `:45-46` | *"One defender stone in any empty cell of a window kills that window permanently."* Why a non-open window can never be completed, hence why `open_X` counts surviving completions. |
| `RULE-EXACT` | `:64-66` | *"t is computed exactly … never read from a pattern table and never derived by weight algebra. Patterns may order moves; only exact t decides truth."* The discipline every component obeys — and, in §5.5, the standard the tuple is measured against and does not meet. |
| `LAW-SUPPORT` | `:68-71` | *"A forced win in k own turns requires an open window already holding ≥ 6−2k own stones."* The stones-still-needed arithmetic, and §6.4's two boundaries. |
| `LAW-OVERLOAD` | `:55-62` | *"t ≥ 3 for the attacker + defender cannot win this turn ⇒ defender lost"*, with its addition floor. §5.5 records that `t ≥ 3` is not reachable on one axis. |
| `LAW-DECOMP` | `:87-90` | *"Regions with disjoint stars: t is additive … **The same additivity is FALSE for static evaluation — never sum regional eval as if independent.**"* §8's standing limitation of the whole additive family, NOT support for it. |
| `THM-WINDOW` | `:152-155` | *"length-6 occupancy tables cannot classify live vs dead fours (needs 8-cell context; fives need 7) … **re-derive the minimal sufficient hex length by enumeration.**"* §6.6 states exactly which half of this the construction can answer and which it cannot. |
| `E-PHASE` | `:156-157` | *"win threshold differs by intra-turn phase (phase 2: plan ≤1 wins)"* — why rule-4 relevance is named and why phase conditioning is out of scope (§9). |
| `PROTO-NODE` step 1 | `:128` | *"**Win-now check.** A completing stone ends the turn instantly (second stone unplayed)."* Game rule 4, in the calculus's own words. |

### 1.3 The query sites, quoted at `d83ac01` and re-quoted at HEAD

`crates/pistol-solver/src/query.rs:80-142`. **MEASURED at `c5123c1`**:
`diff <(git show d83ac01:crates/pistol-solver/src/query.rs) crates/pistol-solver/src/query.rs`
returns nothing — the file is BYTE-IDENTICAL at `d83ac01` and at HEAD, so the
re-quotation is the same quotation. Line 81 is `pub enum LiveCount {` and line
142 is `    pub fn live_windows_at_count(&self, side: Player, count: LiveCount) -> &[Window] {`
at both revisions.

What the range holds, and it is exactly the vocabulary the tuple re-uses:

- `LiveCount` (`:80-86`) — *"Exactly two own stones, no opponent stone"* /
  *"Exactly three own stones, no opponent stone"*: a LIVE window at an exact own
  count.
- `NearHot` (`:88-99`) — *"Hot is `own >= 4`, so exactly one count — three — is
  one stone away from it"*: the one-stone-short reading.
- `WinWitness` (`:101-123`) — `OnePly` *"One stone completes a window already
  holding five. Valid at either budget."* and `Pair` *"Two stones complete a
  window holding four. Valid ONLY at `StonesLeft::Two`"*. The tuple's rule-4
  component is `OnePly`'s condition, and `Pair`'s is `min = 2`.
- `hot_windows` / `win_in_one_ply_windows` / `completed_windows` (`:127`, `:132`,
  `:137`) — the three shipped classes, all thresholds on the same two quantities
  the tuple carries continuously. `HOT_MIN = 4` is asserted at
  `crates/pistol-solver/src/sets.rs:27-31` and not in `query.rs`, which is where
  a reader checking the threshold has to go.

**THE POINT OF QUOTING THE SITE IS THAT THE TUPLE IMPORTS NOTHING FROM IT
EITHER.** `Class::Hot`'s `own >= 4` is a THRESHOLD, and a threshold is what
D-706 forbids taking on faith; the tuple carries `min` and `open` at full
resolution. That the shipped solver's own window predicates are unions of tuple
classes is §6.7 self-check 5, run rather than assumed.

### 1.4 The corpus, and the receipts that hold its counts

- **89 805 deduped positions.** MEASURED at `c5123c1`:
  `/usr/bin/grep -vc "^#" artifacts/arc3r_sweep_deduped_manifest.txt` → `89805`;
  `sha256sum` of that manifest is
  `00f61780cc1654958696786051dbd8de7d1bbab3f5d49153b1694770caf35968`, which is
  the digest `docs/book_v3_ledger.md:50` quotes in its disjointness receipt.
- **74 672 `eval` rows; 45 271 quiet (60.63 %)** — `wp22_phase2_premise.md` §2,
  under R6's filter (D-622): *"no length-6 window holds four or more stones of
  one side and none of the other"*.
- **3 487 distinct games, 25.75 positions each, median 23** —
  `training_pipeline_2026-09.md` §1, instrument
  `artifacts/research_2026-09/game_count.txt`; independently confirmed by
  `docs/book_v3_ledger.md:51`'s disjointness control line, `control: corpus5 ^
  v2: 3487 of 3487`.
- **AND ALL THREE ARE RE-DERIVED BY THIS PACKAGE'S OWN WALK.**
  `tools/hex_enum/seed_pilot.py`, written from scratch against the record
  grammar and sharing no code with the census, reports over the same manifest:
  *"walked 45271 quiet rows over 3487 games, 38983 distinct folded L11 codes"* —
  the premise memo's 45 271, the pipeline's 3 487, and `eval_families` §0.2's
  38 983 folded L11 cells, from one independent pass.
- **The research receipt verifies.** MEASURED at `c5123c1`: `sha256sum -c
  artifacts/research_2026-09/RECEIPT_research_2026-09.sha256` → **7 of 7 OK**,
  and the receipt's own digest is
  `13fe5712dafa94e2916267800f7b4162db380e27b03a16e00fd97f0981916dab`, equal to
  the figure `training_pipeline_2026-09.md` §9 states.
- **The census this memo's instrument re-points**, from `eval_families_2026-09.md`
  §0.2, folded columns, over the 45 271-row quiet population:

  | L | folded cells | median obs/cell | cells < 10 obs | cells covering 90 % |
  |---|---|---|---|---|
  | 7 | 1 029 | 1 282 | 0 (0.0 %) | 190 |
  | 9 | 8 374 | 56 | 1 079 (12.9 %) | 434 |
  | 11 | 38 983 | 7 | 22 142 (56.8 %) | 810 |

  **That census counts WINDOWS by `(axis, start)`. This memo's does not** — §4
  explains why a class enum is indexed by CELL, and §6.3 records that the two
  populations are different and are not compared cell for cell. §0.2's `L = 8`
  row is omitted here because §3 refuses even lengths and a row nothing in this
  document can use would only invite the comparison §6.3 forbids.

### 1.5 D-706 and this memo are ONE ACT, and the memo says so rather than citing itself

D-706 was appended in the same session that wrote revision 1 and landed at
`d73deba`, one commit before the memo's own `c5123c1`. **It is therefore not an
external premise**, and this memo's conformity to it is not evidence of
anything: the ADR contains this document's own conclusions, including
*"reversal invariance is by construction"* and the receipt list §6 produces.
What IS external, and what the kill condition actually rests on, is
`threat_calculus_v1.md:12-13` (§1.1) and `eval_families_2026-09.md` §7's stated
kill — both of which predate this package. D-706 records the ruling; it does not
license it. The first review found this and it is fixed by saying it, not by
moving a citation.

### 1.6 The game rules the tuple depends on, quoted from CLAUDE.md

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
have no names, and `k(L)` is an output of enumerating the tuple over every
pattern, never a number this memo chooses.

---

## §3 The pattern space, and it is defined at ODD lengths only

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
constant and the code has `L − 1` ternary positions, `3^(L−1)` in all.
`eval_families_2026-09.md` §A4 records that Rapfi does the same — *"the centre
cell is never encoded, being the candidate placement"* — and that is a
CONFIRMATION read off a 4-axis engine, not the ground.

### 3.1 EVEN `L` IS REFUSED BY NAME, and the ground is a measurement

Revision 1 defined an even-`L` class as the unordered pair of the two
alignments. **MEASURED, and this is why the convention is deleted rather than
repaired**: two faithful readings of that one sentence give different enums at
every even length, because the two alignments are two different sets of board
cells whose union is `L+1` cells, while a single fixed `L`-cell window with both
middle cells empty is a space of `3^(L−2)`.

| even `L` | both centre cells empty, `L` cells | the two alignments' union, `L+1` cells |
|---|---|---|
| 6 | **k = 10** | **k = 33** |
| 8 | **k = 43** | **k = 177** |
| 10 | **k = 182** | **k = 920** |

Instrument: the enumeration in
`/tmp/…/scratchpad/derive_fixes.py`, whose two functions are the two readings
side by side; the shipped `tools/hex_enum/hexenum.py` implemented the first and
the first review's independent implementation the second. Neither matches the
`3^(L−1)` an earlier revision claimed for every `L`.

**So `centres()` now raises a named error at even `L`** (CLAUDE.md rule 3), the
enum is defined at odd lengths only, and §6.1 enumerates `L ∈ {7, 9, 11, 13}`.
**THIS IS A DEPARTURE FROM THE DISPATCH'S LITERAL LENGTH SET, WHICH NAMED 8, AND
IT IS RECORDED AS ONE**: `L = 8` is not a length at which a CELL-centred class
exists, and reporting one under a convention two readers read two ways is the
under-specification hard rule 3 forbids. `eval_families` §0.2's `L = 8` window
census is a different object and §6.3 says so.

---

## §4 Why the index is a CELL, which cells, and what that does not settle

`R-A4-CLASS` is scored *"on the 3-axis multiset"* with `C(k+2, 3)` parameters,
and `C(k+2, 3)` is the count of size-3 multisets over `k` classes. A cell has
exactly one length-`L` pattern per axis and therefore exactly three classes; a
window has one axis and one class. **`C(k+2, 3)` is a per-CELL object and the
row is only well posed per cell.**

**Which cells, and the restriction is a WELL-DEFINEDNESS requirement rather than
a convenience.** A cell is SCORED iff it is empty and at least one of its three
length-`L` patterns holds a stone. On an unbounded board (rule 1) the cells
outside that set are infinite in number and all take the same all-empty code, so
an additive per-cell sum over them converges only if that code's weight is
exactly zero — the restriction is what makes the sum a number at all, and it is
also the census's own *"holding at least one stone"* rule transposed from
windows to cells.

**§6.3 reports the size of rule 5's legal region beside it, and the two are NOT
two populations for one object.** The legal region is a 2-D disc of 217 cells
per stone; the scored set is a union of three length-`L` segments, 3(L−1) cells
per stone before overlap. Scoring the disc would make the evaluation depend on
the size of the legal region, which carries no line content at all. The second
count is reported so a matrix can see what the scored set excludes, and for
nothing else.

**AND A WINDOW NOT CONTAINING `c` IS NOT `c`'s BUSINESS — a convention that
BINDS NOTHING at every length this memo headlines.** DERIVED: 6-windows lying
inside a length-`L` pattern but missing the centre number `L − 5 − min(6, L−5)`,
which is **0 at `L = 7`, `9` and `11`** and first becomes non-zero at `L = 13`
(2 of 8). The convention is stated because it is load-bearing at `L = 13`; at
`L = 11` it forbids nothing, and revision 1 presented it as a substantive choice
where it was not.

**WHAT §4 DOES NOT SETTLE, and the first review was right to ask.** The scored
CELL is the census's unit and the enum's index. It is **not** a claim about what
a deployed evaluator sums over: `handcrafted_v0` sums 18 window contributions
per STONE (`eval_families` §A6), and whether a codebook backend sums over empty
cells, over stones, or over windows is Phase 2b's, behind the `Eval` trait the
premise memo §4 already fixes. **The matrix must name the summand per row**, and
must not compare a row priced per cell against one priced per window as though
the observation counts were the same currency.

---

## §5 The tuple

Let `W_L(c, a)` be the set of 6-windows that contain `c` and lie wholly inside
`P_L(c, a)`. **DERIVED**, by counting starts: `|W_L| = min(6, L − 5)`, so
`|W_7| = 2`, `|W_9| = 4`, `|W_11| = 6`, and `|W_L| = 6` for every `L ≥ 11` — the
six windows `c−5…c` through `c…c+5`. A window through `c` spans at most
`c−5 … c+5`, which is `2·6 − 1 = 11` cells; that is `eval_families_2026-09.md`
§0.1's covering bound, re-derived here from `DEF-WINDOW` alone.

For each side `X ∈ {own, opp}` (mover-relative):

- **`open_X`** = `|{w ∈ W_L(c, a) : w holds no stone of the OTHER side}|`.
  `DEF-WINDOW`'s *"Open for side X iff no opponent stone in it"*, counted, and
  it is the same predicate for both sides: `open_opp` counts the windows through
  `c` holding no OWN stone. Range `0 … |W_L|`. This is the "count of distinct
  completing windows, live vs dead" component: a window that is not open for `X`
  can never be completed by `X` (`LAW-HIT`), so `open_X` is exactly the count of
  `X`'s surviving completions through `c`.
- **`min_X`** = `min over w open for X of (6 − |X's stones in w|)`, and `∞` when
  `open_X = 0`. The exact number of further `X` stones needed to fill some
  window through `c`, `c` itself among them since `c` is empty and is a cell
  either side may take. Range `1 … 6`, or `∞`. This is `LAW-SUPPORT`'s
  *"≥ 6−2k own stones"* read at one stone per unit instead of two per turn.
- **`r4_X`** = `[min_X = 1]`. Game rule 4 / `PROTO-NODE` step 1: one placed
  stone completes six and the turn ends with its second stone unplayed.

**The class of `P_L(c, a)` is the 6-tuple**
`(min_own, open_own, r4_own, min_opp, open_opp, r4_opp)`.

### 5.1 `r4` IS REDUNDANT AND IS KEPT ANYWAY, AND THIS MEMO SAYS SO FIRST

`r4_X ⟺ min_X = 1` by definition, so the component induces **no refinement at
all**. §6.7 self-check 2 computes `k` both ways and the sets — not merely the
counts — must be identical. It is named in the tuple because `E-PHASE`
conditions the win threshold on the intra-turn phase and `min = 1` is the only
cost a phase-2 mover can pay. **A component that changes no class may not be
used to claim the tuple is richer than it is**, and §6.4's rungs are held to the
same rule: revision 1's `T2` carried an alive-or-dead boolean beside `min`, and
since `open_X = 0 ⟺ min_X = ∞` that boolean refined nothing — MEASURED, `k = 47`
at `L = 11` with it and without it — so it is deleted.

### 5.2 The tuple's range, DERIVED, and the cubic AT that range

`min_X ∈ {1,…,6, ∞}` — 7 values; `min_X = 6` is an all-empty open window and
`min_X ≥ 1` because `c` is empty and must itself be filled; `|X| = 6` cannot
occur in a window containing the empty `c`. `open_X ∈ {0,…,|W_L|}`. The two are
linked: `open_X = 0 ⟺ min_X = ∞`. So per side at `L ≥ 11` there are at most
`1 + 6·6 = 37` combinations, and at most **`37² = 1 369`** joint ones.

**AND THE CUBIC APPLIED AT THIS DOCUMENT'S OWN BOUND, which revision 1 omitted
and which is the number a reader most needs**: `C(1369+2, 3) =
1371·1370·1369/6 = **428 558 605**`. Beside it, `C(k+2,3)` at 16 is 816, at 40
is 11 480, at 100 is 171 700. **The bound is three orders of magnitude past the
largest exemplar**, so a reader who carries only the exemplars away carries the
imported figure. `k` is an output and `C(k+2,3)` is cubic in it; nothing else
about the row's affordability is derivable before §6.1 runs.

### 5.3 Reversal invariance, BY CONSTRUCTION

Reversing `P_L(c, a)` about `c` permutes `W_L(c, a)` and maps each window to a
window with the same per-side stone counts. Every component of the tuple is a
count or a minimum over that set, so the tuple is invariant and the class is
invariant. **This is a priori and needs no measurement** — where
`eval_families_2026-09.md` §0.4 had to MEASURE fold-invariance for raw codes
(0 of 2 200 folded images differ, 1 760 of 2 200 unfolded do), a class enum
inherits it from the tuple's shape. §6.7 checks it anyway, because an invariance
claimed and not checked is a claim.

**The tuple is EQUIVARIANT under the own/opp swap as well**, which revision 1
neither claimed nor used: swapping the two sides' readings swaps the tuple's two
halves. MEASURED at `L = 11`, 0 mismatches over all 59 049 patterns, **17 of the
357 classes swap-fixed and 187 orbits**. A mover-antisymmetric evaluator ties
`w(class) = −w(swap(class))` and would halve the free parameters at zero
inference cost. It is recorded, not adopted: it is an evaluator decision and §9
defers those.

### 5.4 What the tuple deliberately does NOT carry

- **A threshold.** `Class::Hot`'s `own >= 4`, `WinInOnePly`'s `= 5`, Rapfi's 16,
  the survey's 13: none appears in the tuple. §6.4's PROJECTIONS do have
  boundaries — a projection is a choice of resolution and cannot not have them —
  and every one of them is `LAW-SUPPORT`'s or `DEF-WINDOW`'s, cited at the rung.
- **A name.** No class is called a four, a three, a gap trap or a rhombus.
  `PAT-GAP` (`threat_calculus_v1.md:104`) is the reason: *"Absent from named-shape
  taxonomies; tables mis-score it"*.
- **The intra-turn phase**, and the mover's stone budget. See §9.
- **Anything off the axis.** The tuple is single-axis by construction; the three
  axes meet only in the cell's multiset (`LEM-CROSS`, `:38`).

### 5.5 WHAT THE TUPLE GETS WRONG, ON THE PAGE BEFORE §7 RUNS

`RULE-EXACT` (`:64-66`) says *"only exact `t` decides truth"*. **MEASURED, and
the POPULATION is stated first because an earlier revision published the count
without it**: over the length-11 patterns for which a stone at `c` does NOT
complete a six — game rule 4 ends the turn there, so no threat question survives
the placement — the exact `DEF-T` over own's plan family after placing one own
stone at `c`, a minimum hitting set over `DEF-PLAN`'s open windows at 4 or 5 own
stones, **differs within 27 of the 335 classes those patterns present, covering
5 348 of the 57 996 patterns.** **Without the rule-4 exclusion it is 32 classes
of 357 and 6 099 of 59 049**, and both numbers are printed here because the
confirming reviewer found the first published alone. Two witnesses, `.` empty,
`X` own, `c` at index 5:

```
class (min_own=3, open_own=6, r4_own=0, min_opp=6, open_opp=1, r4_opp=0)
   ......X..XX   place X at c ->  .....XX..XX   t = 1
   ......XXX..   place X at c ->  .....XXXX..   t = 2

class (min_own=2, open_own=6, r4_own=0, min_opp=6, open_opp=1, r4_opp=0)
   ......X.XXX   place X at c ->  .....XX.XXX   t = 1
   ......XXX.X   place X at c ->  .....XXXX.X   t = 2
```

`t = 1` against `t = 2` is exactly the calculus §5 table's own split — `PAT-C4`
*"single plan"* at `t = 1` against `PAT-O4`, `PAT-O5` and `PAT-GAP` at `t = 2` —
and `LAW-OVERLOAD` makes `t` the criterion for a forced win. **So the tuple
assigns one weight to a cell that buys a hittable single plan and to a cell that
buys an unhittable pair of them, in the simplest four-shapes the game has.**
Since a cell's code is built from per-axis classes, the merge survives to the
code.

**THIS IS RECORDED AND NOT REPAIRED, AND THE REASON IS A PRICE RATHER THAN AN
IMPOSSIBILITY.** An earlier revision said the merge was *"not something a
single-axis class can hold"*, and that is FALSE: single-axis exact `t` is a
function of the eleven cells, as the next paragraph's enumeration over all
`3^11` lines demonstrates by computing it. **MEASURED, the price of carrying it
as a seventh component**, at `L = 11` under the reading above:

| rung | `k` without | `k` with | `C(k+2,3)` without | with | classes merging different `t` before |
|---|---|---|---|---|---|
| T4 | 357 | **384** | 7 647 059 | 9 511 040 | 27 |
| T3 | 121 | 132 | 302 621 | 392 084 | 11 |
| **T2** | **47** | **58** | **18 424** | **34 220** | 11 |
| T1 | 36 | 45 | 8 436 | 16 215 | 9 |

**The repair removes the merge entirely, and at T2 it costs 11 classes and 86 %
more parameters.** This memo does not adopt it — §9 defers evaluator decisions
and the choice belongs to whoever prices the row — but it states the price
instead of an impossibility. **What genuinely cannot be held single-axis is the
POSITION's `t`**, whose `LAW-OVERLOAD` addition floor is a cross-axis statement,
and that is a true claim about a different object. What a single axis CAN see is
bounded: **MEASURED,
the maximum exact `DEF-T` over all `3^11 = 177 147` length-11 single-axis lines
is 2**, realised at `.....XXXX..`, with the distribution `t=0: 319 606, t=1:
32 002, t=2: 2 686` over the 354 294 (line, side) pairs. `LAW-OVERLOAD`'s `t ≥ 3`
is unreachable on one axis. **The matrix prices `R-A4-CLASS` knowing this**, and
§8's `LAW-DECOMP` limitation is the same fact from the other side.

---

## §6 What the instrument computes, and the instrument is named with its revision

**Instrument**: `tools/hex_enum/hexenum.py` (the tuple, the class table, the
ladder, the refinement analysis), `tools/hex_enum/census.py` and
`tools/hex_enum/report.py` (the corpus walk and its report), driven by
`tools/hex_enum/test_hex_enum.py` and `test_census.py` through
`tools/hex_enum_tests.sh`, which is CI gate 19. `docs/process.md`'s *"Instrument
governing revision"* binds: a change to any of them reopens this memo's review.

### 6.1 `k(L)`, by exhaustive enumeration, at odd lengths

For each `L ∈ {7, 9, 11, 13}`, enumerate all `3^(L−1)` patterns, compute the
tuple, count distinct tuples. `3^6 = 729` at `L = 7` through `3^12 = 531 441` at
`L = 13`: exhaustive, no sampling, no seed. §3.1 says why the set is odd-only.

### 6.2 The code count

`C(k(L) + 2, 3)`, DERIVED. Reported beside the raw-code ceiling for the folded
single-axis code with the centre dropped, `(3^(L−1) + 3^⌈(L−1)/2⌉)/2` —
`eval_families_2026-09.md` §0.2's form at `L−1` ternary positions, without its
`−1`, because that document's `−1` excludes the all-empty WINDOW and a single
axis of a scored CELL may legitimately be empty (§4). Exact at odd `L` by
enumeration: 378, 3 321, 29 646 at `L = 7, 9, 11`.

### 6.3 Observations per code, on the quiet population

The R6 population — 45 271 quiet `eval` rows — walked position by position. For
each position: the scored cell set of §4, each cell's three axis classes, the
sorted triple as one code, one observation per (position, cell). Reported at
BOTH population units — per (cell, axis) window class and per cell code — with
total observations, distinct codes, mean and median per code, the count covering
90 % of observations, and the counts under three published density floors.

**THE FLOORS ARE BURO'S AND THEY CARRY HIS TAG.** `eval_families_2026-09.md`
§A1 quotes them: *"If the count is small (say ≤ 4)… the weight is set to 0"* and
*"sufficiently high (say ≥ 20)"* are rules of thumb, **ARCH**; *"keeps only
configurations seen in ≥ 75 of ~11 M positions"* is a figure from a
square-board engine's own generator and is **EMP, NON-TRANSFERABLE** in its own
source's scheme. All three are reported because a reader wants the shape of the
distribution; the third is a landmark, not a criterion, and the scope rule
(`:12-13`) is why it is tagged rather than used.

A growth curve every 5 000 positions, as the committed census prints one, so
saturation is visible rather than asserted. And the size of rule 5's legal empty
region, sampled by a stated stride — §4 says what that second count is and is
not for.

**These counts are NOT comparable cell-for-cell with `eval_families_2026-09.md`
§0.2**, whose unit is a window at `(axis, start)` and whose population is windows
holding a stone. That document already warns that *"a centred-window codebook
indexes the same space differently, so these counts are indicative rather than
exact for that variant"*, and this memo does not compare them.

### 6.4 The coarsening ladder, with the calculus's own boundaries

Because `C(k+2, 3)` is cubic in `k` (§5.2), the instrument reports the same
statistics for four PROJECTIONS of the same tuple, each strictly coarser, each a
function of the one above it, and none of them a hand-written class list:

| rung | the equivalence | the boundary, and whose it is |
|---|---|---|
| **T4** | the full tuple | none — full resolution |
| **T3** | `(min_own, min(open_own, 2), min_opp, min(open_opp, 2))` | **A CHOICE, not a derivation.** `open` counts WINDOWS, and `DEF-T` counts hitting-set cells; the max-`t = 2` fact (§5.5) is about the second and licenses nothing about the first, which an earlier revision claimed it did. The clip is `{dead, one surviving completion, more than one}` because that is the coarsest split `DEF-WINDOW`'s own predicate can make on a count, and its cost is measured below. |
| **T2** | `(min_own, min_opp)` | none beyond dropping `open`; the alive-or-dead boolean revision 1 carried here refines nothing (§5.1). |
| **T1** | `(clip(min_own), clip(min_opp))`, `clip(x) = x` for `x ≤ 4`, `5` for `5 ≤ x < ∞`, `∞` for dead | **A CHOICE inside a constraint.** `LAW-SUPPORT` (`:69`) reads `own ≥ 6−2k`, so `k = 1 ⟺ cost ≤ 2` and `k = 2 ⟺ cost ≤ 4`; the law CONSTRAINS the clip to refine `{≤2 \| 3–4 \| ≥5 \| dead}` and does not determine it. The shipped clip keeps four cost values apart because the finer resolution is cheap at this rung, and that is a choice this memo makes, not one the law makes. |

**THE CLIP LADDER, EXHAUSTIVELY, BECAUSE TWO REVISIONS GOT ITS LICENCE WRONG.**
Revision 1 clipped at `min(cost, 4)`, which merged `cost = 4` — a `LAW-SUPPORT`
`k = 2` candidate — with 5 and 6, and folded `∞` in, so it could not state
either boundary and landed on `k = 16`. Revision 2 fixed the clip and claimed
the boundaries were *"expressible only if 1, 2, 3 and 4 stay apart"*, **and that
"only if" is false**. MEASURED at `L = 11`, every clip that expresses both
boundaries:

| clip | `k` | `C(k+2,3)` |
|---|---|---|
| **`{≤2 \| 3–4 \| ≥5 \| dead}` — the COARSEST faithful one** | **16** | **816** |
| `{1 \| 2 \| 3–4 \| ≥5 \| dead}` | 25 | 2 925 |
| `{≤2 \| 3 \| 4 \| ≥5 \| dead}` | 25 | 2 925 |
| `{1 \| 2 \| 3 \| 4 \| ≥5 \| dead}` — shipped | 36 | 8 436 |

**AND THE COARSEST FAITHFUL CLIP GIVES 816, WHICH IS THE FIGURE §1.1 CALLS AN
IMPORT — SO THE COINCIDENCE IS NAMED HERE RATHER THAN AVOIDED.** It arrives by a
route that has nothing to do with Rapfi: `LAW-SUPPORT`'s two readings plus
`DEF-WINDOW`'s dead give four cost buckets a side, `4² = 16` joint classes, and
`C(18,3) = 816`. That it equals `C(16+2,3)` for Rapfi's hand-designed 16-member
enum is a fact about the arithmetic of small multisets and not evidence that
either derivation informed the other. **D-706 forbids importing a number; it
does not forbid deriving one that coincides with an imported one**, and the
defence is that the derivation is printed above and can be checked without
reading a word about a 4-axis game.

**WHAT EACH RUNG COSTS IN `t`-CONSISTENCY, MEASURED**, since the clips are
choices and a choice should be priced. Over the 57 996 patterns whose placement
does not complete a six (§5.5's population):

| rung | classes merging different single-axis `t` | patterns in them |
|---|---|---|
| T4 | 27 | 5 348 |
| T3 | 11 | **7 056** |
| T2 | 11 | **11 164** |
| T1 | 9 | 11 164 |

**A coarser rung merges fewer CLASSES and more PATTERNS**, which is what
coarsening does and is the honest cost of the ladder. It is not an argument
against the coarse rungs — §7 measures that they are the only ones the corpus
supports — and it is the number a matrix should carry beside their parameter
counts.

**The ladder is not a selection**: §7 reports `k`, codes, observations and purity
at each rung and recommends nothing.

### 6.5 CLASS PURITY, and both terms are named (D-479)

The label is the corpus row's `score_value` at `SCORE_KIND = eval`
(`tools/texel/extract.py:24-26` names the columns), and it is MOVER-RELATIVE, as
`tools/texel/fit.py:119`'s `signed()` shows by swapping the per-side counts and
leaving the label alone. For each observation the label is its POSITION's label.
Over all observations, with classes `C`:

- **WITHIN-class variance** `= Σ_C (n_C / N) · Var(label | C)`.
- **BETWEEN-class variance** `= Σ_C (n_C / N) · (mean_C − mean)²`.
- Their sum is the total label variance, checked to machine precision.
- **`ω²` is the primary statistic and `η²` is printed beside it.**
  `η² = between/total` rises with the class count whatever the classes mean, and
  at `T4`'s code unit the class count is in the millions, so `η²` there would be
  near 1 by memorisation and would say nothing. `ω²` subtracts the between-group
  sum of squares a partition of that many groups earns from noise alone.

Reported at the WINDOW unit — one observation per `(position, cell, axis)` — and
at the CODE unit — one per `(position, cell)`.

**THREE REFERENTS, AND ONLY ONE OF THEM CAN FAIL.** The named defect class is
*the quotient throws away a distinction that carries value*.

1. **The un-quotiented folded raw code at the same `L` is an IDENTITY, not a
   test, and is labelled as one.** The tuple is a function of the pattern and is
   reversal-invariant, so the folded-code partition REFINES the class partition,
   and between-class variance is monotone under refinement — therefore
   `η²(raw) ≥ η²(tuple)` for every corpus, every tuple, whether or not anything
   of value was lost. It is reported as the CEILING the quotient is measured
   against, and its gap tracks the class-count ratio rather than the loss.
   It cannot fail and it is not offered as a criterion.
2. **The matched random quotient is a FLOOR and is reported as one.** It is
   matched on the enum's own SHAPE — the classes are re-assigned by permuting
   which codes belong to which class, so the class count and the number of codes
   per class are exactly the enum's and only the grouping changes. Three
   replicates at a recorded seed. **Clearing it says the enum is not noise and
   nothing more**, and revision 2 registered it as the criterion, which was the
   BLOCKING error the confirmation found: a quotient of stone counts alone,
   blind to openness, completion cost and game rule 4, cleared it at 1.77x and
   reached 96 % of the full tuple's `ω²` (`hex_threat_enum_v1_CONFIRM.md` N-1).

3. **THE STONE-COUNT QUOTIENT IS THE REFERENT THE CRITERION IS NOW STATED
   AGAINST**, and it is the one that can fail. Its key is
   `(own stones in the pattern, opp stones in the pattern)` and nothing else: no
   window, no openness, no completion cost, no rule-4 relevance, no position
   along the line. It is the strongest partition of the same code space that
   knows nothing the calculus names, so the gap between it and the enum is
   exactly what threat structure buys on this corpus. It shares the corpus, the
   walk and the moments code with the enum arm; `tools/hex_enum/census.py`'s
   `Length.count_key` computes it and `test_hex_enum.py` pins that it merges two
   patterns the tuple separates, one of which completes a six with a single
   stone.

**THE CRITERION, REGISTERED BEFORE THE RUN THAT GOVERNS IT, AND IT HAS NO FREE
PARAMETER.** At the WINDOW unit: **the enum's `ω²` must EXCEED the stone-count
quotient's, at the rung and length a matrix prices the row at.** No margin, no
threshold, no number that could be tuned after the fact — a direction, which is
the only form that cannot be moved post hoc.

**WHAT FAILING MEANS, registered too**: if the enum does not exceed the
stone-count quotient at a rung, then on this corpus, through this statistic, at
this unit, **threat structure adds nothing over counting stones**, and
`R-A4-CLASS` is UNPRICED at that rung — not merely ranked lower. A matrix may
then say what the census measured and may not rank the row on it.

**AND THE DISCLOSURE THAT MAKES THIS A REGISTRATION RATHER THAN A PREDICTION.**
The confirming reviewer already ran this comparison once, at `L = 7` over 3 000
positions, and got enum T4 `ω² = 0.005031` against the stone-count quotient's
**0.004840** — the enum ahead by **1.04x**. This session reproduced it at the
same scope with the shipped instrument before registering the criterion. **So
the criterion is registered knowing it is close to failing**, which is stated
here because a criterion registered in ignorance of the pilot and one registered
in knowledge of it are different objects, and a reader is entitled to know which
this is. What is NOT known at registration is whether the direction survives the
full 45 271-row population, the covering length, or the coarse rungs — and that
is what §7.4 reports.

**WHAT THIS CRITERION IS NOT.** It is not a strength gate — D-614 stands, no
offline number moves a config — and it does not select a rung, a length or a
row. It constrains what the matrix may CONCLUDE from §7, which is what a
criterion is for.

**THE HONEST CAVEAT, stated before the number exists.** Every observation of one
position carries that position's single label, so observations are massively
non-independent and `ω²` is a statement about how much of the LABEL variance the
class marginal explains on this corpus — not a fit, not a bound on a fit. The
effective `n` is nearer the 3 487 games than the observation count
(`training_pipeline_2026-09.md` §1). The matched referent is what makes the
comparison readable in spite of that, because it carries the same dependence.

### 6.6 `THM-WINDOW`: what the construction can answer, and what it cannot

**THE STABILISATION LENGTH IS DERIVED, NOT MEASURED, AND REVISION 1's
"discharges" IS WITHDRAWN.** §5's `|W_L| = min(6, L − 5)` fixes the tuple's reach
at exactly `c ± 5`. Below `L = 11` an odd length always omits at least one window
through `c`, so `L → L+2` always splits; at `L ≥ 11` no further cell can enter
any component, so it never splits. **The answer is 11 by a one-line argument, and
the enumeration can take no other value.** It is reported as a CONFIRMATION of
the derivation — `k(11) = k(13)` and the joint partition equal to both, at every
rung — and a matrix that read "stabilisation length = 11, MEASURED" would be
reading the definition back to itself.

**AND IT IS NOT THE QUESTION `THM-WINDOW` ASKED.** That result is
window-relative: a run of `n` own stones needs `n + 2(6−n)` cells to decide live
against dead — 8 for a four, 7 for a five — and `eval_families` §8 restates the
owed question as *"whether a shorter window still separates structures of
different value once summed over overlaps"*. Neither 7 nor 8 can appear in §7,
because every component of a cell-centred tuple reads a window that may start at
`c−5`. **§7 therefore does not close `THM-WINDOW`**; it answers a cell-centred
sufficiency question and says which one.

**THE MEASURED CONTENT IS THE MERGE CURVE BELOW 11, IT IS NOT A COARSENING, AND
IT IS MEASURED AND NOT DERIVED.** Neither `class_L` nor `class_11` refines the
other for `L < 11` — the shipped `--refine` mode prints *"neither refines the
other"* at `7 → 9` and `9 → 11` at every rung — because a short window asks about
fewer windows rather than asking more coarsely about the same ones. What §7
reports instead is the two-way fanout of the joint partition, and it is
**MEASURED by exhaustive enumeration**, not derived: an earlier revision tagged
it `DERIVED`, which in this document's own header means *"with its arithmetic on
the page"* and in §6.6's preceding paragraph means *"the enumeration can take no
other value"*. Neither is true of a fanout; the numbers are contingent facts
about hex.

**AND THE NAMED INSTRUMENT NOW PRODUCES THEM.** `refinement()` returns the
two-way `(min, max, mean)` partner counts, and `hexenum.py --curve <long>
<short>…` prints the join at an arbitrary step, so the `7 ↔ 11` and `9 ↔ 11`
curves a matrix quotes come out of the instrument this memo names rather than
out of a scratch script no review binds. Whether that fanout costs anything is
§6.5's question, not §6.6's.

### 6.7 Instrument self-checks, run before any reported number

1. **Reversal.** Every pattern's class equals its reverse's class, at every `L`
   enumerated (§5.3).
2. **`r4` redundancy.** The class SET with and without the two `r4` components is
   identical, not merely the same size (§5.1). A disagreement is an instrument
   fault.
3. **Law of total variance.** `within + between = total`, to the reported
   precision (§6.5).
4. **Window count.** `|W_L|` as enumerated equals the DERIVED `min(6, L − 5)`.
5. **Solver agreement, on a predicate that CAN fail.** `Hot` (a live window at
   `≥ 4` own) and `WinInOnePly` (a live window at exactly 5 own) are recomputed
   from the windows themselves and must be constant on every tuple class. The
   third shipped class, `Completed`, is NOT checked: every window through `c`
   contains `c`, which §3 fixes empty, so no pattern satisfies it at any `L ≤ 11`
   and the check would pass vacuously — §5.1's own standard, applied to §6.7.
6. **Even lengths refuse by name**, at 6, 8, 10 and 12 (§3.1).
7. **The ladder is a chain**: each rung is a function of the one above it, so
   `T1` cannot separate what `T2` merges.
8. **T1's clip states both `LAW-SUPPORT` boundaries** and does not land on 816
   (§6.4).
9. **A rung carries no component that refines nothing** — the `T2` boolean check
   (§5.1).
10. **The two variance terms are computed and not asserted.** `terms()` computes
    the total WITHOUT the partition — `Σx² − N·grand²` — and raises by name when
    the decomposition disagrees with it. An earlier revision defined
    `total = within + between`, which made self-check 3 compare a number to
    itself; the four mutants that survived that arrangement (`between` always
    zero, `ω²` always 1.0, both ratios 0.5, the mean correction dropped) are now
    DEAD (`artifacts/wp22_phase2a/n6_mutants/`).
11. **A partition that separates nothing earns nothing**: a two-class `Moments`
    whose class means are equal must report `between = 0`, `η² = 0` and
    `ω² ≤ 0`, and one whose means differ must report the hand-derived values.
12. **The stone-count referent is blind to what the tuple carries**: two
    patterns the tuple separates — one completing a six with a single stone,
    one three stones short — must share one count-only key.
13. **The merge curve comes out of `refinement()`**, one-to-one in both
    directions at `11 → 13` and fanning out at `7 ↔ 11`.

---

## §7 Outputs

**RUN, at round 3, with the stone-count referent §6.5 now registers.** Instrument
`tools/hex_enum/{hexenum,census,report}.py` at the revision that carries this
memo, four processes, one per length, over the whole 45 271-row quiet
population — no sampling, no seed except the random referent's, which is
recorded. Receipt `artifacts/wp22_phase2a/census_r3/`. **NO SELECTION** (D-708).

### 7.1 `k` and the code count — the enum is COMPUTED and no cell of it is 816

Exhaustive over `3^(L−1)` patterns at each length.

| L | T4 `k` / `C(k+2,3)` | T3 | T2 | T1 |
|---|---|---|---|---|
| 7 | **25** / 2 925 | 25 / 2 925 | 15 / 680 | 12 / 364 |
| 9 | **98** / 161 700 | 57 / 32 509 | 27 / 3 654 | 20 / 1 540 |
| 11 | **357** / **7 647 059** | 121 / 302 621 | 47 / 18 424 | 36 / 8 436 |
| 13 | **357** / 7 647 059 | 121 / 302 621 | 47 / 18 424 | 36 / 8 436 |

`T3 = T4` at `L = 7` because `|W_7| = 2` and `min(open, 2)` is the identity
there — a consistency signal, not a coincidence. §6.4's coarsest faithful clip
would give `k = 16` and 816, and is not shipped.

**THE FULL TUPLE AT THE COVERING LENGTH IS NOT AFFORDABLE.** `C(359,3) =
7 647 059` nominal parameters against 8 174 025 scored-cell observations is
**1.07 observations per nominal parameter**, where `eval_families_2026-09.md`
§A1 already kills `R-A1-L11` at 1.2 and Buro's safe-fit line is ≥ 20. **That
matters more than it looks**, because §7.4 measures that T4 is the only rung
worth having.

### 7.2 `THM-WINDOW`'s enumeration — the stabilisation is CONFIRMED, not measured

`k(13) = k(11) = 357` at every rung, and the joint partition of the two equals
both: `hexenum.py --refine 11` prints *"the longer partition is a function of the
shorter"* at all four rungs, with the fanout `1-1 (mean 1.0)` in both
directions. Below the covering length the relation holds in neither direction —
`--refine 7` and `--refine 9` print *"neither refines the other"* at every rung.

**This is the CONFIRMATION §6.6 registered it as**: the answer is 11 by §5's
one-line reach argument and the enumeration can take no other value. The corpus
agrees from the other side and more sharply: at `L = 13` the walk sees **232
distinct T4 classes against `L = 11`'s 231 and 108 075 distinct codes against
108 074** — one class and one code — for 18 % more window traffic per stone.

**THE MERGE CURVE BELOW 11, MEASURED** by `hexenum.py --curve 11 7 9` (§6.6):
one `L = 7` class meets 3 to 104 of the `L = 11` classes (mean 32.4) and one
`L = 11` class meets 1 to 8 `L = 7` classes (mean 2.3); at `L = 9`, 2 to 36
(mean 9.4) and 1 to 9 (mean 2.6).

### 7.3 Observations, at both population units

Scored cells per position: **111.1** at `L = 7`, **180.6** at `L = 11` and
`L = 13`. Rule 5's legal empty region, sampled every 500th position (n = 90):
**mean 459.2, median 451**.

**CODE unit** (one observation per scored cell), the row's own parameter unit:

| L | rung | nominal `C(k+2,3)` | codes OBSERVED | observations | mean | median | ≤4 | <20 | cover 90 % |
|---|---|---|---|---|---|---|---|---|---|
| 7 | T4 | 2 925 | 856 | 5 031 327 | 5 878 | 174 | 84 | 197 | 88 |
| 7 | T2 | 680 | 266 | 5 031 327 | 18 915 | 1 079 | 11 | 30 | 44 |
| 11 | T4 | 7 647 059 | **108 074** | 8 174 025 | 75.6 | **3** | 67 227 | 93 617 | 3 843 |
| 11 | T3 | 302 621 | 9 408 | 8 174 025 | 869 | 7 | 3 977 | 6 350 | 118 |
| 11 | **T2** | **18 424** | **1 533** | 8 174 025 | 5 332 | **41** | 322 | 620 | **32** |
| 11 | T1 | 8 436 | 535 | 8 174 025 | 15 279 | 53 | 118 | 204 | 4 |

**The growth curves separate the rungs cleanly** at `L = 11`, distinct codes
every 5 000 positions:

```
T4  34596 50952 63110 72652 81120 88713 95739 101982 107645   still climbing, steeply
T3   4803  6091  6918  7564  8024  8420  8781   9090   9377   still climbing
T2   1132  1285  1359  1406  1439  1463  1491   1511   1530   +35 % over 9x the data
T1    409   455   470   486   499   511   520    527    535   flat from 15 000 on
```

**Buro's floors, tagged as §6.3 tags them**: at `L = 11` T2, 322 of the 1 533
observed codes sit at or below his `≤ 4` line (**ARCH**) and 620 below his
`≥ 20` line (**ARCH**); at T4 those are 67 227 and 93 617 of 108 074.

### 7.4 THE REGISTERED CRITERION FIRES, AND IT FIRES ON EVERY AFFORDABLE RUNG

The criterion (§6.5): **the enum's `ω²` must EXCEED the stone-count quotient's,
at the WINDOW unit, at the rung and length a matrix prices the row at.** No
margin, no free parameter.

| L | rung | enum classes | enum `ω²` | count-only classes | count-only `ω²` | ratio | criterion |
|---|---|---|---|---|---|---|---|
| 7 | **T4** | 16 | 0.003573 | 23 | 0.003525 | **1.014** | MET |
| 7 | **T3** | 16 | 0.003573 | 23 | 0.003525 | **1.014** | MET |
| 7 | T2 | 10 | 0.002796 | 23 | 0.003525 | 0.793 | **FAILS** |
| 7 | T1 | 8 | 0.002417 | 23 | 0.003525 | 0.686 | **FAILS** |
| 9 | **T4** | 78 | 0.004606 | 38 | 0.004158 | **1.108** | MET |
| 9 | T3 | 45 | 0.003161 | 38 | 0.004158 | 0.760 | **FAILS** |
| 9 | T2 | 21 | 0.002799 | 38 | 0.004158 | 0.673 | **FAILS** |
| 9 | T1 | 15 | 0.002215 | 38 | 0.004158 | 0.533 | **FAILS** |
| 11 | **T4** | 231 | 0.005504 | 52 | 0.004894 | **1.125** | MET |
| 11 | T3 | 61 | 0.003187 | 52 | 0.004894 | 0.651 | **FAILS** |
| 11 | **T2** | 22 | 0.003119 | 52 | 0.004894 | **0.637** | **FAILS** |
| 11 | T1 | 16 | 0.002495 | 52 | 0.004894 | 0.510 | **FAILS** |
| 13 | T4 | 232 | 0.004668 | 69 | 0.005600 | 0.834 | **FAILS** |
| 13 | T3 | 62 | 0.002704 | 69 | 0.005600 | 0.483 | **FAILS** |
| 13 | T2 | 23 | 0.002646 | 69 | 0.005600 | 0.472 | **FAILS** |
| 13 | T1 | 16 | 0.002115 | 69 | 0.005600 | 0.378 | **FAILS** |

**MET IN 4 OF 16 CELLS. THE ONLY RUNG THAT EVER BEATS STONE COUNTING IS T4, THE
FULL TUPLE, AND ONLY BY 1.4 % TO 12.5 % — AND T4 IS THE RUNG §7.1 PRICES AT
1.07 OBSERVATIONS PER NOMINAL PARAMETER.** Every coarsening loses, and it loses
by a lot: `L = 11` T2, the densest affordable rung, sits at **0.637** of a
quotient that cannot see a window, an open window, a completion cost, or game
rule 4.

**SO THE CRITERION LEAVES `R-A4-CLASS` UNPRICED, AND THAT IS THE CRITERION
WORKING.** §6.5 registered what failing means before the run — *"threat
structure adds nothing over counting stones, and `R-A4-CLASS` is UNPRICED at
that rung — not merely ranked lower"* — and that is what the measurement says at
every rung a corpus of this size could fit.

**THREE THINGS THIS DOES NOT SAY, because the temptation to over-read a
decisive-looking table is the whole reason §6.5 was rewritten.**

1. **It does not say the enum is wrong.** Its arithmetic survived two
   independent re-derivations. It says the enum's classes do not separate this
   corpus's labels better than stone counts do, at this statistic and this unit.
2. **It does not say a threat-class eval cannot work.** `ω²` over
   position-level labels is a weak instrument: every observation of one position
   carries that position's single label, the effective `n` is nearer 3 487 games
   than 8 million cells, and a partition can carry decision-relevant structure
   that a marginal variance decomposition cannot see. What it does say is that
   **this corpus, through this instrument, cannot license the row**, and D-483
   forbids pricing on a number that is not there.
3. **It does not transfer to the WINDOW-indexed rows.** `R-A5-TOPK`,
   `R-A2-L11F` and `R-A3-L11F+F7` are not quotients of this tuple and are not
   measured here.

**AND THE FLOOR AND THE CEILING, reported as §6.5 labels them.** The matched
random quotient — the FLOOR — is cleared everywhere by 2.0x to 3.6x, which now
says only that the enum is not noise. The un-quotiented folded window code — the
CEILING, an identity — is 0.005171 at `L = 7`, 0.006837 at `L = 9`, 0.008842 at
`L = 11` and 0.010542 at `L = 13`.

**One cell is reported because it looks like the opposite and is not.** At the
CODE unit at `L = 11`, T4's `ω²` is 0.020985 against the count-only quotient's
0.009248 — 2.27x. **That comparison is between 108 074 observed classes and
6 856**, and `ω²`'s degrees-of-freedom correction does not make partitions two
orders of magnitude apart in class count comparable. The criterion binds the
window unit for exactly this reason, and at the window unit the class counts are
16-to-23, 78-to-38, 231-to-52 — the same order. The code-unit number is not
evidence for the row and is printed so that nobody finds it later and thinks it
was hidden.

### 7.5 What §7 does NOT report

No selection, no recommended rung, no recommended length, no strength claim.
`ω²` gates nothing (D-614). The criterion constrains what a matrix may CONCLUDE
about `R-A4-CLASS` and nothing else.

## §8 The standing limitation, recorded rather than buried

`LAW-DECOMP` says, verbatim: *"The same additivity is FALSE for static
evaluation — never sum regional eval as if independent."* **Every row in family
A is an additive table, this one included**, and a per-cell code summed over
cells is exactly regional eval summed as if independent. This is not an argument
against `R-A4-CLASS` specifically — it binds `R-A1`, `R-A2`, `R-A3` and `R-A5`
identically, and `handcrafted_v0` too, which sums 18 window contributions per
stone — but it is the calculus's own statement that the family's functional form
is known-wrong. §5.5's `t = 1` against `t = 2` merge is the same limitation
arriving one level down, inside a single axis.

---

## §9 What this memo does NOT establish

- **Nothing about which row wins**, which length is chosen, or which ladder rung
  is used. §7 reports; D-708 says selection is the architect's.
- **Nothing about the evaluator.** How a code's weight enters a score, what the
  summand is (§4), and what the accumulator's type is are Phase 2b's, behind the
  `Eval` trait the premise memo §4 already fixes.
- **Nothing about the intra-turn phase.** `E-PHASE` conditions win thresholds on
  the phase; conditioning the CODE on the phase would double `C(k+2, 3)`, and
  every corpus position is recorded at a turn boundary (D-621), so the corpus
  cannot price the conditioned variant at all.
- **Nothing about the own/opp antisymmetry** beyond §5.3's measurement of it.
- **`THM-WINDOW` is not closed** (§6.6).
- **Nothing about tactical value.** D-621 and D-622 foreclose learning it from
  these labels.
- **No strength claim, no Elo, no committed file.** Nothing here touches
  `configs/`.

---

## §10 What the two review rounds changed

Round 1: `hex_threat_enum_v1_REVIEW.md`, fresh context — **FAIL**, 3 BLOCKING,
7 MAJOR, 8 MINOR. Round 2: `hex_threat_enum_v1_CONFIRM.md`, a scoped
confirmation BY BEHAVIOUR of revision 2's fix round — **FAIL**, 1 new BLOCKING
and 5 new MAJOR, with B-3 still landing. Both reviewers wrote their own
implementation of the tuple and enumerated exhaustively; **neither found an
arithmetic error anywhere in the enum**. This revision is round 3 under D-709
and it is remedies-only.

### Round 1's findings, and where revision 2 left them

| finding | round 2's verdict on the fix |
|---|---|
| B-1 stabilisation length is not a measurement | CLOSED BUT MOVED → N-7 |
| B-2 even `L` under-specified | **CLOSED**, at the code, all seven entry points |
| B-3 T1's clip lands on 816 | **STILL LANDS** → N-4 |
| M-1 the `t = 1` / `t = 2` merge | CLOSED BUT MOVED → N-2, N-3 |
| M-2 the cubic at the memo's own bound | **CLOSED** |
| M-3 §6.5 registers no criterion | **STILL LANDS** → N-1 (BLOCKING) |
| M-4 T2's boolean refines nothing | **CLOSED** |
| M-5 T3's justification | CLOSED BUT MOVED → N-5 |
| M-6 D-706 and the memo are one act | **CLOSED** |
| M-7 *"discharges"* | **CLOSED** |
| m-1 … m-8 | all eight **CLOSED** |

### Round 3's remedies, each derived and RUN before this revision was written (D-591)

| finding | what changed, and what was run |
|---|---|
| **N-1** (BLOCKING) — the criterion is passed at 1.77x by a value-free quotient | **§6.5's criterion is REPLACED, not reworded.** The stone-count quotient is now the registered referent, the random permutation is demoted to a floor and labelled one, the raw code is labelled the identity it always was, and the criterion is a DIRECTION with no free parameter. RUN: `Length.count_key` added to the shipped census; four full-corpus runs; **§7.4 reports the criterion FIRING at 12 of 16 cells.** |
| **N-2** — the merge count's population was not stated | §5.5 states it — rule-4 completions excluded, **27 of 335 classes and 5 348 of 57 996 patterns** — and prints the number without the exclusion beside it, **32 of 357 and 6 099 of 59 049**. RUN: both populations enumerated. |
| **N-3** — *"not something a single-axis class can hold"* is false | deleted, and replaced by the MEASURED price per rung: T4 `k` 357 → 384, T2 47 → 58 (+86 % parameters). RUN: the seventh component enumerated at every rung. |
| **N-4** (B-3 still landing) — T1's *"only if"* is false | deleted. All four clips expressing both `LAW-SUPPORT` boundaries are enumerated, the coarsest gives `k = 16` and `C(18,3) = 816`, and §6.4 **names the coincidence with the imported figure and shows the derivation that makes it independent** rather than avoiding the number. |
| **N-5** — T3's clip is not licensed by max-`t = 2` | the justification is deleted and the clip is stated as a choice, with its MEASURED cost: T4 27 classes / 5 348 patterns `t`-inconsistent, T3 11 / 7 056, T2 11 / 11 164. |
| **N-6** (MAJOR) — gate 19 could not see a constant `ω²` | **fixed at the code**: `terms()` computes the total WITHOUT the partition and raises by name on a disagreement; the suite gains a unit test of the statistic with a control. **MEASURED: all four surviving mutants are now DEAD** (`artifacts/wp22_phase2a/n6_mutants/`). |
| **N-7** — the merge curve is tagged DERIVED and no named instrument produces it | retagged MEASURED, with the two tag meanings distinguished on the page; `refinement()` now returns the two-way fanout and `hexenum.py --curve` prints the join at an arbitrary step, so the published curve comes out of the named instrument. |
| **N-8** — the null is matched on the code space, not the observed population | conceded: §6.5 now demotes it to a FLOOR whose only claim is *"the enum is not noise"*, which is exactly as much as a shape-matched permutation can support. |
| **N-9 … N-12** | D-706's residual mentions do no work and stay (D-424); the criterion's unit is named in the criterion itself and §7.4 prints the code-unit cell with its caveat; stale even-`L` prose gone; §7 now lands with the round that ran it. |

**THE ONE FINDING THAT IS NOT CLOSED AND IS NOT CLOSEABLE HERE.** N-1's remedy
does not rescue the row — it kills it. §7.4 is the measurement the criterion was
rewritten to make possible, and it says the affordable rungs are worse than
counting stones. **That is the finding this package delivers**, and it is worth
more than the row would have been.

**Q1** is answered in §4's last paragraph, **Q2** in §6.6's last, **Q3** by
§6.5's registered criterion and §7.4's verdict, **Q4** by §5.2, **Q5** by §5's
`open_X` bullet.
