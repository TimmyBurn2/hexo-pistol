# WP-2.0 — PILOT PRE-REGISTRATION: the label pipeline end to end, on a registered slice

**REVISION 5.** Registered BEFORE the run, at the revision named in §1, under the
WP-2.0 dispatch's Development-round item 4 and the WP-2.0-finish dispatch's §4
(`docs/experiments/wp20_dispatches.md`), and under the operator grant recorded at
`docs/decisions.md` D-552. **THIS IS THE FOURTH AND LAST ROUND THE GRANT ALLOWS**,
it is scoped to the enumerated remedies of
`docs/experiments/wp20_pilot_prereg_REVIEW_rev4.md` (0 BLOCKING, 5 MAJOR,
6 MINOR), and a fifth failure returns the package to the architect.

## 0.1 REVISION HEADER

**NO SECTION OF THIS DOCUMENT HAS BEEN PASSED BY A REVIEWER.** Revision 1 took no
review; revision 2 took one and FAILED it. D-547's passed-section freeze therefore
still binds nothing here, and these tables record the changes for the reader
rather than to discharge the law.

### What moved from revision 4 to revision 5 — one row per finding

**THE RE-CHECK GRADED 11 OF 15 APPLIED AND FOUR AS APPLIED-BUT-INTRODUCED-A-NEW-
DEFECT.** No remedy was skipped and none was partial; **every finding below is a
defect a REMEDY created**, which is the third consecutive round in which that is
the dominant class. It is recorded as the finding about method: in this document,
the danger is no longer that a fix is not made but that making it breaks
something adjacent, and the answer taken here is mechanical — after the edits, a
sweep for every phrase a remedy could have contradicted (§0.2).

| finding | what moved | where |
|---|---|---|
| **MAJ-A** — MIN-8's `book` fix changed a registered instrument in the very commit whose §1 said it changed none, so §1 named a revision whose `corpus-check` could not print a field §4E registers | SLOT R1 stops being a transcribed constant and becomes a slot the SLOT PASS fills, with `git diff --stat` over `crates/ tools/ configs/` as its own check; the dry run is re-taken at the revision that holds the instruments | §1, §7.1, §9 |
| **MAJ-B** — the coldness reconciliation refuted itself: it computed ~2 s of predicted overhead and called that unresolvable by a one-second counter | the reconciliation is redone as QUANTISATION — a difference of two integer readings carries ±2 s — and the conclusion is stated as a BOUND of about 24 ms per ask rather than as an agreement | §7.2 |
| **MAJ-C** — §5's new "if and only if" made a mid-pass filesystem exhaustion a STOP, contradicting the void class that calls it a VOID | exit `2` is classified ONCE into three disjoint limbs, and the environment limb is decided by WHAT failed rather than by WHEN | §5 |
| **MAJ-D** — the enumeration was not exhaustive (the capture's own read, `outpath::claim`'s other failures, argument refusals) and §5 carried two lists of one closed class | the enumeration is completed and limb (b) is DEFINED as its complement, so there is one list | §5 |
| **MAJ-E** — §8's registered block emits no revision or engine receipt, so the pilot's artifact would carry the very defect that superseded its predecessor | the block opens with `git rev-parse HEAD`, the dirty-file count and the engine digest, unconditionally | §8 |
| minors | the second transform is timed; §6.3 stops calling the transform term MEASURED; the manifest's *"the dry run is `tools/`-free"* is corrected (§8 runs `tools/cold_label_check.py`); §9's watchdog ground is restated as a mean and a bound rather than a "measured maximum"; the two C-E injection corpora are indexed; the "three decided games" arithmetic is corrected — a game decided at turn `k` contributes `k` positions, so the exposure is to WHEN games decide | §6.3, §7.1, §8, §9, the manifest |

### What moved from revision 3 to revision 4 — one row per finding of the scoped re-review

**THE RE-REVIEW GRADED REVISION 3 AS 9 OF 14 APPLIED, 2 PARTIAL, 2 APPLIED-WITH-A-
NEW-DEFECT AND 1 THAT INTRODUCED ITS OWN.** Three of its five MAJORs are one
class — **a remedy that fixed a sentence and left its contradiction standing
somewhere else in the same document** — and that is recorded here rather than
spread across three rows, because it is the finding about method.

| finding | what moved | where |
|---|---|---|
| **MAJ-1** — §5's V7-B discriminator ("whether any output was produced") does not exist: neither pass prints before success and the partial output is deleted | the discriminator becomes an EXHAUSTIVE ENUMERATION of the pre-work refusals; anything else at exit `2` is V7-B | §5 |
| **MAJ-2** — the retracted totality claim was still standing verbatim, so §6.1 asserted a proposition and its negation | the claim is corrected AT ITS HEAD, where it was made, and paragraph (ii) records that revision 3's remedy wrote the denial and left the assertion | §6.1 |
| **MAJ-3** — §2, which owns the slice, still stated the pre-amendment RULE-1, which returns 56 | §2 stops restating the rule and points at §6.1; a size stated twice is a size that can drift, and it did | §2 |
| **MAJ-4** — the remedy that re-measured `c` SPENT a claim true in revision 2, leaving "the coldness cost is 2.4 %" standing as a finding of a superseded run | both readings are kept with the budget and artifact each belongs to, and the arithmetic showing they do not disagree is given | §7.2 |
| **MAJ-5** — the item-9 guard landed with no test, in the round whose §8 invokes D-553 by name | `a_corpus_path_carrying_a_control_character_is_a_void_before_it_is_printed` drives the binary over a path holding a newline | the tests |
| **MIN-1** — M4's remedy reached §7.2 and the instrument but not §4E, the criterion | §4E run 1 registers the summary line and states that C-E's reach is whatever it reports | §4E |
| **MIN-2** — C-D named `date +%s.%N`, an instrument §8 rules out and the block does not contain | C-D names `SECONDS` | §4D |
| **MIN-3** — "four measured costs" is six, and one of them read `0` | six, with the transform stated as an upper bound rather than a positive measurement | §6, §6.3, §7 |
| **MIN-4** — §8's block had no timing brackets for replay or the transforms, so the pilot would inherit the stand-in's numbers | both brackets added to the registered block | §8 |
| **MIN-5** — the deviation's cost was overstated 2x and both dispatch quotations were inexact | 32.5 %, and the two dispatches quoted separately because they are not the same sentence | §4B |
| **MIN-6** — the ledger row cited a superseded revision | the row cites revision 4 | the ledger |
| **MIN-7** — floor (b) is applied to a quantity `p = 41` only bounds from above | the 6 % slack is stated, and the closure reports the actual asked-position count beside the rate | §6.1 |
| **MIN-8** — the summary omitted `book`, one of the loader's four token sets | `book` added, with a test that the four token sets are all reported | the instrument |
| **MIN-9** — SLOT A was gitignored and sha-indexed nowhere committed | `docs/experiments/wp20_pilot_artifacts.md`, a committed manifest under rule 8 | a new file |
| **MIN-10** — "zero forfeits" was read from an absence, which §8 forbids | a nonzero count off the CONDITIONAL clause; zero off the completed pass's exit code | §4C |

