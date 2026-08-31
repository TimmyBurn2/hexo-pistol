# SCOPED RE-REVIEW — `docs/experiments/wp20s_design.md` revision 4

## Header

- **Revision adjudicated**: `4a12b46936feb7a58670d114623f7a7779b2f374` (`docs(wp20s): the round where the guard is applied to every finding …`).
- **Matches HEAD**: **YES**. `git rev-parse HEAD` = `4a12b46936feb7a58670d114623f7a7779b2f374`, branch `dev`.
- **Tree state**: **clean throughout** — `git status --porcelain` printed nothing at the start and nothing at the end. The only file I wrote is this one.
- **What I ran**: `git` (`log`, `rev-parse`, `status`, `show`, `diff`, `diff -U0`, `grep`), `/usr/bin/grep`, `sed`, `awk`, `wc`, `cat`, `cut`, `head`, `comm`, `LC_ALL=C sort`, `uniq`, and `python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` — **green, 120 citations checked, 0 unreproduced** (revision 3: 119; revision 2: 104; revision 1: 60).
- **What I refused to run**, per the dispatch's hard constraint: `cargo` in any form, `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Runs I could not make are named in Part 3.
- **Binding reading, all in full**: `docs/experiments/wp20s_design_REVIEW_rev3.md` (N-B1, N-M1…M8, N-m1…m19 and its Part 1 dispositions); `git show df07a38:docs/experiments/wp20s_design.md`; the complete `git diff df07a38 4a12b46` for this file (28 hunks, enumerated with `git diff -U0`); `CLAUDE.md`; `docs/process.md`; `docs/decisions.md` D-483, D-518, D-531, D-537, D-539…D-548; `docs/experiments/wp20_dispatches.md` (all three dispatches); `docs/experiments/wp20m_design.md` **revision 6** (`ff1c575`) — §8, §9, §13, and the freeze table; `docs/experiments/matrix_stage3_detector.md` §5.8 and its band table.
- **Code read**: `crates/pistol-core/src/{board,coord,symmetry,state,zobrist}.rs`, `crates/pistol-arena/src/{replay,openings,transcript,exchange}.rs`, `crates/pistol-arena/tests/replay_chain_tests.rs`, `crates/pistol-cli/src/report.rs`, `crates/pistol-cli/src/corpus/emit.rs`, `crates/pistol-search/src/{info,search}.rs`, `configs/{arena_smoke_v0,random_openings_v1}.toml`.

---

## VERDICT: **FAIL**

**0 BLOCKING · 5 MAJOR · 12 MINOR.**

**Revision 4 is the strongest revision of this document, and the two things this round was dispatched to do it did.** N-B1 is closed — §8 now names a one-sample binomial test in both places, no third site dissents, and the rule is complete enough to compute a minimum from on all six axes. N-M1 is closed — the two book tests name two different positions and both mutants die. N-M4, N-M3, N-M5 and N-M7 are clean, and I verified N-M7's claim about WP-2.0-M's §8 against `ff1c575` directly: it is true. §0.1a now carries **28 rows, one per finding**, which is exactly the format asked for.

**It fails on the truth of that table, and on one remedy that landed at three sites out of five.**

