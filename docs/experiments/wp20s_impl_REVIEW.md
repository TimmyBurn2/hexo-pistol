# REVIEW-impl — WP-2.0-S, the record schema

## Header

- **Named revision adjudicated:** `18c9198f400eb9deeee5ab750ebe7eac07251462`
  (`feat(arena): the record schema — a labels mode that turns a capture into the
  training corpus, with the two node writers replacing their sum and the units
  on the file's own face`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` → `18c9198f400e…`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty**.
- **I did not write this code.**
- **Read as binding, in full:** `docs/experiments/wp20s_design.md` (all 1406
  lines, §0 through §12.3); `CLAUDE.md`; `docs/decisions.md` D-537, D-539…D-550
  and D-6, D-49, D-56, D-80; `docs/experiments/wp20m_design.md` §4.2-§4.3 and §5;
  `docs/experiments/wp20m_impl_REVIEW.md` (all 578 lines, so no finding of its is
  re-raised as new).
- **Code read in full:** `crates/pistol-arena/src/labels.rs`,
  `crates/pistol-arena/src/labels_file.rs`,
  `crates/pistol-arena/src/passes.rs`,
  `crates/pistol-arena/tests/labels_tests.rs`, the complete diff of
  `bin/arena.rs`, `exchange.rs`, `lib.rs`, `record.rs`, `transcript.rs`,
  `usage.rs`, `tests/capture_tests.rs` and `docs/rule9_justifications.md`, plus
  `bin/arena.rs` whole at HEAD. Read for context:
  `crates/pistol-arena/src/capture.rs`, `capture_file.rs`, `transcript.rs`'s
  `read_games`, `tests/capture_tests.rs`, `tests/run_tests.rs`,
  `crates/pistol-cli/src/corpus/emit.rs`, `crates/pistol-core/src/symmetry.rs`,
  `board.rs`, `zobrist.rs`, `tools/file_justification_check.sh`.
- **What I ran:** `git` (`rev-parse`, `status`, `log`, `show`, `show --stat`,
  `grep`), `/usr/bin/grep`, `sed`, `cat`, `wc`, `python3`, `LC_ALL=C sort`. Every
  recorded grep is `/usr/bin/grep` or `git grep` (D-265).
- **What I refused to run, per the dispatch:** `cargo` in any form, `tools/ci.sh`
  and every gate script. Where a claim needs a run I name the run in the last
  section rather than assert the answer.

---

## VERDICT: **FAIL** — **2 BLOCKING, 4 MAJOR, 9 MINOR.**

**The transform is right and its registration is not, in the same shape and one
notch worse than the sibling's.** I attacked every column derivation and every
refusal in `labels.rs` and `labels_file.rs` and could not find a way to make the
transform emit a wrong label from a capture this build wrote: the three keys are
pistol-core's own functions with the colour carried, `to_move` is replayed, the
score is read as a `(tag, number)` pair, the node columns are the two independent
writers with the six-or-none refusal, the book boundary is `k <= opening_turns`,
and the outcome cross-check reads no seating. **What failed is the evidence.**
Thirty of the design's fifty-one registered tests do not exist; INVARIANT 4 has
none of its four; INVARIANT 8's only surviving test refuses on a different rule
than the one it names and passes with the cross-check deleted; and INVARIANT 9's
one test never opens a corpus. Two guards §5 fixes on the file's own face — the
write-side TAB refusal and four of the eight header params — are not written at
all, and one of them has a reachable input that makes the transform emit a corpus
its own loader rejects.

This is the class `docs/experiments/wp20m_impl_REVIEW.md` graded BLOCKING 1 and
BLOCKING 2 against the sibling, arriving at the schema half at 30/51 instead of
15/38. The mechanism is not the problem in either half.

---

## 1. THE SIXTEEN COLUMNS

Every site is in `crates/pistol-arena/src/labels.rs` unless named otherwise; the
record is rendered in §2's order at `crates/pistol-arena/src/labels_file.rs:66-91`.

| # | column | site | as designed? |
|---|---|---|---|
| 1 | `game` | `:198`, from `CaptureRecord::game` | **yes.** The report's game index. `transcript.rs:290-296` refuses a report whose `game` records are not a contiguous 0-based prefix, so `transcript.games.get(record.game)` at `:110` indexes the game whose `index` is `record.game`. |
| 2 | `turns_played` | `:199` | **yes.** Prefix length, not `GameState::turn()`; §2.10's rename honoured, and `:116-123` refuses a length the report cannot hold. |
| 3 | `moves` | `:200` → `render_turns` `:93-102` | **yes.** Turn tokens space-joined, `-` when empty (`EMPTY_FIELD` `:14`). |
| 4 | `key_seq` | `:201`, `pistol_core::canonical_sequence(prefix)` | **yes.** The design's named function (`symmetry.rs:219`), rendered as turn tokens, `-` when empty. |
| 5 | `key_pos` | `:202`, `state.key()` = `GameState::key` (`state.rs:134`) | **yes.** `Key128`'s `Display` is *"the 32 hex digits of the key, high half first"* (`zobrist.rs:70-76`), which is §5's spelling. |
| 6 | `key_full` | `:203`, `pistol_core::canonical_form(&stones)` (`symmetry.rs:165`) rendered at `:81-90` | **yes, and the colour is carried.** `format!("{at}:{}", player.name())` → `q,r:p1` / `q,r:p2`, space-joined in canonical order; `-` when empty. Revision 2's BLOCKING is fixed, and `Player::name` (`board.rs:31-36`) is the `p1`/`p2` spelling §5 fixes for `to_move` too, so one record carries one spelling of colour. |
| 7 | `to_move` | `:204`, `state.to_move().name()` | **yes, from pistol-core.** The state is the replay at `:128-137`; no parity arithmetic appears in the file. |
| 8 | `score_kind` | `:172` ← `score_columns` `:26-38` | **yes.** `cp`→`eval`, `mate`→`mate_in`, `-mate`→`mated_in`; the word `cp` reaches no column, and a fourth tag refuses by name at `:34-36`. |
| 9 | `score_value` | `:172`; negative-mate refusal `:173-178` | **yes.** Read as the PAIR: `value_of(words,"score")` is the tag (`:156`), `words.get(at + 2)` is the number (`:162-171`), and either absent refuses by name. The `i64` covers `Eval(i32)` and both `u16` turn counts. |
| 10 | `best` | `:184-194` | **yes in kind.** The turn token off the captured `bestmove` line, not re-derived. See MAJOR 2 for the one input that makes it empty. |
| 11 | `depth_turns` | `:180-183` | **yes**, and §2.5's two-meaning rule IS on the file's face: `labels_file.rs:122-125` writes `param depth_meaning a completed search depth, except where search_nodes is zero, where it is a proof depth`. |
| 12 | `search_nodes` | `node_columns` `:44-73`, arm `6 =>` at `:63` | **yes.** `value_of(words,"search_nodes")`, never the wire's `nodes`. |
| 13 | `solver_nodes` | same, `:63` | **yes.** The two independent counters. Gate-off fallback at `:67`: `nodes("nodes")?, 0`, with the `// Gate off:` comment citing the doc and the enforcing site — which is §2.4's own argument and not an assumption. The six-or-none refusal is `:62-72`, over the whole block and not the pair, naming the count it saw. |
| 14 | `book` | `:211`, `record.turns_played <= opening_turns` | **yes — the design's boundary, not the ask-boundary.** `opening_turns` comes from the report (`transcript.rs:48`, `:181`) and is carried into the header at `labels_file.rs:109`. |
| 15 | `result` | `:212`, `game.result.token()` | **yes, read and not derived.** `RecordedGame::result` (`transcript.rs:22-28`) is read at `transcript.rs:316-324` through `GameResult::from_token`, so a forfeited game's result survives. |
| 16 | `end` | `:213`, `game.forfeit` | **yes, separable.** `forfeit` is the report's own `end` field (`transcript.rs:316`), so capped and forfeited are two columns and cannot collapse. |

**The pistol-core cross-check reads NO seating.**
`git grep -n a_is_p1 -- crates/pistol-arena/src/labels.rs …/labels_file.rs …/passes.rs`
returns nothing. `agrees` (`:223-251`) skips forfeited games, replays the whole
list, and maps `Outcome::Win{winner}` → `p1_win`/`p2_win` and `Ongoing` →
`capped` with no reference to which engine holds seat one. §2.7 is implemented as
written, and its doc comment (`:217-222`) claims the check for what it is — the
arena's recorded verdict, not an external referent.

---

## 2. THE REFUSALS AND THE INVARIANTS

### §9's twelve read-side refusals

| # | implemented? | site |
|---|---|---|
| 1 | **yes** | `labels.rs:265-270` |
| 2 | **yes**, in the sibling's reader | `capture_file.rs:163-172` |
| 3 | **yes** | `labels.rs:271-281` (re-derived from the capture's own three inputs, not copied) |
| 4 | **yes**, in the sibling's reader | `capture_file.rs:190-203`; a field carrying a TAB becomes an extra field and hits the arity refusal |
| 5 | **yes, and stronger than registered** | `labels.rs:138-148` rebuilds the position line from the report's own prefix at that length and compares it to the captured `position` |
| 6 | **yes** | `labels.rs:110-115` |
| 7 | **yes** | `labels.rs:289-297` |
| 8 | **yes** | `labels.rs:156-171` (absent tag / absent number) and `:34-36` (tag outside the three) |
| 9 | **yes** | `labels.rs:173-178` |
| 10 | **yes** | `labels.rs:62-72` |
| 11 | **yes but unreachable** | `labels.rs:129-137`; see MINOR 8 |
| 12 | **yes** | `labels.rs:242-248` |

No silent fallback, swallowed error or unwrap-with-default reaches a decision:
`/usr/bin/grep -n "unwrap\|expect(\|let _ =\|unwrap_or"` over the three new source
files returns exactly one hit, `let _ = writeln!` at `labels_file.rs:69` (MINOR
3). The sibling's MAJOR 2 (`unwrap_or("<none>")` in a manifest row) does **not**
recur — `labels_file.rs:145-150` uses `ok_or_else` with a named refusal. Its
MINOR 6 (a loader skipping `#` lines in a body) does not recur either:
`labels_file.rs:199` filters only empty lines, so a `#` line in a corpus body
refuses on arity.

### §10's thirteen invariants

| # | held by the code? | pinned by a test? |
|---|---|---|
| 1 | yes — `labels.rs:284-288` is `map`/`collect` over the capture's order | yes, `every_capture_record_produces_one_corpus_record_in_order` |
| 2 | yes — the module reaches `pistol_core`, `crate::{capture, capture_file, error, exchange, labels_file, transcript}` and `pistol_cli::{protocol, report}`; no `Command`, no `Instant` | no test, as §10 says; the `use` list is the evidence and it holds |
| 3 | yes, all three limbs | `to_move` yes (renamed, see §3); `score_kind` partly; node columns partly |
| 4 | yes — `totals_of` keeps its three `?` lookups over the same words (`exchange.rs:171-180`) | **no.** See BLOCKING 1 |
| 5 | yes | yes, `a_rerun_over_one_capture_and_report_is_byte_identical` |
| 6 | **partly** — four of the eight header params are unchecked, and there is no write-side TAB or empty guard | partly |
| 7 | yes — every refusal is `return Err`, no skip | partly |
| 8 | yes | **no.** See BLOCKING 2 |
| 9 | yes — nothing folds on write | **no.** See MAJOR 4 |
| 10 | yes | yes, both boundary tests |
| 11 | yes — `labels_file.rs:139-159` names all six fields | weakly, two of six asserted |
| 12 | yes | **no** — `fields_of_gives_the_word_after_score_and_the_word_after_that` does not exist |
| 13 | yes | weakly, see MINOR 1 |

---

## 3. THE REGISTERED-TEST GAP

§11 registers **51** distinct test names. Twenty-one exist somewhere in the tree
(nineteen in `crates/pistol-arena/tests/labels_tests.rs`, two in
`crates/pistol-arena/tests/capture_tests.rs`). **Thirty do not exist under their
registered name anywhere**, by
`git grep -hn '^fn '` over the workspace matched against §11's table:

```
a_mate_score_becomes_mate_in
a_negative_mate_score_becomes_mated_in
a_score_tag_with_no_number_after_it_refuses_the_run_by_name
a_totals_line_with_solver_fields_yields_the_two_counters_it_carries
a_partial_solver_block_refuses_the_run_by_name
the_sprt_reports_per_game_node_counts_survive_the_totals_of_split
totals_of_still_refuses_a_line_missing_nodes_time_or_depth
fields_of_reads_a_captured_line_that_has_no_time_field
fields_of_gives_the_word_after_score_and_the_word_after_that
a_corpus_record_with_the_wrong_field_count_is_refused_by_name
a_corpus_record_with_an_empty_field_is_refused_by_name
a_corpus_whose_schema_version_is_unknown_is_refused_by_name
a_key_pos_that_is_not_thirty_two_hex_digits_is_refused_by_name
a_capture_whose_header_identity_is_not_its_own_inputs_is_refused_by_name
a_capture_record_naming_a_game_the_report_lacks_is_refused_by_name
a_report_game_with_no_capture_record_refuses_the_run_by_name
a_record_whose_turns_played_and_moves_disagree_is_refused_by_name
the_derived_outcome_agrees_with_the_reports_own_result_field
the_outcome_check_holds_when_engine_b_takes_seat_one
a_forfeited_games_result_is_the_reports_and_is_not_derived
two_positions_alike_up_to_a_symmetry_share_a_key_full_and_not_a_key_pos
to_move_is_the_side_pistol_core_puts_to_move_at_that_prefix
two_symmetric_prefixes_share_a_key_seq_and_two_transpositions_do_not
two_positions_differing_only_in_colour_do_not_share_a_key_full
a_capture_whose_schema_version_is_unknown_is_refused_by_name
a_capture_record_with_a_tab_in_a_field_is_refused_by_name
a_corpus_field_carrying_a_tab_refuses_the_run_by_name
a_corpus_missing_its_opening_turns_param_is_refused_by_name
a_moves_prefix_that_is_not_a_legal_game_is_refused_by_name
a_book_position_is_a_record_flagged_book
```

Three of those thirty are covered in substance under another name and are
**not** part of BLOCKING 1's count of unguarded behaviour:

- `to_move_is_the_side_pistol_core_puts_to_move_at_that_prefix` →
  `labels_tests.rs:148` `side_to_move_comes_from_pistol_core_and_not_from_turn_parity`,
  which does exactly what §2.2 registers: it recomputes the expected side by
  replaying through pistol-core and compares the column. A pure rename.
- `a_capture_whose_schema_version_is_unknown_is_refused_by_name` →
  `capture_tests.rs:332` `a_capture_whose_format_version_is_unknown_is_refused_by_name`.
- `a_capture_record_with_a_tab_in_a_field_is_refused_by_name` → the read-side path
  is the arity refusal, pinned by `capture_tests.rs:312`.

**The ones that leave a behaviour with no guard at all:**

- **INVARIANT 4 has none of its four.** `git grep -n "totals_of\|fields_of\|value_of"`
  shows `fields_of` and `value_of` are called only by `totals_of`, `capture.rs:169`
  and `labels.rs`, and there is **no** `#[cfg(test)]` module anywhere under
  `crates/pistol-arena/src/`. The registered mutant *"`time` made non-fatal in
  `totals_of`"* has no killer. (Its sibling mutant, *"a fourth load-bearing
  lookup added to `totals_of`"*, does die — at the pre-existing
  `report_contains_per_side_compute_fields`, `tests/run_tests.rs:97-107`, which
  asserts `nodes_a > 0` on a real run and would see zero if `totals_of` returned
  `None`. That is luck, not registration.)
- **INVARIANT 8 has none that can bite.** Three of four do not exist and the
  fourth is BLOCKING 2.
- **INVARIANT 9 has one of four, and it pins nothing.** MAJOR 4.
- **§9 rows 6, 7 and 3** — a record naming a game the report lacks, a report game
  with no record, and a header identity its inputs do not produce — have no test.
  Row 7 in particular is the *"a corpus over some of a report's games is a corpus
  over a sample nobody registered"* rule and nothing exercises it.
- **The solver-block arms**: neither
  `a_totals_line_with_solver_fields_yields_the_two_counters_it_carries` nor
  `a_partial_solver_block_refuses_the_run_by_name` exists. §11's own vacuity
  paragraph says these need no engine — *"a synthetic totals line produces [the
  solver spelling] for free"* — so the excuse the design pre-emptively removed is
  the one the implementation took anyway. The gate-off arm has a test
  (`labels_tests.rs:126`) and it `continue`s past every line carrying
  `solver_nodes`, so on this fixture it asserts about the gate-off arm only.
