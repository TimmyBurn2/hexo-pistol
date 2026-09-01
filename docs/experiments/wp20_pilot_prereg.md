# WP-2.0 — PILOT PRE-REGISTRATION: the label pipeline end to end, on a registered slice

**REVISION 2.** Registered BEFORE the run, at the revision named in §1, under the
WP-2.0 dispatch's Development-round item 4 and the WP-2.0-finish dispatch's §4
(`docs/experiments/wp20_dispatches.md`), and under the operator grant recorded at
`docs/decisions.md` D-552.

## 0.1 REVISION HEADER — what moved from revision 1, and why

**REVISION 1 TOOK NO REVIEW.** No section of this document has ever been passed by
a reviewer, so D-547's passed-section freeze binds nothing here yet and this
header records the change for the reader rather than to discharge the law. From
revision 2 onward the freeze binds normally.

| § | what changed | why |
|---|---|---|
| 1 | the instrument table gains four rows and every row gains a revision | revision 1 named five artefacts and no revision for any of them, which is the one thing `docs/process.md`'s "Instrument governing revision" requires by name |
| 2 | the range and its grounds are derived from a MEASURED cost, not chosen | revision 1's *"eight openings … leaves the run inside a single sitting"* is D-500's class exactly: a wall guessed rather than derived |
| 4 | every criterion gains its instrument's EXIT CODES and its literal MESSAGES | revision 1 stated what each criterion must show and named no instrument that could show it — C1 in particular named a check that did not exist |
| 4E | a criterion for the schema and its loader is ADDED | revision 1 had none, and the dispatch requires one |
| 5 | the verdict space is enumerated and the VOID class is defined | revision 1 defined neither, so a run that ended for an environment reason had no reading |
| 6 | the wall estimate is derived from a cited timing artifact | as §2 |
| 7 | the dry run gains its own criterion, defect class, and recorded output | revision 1 described a dry run and registered nothing it could fail |
| 9 | a slot table (D-427) | revision 1 had none, and D-427 is a measured instance of a config drifting from the prose that governs it |

## 0.2 WHAT THIS IS AND IS NOT

It is a shakedown of a pipeline whose two halves have passed design review (D-549,
D-550) and whose implementations are landed. **It is not corpus and it carries no
census** — D-539 is explicit: *"the pilot carries no census and is not corpus"*, so
its games count toward no minimum and no score is fitted on them. **No strength
claim is made here and none can be**: both seats are one engine, which is what
makes the report capturable at all (§4C).

---

## 1. THE INSTRUMENT, WITH ITS GOVERNING REVISION

`docs/process.md` requires an artefact producing a registered number to be named
**with its revision**, and a change to any of them reopens this pre-registration.
The capture's own identity deliberately does not close over the instrument
version (`docs/experiments/wp20m_design.md` §5), so this table is where that fact
is supplied.

| what | which | revision |
|---|---|---|
| pass 1 — the games | `arena --config`, from `crates/pistol-arena/src/bin/arena.rs` | SLOT R1 |
| pass 2 — the capture | `arena --capture`, `crates/pistol-arena/src/capture.rs` | SLOT R1 |
| pass 3 — the corpus | `arena --labels`, `crates/pistol-arena/src/labels.rs` | SLOT R1 |
| the corpus loader | `crates/pistol-arena/src/bin/corpus-check.rs` | SLOT R1 |
| the cold-label referent | `tools/cold_label_check.py` | SLOT R1 |
| the engine | `target/release/pistol`, bound by content | SLOT R2 (`binary_sha256`) |
| the engine config | `configs/instrument_v0.toml` | SLOT R1 |
| the arena config | `configs/arena_wp20_label_pilot.toml` | SLOT R1 |
| the dry-run arena config | `configs/arena_wp20_label_pilot_dryrun.toml` | SLOT R1 |
| the book | `crates/pistol-cli/tests/fixtures/random_openings_v2.txt` | SLOT R1 |

**SLOT R1 is one value**: the commit this document lands in. Every artefact above
except the engine binary is a tracked file, so naming the commit names all of
them at once, and a change to any reopens this document.

