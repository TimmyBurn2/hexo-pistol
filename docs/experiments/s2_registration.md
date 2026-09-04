# S2 — the one-cell forced-reply extension. Registered BEFORE the match, and the first package of tranches 3 and 4 that earns one.

**Governing revision:** `d2333c5` (S1's landing). **Instrument:** the line-protocol
binary at this package's own revision, `configs/instrument_v0.toml` with
`extension_budget` varied and nothing else.

## What the mechanism is

A `FILTERED` row is the mover answering a threat, and its emitted set is the
cover. When that set holds exactly ONE cell the reply is singular by
construction — no exclusion search is needed to know it — so the subtree below
it is searched one whole TURN deeper. Whole turns only: D-111 forbids a horizon
that lands between a turn's two stones, so the grant is two plies, never one.

The grant is charged against a per-LINE budget, decremented on the way down and
**restored on the way back up**, so a sibling subtree begins with the allowance
this one was entered with. Without the restore the first forcing chain in the
tree spends the whole search's grant. `extension_budget = 0` grants none and is
the committed value.

## The sensitivity check, run first — and this time it does not foreclose

The check that foreclosed W1, W2 and S1 asks whether the mechanism is present in
play at all. It is, twice over.

| what was counted | result |
|---|---|
| FILTERED rows over 24 governed openings at `nodes 50000` | **3153**, across 14 of the 24 |
| of those, rows whose emitted set is exactly ONE cell | **933 — 29.6 %** |
| bestmoves changed, `nodes 50000`, budgets 1 / 2 / 4 | 0 / 0 / 0 of 24 |
| bestmoves changed, `depth_turns 2` | 0 of 24 |
| bestmoves changed, `depth_turns 3` | **6 of 24** |
| bestmoves changed, `nodes 200000`, budgets 1 / 2 / 4 | **12 / 12 / 12** of 24 |
| bestmoves changed, `nodes 500000`, budgets 1 / 2 / 4 | **12 / 14 / 14** of 24 |

**The 50 000-node reading is the one that would have misled.** At that budget
the mechanism fires 933 times and changes nothing, because iterative deepening
completes the same iteration either way. Read alone it looks exactly like W1's
and S1's foreclosures. At a budget a match actually spends it changes half the
openings. A sensitivity check run at one budget is a check of that budget, not
of the mechanism, and this package is the counter-example that says so.

**The effect saturates by budget 2**: 1 → 2 buys two more changed openings at
500 000 nodes and 4 buys none. The registered seat is therefore
`extension_budget = 2`.

## Identity at the committed value

Baseline `pistol` at `d2333c5` (`a2c02946def273b2…`) against this revision's
binary (`247aee8e2df33c1b…`), both at `configs/instrument_v0.toml`, 24 openings
at `nodes 200000`: 24 bestmoves a side, 0 errors, and the bestmove-plus-node
transcripts share one digest, `d4ee0f05322d2902…`. The key off is the tree
unchanged, measured rather than argued.

## Pre-registered SPRT — bounds fixed before the first game

- **H0** Elo 0, **H1** Elo +10; alpha = beta = 0.05.
- **Seats**: `extension_budget = 2` against `extension_budget = 0`, one binary,
  configs differing in that key alone.
- **Openings**: the governed paired book, both colours per opening.
- **Budget**: `nodes 200000` per ask — an instrument budget, per hard rule 6, so
  the result is not a clock measurement.
- **Cap**: 600 paired games. Undecided at the cap is reported as UNDECIDED and
  the key stays at 0; an undecided SPRT is not a pass.
- **Disposition**: H1 accepted → the committed default moves to 2 in every
  config, with the LLR and n in the ADR line. H0 accepted or UNDECIDED → the key
  lands at 0 and the package closes as a measured finding, like P2.
- Node identity is NOT asserted here and cannot be: the seats search different
  trees by construction. That is the point of the seat.

---

# RESULT — H0 accepted, and not narrowly. The extension is worse.

The match ran at the registered seat and bounds and stopped itself at the h0
boundary after 90 games, well inside the 600-game cap.

| | |
|---|---|
| verdict | **h0** — `LLR pair -2.956396` against the bound `-2.9444` |
| n / distinct-n | 90 / 90, no duplicate games |
| record for `ext2` | **8 W / 61 L**, 21 capped (capped fraction 0.233) |
| pair outcomes | p0 17, p1 19, p2 9, p3 0, p4 0 over 45 pairs |
| normalized Elo estimate | **−391.52** |
| deepest turn reached | **`ext2` 5, `ext0` 6** |
| compute | `ext2` 162 659 832 nodes / 944 searches; `ext0` 160 022 216 / 972 |
| receipt | `artifacts/s2_sprt_v1.txt`, sha256 `d8132c9dc75af503…`; report `eefbb48e2138ef9c…` |

**The deepest-turn row is the finding.** Both seats spent the same nodes, and
the extended seat reached one whole turn LESS. A singular reply is cheap per
node, but the extension grants it on 29.6 % of FILTERED rows, and those rows sit
on exactly the lines the search visits most; the depth it buys on a forcing
chain is paid for out of the iteration that would have completed. At eval v0 the
trade is not close — p3 and p4 are both empty, so `ext2` did not win a single
pair outright.

**Disposition, as registered**: H0 accepted, so `extension_budget` lands at **0**
in every config and the package closes as a MEASURED FINDING, like P2. The code
lands with it: the mechanism is implemented, tested, gated off, and the number
above is why the gate is shut. What would flip it is a deeper-searching seat or
an eval that can see past the extension's own horizon — the trade is between
depth on one line and depth everywhere, and it was measured at the shallowest
reach this engine has.
