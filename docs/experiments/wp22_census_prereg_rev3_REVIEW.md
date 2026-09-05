# WP-2.2 §B census pre-registration — fresh-context REVIEW, revision 3. Verdict FAIL.

**Reviewed**: `docs/experiments/wp22_census_prereg.md`, revision 3 (its first line
states the revision), at named revision `833b4ed` on `dev`.

**Does it match HEAD?** **Yes.** `git rev-parse HEAD` = `833b4edf47cf…`. The tree
is clean. The document names `0097f83` as its governing revision;
`git diff --stat 0097f83..833b4ed` is two files — `docs/decisions.md` (+2) and
this document (+83/−40) — and `git diff --stat 0097f83..833b4ed -- crates tools
configs Cargo.toml Cargo.lock` is **empty**, so every instrument is byte-identical
at the stated governing revision and at HEAD.

**VERDICT: FAIL.** Four MAJOR, thirteen MINOR.

**THE REVERSAL HOLDS, AND THAT IS THE HEADLINE.** I re-derived it end to end and
attacked it four ways. `p1 = 12/14` really is `knapsack_bound`'s bound over
`stage3_allocator_bound.py`'s nine-`COLUMNS` classes; round 1's M2 was right and
D-619 concedes it correctly. The pilot reproduces to the digit (62 classes, curve
`11 18 37 47 62`, crossing 28 at n = 268); round 1's null-family figures also
reproduce to the digit (12, curve `8 9 10 11 12`), so D-619's claim to have
replicated the round-1 run is true. The crossing is **not** an ordering artifact —
my own 2 000-draw resampling reproduces the receipt's table — and the class count
is **not** saturating: Chao2 on the class partition puts the asymptote at **489
(bias-corrected) to 641 (classic)**, against 15–18 for the null's family. A
conservative bootstrap gives **P(≥ 28 classes at n = 800) = 1.0000** over 2 000
draws with a 5th percentile of 41. **Round 1's M1 is correctly dissolved and the
run is runnable.** I did not manufacture a way to resurrect it.

**Why it still fails.** The document does not say, in one voice, which count
adjudicates: §1, §2 and §7 apply the floor of 28 to `w`, the KEY count, while §6
and D-619 say the CLASS count licenses — and the word "class" **never appears in
§7 at all**, whose report list still registers the "DISTINGUISHABLE-TRIAL count",
the term D-619 retired. That is round 1's M3 relocated, not removed. Underneath
it, the sole ground offered for the tightening — "`roots <= classes <= keys`
always" — is not a theorem: I have executable counterexamples to **both** halves,
and the mechanism behind one of them is present in the pilot data. And §6's
independence claim is refuted by measurement: 28 classes are reachable from as
few as **7** root searches, and in the class currency D-537's own inputs need
**n = 42–150**, not 28. All four fixes are small; none reopens the reversal.

---

## 0. What I ran, and what it consumed

Everything at `833b4ed` with the prebuilt
`target/release/examples/{trigger_census,fixture_key_full}` and the committed
`tools/texel/census_classes.py`. **No `cargo` was run in the live tree** (D-592).

**The governed census slice (rows 600 onward) is untouched by this review.** The
only search I executed was a 4-position replay of the **calibration** slice (rows
100..599 — the sample D-618 retired unrun, and the one the pilot already
consumed), for the provenance check in §1.5. 15.3 s.

Everything else is a re-tally of data already on disk, which is the instruction I
was given and also the right instrument: the question under review is what a
count does as `n` grows, and that is answered by the 600 positions already
censused, not by 600 more.

**Marked limit.** My re-tallies are a REPLICATION of the author's arithmetic over
the author's data, plus one independent 4-position replay of the binary. They are
blind to anything `trigger_census` itself gets wrong. The one thing that would
need a second instrument — whether the census rows describe the positions they
claim to — is discharged by §8's `fixture_key_full` check, which I ran (below).

---

## 1. Re-derivation ledger

Scripts written from the row format, not copied from `census_classes.py`; my
parser steps by two and asserts all sixteen row fields are present and that the
dict has exactly sixteen entries before it counts anything.

### 1.1 The reversal's load-bearing chain

