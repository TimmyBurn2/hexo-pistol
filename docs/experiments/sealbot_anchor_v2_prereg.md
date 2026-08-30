# Sealbot anchor v2 — pre-registration

**What this is**: TWO anchor matches on the local HeXO match platform
(`tools/sealbot/`), pistol against sealbot, at the **deployment budget** —
`go movetime`, not the node budget D-438's anchor ran. Its purpose is to record
where the current `dev` engine stands against the one external engine we have,
under the clock the design point names, with transcripts that replay.

**What this is NOT**: not SPRT, not paired, not an Elo claim, not a strength
claim of any kind beyond *this is what happened in these N games*. Sealbot is
**UNVERIFIED** (`docs/research/sealbot_notes.md`, D-197). The word is **anchor**
everywhere. D-22 is untouched: instrument mode still refuses a wall-clock budget
and every strength claim in this project still comes from instrument mode — this
document makes none.

**This is not a re-reading of D-438.** A budget change is a NEW anchor by that
line's own words (*"An opening-policy change, a budget change, or any engine
change is a NEW anchor, not a re-reading"*). D-438's numbers do not move (D-374).

---

## 1. The registered numbers — fixed before game one of either seat

| What | Value |
|---|---|
| Seats | **TWO**, run as two separate matches with separate output directories |
| Games per seat | **N = 40** (seats alternate: pistol is p1 in odd games, p2 in even — 20 per colour) |
| Opening policy | **The platform's standard setup, every game**: the server auto-plays p1's turn-1 stone at the origin, exactly as the HeXO server's htttx `setup` packet delivers; engines are first asked at turn 2 |
| pistol budget | **`go movetime 500`** — 500 ms, the deployment budget CLAUDE.md's design point states (*"a strong move consistently within 0.5 s"*) and `configs/play_v0.toml`'s own header restates (*"CLAUDE.md's design point is a 0.5 s move, which is a `movetime` budget"*) |
| pistol wall cap | **120 s** per answer on seat 1; **600 s** on seat 2. A hang bound, and on seat 2 it is set from a MEASUREMENT rather than an estimate — D-441 measured a single call at cap 16384 between 9 s and 240 s, so two capped root calls can exceed 120 s and a seat that exceeds its cap forfeits. §7.1 registers the branch |
| sealbot budget | **`time_limit = 0.3` s** per turn — its standing value (D-438 §1, `local/sealbot.example.toml`); wall cap 5 s per answer |
| Turn cap | **60** — the evaluation horizon (game rule 6); a game with no decision is `capped`, never a win |
| Seat 1 config | `configs/play_staged_v0.toml` **as committed** — play mode, staged policy, every gate off (`killers`/`history`/`countermove` false, `safety_net_top_k = 0`, `on_search_path = false`) |
| Seat 2 config | `configs/play_staged_solver_v0.toml` — **byte-identical to seat 1 but for `[solver] on_search_path = true`**, and added by this package |

**Seat 2's derivation, and the numbers it does NOT invent.** The dispatch admits
seat 2 *"ONLY IF a committed play config for it exists or can be derived from
registered values without inventing numbers"*. Every solver knob in
`configs/play_staged_v0.toml` is already committed at a registered value —
`per_call_node_cap = 16384`, `trigger = "any_open_four"`, `epsilon_num/den`,
`zone_orders`, `free_stone_radius`, `tt_entries`, `attacker_policy` — and seat 2
changes exactly one boolean. **The cap stays at the committed play value of
16384 and is NOT swapped for the bench seat's 2048**, which was derived for a
different document; taking a number from one config into another because it
looks apter is the substitution this clause forbids. A committed measurement
config with the gate ON is not new: `configs/gate_staged_solver_v0.toml` and
`configs/bench_wp18c_solver_on.toml` are both that, and D-441's *"gate OFF in
every committed config"* binds the DEPLOYMENT configs, which seat 2 is not.

