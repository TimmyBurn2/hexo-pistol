# Sealbot anchor v3 — pre-registration, revision 1. **NOT RUNNABLE YET.**

> **ONE LINE.** One hundred games against sealbot at the deployment budget, over
> **fifty registered openings played both colours**, on the committed play config
> with every gate off. **Anchor v2 measured what the openings buy**: 40 games,
> **2 distinct stone sequences**, an interval whose nominal N was 40 and whose
> real one was 2. This is that anchor with a denominator.

**THIS DOCUMENT CANNOT GOVERN A RUN YET AND SAYS SO ON ITS OWN FACE.** The local
matchserver plays one hard-coded origin stone; the capability this registration
needs is designed in `docs/experiments/anchor_v3_openings_design.md` revision 2
and is **not implemented**. Every REGISTERED SLOT below is filled from the landed
platform before the review that governs the run, and **an amendment reopens that
review however small the diff** (CLAUDE.md, Process).

**WHAT THIS IS NOT**: not SPRT, not an Elo claim, not a strength claim of any
kind beyond *this is what happened in these N games*. Sealbot is **UNVERIFIED**
(`docs/research/sealbot_notes.md`, D-197). The word is **anchor** everywhere.
D-22 is untouched: instrument mode still refuses a wall-clock budget and every
strength claim in this project still comes from instrument mode — this document
makes none.

**THIS IS NOT A RE-READING OF v1 OR v2.** D-438's own words: *"An opening-policy
change, a budget change, or any engine change is a NEW anchor, not a
re-reading."* An opening-policy change is exactly what this is.

---

## 1. THE REGISTERED NUMBERS — fixed before game one

| what | value |
|---|---|
| **openings** | **50**, from `crates/pistol-cli/tests/fixtures/random_openings_v1.txt`, the first fifty of the body (`skip = 0`, `take = 50`) |
| **games** | **N = 100** — each opening played twice, colours swapped |
| seats | **ONE**. No solver seat |
| pistol budget | **`go movetime 500`** — the deployment budget CLAUDE.md's design point states |
| pistol wall cap | **120 s** per answer, a hang bound. Seat 2's 600 s does not apply: there is no seat 2 |
| sealbot budget | **`time_limit = 0.3` s** per turn — its standing value (D-438 §1); wall cap 5 s per answer |
| turn cap | **60**, the evaluation horizon (game rule 6) |
| config | `configs/play_staged_v0.toml` **as committed** — play mode, staged policy, every gate off (`killers`/`history`/`countermove` false, `safety_net_top_k = 0`, `on_search_path = false`) |

**WHY `book_v1` AND NOT `book_v2`, WITH BOTH RULINGS NAMED.** D-505 forbids a new
pre-registration from slicing `book_v1`; **D-568's fifth limb licenses anchors to
use it** — it is retired for SPRT, and an anchor makes no strength claim, so it
cannot launder a used opening set into one. `book_v2`'s unconsumed range is the
sweep's and its last thousand are a reserved holdout; neither is spent here.

**WHY NO SOLVER SEAT.** D-534 stands: the **725 ms median movetime overshoot** at
the deployment budget blocks any play-config arming of the solver, and no SPRT
discharges it — it is an abort-responsiveness defect, not a strength question.
v2's seat 2 measured a **1225 ms median and an 1866 ms max** against a 500 ms
budget. **A seat that overshoots its budget by 2.5x is measuring its own
overshoot.**

---

## 2. WHAT COUNTS AS WHAT

**Decided**, **capped**, **forfeit** and **compute** are `sealbot_anchor_prereg.md`
§2's definitions, which own them and are not restated (D-423).

**THE INTERVAL IS NOT INHERITED, AND THIS IS THE ONE PLACE v3 MUST DIFFER.** That
section reads: *"Wilson 95% over pistol's share of DECIDED games. No paired
statistic … precisely because nothing here pairs them."* **Under v3 the games ARE
paired** — two per opening, colours swapped — so a Wilson interval over all 100
would be an interval over 100 trials of which at most 50 are independent, and
`report.rs` printing *"not paired"* would be false.

