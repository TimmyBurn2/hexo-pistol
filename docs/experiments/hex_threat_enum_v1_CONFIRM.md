# CONFIRMATION — `docs/experiments/hex_threat_enum_v1.md` revision 2 — **FAIL**

Scoped confirmation by behaviour (D-686, D-691) of the one fix round granted against
`hex_threat_enum_v1_REVIEW.md` (FAIL: 3 BLOCKING, 7 MAJOR, 8 MINOR). Fresh context; not
the author, not the first reviewer.

## Header

**Named revision under confirmation**: `54eb3ba31fbfbc51ced96bb19604be56220d9e2b` (`dev`),
*"docs(enum): revision 2 refuses even lengths, re-derives the clip off LAW-SUPPORT, and puts
the t=1/t=2 merge on the page"*.

**Does it still match HEAD? NO — THE TREE MOVED DURING THIS CONFIRMATION, AND ONE HALF OF
THE MOVE IS LOAD-BEARING.**

- At the start: `git rev-parse HEAD` → `54eb3ba…`, `git status --porcelain` → clean.
- At the end: HEAD is `0f0cb44a1974b69d551fd1d1cbd25734d148ade2`, reached by three commits —
  `02ad9fb` (*"stage E's receipts land — k is 357 at the covering length, L13 buys one class,
  and the purity criterion is met at every rung"*), `200542f` (*"the phase-2 eval field is
  priced…"*) and `0f0cb44` (*"the author's own arithmetic pass…"*). Working tree clean apart
  from this report.
- **`git diff --stat 54eb3ba HEAD -- tools/` is EMPTY.** The instrument is byte-identical at
  the named revision and at HEAD, so every instrument finding below applies verbatim at HEAD.
- **The MEMO is not.** `git diff --stat 54eb3ba HEAD -- docs/experiments/hex_threat_enum_v1.md`
  → 133 insertions: §7 was filled in and the census was RUN while this confirmation was
  outstanding. **My subject is the memo at `54eb3ba`.** §7 is quoted below only where it is
  corroborating evidence, never as the subject — it is unreviewed text. CLAUDE.md: *"A WP is
  not landable while its reviews are outstanding."*
- **And the numbers reached a matrix.** `docs/experiments/matrix_wp22_phase2_eval.md`, landed
  at `200542f`, prices `R-A4-CLASS` on them and states its **kill condition** as *"The
  registered purity criterion failing at the chosen rung — MET at all sixteen cells, so **not
  fired**"* (`:331`), recommending *"R-A4-CLASS at `L = 11`, rung T2 … registered purity
  criterion met at 3.58x"* (`:477-479`). **N-1 below is therefore not a documentation
  complaint; it is a defect in a live kill condition.**

**Instruments.** I wrote my own, from the memo's §5/§6 prose only — not from the shipped
`tools/hex_enum/`, not from the first review's Appendix A (D-691: a confirmation that verifies
a rule against the rule that prescribed it is void, and an implementation copied from the
report cannot catch an error the report shares with the fix).

| file | what it is |
|---|---|
| `…/scratchpad/confirm/mine.py` | independent §5 tuple, odd-`L` pattern space, `k(L)`, four clip readings, `C(k+2,3)` |
| `…/scratchpad/confirm/tcalc.py` | independent `DEF-PLAN` / `DEF-T` — exact minimum hitting set by exhaustive subset search |
| `…/scratchpad/confirm/countonly.py` | the §6.5 criterion attack: drives the SHIPPED `census.py` with the class table replaced by a value-free quotient |
| `…/scratchpad/confirm/tree/` | a copy of `tools/` for mutation testing — **the live tree was never mutated** |

Scratch is ephemeral (MEMORY.md, *"Review reports live in ephemeral scratchpads"*), so every
command and its full output is inlined below. All enumerations exhaustive; no sampling, no
seed except where the shipped census's own referent seed is used.

**Cross-validation.** My tuple and the shipped `hexenum.tuple_of` agree on `k` at every odd
length enumerated (25 / 98 / 357 / 357) and on the whole ladder (T3 121, T2 47, T1 36 at
`L = 11`). **No finding below is an implementation difference.**

**Live-tree discipline.** No `cargo`, no worktree, no commit, no `CARGO_TARGET_DIR`. The
shipped scripts were run read-only with outputs written to scratch; the gate suite and every
mutant ran in the scratch copy. `git status --porcelain` shows nothing of mine.

---

## 1. Confirmation table

`STILL LANDS` = the defect the finding named is still there. `CLOSED` = the attack no longer
reproduces. `CLOSED BUT MOVED` = the fix discharged the finding's sentence and re-created its
property one step to the left (D-630).