- **`a_mate_score_becomes_mate_in` / `a_negative_mate_score_becomes_mated_in`**:
  §2.3 says in terms that *"ALL THREE SPELLINGS GET A FIXTURE AND A TEST"*, because
  the re-review found the prior attempt's triple incomplete. One exists
  (`a_cp_score_becomes_an_eval_column_and_not_a_cp_one`, and it asserts only
  membership in the three, not the `cp`→`eval` mapping on a `cp` line).

### The tests that exist — do they pin what their names say?

Sixteen of the nineteen in `labels_tests.rs` do. The three that do not are
MAJOR 4 (`two_transposed_positions_…`), MINOR 1
(`a_capped_game_and_a_forfeited_game_…`) and BLOCKING 2
(`a_report_whose_result_contradicts_its_moves_…`).

Two more are weaker than their names but not findings:
`a_corpus_file_round_trips_through_its_own_loader_field_by_field` compares twelve
of the sixteen columns (it skips `turns_played`, `book`, `result`, `end`; the
last three are still covered by the loader's token-set refusals, so a reordering
among them is refused rather than round-tripped), and
`a_labels_run_prints_a_corpus_manifest_row_naming_its_digests` asserts two of the
six fields INVARIANT 11 names (MINOR 9).

---

## 4. THE MUTANT ANALYSIS