**WHY `configs/instrument_v0.toml` AND NOT ANOTHER SEAT.** It is the committed
instrument seat every strength claim in this project is made at, and its
candidate policy is the SPRT-positive one: `docs/ROADMAP.md` records that
WP-1.5b's SPRT *"fired the supersession this section names above:
`configs/instrument_v0.toml`'s committed candidate policy moved from radius 2 to
staged in the same commit"* (D-386), and the file reads `kind = "staged"` today.
Its solver gate is `on_search_path = false`, which is where D-441 requires it and
where D-534's unfixed 725 ms overshoot requires it to stay. **The production
teacher is not fixed by this choice** — the pilot is not corpus — but the
throughput §6 measures is throughput AT THIS SEAT, and a production run at a
different seat re-measures it rather than inheriting it.

## 2. THE SLICE, AND THE LEDGER ROW IT CONSUMES

`docs/book_v2_ledger.md` records **no consumed range**: this is the book's first
draw. **`openings_skip = 0`, `openings_take = ` SLOT S1** — a prefix from the top,
which is a sample because the book is emitted in generator order with no
provenance column to sort by.

**THE SIZE IS DERIVED, NOT CHOSEN.** §6 fixes a wall budget for the whole pilot
and §7's dry run measures the per-unit cost of each pass on this machine at these
budgets; SLOT S1 is the largest take whose derived wall fits that budget, with
the arithmetic shown in §6. Revision 1 chose eight and defended it with *"leaves
the run inside a single sitting"*, which is a guess about wall time standing where
a measurement belongs (D-500's class).

**THE LEDGER ROW LANDS IN THE SAME COMMIT AS THIS DOCUMENT AND THE ARENA CONFIG**,
which is `docs/book_v2_ledger.md`'s own rule: *"A new pre-registration takes the
next unconsumed range, adds its row here in the same commit that adds its arena
config"*. The range is consumed by the registration and not by the run — the same
file's rule: *"A slice appears here when its pre-registration is committed, not
when its run finishes"* — so an abandoned pilot does not free it.

## 3. THE TWO BUDGETS

| budget | value | why it is that |
|---|---|---|
| GAME | `nodes 50000` | the standing instrument budget every committed arena config in this repository uses, and the one D-520's and WP-1.5d's governed runs were taken at |
| LABEL | `nodes ` SLOT S2 | the label is the re-scored answer the corpus exists to hold; a label at the game's own budget would be the game's own answer with a `newgame` in front of it. Its VALUE is fixed by §6's RULE-2, which is registered below and before the dry run measures anything |

**BOTH ARE VALUES AND THEREFORE LAND HERE RATHER THAN IN THE DESIGN** (D-483).
**The label budget's KIND is not a choice**: `--label-nodes` takes a node count,
the grammar has no wall-clock spelling, and pass 2 refuses any source report whose
own budget is not `nodes`. `crates/pistol-arena/src/usage.rs` states the reason on
the program's own face: *"there is no wall-clock spelling to refuse, because a
wall-clock label would be a fact about the machine"*.

---

## 4. THE CRITERIA, REGISTERED BEFORE THE RUN

Each names its INSTRUMENT, what its output must show, the DEFECT CLASS the
criterion excludes, and its REGISTERED CONSEQUENCE (`docs/process.md`, "Criterion
and defect class"; "Cost, replication, and the second instrument"). **A criterion
that is a property the defect class PRESERVES passes vacuously and is not a
criterion.**

### C-A — the cold-label agreement check (D-540's second clause)

**THE INSTRUMENT.** `tools/cold_label_check.py` at SLOT R1. Its own usage block
fixes its answers, quoted here so a reader of this document does not have to trust
a paraphrase:

```
Exit:  0 every sampled record agrees byte for byte — THE ANSWER IS YES
       1 a sampled record disagrees — THE ANSWER IS NO
       2 THE RUN IS VOID: no answer was taken (tools/SHELL_CHECKLIST.md item
         12). A void is not a disagreement and must not be read as one.
```

**THE REGISTERED SAMPLE.** `--stride ` SLOT S3, which the script prints back on
its own first line as *"the sample is every record whose zero-based index is a
multiple of `<n>`, which is `<k>` of them"*. **The alignment hazard is named
because a stride is a systematic sample**: the capture's records are ordered
game-major and turn-minor, so a stride equal to (or dividing) the number of
records a game contributes would sample one turn index of every game and nothing
else — the empty board over and over. SLOT S3 is fixed in §6 against the
per-game record counts the dry run measures, and where the budget affords it the
registered value is **1**, which is every record and has no alignment to have.

**WHAT ITS OUTPUT MUST SHOW.** Exit `0`, and the line
`cold_label_check: <k> of <k> sampled record(s) agree byte for byte`.

**THE DEFECT CLASS IT EXCLUDES.** A label produced on a table another `go` warmed
— D-527's own defect, which cost a matrix revision when the check meant to catch
it *"passed vacuously on the two bands a warm table cannot move"*.

**WHY THE REFERENT IS EXTERNAL, WHICH IS WHAT MAKES THE CHECK NON-VACUOUS.** The
script spawns **one process per sampled position** and sends it `newgame`, the
capture's own `position` line and the capture's own `go` line. A fresh process
shares no transposition table, no heuristic table and no solver table with the
capture pass, so a warm-table defect in the pass cannot also be present in the
referent — `docs/process.md`'s *"a value computed by something that does not share
the suspect input"*. **The normalisation is re-derived in the referent on
purpose**: the script strips ` nps <n> time <n>` with its own regular expression
rather than calling `crates/pistol-arena/src/capture.rs`'s `normalise`, and the
error direction is safe — an independent strip can manufacture a mismatch and
cannot hide one.

**THE REGISTERED CONSEQUENCE.** **Exit 1 STOPS THE ARC.** It is not re-run, not
averaged and not attributed to noise: the search is deterministic in instrument
mode by hard rule 4, so two cold answers to one question differ only if something
carried between them. **Exit 2 is a VOID and not a disagreement** (§5).

### C-B — the determinism re-run receipt

**THE INSTRUMENT.** The `arena` binary at SLOT R1, run twice per pass, compared by
`sha256sum`. **THE REGISTERED RANGE IS THE WHOLE PILOT AND NOT A SUB-RANGE**, in
both passes, and the reason is that neither pass has a subsetting flag: `--capture`
walks the report it is given and `--labels` transforms the capture it is given, so
a "sub-range" could only be produced by editing a report — which changes its
`source_sha256` and therefore the capture identity, making the re-run a different
question. §6's wall budget carries the second capture pass as a first-class cost
rather than discovering it afterwards.

**WHAT IT MUST SHOW.** For pass 2: two captures over one report at one label
budget, byte-identical. For pass 3: two corpora over one capture and one report,
byte-identical. Both are `sha256sum` equality on the whole rendered file, header
included.

**WHY PASS 1 IS NOT IN THIS CRITERION, said so a reader does not read its absence
as an oversight**: the arena's report carries its own wall time, so two runs of one
match differ in bytes for a reason that changes no answer. Pass 1's reproducibility
is what C-C measures, by re-driving its games rather than by digesting its file.

**THE DEFECT CLASS.** Anything time-based, hash-order-dependent or environment-read
reaching a written field. **THE REGISTERED CONSEQUENCE**: a difference **STOPS THE
ARC**.

### C-C — `replay_check` over every pilot game, and zero forfeits

**THE INSTRUMENT.** `arena --replay <report> --out <path> --workers <n>` at SLOT R1.
Its answers, from `crates/pistol-arena/src/usage.rs` and
`crates/pistol-arena/src/bin/arena.rs`:

```
exit: 0 completed cleanly, 1 abandoned or forfeited (report still written),
      2 a document this build refuses (no report)
```

and the mode exits `0` only where `divergences == 0 && covered == total`, printing
`arena: replayed <c> of <t> game(s) from <report>, <d> divergence(s)`.

**WHAT IT MUST SHOW.** Exit `0`, `<d>` equal to `0`, and `<c>` equal to `<t>`.
**The printed counts are read and not inferred from the exit code**: a criterion
that reads only the status cannot tell "no divergences over every game" from a
gate that answered about fewer games than it was given, and the mode's own stderr
says exactly that where it happens — *"a criterion over some of a report's games is
not one anybody registered"*.

**ZERO FORFEITS.** The pilot's own pass-1 exit code is the receipt: `--config`
returns `0` only where `score::tally(&played.records).forfeits == 0`, and `1`
otherwise with the report still written. **A nonzero forfeit count does not stop
the arc** — pass 2 captures a forfeited game's positions like any other, which is
D-544's own recorded decision and is safe because `transcript::read`
legality-checks every game before pass 2 exists — but it is reported as a FINDING
rather than a footnote, because a self-match of one deterministic engine against
itself has no protocol reason to forfeit.

**THE DEFECT CLASS.** A report whose recorded games are not the games its engines
play, which would make every label a label of a position that never occurred.

### C-D — throughput, MEASURED

**THE INSTRUMENT.** The `date +%s.%N` pair bracketing each pass in §8's command
block, and the report's own `wall_ms` record for pass 1 as a second reading of the
same quantity.

**WHAT IT MUST SHOW.** Games per hour and labels per hour, each derived from the
run's own elapsed seconds and its own counts, both stated with the worker count
they were taken at — pass 1 runs at `n_workers` and **pass 2 is serial by
construction**, spawning one seat on one channel
(`crates/pistol-arena/src/capture.rs`), so the two numbers are not comparable and
the document says so rather than letting a reader average them.

**NO EXPECTATION IS REGISTERED AND NONE MAY BE READ IN.** D-500's class is exactly
a guessed cost; the number is whatever it is, and §6 of the CLOSURE extrapolates
from it. **This is the one criterion the pilot cannot fail** — it is a
measurement, not a test — and it is registered so that the closure's corpus plan
has a cited source rather than an estimate.

### C-E — the schema and its loader

**THE INSTRUMENT.** `crates/pistol-arena/src/bin/corpus-check.rs` at SLOT R1, which
reads a corpus back through the SAME loader the writer writes for. Its usage block:

```
exit: 0 every document loads
      1 a document this build refuses (the reason is named)
      2 THE RUN IS VOID: no document was named, or one could not be read at
        all. A void is not a refusal and must not be read as one.
```

**WHAT IT MUST SHOW, IN THREE RUNS.**

1. **THE CONTROL.** Over the pilot's own corpus: exit `0` and
   `corpus_check: <path> ok, <n> record(s), capture_sha256 <hex>`, with `<n>`
   equal to the record count `arena --labels` printed. *A loader that refused
   everything would pass runs 2 and 3 and answer nothing.*
2. **A GRAMMAR INJECTION, RE-DIGESTED HONESTLY.** One record's `key_pos` replaced
   by `not-a-key` and the body digest brought back into agreement: exit `1`,
   naming `key_pos`. **The re-digest is what makes this reach the grammar**: an
   injection that left the digest stale would be refused two checks earlier and
   would test the digest instead of the schema, which is the vacuity this
   criterion exists to avoid.
3. **A DIGEST INJECTION, NOT RE-DIGESTED.** One record appended: exit `1`, naming
   `digests to`. This is the run that proves runs 1 and 2 are about different
   guards.

**THE DEFECT CLASS.** A transform that writes a corpus its own reader refuses —
the class `c7f194e` fixed at one guard (*"the write side stops producing a corpus
its own loader refuses"*) and which is only fixed at the level of the whole
grammar by reading a real corpus back. The write side's `writable` checks a
record's ARITY; the key shapes, the three score kinds, the four token sets and the
body digest are enforced only on read.

**THE D-551 PARSER FACTS THIS CRITERION STANDS ON**, quoted rather than
paraphrased because the first design of this package died on their negation:
*"the score is TWO words after its key (`cp <n>`, `mate <t>`, `-mate <t>`), so a
key-value map yields the tag and loses the number"*, and *"a word list also
carries the CAPTURED line, which has no `time` field because gate 9's rule removed
it, and which `totals_of` therefore cannot read at all"*. Run 1 is the check that
those hold over a whole corpus this build actually wrote, at the pilot's own
scale, rather than over the two synthetic lines
`crates/pistol-arena/src/exchange.rs`'s unit tests carry.

**THE REGISTERED CONSEQUENCE.** Any of the three answering other than as
registered **STOPS THE ARC**.

---

## 5. THE VERDICT SPACE, TOTAL — AND THE VOID CLASS

**Every way this pilot can end, with no residue.** A run whose outcome is not one
of these rows is itself a finding about this document.

| # | outcome | what makes it that | what follows |
|---|---|---|---|
| V1 | **PASS** | C-A, C-B, C-C and C-E all as registered; C-D measured | the closure's §3 runs: the corpus plan is derived from C-D |
| V2 | **PASS WITH A FINDING** | as V1, except a nonzero forfeit count | as V1, and the forfeit count and its games are named in the closure |
| V3 | **STOP — cold-label mismatch** | C-A exit 1 | the arc STOPS; artifacts preserved; the disagreeing records are the finding |
| V4 | **STOP — determinism** | C-B: a digest differs | the arc STOPS; both outputs preserved |
| V5 | **STOP — replay** | C-C: exit 1, or a nonzero divergence count, or `covered != total` | the arc STOPS; the replay document is the diagnostic |
| V6 | **STOP — schema** | C-E: any of the three runs answers other than as registered | the arc STOPS; the corpus and the two injections preserved |
| V7 | **VOID** | see below | ONE re-run, on a receipted environment fault; a SECOND void STOPS with artifacts preserved |

**THE VOID CLASS, DEFINED.** A void is *no answer was taken*, never *the answer is
no* (`tools/SHELL_CHECKLIST.md` item 12). It is exactly one of:

- an instrument exiting `2` — `cold_label_check` or `corpus-check` refusing to
  look, or `arena` refusing a document before any game;
- the machine taking the run away: a filesystem filling, a process killed, a
  reboot, or the session ending mid-pass. `/tmp` on this machine is a 24 GiB
  RAM-backed tmpfs and its exhaustion is the recorded instance (D-281, D-285);
- **and nothing else.** A criterion answering "no" is V3-V6 and is NOT a void, and
  a run that is slower than §6 estimated is not a void either — a wall estimate is
  not a criterion and cannot be failed.

**A void is receipted before it is re-run**: the receipt names the filesystem, the
process or the signal, so that "I could not look" is distinguishable in the record
from "the pipeline is wrong". A void with no receipt is a STOP.

---

## 6. THE WALL ESTIMATE, DERIVED — AND THE SLOTS IT FIXES

**Derived from §7's dry run, which is a cited timing artifact and not a guess**
(D-500's class). The dry run measures four per-unit costs on this machine at these
budgets, and this section shows the arithmetic that turns them into SLOT S1
(`openings_take`), SLOT S2 (the label budget) and SLOT S3 (the cold-label stride).

### 6.1 THE TWO RULES, REGISTERED BEFORE THE DRY RUN MEASURES ANYTHING

This is `book_v2`'s own discipline applied to a pilot instead of a book — D-518's
*"the decision rule registered before the sweep, the sweep before the size"* — and
it exists so that no number below can be chosen after seeing which number would
be convenient. **Both rules are total: applied to §7's measurements they leave no
free choice.**

**THE WALL BUDGET, stated as the constraint it is.** The pilot's total machine
time is **at most four hours**, counting every pass this document registers —
pass 1, BOTH capture passes, both corpus transforms and the cold-label check.
This is an operator-attention and session-capacity constraint and is honestly
labelled one: it is not derived from anything about the pipeline, and it is the
only number in §6 that is not.

**RULE-2 — the LABEL budget (SLOT S2), fixed by DEPTH and not by wall.** The
candidates are `100000`, `200000` and `400000` nodes — two, four and eight times
the game budget. SLOT S2 is **the SMALLEST candidate whose median completed
`depth_turns` over the dry run's asked positions is at least one turn greater
than the median `depth_turns` the same positions reach at the GAME budget.** The
ground is revision 1's own sentence with a measurement behind it: a label that
reaches no deeper than the game's own search is the game's own answer with a
`newgame` in front of it. **If no candidate satisfies it, that is a FINDING and
this document is amended rather than the rule relaxed** — a label budget that
buys no depth is a fact about the search worth knowing before a corpus is sized
on it.

**RULE-1 — the SLICE (SLOT S1), the free variable.** With SLOT S2 fixed, SLOT S1
is **the largest `openings_take` whose derived wall — by §6.2's arithmetic, from
§7's measured per-unit costs — is at or under the four-hour budget**, and it is
**at least 4** (eight games), which is the floor at which the report's own
pairing is exercised over more than one pair. **If the floor does not fit at SLOT
S2, SLOT S2 drops to the next smaller candidate and RULE-1 is applied again**;
if the floor does not fit at `100000` either, that is a STOP and the pilot returns
to the architect, because a pilot below its own floor measures nothing the
closure can use.

**RULE-3 — the cold-label stride (SLOT S3).** SLOT S3 is **1 — every record — if
the derived wall including a stride-1 cold check fits the budget under RULE-1;
otherwise the smallest odd number greater than 1 that does not divide any
per-game record count the dry run observed.** The exclusion is not fastidiousness:
the capture's records are ordered game-major and turn-minor, so a stride dividing
a game's record count samples ONE turn index of every game — at the pilot's
capped shape, the empty board over and over — and a systematic sample aligned to
the thing it is sampling is not a sample.

### 6.2 THE ARITHMETIC

Let `g` be the measured seconds per pass-1 GAME at `n_workers = 4`, `p` the
measured POSITIONS per game, `l` the measured seconds per label ask at SLOT S2,
and `c` the measured seconds per COLD ask (a process spawn and a search, which is
a fresh 256 MiB transposition table each time). For a take of `T` openings the
pilot plays `2T` games, so:

```
wall  =  2T*g                      pass 1, at 4 workers
      +  2 * (2T*p*l)              BOTH capture passes, serial (C-B)
      +  2 * (transform)           both corpus transforms, no engine
      +  (2T*p / S3) * c           the cold-label check, serial
```

SLOT W — the four measured costs `g`, `p`, `l`, `c`, the derived wall at the
chosen `T`, and the values RULE-1, RULE-2 and RULE-3 return. **Filled at
registration, from §7's artifact, before the review is dispatched and before any
pilot pass runs.**

**THE COST THIS DOCUMENT DECLARES** (`docs/process.md`, "Cost, replication, and the
second instrument"): SLOT S1 openings is `2 x` SLOT S1 games at the game budget,
then **two** full capture passes over every asked position at the label budget,
then two corpus transforms, then one cold ask per sampled position — each of the
last in its own process, which is a fresh 256 MiB transposition table per ask, the
cost `docs/experiments/wp20m_design.md` §12 declines to guess and which §7
measures. Machine hours are SLOT W; operator attention is one launch and one
reading.

**REPLICATION AND THE SECOND INSTRUMENT.** The pilot's cheap criteria are
replicated by construction: C-B *is* a replication, and C-A is a second instrument
whose agreement criterion is registered above before either it or the capture
runs, with its disagreement consequence registered with it. **The stage under
doubt is the CAPTURE PASS's table state**, and the cold checker does not share it:
it spawns a process per position, where the capture pass keeps one for the whole
walk. C-C is a third reading of pass 1 by a different route — re-driving the games
rather than digesting the report — and it shares the ENGINE with pass 1, which is
stated because two instruments blind to the same stage are one instrument reported
twice.

---

## 7. THE DRY RUN — ITS INPUT, ITS CRITERION, ITS DEFECT CLASS, ITS OUTPUT

`docs/process.md` requires this document's literal commands to be exercised before
its review passes, **on an input of the same KIND as the registered workload and
never on the workload itself.**

**THE INPUT.** `configs/arena_wp20_label_pilot_dryrun.toml`, which differs from
`configs/arena_wp20_label_pilot.toml` in identity and size alone: a different
openings fixture (`crates/pistol-cli/tests/fixtures/openings_v1.txt`) and a
smaller take. **Same kind, stated exactly**: both are sha-pinned opening corpora
with an in-band body digest, both are read by the same `openings::load`, and both
carry the same encoding. **The one difference that is not identity is recorded
rather than smoothed over**: v1's openings are FOUR turns and v2's are THREE
(`configs/random_openings_v2.toml` fixes five stones, *"the position after turn
3"*), so under one turn cap a dry-run game has one fewer engine-chosen turn than
a pilot game. §6's arithmetic uses per-TURN and per-POSITION costs and never a
per-game constant, which is what makes the extrapolation survive that difference.

**IT CONSUMES NO `book_v2` RANGE**, so it is not a draw the ledger records, and it
is not the pilot's first run.

**THE DRY RUN'S OWN CRITERION.** Every command in §8 runs to its registered exit
code on the stand-in, and the four per-unit costs of §6 come back as finite
positive numbers with the counts they are derived from.

**THE DEFECT CLASS THE DRY RUN EXCLUDES.** A command in §8 that cannot run at all
against a real artefact of this kind — a flag spelled wrong, a mode that refuses
the report the previous mode wrote, an instrument that cannot read the file its
predecessor produced. **This is ATTRIBUTION and not syntax**, which is why the
stand-in is a real arena report rather than a synthetic one:
`docs/process.md` records that *"a synthetic stand-in exercises syntax; only a
real instance of the kind exercises ATTRIBUTION"*. **Revision 1's own dry run
found exactly one such defect and it is kept on the record**: the arena's smoke
config produces a report pass 2 cannot read at all, because its budget is
`depth_turns` and not `nodes` — which is why the dry-run config carries a `nodes`
budget and why that is a finding rather than a footnote.

**WHAT THE DRY RUN IS NOT.** It is not a governed sample, it does not consume the
pilot's first run, and its numbers are inputs to §6's arithmetic and never
evidence about the pipeline's correctness.

SLOT D — the dry run's recorded input, every command's exit code, and its output.

---

## 8. THE LITERAL COMMAND BLOCK

Run from the repository root, against a release build. `ART` is a directory
outside the repository (CLAUDE.md rule 8).

Two arguments are §6's own outputs and are written here as `<S2>` and `<S3>`;
SLOT C records the block with those two substituted, as run.

**TIMING IS `SECONDS`, bash's own integer counter.** Not `EPOCHREALTIME`, which
writes the locale's decimal separator and would have to be pinned to be
comparable (`tools/SHELL_CHECKLIST.md` item 4); not `/usr/bin/time`, which is not
installed on this machine. Every pass here is minutes, so integer seconds lose
nothing.

**EVERY EXIT CODE IS TAKEN INTO A VARIABLE AND PRINTED**, never left to `set -e`
and never read out of a pipeline: a command substitution's status is discarded
and a bare `set -e` death prints nothing (`tools/SHELL_CHECKLIST.md` items 1 and
2). The block is deliberately written without `set -e` for that reason — it must
reach the end and report every code, because a criterion that STOPS the arc is
read from a code and not from the absence of output.

```bash
ART=/home/tom/pistol-runs/wp20pilot-artifacts   # outside the repository, CLAUDE.md rule 8
mkdir -p "$ART"
A=target/release/arena
P=target/release/pistol
CFG=configs/arena_wp20_label_pilot.toml

# --- pass 1: the games -------------------------------------------------------
t=$SECONDS
"$A" --config "$CFG" --out "$ART/report_v1.txt"; echo "pass1 exit=$?"
echo "pass1 seconds=$((SECONDS - t))"

# --- pass 2: the capture, TWICE (criterion C-B) ------------------------------
t=$SECONDS
"$A" --capture "$ART/report_v1.txt" --out "$ART/capture_v1.txt" --label-nodes <S2>
echo "capture1 exit=$?"; echo "capture1 seconds=$((SECONDS - t))"
t=$SECONDS
"$A" --capture "$ART/report_v1.txt" --out "$ART/capture_v2.txt" --label-nodes <S2>
echo "capture2 exit=$?"; echo "capture2 seconds=$((SECONDS - t))"
sha256sum "$ART/capture_v1.txt" "$ART/capture_v2.txt"

# --- pass 3: the corpus, TWICE (criterion C-B) -------------------------------
"$A" --labels "$ART/capture_v1.txt" --report "$ART/report_v1.txt" --out "$ART/corpus_v1.txt"
echo "labels1 exit=$?"
"$A" --labels "$ART/capture_v1.txt" --report "$ART/report_v1.txt" --out "$ART/corpus_v2.txt"
echo "labels2 exit=$?"
sha256sum "$ART/corpus_v1.txt" "$ART/corpus_v2.txt"

# --- criterion C-A: the cold-label agreement check ---------------------------
t=$SECONDS
python3 tools/cold_label_check.py --capture "$ART/capture_v1.txt" \
    --binary "$P" --engine-config configs/instrument_v0.toml --stride <S3>
echo "cold exit=$?"; echo "cold seconds=$((SECONDS - t))"

# --- criterion C-C: replay over every pilot game -----------------------------
"$A" --replay "$ART/report_v1.txt" --out "$ART/replay_v1.txt" --workers 4
echo "replay exit=$?"

# --- criterion C-E, run 1: the control ---------------------------------------
target/release/corpus-check "$ART/corpus_v1.txt"; echo "load exit=$?"

# --- criterion C-E, run 2: a grammar injection, RE-DIGESTED honestly ---------
python3 - "$ART/corpus_v1.txt" "$ART/corpus_grammar.txt" <<'PY'
import hashlib, sys
src, dst = sys.argv[1], sys.argv[2]
text = open(src, encoding="utf-8").read()
mark = "# body_sha256 "
at = text.index(mark)
head, body = text[:at], text[text.index("\n", at) + 1:]
rows = [r for r in body.split("\n") if r]
f = rows[0].split("\t"); f[4] = "not-a-key"; rows[0] = "\t".join(f)
body = "".join(r + "\n" for r in rows)
open(dst, "w", encoding="utf-8").write(
    f"{head}{mark}{hashlib.sha256(body.encode()).hexdigest()}\n{body}")
PY
target/release/corpus-check "$ART/corpus_grammar.txt"; echo "inject-grammar exit=$?"

# --- criterion C-E, run 3: a record appended, digest NOT brought back --------
cp "$ART/corpus_v1.txt" "$ART/corpus_digest.txt"
printf '0\t0\t-\t-\t%s\t-\tp1\teval\t0\t0,0\t1\t1\t0\tyes\tcapped\tnormal\n' \
    "$(printf '0%.0s' $(seq 32))" >> "$ART/corpus_digest.txt"
target/release/corpus-check "$ART/corpus_digest.txt"; echo "inject-digest exit=$?"
```

**THE TWO INJECTIONS ARE DIFFERENT GUARDS AND THE BLOCK PROVES IT.** Run 2
re-digests and therefore reaches the GRAMMAR; run 3 does not and is stopped by the
DIGEST two checks earlier. Without run 3, run 2 would be indistinguishable from a
loader that only ever checks digests — which is the vacuity `docs/process.md`
names, in the shape it takes here. The same two injections are pinned as tests
against the shipped binary at
`crates/pistol-arena/tests/labels_tests.rs`; the runs above are a second instance
over a corpus the pilot itself wrote, at the pilot's own scale.

SLOT C — this block as run, with `<S2>` and `<S3>` substituted and every printed
exit code and elapsed second recorded.

---

## 9. THE SLOTS, FILLED AT THE RUN'S OWN LAUNCH (D-427)

D-427 is a measured instance of an arena config still carrying a value the
pre-registration that governs it had changed two revisions earlier, and of there
being no gate that would have caught it. **The slot pass is this document's answer
to that**: every value in `configs/arena_wp20_label_pilot.toml` is quoted here
beside the prose that governs it, at launch, and a disagreement is corrected in
the config before any game.

**THE TABLE'S SHAPE IS FIXED HERE AND ITS VALUES AT LAUNCH**, so a reviewer can
see what will be checked rather than take the check on trust.

| where | key | what fixes it |
|---|---|---|
| config `[run]` | `openings_file` | §1 and §2 — `random_openings_v2.txt`, the only book a governed slice may be drawn from (D-505) |
| config `[run]` | `openings_take` | §6 RULE-1 (SLOT S1) |
| config `[run]` | `openings_skip` | §2 — `0`, the book's first draw, against `docs/book_v2_ledger.md`'s own empty table |
| config `[run]` | `turn_cap` | §7 — `40`, the standing governed cap, and the dry run's too so the measured shape is the pilot's |
| config `[run]` | `n_workers` | §4D — the number C-D's games-per-hour is reported AT, so a value here that the document does not name would make the throughput unreadable |
| config `[run]` | `hang_timeout_ms` | **checked against the LABEL budget and not the game budget**, because pass 2 reads its watchdog out of the report the game budget wrote (`crates/pistol-arena/src/capture.rs`) and a label ask is the longest single search this pilot makes. The slot pass confirms it exceeds the dry run's measured MAXIMUM label ask with room, and a value that does not is corrected before any game |
| config `[budget]` | `kind`, `value` | §3 — `nodes 50000` |
| config `[sprt]` | `elo0`, `elo1`, `alpha`, `beta` | schema completeness alone. **A self-match crosses no bound** — every pair scores alike, so no likelihood ratio is defined (D-156) — and no strength claim is made here (§0.2) |
| config `[engine_a]`/`[engine_b]` | `label` | §4C — the two MUST differ, because `validate` and `transcript::read` both refuse identical labels; the two seats attesting one ENGINE is a different comparison, over `EngineIdentity`, which carries no label |
| config `[engine_a]`/`[engine_b]` | `binary`, `config` | §1 — one binary and one engine config in both seats |
| config `[engine_a]`/`[engine_b]` | `binary_sha256` | SLOT R2, and it is the one slot that can only be true at launch: a path is not an identity and `target/release/pistol` is a different program after every build (D-147) |
| command | `--label-nodes` | §6 RULE-2 (SLOT S2) |
| command | `--stride` | §6 RULE-3 (SLOT S3) |
| command | `--workers` on `--replay` | §4C — `4`; the pass replays every game with no early stop, so what it finds does not depend on this number |

SLOT P — the table above with its values, as read at launch, and any correction
the pass made.

---

## 10. WHAT THE CLOSURE OWES THIS DOCUMENT

The corpus-size plan for the training package, **labelled ESTIMATED with its
arithmetic shown**, extrapolated from C-D's measured throughput and the standing
label-budget values. **Nothing here registers a corpus size**: that is what the
pilot exists to inform, and a size registered before the measurement would be the
guess this document's whole method is against.
