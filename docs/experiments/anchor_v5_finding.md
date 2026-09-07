# Sealbot anchor v5 — the first anchor this project has run with a real denominator, and it overturns v4.

**UNEQUAL MOVETIME (D-697).** This run gave pistol **500 ms** per turn and sealbot **300 ms** (`movetime_ms = 500` against `time_limit_seconds = 0.3`), and the forfeit thresholds were 120 s against 5 s. Every number below is a number from a run in which pistol had 1.67x the clock, and may not be quoted without that qualifier.

**Run revision** `03aedd3` plus the openings reader, engine sha256
`7faefd6ff8d0fb3f04a8c93a120ab988a3d42ca5f950f78c7bdaddf84caff0df`, config
`configs/play_staged_v0.toml` **as committed**, budget `movetime 500`, turn cap
60, **50 openings of `book_v1` each played from BOTH seats** (D-568 licenses
`book_v1` for anchors). Receipts `artifacts/sealbot_anchor_v5_seat1/report.txt`
sha256 `46df71a7567a618f…`, `report.json` `b945df466a8d1342…`.

## The result

| | |
|---|---|
| totals | **40 W / 60 L / 0 capped / 0 forfeited** |
| as p1 | 14 W / 36 L |
| as p2 | 26 W / 24 L |
| Wilson 95 % on decided games | **[0.309, 0.498]** |
| **distinct games** | **100 of 100** |
| **distinct openings** | **50** |
| per-answer wall | 1040 answers, median 500 ms, **max 515 ms** |
| compute | A 171 904 403 nodes, 403.3 s; B 227.1 s |

## What it overturns

**v4, on the platform's single opening, reported 0 W / 100 L** and D-606 recorded
it with the caveat that its honest denominator was 2. **With fifty openings the
same engine, config and budget win 40 % of decided games.** The v4 result was one
position's knife edge, exactly as the caveat said it might be — and the caveat
could not be discharged until the reader existed.

**Two independent runs of this config agree.** The run made before the
`distinct_games` fix reported 40 W / 59 L / 1 capped; this one reports 40 W /
60 L / 0 capped. A movetime seat is not reproducible by construction (D-22), so
the games differ; the aggregate does not.

**The seat asymmetry is the new fact.** Pistol is clearly worse as p1 (14/50)
than as p2 (26/50). Against the platform's own opening it was the reverse. This
is a real signal that fifty openings can see and one cannot, and it is not
explained here.

## What it does NOT say

An anchor is **not a strength claim**. sealbot is UNVERIFIED (D-197) and no
standing judgment moves. The Wilson interval is honest here — n = 100 with
distinct-n = 100 is a real n, the first this project has had — but it is an
interval about ONE opponent at ONE budget, not an Elo.

**D-534 is untouched.** This is the gates-off seat; its 15 ms maximum overshoot
against a 500 ms budget is the same shape D-520 measured at 8 ms, and says
nothing about the solver seat's 725 ms median overshoot, which remains the HeXO
forfeit risk.

## The reader this run needed, and what building it turned up

`docs/experiments/anchor_v3_openings_design.md` specified it and
`matrix_anchor_openings_reader.md` selected option O2 after three red-team
rounds; it had never been implemented, so every anchor before this one had a
denominator of 2. It is implemented now, with eleven refusals each tested.
Building it found two defects in the harness that only a book could expose:

1. **`distinct_games` under-counted.** It hashed the engines' turns and not the
   opening, so two games from DIFFERENT openings whose engine moves coincided
   counted as one — under-reporting the very diversity the number exists to
   measure. The opening is now part of a game's identity.
2. **The transcript's `opening` field was a constant string** naming the
   platform setup whatever had actually been played, and the report header said
   "not paired" of a paired run. Both are now read from the run.

**And a defect in the tests for it**: the body digest is the outermost gate, so
every hand-broken book failed on R1 and masked the refusal its case was written
to prove. Four cases passed while testing nothing. The suite now re-digests each
broken body.
