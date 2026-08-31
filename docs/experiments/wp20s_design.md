# WP-2.0-S — DESIGN: the record schema, and what a label MEANS

**REVISION 2**, the ONE fix round the split's granted design round allows, after
revision 1 (`1157dae`) took a fresh-context REVIEW-design and returned **FAIL** —
6 BLOCKING, 11 MAJOR, 12 MINOR (`docs/experiments/wp20s_design_REVIEW.md`).
**A second FAIL stops this package and returns it to the architect.**

**THE SIX BLOCKING FINDINGS, ON THE FACE, because a fix round that hides what it
is fixing is how this arc's earlier rounds failed (D-545).** (1) §3's field map
could not carry the score, which is TWO tokens on the wire — the second limb of
the very finding that killed WP-2.0's revision 1. (2) `pistol_core::canonical_form`
folds transpositions AND symmetries and is exported on the line this document
cited for its neighbour, so §7 and §12 told a successor to look for something
already there. (3) §2.7's cross-check went through `a_is_p1`, which is about
which ENGINE holds a seat and not about which SEAT won, so as written the
transform refused about half its valid inputs. (4) Book-turn treatment — this
package's charge by D-544's own words, handed here by name by WP-2.0-M — was
never decided. (5) The dispatch's throughput obligation was in no package. (6)
The corpus itself had no manifest.

**Each is taken below, and three of them changed a decision rather than a
sentence**: the parser's return type (§3), a third key and the disjointness
denominator it settles (§2.1, §7, §8), and a `book` column (§2.9).

---

The half of WP-2.0 that D-544 cut out with its own design round:
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
`crates/pistol-arena/src/bin/arena.rs:103`, which claims `--out` **after the mode
is parsed and before it is dispatched** (`:104-107`), so a fourth arm needs no
O_EXCL construction of its own (D-200). `crates/pistol-arena/src/lib.rs:47-69`
gains `pub mod labels;` beside the others; the `--labels` usage paragraph goes in
`crates/pistol-arena/src/usage.rs`, the module WP-2.0-M extracts `USAGE` into;
and the fallback refusal at `crates/pistol-arena/src/bin/arena.rs:94-99` gains
the fourth mode.

**AND THIS IS A DEPARTURE FROM D-542's RECORDED SHAPE, NAMED AS ONE.** D-542
selects row (g) as *"a labelling mode of the existing `arena` binary,
**two-pass**"* whose mechanism is *"a **third arm** in `bin/arena.rs`'s mode
match"*. This design adds a FOURTH arm and a THIRD pass. The grounds below are
why, and this document would take the same decision again — but **hard rule 10
wants the amendment rather than the drift**, so the ADR line this package lands
records the pass count and the arm count and not only §2.8's `pv` clause. It
changes how the work is divided, not what row (g) was selected for: no new crate,
no new protocol spelling, no engine seam, and the expensive pass is still
WP-2.0-M's.

**WHY A SEPARATE MODE AND NOT A SECOND OUTPUT OF `--capture`.** Three grounds and
they agree. **(1)** It is what makes §0's re-derivation real: a transform that
only ever runs inside the expensive pass is a transform nobody can re-run. **(2)**
WP-2.0-M owns its mode and **its own review is outstanding**, so adding an output
to that mode would couple two design rounds and reopen a document under
adjudication elsewhere. (Revision 1 put this as *"its §1 and §4 are frozen"*,
which overstated both facts: D-547 freezes what a reviewer has PASSED, WP-2.0-M's
revisions 1 and 2 both FAILED, and its freeze table names paragraphs rather than
whole sections.) **(3)** The transform spawns nothing and reads no clock, so
pairing it with a pass that spends machine-hours would make a cheap thing inherit
an expensive thing's failure modes.

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
column below answers a clause of it, or is named in §2.8 as deliberately absent
**together with every other field the capture preserves and this record does not
take** — revision 1 claimed that completeness and §2.8 did not deliver it.

| # | column | what it is |
|---|---|---|
| 1 | `game` | the source report's game index |
| 2 | `turn` | the prefix length that was asked |
| 3 | `moves` | the move list (D-6), the turn tokens alone; **empty at `turn` zero** (§2.10) |
| 4 | `key_seq` | `pistol_core::canonical_sequence` over that prefix |
| 5 | `key_pos` | `GameState::key`, the position's own 128-bit key, 32 hex digits |
| 6 | `key_full` | `pistol_core::canonical_form` over the replayed prefix's stones |
| 7 | `to_move` | which side places the next stone |
| 8 | `score_kind` | `eval`, `mate_in` or `mated_in` |
| 9 | `score_value` | the integer that goes with it (§2.3) |
| 10 | `best` | the move the engine would play |
| 11 | `depth_turns` | the depth the label was produced at (§2.5) |
| 12 | `search_nodes` | search nodes the label cost |
| 13 | `solver_nodes` | solver nodes the label cost |
| 14 | `book` | whether neither engine chose this position (§2.9) |
| 15 | `result` | `p1_win`, `p2_win` or `capped` |
| 16 | `end` | `normal` or `forfeit` |

### 2.1 Identity, and the THREE keys — the board-key question, answered

The re-review that split this package left this open: *"The board key's 'sorted,
canonical order' does not say whether symmetries fold"*
(`docs/experiments/wp20_DESIGN_STOP_SPLIT.md` §3). It is answered by carrying
**every key pistol-core defines** and by stating exactly what each folds.
Revision 1 carried two and asserted that a key folding both equivalences *"does
not exist in pistol-core"*. **That was false, and the third key is the one this
package most needed:**