Taking more than the three asked for, since the registration gap is the subject.

**Dies at its registered killer:**

- *the loader's body-digest check removed* →
  `a_corpus_whose_body_digest_is_wrong_is_refused_by_name` compares
  `emit::claimed_body_digest` against a fresh `sha256_hex(body)`
  (`labels_file.rs:187-196`); with the comparison gone the appended 16-field row
  is accepted and `expect_err` fails. **Dies.**
- *the loader's `key_full` shape check removed* → `labels_file.rs:243-247` is the
  only site that rejects `0,0 1,0`; `a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name`
  rebuilds the body with an honest digest so nothing else refuses. **Dies.**
- *the loader's keyed-meaning-param check removed* → `labels_file.rs:184-186`;
  `a_corpus_missing_one_of_its_four_meaning_params_is_refused_by_name` strips each
  of the four in turn and requires the refusal to name the key. **Dies, four ways.**
- *the book boundary read as `k < opening_turns`* → at `k = opening_turns` the
  column flips to `no` and `the_position_reached_by_the_whole_book_and_nothing_else_is_flagged_book`
  fails. **Dies.**
- *the book boundary read as `k <= opening_turns + 1`* → at `k = opening_turns + 1`
  the column flips to `yes` and `the_first_position_reached_by_an_engines_own_choice_is_flagged_not_book`
  fails. **Dies.** The two together bracket the boundary to exactly one reading,
  which is what revision 2's BLOCKING asked for.
