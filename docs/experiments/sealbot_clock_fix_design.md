# Design — make the sealbot seat's measured wall mean its search time. Revision 2.

**Revision 1 is `sealbot_clock_fix_design_rev1_SUPERSEDED.md`** and its
REVIEW-design returned **FAIL**, 7 MAJOR and 10 MINOR. Where a finding changed
this document, it is named at the place it changed it.

**The precondition D-699 names.** Two changes to files this repository owns, in
`tools/sealbot/matchserver/` and `tools/sealbot/sealbot_shim.py`. **No change to
sealbot, to any engine, or to pistol.** No match is run by this package.

**Revision** `8138c3e`. Numbers are **MEASURED** or **ESTIMATED** (D-291).

---

## §1 The before-state: the gap is a BOUND, and the bound is loose

D-699 records the gap as 4.50 % of sealbot's wall in v5 and 3.50 % in v6,
because `engine_time_ms` is null for **1050 of 1050** and **986 of 986** sealbot
answers, so an answer's non-search time can only be bounded by its whole wall.

**MEASURED**, with a copy of the shim instrumented to time `bot.get_move`
separately, driven at a 300 ms limit on real positions, warm process:

| case | wall | `get_move` | Python replay | **non-search** |
|---|---|---|---|---|
| 5 stones, budget-consuming | 300.49 ms | 300.30 | 0.10 | **0.19 ms** |
| 9 stones, budget-consuming | 300.42 ms | 300.20 | 0.10 | **0.22 ms** |
| 21 stones, budget-consuming | 301.0 ms | 300.8 | 0.1 | **0.2 ms** |
| **win-in-one, EARLY RETURN** | **0.43 ms** | **0.30** | 0.00 | **0.13 ms** |
| **win-in-one, EARLY RETURN** | **0.12 ms** | **0.00** | 0.00 | **0.12 ms** |

**THE EARLY-RETURN ROWS ARE THE ONES THAT MATTER AND REVISION 1 DID NOT HAVE
THEM** (MAJOR/PREMISES). About 31 % of v5's sealbot answers return before the
deadline — **MEASURED, 321 of 1050 under 250 ms, and 196 of 1050 at 0 ms** — and
that is the branch the bound is loosest on, because it bounds such an answer's
non-search time by its entire wall. The reviewer re-took the measurement over
**51 genuinely early-returning real positions** and got a median non-search of
**0.147 ms** and a max of **0.305 ms**. The term does not depend on whether the
answer used its budget.

**HOW LOOSE, DERIVED RATHER THAN ASSERTED (MAJOR-5).** Revision 1 said "roughly
sixty times" by dividing an aggregate fraction-of-total-wall by a per-answer
fraction-of-a-300 ms-budget — different denominators, and it credited the fix
with removing the start-up charge that the same section calls real. Against a
model of true non-search of `0.2 ms x answers + 26 ms x games`:

| | v5 | v6 |
|---|---|---|
| bound today | 10 226 ms (4.50 %) | 7 230 ms (3.50 %) |
| model of true non-search | 2 810 ms (1.24 %) | 2 797 ms (1.36 %) |
| **bound is loose by** | **3.6x** | **2.6x** |
| residual bound after Change A | 8 018 ms (3.53 %) | 6 586 ms (3.19 %) |
| model after Change A | 210 ms (0.09 %) | 197 ms (0.10 %) |
| **residual bound is loose by** | **38x** | **33x** |

**So the honest statement is two sentences, not one.** The start-up charge —
0.97 pp in v5, 0.31 pp in v6 — is REAL and Change A removes it. What remains
after that is a bound loose by 33–38x, and only Change B can collapse it to a
measurement.

**TWO CORRECTIONS TO REVISION 1's OWN READING.** The non-search term **does**
grow with the stone count — MEASURED by the reviewer, median 0.177 ms at 0–19
stones rising to **0.287 ms** at 60–79, where revision 1's table stopped at 21
stones and this harness's games reach **110** (MINOR-1). And the 0.2 ms was
measured through a **Python driver standing in for `SealbotClient`**, so the
Rust-side `serde_json` encode of up to 110 coordinate pairs, the pipe write, the
reader-thread channel hop and the reply decode are **not in it** (MINOR-2). The
in-harness referent is the pistol seat, which reports engine time on these same
runs: `wall_ms - engine_time_ms` is **median 1 ms, p95 5 ms, max 15 ms**. That,
and not 0.2 ms, is what a sealbot seat should be held to.

## §2 Change A — the seat is not charged for starting up