- **`depth_meaning`, the fourth keyed param N-M6 forced, is not in §5's header enumeration.** `:752-754` still reads *"THE HEADER CARRIES, as `param`: … **the three unit params below**; and `opening_turns`"*, while `:771-775` declares FOUR keyed params and `:794-795` has the loader refuse *"a header missing any of its params, **the four keyed meaning params included**"*. It has no test and no mutant, and the registered test is still `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name`. **A writer built from the enumeration emits a corpus the loader built from the same section refuses.** §0.1a's N-M6 row says what pins it now is *"§5's header enumeration"* — the one place in §5 that omits it. That is the identical defect revision 3 was failed for (NEW-B1's row naming a loader check that existed in no rule), on the very next round.
- **Five findings the table records as applied were not applied at all.** N-m8, N-m15, N-m16, N-m17 and N-m19 are **byte-identical to `df07a38`** — I checked each passage against `git show df07a38:` and the `git diff -U0` hunk list contains no hunk in any of those five ranges. The rebuilt guard table is false in more rows than the twelve-row table it replaced.
- **N-m8 is not a bookkeeping matter.** `two_positions_differing_only_in_colour_do_not_share_a_key_full` **can be red on correct code**, and I built the counterexample rather than asserting it (below, R-M4). The table says the fixture condition is stated in §11. §11 contains one fixture-condition clause and it is about a different test.

**What is genuinely closed, and I attacked all of it:** the `q,r:p1` spelling and its non-collision, the book boundary from both sides, the one-sample rule and its six inputs, INVARIANT 4's strength in all three places, the TAB seam, the `forfeit_by` reversal, the `totals_of` lean on WP-2.0-M, the manifest row and its six fields, and the whole citation set.

---

# PART 1 — DISPOSITION OF EVERY REVISION-3 FINDING

**Summary: 17 APPLIED · 3 PARTIALLY APPLIED · 3 APPLIED BUT INTRODUCED A NEW DEFECT · 5 NOT APPLIED · 0 MOOT.** Twenty-eight findings, twenty-eight dispositions.

## BLOCKING

### N-B1 — §8 defined its minimum twice — **APPLIED**, and the rule is now complete

`:950-952` is rewritten to *"the smallest number of win-proving firings on disjoint positions at which a **ONE-SAMPLE BINOMIAL** test of the incumbent `p0` against the alternative reaches that level and power"*, with the history in a parenthesis. Item 5 at `:943-948` says the same thing.

**No third site dissents.** `LC_ALL=C /usr/bin/grep -n "two-proportion\|one-sample\|ONE-SAMPLE\|binomial"` returns six lines: `:10-12` and `:84` (the revision header and §0.1a, both narrating the defect in the past tense), `:943`, `:945` and `:947` (item 5, where *"two-proportion"* appears only inside a quotation of revision 2), and `:951` (the operative sentence). **Every prescriptive use is one-sample.** ✓

**Is the rule complete enough to compute a minimum from?** I checked the dispatch's six axes one at a time:

| axis | where | verdict |
|---|---|---|
| band named | item 4, `:934-942` — *"THE BAND IS THE TRIGGER-RICH ONE"* | ✓ |
| null named | item 2, `:923-925` — the incumbent recall, cited to the closed arc | ✓ |
| alternative named | item 3, `:926-933` — *"THE ARC'S MEASURED COLUMN BOUND FOR THAT SAME BAND"* | ✓ **and no longer ambiguous**: the trigger-rich bound is a single figure, `0.857` (12 of 14, `matrix_stage3_detector.md:611`), so "for that same band" resolves it where "the lower end" did not |
| level and power fixed | item 1, `:906` — 0.05 and 0.95 | ✓ |
| test named | item 5 + `:951` — one-sample binomial | ✓ |
| denominator key named | `:980-987` — *"WHAT COUNTS AS DISJOINT IS `key_full`"* | ✓ |

**All six.** A successor has one arithmetic problem with one answer and no choice left in it, which is what D-537 requires. One residue in the paragraph that argues the rule's bias, raised as **R-m5**, and the section's numeral self-claim is separately false (**R-M5**) — neither reaches the rule itself.

## MAJOR

| # | finding | disposition | evidence |
|---|---|---|---|
| N-M1 | the second book test named its sibling's position | **APPLIED** | see below |
| N-M2 | the loader check existed in no rule | **PARTIALLY APPLIED** | `:791-793` adds it to §5's loader; **INVARIANT 6 (`:1058-1060`) is untouched** and the §0.1a row names it — **R-M3** |
| N-M3 | §2.8 said §4 carries `forfeit_by` | **APPLIED** | `:488-489` — *"**`forfeit_by` — not recovered, and not needed.** §4 does NOT widen the transcript reader for it, and says why"*. Matches §4's `:688-693` exactly |
| N-M4 | the third byte-identity site | **APPLIED** | `:699-702` — *"so **INVARIANT 4 is untouched by it** — that invariant is about the SPRT path's node accounting and makes no byte-identity claim"*. `/usr/bin/grep -n "byte-identity\|byte-identical"` now returns six lines: `:88`, `:114` (§0.1a's own narration), `:648` (§3, correct), `:701` (§4, corrected), `:843` and `:1057` (INVARIANT 5's re-run claim, a different property). **All three descriptions of INVARIANT 4 agree with INVARIANT 4** ✓ |
| N-M5 | the TAB mutant paired across the seam | **APPLIED** | `:1169-1170` splits into *"the **write-side** TAB check removed — the corpus this transform writes"* → `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` and *"the **read-side** TAB refusal removed — the capture this transform reads"* → `a_capture_record_with_a_tab_in_a_field_is_refused_by_name`. Both tests are registered (`:1135`, `:1134`). **Neither is paired across the seam and each dies at its own side** — see the closure section. One mapping residue, **R-m9** |
| N-M6 | `depth_meaning` discharged in no section | **APPLIED BUT INTRODUCED A NEW DEFECT** | §2.5 `:407-408` and §5 `:771-775` and the loader `:794-795` all carry it; **§5's header enumeration, the test register and the write-side mutant do not** — **R-M1** |
| N-M7 | the `totals_of` lean omitted | **APPLIED**, and I verified the claim | see below |
| N-M8 | §0.1a incomplete | **APPLIED in form, FAILED in substance** | 28 rows, one per finding ✓; **at least seven are false** — **R-M2**, R-M1, R-M3, R-M4 |

**N-M1, checked against §2.9's own rule rather than against the fix text.** §2.9 `:544-546` derives `k <= opening_turns`; `replay.rs:137-138` is `for (at, recorded) in game.moves.iter().enumerate()` / `if at >= opening_turns as usize {`, so turn indices `0 … opening_turns-1` are the book's and a prefix of length `k` is all-book iff `k <= opening_turns`. The two tests now name **two different positions**:

| test | position | flag |
|---|---|---|
| `the_position_reached_by_the_whole_book_and_nothing_else_is_flagged_book` (`:1145`) | `k = opening_turns` | `book` |
| `the_first_position_reached_by_an_engines_own_choice_is_flagged_not_book` (`:1146`) | `k = opening_turns + 1` — the engine's first choice is at turn index `opening_turns`, so the position it *reaches* is one longer | not `book` |

**Both mutants die at their named tests.** `k < opening_turns` (`:1178`) flags `k = opening_turns` as `no` and the first test fails. `k <= opening_turns + 1` (`:1179`, new) flags `k = opening_turns + 1` as `yes` and the second fails. The boundary is now guarded from both sides, which is exactly what N-M1's FIX asked for. ✓

**N-M7, verified against WP-2.0-M revision 6 at HEAD.** `wp20m_design.md:735-736` still reads *"**The visibility change is for WP-2.0-S**, so that package adds fields to one parser instead of writing a second"*, and `:770-771` still reads *"it is the guard WP-2.0-S inherits when it adds `score` and `pv` as non-fatal `Option`s (D-542)"* — both inside the block WP-2.0-M's freeze table records as lifted from a passed revision. **WP-2.0-S §3 does neither**: `:609-628` splits into `fields_of` plus an unchanged `totals_of` and `:645-646` says *"A word list adds no lookup to `totals_of` at all"*; `:483-484` says no package builds the `pv` half. **The claim at `:1296-1308` is true at HEAD**, both quotations reproduce verbatim, and the assignment (*"a prose-and-ADR correction WP-2.0-M owes, not a change to §3"*) is the right one. ✓ The commit that carried WP-2.0-M to revision 6 (`ff1c575`) did not touch §8 — I checked the hunk list.

## MINOR

| # | finding | disposition | evidence |
|---|---|---|---|
| N-m1 | the `---` glued to §2.10's last word | **APPLIED** | `:594-596` — blank line, `---`, blank line |
| N-m2 | the last `turn` test name | **APPLIED** | `:1120` — `a_record_whose_turns_played_and_moves_disagree_is_refused_by_name` |
| N-m3 | the "only two numerals" self-claim | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:912-918` replaces it with a claim false in two ways, falsified inside its own item — **R-M5** |
| N-m4 | `to_move`'s token set never stated | **APPLIED**, and I attacked the new spelling | `:797-802`; see the attack section |
| N-m5 | "frame" is the arc's word for a different axis | **PARTIALLY APPLIED** | items 3 and 4 change four occurrences; **`:940` keeps two** — **R-m4** |
| N-m6 | items 3 and 4 state the pairing twice | **PARTIALLY APPLIED** | item 3 now points at item 4; **the bias paragraph at `:969-973` states it a third time in the phrasing item 3 deletes** — **R-m5** |
| N-m7 | §9's "one place" claim | **APPLIED** | `:1014-1017` narrows it to enumeration and states D-423's shape. One residue, **R-m7** |
| N-m8 | the colour test's fixture condition | **NOT APPLIED** | `/usr/bin/grep -n "fixture"` returns six lines; none states a condition for `two_positions_differing_only_in_colour_do_not_share_a_key_full`. §11's only fixture-condition clause (`:1209-1213`) is about `the_derived_outcome_agrees_with_the_reports_own_result_field`. **Byte-identical to `df07a38`** — **R-M4** |
| N-m9 | INVARIANT 11 omitted the artifact path | **APPLIED** | `:1070-1072` — *"**and the artifact's path** — the field the manifest exists to index"*. Six fields, matching §12.1's six |
| N-m10 | §12.1 asserted WP-2.0-M's file-creation behaviour | **APPLIED** | `:1250-1255` — *"Whichever package's run is recorded first creates it"*, with the reason. One new "only", **R-m6** |
| N-m11 | the uncited "ten leans" | **APPLIED** | `:1292-1294` — *"Several of this document's leans … and the four named above are the ones this section can support from its own text"* |
| N-m12 | `to_move`'s test mapped to an invariant that did not state its rule | **APPLIED** | INVARIANT 3 `:1047-1048` now opens *"**`to_move` is the side pistol-core puts to move at that prefix**, never a parity of the turn index"*. Two mis-mappings remain, one of them new — **R-m9**, **R-m10** |
| N-m13 | §9 row 4's other two limbs | **APPLIED** | `:1136-1137` — `a_capture_record_with_the_wrong_field_count_is_refused_by_name` and `a_capture_record_with_an_empty_field_is_refused_by_name`, both mapped to 7 and both distinct from the corpus-loader pair at `:1109-1110` |
| N-m14 | `opening_turns` had no test | **APPLIED** | `:1138` — `a_corpus_missing_its_opening_turns_param_is_refused_by_name` |
| N-m15 | *"overwrites both counters"* | **NOT APPLIED** | `:382-384` is **byte-identical to `df07a38:337-339`**; no hunk touches §2.4 |
| N-m16 | the half-read sample-size citation | **NOT APPLIED** | `:909-911` is **byte-identical to `df07a38:846-848`**. `configs/random_openings_v1.toml:47-52` reproduces and its last line is *"At 2000 pairs the same bounds achieve alpha 0.048 and power 0.945"* — the passage's own conclusion is that the gap closes, which the design's sentence does not carry |
| N-m17 | §7's narrow scope | **NOT APPLIED** | `:877-879` is **byte-identical to `df07a38:812-814`**; still *"which on this corpus's records loses nothing (§2.1)"* while §2.1 `:275` and §8 `:987` ground it on any ongoing position |
| N-m18 | the `Phase` citation carried a claim it does not | **APPLIED BUT INTRODUCED A NEW DEFECT** | `crates/pistol-core/src/turn.rs:25-29` is gone (confirmed by a `comm` over both revisions' citation sets); the replacement attributes the fact to the **wrong invariant** — **R-m3** |
| N-m19 | the arena-produced fixture had no route | **NOT APPLIED** | `:1209-1213` is **byte-identical to `df07a38:1125-1129`**. `replay_chain_tests.rs` and the word "stub" appear **nowhere in the document except §0.1a's own row** (`:111`) |

---

# PART 2 — THE §0.1a AUDIT, AND NEW DEFECTS

## The §0.1a audit

**Completeness: PASS.** 28 rows — N-B1, N-M1…N-M8, N-m1…N-m19 — one per finding, `—` where nothing was spent, matching WP-2.0-M §0.2b's format. I checked the reverse direction too: I enumerated all 28 hunks with `git diff -U0 df07a38 4a12b46 | /usr/bin/grep "^@@"` and mapped each to a row. Every hunk maps except three: the revision header (meta), and two improvements that are not remedies of any finding — `:804`'s *"the dispatch's"* → *"the **WP-2.0** dispatch's"*, and §2.10's `q,r:1` → `q,r:p1`, which is folded into N-m4's row without saying it reached §2.10 as well as §5. Neither is a loss and the table claims one row per *finding*, not per hunk, so I record them as observations rather than findings.

**Truth: FAIL. Seven of the twenty-eight rows are false about the document they describe.**

| row | what it claims | what is there |
|---|---|---|
| **N-M2** | *"pins NOW: §5's loader enumeration **and INVARIANT 6**"* | §5's loader ✓; **INVARIANT 6 (`:1058-1060`) enumerates *"schema version, body digest, record arity, header params or token set"* and names neither key's shape** — and §5 uses "token set" for `to_move`/`result`/`end`/`book` only (`:790`), not for an unbounded value space |
| **N-M6** | *"pins NOW: **§5's header enumeration** and the loader's missing-param refusal"* | **§5's header enumeration (`:752-756`) says *"the three unit params below"* and does not list `depth_meaning`**; the missing-param test is named for three |
| **N-m8** | *"the colour test's fixture condition is stated … pins NOW: §11's fixture-condition clause"* | **no such clause exists for that test**; §11's one clause is about a different test |
| **N-m15** | *"the enforcing site names the two fields it overwrites"* | `:382-384` byte-identical to revision 3 |
| **N-m16** | *"the sample-size citation states what the cited passage concludes"* | `:909-911` byte-identical to revision 3 |
| **N-m17** | *"§7's scope matches §2.1's"* | `:877-879` byte-identical to revision 3 |
| **N-m19** | *"the arena-produced fixture names the stub harness it uses"* | `:1209-1213` byte-identical to revision 3; the harness is named only in this row |

**And two more rows are half-true**: N-m5 (*"'frame' becomes 'band'"* — `:940` keeps two) and N-m6 (*"§8 items 3 and 4 state the pairing once"* — `:969-973` states it a third time).

**Did any remedy spend something and go unrowed?** I checked every hunk for a trade. **One did**, and it is the N-m6/N-m3 rewrite of item 3: deleting *"the lower end"* from the rule left the bias paragraph eighteen lines below arguing from a phrase the rule now names as a redundant second statement of itself (R-m5). N-m6's row says *"pinned BEFORE: —"*. **It is a small spend and I want to be exact that it costs no number** — the trigger-rich bound `0.857` is both "the bound for that band" and "the lower end across bands" — so it is a MINOR, not the fourth instance of D-548's class. **The round's real failure is not a spend at all: it is five rows describing work that was never done.** That is a different and, for an instrument, worse failure — a guard table that records intent rather than diff cannot catch the class it exists for, because its author reads it as a checklist already ticked.

---

## MAJOR

### R-M1 — §5 states the corpus header's params two ways, and the fourth one has no enumeration, no test and no mutant

Three sites say **three**, two say **four**:

| site | count |
|---|---|
| `:752-756` — *"THE HEADER CARRIES, as `param`: the corpus schema version; the source report's `experiment_sha256` and `source_sha256`; the label `go` line; **the three unit params below**; and `opening_turns`"* | 3 |
| `:1112` — `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name` | 3 |
| `:1162` — the mutant *"**the three unit params** dropped on write"* | 3 |
| `:771-775` — *"**FOUR** properties a column name cannot carry are therefore keyed params … `score_units`, `score_sign`, `mate_counts`, and **`depth_meaning`**"* | 4 |
| `:794-795` — the loader refuses *"a header missing any of its params, **the four keyed meaning params included**"* | 4 |

**Why this is a wrong answer and not a count.** An implementer building the writer from §5's own enumeration emits three unit params and `opening_turns`. An implementer building the loader from §5's own loader paragraph refuses a header missing `depth_meaning`. **The run then refuses its own output.** The design owes the header's contents once, in the section that owns the header, and it states them twice and differently — the exact D-423 shape the document quotes at itself at `:1016`.

**And the obligation N-M6 forced is still unpinned.** `depth_meaning` has **no test** (the registered one is named for three params and an implementer writes three cases) and **no mutant** (the write-side mutant is named for three). Revision 3's FIX said in terms: *"Then either extend `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name` to four or rename it."* Neither was done. So §2.5's stated consequence — *"a consumer reading the column without the discriminator would average two different quantities"* (`:410-411`) — is guarded by nothing that can go red.

**FIX.** Three edits: `:754` → *"the four keyed meaning params below"*; `:1112` → `a_corpus_missing_one_of_its_four_keyed_meaning_params_is_refused_by_name`; `:1162` → *"the four keyed meaning params dropped on write"*. Nothing else changes.

### R-M2 — five findings the guard table records as applied were not applied at all

N-m8, N-m15, N-m16, N-m17 and N-m19. Each passage at HEAD is **byte-identical to `df07a38`**, and `git diff -U0` contains no hunk in any of the five ranges. Their rows are at `:100`, `:107`, `:108`, `:109`, `:111`.

**Why this is MAJOR and not five MINORs restated.** §0.1a's stated purpose (`:74-80`) is that *"a fix round is where this arc loses true things"* and that the table is what shows the author what a round did. **A table whose rows are written from the review's FIX list rather than from the round's own diff is an instrument that reports its input.** Revision 3 was failed for two false rows out of twelve; revision 4 has seven out of twenty-eight, and five of them are of a kind the previous round did not have — rows for changes that do not exist. The document's own closing sentence on the point (`:117-120`) is *"a guard table that is checked less carefully than the document it guards is an instrument reported twice"*, which is precisely what happened.

**One of the five carries a live defect** (N-m8, below); the other four carry the MINORs revision 3 raised, which now stand unfixed and falsely marked closed. The substance of N-m19 is worth restating because hard rule 8 is involved: §11 `:1212` requires `the_derived_outcome_agrees_with_the_reports_own_result_field`'s reports to be *"produced by the arena"*, hard rule 8 forbids committing a match log, and the route — `crates/pistol-arena/tests/replay_chain_tests.rs:12-31`, which runs the real `arena` binary against `STUB` into a scratch directory, verified — appears **only inside the change table**, which is not where an implementer looks for a test's construction.

**FIX.** Apply the five. Then adopt the rule that makes the table an instrument: **a row is written from `git diff`, not from the finding list** — every row's NOW column must name a `file:line` the author re-read after the edit.

### R-M3 — INVARIANT 6 pins neither key shape, and two registered tests now map to invariants that do not state their rules

§5's loader gained *"a `key_seq` whose elements are not turn tokens, or a `key_full` whose elements are not `q,r:p1` / `q,r:p2` pairs"* (`:791-793`) — the rule N-M2 asked for, and it is right. **INVARIANT 6 was not widened**: `:1058-1060` still enumerates *"schema version, body digest, record arity, header params or token set"*.

Two consequences, one carried and one new:

- `a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name` maps to **6** (`:1133`), which does not state its rule. Carried from revision 3.
- `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` maps to **6** (`:1135`) — **new this round**, and worse-fitting: INVARIANT 6 is about *"a corpus file round-trip[ping] through **its own loader**"*, and §5's TAB rule (`:728`) is a **write-side** refusal by the transform, not a loader refusal. The mutant it guards is *"the **write-side** TAB check removed"*. So N-M5's remedy, which correctly separated the two sides in the mutant table, re-joined them in the invariant column.

**Neither key-shape check has a mutant**, and the `key_seq` limb has no test (**R-m8**).

**FIX.** Widen INVARIANT 6 to name the two key columns' shapes; map the write-side TAB test to INVARIANT 5 or state a new invariant for the write-side grammar; add the two mutants.

### R-M4 — `two_positions_differing_only_in_colour_do_not_share_a_key_full` can be red on correct code, and I built the pair

The fixture condition is stated nowhere; §0.1a says it is in §11; it is not. **This is not hypothetical — here is a legal pair that falsifies the test.**

`canonical_form` is *"the least of its twelve images"* under `Symmetry::ALL` (`crates/pistol-core/src/symmetry.rs:157-176`, read), and `transform` maps `(cell, player)` to `(symmetry.apply(cell), player)` — **colour preserved, no translation** (`:148-155`). The twelve symmetries fix the origin, and game rule 3 puts turn 1's single stone there.

Take two 3-turn prefixes over the cell set `{(0,0), (1,0), (0,1), (-1,0), (0,-1)}`:

- **A** — turn 1: P1 `(0,0)`; turn 2: P2 `(-1,0)`, `(0,-1)`; turn 3: P1 `(1,0)`, `(0,1)`.
- **B** — turn 1: P1 `(0,0)`; turn 2: P2 `(1,0)`, `(0,1)`; turn 3: P1 `(-1,0)`, `(0,-1)`.

Both are legal (all cells within hex-distance 8 of a stone), both have `turns_played = 3` with the required 3-P1 / 2-P2 split, and **they differ only in colour**. The 180° rotation maps A's stone list onto B's stone list with colour intact, so **A and B are symmetry images and share one `canonical_form` — one `key_full`.** The test asserts they do not, and fails on a correct implementation.

**The mutant is fine** — *"`key_full` rendered as bare cells"* (`:1164`) dies on any colour-differing pair with a shared cell set, which "differing only in colour" guarantees. It is the test that needs the condition: **the pair must not be images of one another under any of the twelve symmetries.**

**FIX.** One clause beside the test: *"its two positions must not be symmetry images of one another — a colour difference that a lattice symmetry realises is not a colour difference to this key."*

### R-M5 — §8's replacement numeral self-claim is false in two ways, and both falsifiers are inside its own item

`:912-918`, new this round:

> **They are here deliberately, and the document does not claim they are its only numerals** — a claim it has made wrongly in three revisions running. What it claims is narrower and checkable: **no number in this document is one a pre-registration, gate or criterion consumes as a threshold.** The figures §8 cites from the closed arc, and the measured pair in the note below, are named as measurements belonging to documents that own them and **are not restated as values here**.

**Limb 1 is false of the two numbers the same sentence is about.** `:906` fixes *"a level of 0.05 and a power of 0.95"*, and `:950-952` — the section's operative rule, which the document says D-537 forbids a successor to loosen — is *"the smallest number of win-proving firings … at which a ONE-SAMPLE BINOMIAL test … **reaches that level and power**"*. A significance level is a threshold and the census minimum is a criterion. **A criterion in this document consumes two numbers in this document as thresholds**, which is item 1's whole point (*"THE SIGNIFICANCE LEVEL AND THE POWER ARE FIXED HERE"*).

**Limb 2 is false of two numerals within nine lines of it.** `:910` restates *"five hundred pairs"* from `configs/random_openings_v1.toml:47-52`, and `:946` restates *"the arc's own **fourteen** firings"* from the closed detector arc (D-537's own figure, `docs/decisions.md:1142`). Both are values restated here.

**This is the fourth consecutive revision whose self-claim about its own numerals is false** (rev 1 MINOR 2; rev 2 NEW-m1/NEW-m4; rev 3 N-m3). I raise it MAJOR rather than MINOR this time for one reason the earlier instances did not have: the previous claims were counts, and a false count misleads nobody. **This one is a compliance claim about D-483**, offered as *"narrower and checkable"*, and §0.1a's row (`:95`) records it as what now pins the finding. A false certificate of compliance is the "manufactured authority" the document itself names as *"the shape this arc keeps paying for"* (`:1204-1205`).

**There is no D-483 breach.** 0.05/0.95 are a convention adopted from a committed config, which item 1 argues correctly and which the previous two reviewers cleared; "fourteen" and "five hundred" are illustrative and neither is consumed. **What is wrong is the claim, not the numbers.**

**FIX.** Say what is true: *"The two numerals this rule consumes are the level and the power, and both are conventions adopted from a committed config rather than measurements. Every other figure §8 names — the arc's own denominators, the book's measured pair — is cited to the document that owns it and is not a value this rule reads."*

---

## MINOR

1. **R-m1** — the test table lists `two_symmetric_prefixes_share_a_key_seq_and_two_transpositions_do_not` **twice**, at `:1129` and `:1131`. The second is new this round (the `git diff` hunk `@@ -1053,0 +1131 @@` adds it). No row in §0.1a; harmless to an implementer, but it makes the register's own count wrong.
2. **R-m2** — §5's params paragraph is grammatically broken by the sentence inserted into it. `:775-778` reads *"…where it is a proof depth** (§2.5). Revision 3 sent that fourth one to a `note` block this same paragraph abolishes, so §2.5's obligation was discharged in no section: that `eval` is in pistol-eval's own integer units, that the sign is from the side to move at the root, and that `mate_in`/`mated_in` count both sides' turns (§2.3)."* The three `that …` clauses are the **values** of the three unit params and belonged to *"keyed params whose values a loader can check:"*; they now hang off a historical note. The values are still recoverable in order, so no reading is lost, but the paragraph that defines the header's params no longer parses.
3. **R-m3** — `:290-291` says *"every asked position is a turn boundary at `Phase::First` — which is **WP-2.0-M's INVARIANT 2**"*. At HEAD, WP-2.0-M's INVARIANT 2 is *"**No asked position is decided**, and the initial position is asked as bare `position start`"* (`wp20m_design.md:781-782`). The turn-boundary claim is **INVARIANT 1** (`:777-780`, *"The asked set is every turn boundary of every recorded game at which the engine can legally be asked"*). Same numbering in both revisions — I checked `df07a38:740-745`. New this round, and the previous reviewer's own FIX carried the same mis-numbering, which is how it got in.
4. **R-m4** — N-m5 partial: `:940` still reads *"Both figures are read from the trigger-rich **frame**, which is the **frame** the arc's own gap argument is about"*. In the cited document "frame" names PER-SEARCH vs AGGREGATE (`matrix_stage3_detector.md:607`), and the gap argument is explicitly frame-dependent there — *"In the PER-SEARCH frame … **there the gap is ZERO** and the barrier is arithmetic, not sample size at all"* (`:643-650`). **The pairing §8 uses is still the arc's own** (0.571 measured ordering against 0.857 column bound is the arc's own sentence at `:632-635`), so nothing about the rule is wrong; the word is.
5. **R-m5** — N-m6 partial: item 3 deletes *"the lower end"* and names it *"a comparison ACROSS bands [that] states the same rule a second, different way"* (`:931-933`), and then `:969-973` — untouched — argues the rule's bias from *"**The bound's lower end** is the SMALLEST effect the arc licenses … so **the lower end** yields the LARGEST minimum"*. Under the rule as now written the alternative is a single figure for one band and has no "lower end". **The number does not move** (trigger-rich's bound `0.857` is also the lower of the two bands' bounds), so the conservatism conclusion and the FLOOR rule survive; the section states the alternative's selection in a phrasing it has just repudiated.
6. **R-m6** — `:1252` says *"that is the **only** claim about the file this design makes"*, two paragraphs after `:1243-1245` asserts *"**THE FILE HOLDS TWO TABLES, UNDER TWO HEADINGS** … WP-2.0-M's rows are one per CAPTURE and carry its six fields"* — which is a claim about the whole file, WP-2.0-M's half included. (The six-field claim itself is **true**: `wp20m_design.md:1034-1036` lists exactly six.) New this round, inside the N-m10 remedy.
7. **R-m7** — `:1014-1015`: *"This table is the one place the refusals are **ENUMERATED**"*. §5 enumerates the loader's eleven refusals in full at `:788-796`, and §5 `:728` states a write-side TAB refusal that appears in no row of §9's table. The narrowing N-m7 asked for landed; the claim is still false of two enumerations in §5.
8. **R-m8** — the loader's new `key_seq` shape check has **no test** (only `key_full`'s has one, `:1133`), and **neither has a mutant**. §5's own ground for fixing the spellings is *"A LOADER CANNOT CHECK A TOKEN IT CANNOT PREDICT"* (`:732-733`); the check now exists and half of it is unguarded.
9. **R-m9** — `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` → INVARIANT 6 (`:1135`), a loader round-trip invariant, for a write-side refusal. New this round; detail in R-M3.
10. **R-m10** — `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` → INVARIANT 8 (`:1126`), which states the outcome relation, not §2.7's two-column decision. Carried unchanged from revision 1 through four rounds and never raised as a finding in its own right; recorded here so it stops travelling.
11. **R-m11** — the test name and the write-side mutant both say *"three unit params"* (`:1112`, `:1162`) against §5's four. Recorded separately from R-M1 so the fix is complete at every site.
12. **R-m13** — `a_rerun_over_one_capture_and_report_is_byte_identical`'s mutant *"the record order taken from a hash map rather than the capture"* (`:1181`) turns on whether two `HashMap`s in one process draw different `RandomState` keys. Carried unresolved from revision 2 through three rounds; not a defect in the design, but it is the one mutant whose death I still cannot argue from the text.

---

## The citation check, and AUTHOR DEBT

`python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` → **120 citations checked, 0 unreproduced.** Green before the review, per D-546's condition.

**The citation set changed by three.** A `comm` over both revisions' extracted `path:line` sets gives: **two added** — `crates/pistol-core/src/board.rs:30-36` and `crates/pistol-arena/tests/replay_chain_tests.rs:12-31` — and **one removed**, `crates/pistol-core/src/turn.rs:25-29`, which is N-m18's remedy.

**I hand-verified twenty-one citations' CONTENT — both new ones and nineteen load-bearing carried ones.**

**New in revision 4, both ✓:**

- `crates/pistol-core/src/board.rs:30-36` ✓ — `/// The protocol and fixture spelling of this player.` / `pub const fn name` / `Player::P1 => "p1"` / `Player::P2 => "p2"`. **`p1`/`p2` really is pistol-core's spelling**, and `crates/pistol-core/tests/board_tests.rs:124-126` pins it. The claim built on it (`:797-800`) is exact.
- `crates/pistol-arena/tests/replay_chain_tests.rs:12-31` ✓ — `stub_config`, `binary_a: STUB`, `binary_b: STUB`, `run(scratch, …)`. It is the stub-into-scratch harness the row says it is. **It is cited only in §0.1a** (R-M2).

**Carried and re-verified, all ✓:** `configs/arena_smoke_v0.toml:66-67` (`alpha = 0.05` / `beta = 0.05`); `configs/random_openings_v1.toml:47-52` (reproduces, half-read — N-m16); `crates/pistol-arena/src/exchange.rs:163-168` (D-80's *"a driver that billed compute to the wrong one would under-count every interrupted iteration"*, verbatim) and `:169-188` (the `?`-chain over `nodes`, `TIME_FIELD`, `depth_turns`) and `:76-79` (`compute.add`); `crates/pistol-arena/src/replay.rs:137-138`; `crates/pistol-arena/src/transcript.rs:39-40` and `:307`; `crates/pistol-arena/src/openings.rs:39`; `crates/pistol-cli/src/report.rs:55-61` (*"the one substring parser in the tree"*), `:82-84` (`… hashfull {} score {} pv`), `:145-158` (`score_token`'s three arms); `crates/pistol-cli/src/corpus/emit.rs:19-28` and `:51-58` (both render a bare `# …`, so §5's `note` argument is right) and `:40-44` (*"Never a `param`: a reader has to be able to tell a choice from a measurement"*); `crates/pistol-core/src/coord.rs:136-141`; `crates/pistol-core/src/state.rs:111-115`; `crates/pistol-core/src/symmetry.rs:143-155`, `:157-165`, `:213-218`; `crates/pistol-core/src/zobrist.rs:70-76`; `crates/pistol-search/src/info.rs:162-166` and `:259-261`; `crates/pistol-search/src/search.rs:513-514`.

**AUTHOR DEBT the checker could have caught: NONE.** Every `path:line` reproduces and every one I checked supports the claim built on it. **Four consecutive rounds with zero author debt.**

**The debt the checker cannot catch is all of Part 2 except R-m3.** Of this round's five MAJOR findings, four are contradictions between two sentences of this document or false rows in its own change table, and one is a false compliance claim. **None is a claim about the code the code does not make.** The failure class has not moved from revision 3; it has concentrated.

---

## Closure over the twelve invariants, forty-eight tests and thirty-five mutants

**(a) Invariants with no test**: **one, declared** — INVARIANT 2 (`:1076-1088`), with the reason (`the_transform_spawns_no_process_and_reads_no_clock` cannot fail) and the diff named as its evidence. Everything else maps. ✓ **Closed.**

**(b) Tests pinning nothing / mis-mapped**: **three** — `a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name` → 6 (R-M3, carried); `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` → 6 (R-m9, **new**); `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` → 8 (R-m10, carried four rounds). Revision 3's `to_move` mis-mapping is **closed** by INVARIANT 3's new clause.

**(c) Rules with neither test nor mutant**: **`depth_meaning`** (R-M1, **new — the round's own addition**); the loader's `key_seq` shape check (R-m8); the key-shape checks' mutants (R-m8); §5's write-side TAB refusal has a test now ✓ (revision 3's gap closed); `opening_turns` has a test now ✓; §9 row 4's three limbs have three tests now ✓; the manifest row ✓; `key_seq`'s value ✓; `to_move`'s value ✓.

**(d) Mutants that cannot die**: **none.** I checked all thirty-five individually. Revision 3's one (the TAB mutant across the seam) is properly split and each side dies at its own test. The two book mutants die at two different tests, verified against `replay.rs:137-138` arithmetic. *"`key_seq` computed from `canonical_form`"* dies at the transposition limb. *"records deduplicated by `key_full` on write"* dies because `key_full` folds transpositions and the test asserts two records. *"the three unit params dropped on write"* is honestly annotated with why the loader test survives and the round trip dies. The one I cannot settle by reading is the hash-ordering mutant (R-m13), unchanged from revision 2.

**(e) Tests that would pass vacuously**: **none.** But **one can FAIL on correct code** — `two_positions_differing_only_in_colour_do_not_share_a_key_full` on a symmetry-image pair (R-M4), with a worked counterexample. Revision 3's *"cannot pass on correct code"* case (the renamed book test) is **closed**.

**(f) Rules stated twice and differently** — the sweep the dispatch asked for, aimed at this round's three renames plus everything else:

| where | verdict |
|---|---|
| `turn` → `turns_played` completion | **clean** — `/usr/bin/grep -n "\bturn\b"` over test names returns none; `:1120` is the last one and it is renamed |
| `q,r:1` → `q,r:p1` | **clean** — `:582` (§2.10), `:747` (§5), `:792` (the loader) all agree, and `:801-802` states why `key_full`'s colour and `to_move`'s token set are one spelling |
| "frame" → "band" | **NOT clean — R-m4** (`:940` keeps two) |
| **§5's header params** | **NOT clean — R-M1** (three vs four, at five sites) |
| **§8's alternative** | **NOT clean — R-m5** (the bias paragraph's "lower end") |
| **§9's "one place"** | **NOT clean — R-m7** (§5's two enumerations) |
| **§12.1's "only claim"** | **NOT clean — R-m6** |
| INVARIANT 4's strength | **clean** — three sites, all agreeing (N-M4 verified) |
| the `forfeit_by` widening | **clean** — §2.8 and §4 agree (N-M3 verified) |
| §8's test | **clean** — one-sample in both prescriptive sites (N-B1 verified) |
| the book boundary | **clean** — prose, two tests and two mutants all agree |
| "the dispatch" | **clean** — three dispatches named at `:829-835`, and every bare *"the dispatch"* (`:208`, `:863`, `:891`) resolves to the WP-2.0 dispatch's own text at `wp20_dispatches.md:88`, `:102`, `:106`, which sit inside its section |

**(g) The false-universal sweep** (`every`, `no`, `none`, `only`, `always`, `never`, `all`). I stripped inline-code spans and inspected every surviving instance across 1329 lines. **Two are false**: `:914-915` (*"no number in this document is one a pre-registration, gate or criterion consumes as a threshold"* — R-M5) and `:1252` (*"the only claim about the file this design makes"* — R-m6). **One is imprecise but not false**: `:1014` (R-m7). Everything else holds, and I checked the ones with history individually — `:275-276` (*"a total position identity on every ongoing position, which every asked position is"*: sound, WP-2.0-M INVARIANT 2 gives the undecidedness); `:1196-1206` (the three-config counterexample: `LC_ALL=C /usr/bin/grep -rn "on_search_path" configs/ | LC_ALL=C sort` returns exactly those three `= true`, verified again); `:292-293` (*"parity and pistol-core agree on every legal prefix"*: sound); `:729` (*"no field is ever empty"*: sound given the `-` sentinel).

**(h) The D-483 numeral sweep.** Stripping inline code, D-numbers, `§`/WP/INVARIANT/revision/item/row/rule/gate indices leaves: section and column and refusal-row and invariant indices; three code facts (**twelve** symmetries, **128-bit**, **32 hex digits** / *"thirty-two"*, all pinned to `symmetry.rs`, `state.rs`, `zobrist.rs:70-76`); the `0.05`/`0.95` pair at `:906`; and **two measured quantities restated from measurement documents** — *"five hundred pairs"* (`:910`, from `configs/random_openings_v1.toml:47-52`) and *"fourteen firings"* (`:946`, D-537's own figure, `docs/decisions.md:1142`). **No measured number enters the rule**: the rule consumes only the level, the power, and two figures it cites rather than restates. **The D-483 position is sound; the document's claim about it is not** (R-M5).

---

# PART 3 — THE VERDICT

## **FAIL** — 0 BLOCKING · 5 MAJOR · 12 MINOR

Prior-round disposition: **1 of 1 BLOCKING applied cleanly**; **6 of 8 MAJOR applied, 1 partially, 1 with a new defect**; **10 of 19 MINOR applied, 2 partially, 2 with a new defect, 5 not applied.**

**Two things are worth recording as movement.** The BLOCKING is gone and did not come back in another form — §8's rule is now complete on all six axes and I could not find a seventh input it leaves open. And the count of defects that would make the package produce a wrong answer has fallen to **one** (R-M1, the header's param set), from three in revision 3 and five in revision 2.

**What has not moved is the instrument.** The guard table is the round's headline and it is false in seven of twenty-eight rows, five of them for work that was never done. **That is why this fails rather than passes**: the table is what a fifth round would be built from, and a fifth round built from this table would re-tick five findings that are still open.

## "Could an implementer build from this without deciding something the design should have decided?"

**NO.** They would have to decide:

1. **How many keyed params the corpus header carries** — three (§5's enumeration, the registered test's name, the write-side mutant) or four (§5's meanings paragraph and §5's loader). A writer built from the first produces a corpus the loader built from the second refuses (**R-M1**).
2. **What guards `depth_meaning`** — it is in no test and no mutant, and §2.5's stated consequence for getting it wrong is a consumer averaging two different quantities (**R-M1**).
3. **Which invariant states the loader's key-shape refusals, and whether the `key_seq` limb is checked at all** — §5 has the rule, INVARIANT 6 has neither, one limb has a test, neither has a mutant (**R-M3**, **R-m8**).
4. **Which fixture makes `two_positions_differing_only_in_colour_do_not_share_a_key_full` green** — a legal pair exists that makes it red on correct code, and the condition that excludes it is stated nowhere (**R-M4**).
5. **How `the_derived_outcome_agrees_with_the_reports_own_result_field` gets an arena-produced report without committing a match log** — the route is named only inside the change table (**R-M2**, N-m19's substance).

Everything else — the sixteen columns and their order, the three keys and exactly what each folds, the `q,r:p1` spelling and its non-collision, the `-` sentinel and why no field is empty, the parser split and the two-token score read, the node pair and its gate-off fallback, the all-six-or-none solver refusal, `depth_turns`'s two meanings, the outcome relation and its seat-blindness, the no-dedup and no-seed policies with the mutant honestly left without a site, the book boundary in prose and in both tests and both mutants, the twelve refusal rows, §8's complete rule and its `key_full` denominator, the manifest row and the two-table file, and §12.3's five leans — **is decided, and decided well.**

## The strongest attack that did not land

**I set out to break the `q,r:p1` re-spelling**, on the theory that lengthening the colour token under review pressure would reopen the collision NEW-B1's remedy had just closed — which would have made the third BLOCKING remedy in a row worse than the defect it fixed, in the column D-537's non-loosenable denominator counts over. **It held on every front, and the tree closed each one.**

- **The spelling is pistol-core's.** `Player::name` is `"p1"` / `"p2"` under the doc *"The protocol and fixture spelling of this player"* (`crates/pistol-core/src/board.rs:30-36`), and `crates/pistol-core/tests/board_tests.rs:124-126` pins both arms and `Display`. So the design did not invent a token; it adopted the one the protocol already writes, which is why `to_move` and `key_full` can share it.
- **The colon still collides with nothing, and the longer token changes nothing** — the separator does the work, not the token's length. `PAIR_SEPARATOR` is `'/'` (`turn.rs:107`), so `moves`, `key_seq` and `best` carry no colon; `Coord`'s `Display` is `write!(f, "{},{}", self.q, self.r)` with no colon and no space, negatives included (`coord.rs:136-141`); `score_token`'s three arms are `cp {v}`, `mate {t}`, `-mate {t}` (`report.rs:153-158`); `key_pos` is 32 hex digits; every remaining column is an integer or a fixed word token.
- **The rendering is injective, not merely non-colliding.** `transform` sorts and preserves colour (`symmetry.rs:148-155`), so `canonical_form` returns a totally ordered `Vec<(Coord, Player)>` whose space-joined `q,r:pN` spelling parses back element by element (`:165-176`).
- **And `to_move` and `key_full` now spell one concept one way**, which is the half of N-m4 that mattered — a loader can predict the token in both columns from one rule.

**A second attack failed.** I tried to show N-M1's rename had moved the boundary again — that the new name and its sibling still describe one position under some reading of §2.9's two glosses. They do not: `the_position_reached_by_the_whole_book_and_nothing_else` is `k = opening_turns` and `the_first_position_reached_by_an_engines_own_choice` is `k = opening_turns + 1`, the flags differ under `k <= opening_turns`, and each of the two mutants dies at exactly one of them. I re-derived the boundary from `replay.rs:137-138` without reading the document's derivation and got the same answer at both ends.

**A third failed.** I tried to show N-M7's claim was itself stale — that `ff1c575`, which took WP-2.0-M to revision 6 between the two revisions under review, had already corrected §8. It had not; the commit's hunk list touches nothing in that range, and `wp20m_design.md:735-736` and `:770-771` still describe WP-2.0-S as adding fields to `totals_of` and adding `score` and `pv` as `Option`s. **§12.3's new paragraph is true at HEAD, correctly quoted, and correctly assigned to WP-2.0-M as a prose-and-ADR correction rather than a change to §3.**

## What I could not settle by reading, and the run that would

1. **Whether `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` is constructible.** No corpus field the transform computes can carry a TAB on the shipped path — every one is a pistol-core rendering, an integer, or a fixed token, and a capture field with a TAB is refused first by §9 row 4. So the test must sit at a unit seam (a `write_record(fields)` that validates before writing), which the design does not name. **The run**: write the test in a `git worktree add --detach` and see whether it needs an entry point the design has not specified. **Refused by the dispatch's hard constraint.** I did not raise it as a finding because a validating write seam is the ordinary shape and the mutant dies there.
2. **Whether the `fields_of`/`totals_of` split compiles and leaves the SPRT report's per-game node counts intact.** Three identical expressions over the same `Vec<&str>` with elided lifetimes; I expect yes. **The run**: `cargo test --workspace --locked` plus gate 15. **Refused.**
3. **Whether `a_rerun_over_one_capture_and_report_is_byte_identical` kills its ordering mutant in one process.** Turns on whether two `HashMap`s in one thread draw different `RandomState` keys. **The run**: the mutant in a detached worktree with its own `CARGO_TARGET_DIR`. **Carried unresolved from revision 2.**
4. **Whether the one-sample rule yields a finite, sensible `n`** at `p0 = 0.571` against `p = 0.857`, level 0.05, power 0.95. **The run**: a `scipy`/`statsmodels` power calculation. I could not make it here, and **D-483 forbids the answer entering the design** — which is the design's own position and is correct. After N-B1 it owes exactly one rule and it has exactly one.
5. **Whether gate 17's soft cap holds once the fourth arm and the loader land.** `wc -l crates/pistol-arena/src/bin/arena.rs` is 283 against `SOFT_CAP=300`, and WP-2.0-M extracts `USAGE` first. **The run**: `tools/file_justification_check.sh` at the post-implementation revision. Not run.

---

## One paragraph for the operator

**This document is one small round from done, and the round is entirely deletion and one-clause correction — but it must be a round whose table is written from the diff.** The BLOCKING is closed and stayed closed; both of the arc's hardest decisions (the `key_full` spelling and the book boundary) survived a fresh attack from four directions each; the citation set has grown to 120 with zero author debt for the fourth consecutive round; and exactly **one** finding in this report is a way the package produces a wrong answer — §5 says the corpus header carries three keyed params in one sentence and four in the next, which would have a run refuse its own output. **What failed is the guard table, again, and differently**: revision 3's twelve rows were incomplete and two were false about the text; revision 4's twenty-eight rows are complete and **five of them describe changes that were never made**, because they were written from the review's FIX list rather than from `git diff`. That is why five MINORs from revision 3 are still standing, and one of them — the colour test's fixture condition — is a registered test I can make red on correct code with a five-stone pair I built from `symmetry.rs`. **The fix is: apply the five that were never applied, complete `depth_meaning` at the two sites §5 missed and at the test name, widen INVARIANT 6, state one clause beside the colour test, and rewrite §8's numeral self-claim into one that is true.** None of them touches a decision. Two grant rounds remain under D-548, and I would expect the next one to pass if its first act is to re-read each row of §0.1a against the file it names rather than against the finding it answers.