- **`key_seq` is `pistol_core::canonical_sequence`** — *"the least of its twelve
  images … two games with the same canonical form are the same game up to a
  symmetry of the lattice"* (`crates/pistol-core/src/symmetry.rs:206-219`). It is
  the key the arena already counts DISTINCT games by
  (`crates/pistol-arena/src/dedupe.rs:12-26`), applied to a prefix.
  **It folds the twelve lattice symmetries and it does NOT fold transpositions**,
  because it canonicalises a SEQUENCE: *"two different games can reach the same
  stones"* (`crates/pistol-core/src/symmetry.rs:181-193`). Renamed from revision
  1's `key_sym`, because with three keys a name has to say which equivalence.
- **`key_pos` is `GameState::key`** (`crates/pistol-core/src/state.rs:124-136`),
  the stones' key XORed with the side and the phase, whose own doc says *"two
  positions this key cannot tell apart are the same position"*.
  **It folds transpositions and it does NOT fold symmetries.**
- **`key_full` is `pistol_core::canonical_form`** over the stones the replayed
  prefix reaches — *"the least of its twelve images … two positions with the same
  canonical form are the same position up to a symmetry of the lattice"*, and
  *"the input need not be sorted and its play order is not read"*
  (`crates/pistol-core/src/symmetry.rs:157-165`). Play-order independence over a
  stone set **is** transposition folding and the twelve-image minimum **is**
  symmetry folding, so **`key_full` folds both.**

