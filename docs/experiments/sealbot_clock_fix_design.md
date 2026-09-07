# Design — make the sealbot seat's measured wall mean its search time. Revision 3.

**Revisions 1 and 2 are kept as `..._rev1_SUPERSEDED.md` and
`..._rev2_SUPERSEDED.md`.** Both FAILED their REVIEW-design — 7 MAJOR / 10 MINOR
and 6 MAJOR / 8 MINOR. Where a finding changed this document it is named where it
changed it. **The implementation landed at `2fe3f32` and is under its own
REVIEW-impl**; this revision brings the design into line with what was found and
with what the instrument now does.

**The precondition D-699 names.** Changes to `tools/sealbot/` only. No change to
sealbot, to any engine, or to pistol. No match is run by this package.

---

## §1 The before-state: the gap is a BOUND, and the bound is loose

D-699 records the gap as 4.50 % of sealbot's wall in v5 and 3.50 % in v6, because
`engine_time_ms` is null for **1050 of 1050** and **986 of 986** sealbot answers,
so an answer's non-search time can only be bounded by its whole wall.

**MEASURED** with an instrumented copy of the shim, warm process, at a 300 ms
limit — including the early-return branch the bound is loosest on, which
revision 1 omitted and a reviewer supplied over 51 real positions:

| case | wall | `get_move` | **non-search** |
|---|---|---|---|
| 5 / 9 / 21 stones, budget-consuming | 300.4–301.0 ms | 300.2–300.8 | **0.19–0.22 ms** |
| **win-in-one, EARLY RETURN** | **0.43 / 0.12 ms** | 0.30 / 0.00 | **0.13 / 0.12 ms** |
| 51 early-returning real positions (reviewer) | — | — | median **0.147**, max **0.305 ms** |

**BUT 0.2 ms IS NOT THE REFERENT AND THIS DOCUMENT NO LONGER USES IT AS ONE.**
It was measured through a Python driver standing in for `SealbotClient`, so the
Rust-side JSON encode of up to 110 coordinate pairs, the pipe write, the reader
channel hop and the decode are not in it. The in-harness referent is the seat
that already reports engine time — pistol, on these same runs:
**min 0, median 1 ms, p95 5 (v5) / 8 (v6), max 15, zero negatives.**
Revision 2 stated that and then computed its own table at 0.2 ms anyway
(**M2**); this one uses **1 ms per answer** throughout.

**THE START-UP CHARGE IS PER RUN, NOT A CONSTANT, AND REVISION 2 USED 26 ms FOR
BOTH (M1).** Measured, sealbot's mean first-of-game wall excess:

| | v5 | v6 |
|---|---|---|
| sealbot's mean first-of-game excess | **22.08 ms/game** | **6.44 ms/game** |
| which harness slot sealbot occupies | B | **A** |

**The difference is spawn order, not noise.** `referee.rs:156,163` calls
`a.new_game` and then `b.new_game`, so slot A is started first; in v6 sealbot IS
slot A, and pistol's own spawn absorbs most of sealbot's start-up before sealbot
is first asked. Revision 2 attributed the run-to-run difference to machine load
and charged 26 ms to both — which is revision 1's different-denominators defect
recommitted in the table written to fix it.

**So the looseness, computed on one referent and one measured constant:**

| | v5 | v6 |
|---|---|---|
| bound today | 10 226 ms (4.50 %) | 7 230 ms (3.50 %) |
| measured non-search: `1 ms x answers + charge x games` | 3 258 ms (1.43 %) | 1 630 ms (0.79 %) |
| **bound is loose by** | **3.1x** | **4.4x** |
| residual bound with the first answers removed | 8 018 ms (3.53 %) | 6 586 ms (3.19 %) |
| measured, at 1 ms/answer | 1 050 ms (0.46 %) | 986 ms (0.48 %) |
| **residual bound is loose by** | **7.6x** | **6.7x** |

