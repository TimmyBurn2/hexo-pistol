# Where the engine stands after the arc — 1.3x the nodes, two turns of depth, and three turns behind the opponent.

**Instrument:** the line-protocol binary at each revision, 24 openings of
`random_openings_v2.txt`, `configs/play_staged_v0.toml` at the movetime budgets
and `configs/instrument_v0.toml` at the node budgets. Baseline `ffc5c10`
(arc III's close), candidate `03aedd3` (the arc's end). Receipt
`artifacts/opt_arc_perf_sweep_v1.txt`, sha256 `062b5f98ccb6851e…`. Every figure
is a MEDIAN over the 24, read from the last completed iteration of each search.

## The table

| budget | baseline depth (turns) | after the arc | what changed |
|---|---|---|---|
| `movetime 100` | 1.5 med, max 2 | **2.0 med, max 3** | **1.331x** the nodes in the same wall |
| `movetime 300` | 2.0 med, max 3 | 2.0 med, max 3 | **1.630x** the nodes |
| `movetime 500` | 2.0 med, max 3 | 2.0 med, **max 4** | 1.085x the nodes |
| `nodes 50 000` | 2.0 | 2.0 | **1.397x** faster for the same nodes |
| `nodes 200 000` | 2.5 | 2.5 | **1.276x** faster |
| `nodes 1 000 000` | 4.0 | 4.0 | **1.360x** faster |

**The node counts at the node budgets are IDENTICAL between the revisions** —
14 879, 59 534 and 508 871 on both — which is the arc's byte-identity claim
holding at three budgets it was not registered against. The speed is real and
the search is unchanged.

**The movetime rows are noisier than the node rows and should be read as
such**: a wall-clock seat is not reproducible by construction (D-22), the box
was shared, and the 0.3 s row's 1.630x against the 0.5 s row's 1.085x is
variance, not a budget-dependent effect. **The node rows are the measurement**;
they agree with tranche 1's registered bench (1.294 / 1.293) and with the
anchor's 1.425x nodes per answer.

## The finding: TWO TURNS

**At the 0.5 s deployment budget this engine searches a median of two turns.**
A turn is two stones, so that is four plies against a branching factor whose own
measured medians are 12 at Tier T and 76 at the quiet ball (I1's histogram). A
third more nodes moved the median depth at 0.1 s and nowhere else, because
depth is logarithmic in nodes and the branching factor here is large.

**This is why every behaviour-changing package in the arc came back null.** S2's
forced-reply extension, S3's late move reductions, W1's Tier-T cap and W2's root
re-ordering are all ways of spending depth better. At two turns there is
almost no depth to spend, and each of them was measured against that:

- S2 bought a turn on forcing lines and lost the iteration everywhere else —
  5 turns deepest against the committed seat's 6, and 8 W / 61 L (D-605).
- S3 cut two plies off late children, which at six turns is a THIRD of what
  remains, so the verification re-search cost more than the cut saved: both
  seats reached 6 turns, 2.9 % fewer nodes, UNDECIDED at the cap (D-607).
- W1's cap at the calibrated K removed 11.2 % of Tier-T cells and changed no
  move at all, because alpha-beta already refutes the delta-ranked tail cheaply.

## Against sealbot, which is the number that matters

Sealbot's own counters, replayed on the anchor's positions at its registered
budget (`artifacts/sealbot_anchor_v1_analysis_sealbot_depths.txt`, 43 answers,
every move reproducing the governed transcripts):

| | budget | depth | nodes | nps |
|---|---|---|---|---|
| **sealbot** | 0.3 s | **median 5 turns** (max 7) on the answers that used the full budget | 259 072 | **~864 000** |
| **pistol, after the arc** | 0.5 s | **median 2 turns** (max 4) | ~60 850 | ~488 000 |

**Sealbot searches about three turns deeper, on four times the nodes, in less
than two thirds of the wall.** A 1.29x speedup does not close a gap of that
shape, and the arc's own anchor agrees: 40 W / 59 L over 50 paired openings
(anchor v5) — competitive, and behind.

**The two candidates this points at, in order:**

1. **Nodes per second.** Sealbot is at ~864 000 nps to this engine's ~488 000,
   a factor of **1.77** still on the table, and the arc's own profile named the
   eval's window map as the next hotspot after the threat state (D-603's flip
   clause). Depth is logarithmic in nodes, so this alone is worth well under a
   turn — it is necessary and not sufficient.
2. **The branching factor, which is the bigger term.** The quiet ball's median
   width is **76** and it carries 24.3 % of every cell the search expands, with
   `safety_net_top_k` already built for it and a reserved holdout book to
   measure it on (W1's calibration). A width cap that binds where the width
   actually is buys depth multiplicatively, where a speedup buys it
   logarithmically. **This is the package the evidence points at.**
