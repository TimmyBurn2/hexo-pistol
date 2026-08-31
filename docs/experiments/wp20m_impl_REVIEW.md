# REVIEW-impl — WP-2.0-M, the labelling pass

## Header

- **Named revision adjudicated:** `40818fd8404f09c58b982a0a1550a086e72b0ce7`
  (`feat(arena): the labelling pass — a capture mode that re-asks every recorded
  position cold and writes the engine's own bytes, less the two fields gate 9
  strips`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` → `40818fd8404f…`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty**. The mutation receipt
  `artifacts/wp20m_mutants_40818fd.txt` is untracked and gitignored
  (`.gitignore:19`), so it does not appear there; its sha256 is
  `acc0e9c5be5c70cecd0b73533870a04ddde734b6ad45afedda27edb392fe6534`, which is
  the digest the dispatch names.
- **I did not write this code.**
- **Read as binding, in full:** `docs/experiments/wp20m_design.md` (all 1238
  lines, §0 through §15); `CLAUDE.md`; `docs/decisions.md` D-539…D-550 and D-5,
  D-6, D-80, D-200, D-252, D-413; `docs/experiments/wp20m_design_REVIEW_rev5.md`
  and `..._rev7.md` headers and verdicts; `artifacts/wp20m_mutants_40818fd.txt`.
- **Code read in full:** `crates/pistol-arena/src/capture.rs`,
  `crates/pistol-arena/src/capture_file.rs`,
  `crates/pistol-arena/src/usage.rs`,
  `crates/pistol-arena/tests/capture_tests.rs`, the complete diff of
  `bin/arena.rs`, `bin/stub_engine.rs`, `exchange.rs`, `lib.rs`, `record.rs`,
  `transcript.rs` and `docs/rule9_justifications.md`. Read for context:
  `channel.rs`, `seats.rs`, `replay.rs::verify_engines`, `conclusion.rs`'s game
  writer, `crates/pistol-cli/src/report.rs`,
  `crates/pistol-cli/src/corpus/emit.rs`, `tests/common/mod.rs`,
  `tests/replay_refusal_tests.rs`, `tests/run_tests.rs`,
  `tools/file_justification_check.sh`.
- **What I ran:** `git` (`rev-parse`, `status`, `show`, `show --stat`, `grep`,
  `ls-files`, `check-ignore`), `/usr/bin/grep`, `sed`, `awk`, `wc`, `cat`, `ls`,
  `sha256sum`, `LC_ALL=C sort`. Every recorded grep is `/usr/bin/grep` or
  `git grep` (D-265).
- **What I refused to run, per the dispatch:** `cargo` in any form, `tools/ci.sh`
  and every gate script. Where a claim needs a run I name the run in the last
  section rather than assert the answer.

---

## VERDICT: **FAIL** — **2 BLOCKING, 6 MAJOR, 7 MINOR.**

**The mechanism is right and the tests are not.** I could not find a way to make
`capture.rs` or `capture_file.rs` produce a wrong capture: the asked set, the
bare `position start`, the seat check over `identities`, the `newgame` per ask,
the normalisation, the record grammar, the loader and the digest are each
implemented as §2 … §5 specify, and the three mutants the implementer ran died
where they were registered to die. **What failed is the registration.** Fifteen
of the design's thirty-eight registered tests do not exist, INVARIANT 9's six
tests are all of them absent, and three tests that DO exist cannot fail under the
mutants they are registered against — one of them
(`a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote`) in
exactly the vacuous form design revisions 6 and 7 spent a round removing (F-3).
This is D-544's and D-548's named class — *a test whose name claims a pin its
driver cannot make* — arriving one stage later than the arc had been fighting it.

---

## 1. THE TWELVE INVARIANTS

| # | invariant | site | holds? |
|---|---|---|---|
| 1 | the asked set is every legal turn boundary, `k ∈ [0,len]` less `k=len` on a winning last turn; book and forfeited included | `capture.rs:27-43` (the filter at `:42`), driven at `capture.rs:229-231` | **YES in code.** Book turns are never distinguished anywhere, and `forfeit` is never read on the capture path, so both are included by construction. Its test is self-referential (m3) and both its book/forfeit tests are absent (B1) |
| 2 | no asked position is decided; the initial position is bare `position start` | decidedness `capture.rs:39,42`; bare line `capture.rs:49-55`, `:51` builds `POSITION + " start"` and does **not** call `exchange::position_line` for the empty slice | **YES in code.** The bare-`start` limb is tested (`capture_tests.rs:188-202`); the decided limb has no test (B1) |
| 3 | refuse a report whose two seats do not attest one engine; spawn slot zero; verify against the recorded identities | `capture.rs:128-146` compares `transcript.identities[0]`/`[1]` — **not** `engines` — and walks `binary_sha256`, `config_sha256`, `weights_sha256`, `id_lines` naming the one that differed; `replay::verify_engines` is called at `capture.rs:221`; slot zero at `capture.rs:223-226` | **YES in code**, all three limbs. Two of its five registered tests are absent, so the `id_lines` limb and the `verify_engines` call are unguarded (B1) |
| 4 | a `newgame` before every label `go`; the `go` is `BudgetSection::go_line`'s | `capture.rs:164` sends `[NEW_GAME, position, go]` per ask; `capture.rs:112-116` builds the `go` through `BudgetSection::Nodes{..}.go_line()` and never by hand | **YES**, and it is the best-pinned invariant in the package: mutant 1 died at its registered test alone |
| 5 | pass 2 never plays a move; every position is a prefix of the recorded list | `capture.rs:231` `&game.moves[..k]` | **YES in code — and NO TEST AT ALL.** `every_captured_position_is_a_prefix_of_the_reports_own_move_list` is absent (B1) |
| 6 | the engine's own bytes less ` nps <n> time <n>`; `bestmove` untouched; a TAB refuses before the write | `capture.rs:65-95` (normalise), applied at `:244`; `bestmove` carried through untouched from `capture.rs:190`; `no_tab` at `:247`,`:257-272` | **YES in code.** The normalisation limb is well pinned (mutant 2 died). The `bestmove` limb's test cannot fail (**B2**). The TAB limb has **no test** (B1) |
| 7 | no behaviour rides along with `totals_of`'s visibility change | `exchange.rs:163-207` | **Behaviour: YES** — `fields_of`+`value_of` reproduce the original `strip_prefix`/`split_whitespace`/whole-word-position lookup exactly and all three lookups keep their `?`. **But the change is far more than the visibility keyword the design authorised** (M4), and its registered test is absent; the mutant dies at `tests/run_tests.rs:97-105` instead |
| 8 | a re-run over one report at one budget is byte-identical | ordering `capture.rs:229-230`, rendering `capture_file.rs:73-78`; no map iteration, no clock on any choice path | **YES**, tested as the shape test §14.3 declares |
| 9 | any failure refuses the whole run; no position silently skipped | `capture.rs:158-163` (unsolicited), `:165-167` (closed input), `:172-174` (`Closed`), `:175-181` (`Overlong`), `:184-189` (no totals line), `:192-197` (`error`), `:205-208` (unknown line), `:244`, `:247`; the watchdog is `channel.rs:155-171`'s `ArenaError::Hung` via the `?` at `capture.rs:171` | **YES in code — there is no skip path anywhere.** But **none of its six registered tests exists** (B1), and two silent fallbacks survive elsewhere in the package (M2, m4) |
| 10 | pass 1 unmodified | the diff | **YES in substance.** `bin/arena.rs`'s two existing arms and `outpath::claim` (`arena.rs:64-69`) are untouched; the new arm is beside them. `exchange.rs`'s split is output-neutral by reading. **But the invariant's own registered evidence — *"only its one visibility keyword"* — is now false as written** (M4) |
| 11 | round-trips field by field; a wrong body digest, wrong arity or empty field is refused by name | `capture_file.rs:137-206`; digest `:148-157`, arity `:165-171`, empty `:172-178` | **YES**, and all four refusals are tested — except that the loader silently skips `#` body lines (m6) and the round-trip test is weaker than its name (m2) |
| 12 | the identity is a function of exactly three inputs | `capture.rs:103-109` — capture format version, `experiment_sha256`, label `go` line, one per line, nothing else; `arena_version` appears nowhere in the package | **YES at the function**, and each of the three has its own test. **The CALL is unpinned**: `capture_file.rs:52-56` and `:92-96` are what feed it, and no test binds them (M3) |

---

## 2. THE REGISTERED TESTS

`crates/pistol-arena/tests/capture_tests.rs` holds **24** `#[test]` functions.
Twenty-three of them are registered names; one
(`a_capture_whose_format_version_is_unknown_is_refused_by_name`,
`capture_tests.rs:332`) is an unregistered extra and is welcome. The design
registers **38**. I checked each registered name with
`git grep "fn <name>" -- crates/`.

### 2.1 The fifteen that do not exist

| absent test | pins | what is therefore unguarded |
|---|---|---|
| `a_book_turns_position_is_captured_like_any_other` | 1 | — (a mutation skipping book prefixes would still shrink the row count and die at `the_asked_set_…`) |
| `a_forfeited_games_positions_are_captured_like_any_other` | 1 | the registered mutant *"forfeited or book positions skipped"* on its forfeit limb: no fixture in the file contains a forfeited game under `honest` |
| `a_decided_terminal_position_is_never_asked` | 2 | the *"decided-position guard removed"* mutant — see §3 |
| `two_identities_differing_only_in_an_id_line_are_refused_naming_that_line` | 3 | the *"`id_lines` dropped from the refusal message's field walk"* mutant. Dropping that branch turns a refusal into an **acceptance** of a two-teacher report |
| `a_respawned_engine_that_does_not_match_the_report_is_refused` | 3 | the *"`replay::verify_engines` not called"* mutant |
| `every_captured_position_is_a_prefix_of_the_reports_own_move_list` | 5 | INVARIANT 5 entirely |
| `a_captured_record_carries_the_normalised_totals_line` | 6 | nothing: its coverage moved into `a_captured_totals_line_keeps_every_field_but_nps_and_time`, and mutant 2's receipt confirms the call dies there |
| `a_captured_field_containing_a_tab_refuses_the_run_by_name` | 6 | the *"write-side TAB check removed"* mutant, and `no_tab` in its entirety |
| `the_sprt_reports_per_game_node_counts_are_billed_from_the_totals_line` | 7 | nothing: `tests/run_tests.rs:97-105` (`report_contains_per_side_compute_fields`) already asserts `nodes_a > 0` per game and kills the fourth-lookup mutant |
| `an_error_answer_refuses_the_run_and_names_the_game_and_turn` | 9 | the *"an `error` answer skipped instead of refusing"* mutant |
| `a_report_pass_two_cannot_read_is_refused_by_name` | 9 | — (partly covered by `a_capture_over_a_report_whose_budget_is_not_nodes_…`) |
| `an_engine_that_stops_answering_refuses_the_run_at_the_watchdog` | 9 | the watchdog mutant |
| `an_engine_that_closes_its_pipe_refuses_the_run_by_name` | 9 | the `Closed` arm |
| `an_overlong_non_line_refuses_the_run_by_name` | 9 | the *"overlong mapped to `Closed`"* mutant — the finding M-7 of design revision 6 exists to cover, now unpinned again |
| `an_unrecognised_totals_line_refuses_the_run_and_names_the_game_and_turn` | 9 | the *"unrecognised totals line treated as ordinary `info`"* mutant |

**INVARIANT 9 has six registered tests and zero of them.** INVARIANT 5 has one
and zero. That is the gap.

### 2.2 The twenty-three that exist — do they pin what they say?

**Pin what they say (17):** `the_initial_position_is_asked_without_a_moves_keyword`,
`a_report_whose_seats_attest_different_engines_is_refused_by_name` (it asserts the
field name `config_sha256`, `capture_tests.rs:110`),
`a_self_play_report_whose_seats_carry_distinct_labels_is_accepted`,
`every_label_go_is_preceded_by_a_newgame` (driven by the new
`demands_newgame_per_ask`, and mutant 1 confirms it bites),
`the_label_go_line_is_the_one_budget_section_spells`,
`a_captured_totals_line_keeps_every_field_but_nps_and_time` (see the note below),
`the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line`,
`two_totals_lines_differing_only_in_nps_and_time_normalise_equal`,
`a_rerun_over_one_report_is_byte_identical`,
`a_capture_whose_body_digest_is_wrong_is_refused_by_name`,
`a_capture_record_with_the_wrong_field_count_is_refused_by_name`,
`a_capture_record_with_an_empty_field_is_refused_by_name`,
`two_captures_of_different_experiments_do_not_share_an_identity`,
`a_capture_identity_moves_when_the_format_version_moves`,
`a_capture_identity_moves_when_the_label_budget_moves`,
`a_capture_over_a_report_whose_budget_is_not_nodes_is_refused_by_name`,
`a_label_node_count_spelled_a_way_this_program_will_not_echo_back_is_refused`,
`a_capture_prints_a_manifest_row_naming_its_digests`.

**A driver change that is BETTER than registered (1).**
`a_captured_totals_line_keeps_every_field_but_nps_and_time`
(`capture_tests.rs:205-231`) is registered as *"unit, synthetic totals line"* and
is implemented stub-driven end to end over a written capture. It therefore does
the work of the absent `a_captured_record_carries_the_normalised_totals_line` as
well — which mutant 2's receipt confirms — while still catching a widening to a
non-solver field (`hashfull`, `score`, `pv` are all asserted present). **This is
the one place the implementation improved on the registration**, and the two
names should be reconciled in §10 rather than the test changed.

**Do NOT pin what they say (5):** see B2, M3, M6, m2 and m3 below.

---

## 3. THE MUTANTS

**The three that were run** (`artifacts/wp20m_mutants_40818fd.txt`) each died
where registered, and mutant 3's blast radius (11 failures) is the measured form
of revision 2's *"it would have refused one hundred per cent of its inputs"*. I
take those three as settled.

**Five I reason about that were not run.** I can settle four by reading.

1. **"the `bestmove` field written from the parsed turn rather than from the
   engine's own line" → SURVIVES.** The stub's answer is
   `pistol_cli::report::bestmove_line(best)` = `"bestmove " + turn`, and a turn
   token (`crates/pistol-cli/src/report.rs:106-108`, D-5/D-6) carries no space.
   The test asserts only `best.starts_with("bestmove ")` and
   `best.matches(' ').count() == 1` (`capture_tests.rs:242-245`). A field
   re-rendered from the parsed `Turn` satisfies both, byte for byte. **This is
   B2**, and the design says so in the driver column it registered against
   exactly this mutation.
2. **"the write-side TAB check removed" → SURVIVES.** `no_tab`
   (`capture.rs:257-272`) has no caller in any test and no engine in this tree
   can emit a TAB, which is the design's own reason for registering the test as a
   synthetic unit. Deleting the whole function is invisible.
3. **"`source_sha256` used as the identity" → SURVIVES.** The mutation lives at
   the call — `capture_file.rs:52-56` and `:92-96` — and all four identity tests
   call `capture::capture_sha256` directly with a literal
   (`capture_tests.rs:360-385`). `a_rerun_over_one_report_is_byte_identical`
   captures the same file twice, so its `source_sha256` is constant;
   `a_capture_prints_a_manifest_row_naming_its_digests` never checks the value.
   **This is M3.**
4. **"`id_lines` dropped from the refusal message's field walk" → SURVIVES**, and
   its consequence is worse than a bad message: removing the `else if` at
   `capture.rs:136` makes `one_engine` return `Ok` for two seats that differ only
   there, which is the precise case §3 says the refusal exists to foreclose (two
   seats naming one config file by two different path strings).
5. **"the decided-position guard removed" → I CANNOT SETTLE BY READING**, and it
   is the one place the answer turns on data rather than text. See the last
   section.

Two more worth recording because they are the reason B1 matters: **"an `error`
answer skipped instead of refusing"** and **"a watchdog timeout skipped instead
of refusing"** both survive, because no test in the file drives an engine that
errors or hangs during capture; and **"an overlong non-line mapped to the
`Closed` refusal"** survives for the same reason.

---

## 4. DEVIATIONS FROM THE DESIGN

**The loader's name — RESOLVED, not a finding.** The design names it
`capture::read`; the code defines it in `capture_file.rs:137` and re-exports it
at `capture.rs:11` (`pub use crate::capture_file::{CAPTURE_FORMAT_VERSION,
CaptureRecord, read, render}`). `capture::read` resolves and
`capture_tests.rs:284` uses that path. (One wart: `Capture`, its return type, is
not re-exported — m7.)

**§4.3's header enumeration — MATCHES.** `capture_file.rs:66-72` writes exactly
`param capture_format_version`, `param experiment_sha256`, `param
source_sha256`, `param label_go`, `derived capture_sha256`, `derived games`,
`derived records`. `arena_version` appears nowhere in the package, which is §5's
requirement and N2's remedy.

**`count_of` and `--workers` — one dead branch and one changed message (m1).**
`workers_of` (`arena.rs:91-98`) now delegates to `count_of`
(`arena.rs:106-120`). The spelling refusal is byte-identical in shape. But
`count_of` refuses zero at `:116-118` **before** `workers_of`'s own zero check at
`:94-96`, so that branch is unreachable and `--workers 0` now says *"a worker
count of zero asks for nothing at all"* instead of *"--workers 0 would replay
nothing at all"*. The existing test
(`tests/replay_refusal_tests.rs:227-241`) asserts only exit 2 and no document for
the spellings `["04","+4","0","four","4.0"]`, so it still passes — the attack did
not land as a break, only as dead code.

**`demands_newgame_per_ask` — does not disturb `demands_newgame`.**
`stub_engine.rs:284-293`: the clearing branch is guarded by
`behave == Behave::DemandsNewGamePerAsk`, so `DemandsNewGame`'s latch still
never clears and its two existing users
(`tests/replay_tests.rs`, `tests/replay_refusal_tests.rs`) see the same engine
they saw. `Behave::ALL` gained the spelling
(`stub_engine.rs:88-89`), so the refusal that has to list them still does.

**Three deviations that are findings: M1 (`usage.rs`), M4 (`exchange.rs`), M5
(`transcript.rs`/`record.rs`).** All three are WP-2.0-S content landing inside
WP-2.0-M's commit — `exchange.rs:173` and `transcript.rs:27` say so themselves,
citing `docs/experiments/wp20s_design.md` §3 and §4. That design exists but is a
different package with its own review arc (D-550), and `wp20m_design.md` §0.3
closes with *"No engine diff; only `pistol-arena` changes"* and §8 with
*"gains nothing in this package"*.

---

## 5. HARD RULES AND STYLE

- **Rule 1 (no code-side default for a tunable).** Clean. `--label-nodes` is
  required by the match arm (`arena.rs:51-54`); there is no default node count,
  and no movetime spelling exists to refuse, which is §7's stronger form.
  `CAPTURE_FORMAT_VERSION` (`capture_file.rs:10`) is a format version, not a
  tunable.
- **Rule 3 (fail loud).** **One survivor: `capture_file.rs:97`** — see M2. The
  `unwrap_or_default` the dispatch mentions is gone; `header()`
  (`capture_file.rs:112-126`) refuses by name on both absence and duplication,
  which is stricter than the design asked for. Two lesser swallows: m4 and m5.
- **Rule 4 (determinism).** Clean. `capture::run` walks `transcript.games` (a
  `Vec`) and `asked_prefixes`' `Vec<usize>` in order; there is no map iteration,
  no clock and no thread on any choice path. The watchdog is a refusal, never a
  choice.
- **Rule 8 (nothing written inside the repository).** Clean. `capture_pass`
  (`arena.rs:150-171`) writes only the claimed out path and prints the manifest
  row to stdout, which is §13(a)'s decision. The mutation receipt is under
  gitignored `artifacts/`.
- **Rule 9.** `bin/arena.rs` is 294 lines after the `USAGE` extraction — under
  the cap, which is what §1 predicted. `capture.rs` 272 and `capture_file.rs` 206
  need no entry. `tests/capture_tests.rs` is 479 and gained an entry
  (`docs/rule9_justifications.md:25`); **it states no count** and gives a real
  why (one two-stage fixture, one thing varied per case). Compliant.
- **Rule 10.** §15's owed ADR act — `label_sha256` → `capture_sha256` — is not in
  this commit. `/usr/bin/grep -c "capture_sha256" docs/decisions.md` still
  returns zero. The design says it *"lands with the code"*; the code has landed.
  Recorded here rather than graded, since the operator owns the ADR act.
- **Style.** Clean throughout the new files: no file-top narrative headers, no
  `//!` outside the crate root, `///` on every public item, `# Errors` on
  `asked_prefixes`, `normalise`, `one_engine`, `run` and `capture_file::read`,
  and the comments say why (`capture.rs:255-256`, `capture_file.rs:110-111`,
  `stub_engine.rs:289-290`). Tests are named for behaviour. One observation, not
  a finding: `capture::label_go_line` can `unreachable!` and carries no
  `# Panics`, matching the precedent at `arena.rs:240`.

---

## 6. FINDINGS

### BLOCKING 1 — fifteen of thirty-eight registered tests do not exist, and INVARIANT 9 has none of its six

`crates/pistol-arena/tests/capture_tests.rs` (whole file) against
`docs/experiments/wp20m_design.md` §10.

**Why it is wrong.** §10 is the design's registration, and CLAUDE.md's process
makes a pre-registered test set the thing the implementation is checked against.
Six registered mutants are left with no killer at all: the `error` skip, the
watchdog skip, the `Overlong`-as-`Closed` mapping, the unrecognised totals line,
the write-side TAB check, and `verify_engines` not being called. INVARIANT 5 has
no test of any kind, so a pass 2 that sent the wrong game's prefix would be
caught by nothing. None of this is a defect in the shipped code — I could not
find one — but the package's own claim is that these refusals are pinned, and
they are not.

**FIX.** Write the fifteen, or amend §10 by ADR to withdraw the ones judged
unwritable and say why. Five are cheap: `an_error_answer_…`,
`an_engine_that_closes_its_pipe_…` and `an_engine_that_stops_answering_…` are
stub behaviours that already exist (`garbage`, `exit`, `hang`);
`an_overlong_non_line_…` and `an_unrecognised_totals_line_…` are registered as
units over the failure mapping and the recogniser, needing no engine; and
`a_captured_field_containing_a_tab_refuses_the_run_by_name` needs `no_tab` (or a
record-writing entry point) made reachable from a test.

### BLOCKING 2 — `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote` cannot fail under its own mutant

`crates/pistol-arena/tests/capture_tests.rs:234-247`.

**Why it is wrong.** The design registers this test as **"unit, over the record
writer, synthetic"** and states the reason on its face: *"the stub's `bestmove`
is always one canonical turn after one space, so a field re-rendered from the
parsed `Turn` would write identical bytes and the mutation would be invisible"*
(§10). The implementation drove it with the stub anyway and asserted exactly the
two properties a re-rendered field also has. **The registered mutant survives.**
This is design finding F-3 — raised against revision 6, fixed in revision 7,
verified by the rev-7 reviewer as *"the round's one substantive correction"* —
re-introduced in the implementation. A test named for byte-identity that cannot
see a byte difference is worse than no test, which is the design's own phrasing.

**FIX.** Make it a unit over the record writer with a synthetic engine line the
parsed-`Turn` path cannot reproduce — e.g. `"bestmove  0,0"` (two spaces) or
`"bestmove 0,0 "` — and assert the written field equals that string byte for
byte. That needs the record-writing step reachable from a test, which
`a_captured_field_containing_a_tab_refuses_the_run_by_name` needs too.

### MAJOR 1 — `usage.rs` documents a mode the binary does not have

`crates/pistol-arena/src/usage.rs:15` and `:57-63`.

**Why it is wrong.** `USAGE` advertises
`arena --labels <capture path> --report <report path> --out <path>` and gives
both flags their own paragraphs. `git grep '"--labels"' crates/` returns nothing:
`bin/arena.rs:36-59` has three arms and no fourth. A user who copies that line
gets the fallback refusal at `arena.rs:55-60`, which names three modes and then
prints the usage block showing the fourth. The design's §1 authorised exactly one
addition — *"the `USAGE` constant gains a `--capture` paragraph in the shape of
the two beside it"* — and stated the rule this inverts: *"A binary whose help
text does not mention a mode it has is a binary whose help text is wrong."* A
binary whose help text mentions a mode it does not have is wrong in the direction
that costs a user a run. No test covers `--help` or `USAGE`
(`git grep -i "usage\|--help" -- crates/pistol-arena/tests/` is empty), so
nothing caught it.

**FIX.** Delete `usage.rs:15` and `:57-63`; they belong in WP-2.0-S's commit.
Also drop *"and fourth"* from the doc comment at `usage.rs:3`, which describes a
mode arm this build does not carry.

### MAJOR 2 — a silent fallback in the committed manifest row

`crates/pistol-arena/src/capture_file.rs:97`:
`let body = emit::claimed_body_digest(rendered).unwrap_or("<none>");`

**Why it is wrong.** Hard rule 3: *"No silent fallback, swallowed error, or
skip-with-default."* The manifest row is the committed index of an uncommitted
artifact (hard rule 8, §13(a)), and a row printed with `<none>` where its body
digest belongs is a corpus index that indexes nothing — printed on stdout for a
human to paste into `docs/label_corpus_manifest.md`, where it would sit as a
committed claim. It is currently unreachable (`Fixture::render`,
`crates/pistol-cli/src/corpus/emit.rs:93-99`, always writes the digest line), but
unreachability is the argument for `unreachable!` or `?`, not for a placeholder.
Note that the loader's own use of the same function is correct
(`capture_file.rs:148-151` uses `ok_or_else` with a named refusal), which makes
this the odd one out.

**FIX.** Return `Result<String, ArenaError>` and refuse by name, or take the
digest as a parameter from `render`'s own computation so the `Option` never
arises.

### MAJOR 3 — the capture identity is pinned at the function and not at the call

`crates/pistol-arena/tests/capture_tests.rs:360-385` against
`crates/pistol-arena/src/capture_file.rs:52-56` and `:92-96`.

**Why it is wrong.** All four identity tests call
`capture::capture_sha256(literal, literal, literal)`. Nothing binds the two call
sites to `transcript.experiment_sha256` rather than `transcript.source_sha256`,
so the registered mutant *"`source_sha256` used as the identity"* survives every
test in the file. `two_reports_of_one_experiment_share_a_capture_identity` is
named for the very experiment that would catch it — **two reports**, of one
experiment, sharing an identity — and instead calls one pure function twice with
the same arguments, which asserts only that sha256 is a function. This is the
class D-548 names as the one nothing guards (*"a remedy that pins the FUNCTION
and not the CALL"*), reproduced at the identity after the arc closed it at the
normalisation.

**FIX.** Drive two arena runs of one experiment (same engine sections, same
openings, same budget) so the two reports differ in `source_sha256` and agree in
`experiment_sha256`, capture each, and assert the two files' `derived
capture_sha256` header values are equal. That is the assertion the name promises
and it kills the mutant.

### MAJOR 4 — `exchange.rs` is widened far beyond the visibility change the design authorised

`crates/pistol-arena/src/exchange.rs:163-207`.

**Why it is wrong.** §8 says *"`exchange::totals_of` rises to `pub(crate)` and
**gains nothing in this package**"*, and INVARIANT 10's evidence paragraph in §9
says *"the only file whose SPRT-path BEHAVIOUR this package changes is
`crates/pistol-arena/src/exchange.rs`, and only its one visibility keyword."*
The commit instead splits it into `pub(crate) fields_of` and `pub(crate)
value_of` with a doc comment citing `docs/experiments/wp20s_design.md` §3 — a
different package's design, on its own review arc. **The behaviour is
unchanged**: `fields_of` reproduces `strip_prefix(&prefix)?` +
`split_whitespace().collect()`, `value_of` reproduces the whole-word `position`
lookup, and `totals_of` keeps all three `?` lookups load-bearing in the same
order with the same keys. So INVARIANT 7 and INVARIANT 10 hold in substance. What
does not hold is the invariant's stated evidence, which was its only evidence —
§9 is explicit that no test can pin INVARIANT 10 and that the diff IS the
argument. A reviewer of this commit reading §9 and then the diff finds them
disagreeing.

**FIX.** Either revert `exchange.rs` to the visibility keyword alone and let
WP-2.0-S land the split, or amend §8/§9 by ADR to record that the split landed
here and why it is output-neutral. Prefer the first: WP-2.0-S's design has its
own review that this commit's reviewers have not read.

### MAJOR 5 — `RecordedGame::result` and `GameResult::from_token` are undesigned, unused, and add a refusal to an existing mode's reader

`crates/pistol-arena/src/transcript.rs:22-28` and `:316-324`;
`crates/pistol-arena/src/record.rs:24-34`.

**Why it is wrong.** Neither appears anywhere in `wp20m_design.md`; the field's
own doc cites `docs/experiments/wp20s_design.md` §4. `git grep` shows the new
field is **read by nothing** in this commit — every `record.result` hit is the
unrelated `GameRecord`. Meanwhile `read_games` now requires a `result` word and
refuses a report whose spelling it does not know, which is a **new refusal on
`transcript::read`**, and `transcript::read` is the reader the shipped
`--replay` mode uses (`arena.rs:146`, `arena.rs:173`). No report this build
writes can trip it — `conclusion.rs:37-44` always writes
`record.result.token()` and `from_token` is its exact inverse — so nothing
breaks. But CLAUDE.md's rule 10 calls unrecorded architecture drift a breach, and
a dead field plus a new refusal on an existing mode is exactly that.

**FIX.** Move both to WP-2.0-S's commit, where the field has a reader.

### MAJOR 6 — `a_totals_line_with_no_score_at_all_is_captured_as_written` is aimed at the wrong site

`crates/pistol-arena/tests/capture_tests.rs:455-462`.

**Why it is wrong.** The design registers this test as *"unit, over the totals-line
recogniser, synthetic"* and adds the reason: *"Not over the normalisation, which
is a `fn(&str) -> String` and cannot refuse anything"*. The implementation made
`normalise` fallible (`capture.rs:65`) and aimed the test at it. So the test can
bite — but not on the mutation registered against it. *"A score-less totals line
refused instead of captured"* has its natural site at the recogniser
(`capture.rs:198`, `exchange::totals_of`): add a `value_of(&words,"score")?`
lookup and a score-less line stops being recognised, the search closes with no
totals line, and `capture.rs:184-189` refuses the run. **The implemented test
does not see that at all**, and §8's whole argument — and D-542's *"`score` and
`pv` as non-fatal `Option`s"* — is about that site.

**FIX.** Add the registered unit over `exchange::totals_of` (or over a
recogniser wrapper) asserting a score-less totals line is still recognised, and
keep the existing `normalise` test as the extra it is.

### MINOR 1 — `workers_of`'s zero branch is dead and `--workers 0`'s refusal changed

`crates/pistol-arena/src/bin/arena.rs:94-96`, unreachable because
`count_of` refuses zero at `:116-118`. **FIX.** Delete `:94-96`, or move the
zero check out of `count_of` and give each caller its own message.

### MINOR 2 — the round-trip test compares the file with itself

`crates/pistol-arena/tests/capture_tests.rs:278-294`. `records(&text)` and
`capture::read(&text)` both read the SAME rendered file, so
`record.game.to_string() == row[0]` is true by construction whatever the writer
put in field 1. It pins the loader's field order (real, and it can fail) but not
the writer's, so the registered mutant *"a capture record's first two fields
swapped on write"* is invisible to it — it dies at
`the_asked_set_is_every_legal_turn_boundary` instead, provided some row has
`game != turns_played`. **FIX.** Assert against the `Vec<CaptureRecord>` the run
produced, or against independently known values, not against a re-split of the
same bytes.

### MINOR 3 — `the_asked_set_is_every_legal_turn_boundary` computes its expectation from the function under test

`crates/pistol-arena/tests/capture_tests.rs:158-184` builds `expected` by calling
`pistol_arena::capture::asked_prefixes`. It pins that `run` asks what
`asked_prefixes` returns — real, and it is what kills the swap mutant — but it
cannot falsify `asked_prefixes` itself, which is INVARIANT 1's actual content.
**FIX.** Add one case with hand-stated prefixes for a game of known length,
including a won game (which is also the absent
`a_decided_terminal_position_is_never_asked`).

### MINOR 4 — a silent clamp on a refusal's turn number

`crates/pistol-arena/src/capture.rs:171`:
`u32::try_from(k).unwrap_or(u32::MAX)`. Unreachable, but it makes a watchdog
refusal name the wrong turn rather than fail. **FIX.** `unreachable!` with the
reason, matching `capture.rs:115`'s own idiom.

### MINOR 5 — swallowed writes inside the identity's canonical form

`crates/pistol-arena/src/capture.rs:105-107`: three `let _ = writeln!(canonical, …)`.
A failure would digest a truncated canonical form under a name that says it is
the identity. Infallible for `String`, but hard rule 3 asks for the loud
spelling. **FIX.** `push_str(&format!(…))`, which cannot fail and needs no
discard.

### MINOR 6 — the loader silently skips `#` lines in the body

`crates/pistol-arena/src/capture_file.rs:161`. INVARIANT 11 and the design's
loader paragraph say the whole file is refused rather than partially read;
`render` never writes a body comment, so a `#` line in a body is a file this
build did not write and should be refused, not skipped. It is covered by the body
digest against tampering, so this is a shape point, not a hole. **FIX.** Refuse a
non-empty body line that is not a record.

### MINOR 7 — `capture::read`'s return type is not re-exported beside it

`crates/pistol-arena/src/capture.rs:11` re-exports `read` but not `Capture`
(`capture_file.rs:33`), so a caller reached through `capture::read` must name
`capture_file::Capture`. **FIX.** Add `Capture` to the `pub use`.

---

## 7. THE STRONGEST ATTACK THAT DID NOT LAND

**I tried to break `normalise` on the substring route, twice, and it holds.**
The function finds the FIRST ` nps ` (`capture.rs:68`) and then requires ` time `
to follow the digits immediately (`:80`). Two ways that could be wrong: (a) an
earlier ` nps ` in the line, and (b) `nps` and `time` not adjacent when the
solver block is present. Both are closed at
`crates/pistol-cli/src/report.rs:60-90`: the solver block is interpolated
**strictly between `nodes` and `nps`**, so `nps <n> time <n>` is adjacent in both
spellings — which is §4.1's argument, and the solver-bearing test at
`capture_tests.rs:418-440` executes it — and every field value before `nps` is a
decimal, so the bare word `nps` cannot appear. Route (c), a `pv` turn token
spelling `nps`, dies on D-5's token grammar (`q,r` / `q,r/q,r`). And the failure
mode if I were right is a **refusal**, not a corrupted record, because `:80-84`
and `:89-93` both return `Err` rather than falling through — which is the
direction hard rule 3 wants.

**Second attack, also failed: does the `totals_of` split change the SPRT
path?** I compared the old closure and the new `value_of` character by character
against the diff. `fields_of` = `strip_prefix(&prefix)?` +
`split_whitespace().collect()`; `value_of` = `position(|word| *word == key)` +
`get(at+1)` + `copied()`; `totals_of` calls them with `"nodes"`,
`TIME_FIELD`, `"depth_turns"` in that order, each `?`-chained, each `.parse().ok()?`.
Identical semantics on every input, including a totals line missing one field.
**The three lookups remain load-bearing.** The finding at M4 is about scope and
about INVARIANT 10's stated evidence, not about behaviour.

**Third, failed: does `count_of` break the existing `--workers` spellings test?**
`tests/replay_refusal_tests.rs:227-241` asserts only exit 2 and that no document
was written, for all five spellings including `"0"`. Both still hold. The
consequence is dead code (m1), not a break.

---

## 8. WHAT I COULD NOT SETTLE BY READING, AND THE RUN THAT WOULD

1. **Does the honest stub self-play fixture contain a game whose last recorded
   turn WINS?** This decides whether the registered mutant *"the decided-position
   guard removed"* dies at
   `a_self_play_report_whose_seats_carry_distinct_labels_is_accepted` (the engine
   would answer `error` on the terminal prefix and `capture.rs:192-197` would
   refuse, so the capture file would not exist) or survives unnoticed. The stub's
   `greedy` grows the mover's own cluster precisely so *"the win path"* is
   reachable (`crates/pistol-arena/src/bin/stub_engine.rs:160-166`), but whether
   six in a row is reached inside `TURN_CAP = 8` over `openings_prefix(2)` is a
   fact about the run. **The run:** build the same fixture
   (`capture_tests.rs:15-34`) and
   `/usr/bin/grep -c "result p1_win\|result p2_win" <report>`; a non-zero count
   settles it. Equivalently, `cargo test -p pistol-arena --test capture_tests`
   with the decided-guard mutation applied in a detached worktree.
2. **Is the suite green at this revision?** The receipt implies a 24-test
   baseline, but I did not run it. **The run:** `cargo test -p pistol-arena
   --test capture_tests` and `cargo test --workspace --locked`.
3. **Does gate 17 pass with `tests/capture_tests.rs` at 479 lines?** The entry at
   `docs/rule9_justifications.md:25` is well formed and states no count, and
   `bin/arena.rs` is 294 against `SOFT_CAP=300`. **The run:**
   `tools/file_justification_check.sh`.
4. **Does clippy accept `pub(crate) fields_of` / `value_of`?** Both are used by
   `totals_of`, so `dead_code` should not fire, and `redundant_pub_crate` is
   nursery (§8's own note). **The run:** `cargo clippy --workspace --all-targets
   -- -D clippy::all`, which is gate 4.
5. **Whether any gate reads `USAGE`.** SETTLED, and the answer is no: no arena
   test mentions `usage` or `--help`, and
   `/usr/bin/grep -rn -- "--labels" tools/` returns nothing. So M1's wrong help
   text is caught by no gate and no test, which is why it reached HEAD. Recorded
   here rather than left open.