The residual row is `bound minus the measured charge` and is not a recomputed
bound: the bound is discontinuous at the budget, so subtracting start-up from a
first answer can push it below the deadline and into the looser branch.

## §2 Change A — the seat is not charged for starting up

Measured: **26.1 ms** from spawn to `sealbot_shim: ready`, of which 13.4 ms is
interpreter start before `main()` runs. The shim writes the line only to
**stderr**, which `LineProcess::spawn` redirects to a file, and `new_game`
returns without reading anything — so the prefix lands in the first answer.

| | how the client learns the shim is ready | verdict |
|---|---|---|
| **A1** | the shim writes the line to **stdout as well as stderr**; `new_game` reads the stdout line before returning | **SELECTED** |
| A2 | poll the stderr file | rejected: a redirect gives the reader no flush guarantee |
| A3 | send a throwaway SEARCH request | rejected: **ESTIMATED** 300 ms x 100 games to measure nothing |
| A5 | send a `ping` request and read a `ping` reply | rejected, **and for a stronger reason than revision 2 gave**: the shim reads `request.get("setup", [])` / `get("moves", [])` and then calls `bot.get_move` unconditionally, so an unmodified shim answers a `ping` with a real search and a move array indistinguishable from a reply. **A5's failure mode is silent; A1's is a named `Protocol` refusal.** |
| A4 | one shim process for the whole match | **rejected on correctness**: `_tt` is sized in both constructors (`bot.h:33,37`) and nothing under `current/` clears, resizes or reassigns it — only `_history` and the killers reset (`search.h:47-48`) — so one process across paired games would leak between colours |

**BOTH STREAMS**: the per-game `.stderr` files carry the `ready` line and nothing
else, and that is what proves one shim process per game. Moving it would empty
them.

**THE TIMEOUT IS `turn_timeout_seconds`**, as `pistol_client.rs:97` already uses
for its own handshake — no new literal, no new config key (hard rule 1).

**AND THE SHIM MUST FLUSH STDOUT** (MINOR): Python block-buffers stdout to a
pipe, so an unflushed preamble deadlocks `new_game` until the timeout. The
shipped shim flushes both streams.

**SCOPE**: `tools/sealbot/tests/stub_sealbot.py` is the sealbot seat in every
matchserver test, and emits the same preamble. Without that, all 19 configs
`run_tests.sh` builds would take a pregame forfeit.

## §3 Change B — the seat reports its own elapsed time

The shim times `bot.get_move(game)` and returns `engine_time_ms`; the client
REQUIRES it and fills `EngineReply::engine_time_ms`, already plumbed to the
transcript (`referee.rs:291`, `transcript.rs:44`).

**WHOLE MILLISECONDS, FLOORED — NOT ROUNDED (M6).** `wall_ms` is
`started.elapsed().as_millis()`, which truncates. A shim that ROUNDED would
report more than the wall it was measured over on any answer whose fractional
part exceeded the client's: **MEASURED over 100 real positions, `int()` gives 0
negatives and `round()` gives 27**, which would fail §4's lower bound on a
conforming shim. Revision 2 pinned "integer" — which `round()` satisfies — and
missed this.

**IT IS NOT PURE SEARCH**: `get_move` carries sealbot's untimed setup, its
≤1024-node overrun and the `memcpy` rollback. The field is **the engine's own
elapsed time for the answer**.

## §4 The criteria — three, not four

**A TURN THE ENGINE NEVER ANSWERED IS NOT AN ANSWER (M5).** `referee.rs:179-189,
220-232` writes a record with `wall_ms: 0` and a null engine time on any engine
failure, pregame ones included — and **Change A newly creates the pregame case**,
because a shim that never prints `ready` now blocks and forfeits. Counting those
poisons everything: **MEASURED on a two-game fixture with one timeout, the
reader reported `3 of 4` engine times and a first-of-game median excess of
−150 ms**, which reads as a start-up charge that has been fixed on a seat that
never started. `tools/anchor_overshoot.py` now excludes `engine_failure` records
and REPORTS how many it excluded; every criterion below is over answered turns.