`tools/sealbot/sealbot_shim.py` imports the extension, imports `game` and
constructs `MinimaxBot` before writing `sealbot_shim: ready` to **stderr**;
`SealbotClient::new_game` spawns and returns without reading anything, and
`LineProcess::spawn` redirects stderr to a file. **MEASURED**: 25.8 ms spawn to
`ready`, of which 13.4 ms is interpreter start before `main()` runs — so the
whole prefix lands inside the first measured answer of every game.

### The decision, and the options

| | how the client learns the shim is ready | verdict |
|---|---|---|
| **A1** | the shim writes `sealbot_shim: ready` to **stdout as well as stderr**; `new_game` reads exactly that line before returning | **SELECTED** |
| A2 | poll the stderr file until the line appears | rejected: a file written through a redirect gives the reader no flush guarantee |
| A3 | `new_game` sends a throwaway SEARCH request and discards the reply | rejected: **ESTIMATED** 300 ms x 100 games to measure nothing |
| A5 | `new_game` sends a `ping` request and reads a `ping` reply | rejected, narrowly: it preserves the one-reply-per-request invariant, which A1 breaks with a preamble — but it adds a request KIND to the contract for a guarantee A1 already gives, since the shim cannot print `ready` before the bot is constructed |
| A4 | keep one shim process for the whole match | **rejected on correctness**: `_tt` is sized in the constructor (`current/engine/bot.h:33`) and **nothing in `current/` ever clears it** — only `_history` and the killers are reset, at `current/engine/search.h:46` — so one process across paired games would leak search results between colours. The per-game respawn is what prevents it. |

**A1 WRITES TO BOTH STREAMS, WHICH REVISION 1 DID NOT SAY (MINOR-6).** The 100
`*_engine_b.stderr` files of a run today contain the `ready` line **and nothing
else**, and the item-S confirmation used exactly that to prove one shim process
per game. Moving the line would empty those files and destroy the artifact; it
costs nothing to write it to both.

**A1 NEEDS A TIMEOUT AND REVISION 1 CLAIMED IT DID NOT (MAJOR-6).**
`LineProcess::read_line` takes a deadline and has no timeout-free form, so the
choice is load-bearing under hard rule 1. **A1 reuses `turn_timeout_seconds`,
exactly as `pistol_client.rs:97` already does for its own handshake** — no new
literal, no new config key. A2's rejection is therefore narrowed to the flush
ground alone, since half of revision 1's objection to it applied to A1 too.

**SCOPE, AND REVISION 1 OMITTED IT (MAJOR-4).**
`tools/sealbot/tests/stub_sealbot.py` is the sealbot seat in **every** matchserver
test — `run_tests.sh` builds 19 configs from that template — and it writes
nothing until it receives a request. Under A1 unamended, `new_game` would block
on a line that never comes until `turn_timeout_seconds`, and every one of those
19 configs would take a pregame forfeit. **The stub emits the same preamble.**

## §3 Change B — the seat reports its own elapsed time

The shim times `bot.get_move(game)` and returns `engine_time_ms` beside `moves`;
`SealbotClient` requires it and fills `EngineReply::engine_time_ms`, which is
already plumbed to the transcript (`referee.rs:291`, `transcript.rs:44`).

**REQUIRED, NOT OPTIONAL** (hard rule 3): a shim that stopped reporting it would
put the seat back in today's state, a null column nobody notices, so its absence
is a named `Protocol` failure. Both producers this repository owns send it.

**THE WIRE TYPE IS PINNED (MINOR-7)**: a JSON **integer** number of
milliseconds. `EngineReply::engine_time_ms` is `Option<u64>` and a float would
make `as_u64()` return `None`, firing the "required" refusal on every answer —
loud, but for the wrong reason.

**AND IT IS NOT "PURE SEARCH", WHICH REVISION 1 IMPLIED (MINOR-9).**
`bot.get_move` carries protocol §A2's **term (2)** — sealbot's untimed setup, its
≤1024-node overrun past the deadline and the `memcpy` rollback, MEASURED here as
the 0.3–1.4 ms by which `get_move` exceeds its own limit. So the field is **the
engine's own elapsed time for the answer**, and the design says so rather than
calling it search time.

## §4 The criteria, rebuilt — revision 1's did not work

`docs/process.md`: *"A criterion that is a property the named defect class
PRESERVES … passes vacuously and is not a criterion."* Revision 1's three did
exactly that, and the rebuild is verified against the defective data itself.

