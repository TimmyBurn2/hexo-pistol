# WP-2.0-M — DESIGN: the labelling pass

**REVISION 2**, after a fresh-context REVIEW-design returned **FAIL** on revision
1 (`5064b05`) — 3 BLOCKING, 11 MAJOR, 6 MINOR. **The verdict on revision 1's
central claim was "NO", and it was right.**

**WHAT REVISION 1 CLAIMED AND WHY IT WAS FALSE.** It said the package
*"interprets nothing"* and was *"deliberately unable to get them wrong"*. Two
counter-examples, neither marginal:

- **The verbatim capture cannot be reproducible.** The `info totals` line carries
  `nps` and `time`, which are wall-clock. `tools/determinism.sh` — CI **gate
  9**, this project's hard-rule-4 gate — normalises by stripping exactly
  ` nps <n> time <n>`, with the comment *"`nps` and `time` are the only fields
  two runs may disagree about"*. So revision 1's INVARIANT 3 (byte-identical
  bytes) and INVARIANT 5 (byte-identical re-run) **contradicted each other**.
  And the test registered for it would have **passed vacuously**: the arena's
  stub engine hardcodes `nps: 1, time_ms: 0`, and it is the only engine the
  arena test suite drives — D-527's own defect class, for the third time in this
  arc.
- **"Every to-move position" was undefined and broke at all three boundaries.**
  Ply-versus-turn was never taken; `position start moves` with no turns after it
  is **refused by name**; and a won game's terminal prefix is a decided position
  which `set_position` refuses, so pass 2 would have received an `error`, never
  a `bestmove`, and hung to its watchdog.

**THE CLAIM IS REPLACED, NOT REPAIRED.** This package makes **no decision about
what the score, the node counts or the provenance MEAN**. It **does** make four
decisions about capture, and they are on the face of this document because a
licence that is false suppresses the attack rather than answering it:

| decision | §  |
|---|---|
| WHICH positions are asked | §2 |
| WHICH engine answers | §3 |
| WHAT counts as the same capture | §5 |
| WHAT happens when an ask fails | §6 |

**AND THE SPLIT'S OWN PREMISE IS CORRECTED (D-544).** That line says every prior
failure was an interpretation question. **It is overstated**: five capture
findings from the first review — which seat answers, the budget's kind, the
failure modes, the CLI grammar, and ply-versus-turn — crossed the split line
unfixed. They are taken here.

**D-483 binds this document: mechanisms, invariants and tests only.** No numbers.
**No engine diff**; only `pistol-arena` changes.

---

## 1. THE MECHANISM

**Pass 1 — PLAY, and it gets no new code.** `arena --config <experiment> --out
<report>` on the unmodified SPRT path, both engine sections naming the same
committed config. Self-play is expressible today.

**Pass 2 — CAPTURE.** `arena --capture <report> --out <capture> --go <go line>`.
Read the report, and for each game, for each asked position (§2): send
`newgame`, send the position, send the `go`, read to `bestmove`. Write one line
per position.

**WHERE THE CODE LIVES, because the gate makes it a design question.**
`crates/pistol-arena/src/bin/arena.rs` is **283 lines** against
`tools/file_justification_check.sh`'s hard **300**, which is CI gate 17. A third
mode arm plus its `USAGE` paragraph crosses that. **So pass 2 lives in a new
module `crates/pistol-arena/src/capture.rs`**, and `bin/arena.rs` gains only a
`Mode` variant, a dispatch pattern and a call. **If the `USAGE` text still
carries it over, the `USAGE` constant moves to its own module** — a mechanical
extraction that adds no behaviour and no rule-9 entry, which is preferable to a
justification for a binary that is mostly help text.

## 2. WHICH POSITIONS ARE ASKED — decision 11, TAKEN

**Per TURN, not per ply.** The grounds are three and they agree: game rule 3
makes the turn the unit of play; `depth_turns` — the only depth on the wire — is
in turns; and `PositionSpec::Start` **cannot express a mid-turn position at all**,
so a per-ply capture would need `PositionSpec::Set` and a second position
grammar. Revision 1 assumed this reading as a fact about the domain; it is a
decision and it is taken here.

**THE SET.** Every turn boundary of every recorded game at which the engine can
legally be asked:

- **The initial position is asked as bare `position start`**, never
  `position start moves` — which the engine refuses by name when no turns follow
  it. `exchange::position_line` produces the refused form for an empty slice, so
  **pass 2 does not use it for the empty case.**