**A consequence of that cap, registered before the run so it cannot be read as a
surprise afterwards**: a solver call absorbs its whole node count at once, and
the root's two calls are made BEFORE the first deepening iteration
(`crates/pistol-search/src/search.rs`'s `solver.solve(state, cap)` and
`solver.solve_defender(state, cap)`, before the deepening loop) where nothing is
abortable.
Seat 2 is therefore EXPECTED to overshoot 500 ms, possibly by seconds. **That
overshoot is a recorded observation of this anchor, not a failure of it** — it
is the D-95 / WP-1.4 forfeit-risk class measured under the deployment budget,
and the 120 s wall cap exists so it lands in the record instead of in a forfeit.

## 2. What counts as what

Unchanged from `docs/experiments/sealbot_anchor_prereg.md` §2, which owns these
definitions and is not restated here (D-423), with two additions this anchor
needs:

- **Distinct games**: the number of DISTINCT stone sequences among a seat's 40
  transcripts. D-438's anchor had **two**, because both engines were
  deterministic from the fixed opening; a `movetime` seat is not reproducible by
  construction (D-22), so distinct-game diversity is EXPECTED here and is
  reported as the honest denominator behind the interval's nominal N.
- **Overshoot**: per pistol answer, the wall time the record carries against the
  500 ms budget. Reported per seat as median and maximum. Not a verdict.

**BOTH ARE PRODUCED BY A NAMED INSTRUMENT AND NEITHER IS COMPUTED BY HAND.** A
first revision of this document registered them as outputs while the harness
produced neither, which a fresh-context REVIEW-impl found and which
`docs/process.md` forbids — a registered output names the artefact that produces
it, with its revision. `tools/sealbot/matchserver/src/report.rs` now computes
both: `distinct_games` folds each game's stones in submitted order into a set,
and `a_answer_wall_ms` collects engine A's per-answer wall times, from which the
report prints `answers`, `answer_wall_ms_median` and `answer_wall_ms_max` in
both its JSON and its text. The median is the lower middle on an even count, so
the number printed is a number that was measured. The stub suite asserts all
four fields are populated on a scripted match.

## 3. The instruments, at their revisions

| Instrument | Revision | Identity |
|---|---|---|
| The match platform (`tools/sealbot/`, whole tree) | REGISTERED SLOT — the commit this document is reviewed at | includes this package's `movetime` seat |
| `run_match.sh` | same commit | drives the match; builds with `--locked` |
| `replay_check` (second instrument) | same commit | replays transcripts against pistol-core |
| pistol binary | REGISTERED SLOT — built `--release --locked` from `dev` at the run revision | sha256 REGISTERED SLOT, recorded at launch; **a rebuild means a re-record** |
| Seat 1 run config | `local/sealbot_anchor_v2_seat1.toml` | sha256 REGISTERED SLOT |
| Seat 2 run config | `local/sealbot_anchor_v2_seat2.toml` | sha256 REGISTERED SLOT |
| sealbot | local tree, unversioned | recorded as the shim's argv in the config; **UNVERIFIED by design** |

A change to any of these before the run reopens this pre-registration, however
small the diff (`docs/process.md`, instrument governing revision).

## 4. The registered commands

Run from the repository root, **one seat at a time and alone on the machine** —
this is a wall-clock instrument, so a concurrent build or bench voids it
(`ps -eo cmd | /usr/bin/grep -c '[c]argo'` must read 0 before each seat):

```
tools/sealbot/run_match.sh local/sealbot_anchor_v2_seat1.toml
tools/sealbot/matchserver/target/release/replay_check artifacts/sealbot_anchor_v2_seat1
tools/sealbot/run_match.sh local/sealbot_anchor_v2_seat2.toml
tools/sealbot/matchserver/target/release/replay_check artifacts/sealbot_anchor_v2_seat2
```

Digests are printed by `run_match.sh` itself over the bytes each seat wrote.

