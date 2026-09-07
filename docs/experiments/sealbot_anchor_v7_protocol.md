# Sealbot anchor v7 — protocol. Written, not run.

> **THE BUDGET DECISION THIS PROTOCOL DEPENDS ON IS NOT SETTLED (D-699).** Its
> OPTION MATRIX — `matrix_anchor_v7_budget.md` — has had two DECISION-RED-TEAM
> rounds and both recommendations fell. §A1's "v7 PINS EQUAL MEASURED MOVETIME
> PER SIDE" is therefore a REGISTERED INTENTION AND NOT A SELECTED OPTION, and it
> is in unresolved conflict with the series budgets that D-697 records. **This
> protocol may not be run until that matrix survives its red team.** Everything
> else in it — the cap ruling (§A3, D-698), the overshoot terms (§A2), the
> opponent pin (§A5) and the openings (§A6) — is settled and stands.

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

**v7 REGISTERS IT AS A MULTIPLE OF EACH SIDE'S OWN BUDGET RATHER THAN INHERITING
120/5.** Both seats get `turn_timeout_seconds` = **16x that seat's budget**,
which is 8.0 s against a 500 ms budget and 4.8 s against a 300 ms one. D-534
explains pistol's 120 s — *"its wall cap was raised from a registered timing
probe precisely so the overshoot would be RECORDED rather than converted into a
loss"* — and that is a good reason for a generous threshold on BOTH seats, and no
reason at all for the ratio between them. 16x is generous against a MEASURED
worst case of 106 ms of excess (§A2) and still bounded, so a genuinely hung
engine is caught rather than waited on for two minutes. **v5 and v6 recorded 0
forfeits at 120/5, so nothing in the series turns on this**; it is registered so
that v7 does not carry an asymmetry it has just spent a section naming.

**THE RE-LABELLING IS DONE, NOT OWED** (D-697): `anchor_v5_finding.md` and
`anchor_v6_seatswap_finding.md` carry the label in their own headers, and the
three downstream records that quoted the numbers unqualified —
`opt_arc_CLOSURE.md`, `opt_arc_ledger.md`, `opt_arc_perf_finding.md` — carry it
beside the number. D-608 and D-612 are append-only and are amended by D-697
naming them, which is the log's own mechanism.

**AND THE FINDING IS NARROWER THAN A FIRST READING OF IT.** The inequality was
REGISTERED all along — `sealbot_anchor_v2_prereg.md:29,31` and
`sealbot_anchor_v3_prereg.md:36,38` put both budgets in adjacent rows of the same
table, and v6's own text states both. Nothing was concealed; what never happened
is that the inequality reached the SENTENCE the results are quoted in.

**WHY v6's SEAT-SWAP CONCLUSION STANDS, and the reason is not the one an earlier
revision gave.** It is not that the handicap is equal — it is not equal anywhere
in v5 or v6. It is that the handicap TRAVELS WITH THE ENGINE and is therefore
INVARIANT across the slot exchange, which is exactly what makes D-612's
slot-versus-colour test survive it. **The exemption is drawn at that conclusion
by name**, and not at "either result": v6's other claim — that both engines are
worse as p1, and the 38/62 colour split — is a statement about play at these
budgets and is labelled with them like any other.

**WHAT v7 DOES ABOUT THE INEQUALITY IS NOT SETTLED HERE, AND THE MATRIX THAT
WOULD SETTLE IT HAS FALLEN TWICE (D-699).** An earlier revision of this section
registered "v7 PINS EQUAL MEASURED MOVETIME PER SIDE" as though it were decided.
It is not. Two things a resume inherits rather than re-derives:

- **Equal WALL is not equal SEARCH**, and the gap is not small in the way it
  looked: the harness cannot report sealbot's own search time at all
  (`engine_time_ms` null for **1050 of 1050** and **986 of 986** answers), so an
  early-returning answer's non-search time is bounded only by its whole wall.
  Consuming the shim's `ready` line takes the bound from **4.50 % to 3.53 %**,
  not to zero.
- **D-695 pins "equal measured movetime per side" and the series budgets are
  unequal.** One of those has to move, by an ADR line that names the other.
  Neither has moved here.

## §A2 — The overshoot terms, MEASURED — and there are three, not two

**A REVISION OF THIS SECTION CLAIMED TWO TERMS AND WAS WRONG. The measurement is
below and it is the section's whole content**, derived from the 200 per-game
transcripts of v5 and v6 by a script written for this protocol — not from
`report.txt`, which publishes only slot A (see the gap named at the end):

| per-answer wall minus the configured budget | v5 | v6 |
|---|---|---|
| pistol: answers over budget / max excess | 87 of 1040 / **15 ms** | 50 of 978 / **14 ms** |
| sealbot: answers over budget / max excess | 406 of 1050 / **106 ms** | 300 of 986 / **15 ms** |
| sealbot, FIRST answer of each game | median **15 ms**, max **106 ms** | median 6 ms, max 15 ms |
| sealbot, every later answer | median 0 ms, max **11 ms** | median 0 ms, max 4 ms |