| # | the finding | my attack | output | verdict |
|---|---|---|---|---|
| **B-1** | stabilisation length is not a measurement | `python3 tools/hex_enum/hexenum.py --refine 7 9 11`; my own `k(L)` for `L ∈ {7,9,11,13}`; my own two-way fanout of the 7↔11 and 9↔11 joint partitions | `k = 25 / 98 / 357 / 357`; refine prints *"neither refines the other"* at 7→9 and 9→11 **at all four rungs** and *"the longer partition is a function of the shorter"* at 11→13 at all four; **merge curve reproduces exactly**: 3–104 mean 32.4, 1–8 mean 2.3, 2–36 mean 9.4, 1–9 mean 2.6 | **CLOSED BUT MOVED** → N-7 |
| **B-2** | even `L` under-specified; `3^(L−1)` matches no reading | my own enumeration of BOTH readings; `E.centres(8)`; every entry point; the census; the gate suite | reading (B) `k = 10 / 43 / 182`, reading (A) `k = 33 / 177 / 920` — **the memo's §3.1 table is exactly right**; `centres`, `patterns`, `table`, `tuple_of`, `reverse_code`, `refinement`, `census.py 8`, `hexenum.py 8` all raise `ValueError: length 8 is even … the enum is defined at odd lengths only` | **CLOSED** |
| **B-3** | T1's clip lands on the imported 816 and destroys the boundary it cites | four clip readings enumerated at `L = 11` under my own tuple, including **the coarsest clip that states both LAW-SUPPORT boundaries** | memo's clip `k = 36`, `C(38,3) = 8 436` ✅ reproduces. `min(cost,4)` ∞-apart `k = 25`. ∞-folded `k = 16`, 816. **`{cost ≤ 2 \| 3–4 \| ≥5 \| dead}` — the coarsest partition expressing both boundaries — gives `k = 16` and `C(18,3) = 816`** | **STILL LANDS** → N-4 |
| **M-1** | the tuple merges `t = 1` and `t = 2` | both §5.5 witness pairs recomputed from `DEF-PLAN`/`DEF-T`; the aggregate recomputed under the memo's own stated definition | both witness pairs reproduce **exactly**, tuples and `t` values. Aggregate under the memo's stated definition (*"open windows at 4 or 5 own stones"*): **32 classes / 6 099 patterns**, not 27 / 5 348 | **CLOSED BUT MOVED** → N-2, N-3 |
| **M-2** | the cubic is never applied to the memo's own bound | arithmetic | `C(1371,3) = 428 558 605` ✅; per-side 37, joint 1 369 ✅; exemplars 816 / 11 480 / 171 700 ✅ | **CLOSED** |
| **M-3** | §6.5 registers no criterion; referent 1 is an identity | referent 1 is now labelled an identity ✅. **Criterion tested**: built a quotient that discards every calculus distinction and ran the SHIPPED census on it, window unit, `L = 7`, 3 000 positions | enum T4 `ω² = 0.005031` vs worst null `0.001958` → **2.57x, MET**. Value-free count-only quotient `ω² = 0.004840` vs worst null `0.002740` → **1.77x, ALSO MET** | **STILL LANDS** → **N-1** |
| **M-4** | T2's boolean refines nothing | my own `k` with and without | 47 and 47; boolean deleted from the shipped rung ✅ | **CLOSED** |
| **M-5** | T3's justification names the wrong quantity and an unreachable threshold | exhaustive `DEF-T` over all `3^11` lines; then: does the clip destroy `t` distinctions? | max `t = 2` ✅ and the distribution **reproduces exactly** (`t=0` 319 606, `t=1` 32 002, `t=2` 2 686 over 354 294 pairs). But the clip at `≥2` **merges T4 classes whose exact `t` differs** — T4 6 099 patterns in `t`-inconsistent classes, **T3 7 807** | **CLOSED BUT MOVED** → N-5 |
| **M-6** | D-706 and the memo are one act | `/usr/bin/grep -n "D-706"` over the memo; read D-706 in full | §1.5 says it, names the external ground (`threat_calculus_v1.md:12-13`, `eval_families` §7), and says *"D-706 records the ruling; it does not license it"*. Two residual mentions (`:21`, `:114`) do no work — deleting them changes no reading (D-424) | **CLOSED** → minor N-9 |
| **M-7** | *"discharges"* closes THM-WINDOW on a different question | read §1.2's row and §6.6 ¶2 | *"discharges"* is gone; §1.2 now says *"§6.6 states exactly which half of this the construction can answer and which it cannot"*; §6.6 ¶2 states 7 and 8 are window-relative and **cannot appear in §7**, and §9 lists *"THM-WINDOW is not closed"* | **CLOSED** |
| **m-1** | containment convention vacuous at the headline lengths | enumerate 6-windows inside `P_L` missing `c` | 0 at `L = 7, 9, 11`; **2 of 8 at `L = 13`** — §4's *"0 at L = 7, 9 and 11 and first becomes non-zero at L = 13 (2 of 8)"* is exact | **CLOSED** |
| **m-2** | self-check 5's `Completed` arm cannot fail | read §6.7 | dropped, with the reason on the page and §5.1's standard cited | **CLOSED** — but see **N-6**: self-check 3 is now the arm that cannot fail |
| **m-3** | Buro's floors carry no transferability tag | read §6.3 | all three tagged: `≤4` and `≥20` **ARCH**, `75` **EMP, NON-TRANSFERABLE**, *"a landmark, not a criterion"* | **CLOSED** |
| **m-4** | the rungs' clip points are hand-written | read §5.4 / §6.4 | §5.4 now concedes *"a projection is a choice of resolution and cannot not have them"*. But T1's and T3's cited grounds do **not** determine their clips (N-4, N-5) | **CLOSED in form, N-4/N-5 in substance** |
| **m-5** | scored-cell restriction is a well-definedness requirement | read §4 ¶2–3 | states the divergence argument and *"the restriction is what makes the sum a number at all"*; the legal-region count is *"reported so a matrix can see what the scored set excludes, and for nothing else"* | **CLOSED** |
| **m-6** | code-unit `η²` ≈1 by memorisation | read §6.5; read `report.py:33-59` | `ω²` is primary and the reason is stated; `report.py` prints both | **CLOSED** → minor N-11 |
| **m-7** | two citation slips | `extract.py` line check; length-set reference | `extract.py:24-26` ✅; the length set is stated in §3.1/§6.1, not by reference to a dispatch ✅ | **CLOSED** |
| **m-8** | own/opp equivariance unclaimed | my own enumeration over all 59 049 patterns | **0 mismatches, 17 swap-fixed classes, 187 orbits** — §5.3 exact | **CLOSED** |
| **Q1** | what carries the value of the stones | read §4 ¶4 | names the summand as undecided, binds the matrix (*"must name the summand per row"*), defers to Phase 2b | **ANSWERED** |
| **Q2** | merge curve when the `L = 10` point is 1.0 | even `L` refused, so `L = 10` is not an object; §6.6 gives the odd-only curve | reproduces (see B-1) | **ANSWERED** |
| **Q3** | what number would make the architect not price the row | §6.5's registered criterion | a criterion exists — but a value-free quotient passes it (N-1), and the matrix at HEAD records it as *"not fired"* | **ANSWERED IN FORM, EMPTY IN SUBSTANCE** → N-1 |
| **Q4** | D-483 and the memo's own bound | §5.2 | `C(1371,3) = 428 558 605` is on the page with its arithmetic | **ANSWERED** |
| **Q5** | does `min_opp` range over windows open for OPP | §5's `open_X` bullet | *"it is the same predicate for both sides: `open_opp` counts the windows through `c` holding no OWN stone"* — my implementation read it that way and agrees with the shipped one on all 66 339 patterns | **ANSWERED** |

