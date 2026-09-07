# Sealbot anchor v7 — protocol. Written, not run.

**No run date. No verdict. Direction only.** This document is a
pre-registration for the NEXT anchor in the v3–v6 series, and it is landed
before the run it governs so the run cannot be shaped by its own numbers
(D-483). Its trigger is stated in §A7 and is nothing sooner.

**Governing revision of this document**: `235b6db`, the revision every file
and line quoted below was read at. A citation re-quoted at a later revision
carries that revision beside it (D-692). Nothing here is measured; every
number below is a CONFIGURED value read out of a committed file, or a
MEASURED value cited from the artifact that produced it, and each is labelled.

**What an anchor is.** Not a strength claim. sealbot is UNVERIFIED (D-197) and
no standing judgment moves on an anchor result. What the series buys is a
direction against one fixed external opponent at one budget, run the same way
each time, so that a change in pistol shows up as a change in the series.

---

## §A1 — The v5 and v6 per-side budgets, verbatim. **FINDING: they are unequal.**

`local/sealbot_anchor_v5_seat1.toml` and `local/sealbot_anchor_v6_seatswap.toml`
are untracked local files; quoted here at their state on 2026-09-07, which is
the state `anchor_v5_finding.md` and `anchor_v6_seatswap_finding.md` cite.

```toml
# v5, [engine_a] kind = "pistol"          # v6, [engine_b] kind = "pistol"
movetime_ms = 500
turn_timeout_seconds = 120.0

# v5, [engine_b] kind = "sealbot"         # v6, [engine_a] kind = "sealbot"
time_limit_seconds = 0.3
turn_timeout_seconds = 5.0
```

**THE TWO SEATS DID NOT RUN AT THE SAME BUDGET IN EITHER RUN.** pistol was
given **500 ms** per turn and sealbot **300 ms** — a 1.67× advantage to pistol
on the clock — and v6's own header comment calls the two files' budgets "the
same per-engine budgets", which is true of v6 against v5 and says nothing
about the two engines against each other. The runs' own receipts show it and
neither finding document reads it as an asymmetry: v5 reports pistol at
"median 500 ms, max 515 ms" and v6 reports sealbot at "median 300 ms and max
315 ms against its 0.3 s limit".

**A SECOND ASYMMETRY, in the forfeit threshold**: `turn_timeout_seconds` is
**120.0** for pistol and **5.0** for sealbot. That is not a search budget — it
is the wall at which the harness declares a forfeit — but it is unequal by a
factor of 24, and it interacts with §A2's overshoot: a pistol answer that ran
long is recorded, where a sealbot answer that ran long is a loss.

**THE RE-LABELLING THIS FINDING OWES.** `anchor_v5_finding.md` and
`anchor_v6_seatswap_finding.md`, and every later citation of the v5/v6 numbers,
are **"unequal movetime (pistol 500 ms / sealbot 300 ms)"** anchors. The
finding does not overturn either result — v6's seat-swap conclusion is about
which COLOUR an asymmetry follows, and an equal handicap on both slots cannot
produce that — but "pistol won 40 % of decided games" is a sentence about a
run in which pistol had 1.67× the clock, and it may not be quoted without the
qualifier.

**v7 PINS EQUAL MEASURED MOVETIME PER SIDE.** Equal CONFIGURED movetime is not
enough and is not what is registered: §A2's two overshoot terms differ between
the engines, so equality is registered on what the log MEASURES, and the run's
receipt is the per-answer distribution of both sides.

## §A2 — The two overshoot terms, both named, both measured per move

**Sealbot's** (`docs/audit/sealbot_study_2026-09.md` §4, CODE-derived at
`current/` `c94749c`): the deadline is `now + time_limit` set **AFTER the
untimed setup**, `_check_time` fires every 1024 nodes and throws, and the catch
restores five flat arrays by `memcpy`. So its overshoot is

> "≤ 1024 nodes of search past the deadline + the untimed setup + the rollback"

and the untimed setup prefix is charged to the wall clock the server measures
but not to the budget the engine was given.

**Pistol's** (D-534, citing D-520's measurement): gates off, at a 500 ms budget,
**8 ms maximum overshoot** — MEASURED, and the figure v5's own 515 ms maximum
against 500 ms is consistent with. With `[solver] on_search_path = true` the
same instrument measured a **725 ms median overshoot**; **v7 runs the gates-off
seat**, and a solver-armed seat is a different protocol and not this one.

