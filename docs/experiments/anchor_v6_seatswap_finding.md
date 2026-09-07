# Sealbot anchor v6 — the R3 seat swap: the asymmetry is the COLOUR's, and D-612's flip clause fires. Revision 1.

**UNEQUAL MOVETIME (D-697).** This run gave pistol **500 ms** per turn and sealbot **300 ms** (`movetime_ms = 500` against `time_limit_seconds = 0.3`), and the forfeit thresholds were 120 s against 5 s. Every number below is a number from a run in which pistol had 1.67x the clock, and may not be quoted without that qualifier. **The seat-swap conclusion stands**: the handicap travels with the engine, so it is INVARIANT across the slot exchange, which is what makes the slot-versus-colour test survive it.

**Run revision** `93df506` (`dev`), config `local/sealbot_anchor_v6_seatswap.toml`,
engine config `configs/play_staged_v0.toml` as committed, budget `movetime 500`,
turn cap 60, **the same 50 openings of `book_v1`, each played from both seats**.
Receipts `artifacts/sealbot_anchor_v6_seatswap/report.txt` sha256
`7eeacb7b9a418b7d…`, `report.json` `f89364c5856ae775…`. The v5 run it is compared
against is `artifacts/sealbot_anchor_v5_seat1/report.json` `b945df466a8d1342…`,
which is the digest `anchor_v5_finding.md` names.

**What D-612 registered**: *"the same 50 openings with the engines' seats swapped
relative to the harness's seat assignment"*, to separate opening bias from
harness asymmetry — *"an asymmetry that follows the SLOT is the harness's, one
that follows the COLOUR is the game's."* **Report only.** An anchor is not an
SPRT, sealbot is UNVERIFIED (D-197), and no standing judgment moves.

## The result, by seat and by colour

The config is byte-identical to v5's but for which harness slot each engine
occupies and the output directory. In v5 pistol was `engine_a`; in v6 it is
`engine_b`.

| | v5 (pistol in slot A) | v6 (pistol in slot B) |
|---|---|---|
| **pistol as p1** | **14 / 50** | **15 / 50** |
| **pistol as p2** | **26 / 50** | **27 / 50** |
| pistol total | 40 / 100 | 42 / 100 |
| sealbot as p1 | 24 / 50 | 23 / 50 |
| sealbot as p2 | 36 / 50 | 35 / 50 |
| sealbot total | 60 / 100 | 58 / 100 |
| **p1 wins, both engines** | **38 / 100** | **38 / 100** |
| **p2 wins, both engines** | **62 / 100** | **62 / 100** |

## The one sentence D-612 asked for

**The 14/26 asymmetry follows the OPENING COLOUR and not the harness seat**: with
the slots exchanged, every per-colour count reproduces within one game — pistol
14→15 as p1 and 26→27 as p2, sealbot 24→23 and 36→35 — and the colour totals
across both engines are **38 / 62 in each run, identical**.

## What that licenses, and what it does not

**D-612's flip clause fires as written**: *"Flips if the swap reproduces the
split on the same colour, which makes the asymmetry a property of this engine at
this budget and a package of its own."* It reproduces. So the asymmetry is not
the harness's, and the harness needs no fix on this evidence.

**It is not a p1/p2 imbalance claim about the GAME.** Two things are confounded
and this test separates neither: `book_v1`'s fifty openings are three-turn
prefixes drawn from one book, and both engines are the ones being measured. A
second-player edge of this size over one book, two engines and two runs is a
finding about *this measurement*, not about hex-lattice Connect(6,2,1).

**What is new and was not visible in v5**: the effect is **not pistol's alone**.
sealbot also wins far more as p2 (36 and 35 of 50) than as p1 (24 and 23), so the
v5 finding's framing — *"Pistol is clearly worse as p1"* — is the smaller half of
what is there. Both engines are worse as p1, by similar margins, in both runs.

**The compute, for the record**: 100 of 100 distinct games, 50 distinct openings,
sealbot 986 answers at median 300 ms and max 315 ms against its 0.3 s limit,
206.4 s wall against pistol's 369.8 s.

## What is owed

Nothing by this document. The package D-612's flip clause names — why this engine
is worse as p1 at this budget — is a package of its own, and it wants an
instrument that is not an anchor against an unverified opponent.