**Score: 3 findings STILL LAND or MOVED at BLOCKING/MAJOR weight (B-3, M-3, M-5), 2 MOVED
(B-1, M-1), 15 CLOSED, 5 questions answered.**

---

## 2. NEW findings against revision 2

Revision 2's own new text is unreviewed, and this is where most of what follows comes from.

### N-1 — BLOCKING. §6.5's registered criterion is passed by a quotient that discards EVERY distinction the calculus names. The memo's new sentence claiming otherwise is measurably false, and the criterion is the matrix's kill condition at HEAD.

§6.5, revision 2's new text:

> **This is a criterion the named defect could falsify: a quotient that discarded the
> value-carrying distinctions would leave `ω²` at the null's level**, and the null shares the
> corpus, the class count and the class-size distribution, so it is invariant under everything
> except which codes were grouped.

**I built exactly that quotient and it does not leave `ω²` at the null's level.** The quotient
is `(own stones in the pattern, opp stones in the pattern)` — no window, no openness, no
completion cost, no rule-4 relevance, no position at all. It is the crudest thing that is still
a function of the line, and it discards, by construction, every distinction §5 says the tuple
exists to carry.

*Reproducer* — `countonly.py` replaces `hexenum.table` and then runs the **shipped** `census.py`
unmodified, so both arms share the corpus, the walk, the moments code and the matched-null
machinery:

```
$ python3 countonly.py /home/tom/Projects/HeXO-AlphaBeta enum      out_enum.txt      7 --limit 3000 --legal-stride 500
$ python3 countonly.py /home/tom/Projects/HeXO-AlphaBeta countonly out_countonly.txt 7 --limit 3000 --legal-stride 500
```

WINDOW unit — the unit §6.5's criterion names — `n = 530 785` observations in both arms:

| partition | classes | `ω²` | worst of 3 matched nulls | ratio | §6.5's criterion |
|---|---|---|---|---|---|
| **the enum, T4** | 16 | 0.005031 | 0.001958 | **2.57x** | MET |
| the enum, T2 | 10 | 0.003807 | 0.001567 | 2.43x | MET |
| the enum, T1 | 8 | 0.003220 | 0.001565 | 2.06x | MET |
| **count-only quotient** | 23 | **0.004840** | 0.002740 | **1.77x** | **MET** |

The value-free quotient reaches **96 % of the full tuple's `ω²`** and clears its own matched
null by 1.77x. Two consequences, and the second is the finding:

1. Almost all of the enum's measured purity is stone-counting. The entire threat structure —
   openness, completion cost, rule-4 relevance — buys 4 %.
2. **The criterion cannot separate the enum from a quotient that threw away everything the
   defect class names.** `process.md`: *"A criterion that is a property the named defect class
   PRESERVES … passes vacuously and is not a criterion; it must be one that defect could
   falsify."* Measured: the named defect preserves it.