### What moved from revision 2 to revision 3 — one row per review finding

| finding | what moved | where |
|---|---|---|
| **B1** — a pass-1 run FAILURE was registered as `V2 — PASS WITH A FINDING`, because `arena --config` returns `1` from two disjoint branches and §4C read the code as if it had one meaning | the criterion is read from TWO receipts — whether the `VERDICT` line printed, and the forfeit count off the summary block — never from the exit status; a died pass 1 becomes its own STOP row | §4C, §5 |
| **M1** — RULE-2's depth table cited no artifact and named no instrument, while fixing 65 % of the pilot's wall | the depth distribution is now printed by `corpus-check` itself, from the corpus's own `depth_turns` column, with a test driving the shipped program; the sweep is re-run and cited by digest | §1, §6.3, §7.1 |
| **M2** — the void class said "and nothing else" and omitted a pass failing part-way, which also exits `2` | the class is re-stated with the before-any-work qualifier, and a mid-walk failure becomes STOP row V7-B | §5 |
| **M3** — RULE-1's floor (a) carried a ground the dry run's own output falsifies, and the section claimed both rules were total when RULE-1 is not | the false ground is withdrawn, the totality claim is retracted, and floor (b)'s sensitivity is tabulated | §6.1 |
| **M4** — §7.2 exempted finding 1 from this pilot, but C-E is a pilot criterion that finding 1 narrows | the exemption is withdrawn for finding 1 and the narrowness is put on the run's own face by the summary line | §7.2 |
| **M5** — §8 claimed both loader injections were pinned against the shipped binary; the digest one was pinned by a direct call | the claim is corrected AND the gap closed with a binary-driven test — this document's own instance of D-553 | §8 |
| **M6** — the dry run ran against a working-tree edit, at a revision whose committed config carried a placeholder digest | the dry run is RE-RUN at a committed revision and re-cited | §1, §7.1 |
| **M7** — `docs/book_v2_ledger.md` asserted RULE-1 "was registered before those costs existed", contradicting §6.1's disclosure | the ledger row now carries the amendment and points at §6.1 | the ledger |
| minors | the replay and transform wall terms are measured rather than asserted; §7's extrapolation ground is restated as `p = turn_cap + 1`; C-B's whole-range deviation from both dispatches' "sub-range" is named as a deviation; the forfeit relaxation against the dispatch's DONE line is named; C-B is read from `cmp`'s exit code rather than from two digests compared by eye; `corpus-check` gains the item-9 path guard its sibling had | §3, §4B, §4C, §6.1, §7, §8 |

### What moved from revision 1 to revision 2

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
| the corpus loader, **and the instrument RULE-2's depth table is read from** | `crates/pistol-arena/src/bin/corpus-check.rs` | SLOT R1 |
| the cold-label referent | `tools/cold_label_check.py` | SLOT R1 |
| the engine | `target/release/pistol`, bound by content | `180b4c40…` (`binary_sha256`), §9 |
| the engine config | `configs/instrument_v0.toml` | SLOT R1 |
| the arena config | `configs/arena_wp20_label_pilot.toml` | SLOT R1 |
| the dry-run arena config | `configs/arena_wp20_label_pilot_dryrun.toml` | SLOT R1 |
| the book | `crates/pistol-cli/tests/fixtures/random_openings_v2.txt` | SLOT R1 |

**SLOT R1 IS ONE VALUE: the commit that holds every artefact in the table above.**
It is filled by the SLOT PASS, from the revision the run is actually taken at, and
**this document has already got it wrong once in the way this rule exists to
prevent**: revision 4 filled it with `85e6261` and then, in the very commit
asserting that, changed `crates/pistol-arena/src/bin/corpus-check.rs` — an
instrument in the table — so §1 named a revision whose `corpus-check` could not
print a field §4E registers. **The rule fired on the document that wrote it.**

**THE RULE, RESTATED SO IT BINDS THE SLOT PASS AND NOT ONLY A READER**: SLOT R1 is
correct only if `git diff --stat <SLOT R1> <the run's own HEAD> -- crates/ tools/
configs/` is EMPTY. The slot pass runs that command and records its output, and a
non-empty result is not a note to add — it means SLOT R1 is the wrong commit and
the dry run must be re-taken at the right one. A change to any artefact in the
table reopens this document (`docs/process.md`, "Instrument governing revision"). Every artefact above except
the engine binary is a tracked file, so naming the commit names all of them at
once, and a change to any of them reopens this document
(`docs/process.md`, "Instrument governing revision"). **The review of this pre-registration is
dispatched against the revision that GOVERNS the run, and a review of a superseded
revision does not transfer** (`CLAUDE.md`, Process).

