# WP-2.0-S — DESIGN: the record schema, and what a label MEANS

**REVISION 1.** The half of WP-2.0 that D-544 cut out with its own design round:
*"WP-2.0-S (the record schema) is cut out with its own design round —
provenance, score representation, node columns, book and forfeit treatment, the
board key, capped-versus-forfeited, and the census-minimum rule."* No prior
revision of this document exists and no reviewer has passed any section of it, so
**D-547's freeze binds nothing here yet**; it binds every revision after this one.

**AUTHORED UNDER THE SAME DISCIPLINE AS WP-2.0-M REVISION 3** (D-546's
conditions, applied here by choice rather than by grant): every claim about the
tree carries a `path:line` citation, `tools/design_citation_check.py` is green
over this document before its review is dispatched, and text that survived a
review elsewhere enters by quotation rather than by paraphrase (D-543, D-545).

**D-483 binds this document: mechanisms, invariants and tests only.** No measured
numbers; the citation line numbers are pointers, not measurements, on the ground
`docs/experiments/wp20m_design.md` §0.1 states once and this document points to
rather than restates (D-423).

**No engine diff.** Only `pistol-arena` changes, and nothing new reaches the wire.

---

## 0. WHAT THIS PACKAGE IS, AND WHERE ITS LINE FALLS

WP-2.0-M captures the engine's answer **as the engine wrote it**, beside the
position and the run's identity, and decides nothing about what any field means.
**This package decides what the fields mean and writes the record a trainer
reads.** The split's own reason, quoted from `docs/experiments/wp20_DESIGN_STOP_SPLIT.md`:

> **A package that captures verbatim cannot be wrong about meaning**, which is
> why the mechanism can land before the meaning is settled: WP-2.0-M writes the
> totals line **as the engine wrote it**, beside the position and the run's
> identity, and WP-2.0-S decides the columns.

**THAT PROMISE IS MADE OPERATIONAL HERE AND NOT LEFT ASPIRATIONAL.** The corpus
is a **pure function of two files** — the capture and the source report — and of
nothing else. No engine runs, no channel is spawned, no clock is read. So a
successor who disagrees with any decision below re-runs this transform over the
same capture and gets a different corpus **without re-running the engine**, which
is the property D-544 named as the split's whole point.

**WHAT IT INHERITS AND MAY NOT RE-OPEN.** The asked set, coldness, the
normalisation, the capture identity, the capture record's grammar and the label
budget's kind are WP-2.0-M's, settled in `docs/experiments/wp20m_design.md` and
not restated here (D-423).

---

## 1. THE MECHANISM

**A fourth arm in `crates/pistol-arena/src/bin/arena.rs`'s mode match**
(`crates/pistol-arena/src/bin/arena.rs:82-100`), beside `--config`, `--replay`
and WP-2.0-M's `--capture`:

```
arena --labels <capture path> --report <report path> --out <corpus path>
```

It reads the capture and the report, checks that they belong together, and writes
one corpus record per capture record. **The code lives in a new module
`crates/pistol-arena/src/labels.rs`**, for the reason WP-2.0-M gives for
`capture.rs`: `crates/pistol-arena/src/bin/arena.rs` is measured against
`tools/file_justification_check.sh`'s cap by CI gate 17, and a mode arm plus its
usage paragraph is what that gate decides. The binary gains a `Mode` variant, a
dispatch pattern and a call; the exclusive output claim is inherited from
`crates/pistol-arena/src/bin/arena.rs:103`, which claims `--out` before the mode
match.

**WHY A SEPARATE MODE AND NOT A SECOND OUTPUT OF `--capture`.** Three grounds and
they agree. **(1)** It is what makes §0's re-derivation real: a transform that
only ever runs inside the expensive pass is a transform nobody can re-run. **(2)**
It leaves WP-2.0-M's design untouched — that document is reviewed and its §1 and
§4 are frozen (D-547), and adding an output to its mode would edit a frozen
section from outside the package that owns it. **(3)** The transform spawns
nothing and reads no clock, so pairing it with a pass that spends machine-hours
would make a cheap thing inherit an expensive thing's failure modes.

**WHAT PAIRS THE TWO FILES.** The capture's header carries the source report's
`source_sha256` (`docs/experiments/wp20m_design.md` §4.4). The transform digests
the report it was given and **refuses by name** if the two disagree — so a corpus
cannot be built from a capture and a report that are not each other's. This is
the same binding `Transcript::source_sha256` exists to provide, *"so a consumer
can bind the replay it produces to the report it was taken from"*
(`crates/pistol-arena/src/transcript.rs:48-50`).