*What the quotient actually discards*, so this is not an abstraction:

```
XXXXX......   own=5 opp=0   tuple = (1, 6, True,  6, 1, False)    <- one stone completes a six
X.X.X.X.X..   own=5 opp=0   tuple = (3, 6, False, inf, 0, False)  <- three stones short, opp dead
```

Same class under the count-only quotient. It cannot see **game rule 4** — the memo's own third
tuple component — and it merges both of §5.5's `t = 1` / `t = 2` witness pairs as well.

**This is M-3's second half, intact.** The first reviewer wrote: *"a tuple that separated only
stone count would beat a random quotient comfortably while discarding every structural
distinction."* Revision 2 answered the sentence — it registered a threshold and labelled
referent 1 as an identity — and left the property. The fix round did not test its own criterion.

**And it is load-bearing at HEAD.** `matrix_wp22_phase2_eval.md:331` states R-A4-CLASS's kill
condition as *"The registered purity criterion failing at the chosen rung — MET at all sixteen
cells, so **not fired**"*, and `:477-479` recommends T2 at `L = 11` partly on *"registered
purity criterion met at 3.58x"*. A kill condition a value-free quotient passes did not decline
to fire; it could not fire.

*Corroboration from the author's own run* (unreviewed §7, cited as evidence only): the criterion
is MET at **all sixteen cells including T1**, which collapses 357 classes to 16. A criterion met
by every rung of a ladder whose bottom rung is that coarse is reporting the corpus, not the enum.

### N-2 — MAJOR. §5.5's two measured claims are computed under two mutually inconsistent readings of `DEF-PLAN`, and under the reading §5.5 states, the merge it discloses is BIGGER than the number it publishes.

§5.5 states its definition once:

> the exact `DEF-T` over own's plan family after placing one own stone at `c` — **a minimum
> hitting set over `DEF-PLAN`'s open windows at 4 or 5 own stones**

I implemented exactly that (`tcalc.plans`, `seg.count(side) in (4, 5)`), over all 59 049
length-11 patterns:

| reading | classes whose members' exact `t` disagrees | patterns in them |
|---|---|---|
| **V1 — §5.5's stated definition (own ∈ {4,5})** | **32 of 357** | **6 099 of 59 049** |
| V2 — drop patterns whose placement completes a six | 27 (of 335 surviving classes) | 5 348 |
| V3 — `DEF-PLAN` literal (own ≥ 4, so a 6-own window contributes the unhittable ∅) | **27 of 357** | **5 348** |

**The published 27 / 5 348 is not V1.** It requires either dropping the rule-4 patterns (V2 —
in which case *"27 of the 357 classes"* has a numerator over 335 and a denominator over 357) or
admitting the size-0 plan (V3), and §5.5 says neither.

**And the same paragraph needs the other reading three sentences later.** §5.5's second measured
claim — *"the maximum exact `DEF-T` over all `3^11 = 177 147` length-11 single-axis lines is 2 …
with the distribution `t=0: 319 606, t=1: 32 002, t=2: 2 686` over the 354 294 (line, side)
pairs"* — reproduces **exactly and only under V1**:

```
V1 (4 or 5 own, memo prose): {0: 319606, 1: 32002, 2: 2686}  total 354294   max t = 2
V3 (DEF-PLAN literal >=4):   {0: 318836, 1: 30950, 2: 2402, INF: 2106}      max t unbounded
```

Under V3 the max-`t` claim is **false** — 2 106 (line, side) pairs carry an unhittable plan.
Under V1 the merge claim is **understated** — 32 classes and 6 099 patterns, not 27 and 5 348.
One paragraph, one named quantity, two incompatible computations, and the memo publishes the
smaller merge. §5.5 is the M-1 remedy; the number a matrix will quote from it is the wrong one
under the definition printed beside it.

*Fix*: state which reading of `DEF-PLAN` §5.5 uses (rule 4 makes V2 the defensible one — a
completing stone ends the turn and there is no threat question), and republish the number that
reading gives, with the denominator that reading gives.

### N-3 — MAJOR. §5.5's reason for not repairing the merge is false, and the memo refutes it itself two sentences later.

§5.5:

> Repairing it means carrying exact `t` in the tuple, which is a per-POSITION quantity over a
> plan family that spans axes … **not a property of one line's neighbourhood, and not something
> a single-axis class can hold.**

The merge §5.5 measures is a **single-axis** merge, and single-axis exact `t` **is** a property
of one line's neighbourhood — §5.5 computes it as one, over all `3^11` lines, in the very next
paragraph, to get its max-`t = 2` result. A quantity the memo enumerates as a function of an
11-cell line cannot also be a quantity a class of 11-cell lines cannot hold.

*Reproducer* — add single-axis `t_own` (exact `DEF-T` after placing own at `c`, computed from
the line alone) as a seventh tuple component:

```
tuple as shipped          : k = 357, classes merging different single-axis t: 32 (6 099 patterns)
tuple + single-axis t_own : k = 393, classes merging different single-axis t:  0 (0 patterns)
C(k+2,3): 7 647 059 -> 10 193 765   (k +10 %, parameters +33 %)
```

