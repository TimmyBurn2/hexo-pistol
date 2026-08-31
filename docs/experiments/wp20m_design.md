# WP-2.0-M — DESIGN: the labelling pass, capturing verbatim

**What this package is.** The half of WP-2.0 that **two independent reviews
verified and could not break** (D-544): the two-pass shape, coldness, branch B's
widening, and a labelling run's identity. It carries no schema decision.

**THE ONE RULE THAT MAKES THIS PACKAGE POSSIBLE.** Every finding that killed
WP-2.0's design was an INTERPRETATION question — what a column means. **So this
package interprets nothing.** It writes the engine's answer **exactly as the
engine wrote it**, beside the position it was asked about and the identity of the
run that asked. **A capture that adds no meaning cannot be wrong about meaning**,
and WP-2.0-S decides the columns afterwards from bytes this package did not
reshape.

**D-483 binds this document: mechanisms, invariants and tests only.** No numbers.

**No engine diff.** Only `pistol-arena` changes.

---

## 1. THE MECHANISM

A third arm in `bin/arena.rs`'s mode match, beside `--config` and `--replay`.

**Pass 1 — PLAY, and it gets no new code.** `arena --config <experiment> --out
<report>` on the unmodified SPRT path, both engine sections naming the same
committed config. Self-play is expressible today: `validate_engines` refuses
identical *labels*, not identical binaries or configs.

**Pass 2 — CAPTURE.** Read the report pass 1 wrote. For each game, for each
to-move position, on one channel: send `newgame`, send the position, send a `go`
at the **label** budget, and read to `bestmove`. Write one line per position
holding the position, the **verbatim `info totals` line**, the **verbatim
`bestmove` line**, and the game and turn indices.

**WHY VERBATIM.** The totals line already carries every field a label needs and
several whose meaning is contested — `nodes` is `search_nodes + solver_nodes`,
the score has three spellings, and the solver fields appear only when the solver
was consulted. **This package does not decide which of those means what.** It
preserves the bytes; WP-2.0-S parses them. A consumer that disagrees with
WP-2.0-S can re-parse the capture without re-running the engine, which is the
property that makes an expensive corpus survive a schema mistake.

## 2. WHAT PASS 2 READS

`Transcript` carries the engines, both identities, the run's `go_line`,
`opening_turns`, `turn_cap`, `experiment_sha256`, `source_sha256`, and every
game's full move list. **Every to-move position is a PREFIX of a recorded move
list**, which is what `position_line` sends — so pass 2 truncates and does not
reconstruct.

**`transcript::read` legality-checks every game through `pistol-core` at read
time**, refusing the whole report on an illegal turn or on moves after a win. So
**every move list pass 2 walks is a guaranteed legal prefix before pass 2
exists** — it cannot be handed something that panics. This is the property the
scoped re-review confirmed as its own strongest failed attack.

**Which positions are captured**: every to-move position of every game, book
turns and forfeited games included. **This package excludes nothing**, because
every exclusion rule in WP-2.0's design was a finding — and an exclusion is a
meaning decision, which is WP-2.0-S's. A consumer filters; a capture does not.

## 3. WHAT IDENTIFIES A CAPTURE RUN

Pass 2 computes a `capture_sha256` over the canonical concatenation of: the
source report's **`experiment_sha256`**, the label `go` line, both engine
identities, and this package's format version.

**Not `source_sha256`**, which digests the whole report file including its timing
block: two capture runs over reports of one experiment taken on different days
would otherwise have different identities for a reason that changes no answer.
`experiment_digest` is the arena's own answer to that question and this design
takes it rather than inventing a second.

**Nothing about SAMPLING is in it, because this package samples nothing** and
takes no seed. WP-2.0-S's sampling rule, when it exists, extends the digest.

## 4. COLDNESS

