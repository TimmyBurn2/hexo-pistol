# WP-2.2 §B — census run pre-registration

**Revision 1.** Governing revision: `71fa6f1` (`dev`). **This document does not
run until `wp22_cap_prereg.md` has passed its review and returned a cap.** The
cap is this registration's one free input and it is written in below before the
run, never after.

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

The **census slice** of `tools/texel/draw_census_samples.py`: rows 520 onward of
the `sha256(key_full)` order over
`artifacts/arc3r_sweep_deduped_manifest.txt` — **89 285 positions**, disjoint by
construction from the calibration sample (rows 20..519) and the dry-run slice
(rows 0..19).

**How many of them are searched is set by §5, and the rule is registered before
the run.**

## §4 The seat

```
trigger_census --fixture <census slice prefix> --nodes 50000 \
               --cap <the cap wp22_cap_prereg.md returns> --gate on
```

`--nodes 50000` is the seat the cap was chosen at, and
`wp22_cap_prereg.md` §1 binds the cap to that budget: a census at another
budget would need the calibration re-run, not this document amended.

**Why a census-capable seat is needed at all**: D-563 measured that the sweep's
own labelling seat (`configs/instrument_v0.toml`, gate off) records ZERO
firings, so a census token there starts a clock that never advances. D-596
records that the WP-2.1 sweep therefore ran with the census OFF, and that the
count *"comes from the census run over this corpus's positions … on a
census-capable seat"*. **This is that run.**

## §5 Sizing, and the stopping rule

**`n` is fixed before the run** from the calibration's own measured
position-level yield `y` at the selected cap:

```
n  =  ceil( 3 · 28 / y )
```

**The factor of 3 is a margin against the yield estimate, and it is registered
rather than tuned**: `y` comes from 500 calibration positions, and a census
sized at exactly `28/y` reaches the floor only if the estimate is exactly right
and the count is noiseless. Three times that expects ~84 against a floor of 28.

**The run does NOT stop when it reaches 28.** It runs its registered `n` and
reports the count. A run that stopped at the floor would make the count a
function of the floor, and every subsequent reading of *"how rich is this
corpus"* would be circular.

**If `n` exceeds what the box affords** — stated in the receipt as hours before
the run starts — the shortfall is registered by cutting `n` to what fits **and
saying so in the closure line**, with the count reported against the floor
either way. A short run is a short run; it is never reported as a floor that
could not be cleared.

## §6 The cross-root collision correction

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