The precondition is discharged when, on a run of **at least 20 games**:

1. **On the ASKED-FIRST sub-population** — games whose first turn record is
   sealbot's — its first-of-game median wall excess is within **2 ms** of its
   later-answer median.
   **POWER, MEASURED on the pre-fix data**: fails at **41 of 41** game counts
   from 20 to 60, in both v5 and v6, under both readings of "later" — asked-first
   median excess 42.5 ms (v5) and 13.0 ms (v6) against later-answer medians of 0.
   **FALSE-FAIL RATE, MEASURED**: 0.00 % by bootstrap over the post-fix proxy
   population at n = 10, 15, 25, 30 and 50 asked-first games. The threshold is
   derived from a measured residual of ~1 ms and is not tight.
2. `engine_time_ms` is a **non-null integer on every answered sealbot turn**.
3. **Two-sided**, per side: `0 <= wall_ms - engine_time_ms`, with **p95 <= 5 ms
   and max <= 25 ms**. Derived from the in-harness referent above, not from a
   multiple of the Python-driver number: revision 1's "per answer <= 5 ms" was
   failed by the seat that already works (4.90 % / 5.11 % of pistol's answers).
   **The lower bound is what excludes a shim reporting its CONFIGURED budget**:
   on the **196 of 1050 v5 answers whose wall is 0 ms** such a shim reports a gap
   of −300. `tools/anchor_overshoot.py` prints the min and the negative count.

**REVISION 2's FOURTH CRITERION IS DELETED, NOT REFINED (M3, D-424).** It asked
that the fraction of answers with `engine_time_ms < 250` be within 5 pp of the
fraction with `wall_ms < 250`. **It cannot fire while (3) holds**: (3) forces
`w − 25 <= e <= w`, so the two fractions can differ only on answers with
`250 <= w < 275` — **MEASURED, 3 of 1050 (0.29 pp) in v5 and 1 of 986 (0.10 pp)
in v6**, seventeen times inside its own 5 pp allowance, and 0.00 pp when driven
against the working pistol seat and against the shipped shim. And the defect it
named is already excluded by (3)'s lower bound. Both sides of the distinction
licensed the same conclusion, so it is deleted.

**WHAT NO CRITERION HERE PINS, stated rather than left implicit**: WHICH SPAN
`engine_time_ms` wraps. (3) is monotone the wrong way — a shim folding more
overhead into the reported time scores better. Measured materiality is small (the
whole foldable span is 0.118–1.270 ms per answer, median 0.541, against an
integer comparison with a 5 ms allowance), and the test that would pin it is a
shim driven against a bot whose `get_move` sleeps a known interval. **Recorded as
a known limit, not scheduled.**

## §5 What this delivers, and what it does not

**The instrument** `tools/anchor_overshoot.py` now carries the asked-first /
later split, the `wall − engine_time` min/median/p95/max with a negative count,
per-engine wall and engine-time totals with the engine's share of wall, and the
excluded-turn count — each with a test, and the engine-time column driven by
non-null fixtures. `report.rs` still carries no `engine_time_ms`, so the
summary report does not report it and the transcripts remain the instrument.

**Of D-699's three resume items this delivers two** — the shim change, and a
criterion — and defers the third, the ADR amending D-695. **AND THE CRITERIA ARE
REGISTERED, NOT RUN**: no match is run by this package, so the precondition is
not discharged until an anchor is played and read against §4. This document does
not select an option in `matrix_anchor_v7_budget.md` and does not touch pistol.

**ADR DEBT**: §2's matrix was attacked by two fresh-context REVIEW-design rounds
before selection; the ADR line recording the choice carries the strongest
surviving attack, and is written when the REVIEW-impl closes.
