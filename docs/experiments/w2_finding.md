# W2 — root re-ordering by the previous iteration's scores: a MEASURED NULL

**The mechanism.** sealbot re-sorts its root candidate list by the previous
iteration's scores (`sealbot_study_2026-09.md` §6 item 3, `engine/search.h:171-177`),
and the study lists it as one of three things sealbot has that this engine does
not. This package implements it: each completed iteration hands the next one its
root children ordered best-first, a stable sort so the delta rank survives
underneath, and only a COMPLETED iteration may — an aborted one scored an
arbitrary prefix, and ordering by a prefix is worse than not ordering.

**Armed per seat by `root_reorder`, `false` in all fifteen committed documents.**

## The sensitivity check, and it forecloses the SPRT

Same instrument W1's used, and the same reason: a match on a change absent from
governed play concludes nothing (D-492).

| budget | depth reached | openings whose chosen move changes | throughput |
|---|---|---|---|
| `go nodes 50000` (the governed budget) | ~2 turns | **0 of 24** | 758 417 → 758 870 nps |
| `go nodes 400000` | ~4 turns | **0 of 12** | 7 951 → 7 858 ms total |

**Not one opening answers differently, at either depth, and the clock does not
move.** The SPRT is not run.

## Why it does nothing here, which is the finding

**This engine already holds the stronger mechanism, and the study said so.**
Every node — the root included — promotes the transposition table's move to the
front before anything else orders, so the best move the last iteration found
already leads the next one. What a root re-sort adds is the order of the cells
BELOW that move, and those are already delta-ranked by the same evaluation the
search uses. sealbot needs the re-sort because it has no PVS null-window scan
and orders by a plain delta with two killers; pistol has PVS, a TT that
promotes within the tier, and the delta rank on every row.

The study's own summary of the comparison — *"on every one of these pistol
already has the stronger mechanism"* — is confirmed by measurement here rather
than accepted from reading. **Width was the one exception it named, and W1
measured that too.**

## Disposition

The mechanism lands, gated `false`, on the same footing as `tier_t_top_k` and
`safety_net_top_k`: implemented, validated, measured, off. What would flip it is
a change that makes the previous iteration's scores carry information the TT
move does not — aspiration windows (S1) are the nearest candidate, since a
fail-low there re-searches with an ordering the TT alone cannot supply.