## 5. The dry run — input of the same kind, and never the registered workload

**Input**: **FOUR** reduced-budget configs of the same kind — the same two real
engines, the same seats rule, at `movetime 100` (seats 1 and 2) or
`nodes 5000`, sealbot `0.05` s, turn cap 20, **2 games** each:

| config | budget | pistol config | what it exercises |
|---|---|---|---|
| `dryrun_seat1` | `movetime 100` | `configs/play_staged_v0.toml` | seat 1's plumbing end to end |
| `dryrun_seat2` | `movetime 100` | `configs/play_staged_solver_v0.toml` | seat 2's, and its overshoot |
| `dryrun_modepin_a` | `movetime 100` | `configs/instrument_staged_v0.toml` | criterion D, one way |
| `dryrun_modepin_b` | `nodes 5000` | `configs/play_staged_v0.toml` | criterion D, the other way |

A first revision named two configs and a criterion (D) needing four, which a
REVIEW-impl found. The two mode-pin configs are **expected to forfeit at the
handshake** — that is the criterion, not a failure of it.

**Criteria, each with the defect class it excludes:**

- **A. Both games of both seats run with zero forfeits**, and the report's
  winner and turn number for each match the transcript's own last record.
  *Defect class: a driver-protocol break wearing a game outcome — a seat that
  cannot speak `go movetime` loses by forfeit and the anchor would silently
  measure plumbing rather than play.*
- **B. `replay_check` exits 0 on every dry-run transcript.** *Defect class: the
  written record is not the game that was played.*
- **C. The report's `nodes_total` equals the sum of the per-turn `nodes` in the
  transcripts.* *Defect class: compute misattribution — per-side compute is a
  reporting requirement (CLAUDE.md rule 6).*
- **D. THE MODE PIN IS EXERCISED IN BOTH DIRECTIONS.** A `movetime` seat pointed
  at an INSTRUMENT-mode config is refused by name, and a `nodes` seat pointed at
  a PLAY-mode config is refused by name. *Defect class: a client that accepts
  any mode — the pin that made D-438 reproducible would be gone and nothing
  would say so.* This is the one criterion the v1 dry run could not have, because
  v1 had one budget kind.
- **E. The stub suite passes**, including all three tampered-record negative
  controls. *Defect class: an instrument that cannot say no.*
- **F. THE PER-ANSWER NODE COUNTS VARY, AND NONE OF THEM IS A NODE BUDGET.**
  *Defect class: a `movetime` seat that is really still node-budgeted — a config
  that names one budget while the client sends the other, which every other
  criterion here survives.* Under a wall-clock budget the nodes an answer spends
  are whatever the position and the clock allow and they vary by orders of
  magnitude; under `go nodes N` every answer lands at about `N`, which is the
  observable the defect cannot fake. **A first revision registered "the
  overshoot column is not zero and not constant" for this defect class and a
  REVIEW-impl struck it: `wall_ms` is `started.elapsed()`, so a seat that IS
  still node-budgeted produces a non-zero, non-constant column exactly like one
  that is not — the criterion is preserved by the defect it named, which
  `docs/process.md` says is not a criterion at all.** A second revision
  registered the seat's own stderr `go` line, which the STUB prints and the
  shipped binary does not; that is struck too, and this is what replaces both.
- **G. THE OVERSHOOT COLUMN IS POPULATED AND ITS MAXIMUM EXCEEDS THE BUDGET.**
  *Defect class: a wall column copied from the budget rather than measured, or
  absent.* This is what F used to be, kept for the defect class F does not
  reach, and stated as the weaker criterion it is.

**THE RECORD**, from the four runs' own bytes. Engine `target/release/pistol`
sha256 `bca86067db0d685d7fdf7f5028ff5f2108a27ce986fcfe727b0235323562d881`;
artifacts under `artifacts/sealbot_anchor_v2_dryrun_{seat1,seat2,modepin_a,modepin_b}/`.