**AND `key_full` LOSES NOTHING ON AN ASKED POSITION, WHICH IS WHY IT IS THE ONE
§8's DENOMINATOR USES.** `canonical_form` reads stones and not the mover, so in
general it discards the side to move — but **every asked position is a turn
boundary at the start of a turn** (WP-2.0-M's INVARIANT 1 and 2), and there the
stone count fixes the rest: `GameState::key`'s own doc says *"for an ongoing game
the stone count fixes the turn, the phase and the mover together"*
(`crates/pistol-core/src/state.rs:128-133`). On this corpus's records, and only
on them, `key_full` is a total position identity that folds both equivalences.

**NONE OF THE THREE IS INVENTED HERE**, which is the point: rule 2 puts game truth
in pistol-core, and a fourth notion of "the same position" written in this crate
would be a second judge of sameness. All three are exported from
`crates/pistol-core/src/lib.rs:85-90`.

### 2.2 Side to move

Derived by replaying the prefix through pistol-core and reading the state, never
by parity arithmetic on the turn index. **Rule 2 is the whole reason, and this
document does not pretend it is a behavioural one**: game rule 3 makes the mover
alternate strictly by turn, every asked position is a turn boundary at
`Phase::First` (`crates/pistol-core/src/turn.rs:25-29`), and a `Turn::Single` can
only be the first turn or a game's last — so **parity and pistol-core agree on
every legal prefix**, and a parity implementation would produce the same column.
Revision 1 registered *"`to_move` computed from turn parity"* as a mutant; it
cannot die, and it is removed for the reason revision 1 of WP-2.0-M's own §8 was
corrected for. The reason to replay is that pistol-core is the only judge of
legality and of whose stone comes next (rule 2), and the transform replays the
prefix anyway for §2.7, `key_pos` and `key_full`, so the column costs nothing.

**WHAT REPLACES THE MUTANT IS ONE THAT CHANGES AN ANSWER**: deriving `to_move`
from the record's `turn` column WITHOUT replaying, which diverges the moment a
capture record's `turn` and `moves` disagree — a disagreement §9 now refuses by
name, and which the mutation would silently absorb.

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

**`score_value`'s TYPE FOLLOWS `score_kind` AND THE COLUMN SAYS SO.** `ScoreKind`
is `Eval(i32)`, `MateIn(u16)`, `MatedIn(u16)`
(`crates/pistol-search/src/score.rs:53-61`): an `eval` value is a SIGNED integer,
and a `mate_in`/`mated_in` value is a turn count and is never negative. **The
loader refuses a negative value under either mate kind by name**, which is a
refusal the "not a number this format writes" rule does not cover.

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

**THE SOLVER BLOCK IS ALL SIX FIELDS OR NONE, AND ANY OTHER SUBSET REFUSES THE
RUN BY NAME.** `render_info` emits `search_nodes`, `solver_nodes`,
`solver_firings`, `solver_invocations`, `solver_proofs` and `solver_root_nodes`
inside one conditional (`crates/pistol-cli/src/report.rs:62-81`), so the block is
atomic on the wire. Revision 1 refused only a half-present PAIR, which made the
refusal arbitrary: a line carrying the pair and not `solver_proofs` was exactly
as impossible and was accepted in silence. The refusal is over the block (hard
rule 3), and **the four call counters this record does not take are named in
§2.8** rather than passed over.

### 2.5 Depth, and 2.6 the best move

`depth_turns` is the totals line's own field, and **it has two meanings, which
this document reconciles rather than carrying both silently.** Ordinarily it is
`SearchInfo::depth_turns` — *"always a depth that was actually COMPLETED"*
(`crates/pistol-search/src/info.rs:133-138`). **On a record whose `search_nodes`
is zero it is a PROOF depth**: that is a root solver proof, whose own doc says
*"`depth_turns` is the proof's depth in turns"*
(`crates/pistol-search/src/info.rs:259-261`), written from `tree.win_depth_turns()`
(`crates/pistol-search/src/search.rs:785-793`). **So the column's meaning is:
a completed search depth, except where `search_nodes` is zero, where it is the
proof's depth in turns** — and §5's header `note` block says so beside the score's
three properties, because a consumer reading the column without the discriminator
would average two different quantities. This is the meaning half of the finding
that killed WP-2.0's revision 1; §2.8 discharges its discriminator half. `best` is the turn token off the
`bestmove` line, which `bestmove_line` builds from the `BESTMOVE_PREFIX` and the
turn (`crates/pistol-cli/src/report.rs:105-108`). **Neither is re-derived**: both are
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

**AND THE COLUMNS ARE CROSS-CHECKED AGAINST pistol-core RATHER THAN TRUSTED, ON
A RELATION THAT READS NO SEATING.** For every game that did not forfeit, the
transform replays the whole recorded move list — `Outcome` has exactly `Ongoing`
and `Win` (`crates/pistol-core/src/turn.rs:34-45`) — and requires:

> `Outcome::Win { winner: Player::P1 }` ⟺ `result == p1_win`;
> `Outcome::Win { winner: Player::P2 }` ⟺ `result == p2_win`;
> `Ongoing` at the end of the list ⟺ `result == capped`.

**`a_is_p1` IS NOT READ, AND REVISION 1's SENTENCE THAT IT WAS IS THE REASON THIS
IS SPELLED OUT.** `GameResult` is about SEATS — `P1Win => "p1_win"`
(`crates/pistol-arena/src/record.rs:16-22`), written straight off pistol-core's
own `Outcome` (`crates/pistol-arena/src/game.rs:103-107`) — while `a_is_p1` is
about which ENGINE holds seat one (`crates/pistol-arena/src/transcript.rs:17-18`).
A comparison gated on `a_is_p1` inverts on every game where engine B holds seat
one, which is half of a paired-openings run, and §9 makes disagreement refuse the
whole run: revision 1's transform would have refused about half its valid inputs.

**WHAT THE CHECK IS, CLAIMED FOR WHAT IT IS.** Revision 1 called it
`docs/process.md`'s *"externally derived referent"*. **It is not**: `game.rs`
derives `result` from the same `GameState::make_turn` the transform replays with,
and `transcript::read` replays every list through it again at read time
(`crates/pistol-arena/src/transcript.rs:359-379`), so the two sides share their
input and `docs/process.md` calls that internal agreement. **The referent is the
arena's RECORDED VERDICT, and the defect class the check excludes is a defect in
THIS transform's replay, mapping and record assembly** — which is exactly the
defect revision 1 had. It is **not** an independent check of pistol-core's win
detection, and this document does not claim one.

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

**AND THE SIX FIELDS THE CAPTURE PRESERVES THAT NO COLUMN TAKES, NAMED BECAUSE
§2's OPENING CLAIMS THEY ARE.** `render_info` writes `seldepth` and `hashfull`
beside the fields §2 takes (`crates/pistol-cli/src/report.rs:82-84`), and its
solver block carries four more beyond the node pair — `solver_firings`,
`solver_invocations`, `solver_proofs`, `solver_root_nodes`
(`crates/pistol-cli/src/report.rs:69-78`).

- **`seldepth` and `hashfull` are facts about the SEARCH's shape and the TABLE's
  occupancy, not about the position**, and requirement 2 asks for neither. A
  trainer reading a label does not use them, and the capture keeps them.
- **The four call counters are the DETECTOR's quantities, not a label's** — they
  count what the solver was ASKED as opposed to what it SPENT (D-465, D-508,
  `crates/pistol-search/src/info.rs:176-178`) — and **`solver_proofs` in
  particular is a census quantity, which D-539 moved to WP-2.0b.** They are not
  columns here because a census row needs the POSITION IDENTITY that WP-2.0b
  exists to add, and a corpus column carrying half of a census would invite the
  count D-537 forbids being taken on it. §8's own rule says which count is the
  one that matters and where it happens.

**None of the six is lost**: all six are in the capture verbatim, and this
transform is a pure function that a successor re-runs (§0).

**`dup_of` — a property of a GAME within a RUN, not of a position.** It is
`dedupe::duplicates`' answer over one report's records
(`crates/pistol-arena/src/dedupe.rs:12-26`) and says nothing a corpus consumer
can use across reports; `key_full` is the per-position column that does.

### 2.9 Book turns — this package's charge, and it is taken

D-544 cuts this package out with *"book and forfeit treatment"* among its list,
and WP-2.0-M hands it over by name: *"**Book turns and forfeited games are asked
like any other**, because those ARE exclusions by meaning and **they belong to
WP-2.0-S**"* (`docs/experiments/wp20m_design.md`). **Revision 1 decided the
forfeit half and left the book half undecided in a third round; it is decided
here.**

**EVERY BOOK POSITION GETS A RECORD, AND EVERY RECORD SAYS WHETHER IT IS ONE.**
The `book` column is `yes` or `no`.

**THE BOUNDARY IS THE ARENA'S OWN AND IS NOT INVENTED HERE.** A report carries
`opening_turns` (`crates/pistol-arena/src/transcript.rs:39-40`), and the arena
asks an engine at turn index `at` only when `at >= opening_turns`
(`crates/pistol-arena/src/replay.rs:137-138`). A capture record whose prefix
length is `k` is the position BEFORE turn index `k`, so **the record is `book`
exactly when `k < opening_turns`** — which is precisely "neither engine chose to
be here".

**WHY LABELLED-AND-FLAGGED RATHER THAN EXCLUDED, and the ground is asymmetry of
loss (§7's).** A book position is a legal position the teacher scored at the
label budget, and its label is exactly as good as any other; what is different is
that the MOVES leading to it were drawn from a book rather than chosen, which
matters to a trainer weighting positions by how a game reached them and matters
not at all to one learning a value function. **A consumer can drop a flagged
record; it cannot recover one this package dropped.** So the flag is carried and
the decision is left where the training objective is.

### 2.10 The record at turn zero, and why no field is ever empty

WP-2.0-M asks the initial position of every game and asks it as bare
`position start` (`docs/experiments/wp20m_design.md` §2), so **every capture
holds a record whose prefix length is zero** — and three of this record's columns
are computed over an empty prefix: `moves` is the empty move list, `key_seq` is
`canonical_sequence(&[])` and `key_full` is `canonical_form(&[])`, all three
empty.

**THEY ARE WRITTEN AS A SINGLE `-`, AND NO FIELD OF THIS RECORD IS EVER EMPTY.**
A `-` cannot collide with any value these columns can hold: a turn token is one
or two `q,r` cells (D-6), never the single character. The alternative — an empty
field legal in exactly three columns and nowhere else — is a conditional the
loader would have to carry and a reader would have to remember, and **a loader
rule that reads "no field is ever empty" is one a mutation cannot slip past.**
`key_pos` needs no sentinel: `GameState::key` on a new game is a key like any
other.

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

- **`fields_of(line) -> Option<Vec<&str>>`** — recognises the totals marker and
  returns the line's tail **as its WORDS, in order**. **It owns D-80's
  discipline**: *"a driver that billed compute to the wrong one would under-count
  every interrupted iteration"* (`crates/pistol-arena/src/exchange.rs:163-168`,
  `crates/pistol-cli/src/report.rs:20-29`), and it is the only place **in
  `pistol-arena`** that tells `info totals …` from `info …` — which is the scope
  row (b)'s kill condition is drawn in
  (`docs/experiments/matrix_wp20_pipeline_shape.md` §3). It is **not** the only
  such recogniser in the workspace: `tools/sealbot/matchserver/src/pistol_client.rs`
  has one, which `crates/pistol-cli/src/report.rs:55-61` names as *"the one
  substring parser in the tree"*, and revision 1's workspace-wide claim was
  false.
- **`totals_of(line)`** — unchanged in contract: `fields_of` then the same three
  **load-bearing** `value()` lookups over its word list, returning `(nodes, time,
  depth_turns)` or `None`. The lookup is the one it already performs —
  *"the word after the key"* — over the same words it already splits
  (`crates/pistol-arena/src/exchange.rs:176-188`).

**THE RETURN TYPE IS THE WORD LIST AND NOT A KEY-VALUE MAP, AND THAT IS THE FIX
FOR REVISION 1's BLOCKING.** Revision 1 typed it `Option<Vec<(&str, &str)>>`.
**The score is TWO tokens after its key** — `render_info` writes
`… hashfull {} score {} pv` and `score_token` expands to `cp <n>`, `mate <t>` or
`-mate <t>` (`crates/pistol-cli/src/report.rs:82-84`, `:153-158`) — so a map keyed
by field name yields the literal `cp` and **the number is unreachable**. That is
the second limb of the finding that killed WP-2.0's revision 1
(`docs/experiments/wp20_design_REVIEW.md`), reintroduced under a new name; a word
list carries it because a caller may read the word after the key AND the word
after that.

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

**HOW THE CORPUS TRANSFORM READS EACH FIELD, since revision 1 said only "out of
the map".** `depth_turns`, `nodes` and, when the block is present, the six solver
fields (§2.4) are each **the word after their key**. **The score is the PAIR
`(tag, number)`** — the word after `score` is the tag, one of `cp`, `mate` or
`-mate`, and the word after THAT is the number — which §2.3 maps onto
`score_kind` and `score_value`. **A tag outside the three, a number that does not
parse, or either word absent refuses the run by name** (§9). `pv` is not read at
all: it is the line's variable-length tail and no column takes it (§2.8). **`totals_of` keeps
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
reason WP-2.0-M's §4.2 owns and which this document points at rather than
restates (D-423). Two of these columns carry spaces — `moves`, and `key_seq`
rendered from a turn list — which is enough to make a space-delimited record
lose its arity. **A field carrying a TAB refuses the run by name**, and by §2.10
**no field is ever empty.**

**TWO COLUMNS' SPELLINGS ARE FIXED HERE BECAUSE A LOADER CANNOT CHECK A TOKEN IT
CANNOT PREDICT.** `key_pos` is `Key128`'s own `Display` — *"the 32 hex digits of
the key, high half first"* (`crates/pistol-core/src/zobrist.rs:70-76`). `key_seq`
and `key_full` are their values rendered as turn tokens and as `q,r` cells
respectively, space-joined, in the order the canonicaliser returns them.

**THE HEADER CARRIES**, as `param`: the corpus schema version; the capture's
`capture_sha256`; the source report's `experiment_sha256` and `source_sha256`;
and the label `go` line. As `derived`: the counts of games and records. **The
arena version is NOT a header param**, for the reason WP-2.0-M's §5 gives once
and this document points at: the workspace version has never moved, so it is a
second hand-maintained number wearing a mechanism's clothes, and what binds an
artifact to the code that made it is the **governing revision** the pilot's
pre-registration names (`docs/process.md`).

**AND THE UNITS GO IN AS `param` LINES, NOT `note` LINES.** `Fixture::note`
renders a bare `# <text>` indistinguishable from the title lines above it
(`crates/pistol-cli/src/corpus/emit.rs:19-28`, `:51-58`), so a machine reader
cannot find them. The three properties a column name cannot carry —
`score_units`, `score_sign` and `mate_counts` — are therefore **keyed** params
whose values a loader can check: that `eval` is in pistol-eval's own integer
units, that the sign is from the side to move at the root, and that
`mate_in`/`mated_in` count both sides' turns (§2.3). A corpus whose units live
only in a design document is a corpus whose units are lost the first time it is
copied; a corpus whose units are prose in a comment is one no program can read
them from.

**THE LOADER IS `labels::read`**, shaped like `transcript::read`
(`crates/pistol-arena/src/transcript.rs:135-209`): named refusals, and **the whole
file refused rather than partially read**. It refuses, each by name: a schema
version it does not write; a body whose digest is not the one the header claims
(`crates/pistol-cli/src/corpus/emit.rs:102-118` gives it both halves); a record
whose TAB count is wrong; **any field that is empty**; a `score_kind` that is not
one of the three; **a negative `score_value` under either mate kind** (§2.3); a
`to_move`, `result`, `end` or `book` outside its own token set; a `key_pos` that
is not thirty-two hex digits; **a header missing any of its params, the three
unit params included**; and a number spelled a way this format does not write.
**This is the dispatch's *"documented, versioned schema with a loader test"***,
and **INVARIANT 6** pins it.

**AND THE LOADER RE-DERIVES THE CAPTURE IDENTITY RATHER THAN COPYING IT.** The
header's `capture_sha256` is a claim about a file this transform read; the
transform recomputes it from WP-2.0-M's own three inputs — the capture format
version, the report's `experiment_sha256` and the label `go` line
(`docs/experiments/wp20m_design.md` §5) — and **refuses by name on disagreement**,
so a corpus cannot name a capture identity its own inputs do not have.

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

**A REGISTERED MUTANT IS LEFT WITHOUT A SITE, AND WHICH DISPATCH SAYS WHAT IS
STATED EXACTLY, BECAUSE REVISION 1 GOT IT WRONG.** There are three governing
dispatches (`docs/experiments/wp20_dispatches.md`). The **WP-2.0 dispatch**
registers *"seed ignored -> determinism receipt dies"* **unconditionally**. The
**M-design-by-quotation dispatch** qualifies it — *"seed ignored **where the
pipeline samples**"*. Revision 1 quoted the qualified form and attributed it to
the first, which is a claim about a governing text that text does not make: the
class D-545 names, applied to a dispatch instead of to code.

**THE MUTANT HAS NO SITE EITHER WAY, AND THAT IS A DEPARTURE THIS DOCUMENT OWNS
RATHER THAN A READING IT CLAIMS.** No seed exists to ignore, so no mutation of a
seed can change an output; inventing a sampling rule to host a mutant would be
adding a knob for a test's sake. **What replaces it is the determinism the
pipeline does have**: the transform is a pure function of two files, so
**INVARIANT 5** is that a re-run over one capture and one report is
byte-identical, and its mutant is any ordering that depends on hash iteration
rather than on the capture's own record order — which is hard rule 4's own
concern and is the property a seed mutant would have been guarding.

---

## 7. DEDUP, AND WHY NOTHING IS DELETED

The dispatch asks for *"dedup policy for transposed positions (by canonical move
list, stated)"*.