- *the corpus manifest row not printed* → `a_labels_run_prints_a_corpus_manifest_row_naming_its_digests`. **Dies.**
- *the capture/report digest binding removed* →
  `a_capture_whose_source_digest_is_not_the_reports_is_refused_by_name` crosses
  capture A with report B and requires the message *"taken from a report
  digesting"*. **Dies.**

**Dies, but not where it was registered:**

- *`key_full` rendered as bare cells* — its registered killer
  (`two_positions_differing_only_in_colour_do_not_share_a_key_full`) does not
  exist, but the loader's own `key_full` shape check refuses a bare-cell field, so
  the corpus the writer produces fails
  `a_corpus_file_round_trips_through_its_own_loader_field_by_field` at
  `expect("its own loader reads it")`. **Dies by accident.**
- *`key_seq` computed from `canonical_form`* — registered killer absent; the
  nearest type-valid mutation renders `(Coord, Player)` pairs into `key_seq`,
  which the loader's turn-token shape check refuses, so the round trip dies.
  **Dies by accident.**
- *`to_move` written as the opposite side* — killer exists under the renamed
  `side_to_move_comes_from_pistol_core_and_not_from_turn_parity`. **Dies.**
- *a corpus schema field dropped on write* / *the four keyed meaning params
  dropped on write* / *`depth_meaning` dropped* — all three make the loader refuse
  and the round trip dies. **Die.**
- *`result` and `end` collapsed into one column* — `FIELDS` becomes 15 and
  `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` indexes
  `row[15]`, which panics. **Dies, on an index rather than on the distinction its
  name is about** (MINOR 1).

**Survives:**

- *the pistol-core outcome cross-check removed* — **BLOCKING 2**. Its only
  registered killer that exists refuses on a different rule.
- *records deduplicated by `key_full` on write* — **MAJOR 4**. Its killer never
  reads a corpus, and no other test counts records against the capture's count…
  except `every_capture_record_produces_one_corpus_record_in_order`, which asserts
  `rows.len() == capture.records.len()`. That does kill a dedupe **if the fixture
  contains two records sharing a `key_full`**, which is precisely what the missing
  test was for and what I cannot settle by reading (see §8).
- *the loader's empty-field check removed* — killer
  `a_corpus_record_with_an_empty_field_is_refused_by_name` does not exist, and no
  other test presents an empty field. **Survives.**
- *the loader's schema-version check removed* — killer
  `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` does not exist.
  **Survives.**
- *a partial solver block accepted with the missing fields defaulted* — killer
  does not exist; the fixture's own lines are all-or-none, so nothing presents a
  subset. **Survives.**
- *the score read as the word after its key alone* — both registered killers
  (`a_mate_score_becomes_mate_in`, `fields_of_gives_the_word_after_score_and_the_word_after_that`)
  are absent. On this fixture the tag is `cp`, so reading only the tag would put
  `cp` in `score_value`; `a_cp_score_becomes_an_eval_column_and_not_a_cp_one`
  checks `row[7]`, not `row[8]`, and the loader's `number(f[8], …)` would then
  refuse — killing the round-trip test. **Dies by accident, on the loader.**
- *`time` made non-fatal in `totals_of`* — killer absent; a live totals line always
  carries `time`, so no existing test can see it. **Survives.**
- *the write-side TAB check removed* — **there is no check to remove**; MAJOR 2.
- *a report game with no records passed over* — killer absent, and the fixture has
  a record for every game. **Survives.**
- *the header's `capture_sha256` copied instead of re-derived* — killer absent.
  The corpus header does copy it (`labels_file.rs:126`), which is safe only
  because `labels.rs:271-281` refused already; delete that refusal and nothing
  fails. **Survives.**

**I could not settle by reading:** *records emitted in any order but the
capture's*. `every_capture_record_produces_one_corpus_record_in_order` zips rows
against `capture.records` and compares `game` and `turns_played`, which kills any
reordering that changes either — but a reordering that is a permutation within one
`(game, turns_played)` class cannot exist, and whether any two capture records
share both values depends on the fixture. I believe it dies; I cannot prove it
from the source alone.

---

## 5. DEVIATIONS AND CROSS-PACKAGE EFFECTS

**`labels::read` vs `labels_file`.** §5 says *"THE LOADER IS `labels::read`"*. The
code puts the grammar — the record type, the renderer, the manifest row and the
loader — in `crates/pistol-arena/src/labels_file.rs` and the transform in
`labels.rs`. **The split itself is right**: it mirrors `capture.rs` /
`capture_file.rs`, it is the arrangement `docs/rule9_justifications.md`'s new
entry argues for, and `labels.rs` is 299 lines — the split is what keeps it under
the cap. **What is wrong is that the name the design fixes does not exist**:
`capture.rs:11` re-exports `read`, `render`, `CaptureRecord` and
`CAPTURE_FORMAT_VERSION` so `capture::read` resolves, and `labels.rs` re-exports
nothing (`/usr/bin/grep -n "pub use" crates/pistol-arena/src/labels.rs` → no
match). MINOR 5.