**THE OVERSHOOT IS CONCENTRATED IN THE FIRST ANSWER OF EACH GAME, AND WHAT
VARIES IT IS NOT LOAD BUT WHICH SEAT MOVES FIRST.** Split by whether sealbot's
first answer of a game comes straight after its process is spawned or after
pistol has burned ~500 ms — **MEASURED**, and within a single run, so load is
held:

| sealbot's first answer arrives | v5 median excess | v6 median excess |
|---|---|---|
| immediately after the spawn (n=50) | **42 ms** | 13 ms |
| after pistol's answer (n=50) | **1 ms** | 0 ms |

**A 40x split inside one run at one load**, and the run-to-run difference is the
seat swap mixing those two modes in different proportions — v6 is v5's config
with the two engines' harness SLOTS exchanged, not a byte-identical rerun. The
reported medians of 15 ms and 6 ms are medians over a 50/50 mixture and describe
neither mode. The third term, and it is the largest:

**(1) Process start-up and the Python replay, charged to sealbot and not to
pistol.** `tools/sealbot/matchserver/src/sealbot_client.rs:112-121` respawns the
shim per game and returns WITHOUT consuming its `sealbot_shim: ready` line, so
interpreter start-up, the extension import and the bot's construction all land
inside the first measured answer; `tools/sealbot/sealbot_shim.py` then rebuilds
the game in Python on every request. `tools/sealbot/matchserver/src/pistol_client.rs`
completes its handshake inside `new_game`, outside the measurement. **This is a
one-line asymmetry in a file this repository owns**, and closing it — consuming
the `ready` line before the clock starts — is registered as OWED below.

**(2) Sealbot's own untimed setup and abort granularity**
(`docs/audit/sealbot_study_2026-09.md` §4, CODE-derived at `c94749c`): the
deadline is `now + time_limit` set **after** the untimed setup, `_check_time`
fires every 1024 nodes, and the catch restores five flat arrays by `memcpy` —
"≤ 1024 nodes of search past the deadline + the untimed setup + the rollback".

**(3) Pistol's own**, gates off: **8 ms** maximum overshoot at a 500 ms budget —
**D-519**, seat 1 of anchor v2, 320 answers, median 500 ms, max 508 ms. *(An
earlier revision credited this to D-520, which is the SOLVER-ON seat and its
725 ms median overshoot; D-608 makes the same substitution.)* **It is not this
series' figure and is not substituted for one**: v5 and v6 measured pistol at
15 ms and 14 ms in their own runs, and D-479 binds a measured number to the run
that produced it.

**EQUAL WALL IS NOT EQUAL SEARCH, and the protocol says so on its own face.**
Sealbot's measured wall carries terms (1) and (2); pistol's carries (3) and about
a millisecond of IPC. Nothing separates sealbot's search from its overhead:
`engine_time_ms` is **null for all 2036 sealbot answers** across v5 and v6, and
non-null for **all 2018 pistol answers** — MEASURED. Any equalisation of the
wall therefore hands pistol strictly more SEARCH time, by an amount this harness
cannot currently report.

