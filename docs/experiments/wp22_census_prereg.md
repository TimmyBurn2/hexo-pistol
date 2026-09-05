# WP-2.2 §B — census run pre-registration, revision 4.

Governing revision: `0097f83` (`dev`).

**Revision 4 answers `wp22_census_prereg_rev3_REVIEW.md` (FAIL, 4 MAJOR), whose
adjudication of the reversal is the reason this document is still alive**: it
attacked revision 3's partition correction four ways and could not break it —
`p1 = 12/14` IS `knapsack_bound`'s bound over the nine `COLUMNS`
(`matrix_stage3_detector.md` **§5.8**), the pilot reproduces to the digit, the
curve is not an ordering artifact, and Chao2 on the class partition gives an
asymptote of 489-641 against 15-18 for the null's family. **Round 1's M1 is
correctly dissolved and the run is runnable.** What revision 4 fixes is the four
majors that survived: the criterion stated two ways (§2 now owns it), the
`roots <= classes <= keys` claim asserted as a theorem (now checked and refused),
the tests that nothing ran (now CI gate 18), and the independence REASON, which
was wrong (§2).

**Revision 3 was the granted fix round on `wp22_census_prereg_REVIEW.md`
(FAIL, 6 MAJOR).** Its two decisive findings are conceded and corrected in
D-619: revision 2 computed its sizing quantity over the NULL's family of
predicates rather than the ALTERNATIVE's column partition, and it extrapolated a
coupon-collector count linearly. Both are fixed below, on a 500-position pilot
run for this revision.

**Revision 2 implements D-618, which RETIRED the cap calibration UNRUN.** The
cap is no longer this document's free input: it is **2048**, fixed by D-618 on
measurements that also showed the cap barely moves the quantity this census
exists to produce (distinguishable-trial rate 0.10 / 0.07 / 0.08 per position
across the three rungs, a spread of 1.4x). Revision 1 waited on a calibration
that no longer exists.

## §1 What this run answers

**One number: how many WIN-PROVING FIRINGS ON DISJOINT POSITIONS the sweep
corpus yields**, counted against D-537's floor of **28** in the currency §2
registers.

D-537 fixes the floor and the counting rule; `overnight2_ledger.md` §4 computes
the floor from `p0 = 8/14`, `p1 = 12/14`, `alpha = beta = 0.05`, giving
`n = 28, c = 21`. **This document may not recompute, reinterpret or lower it.**
It is a FLOOR: a larger minimum may be registered with grounds, a smaller one
never.

**What clearing it does**: it re-opens detector round 3 (D-538, licensed-not-
scheduled). **This run schedules nothing** — it produces a count.

## §2 The quantity — stated ONCE here, and pointed at from everywhere else

**Three counts are taken over the win-proving firings. THE CLASS COUNT IS THE
REGISTERED MINIMUM AND IS WHAT LICENSES DETECTOR ROUND 3.** Every other section
of this document points here rather than restating it, because a criterion
stated twice is the defect D-423 names and the round-1 and round-2 reviews both
caught it stated two ways.