| # | claim | my command / check | result | reproduced? |
|---|---|---|---|---|
| 1 | `p1 = 12/14` is `knapsack_bound`'s bound over the nine `COLUMNS` | read `matrix_stage3_detector.md` §5.8 + `stage3_allocator_bound.py` | §5.8's table: trigger-rich "bound over the columns" **0.857 (12 of 14)**; printed by `knapsack_bound`, which partitions on `COLUMNS` and treats a class as one knapsack item | **YES** |
| 2 | that file states the quoted principle | `stage3_allocator_bound.py` module docstring | quoted verbatim and in context | **YES** |
| 3 | the null's family under-counts 2.6x at n=100 (10 vs 26) | my own 13-predicate tally + `census_classes.py` on `d3_c2048.txt` | **10** and **26**; 26/10 = 2.6 | **YES** |
| 4 | pilot at n=500: 104 keys / 36 roots / 62 classes / 91 loss keys, 5 935 firings | `census_classes.py` **and** my independent tally | identical on both | **YES** |
| 5 | class curve `100:11 200:18 300:37 400:47 500:62` | my tally, binned by `entry` | identical | **YES** |
| 6 | crosses 28 between n=200 and n=300 | my per-entry accumulation | crosses at **n = 268** — round 1's own figure | **YES** |
| 7 | round 1's null-family run replicates (12; `8 9 10 11 12`) | my 13-predicate tally on the pilot | **12**; `100:8 200:9 300:10 400:11 500:12` | **YES** |
| 8 | rates: roots held 0.080→0.072, classes fell 2.1x, keys 4.1x | 0.26/0.124 = 2.10; 0.86/0.208 = 4.13 | identical | **YES** |
| 9 | 3.31 s/position; n=800 ≈ 44 min | pilot `WALL=1654`/500 = 3.308; 800 × 3.31 = 2 648 s = 44.1 min | identical | **YES** |
| 10 | the floor itself: `p0=8/14`, `p1=12/14`, α=β=0.05 → n=28, c=21 | my own binomial search | **n=28, c=21**, size 0.040184, power 0.962225 — matches `overnight2_ledger.md` §4 to six decimals | **YES** |

### 1.2 The reversal, attacked

| attack | what I did | result |
|---|---|---|
| **is the curve an ordering artifact?** `curve()` bins by `entry`, and `entry` is fixture order | 2 000 random orderings of the same 500 positions, per n | mean 13.0 / 25.6 / 38.0 / 50.0 at n = 100/200/300/400; P(≥28) = **0.008 / 0.376 / 0.948 / 1.000**. Reproduces the RECEIPT's table (13.1 / 25.9 / 38.8 / 50.6; 0.01 / 0.42 / 0.95 / 1.00). **The crossing is not an ordering artifact.** The fixture order is `sha256(key_full)`, content-derived, so it is a random order and not an adversarial one |
| **is it really non-saturating, or a slow saturation?** | Chao2 incidence, positions as sampling units, on the CLASS partition | S_obs = 62, Q1 = **59**, Q2 = 3 → asymptote **488.9** bias-corrected, **641.0** classic; extrapolated **96 at n=800**, 207 at n=2 000. Contrast the null's family: 15.3–18.2. **Genuinely non-saturating** — 59 of 62 classes were seen in exactly one root position |
| **what is P(fail to reach 28 at n=800)?** | bootstrap, 2 000 resamples of 800 positions **with replacement** from the 500 (conservative: it cannot discover a class absent from the 500) | mean 49.9, p5 **41**, **P(≥28) = 1.0000**. At n=500 it is already 0.9745 |
| **did the pilot consume governed sample?** | redrew all three slices; set intersections in Python on both fixture lines and `expected_key_full` | dry/calib/census = 100 / 500 / **89 205**; **all three pairwise intersections are 0**, on fixture lines *and* on `key_full`, so no symmetry-duplicate group straddles the row-600 boundary. `dryrun` draw is byte-identical to `d3.txt` |
| **is the pilot really the calibration slice?** | re-ran `trigger_census --nodes 50000 --cap 2048 --gate on` on the first 4 drawn calibration positions and diffed entries 0..3 against the pilot | **byte-identical**, 40 rows. The pilot's provenance and the seat's determinism are both confirmed |

### 1.3 §4, §7, §9 — mechanical

