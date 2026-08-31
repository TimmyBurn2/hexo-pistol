# SCOPED RE-REVIEW — `docs/experiments/wp20s_design.md` revision 3

## Header

- **Revision adjudicated**: `df07a38d6a094d652c5587b3e69a0f7a9b9eb50c` (`docs(wp20s): the round that adds the guard its sibling had …`).
- **Matches HEAD**: **YES**. `git rev-parse HEAD` = `df07a38d6a094d652c5587b3e69a0f7a9b9eb50c`, branch `dev`.
- **Tree state**: **clean at the start** — `git status --porcelain` printed nothing. **At the end it is not**, and none of it is mine: `docs/experiments/wp20m_design.md` is modified and `docs/experiments/wp20m_design_REVIEW_rev5.md` is untracked, both written by the concurrent WP-2.0-M session. **The file under adjudication, `docs/experiments/wp20s_design.md`, is unmodified and identical to `df07a38` throughout** (it appears in no `git status` line). Every claim below about `wp20m_design.md` was read at the point recorded here and is cited to HEAD (`df07a38`) — a reader re-checking those line numbers after the concurrent session commits should re-read by section heading, not by line. **The only file I wrote is this one.**
- **What I ran**: `git` (`log`, `rev-parse`, `status`, `show`, `diff`, `diff --stat`, `grep`, `ls-files`), `/usr/bin/grep`, `sed`, `awk`, `wc`, `cat`, `cat -A`, `cut`, `head`, `ls`, `LC_ALL=C sort`, and `python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` — **green, 119 citations checked, 0 unreproduced** (revision 2: 104; revision 1: 60).
- **What I refused to run**, per the dispatch's hard constraint: `cargo` in any form, `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Runs I could not make are named in Part 3.
- **Binding reading, all in full**: `docs/experiments/wp20s_design_REVIEW_rev2.md` (NEW-B1, NEW-B2, NEW-M1…M10, NEW-m1…m18, and its Part 1 dispositions); `git show f96593b:docs/experiments/wp20s_design.md`; the complete `git diff f96593b df07a38` (24 hunks in this file, plus `docs/decisions.md` and `docs/experiments/wp20_dispatches.md`); `CLAUDE.md`; `docs/process.md`; `docs/decisions.md` D-483, D-518, D-523, D-531, D-537, D-539…D-548; `docs/experiments/wp20_dispatches.md`; `docs/experiments/wp20m_design.md` **revision 5** (§0.2, §0.2a, §0.2b, §8, §11, §12, §13); `docs/experiments/matrix_stage3_detector.md` §5.8 and its closing tables.
- **Code read**: `crates/pistol-arena/src/{replay,openings,transcript,exchange,lib,record,conclusion,game,dedupe}.rs`, `crates/pistol-arena/src/bin/arena.rs`, `crates/pistol-arena/tests/replay_chain_tests.rs`, `crates/pistol-cli/src/{report,protocol}.rs`, `crates/pistol-cli/src/corpus/emit.rs`, `crates/pistol-core/src/{state,coord,turn,symmetry,board,rules}.rs`, `crates/pistol-search/src/{info,search,pvs,score}.rs`, `configs/` (all `on_search_path`, all `alpha`/`beta`).

---

## VERDICT: **FAIL**

**1 BLOCKING · 8 MAJOR · 19 MINOR.**

**Revision 3's work on the thirty findings is the best of the arc.** Twenty-six of the thirty are fully applied and verified against the tree; two are partially applied; two applied with a new defect. Both BLOCKING remedies are substantively right — I attacked the `q,r:1` spelling four ways and could not break it, and I re-derived the `k <= opening_turns` boundary independently from `replay.rs:137-138` and it is correct. The citation checker is green over a larger citation set than any previous revision, and **every citation new in revision 3 that I hand-checked reproduces AND supports the claim built on it.** The false universal about configs is gone and its three counterexamples are named and verified. The false claim about WP-2.0-M is gone and the pointer that replaced it is accurate: WP-2.0-M §11 does carry the throughput shape at HEAD (`wp20m_design.md:926-935`).

**It fails because §0.1a — the guard this round exists to add — is both incomplete and, in two rows, wrong about the tree it describes.**

- **It is not complete.** It carries twelve rows against thirty findings and twenty-four hunks. The remedies it omits include the one that broke a test: **NEW-B2's boundary change renamed a second test to `the_first_position_an_engine_chose_from_is_flagged_not_book`, which under §2.9's own new rule names the position the rule flags `book`.** The old name was correct under the old rule; the new name is false under the new one. That is a remedy spending a true thing while applying a finding — D-548's named class — wearing this round's own guard and not listed on it.
- **Two of its right-hand columns are false.** NEW-M3's row says the byte-identity claim is now "per-game node counts, **in both places**" — there are three places, and `:653` still says *"INVARIANT 4's byte-identity claim"*. NEW-B1's row says the spelling is pinned by *"the loader's shape check"* — §5's loader enumeration and INVARIANT 6 contain no such check; the refusal exists only in a test name.
- **And a third remedy left its old site standing**: §0.1a's NEW-m14 row says *"§4 now widens with `result` alone"*, while §2.8 at `:443` still says *"§4 widens the transcript reader to carry it, so it is available."*

**The BLOCKING is separate and is in §8.** Item 5 fixes NEW-M7's second limb — *"THE TEST IS ONE-SAMPLE AND NOT TWO"* — and eight lines later the operative sentence still reads *"a **two-proportion** test at that level and power separates those two recalls."* Item 5 itself says the two-proportion form is *"unsatisfiable at any power worth registering."* So D-537's non-loosenable rule prescribes, in its own summary, a test the same section declares cannot be satisfied, and a successor chooses.

---

# PART 1 — DISPOSITION OF EVERY REVISION-2 FINDING

## BLOCKING

### NEW-B1 — `key_full`'s spelling dropped `Player` — **APPLIED**, and I attacked the replacement hard

`:690-701` renders each element `q,r:1` or `q,r:2`, space-joined in canonical order, and states the reason. `:537` restates it in §2.10, `:677-678` counts three space-bearing columns, `:682` says three spellings, `:179` puts *"colour spelled"* on the schema table's face.

**I tried four ways to break it and could not.**

1. **Injectivity.** `canonical_form` returns `Vec<(Coord, Player)>` sorted by `transform` (`crates/pistol-core/src/symmetry.rs:143-155`, verified: `.map(|&(cell, player)| (symmetry.apply(cell), player))` then `image.sort_unstable()`), and its return type is on `:165`. `Coord`'s `Display` is `write!(f, "{},{}", self.q, self.r)` (`crates/pistol-core/src/coord.rs:136-141`). A colour tag drawn from a two-token set appended after a separator that appears in neither half makes the render a bijection onto its image. **Injective on the key's own value space.** ✓
2. **Colon collision, every column.** `PAIR_SEPARATOR` is `'/'` (`crates/pistol-core/src/turn.rs:107`), so `moves`, `key_seq` and `best` carry `q,r` and `q,r/q,r` and no colon. `key_pos` is 32 hex digits. `score_kind`/`result`/`end`/`book`/`to_move` are fixed word tokens. `score_value`, `depth_turns` and the two node columns are integers. `score_token`'s three expansions are `cp {value}`, `mate {turns}`, `-mate {turns}` (`crates/pistol-cli/src/report.rs:153-158`) — no colon. **`:699-701` is true as stated.** ✓
3. **Does the loader check it?** **NO — see N-M2.** This is the one limb of §0.1a's row that does not hold.
4. **Does §2.10's sentinel argument still hold over the new spelling?** **YES.** `:535-537` restates it over all three columns and the argument is stronger, not weaker: every token any of the three can hold contains a comma (`coord.rs:136-141`, D-5), and `-` is one character with no comma. The `key_full` element token now contains a comma AND a colon. ✓