| criterion | observed | verdict |
|---|---|---|
| A — both games of both seats, zero forfeits | seat 1: win p1 t17, win p1 t13 (pistol 1 W / 1 L); seat 2: win p2 t14, win p1 t15 (pistol 0 W / 2 L); **0 forfeits on either** | **MET** |
| B — `replay_check` exits 0 on every transcript | `replay-check: 2 transcript(s) replayed to their recorded outcomes` on all four runs | **MET** |
| C — `nodes_total` equals the transcript sum | seat 1: **502,769 == 502,769**; seat 2: **244,114 == 244,114** | **MET** |
| D — the mode pin fires from BOTH sides, on the REAL binary | a `movetime` seat at an instrument config: *"engine mode is instrument, not play: this seat's budget seats play mode"*; a `nodes` seat at a play config: *"engine mode is play, not instrument: this seat's budget seats instrument mode"* — each a forfeit at the handshake, which IS the criterion | **MET** |
| E — the stub suite passes, negative controls included | `sealbot-tests: PASS (all scripted matches matched their hand-derived outcomes)` | **MET** |
| F — per-answer node counts vary and none is a node budget | seat 1's fourteen answers span **3 … 66,645 nodes, all distinct**; a `go nodes N` seat would land every answer at about `N` | **MET** |
| G — the overshoot column is populated and its max exceeds the budget | seat 1: 14 answers, median **100 ms**, max **107 ms** at a 100 ms budget; seat 2: 13 answers, median **397 ms**, max **2262 ms** | **MET** |

**Two readings the dry run hands to the governed run, neither of which is a
verdict.** Seat 1's overshoot is **7 ms** at a 100 ms budget — the WP-1.4
movetime ceiling holding well inside its 50 ms epsilon. Seat 2's median answer
is **four times its budget** and its maximum **twenty-two times**, which is the
uninterruptible solver call measured at the deployment budget and is the one
thing this anchor can say that a node-budgeted one cannot.

## 6. The governed run's agreement criterion, and its consequence

`replay_check` over each seat's 40 transcripts must print
`40 transcript(s) replayed to their recorded outcomes` and exit 0.

**The stage under doubt is the RECORD, and the second instrument does not share
it.** The two instruments share pistol-core deliberately — the rules are not the
stage under doubt — and what only the re-read exercises is that the bytes on disk
are the game that was played.

**Registered consequence of disagreement**: that seat is NOT a measurement. Its
transcripts stand as raw material, its anchor numbers are withheld, and the
platform is fixed and the seat re-run under a NEW pre-registration on a fresh
output directory. No post-hoc repair of the record and no partial reading. **A
disagreement on one seat does not void the other**: the seats are separate
matches with separate directories, and each carries its own criterion.

## 7. Cost, and the abort bound

**Seat 1**, ESTIMATED from D-438's own measured game lengths (longest 46 turns)
at 500 ms per pistol answer plus 0.3 s per sealbot answer: ~25 min.

**Seat 2** is ESTIMATED and the estimate is wide on purpose: the per-turn wall
is the 500 ms budget plus an uninterruptible solver overshoot whose size is what
this seat measures. At the committed cap the root's two calls alone can exceed
the budget several times over, so ~1–2 h is the expectation and the number is
not defended.

**Abort bound: 4 hours of wall per seat.** A seat that exceeds it is stopped and
recorded as stopped — a cost anomaly, never a verdict, and never a partial
reading of the games that did finish. Operator attention: one read of each
seat's report against §2.

### 7.1 SEAT 2 MAY NOT BE RUNNABLE, and the branch is registered before it runs

A first revision registered the 120 s wall cap as *"deliberately far above the
budget"* on an unmarked ESTIMATE. **D-441 contradicts it with a measurement**: a
single solver call at cap 16384 was measured between 9 s and 240 s. Two capped
calls at the root alone can therefore exceed the 120 s wall cap, and a seat that
exceeds it **forfeits** — which would make seat 2 a measurement of its own wall
cap and of nothing else.