---

## 2. WHAT A RECORD IS

One record per capture record. The dispatch's requirement 2 asks for *"canonical
move list (D-6), side to move, the deep-search label (score + best move + depth +
nodes), and game outcome"* (`docs/experiments/wp20_dispatches.md`), and every
column below answers a clause of it or is named in §2.8 as deliberately absent.

| # | column | what it is |
|---|---|---|
| 1 | `game` | the source report's game index |
| 2 | `turn` | the prefix length that was asked |
| 3 | `moves` | the canonical move list (D-6), as `position start moves …` spells it |
| 4 | `key_sym` | `pistol_core::canonical_sequence` over that prefix |
| 5 | `key_pos` | `GameState::key`, the position's own 128-bit key |
| 6 | `to_move` | which side places the next stone |
| 7 | `score_kind` | `eval`, `mate_in` or `mated_in` |
| 8 | `score_value` | the integer that goes with it |
| 9 | `best` | the move the engine would play |
| 10 | `depth_turns` | the depth the label was produced at |
| 11 | `search_nodes` | search nodes the label cost |
| 12 | `solver_nodes` | solver nodes the label cost |
| 13 | `result` | `p1_win`, `p2_win` or `capped` |
| 14 | `end` | `normal` or `forfeit` |

### 2.1 Identity, and the two keys — the board-key question, answered

The re-review that split this package left this open: *"The board key's 'sorted,
canonical order' does not say whether symmetries fold"*
(`docs/experiments/wp20_DESIGN_STOP_SPLIT.md` §3). It is answered by carrying
**both** keys the tree already defines and by stating exactly what each folds:

- **`key_sym` is `pistol_core::canonical_sequence`** — *"the least of its twelve
  images … two games with the same canonical form are the same game up to a
  symmetry of the lattice"* (`crates/pistol-core/src/symmetry.rs:206-219`). It is
  the key the arena already counts DISTINCT games by
  (`crates/pistol-arena/src/dedupe.rs:12-26`), applied to a prefix.
  **It folds the twelve lattice symmetries and it does NOT fold transpositions**,
  because it canonicalises a SEQUENCE: two move orders reaching one stone
  configuration have two canonical forms.
- **`key_pos` is `GameState::key`** (`crates/pistol-core/src/state.rs:124-136`),
  the stones' key XORed with the side and the phase, whose own doc says *"two
  positions this key cannot tell apart are the same position"*.
  **It folds transpositions and it does NOT fold symmetries.**

**NEITHER IS INVENTED HERE**, which is the point: rule 2 puts game truth in
pistol-core, and a third notion of "the same position" written in this crate
would be a second judge of sameness. Both are reachable from `pistol-arena`
today (`crates/pistol-core/src/lib.rs:86` and `:90` re-export what is needed, and
`GameState` at `:85`).

### 2.2 Side to move

Derived by replaying the prefix through pistol-core and reading the state, never
by parity arithmetic on the turn index. **Rule 2 is the whole reason**: whose
stone comes next is a rules question, and the arithmetic that looks obvious
(alternate every turn) is a re-implementation of it. The transform replays the
prefix anyway for §2.7 and for `key_pos`, so this column costs nothing extra.

### 2.3 The score, and what it is CALLED — matrix decision 10

The matrix left this open with a warning: *"a wrong name here makes every label
in the corpus wrong in a way no loader test catches"*
(`docs/experiments/matrix_wp20_pipeline_shape.md` §5).

**TWO COLUMNS, `score_kind` AND `score_value`, AND THE WIRE'S OWN WORD `cp` DOES
NOT APPEAR IN EITHER.** The protocol spells a score three ways
(`crates/pistol-cli/src/report.rs:145-158`), and this is the mapping:

| wire | `score_kind` | `score_value` |
|---|---|---|
| `cp <n>` | `eval` | `<n>` |
| `mate <t>` | `mate_in` | `<t>` |
| `-mate <t>` | `mated_in` | `<t>` |

**`cp` IS RENAMED RATHER THAN CARRIED, AND THAT IS THE DECISION.** The protocol's
own doc says why the wire keeps the word and why a corpus should not: *"`cp <n>`
is the static evaluation in the integer units pistol-eval works in — there is no
pawn on this board to be a hundredth of, and inventing a conversion factor would
make the number less honest, not more familiar"*
(`crates/pistol-cli/src/report.rs:149-152`). A trainer reading a column called
`cp` will read centipawns; a column called `eval` cannot be misread that way, and
the corpus header says the units are pistol-eval's own.

