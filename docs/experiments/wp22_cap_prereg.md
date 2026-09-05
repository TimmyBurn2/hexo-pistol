# WP-2.2 §B — cap calibration pre-registration, revision 4.

**REMEDIES ONLY** — the revision is the one this document's first line states,
and it is stated there and nowhere else (D-599). Governing revision of the work
this registers:
`b876d1d` (`dev`). The run this document governs has not been taken.

**This revision fixes the findings of `wp22_cap_prereg_rev3_REVIEW.md` and adds
nothing else.** That review returned FAIL on seven findings, three of them
measured rather than argued. **Every one of the three was reproduced by this
session before this revision was written** (§10), and the most important of them
reversed a change revision 3 had made on round 1's advice:

> **M2 — revision 3's claim that searching the canonical image and searching the
> corpus's own move order are "the same census" is FALSE.** Round 1's F11 argued
> the join to the corpus files was unnecessary because `canonical_key` is
> symmetry-invariant. Round 2 MEASURED the substitution and it is not neutral:
> the same twenty rows give **229 against 236 firings** and **206 against 210
> distinct keys**. The keys are invariant; the SEARCH that finds them is not,
> because its tie-breaks are coordinate-lexicographic and a mirrored board
> breaks them differently. **A measurement beats an argument, and the join is
> back.**

## §1 What is decided here, and what is not

One number: the solver's **`per_call_node_cap`** — the `trigger_census --cap`
argument — that **§B's census registration will use**. That census is registered
by its own document.

**The cap is valid for a census at `--nodes 50000` and at no other budget.** A
census registered at another budget re-opens this calibration rather than
importing its answer.

**REVISION 3's SCALING LAW IS WITHDRAWN.** Revision 3 justified the ladder's top
rung with *"firings per position scale as `nodes / (2·cap)`"*. Its own dry run
contradicts that law — the law predicts 3.1 and 1.5 firings per position at 8192
and 16384 against **4.40 and 3.05 measured** (M9). What survives, and it is all
that the ladder needs, is the BOUND: a cap at or above the node budget is never
reached, because the stop condition reads
`total_nodes() = search_nodes + solver_nodes`
(`crates/pistol-search/src/pvs.rs:196-198`). No predictive law is claimed.

**What this document may not do.** It may not state the census count or read
anything about the floor.

## §2 The seat

```
trigger_census --fixture <sample> --nodes 50000 --cap <arm> --gate on
```

**The seat, stated exactly, because revision 3 overclaimed it twice** (F1, M5).
The tree holds **25 `trigger_census` outputs: 15 at `--nodes 50000` and 10 at
`--nodes 400000`.** The fifteen are every `stage3*` census run; the ten are all
`wp20b_*`, D-563's cap and `key_pos` measurements on BENCH fixtures.

So: **50 000 is the budget every prior CENSUS ran at, and 400 000 is the budget
every prior CAP MEASUREMENT ran at.** Revision 2 imported D-563's cap figures as
if they were this seat's; they are not, and this revision cites them only as
evidence about the 400 000-node seat.

## §3 The corpus, the three samples, and the draw

**The corpus.** `artifacts/arc3r_sweep_deduped_manifest.txt`, 89 805 body rows,
sha-indexed with the sixteen corpus files in `docs/experiments/arc3_ledger.md`
§5. The receipt records the digest read; a disagreement with the ledger is a
VOID.

**Positions are the corpus's OWN move order, reached by a join that is verified
on every row.** `tools/texel/draw_census_samples.py` joins
`(corpus_index, record_number)` to the corpus record and **refuses unless that
record carries BOTH the manifest's `key_pos` and its `key_full`** — either alone
leaves a shift undetectable on rows that happen to share one.

**Why not the manifest's `key_seq`, which needs no join**: because M2 measured
that it is a different census. Recorded here so a successor does not re-derive
round 1's argument and re-introduce the defect.

**The draw** orders rows by `sha256(key_full)` ascending — a fixed function of
the position, no seed — and takes three **disjoint** slices:

| slice | rows | used by |
|---|---|---|
| dry run | 0 .. 99 (**100 positions**) | §9, consumed by nothing else |
| **calibration** | 100 .. 599 (**500 positions**) | this document's governed run |
| census | 600 .. end | §B's census registration |

