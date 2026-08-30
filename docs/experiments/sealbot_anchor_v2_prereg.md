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
| pistol wall cap | **120 s** per answer — a hang bound, deliberately far above the budget so an OVERSHOOT is RECORDED rather than converted into a forfeit |
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
(`crates/pistol-search/src/search.rs:266`, `:283`) where nothing is abortable.
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
- **Overshoot**: per pistol answer, the wall time the transcript records against
  the 500 ms budget. Reported per seat as median and maximum. Not a verdict.

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

**Input**: two reduced-budget configs of the same kind — the same two real
engines, the same seats rule, at `movetime 100`, sealbot `0.05` s, turn cap 20,
**2 games** — one per seat, so BOTH seats' plumbing is exercised.

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
- **F. THE OVERSHOOT COLUMN IS NOT ZERO AND NOT CONSTANT.** *Defect class: a
  `movetime` seat that is really still node-budgeted, or a wall column copied
  from the budget rather than measured — either makes §2's overshoot report a
  transcription of the input.*

**The record** — REGISTERED SLOT, filled from the dry run's own bytes before this
document's review passes, together with the binary and config digests of the
instance that produced it.

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

## 8. What flips or reopens this

- Any change to an instrument revision (§3) before the run.
- Any change to a registered number in §1 — each is an amendment and a fresh
  review of THIS document at its new revision, however small the diff.
- **Nothing about the outcome reopens it.** A 40–0 sweep either way is an
  equally valid anchor, and both leave the standing judgment — pistol below
  sealbot below strong humans (D-197) — exactly where it is. The comparison to
  D-438 that this package's ADR line is permitted to make is **one sentence, of
  DIRECTION only**, because the budget differs and the two are not commensurable.