**The re-added `exchange::fields_of` / `value_of` and `RecordedGame.result`.**
This is the sibling's MAJOR 4 and MAJOR 5 landing where those findings said they
belong, and it is correct here. **`totals_of` keeps its three load-bearing
lookups**: `exchange.rs:171-180` is `fields_of(line)?` followed by
`value_of(&words,"nodes")?`, `value_of(&words, TIME_FIELD)?` and
`value_of(&words,"depth_turns")?` — the same three keys in the same order, each
still `?`, each still `.parse().ok()?`. No fourth lookup is added, so row (e)'s
priced hazard has nothing to attach to and §3's claim holds. **The SPRT path is
unaffected**: `exchange.rs:76` is the only consumer and its call is unchanged.
`value_of`'s doc comment states the whole-word property (`nodes` cannot match
`search_nodes`), which is what `node_columns` depends on. `RecordedGame.result`
now has its reader (`labels.rs:212`), and the new refusal it adds to
`transcript::read` (`transcript.rs:316-324`) is inert against reports this build
writes, since `conclusion.rs:37-52` always emits `record.result.token()` and
`from_token` is its exact inverse.

**`bin/arena.rs`'s two passes moved to `passes.rs`.** `read_report` and
`capture_pass` moved verbatim: the diff shows only `fn` → `pub fn`,
`transcript::` → `crate::transcript::` and `capture::`/`capture_file::` →
`crate::…`. `--config`'s arm (`arena.rs:44-46`, `run` at `:175-244`) and
`--replay`'s (`:47-50`, `replay_pass` at `:129-173`) are otherwise untouched;
`replay_pass` now calls `passes::read_report` with the same argument. The only
user-visible change is the fallback refusal string at `:59-65`, which gains the
fourth mode — `git grep` finds no test asserting that message. The O_EXCL claim
still happens after mode parse and before dispatch (`:69`), so the fourth arm
inherits it as §1 says. **The two rule-9 entries state no line count** — I read
both; they argue single-responsibility and name no number, satisfying rule 9's
own "counts are derived, never asserted".

**`docs/label_corpus_manifest.md` does not exist.** **That is not this commit's
gap.** §12.1 says in terms: *"`docs/label_corpus_manifest.md` does not exist yet.
Whichever package's run is recorded first creates it"*, and *"The row is added to
`docs/label_corpus_manifest.md` in the commit that records the run"*. This commit
records no run and commits no artifact (hard rule 8 is clean — the diff carries no
capture, corpus, log or bench output). The file is owed by the pilot, under the
two headings §12.1 fixes, and the row's shape is already discharged here by
`labels_file::manifest_row`.

---

## 6. HARD RULES AND STYLE

- **Rule 1 (config).** Nothing new is configurable. `--labels` takes three paths
  and no budget; no code-side default is introduced. Clean.
- **Rule 3 (fail loud).** One `let _ = writeln!` (MINOR 3); otherwise every path
  ends in a named `ArenaError::config("labels" | "corpus file", …)`. The
  sibling's `unwrap_or("<none>")` class does not recur. But rule 3 also covers
  what the code accepts: MAJOR 1 (four header params unchecked) and MAJOR 2 (no
  write-side arity or emptiness guard) are its two breaches here.
- **Rule 4 (determinism).** No `HashMap` iteration, no clock, no thread on any
  choice path. `Board::stones()` is documented as *"in ascending (q, r) order …
  part of the contract"* (`board.rs:87-93`); `canonical_form` and
  `canonical_sequence` scan `Symmetry::ALL` in order. `a_rerun_…_is_byte_identical`
  pins it. Clean.
- **Rule 8 (artifacts).** No artifact committed. `docs/experiments/wp20m_impl_REVIEW.md`
  entering the tree is a review report, not an artifact, and it fixes the
  ephemeral-scratchpad problem for the sibling's findings.
- **Rule 9 (files).** `labels.rs` 299, `passes.rs` 86 — under the 300 cap
  (`tools/file_justification_check.sh:65`, `-le`); `labels_file.rs` 303 and
  `tests/labels_tests.rs` 469 — over, and both have entries stating no count.
  `bin/arena.rs` drops from 296 to 245. Clean.