**The merge is repairable inside a single-axis class, completely, for 36 extra classes.** What
is *not* repairable single-axis is the POSITION's `t` (`LAW-OVERLOAD`'s cross-axis addition
floor) — a true statement about a different object, standing in for a false one about this one.

This does not oblige the memo to adopt the component: §9 defers evaluator decisions, and 33 %
more parameters at an already-unaffordable count is a real cost. It obliges the memo to say
*"repairing this costs 36 classes and we are not paying it"* instead of *"a single-axis class
cannot hold it"*. **The correctness defect is that the second sentence is false**, and a matrix
that reads it will price the row as though the merge were structural.

### N-4 — MAJOR (B-3 STILL LANDS). T1's new clip is not determined by the ground it cites, its "only if" is false, and the coarsest clip that does satisfy that ground is `k = 16` → 816.

§6.4 and `hexenum.clip`'s docstring make the same claim:

> `LAW-SUPPORT` (`:69`) reads `own ≥ 6−2k`, so `k = 1 ⟺ cost ≤ 2` and `k = 2 ⟺ cost ≤ 4`.
> **Both boundaries are expressible only if 1, 2, 3 and 4 stay apart** and everything above them
> merges.

**The arithmetic is right and the "only if" is false.** I re-derived the boundary from
`threat_calculus_v1.md:68-71` myself. `min_X = 6 − |X's stones in w|` for `w` open for `X`;
because `w` is open, every non-`X` cell in it is empty, so `min_X` is exactly the count of
stones `X` still owes, the empty centre `c` among them. LAW-SUPPORT's *"already holding
≥ 6−2k"* is then `6 − holding ≤ 2k`, i.e. `cost ≤ 2k`: `k = 1 ⟺ cost ≤ 2`, `k = 2 ⟺ cost ≤ 4`.
**No off-by-one — counting `c` is correct**, since `c` is one of the cells the side must fill.

But expressing two boundaries requires only that the partition refine `{≤2 | 3–4 | ≥5}`. Keeping
1 apart from 2, and 3 apart from 4, is extra resolution LAW-SUPPORT does not ask for and the
memo does not otherwise justify. Enumerated at `L = 11` under my own tuple:

| clip | `k` | `C(k+2,3)` |
|---|---|---|
| rev-2 shipped, `{1,2,3,4,5+,∞}` | 36 | 8 436 |
| rev-1, `min(cost,4)`, ∞ apart | 25 | 2 925 |
| rev-1, `min(cost,4)`, ∞ folded | 16 | **816** |
| **coarsest expressing both LAW-SUPPORT boundaries, `{≤2 \| 3–4 \| ≥5 \| dead}`** | **16** | **816** |

