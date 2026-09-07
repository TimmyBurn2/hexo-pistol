# Design — make the sealbot seat's measured wall mean its search time

**The precondition D-699 names**, and it is the thing that stopped the v7 budget
decision. Two changes to files this repository owns, in
`tools/sealbot/matchserver/` and `tools/sealbot/sealbot_shim.py`. **No change to
sealbot.** No change to any engine. No match is run by this package.

**Revision** `1b89034`. Numbers below are **MEASURED** or **ESTIMATED** (D-291).

## §1 The before-state, measured rather than bounded

`docs/decisions.md` D-699 records the gap as a BOUND — 4.50 % of sealbot's wall
in v5, 3.50 % in v6 — because `engine_time_ms` is null for **1050 of 1050** and
**986 of 986** sealbot answers, so an answer's non-search time can only be
bounded by its whole wall. **The bound is loose by roughly sixty times, and this
is the measurement that says so.**

A copy of the shim instrumented to time `bot.get_move` separately from the
Python game replay, driven directly at a 300 ms limit on the v5 anchor's own
opening and on mid-game positions (**MEASURED**, this workstation, warm process,
no match running):

| stones | server-observed wall | sealbot's search | Python replay | **non-search** |
|---|---|---|---|---|
| 5 | 300.49 ms | 300.30 | 0.10 | **0.19 ms** |
| 7 | 300.70 ms | 300.50 | 0.00 | **0.20 ms** |
| 9 | 300.42 ms | 300.20 | 0.10 | **0.22 ms** |
| 13 | 301.6 ms | 301.4 | 0.1 | **0.2 ms** |
| 21 | 301.0 ms | 300.8 | 0.1 | **0.2 ms** |

**On a warm process the server's wall IS sealbot's search time, to about
0.2 ms** — 0.07 % of a 300 ms budget — and the figure does not grow with the
stone count, so the Python replay that revision 2 of the budget matrix worried
about is not a term worth naming. Everything else is the **26 ms** of per-game
process start-up already measured twice (`matrix_anchor_v7_budget.md` §2.1).

**SO THE WHOLE OBSERVABLE GAP IS ONE PER-GAME CONSTANT PLUS A TENTH OF A
MILLISECOND**, and the reason the project could not say so is that the harness
throws away the one number that would prove it. That is what this package fixes:
it does not make sealbot faster, it makes the measurement honest.

## §2 Change A — the seat is not charged for starting up

`tools/sealbot/sealbot_shim.py` imports the extension, imports `game` and
constructs `MinimaxBot` before writing `sealbot_shim: ready` to **stderr**;
`tools/sealbot/matchserver/src/sealbot_client.rs` `new_game` spawns the process
and returns without reading anything, and `LineProcess::spawn` redirects stderr
to a FILE. So the readiness signal is not on any pipe the client reads, and the
whole start-up prefix lands inside the first measured answer of every game.

**The seat that does this right is in the same crate**: `PistolClient` completes
its handshake inside `new_game`, outside the measurement.

### The one real decision, and the options

| | how the client learns the shim is ready | verdict |
|---|---|---|
| **A1** | the shim writes `sealbot_shim: ready` to **stdout**; `new_game` reads exactly that line before returning | **SELECTED** — it is `PistolClient`'s own shape, it is synchronous, and it needs no new file, poll or timeout policy |
| A2 | the client polls the stderr file until the line appears | rejected: a poll needs an interval and a timeout, and a file written by a redirect has no flush guarantee the reader can rely on |
| A3 | `new_game` sends a throwaway request and discards the reply | rejected: it costs a real search per game — **ESTIMATED** 300 ms × 100 games — to measure nothing |
| A4 | keep one shim process for the whole match | **rejected on correctness**: sealbot's TT is never cleared (`bot.h:33,149`), so one process across paired games would leak search results between colours, and the per-game respawn is what prevents it — confirmed by the item-S confirmation, which found exactly one `sealbot_shim: ready` in each of 100 stderr files |

A1 moves the line from stderr to stdout, so the reply grammar gains a preamble
line. That is the same shape `PistolClient` already reads and the contract is
documented in `sealbot_client.rs`'s module doc, which this change amends.

## §3 Change B — the seat reports its own search time

The shim times `bot.get_move(game)` and returns `engine_time_ms` beside `moves`;
the client requires it and fills `EngineReply::engine_time_ms`.

**REQUIRED, NOT OPTIONAL** (hard rule 3). A shim that stopped reporting it would
otherwise put the seat back in exactly today's state — a null column nobody
notices — so its absence is a named `Protocol` failure. Both producers this
repository owns (`sealbot_shim.py` and `tools/sealbot/tests/stub_sealbot.py`)
send it.

**What it buys, and it is the whole point**: the transcripts stop carrying a
bound and start carrying a measurement, so "equal measured movetime per side" —
which D-695 pins and `sealbot_anchor_v7_protocol.md` §A1 registers — becomes a
statement a run can check instead of a statement about two config keys.

## §4 The criterion, registered before the change is made

D-699's third resume item. **The precondition is discharged when, on a run of at
least 20 games:**

1. sealbot's **first-of-game** wall-excess distribution is indistinguishable from
   its **later-answer** distribution — concretely, first-of-game median excess
   ≤ later-answer median excess + **2 ms**, against the v5 gap of 15 ms against
   0 ms and the isolated spawn charge of **26 ms**; and
2. `engine_time_ms` is non-null for **every** sealbot answer; and
3. per answer, `wall_ms − engine_time_ms` ≤ **5 ms**, which is 25x the 0.2 ms
   measured above and still 20x below the 106 ms this is fixing.

**THE DEFECT CLASS EACH EXCLUDES.** (1) excludes the start-up charge surviving
the handshake — a defect that would leave the first answer of each game inflated
and is invisible in any aggregate. (2) excludes a shim that silently drops the
field. (3) excludes a handshake that returns before the process is actually warm,
which would move the charge from the first answer into the second rather than
removing it, and which neither (1) nor (2) would catch.

**The instrument is `tools/anchor_overshoot.py`**, which already prints the
first-of-game and later-answer split and the `engine_time_ms` reported count, at
the revision this design ships in. It needs one addition for (3), the
wall-minus-engine-time column, and that addition carries its own test.

## §5 What this does NOT do

It does not select an option in `matrix_anchor_v7_budget.md`; that matrix has
fallen twice and a revision 3 selecting the surviving option (O7) must cite and
amend D-695, which is a separate act. It does not run an anchor. It does not
touch pistol. **It removes the reason the budget decision could not be settled on
measured ground, and stops there.**
