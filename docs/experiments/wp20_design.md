# WP-2.0 — DESIGN: the label pipeline as a second pass over an arena report

**REVISION 2**, after a fresh-context REVIEW-design returned **FAIL** on
revision 1 (`bb44d6f`) — 2 BLOCKING, 8 MAJOR, 11 MINOR. **The shape survived**:
the reviewer re-traced the coldness chain itself and found nothing that carries
across a position, and confirmed branch B is output-neutral because
`totals_of`'s return value has exactly one consumer. **What failed was what the
RECORD CONTAINS**, and both BLOCKING findings are in that half.

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
  reading `GameState::outcome`. **`Outcome` has exactly TWO variants, `Ongoing`
  and `Win`**, so a replay distinguishes a decided game from an undecided one and
  **cannot tell a TURN-CAPPED game from a FORFEITED one**: both end `Ongoing`.
  Revision 1's INVARIANT 1 said the replay is the only source of a game's
  outcome, and for those two it is not enough. **The referee supplies the
  outcome where the game was decided; where the replay says `Ongoing`, the
  record's outcome is `undecided` and the design does not guess which kind.**
- **A forfeited game's positions are not thrown away.** Revision 1 skipped the
  whole game, which discards every position BEFORE the forfeit — and those are
  ordinary positions whose labels are as good as any other. **Pass 2 labels every
  position of every game and marks the game's outcome**; what a forfeit costs is
  the outcome column's precision on that game, not the positions. The count of
  undecided games is a `derived` header field so a consumer can filter on it.
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
| label provenance | whether the answer came from the search or from a solver proof — see §4a |
| game outcome | from INVARIANT 1's referee |
| game index and turn index | so a record points back into the report it came from |

**Schema version is a `param` in the header**, and the loader refuses a version
it does not know rather than reading a field it does not have (rule 3).

## 4a. LABEL PROVENANCE — the memo's decision 9, which revision 1 DROPPED

Revision 1 neither took this decision nor listed it as deferred. It is taken
here, because the reviewer showed the escape hatch the memo offered does not
exist.

**THE HAZARD.** When the solver gate is on, an answer can come from a solver
PROOF rather than from the search, and then `depth_turns` is a **proof depth**
and `nodes` are **solver nodes** — different quantities under the same two column
names, with no discriminator in the record.

**THE MEMO SAID THIS COULD BE EXCLUDED BY CONFIGURATION, AND THAT IS FALSE AT
HEAD.** `LC_ALL=C /usr/bin/grep -rln "on_search_path = true" configs/` returns
**three committed configs** — the WP-1.8c bench seat, the staged solver gate seat
and the play solver seat. D-441's *"gate off in every committed config"* binds
DEPLOYMENT configs, and D-520 says so explicitly; measurement seats exist and are
committed. **So a gate-on pass 1 is a legal use of this design**, and a schema
that assumes otherwise records two different quantities in one column.

**THE MECHANISM, and it needs nothing new on the wire.** `crates/pistol-cli/src/report.rs`
emits `search_nodes`, `solver_nodes` and the call counters **only inside a
`if info.solver_nodes > 0` conditional** — a gate-off line prints none of them.
So the discriminator is already there: pass 2 records the provenance it observes
from whether the solver fields appeared, and records the solver node count
alongside when they did.

**INVARIANT 8** pins that a record whose totals line carried solver fields is
marked, and **INVARIANT 9** pins that the two node quantities are never summed
into one column.

## 4. THE SCORE FIELD'S NAME — the memo's decision 10, taken

**The score is NOT centipawns, and the schema does not call it `cp`.** Three
facts the corpus would otherwise silently mis-record:

1. the unit is the engine's own evaluation unit, and `report.rs`'s `cp` token is
   a protocol spelling rather than a claim about pawns;
2. the sign is **root side-to-move relative**, so a record's score is about the
   side named in its own `side to move` field and not about P1;
3. `mate T` counts **both sides' turns**, so a mate score is a distance in the
   rules' own unit and not a ply count.

**THERE ARE THREE SPELLINGS AND REVISION 1 NAMED TWO.** `score_token` emits
`cp <value>`, `mate <turns>` and **`-mate <turns>`** — and the third is the one
where the TOKEN ITSELF carries the sign that INVARIANT 4 pins, so omitting it
omitted the case the invariant is about.