**MECHANISM.** `newgame` before every label `go`. Verified end to end by two
reviewers: `Table::clear` is a true `fill(EMPTY)`, not the epoch bump beside it;
`Solver::reset` rebuilds its table rather than bumping an epoch;
`Position::reset_to` unwinds the eval and replaces the `ThreatState`; `params` is
immutable; `census` is `None` in every shipped path; and the `PvTable` is
per-`Run`, not a `Searcher` field. **Nothing that could carry across a position
survives.**

**COST, NOT CLAIMED FREE.** A `newgame` fills every bucket of a table whose size
the committed seats set, so it is a memset per captured position. **The pilot
measures it** (D-500).

**WHAT THE PILOT'S PRE-REGISTRATION OWES**: D-540's second clause — a
**fresh-process agreement criterion**, proving the construction holds by agreement
between a pass-2 capture and the same position re-asked in a fresh process, and
**naming the defect class it excludes**, because a criterion that is a property
of the named defect passes vacuously (D-527).

## 5. THE LABEL SEAM — branch B

`exchange::totals_of` rises to `pub(crate)`. **Pass 2 does not use it.** It reads
the totals line verbatim off the channel, because parsing is meaning and this
package has none.

**What the widening is for**: WP-2.0-S needs one parser rather than two, and
raising visibility now — with no behaviour change — means that package adds
fields to an existing function instead of writing a second reader and inheriting
row (b)'s kill condition. **INVARIANT 6** pins that this visibility change alters
no output.

## 6. INVARIANTS

1. **Every label `go` is preceded by a `newgame` on that channel**, and no label
   `go` follows another without one.
2. **Pass 2 never plays a move.** It asks, records, and advances along the
   recorded list, so it cannot produce a position pass 1 did not.
3. **The captured totals and bestmove lines are byte-identical to what the engine
   wrote**, less the trailing newline. No field is reordered, renamed, dropped or
   combined.
4. **Every to-move position of every game is captured**, with no exclusion by
   book, forfeit, outcome or turn.
5. **A re-run of pass 2 over one report at one label budget produces a
   byte-identical capture file.**
6. **Raising `totals_of` to `pub(crate)` changes no output**: the SPRT path's
   report is byte-identical across the change.
7. **Pass 1 is unmodified.** No file on the SPRT path changes behaviour.

## 7. TESTS

- `every_label_go_is_preceded_by_a_newgame`
- `a_captured_totals_line_is_byte_identical_to_what_the_engine_wrote`
- `a_capture_covers_every_to_move_position_of_every_game`
- `a_forfeited_games_positions_are_captured_like_any_other`
- `a_book_turns_position_is_captured_like_any_other`
- `a_rerun_over_one_report_is_byte_identical`
- `a_capture_run_is_identified_by_its_experiment_and_its_budget`
- `two_reports_of_one_experiment_share_a_capture_identity`
- `raising_totals_of_leaves_the_sprt_report_byte_identical`
- `the_capture_refuses_a_report_it_cannot_read_by_name`

**MUTANTS, each with the test that must die:**

| mutation | the test that dies |
|---|---|
| the `newgame` removed from pass 2's loop | `every_label_go_is_preceded_by_a_newgame` |
| a totals field reordered or renamed on write | `a_captured_totals_line_is_byte_identical...` |
| forfeited games skipped | `a_forfeited_games_positions_are_captured_like_any_other` |
| book turns skipped | `a_book_turns_position_is_captured_like_any_other` |
| `source_sha256` used in the identity | `two_reports_of_one_experiment_share_a_capture_identity` |
| the label budget dropped from the identity | `a_capture_run_is_identified_by_its_experiment_and_its_budget` |
| a `totals_of` lookup made load-bearing | `raising_totals_of_leaves_the_sprt_report_byte_identical` |

## 8. WHAT THIS PACKAGE DOES NOT DECIDE

The label budget's VALUE; the pilot's `book_v2` range; and **every question of
meaning** — provenance, the score's representation, how many node columns exist,
which positions a trainer should use, transposition dedup, and the
census-minimum rule. **All of them are WP-2.0-S's**, and this package is
deliberately unable to get them wrong.