### NEW-B2 — the `book` boundary contradicted the column's own definition — **APPLIED BUT INTRODUCED A NEW DEFECT** (N-M1)

**The arithmetic is right, and I re-derived it rather than reading the document's.** `crates/pistol-arena/src/replay.rs:137-138` is `for (at, recorded) in game.moves.iter().enumerate() {` / `if at >= opening_turns as usize {`, so turn indices `0 … opening_turns-1` are the book's. A prefix of length `k` is reached by turn indices `0 … k-1`; all are book turns iff `k-1 <= opening_turns-1` iff **`k <= opening_turns`**. The document's own derivation (`:499-501`, via `k-1 < opening_turns`) is the same statement over integers. ✓ It is also right at the ends: `k = 0` is vacuously all-book and is flagged `book`, which the column's definition (*"neither engine chose this position"*) requires.

**`opening_turns` is reachable to put in the header.** It is a field of `Transcript` (`crates/pistol-arena/src/transcript.rs:39-40`, *"How many turns of every game are book"*) and is **parsed by `transcript::read`** — `let opening_turns: u32 = number(one_line(text, "opening_turns")?.trim(), "turn count")?;` at `transcript.rs:173`, assigned at `:202`. Uniformity is real: `openings.rs:39` is *"How many turns every opening has"* and `uniform_turn_count` enforces it. ✓ No widening of `RecordedGame` is needed for it. ✓

**The inverted mutant dies at its named test.** `:1095` registers *"the book boundary read as `k < opening_turns`"* against `the_position_reached_by_the_whole_book_and_nothing_else_is_flagged_book`. At `k = opening_turns` the mutation yields `no` and the test asserting `book` fails. ✓

**But the SECOND test was renamed and the rename is wrong** — **N-M1**.

## MAJOR

| # | finding | disposition | evidence |
|---|---|---|---|
| M1 | the turn-zero sentinel reversed at two sites of four | **APPLIED** | `:176` *"`-` when `turns_played` is zero"*, `:177` and `:179` add the same to rows 4 and 6, `:948` §9 row 4 is *"**any field is empty** (§2.10 leaves none)"*. All four sites now agree with `:534` and `:680` |
| M2 | §12.2's false claim about WP-2.0-M, and the requirement discharged twice | **APPLIED** | `:1174-1180` deletes the claim, names it false, and points. **Verified at HEAD**: `wp20m_design.md:926-935` (§11) carries *"AND THE DISPATCH'S THROUGHPUT OBLIGATION IS THE PILOT'S … one label `go` at the label budget per asked position, plus one `newgame` per asked position"*. The pointer is accurate and the restatement is gone |
| M3 | §3 said INVARIANT 4 pins byte-identity | **PARTIALLY APPLIED** | `:600-602` now says *"what a same-build test can show … and **not** byte-identity"*, matching INVARIANT 4 at `:976-978`. **`:653` is untouched and still says *"INVARIANT 4's byte-identity claim"*** — **N-M4** |
| M4 | the `to_move` mutant could not die; the column had no test | **APPLIED** | `:256-263` drops the mutant and says why plainly — *"A column whose two possible implementations agree everywhere has no behavioural mutant"* — which is the honest answer the finding's FIX named first. `to_move_is_the_side_pistol_core_puts_to_move_at_that_prefix` is registered (`:1051`) with the mutant *"`to_move` written as the opposite side"* (`:1084`). **Dropping it is right** (see below) and **the test can fail** (see below) |
| M5 | `key_seq` had no test and no mutant | **APPLIED** | `two_symmetric_prefixes_share_a_key_seq_and_two_transpositions_do_not` (`:1052`, INVARIANT 9) with *"`key_seq` computed from `canonical_form` instead of `canonical_sequence`"* (`:1083`). Under that mutation two transpositions of one stone set share the key and the test's second limb fails — **the mutant dies** ✓ |
| M6 | §8 named its bias backwards | **APPLIED**, and the arithmetic is right | see below |
| M7 | no frame named; two-proportion under-specified | **PARTIALLY APPLIED** | the frame is named (`:864-872`) and the test is named one-sample (`:873-878`); **`:880-882` still says two-proportion** — **N-B1** |
| M8 | *"and only on them"* used outside its scope | **APPLIED** | `:222-234` deletes the hedge, grounds the property on *"an **ongoing game**, not a turn boundary"* and cites `state.rs:128-133`. Verified verbatim. `:913` uses the corrected scope |
| M9 | false universal about committed configs | **APPLIED**, all three verified | `:1112-1123`. `LC_ALL=C /usr/bin/grep -rn "on_search_path" configs/ \| LC_ALL=C sort` returns exactly three `= true`: `bench_wp18c_solver_on.toml:45`, `gate_staged_solver_v0.toml:47`, `play_staged_solver_v0.toml:75`. All three cited lines read `on_search_path = true`; `gate_v0.toml:94` reads `false`. **The universal is deleted rather than narrowed, and the conclusion is re-grounded** |
| M10 | the manifest row had no invariant, no table entry, no mutant, no file shape | **APPLIED** | INVARIANT 11 (`:993-995`), the test in the table (`:1058`), the mutant (`:1085`), and the two-table file shape (`:1159-1167`) |

**M4, the two questions the dispatch asked.**

**Is dropping the mutant right?** **YES.** The design's ground is exact: §9 row 5 (`:949`) refuses a record whose `turns_played` and `moves` disagree *before any column is written*, so the only input on which "replay" and "read the `turns_played` column" differ never reaches the writer. That is the same no-op property that killed the parity mutant, and revision 2's replacement had it too. Registering a third mutant of the same shape would have been the third instance of D-544's class; naming the absence is the correct move and it is exactly what MAJOR 8's own FIX offered as *"the honest answer"*.

**Can `to_move_is_the_side_pistol_core_puts_to_move_at_that_prefix` actually fail?** **YES.** The expected value is *"computed by replaying through pistol-core rather than written by hand"* (`:266-268`), so the test compares the corpus column against an independently computed side. Under the registered mutation (*"written as the opposite side"*) the column differs and the assertion fails. The design also names why the round-trip test could not do this job — *"a round trip is self-consistent under a wrong value"* (`:268-269`) — which is correct. ✓ One mis-mapping remains (**N-m12**).

**M6, verified arithmetically.** The trigger-rich band's incumbent is **0.571** and its column bound is **0.857**; the other band's are **0.333** and **1.000** (`matrix_stage3_detector.md:611-614`). Against a fixed null, the required `n` falls as the null-to-alternative gap grows. Gap at the bound's lower end = `0.857 − 0.571 = 0.286`; at its upper end = `1.000 − 0.571 = 0.429`. **Smaller gap ⇒ larger `n`.** So the lower end is the smallest effect and yields the **largest** minimum, and `:896-899` — *"the lower end is the SMALLEST effect the arc licenses … so the lower end yields the LARGEST minimum. The rule is conservative by construction"* — **is correct**, and revision 2's inversion is gone. The floor rule survives with the right justification (`:899-903`). ✓

**M7's frame, and the figures.** `matrix_stage3_detector.md:611-614` carries the table the document points at, and both figures exist there: trigger-rich `0.571` (per-search ceiling / best written ordering, restated at `:643-645`) and `0.857` (bound over the columns, named at `:627-630` as *"what a score fitted with full knowledge of which column-classes hold wins could reach"*). ✓ The figures are where the document says they are. Two wording defects remain (**N-m5**, **N-m6**) and the one-sample limb is contradicted (**N-B1**).

