# WP-2.0 — DESIGN: the label pipeline as a second pass over an arena report

**Governed by** `docs/experiments/wp20_dispatches.md` (WP-2.0), the selection
`docs/experiments/matrix_wp20_shape_selection.md` (row (g), branch B), and
D-539–D-543.

**D-483 binds this document: mechanisms, invariants and tests only.** Every
numeric value — the label budget, the pilot's range, the census minimum's grid —
lands in a pre-registration, not here. Where this document names a quantity it
names the QUANTITY and not its value.

**No engine diff.** Only `pistol-arena` changes. No committed config moves, and
nothing new reaches the wire.

---

## 1. THE SHAPE

**Two passes, and the first one already exists.**

- **PASS 1 — PLAY.** `arena --config <experiment> --out <report>` on the
  **unmodified** SPRT path, with both engine sections naming the same committed
  config. Self-play is expressible today: `validate_engines` refuses only
  identical *labels*, not identical binaries or configs, and the replay tests
  already run both seats from one binary. **This pass gets no new code.**
- **PASS 2 — LABEL.** A third mode of the same binary, beside `--config` and
  `--replay`: read the report pass 1 wrote, walk every game's recorded moves, and
  at each to-move position send `newgame`, then the position, then a `go` at the
  **label** budget — writing one record per position.