**THE POLICY IS THAT THE CORPUS DEDUPLICATES NOTHING, AND CARRIES THE THREE KEYS
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
- Two records with the same `key_seq` are the same GAME PREFIX **up to a symmetry
  of the lattice**, and it *"has no false positives"* though it does have false
  negatives (`crates/pistol-core/src/symmetry.rs:213-218`).
- Two records with the same `key_pos` are the same position **up to
  transposition**, side and phase included
  (`crates/pistol-core/src/state.rs:128-133`).
- Two records with the same `key_full` are the same position **up to
  transposition AND symmetry together**
  (`crates/pistol-core/src/symmetry.rs:157-165`), which on this corpus's records
  loses nothing (§2.1). **This is the coarsest of the three and it is the one
  §8's disjointness counts over.**

Revision 1 closed this list with *"no pair of these folds the other's
equivalence … a key [pistol-core] does not define"*. **`canonical_form` defines
it, exported three identifiers from the one revision 1 cited**, and both
sentences are deleted rather than softened.

---

## 8. THE CENSUS-MINIMUM RULE — D-537, landed before any corpus exists

D-537 fixes two conditions *"a successor may not loosen"*: the minimum is counted
in **win-proving firings on DISJOINT POSITIONS**, and it is **fixed by a
power-style rule BEFORE any score is fitted**. The dispatch requires the rule to
land now *"so it cannot be fitted later"*.