**The wall cap is therefore raised to 600 s for seat 2** (seat 1 keeps 120 s;
its calls do not exist), and **a TIMING PROBE runs before game one**:

> `dryrun_seat2` at the governed cap and budget, **2 games**, turn cap 20.
> Record the per-answer wall median and maximum, and the wall for the match.

**THE PROBE'S RESULT**: `artifacts/sealbot_anchor_v2_dryrun_seat2/`, 2 games,
13 answers, **median 397 ms / max 2262 ms** at a 100 ms budget, **0 forfeits**,
match wall **10.3 s**.

**THE PROJECTION, taken as a BOUND rather than an estimate.** Per answer at the
governed budget: at most `500 ms + the probe's maximum overshoot of 2162 ms` =
**2.662 s**. Answers per game: at most `turn_cap / 2` = **30**. Sealbot: 30
answers at 0.3 s = 9 s. So **at most 89 s per game and 59 minutes for 40
games** — a bound from the probe's own maximum and the registered caps, not a
mean scaled by a guess. **59 minutes is inside the 4-hour abort bound, so SEAT 2
RUNS.**

**The registered branch, decided before the probe:**

- If the probe's projection — its per-game wall × 40 — is **under 4 hours**,
  seat 2 runs as registered.
- If it is **over**, **seat 2 is DROPPED with an ADR line**, which is the
  dispatch's own provision (*"otherwise seat 2 is dropped with a D-line, not
  improvised"*). It is **not** re-scoped by lowering N, lowering the cap, or
  raising the abort bound: each of those invents a number this document does not
  hold, and a seat that is not the registered seat is not the registered seat.
- If a probe game forfeits on the wall cap even at 600 s, seat 2 is dropped on
  the same ground and the forfeit is recorded as the reason.

**Seat 1 does not depend on this branch** and runs either way.

## 8. What flips or reopens this

- Any change to an instrument revision (§3) before the run.
- Any change to a registered number in §1 — each is an amendment and a fresh
  review of THIS document at its new revision, however small the diff.
- **Nothing about the outcome reopens it.** A 40–0 sweep either way is an
  equally valid anchor, and both leave the standing judgment — pistol below
  sealbot below strong humans (D-197) — exactly where it is. The comparison to
  D-438 that this package's ADR line is permitted to make is **one sentence, of
  DIRECTION only**, because the budget differs and the two are not commensurable.


---

## 10. THE RESULT — both seats, taken under this registration

**Run revision `411c122`**, engine `target/release/pistol` sha256
`bca86067db0d685d7fdf7f5028ff5f2108a27ce986fcfe727b0235323562d881`, configs
`local/sealbot_anchor_v2_seat{1,2}.toml` sha256 `8ba4389b…` / `b5d59cea…`,
sealbot `current/` at 0.3 s. Nothing else ran on the machine
(`ps -eo cmd | /usr/bin/grep -c '[c]argo'` read 0 before each seat).

**THE AGREEMENT CRITERION HOLDS ON BOTH SEATS**: `replay-check: 40
transcript(s) replayed to their recorded outcomes`, exit 0 each. §6's registered
consequence is not reached.

| | **seat 1** (gates off) | **seat 2** (solver gate ON, cap 16384) |
|---|---|---|
| W / L | **20 / 20** | **0 / 40** |
| as p1 | **20–0** | **0–20** |
| as p2 | **0–20** | **0–20** |
| capped | 0 | 0 |
| forfeited | 0 | 0 |
| Wilson 95 % over decided | [0.352, 0.648] | [0.000, 0.088] |
| **DISTINCT GAMES** | **2 of 40** | **2 of 40** |
| pistol answers | 320 | 240 |
| pistol nodes | 47,085,009 | 19,958,155 |
| pistol wall | 109.5 s | 243.9 s |
| sealbot wall | 57.8 s | 56.8 s |
| **per-answer wall, median** | **500 ms** | **1225 ms** |
| **per-answer wall, max** | **508 ms** | **1866 ms** |
| match wall | 2 m 49 s | 5 m 03 s |