- **A DECIDED position is never asked.** `set_position` refuses a won position,
  and asking one would earn an `error` and no `bestmove`. **This is the
  protocol's own precondition, not an exclusion by outcome**: the terminal prefix
  of a won game is not a position any engine can be asked about, so it is not in
  the set at all. Revision 1's INVARIANT 4 forbade "exclusion by outcome" and
  would have forced the hang.
- **Book turns and forfeited games are asked like any other**, because those
  ARE exclusions by meaning and they belong to WP-2.0-S.

**INVARIANT 1** pins the set; **INVARIANT 2** pins that no asked position is
decided.

## 3. WHICH ENGINE ANSWERS — the first review's MAJOR, unfixed across the split

A report names **two** engine sections. A label is an answer by a **named
engine**, so pass 2 must say which, and must not silently pick one.

**MECHANISM.** Pass 2 **refuses a report whose two engine sections are not
identical**, by name. A self-play report — the only kind this pipeline produces
— has identical sections, so the refusal costs nothing it is meant to accept and
forecloses the case where a capture's labels came from two different engines
without saying so.

**AND IT VERIFIES WHAT IT SPAWNED.** The report carries an `EngineIdentity` per
slot, captured by the original run. Pass 2 spawns its engine and verifies against
that identity the way the arena's own replay does, so a capture cannot silently
be taken from a rebuilt binary. **INVARIANT 3.**

## 4. WHAT IS WRITTEN, AND THE ONE NORMALISATION

**One line per asked position**, carrying: the position as sent; the `info
totals` line **as the engine wrote it, less the wall-clock fields**; the
`bestmove` line as the engine wrote it; and the game and turn indices.