**THE RULE, AND EVERYTHING IT FIXES IS FIXED HERE.** Revision 1 registered four
inputs and deferred two of them to an unnamed future pre-registration. **That is
the deferral that killed the last §8** — D-518's own item 3 fixes a threshold
*"before the sweep, precisely because moving it afterwards is the post-hoc
threshold move CLAUDE.md forbids"* — and a level and a power nobody has chosen
are not sheltered by D-483, because neither is a measured number. So:

1. **THE SIGNIFICANCE LEVEL AND THE POWER ARE FIXED HERE, AT THE PROJECT'S OWN
   REGISTERED CONVENTION.** Every committed arena experiment config in this
   repository carries `alpha = 0.05` and `beta = 0.05`
   (`configs/arena_smoke_v0.toml:66-67`), which is the pair CLAUDE.md's hard rule
   6 makes the judge of every search and eval change. **This rule takes the same
   pair**: a level of 0.05 and a power of 0.95. The ground is that a detector
   round is a decision of the same kind and at the same cost as the SPRT
   decisions this project already registers at those error rates, and adopting a
   convention already in force is the one choice a successor cannot say was
   tuned to this corpus. **These are the only two numerals this document
   carries, and they are here deliberately**: D-483 forbids a MEASURED number,
   and an error rate adopted from a committed config is a convention rather than
   a measurement — while deferring it is the exact deferral that killed the
   previous §8. The two recalls it is applied to ARE measured, and neither is
   restated here.
2. **THE NULL IS THE INCUMBENT RECALL**, the fraction of win-direction proofs the
   best written ordering keeps, registered in the closed detector arc and cited
   there rather than restated here (D-423, D-531).
