# OPTION MATRIX — what v7 does about the unequal budget — **REVISION 1, SUPERSEDED**

**SUPERSEDED by `matrix_anchor_v7_budget.md` revision 2.** Its
DECISION-RED-TEAM landed **14 of 15** attacks and the recommendation FELL. Kept
because the ADR that settles this decision cites what was attacked, and because
two of the attacks are about defects in this document that a successor should be
able to read in place: its rejection of O1 was measured against a population in
which the thing measured did not yet exist (A5), and it called an estimate
unmeasurable that the red-team measured in seconds (A10).

**The named decision.** `docs/experiments/sealbot_anchor_v7_protocol.md` §A1 found
that anchors v5 and v6 gave pistol **500 ms** per turn and sealbot **300 ms**, and
registered "equal MEASURED movetime per side" as the remedy. Its REVIEW-design
returned FAIL, MAJOR-1: **equalising the budget IS a budget change, and this
series' own standing rule makes a budget change a NEW anchor** —
`docs/experiments/sealbot_anchor_v3_prereg.md:24`, quoting D-438:

> "An opening-policy change, a **budget change**, or any engine change is a NEW
> anchor, not a re-reading."

So the protocol cannot both equalise the clock and call itself "the next anchor
in the v3-v6 series" with a comparison table against v5 and v6. That is a named
decision with more than one viable option, and CLAUDE.md settles those with a
matrix attacked by a fresh-context DECISION-RED-TEAM **before** selection.

**Revision** `e5f4364`. Every number below is marked **MEASURED** or
**ESTIMATED** (D-291).

## The measured ground

Derived from the 200 per-game transcripts under
`artifacts/sealbot_anchor_v5_seat1/` and `artifacts/sealbot_anchor_v6_seatswap/`
by a script written for this matrix (not by re-reading `report.txt`, which
publishes only slot A):

| quantity | v5 | v6 | how |
|---|---|---|---|
| games / answers | 100 / 2090 | 100 / 1964 | **MEASURED** |
| answer-wall, pistol | 403.3 s | 369.8 s | **MEASURED** |
| answer-wall, sealbot | 227.1 s | 206.4 s | **MEASURED** |
| answer-wall, total | **630.4 s (10.5 min)** | **576.2 s (9.6 min)** | **MEASURED** |
| pistol result | 40 W / 60 L | 42 W / 58 L | **MEASURED** |
| pistol per-answer excess over budget, max | 15 ms | 14 ms | **MEASURED** |
| sealbot per-answer excess over budget, max | **106 ms** | 15 ms | **MEASURED** |
| sealbot excess, first answer of each game | median 15 ms, max 106 ms | median 6 ms, max 15 ms | **MEASURED** |
| sealbot excess, every later answer | median 0 ms, max 11 ms | median 0 ms, max 4 ms | **MEASURED** |

**THE LAST TWO ROWS ARE WHY THIS MATRIX EXISTS AND NOT ONLY MAJOR-1.** Sealbot's
overshoot is concentrated in the FIRST answer of each game and is absent
afterwards, and its size differs 7× between two runs of a byte-identical config.
That is not either of the two terms §A2 named (`≤ 1024 nodes past the deadline`
and the untimed C++ setup): `tools/sealbot/matchserver/src/sealbot_client.rs`
respawns the shim per game and does not wait for its `ready` line, and
`tools/sealbot/sealbot_shim.py` rebuilds the game in Python on every request, so
interpreter start-up and the Python replay are inside the wall the server
measures. `engine_time_ms` is `null` for **all 2036** sealbot answers across both
runs (**MEASURED**) and non-null for **all 2018** pistol answers, so nothing
separates sealbot's search from its overhead. **Equal WALL is therefore not equal
SEARCH, and the residual is entirely on the opponent's side.**

## The options

### O1 — run at the series budgets (500/300), label the inequality

v7 changes nothing about the clock. The inequality becomes a standing LABEL on
v5, v6 and v7 alike, and the comparison table is legitimate because all three
runs share the handicap.

- **Cost**: **MEASURED** ~10.5 min of answer-wall per 100 games, the same as v5.
- **What it buys**: the series keeps its one job — a change in pistol shows up as
  a change in the series, because everything else is held.