- **Rule 10 (decisions).** MAJOR 3.
- **Comments.** WHY not WHAT throughout: `labels.rs:126-127` (*"pistol-core is the
  only judge of legality"*), `:64-66` (the gate-off ground), `labels_file.rs:9-15`,
  and the test comment at `labels_tests.rs:415-419` explaining why a transposition
  here is two turns swapped and not two cells (D-56). No file-top narrative, no
  `//!` in an ordinary module, no restating of names.
- **`///` docs.** Every public item in `labels.rs`, `labels_file.rs` and every
  `CorpusRecord` field carries one. `# Errors` is on `labels::run`,
  `labels_file::manifest_row` and `labels_file::read`. It is missing on all three
  public fallible functions in `passes.rs` (MINOR 6).
- **Test naming.** Behaviour-named, deterministic, no wall-clock waits — except
  the three whose names promise more than they assert (BLOCKING 2, MAJOR 4,
  MINOR 1).

---

## 7. FINDINGS

### BLOCKING 1 — thirty of fifty-one registered tests do not exist, and INVARIANT 4 has none of its four

`crates/pistol-arena/tests/labels_tests.rs` (whole file) against
`docs/experiments/wp20s_design.md` §11.

**Why it is wrong.** §11 is the pre-registration the implementation is checked
against, and CLAUDE.md's process makes a registered test set binding. Six
registered mutants are left with no killer at all (§4 above): the loader's
empty-field check, the loader's schema-version check, a defaulted partial solver
block, `time` made non-fatal in `totals_of`, a report game passed over, and the
copied-rather-than-re-derived capture identity. Four more die only by accident, at
a test registered against something else — which means a later edit that changes
that test's shape silently un-guards them. INVARIANT 4 is the one the design
argues hardest for, because it is the only invariant whose failure mode reaches
the SPRT report (*"a new lookup made load-bearing would suppress `compute.add` and
zero the SPRT report's node counts"*, §3), and it has no test of any kind: `git
grep -n "totals_of\|fields_of\|value_of"` finds no test caller, and there is no
`#[cfg(test)]` module under `crates/pistol-arena/src/`. This is not a defect in
the shipped transform — I could not find one — but the package's claim is that
these refusals are pinned, and they are not. It is the sibling's BLOCKING 1 at a
worse ratio.

**FIX.** Write them, or amend §11 by ADR withdrawing the ones judged unwritable
with the reason. Most are cheap and need no engine, exactly as §11's own vacuity
paragraph argues: the four `fields_of`/`totals_of` tests are units over synthetic
lines inside `exchange.rs` (they must live there — both functions are
`pub(crate)`); the solver-block pair, the score-triple pair, the loader refusals
(empty field, wrong arity, unknown version, bad `key_pos`, missing
`opening_turns`) and the three `§9` refusals are all reachable by editing a
fixture corpus or a fixture capture through the `rebuild` helper that already
exists at `labels_tests.rs:452`.

### BLOCKING 2 — `a_report_whose_result_contradicts_its_moves_refuses_the_run_by_name` never reaches the check it names, so INVARIANT 8 is pinned by nothing

`crates/pistol-arena/tests/labels_tests.rs:374-397`, against
`crates/pistol-arena/src/labels.rs:261-282`.

**Why it is wrong.** The test builds its contradicted report by
`staged.text.replace("result capped", "result p1_win")` and writing the result to
a **new file**. `passes::read_report` digests the file's bytes
(`passes.rs:24`), so that report's `source_sha256` is not the capture's, and
`labels::run`'s **first** refusal — `capture.source_sha256 != transcript.source_sha256`
at `:265` — fires before `agrees(transcript)` at `:282` is ever called. The test
knows this: its assertion is

```rust
stderr.contains("its moves reach") || stderr.contains("digesting")
```

and `"digesting"` is the binding refusal's word (`:267`). **Delete `agrees`
entirely — replace its body with `Ok(())` — and this test still passes**, because
the run is refused two checks earlier. The design registers four tests for
INVARIANT 8; the other three
(`the_derived_outcome_agrees_with_the_reports_own_result_field`,
`the_outcome_check_holds_when_engine_b_takes_seat_one`,
`a_forfeited_games_result_is_the_reports_and_is_not_derived`) do not exist. So the
registered mutants *"the pistol-core outcome cross-check removed"* and *"the
outcome relation gated on `a_is_p1`"* — the second being revision 1's own recorded
defect, the one that would have refused half of every paired run — have no killer
in the tree. This is the sibling's BLOCKING 2 class: a test whose name claims a
pin its driver cannot make.

**FIX.** Contradict the report **after** the capture is taken from it and re-take
the capture's `source_sha256`, or contradict it in a way the binding cannot see:
build the fixture report, capture it, then write a flipped report AND rewrite the
capture header's `source_sha256` and `capture_sha256` to the flipped report's — or,
simpler, drive the check as a unit by constructing a `Transcript` whose `result`
disagrees with its `moves` and calling the outcome check directly. Then drop the
`||` — a refusal test that accepts a second message is not a test of the first.

### MAJOR 1 — the loader checks four of the corpus's eight header params, and `opening_turns` is one of the four it skips

`crates/pistol-arena/src/labels_file.rs:174-186` against
`docs/experiments/wp20s_design.md` §5 and §11.

**Why it is wrong.** §5 says the loader refuses *"a header missing any of its
params, the four keyed meaning params included"*, and §11 registers
`a_corpus_missing_its_opening_turns_param_is_refused_by_name` against INVARIANT 6.
`read` checks `corpus_schema_version` (`:175`), the four meaning params
(`:184-186`) and `derived capture_sha256` (`:286`). It never looks for
`opening_turns`, `experiment_sha256`, `source_sha256` or `label_go`, all four of
which `render` writes (`:106-109`). The consequence is not decorative:
§2.9's whole ground for choosing the provenance reading of `book` over the
ask-boundary reading is that *"the header carries the datum and not only the
verdict"* — `opening_turns` is what lets a consumer compute the other reading. A
corpus that reaches a trainer without it passes this loader, and the `book` column
is then the only reading available, which is the loss §2.9 says the design avoids.
Likewise `source_sha256` and `experiment_sha256` are what bind a corpus to its
report; a corpus stripped of them loads clean.

**FIX.** Extend the loop at `:184` to
`["opening_turns", "experiment_sha256", "source_sha256", "label_go", "score_units", "score_sign", "mate_counts", "depth_meaning"]`,
and parse `opening_turns` as a number so a non-numeric value refuses too. Write
`a_corpus_missing_its_opening_turns_param_is_refused_by_name` against it.

### MAJOR 2 — no write-side arity or emptiness guard, and one reachable input makes the transform write a corpus its own loader refuses

`crates/pistol-arena/src/labels_file.rs:66-91` and
`crates/pistol-arena/src/labels.rs:184-194`, against
`docs/experiments/wp20s_design.md` §5.

**Why it is wrong.** §5 fixes two write-side properties on the file's own face:
*"**A field carrying a TAB refuses the run by name**, and by §2.10 **no field is
ever empty**"*, §11 registers `a_corpus_field_carrying_a_tab_refuses_the_run_by_name`
and the mutant *"the **write-side** TAB check removed — the corpus this transform
writes"*. `render_records` writes sixteen fields into a `writeln!` with no check of
either property, and there is no `no_tab`-shaped function anywhere on this path —
the sibling has one (`capture.rs:285-300`) and this half does not.

The emptiness half has a reachable input. `best` is
`record.bestmove.strip_prefix("bestmove ")?.trim()`. A capture record whose
`bestmove` field is the six characters `bestmove` plus one trailing space is
non-empty, so `capture_file::read`'s empty-field check (`:197-203`) passes it;
`strip_prefix` then yields `""`, `trim` yields `""`, and the corpus gets a record
whose tenth field is empty. `labels_file::read` refuses exactly that
(`:208-214`). **The transform therefore writes a file it cannot read, with no
refusal in between**, which is the property §2.10 exists to make impossible and
which INVARIANT 6's round trip asserts. A capture is a file, and §0's whole
premise is that this transform is a function of two files a successor may re-run —
so "no engine in this tree writes that" is the argument the sibling's own `no_tab`
doc rejects for the same reason (*"this is checked rather than assumed"*).

**FIX.** Add a `no_tab`-shaped guard beside `render_records` that refuses any of
the sixteen fields that is empty or contains a TAB, naming the record and the
column, and call it from `labels_file::render` before the body is built. Register
`a_corpus_field_carrying_a_tab_refuses_the_run_by_name` against it and add the
empty-`best` case as its sibling.

### MAJOR 3 — the fourth arm and the third pass land with no ADR line, and D-542 still records two-pass, a third arm, and a `pv` this package does not build

`docs/decisions.md` (unchanged by this commit; `git log --oneline -3 --
docs/decisions.md` → last touched at `dd6c4df`) against
`crates/pistol-arena/src/bin/arena.rs:37`, `:55-58`, `:74` and
`docs/experiments/wp20s_design.md` §1 and §12.3 residue 2.

**Why it is wrong.** D-542 (`docs/decisions.md:1152`) records the selected
mechanism as *"a **third arm** in `bin/arena.rs`'s mode match"*, the shape as
*"TWO-PASS"*, and branch B as *"`exchange::totals_of` rises to `pub(crate)` and
gains `score` and `pv` as **non-fatal `Option`s**"*. At HEAD there are four arms,
three passes, no `pv` anywhere, and `totals_of` gained nothing — it was split. The
design anticipated all of this and named it: §1 says *"**hard rule 10 wants the
amendment rather than the drift**, so the ADR line this package lands records the
pass count and the arm count"*, and §12.3 lists *"Two ADR acts … owed at landing"*.
Neither landed. Hard rule 10 calls unrecorded architecture drift a breach, and the
tree and the ADR now disagree about what was built.

**FIX.** Append the two ADR lines the design specifies — one recording the fourth
arm and third pass with the three grounds §1 gives, one recording that branch B
shipped as `fields_of` + an unchanged `totals_of` and that no package builds the
`pv` half — each in `D-n: choice — reason — what flips it` form.

### MAJOR 4 — `two_transposed_positions_are_two_records_sharing_a_key_full` never opens a corpus, so INVARIANT 9 is pinned by nothing

`crates/pistol-arena/tests/labels_tests.rs:414-448`.

**Why it is wrong.** The name promises **two records** of a corpus sharing a
`key_full`. The body builds two `Vec<Turn>` by hand, replays each through
`GameState`, and asserts `state.key()` and `canonical_form` agree across the two
orders. It never runs the arena, never reads a corpus, and never touches
`labels.rs` or `labels_file.rs` — it is a pistol-core property test living in a
schema test file, and `crates/pistol-core/src/symmetry.rs` is where that property
already belongs. Its registered mutant is *"records deduplicated by `key_full` on
write"*, and deleting duplicate records in `labels::run` changes nothing this test
observes. §11's other three INVARIANT 9 tests
(`two_positions_alike_up_to_a_symmetry_share_a_key_full_and_not_a_key_pos`,
`two_symmetric_prefixes_share_a_key_seq_and_two_transpositions_do_not`,
`two_positions_differing_only_in_colour_do_not_share_a_key_full`) do not exist, so
INVARIANT 9 — *"the corpus deduplicates nothing"* — has no guard. The third of
those is also the only killer for *"`key_full` rendered as bare cells"*, and §11
spends a paragraph specifying its fixture's two conditions; that paragraph is
unspent.

**FIX.** Make the test read the corpus: assert that the two hand-built prefixes
appear as two distinct records whose `key_full` fields are equal and whose
`key_seq` fields differ — driving them through the pipeline, or at minimum through
`labels::run` over a synthetic capture. Then write
`two_positions_differing_only_in_colour_do_not_share_a_key_full` with the fixture
§11 specifies (shared cell set, not symmetry images).

### MINOR 1 — `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` never sees a forfeited game

`crates/pistol-arena/tests/labels_tests.rs:211-219`. It asserts only that
`row[14]` is one of three tokens and `row[15]` one of two. The fixture is a
self-play run against the `honest` stub, which forfeits nothing, so the
distinction the name is about is never exercised; the assertion would pass on a
corpus where every game is capped and normal. **FIX.** Drive one game to a forfeit
(the stub already has `garbage`, `exit` and `hang` behaviours the sibling's tests
use) and assert that the forfeited game's records carry `end forfeit` with the
report's own `result`, which also supplies the missing
`a_forfeited_games_result_is_the_reports_and_is_not_derived`.

### MINOR 2 — `labels_file::header` takes the first match where its sibling refuses a duplicate

`crates/pistol-arena/src/labels_file.rs:162-168` uses `find_map`;
`crates/pistol-arena/src/capture_file.rs:137-151` refuses with *"it carries more
than one `{kind} {key}` line, so there is no one answer to read"*. The corpus
header is outside the body digest, so a file carrying two `param depth_meaning`
lines is read by its first and the second is silently ignored — one grammar, two
strictnesses, in two files written a day apart. **FIX.** Use the sibling's
`header`, or lift it into one place both call.

### MINOR 3 — `let _ = writeln!` in the record renderer

`crates/pistol-arena/src/labels_file.rs:69`. Hard rule 3 asks for the loud
spelling; the sibling's review graded the same shape MINOR 5 at
`capture.rs:105-107` and it recurs here. Infallible for `String`, so nothing can
go wrong — which is the argument for `push_str(&format!(…))`, not for the
discard. **FIX.** `out.push_str(&format!(…))`.

### MINOR 4 — the loader accepts number spellings the writer never produces

`crates/pistol-arena/src/labels_file.rs:215-218`, `:262-279`. §5's loader list
ends with *"a number spelled a way this format does not write"*. `i64::from_str`
accepts `+5`, so `+5` in `score_value` or `depth_turns` loads. The tree's own
idiom for this is `bin/arena.rs:112-126`'s `parsed.to_string() != word`
round-trip. **FIX.** Apply that round-trip inside the `number` closure.

### MINOR 5 — `labels::read` does not exist

§5 names the loader `labels::read`; it is `labels_file::read`, and `labels.rs`
re-exports nothing, so the design's spelling does not resolve. `capture.rs:11`
shows the intended shape for the same split. **FIX.**
`pub use crate::labels_file::{CORPUS_SCHEMA_VERSION, Corpus, CorpusRecord, read, render};`
at the top of `labels.rs`, and point the tests at it.

### MINOR 6 — `passes.rs`'s three public fallible functions carry no `# Errors`

`crates/pistol-arena/src/passes.rs:13`, `:35`, `:63`. All three return
`Result<_, ArenaError>` from a `pub` item in a `pub mod`; CLAUDE.md asks for
`# Errors` where a caller would reasonably handle them, and `read_report`'s
whole doc is about a refusal it raises. **FIX.** Add the sections.

### MINOR 7 — the `position start` spelling is written a second time

`crates/pistol-arena/src/labels.rs:138-142` reconstructs
`format!("{} start", pistol_cli::protocol::POSITION)`; `capture.rs:47-54` has the
private `position_line` that produced the string being compared against. The
comparison at `:143` is the whole load-bearing pairing between a capture record
and the report's prefix, and it now depends on two independent spellings of one
grammar staying equal. **FIX.** Make `capture::position_line` `pub(crate)` and
call it.

### MINOR 8 — §9 row 11's refusal is unreachable, so its registered test cannot be written as registered

`crates/pistol-arena/src/labels.rs:129-137`. The prefix replayed there comes from
`game.moves`, and `transcript::read` already replays every recorded move list
through `GameState::make_turn` and refuses the whole report on an illegal turn
(`transcript.rs:361`, `:376-390`). So *"a captured `moves` prefix is not a legal
game under pistol-core"* cannot fire from any report `read_report` returns, and
`a_moves_prefix_that_is_not_a_legal_game_is_refused_by_name` has no driver short of
bypassing the reader. The `?` is correct defensive code and should stay. **FIX.**
Record it: either write the test as a unit against `labels::one` with a
hand-built `Transcript`, or amend §11 to withdraw it with this reason — the design
itself notes at §2.7 that the two sides share their input.

### MINOR 9 — the manifest-row test asserts two of the six fields INVARIANT 11 names

`crates/pistol-arena/tests/labels_tests.rs:399-411` checks `"corpus_manifest "`
and the body digest. INVARIANT 11 requires the row to name the corpus's body
digest, the schema version, the capture's identity, the report's two digests **and
the artifact's path** — *"the field the manifest exists to index"*. **FIX.**
Assert all six against `manifest_row`'s inputs.

---

## 8. THE STRONGEST ATTACK THAT DID NOT LAND

**That `transcript.games.get(record.game)` indexes by position while the
"every game has a record" check compares `game.index`.** `labels.rs:110` looks a
game up by `Vec` position and `labels.rs:290` matches on `RecordedGame::index`. If
a report's `game` records were ever non-contiguous or out of order, the two would
disagree and the transform would silently label a record against **another game's
move list** — every column from `moves` to `end` wrong, and the outcome cross-check
would still pass because it iterates the games independently. That is a genuine
wrong-answer shape.

It does not land. `crates/pistol-arena/src/transcript.rs:289-297` refuses at read
time: `if index != games.len()` → *"the report's Nth `game` record is `game
{index}`; the records must be the run's own contiguous prefix or the pairing is
not what the report says it is"*. So `games[i].index == i` is an invariant of every
`Transcript` that exists, the two lookups are the same lookup, and the position
check at `labels.rs:143` would catch a mismatch even if it were not.

Two smaller attacks also failed. **The capture's `experiment_sha256` is never
compared to the report's** — but `capture_sha256` is re-derived from *the
transcript's* `experiment_sha256` at `labels.rs:271-275` and compared against the
capture's header value, which was written from the capture-time transcript's, so a
disagreement refuses; and `source_sha256` equality already pins the report's bytes
exactly. And **`node_columns` counts presence by `value_of(...).is_some()`**, which
returns `None` for a key that is the line's last word — but `render_info` never
ends a line on a bare key, and any such line refuses on the subset rule rather
than being mis-read.

---

## 9. WHAT I COULD NOT SETTLE BY READING, AND THE RUN THAT WOULD

1. **Whether the fixture's four games include a DECIDED game, and whether one of
   those has engine B in seat one.** This decides whether the registered mutant
   *"the outcome relation gated on `a_is_p1`"* dies by accident: if every game in
   `staged()`'s two-opening, turn-cap-8 self-play run ends `capped`, an `a_is_p1`
   gate on the two `Win` arms is invisible and the mutant survives everything.
   `a_report_whose_result_contradicts_its_moves_…` asserts at least one `result
   capped` exists; it says nothing about a decided one. **The run:**
   `cargo test -p pistol-arena --test labels_tests -- --nocapture` with a
   temporary `eprintln!` of each game's `result` and `a_is_p1`, or simply
   `/usr/bin/grep -o 'result [a-z0-9_]*' ` over one scratch report from a manual
   `arena --config` run of that spec.
2. **Whether any two capture records in the fixture share both `game` and
   `turns_played`** — the one input under which
   `every_capture_record_produces_one_corpus_record_in_order` fails to kill the
   reordering mutant. **The run:** the same fixture report, then
   `awk -F'\t' '{print $1,$2}' capture-*.txt | LC_ALL=C sort | uniq -d`.
3. **Whether the eight tests that call `pistol_arena::labels_file::read` compile
   against a `pub(crate)`-free path** — I read the `lib.rs` diff and
   `pub mod labels_file;` is there (`lib.rs:59`), so I expect yes, but only a
   build settles it. **The run:** `cargo test --workspace --locked --no-run`.
4. **Whether gate 17 accepts the two new `docs/rule9_justifications.md` entries.**
   I read both and they state no count, which is the rule's own condition, and the
   cap check is `-le 300` so `labels.rs` at 299 needs none — but the gate also
   refuses an entry for a file *under* the cap, and I did not trace whether
   `passes.rs` or `labels.rs` could ever be registered by mistake. **The run:**
   `tools/ci.sh` gate 17, or `tools/file_justification_check.sh` alone.
5. **Every gate.** Per the dispatch I ran no `cargo` and no gate script, so this
   report makes no claim that the suite is green. The claims above are read off
   the source and the diff.