| count | what it is | role |
|---|---|---|
| **CLASSES** | distinct nine-column vectors (`tools/stage3_allocator_bound.py` `COLUMNS`) | **the registered minimum: 28. This licenses round 3.** |
| KEYS | distinct `pistol_core::canonical_key` (D-570's identity) | D-537's literal figure. Reported always, never suppressed. |
| ROOTS | distinct fixture positions that proved | the unit carrying statistical independence. Reported. |

**Win direction only** (D-522, D-535): `def_proved` answers whether the OPPONENT
forces a win and is a proven LOSS. It is reported in its own column and **never
summed** into any of the three.

**WHY 28 CLASSES IS REGISTERED, AND THE REASON IS NOT THE ONE REVISION 3 GAVE.**
D-537 forbids a successor to LOOSEN its minimum and expressly permits *"a larger
one with grounds"*. Requiring 28 classes is strictly larger than requiring 28
keys **whenever `classes <= keys` holds**, and that is the whole ground.

**It is NOT registered on the ground that classes restore the binomial's
independence, which the round-2 review refuted by measurement**:
`knapsack_bound` scores FIRINGS while choosing CLASSES, so `p1 = 12/14` is not a
per-class rate — in class currency the same reference band gives 6/8 = 0.750 —
and 28 classes are reachable from as few as **7** root searches. **The
independence defect D-618 identified is real and is NOT fixed by this
registration.** It is carried by the ROOT count, which is why the root count is
reported, and whether D-537's floor should be re-solved in class currency
(ESTIMATED at n = 42-150) is round 3's question and not this document's.

**`roots <= classes <= keys` IS NOT A THEOREM.** `turns` is root-relative, so
one key can carry several classes and `classes <= keys` is contingent. It held
over 600 positions and 6 366 keys and it is **checked on every run**:
`tools/texel/census_classes.py` raises `OrderingViolated` and the run is refused
rather than reported, because a run that breaks it is a run whose count is not a
tightening.

## §3 The sample

The **census slice** of `tools/texel/draw_census_samples.py`: rows **600
onward** of the `sha256(key_full)` order over
`artifacts/arc3r_sweep_deduped_manifest.txt` — **89 205 positions**, disjoint by
construction from the calibration sample (rows 100..599) and the dry-run slice
(rows 0..99).

**The boundaries track `wp22_cap_prereg.md` revision 4**, which enlarged the
dry-run slice from 20 rows to 100 after its round-2 review showed a 20-position
rate is a single event with a 95 % interval spanning two orders of magnitude. A
successor changing them again changes BOTH documents in one commit (D-599).

**How many of them are searched is set by §5, and the rule is registered before
the run.**

## §4 The seat

```
trigger_census --fixture <census slice prefix> --nodes 50000 \
               --cap 2048 --gate on
```

**Single instrument, one arm.** D-618 fixes the cap at 2048 and declines a
second arm: the red team measured that two of the three roots cap 2048 alone
proves are recovered by cap 16384 at `--nodes 200000`, so most of the apparent
cap complementarity is a NODE-BUDGET artifact, and a union across two caps
licenses no conclusion D-537 can read.

**Why a census-capable seat is needed at all**: D-563 measured that the sweep's
own labelling seat (`configs/instrument_v0.toml`, gate off) records ZERO
firings, so a census token there starts a clock that never advances. D-596
records that the WP-2.1 sweep therefore ran with the census OFF, and that the
count *"comes from the census run over this corpus's positions … on a
census-capable seat"*. **This is that run.**

## §5 Sizing — on a MEASURED curve, over the ALTERNATIVE's own partition

**D-537's floor is a one-sample BINOMIAL** (`overnight2_ledger.md` §4:
`p0 = 8/14`, `p1 = 12/14`, `alpha = beta = 0.05`, `n = 28`, `c = 21`), which
assumes independent trials. **The counted unit does not deliver them**: MEASURED
at cap 2048 on 100 positions, 86 win-proving keys come from 8 roots distributed
`[35, 33, 11, 3, 1, 1, 1, 1]`, so **28 keys are reachable from ONE search tree**.

**THE PARTITION IS THE ALTERNATIVE'S, AND REVISION 2 HAD IT WRONG.**
`p1 = 12/14` is produced by `tools/stage3_allocator_bound.py`'s `knapsack_bound`
over its nine-`COLUMNS` tuple applied atomically. That file states the principle:

> *"A score is a function of the columns, so it cannot tell two firings apart
> when every column agrees. The finest partition it can act on is the
> column-vector CLASS."*

Revision 2 used `stage3_census_rank.py`'s thirteen written-ordering predicates —
**the NULL's family** — which under-counts by 2.6x and made the criterion
unreachable. Conceded in full; the correction is D-619.

**THE SIZING IS A MEASURED CURVE, NOT A RATE.** A class count accumulates
distinct things and cannot be extrapolated: from n = 100 to n = 500 the root
rate held (0.080 -> 0.072) while the class rate fell 2.1x and the key rate 4.1x.
So `n` is read off the curve itself. Pilot: **500 positions of the ORPHANED
calibration slice** (rows 100..599 — the sample D-618 retired unrun, disjoint
from the census slice, so nothing governed was consumed),
`artifacts/wp22_census_pilot/`:

```
positions  100   200   300   400   500
classes     11    18    37    47    62
```