**WHY (1) HAD TO BE REPLACED, MEASURED (MAJOR-1).** `main.rs:105` sets
`a_is_p1 = game % 2 == 1`, so exactly half of each run's first-of-game answers
follow sealbot's own spawn and half follow pistol's ~500 ms answer, which absorbs
it. The pooled median describes neither mode. Applied to **v6's own pre-fix
transcripts, where the defect is 100 % present**, revision 1's criterion (1)
**PASSES at every odd game count** — 21, 23, 25 … 39 — and fails only at even
ones. Ten of the twenty-one admissible counts declared the precondition
discharged with the defect intact.

The precondition is discharged when, on a run of **at least 20 games**:

1. **On the ASKED-FIRST sub-population** — the games in which sealbot answers
   before pistol has answered at all — sealbot's first-of-game median wall excess
   is within **2 ms** of its later-answer median.
   **POWER, MEASURED against the pre-fix data**: this fails at **every** game
   count from 20 to 60, in **both** v5 and v6. Pre-fix the asked-first median
   excess is **42 ms (v5)** and **13 ms (v6)** against later-answer medians of
   ~0 ms; post-`ready` the residual is **MEASURED at ~1.1 ms**, so 2 ms is about
   twice the measured residual and an order of magnitude below the defect.
2. `engine_time_ms` is a **non-null integer for every sealbot answer**.
3. **Two-sided**, per side: `0 <= wall_ms - engine_time_ms`, with **p95 <= 5 ms
   and max <= 25 ms**. **DERIVED from the in-harness referent**, not from 25x a
   Python-driver number: the working pistol seat on these same runs is median 1,
   p95 5–8, max 15, with **~5 % of answers over 5 ms** — so revision 1's
   "per answer <= 5 ms" was failed by the seat that already works (MAJOR-2).
4. **THE EXTERNAL REFERENT, and revision 1 had none (MAJOR-3).** The fraction of
   sealbot answers with `engine_time_ms < 250` must be within **5 percentage
   points** of the fraction with `wall_ms < 250` — **MEASURED at 30.6 % (v5) and
   32.3 % (v6)**. `wall_ms` is taken by the Rust client and does not share the
   shim's timer, so this is a value computed by something that does not share the
   suspect input.

**THE DEFECT CLASS EACH EXCLUDES.** (1) the start-up charge surviving the
handshake — invisible in any aggregate. (2) a shim silently dropping the field.
(3) a handshake that returns before the process is warm, moving the charge into
the second answer rather than removing it. **(4) A SHIM THAT REPORTS ITS
CONFIGURED BUDGET INSTEAD OF ITS ELAPSED TIME** — a one-token slip,
`int(bot.time_limit * 1000)` — which passes (2) and passes (3) trivially on the
**18.7 % of v5 answers whose wall is 0 ms**, and would launder the configured
500/300 ratio as a measured one. That is the single thing D-699's surviving
option exists to prevent, and no criterion in revision 1 could see it.

## §5 What this delivers, and what it does not

**MEASURED, not claimed (MAJOR-7).** `report.rs` carries no `engine_time_ms` at
all — only `a_answer_wall_ms`, `a_wall_ms_total` and `b_wall_ms_total` — so after
these two changes the transcripts carry the field and **no summary reports it**.
Revision 1 said this fix "removes the reason the budget decision could not be
settled" and that overstates: of D-699's three resume items it delivers **two**
(the shim change, and a criterion), and explicitly defers the third, the ADR
amending D-695.

**THE INSTRUMENT ADDITION IS THEREFORE THREE THINGS, NOT ONE**:
`tools/anchor_overshoot.py` gains (a) the asked-first / later split criterion (1)
needs, (b) a `wall_ms - engine_time_ms` column with the p95 and max criterion (3)
needs, and (c) **per-engine `engine_time_ms` totals and their ratio**, which is
the headline number D-699's O7 asks for and which nothing currently computes.
Each carries a test, and criterion (2)'s reported-count column must be driven by
a fixture with a **non-null** value — every fixture in
`anchor_overshoot_tests.rs` writes `null` today, so that branch has never been
exercised (MINOR-8, `docs/process.md`'s tools/ coverage rule).

**ADR DEBT, OWED AND NAMED (MINOR-10).** §2's option matrix is settled by the
REVIEW-design that attacked this document; the ADR line recording it carries the
strongest surviving attack. It is written when the implementation lands, not
before.

**This does not select an option in `matrix_anchor_v7_budget.md`**, does not run
an anchor, and does not touch pistol.