**SLOT R2 IS FILLED AT THE SLOT PASS AND NOWHERE ELSE**: it is the digest of
`target/release/pistol` as built for the run, and a path is not an identity
(D-147).

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
draw. **`openings_skip = 0`, `openings_take = 13`** (SLOT S1, fixed by §6.3's RULE-1) — a prefix from the top,
which is a sample because the book is emitted in generator order with no
provenance column to sort by.

**THE SIZE IS DERIVED FROM A RULE, AND THE RULE IS §6.1's RULE-1 — WHICH THIS
SECTION DOES NOT RESTATE.** A slice size stated twice is a slice size that can
drift, and it did: revision 3 amended RULE-1 in §6.1 and left this paragraph
carrying the superseded form, which returns 56 rather than 13. **The rule lives in
§6.1 and its answer in §6.3; this section owns the RANGE and points at them.**
Revision 1 chose eight and defended it with *"leaves the run inside a single
sitting"*, which is a guess about wall time standing where a measurement belongs
(D-500's class), and that is what the rule replaced.

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
| LABEL | `nodes 400000` (SLOT S2, §6.3 RULE-2) | the label is the re-scored answer the corpus exists to hold; a label at the game's own budget would be the game's own answer with a `newgame` in front of it. Its VALUE is fixed by §6's RULE-2, which is registered below and before the dry run measures anything |

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

**THE REGISTERED SAMPLE.** `--stride 1` (SLOT S3, §6.3 RULE-3), which the script prints back on
its own first line as *"the sample is every record whose zero-based index is a
multiple of `<n>`, which is `<k>` of them"*. **The alignment hazard is named
because a stride is a systematic sample**: the capture's records are ordered
game-major and turn-minor, so a stride equal to (or dividing) the number of
records a game contributes would sample one turn index of every game and nothing
else — the empty board over and over. SLOT S3 is fixed in §6.3 against the
per-game record counts the dry run measured, and the budget affords the value it
returns: **1**, which is every record and has no alignment to have.

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
byte-identical. Both comparisons are **`cmp -s`, read from its EXIT CODE** —
`0` identical, `1` differing, `2` a file it could not read — and not from two
`sha256sum` lines a reader compares by eye. §8's own rule is that a criterion is
read from a code; a pair of printed digests is a criterion read by a human, which
is the weaker instrument and the one this document was using. The digests are
printed as well, because they are what a later reader cites.

**THIS IS A DEVIATION FROM BOTH DISPATCHES AND IS NAMED AS ONE.** The WP-2.0
dispatch asks for *"the re-run determinism receipt on a sub-range"* and the
WP-2.0-finish dispatch for *"the determinism re-run receipt on a sub-range"*
(`docs/experiments/wp20_dispatches.md`, quoted separately because they are not
the same sentence); this document registers the WHOLE range instead, which is
strictly stronger and costs the SECOND capture pass — `82.5 T` of §6.3's
`253.5 T`, or **32.5 % of the pilot's wall**. (The 65 % figure elsewhere in this
document is the share of BOTH capture passes together, which is a different
quantity.) The deviation is taken because the two
passes have no subsetting flag and the only way to make a sub-range would be to
edit a report, which changes its `source_sha256` and so changes the question.

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

**ZERO FORFEITS, AND WHY THE EXIT CODE ALONE CANNOT BE THE RECEIPT.**
`crates/pistol-arena/src/bin/arena.rs` returns `1` from **two disjoint
branches**, and its own usage block says so — *"1 abandoned **or** forfeited
(report still written)"*. One branch is a completed match with a nonzero forfeit
count; the other is a run that DIED, whose stderr calls the partial report *"a
diagnostic, not as a sample"*. **A criterion that read `1` as "forfeits" would
read an aborted pilot as a completed one with a finding**, and the two have
opposite consequences.

So the criterion is read from **two receipts, not one**:

1. **Did pass 1 complete?** `summary::render`'s block is printed only on the
   completing branch, so the run's stdout carrying the `VERDICT` line is what
   says the match finished. **A pass 1 that died is verdict V7-A of §5 and STOPS
   the arc**; its report is not captured, because a capture over a partial report
   would be a corpus over a sample nobody registered.
2. **How many forfeits?** A NONZERO count is read off that same printed summary
   block, whose `CONDITIONAL: <n> forfeited game(s) excluded` clause
   (`crates/pistol-arena/src/summary.rs`) fires only when the count is nonzero.
   **ZERO forfeits is read from the exit code of a pass 1 that completed**, which
   is `0` if and only if the tally is zero — receipt 1 having already established
   that the pass completed, which is what makes the code readable here. It is
   stated this way because the block says NOTHING about forfeits at zero, and a
   criterion read from an absence is the thing §8 forbids by name.

**A NONZERO FORFEIT COUNT ON A COMPLETED PASS 1 DOES NOT STOP THE ARC** — pass 2
captures a forfeited game's positions like any other, which is D-544's own
recorded decision and is safe because `transcript::read` legality-checks every
game before pass 2 exists — but it is reported as a FINDING rather than a
footnote, because a self-match of one deterministic engine against itself has no
protocol reason to forfeit. **This is a RELAXATION of the WP-2.0 dispatch's DONE
line "zero forfeits", and it is named as one rather than left to be noticed**:
the dispatch lists zero forfeits among the pilot's receipts, and this document
registers a nonzero count as a reported finding instead of a stop, on the ground
that D-544 already settled that forfeited games are capturable.

**THE DEFECT CLASS.** A report whose recorded games are not the games its engines
play, which would make every label a label of a position that never occurred.

### C-D — throughput, MEASURED

**THE INSTRUMENT.** The `SECONDS` brackets around each pass in §8's command block
— bash's own integer counter, which is what §8 registers and the only timer that
block contains — and the report's own `wall_ms` record for pass 1 as a second
reading of the same quantity.

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
   everything would pass runs 2 and 3 and answer nothing.* **AND THE RUN RECORDS
   ITS OWN REACH**: the `corpus_check:   depth_turns … ; score_kind …; to_move …;
   book …; result …; end …` line names how many values each closed-set column
   actually took, and the closure quotes it. **C-E's reach is whatever that line
   says and no more** — a column the pilot's games drove to one value has had one
   arm of its guard exercised, and §7.2's finding 1 measured exactly that on the
   stand-in, where `result` and `end` each held a single value.
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
| V2 | **PASS WITH A FINDING** | as V1, except a nonzero forfeit count on a pass 1 that COMPLETED | as V1, and the forfeit count and its games are named in the closure |
| V3 | **STOP — cold-label mismatch** | C-A exit 1 | the arc STOPS; artifacts preserved; the disagreeing records are the finding |
| V4 | **STOP — determinism** | C-B: a digest differs | the arc STOPS; both outputs preserved |
| V5 | **STOP — replay** | C-C: exit 1, or a nonzero divergence count, or `covered != total` | the arc STOPS; the replay document is the diagnostic |
| V6 | **STOP — schema** | C-E: any of the three runs answers other than as registered | the arc STOPS; the corpus and the two injections preserved |
| V7-A | **STOP — pass 1 died** | pass 1 exits `1` with **no `VERDICT` line on stdout**, so the report is a partial one the program itself calls a diagnostic | the arc STOPS. The partial report is preserved and is NOT captured: a capture over it would be a corpus over a sample nobody registered, and C-C would pass on it because `covered == total` over the games that are there |
| V7-B | **STOP — a pass died after pass 1** | `--capture` or `--labels` exits `2` **after work had begun** — a channel failing mid-walk, an engine refusing an ask, a record refused on write | the arc STOPS. This is a refusal BY the pipeline about its own inputs and is not a void; §5's void class excludes it by name |
| V8 | **VOID** | see below | ONE re-run, on a receipted environment fault; a SECOND void STOPS with artifacts preserved |

**THE VOID CLASS, DEFINED — AND EXIT `2` CLASSIFIED ONCE, HERE, WITH EVERY OTHER
SECTION POINTING AT THIS LIST** (D-423). A void is *no answer was taken*, never
*the answer is no* (`tools/SHELL_CHECKLIST.md` item 12). **An exit `2` from any
instrument of this pilot is exactly one of three things**, and the three are
disjoint and exhaust the code:

**(a) THE PROGRAM DECLINED TO LOOK AT A DOCUMENT — a VOID.** Every refusal decided
BEFORE the first ask, enumerated so that (b) can be defined as its complement:

- the command line itself — a flag order, or a count spelled a way the program
  will not echo back (`crates/pistol-arena/src/bin/arena.rs`);
- the `--out` path could not be claimed exclusively, for ANY reason and not only
  "already claimed" (`crates/pistol-arena/src/outpath.rs`);
- the source report could not be read as one: absent, not a regular file, not
  UTF-8 (`crates/pistol-arena/src/passes.rs`);
- the report is not one this build reads — wrong schema, aborted, a budget that is
  not `nodes`, two seats carrying one label, a record it refuses
  (`crates/pistol-arena/src/transcript.rs`);
- the two seats do not attest one engine, or an engine's digest has drifted since
  the report was written (`capture.rs`'s `one_engine`, `replay::verify_engines`);
- **for `--labels` only** — the capture file could not be read, or its grammar was
  refused, or its `source_sha256` or `capture_sha256` does not bind to the report
  (`passes.rs`, `crates/pistol-arena/src/capture_file.rs`, `labels.rs`'s two
  header checks). All four are decided before the first record is transformed;
- `cold_label_check.py` or `corpus-check` refusing to look — a path that is not a
  file, a path carrying a control character, a stride spelled unusably, a capture
  whose body does not digest to its header.

**(b) THE PROGRAM REFUSED SOMETHING IT FOUND MID-WALK — a STOP, verdict V7-B.**
The complement of (a) among the program's own refusals: an engine that spoke out
of turn, closed its pipe, wrote an unreadable line or answered `error`; a totals
line the normalisation cannot read; a record the write side refused. **This is
defined as a complement and not as a second list, because §5 previously carried
two lists of one closed class and they disagreed.**

**(c) THE MACHINE TOOK THE RUN AWAY — a VOID, and it is NOT decided by the
enumeration.** A filesystem filling, a process killed, a reboot, the session
ending mid-pass. `/tmp` on this machine is a 24 GiB RAM-backed tmpfs and its
exhaustion is the recorded instance (D-281, D-285). **This limb is independent of
where the failure lands**: an `ArenaError::Io` naming a failed read or write is
the environment, not a refusal about a document, and it is a void whether it
arrives before the first ask or during the walk. Revision 4 made (b) the
complement of (a) alone, which classified a mid-pass ENOSPC as a STOP and
contradicted this limb; the two are separated here by WHAT FAILED — an I/O error
against the machine, a refusal against a document — and not by WHEN.

**THE TWO EXCLUSIONS THAT COST THIS DOCUMENT A REVIEW ROUND, named so they cannot
be read back in.** (i) `arena` also exits `2` when a pass fails PART-WAY — the
same code, a different meaning — because `dispatch` maps every `ArenaError` to one
status (`crates/pistol-arena/src/bin/arena.rs`). That is the pipeline refusing its
own input and it is **V7-B, a STOP**, not a void. (ii) A criterion answering "no"
is V3-V6, and a run slower than §6 estimated is neither — a wall estimate is not a
criterion and cannot be failed.

**HOW V7-B IS TOLD FROM A VOID, and the first answer this document gave was
wrong.** Revision 3 said the two are separated by *"whether any output was
produced before the refusal"*. **They are not**: `crates/pistol-arena/src/capture.rs`
and `crates/pistol-arena/src/labels.rs` contain no print statement at all — every
line a pass prints is in `crates/pistol-arena/src/passes.rs` AFTER the walk has
succeeded — and `outpath::abandon` removes the claimed output on any error, so a
mid-walk failure and a pre-work refusal look identical in stdout and on disk.
**The discriminator is the classification of exit `2` above**, whose limb (a) is
an exhaustive enumeration of the refusals decided before the first ask, whose
limb (b) is defined as its complement, and whose limb (c) is the environment and
is decided by what failed rather than by when. **A refusal a reader cannot place
in that classification is itself a finding about this document.**

**A void is receipted before it is re-run**: the receipt names the filesystem, the
process or the signal, so that "I could not look" is distinguishable in the record
from "the pipeline is wrong". A void with no receipt is a STOP.

---

## 6. THE WALL ESTIMATE, DERIVED — AND THE SLOTS IT FIXES

**Derived from §7's dry run, which is a cited timing artifact and not a guess**
(D-500's class). The dry run measures six per-unit costs on this machine at these
budgets, and this section shows the arithmetic that turns them into SLOT S1
(`openings_take`), SLOT S2 (the label budget) and SLOT S3 (the cold-label stride).

### 6.1 THE TWO RULES, REGISTERED BEFORE THE DRY RUN MEASURES ANYTHING

This is `book_v2`'s own discipline applied to a pilot instead of a book — D-518's
*"the decision rule registered before the sweep, the sweep before the size"* — and
it exists so that no number below can be chosen after seeing which number would
be convenient. **RULE-2 and RULE-3 are total: applied to §7's measurements they
leave no free choice. RULE-1 is NOT, and §6.1's own paragraph (ii) says why and
tabulates the sensitivity** — a document that claimed totality for all three
would be claiming it for the one rule whose answer moves with a judgement.

**THE WALL BUDGET, stated as the constraint it is.** The pilot's total machine
time is **at most four hours**, counting every pass this document registers —
pass 1, BOTH capture passes, both corpus transforms, the replay and the
cold-label check. This is an operator-attention and session-capacity constraint
and is honestly labelled one: it is not derived from anything about the pipeline.
**Measured, it binds nothing** — RULE-1's own sensitivity table shows every
candidate floor fitting inside it — so it is a ceiling that has not had to
refuse anything, and the document says that rather than letting its presence
imply a constraint it did not exert.

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

**RULE-1 — the SLICE (SLOT S1). AMENDED AFTER THE DRY RUN, AND THE AMENDMENT IS
DISCLOSED HERE RATHER THAN SMOOTHED OVER, BECAUSE A RULE CHANGED AFTER NUMBERS
ARRIVE IS THE EXACT MOVE THIS SECTION'S DISCIPLINE EXISTS TO CATCH.**

*What it said before the dry run:* **the LARGEST `openings_take` whose derived
wall is at or under the four-hour budget**, floor 4 openings.

*What happened:* pass 1 of the dry run returned, and applying that rule to its
numbers gives **T = 56** — a four-hour pilot producing 4 592 labelled positions
that D-539 says *"count toward no minimum"*. Reading that, the rule is wrong on
its face: it maximises against the CLOCK when every criterion in §4 is satisfied
by a far smaller draw, and it spends the session's remaining capacity on data the
package itself declares is not corpus. **I did not notice this until the numbers
made it concrete, and that is the honest account.**

*What it says now:* SLOT S1 is **the SMALLEST `openings_take` satisfying all
three of** — (a) at least **4** openings; (b) at least **1 000 asked
positions**, so that C-D's labels-per-hour is a rate over a thousand searches
rather than a handful, and C-A at stride 1 is a total check over a sample of that
size; (c) a derived wall at or under the four-hour ceiling. **If (a) and (b)
cannot both fit under (c) at SLOT S2, SLOT S2 drops to the next smaller candidate
and RULE-1 is applied again**; if they do not fit at `100000` either, that is a
STOP and the pilot returns to the architect.

**THE FIRST REVIEW ROUND FOUND TWO THINGS WRONG WITH THAT PARAGRAPH AND BOTH ARE
CORRECTED HERE RATHER THAN DEFENDED.**

**(i) FLOOR (a)'s STATED GROUND WAS FALSE.** It read *"the floor at which the
report's own pairing is exercised over more than one pair"*, and the dry run's
own line 5 falsifies it: at `openings_take = 2` the report prints `(2 pairs)`.
Two openings already give two pairs. **The ground is withdrawn. Floor (a) is
kept, at 4, as a floor with no derivation** — it is a judgement that a shakedown
should span more than a couple of openings, and it binds nothing here because
floor (b) dominates it at every candidate budget.

**(ii) THE RULE IS NOT TOTAL.** Revision 2 said *"Both rules are total: applied
to §7's measurements they leave no free choice"* — **true of RULE-2 and RULE-3,
FALSE of RULE-1**, because floor (b)'s value is a judgement and the answer moves
with it. The claim is corrected where it was made, at the head of §6.1, and not
only denied down here: **revision 3's first attempt at this remedy wrote the
denial and left the assertion standing, so the section asserted a proposition and
its negation** — which is the defect the remedy was for, reproduced by the
remedy. The sensitivity is put on the document's face instead of being implied
away:

| floor (b) | smallest `T` | asked positions | derived wall | inside the ceiling? |
|---|---|---|---|---|
| 500 positions | 7 | 574 | ~30 min | yes |
| **1 000 positions** | **13** | **1 066** | **~55 min** | **yes** |
| 2 000 positions | 25 | 2 050 | ~1 h 46 | yes |
| (the ceiling itself) | 56 | 4 592 | ~3 h 57 | the only row it refuses is 57 |

**FLOOR (b) IS APPLIED TO AN UPPER BOUND, DELIBERATELY, AND THE SLACK IS 6 %.**
`2T x 41` counts a CAPPED game's positions, and §6.3 records that `p = 41` bounds
from above; at `T = 13` the floor is met with 1 066 against 1 000, a slack of 66
positions. **How many decided games it takes to spend that slack is NOT a fixed
number**, and revision 4's "three" was wrong: `asked_prefixes` drops only the
terminal position, so a game decided at turn `k` contributes `k` positions and not
`41` — **one game decided at turn 20 costs 21 positions, and four such games spend
the whole slack, while four games decided at turn 39 cost eight.** The exposure is
to WHEN games decide and not to how many do. That is accepted rather
than guarded, on the ground that floor (b) is not a criterion — it fixes a size
BEFORE the run and cannot be re-read after it without becoming the after-the-
numbers decision §6.1 exists to forbid. **The closure reports the pilot's ACTUAL
asked-position count beside the rate**, so a reader can see whether the thousand
was reached; a rate over 900 searches is not thereby wrong, it is a rate over 900.

**And the four-hour ceiling constrains nothing at any of these floors, so floor
(b) does all the work.** The document does not pretend otherwise. **1 000 is
chosen and not derived**, on the stated ground that a throughput rate quoted to a
corpus plan should rest on at least a thousand searches; a reader who thinks 500
or 2 000 is the better number is disagreeing with a judgement, not catching an
error, and the table is here so that disagreement is cheap to act on.

**WHAT IS OFFERED AGAINST THE SUSPICION THAT THE AMENDMENT IS CONVENIENCE**, now
that the weaker of the two original defences has been withdrawn: **RULE-2, which
was NOT amended, fired against convenience** — it selected `400000`, the most
expensive of its three candidates and 65 % of the pilot's wall, and it was left
where it fired. That is evidence about this document's method and it is not
proof about this rule.

**AND THE ONE THING FLOOR (b) DOES NOT WEIGH, which §7.2's finding 1 raises**:
game SHAPE. Every dry-run game reached the turn cap undecided, so a larger draw
might buy a decided game and with it the only `result` value the corpus has never
carried. **RULE-1 weights that at zero and the closure is told so** (§10), because
a floor set on position COUNT cannot reach a property of the game DISTRIBUTION,
and inventing a coverage floor after seeing which coverage was missing is the
move this whole section exists to forbid.

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

### 6.3 SLOT W — FILLED, from SLOT A (the dry-run artifact named in §7.1)

**THE SIX MEASURED COSTS**, every one from that artifact's own printed lines
over 4 games and 164 asked positions:

| symbol | measured | from |
|---|---|---|
| `g` | **1.5 s** per game at `n_workers = 4` | `pass1 seconds=6` over 4 games |
| `p` | **41** positions per game | `captured 164 position(s) from 4 game(s)` |
| `l` at `400000` | **1.006 s** per label ask | `capture_400000 seconds=165` over 164 |
| `c` at `400000` | **1.006 s** per cold ask | `cold seconds=165` over 164, **at the chosen budget** |
| replay | **1.5 s** per game at `n_workers = 4` | `replay seconds=6` over 4 games |
| a corpus transform | **under 1 s**, charged as 1 s | `labels-transform seconds=0` — a BOUND, not a measurement: the transform finished inside this instrument's one-second resolution |

**RULE-2, APPLIED AS WRITTEN, AND ITS INSTRUMENT IS NAMED.** The depth
distribution is not computed by this document: each candidate's capture is turned
into a corpus and read back by `crates/pistol-arena/src/bin/corpus-check.rs`,
which prints it from the corpus's own `depth_turns` column. Every row below is a
`corpus_check:   depth_turns median … mean …` line in SLOT A, quoted:

| label budget | median `depth_turns` | mean | satisfies "at least one turn above the game budget's 3.0"? |
|---|---|---|---|
| `50000` (the GAME budget) | **3.0** | 2.7195 | — it is the referent |
| `100000` | 3.0 | 3.0366 | **no** |
| `200000` | 3.0 | 3.3049 | **no** |
| `400000` | **4.0** | 3.6341 | **yes** |

**THAT LINE IS A RECORDED NUMBER AND SO CARRIES A TEST DRIVING THE SHIPPED
PROGRAM** (`tools/SHELL_CHECKLIST.md` item 10):
`a_corpus_summarises_the_columns_a_pre_registration_reads_a_number_off` in
`crates/pistol-arena/tests/labels_tests.rs` re-derives the median from the
corpus's own column and compares — the referent is the file, not the program's
own earlier output.

**SLOT S2 = `400000`.** The rule selects the smallest candidate that satisfies
it, and that is the LARGEST of the three — the most expensive answer available,
which is recorded because a rule that only ever returns the cheap option is a
rule worth doubting. The mean rises smoothly across all four budgets while the
median steps only at `400000`; **RULE-2 named the median and the median is what
was read.**

**`c` IS MEASURED AT THE CHOSEN BUDGET AND IS NO LONGER DERIVED.** Revision 2
measured the cold check at `200000` and extrapolated; this revision runs it at
`400000`, the budget RULE-2 selected, and the two costs are **1.006 s each** —
`capture_400000 seconds=165` and `cold seconds=165`, over the same 164 positions.
**The coldness overhead is therefore below this instrument's one-second
resolution**, which is a MEASURED answer to the cost
`docs/experiments/wp20m_design.md` §12 declines to guess and D-542 recorded as
*"a 256 MiB memset at every committed seat's `tt_bytes`"*: beside a search of this
size it does not register. (The earlier `200000` reading put it at 2.4 %; both are
in SLOT A's predecessor and neither changes a value here.)

**THE ARITHMETIC, at `T` openings** (`2T` games, `2T x 41` positions):

```
pass 1                2T * 1.5                        =    3.0 T
two capture passes    2 * (2T * 41 * 1.006)           =  165.0 T
cold check, stride 1  (2T * 41) * 1.006               =   82.5 T
replay (C-C)          2T * 1.5, MEASURED not assumed  =    3.0 T
two corpus transforms BOUNDED at under a second each  =    2   s
                                                        ---------
                                                         253.5 T + 2 s
```

**Every term is timed rather than asserted.** Revision 2 asserted the replay and
transform terms from their shape; SLOT A times both. **Five of the six are
positive measurements and the transform is a bound**, which §6.3's table and §7's
criterion both say — a term that reads `0` on a one-second counter is not a
measurement of a positive quantity.

**RULE-1, APPLIED AS AMENDED.** (b) needs `2T x 41 >= 1000`, so `T >= 12.2` and
the smallest integer is **13**; (a) is satisfied at 13; (c) gives
`253.5 x 13 + 2 = 3 297 s`, about **55 minutes**, well under the ceiling.
**SLOT S1 = 13** — 26 games, 1 066 asked positions. *(The rule as it stood before
the amendment would have returned 56, the largest `T` with
`253.5T + 2 <= 14 400`.)*

**RULE-3, APPLIED AS WRITTEN.** The stride-1 wall is inside the ceiling, so
**SLOT S3 = 1** — every record. The alignment hazard RULE-3 guards therefore does
not arise; it is recorded that it would not have arisen anyway, because every
game of the dry run contributed exactly **41** records and a stride of 1 has no
alignment to have.

**THE ESTIMATE ERRS THE SAFE WAY, and the reason is named.** `p = 41` is an
UPPER bound: it is the count for a game that reaches the turn cap, and all four
dry-run games did. A game that ends in a win contributes FEWER asked positions
(rule 4's terminal position is never asked), so the true wall is at or below the
derived one. **Whether the pilot's own games are also all capped is not known
from the dry run** — `random_openings_v2`'s openings are drawn at random where
`openings_v1`'s come from human games — and §10 records what follows if they are
not.

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
a pilot game. §6's arithmetic is stated per POSITION, and the positions a
capped game contributes are `turn_cap + 1` — which is a function of the CAP and
not of the opening's length, so it is 41 under both books. That, and not the
absence of a per-game constant, is what makes the extrapolation survive the
difference; §6.3 uses `p = 41` for exactly this reason and records that it is an
upper bound because a DECIDED game contributes fewer.

**IT CONSUMES NO `book_v2` RANGE**, so it is not a draw the ledger records, and it
is not the pilot's first run.

**THE DRY RUN'S OWN CRITERION.** Every command in §8 runs to its registered exit
code on the stand-in, and the six per-unit costs of §6 come back with the counts
they are derived from — five as finite positive numbers, and the corpus transform
as an upper bound, because a pass that finishes inside a one-second timer's
resolution reads `0` and `0` is not a measurement of a positive quantity.

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

### 7.1 SLOT D — FILLED

**THE RECORDED INPUT.** `configs/arena_wp20_label_pilot_dryrun.toml` at the
revision this document lands in: `openings_v1.txt`, `openings_take = 2`,
`openings_skip = 0`, `turn_cap = 40`, `n_workers = 4`, `hang_timeout_ms =
120000`, budget `nodes 50000`, both seats `target/release/pistol` with
`configs/instrument_v0.toml`.

**SLOT A = `artifacts/wp20pilot_dryrun_85e6261_v1.txt`.**

**IT RAN AT A COMMITTED REVISION AGAINST COMMITTED BYTES, AND ITS FIRST TWO LINES
ARE THE RECEIPT FOR THAT.** `revision 85e62613c358b105adfb5d068c5ca10084d24c38
tree 0 modified`, and `engine sha256 180b4c40…`, which is the digest
`configs/arena_wp20_label_pilot_dryrun.toml` carries at that same commit.
**REVISION 2's DRY RUN DID NOT**: it ran at `f297eab`, whose committed dry-run
config carried a placeholder digest `crates/pistol-arena/src/identity.rs` refuses,
so it in fact ran against an uncommitted working-tree edit and its artifact's name
attributed it to a revision that could not have produced it. That artifact is
superseded, this one replaces it, and the defect is named because it is D-479's
class — a measured number is bound to the run that produced it.

Every command of §8 ran to its registered exit code:

| command | exit | what it printed |
|---|---|---|
| pass 1 | `0` | `n 4  distinct-n 2  (2 duplicate games)`, `VERDICT inconclusive_degenerate`, `pass1 seconds=6` |
| capture + labels + load, x4 (the RULE-2 sweep) | all `0` | `captured 164 position(s) from 4 game(s)` at each candidate, then the four `depth_turns median` lines §6.3 tabulates |
| capture re-run (C-B) | `0`, and **`capture-determinism exit=0`** from `cmp -s` | both files `5fe1f1a36bef97d05679807c06df1efe85245ccd51362c6c670b5943ea95af20` |
| labels re-run (C-B) | `0`, and **`labels-determinism exit=0`** | — |
| cold check (C-A) | `0` | `164 of 164 sampled record(s) agree byte for byte`, at `go nodes 400000` |
| replay (C-C) | `0` | `replayed 4 of 4 game(s) ... 0 divergence(s)`, `replay seconds=6` |
| corpus-check, control (C-E 1) | `0` | `ok, 164 record(s)`, and the summary line naming `result 1 (capped); end 1 (normal)` |
| corpus-check, grammar injection (C-E 2) | `1` | `REFUSED: ... record 1: `key_pos` is not thirty-two hex digits` |
| corpus-check, digest injection (C-E 3) | `1` | `REFUSED: ... its body digests to ffc96a13... and its header claims 532a23fb...` |

**THE DRY RUN'S CRITERION IS MET**: every command ran to its registered code, and
§6's six per-unit costs came back with the counts they are derived from —
including the two revision 2 asserted rather than timed. **Five are positive
measurements; the corpus transform read `0` and is carried as an upper bound of
1 s**, which is stated rather than rounded into the positive column.

### 7.2 THREE MEASURED FINDINGS THE DRY RUN PRODUCED THAT THIS DOCUMENT DID NOT ASK FOR

Recorded here because they bound what the closure may conclude, and because a dry
run that only confirmed what was expected would be worth less.

1. **EVERY GAME WAS CAPPED AND NONE WAS DECIDED.** All four dry-run games reached
   `turn_cap = 40` undecided, so the corpus's `result` column is `capped` on all
   164 records and `end` is `normal` on all 164. **Two of the loader's four
   token-set columns are therefore exercised at one value each**, and the loader
   now PRINTS that — `result 1 (capped); end 1 (normal); to_move 2 (p1,p2);
   score_kind 3 (eval,mate_in,mated_in)` — so the narrowness is on the run's own
   face and no reader has to take this paragraph's word for it. C-E's control is
   real and narrower than the schema, and §4E's run 1 registers that its reach is
   whatever the line reports.
2. **A SELF-MATCH OF ONE DETERMINISTIC ENGINE PLAYS EACH OPENING TWICE AND GETS
   THE SAME GAME**: `distinct-n 2` of `n 4`. The capture walks every game in the
   report, so **a corpus built this way carries each position twice.** This is
   not a defect and cannot be removed by configuration: `arena --capture` refuses
   a report whose two seats do not attest one engine, so a self-match is the only
   shape a capture can be taken from at all. It is arithmetic the closure's
   corpus plan must carry, not a finding against the pipeline.
3. **THE COLDNESS COST DOES NOT REGISTER AT THE CHOSEN BUDGET, AND BOTH READINGS
   ARE KEPT WITH THE BUDGET AND ARTIFACT EACH BELONGS TO.** At `nodes 200000`,
   measured in the SUPERSEDED dry run, the cold ask cost 0.518 s against the
   in-process ask's 0.506 s — **2.4 %**. At `nodes 400000`, the budget RULE-2
   selected, measured in SLOT A, both are **1.006 s** and the difference is below
   the instrument's one-second resolution. **WHAT THE TWO READINGS TOGETHER
   SUPPORT, and it is less than revision 4 claimed.** Revision 4 said they "do not
   disagree" because 12 ms x 164 is about 2 s, "which would not separate two
   integer-second readings" — **and 2 s is exactly what a one-second counter
   does separate**, so that reconciliation refuted itself. The honest statement is
   about QUANTISATION: each pass is timed to the second, so a DIFFERENCE of two
   such readings carries plus or minus two seconds. At `200000` the difference is
   `85 - 83 = 2 s`, which is `2 ± 2`; at `400000` it is `165 - 165 = 0 s`, which is
   `0 ± 2`. **Both intervals contain everything from 0 to about 4 s over 164 asks,
   so this instrument bounds the coldness overhead at roughly 24 ms per ask and
   resolves it no further.** That bound is still the answer
   `docs/experiments/wp20m_design.md` §12 declines to guess, and it is stated as a
   bound rather than as an agreement between two readings that cannot in fact
   agree to that precision. **Revision 3's first attempt at this section
   left "THE COLDNESS COST IS 2.4 %" standing as a finding of a run this document
   had just declared superseded** — a remedy that re-measured `c` correctly and
   spent a true claim on the way, which is the class D-548 names and neither the
   citation checker nor the passed-section freeze can see.

**FINDING 2 IS FOR THE CLOSURE. FINDING 1 IS FOR THE CLOSURE *AND* BINDS C-E
HERE**, and the earlier revision's claim that both were the closure's alone is
withdrawn. C-E's control run loads a corpus whose `result` and `end` columns hold
ONE value each, so it exercises one arm of each of those two token-set guards and
not both. **C-E is not thereby vacuous** — the two injections fail on columns the
corpus does exercise, and `corpus-check` now prints each closed-set column's
spread so the narrowness is on the run's own face rather than in this paragraph —
but a reader must not take a green C-E for a claim that the loader was exercised
across its whole grammar. Neither finding changes a value in §6.

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

# --- THE PROVENANCE RECEIPT, FIRST AND UNCONDITIONALLY ------------------------
# Without these two lines the pilot's artifact would carry exactly the defect
# that superseded its predecessor (§7.1): a transcript attributed to a revision
# by its filename alone, with nothing in it to say which bytes ran.
echo "revision $(git rev-parse HEAD)  tree $(git status --porcelain | wc -l) modified"
echo "engine sha256 $(sha256sum "$P" | cut -d' ' -f1)"

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
cmp -s "$ART/capture_v1.txt" "$ART/capture_v2.txt"; echo "capture-determinism exit=$?"
sha256sum "$ART/capture_v1.txt" "$ART/capture_v2.txt"

# --- pass 3: the corpus, TWICE (criterion C-B) -------------------------------
t=$SECONDS
"$A" --labels "$ART/capture_v1.txt" --report "$ART/report_v1.txt" --out "$ART/corpus_v1.txt"
echo "labels1 exit=$?"; echo "labels-transform seconds=$((SECONDS - t))"
t=$SECONDS
"$A" --labels "$ART/capture_v1.txt" --report "$ART/report_v1.txt" --out "$ART/corpus_v2.txt"
echo "labels2 exit=$?"; echo "labels2-transform seconds=$((SECONDS - t))"
cmp -s "$ART/corpus_v1.txt" "$ART/corpus_v2.txt"; echo "labels-determinism exit=$?"
sha256sum "$ART/corpus_v1.txt" "$ART/corpus_v2.txt"

# --- criterion C-A: the cold-label agreement check ---------------------------
t=$SECONDS
python3 tools/cold_label_check.py --capture "$ART/capture_v1.txt" \
    --binary "$P" --engine-config configs/instrument_v0.toml --stride <S3>
echo "cold exit=$?"; echo "cold seconds=$((SECONDS - t))"

# --- criterion C-C: replay over every pilot game -----------------------------
t=$SECONDS
"$A" --replay "$ART/report_v1.txt" --out "$ART/replay_v1.txt" --workers 4
echo "replay exit=$?"; echo "replay seconds=$((SECONDS - t))"

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
names, in the shape it takes here.

**A CLAIM ABOUT THOSE TESTS WAS WRONG IN REVISION 2 AND IS CORRECTED, BECAUSE IT
WAS THIS DOCUMENT'S OWN INSTANCE OF THE LAW IT CITES.** Revision 2 said *"the
same two injections are pinned as tests against the shipped binary"*. Only the
GRAMMAR one was: the digest refusal's test called `labels_file::read` DIRECTLY,
which pins the function and leaves "the program never checked" alive — **D-553's
class, in a document written three commits after D-553 was appended.** The gap is
closed rather than the sentence softened:
`a_corpus_whose_body_digest_is_wrong_is_refused_by_the_shipped_loader` in
`crates/pistol-arena/tests/labels_tests.rs` now drives the binary, beside the
direct test that pins the message. With that landed, both injections are pinned
against the shipped program, and the runs above are a second instance over a
corpus the pilot itself wrote, at the pilot's own scale.

**SLOT C — FILLED.** `<S2>` is **`400000`** (§6.3, RULE-2) and `<S3>` is **`1`**
(§6.3, RULE-3). The block as run, its exit codes and its elapsed seconds are
recorded at the run in `artifacts/`, and the dry run's instance of the same block
is SLOT A (§7.1). The two blocks differ in
three ways: the arena config they name, the four-budget RULE-2 sweep the pilot
does not repeat, and the per-candidate `--labels` runs that sweep needs. **What
they do NOT differ in is what a later reader needs**: both open with the same
provenance receipt, and both time every pass, so the pilot measures its own wall
and states its own revision rather than inheriting the stand-in's. The claim is
stated as an enumeration of differences rather than as "and no others", because
revision 4 wrote "and no others" over a block that in fact carried two timing
lines the registered one did not.

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
| config `[run]` | `hang_timeout_ms` | **checked against the LABEL budget and not the game budget**, because pass 2 reads its watchdog out of the report the game budget wrote (`crates/pistol-arena/src/capture.rs`) and a label ask is the longest single search this pilot makes. The slot pass confirms it exceeds the dry run's whole capture pass — which bounds any single ask inside it — with room, and a value that does not is corrected before any game. It is stated as a BOUND because SLOT A times passes and not individual asks, so a "maximum label ask" is a quantity nothing here measured |
| config `[budget]` | `kind`, `value` | §3 — `nodes 50000` |
| config `[sprt]` | `elo0`, `elo1`, `alpha`, `beta` | schema completeness alone. **A self-match crosses no bound** — every pair scores alike, so no likelihood ratio is defined (D-156) — and no strength claim is made here (§0.2) |
| config `[engine_a]`/`[engine_b]` | `label` | §4C — the two MUST differ, because `validate` and `transcript::read` both refuse identical labels; the two seats attesting one ENGINE is a different comparison, over `EngineIdentity`, which carries no label |
| config `[engine_a]`/`[engine_b]` | `binary`, `config` | §1 — one binary and one engine config in both seats |
| config `[engine_a]`/`[engine_b]` | `binary_sha256` | SLOT R2, and it is the one slot that can only be true at launch: a path is not an identity and `target/release/pistol` is a different program after every build (D-147) |
| command | `--label-nodes` | §6 RULE-2 (SLOT S2) |
| command | `--stride` | §6 RULE-3 (SLOT S3) |
| command | `--workers` on `--replay` | §4C — `4`; the pass replays every game with no early stop, so what it finds does not depend on this number |

### 9.1 SLOT P — FILLED, read from `configs/arena_wp20_label_pilot.toml` at SLOT R1

| key | committed value | governed by | agrees? |
|---|---|---|---|
| `run.openings_file` | `crates/pistol-cli/tests/fixtures/random_openings_v2.txt` | §1, §2 | yes |
| `run.openings_take` | `13` | §6.3 RULE-1 | yes |
| `run.openings_skip` | `0` | §2, and the ledger's first row | yes |
| `run.turn_cap` | `40` | §7 | yes |
| `run.n_workers` | `4` | §4D | yes |
| `run.hang_timeout_ms` | `120000` | §9, **against the LABEL budget** | yes, and the ground is a BOUND rather than a maximum: SLOT A times a whole capture pass and not individual asks, so the per-ask figure of about **1.0 s** is a MEAN (`165 s / 164`). A single ask cannot exceed the pass it sits in, so **165 s is the hard upper bound on the slowest one**, and the watchdog at 120 000 ms clears even that. The document says mean-and-bound rather than "maximum", which is a quantity nothing here measured |
| `budget.kind` / `budget.value` | `nodes` / `50000` | §3 | yes |
| `sprt.elo0` / `elo1` / `alpha` / `beta` | `0.0` / `15.0` / `0.05` / `0.05` | schema completeness only; a self-match crosses no bound (D-156) | yes |
| `engine_a.label` / `engine_b.label` | `a` / `b` | §4C — they MUST differ | yes |
| `engine_a.binary` / `engine_b.binary` | `target/release/pistol` both | §1 | yes |
| `engine_a.config` / `engine_b.config` | `configs/instrument_v0.toml` both | §1 | yes |
| `engine_a.binary_sha256` / `engine_b.binary_sha256` | `180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476` both | SLOT R2 | **re-read at the run's own launch, immediately before pass 1, and corrected there if the binary was rebuilt** |
| command `--label-nodes` | `400000` | §6.3 RULE-2 | yes |
| command `--stride` | `1` | §6.3 RULE-3 | yes |
| command `--workers` (replay) | `4` | §4C | yes |

**NO CORRECTION WAS NEEDED AT THIS PASS**, which is stated rather than left to
silence: D-427's own instance was a slot pass that DID find a stale value, and a
pass that reports nothing is indistinguishable from a pass nobody made. The one
slot that cannot be discharged here is `binary_sha256`, and §1 says why.

---

## 10. WHAT THE CLOSURE OWES THIS DOCUMENT

The corpus-size plan for the training package, **labelled ESTIMATED with its
arithmetic shown**, extrapolated from C-D's measured throughput and the standing
label-budget values. **Nothing here registers a corpus size**: that is what the
pilot exists to inform, and a size registered before the measurement would be the
guess this document's whole method is against.

**AND THREE THINGS §7.2 PUTS ON THE CLOSURE'S DESK, which this document raises
and does not settle.** (i) A corpus from this pipeline carries **each position
twice**, so a plan quoting positions-per-hour must say whether it means records or
distinct records. (ii) If the pilot's own games are also all capped, the corpus
has **no outcome signal at all** — every `result` is `capped` — and whether a
value-training corpus can be built from score labels alone, or whether the turn
cap must rise, is a Stage-2 eval design question that this package must hand over
rather than answer. (iii) `p = 41` is an upper bound (§6.3); if the pilot's games
decide, the measured throughput per POSITION still holds but positions per game
falls, and the plan must be stated per position for that reason.