**THE INSTRUMENT, NAMED WITH ITS REVISION** (`docs/process.md`, "Instrument
governing revision"): `tools/anchor_overshoot.py`, which reads a run's
`g*.jsonl` transcripts and prints, per engine, `n`, median, p95, max, the count
over budget, the max excess, and that excess split into the first answer of each
game and every later one. It is pinned by the revision the v7 run is taken at,
and a change to it reopens this review. It produced the table above; that run is
its DRY RUN, on runs of the same kind as v7 and not on v7 itself.

**THE SECOND INSTRUMENT** (`docs/process.md`, "Cost, replication, and the second
instrument"), which v2 and v3 both registered and an earlier revision of this
document dropped: `tools/sealbot/matchserver/src/bin/replay_check.rs`, run over
v7's transcripts. **Registered agreement criterion**: it replays every game to
its recorded outcome and exits 0. **Registered consequence of disagreement**: the
run is VOID and reported as void — not re-interpreted, and not reported with the
disagreement as a caveat. It does not share the stage under doubt, which is the
CLOCK: it re-drives recorded moves and checks outcomes, so it is blind to timing
and cannot corroborate the overshoot table. **It corroborates the GAMES, not the
budget**, and this document says so rather than letting a green replay read as a
fair-clock certificate.

**THE REPORT'S OWN GAP, and it is why the transcripts are the instrument.**
`tools/sealbot/matchserver/src/report.rs:101` declares `a_answer_wall_ms` and
there is no `b_answer_wall_ms`, so `report.txt` publishes slot A's distribution
alone. That is why v5 published pistol's and v6 published sealbot's, and why no
single run has ever reported both. The transcripts
(`tools/sealbot/matchserver/src/transcript.rs:37-45`) carry `wall_ms` for every
turn of both engines, which is enough; `engine_time_ms` they carry for the pistol
seat only.

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

**NEITHER ENGINE IS TOLD THE CAP.** `git grep -n turn_cap -- tools/sealbot/`
at `235b6db` — the whole subtree, tests included, not just `src/` — returns
`matchserver/src/{config,main,openings,referee,report}.rs` and
`tools/sealbot/tests/run_tests.sh`, and **neither `pistol_client.rs` nor
`sealbot_client.rs`**. The cap is the referee's alone. So there is no cap
asymmetry to state, both engines search cap-blind, and v7 reports capped games
as their own cell because that is what the harness already does, not because
one engine knows something the other does not.

**D-698 AMENDS D-695 FOR THIS.** D-695 registers that this protocol pins "the
cap asymmetry"; there is none to pin, so the ADR moves to meet the code rather
than leaving the log saying the protocol does something it declines to do
(CLAUDE.md hard rule 10).

**This corrects the deep dive's standing entry** rather than citing it as
UNVERIFIED: `docs/research/sealbot_deep_dive.md:1365` describes the TT key as
carrying "stones-left", which is right, and `:545` records a `deadline = now +
time_limit * moves_left_in_turn` allowance that `current/` does not have
(§4's row is the one that governs). Neither entry claimed a cap term; this
section records that nothing in `current/` supplies one.

## §A4 — Hardware and execution: one process at a time, sequential, single thread

- **Games are sequential by construction.**
  `tools/sealbot/matchserver/src/main.rs:104-132` is a plain `for game in …`
  loop calling `run_game` and writing the transcript before the next game, and
  `referee.rs` asks one engine at a time inside it. There is no worker count in
  `MatchConfig` (`config.rs:21-38`) to set wrong.
- **AND THE MATCHSERVER IS NOT SINGLE-THREADED, WHICH AN EARLIER REVISION SAID
  IT WAS ON EVIDENCE THAT COULD NOT ESTABLISH IT.** That revision cited the
  crate's dependency list — but `std::thread` needs no dependency, and
  `git grep -nE "std::thread|thread::spawn" -- tools/sealbot/matchserver/src/`
  returns `client.rs:10,113,129`: `LineProcess::spawn` starts two reader threads
  per engine process, so at least five threads are live while either engine
  searches, in the very module that produces `wall_ms`. The conclusion the
  evidence was offered for survives — those threads block on I/O and the game
  loop is sequential — but the argument is the loop, not the manifest. **This is
  `docs/process.md`'s own named class, a claim checked against the wrong
  population, committed inside a document whose §A1 is a finding about an
  unchecked reading.**
- **v7 registers nothing about the machine except that no second engine process
  runs beside the match.** `ps` is quoted BEFORE and AFTER the run, with a load
  average sampled at each: a check taken only at the start bounds nothing over
  the ~10 minutes the run takes, and under a wall-clock budget background load
  does not change the wall — it changes the nodes each engine gets inside it,
  which is the quantity being compared.
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

**Build flags, quoted** from `current/setup.py:12-15`:

```python
Pybind11Extension("minimax_cpp", ["minimax_bot.cpp"],
                  cxx_std=17,
                  extra_compile_args=["-O3", "-march=native", "-DNDEBUG"],
```

**BUT `git rev-parse` DOES NOT REACH THE ARTEFACT THAT PLAYS, and the pin is
not discharged by it.** `-march=native` makes the binary a fact about this
workstation, and the compiled extension is not in git: `current/*.so` is
gitignored, so a stale build from before the pin, a rebuild under a different
compiler, and a rebuild on different hardware all pass a `git rev-parse` check
identically. The interpreter is unpinned too — the config's command is
`["python3", …]`, which on this machine resolves through a moving `mise`
symlink, and the shim's Python replay is inside the measured wall (§A2 term 1).

**SO THE RUN PINS THE ARTEFACT, NOT ONLY THE SOURCE**: the v7 log records the
`.so`'s `sha256`, the output of `python3 -VV`, and the compiler version, and the
extension is REBUILT FROM THE PINNED SOURCE as a step of the run rather than
trusted from an mtime.

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

**THE EMPTY BOARD NEVER ARISES UNDER THIS OPENING POLICY, so there is nothing
for memorisation to reach.** `book_v1` seeds five stones — v5's own transcripts
record `"opening":"server: 5-stone book opening 0,0 -4,3 -1,-1 0,-4 1,3"` and the
first engine answer at turn 4 — so neither engine is ever asked from an empty
board in a book run. An earlier revision spent this paragraph on a mechanism its
own `[openings] kind = "book"` forecloses, and cited **SB-26, which is
UNVERIFIED** (`docs/research/sealbot_deep_dive.md:449`) to do it.

**What the pairing does buy is narrower and is stated at its width**: each engine
plays each colour equally often, so an opening's own first-player advantage
cancels in the AGGREGATE. It does not neutralise a colour-specific asymmetry in
either engine — and §A7's registered output reports W/L **by colour**, which is
exactly where such an asymmetry would show.

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