- **What it fails at**: it does not answer "is pistol better than sealbot", and
  never claimed to. A reader who wants a fair head-to-head does not get one.
- **Failure mode**: the label is written and then dropped by a later citation, as
  it already has been — **MEASURED**: `git grep` finds the v5 numbers unqualified
  at `docs/decisions.md:1282` (D-608), `:1290` (D-612),
  `docs/experiments/opt_arc_CLOSURE.md:13,36`,
  `docs/experiments/opt_arc_ledger.md:498`,
  `docs/experiments/opt_arc_perf_finding.md:67`.

### O2 — equalise, and declare v7 the head of a NEW series

v7 runs at 500/500, states plainly that it is series B, publishes no comparison
table, and v5/v6 stay where they are as series A.

- **Cost**: **ESTIMATED** ~13 min per 100 games — pistol's 403 s unchanged,
  sealbot's 227 s scaled by 500/300 to ~378 s. The scaling is an estimate and
  could be measured only by running it.
- **What it buys**: the first anchor in this project's history at an equal
  configured clock.
- **What it fails at**: **it starts a series at n=1.** Its own value is a single
  point until a second run exists, and the trigger (§A7: the first Phase 2 `h1`)
  is the only scheduled occasion for one.
- **Failure mode, and it is the sharp one**: equal WALL is not equal SEARCH (the
  measured ground above), so "equalised" would be a claim the run cannot support.
  A reader would take series B as the fair one and it would not be.

### O3 — run both budget pairs, and measure the budget change's own effect

v7 runs 500/300 AND 500/500 over the same 50 openings, and reports both.

- **Cost**: **ESTIMATED** ~23.5 min of answer-wall per pair of runs (10.5 + 13),
  plus double the operator attention. Still under half an hour of machine time.
- **What it buys**: the confound MAJOR-1 names is DISSOLVED rather than argued
  about — the 500/300 arm is comparable to v5/v6 and the 500/500 arm is the new
  series' first point, from the same session, same machine, same openings.
- **What it fails at**: two runs is two chances for the harness to differ, and
  the two arms are not independent samples of anything — they share the openings.
- **Failure mode**: it makes v7 twice as expensive for a document whose registered
  output is "direction only", and a reader may read the DIFFERENCE between the two
  arms as a measurement of the budget's worth, which at n=100 with no SPRT it is
  not.

### O4 — do not run v7; split the fairness instrument off as its own package

The measured ground says the harness cannot currently express "equal search
budget": sealbot reports no engine time, and its measured wall carries
per-game process start-up. O4 says the anchor waits for a harness that reads
sealbot's own clock, and v7 is withdrawn until then.

- **Cost**: **MEASURED** zero machine time now; **ESTIMATED** a harness package —
  consume the shim's `ready` line in `new_game`, and have the shim report the
  bot's own elapsed time — which is two small changes to files this repository
  owns (`sealbot_client.rs`, `sealbot_shim.py`), not to sealbot.
- **What it buys**: every later anchor gets a fair clock, and the 106 ms outlier
  stops being unexplained.
- **What it fails at**: it leaves the project with no anchor protocol at all
  until that package lands, and the trigger for v7 may arrive first.
- **Failure mode**: the package is never scheduled, and "we will fix the clock
  first" becomes the reason no anchor is ever run again.

## Recommendation

**O3**, with O4's harness fix named as owed but not blocking.

The reason is the measured ground and not a preference: the 500/300 arm is the
only thing that keeps faith with D-438, and the 500/500 arm is the only thing
that answers the question §A1 raised. Running both costs **ESTIMATED** ~23.5 min
against a budget the project spends on a single SPRT tranche in hours, so the
cheapness is not a tiebreaker — it is the whole argument for refusing to choose.

O1 is rejected because it re-registers the defect as a label and the label has
**measurably** already failed to travel. O2 is rejected because it would publish
"equalised" as a property the harness cannot deliver. O4 is right about the
mechanism and wrong about the schedule: the harness fix is small and should be
done, but it is not a precondition for a run that reports DIRECTION ONLY and
names its own overshoot terms.

**What O3 owes that the current §A7 does not register**: the two arms are
reported as two rows, never subtracted; the 500/500 arm carries the sentence that
its equality is a CLOCK equality and not a search equality, with the measured
first-answer overhead beside it.