**M10, does the two-table shape collide with WP-2.0-M's own definition?** **No collision, one unsourced claim.** WP-2.0-M §13(a) at HEAD (`wp20m_design.md:998-1006`) defines `docs/label_corpus_manifest.md` as holding *"one row per capture: the `capture_sha256`, the body digest, the source report's `experiment_sha256` and `source_sha256`, the label `go` line, and the artifact's path"* — six fields, printed not written, added in the commit that records the run. WP-2.0-S's row is six different fields, one per corpus, under its own heading, each naming the `capture_sha256` of a capture row (`:1147-1167`). The two coexist and the cross-reference direction is stated. **What is not sourced** is `:1166-1167`'s *"WP-2.0-M creates it with the capture table"* — WP-2.0-M §13 says nothing about who creates the file or about headings (**N-m10**). And INVARIANT 11 names five of the six fields (**N-m9**).

## MINOR

| # | finding | disposition | evidence |
|---|---|---|---|
| m1 | *"Two of these columns carry spaces"* | **APPLIED** | `:677-678` — *"Three … `moves`, `key_seq` and `key_full`"* |
| m2 | *"TWO COLUMNS' SPELLINGS"* | **APPLIED** | `:682` — *"THREE COLUMNS' SPELLINGS"*, with three bullets |
| m3 | *"A field map adds no lookup"* | **APPLIED** | `:597` — *"A word list adds no lookup"* |
| m4 | *"the only two numerals this document carries"* | **APPLIED BUT INTRODUCED A NEW DEFECT** | narrowed to *"of the kind D-483 governs"* (`:849`), and falsified again by this round's own additions — **N-m3** |
| m5 | *"WP-2.0-M REVISION 3"* | **APPLIED** | `:42` — *"WP-2.0-M's CURRENT REVISION"*, which does not go stale |
| m6 | `capture_sha256` filed as `param` | **APPLIED** | `:707-710` moves it to `derived` and quotes `emit.rs:40-44` verbatim (verified: *"A value the extraction computed. Never a `param`: a reader has to be able to tell a choice from a measurement."*) |
| m7 | the arena-version reason mis-attributed | **APPLIED** | `:711-717` claims the reason as this document's own and distinguishes WP-2.0-M's digest argument from its header |
| m8 | *"a linear pass"* | **APPLIED** | `:1183-1186` — *"quadratic in game length and linear in games"*. Correct: `Σ k` over a game's prefixes is `O(len²)` |
| m9 | the disjointness reason argued for `key_pos` | **APPLIED** | `:907-913` gives a reason for each fold separately and names which key supplies which |
| m10 | the memset claim with no line | **MOOT** | the sentence left with §12.2's rewrite |
| m11 | `turn` invites a unit confusion with `depth_turns` | **APPLIED** | renamed `turns_played` (`:175`) with a paragraph (`:544-549`). `GameState::turn()` at prefix `k` really is `k+1`: `FIRST_TURN = 1` (`crates/pistol-core/src/rules.rs:18`), set at `state.rs:63`. One leftover — **N-m2** |
| m12 | §9 rows 2, 4, 11 and §5's write-side TAB unpinned | **PARTIALLY APPLIED** | three tests added (`:1055-1057`) covering row 2, row 4's TAB limb and row 11. **Row 4's field-count and empty-field limbs still have none** (**N-m13**), and **§5's write-side TAB refusal still has no test — its new mutant is paired with a read-side one** (**N-M5**) |
| m13 | the unit-param mutant mis-paired | **APPLIED**, correctly | `:1080-1081` splits into two rows and says which test dies for which side |
| m14 | `forfeit_by` widened for nothing | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:641-646` drops it. **§2.8 at `:443` still says §4 carries it** — **N-M3** |
| m15 | the dispatches preamble did not name every difference | **APPLIED** | `wp20_dispatches.md:14-24` now names the dropped census mutant and the added cold-label mutant |
| m16 | *"the same error rates"* | **APPLIED** | `:843-849` re-grounds it on the convention and disclaims delivery. One half-read citation — **N-m16** |
| m17 | no invariant states how `score_value` is read | **APPLIED** | INVARIANT 12 (`:996-997`), with two tests remapped to it (`:1021`, `:1028`). INVARIANT 3 and 4 keep enough tests |
| m18 | the gate-off fallback cited only a doc-comment | **APPLIED** | `:338-340` adds `crates/pistol-search/src/search.rs:513-514`. One imprecision — **N-m15** |

---

# PART 2 — THE §0.1a AUDIT, AND NEW DEFECTS

## The §0.1a audit

**Is the table true?** Ten of its twelve rows are. **Two are false about the tree they describe:**

- **NEW-M3's row** — *"per-game node counts, **in both places**"*. There are three places. `:653` still reads *"INVARIANT 4's byte-identity claim is untouched by it"* (**N-M4**).
- **NEW-B1's row** — *"§5's spelling rule, **the loader's shape check**, and `two_positions_differing_only_in_colour_do_not_share_a_key_full`"*. §5's loader enumeration (`:731-740`) lists nine refusals and none is about `key_full`'s shape; INVARIANT 6 (`:981-983`) enumerates *"schema version, body digest, record arity, header params or token set"* and none of those is it (**N-M2**).

And **NEW-m14's row** — *"§4 now widens with `result` alone"* — is true of §4 and false of §2.8, which still says the opposite (**N-M3**).

**Is the table complete?** **No.** It carries **twelve rows** against **thirty findings** and **twenty-four hunks** in this file. Every MINOR remedy except m14 is off it — including m11, which renamed a column across the whole document, and m12, which added four tests and three mutants. **That would be tolerable if nothing was spent. Something was.**

**The one change that spent a true thing and is not on the table** is inside NEW-B2's own remedy. Revision 2 registered two book tests: `a_book_position_is_a_record_flagged_book` and `the_first_position_after_the_book_is_flagged_not_book`. Revision 3 renamed the second to `the_first_position_an_engine_chose_from_is_flagged_not_book` (`:1061`). §0.1a's NEW-B2 row names only the first rename and the inverted mutant. **The second rename is where the round lost something** — see N-M1. That is the third consecutive round in which a remedy spent a true thing, and the first in which it happened underneath the guard built to stop it.

---

## BLOCKING

### N-B1 — §8 states its own test two ways, and the operative sentence names the test §8 itself calls unsatisfiable

`:873-878`, new in revision 3:

> 5. **THE TEST IS ONE-SAMPLE AND NOT TWO.** The incumbent enters as a fixed `p0` taken from the closed arc, not as a second arm with its own `n` — revision 2 said *"two-proportion"*, which needs two arm sizes and, with the arc's own fourteen firings as the second, **is unsatisfiable at any power worth registering**. **The minimum is the smallest `n` at which a one-sample binomial test of `p0` against the alternative reaches the registered level and power.**

`:880-882`, eight lines later and **unchanged from revision 2**:

> **The minimum is then the smallest number of win-proving firings on disjoint positions at which a **two-proportion** test at that level and power separates those two recalls.** Nothing in it is a choice a successor makes after seeing data.

**Both sentences define "the minimum". They define it differently, and the second is the one the section closes on.** The `:880` sentence is the paragraph a successor lifts — it is the sentence that carries D-537's own denominator language (*"win-proving firings on disjoint positions"*), it sits immediately before *"Nothing in it is a choice a successor makes after seeing data"*, and it is what §12.3's residue 3 and §8's WP-2.0b constraint hang off.

**Why this is BLOCKING and not a wording slip.**

1. **The two rules produce different numbers.** A one-sample binomial test of `p0 = 0.571` against `p = 0.857` at level 0.05 and power 0.95 has one answer. A two-proportion test at the same level and power has another, and needs a second arm size the document does not give.
2. **The document declares its own operative sentence unsatisfiable.** Item 5's ground for preferring one-sample is that the two-proportion form, with the arc's registered `n` as the second arm, *"is unsatisfiable at any power worth registering"*. So `:880-882` prescribes a computation §8 says cannot be done.
3. **D-537 forbids exactly the choice this leaves open.** `:882` says *"Nothing in it is a choice a successor makes after seeing data"* — but which of the two tests to run is now a choice, and the document offers no rule for making it. This is the section the dispatch required to land now *"so it cannot be fitted later"*, and it is the one rule D-537 says a successor **may not loosen**.
4. **The guard table asserts this is closed.** `:71` says what pins it now is *"the frame, the fixed `p0`, and the floor rule stated the right way round"*. The fixed `p0` is the limb `:881` contradicts.

**FIX, one sentence.** Delete or rewrite `:880-882` so it states item 5's rule: *"The minimum is then the smallest number of win-proving firings on disjoint positions at which a one-sample binomial test of the incumbent `p0` against the alternative reaches that level and power."* Nothing else in §8 changes.

---

## MAJOR

### N-M1 — the second book test's rename makes it assert the negation of §2.9's new rule, and the boundary's other direction loses its only guard

INVARIANT 10's three tests (`:1059-1061`):

| test | what it asserts |
|---|---|
| `a_book_position_is_a_record_flagged_book` | some book position is flagged `book` |
| `the_position_reached_by_the_whole_book_and_nothing_else_is_flagged_book` | **`k = opening_turns` is flagged `book`** |
| `the_first_position_an_engine_chose_from_is_flagged_not_book` | **`k = opening_turns` is flagged `not book`** |

**They name the same position.** §2.9 says the arena asks an engine at turn index `at` only when `at >= opening_turns` (`:494-496`), so the first turn index an engine is asked at is `opening_turns`, and **the position it is asked FROM is the position at `turns_played = opening_turns`** — which is precisely *"the position reached by playing the whole book and nothing else"* (`:504-505`). §2.9's own gloss confirms the reading: `:511-512` calls the ask-boundary reading *"was the move played from here chosen by an engine"*, `k >= opening_turns`. **At `k = opening_turns` the move played from here WAS chosen by an engine, and the `book` column says `yes`.**

So the two tests cannot both be green. The rule in prose is unambiguous — `k <= opening_turns`, derived correctly — and the register contradicts it.

**Revision 2's name was correct under revision 2's rule** (`the_first_position_after_the_book_is_flagged_not_book`, where "after the book" was `k = opening_turns`). Revision 3 moved the boundary by one and renamed the test to a phrase that moved it back. Under the new rule the first `not book` position is the one reached **after** an engine's first choice, `k = opening_turns + 1`.

**And the loss is not only in the name.** The two-test pair existed to pin the boundary from both sides: one test kills a boundary that is too tight, the other a boundary that is too loose. With the second test naming the wrong position, **no registered test kills a mutation reading the boundary as `k <= opening_turns + 1`** — the direction that would mislabel one engine-chosen position per game as `book`, which is the direction that corrupts a trainer weighting by provenance (`:519-521`).

**Not BLOCKING, and I want to be exact about why**: the RULE is stated once and correctly in prose (`:499-507`), so an implementer who reads §2.9 first gets the right boundary and would hit a red test and fix the name. What they cannot recover from the document is which position the third test is supposed to be about, and the missing mutant coverage is real.

**FIX.** Rename to `the_first_position_reached_by_an_engines_own_choice_is_flagged_not_book` (or `..._after_an_engine_chose_...`), and add the mutant it guards: *"the book boundary read as `k <= opening_turns + 1`"*.

### N-M2 — the loader check §0.1a names as pinning `key_full`'s spelling is in no rule of the document

`:64` claims the spelling is pinned by *"§5's spelling rule, **the loader's shape check**, and `two_positions_differing_only_in_colour_do_not_share_a_key_full`"*.

**§5's loader is enumerated in full at `:731-740`** and lists: a schema version it does not write; a bad body digest; a wrong TAB count; **any field that is empty**; a `score_kind` outside the three; a negative `score_value` under either mate kind; a `to_move`, `result`, `end` or `book` outside its token set; **a `key_pos` that is not thirty-two hex digits**; a header missing any param; a number spelled a way this format does not write. **No `key_full` shape check.** It is not in INVARIANT 6's enumeration either (`:981-983`), and §9 — which says at `:940` that it is *"the one place they are stated"* — has no row for it.

The refusal exists in exactly one place in the document: the test name `a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name` (`:1054`). **An implementer building the loader from §5 does not write that check, and the registered test then fails.** An implementer building it from the test list writes a refusal no rule specifies and no invariant covers.

This matters more than a missing bullet because §5's whole premise is that *"A LOADER CANNOT CHECK A TOKEN IT CANNOT PREDICT"* (`:682`) — the section fixes `key_full`'s spelling **so that** the loader can check it, and then does not have it check it.

**FIX.** Add to `:738` — *"a `key_full` field whose elements are not `q,r:1` / `q,r:2` pairs"* — and widen INVARIANT 6's enumeration to name the key columns' shapes.

### N-M3 — §2.8 says §4 widens the reader to carry `forfeit_by`; §4 says it does not

`:443-444`:

> **`forfeit_by` — recoverable and not useful.** §4 widens the transcript reader to carry it, **so it is available**; it is not a column because pass 1 is a self-match between two seats of ONE engine …

`:641-646`:

> **SO `RecordedGame` GAINS `result`, AND ONLY `result`** … Revision 2 widened it with `forfeit_by` too, and §2.8 then explained that nothing reads it — a second fatal lookup added for a field no column takes. **It is dropped**: `forfeit` already comes off the `end` field (`crates/pistol-arena/src/transcript.rs:307`) …

**§4's decision is right and the tree backs it**: `transcript.rs:307` is `forfeit: value(&fields, "end", record)? == "forfeit",` ✓, and `read_games` never reads `forfeit_by` (`:300-313`). **§2.8 was not updated**, so the document says in one section that the field is available and in another that it was deliberately removed. §2.8's reason for excluding the column now rests on an availability the design withdrew.

This is the reversal-at-some-sites class, on the change **§0.1a lists as fully closed** (`:75`, *"nothing is owed"*). It is the third consecutive round with an instance, and the first where the contradicted governing document is this one.

**FIX.** `:443` → *"`forfeit_by` — not recovered, and not needed. §4 does not widen the reader for it (and says why); it is not a column because …"*.

### N-M4 — the third site of INVARIANT 4's byte-identity claim is untouched, and §0.1a says there were two

`:653`:

> **It is not on the SPRT path**: nothing the generation path runs reads a report … so **INVARIANT 4's byte-identity claim** is untouched by it.

INVARIANT 4 (`:976-978`) claims no such thing: *"`totals_of`'s three lookups stay load-bearing, and the SPRT path still bills each game's compute from the totals line."* §3 was corrected (`:600-602`) with the correction explicitly narrated. §4 was not.

`/usr/bin/grep -n "byte-identity\|byte-identical"` returns five lines; `:653` is the only one that mis-describes INVARIANT 4. **§0.1a's row says the fix landed *"in both places"*** — a claim about this document's own record that the document falsifies.

**FIX.** `:653` → *"so INVARIANT 4 is untouched by it"*, or name what it actually is untouched by (the SPRT path's node accounting).

### N-M5 — a mutant registered this round cannot die at its named test: it is on the write side and the test is on the read side

`:1087`: *"| the **write-side** TAB check removed | `a_capture_record_with_a_tab_in_a_field_is_refused_by_name` |"*

These are two different checks on two different files.

- **The write-side TAB refusal** is §5's, over the corpus this transform WRITES: *"A field carrying a TAB refuses the run by name"* (`:679`). Its fields are computed here — `moves`, the three keys, the score columns.
- **The named test** is about a **capture** record — the file this transform READS — and is §9 row 4's refusal (`:948`), which fires *"naming the record"*.

Removing the write-side check leaves §9 row 4's read-side refusal intact, so `a_capture_record_with_a_tab_in_a_field_is_refused_by_name` **still passes under the mutation**. Revision 2's NEW-m12 named these as two distinct unpinned rules; the round added a test for one and a mutant for the other and paired them across the seam.

**Two consequences.** The mutant cannot die — the same defect NEW-M4 raised, in a mutant added by the round that fixed NEW-M4. And **§5's write-side TAB refusal still has no test at all**, which was half of what m12 asked for.

**FIX.** Two rows: *"the write-side TAB check removed"* → a new `a_corpus_field_carrying_a_tab_refuses_the_run_by_name`; *"the capture-side TAB refusal removed"* → `a_capture_record_with_a_tab_in_a_field_is_refused_by_name`.

### N-M6 — §2.5 sends the `depth_turns` discriminator to a header `note` block §5 abolished, and §5's header has no entry for it

`:361-364`:

> **So the column's meaning is: a completed search depth, except where `search_nodes` is zero, where it is the proof's depth in turns** — and **§5's header `note` block says so** beside the score's three properties, because a consumer reading the column without the discriminator would average two different quantities.

§5 says the opposite twice. `:719-724`: *"**AND THE UNITS GO IN AS `param` LINES, NOT `note` LINES.** `Fixture::note` renders a bare `# <text>` indistinguishable from the title lines above it … so a machine reader cannot find them. The three properties a column name cannot carry — `score_units`, `score_sign` and `mate_counts` — are therefore **keyed** params."* And §5's header enumeration (`:703-717`) lists every `param` and every `derived` value: **there is no depth-meaning entry of either kind.**