3. **THE ALTERNATIVE IS THE LOWER END OF THE ARC'S MEASURED COLUMN BOUND**, also
   registered in the closed arc and also not restated. Revision 1 said *"the
   target recall the detector must beat"* and gave it no referent: the arc
   registers an incumbent and a BOUND, not a target. This rule names the bound's
   lower end, because it is the only figure in the record that says what a score
   over the census columns could reach.

**The minimum is then the smallest number of win-proving firings on disjoint
positions at which a two-proportion test at that level and power separates those
two recalls.** Nothing in it is a choice a successor makes after seeing data.

**WHY A RULE AND NOT A NUMBER.** D-483 forbids a design from carrying a measured
number, and the minimum is computed from two measured recalls, so writing it here
would be exactly that. **The rule is the thing that has to land early**, because
the defect it forecloses is a threshold chosen after seeing which positions
proved — and every input above is either a convention already in force or a
figure a CLOSED arc registered, so **none of them can be tuned by the corpus the
rule will be applied to.** This is `docs/experiments/book_v2_registration.md`'s
own discipline — the decision rule registered before the sweep, the sweep before
the size (D-518) — applied to a sample instead of a book, which is what D-537
asks for in those words.

**AND THE DIRECTION OF ITS BIAS IS NAMED, BECAUSE A RULE THAT HIDES ITS OWN
ANTI-CONSERVATISM IS NOT A GUARD.** The bound's lower end is the LARGEST effect
size the arc licenses, so it yields the SMALLEST minimum. **The number this rule
produces is therefore a FLOOR**: a pre-registration opening round 3 may register a
larger minimum with grounds and **may never register a smaller one.**

**WHAT COUNTS AS DISJOINT IS `key_full`.** D-537's denominator is *"win-proving
firings on DISJOINT POSITIONS"*, and this corpus defines three notions of
sameness (§2.1). **The coarsest must rule**, or two firings on one position
reached two ways would count as two: `key_full` folds transposition and symmetry
together and loses nothing on an asked position (§2.1). **WP-2.0b's identity form
must be consistent with it** — that package chooses *"the full-turn 128-bit key
per D-8, or the canonical move-list prefix"*
(`docs/experiments/wp20_dispatches.md`), and neither of those is `key_full`, so
this is a constraint that package inherits and not an observation this one makes.

**AND IF NO CANDIDATE CLEARS, THE RULE IS AMENDED AND NEVER EXTRAPOLATED** —
D-518's own off-the-end clause. One reviewed amendment, recorded, with the reason
the minimum moved; a minimum reached by extending a curve past its data is the
post-hoc move in a different coat.

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

Every row refuses the WHOLE run, and the table is the one place they are stated
(D-423). Ordered by when they can first fire.

| # | condition | when |
|---|---|---|
| 1 | the capture's `source_sha256` is not the digest of the report given | before any record is read |
| 2 | the capture's schema version is not the one WP-2.0-M writes | before any record is read |
| 3 | the capture header's `capture_sha256` is not the one its own three inputs produce (§5) | before any record is read |
| 4 | a capture record's field count is wrong, or a field carries a TAB, or a field is empty where §2.10 does not allow it | naming the record |
| 5 | a capture record's `turn` disagrees with its `moves` prefix length | naming the record |
| 6 | a capture record names a game the report does not hold | naming the record |
| 7 | a report game holds NO capture record | naming the game |
| 8 | the totals line's `score` tag is none of the three, or no number follows it | naming the record |
| 9 | a `mate_in` or `mated_in` value is negative | naming the record |
| 10 | the solver block is present as any subset of its six fields rather than all or none (§2.4) | naming the record |
| 11 | a captured `moves` prefix is not a legal game under pistol-core | naming the record |
| 12 | the derived outcome disagrees with the report's own `result` (§2.7) | naming the game |

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
   protocol's three spellings and never carrying the word `cp`; and the node
   columns are the two independent counters, **never the `nodes` the wire prints
   as their sum** — the gate-off case excepted, where the sum and its first term
   are the same number and §2.4 says so.
4. **Splitting `totals_of` changes no output**: `totals_of`'s three lookups stay
   load-bearing, and the SPRT path still bills each game's compute from the
   totals line.
5. **A re-run of the transform over one capture and one report produces a
   byte-identical corpus file.**
6. **A corpus file round-trips through its own loader FIELD BY FIELD**, and one
   whose schema version, body digest, record arity, header params or token set is
   wrong is refused by name.
7. **Any failure refuses the whole run**; no record is silently skipped, and no
   report game goes unrepresented.
8. **The derived outcome agrees with the report's own `result` on every
   non-forfeited game**, by a relation that reads no seating, or the run is
   refused.
9. **The corpus deduplicates nothing**, and carries the three keys that let a
   consumer fold.
10. **Every record says whether its position is book**, by the arena's own
    boundary.

**INVARIANT 2 IS THE ONE NO TEST PINS, AND THIS DOCUMENT SAYS SO RATHER THAN
REGISTERING A TEST THAT CANNOT FAIL.** Revision 1 registered
`the_transform_spawns_no_process_and_reads_no_clock`; **no in-process Rust test
observes the absence of a `Command::new` or an `Instant::now` on a path it does
not take**, so that test passes whatever the code does — which
`docs/process.md` calls a criterion that is not one, and which WP-2.0-M refused to
register for the same reason. **Its evidence is the diff**: `labels.rs` reaches
`pistol-core`, `crate::transcript`, `crate::exchange` and
`pistol_cli::corpus::emit`, and nothing else; a reviewer reads the module's `use`
list and the claim is settled there. INVARIANT 5's test is what would catch a
clock read that changed an output.

---

## 11. TESTS AND MUTANTS