**WHAT IS REGISTERED INSTEAD, and v2's own numbers are why:**

1. **TWO Wilson 95% intervals, one per COLOUR**, each over that colour's DECIDED
   games (n ≤ 50). Within one colour the fifty games come from fifty distinct
   openings, so they are as independent as anything here is. **These are the only
   intervals this anchor reports.**
2. **The PAIR-OUTCOME distribution — p0 / p1 / p2 over 50 pairs** — reported as a
   DIAGNOSTIC and never as an interval. It says how often an opening was decided
   by colour rather than by play.
3. **The aggregate over 100 games is reported as a raw tally with NO interval**,
   and the reason is printed beside it.

**THE REASON, MEASURED.** v2 seat 1 went **20-0 as p1 and 0-20 as p2**. The
aggregate is exactly 20/20 — 50.0% — and that number is an artifact of colour
balance carrying no information about either engine. **An interval around it
would be an interval around an artifact.** The per-colour split is where v2's
information was, and §10.1 read it that way.

**DISTINCT OPENINGS PLAYED, which replaces a diagnostic that goes vacuous.**
`distinct_games` keys on engine turns only, so under v3 it is 100 of 100 by
construction. The report therefore carries **`distinct_openings_played`, expected
50 of 50, each twice** — and `distinct_games` stays, with its expectation stated,
so a value BELOW 100 is the finding it always was.

---

## 3. THE INSTRUMENTS, AT THEIR REVISIONS

| instrument | revision | identity |
|---|---|---|
| the match platform (`tools/sealbot/`, whole tree) | **REGISTERED SLOT** — the commit this document is reviewed at, and it must contain the opening capability | includes the opening reader and `replay_check`'s book binding |
| `run_match.sh` | same commit | drives the match; builds with `--locked` |
| `replay_check` (SECOND INSTRUMENT) | same commit | replays transcripts against `pistol-core` **and binds each game to its opening** |
| the opening source | **REGISTERED SLOT** — decided by `matrix_anchor_openings_reader.md` and its DECISION-RED-TEAM | with the book's `# body_sha256` and the whole-file digest |
| pistol binary | **REGISTERED SLOT** — built `--release --locked` at the run revision | sha256 recorded at launch; **a rebuild means a re-record** |
| run config | `local/sealbot_anchor_v3.toml` | sha256 **REGISTERED SLOT** |
| sealbot | local tree, unversioned | recorded as the shim's argv; **UNVERIFIED by design** |

A change to any of these before the run reopens this pre-registration, however
small the diff (`docs/process.md`, instrument governing revision).

---

## 4. WHAT THE RUN COSTS, DERIVED FROM A MEASUREMENT

`docs/process.md`: a pre-registration states what its governed run costs.

**v2 MEASURED seat 1 at 40 games, `movetime 500`, match wall 2 m 49 s** — 169 s,
**4.2 s per game** (§10). Scaling by games alone:

```
ESTIMATED  100 games x 4.2 s = 422 s = ~7 minutes
```

**TWO THINGS PULL THAT NUMBER IN OPPOSITE DIRECTIONS AND NEITHER IS GUESSED
HERE**: a book opening starts three turns in, which shortens a game; fifty
distinct openings may produce longer games than the one v2 played forty times.
**The dry run in §6 measures it rather than settling it by argument.**

**THE DISPATCH'S "~1-2 h" IS THE CAPABILITY'S COST AND NOT THE RUN'S**, and
revision 1 of the design endorsed it without marking it (D-291). The abort bound
below is set from the measurement, not from that figure.

**ABORT BOUND: 1 hour of match wall.** If the dry run's projection — its
per-game wall x 100 — exceeds it, the anchor is **not re-scoped by lowering N or
the cap**: it is reported as a platform finding and the operator decides. Each of
those re-scopings invents a number this document does not hold.

