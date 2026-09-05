# WP-2.2 §B — census run pre-registration, revision 3.

Governing revision: `0097f83` (`dev`).

**Revision 3 is the granted fix round on `wp22_census_prereg_REVIEW.md`
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
corpus yields**, counted against D-537's registered floor of **28**.

D-537 fixes the floor and the counting rule; `overnight2_ledger.md` §4 computes
the floor from `p0 = 8/14`, `p1 = 12/14`, `alpha = beta = 0.05`, giving
`n = 28, c = 21`. **This document may not recompute, reinterpret or lower it.**
It is a FLOOR: a larger minimum may be registered with grounds, a smaller one
never.

**What clearing it does**: it re-opens detector round 3 (D-538, licensed-not-
scheduled). **This run schedules nothing** — it produces a count.

## §2 The quantity, spelled out because it has been miscounted twice

`w` = the number of **distinct `key` values** (`pistol_core::canonical_key`, the
identity D-570 selected as C2) carrying at least one census row with
`att_proved true`.

- **Win direction only.** D-522: `def_proved` answers whether the OPPONENT
  forces a win and is a proven LOSS. It is counted and reported in its own
  column and is **never summed** into `w`. D-535 preserves that distinction.
- **Disjoint POSITIONS, not firings and not games** — D-537's own words. The
  denominator is distinct keys, which is why the count is over a set and not a
  sum.

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

## §6 What the three counts are for, and which one licenses what

- **Distinct canonical KEYS** — D-537's literal unit, fixed by D-570. Reported
  always, and never suppressed.
- **Distinct proving ROOTS** — how many separate searches the evidence comes
  from. It is the rate that transferred between samples (0.080 -> 0.072).
- **Distinct COLUMN CLASSES** — the partition `p1 = 12/14` is defined over, and
  therefore the count the floor's binomial actually assumes.

**The CLASS COUNT licenses detector round 3.** Revision 2 said both that the
floor is adjudicated on the key count and that a larger minimum is registered;
those are incompatible, and D-619 resolves it — registering a larger minimum
under D-537's own permission means the LARGER one licenses.

**`roots <= classes <= keys` always**, which is what makes this a tightening
rather than a change of unit; `tools/texel/test_texel.py` pins the ordering.
D-537 forbids a successor to LOOSEN its minimum and expressly permits a larger
one with grounds, so nothing here needs an operator ruling — though the grounds
are the measurement in `artifacts/wp22_census_pilot/`, not this paragraph.

**The calibration this section used to defer to no longer exists** (D-618
retired it unrun), and §5 applies no rate correction because it no longer sizes
on a rate.

## §6b The instruments, with their governing revisions

| instrument | revision | produces |
|---|---|---|
| `crates/pistol-search/examples/trigger_census.rs` | `0f58533`, unchanged at HEAD | every census row |
| `tools/texel/draw_census_samples.py` | `0097f83` | the census fixture |
| `tools/texel/census_classes.py` | `0097f83` | all three counts and the class curve |
| `crates/pistol-core/examples/fixture_key_full.rs` | `0097f83` | §8's referent |

**All four are tracked and the class counter is tested**
(`tools/texel/test_texel.py` pins its partition against
`stage3_allocator_bound.py`'s and the `roots <= classes <= keys` ordering).
Revision 2 registered a count no committed instrument produced, which the review
raised as M5; that is what this section answers.

## §7 What is reported — the closure line

- `w` against the floor of **28**, and whether the floor is cleared.
- Positions searched, total firings, invocations, rows with `att_proved`.
- Distinct keys with `def_proved`, in their own column, never summed.
- **The number of PROVING ROOT POSITIONS** beside `w`, the keys-per-root
  distribution, and the **DISTINGUISHABLE-TRIAL count** — without which `w`
  cannot be read for how much independent evidence it represents.
- The **cumulative trial curve** against positions processed, so a successor can
  see whether the count was still climbing when the run ended.
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
