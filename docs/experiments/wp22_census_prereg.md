# WP-2.2 §B — census run pre-registration, revision 2.

Governing revision: `84c1af2` (`dev`).

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

## §5 Sizing — on the count the floor's own statistics assume

**D-537's floor is a one-sample BINOMIAL** (`overnight2_ledger.md` §4:
`p0 = 8/14`, `p1 = 12/14`, `alpha = beta = 0.05`, `n = 28`, `c = 21`), and a
binomial assumes independent trials. **The counted unit does not deliver them**,
and the collapse is MEASURED (`artifacts/wp22_cap_decision/`, 100 positions at
cap 2048): 86 win-proving keys, from **8 roots**, distributed
`[35, 33, 11, 3, 1, 1, 1, 1]`, collapsing to **10 DISTINGUISHABLE TRIALS**
against the candidate field of `tools/stage3_census_rank.py`. **28 keys are
reachable from ONE search tree.**

**So `n` is sized on distinguishable trials, not on keys.** A candidate detector
is a boolean predicate over the census columns, and `p1 = 12/14` is defined as a
bound OVER THOSE COLUMNS (`matrix_stage3_detector.md` §5.4), so two proving rows
every candidate scores identically are one trial by the alternative's own
definition.

| quantity | MEASURED rate / position, cap 2048 | ESTIMATED `n` for 28 |
|---|---|---|
| distinct keys (D-537's letter) | 0.86 | 33 |
| distinct proving roots | 0.08 | 350 |
| **distinguishable trials** | **0.10** | **280** |

**REGISTERED: `n = 800`**, ESTIMATED **45 minutes** at the MEASURED 3.41 s per
position. That is well above the 280 the point estimate needs, and the margin is
not decoration: the trial count advances **lumpily**, in steps, when a new root
proves rather than smoothly — MEASURED, it sat at 2 through the first sixty
positions and reached 10 over the next forty. A sample sized at the point
estimate would be a coin toss.

**Saturation was checked before this was registered**, because distinct
verdict-vectors are a coupon-collector quantity and a ceiling near 10 would put
28 out of reach: the cumulative curve is still climbing over the second fifty
positions at both caps examined, so **no ceiling is visible** and 28 is
reachable. If the governed run's curve flattens instead, that is the finding
§7 reports and not a failure of the run.

**The run does NOT stop when any count reaches 28.** It runs its registered `n`.
A run that stopped at the floor would make the count a function of the floor.

## §6 What the three counts are for, and which one licenses what

`w` counts distinct keys, so a position proving a win under two different roots
counts once. **The calibration measures the proving-rows-to-distinct-keys ratio
and this run applies no correction to `n` for it** — the margin of §5 absorbs
it, and applying a measured correction from one sample to another is the
extrapolation D-563 warns against. The ratio is reported for both samples so a
successor can see whether the margin was doing that work.

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
