# WP-2.0 — PILOT PRE-REGISTRATION: the label pipeline end to end, on a registered slice

**REVISION 1.** Registered BEFORE the run, at the revision named in §1, under the
WP-2.0 dispatch's Development-round item 4 (`docs/experiments/wp20_dispatches.md`).

**WHAT THIS IS AND IS NOT.** It is a shakedown of a pipeline whose two halves have
passed design review (D-549, D-550) and whose implementations are landed. **It is
not corpus and it carries no census** — D-539 is explicit: *"the pilot carries no
census and is not corpus"*, so its games count toward no minimum and no score is
fitted on them. **No strength claim is made here and none can be**: both seats are
one engine.

---

## 1. THE INSTRUMENT, WITH ITS GOVERNING REVISION

`docs/process.md` requires an artefact producing a registered number to be named
**with its revision**, and this pre-registration is what supplies it — the capture
identity deliberately does not (`docs/experiments/wp20m_design.md` §5).

| what | which |
|---|---|
| pass 1 and pass 2 | the `arena` binary at the commit this document lands in |
| the engine | `target/release/pistol` at that same commit, bound by content |
| the engine config | `configs/instrument_v0.toml` — a committed instrument seat |
| the arena config | `configs/arena_wp20_label_pilot.toml`, landed with this document |
| the book | `crates/pistol-cli/tests/fixtures/random_openings_v2.txt` |

**The commit SHA is filled in by the commit that lands this file** and is the
revision every receipt below is read at. A change to any of the five reopens this
pre-registration (`docs/process.md`, "Instrument governing revision").

## 2. THE SLICE, AND THE LEDGER ROW IT CONSUMES

`docs/book_v2_ledger.md` records no consumed range: this is the book's first
draw. **`openings_skip = 0`, `openings_take = 8`** — the first eight openings, and
`docs/book_v2_ledger.md` gains its row in the same commit that adds the arena
config, which is that file's own rule.

**WHY EIGHT AND NOT MORE.** The pilot's job is to show the pipeline runs end to
end and to MEASURE its throughput, not to produce a sample anything is concluded
from. Eight openings is sixteen paired games, which is the smallest draw that
exercises the pairing the report's own verdict block assumes and still leaves the
run inside a single sitting. **The production corpus is sized from this pilot's
measured throughput and not from this number** (§6).

## 3. THE TWO BUDGETS

| budget | value | why it is that |
|---|---|---|
| GAME | `nodes 50000` | the standing instrument budget every committed arena config in this repository uses |
| LABEL | `nodes 200000` | four times the game budget: the label is the re-scored answer the corpus exists to hold, and a label at the game's own budget would be the game's own answer with a `newgame` in front of it |

**The label budget is a VALUE and therefore lands here rather than in the design**
(D-483). **Its KIND is not a choice**: `--label-nodes` takes a node count and the
grammar has no wall-clock spelling, and pass 2 refuses any source report whose own
budget is not `nodes`.

## 4. THE CRITERIA, REGISTERED BEFORE THE RUN

Each names what its output must show AND the defect class it excludes
(`docs/process.md`, "Criterion and defect class"). **A criterion that is a property
the defect class PRESERVES passes vacuously and is not a criterion.**

### C1 — the cold-label agreement check (D-540's second clause)

**WHAT IS RUN.** After the capture, a sample of its records is re-asked in a
FRESH PROCESS: one `pistol` per sampled position, sent `newgame`, the position,
and the same `go` line, its totals line normalised by the same rule.

**WHAT IT MUST SHOW.** Byte equality with the capture's own record, for every
sampled position.

**THE DEFECT CLASS IT EXCLUDES.** A label produced on a table another `go`
warmed — D-527's own defect, which cost a matrix revision when a check meant to
catch it passed vacuously on the two bands a warm table cannot move. **The
referent is EXTERNAL**: a fresh process shares no transposition table, no
heuristic table and no solver table with the capture pass, so a warm-table defect
in the pass cannot also be present in the referent. This is what makes the check
one the defect could falsify.

**THE REGISTERED CONSEQUENCE.** Any disagreement STOPS the arc. It is not
re-run, not averaged and not attributed to noise: the search is deterministic in
instrument mode by hard rule 4, so two cold answers to one question differ only
if something carries between them.

### C2 — throughput, MEASURED

**WHAT IS RUN.** Wall time of pass 1 and of pass 2, taken separately.
**WHAT IT MUST SHOW.** Games per hour and labels per hour, both derived from the
run's own elapsed time and its own counts. **No expectation is registered**,
because D-500's class is exactly a guessed cost; the number is whatever it is and
§6 extrapolates from it.

### C3 — the determinism re-run receipt

**WHAT IS RUN.** Pass 2 twice over one report at one label budget, and the labels
transform twice over one capture and report.
**WHAT IT MUST SHOW.** Byte-identical output both times.
**THE DEFECT CLASS.** Anything time-based, hash-order-dependent or environment-read
reaching a written field. **THE REGISTERED CONSEQUENCE**: a difference STOPS the
arc.

### C4 — `replay_check` over the pilot's games

**WHAT IS RUN.** `arena --replay` over pass 1's report.
**WHAT IT MUST SHOW.** Zero divergences over every game.
**THE DEFECT CLASS.** A report whose recorded games are not the games its engines
play, which would make every label a label of a position that never occurred.

### C5 — zero forfeits

**WHAT IT MUST SHOW.** The report's own forfeit count is zero.
**THE DEFECT CLASS.** A pipeline whose games end for a protocol reason rather than
a game reason. **A forfeit does not stop the arc** — pass 2 captures a forfeited
game's positions like any other — but a nonzero count means the shakedown found
something and is reported as a finding rather than a footnote.

## 5. THE DRY RUN, AND WHAT IT IS OVER

`docs/process.md` requires this document's literal commands to be exercised
before its review passes, **on an input of the same KIND as the registered
workload and never on the workload itself.** The dry run is the arena's own smoke
config — a self-match over four openings of `openings_v1.txt` at `depth_turns 1`
— **except that pass 2 cannot read that report at all**, because its budget is
not `nodes`. That is itself the dry run's finding and the reason the pilot's own
config carries a `nodes` budget. The dry run therefore uses a scratch arena config
of the same shape as §1's over `openings_v1.txt`, differing only in identity.

**The dry run consumes no book_v2 range and is not the pilot's first run.**

## 6. WHAT THE CLOSURE OWES THIS DOCUMENT

The corpus-size plan for the training package, **labelled ESTIMATED with its
arithmetic shown**, extrapolated from C2's measured throughput. Nothing here
registers a corpus size: that is what the pilot exists to inform.

## 7. COST

Sixteen games at the game budget, then one label ask per asked position at the
label budget, plus one `newgame` per ask — a memset of the whole transposition
table at the committed seat's `tt_bytes`, which is the cost
`docs/experiments/wp20m_design.md` §12 declines to guess. Expected to run inside a
single sitting on this workstation; if it does not, the run is abandoned and the
slice is recorded as consumed anyway, because a range a document reserved is spent
whether or not its run finished.