**HOW BOTH ARE MEASURED PER MOVE IN THE v7 LOG — and the gap that has to be
closed first.** `tools/sealbot/matchserver/src/referee.rs:58-68` records a
`TurnRecord` per turn carrying `wall_ms` (the server's own measurement) and
`engine_time_ms` (what the engine reported), and
`tools/sealbot/matchserver/src/transcript.rs:37-45` writes both into every
per-game transcript for **both** engines. **The summary report does not**:
`tools/sealbot/matchserver/src/report.rs:101` declares `a_answer_wall_ms` and
there is no `b_answer_wall_ms` — so `report.txt`'s per-answer distribution is
**slot A's alone**. That is why v5 reports pistol's distribution and v6
reports sealbot's: each was in slot A for its own run, and no single run has
ever reported both.

**v7's registered instrument for this is the TRANSCRIPTS, not the report.** The
per-answer distributions of both sides are derived from the per-game transcript
files, which already carry every turn of both engines. No harness change is
required and none is registered here. The v7 log states, for each side:
`n` answers, median, 95th percentile, maximum, and the maximum minus the
configured budget as **overshoot**, with the two terms above named beside them.

## §A3 — The turn cap, and **sealbot does not search it**

**The cap value** is the config's `turn_cap`; v5 and v6 both used **60** and v7
keeps 60 for series continuity. Game rule 6 makes a cap an evaluation horizon
and never a rule.

**How a capped game is scored**:
`tools/sealbot/matchserver/src/referee.rs:203-204` breaks the game loop with
`GameResult::Capped { turn: turn_cap }` the moment `state.turn() > turn_cap`,
and `tools/sealbot/matchserver/src/report.rs:5` states the disposition —

> "capped games and forfeits are reported separately and excluded from the"
> [decided denominator]

— with `capped` its own tally field (`report.rs:37`) and the Wilson interval
taken over `decided` only (`report.rs:204`). **Capped games are already a
separate cell and are already never folded into W/L.**

**AND THE ASYMMETRY THIS SECTION WAS WRITTEN TO RULE ON DOES NOT EXIST.** The
dispatch's ruling to transcribe is conditional — *"if sealbot searches the cap
as a game rule, the protocol states the asymmetry"* — and the condition is
FALSE, verified from source rather than left UNVERIFIED, because the source is
available at `/home/tom/Work/sealbot-scope/sealbot`, `git rev-parse HEAD` =
`c94749c21c16c3b072fff6da49762dd5f92f3986`:

- `current/types.h:21` — `int8_t moves_left;   // 1 or 2`
- `current/engine/board.h:90-93` — `_moves_left--; if (_moves_left <= 0) { … _moves_left = 2; }`
- `current/minimax_bot.cpp:32` — `gs.moves_left = game.attr("moves_left_in_turn").cast<int8_t>();`
- `current/engine/bot.h:216` — `^ (static_cast<uint64_t>(_moves_left) * 0x517cc1b727220a95ULL)`

**`moves_left` is the INTRA-TURN PHASE — which of the two stones of a turn is
owed — and not a count of turns left in the game.** It takes the values 1 and 2,
its own header comment says so, it is read from `moves_left_in_turn`, and it
enters the TT key for exactly the reason pistol's zobrist carries an intra-turn
phase. Sealbot has no cap term at all: its iterative deepening runs
`depth 1..max_depth = 200` and stops on a mate score
(`sealbot_study_2026-09.md` §4).

**NEITHER ENGINE IS TOLD THE CAP.** `git grep -n turn_cap --
tools/sealbot/matchserver/src/` at `235b6db` returns `config.rs`, `main.rs`,
`openings.rs`, `referee.rs` and `report.rs` — and **neither `pistol_client.rs`
nor `sealbot_client.rs`**. The cap is the referee's alone. So there is no cap
asymmetry to state, both engines search cap-blind, and v7 reports capped games
as their own cell because that is what the harness already does, not because
one engine knows something the other does not.

**This corrects the deep dive's standing entry** rather than citing it as
UNVERIFIED: `docs/research/sealbot_deep_dive.md:1365` describes the TT key as
carrying "stones-left", which is right, and `:545` records a `deadline = now +
time_limit * moves_left_in_turn` allowance that `current/` does not have
(§4's row is the one that governs). Neither entry claimed a cap term; this
section records that nothing in `current/` supplies one.

## §A4 — Hardware and execution: one process at a time, sequential, single thread

- **Sequential by construction, not by configuration.**
  `tools/sealbot/matchserver/src/main.rs:105-131` is a plain `for game in …`
  loop calling `run_game` and writing the transcript before the next game; the
  crate's `Cargo.toml` has no thread or task dependency (`pistol-core`, `serde`,
  `serde_json`, `toml`). There is no worker count in `MatchConfig`
  (`config.rs:21-38`) to set wrong. **v7 registers nothing here except that no
  second engine process runs beside the match**, which is an operator act and
  is checked with `ps` before the run and quoted in the log.
- **Single thread each.** pistol's instrument mode is single-threaded (hard
  rule 4); sealbot's `current/` has no threading on any search path.