**The rung's own stated ground, taken at its coarsest, lands on the imported figure again.**
B-3 said the clip *"lands on the imported 816"*; the fix moved the clip and left the property —
the number that falls out of the cited law is still 816, and what keeps the shipped rung off it
is a hand-written extra split (m-4's defect), not `LAW-SUPPORT`.

The shipped test `the_ladders_bottom_rung_states_law_supports_two_boundaries` asserts
`values == {1,2,3,4,5,INF}` and `k == 36` — it pins the chosen clip, not the property
*"both boundaries are expressible"*, which the 816 clip also has. A test of the prescribed form,
in D-691's sense.

*Fix*: either delete the *"only if"* and say plainly that T1 keeps four cost values because the
finer resolution is cheap and the clip is a choice — in which case §1.1's *"§6.4's rungs now
take their boundaries from `LAW-SUPPORT`'s own … and from nothing else"* must go too — or adopt
the coarsest faithful clip and report 816 as the enumerated answer with the derivation that
makes it not an import.

### N-5 — MAJOR (M-5 CLOSED BUT MOVED). T3's new justification reasons from a bound on `t` to a clip on a count of WINDOWS, and the clip destroys exact-`t` distinctions the full tuple kept.

§6.4's T3 row:

> `open` at 0 / 1 / ≥2. **MEASURED**: the maximum exact `DEF-T` a single axis can carry is 2
> (§5.5), so `{dead, one surviving completion, more than one}` is the whole resolution `open`
> supports on one line.

`max t = 2` is a bound on the *value* of a hitting-set number over the plan family. `open_X` is
a *count of open windows*, ranging 0…6 and taking its value from windows with no stones at all.
A bound on one is not a resolution limit on the other, and the inference is the same category
error M-5 named in revision 1 (`LiveCount` is a stone count, `t ≥ 3` is a hitting-set number,
`open` is neither).

*Reproducer* — the clip merges T4 classes whose exact `t` differs, i.e. it destroys the very
quantity its licence invokes:

```
T4: 32 of 357 classes disagree on exact t   (6 099 patterns)
T3: 12 of 121 classes disagree on exact t   (7 807 patterns)   <- 28 % MORE patterns
```

Three witness pairs, all with `min_own > 1` so no instant win is involved (`.` empty, `X` own,
`O` opp, `c` = index 5):

```
T3 class (3, 2, inf, 0) merges:
   ....X...XX.   open_own=6   exact t=1     T4 = (3, 6, False, inf, 0, False)
   ..XXX...O.X   open_own=3   exact t=2     T4 = (3, 3, False, inf, 0, False)

T3 class (2, 2, inf, 0) merges:
   ....X..XXX.   open_own=6   exact t=1     T4 = (2, 6, False, inf, 0, False)
   ..XO..XXXX.   open_own=2   exact t=2     T4 = (2, 2, False, inf, 0, False)

T3 class (3, 2, 5, 1) merges:
   ...O..X..XX   open_own=2   exact t=1     T4 = (3, 2, False, 5, 1, False)
   ..XXX...O..   open_own=3   exact t=2     T4 = (3, 3, False, 5, 1, False)
```

Each pair differs **only** in `open_own`, is separated by the full tuple, is merged by the clip,
and has different exact `t`. *"The whole resolution `open` supports on one line"* is refuted by
the rung's own criterion.

*Fix*: T3 is a coarsening and is allowed to be one. Say *"`open` is clipped at ≥2 to bound the
parameter count; the boundary is a choice"* and drop the derivation. Under D-424 the sentence
that survives is the one that constrains something.

### N-6 — MAJOR. CI gate 19's control does not cover the statistic the registered criterion is stated in. Four mutants of `report.py` survive the whole suite, and §6.7 self-check 3 is a tautology in the implementation.

`tools/SHELL_CHECKLIST.md` item 10: *"a test in a suite CI runs, driving the SHIPPED script …
with a control run so a pass cannot come from a gate that refuses everything"*, and
`test_hex_enum.py`'s own docstring: *"with a control run so a pass cannot come from a routine
that reports the same numbers whatever it reads."*

Baseline in the scratch copy: `bash tools/hex_enum_tests.sh` → exit 0, both suites all-pass.
Then, one mutation at a time, restoring between each:

| mutant of `tools/hex_enum/report.py` | gate 19 |
|---|---|
| `between += 0.0` — between-class variance always zero | **exit 0, SURVIVES** |
| `omega2 = 1.0` always | **exit 0, SURVIVES** |
| `eta2 = 0.5` and `omega2 = 0.5` always | **exit 0, SURVIVES** |
| `within += self.square[key]` — the mean correction dropped | **exit 0, SURVIVES**, and still prints `ok: every reported partition satisfies the law of total variance` |

`ω²` is the statistic §6.5's criterion is written in and the number `matrix_wp22_phase2_eval.md`
prices the row on. **No test in the suite reads it.** A routine that printed a constant `ω²`
would pass gate 19 and the matrix would quote it.

**And the law-of-total-variance check cannot fail, by construction.** `report.py:53`:

```python
total = within + between
```

`total` is *defined* as the sum, so `test_census.py`'s `abs(within + between - total) > tol` and
§6.7 self-check 3 both compare a number to itself. That is precisely the defect **m-2** named for
self-check 5 (*"A self-check that cannot fail is not a check"*), which revision 2 removed there
and left here — one step to the left. (The arithmetic itself is right: I checked `terms()`
against an independently computed total variance on 400 synthetic observations and they agree to
1e-9. The code is correct; the check is empty.)

*Fix*: compute `total` independently — `Σx²/N − grand²` — and assert the identity against it;
and add a control in which a partition known to carry signal must beat a known-flat one on `ω²`,
so the criterion's own statistic has a test.

### N-7 — MINOR (B-1 MOVED). The merge curve is tagged `DERIVED`, is an exhaustive enumeration, contradicts the memo's own header, and no named instrument produces it.

§6.6: *"What §7 reports instead is the two-way fanout of the joint partition, **DERIVED** at the
full tuple: one `L = 7` class meets 3 to 104 of the `L = 11` classes (mean 32.4)…"*

Three things:

1. **It is measured, not derived.** The header's own rule is *"Every number in §1 through §6 is
   a quotation, a DERIVED count **with its arithmetic on the page**, or a bound"*. There is no
   arithmetic; 3, 104, 32.4, 2.3, 2, 36, 9.4, 1, 9, 2.6 are outputs of an exhaustive walk over
   59 049 patterns. I reproduced all ten exactly — they are *right*, and they are *measured*.
   §6.6's preceding paragraph uses `DERIVED` in its strong sense (*"the enumeration can take no
   other value"*), so the tag tells a reader these fanouts are forced by the construction. They
   are contingent facts about hex. **B-1's defect class was exactly a derived/measured mislabel;
   the fix swapped the labels rather than removing the confusion.**
2. **It contradicts the header**, which says *"the merge curve land[s] in §7 only after §6's
   instrument has run"*, and §7 at the named revision says **EMPTY UNTIL**. The same paragraph
   calls it *"THE MEASURED CONTENT"* in its heading and `DERIVED` in its sentence.
3. **The named instrument cannot produce it.** `refinement()` returns only
   `(len(shorts), len(longs), len(pairs))` — no fanout minimum, maximum or mean — and `main`
   calls it only with `step=2`, so the 7→11 join the curve needs is never computed.
   `/usr/bin/grep -rn "refinement(\|step" tools/hex_enum/*.py` confirms both. §6 says a change to
   the named instrument reopens this review; a number no named instrument computes is outside
   that binding altogether.

(The mislabel propagated: §7.2 at HEAD repeats *"The measured content is the merge curve below
11, DERIVED at T4"*.)

### N-8 — MINOR. The "matched" null is not matched on the population where the comparison is made.

§6.5: *"it is matched on the enum's own SHAPE … the class count and the number of codes per
class are exactly the enum's."* True over the `3^(L−1)` code space; false over the observed
corpus, which is where `ω²` is compared. Measured, `L = 7`, 3 000 positions, window unit:

```
enum T4       classes = 16
null T4 r0/r1/r2  classes = 25 / 24 / 25
```

The enum realises 16 classes and its "matched" nulls realise 24–25, because the permutation
scatters the enum's unobserved codes into classes the observed traffic then reaches. The
comparison is therefore between partitions of different realised size — the one property the
matching exists to hold fixed. I have not established which way the bias runs; that it exists
undercuts the sentence that makes the referent readable.

### N-9 — MINOR. Residual D-706 lean, and one consequence §1.5 does not draw.

§1.5 is a good disclosure and closes M-6. Two leftovers:

- `:114` still uses D-706 normatively — *"a threshold is what D-706 forbids taking on faith"* —
  after §1.5 has said *"D-706 records the ruling; it does not license it."* The ground is
  independently available (§1.1's scope rule), so the mention constrains nothing and should go
  (D-424).
- **Substantive**: D-706 names five receipts that *"are the only basis on which A4-CLASS is
  priced"*, one of which is *"THM-WINDOW stabilisation length"*. §6.6 now says that length is
  derived and *"the enumeration can take no other value"*. So one of the five pricing receipts
  is a definitional consequence, and the memo does not say that the basis is thereby four
  receipts and not five. A matrix reading D-706 would not know.

### N-10 — MINOR. The criterion binds the WINDOW unit; the instrument prints both units in one table with nothing marking which is covered.

§6.5 restricts the criterion to the window unit and gives the reason, then says purity is
*"Reported at the WINDOW unit … and at the CODE unit"*. `report.py:121-126` prints window and
code rows in a single `## purity` table with no column saying which rows a criterion covers.
Nothing in the artifact carries §6.5's restriction across, and a matrix reading the artifact
would have to remember it.

### N-11 — TRIVIAL. Stale prose from the deleted even-`L` convention.

`hexenum.py:73`: `"""The class key of one pattern: §5's 6-tuple, or a pair of them if even."""`
— even lengths now raise. The key is also still wrapped as a one-element tuple
(`tuple(sorted(keys))`), an artefact of the same deleted convention; harmless, but §5 says
*"The class of `P_L(c, a)` is the 6-tuple"* and the instrument's key is not one.

### N-12 — PROCESS. §7 landed while this confirmation was outstanding.

`02ad9fb` filled §7 and `200542f` priced the row on it, both after the named revision and before
this report. CLAUDE.md: *"A WP is not landable while its reviews are outstanding."* §7 and the
matrix are unreviewed by me; N-1 in particular applies to a kill condition now recorded in a
committed matrix.

---

## 3. What I attacked and it SURVIVED

Several of these were the attacks I expected to land.

1. **`k` itself.** Independent enumeration from the prose: 25 / 98 / 357 / 357 at `L = 7, 9, 11,
   13`, and the whole ladder at `L = 11` (357 / 121 / 47 / 36). Exact agreement with the shipped
   instrument on every cell. **No arithmetic error anywhere in the enum.**
2. **The even-`L` refusal, on the question of whether it costs anything.** It does not. The
   matrix's affordable lengths survive it — `L = 7` at `k = 25` / `C(27,3) = 2 925`, `L = 9` at
   98 / 161 700 — and `eval_families` §0.2's `L = 8` row (2 920 folded cells, median 273) is a
   census of **windows at `(axis, start)` with all eight cells encoded**, not of cell-centred
   classes. §3.1 and §6.3 are right that it is a different object, and §1.4 correctly drops the
   row rather than inviting the comparison. Refusing rather than conventionalising is the CLAUDE.md
   rule-3 answer and it loses nothing.
3. **The refusal's reach.** I tried to leak an even length past it through `patterns`, `table`,
   `tuple_of`, `reverse_code`, `refinement`, `census.py` and `hexenum.py`'s `main`. All seven
   raise the named error. No silent path.
4. **`min_X` and the empty centre.** I re-derived LAW-SUPPORT's boundary from
   `threat_calculus_v1.md:68-71` independently, specifically hunting an off-by-one from counting
   `c`. There is none: `c` is a cell the side must fill and belongs in the cost. `cost ≤ 2 ⟺ k=1`
   and `cost ≤ 4 ⟺ k=2` are both correct. (What fails is the *"only if"* built on top — N-4.)
5. **`|W_L| = min(6, L−5)`**, the reversal invariance (0 mismatches / 59 049), the swap
   equivariance (17 fixed, 187 orbits), `r4`'s redundancy as **sets** and not merely counts, the
   T2 boolean's redundancy (47 = 47), the folded raw-code ceiling (378 / 3 321 / 29 646), the
   window-containment count (0/0/0/2), `C(1371,3)`, and the ladder-is-a-chain property — every
   one reproduces exactly under my own code.
6. **Both `t` witnesses in §5.5**, tuples and plan families and `t` values, recomputed from
   `DEF-PLAN` and `DEF-T` with no reference to the memo's arithmetic. Exact.
7. **`max t = 2` on one axis**, over all 177 147 lines and 354 294 (line, side) pairs, with the
   memo's exact distribution. A genuine and non-obvious fact about hex, correctly derived and
   correctly used to retire revision 1's unreachable `t ≥ 3`. (It just does not license the clip
   it is attached to — N-5.)
8. **`Moments.terms()`'s arithmetic.** I checked within/between/total and `η²` against an
   independent computation on 400 synthetic observations: agreement to 1e-9. The variance code is
   right; only its *check* is empty (N-6).
9. **Rule 9.** Every file under `tools/hex_enum/` is under the ~300-line soft cap — 136, 197,
   200, 225, 251, 260, and `hex_enum_tests.sh` at 46 — so no `docs/rule9_justifications.md`
   entry is owed and none is missing.
10. **`hex_enum_tests.sh` against the rest of the checklist.** `set -euo pipefail`; programs
    resolved through `tools/require_tool.sh` rather than `command -v` (item 8); a genuine
    exit-2 `void` class distinct from failure (item 12); the scratch directory namespaced
    `pistol-testscratch-` (item 6); no unguarded `grep` in statement position (item 3); no trap
    (item 7 n/a). **Item 10 is the one it fails** — N-6.
11. **The census's own refusals.** `--tranche` without a `{}` placeholder, an unknown argument,
    and a non-reversal-invariant class table are each refused by name. Fail-loud holds.

---

## 4. VERDICT: **FAIL**

A confirmation PASSes when every finding is CLOSED and the fix round introduced no new BLOCKING
or MAJOR. Neither half holds.

**Findings that still land**

- **B-3 → N-4**: the clip moved; the property did not. The coarsest clip expressing the two
  LAW-SUPPORT boundaries the rung cites is `k = 16`, `C(18,3) = 816`, and the memo's *"only if"*
  that would rule it out is false.
- **M-3 → N-1 (BLOCKING)**: the criterion was registered and is passed at 1.77x by a quotient
  that discards openness, completion cost and game rule 4. `process.md`'s clause is not
  satisfied by a criterion the named defect class preserves, and this one is now a committed
  matrix's kill condition.
- **M-5 → N-5**: the justification's wrong-quantity defect survives the substitution of a true
  fact for a false one; the clip destroys 28 % more `t`-inconsistent patterns than the tuple it
  coarsens.
- **B-1 → N-7** and **M-1 → N-2, N-3**: closed in their sentences, re-created as a
  derived/measured mislabel and as two false claims in §5.5.

**New at MAJOR or above**: N-1 (BLOCKING), N-2, N-3, N-4, N-5, N-6.

**What this is not.** The enum is arithmetically sound. Every count I could check independently
was right, including several the memo had no obligation to get right on the first pass. Fifteen
of eighteen findings are genuinely closed, and B-2 — the largest of the three BLOCKINGs — is
closed cleanly and at the code, with the refusal reaching every entry point. The failure is
concentrated in the three places where revision 2 wrote a NEW justification: T1's clip, T3's
clip, and §6.5's criterion. In each, a sentence was constructed to discharge the finding and
was not itself attacked. That is this project's recorded pattern (D-630), and it is the reason
the fix round's own text needs the round it did not get.

**Not overruleable (D-424).** N-1, N-2, N-3, N-4 and N-5 each name a way the package produces a
wrong answer — a kill condition that cannot fire, a published merge count that is the wrong
number under its own definition, a false impossibility claim, a false *"only if"*, and a clip
whose stated licence its own criterion refutes. D-424 reaches prose that constrains nothing;
none of these is that.

**Cheapest route to a pass.** Four are deletions and two are one-line runs:

1. **N-4, N-5**: delete the two derivations. Say the clips are choices bounding the parameter
   count. §1.1's *"and from nothing else"* goes with them.
2. **N-2**: name the `DEF-PLAN` reading (rule 4 makes "drop the completing placements" the
   defensible one) and republish 27 / 5 348 with its own denominator, or publish 32 / 6 099.
3. **N-3**: replace *"a single-axis class cannot hold it"* with the measured price — `k` 357 →
   393, parameters +33 % — and say it is not being paid.
4. **N-7**: tag the merge curve MEASURED, and either add the fanout to `refinement()` or drop it
   to §7 where the header already sends it.
5. **N-1** is the one that costs a run rather than an edit. The criterion needs a floor a
   value-free quotient cannot clear — the natural one is *the enum must beat a stone-count-only
   quotient of its own construction*, which is externally derived in `process.md`'s sense,
   shares the corpus and the dependence, and is one extra census arm at the cost measured above.
   Until it has one, `matrix_wp22_phase2_eval.md:331`'s kill condition should read *"no kill
   condition is registered"*.
6. **N-6**: compute `total` independently and give `ω²` a control.