**The dry-run slice is 100 rather than revision 3's 20**, because M7 showed a
20-position rate is a single event with a 95 % interval spanning two orders of
magnitude. **It asserts its schema before it draws**: the manifest's `#
columns:` line, 8 columns per row, exactly 89 805 body rows — named refusals,
not an `IndexError`.

## §4 The ladder

```
2048    8192    16384
```

The cap must stay below the node budget or it never binds; at 16384 it does bind
(the review confirmed `att_visits` topping out at exactly the cap). Revision 2's
131072 rung is gone for that reason.

**THE ARMS ARE NOT EQUAL-COMPUTE, AND REVISION 3's CLAIM THAT THEY DIFFER "IN
THE CAP AND IN NOTHING ELSE" IS WITHDRAWN** (M9). Measured mean total nodes per
position run to 41 888 / 44 519 / 49 344, with a tail to 1.64x the registered
budget. **This is why §5 normalises by SECONDS and not by nodes**: wall clock
already prices whatever compute an arm actually spends, and the selection rule
never needed the arms to be equal-compute.

**No arm is expected to return zero.** At the registered seat the tree's own
censuses prove wins at cap 2048 (`stage3b_census_corpus_r0_v1.txt`: 1 in 361
firings; `r1`: 4 in 370), and §6's dry run proves them at every arm.

## §5 The selection rule — registered before the run

For cap `c`, on the calibration sample:

- `w(c)` = **distinct `key` values carrying at least one row with `att_proved
  true`**. Win direction only (D-522, D-535); `def_proved` is a proven LOSS,
  reported in its own column, **never summed**.
- `s(c)` = the arm's wall-clock seconds, **measured as `SECONDS` around the
  single `trigger_census` invocation, fixture build excluded, one run per arm,
  on an otherwise quiet box** (M4c). The arms run sequentially, never
  concurrently.

**The rule.** Reaching any fixed count `N` of win-proving positions costs
`N · s(c)/w(c)`, so the cheapest cap maximises `w(c)/s(c)` whatever `N` is —
which is why the rule needs no floor value to state it.

**The floor is applied FIRST, to every arm** (M4b — revision 3 applied it to the
selected arm alone, which could discard a qualifying arm because a different one
was thin):

> 1. An arm is **SELECTABLE** iff `w(c) >= 10`.
> 2. Among selectable arms, **select the SMALLEST cap whose ratio satisfies
>    `ratio >= 0.9 * max_ratio`** — that inequality, in that direction, and not
>    "within 10 %", which has two readings (M4a).
> 3. **If no arm is selectable, report UNDERPOWERED and the incumbent cap
>    stands.**

**THE INCUMBENT CAP IS 2048** (M3 — revision 3 left it undefined, and the
candidates are 8x apart). It is the cap **every census in this tree has been
taken at** (`artifacts/stage3{,b,c}_census_*`, all fifteen). The committed
configs' 16384 is the SEARCH's cap and has never been a census's.

**If every arm returns zero**, the run reports per arm the firings observed and
the rule-of-three 95 % upper bound `3/F` on the per-firing rate, and licenses
nothing below it. **The remedy for that is a larger SAMPLE, not a larger
ladder.**

## §6 Sizing, and the clustering that dominates it

D-563 addresses the sizing to whoever runs this. Measured on §9's **100-position**
dry-run slice, at the registered seat, on the corpus's own move order:

| cap | firings / position | **`w` (distinct keys)** | keys / position | **proving POSITIONS** | keys per proving position | s / position |
|---|---|---|---|---|---|---|
| 2048 | 11.63 | 86 | 0.86 | **8 of 100** | **10.75** | 3.41 |
| 8192 | 3.90 | 17 | 0.17 | **8 of 100** | **2.12** | 4.03 |
| 16384 | 2.67 | 19 | 0.19 | **9 of 100** | **2.11** | 4.80 |

**THE COLUMN THAT MATTERS IS NOT THE ONE REVISION 3 SIZED ON** (M7a). `w` counts
distinct in-tree `key`s, which is D-570's identity and the right unit for
D-537's denominator — but those keys are **CLUSTERED IN A HANDFUL OF ROOT
POSITIONS**. At cap 2048, 86 keys come from **8** roots. The effective sample
size for `w` is the number of PROVING ROOTS, near 8, and not 86; a Bernoulli
model over positions understates `w`'s variance badly, and revision 3's sizing
used exactly that model.

**So `n = 500` is registered on the ROOT rate, which is the quantity with the
honest denominator.** The three arms prove on **8, 8 and 9 of 100** roots — a
rate the arms agree on closely, unlike the key counts. At 500 positions that
expects **40 to 45 proving roots**; at the measured clustering that is `w` of
roughly 430 at cap 2048 and 85 to 95 at the two larger caps, all far clear of
§5's floor of 10. The Clopper-Pearson 95 % interval on 8/100 is
**[0.035, 0.151]**, whose lower bound still gives ~18 proving roots at n = 500,
and `w` above the floor at every arm.

**What the dry run does NOT establish, stated plainly** (M7c): it does not
separate the arms. Nothing here identifies "the least productive arm", and the
selection between them is the governed run's to make under §5.

**Revision 3's mechanism claim is deleted** (M8): the per-firing proof rates do
not rise monotonically with the cap, and no part of this sizing rests on their
doing so.

## §7 What is reported

Per cap: `w(c)`, `s(c)`, the ratio, firings, invocations, rows with
`att_proved`, **the count of PROVING ROOT POSITIONS beside `w`** so the
clustering is visible, distinct keys with `def_proved` separately, `distinct
key` against `distinct key_pos`, and `w` by the position's own turn count.

## §8 Instruments, with their governing revisions

| instrument | revision | produces |
|---|---|---|
| `crates/pistol-search/examples/trigger_census.rs` | `0f58533`, unchanged at HEAD | every census row |
| `crates/pistol-core/examples/fixture_key_full.rs` | `b876d1d` | §9's referent |
| `tools/texel/draw_census_samples.py` | `b876d1d` | all three samples |

**These are tracked** (M6/F13: revision 3's were untracked and identified by a
document revision that existed nowhere).

**Cost.** At 500 positions from §6's measured per-position seconds:
**2048 ≈ 28 min, 8192 ≈ 34 min, 16384 ≈ 40 min**. Detached, polled, and the
box carries nothing else being timed.

## §9 Dry run — input, criterion, defect class

**Input**: the 100-position dry-run slice, real corpus positions from the same
manifest, disjoint from the calibration and census slices.

**Defect class**: **THE FIXTURE IS NOT THE POSITIONS THE DRAW CLAIMS** — lines
that are well formed, replay cleanly, and belong to other rows than the ones
drawn. Every property internal to the pipeline survives it.

**Criterion — an externally derived referent, paired row by row** (never
`diff`, which realigns a shift and understates it): the manifest's `key_full`
must equal the canonical form recomputed from the **replayed board** by
`pistol-core`'s `canonical_form`, which does not read the manifest.

**Recorded results:**

- honest run: **0 of 100 rows disagree**. The criterion is a property of the
  fixture and not of the cap, so it is taken once and not "at all three caps",
  which was a category error (M10).
- the expected list shifted by one row: **19 of 19 disagree**.
- the 500-position calibration fixture, checked the same way before the governed
  run: **0 of 500 disagree**.

**A second registered check, because the criterion above cannot see it** (F8):
`--gate on` must produce a positive firing count — D-563 is the record of a
census token that armed nothing. It does, at every arm (§6).

**Receipt**: `artifacts/wp22_cap_dryrun_v2/`. The earlier
`artifacts/wp22_cap_dryrun/` documents the SUPERSEDED join-free draw and is
retained, marked superseded, rather than deleted (M11).

## §10 The reproductions this revision rests on

Run before this revision was written, per D-591.

1. **M2.** The same 20 rows, censused from the canonical image and from the
   corpus's move order at cap 2048: **229 vs 236 firings, 206 vs 210 distinct
   keys, 6 vs 8 loss-direction keys.** The join was verified on `key_pos` AND
   `key_full` for all 20 before the comparison.
2. **M5.** Reported as the SEARCH rather than as a transcribed integer
   (D-601, D-602), run at `b876d1d`:

   ```
   LC_ALL=C grep -l "trigger_census: argv" artifacts/*.txt | while read f; do
       LC_ALL=C grep -o -m1 '--nodes [0-9]*' "$f"; done | LC_ALL=C sort | uniq -c
   ```

   **25 files, 25 argv lines: 10 at `--nodes 400000`, 15 at `--nodes 50000`.**
   The fifteen are every `stage3*` census; the ten are all `wp20b_*`. Counting
   argv LINES rather than files gives the same split, so no file mixes budgets.
   The round-2 review reported 17 and 6 over a differently scoped set; this
   command and its scope are what this document's claim rests on.
3. **F1/§10.1 carried.** `wp20b_cap_out_*`: **6 files, all `--nodes 400000`.**

## §11 Void against fail

**VOID AND RE-RUN**: the box busy with a timed run, transient I/O.

**VOID AND REPAIR**: a fixture line refused by the rules, a manifest digest
disagreeing with `arc3_ledger.md` §5, **positive firings with zero census
rows**, or **zero firings at every arm**.

**FAIL**: the run adjudicated — every arm completed and §5 selects a cap,
reports UNDERPOWERED, or returns none.