Verified against the tree: `Fixture::note` writes `# {line}` and `Fixture::param` writes `# param {name} {value}` (`crates/pistol-cli/src/corpus/emit.rs:36-58`) — so §5's argument is right and §2.5's mechanism is the one §5 rejects.

**So the obligation §2.5 states — that the corpus tells a consumer how to read `depth_turns` — is discharged in no section.** An implementer must decide whether to emit a fourth keyed param (and name it), a `note` §5 forbids, or nothing. The consequence §2.5 names for getting it wrong is that a consumer *"would average two different quantities"*, which is a wrong answer in the corpus's own terms.

Carried from revision 2 (`f96593b:316`) and not raised there; it is live at revision 3 and was not swept when the header was rewritten this round.

**FIX.** Add a fourth keyed param to `:703-717` — `depth_meaning`, or the discriminator stated as `depth_is_proof_when_search_nodes_zero` — and change `:363` to name it. Then either extend `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name` to four or rename it.

### N-M7 — §12.3's structural-risk list omits the one lean that is already broken at HEAD

`:1201-1210`, new this round, names four leans on unfrozen WP-2.0-M material: *"the record's TAB grammar, the capture identity's three inputs, the corpus manifest file and the throughput shape"*. **All four are accurate** — I checked each against WP-2.0-M's own freeze table (`wp20m_design.md:81-97`), which lists §5 and the requirement-5 claim explicitly under *"NOT FROZEN, because no reviewer passed them"*, and §4.2 and §11 are new in revisions 3 and 4 and are not in the frozen list. ✓

