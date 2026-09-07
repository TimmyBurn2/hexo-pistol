# ROUND 3 REVIEW — `docs/experiments/hex_threat_enum_v1.md` revision 3 — **FAIL**

Round 3 of four under D-709, remedies-only. Fresh context; not the author, not the
first reviewer, not the confirmation reviewer. D-691 binds: every verdict below is
the finding's own attack re-run against revision 3, never a check that a prescribed
sentence appears.

## Header

**Named revision under review**: `6238a0518c8bb0f4695df1ed18d43ecabae3bf0f` (`dev`),
*"docs(enum): the census receipt is wired and the two-round stop is marked superseded
by round three"*.

**Does it still match HEAD? NO — THE TREE MOVED, AND THE SUBJECT MOVED WITH IT.**

- At the start `git rev-parse HEAD` → `6238a05`, `git status --porcelain` clean.
- At the end HEAD is `564dc9e4ff7682e01601dcf65ea372eb9470e739`, reached by three
  commits: `a5687d6` (*"the round-three census replicates the round-two one on all
  two hundred shared cells"*), `0c2f519` and `564dc9e` (both `matrix_…`).
- **`git diff --stat 6238a05 HEAD -- tools/` is EMPTY.** The instrument is
  byte-identical at the named revision and at HEAD, so every instrument finding
  below applies verbatim at HEAD.
- **The memo is NOT identical**: `a5687d6` added **8 lines** at `:757` — a
  REPLICATED paragraph in §7. That text is unreviewed by anyone. I attacked it and
  it survives (§4.11); it changes no verdict below.
- The other two commits touch `matrix_wp22_phase2_eval*.md`, which another agent is
  red-teaming concurrently and which I did not modify. I read `:471` read-only,
  once, to establish whether N-1's remedy reached the matrix.

**Instruments.** Written by me from the memo's §5/§6 prose and the corpus schema.
Nothing below imports `tools/hex_enum/`, and nothing is copied from either prior
report — D-691, and an implementation copied from a report cannot catch an error the
report shares with the fix. Scratch is ephemeral (MEMORY.md), so every number is
inlined here with the command that produced it.

| file | what it is |
|---|---|
| `…/r3/mine.py` | independent §5 tuple, odd-`L` pattern space, the four rungs, `k(L)`, `C(k+2,3)` |
| `…/r3/tcalc.py` | independent `DEF-PLAN` / `DEF-T` — exact minimum hitting set by exhaustive subset search |
| `…/r3/clips.py` | **every** partition of `min_X` that expresses `LAW-SUPPORT`'s two boundaries, enumerated by set-partition search |
| `…/r3/walk.py` | independent full-corpus walk — my own manifest parse, R6 filter, `lines_of`, scored-cell rule, moments; shares no code with `tools/` |
| `…/r3/percode.py` + `fair.py` / `joint.py` / `vacuity.py` / `excess.py` | per-code window-unit moments, then matched-null entitlement, nested joins, and the vacuity attacks |
| `…/r3/tree/` + `mut.sh` | a copy of `tools/` for mutation testing — **the live tree was never mutated** |

**Cross-validation.** My tuple agrees with the shipped `hexenum.tuple_of` on `k` at
every odd length (25 / 98 / 357 / 357) and on the whole ladder. **My independent
full-corpus walk reproduces all four census receipts cell for cell** — `kept`,
scored cells, window-unit `n`, every realised class count and every `ω²` at
`L = 7, 9, 11, 13`. **No finding below is an implementation difference.**

**Live-tree discipline.** No `cargo`, no worktree, no commit, no
`CARGO_TARGET_DIR`, no edit outside this report. Shipped scripts were run
read-only; the gate suite and every mutant ran in the scratch copy on `/tmp`
(the copy is small; the 24 GiB tmpfs was at 3 %). `git status --porcelain` shows
nothing of mine but this file.

**Receipts, verified rather than trusted.**

```
$ cd artifacts/wp22_phase2a/census_r3 && sha256sum -c RECEIPT_census_r3.sha256
census_L11.log: OK   census_L11.txt: OK   census_L13.log: OK   census_L13.txt: OK
census_L7.log:  OK   census_L7.txt:  OK   census_L9.log:  OK   census_L9.txt:  OK
$ sha256sum RECEIPT_census_r3.sha256
9bb8fc6f2e0500bcf10f48e493dd9e3615897d935b73b63d8a8f47bf179e9c39   <- §7's digest, exact
$ cd artifacts/wp22_phase2a/n6_mutants && sha256sum -c RECEIPT_n6_mutants.sha256
mutants.txt: OK   run_mutants.sh: OK
$ git check-ignore -v artifacts/…/census_L11.txt   ->  .gitignore:19:/artifacts/
```

Hard rule 8 holds: the artifacts are gitignored and sha-indexed from the memo.

---

## 1. Confirmation table

`STILL LANDS` = the defect the finding named is still there. `CLOSED` = my attack no
longer reproduces it. `CLOSED BUT MOVED` = the fix discharged the finding and
re-created its property one step to the left (D-630).

### Round 1 (`hex_threat_enum_v1_REVIEW.md`)

| # | the finding | MY attack | output | verdict |
|---|---|---|---|---|
| **B-1** | stabilisation length is not a measurement | my own `k(L)`; then `hexenum.py --refine 7 9`, `--refine 11`, `--curve 11 7 9`, all run read-only | `k = 25 / 98 / 357 / 357`; `11 → 13` prints *"the longer partition is a function of the shorter"* at all four rungs with fanout `1-1 (mean 1.0)` both ways; `7 → 9` and `9 → 11` print *"neither refines the other"* at all four | **CLOSED** (via N-7) — but see **P-6b**, §6.6's *"always splits"* contradicts its own next paragraph |
| **B-2** | even `L` under-specified | my own enumeration of BOTH readings, from scratch; then every entry point | reading (B) **10 / 43 / 182**, reading (A) **33 / 177 / 920** — §3.1's table is exact. `centres`, `patterns`, `table`, `tuple_of`, `reverse_code`, `refinement`, `census.py 8`, `hexenum.py 8`: all eight raise `ValueError: length 8 is even …` | **CLOSED** |
| **B-3** | T1's clip lands on the imported 816 | shipped T1 re-enumerated under my own tuple; then the whole faithful-clip family | shipped T1 = `k 36`, `C(38,3) = 8 436` — off 816 ✅. The 816 clip is now named on the page with its derivation | **CLOSED** at the shipped rung → the enumeration around it is **P-2** |
| **M-1** | the `t = 1` / `t = 2` merge | both witness pairs recomputed from my own `DEF-PLAN`/`DEF-T` | both reproduce exactly, tuples and `t` | **CLOSED** (via N-2, N-3) |
| **M-2** | the cubic at the memo's own bound | arithmetic | `C(1371,3) = 428 558 605`; 816 / 11 480 / 171 700 | **CLOSED** |
| **M-3** | §6.5 registers no criterion | see **N-1** below | | **CLOSED BUT MOVED** → **P-1** |
| **M-4** | T2's boolean refines nothing | my own `k` with and without | 47 = 47; the boolean is gone from `project` | **CLOSED** |
| **M-5** | T3's justification | see **N-5** | | **CLOSED** |
| **M-6** | D-706 and the memo are one act | read §1.5; `git log -S` on D-706 | §1.5 says it and names the external ground; D-706 first appears at `d73deba`, after `0585e25` | **CLOSED** |
| **M-7** | *"discharges"* | `/usr/bin/grep -n discharge` over the memo | one hit, `:668`, and it is the WITHDRAWAL of the word | **CLOSED** |
| **m-1** | containment convention vacuous | `L − 5 − min(6, L−5)` enumerated | `0 / 0 / 0` at `L = 7, 9, 11`; `2 of 8` at `L = 13` — §4 exact | **CLOSED** |
| **m-2** | a self-check that cannot fail | §6.7's `Completed` arm is gone with its reason | gone | **CLOSED** — but the class recurs at the referent: **P-3** |
| **m-3** | Buro's floors untagged | read §6.3 | `≤4`/`≥20` **ARCH**, `75` **EMP, NON-TRANSFERABLE**, *"a landmark, not a criterion"* | **CLOSED** |
| **m-4** | clip points hand-written | read §5.4 / §6.4 | both rungs now say *"A CHOICE"* on their own face | **CLOSED** |
| **m-5** | scored-cell restriction | read §4 | the divergence argument is on the page | **CLOSED** |
| **m-6** | code-unit `η²` ≈ 1 | read §6.5 / `report.py` | `ω²` primary, reason stated, both printed | **CLOSED** |
| **m-7** | two citation slips | `awk 'NR>=22&&NR<=28'` on `extract.py`; `awk` on `fit.py` | `24 GAME = 0`, `25 MOVES, KEY_FULL, TO_MOVE`, `26 SCORE_KIND, SCORE_VALUE…` — **`:24-26` is now exact**; `fit.py:119` is `signed()`'s swap line | **CLOSED** |
| **m-8** | own/opp equivariance | my own enumeration over all 59 049 patterns | **0 mismatches, 17 swap-fixed, 187 orbits** — §5.3 exact | **CLOSED** |

### Round 2 (`hex_threat_enum_v1_CONFIRM.md`)

| # | the finding | MY attack | output | verdict |
|---|---|---|---|---|
| **N-1** (BLOCKING) | the criterion is passed at 1.77x by a value-free stone-count quotient | built the count-only arm myself and ran it over the full corpus at all four lengths; then attacked the REPLACEMENT criterion with two value-free constructions | **the quotient is now the referent and the enum LOSES to it at 12 of 16 cells** — the named quotient can no longer pass. But the replacement statistic is class-count-monotone: **pure noise with 1000 buckets scores `ω² = 0.006179` against the referent's `0.004894` — 1.263x, MET** | **CLOSED BUT MOVED** → **P-1** |
| **N-2** | §5.5's two claims use two incompatible `DEF-PLAN` readings | my own `DEF-T`, both readings × both populations, all 59 049 patterns | rule-4 excluded: **27 of 335 classes, 5 348 of 57 996** under BOTH readings (the exclusion makes V1 and V3 agree); no exclusion, V1: **32 of 357, 6 099 of 59 049**; max-`t` distribution `{0: 319 606, 1: 32 002, 2: 2 686}` over 354 294 pairs under V1. **Every published number reproduces and the readings no longer conflict** | **CLOSED** |
| **N-3** | *"not something a single-axis class can hold"* is false | added the seventh component myself, four ways | the false sentence is gone. §5.5's **357 → 384 / 121 → 132 / 47 → 58 / 36 → 45** reproduces exactly under one construction (rule-4 patterns collapsed to one sentinel). The confirmation's **393** is the same arithmetic with `t` computed at the rule-4 patterns too. **Both are right; the memo's is the one consistent with its own stated population** | **CLOSED** (residual: **P-8**) |
| **N-4** | T1's *"only if"* is false; the coarsest faithful clip is 816 | enumerated **every** faithful clip by set-partition search | the *"only if"* is deleted ✅ and all four listed rows reproduce (16/816, 25/2925, 25/2925, 36/8436). **But there are 8 faithful clips under the memo's own stated constraint and 20 under the header's, and the table lists 4** | **CLOSED in its sentence** → **P-2** |
| **N-5** | T3's clip is not licensed by max-`t = 2` | read §6.4's T3 row; recomputed the `t`-inconsistency cost per rung | the derivation is deleted and the row says *"A CHOICE, not a derivation"* ✅. Cost table reproduces exactly: **T4 27 / 5 348, T3 11 / 7 056, T2 11 / 11 164, T1 9 / 11 164** over the 57 996 | **CLOSED** |
| **N-6** (MAJOR) | gate 19 cannot see a constant `ω²`; 4 mutants survive | re-ran all four in my own scratch copy, plus 20 mutants of my own | **all four DEAD** (exit 1), and so are my `omega2 = 0.5` and `omega2 = eta2`. `report.py:59` computes the total partition-free and raises by name ✅ | **CLOSED for the statistic** → the class recurs at the referent: **P-3** |
| **N-7** | the merge curve is tagged DERIVED and no named instrument makes it | ran `hexenum.py --curve 11 7 9` read-only | *"one L7 class meets **3-104 (mean 32.4)** L11 classes; one L11 class meets **1-8 (mean 2.3)**"*; at L9 **2-36 (mean 9.4)** and **1-9 (mean 2.6)** — §7.2 verbatim, out of the named instrument, and tagged MEASURED | **CLOSED** |
| **N-8** | the null is not matched on the observed population | read §6.5; measured the realised counts | demoted to a FLOOR whose only claim is *"the enum is not noise"* ✅. Measured, the mismatch is real and now harmless: enum T4 realises 231 classes, its nulls 294-310 | **CLOSED** |
| **N-9** | residual D-706 lean; one of five pricing receipts is definitional | `/usr/bin/grep -n D-706`; read §6.6 | the two residual mentions stay (permitted). The substantive half is answered in substance: §6.6 says *"a matrix that read 'stabilisation length = 11, MEASURED' would be reading the definition back to itself"* | **CLOSED** |
| **N-10** | nothing in the artifact marks which unit the criterion covers | read `report.py:114-139` and a receipt | `report.py` is unchanged; the purity table still prints window and code rows together with a `unit` column and no criterion marker. The remedy is prose-only | **CLOSED in prose, artifact unchanged** — trivial, **P-9** |
| **N-11** | stale even-`L` prose in the instrument | `/usr/bin/grep -n "if even" tools/hex_enum/*.py` | `hexenum.py:75` still reads *"§5's 6-tuple, **or a pair of them if even**"*, and the key is still wrapped `tuple(sorted(keys))`. §10 claims *"stale even-`L` prose gone"* | **STILL LANDS** (trivial) — **P-9** |
| **N-12** (process) | §7 landed while its review was outstanding | read the banner and §7 | §7 now lands with the round that ran it ✅. The structure repeats one level up — §6.5's replaced criterion governs §7.4's run and was written, run and published in one act — but **D-591 commands exactly that ordering**, and §6.5 discloses the pilot it knew. Not held against the author | **CLOSED**, with the standing tension recorded |

**Score: 26 of 28 prior findings CLOSED, 1 trivial STILL LANDS (N-11), 2 CLOSED BUT
MOVED (N-1 → P-1, N-6 → P-3).** No arithmetic error anywhere in the enum — a third
independent implementation now says so.

---

## 2. NEW findings against revision 3

### P-1 — MAJOR (N-1's defect class, one step to the left). §6.5's replaced criterion is class-count-monotone, so its PASS side licenses nothing: **pure noise passes it at 1.263x**. The memo's stated reason for choosing `ω²` is measurably false at this `n`.

§6.5's justification for the statistic:

> `η²` rises with the class count whatever the classes mean … **`ω²` subtracts the
> between-group sum of squares a partition of that many groups earns from noise
> alone.**

**It subtracts about one per cent of that.** MEASURED, from the memo's own receipt,
`L = 11`, window unit, `n = 12 980 519`:

| partition | classes | `η²` | `ω²` | the correction `η²−ω²` | what a matched RANDOM partition of the same shape actually earns |
|---|---|---|---|---|---|
| enum T4 | 231 | 0.005522 | 0.005504 | **0.000018** | **0.001822** (my 25 replicates; the receipt's own three give 0.001732–0.001986) |
| COUNT-ONLY | 52 | 0.004898 | 0.004894 | **0.000004** | **0.001319** |

The degrees-of-freedom correction removes `0.000018` where the entitlement is
`0.001822`. The receipt proves the point on its own face: **its null replicates rise
monotonically with class count while carrying zero information** — 35 classes →
`ω² 0.00072`, 44 → `0.00076`, 113 → `0.00112`, 303 → `0.00190`.

*Reproducer* — `…/r3/vacuity.py`, `L = 11`, full corpus, window unit, against the
registered referent's `0.004894`:

```
T2 x value-free noise( 1):   22 classes  omega2 0.003119  ratio 0.637  FAILS   <- the published cell
T2 x value-free noise(16):  346 classes  omega2 0.004843  ratio 0.990  FAILS
T2 x value-free noise(32):  665 classes  omega2 0.005739  ratio 1.173  MET
T1 x value-free noise(64):  936 classes  omega2 0.005133  ratio 1.049  MET
CONTROL, PURE NOISE, no enum information at all:
      noise(1024) alone :  1000 classes  omega2 0.006179  ratio 1.263  MET
```

A random integer attached to each code — no window, no stone count, no structure —
**passes the registered criterion**. So does T2 padded with that integer, while T2
itself fails at 0.637. `process.md`: *"A criterion that is a property the named
defect class PRESERVES … passes vacuously."* §6.5's named defect class is *the
quotient throws away a distinction that carries value*; a noise partition throws away
every one and passes.

**WHAT THIS DOES AND DOES NOT REACH, because the distinction matters and the memo
deserves it.** The criterion is sound in the direction it FIRED and unsound in the
direction it did not. A FAIL still means *this partition is weaker than stone
counts*, so §7.4's twelve failures and the UNPRICED verdict stand. What is
unsupported is the PASS side, and the memo reads it twice:

- §7.4: *"THE ONLY RUNG THAT EVER BEATS STONE COUNTING IS T4"*;
- §7.1: *"§7.4 measures that **T4 is the only rung worth having**"* — which §7.4
  measures nowhere and which the vacuity result forbids.

**AND THE FIX IS ONE LINE, MEASURED.** §6.5 already names the right statistic in the
same paragraph — *"which is the correction the random-quotient referent estimates
empirically"* — and never uses it. State the criterion on the EXCESS (`ω²` minus the
partition's own matched-null mean). The census already computes that null for every
enum rung; it computes **none for the count-only arm**, which is the only thing
missing. Measured, `…/r3/excess.py`, 25 replicates, `L = 11`:

| partition | classes | `ω²` | own matched null | EXCESS | raw verdict | excess verdict |
|---|---|---|---|---|---|---|
| COUNT-ONLY | 52 | 0.004894 | 0.001319 ± 0.000188 | **0.003574** | referent | referent |
| enum T4 | 231 | 0.005504 | 0.001822 ± 0.000206 | **0.003682** | MET 1.125 | **MET 1.030** |
| enum T2 | 22 | 0.003119 | 0.000744 ± 0.000201 | **0.002375** | FAILS 0.637 | **FAILS 0.665** |
| T2 × noise(32) | 665 | 0.005739 | 0.004287 ± 0.000262 | 0.001451 | **MET 1.173** | **FAILS 0.406** |
| PURE NOISE ×1000 | 1000 | 0.006179 | 0.006133 ± 0.000112 | **0.000046** | **MET 1.263** | **FAILS 0.013** |

**And no re-run is needed**: I recomputed all sixteen cells on the excess statistic at
all four lengths (`…/r3/fairall.py`, my own walk, 12 replicates per arm) and **not one
verdict changes** — 4 MET, 12 FAILS, same cells. §7.4's table survives; only its
statement and its two positive readings need correcting.

### P-2 — MAJOR (N-4 / B-3's remedy). §6.4's clip table claims to be exhaustive and lists **4 of 8** faithful clips under its own stated constraint (4 of 20 under its header's), and *"the COARSEST faithful one"* is not the coarsest.

§6.4, the round-3 remedy paragraph:

> **THE CLIP LADDER, EXHAUSTIVELY, BECAUSE TWO REVISIONS GOT ITS LICENCE WRONG.** …
> **MEASURED at `L = 11`, every clip that expresses both boundaries**:

Four rows follow, headed by *"`{≤2 | 3–4 | ≥5 | dead}` — the COARSEST faithful one"*
at `k = 16` / `C(18,3) = 816`.

*Reproducer* — `…/r3/clips.py` enumerates every set partition of
`min_X ∈ {1,2,3,4,5,6,∞}` such that `{cost ≤ 2}` and `{cost ≤ 4}` are each a union of
blocks, then computes `k` at `L = 11` under my own tuple. There are **20**. All four
of the memo's rows reproduce exactly. Twelve of the other sixteen keep `dead` apart —
the constraint the memo's own T1 row states (*"the law CONSTRAINS the clip to refine
`{≤2 | 3–4 | ≥5 | dead}`"*) — leaving **8** clips under the memo's own reading:

```
{1,2 | 3,4 | 5,6,dead}   k=  9   C(k+2,3)=  165   <- COARSER than the memo's "coarsest", omitted
{1,2 | 3,4 | 5,6 | dead} k= 16   C(k+2,3)=  816   <- the memo's row 1
{1,2 | 3,4 | 5   | 6 | dead} k= 23  C(k+2,3)= 2300  <- OMITTED, and CHEAPER than both listed 5-block clips
{1   | 2   | 3,4 | 5,6 | dead} k= 25  C= 2925      <- the memo's row 2
{1,2 | 3   | 4   | 5,6 | dead} k= 25  C= 2925      <- the memo's row 3
{1   | 2   | 3,4 | 5 | 6 | dead} k= 34  C= 7140    <- OMITTED, cheaper than the shipped rung
{1,2 | 3   | 4   | 5 | 6 | dead} k= 34  C= 7140    <- OMITTED, cheaper than the shipped rung
{1   | 2   | 3   | 4   | 5,6 | dead} k= 36  C= 8436 <- the memo's row 4, shipped
{1   | 2   | 3   | 4   | 5 | 6 | dead} k= 47  C=18424 <- OMITTED (this is T2's own `min`)
```

Two defects, both in the paragraph written to close a twice-landing BLOCKING:

1. **The superlative is false under the header's own criterion.** The header says
   *"every clip that expresses both boundaries"* — `LAW-SUPPORT`'s two. `∞` is on the
   far side of both, so `{1,2 | 3,4 | 5,6,dead}` expresses both and is strictly
   coarser, at `k = 9` and `C(11,3) = 165`. It is coarsest only if `dead` must stay
   apart, which is `DEF-WINDOW`'s requirement and not `LAW-SUPPORT`'s — the memo
   states that constraint in the T1 row and drops it in the table's header. **One
   table, two criteria, and the header licenses the weaker one.**
2. **"EXHAUSTIVELY" is false either way**, and the omissions are not decorative: the
   table never shows `k = 23 / 2 300`, which is cheaper than both of the `k = 25`
   rows it does show, nor `k = 34 / 7 140`, which is cheaper than the shipped rung. A
   matrix pricing the row from this table is pricing from half a menu.

**No conclusion in the memo flips** — the *"only if"* really is false, T1 really is
off 816, and the 816-coincidence paragraph is sound under the dead-apart reading. The
defect is that a MEASURED completeness claim and a superlative are both wrong in the
remedy paragraph.

*Fix (deletion + one row)*: drop *"EXHAUSTIVELY"* and *"every clip"*, say *"the four
clips between the coarsest `DEF-WINDOW`-faithful one and the shipped one"*, and either
put `dead`-apart into the table's stated criterion or add the `k = 9` row.

### P-3 — MAJOR (N-6 / m-2's defect class, one step to the left). CI gate 19 cannot see a broken stone-count referent. **A `count_key` that returns a CONSTANT passes the whole suite** — and the referent is the entire content of round 3's criterion.

N-6 was fixed for `ω²`. The same coverage hole now sits under the object the fix
introduced. §6.7 self-check 12:

> **The stone-count referent is blind to what the tuple carries**: two patterns the
> tuple separates … must share one count-only key.

That is one-directional. *Blindness* is what an everything-merges referent has in
abundance; the check never demands the referent SEPARATE anything, and
`test_hex_enum.py:250` asserts exactly `unit.counts[a] == unit.counts[b] and a != b`.

*Reproducer* — my own mutation driver `…/r3/mut.sh`, one mutation at a time in a
scratch copy of `tools/`, restoring between each, `bash tools/hex_enum_tests.sh`:

```
baseline: exit 0
Y1 count_key ignores opp stones                      exit 0  ALIVE
Y2 count_key ignores own stones                      exit 0  ALIVE
Y3 count_key collides own/opp (radix too small)      exit 0  ALIVE
Y4 count_key is a constant (return 0)                exit 0  ALIVE
Y5 count_key sees POSITION (returns the raw code)    exit 1  DEAD
```

**The direction that survives is the direction that produces a wrong PASS.** A
referent weakened by any of Y1–Y4 scores lower, the enum then exceeds it, and §7.4
reports MET where the shipped referent reports FAILS. Gate 19 is green for all four.
(The shipped `count_key` is CORRECT — `count(1)*(L+1) + count(2)` is injective on
`(own, opp)` because both counts are `≤ L < L+1` — so no published number is wrong.
What is wrong is that nothing would tell you if it were.)

**And the statistic's own control is a range check, not a value check.** §6.7
self-check 11 says a partition whose means differ *"must report the hand-derived
values"*. For `ω²` the suite asserts only `0.0 < omega2 < eta2`
(`test_hex_enum.py:198-200`). Three formula mutants pass:

```
X3 df correction uses `classes` not `classes - 1`      exit 0  ALIVE
X4 mean_within denominator `count` not `count-classes` exit 0  ALIVE
X5 omega2 denominator `total` not `total + mean_within`exit 0  ALIVE
```

(These three are near-equivalent at the census's `n` — which is itself P-1's evidence
that the correction does no work there.)

*Fix*: add the other direction — the referent must SEPARATE a pair the count differs
on (`xxxxx.` against `xxxx..`, say) — and pin `ω²` to a hand-derived value, not a
range. Ten lines, in a file already 284 lines and under rule 9's cap.

**What I verified is genuinely fixed**: the four receipted mutants are DEAD under my
own driver, and so are two stronger ones the receipt does not carry
(`omega2 = 0.5` — the confirmation's actual M3 arm — and `omega2 = eta2`). Six of my
tuple mutants also die (`open` counting dead windows, `min` dropping the centre cost,
`r4` at the wrong threshold, `clip` folding dead into 5, `windows()` admitting
windows that miss the centre, `count_key` seeing position).

### P-4 — MAJOR. §7.4's heading — *"IT FIRES ON EVERY AFFORDABLE RUNG"* — is falsified by the memo's own §7.1 and §7.3 tables. **`L = 7` T4 is affordable and MET.**

§7.4's argument is: T4 wins, T4 is unaffordable, every affordable rung loses. Its
support is one sentence:

> **AND T4 IS THE RUNG §7.1 PRICES AT 1.07 OBSERVATIONS PER NOMINAL PARAMETER.**

§7.1 is careful — its heading is *"THE FULL TUPLE **AT THE COVERING LENGTH** IS NOT
AFFORDABLE"*. §7.4 drops the qualifier and prices T4 at every length with `L = 11`'s
number. From the memo's own §7.1 `k` column and §7.3 observation counts:

| L | T4 `k` | nominal `C(k+2,3)` | scored-cell observations | **obs per nominal parameter** | §7.4 verdict |
|---|---|---|---|---|---|
| 7 | 25 | **2 925** | 5 031 327 | **1 720.1** | **MET (1.014)** |
| 9 | 98 | **161 700** | 6 537 654 | **40.4** | **MET (1.108)** |
| 11 | 357 | 7 647 059 | 8 174 025 | 1.069 | MET (1.125) |
| 13 | 357 | 7 647 059 | 9 979 912 | 1.305 | FAILS |

§7.1's own two yardsticks are *"`eval_families` §A1 already kills `R-A1-L11` at 1.2
and Buro's safe-fit line is ≥ 20"*. **`L = 7` T4 clears both by three orders of
magnitude and `L = 9` T4 clears both**, and §7.3's own row for `L = 7` T4 — 856 codes
observed, median **174** observations, 84 at or below Buro's `≤ 4` line — is a density
the memo nowhere calls unaffordable. So there are two affordable-and-MET cells and the
heading says there are none. §10 repeats it (*"it says the affordable rungs are worse
than counting stones"*), and so does `wp22_phase2a_STOP_E.md`'s banner (*"every
affordable rung at every length"*).

**And §7.3's table cannot be used to check this**, because it prints only the `L = 7`
(T4, T2) and `L = 11` (all four) rows — the `L = 9` and `L = 13` rows, which are in
the receipts and which are exactly the ones that test the claim, are omitted from a §7
headed **NO SELECTION**.

**The claim is rescuable by two words and the memo may not use them without a
change.** Insert *"at a covering length"*: `L ≥ 11` is `eval_families` §0.1's covering
minimum, re-derived in §5 from `DEF-WINDOW` alone, and restricted to `L ∈ {11, 13}`
the sentence is true. But §9 says *"Nothing about … which length is chosen"* and §7.5
says *"no recommended length"*, so §7.4 as written is asserting on a length
restriction the memo elsewhere disclaims. Either the covering restriction is stated as
a premise of §7.4 (with §9 amended to say the memo restricts lengths but does not rank
them), or the heading and §10's sentence are corrected to *"every rung at the covering
length"*.

### P-5 — MINOR. §7.3 publishes a scored-cell density that contradicts its own receipt.

> Scored cells per position: **111.1** at `L = 7`, **180.6** at `L = 11` and `L = 13`.

```
$ /usr/bin/grep -n "scored cells per position" artifacts/wp22_phase2a/census_r3/*.txt
census_L7.txt:6:  111.1     census_L9.txt:6:  144.4
census_L11.txt:6: 180.6     census_L13.txt:6: 220.4
```

`L = 13` is **220.4**, and my own walk gives `9 979 912 / 45 271 = 220.4`. The
published 180.6 is `L = 11`'s, carried across. Nothing downstream uses it — §7.2's
*"18 % more window traffic per stone"* is computed on the window unit and is right
(`15 346 355 / 12 980 519 = 1.182`) — but it is a receipted output misquoted in the
section that reports it.

### P-6 — MINOR. Two stale statements in the header, one of which is D-692's own rule.

**(a) The declared governing revision does not contain the instrument.**

> **Governing revision**: `c5123c1` (`dev`), **the revision every file, line and count
> quoted below was read at** (D-692).

```
$ git ls-tree -r --name-only c5123c1 -- tools/hex_enum
(empty)
$ git log --oneline --diff-filter=A -- tools/hex_enum/hexenum.py
c9a89b8 tools(hex-enum): the threat enum, its corpus census and its seed pilot land behind CI gate 19
```

`tools/hex_enum/` did not exist at `c5123c1`, so §6's named instrument and every
number in §7 were not read there — §7 says so itself (*"at the revision that carries
this memo"*). This is round 1's M-6 defect class in its remaining half: the declared
governing revision is stale for the two sections a reader most needs to pin. **No
number is affected** — I re-derived §1.3 and §1.4 at HEAD and all of them hold
(89 805 by `awk '!/^#/ && NF'`; manifest digest `00f61780…f35968` matching
`book_v3_ledger.md:50`; `RECEIPT_research_2026-09.sha256` 7 of 7 OK with digest
`13fe5712…916dab`; `query.rs` byte-identical `d83ac01`↔HEAD with `:81` and `:142`
exact; all 13 calculus line anchors and all 15 verbatim calculus quotations present).
The label is wrong, not the numbers. *Fix*: name two revisions, or name HEAD.

**(b) *"REVISION 2 IS THE ONE FIX ROUND"*** stands at `:24` in a document whose banner
three paragraphs above says this is revision 3 under a grant that superseded the
one-fix-round cap. Stale.

**(c) §6.6 contradicts its own next paragraph.** *"Below `L = 11` an odd length always
omits at least one window through `c`, so `L → L+2` **always splits**"* — and the next
paragraph says *"Neither `class_L` nor `class_11` refines the other for `L < 11`"*,
which the shipped `--refine 7` and `--refine 9` confirm at all eight rung-length
pairs. A partition that "splits" another refines it. The upper half of §6.6's
one-line argument (`k(L) = k(11)` for odd `L ≥ 11`) really is forced by §5's reach;
the lower half (that `L = 9` has not already stabilised) is a contingent fact —
`k = 25, 98, 357` — that the reach argument does not deliver, and the memo tags the
whole claim DERIVED. This is B-1's derived/measured class in the sentence round 2 did
not re-open. Trivial to fix: *"and `k(7) = 25 ≠ k(9) = 98 ≠ k(11) = 357` is
measured"*.

### P-7 — MINOR. The N-6 mutation receipt is cited without a digest, and one of its four mutants is not the one it names.

§6.7 item 10 and §10 cite `artifacts/wp22_phase2a/n6_mutants/` as the evidence that
the four mutants are dead. D-483 wants a run's artifact cited **by digest**; §7 gives
one for `census_r3` and §6.7 gives none for this one. `artifacts/` is gitignored, so
the citation points at something that will not survive the directory.

And `run_mutants.sh`'s third mutant is labelled *"M3 eta2 and omega2 both 0.5"* while
its replacement pair is `eta2 = between / total …` → `eta2 = 0.5` — **`omega2` is not
touched**. The confirmation's M3 set both. The claim survives anyway: I ran the real
one (`omega2 = 0.5`) in my own copy and it is **DEAD**, exit 1. But a receipt whose
label overstates its own mutation is a receipt a reader cannot use.

### P-8 — TRIVIAL. §5.5's price table mixes two populations in one row, and its seventh-component construction is not stated.

The table's `k without` column is over the full 59 049-pattern space (357, 121, 47,
36); its last column, *"classes merging different `t` before"*, is over the
57 996-pattern rule-4-excluded space (27 of **335**, 11 of **109**, 11 of **40**, 9 of
**30**). A reader reads "27 of 357", which is the exact numerator/denominator
confusion N-2 named and §5.5's prose then avoids two paragraphs earlier. §6.4's copy
of the same table carries the population in its own sentence; §5.5's does not.

And the `k with` column is one of three defensible constructions, unnamed:

```
A  t computed at every pattern, rule-4 included : T4 393  T3 134  T2 64  T1 51   (the confirmation's number)
B  rule-4 patterns collapsed to one sentinel     : T4 384  T3 132  T2 58  T1 45   (the memo's, and the one its own
                                                                                  rule-4 exclusion implies)
D  the class space restricted to the 57 996      : T4 362  T3 120  T2 51  T1 39
```

*"under the reading above"* does not choose between them; *"k without = 357"* rules
out D, and nothing rules out A. One clause fixes it.

### P-9 — TRIVIAL. §10 says *"stale even-`L` prose gone"*; `hexenum.py:75` still says *"or a pair of them if even"*, and `tuple_of` still wraps §5's 6-tuple in a one-element tuple. N-10's artifact half is likewise unchanged: `report.py` prints window and code purity rows in one table with nothing marking which the criterion binds.

### P-10 — TRIVIAL. §7.3 calls T1's growth curve *"flat from 15 000 on"* where the receipt shows `470 → 535`, **+13.8 %** over three times the data, in the same block that calls T2's `+35 %` a climb.

---

## 3. What I attacked and it SURVIVED

This document is much further along than its two FAILs suggest, and most of what I
tried did not land.

1. **The whole enum, again, from scratch.** `k = 25 / 98 / 357 / 357` and the full
   ladder `121 / 47 / 36` at `L = 11`; `C(k+2,3)` at every cell of §7.1. **Third
   independent implementation, zero disagreements.** §7.1's table is right.
2. **THE FAIRNESS ATTACK THE DISPATCH PREDICTED WOULD LAND DOES NOT.** I recomputed
   all sixteen §7.4 cells on a statistic that corrects each arm by its own matched-null
   expectation, at all four lengths, from my own walk. **Not one verdict changes**:
   `L7 T4 1.014→1.151`, `L9 T4 1.108→1.192`, `L11 T4 1.125→1.067`, `L13 T4
   0.834→0.706`, and every FAILS stays FAILS. The 231-to-52 class-count ratio at
   `L = 11` is real and is not what produces the result. (`ω²`'s df correction is
   nonetheless inoperative — that is P-1, and it is a defect in the STATEMENT, not in
   the verdict.)
3. **The full census, independently walked.** My own manifest parse, R6 filter,
   mover-relative line projection, scored-cell rule and moments — sharing no code with
   `tools/` — over all 45 271 quiet rows at `L = 7, 9, 11, 13`. Every receipt cell
   reproduces: `kept 45 271`; scored cells `5 031 327 / 6 537 654 / 8 174 025 /
   9 979 912`; window `n` `8 241 249 / 10 613 918 / 12 980 519 / 15 346 355`; all
   sixteen enum `ω²` and all four COUNT-ONLY `ω²` to six decimals; all twenty realised
   class counts. **§7.3 and §7.4's numbers are right.**
4. **The criterion's honesty about its own pilot.** §6.5 discloses `0.005031` against
   `0.004840` at `L = 7` / 3 000 positions, 1.04x. I reproduced both to six decimals
   with my own walk at that exact scope. A criterion registered in knowledge of a
   near-miss, saying so, is better practice than one registered in silence.
5. **The negative conclusion.** *"`R-A4-CLASS` is UNPRICED at the affordable rungs"*
   survives every correction I could apply, and one of them **strengthens** it — see
   §5.
6. **`count_key` is genuinely structure-free.** `count(1)*(L+1) + count(2)` over the
   `L`-cell pattern is the finest function invariant under arbitrary permutations of
   the non-centre cells, so *"the strongest partition that knows nothing the calculus
   names"* is true under the only reading that makes it precise — though the memo
   never states that reading, and a superlative without one is the D-291 shape.
   §6.7 item 12's witness pair reproduces: `xxxxx.` and `x.x.x.x.x..` share key `60`
   with tuples `(1,6,True,6,1,False)` and `(3,6,False,inf,0,False)`.
7. **N-2 and N-3, the two claims the confirmation called mutually inconsistent.** They
   are now consistent, and I proved it the hard way: under the rule-4 exclusion the
   two `DEF-PLAN` readings **coincide** (both give 27 / 335 / 5 348), while the max-`t`
   distribution over all `3^11` lines reproduces exactly and only under the reading
   §5.5 states. One reading, two populations, both stated. That is a clean fix.
8. **`max t = 2` on one axis**, over 177 147 lines and 354 294 (line, side) pairs, with
   the memo's exact distribution `{0: 319 606, 1: 32 002, 2: 2 686}` — recomputed from
   my own hitting-set solver.
9. **`|W_L| = min(6, L−5)`**, reversal invariance (0 of 59 049), swap equivariance
   (17 fixed, 187 orbits), `r4`'s redundancy, T2's boolean (47 = 47), the folded
   raw-code ceiling (378 / 3 321 / 29 646), the window-containment count (0/0/0/2 of
   8), `C(1371,3) = 428 558 605`, and the ladder-is-a-chain property — every one
   reproduces.
10. **Fail-loud.** All eight even-`L` entry points refuse by name, `--tranche` without
    a placeholder refuses, an unknown argument refuses, and a non-reversal-invariant
    class table refuses. Rule 9: every file under `tools/hex_enum/` is under the
    ~300-line cap (149–284, and the runner at 46), so no justification is owed.
11. **THE NEW REPLICATION PARAGRAPH (`a5687d6`, unreviewed by anyone else).** I diffed
    `artifacts/wp22_phase2a/census/` against `census_r3/` field by field. **232 shared
    rows, 0 differences**; the only rows present in `r3` and absent in `census` are the
    four COUNT-ONLY rows per length, as they must be. The memo's *"200 cells"* is exact
    under its own enumeration (4 `k` + 10 observation + 34 purity + 2 totals, × 4
    lengths). *One quibble*: the paragraph cites `process.md`'s replication clause,
    which asks for **a second instrument whose agreement criterion is registered before
    either runs** and warns that *"two instruments blind to the same stage are one
    instrument reported twice"*. This is one instrument executed twice, compared post
    hoc. The paragraph's own words (*"two independent executions"*) are accurate; the
    citation over-reaches. (The memo does have a genuine second instrument —
    `seed_pilot.py` in §1.4 — and now a third, mine.)

---

## 4. WHERE THE MEMO **UNDER**-READS, which the dispatch asked about

There is a stronger conclusion in this data and the memo declines to draw it, on the
grounds of a caveat that a measurement retires.

§7.4 disclaims: *"It does not say a threat-class eval cannot work … a partition can
carry decision-relevant structure that a marginal variance decomposition cannot see."*
That is a fair caution about a MARGINAL comparison of two non-nested partitions —
which is what the criterion is. **The NESTED question is the one the memo's own §6.5
sentence claims to answer** (*"the gap between it and the enum is exactly what threat
structure buys on this corpus"*, which is not what a marginal gap measures), and it is
one join away.

MEASURED, `…/r3/joint.py`, `L = 11`, full corpus, window unit, excess over each
partition's own matched null:

| partition | classes | `ω²` | matched null | **EXCESS** |
|---|---|---|---|---|
| COUNT-ONLY alone | 52 | 0.004894 | 0.001352 | **0.003542** |
| COUNT-ONLY **×** enum T4 | 812 | 0.007637 | 0.004075 | **0.003563** |
| COUNT-ONLY **×** enum T3 | 435 | 0.006925 | 0.003543 | 0.003382 |
| COUNT-ONLY **×** enum T2 | 233 | 0.006470 | 0.003024 | 0.003446 |
| COUNT-ONLY **×** enum T1 | 195 | 0.006082 | 0.002995 | 0.003086 |

**Adding the full tuple to stone counts buys `0.000021`, against a replicate spread of
`±0.0002`.** At every coarser rung the join measures no better than counts alone. So
on this corpus, at this unit, the enum's classes carry **no label information that
stone counts do not already carry** — which is a stronger and cleaner statement than
*"the affordable rungs score lower"*, is not vulnerable to the class-count objection
P-1 raises against the marginal form, and would let §7.4 say what it wants to say
without leaning on the PASS side at all.

It also explains T4's small marginal win: `1.014`–`1.125` is what 231 classes buy over
52 when the extra classes are informative about nothing counts did not already say.

I record this as a finding the memo could claim and does not — not as a defect. One
census arm and one paragraph.

---

## 5. VERDICT: **FAIL**

A round PASSes when every prior finding is CLOSED and the round introduced no new
BLOCKING or MAJOR. The first half nearly holds — **26 of 28 CLOSED**, one trivial
survivor, no arithmetic error anywhere in three independent implementations. The
second does not: **four new MAJOR**, three of them in text no round has reviewed and
one of them the same defect class as the BLOCKING that failed round 2.

**New at MAJOR**: P-1 (the criterion's PASS side is passed by pure noise, and §6.5's
reason for `ω²` is measurably false), P-2 (the clip table is neither exhaustive nor
headed by the coarsest faithful clip), P-3 (gate 19 cannot see a constant referent),
P-4 (§7.4's heading is false on the memo's own tables). **New at MINOR**: P-5, P-6,
P-7. **Trivial**: P-8, P-9, P-10.

**Not overruleable (D-424).** P-2 and P-4 each name a false claim a matrix would price
from; P-3 names a way the package can produce a wrong answer with a green gate; P-1
names a positive reading the instrument does not support. D-424 reaches prose that
constrains nothing; none of these is that. P-5 through P-10 are corrections, and
P-9/P-10 would be fair overrule candidates if anyone wanted them.

### The shortest route, and it is a deletion or a correction in every case

Nothing here needs a re-run. The census stands; I reproduced it independently at four
lengths and it replicates against round 2's own run.

1. **P-4 — two words, or one sentence.** Either qualify §7.4's heading and §10's
   closing sentence to *"every rung at the covering length"* (and say in §7.4 that
   `L ≥ 11` is §5's covering bound, amending §9 to match), or restate them as *"every
   affordable rung at `L = 11` and `L = 13`"*. Add the `L = 9` and `L = 13` rows to
   §7.3 so a reader can check the density claim the argument turns on.
2. **P-2 — delete two words, add one row.** Drop *"EXHAUSTIVELY"* and *"every clip
   that expresses both boundaries"*; either state `dead`-apart as part of the table's
   criterion or add `{≤2 | 3–4 | ≥5 or dead}` at `k = 9` / `C(11,3) = 165`.
3. **P-1 — one statistic, one census arm, no re-run.** State the criterion on the
   EXCESS (`ω²` minus the partition's own matched-null mean) and give the count-only
   arm the null every enum rung already gets — `census.py` builds nulls in one
   comprehension at `:101`. Every verdict is unchanged (measured, all sixteen cells,
   both statistics). Then delete §7.1's *"T4 is the only rung worth having"*, which
   §7.4 does not measure. Deleting §6.5's *"`ω²` subtracts the between-group sum of
   squares a partition of that many groups earns from noise alone"* is required either
   way: it is false at this `n`.
4. **P-3 — ten lines of test.** One assertion that the referent SEPARATES a pair whose
   stone counts differ, and one hand-derived `ω²` value in place of the range check.
5. **P-5, P-6, P-8** — three corrections and one deleted stale sentence.

### What is finished, and it is most of the document

§1's premises and citations (re-derived at HEAD with my own scopes, all of them).
§2, §3, §3.1's even-`L` refusal and its measured ground. §4. §5's tuple, its range,
its reversal invariance, its swap equivariance. §5.5 entire — the merge, both
populations, both `DEF-PLAN` readings reconciled, the max-`t` fact, and the priced
repair. §6.1, §6.2, §6.3. §6.4's cost table. §6.7's items 1–7, 10 and 13. §7.1, §7.2
and §7.3's numbers. §7.4's sixteen cells, which survive the fairness attack that was
expected to break them. The receipts, which verify. **The enum is arithmetically
sound and the census is trustworthy; what fails is, for the third round running, the
sentences written to explain them.**