**THE NORMALISATION IS THE PROJECT'S OWN AND IS NOT A NEW DECISION.** ` nps <n>
time <n>` is removed, by exactly the rule `tools/determinism.sh` states and gate
9 enforces: *"`nps` and `time` are the only fields two runs may disagree about."*
**This is what makes INVARIANT 6 achievable at all**, and it costs no label —
`nps` and `time` are facts about the machine, not about the position.

**Nothing else is touched.** No field is reordered, renamed, dropped or combined.
`nodes` stays the sum the engine printed; the score keeps whichever of its three
spellings it arrived in; the solver fields appear exactly when the engine printed
them. **Those are the meanings WP-2.0-S decides, and this package still decides
none of them.**

**THE FILE'S SHAPE.** `pistol_cli::corpus::emit::Fixture`: a header of `param`
and `derived` lines, a body of one record per line, and the in-band
`body_sha256` that type appends. **INVARIANT 6 pins the file byte-for-byte, so
the shape is specified rather than left to the implementer** — which revision 1
did not do.

**THE SOURCE IS NAMED ON THE FACE OF THE FILE.** The header carries the source
report's `experiment_sha256` **and** its `source_sha256`, so WP-2.0-S can find
the report that holds the game outcomes and the forfeit flags this capture does
not carry. Without it the outcome would be unrecoverable from the capture alone,
and requirement 2 would fall between the two packages.

## 5. WHAT COUNTS AS THE SAME CAPTURE

`capture_sha256` over the canonical concatenation of: the source report's
**`experiment_sha256`**, the label `go` line, the engine identity pass 2
verified, and this package's format version.

**Not `source_sha256`**, which digests the whole report file including its timing
block: two captures over reports of one experiment taken on different days would
otherwise differ for a reason that changes no answer. `source_sha256` is still
**recorded** (§4) — it is provenance, not identity.

**Nothing about sampling is in it, because this package samples nothing and takes
no seed.** WP-2.0-S extends the digest when it adds a sampling rule.

## 6. WHAT HAPPENS WHEN AN ASK FAILS

Named, because revision 1 left them undesigned while BLOCKING 2 put them on the
main line.

| condition | pass 2's answer |
|---|---|
| the report is unreadable, or its two engine sections differ | **refuse the run**, by name, before spawning anything |
| the spawned engine's identity does not match the report's | **refuse the run**, by name |
| an ask returns `error` | **refuse the run**, by name, naming the game and turn — it means §2's set is wrong, and a capture with a hole is worse than none |
| an ask returns nothing before the watchdog | **refuse the run**, by name |
| the totals line carries no score at all | **capture it as written.** The score's presence is a meaning question and belongs to WP-2.0-S |

**Every failure is a refusal of the whole run and none is a skip**, because a
capture that silently omits positions is a corpus whose gaps are invisible to
its consumer.

## 7. THE LABEL BUDGET'S KIND

**`nodes`, and never `movetime_ms`.** The arena already refuses a movetime budget
in the one place it validates — *"the one refusal this crate exists to make
loudly"* — and the reason applies with more force here: a wall-clock budget makes
a label a fact about the machine, so INVARIANT 6 could never hold. **The VALUE is
a number and belongs to the pilot's pre-registration** (D-483); the KIND is a
mechanism and belongs here.

## 8. THE `totals_of` WIDENING

`exchange::totals_of` rises to `pub(crate)` and **gains nothing in this package.**
Pass 2 does not call it: it captures the totals line without parsing it.

**The visibility change is for WP-2.0-S**, so that package adds fields to one
parser instead of writing a second and inheriting row (b)'s kill condition.
`clippy::redundant_pub_crate` is a nursery lint and gate 4 denies only
`clippy::all`, so the change is not gate-rejected. **INVARIANT 7** pins that it
alters no output.

**Revision 1 registered a mutant for a mutation this package does not make** —
"a `totals_of` lookup made load-bearing" — which cannot die because nothing here
adds a lookup. It is removed.

## 9. INVARIANTS

1. **The asked set is every turn boundary of every recorded game at which the
   engine can legally be asked**, book turns and forfeited games included.
2. **No asked position is decided**, and the initial position is asked as bare
   `position start`.
3. **Pass 2 refuses a report whose engine sections differ, and verifies the
   engine it spawns against the identity the report recorded.**
4. **Every label `go` is preceded by a `newgame` on that channel**, and no label
   `go` follows another without one.
5. **Pass 2 never plays a move.** It asks, records and advances along the
   recorded list.
6. **A re-run of pass 2 over one report at one `go` line produces a
   byte-identical capture file**, wall-clock fields having been normalised out by
   gate 9's own rule.
7. **Raising `totals_of` to `pub(crate)` changes no output.**
8. **Any failure refuses the whole run**; no position is silently skipped.
9. **Pass 1 is unmodified.**

## 10. TESTS

- `the_asked_set_is_every_legal_turn_boundary`
- `the_initial_position_is_asked_without_a_moves_keyword`
- `a_decided_terminal_position_is_never_asked`
- `a_report_whose_engines_differ_is_refused_by_name`
- `a_respawned_engine_that_does_not_match_the_report_is_refused`
- `every_label_go_is_preceded_by_a_newgame`
- `a_captured_totals_line_keeps_every_field_but_nps_and_time`
- `a_rerun_over_one_report_is_byte_identical`
- `two_reports_of_one_experiment_share_a_capture_identity`
- `an_error_answer_refuses_the_run_and_names_the_game_and_turn`
- `a_forfeited_games_positions_are_captured_like_any_other`
- `a_book_turns_position_is_captured_like_any_other`
- `raising_totals_of_leaves_the_sprt_report_byte_identical`

**AND ONE TEST OBLIGATION THAT IS NOT A TEST NAME.** The re-run test **must not
be driven by the arena's stub engine alone**, whose `nps` and `time` are
hardcoded constants: against that engine the normalisation is unobservable and
the test passes whether or not it exists. **It is driven by the real `pistol`
binary**, and the normalisation test above is what makes the vacuity visible.

**MUTANTS:**

| mutation | the test that dies |
|---|---|
| the `newgame` removed from pass 2's loop | `every_label_go_is_preceded_by_a_newgame` |
| the normalisation removed | `a_rerun_over_one_report_is_byte_identical` (real binary) |
| the normalisation widened to strip another field | `a_captured_totals_line_keeps_every_field_but_nps_and_time` |
| the decided-position guard removed | `a_decided_terminal_position_is_never_asked` |
| `position start moves` used for the empty case | `the_initial_position_is_asked_without_a_moves_keyword` |
| the two-engine refusal removed | `a_report_whose_engines_differ_is_refused_by_name` |
| an `error` answer skipped instead of refusing | `an_error_answer_refuses_the_run_and_names_the_game_and_turn` |
| `source_sha256` used as the identity | `two_reports_of_one_experiment_share_a_capture_identity` |
| forfeited or book positions skipped | their two tests |

## 11. WHAT THIS PACKAGE DOES NOT DECIDE

The label budget's VALUE and the pilot's `book_v2` range (both numbers, both the
pilot's pre-registration). And **every question of MEANING** — what the score,
the node counts and the provenance mean, which positions a trainer should use,
transposition dedup, and the census-minimum rule. Those are WP-2.0-S's.

**Requirement 5's corpus manifest is THIS package's** and is delivered by §4's
header plus the `body_sha256` the fixture type appends; the `book_v2` range
ledger belongs to pass 1's config, which is the arena's existing business.
Revision 1 left requirement 5 in neither package.