---

## 5. THE REGISTERED COMMANDS

Run from the repository root, **alone on the machine** — this is a wall-clock
instrument, so a concurrent build or bench voids it
(`ps -eo cmd | /usr/bin/grep -c '[c]argo'` must read 0 before the seat):

```
tools/sealbot/run_match.sh local/sealbot_anchor_v3.toml
tools/sealbot/matchserver/target/release/replay_check artifacts/sealbot_anchor_v3 <THE BOOK ARGUMENT — REGISTERED SLOT>
```

Digests are printed by `run_match.sh` itself over the bytes the seat wrote.

---

## 6. THE DRY RUN — input of the same kind, never the registered workload

**FOUR games, TWO openings, both colours**, at `movetime 100`, against the real
sealbot. `docs/process.md`'s dry-run discipline: the same sort of artefact,
differing only in identity, and never the registered workload.

| criterion | the DEFECT CLASS it excludes | falsifiable? |
|---|---|---|
| **D1** every transcript replays under `replay_check` **with the book argument**, exit 0 | a record whose moves are not what its attested engines answer, and an opening not bound to its book line | yes |
| **D2** the four transcripts show **two distinct book lines, each twice, colours swapped** | a window off-by-one that plays one opening a hundred times — which would otherwise exit 0 with a report echoing `skip`/`take` from the config it was checking | yes |
| **D3** sealbot's first reply from a book position is a **legal turn** | **THE UNVERIFIED LIMB.** The platform has never handed sealbot a non-origin, multi-stone setup. The shim's contract can express one, which is a reading of the matchserver's code and NOT a claim about sealbot's own `HexGame` | yes — the referee converts an illegal sealbot reply into a named forfeit |
| **D4** the seat is in **play mode**, by the mode pin and by node counts that VARY across answers | a seat that silently ran the instrument budget | yes |
| **D5** per-answer wall median and maximum, and the match wall, recorded | — this is a MEASUREMENT, not a criterion, and §4's abort bound is read from it | n/a |

**D4 REPLACES A CRITERION THE SHIPPED BINARY CANNOT SATISFY.** A draft of the
design registered *"the go line read back from the record is `go movetime <ms>`"*;
no transcript, report or engine-stderr field carries the go line, only the stub
prints it, and v2's own criterion F had already struck it in as many words.

**THE REGISTERED CONSEQUENCE OF EACH FAILURE.** D1, D2 or D4 failing means the
platform is wrong and **the anchor does not run** — it is a platform finding and
a package, not a retry. **D3 failing STOPS the anchor** and is reported as a
sealbot-compatibility finding; it is not worked around by changing the opening
source, because an opening source chosen to make sealbot answer is an opening
source chosen after seeing data.

---

## 7. WHAT IS REPORTED

W / L / capped / forfeits, **split by colour**; the two per-colour Wilson 95%
intervals over decided games; the pair-outcome distribution; `distinct_games` and
`distinct_openings_played`; per-side compute (**nodes and wall**); and the
per-answer **overshoot median and maximum** against the 500 ms budget.

**Opponent UNVERIFIED. No Elo. The word is ANCHOR everywhere.**

**ONE ADR LINE, and its comparison to v1 and v2 is ONE SENTENCE OF DIRECTION
ONLY** — the opening policy differs, so the three are not commensurable, which is
the same limit D-438 placed on v2's comparison to it.

---

## 8. WHAT REOPENS THIS

- Any change to an instrument revision in §3.
- Any change to a registered number in §1 — an amendment and a fresh review of
  THIS document at its new revision, however small the diff.
- **Nothing about the OUTCOME reopens it.** A 100-0 sweep either way is an equally
  valid anchor, and both leave the standing judgment — pistol below sealbot below
  strong humans (D-197) — exactly where it is.