**The list omits the `totals_of` lean, and that one is not a risk — it is already inconsistent.** WP-2.0-M's §8 at HEAD (`wp20m_design.md:698-706, 736-738`), inside the block the freeze table records as passed and lifted verbatim, still says:

> **The visibility change is for WP-2.0-S**, so that package **adds fields to one parser** instead of writing a second …
> It is registered here because it is the guard WP-2.0-S inherits when **it adds `score` and `pv` as non-fatal `Option`s** (D-542).

**WP-2.0-S revision 3 does neither.** §3 splits `totals_of` into `fields_of` + `totals_of` and *"adds no lookup to `totals_of` at all"* (`:597-598`); §2.8 says *"**no package is building the `pv` half**"* (`:438-439`). So at HEAD the two designs disagree about what WP-2.0-S builds, and about the stated justification for a visibility change §3 depends on — and §12.3 instead reassures the reader that *"§3's whole premise rests on the normalisation removing `time` (a passed paragraph, low risk)"*, which is a different, genuinely low-risk lean.

The paragraph's stated purpose is *"what tells a successor which sections to re-read when WP-2.0-M lands"*. The section it most needs to name is §3, and §3 is the one it clears.

**FIX.** Add the fifth lean: *"and WP-2.0-M's §8, whose frozen text still describes this package as adding `score` and `pv` to `totals_of` — which §3 does not do. That description is stale at HEAD and is an ADR/prose correction WP-2.0-M owes, not a change to §3."*

### N-M8 — §0.1a is incomplete, and the incompleteness is where the round's loss is

Detailed above. Twelve rows against thirty findings and twenty-four hunks; every MINOR remedy except m14 is unlisted; the book test rename that broke a test (N-M1) is unlisted; and the table's own preamble — *"An empty right-hand column below is the finding"* — implies coverage it does not have.

I raise it separately from N-M1 because the guard is the round's headline claim (`:10-17`) and because **two of the three defects that survived this round would have been caught by the table if it had been complete**: N-M1 (an unlisted rename), N-M4 (a listed row whose count was not checked). It is a real instrument and it works when it is applied; it was applied to thirteen of thirty findings.

**FIX.** One row per finding, thirty rows, with `—` where nothing was spent — which is the format WP-2.0-M's §0.2b uses for its own nine.

---

## MINOR