### 10.1 The reading, and what it is not

**THE INTERVAL'S NOMINAL N IS 40 AND ITS REAL ONE IS 2.** Both seats played
exactly TWO distinct stone sequences, each replayed twenty times — the same
shape D-438 measured and had to have recovered by hand afterwards. A
`movetime` budget did not buy diversity: the movetime ceiling lands answers at
the budget and the search's completed-depth answer is stable, so the seat is
effectively deterministic from the fixed opening even under a clock. **The
distinct-game count is why that is on the report's face rather than in a
successor's re-analysis.**

**What seat 1's 2-of-2 fact actually says: from the standard opening, at these
budgets, THE FIRST PLAYER WINS.** pistol converts its p1 seat every time at turn
19; sealbot converts its p1 seat every time at turn 15. The 20–20 split is the
seat alternation and is not a measure of either engine.

**Seat 2 loses both seats, and the direction is the one the bracket predicted.**
Turning the solver on at the deployment budget cost pistol its p1 conversion:
0–40, and the compute says why — **2.4x FEWER search nodes in 2.2x MORE wall**
(19.96 M in 243.9 s against 47.09 M in 109.5 s). The WP-1.8c bracket measured
the gate-on seat at an nps ratio near 0.04 and aborted on it; this is that abort
seen from the other end, at the deployment budget rather than the instrument
one, and it is consistent with it rather than a second measurement of it.

**AND THE D-95 / WP-1.4 FORFEIT RISK IS MEASURED AT THE DEPLOYMENT BUDGET,
which is the one thing this anchor can say that D-438's could not.** Seat 1's
overshoot is **8 ms over 320 answers at a 500 ms budget** — the WP-1.4 movetime
ceiling holding well inside its 50 ms epsilon. Seat 2's is **725 ms at the
median and 1366 ms at the maximum**, because a solver call absorbs its whole
node count at once and the root's two calls are made before anything is
abortable. **On HeXO the server owns the clock and hard-clamps the call
(D-478, D-503's residue item 1): a seat answering at 1866 ms against a 500 ms
budget is a forfeit there.** This anchor's local server does not clamp — its
wall cap is 600 s, raised for exactly this reason — so it recorded the overshoot
instead of converting it into a forfeit.

### 10.2 What this is NOT

Not an SPRT, not paired, not an Elo claim, not a strength claim of any kind
beyond *this is what happened in these games*. **Sealbot is UNVERIFIED** (D-197)
and the standing judgment — pistol below sealbot below strong humans — is not
moved by anything here. **The comparison to D-438 is DIRECTION ONLY and one
sentence**: where that node-budgeted anchor had sealbot converting both seats
40–0, this deployment-budget anchor has each side converting its own p1 seat —
a move in pistol's favour, on a different budget, and not commensurable with it.

### 10.3 Artifacts

`artifacts/sealbot_anchor_v2_seat{1,2}/`, each with `report.json`, `report.txt`,
40 transcripts and 80 stderr files, sha-indexed in its own `MANIFEST.sha256`:

| seat | `report.json` | `MANIFEST.sha256` |
|---|---|---|
| 1 | `1d41a36a9c8d7d25e16d79f78a5a36ef153181de0b17149a7aa863d49d884153` | `0d7ff2c469b6ae1c576c3cc2fbbfb16b854e89c9814186c1feca56872c151a8b` |
| 2 | `b9a606eaedc1e457e3d13697c227a5af5f529e4772615310409b658650212114` | `d1097cc72fdf5db4c3107b31caed3ad53759f8121332d041605315623f2b3707` |