**AND THE VALUE IS TWO WORDS, WHICH THE EXISTING HELPER CANNOT EXPRESS.**
`totals_of`'s `value()` returns the single word after a key; a score is a keyword
plus a number, and `cp` is that keyword rather than the field's name. **Branch
B's widening therefore parses the score as a PAIR**, and the schema's column is
named for the quantity — an engine score, side-to-move relative, mate distances
in turns — never `cp`.

**THE NON-FATAL `Option` IS RIGHT FOR SPRT AND WRONG FOR THE LABEL PATH, AND
REVISION 1 APPLIED IT TO BOTH.** INVARIANT 7 makes `score` and `pv` non-fatal so
the SPRT path cannot change — correct, and it stays. But a *record* whose score
is absent is a swallowed error, which rule 3 forbids: **pass 2 REFUSES a position
whose totals line carried no parsable score**, loudly and by name, rather than
writing a record with an empty column. One parser, two consumers, two different
obligations — and **INVARIANT 10** pins the label side so that a build in which
every score parsed as `None` fails a test instead of emitting a corpus of blanks.

The memo's warning is why all of this is spelled out: a wrong name here makes
every label in the corpus wrong in a way no loader test catches, because the
loader would still parse it.

## 5. WHAT IDENTIFIES A LABELLING RUN — the memo's decision 4, resolved

The arena deliberately excludes `--workers` from `experiment_sha256` because it
is a **run mechanic**: two runs of one experiment at one worker and at eight are
the same experiment.

**The label budget is NOT a run mechanic. It changes the labels.** So a label
budget arriving as `--workers` does would be a parameter of the experiment
sitting outside the digest that identifies it — which is the defect the arena's
own boundary comment exists to prevent, arriving from the other side.

**MECHANISM.** Pass 2 computes and records a `label_sha256` over the canonical
concatenation of: the source report's **`experiment_sha256`**, the label `go`
line, both engine identities, the schema version, **the seed, and the sampling
rule** — the last two because the arena's own precedent puts WHICH-selection
inside: `openings_skip` is in `experiment_digest`, one line from the `--workers`
exclusion this design cites. **A seed and a fraction choose which positions get
labelled, so two runs differing in them are different experiments.**

**It hashes `experiment_sha256` and NOT `source_sha256`.** Revision 1 used the
latter, which is a digest of the whole report FILE — timing block included — so
two labelling runs over reports of one experiment taken on different days would
have had different identities for a reason that changes no label. The
experiment digest is the arena's own answer to that same question and this
design takes it rather than inventing a second one.

The label budget still arrives on the command line — there is no config document
in this mode and rule 1 forbids a code-side default — but it is **inside the
run's identity**, not outside it.

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

**THE BOARD KEY IS COMPUTED BY A STATED, ORDER-INDEPENDENT RULE AND NEVER BY
HASH-MAP ITERATION.** Hard rule 4 forbids unseeded hash iteration on any path
that decides an output, and a board key assigned by "first one seen wins" over a
`HashMap` is exactly that: the same corpus would key the same transposition
differently between runs, and INVARIANT 11's byte-identity receipt would fail
intermittently — the worst way for this to be found. **The key is derived from
the board's own contents in a sorted, canonical order**, so it is a function of
the position and of nothing else.

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

**WHAT LANDS HERE AND WHAT DOES NOT — AND REVISION 1 DEFERRED THE PART THAT DOES
THE WORK.** The reviewer measured this against D-518, the model this section
cites: D-518 registered `n_openings = ceil_to_500(P + 500)` **and** the nine-point
grid **and** the power threshold 0.90 **before** the sweep — because *"moving it
afterwards is the post-hoc threshold move"*. Revision 1 fixed only the five-step
form and deferred grid, floor and confidence, which is the half that decides the
answer.

**So the split rule lands HERE, in full, because it is a MECHANISM and not a
number**: the corpus is partitioned by POSITION, never by firing; the partition
is a stated function of the position's own board key and of nothing about its
outcome; and **the held-out part is never read while the score is being fitted**.
A split chosen after seeing which positions prove is the defect this whole
section exists to prevent, and it is not a number.