| test | pins |
|---|---|
| `every_capture_record_produces_one_corpus_record_in_order` | 1 |
| `a_cp_score_becomes_an_eval_column_and_not_a_cp_one` | 3 |
| `a_mate_score_becomes_mate_in` | 3 |
| `a_negative_mate_score_becomes_mated_in` | 3 |
| `a_score_tag_with_no_number_after_it_refuses_the_run_by_name` | 3, 7 |
| `a_totals_line_without_solver_fields_yields_all_nodes_as_search_nodes` | 3 |
| `a_totals_line_with_solver_fields_yields_the_two_counters_it_carries` | 3 |
| `a_partial_solver_block_refuses_the_run_by_name` | 3, 7 |
| `the_sprt_reports_per_game_node_counts_survive_the_totals_of_split` | 4 |
| `totals_of_still_refuses_a_line_missing_nodes_time_or_depth` | 4 |
| `fields_of_reads_a_captured_line_that_has_no_time_field` | 4 |
| `fields_of_gives_the_word_after_score_and_the_word_after_that` | 4 |
| `a_rerun_over_one_capture_and_report_is_byte_identical` | 5 |
| `a_corpus_file_round_trips_through_its_own_loader_field_by_field` | 6 |
| `a_corpus_whose_body_digest_is_wrong_is_refused_by_name` | 6 |
| `a_corpus_record_with_the_wrong_field_count_is_refused_by_name` | 6 |
| `a_corpus_record_with_an_empty_field_is_refused_by_name` | 6 |
| `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` | 6 |
| `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name` | 6 |
| `a_score_kind_outside_the_three_is_refused_by_name` | 6 |
| `a_negative_mate_value_is_refused_by_name` | 6 |
| `a_key_pos_that_is_not_thirty_two_hex_digits_is_refused_by_name` | 6 |
| `a_capture_whose_source_digest_is_not_the_reports_is_refused_by_name` | 7 |
| `a_capture_whose_header_identity_is_not_its_own_inputs_is_refused_by_name` | 7 |
| `a_capture_record_naming_a_game_the_report_lacks_is_refused_by_name` | 7 |
| `a_report_game_with_no_capture_record_refuses_the_run_by_name` | 7 |
| `a_record_whose_turn_and_moves_disagree_is_refused_by_name` | 7 |
| `the_derived_outcome_agrees_with_the_reports_own_result_field` | 8 |
| `a_report_whose_result_contradicts_its_moves_refuses_the_run_by_name` | 8 |
| `the_outcome_check_holds_when_engine_b_takes_seat_one` | 8 |
| `a_forfeited_games_result_is_the_reports_and_is_not_derived` | 8 |
| `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` | 8 |
| `two_transposed_positions_are_two_records_sharing_a_key_full` | 9 |
| `two_positions_alike_up_to_a_symmetry_share_a_key_full_and_not_a_key_pos` | 9 |
| `a_book_position_is_a_record_flagged_book` | 10 |
| `the_first_position_after_the_book_is_flagged_not_book` | 10 |
| `the_turn_zero_record_writes_a_dash_for_its_three_empty_columns` | §2.10 |

**MUTANTS:**

