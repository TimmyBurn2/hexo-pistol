# W1 — the width cap: premise, calibration rule, and what I1 measured

**The calibration rule is stated BEFORE K is computed**, which is the whole
point of registering it: a K chosen after seeing which value looks good is not a
calibration, it is a fit.

## What I1 measured, and it moves the package

Four governed-book openings (`random_openings_v2.txt` lines 3, 7, 12, 20) at the
committed instrument seat and the governed `go nodes 50000`, via the width
histogram landed as I1:

| series | nodes | cells expanded | mean width | median |
|---|---|---|---|---|
| FILTERED | 87 | 190 | 2.2 | 2 |
| BATCHED (emitted) | 21 746 | 372 618 | 17.1 | 12 |
| Tier T's own union | 21 746 | 282 203 | 13.0 | 12 |
| the quiet ball | 1 187 | 90 415 | **76.2** | 76 |

**THE DISPATCH'S TARGET IS THE SMALLER LEVER ON THE POSITIONS THAT MATTER.**
W1 is ruled as a *Tier-T* width cap. On the book the SPRT actually plays, Tier T's
median width is **12** — already narrower than sealbot's 15-cell interior cap —
while the quiet-ball safety net, which fires when Tier T is EMPTY, has a median
of **76**. The net fires on only **5.5 %** of BATCHED nodes and accounts for
**24.3 %** of every cell the search expands.

**And the quiet ball already has a cap**: `safety_net_top_k`, implemented,
oracle-gated, and set to `0` in all twelve committed documents. WP-1.5d measured
it at K = 16 and got `inconclusive_at_game_cap`, `nelo_pair +16.9 ci95 21.5` —
the point estimate favours the cap and the interval spans zero (D-491, D-492),
with D-492 recording that the book was then spent and a larger-n run needs a
fresh one. `random_openings_v2.txt`'s reserved holdout is that fresh book.

**This document does not reorder the arc.** W1 caps Tier T, as ruled. What the
measurement adds is the honest expectation below, and a finding the arc closure
carries: the width that remains after W1 is mostly the net's.

## The calibration rule, pre-stated

**K is the 75th percentile of the Tier-T width distribution over the governed
book, rounded up**, and the cap therefore binds on the widest quarter of BATCHED
nodes and leaves the median node untouched. The ground: a cap below the median
changes the search everywhere and makes the SPRT a test of a different engine;
a cap above the 90th percentile binds too rarely to be measurable at the
registered n. The quartile is the coarsest cut that is neither.

**Exemptions, registered with the rule.**
- The FILTERED row is NEVER capped: its cells are the cover, its median width
  is 2, and dropping one is dropping a defence the search proved it needs.
- The WIN-NOW row is never capped, for the same reason at the other end.
- The ROOT TURN is never capped (A-01's own exemption, `pvs.rs`): the root's
  width is the one the arena's budget cannot bound, and narrowing it there would
  change what a VOID means.
- The quiet-ball safety net is not this package's to cap: `safety_net_top_k`
  owns that and stays at `0`.

## Honest expectation

`h0` is a legitimate outcome and the dispatch says so. Two measured facts point
that way and are recorded before the run: WP-1.5d's cap on the *wider* set
measured `+16.9 ± 21.5` and could not separate from zero at 500 pairs; and this
package caps a set whose median is already 12, so the search it changes is a
narrower change than that one. The compensating risk is colony blindness
(`sealbot_notes.md`), which the FILTERED-row exemption is the mechanism against.

---

## The sensitivity check, and it forecloses the SPRT

**Taken before any match, because a run whose class is absent from governed play
concludes nothing** — D-492 registered exactly this test for WP-1.5d and named
its failure mode: *"§6.4 named what would have made it uninformative — a
divergence rate near zero, or the class absent from governed play"*. The check:
the first 24 openings of `random_openings_v2.txt` at the governed
`go nodes 50000`, capped seat against the committed one, counting the openings
whose `bestmove` differs.

| K | openings whose chosen move changes |
|---|---|
| 4 | 18 of 24 (75.0 %) |
| 6 | 6 of 24 (25.0 %) |
| 8 | 4 of 24 (16.7 %) |
| 10 | 4 of 24 (16.7 %) |
| 12 | **0 of 24** |
| **16 — the calibrated value** | **0 of 24** |

**THE CALIBRATED CAP CHANGES NOTHING THE SPRT COULD SEE, AND THE SPRT IS
THEREFORE NOT RUN.** At K = 16 the histogram says the cap BINDS on 23.3 % of
BATCHED nodes and removes 11.2 % of Tier-T cells — and not one of 24 governed
openings answers differently. The cells it removes are the delta-ranked tail,
and alpha-beta refutes them so cheaply that removing them changes no choice.
WP-1.5d's cap moved the played turn on 4.9–5.8 % of searches and STILL could not
separate from zero at 500 pairs (D-492); this one moves it on 0 %.

**THE THRESHOLD IS NOT MOVED TO A K THAT WOULD BIND.** K = 8 changes 16.7 % of
openings and would be measurable, and choosing it now would be precisely the
post-hoc threshold move D-374 forbids — the calibration rule was written down
first for this reason. K = 8 also cuts below the measured median width of 12,
which that rule's own ground rejects: *"a cap below the median changes the
search everywhere and makes the SPRT a test of a different engine"*.

## W1's verdict: a MEASURED NULL, mechanism landed and gated off

`tier_t_top_k` lands implemented, validated and set to **0** in all fifteen
committed documents — the same disposition `safety_net_top_k` has carried since
WP-1.5d, and for a better-evidenced reason: that cap was *inconclusive*, this one
is *invisible*. Rule 5's own words: a measured structural floor is a finding, not
a failure.

**What the arc learns, and it is the third time this has happened.** P2's hotspot
was attributed to the wrong caller; P1's premise was worth under 3 %; and W1's
ruled target — Tier T — is on the governed book a set whose median width is 12,
already narrower than sealbot's 15-cell cap. **The width that is actually there
is the quiet ball's**: median 76, firing on 5.5 % of BATCHED nodes and carrying
24.3 % of every cell the search expands, with a cap mechanism that already exists
(`safety_net_top_k`), already measured `+16.9 ± 21.5` at K = 16, and a reserved
holdout book to re-measure it on. That is the package the evidence points at, and
naming it is what this one leaves behind.
