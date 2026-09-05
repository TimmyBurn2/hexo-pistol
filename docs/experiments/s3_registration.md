# S3 — late move reductions on the unforced range. Registered before the match.

**Governing revision:** `d2333c5`. **Instrument:** this package's own binary,
`configs/instrument_v0.toml` with `lmr_min_depth_turns` varied and nothing else.

## What the mechanism is

A candidate deep in the UNFORCED range of a `BATCHED` or `BATCHED-lost` row is
one the generator ranked last and the ordering heuristics did not lift. Its
null-window SCAN runs one whole TURN shallower. **Any reduced scan that beats
alpha is re-searched at full depth on the same null window before it may raise
alpha or open the full window**, so a reduction can cost time and never a move.

Three things are never reduced: the forced prefix of any row (its cells are
threat covers), every `FILTERED` and `WIN-NOW` row, and the first
`lmr_late_index` candidates of the unforced range. The reduction is two plies,
never one — D-111 forbids a horizon that lands between a turn's two stones.

## Sensitivity check, run first

24 governed openings at `nodes 200000`, bestmoves against the committed seat:

| `lmr_min_depth_turns` | bestmoves changed |
|---|---|
| 1 | **most of the 24** — the mechanism dominates at this reach |
| 2 | 2 of 24 |
| 3 | 0 of 24 |

The reach is what decides: at 200 000 nodes the search completes few enough
turns that a floor of 3 puts every node below the reduction's own threshold.
**The registered seat is `lmr_min_depth_turns = 1`, `lmr_late_index = 3`** — the
only setting with a population large enough for a match to answer.

## Identity at the committed value

Baseline `pistol` at `d2333c5` against this revision's binary, both at
`configs/instrument_v0.toml`, 24 openings at `nodes 200000`: 24 bestmoves a
side, 0 errors, one shared bestmove-plus-node digest `d4ee0f05322d2902…`.

## Pre-registered SPRT — bounds fixed before the first game

- **H0** Elo 0, **H1** Elo +10; alpha = beta = 0.05; cap 600 paired games.
- Budget `nodes 200000`, an instrument budget per hard rule 6.
- **Disposition**: H1 accepted → the committed default moves to 1. H0 or
  UNDECIDED → the key lands at 0 and the package closes as a measured finding.
- Node identity is not asserted and cannot be: the seats search different trees
  by construction.

---

# RESULT — UNDECIDED at the 600-pair cap, and the key lands at 0 as registered.

The match ran the full cap without the log-likelihood ratio reaching either
bound, which is the registered UNDECIDED disposition and not a pass.

| | |
|---|---|
| verdict | **`inconclusive_at_game_cap`** — `LLR pair -1.831181`, bounds ±2.9444 |
| n / distinct-n | **1200 / 1200**, no duplicate games, 600 pairs |
| record for `lmr1` | 439 W / 475 L / 286 capped (capped fraction 0.238) |
| pair outcomes | p0 73, p1 102, **p2 273**, p3 92, p4 60 |
| normalized Elo estimate | **−13.42** |
| deepest turn reached | **6 for both seats** |
| nodes | `lmr1` 2 194 393 593 over 13 455 searches; `lmr0` 2 259 236 513 over 13 479 |
| receipt | `artifacts/s3_sprt_v1.txt` |

**THE REDUCTION BOUGHT NO DEPTH.** Both seats reached 6 turns on nearly equal
nodes — the reduced seat spent 2.9 % FEWER nodes and got no further. A reduction
pays for depth by scanning late children shallower; when the verification
re-search fires often enough, the saving is spent re-searching and the depth
never arrives. The pair histogram says the same thing from the other side: 273
of 600 pairs split 1-1, the largest bucket by a wide margin, which is what two
engines that mostly agree look like.

**THE WALL COLUMN IS NOT EVIDENCE HERE and is deliberately not quoted as a
ratio.** The seats' wall figures differ by 1.77x, but this match shared the box
with a sealbot anchor and a workspace test run. The budget is `nodes`, so the
VERDICT is untouched by that contention — which is the reason hard rule 6 wants
an instrument budget — but any timing read off this run would be reading the
other jobs.

**Disposition, as registered**: UNDECIDED, so `lmr_min_depth_turns` lands at
**0** and the package closes as a MEASURED FINDING. An undecided SPRT is not a
pass, and the registration said so before the first game.

**What would flip it**: a seat that searches deep enough for a whole-turn
reduction to be a small fraction of the remaining depth. At six turns, cutting
two plies is cutting a third of what is left, and the verification re-search
then costs more than the reduction saves. This is the same shape as S2's finding
from the other direction (D-605) and points at the same cause.

## One fix landed after the match, and why the verdict still holds

Reading the diff for early returns turned up an overflow: `lmr_min_depth_turns *
TURN_STONES` wraps for a config value above 2^31, turning a huge floor into a
tiny one and granting reductions the guard exists to refuse. That is a wrong
answer from a legal config value, so it is fixed rather than noted (CLAUDE.md's
overrule clause reaches prose, never correctness).

**The binary therefore changed after the match, and the transfer is MEASURED
rather than argued.** The benched binary
(`761f1b3a08cc243aa858bfee19e90514075344c43abc4e3fd33e497a0bbf4f20`) and the
landed one (`45063b3679b2934e7881ace0b6f4885b362a4cac368a849e4d3bb78632267135`)
were run over the same 24 governed openings at `nodes 200000` at **both seats**,
and each seat's bestmove-plus-node transcript has one digest across the two
binaries: `d4ee0f05322d2902…` at `lmr_min_depth_turns = 0` — which is also the
pre-S3 baseline's digest — and `b66c19e679892a5e…` at 1. The seats the SPRT
played answer identically before and after the fix.
