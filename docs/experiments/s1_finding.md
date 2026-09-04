# S1 — aspiration windows: a MEASURED NEGATIVE, and the audit ranked it first

**The mechanism.** Every iteration opened `(-INFINITY, INFINITY)`
(`pvs.rs`'s `iterate`). S1 opens `(previous - delta, previous + delta)` for each
iteration after the first, and re-searches at full width when the score lands
outside. Mate scores open full width: `score.rs` puts mates far above
`EVAL_MAX`, so a window of a few evaluation units around one is a window the
next iteration leaves by construction. A fail re-searches at full width rather
than widening in steps, because one re-search is the whole cost and a staged
widening pays it twice on the same iteration.

**Armed per seat by `aspiration_delta`, `0` in all fifteen committed documents.**

## The measurement

16 governed-book openings at `go nodes 200000` (deep enough for four
iterations, so the window is actually exercised), capped seat against the
committed one:

| `aspiration_delta` | openings whose move changes | total ms | ratio |
|---|---|---|---|
| 25 | 0 of 16 | 4 371 → 5 364 | **0.815** |
| 50 | 0 of 16 | 4 371 → 8 607 | **0.508** |
| 100 | 0 of 16 | 4 371 → 8 407 | **0.520** |
| 200 | 0 of 16 | 4 371 → 7 790 | **0.561** |

**Every width is slower and none changes a move.** At `delta = 50` the search
takes nearly twice as long to reach the same answers.

## Why, and the audit half-predicted it

`search_gap_2026-09.md` ranks C-1 **first** — *"cheapest positive-expectation
change"* — while its own analysis contains the reason it is not:

> a re-search on a window the PVS null-window scan already exploits buys less
> than in engines that lack PVS

The measurement says it buys less than nothing. Two things this engine's shape
does to the bet: iterative deepening here runs to **2–4 turns**, not the 20-plus
of a chess engine, so a score has few iterations to settle and swings between
them by more than any small delta; and every node already runs a PVS null-window
scan, so the cutting a narrow root window would buy is largely already being
had. A failed window then costs a whole extra search of the same tree, and at
these depths it fails most of the time.

**The registered expectation was *"small positive or null on node count"*.
The measured result is a 20–100 % slowdown.** Rule 5: a measured structural
floor is a finding, not a failure.

## Disposition

The mechanism lands gated `0`, on the same footing as `tier_t_top_k`,
`root_reorder` and `safety_net_top_k`. What would flip it is a search that runs
appreciably deeper — a faster engine, a bigger budget, or a Stage-2 eval that
makes scores settle across iterations — since the bet's cost is fixed and its
payoff grows with the number of iterations it can inform.