**Why two passes and not one.** A one-pass shape must either warm the label (a
`go` on the table the game's own search just filled) or clear the table inside a
live game, which changes the game. Two passes make coldness a property of the
SHAPE: a `newgame` in pass 2 lands between recorded games and disturbs nothing,
because pass 1 is already over and written to disk.

## 2. WHAT PASS 2 READS, AND WHAT IT MUST RECOVER

`Transcript` carries the engines, both identities, the run's `go_line` and
`budget_nodes`, `opening_turns`, `turn_cap`, `hang_timeout_ms`,
`experiment_sha256`, `source_sha256`, and the games with their full move lists.

**Every to-move position is a PREFIX of a recorded move list**, which is exactly
what `position_line` sends — so pass 2 needs no position reconstruction beyond
truncation.

**What the reader drops, and how the design recovers it.** `transcript::read`
does not surface `result`, `forfeit_by`, or the `openings_*` counts that the
dispatch's requirements 2 and 5 name.

- **`result`** — recovered by replaying the move list through `pistol-core` and
  reading `GameState::outcome`, which is the referee rule 2 requires anyway.
  **INVARIANT 1** below pins that this is the only source of a game's outcome.
- **A FORFEITED game cannot have its outcome recovered that way**, because the
  forfeit is a fact about the protocol and not about the position. **Pass 2
  therefore SKIPS forfeited games and records the skip**, rather than labelling
  positions from a game whose end it cannot describe.
- **`openings_*`** — the ledger's business (§7), taken from the pass-1 config
  and the report's own `opening_turns`, never re-derived.

## 3. THE RECORD, AND ITS SCHEMA

**Sink**: `pistol_cli::corpus::emit::Fixture`, the same plain-text-plus-in-band-
`body_sha256` shape `openings_v1.txt` and `bench_positions_v1.txt` already use.
It needs no dependency `pistol-arena` lacks, and it gives the corpus the digest
discipline every other fixture in this tree has.

**Header** — `param` for every input, `derived` for every computed count, per
that type's own rule that a reader must be able to tell a choice from a
measurement.

**One body line per record**, carrying:

| field | what it is |
|---|---|
| position | the canonical move list (D-6), as the `position` verb spells it |
| side to move | derived from the move list, recorded because a consumer should not have to re-derive the rules to read the corpus |
| label score | the engine's score for the position, in the units §4 fixes |
| label best move | the `bestmove` line's turn |
| label depth | `depth_turns` from the totals line |
| label nodes | `nodes` from the totals line |
| game outcome | from INVARIANT 1's referee |
| game index and turn index | so a record points back into the report it came from |

**Schema version is a `param` in the header**, and the loader refuses a version
it does not know rather than reading a field it does not have (rule 3).

## 4. THE SCORE FIELD'S NAME — the memo's decision 10, taken

**The score is NOT centipawns, and the schema does not call it `cp`.** Three
facts the corpus would otherwise silently mis-record:

1. the unit is the engine's own evaluation unit, and `report.rs`'s `cp` token is
   a protocol spelling rather than a claim about pawns;
2. the sign is **root side-to-move relative**, so a record's score is about the
   side named in its own `side to move` field and not about P1;
3. `mate T` counts **both sides' turns**, so a mate score is a distance in the
   rules' own unit and not a ply count.

**The schema names the field for what it is** — an engine score, side-to-move
relative, with mate distances in turns — and **INVARIANT 4** pins the sign
convention against a fixture whose expected sign is known by construction. The
memo's warning is the reason: a wrong name here makes every label in the corpus
wrong in a way no loader test catches, because the loader would still parse it.

## 5. WHAT IDENTIFIES A LABELLING RUN — the memo's decision 4, resolved

The arena deliberately excludes `--workers` from `experiment_sha256` because it
is a **run mechanic**: two runs of one experiment at one worker and at eight are
the same experiment.

**The label budget is NOT a run mechanic. It changes the labels.** So a label
budget arriving as `--workers` does would be a parameter of the experiment
sitting outside the digest that identifies it — which is the defect the arena's
own boundary comment exists to prevent, arriving from the other side.

**MECHANISM.** Pass 2 computes and records a `label_sha256` over the canonical
concatenation of: the source report's `source_sha256`, the label `go` line, both
engine identities, and the schema version. The label budget still arrives on the
command line — there is no config document in this mode and rule 1 forbids a
code-side default — but it is **inside the run's identity**, not outside it.

## 6. COLDNESS

**MECHANISM.** `newgame` before every label `go`. `Table::clear` is a true wipe;
`Searcher::clear` reaches the table, the heuristics and the solver, whose own
table is REBUILT rather than merely epoch-bumped; `position` is rebuilt by
`reset_to` at the top of every search; `params` is immutable; and `census` is
`None` in every shipped path. Nothing that could carry across a position
survives.

**INVARIANT 2** pins it. **And the design does not claim it is free**: a
`newgame` fills every bucket of a table whose size the committed seats set, so
the cost is a memset per label. **The pilot measures it; this document does not
guess it** (D-500).

**WHAT THE PILOT'S PRE-REGISTRATION OWES, AND THIS DESIGN DOES NOT SUPPLY**:
D-540's second clause — a **fresh-process agreement criterion**, proving the
construction holds by agreement between a pipeline-produced label and the same
position re-scored in a fresh process. It must **name the defect class it
excludes**, because a criterion that is a property the named defect preserves
passes vacuously — which is D-527's own failure and the precedent D-540 cites.

## 7. LABEL POLICY, DEDUP, SEEDS AND LEDGERS

**LABEL POLICY.** Every to-move position of every non-forfeited, non-book turn
is a candidate. Whether all candidates are labelled or a fraction is sampled is a
**registered rule in the pilot's pre-registration**, not a default here.

**SEEDS.** D-540 fixes that seeds attach to **pipeline sampling only** — the
search is deterministic in instrument mode by hard rule 4, so a seed on the
engine's path would be a knob where the law says there is none. **The seed's one
job is to choose which candidate positions are labelled** when the policy samples.
When the policy labels every candidate, the seed is recorded and inert, and
**INVARIANT 5** pins that inertness rather than leaving it as a claim.

**DEDUP.** Records are keyed by the **canonical move list** (D-6). Two records
whose move lists differ but whose boards agree — a transposition — are **kept as
two records and marked with a shared board key**, because the corpus's consumer
is a trainer that may want either treatment and a pipeline that collapses them
has destroyed the choice. **The count of distinct boards is a `derived` header
field**, so the collapse is available without being imposed.

**LEDGERS.** Three, all append-only: the `book_v2` consumed-ranges ledger (shared
with the SPRT ledger, per its own rule that a row lands in the same commit as the
config that consumes it); a corpus manifest with per-file digests; and the census
corpus manifest, which stays EMPTY until WP-2.0b lands (D-539).

## 8. THE CENSUS-MINIMUM RULE FOR DETECTOR ROUND 3 (D-537)

**This rule lands NOW, before any corpus exists, so it cannot be fitted later.**
That is the whole reason it is in this design and not in the package that will
benefit from it.

**THE QUANTITY** is the one D-537 names and no other: **win-proving firings on
DISJOINT POSITIONS**. Not firings, not games, not proofs of either direction.

**THE RULE'S FORM**, in `book_v2`'s own shape (D-518: the decision rule
registered before the sweep, the sweep before the size):

1. The corpus is split by POSITION into a FIT part and a HELD-OUT part, disjoint
   by construction — never by firing, because firings repeat positions and a
   split by firing would leak.
2. A precision score is fitted on the FIT part alone.
3. Its win recall is measured on the HELD-OUT part at the band's own visit
   budget.
4. **The minimum N is the smallest value in a registered grid at which the
   held-out recall's lower confidence bound clears a registered floor.**
5. **Detector round 3 re-opens when the corpus holds at least N**, and not
   before.

**WHAT LANDS HERE AND WHAT DOES NOT.** The FORM above is fixed by this design.
The **grid, the floor and the confidence level are numbers** and land in a
registration written before the corpus reaches any candidate size — never after,
and never by the session that wants the answer. **D-483 is why they are not
here**, and D-537's *"before any score is fitted"* is why they may not wait for
the corpus.

## 9. THROUGHPUT — a SHAPE, and the pilot measures it

The pipeline's cost is **one label `go` plus one table clear per labelled
position**, over the positions of a game, over the games of a report. That is the
shape. **Games per hour and labels per hour are MEASURED in the pilot and are
not estimated here** (D-500's class, and the premise memo's own refusal to
extrapolate nps to a rate).

## 10. INVARIANTS

1. **A game's outcome comes from replaying its move list through `pistol-core`
   and from nothing else.** Rule 2 forbids a second win detector.
2. **Every label `go` is preceded by a `newgame` on that channel**, and no label
   `go` follows another without one.
3. **A forfeited game contributes no records**, and its skip is counted in the
   header.
4. **A record's score is side-to-move relative**, with mate distances in turns.
5. **When the label policy labels every candidate, the seed changes no output
   byte.**
6. **Pass 2 never plays a move**: it asks, records and advances along the
   recorded list, so it cannot produce a position pass 1 did not.
7. **The SPRT path's output is unchanged** by the `totals_of` widening: the three
   existing lookups stay load-bearing, `score` and `pv` are non-fatal `Option`s.

## 11. TESTS, named for the behaviour they pin

- `a_record_round_trips_through_the_loader`
- `the_loader_refuses_a_schema_version_it_does_not_know`
- `the_loader_refuses_a_record_missing_a_field_by_name`
- `a_forfeited_game_contributes_no_records_and_is_counted`
- `every_label_go_is_preceded_by_a_newgame`
- `a_transposition_is_two_records_sharing_one_board_key`
- `the_seed_changes_no_byte_when_every_candidate_is_labelled`
- `a_labelling_run_is_identified_by_its_source_report_and_its_budget`
- `the_ledger_appends_and_never_rewrites`
- `widening_totals_of_leaves_the_sprt_report_byte_identical`
- `a_score_is_recorded_from_the_movers_point_of_view`

**MUTANTS, each with the test that must die:**

| mutation | the test that dies |
|---|---|
| a schema field dropped from the writer | the loader round-trip |
| the `newgame` removed from pass 2's loop | `every_label_go_is_preceded_by_a_newgame` |
| the seed read on a label-everything policy | the seed-inertness test |
| the ledger opened for write instead of append | the append test |
| a new `totals_of` lookup made load-bearing with `?` | the SPRT byte-identity test |
| the score's sign flipped | `a_score_is_recorded_from_the_movers_point_of_view` |

## 12. WHAT THIS DESIGN DOES NOT DECIDE

The label budget's VALUE; the pilot's `book_v2` range; the label policy's
sampling fraction; the census minimum's grid, floor and confidence level; and
D-540's fresh-process agreement criterion. **All five are numbers or registered
criteria and all five belong to the pilot's pre-registration** (D-483).