**The floor of 28 is crossed between n = 200 and n = 300, and the curve is still
accelerating at 500.** It does not saturate on this partition.

**REGISTERED: `n = 800`**, ESTIMATED **44 minutes** at the MEASURED 3.31 s per
position. That is more than twice the position count at which the floor was
observed to be crossed, on a disjoint sample of the same corpus, and the margin
is there because a class curve is lumpy — it advances when a new root proves.

**If the governed curve flattens below 28**, that is the finding §7 reports, and
D-619's flip clause fires. It is not a failed run.

## §6 Which count licenses — see §2

**§2 owns this and states it once**: the CLASS count is the registered minimum
and licenses detector round 3; the KEY count is D-537's literal figure and is
always reported; the ROOT count carries independence and is always reported.

**The calibration this section used to defer to no longer exists** (D-618
retired it unrun), and §5 applies no rate correction because it no longer sizes
on a rate.

## §6b The instruments, with their governing revisions

| instrument | revision | produces |
|---|---|---|
| `crates/pistol-search/examples/trigger_census.rs` | `0f58533`, unchanged at HEAD | every census row |
| `tools/texel/draw_census_samples.py` | `42967e0` | the census fixture |
| `tools/texel/census_classes.py` | `0097f83`, gated by `tools/texel_tests.sh` | all three counts, the class curve, and the ordering refusal |
| `crates/pistol-core/examples/fixture_key_full.rs` | `42967e0` | §8's referent |

**All four are tracked, and the class counter's tests now RUN**: CI gate 18
(`tools/texel_tests.sh`) invokes them. The round-2 review found the suite was
invoked by nothing and self-SKIPPED to a pass when a gitignored artifact was
absent — so in CI it would have reported success having checked nothing. The
tests are hermetic now, they check the partition against
`stage3_allocator_bound.py` ITSELF rather than a second copy of it, and one
seeds an ordering violation to prove the refusal fires.

## §7 What is reported — the closure line

- **The three counts of §2**, and whether the registered minimum (28 CLASSES) is cleared. §2 owns the criterion; this section does not restate it.
- Positions searched, total firings, invocations, rows with `att_proved`.
- Distinct keys with `def_proved`, in their own column, never summed.
- The keys-per-root distribution, without which the key count cannot be read
  for how much independent evidence it represents.
- The **cumulative CLASS curve** against positions processed, so a successor can
  see whether the count was still climbing when the run ended — and so the
  saturation question round 1 raised is answered by this run's own data rather
  than by extrapolation.
- The attacker-invocation **truncation rate** (invocations that hit the cap
  without proving), MEASURED at 71.2 % at this cap on the dry-run slice, so what
  a larger cap might have added is visible rather than argued.
- `distinct key` against `distinct key_pos`, which is the symmetry fold's
  in-tree yield on a third population (the first two: D-570's 798 firings and
  this package's own dry run, both zero).
- `w` by the position's turn count.
- **Detector round 3 licensed, or the shortfall stated.** Nothing else in this
  project reads this run.

## §8 Dry run

**The census slice's first 20 positions are NOT used**, because they are part of
the governed sample. The dry run of `wp22_cap_prereg.md` §9 exercised the
identical command on the identical instrument at the identical seat, on a
disjoint slice, and its criterion — the manifest's `key_full` against
`pistol-core`'s canonical form recomputed from the replayed board, paired row by
row — passed 0 of 20 and killed a one-row shift at 19 of 19. **That dry run
covers this document's commands too**, and re-running it on a governed slice
would consume the sample it is meant to protect.

**What is checked immediately before this run, and is not a dry run**: the whole
census fixture replays and canonicalizes against the manifest, the same way the
500-position calibration fixture did (0 of 500). A fixture that cannot replay is
a VOID AND REPAIR under §9.

## §9 Void against fail

**VOID AND RE-RUN**: the box busy with a timed run, transient I/O.

**VOID AND REPAIR**: a fixture line refused by the rules, a manifest digest
disagreeing with `arc3_ledger.md` §5, **positive firings with zero census
rows**, or **zero firings over the whole run** — the last two being the
signature D-563 is the record of, a gate that armed nothing.

**FAIL** — the run adjudicated: it completed its registered `n` and §7's count
is stated.