| mutation | the test that dies |
|---|---|
| `score_kind` collapsed so `mate` and `-mate` share a token | `a_negative_mate_score_becomes_mated_in` |
| the score column emitted as the wire's `cp` | `a_cp_score_becomes_an_eval_column_and_not_a_cp_one` |
| **the score read as the word after its key alone** (revision 1's field map) | `a_mate_score_becomes_mate_in` and `fields_of_gives_the_word_after_score_and_the_word_after_that` |
| `search_nodes` sourced from the line's `nodes` when the solver block IS present | `a_totals_line_with_solver_fields_yields_the_two_counters_it_carries` |
| a partial solver block accepted with the missing fields defaulted | `a_partial_solver_block_refuses_the_run_by_name` |
| a fourth load-bearing lookup added to `totals_of` | `the_sprt_reports_per_game_node_counts_survive_the_totals_of_split` |
| `time` made non-fatal in `totals_of` | `totals_of_still_refuses_a_line_missing_nodes_time_or_depth` |
| **a corpus schema field dropped on write** | `a_corpus_file_round_trips_through_its_own_loader_field_by_field` |
| a corpus record's fields reordered on write | the same test |
| the loader's body-digest check removed | `a_corpus_whose_body_digest_is_wrong_is_refused_by_name` |
| the loader's empty-field check removed | `a_corpus_record_with_an_empty_field_is_refused_by_name` |
| the loader's schema-version check removed | `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` |
| the three unit params dropped from the header | `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name` |
| the capture/report digest binding removed | `a_capture_whose_source_digest_is_not_the_reports_is_refused_by_name` |
| the header's `capture_sha256` copied instead of re-derived | `a_capture_whose_header_identity_is_not_its_own_inputs_is_refused_by_name` |
| a report game with no records passed over | `a_report_game_with_no_capture_record_refuses_the_run_by_name` |
| **the outcome relation gated on `a_is_p1`** (revision 1's own defect) | `the_outcome_check_holds_when_engine_b_takes_seat_one` |
| the pistol-core outcome cross-check removed | `a_report_whose_result_contradicts_its_moves_refuses_the_run_by_name` |
| `result` and `end` collapsed into one column | `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` |
| `to_move` taken from the record's `turn` column without replaying | `a_record_whose_turn_and_moves_disagree_is_refused_by_name` |
| the book boundary read as `k <= opening_turns` | `the_first_position_after_the_book_is_flagged_not_book` |
| the `book` column dropped | `a_book_position_is_a_record_flagged_book` |
| records deduplicated by `key_full` on write | `two_transposed_positions_are_two_records_sharing_a_key_full` |
| the turn-zero sentinel written as an empty field | `the_turn_zero_record_writes_a_dash_for_its_three_empty_columns` |
| the record order taken from a hash map rather than the capture | `a_rerun_over_one_capture_and_report_is_byte_identical` |

**TWO MUTANTS REVISION 1 REGISTERED ARE REMOVED BECAUSE THEY CANNOT DIE.**
*"`to_move` computed from turn parity"* is a no-op: parity and pistol-core agree
on every legal prefix (§2.2), so the mutation changes no output. And
`the_transform_spawns_no_process_and_reads_no_clock` is removed as a test for the
reason §10 gives. **A mutant that cannot die and a test that cannot fail are the
same defect from two ends**, and this arc has registered both before.

**THE VACUITY THIS PACKAGE MUST NOT WALK INTO**, named because the arc has paid
for it four times (D-527, and WP-2.0-M's §14): **none of these tests needs a real
engine, and that is a property of the subject rather than a shortcut.** The
transform reads FILES, so a fixture capture and a fixture report exercise every
path — including the solver spelling, which no engine in this workspace can
produce because every committed config has the solver off the search path
(`configs/gate_v0.toml:94`), and which a synthetic totals line produces for free.

**THE ONE TEST WHOSE FIXTURE MUST NOT BE HAND-WRITTEN** is
`the_derived_outcome_agrees_with_the_reports_own_result_field`: if the fixture's
`result` field were written by the same hand as the expectation, the test would
agree with itself. **Its reports are produced by the arena**, so the `result` it
checks against is the arena's own. §2.7 states what that check is and is not.

---

## 12. THE CORPUS'S OWN MANIFEST, THE THROUGHPUT SHAPE, AND WHAT IS NOT DECIDED

### 12.1 The corpus manifest — requirement 5, for the artifact this package writes

WP-2.0-M discharges requirement 5's *"corpus manifest with digests"* **for the
capture**, one row per capture in `docs/label_corpus_manifest.md`
(`docs/experiments/wp20m_design.md` §13). **This package writes a DIFFERENT
uncommitted artifact and revision 1 gave it no row**, which is the matrix's own
open decision 12 (*"the corpus manifest's digest boundary"*,
`docs/experiments/matrix_wp20_pipeline_shape.md` §5) falling between two packages.

**THE LABELS MODE PRINTS A CORPUS MANIFEST ROW ON STDOUT AND NEVER WRITES THE
FILE**, on WP-2.0-M's own ground: `pistol-arena` writes nothing inside the
repository (`crates/pistol-arena/src/lib.rs:41-45`), and printing rather than
retyping is D-543's remedy applied to a ledger row. The row carries the corpus's
body digest, the corpus schema version, the capture's `capture_sha256`, the
report's `experiment_sha256` and `source_sha256`, and the artifact's path — **the
digest boundary decision 12 asks for, stated: the row binds the corpus to the
capture and the capture's own row binds that to the report, so a reader walks the
chain from a committed file to every artifact in it.** The row is added to
`docs/label_corpus_manifest.md` in the commit that records the run, and
`a_labels_run_prints_a_corpus_manifest_row_naming_its_digests` pins its shape.

### 12.2 The throughput expectation, stated as a shape

The dispatch asks for *"throughput expectation stated as a shape, measured in the
pilot, never guessed (D-500's class)"*. **Revision 1 had it in no section and no
deferral list, and WP-2.0-M did not have it either** — the "falls between" defect
on a requirement the split's own governing text names.

**THE SHAPE, for the pipeline end to end.** Pass 1 costs one game at the GAME
budget per game. Pass 2 costs, per asked position, **one `newgame` plus one `go`
at the LABEL budget** — and the `newgame` is a memset of the whole transposition
table, whose size the committed seat sets, which is why WP-2.0-M refuses to guess
it (D-500). The number of asked positions is the number of turn boundaries in the
report, less one per won game. **This transform is a linear pass over the capture
with one pistol-core replay per record and no process and no engine**, so its cost
is dominated by pass 2's and is not the throughput anyone measures.

**THE MAGNITUDES ARE THE PILOT'S** — games per hour and labels per hour — and are
measured there rather than stated here, which is where D-483 and D-500 between
them put them.

### 12.3 What this package does not decide

**Not decided here, and correctly elsewhere:** the label budget's value and the
pilot's `book_v2` range (the pilot's pre-registration, D-483); the asked set,
coldness, the normalisation and the capture identity (WP-2.0-M); the census
identity column and the census logging flag (WP-2.0b, D-539); any train/test
split, which is a training decision over a corpus and not a property of one; and
whether a trainer uses book positions, which §2.9 flags rather than decides.

**RESIDUE, NAMED SO A SUCCESSOR FINDS IT RATHER THAN REDISCOVERS IT:**

1. **`Provenance` is not on the wire**, so no corpus column can carry it (§2.8),
   and `depth_turns` therefore carries two meanings distinguished by
   `search_nodes == 0` (§2.5). Putting provenance on the wire is an engine diff
   and belongs with WP-2.0b's protocol work if anyone wants it.
2. **Two ADR acts are owed at landing**, and hard rule 10 wants them rather than
   the drift. (a) D-542 records branch B as widening `totals_of` so *"`score` and
   `pv` come out of the one parser"*; **this design builds `fields_of` and takes
   no `pv`**, so the recorded mechanism describes something no package ships.
   (b) D-542 records row (g) as **two-pass** with a **third arm**; this design
   adds a third pass and a fourth arm (§1).
3. **WP-2.0b's identity form is constrained by §8**: its disjointness must be
   `key_full`'s, and neither candidate that dispatch names is that key.
4. **The six wire fields no column takes** are listed in §2.8, and the four
   solver call counters among them are the census's quantities, which WP-2.0b
   owns.
