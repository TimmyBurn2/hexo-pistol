# Sealbot anchor v4 — the arc made the engine 1.43x faster per answer and it LOST the one game it used to win.

**Run revision** `03aedd3`, engine sha256 `7faefd6ff8d0fb3f04a8c93a120ab988a3d42ca5f950f78c7bdaddf84caff0df`,
config `configs/play_staged_v0.toml` **as committed**, budget `movetime 500`,
100 games, turn cap 60. Receipts: `artifacts/sealbot_anchor_v4_seat1/report.txt`
sha256 `f4789152a1c62a14…`, `report.json` `5595136916320e5c…`.

## The two anchors side by side, same config, same budget, same opponent

| | v2 (`411c122`, before the arc) | **v4 (`03aedd3`, after)** |
|---|---|---|
| pistol as p1 | **20 W / 0 L** — pistol wins at turn 19 | **0 W / 50 L** — sealbot wins at turn 42 |
| pistol as p2 | 0 W / 20 L — sealbot wins at turn 15 | 0 W / 50 L — sealbot wins at turn 15, **unchanged** |
| totals | 20 W / 20 L | **0 W / 100 L** |
| distinct games | 2 of 40 | **2 of 100** |
| nodes per answer | 147 141 | **209 743 — 1.425x** |
| per-answer wall | median 500 ms, max 508 ms | median 500 ms, max **510 ms** |

**EXACTLY ONE OF THE TWO DISTINCT GAMES CHANGED.** The p2 game is the same game
it was: sealbot moves first and wins at turn 15, before and after. The p1 game
flipped from a pistol win at turn 19 to a pistol loss at turn 42.

**THE OPPONENT DID NOT MOVE.** `/home/tom/Work/sealbot-scope/sealbot/current` was
last modified **2026-08-19** and nothing under it is newer than 2026-08-20; the v2
anchor was recorded **2026-08-31**. Same binary path, same shim, same 0.3 s limit.
The change is attributable to pistol and to nothing else in the setup.

## What this is, and what it is NOT

**It is not a strength claim and not a regression finding.** The honest
denominator is **2**, exactly as D-519 said of v2 — one game per seat assignment,
repeated 50 times each because both engines are deterministic from the platform's
fixed opening. The report's `Wilson 95% [0.000, 0.037]` is computed on n = 100
and is meaningless at n = 2; it is quoted here only because the report prints it.
sealbot remains UNVERIFIED (D-197) and the standing judgment is not moved.

**It is a flag, and a sharp one.** The arc's one unambiguous success is speed —
**1.294x** on the bench, **1.425x** in nodes per answer at the deployment budget
— and on the single position anyone can watch, that extra depth selected a
sequence that loses. The gated-off packages cannot be the cause: S1, S2 and S3
are byte-identical at their committed zeros, measured, and P1 and P3 are
output-identical by construction and by their own identity legs. **Deeper search
with the same eval is the only thing that changed.**

**The hypothesis this points at is already on the record.** D-428 holds that eval
v0 misreads horizons; a deeper search hands that eval more positions it reads
wrongly, and search pathology — more depth, worse choice — is exactly what a weak
static evaluator produces. S2's own SPRT is the same shape from the other
direction: an extension that bought depth on forcing lines lost 8 W / 61 L.

**What would settle it, and it is item 2 of what this arc leaves owed**: an
anchor with an openings book. At two distinct games a flip like this cannot be
told from noise; at a hundred it can. The reader was designed, option-matrixed
through three red-team rounds and reviewed, and never implemented, so the anchor
still cannot answer the question it is being asked. **This result is the
strongest argument yet for building it.**