**TWO PROPERTIES THE HEADER STATES BECAUSE A COLUMN NAME CANNOT CARRY THEM.**
(1) **The sign is from the point of view of the side to move at the root**
(`crates/pistol-search/src/info.rs:153-155`), which is why `to_move` is a column
and not a convenience. (2) **`mate_in` and `mated_in` count every turn from the
root, both sides'** — *"so a win for the side to move is always an odd distance
and a loss an even one"* (`crates/pistol-cli/src/report.rs:147-149`, D-3, D-72) —
which is not the convention a reader arriving from chess brings.

**ALL THREE SPELLINGS GET A FIXTURE AND A TEST.** The re-review found the prior
attempt's triple incomplete — *"plain `mate` has no fixture"*
(`docs/experiments/wp20_DESIGN_STOP_SPLIT.md` §3). Three tests, named in §11, one
per arm of `classify`.

### 2.4 The node columns — D-544's own BLOCKING, not repeated

D-544 records what the dead revision did: *"`nodes` IS `search_nodes +
solver_nodes` … and the schema sourced `label nodes` from it, **so the record
summed exactly what the same revision's INVARIANT 9 said it never sums**"*.

**THE CORPUS CARRIES THE TWO INDEPENDENT WRITERS AND NOT THEIR SUM.**
`SearchInfo::solver_nodes`'s own doc names the relation: *"ONE OF THE TWO
INDEPENDENT COUNTERS — `nodes` is their derived sum at report time"*
(`crates/pistol-search/src/info.rs:167-172`), and the sum is taken at
`crates/pistol-search/src/pvs.rs:148-152`. **A sum is recoverable from the pair
by addition; the pair is not recoverable from the sum**, so a corpus that carried
only `nodes` would have thrown away the one distinction a consumer needs.

**AND THE PAIR IS NOT ALWAYS ON THE WIRE.** The solver block is printed only
inside `if info.solver_nodes > 0` (`crates/pistol-cli/src/report.rs:62-81`). So:

- totals line **carries** `search_nodes` and `solver_nodes` → both columns are
  those fields.
- totals line **carries neither** → `search_nodes` is the line's `nodes` and
  `solver_nodes` is zero. This is not an assumption: `SearchInfo::search_nodes`'s
  own doc is *"Zero difference from `nodes` whenever the gate is off"*
  (`crates/pistol-search/src/info.rs:162-166`).

**A LINE CARRYING ONE OF THE TWO AND NOT THE OTHER REFUSES THE RUN BY NAME.** It
cannot be produced by `render_info`, and a corpus is the wrong place to find out
that something else produced it (hard rule 3).

### 2.5 Depth, and 2.6 the best move

`depth_turns` is the totals line's own field; its meaning is
`SearchInfo::depth_turns`'s — *"always a depth that was actually COMPLETED"*
(`crates/pistol-search/src/info.rs:133-138`). `best` is the turn token off the
`bestmove` line, which the protocol writes as *"`bestmove <turn>`"*
(`crates/pistol-cli/src/report.rs:105-108`). **Neither is re-derived**: both are
in the capture as the engine wrote them, and re-deriving either would be this
package inventing an answer the engine already gave.

### 2.7 The outcome — capped and forfeited, separable

The re-review's first owed item: *"`undecided` discards information the
transcript has. `RecordedGame` carries a `forfeit` field, so capped and forfeited
ARE separable"* (`docs/experiments/wp20_DESIGN_STOP_SPLIT.md` §3).

**TWO COLUMNS, BECAUSE THE REPORT ITSELF KEEPS THEM APART.**
`crates/pistol-arena/src/conclusion.rs:37-52` writes `result` and `end` as
separate fields of one `game` record, and they are separate types:
`GameResult` is `p1_win` / `p2_win` / `capped`
(`crates/pistol-arena/src/record.rs:16-22`) and `End` is `normal` or `forfeit`
(`crates/pistol-arena/src/record.rs:25-28`). Collapsing them into one column is
what the dead revision did, and it is why a capped game and a forfeited one
became indistinguishable.

**AND THE COLUMNS ARE CROSS-CHECKED AGAINST pistol-core RATHER THAN TRUSTED.**
For every game that did not forfeit, the transform replays the whole recorded
move list — `Outcome` has exactly `Ongoing` and `Win`
(`crates/pistol-core/src/turn.rs:34-45`) — and requires the derived outcome to
agree with the report's own `result`: a `Win` by the side `a_is_p1` names, or
`capped` when the list ends `Ongoing`. **Disagreement refuses the run by name.**
This is `docs/process.md`'s *"externally derived referent"* — pistol-core does not
share the report's own arithmetic, so the check is one the defect could falsify
rather than one it preserves.

### 2.8 What is NOT a column, each with the reason it is not

**`provenance` — and this is the finding D-544 recorded, not repeated.** The dead
revision marked a record solver-provenance when the totals line carried the
solver fields; that condition means *the solver was CONSULTED*, and `Provenance`
has four variants (`crates/pistol-search/src/info.rs:250-267`). **Provenance is
not on the wire at all**: `SearchOutcome` carries it
(`crates/pistol-search/src/info.rs:237-245`), and the one site that turns a
`SearchOutcome` into protocol output writes the info lines, the totals line and
the bestmove line and **discards `outcome.provenance`**
(`crates/pistol-cli/src/protocol.rs:172-174`). A `git grep` for `Provenance`
under `crates/pistol-cli/src/` names the type on no line at all. **So no column of this corpus can carry it honestly,
and this package invents no proxy for it.** What a consumer gets instead is the
finer discriminator D-544 named and §2.4 already carries: `search_nodes == 0`
distinguishes an answer that is all solver from one that is not. **Putting
provenance on the wire is an engine diff and is out of scope here** (D-539); it
is recorded in §12 as residue so a successor finds it.

**`pv` — no consumer, and D-542's clause is amended rather than followed
silently.** Requirement 2 asks for the best move, not the line, and `best`
carries it. The capture preserves the whole `pv` verbatim, so a later package
that wants it re-runs this transform rather than the engine. D-542 records branch
B as widening `totals_of` so *"`score` and `pv` come out of the one parser"*; **no
package is building the `pv` half, and hard rule 10 wants that amended rather
than left describing something nobody ships** — §3 states what is actually built
and the closure carries the ADR line.

**`forfeit_by` — recoverable and not useful.** §4 widens the transcript reader to
carry it, so it is available; it is not a column because pass 1 is a self-match
between two seats of ONE engine (`configs/arena_smoke_v0.toml` is the shape), so
which seat forfeited is not a fact about the position or about the teacher.

**`nps` and `time` — not on the wire this corpus reads.** WP-2.0-M normalises
them out by gate 9's own rule, and they are *"a measurement of the machine, not of
the search"* (`crates/pistol-cli/src/report.rs:15-18`).

**`dup_of` — a property of a GAME within a RUN, not of a position.** It is
`dedupe::duplicates`' answer over one report's records
(`crates/pistol-arena/src/dedupe.rs:12-26`) and says nothing a corpus consumer
can use across reports; `key_sym` is the per-position column that does.

---

## 3. THE WIDENING THAT MAKES THIS PARSE — and why `totals_of` alone cannot

**`exchange::totals_of` CANNOT READ A CAPTURED TOTALS LINE, AND THIS IS THE ONE
INTERACTION BETWEEN THE TWO PACKAGES THAT IS NOT OBVIOUS.** `totals_of` is a
`?`-chain requiring `nodes`, `time` and `depth_turns`
(`crates/pistol-arena/src/exchange.rs:169-188`) — and WP-2.0-M's capture removes
` nps <n> time <n>` from every line it writes. **So a captured line has no
`time`, and `totals_of` returns `None` on every record in the corpus's own
input.**

**THE FIX IS THE ONE SHAPE THAT DOES NOT CREATE A SECOND READER.**
`exchange::totals_of` is split, in place, into two `pub(crate)` items:

- **`fields_of(line) -> Option<Vec<(&str, &str)>>`** — recognises the totals
  marker and returns the line's key-value tail. **It owns D-80's discipline**:
  *"a driver that billed compute to the wrong one would under-count every
  interrupted iteration"* (`crates/pistol-arena/src/exchange.rs:163-168`,
  `crates/pistol-cli/src/report.rs:20-29`), and it is the ONLY place in this
  workspace that tells `info totals …` from `info …`.
- **`totals_of(line)`** — unchanged in contract: `fields_of` then the same three
  **load-bearing** lookups over its result, returning `(nodes, time,
  depth_turns)` or `None`.

**THIS IS BRANCH B, AND IT IS SAFER THAN THE FORM D-542 WROTE.** D-542 records
branch B as `totals_of` *"widened so `score` and `pv` come out of the one parser
… the two new ones non-fatal `Option`s"*, whose hazard the matrix priced as row
(e)'s: a new lookup made load-bearing would suppress `compute.add` and zero the
SPRT report's node counts (`crates/pistol-arena/src/exchange.rs:76-79`). **A
field map adds no lookup to `totals_of` at all**, so that hazard has nothing to
attach to: the SPRT path's three lookups are the same three expressions over the
same words. **INVARIANT 4** pins that the SPRT report is byte-identical across the
change, and the mutant that kills its test is a fourth load-bearing lookup added
to `totals_of` — the same mutant WP-2.0-M registers, now with a consumer.

The corpus transform reads `score`, `depth_turns`, `nodes` and, when present,
`search_nodes` and `solver_nodes` out of `fields_of`'s map. **`totals_of` keeps
its one existing consumer** and WP-2.0-M keeps its recogniser: a LIVE totals line
carries `time`, so `totals_of(&line).is_some()` is unchanged on the only lines
pass 2 sees.

---

## 4. WHAT `transcript::read` DROPS, AND THE ONE WIDENING THAT RECOVERS IT

D-542 left this owed: *"What `transcript::read` drops — `result`, `forfeit_by`,
and the `openings_*` records that dispatch requirements 2 and 5 name. Recoverable
through pistol-core except for forfeited games; the design says how."*

**THE PARTITION, STATED.** `read_games` reads `game`, `opening`, `p1`, `p2`,
`turns`, `end`, `nodes_a` and `nodes_b`
(`crates/pistol-arena/src/transcript.rs:270-313`) and drops `result`,
`forfeit_by`, `reason`, `dup_of`, the per-side depths and the two LLR fields,
all of which `conclusion.rs` writes
(`crates/pistol-arena/src/conclusion.rs:37-52`).

- **A NON-FORFEITED game's `result` is recoverable through pistol-core**, by the
  replay §2.7 describes — which is why §2.7 makes it a cross-check rather than a
  substitute.
- **A FORFEITED game's `result` is NOT recoverable.** Its move list stops where
  the forfeit happened, so a replay says `Ongoing` and cannot say who was
  awarded the game. The matrix said as much and left it to this design.

**SO `RecordedGame` GAINS `result` AND `forfeit_by`**, read with the same
`value()` lookups the other fields use. **The fatal lookup is safe and the reason
is checkable**: `crates/pistol-arena/src/conclusion.rs:37-52` writes both fields
into every `game` record this build produces, and a report of any other schema is
already refused before `read_games` runs
(`crates/pistol-arena/src/transcript.rs:152-158`). **It is not on the SPRT
path**: nothing the generation path runs reads a report — `transcript.rs` is
consumed by the replay mode and by these two packages — so INVARIANT 4's
byte-identity claim is untouched by it.

**THE `openings_*` RECORDS ARE NOT RECOVERED AND DO NOT NEED TO BE.** They are
identified by digest rather than by value: `experiment_digest` closes over
`openings_body_sha256`, `openings_take` and `openings_skip`
(`crates/pistol-arena/src/report.rs:41-50`), and the capture header carries that
digest and the whole report's `source_sha256` besides. **The re-review's third
owed item is answered by the pair**: `experiment_sha256` *"excludes the timing
block but does not close over the games"*, and `source_sha256` closes over every
byte of the report including them — so the two together identify the experiment
AND the exact games, which neither does alone.

---

## 5. THE FILE, THE SCHEMA VERSION, AND THE LOADER

**THE SINK IS `pistol_cli::corpus::emit::Fixture`**
(`crates/pistol-cli/src/corpus/emit.rs:12-100`), the same type WP-2.0-M writes
its capture with: a header of `param` and `derived` lines, a body of one record
per line, and the in-band `body_sha256` that type appends
(`crates/pistol-cli/src/corpus/emit.rs:92-99`).

**THE RECORD IS TAB-SEPARATED, FIXED ARITY, IN §2's COLUMN ORDER**, for the
reason WP-2.0-M's §4.2 gives for its own record and which this document does not
restate (D-423): three of the fields are whitespace-bearing and this crate already
refuses a whitespace-bearing path *"because the format is whitespace-delimited and
does not quote"* (`crates/pistol-arena/src/transcript.rs:124-131`). **A field
carrying a TAB refuses the run by name.**

**THE HEADER CARRIES**, as `param`: the corpus schema version; the arena version;
the capture's `capture_sha256`; the source report's `experiment_sha256` and
`source_sha256`; and the label `go` line. As `derived`: the counts of games and
records. **And as `note` lines, the three things a column name cannot carry** —
that `eval` is in pistol-eval's own integer units, that the sign is from the
side to move at the root, and that `mate_in`/`mated_in` count both sides' turns
(§2.3). A corpus whose units live only in a design document is a corpus whose
units are lost the first time it is copied.

**THE LOADER IS `labels::read`**, shaped like `transcript::read`
(`crates/pistol-arena/src/transcript.rs:135-209`): named refusals, and **the whole
file refused rather than partially read**. It refuses, each by name: a schema
version it does not write; a body whose digest is not the one the header claims
(`crates/pistol-cli/src/corpus/emit.rs:102-118` gives it both halves); a record
whose TAB count is wrong; a `score_kind` that is not one of the three; a
`to_move`, `result` or `end` outside its own token set; and a number spelled a way
this format does not write. **This is the dispatch's *"documented, versioned
schema with a loader test"***, and **INVARIANT 6** pins it.

---

## 6. THE LABEL POLICY AND THE SEED

The dispatch asks the design to record *"the label policy (which positions get
deep labels — all, or a registered sampling rule)"*.

**ALL, AND THEREFORE NO SEED.** WP-2.0-M asks every turn boundary at which the
engine can legally be asked, and this transform writes one record per capture
record. There is no sampling rule, so **there is nothing for a seed to choose**,
and this package takes none. D-540 fixes that *"seeds attach to pipeline SAMPLING
only"*; with no sampling, a seed anywhere in this pipeline would be a knob where
hard rule 4 says there is none.

**THE DISPATCH'S REGISTERED MUTANT *"seed ignored → determinism receipt dies"* HAS
NO SITE, AND THIS DOCUMENT SAYS SO RATHER THAN INVENTING SAMPLING TO HOST IT.**
The dispatch's own wording is conditional — *"seed ignored **where the pipeline
samples**"* — and the honest reading is that the clause does not fire on a
pipeline that samples nothing. What replaces it is the determinism the pipeline
does have: the transform is a pure function of two files, so **INVARIANT 5** is
that a re-run over one capture and one report is byte-identical, and its mutant is
any ordering that depends on iteration order rather than on the capture's own
record order.

---

## 7. DEDUP, AND WHY NOTHING IS DELETED

The dispatch asks for *"dedup policy for transposed positions (by canonical move
list, stated)"*.

**THE POLICY IS THAT THE CORPUS DEDUPLICATES NOTHING, AND CARRIES THE TWO KEYS
§2.1 DEFINES SO A CONSUMER CAN.** The ground is asymmetry of loss: **a consumer
can fold a corpus and cannot unfold one**, and a record dropped at write time is
a record no re-run of this transform recovers — while §0's whole promise is that
a disagreement costs a re-run and not a re-play. Dedup is a training decision,
made against a training objective that does not exist yet, and making it here
would be making it early and irreversibly.

**WHAT "STATED" THEREFORE MEANS HERE**, in full, because an unstated fold is the
defect the dispatch's parenthesis is guarding against:

- Two records with the same `moves` are the same position reached the same way.
  Within one corpus this cannot happen — WP-2.0-M asks each prefix once — and
  across corpora it can.
- Two records with the same `key_sym` are the same position **up to a symmetry of
  the lattice**, and it *"has no false positives"* though it does have false
  negatives (`crates/pistol-core/src/symmetry.rs:213-218`).
- Two records with the same `key_pos` are the same position **up to
  transposition**, side and phase included
  (`crates/pistol-core/src/state.rs:128-133`).
- **No pair of these folds the other's equivalence.** A consumer wanting both
  folds at once needs a key this design does not carry and pistol-core does not
  define, and inventing one is out of scope by rule 2.

---

## 8. THE CENSUS-MINIMUM RULE — D-537, landed before any corpus exists

D-537 fixes two conditions *"a successor may not loosen"*: the minimum is counted
in **win-proving firings on DISJOINT POSITIONS**, and it is **fixed by a
power-style rule BEFORE any score is fitted**. The dispatch requires the rule to
land now *"so it cannot be fitted later"*.

**THE RULE.** Before any census row is counted against any candidate minimum, a
pre-registration states four inputs and computes the minimum from them:

1. the **incumbent recall** — what the best written ordering keeps, taken from
   the closed detector arc's own registered figure and cited there rather than
   restated (D-423, D-531);
2. the **target recall** the detector must beat for round 3 to be worth opening,
   taken from the same closed arc;
3. the **significance level**, and
4. the **power**,

and the minimum is **the smallest number of win-proving firings on disjoint
positions at which a two-proportion test at that level and power separates the
two recalls.** The four inputs are registered together, before the count; the
minimum is then arithmetic and nobody's choice.

**WHY THIS SHAPE AND NOT A NUMBER.** D-483 forbids a design from carrying a
measured number, and a minimum written here would be exactly that. **The rule is
the thing that has to land early**, because the defect it forecloses is a
threshold chosen after seeing which positions proved — and a rule whose four
inputs come from a CLOSED arc cannot be tuned by the corpus it will be applied
to. This is `docs/experiments/book_v2_registration.md`'s own discipline — the decision rule
registered before the sweep, the sweep before the size (D-518) — applied to a
sample instead of a book, which is what D-537 asks for in those words.

**WHERE IT IS ENFORCED, AND THE SEQUENCING NOTE D-544 LEFT FOR THE OPERATOR.**
D-539 moved census logging out of WP-2.0 entirely, so **no census row exists
anywhere until WP-2.0b lands** and the first count against this rule is WP-2.0b's.
D-544 flagged that as *"a sequencing question for the operator"*. **This design
does not re-open it and does not need it answered**: the rule's protective force
is that it is committed and dated before any census row exists, which git shows
and no later package can undo — the enforcing package is a separate question from
the registering one. **What this package produces is not census and is not corpus
for that count**: D-539 is explicit that *"the pilot carries no census and is not
corpus"*.

---

## 9. WHAT HAPPENS WHEN THE TRANSFORM CANNOT ANSWER

| condition | the answer |
|---|---|
| the capture's `source_sha256` is not the report's digest | **refuse the run**, by name, before reading a record |
| the capture's schema version is not the one WP-2.0-M writes | **refuse the run**, by name |
| a capture record's field count is wrong, or a field carries a TAB | **refuse the run**, by name, naming the record |
| a totals line carries no `score`, or a `score` spelling that is none of the three | **refuse the run**, by name |
| a totals line carries one solver node field and not the other | **refuse the run**, by name |
| a captured `moves` prefix is not a legal game under pistol-core | **refuse the run**, by name |
| the derived outcome disagrees with the report's own `result` | **refuse the run**, by name, naming the game |
| a capture record names a game the report does not hold | **refuse the run**, by name |

**Every failure refuses the whole run and none is a skip**, for WP-2.0-M's reason
and `replay::run`'s before it: *"a criterion over SOME of a report's games is a
criterion over a sample nobody registered"*
(`crates/pistol-arena/src/replay.rs:16-19`). **INVARIANT 7.**

---

## 10. INVARIANTS

1. **Every capture record produces exactly one corpus record**, in the capture's
   own order, with nothing dropped, added or reordered.
2. **Every column is a function of the capture and the report alone.** The
   transform spawns no process, opens no channel and reads no clock.
3. **`score_kind` is one of `eval`, `mate_in`, `mated_in`**, mapped from the
   protocol's three spellings and never carrying the word `cp`; and
   `search_nodes` and `solver_nodes` are the two independent counters, never
   their sum.
4. **Splitting `totals_of` changes no output**: the SPRT path's report is
   byte-identical across the change, and `totals_of`'s three lookups stay
   load-bearing.
5. **A re-run of the transform over one capture and one report produces a
   byte-identical corpus file.**
6. **A corpus file round-trips through its own loader**, and one whose schema
   version, body digest, record arity or token set is wrong is refused by name.
7. **Any failure refuses the whole run**; no record is silently skipped.
8. **The derived outcome agrees with the report's own `result` on every
   non-forfeited game**, or the run is refused.
9. **The corpus deduplicates nothing.**

---

## 11. TESTS AND MUTANTS

| test | pins |
|---|---|
| `every_capture_record_produces_one_corpus_record_in_order` | 1 |
| `the_transform_spawns_no_process_and_reads_no_clock` | 2 |
| `a_cp_score_becomes_an_eval_column_and_not_a_cp_one` | 3 |
| `a_mate_score_becomes_mate_in` | 3 |
| `a_negative_mate_score_becomes_mated_in` | 3 |
| `a_totals_line_without_solver_fields_yields_all_nodes_as_search_nodes` | 3 |
| `a_totals_line_with_solver_fields_yields_the_two_counters_it_carries` | 3 |
| `a_totals_line_carrying_one_solver_field_refuses_the_run_by_name` | 3, 7 |
| `splitting_totals_of_leaves_the_sprt_report_byte_identical` | 4 |
| `totals_of_still_refuses_a_line_missing_nodes_time_or_depth` | 4 |
| `fields_of_reads_a_captured_line_that_has_no_time_field` | 4 |
| `a_rerun_over_one_capture_and_report_is_byte_identical` | 5 |
| `a_corpus_file_round_trips_through_its_own_loader` | 6 |
| `a_corpus_whose_body_digest_is_wrong_is_refused_by_name` | 6 |
| `a_corpus_record_with_the_wrong_field_count_is_refused_by_name` | 6 |
| `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` | 6 |
| `a_score_kind_outside_the_three_is_refused_by_name` | 6 |
| `a_capture_whose_source_digest_is_not_the_reports_is_refused_by_name` | 7 |
| `a_capture_record_naming_a_game_the_report_lacks_is_refused_by_name` | 7 |
| `the_derived_outcome_agrees_with_the_reports_own_result_field` | 8 |
| `a_report_whose_result_contradicts_its_moves_refuses_the_run_by_name` | 8 |
| `a_forfeited_games_result_is_the_reports_and_is_not_derived` | 8 |
| `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` | 8 |
| `two_transposed_positions_are_two_records` | 9 |
| `two_positions_alike_up_to_a_symmetry_are_two_records_sharing_a_key_sym` | 9 |
| `side_to_move_comes_from_pistol_core_and_not_from_turn_parity` | §2.2 |

**MUTANTS:**

| mutation | the test that dies |
|---|---|
| `score_kind` collapsed so `mate` and `-mate` share a token | `a_negative_mate_score_becomes_mated_in` |
| the score column emitted as the wire's `cp` | `a_cp_score_becomes_an_eval_column_and_not_a_cp_one` |
| `search_nodes` sourced from the line's `nodes` when the solver fields ARE present | `a_totals_line_with_solver_fields_yields_the_two_counters_it_carries` |
| `solver_nodes` defaulted to zero instead of refusing on a half-present pair | `a_totals_line_carrying_one_solver_field_refuses_the_run_by_name` |
| a fourth load-bearing lookup added to `totals_of` | `splitting_totals_of_leaves_the_sprt_report_byte_identical` |
| `time` made non-fatal in `totals_of` | `totals_of_still_refuses_a_line_missing_nodes_time_or_depth` |
| **a corpus schema field dropped on write** | `a_corpus_file_round_trips_through_its_own_loader` |
| the loader's body-digest check removed | `a_corpus_whose_body_digest_is_wrong_is_refused_by_name` |
| the loader's schema-version check removed | `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` |
| the capture/report digest binding removed | `a_capture_whose_source_digest_is_not_the_reports_is_refused_by_name` |
| the pistol-core outcome cross-check removed | `a_report_whose_result_contradicts_its_moves_refuses_the_run_by_name` |
| `result` and `end` collapsed into one column | `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` |
| `to_move` computed from turn parity | `side_to_move_comes_from_pistol_core_and_not_from_turn_parity` |
| records deduplicated by `key_pos` on write | `two_transposed_positions_are_two_records` |
| the record order taken from a hash map rather than the capture | `a_rerun_over_one_capture_and_report_is_byte_identical` |

**THE VACUITY THIS PACKAGE MUST NOT WALK INTO**, named because the arc has paid
for it three times (D-527, and WP-2.0-M's §14): none of these tests needs a real
engine. The transform reads FILES, so a fixture capture and a fixture report —
written by the test, or produced by the arena's stub engine — exercise every path
without the vacuity that hardcoded stub timings create. **The one test that would
be vacuous against a hand-written fixture is `the_derived_outcome_agrees_with_the_reports_own_result_field`**
if the fixture's `result` were computed by the same code the test checks, so **its
fixture reports are produced by the arena itself** and their `result` fields are
the arena's own.

---

## 12. WHAT THIS PACKAGE DOES NOT DECIDE, AND THE RESIDUE IT NAMES

**Not decided here, and correctly elsewhere:** the label budget's value and the
pilot's `book_v2` range (the pilot's pre-registration, D-483); the asked set,
coldness, the normalisation and the capture identity (WP-2.0-M); the census
identity column and the census logging flag (WP-2.0b, D-539); any train/test
split, which is a training decision over a corpus and not a property of one.

**RESIDUE, NAMED SO A SUCCESSOR FINDS IT RATHER THAN REDISCOVERS IT:**

1. **`Provenance` is not on the wire**, so no corpus column can carry it (§2.8).
   Putting it there is an engine diff and belongs with WP-2.0b's protocol work if
   anyone wants it.
2. **D-542's branch-B text describes a `pv` widening no package is building**
   (§2.8, §3). Hard rule 10 wants the amendment, and the closure carries it.
3. **A key folding transpositions AND symmetries together does not exist in
   pistol-core** (§7), so no column carries one.