- **The receipt.** Per side, in the v7 log: total nodes where the engine
  reports them (`report.rs:77` records `a_nodes_total` only for the pistol
  seat — sealbot reports no node count through the shim), total wall
  (`a_wall_ms_total`, `b_wall_ms_total`, `report.rs:78-79`), the per-answer
  distributions of §A2, and derived nps for the side that reports nodes.
  **nps is NOT comparable between the two engines** — pistol's node is one ply
  and sealbot's is one two-stone turn (`sealbot_study_2026-09.md` §5) — and the
  log says so on its own face rather than leaving a reader to divide.

## §A5 — The opponent, pinned

**sealbot master `current/` at `c94749c21c16c3b072fff6da49762dd5f92f3986`**
("Split engine.h into engine/ directory and move vendor header"), the same
revision `docs/audit/sealbot_study_2026-09.md` was written against.

**Build flags, quoted** from `current/setup.py:13-14`:

```python
Pybind11Extension("minimax_cpp", ["minimax_bot.cpp"],
                  cxx_std=17,
                  extra_compile_args=["-O3", "-march=native", "-DNDEBUG"],
```

`-march=native` makes the binary a fact about this workstation; the v7 log
records the machine.

**A STRONGER BRANCH IS A SECOND SERIES, NEVER A REPLACEMENT.** `best/` exists
in the same checkout and disagrees with `current/` on 7 of 10 positions at
depth 4 (`sealbot_study_2026-09.md` §5). Running v7 against `best/` produces a
number that is not comparable to v3–v6 and would silently break the one thing
the series is for. If `best/` is wanted it is anchor series **B**, numbered
separately, and D-695's flip clause is what fires.

## §A6 — Openings

`crates/pistol-cli/tests/fixtures/random_openings_v1.txt`, `take = 50`,
`skip = 0`, each opening played from **both** seats — 100 games — identical to
v5 and v6, for series continuity. D-568 licenses `book_v1` for anchors: it is
retired for SPRT, and an anchor makes no strength claim, so it cannot launder a
spent opening set into one.

**Distinct-n is reported** (v5 and v6 each returned 100 of 100 distinct games,
50 distinct openings). A run whose distinct-n falls below n reports both numbers
and the shortfall is a finding about the run, not a footnote.

**Empty-board memorisation is neutralised by the pairing** — every opening is
played from both seats, so an engine that has memorised a first move plays it
into both colours and the pair cancels it. This rests on **SB-26, which is
UNVERIFIED** (`docs/research/sealbot_deep_dive.md:449`, the champions'
configurations entry) and is cited as such: the neutralisation argument is
sound on its own terms, and the premise that sealbot has such memorisation at
all has never been verified here.

## §A7 — Verdict vocabulary: **NONE**

**This protocol registers no verdict.** There is no h0/h1, no bound, no
threshold and no criterion any number can fail. What v7 produces is a
DIRECTION — W/L/capped by colour and by seat, both per-answer wall
distributions, distinct-n, and the compute — reported beside v5's and v6's
so that a reader can see whether the series moved.

**No Elo. No human-strength claim. D-197 stands**: pistol is not at
strong-human strength, sealbot is a MILESTONE and not a human-strength proxy,
and sealbot is UNVERIFIED. A Wilson interval on decided games is reported
because the harness computes one, and it is an interval about ONE opponent at
ONE budget.

**TRIGGER TO RUN: the first Phase 2 `h1`, and nothing before.** Not a green CI,
not a landed work package, not a promising bench — an SPRT that has crossed
`h1`. Until then this document is inert. D-695 records the trigger; D-696
records that the depth diagnostics are read before any search lever is re-tested,
and this anchor is downstream of both.

**What v7 does NOT do**: it does not re-open v5's or v6's conclusions, it does
not measure the p1/p2 colour asymmetry v6 found (that wants an instrument which
is not an anchor against an unverified opponent, as v6's own closing section
says), and it proposes no remedy for anything it measures.

---

## What a v7 run owes on the day it is run

1. A fresh revision line: the run's own commit, the pistol binary's sha256, the
   engine config as committed, and sealbot's `c94749c` re-confirmed with
   `git rev-parse` in the sealbot checkout.
2. `ps` quoted, showing no other engine process.
3. The match config, with **measured** movetime equal per side per §A2 — and
   if the configured values had to differ to make the measured ones equal, the
   configured values are quoted too and the difference is the finding.
4. Both per-answer wall distributions, derived from the transcripts.
5. Distinct-n, capped count as its own cell, forfeits as their own cell.
6. The comparison table against v5 and v6, with both of those labelled
   **unequal movetime** per §A1.