1. **N-m1** — `:549` ends *"…read the two as one unit.`---`"*: the horizontal rule that separated §2 from §3 is glued to the last word of §2.10. The section break is lost and a literal `---` renders in the prose. Introduced by this round's §2.10 hunk (`cat -A` confirms one line, no newline before `---`).
2. **N-m2** — `:1043` is still `a_record_whose_turn_and_moves_disagree_is_refused_by_name` while the column is `turns_played` everywhere else (`:175`, `:949`, and §2.2's own `:258-259` describing this very refusal). The m11 rename reached every prose site and one test name.
3. **N-m3** — `:849`: *"These are the only two numerals of the kind D-483 governs"*. This round added two more: `:876` *"the arc's own **fourteen** firings"* (the trigger-rich band's measured win denominator, `matrix_stage3_detector.md:611`) and `:847` *"at **five hundred** pairs"* (the measured run in `configs/random_openings_v1.toml:47`). D-483 (`docs/decisions.md:1034`) forbids *"measured numbers"*; these are measurements from measurement packages, not code facts like *"twelve symmetries"* or *"32 hex digits"*. The previous reviewer cleared the code facts; these two are a different class. **The claim is false as written for the fourth revision running** (rev 1 MINOR 2, rev 2 NEW-m1 and NEW-m4, now this). The numbers are used illustratively rather than consumed as thresholds, so I do not raise a D-483 breach — I raise the self-claim.
4. **N-m4** — **`to_move`'s token set is never stated anywhere in the document**, in any revision, while §5 fixes three spellings on the ground that *"A LOADER CANNOT CHECK A TOKEN IT CANNOT PREDICT"* (`:682`) and then has the loader check `to_move` against *"its own token set"* (`:738`). `result`, `end` and `book` all have theirs (`:188-189`, `:489`). New this round: `key_full` now spells a side as `1`/`2` (`:698-699`) while pistol-core's own protocol spelling is `p1`/`p2` (`crates/pistol-core/src/board.rs:30-36`, *"The protocol and fixture spelling of this player"*), so one record will carry two spellings of one concept and the document fixes only the newer of them.
5. **N-m5** — `:864-866` calls trigger-rich a **frame**. `matrix_stage3_detector.md` calls it a **band** and reserves *"frame"* for **PER-SEARCH vs AGGREGATE** (`:608`, *"Two frames, and they are not the same number"*), an axis §8 is silent on and on which the arc's own conclusion differs sharply: *"In the PER-SEARCH frame … **there the gap is ZERO** and the barrier is arithmetic, not sample size at all"* (`:645-650`). The pairing §8 wants is still determinate — item 3 names *"the measured column bound"*, which is the table's third column and neither the per-search ceiling nor the aggregate oracle — but the word the remedy chose is the cited document's word for a different distinction.
6. **N-m6** — items 3 and 4 define the alternative two ways. Item 3 says *"the bound's **lower end**"*, which is a comparison ACROSS bands (0.857 vs 1.000). Item 4 says both figures are read from the trigger-rich band alone, where the bound is a single number with no "lower end". They agree numerically; they are two statements of one rule, which is the pattern D-423 names.
7. **N-m7** — `:940`: *"the table is the one place they are stated (D-423)."* Row 3 is also stated at `:744-749` and row 9 at `:308-309`.
8. **N-m8** — `two_positions_differing_only_in_colour_do_not_share_a_key_full` needs a fixture whose two positions are **not** images of each other under a symmetry that permutes the shared cells consistently with the colour swap; `transform` preserves colour (`symmetry.rs:143-155`), so a colour-symmetric pair has one canonical form and the test asserts something false. The design does not state the fixture condition, and the mutant *"`key_full` rendered as bare cells"* (`:1082`) dies only if the pair also shares its cell set. §11 already carries a fixture-condition clause for one test (`:1125-1129`); this one needs the same.
9. **N-m9** — INVARIANT 11 (`:993-995`) names four of the row's six fields and omits the artifact's path, which `:1149` includes and which is the field the manifest exists to index.
10. **N-m10** — `:1166-1167`: *"WP-2.0-M creates it with the capture table, and this package adds the second heading."* WP-2.0-M §13(a) (`wp20m_design.md:998-1006`) says the file holds one row per capture, that the mode prints the row, and that the row is added in the commit that records the run. It says nothing about creating the file or about headings. An inference about another package's behaviour, stated as its decision.
11. **N-m11** — `:1205`: *"Four of this document's **ten** leans"*. The ten comes from the revision-2 review's dependency table and is uncited; the count itself is not a claim the design can support from its own text.
12. **N-m12** — `to_move_is_the_side_pistol_core_puts_to_move_at_that_prefix` maps to INVARIANT 3 (`:1051`), which is about `score_kind`'s three tokens and the node columns. **No invariant states §2.2's rule.** Likewise `a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name` maps to INVARIANT 6, which does not state it (N-M2), and `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` maps to INVARIANT 8 while pinning §2.7's two-column decision (carried unchanged from revision 2).
13. **N-m13** — §9 row 4 has three limbs and one test. `a_capture_record_with_a_tab_in_a_field_is_refused_by_name` covers the TAB; **the capture record's field count and its empty fields have no test** (the two similarly-named tests at `:1032-1033` are corpus-loader tests, a different file).
14. **N-m14** — `opening_turns` joins the header this round (`:705-706`) with no test and no mutant. The loader rule is *"a header missing any of its params"* (`:739`) but the only header test is `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name`. The header now carries seven-plus params and one test covers three of them.
15. **N-m15** — `:339-340`: *"the site that ENFORCES it is `crates/pistol-search/src/search.rs:513-514`, which overwrites **both counters** from the run"*. Those two lines are `outcome.info.nodes = run.total_nodes();` and `outcome.info.search_nodes = run.search_nodes;`. In §2.4's own vocabulary *"the two independent counters"* are `search_nodes` and `solver_nodes` (INVARIANT 3, `info.rs:167-172`), and `solver_nodes` is overwritten at `:519`, outside the cited range. The claim is true of `nodes`/`search_nodes` and the citation supports the fallback; the phrase invites the other reading.
16. **N-m16** — `:846-848` cites `configs/random_openings_v1.toml:47-52` for *"a measured level and power well short of them"*. The cited passage's own conclusion, on its last two lines, is *"At 2000 pairs the same bounds achieve alpha 0.048 and power 0.945"* — the book was sized to 2000 **because** 500 did not deliver. The design's sentence is true of the 500-pair case and the passage says the opposite about the size actually in force.
17. **N-m17** — §7 `:813-814` still says the fold *"on this corpus's records loses nothing (§2.1)"* while §2.1 `:232` and §8 `:913` now ground the property on any ONGOING position. Not false — corpus records are ongoing positions — but it is the narrow scope NEW-M8 had the document repudiate, surviving in the one section that enumerates what each key folds.
18. **N-m18** — `:246-247` still cites `crates/pistol-core/src/turn.rs:25-29` for *"every asked position is a turn boundary at `Phase::First`"*. Those lines are `Phase::index`'s match arms; they show `Phase` exists. The claim is WP-2.0-M's INVARIANT 2. Carried from revision 1 through three rounds.
19. **N-m19** — §11 `:1125-1129` requires `the_derived_outcome_agrees_with_the_reports_own_result_field`'s reports to be *"produced by the arena"*, eleven lines after *"none of these tests needs a real engine"* and *"the transform reads FILES, so a fixture capture and a fixture report exercise every path"*. Hard rule 8 forbids committing a match log, so the report cannot be a committed fixture. The tree's answer is the STUB harness — `crates/pistol-arena/tests/replay_chain_tests.rs:12-31` runs the real `arena` binary against `STUB` into a scratch directory — and the design names neither the stub nor the scratch run.

---

## The citation check, and AUTHOR DEBT

`python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` → **119 citations checked, 0 unreproduced.** Green before the review, per D-546's condition.

**I hand-verified twenty-three citations' CONTENT — every one new in revision 3, and the load-bearing carried ones.**

**New in revision 3, all ✓:**

- `crates/pistol-search/src/search.rs:513-514` ✓ — `outcome.info.nodes = run.total_nodes(); outcome.info.search_nodes = run.search_nodes;`, the gate-off enforcement (one imprecision, N-m15).
- `crates/pistol-core/src/symmetry.rs:143-155` ✓ — `transform`'s doc and body, `.map(|&(cell, player)| (symmetry.apply(cell), player))` then `sort_unstable`. Colour preservation is exact.
- `crates/pistol-core/src/symmetry.rs:157-165` ✓ — both quoted sentences, and the return type `Vec<(Coord, Player)>` on `:165`, which is the whole of NEW-B1.
- `crates/pistol-core/src/coord.rs:136-141` ✓ — *"`\"q,r\"` — the stone token of the line protocol"*, `write!(f, "{},{}", …)`, no colour and no colon.
- `crates/pistol-core/src/state.rs:111-115` ✓ — `turn()`'s doc, *"counting from [`FIRST_TURN`]"*. The *"one more"* arithmetic additionally needs `FIRST_TURN = 1` (`crates/pistol-core/src/rules.rs:18`, `state.rs:63`), which the document does not cite; the claim is right.
- `crates/pistol-arena/src/openings.rs:39` ✓ — `pub opening_turns: u32,` under *"How many turns every opening has"*, which is the uniformity the design leans on.
- `crates/pistol-arena/src/transcript.rs:307` ✓ — `forfeit: value(&fields, "end", record)? == "forfeit",`, so `forfeit` really does come off `end`.
- `configs/bench_wp18c_solver_on.toml:45` ✓, `configs/gate_staged_solver_v0.toml:47` ✓, `configs/play_staged_solver_v0.toml:75` ✓ — all three `on_search_path = true`; `configs/gate_v0.toml:94` ✓ `false`. Exhaustive against `LC_ALL=C /usr/bin/grep -rn "on_search_path" configs/ | LC_ALL=C sort`.
- `configs/random_openings_v1.toml:47-52` ✓ reproduces (half-read, N-m16).
- `crates/pistol-cli/src/corpus/emit.rs:40-44` ✓ — *"A value the extraction computed. Never a `param` …"*, verbatim.
- `docs/experiments/wp20m_design.md` §11 ✓ — the throughput obligation is there at HEAD (`:926-935`).

**Carried and re-verified, all ✓:** `crates/pistol-arena/src/replay.rs:137-138`; `transcript.rs:39-40` (and `:173`, `:202`, where `read` parses it); `crates/pistol-arena/src/lib.rs:41-45` (*"Nothing here writes inside the repository"*) and `:47-69`; `crates/pistol-arena/src/bin/arena.rs:82-100`, `:94-99`, `:103`, `:104-107`; `crates/pistol-cli/src/protocol.rs:172-174` (writes info, totals and bestmove; never reads `outcome.provenance` — and `git grep -n "Provenance" -- crates/pistol-cli/src/` returns nothing, exit 1, so `:427` is TRUE); `crates/pistol-cli/src/report.rs:15-18`, `:20-29`, `:145-158`, `:147-152`; `crates/pistol-search/src/info.rs:153-155`, `:162-166`, `:167-172`; `crates/pistol-search/src/pvs.rs:148-152` (`total_nodes` = the derived sum); `crates/pistol-core/src/state.rs:128-133`; `crates/pistol-core/src/symmetry.rs:206-219` and `:213-218`; `crates/pistol-core/src/turn.rs:107` and `:177-186`; `crates/pistol-core/src/board.rs:30-36`.

**AUTHOR DEBT the checker could have caught: NONE.** Every `path:line` reproduces and every one I checked supports its claim. **Three consecutive rounds with zero author debt** — this is the part of D-546's method that is demonstrably working, and I record it as such.

**The debt the checker cannot catch is all of Part 2 except N-m15 and N-m16.** Of this round's nine BLOCKING/MAJOR findings, **six are contradictions between two sentences of this document** (N-B1, N-M1, N-M2, N-M3, N-M4, N-M6), one is a claim about another document's completeness (N-M7), one is about the guard table (N-M8), and none is a false citation. **The failure class has moved again**: revision 2 failed largely on claims about the tree and about governing documents; revision 3's are almost entirely internal consistency, which is what §0.1a exists to catch and did not, because it was applied to thirteen findings out of thirty.

---

## Closure over the twelve invariants, forty-six tests and thirty-two mutants

**(a) Invariants with no test**: **one, declared** — INVARIANT 2 (`:999-1009`), honestly, with its evidence named and INVARIANT 5's test correctly identified as the one that would catch a clock read that changed an output. INVARIANT 11 and INVARIANT 12, new this round, each have tests. Everything else maps. ✓ **Closed.**

**(b) Tests pinning nothing / mis-mapped**: **three** — `to_move_is_the_side_pistol_core_puts_to_move_at_that_prefix` → INVARIANT 3 (N-m12); `a_key_full_field_that_is_not_cell_colour_pairs_is_refused_by_name` → INVARIANT 6 (N-M2); `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` → INVARIANT 8 (carried). Revision 2's two `fields_of` mis-mappings are **closed** by INVARIANT 12.

**(c) Rules with neither test nor mutant**: §5's write-side TAB refusal (N-M5); §9 row 4's field-count and empty-field limbs on the capture (N-m13); the header params other than the three units, including the new `opening_turns` (N-m14); `to_move`'s token set, which is not a rule at all (N-m4); §2.5's `depth_turns` discriminator, which is in no section (N-M6). **Revision 2's list is otherwise closed**: `key_seq` ✓, `to_move`'s value ✓, the manifest row ✓, §9 rows 2 and 11 ✓.

**(d) Mutants that cannot die**: **one, and it is new** — *"the write-side TAB check removed"* against a read-side test (N-M5). Revision 2's one (`to_move` from the `turn` column) is **removed rather than replaced**, which is right. I checked the other thirty-one individually. The strongest are *"the book boundary read as `k < opening_turns`"* (dies at the whole-book test), *"`key_full` rendered as bare cells"* (dies at the colour test, given the fixture condition of N-m8), *"`key_seq` computed from `canonical_form`"* (dies at the transposition limb), and *"the outcome relation gated on `a_is_p1`"* (carried, still clean).

**(e) Tests that would pass vacuously**: **none.** But one test **cannot pass on correct code** — `the_first_position_an_engine_chose_from_is_flagged_not_book` (N-M1) — and one **could fail on correct code** if its fixture is a colour-symmetric pair (N-m8). Both are failures of the register rather than vacuity, and both are new this round.

**(f) Rules stated twice and differently** — the sweep the dispatch asked for, over the four places it would hide:

| where | verdict |
|---|---|
| the `turn` → `turns_played` rename | **clean in prose**, one test name left behind (N-m2) |
| the key spellings | **clean** — `:537`, `:677-678`, `:682-701`, `:179` all agree on three columns and on `q,r:1`/`q,r:2` |
| the invariant renumbering | **clean** — INVARIANT 12's two remapped tests leave 3 and 4 adequately pinned |
| INVARIANT 11 / 12 | **11 states four of six fields (N-m9); 12 is consistent with §3** |
| **§8's test** | **NOT clean — N-B1** |
| **INVARIANT 4's strength** | **NOT clean — N-M4** |
| **`forfeit_by`** | **NOT clean — N-M3** |
| **the `depth_turns` header entry** | **NOT clean — N-M6** |
| **the book boundary in the test register** | **NOT clean — N-M1** |

**(g) The D-483 numeral sweep.** I stripped inline-code spans, D-numbers, `§`-references, invariant/row/rule indices and WP tags from all 1228 lines and inspected every surviving numeral. They are: section numbers, column indices 1–16, refusal-row indices 1–12, invariant indices 1–12, three code facts (twelve symmetries, 128-bit, 32 hex digits), the `0.05`/`0.95` pair at `:843` (deliberate, argued, and in my judgment correctly sheltered for the reason the previous reviewer gave), and **two spelled-out measured quantities new this round** — *"fourteen firings"* (`:876`) and *"five hundred pairs"* (`:847`). Neither is consumed as a threshold; both falsify `:849`'s claim about the document's own record (N-m3). **No measured number enters the rule.**

---

# PART 3 — THE VERDICT

## **FAIL** — 1 BLOCKING · 8 MAJOR · 19 MINOR

Prior-round disposition: **2 of 2 BLOCKING applied** (one clean, one with a new defect); **8 of 10 MAJOR applied, 2 partially**; **16 of 18 MINOR applied, 1 partially, 1 with a new defect**. **This is the best fix round in the arc by a wide margin** — the substantive decisions the two BLOCKINGs forced are both right and I could not break either.

## "Could an implementer build from this without deciding something the design should have decided?"

**NO.** They would have to decide:

1. **Whether §8's minimum comes from a one-sample binomial test or a two-proportion test** — item 5 says the first, the operative sentence says the second, and item 5 says the second cannot be satisfied (**N-B1**). D-537 says this is not a successor's choice.
2. **Which position `the_first_position_an_engine_chose_from_is_flagged_not_book` is about**, since as named it contradicts its sibling test and §2.9's rule (**N-M1**).
3. **Whether the loader checks `key_full`'s shape**, and against what rule — §5's enumeration says no, a registered test says yes (**N-M2**).
4. **Whether `RecordedGame` carries `forfeit_by`** — §2.8 says it does, §4 says it was deliberately dropped (**N-M3**).
5. **What INVARIANT 4 asserts** — §3 says per-game node counts, §4 says byte-identity (**N-M4**).
6. **Which side of the transform the TAB mutant is about**, and what test kills the write-side check (**N-M5**).
7. **Where the `depth_turns` discriminator goes in the header, and in what line kind** — §2.5 names a `note` block §5 abolishes and §5's header has no entry for it (**N-M6**).
8. **`to_move`'s token set** (**N-m4**), and the fixture condition that makes the colour test meaningful (**N-m8**).

Everything else — the parser split and the two-token score read, the three keys and what each folds, the `q,r:1` spelling and its non-collision, the `-` sentinel, the node pair and its gate-off fallback, the all-six-or-none solver refusal, `depth_turns`'s two meanings and their discriminator, the outcome relation and the seat-blind check claimed for what it is, the no-dedup and no-seed policies, the book boundary IN PROSE, the twelve refusal rows, the TAB record, α and the power, the manifest row and the two-table file — **is decided, and decided well.**

## The strongest attack that did not land

**I set out to break the `q,r:1` spelling**, on the theory that NEW-B1's remedy had been written quickly and that a one-character separator chosen under review pressure would collide somewhere — which would have made the third consecutive round's BLOCKING remedy worse than the defect, in the column D-537's non-loosenable denominator counts over.

**It failed on four fronts and the tree closed each.** `PAIR_SEPARATOR` is `'/'` (`crates/pistol-core/src/turn.rs:107`), so no turn token or `best` value carries a colon. `Coord`'s `Display` is `write!(f, "{},{}", self.q, self.r)` (`coord.rs:136-141`) — comma only, and it renders negative axials without introducing one. `score_token`'s three arms are `cp {value}`, `mate {turns}`, `-mate {turns}` (`report.rs:153-158`). Every remaining column is an integer, a 32-hex key or a fixed word token. And the rendering is genuinely injective, not merely non-colliding: `transform` sorts and preserves colour (`symmetry.rs:143-155`), so `canonical_form` returns a totally ordered `Vec<(Coord, Player)>` whose space-joined `q,r:c` spelling can be parsed back element by element. **`:699-701`'s claim is exactly true and the spelling is the right fix.**

**A second attack also failed.** I tried to show that NEW-M2's remedy had spent the requirement into no package — that §12.2's deletion of the throughput shape left it homeless the way revision 1 did, which would have been the purest instance of D-548's class. **WP-2.0-M §11 carries it at HEAD** (`wp20m_design.md:926-935`), including the per-position `newgame` + `go` shape and the magnitudes' assignment to the pilot. The asked-position count that §12.2 deleted is derivable from WP-2.0-M §2's own prefix enumeration, so nothing was lost. **The pointer is honest and D-423-correct.**

**A third.** I re-derived the book boundary from `replay.rs:137-138` without reading the document's derivation, including both ends (`k = 0` vacuously book; `k = opening_turns` all-book). **`k <= opening_turns` is right**, `opening_turns` is on `Transcript` and is parsed by `transcript::read` at `:173`, and the inverted mutant dies at its named test. The prose half of NEW-B2's remedy is complete and correct; only the test name is wrong.

## What I could not settle by reading, and the run that would

1. **Whether the `fields_of`/`totals_of` split compiles and leaves the SPRT report's per-game node counts intact.** The three lookups are the same three expressions over the same `Vec<&str>` and the lifetime elides, so I expect yes. **The run**: `cargo test --workspace --locked` plus `tools/arena_smoke.sh` (gate 15) on the split. **Refused by the dispatch's hard constraint.**
2. **Whether the fourth arm fits gate 17.** `wc -l crates/pistol-arena/src/bin/arena.rs` is **283** against `SOFT_CAP=300`, and WP-2.0-M extracts `USAGE` (43 lines) into `usage.rs` first. Arithmetic says yes with room. **The run**: `tools/file_justification_check.sh` at the post-implementation revision. Not run.
3. **Whether a legal fixture pair exists for `two_positions_differing_only_in_colour_do_not_share_a_key_full`.** At `turns_played = 3` the position holds five stones, three P1 and two P2 (rule 3: 1, then 2 per turn), so two colourings of one cell set are arithmetically available; whether a pair exists that is also legal under the radius-8 rule AND not a colour-swap symmetry image needs enumeration. **The run**: a scratch test in a `git worktree add --detach` enumerating depth-3 prefixes from two fixture games and comparing `canonical_form` outputs. N-m8 does not depend on it — the fixture condition is unstated either way — but the run would price it.
4. **Whether the one-sample rule N-B1's fix would install yields a finite, sensible `n`.** A one-sample binomial test of `p0 = 0.571` against `p = 0.857` at level 0.05 and power 0.95. **The run**: a `statsmodels`/`scipy` power calculation. I could not make it here, and **D-483 forbids the answer entering the design** — which is the design's own position and is correct. What the design owes is the rule, not the number, and after the fix it would have exactly one rule.
5. **Whether `a_rerun_over_one_capture_and_report_is_byte_identical` kills its ordering mutant in one process.** It turns on whether two `HashMap`s in one thread draw different `RandomState` keys. **The run**: the mutant in a `git worktree add --detach` with its own `CARGO_TARGET_DIR`. Carried unresolved from revision 2.

---

## One paragraph for the operator

**The method is working and this document is close.** Three rounds with zero author debt, a citation set that has grown from 60 to 119 and reproduces entirely, both BLOCKING decisions taken correctly on the substance, and a failure class that has moved again — this round's nine serious findings are almost all one sentence of this document contradicting another sentence of it, and none is a claim about the code the code does not make. **What it failed on is its own new guard.** §0.1a is the right instrument and it caught nothing here because it was pointed at thirteen of thirty findings: the round's one genuine loss — a renamed book test that now asserts the negation of the rule it was renamed for — is inside a remedy the table lists and describes incompletely, and two of the table's own right-hand columns are false about the document they describe. **The fix is small and almost entirely deletion or one-clause correction**: one sentence in §8, one test name and one mutant row, one clause in §2.8, four words in §4, one bullet in §5's loader, one header param, and §0.1a extended from twelve rows to thirty. None of them touches a decision. I would expect a fourth round under this method to pass, and I would ask that the round's first act be to complete §0.1a rather than to start from the findings — because the two defects that survived this round are both things a complete table would have shown its author before I did.