**What genuinely remains numeric** — the grid, the floor, the confidence level —
lands in a registration that this design **binds to a deadline it can enforce**:
it is written **before the corpus is first counted against any candidate
minimum**, and the pipeline records the count in its manifest, so a registration
arriving later is visibly later. D-483 is why the numbers are not here; the
deadline is why deferring them is not the post-hoc move D-518 warns about.

## 9. THROUGHPUT — a SHAPE, and the pilot measures it

The pipeline's cost is **one label `go` plus one table clear per labelled
position**, over the positions of a game, over the games of a report. That is the
shape. **Games per hour and labels per hour are MEASURED in the pilot and are
not estimated here** (D-500's class, and the premise memo's own refusal to
extrapolate nps to a rate).

## 10. INVARIANTS

1. **A DECIDED game's outcome comes from replaying its move list through
   `pistol-core` and from nothing else** (rule 2 forbids a second win detector);
   **an undecided replay is recorded as undecided**, never guessed into a cap or
   a forfeit.
2. **Every label `go` is preceded by a `newgame` on that channel**, and no label
   `go` follows another without one.
3. **Every position of every game is a label candidate**, forfeited games
   included; the outcome column carries what the referee could establish.
4. **A record's score is side-to-move relative**, with mate distances in turns.
5. **When the label policy labels every candidate, the seed changes no output
   byte.**
6. **Pass 2 never plays a move**: it asks, records and advances along the
   recorded list, so it cannot produce a position pass 1 did not.
7. **The SPRT path's output is unchanged** by the `totals_of` widening: the three
   existing lookups stay load-bearing, `score` and `pv` are non-fatal `Option`s.
8. **A record whose totals line carried the solver fields is marked as
   solver-provenance**, and one whose line did not is marked as search.
9. **Search nodes and solver nodes are never summed into one column.**
10. **Pass 2 refuses a position whose totals line carried no parsable score**,
    by name, and writes no record for it. A build in which every score parsed as
    absent fails a test rather than emitting a corpus of blanks.
11. **A re-run of pass 2 over one report, at one label budget and one seed,
    produces a byte-identical corpus file.** This is requirement 4's receipt and
    it is a test, not a claim.

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
- `a_negative_mate_token_is_read_as_the_mover_being_mated`
- `a_position_with_no_parsable_score_is_refused_by_name`
- `a_solver_provenance_answer_is_marked_and_its_nodes_kept_separate`
- `a_rerun_over_one_report_is_byte_identical`
- `an_undecided_replay_is_recorded_undecided_and_not_guessed`
- `a_board_key_does_not_depend_on_iteration_order`

**MUTANTS, each with the test that must die:**

| mutation | the test that dies |
|---|---|
| a schema field dropped from the writer | the loader round-trip |
| the `newgame` removed from pass 2's loop | `every_label_go_is_preceded_by_a_newgame` |
| the seed read on a label-everything policy | the seed-inertness test |
| the ledger opened for write instead of append | the append test |
| a new `totals_of` lookup made load-bearing with `?` | the SPRT byte-identity test |
| the score's sign flipped | `a_score_is_recorded_from_the_movers_point_of_view` |
| the `-mate` spelling folded into `mate` | `a_negative_mate_token_is_read_as_the_mover_being_mated` |
| an absent score written as an empty column | `a_position_with_no_parsable_score_is_refused_by_name` |
| the provenance mark dropped | `a_solver_provenance_answer_is_marked_and_its_nodes_kept_separate` |
| solver nodes summed into the node column | the same test |
| the board key taken from map iteration order | `a_board_key_does_not_depend_on_iteration_order` |
| the seed dropped from `label_sha256` | `a_labelling_run_is_identified_by_its_source_report_and_its_budget` |

## 12. WHAT THIS DESIGN DOES NOT DECIDE

The label budget's VALUE; the pilot's `book_v2` range; the label policy's
sampling fraction; the census minimum's grid, floor and confidence level; and
D-540's fresh-process agreement criterion. **All five are numbers or registered
criteria and all five belong to a pre-registration** (D-483).

**What is NOT on this list, because revision 1 left it off and a reviewer had to
find it**: the label's PROVENANCE (§4a) and the score's third spelling (§4).
Both were decisions, not deferrals, and neither was taken. A design's deferral
list is only worth what its completeness is worth, so this one is now checked
against the premise memo's twelve item by item, and the two that were missing
are taken above rather than moved here.