| claim | result |
|---|---|
| 86 keys from 8 roots, `[35, 33, 11, 3, 1, 1, 1, 1]`, "28 keys reachable from ONE search tree" | **YES**, all four; largest root supplies 35 |
| truncation "71.2 % at this cap on the dry-run slice" | **YES** — 828 at-cap attacker invocations of 1 163 = 71.2 % (round 1 described the denominator as "unproved invocations"; it is *all* attacker invocations — 1 066 are unproved. The number and §7's own definition agree) |
| manifest digest vs `arc3_ledger.md` §5 | **YES** |
| §8's pre-run check on the **whole** census fixture | `fixture_key_full` over 89 205 rows: **0 disagree, 2.28 s**. §9's VOID-AND-REPAIR condition is well-founded and currently satisfied |
| `--nodes 50000`, `--cap 2048`, `--gate on` is a real seat | **YES**, replayed |

### 1.4 What does NOT reproduce

| claim | what I got |
|---|---|
| §6: "**`roots <= classes <= keys` always**" | **false as stated, both halves** — see M2 |
| §6: the class count is "the count the floor's binomial actually assumes" | **false** — see M4 |
| §6b: `draw_census_samples.py` and `fixture_key_full.rs` at revision `0097f83` | both last changed at **`42967e0`**; `0097f83` did not touch either (bytes identical, so immaterial) — m1 |
| §6b: the test "pins its partition **against `stage3_allocator_bound.py`'s**" | it pins it against a **hard-coded copy** of the literal; the two can diverge silently — M3 |
| intro/§4: "distinguishable-trial rate **0.10 / 0.07 / 0.08** … spread of **1.4x**" is "the quantity this census exists to produce" | on the partition D-619 **corrected to**, the rates are **0.26 / 0.14 / 0.15**, spread **1.86x** — m2 |
| §8: the dry-run criterion "passed **0 of 20**" | `wp22_cap_prereg.md` §9 line 226 and `artifacts/wp22_cap_dryrun_v2/RECEIPT.md` line 11 both say **0 of 100** — m3 (round 1's m2, not landed) |
| §5: "the curve is still **accelerating** at 500" | increments are `11, 7, 19, 10, 15` — lumpy, not accelerating — m4 |
| §3: the census slice is "**89 205 positions**" | 89 205 rows; **87 228 distinct `key_full`**, the identity §2 counts in — m5 |

---

## 2. MAJOR findings

### M1 — the document states its criterion two ways, and §7 never names the licensing count at all

**What is wrong.** D-619 (3) is unambiguous: *"the CLASS count is the registered
minimum and is what licenses detector round 3."* §6 says the same. But:

- §1: *"**One number: how many WIN-PROVING FIRINGS ON DISJOINT POSITIONS** the
  sweep corpus yields, counted against D-537's registered floor of **28**."*
- §2: *"`w` = the number of **distinct `key` values** … carrying at least one
  census row with `att_proved true`."*
- §7, first bullet: *"`w` against the floor of **28**, and whether the floor is
  cleared."*

So the closure line adjudicates the KEY count against 28 while §6 licenses on the
CLASS count, and both are called "the floor of 28". Worse, **`/usr/bin/grep -n
"class\|CLASS"` returns nothing in §7**: the class count is not among the
registered outputs under any name. What §7 *does* register is *"the
**DISTINGUISHABLE-TRIAL count**"* and *"the cumulative **trial** curve"* — and
"distinguishable trial" is D-618's term for the thirteen-predicate quantity
**D-619 retired as the null's family**. A successor executing §7 literally
reports 12-style numbers and never reports the 62-style number the licence turns
on.

**Why it matters.** This is the one thing a pre-registration exists to fix, and
it is live precisely in the outcome §5 flags as interesting. The discriminating
case is `keys ≥ 28 > classes` — on the pilot's own rates that is roughly what
n = 200 looks like (22 keys, 18 classes) and it is not exotic. There, §7 bullet 1
says the floor is cleared and §6 says round 3 is not licensed, and §7's closing
bullet — *"Detector round 3 licensed, or the shortfall stated"* — supplies no rule
to choose. Round 1 charged this as M3 against revision 2; revision 3 answered it
in §6 and left §1, §2 and §7 in the old voice.

**Minimal fix.** Three edits, no new measurement:
1. §7 bullet 1 → *"the CLASS count against the registered minimum of 28, and
   whether it is cleared; `w` beside it as D-537's literal figure, never
   suppressed."*
2. §7 bullet 4 and 5 → replace "DISTINGUISHABLE-TRIAL count" and "trial curve"
   with "COLUMN-CLASS count" and "class curve", the names §6 and `census_classes.py`
   use.
3. §1 → say the run produces three counts and that the class count is the one the
   licence reads.

### M2 — "`roots <= classes <= keys` always" is the sole ground for the tightening, and it is not a theorem; both halves are falsifiable and one mechanism is present in the pilot

**What is wrong.** §6: *"**`roots <= classes <= keys` always**, which is what
makes this a tightening rather than a change of unit."* That word is doing all
the work: D-537 forbids a successor to LOOSEN its minimum, and the ONLY reason
28 classes is not a loosening of 28 keys is `classes <= keys`. Neither inequality
follows from anything.

**Executable counterexamples**, both accepted by the committed instrument:

- **`roots <= classes` is false.** Two roots whose win-proving firings all land
  in one class give roots = 2, classes = 1. `census_classes.py` on a two-row
  synthetic fixture prints `ROOTS 2 / CLASSES 1`. **This is not hypothetical**:
  in the real pilot, **3 classes are each shared by 2 roots**, so the mechanism
  fires — the inequality survives there only because other roots carry several
  classes each.
- **`classes <= keys` is false**, and this is the load-bearing half. `turns` is
  `TriggerColumns::turns_from_root` — *"Turns from the search root. 0 is the
  root's own firing"* (`crates/pistol-search/src/census.rs`) — so it is **not a
  function of the position**. One canonical key reached from two roots at
  different depths carries two `turns` values and therefore two classes.
  `census_classes.py` on that synthetic fixture prints `KEYS 1 / CLASSES 2`.

**What I measured, honestly, about how likely it is to bite.** Over the 600
censused positions (pilot + dry-run slice), **6 366 distinct in-tree keys, and
not one is seen under more than one root; not one carries more than one `turns`
value.** Class-to-key ratios are 62/104 and 26/86, so there is large headroom.
**I am not claiming the governed run will invert the inequality.** I am claiming
the registration asserts a theorem it does not have, for the one clause that
keeps it inside D-537, and registers no check that would catch the inversion if
it happened.

**Why it matters.** If `classes > keys` on the governed sample, the registration
licenses round 3 on fewer than 28 disjoint positions — a LOOSENING, which D-537's
condition (1) forbids and which no operator has ruled on. CLAUDE.md is explicit
that a finding naming a way the work can produce a wrong answer is fixed, never
overruled. The fix is one registered guard.

**Minimal fix.** Two sentences:
1. §6: replace "always" with the truth — *"`classes <= keys` because every column
   but `turns` is a function of the position, and `turns` is root-relative;
   MEASURED over 600 positions and 6 366 in-tree keys, no key carries two `turns`
   values."* Delete `roots <= classes`, which licenses nothing and is false.
2. §9 or §7: register the guard — *"`classes > keys` on the governed run VOIDS
   the tightening: the licence falls back to `w` against 28 and the inversion is
   reported."* `census_classes.py` already computes both numbers.

### M3 — the instrument answering round-1 M5 does not satisfy the coverage rule it was raised under, and §6b mis-describes what it checks

**What is wrong.** §6b: *"All four are tracked and **the class counter is
tested** (`tools/texel/test_texel.py` pins its partition **against
`stage3_allocator_bound.py`'s** and the `roots <= classes <= keys` ordering)."*
Four things:

1. **No gate runs it.** `tools/ci.sh` defines twenty gates (`GATE_TOTAL=20`) and
   `/usr/bin/grep -n texel tools/ci.sh` returns **nothing**. `tools/SHELL_CHECKLIST.md`
   §10, which `docs/process.md` calls *"the binding one"*, does not say "carries a
   test" — it says *"a test **in a suite CI runs**, driving the SHIPPED script …
   **with a control run** so a pass cannot come from a gate that refuses
   everything."*
2. **It self-skips into a pass.** `test_census_classes_partition` reads the
   relative path `artifacts/wp22_cap_dryrun_v2/d3_c2048.txt`; `artifacts/` is
   gitignored (rule 8), so on any clean checkout — the one place a CI would run
   it — it prints `ok census fixture present (skipped, artifact absent)` and exits
   0. I ran it from an empty directory and it did exactly that. That is
   skip-with-default, which hard rule 3 forbids.
3. **It does not pin the partition against `stage3_allocator_bound.py`.** It
   asserts `CC.COLUMNS == ("turns", …)` — a **hard-coded second copy of the
   literal**. If `stage3_allocator_bound.py`'s `COLUMNS` changed, both the
   allocator's bound and the census partition would diverge and the test would
   stay green. The one invariant that matters here — that the census counts the
   partition `p1` is computed over — is the one the test does not check.
4. **No control run**, and `main()` — which prints every recorded number and the
   curve — is never driven; only `tally()` and `curve()` are imported.

**Why it matters.** M5 was raised because *"a number nothing tests is a number
nothing defends"* (SHELL_CHECKLIST §10, quoting D-220/D-231). Revision 3's answer
is a test that no gate runs, that passes vacuously without a gitignored file, and
that checks a copy of a literal instead of the literal. §6b's sentence is the
document's evidence that M5 is discharged, and it is not accurate.

**Minimal fix.**
1. Make the test import `stage3_allocator_bound` and assert
   `CC.COLUMNS == SAB.COLUMNS`. One line; it is the only assertion that pins the
   reversal's premise.
2. Replace the artifact dependence with a **committed synthetic fixture** (a
   dozen rows in the test's own tempdir), including a control row set where
   `classes > keys` so the ordering check can be seen to fail. That also removes
   the skip.
3. Add it to `tools/ci.sh` as a gate and bump `GATE_TOTAL`, or state in §6b that
   it is a hand-run check and not gated. Either is honest; the present sentence is
   not.

### M4 — §6's independence claim is refuted by measurement, and if believed it under-powers round 3 by 1.5x to 5.4x

**What is wrong.** §6: *"**Distinct COLUMN CLASSES** — the partition `p1 = 12/14`
is defined over, **and therefore the count the floor's binomial actually
assumes**."* §5 sets it up: the binomial *"assumes independent trials. **The
counted unit does not deliver them**."* The implication is that the class count
does. Two separate errors, both measured.

**(a) `p1` is not a fraction over classes.** `knapsack_bound` partitions on
`COLUMNS` and chooses whole classes, but its numerator and denominator are
**firings**: `classes[key][0] += 1 if won(row)`, `max(table)` over win counts,
divided by `len(wins)`. On the trigger-rich band `artifacts/stage3c_allocator_bound_v1.txt`
prints *"BOUND over the census COLUMNS: 12/14 = 0.857 (83 distinct
column-classes, **8 holding a win**)"*. So 14 win-proving firings sit in **8**
win-holding classes, with wins distributed **`[2, 2, 2, 2, 2, 2, 1, 1]`**
(MEASURED, my tally of the same file). Reaching 12 wins requires exactly the six
2-classes, so **in the class currency the alternative is 6/8 = 0.750, not
0.857.** The class is the score's *action space*; it is not the unit the
probability is a fraction of.

**(b) Counting classes does not restore independence — it dents it.** Two
firings in one class are indistinguishable to any column-score, so collapsing
them is right and necessary. But two firings in *different* classes from the
*same root search* remain correlated in the ordinary statistical sense, and the
class partition is far too fine to remove that. MEASURED on the pilot: **28
classes are reachable from as few as 7 root searches** (greedy over the
classes-per-root distribution `[7, 5, 4, 4, 4, 3, 3, 3, 2, 2, …]`). The defect
D-618 called a correctness finding — *"28 keys are reachable from ONE search
tree"* — becomes "28 classes from 7 trees". Better; not independence.

**The consequence, quantified.** If a round-3 registration takes §6 at face value
and runs a one-sample binomial on 28 classes against `(p0, p1) = (0.571, 0.857)`,
it is using firing-currency probabilities on class-currency trials. Re-solving
D-537's own rule in the class currency on the reference band — p1 = 6/8 MEASURED,
p0's class image ESTIMATED at 4/8 (its most favourable value) or 5/8 — gives:

| currency | (p0, p1) | smallest n, c |
|---|---|---|
| firings (the ledger's) | (8/14, 12/14) | **28, 21** (size 0.0402, power 0.9622 — reproduces §4 exactly) |
| classes, p0 image 4/8 **ESTIMATED** | (0.500, 0.750) | **42, 27** |
| classes, p0 image 5/8 **ESTIMATED** | (0.625, 0.750) | **150, 104** |

So §6's sentence, if believed, licenses round 3 on **1.5x to 5.4x too few**
trials. That is not prose: it changes what a successor may conclude, which is
CLAUDE.md's own test.

**What this does NOT do.** It does not reopen the reversal and it does not make
the run unrunnable. The registration is still **safe as a tightening** — 28
classes implies ≥ 28 keys (M2's caveat aside), so D-537's floor is met in
D-537's own unit a fortiori. Only the stated *reason* is wrong.

**Minimal fix.** Replace the "and therefore" clause with the accurate ground, and
register the diagnostic that makes it checkable:

> *"— the finest partition any column-score can act on, so firings inside one
> class return one verdict to every hypothesis the alternative can entertain.
> Counting classes is therefore a conservative floor on how many firings the test
> can tell apart. **It does not make the trials independent**: MEASURED on the
> pilot, 28 classes are reachable from 7 root searches, and re-deriving n in the
> class currency on the reference band gives 42–150, not 28. Whether round 3's
> test runs on one representative per class, and in which currency it powers
> itself, is round 3's to register — this run supplies the classes-per-root
> distribution it needs to do so."*

and change §7's "keys-per-root distribution" to **"the classes-per-root and
keys-per-root distributions"**, since classes are now the licensing unit.

This is also the honest answer to round 1's Q1, which revision 3 left unanswered
and then contradicted.

---

## 3. MINOR findings

**m1 — §6b names `0097f83` for two instruments that commit did not touch.**
`draw_census_samples.py` and `fixture_key_full.rs` were both last changed at
**`42967e0`**; `0097f83` added `census_classes.py` and `test_texel.py` only. The
bytes are identical at `42967e0`, `0097f83` and HEAD (verified by `sha256sum` on
`git show`), so nothing is materially wrong — but the table is internally
inconsistent, since its other two rows do name last-touching commits
(`0f58533`, `0097f83`). **Fix**: `42967e0` for both, or state that the column is
"revision the run is taken over" and use one value throughout.

**m2 — the cap decision's ground is still quoted in the currency D-619 retired.**
The opening paragraph: *"measurements that also showed the cap barely moves **the
quantity this census exists to produce** (distinguishable-trial rate
**0.10 / 0.07 / 0.08** per position across the three rungs, a spread of
**1.4x**)"*, echoed in §4. Under D-619 the quantity this census exists to produce
is the class count, whose rates across the same three files are
**0.26 / 0.14 / 0.15**, a spread of **1.86x**. The conclusion survives — 1.86x is
still far below the key count's 4.5x — but the stated number is in the retired
partition, which is the exact error class D-619 exists to correct. **Fix**: swap
the three rates and the spread; the sentence's force is unchanged.

**m3 — §8 still says "passed 0 of 20"** where `wp22_cap_prereg.md` §9 (line 226)
and `artifacts/wp22_cap_dryrun_v2/RECEIPT.md` (line 11) both record **0 of 100**;
the "19 of 19" shift test is the separate 20-row one. Round 1's m2, not landed.
**Fix**: `0 of 100`.

**m4 — §5's "the curve is still accelerating at 500" is not supported.** The
increments are `11, 7, 19, 10, 15`. The defensible claim is *not saturating*,
which my Chao2 (asymptote 489–641) supports strongly. **Fix**: say "not
saturating", and cite the asymptote rather than the shape.

**m5 — §3's "89 205 positions" over-counts under §2's own identity.** The
manifest's header says *"one record per distinct position"*; it is deduped on
`key_pos` (89 805 distinct, all rows) and holds **87 823 distinct `key_full`** —
387 `key_full` values covering **1 982 extra rows**, each with more than one
`key_pos`, i.e. symmetry-merged. The census slice is 89 205 rows / **87 228**
distinct canonical positions (2.2 %). Slice-to-slice disjointness is unaffected
(verified 0 overlap on `key_full` across all three pairs). **Fix**: "89 205 rows,
87 228 distinct canonical positions". Worth doing because §7's symmetry-fold
bullet says the fold's only measurements are "both zero" — here is a third, at
the root population of this very corpus, and it is 2.2 %.

**m6 — §4's `--fixture <census slice prefix>` is still an undefined
placeholder.** Round 1's m4, not landed, and now load-bearing: §5 registers
n = 800, and nothing in the document or the tree produces an 800-row prefix or
its `.expected_key_full`, while `draw_census_samples.py census` emits all 89 205
and §8's pre-run check is over the whole file. **Fix**: print the literal
two-command block (`draw`, then `head -800` of both files) and register the
prefix's `sha256`.

**m7 — every §5 number rests on a gitignored artifact nothing sha-indexes.**
`artifacts/wp22_census_pilot/calibslice_c2048.txt`
(`acdaee2b061e2c683a43781eb22505cee86ab76d7d878f0027357464090d542a`) appears in no
file under `docs/` or `tools/` (`git grep` returns nothing). Rule 8's own escape
is *"a committed manifest may sha-index them"*, and §9 already applies that
discipline to the corpus manifest. Round 1's m5, not landed, and the exposure
grew: revision 3's entire sizing now rests on this one file. **Fix**: name the
digest in §5.

**m8 — §3 still binds a successor to a retired document.** *"A successor changing
them again changes BOTH documents in one commit (D-599)"* — one of the two is
`wp22_cap_prereg.md`, retired by D-618 and no longer maintained, and neither is
the artefact that decides the boundaries (`DRY_RUN = 100` / `CALIBRATION = 600` in
`draw_census_samples.py`). Round 1's m6, not landed. **Fix**: point §3 at the two
constants.

**m9 — §7's "`w` by the position's turn count" is still not computable as
registered.** The census row's `turns` is depth from the search root
(`census.rs`: *"Turns from the search root"*); the root's own turn count lives in
manifest column 5 and needs a join on the fixture line number. Round 1's m7, not
landed. **Fix**: say which is meant and register the join.

**m10 — §9's first VOID condition still cannot fire.** *"VOID AND RE-RUN: the box
busy with a timed run, transient I/O."* The seat is node-budgeted and
deterministic — I re-ran 4 positions and got byte-identical rows — so a busy box
moves the wall clock and not one census row. Round 1's m9, not landed. D-424:
prose that cannot change a reading is prose a reviewer must still attack. **Fix**:
delete it, or scope it to the 44-minute estimate.

**m11 — `census_classes.py`'s row parser steps by one and half-populates
garbage.** `{words[i]: words[i+1] for i in range(2, len(words) - 1)}` builds a
28-entry dict from a 16-field row: every value also becomes a key mapping to the
next field name. No census value currently collides with a column name, so the
count is right today, and I verified that. But there is no field-count assertion
and no named error, so a reordered, truncated or extended row is read silently —
hard rule 3. Compare `stage3_allocator_bound.py::fields`, which steps by two, and
`firings`, which `fail()`s on a missing `cover` column. **Fix**: step by two and
assert the field set, raising a named error.

**m12 — `census_classes.py` dies with a traceback on a wrong-shape census.** Run
on `artifacts/stage3c_census_trigger_v1.txt` (the historical format, no `key`
column) it raises an unhandled `KeyError` from `main`. `stage3_allocator_bound.py`
has `void()`/`fail()` with exit codes 2/1 for exactly this. Hard rule 3 again.
**Fix**: a named refusal.

**m13 — `curve()`'s x-axis is the largest PROVING entry, not positions
processed.** `highest = max(entry for proving rows) + 1`, and `main` accepts
`positions` but never passes it to `curve`. §7 registers *"the cumulative trial
curve **against positions processed**"*. On the pilot the last proving entry is
493, so the axis reaches 500 by luck. On a governed run whose last 100 positions
prove nothing, the curve silently stops short and a reader mis-reads the run
length. **Fix**: pass `positions` into `curve` and bound the axis with it.

**Process note, not a numbered finding.** Six of round 1's nine MINORs (m2, m4,
m5, m6, m7, m8, m9 there) are untouched in revision 3, with no recorded rejection
and no attempted reproducer. CLAUDE.md's Process section asks for a fix or a
recorded rejection, not silence. I reproduced every one of them.

**Also worth one line to the operator, and I do not raise it as a finding because
it is D-619's to answer, not this document's**: D-618 recorded its strongest
surviving attack as *"if the operator rules that D-537's counted unit is the
canonical key and may not be supplemented … then the third tally is premature and
belongs to round 3's pre-registration"*, and accommodated it by *"reporting all
three counts and **licensing on the letter**"*. D-619 (3) withdraws exactly that
accommodation and records no replacement attack. §6's *"nothing here needs an
operator ruling"* was backed by the accommodation; with it gone, that clause is
carrying more weight than it did.

---

## 4. What I attacked and could not break

Stated explicitly, because a FAIL is not a verdict on everything, and because
this round's job was to adjudicate a reversal.

1. **THE REVERSAL IS SOUND.** `p1 = 12/14` is `knapsack_bound`'s bound over
   `stage3_allocator_bound.py`'s nine `COLUMNS`, printed by `matrix_stage3_detector.md`
   §5.8 as *"BOUND over the census COLUMNS: 12/14 = 0.857"*. Round 1's M2 is
   correct, D-619 concedes it correctly, and the class partition — not the
   thirteen written-ordering predicates — is the one the alternative acts on.
   The null's family is `p0`'s neighbourhood, not `p1`'s.

2. **M1's unreachability conclusion really does dissolve.** I reproduced round 1's
   own figures exactly (12 trials; `8 9 10 11 12`; Chao2 15–18) **and** the class
   figures exactly (62; `11 18 37 47 62`; crossing at 268), from the same single
   run. D-619's claim to have replicated the round-1 measurement is true. On the
   class partition Chao2 gives an asymptote of **489–641**, three orders of
   magnitude away from the 15–18 that made 28 unreachable. There is no saturation
   to find.

3. **The curve is not an ordering artifact**, which is the specific way this
   could have been rigged. My own 2 000-draw resampling reproduces the receipt's
   table within noise, and the binning order is `sha256(key_full)` — content-
   derived, so arbitrary in the right sense. The receipt is also candid that its
   resampling captures ordering and not between-sample variation, and it says so
   in its own text.

4. **n = 800 is a real margin, and the transfer across disjoint slices is
   justified.** The slices are contiguous blocks of a `sha256(key_full)`
   permutation, so they are exchangeable by construction, and I verified all three
   pairwise intersections are empty on both fixture lines and `key_full`. My
   conservative bootstrap — resampling positions **with replacement**, which
   cannot discover a class the 500 did not hold — puts **P(reach 28 at n = 800) at
   1.0000 with a 5th percentile of 41**. The realistic probability of failing to
   reach 28 classes at n = 800 is small enough that I could not produce it from
   this data. The residual risk is between-sample variation the bootstrap cannot
   see; the n = 100 evidence (11 on one slice, 26 on another, 2.4x) says that
   variation is large at n = 100, and §5's own margin argument — 800 against a
   crossing at 268 — is the right response to it.

5. **The tightening is legitimate in the direction that matters, and my sharpest
   attack on it failed.** I tried to show that registering 28 classes is a
   LOOSENING of D-537, on the ground that the arc's own reference sample reads 14
   win-proving firings as **4 positions** (§5.4) while the same firings hold
   **8 win-holding classes** — 2x more classes than positions. That attack does
   not land: D-570 settles that D-537's *disjoint positions* denominator is
   *"counted over census rows"* under `canonical_key`, i.e. over the firing's own
   position and not the root's, and §2 applies it faithfully. Against the unit
   D-570 fixed, `classes <= keys` holds in every sample I have (62/104, 26/86) and
   the tightening stands. M2's residue is that the document asserts it as a
   theorem rather than as the measured, guardable fact it is.

6. **Retiring the cap calibration unrun survives the partition fix.** The spread
   the decision rests on is 1.86x on the corrected partition (m2), against the key
   count's 4.5x — the same shape of argument with a different number, and it still
   supports declining a second arm. Nothing in this registration is a function of
   an unmade cap decision.

7. **Everything mechanical in §3 and §8 works.** All three draws reproduce; the
   `dryrun` draw is byte-identical to `d3.txt`; the census slice is 89 205 rows;
   the manifest digest matches `arc3_ledger.md` §5; `fixture_key_full` returns
   **0 of 89 205 in 2.28 s** on the whole governed fixture, which is stronger than
   §8's inherited dry run and discharges round 1's Q3 on the merits.

8. **The seat is deterministic and the pilot's provenance is genuine.** My
   4-position replay of the drawn calibration slice is byte-identical to the
   pilot's entries 0..3. D-615's determinism claim holds here.

9. **The floor arithmetic is right.** `p0 = 8/14`, `p1 = 12/14`, α = β = 0.05
   gives n = 28, c = 21 with size 0.040184 and power 0.962225 — six-decimal
   agreement with `overnight2_ledger.md` §4. §1's refusal to recompute it is
   correct discipline.

10. **"This run schedules nothing — it produces a count"** and **"if the governed
    curve flattens below 28, that is the finding §7 reports"** are both correct
    discipline and I have no attack on either.

---

## 5. What would turn this into a PASS

M1 and M4 are the two that must move; M2 and M3 are one commit between them. None
of the four requires a new measurement, and none reopens the reversal.

1. **Speak with one voice about the criterion (M1).** §7 bullet 1 adjudicates the
   CLASS count against 28 with `w` reported beside it; §7 bullets 4–5 use the
   words "class count" and "class curve" rather than "distinguishable trial";
   §1 says the run produces three counts and names the licensing one.
2. **State the tightening's ground accurately and guard it (M2).** Drop
   `roots <= classes`; justify `classes <= keys` from the columns and mark it
   MEASURED over 600 positions; register the `classes > keys` fallback in §9.
3. **Fix the independence prose and register its diagnostic (M4).** The class
   partition is the alternative's action space, not the binomial's independence;
   record the measured 28-classes-from-7-searches, and report the
   classes-per-root distribution in §7.
4. **Make the instrument's test do what §6b says it does (M3).** Assert
   `CC.COLUMNS == SAB.COLUMNS` against the shipped `stage3_allocator_bound.py`;
   replace the gitignored-artifact dependence with a committed synthetic fixture
   including a failing control; gate it or stop calling it gated.
5. The thirteen MINORs are one commit; m2, m3 and m6 are the three that would
   otherwise be quoted onward.
