# SCOPED RE-REVIEW — `docs/experiments/wp20s_design.md` revision 2

## Header

- **Revision adjudicated**: `f96593b94e0da78711071367e6b8f5e214e1f321` (`docs(wp20s): the fix round — the parser that could not carry a two-token score becomes a word list …`).
- **Matches HEAD**: **YES**. `git rev-parse HEAD` = `f96593b94e0da78711071367e6b8f5e214e1f321`, branch `dev`.
- **Tree state**: **clean** at the start and end of this adjudication — `git status --porcelain` printed nothing both times. The only file I wrote is this one.
- **What I ran**: `git` (`log`, `rev-parse`, `status`, `show`, `diff`, `grep`), `/usr/bin/grep`, `awk`, `sed`, `wc`, `cat`, `ls`, `LC_ALL=C sort`, `python3` (two throwaway text-processing scripts over `docs/decisions.md` and the design itself), and `python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` — **green, 104 citations checked, 0 unreproduced** (revision 1: 60).
- **What I refused to run**, per the dispatch's hard constraint: `cargo` in any form, `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Where a claim needed a run I could not make, I name the run in Part 3.
- **Binding reading, all in full**: `docs/experiments/wp20s_design_REVIEW.md` (BLOCKING 1–6, MAJOR 1–12, MINOR 1–12); `git show 1157dae:docs/experiments/wp20s_design.md`; `CLAUDE.md`; `docs/process.md`; `docs/decisions.md` D-483, D-518, D-531, D-537, D-539, D-540, D-542, D-543, D-544, D-545, D-546, D-547; `docs/experiments/wp20_dispatches.md` (all three transcriptions and the preamble); `docs/experiments/wp20m_design.md` **revision 4** (§0–§5, §8, §9, §11–§14); `docs/experiments/wp20_DESIGN_STOP_SPLIT.md` §3; `docs/experiments/matrix_wp20_pipeline_shape.md` §3 row (b), §5; `docs/experiments/matrix_stage3_detector.md` §5.1 and the closing tables.
- **Code read**: `crates/pistol-arena/src/{exchange,transcript,conclusion,record,dedupe,game,replay,openings,report,config,lib}.rs`, `crates/pistol-arena/src/bin/arena.rs`, `crates/pistol-cli/src/{report,protocol}.rs`, `crates/pistol-cli/src/corpus/emit.rs`, `crates/pistol-core/src/{symmetry,state,turn,zobrist,coord,board,lib}.rs`, `crates/pistol-search/src/{info,score,search,pvs,census}.rs`, `configs/` (all 36 files grepped for `alpha`/`beta`/`on_search_path`), `tools/sealbot/matchserver/src/pistol_client.rs`.

---

## VERDICT: **FAIL**

**2 BLOCKING · 10 MAJOR · 18 MINOR**, all of them **NEW** — introduced by this fix round, not carried forward.

**The prior round's twenty-nine findings were applied well.** All six BLOCKING findings changed the document; nineteen of the twenty-three MAJOR/MINOR findings are fully applied and verified against the tree; three are partial and one limb is untouched. The two attacks the prior reviewer said were the document's best work still hold, and I re-attacked both and could not break them.

**It fails for exactly the reason this arc keeps failing, and the dispatch predicted the shape.** Applying the findings introduced fresh defects of D-544's and D-545's two classes:

- **a remedy that reproduces the defect it was fixing** — MAJOR 8 removed a mutant that could not die and replaced it with a mutant that also cannot die at its named test, leaving `to_move` with no test at all;
- **a reversal applied to some sites and not all** — the turn-zero sentinel is `-` in §2.10 and "empty" in the §2 schema table and in §9's refusal row; INVARIANT 4 pins per-game node counts while §3 says it pins byte-identity; §5 counts two whitespace-bearing columns and then names three;
- **a claim about a governing document that document does not make** — §12.2 says WP-2.0-M "did not have" the throughput obligation, when WP-2.0-M revision 4 assigned it eleven minutes earlier in the commit immediately below this one; and §11 says every committed config has the solver off the search path, when three do not.

And two decisions that BLOCKING findings forced into existence are each stated twice, differently: the `book` column's boundary contradicts the column's own definition, and `key_full`'s spelling drops half of what `canonical_form` returns.

---

# PART 1 — DISPOSITION OF EVERY PRIOR FINDING

## BLOCKING

### BLOCKING 1 — `fields_of` cannot produce `score_value` — **APPLIED**

`wp20s_design.md:494-496` types it `fields_of(line) -> Option<Vec<&str>>`, *"returns the line's tail **as its WORDS, in order**"*; `:512-521` names the reintroduction and the fix; `:536-540` states the read as *"the word after `score` is the tag … and the word after THAT is the number"*.

**Verified against the tree, four ways.**

1. **The read is correct against `render_info`'s field order.** `crates/pistol-cli/src/report.rs:82-84` writes `info[ totals] depth_turns <n> seldepth <n> nodes <n>[solver block] nps <n> time <n> hashfull <n> score <token> pv …`. `score_token`'s three expansions are `cp {value}`, `mate {turns}`, `-mate {turns}` (`report.rs:153-158`) — **exactly two words each**, so "the word after `score`" is the tag and "the word after that" is the number in all three arms. ✓
2. **`totals_of` keeps its three lookups load-bearing over a word list.** The existing implementation ALREADY splits to words and looks up "the word after the key": `let words: Vec<&str> = rest.split_whitespace().collect();` and `words.iter().position(|word| *word == key).and_then(|at| words.get(at + 1))` (`crates/pistol-arena/src/exchange.rs:176-183`), with three `?`-chained lookups at `:185-187`. The design's *"The lookup is the one it already performs — 'the word after the key' — over the same words it already splits"* (`:508-510`) is exact. ✓ The split is therefore a **pure extraction**, not a rewrite — which is why the SPRT path's three expressions are unchanged.
3. **Word-exactness is real, and this matters.** The lookup compares whole words (`*word == key`), so `nodes` cannot match `search_nodes` or `solver_root_nodes`. Had the helper used `contains`, the word list would have silently mis-read the solver block. ✓
4. **Lifetime shape is workable.** `fn fields_of(line: &str) -> Option<Vec<&str>>` elides to one input lifetime, and every element is a subslice of `line` — `rest` at `exchange.rs:175` is already such a subslice, and the current `words` Vec is already `Vec<&'line str>`. `totals_of` then binds `words` locally and closes over it exactly as today. ✓

**Nothing else in the document assumes a key→value map** — except one leftover phrase at `:528` (**NEW-m3** below), which does not change the argument it sits in.

### BLOCKING 2 — `pistol_core::canonical_form` exists — **APPLIED**, and I attacked the new claim hard

`:718-721` deletes both false sentences by name. `:184-190` adds `key_full`. `:204` replaces revision 1's two-site export citation with `crates/pistol-core/src/lib.rs:85-90`, verified: `GameState` at `:85`, `canonical_form`/`canonical_sequence` at `:86`, `Key128` at `:90`. ✓

**The "loses nothing" claim (`:192-199`) is TRUE, and the document under-claims it.** `canonical_form` carries `Player` per stone (`crates/pistol-core/src/symmetry.rs:148-155,165`), and `transform` maps `(cell, player) -> (symmetry.apply(cell), player)` — colour is preserved by every image, so the stone COUNT is an invariant of the canonical form. `GameState::key`'s own doc is *"for an ongoing game the stone count fixes the turn, the phase and the mover together"* (`crates/pistol-core/src/state.rs:129-133`) — **for an ongoing game**, not merely at a turn boundary. I checked the arithmetic: turn 1 places one stone, every later turn two (`crates/pistol-core/src/rules.rs`'s `stones_in_turn`, CLAUDE.md rule 3), so a count `n` maps to exactly one `(turn, phase, mover)` triple whether or not the count is at a boundary. **So the property holds on every ongoing position, and the document's "and only on them" (`:198-199`) is false in the safe direction** — which is not free, because §8 then leans on it outside that scope (**NEW-M8**).

- **Turn zero**: `canonical_form(&[])` does **not** panic. `transform(&[], s)` returns an empty `Vec`, all twelve images compare equal, `best` is assigned on the first iteration, and `best.unwrap_or_else(|| unreachable!(…))` (`symmetry.rs:174-178`) is never reached. ✓ §2.10's `canonical_form(&[])` is well defined.
- **After a `Turn::Single`**: a `Turn::Single` is either turn 1 or a game's last (`crates/pistol-arena/src/transcript.rs:369-376` refuses any turn recorded after a win). A won prefix is not asked (`wp20m_design.md:237-242,257-258`), so every asked position is ongoing and the count argument applies. ✓
- **Is `key_full` the coarsest of the three?** **YES.** `key_seq`-equal ⟹ same symmetry image of the sequence ⟹ same symmetry image of the stone set ⟹ `key_full`-equal. `key_pos`-equal ⟹ identical stones ⟹ `key_full`-equal (identity is one of the twelve). Both implications are strict in general. ✓ `:715-716` is right.

**But the remedy introduced NEW-B1**: §5 spells the new column in a way that discards `Player`.

### BLOCKING 3 — the outcome relation went through `a_is_p1` — **APPLIED**, clean

`:340-356` restates the relation as `Outcome::Win{winner: P1} ⟺ result == p1_win`, `P2 ⟺ p2_win`, `Ongoing ⟺ capped`, and says `a_is_p1` is not read. A test (`:916 the_outcome_check_holds_when_engine_b_takes_seat_one`) and a mutant (`:945 the outcome relation gated on a_is_p1`) are registered.

**Verified against the arena on every path.**

- **Non-forfeited win**: `crates/pistol-arena/src/game.rs:103-108` maps `pistol_core::Player::P1 => GameResult::P1Win`, `P2 => P2Win`, and `record.rs:16-22` tokenises `P1Win => "p1_win"`. ✓
- **Turn cap**: `game.rs:52-61` returns `GameResult::Capped` with `End::Normal` when `state.turn() > rules.turn_cap`, and the recorded move list contains no win, so a replay ends `Ongoing`. ✓ `Ongoing ⟺ capped` holds.
- **Forfeited**: `game.rs:64,77-85,92-100` writes `GameResult::loser_of(mover_is_p1)` with `End::Forfeit(_)`. The relation is explicitly restricted to non-forfeited games (`:341`, INVARIANT 8 at `:861-863`), so no forfeited game is tested against it. ✓ An illegal answering turn is never pushed (`game.rs:89-102` pushes at `:102`, after the `make_turn` succeeds), so the recorded list stays legal.
- `a_is_p1` really is about which ENGINE holds seat one (`crates/pistol-arena/src/transcript.rs:17-18`, *"Whether engine A held the first seat"*). ✓

This is the cleanest fix in the document.

### BLOCKING 4 — book-turn treatment — **APPLIED, BUT THE BOUNDARY CONTRADICTS THE COLUMN'S OWN DEFINITION** (NEW-B2)

A decision is taken (`:432-459`), a column added (`:157`), INVARIANT 10 added (`:866-867`), two tests and two mutants registered. `opening_turns` **is** reachable on `Transcript` (`crates/pistol-arena/src/transcript.rs:39-40`, read at `:173`) — verified. The boundary is verified below and is **wrong by one against the column's own two glosses**. See **NEW-B2**.

### BLOCKING 5 — the throughput expectation — **APPLIED, ON A FALSE CLAIM ABOUT WP-2.0-M** (NEW-M2)

`:1001-1019` states the shape with no magnitudes: pass 1 one game per game at the GAME budget; pass 2 one `newgame` + one `go` at the LABEL budget per asked position; the asked-position count as *"the number of turn boundaries in the report, less one per won game"* (correct against `wp20m_design.md:255-259`: `k` from zero to `len` inclusive, less `k = len` when the last turn wins); this transform a file pass with a replay per record; the magnitudes the pilot's. That is a shape and not a number, and it discharges the dispatch's clause. ✓ The framing claim is false — **NEW-M2**.

### BLOCKING 6 — the corpus manifest — **APPLIED**, with a closure hole (NEW-M9)

`:980-999` gives the corpus its own printed manifest row, on the ground `crates/pistol-arena/src/lib.rs:41-45` states (*"Nothing here writes inside the repository"* — verified verbatim), naming the row's six fields.

**Does it answer matrix decision 12's "digest boundary"?** Decision 12 at `matrix_wp20_pipeline_shape.md:358` is one line: *"the corpus manifest's digest boundary | open"*. `:995-997` answers it explicitly — *"the row binds the corpus to the capture and the capture's own row binds that to the report, so a reader walks the chain from a committed file to every artifact in it"* — and names where each digest sits. **That is a boundary decision, not an assertion that one was taken.** ✓ Delivered.

**But** the test it registers exists nowhere else in the document, and the rule has no invariant — **NEW-M9**.

## MAJOR

| # | finding | disposition | evidence |
|---|---|---|---|
| 1 | §6 declines a mutant on a quotation the dispatch does not contain | **APPLIED** | `:666-683`. Three dispatches now transcribed; `wp20_dispatches.md:113-115` (unconditional) and `:318` (*"seed ignored where the pipeline samples"*) both quoted exactly and attributed correctly. The departure is owned at `:675-676` rather than read away |
| 2 | the cross-check is not an externally derived referent | **APPLIED**, verbatim to the FIX | `:358-367` — *"**It is not**"*, the shared input named (`game.rs`'s `make_turn`, `transcript.rs:359-379`), the referent restated as the arena's recorded verdict, the defect class named as this transform's own replay/mapping, and the independence claim disowned. Matches `docs/process.md:49-55` |
| 3 | §8 defers α and power; input 2 has no referent; the denominator is not countable | **PARTIALLY APPLIED** | α/power fixed at `:739-753` (`configs/arena_smoke_v0.toml:66-67` verified: `alpha = 0.05`, `beta = 0.05`); input 2 replaced and **its referent exists** (below); D-518's off-the-end clause added `:795-798`; disjointness fixed to `key_full` `:785-793`. Four defects remain: **NEW-M5, NEW-M6, NEW-M7, NEW-M8** |
| 4 | corpus manifest | **MOOT** — folded into BLOCKING 6 | — |
| 5 | fourth arm / third pass departs from D-542 | **APPLIED** | `:100-109` names the departure and promises the amendment; residue 2(b) at `:1040-1041` carries it to closure |
| 6 | six solver fields and two more totals fields | **APPLIED** | `:295-303` restates the refusal over the whole block; `:405-422` names all six with reasons; `:138-140` fixed. Verified: `crates/pistol-cli/src/report.rs:69-78` emits exactly `search_nodes`, `solver_nodes`, `solver_firings`, `solver_invocations`, `solver_proofs`, `solver_root_nodes` inside one `format!` under `if info.solver_nodes > 0` (`:62`) — atomic, six fields |
| 7 | `the_transform_spawns_no_process_and_reads_no_clock` cannot fail | **APPLIED** | removed from `:885-923`; `:869-879` declares INVARIANT 2 unpinned and names the evidence. Honesty and sufficiency judged below |
| 8 | the `to_move` mutant is a no-op | **APPLIED BUT INTRODUCED A NEW DEFECT** | the ground is correct (below); the replacement mutant cannot die at its named test and `to_move` now has no test — **NEW-M4** |
| 9 | `depth_turns` has two meanings | **APPLIED** | `:305-319`. Verified: `crates/pistol-search/src/search.rs:785` `let depth = tree.win_depth_turns();` → `:793` `depth_turns: depth`, with `search_nodes: 0` at `:796`; `Provenance::SolverProof`'s doc at `crates/pistol-search/src/info.rs:259-261` is quoted exactly. §5's header `note` obligation moved to a `param` at `:620-630` |
| 10 | the `k = 0` record's `moves` and `key_seq` | **APPLIED BUT INTRODUCED A NEW DEFECT** | §2.10 at `:461-477` is a real decision with a test and a mutant, and the `-` sentinel's non-collision claim is sound (below). But `:146` and `:823` still say "empty" — **NEW-M1** |
| 11 | "the ONLY place in this workspace" | **APPLIED** | `:499-505` scopes it to `pistol-arena` and names the counterexample. Verified: `git grep -n "TOTALS_MARKER\|totals" -- crates/pistol-arena/src/` finds the marker on `exchange.rs:169-175` and nowhere else in that crate; `tools/sealbot/matchserver/src/pistol_client.rs:41,241` is the other recogniser; `crates/pistol-cli/src/report.rs:60` says *"the one substring parser in the tree (`tools/sealbot`)"* verbatim; row (b)'s kill condition is at `matrix_wp20_pipeline_shape.md:176-177` |
| 12 | §1's ground (2) overstates the freeze | **APPLIED** | `:114-119` — *"its own review is outstanding"*, with revision 1's overstatement quoted and corrected on three counts. True at HEAD: WP-2.0-M is revision 4 and unreviewed |

**MAJOR 3, the four sub-questions the dispatch named:**

**(a) Does `configs/arena_smoke_v0.toml` carry those values at those lines?** **YES.** `:66 alpha = 0.05`, `:67 beta = 0.05`. And the universal the design builds on it — *"Every committed arena experiment config in this repository carries `alpha = 0.05` and `beta = 0.05`"* (`:740-742`) — is **TRUE**: `LC_ALL=C /usr/bin/grep -rn "^alpha\|^beta" configs/ | LC_ALL=C sort` returns exactly 26 lines over the 13 `configs/arena_*.toml` files, every value `0.05`. `alpha` and `beta` are the type-I and type-II error probabilities (`crates/pistol-arena/src/config.rs:157-160`, *"Probability of accepting H1 when H0 holds"* / *"…H0 when H1 holds"*), so power `= 1 - beta = 0.95` is derived correctly.

**(b) Does "the lower end of the arc's measured column bound" have a real referent?** **YES — I found it.** `docs/experiments/matrix_stage3_detector.md:613-614` gives the bound as **0.857** on trigger-rich (12 of 14) and **1.000** out of sample; `:627-628` names it *"the best a score fitted with full knowledge of which column-classes hold wins could reach"*. Its lower end is **0.857**. The null has a referent too: `:634` gives the best written ordering as **0.571** on trigger-rich and **0.333** out of sample. Revision 1's phantom "target recall" is gone and both surviving inputs are real registered figures. ✓ **This is a genuine repair.** What is missing is which FRAME (**NEW-M7**) and which direction the choice biases (**NEW-M6**).

**(c) Is fixing α in a design a D-483 breach?** **NO, in my judgment, and the prior reviewer's own FIX directed it** (*"Fix α and the power **here**, with grounds, as D-518 fixed its own"*, `wp20s_design_REVIEW.md:324`). D-483 (`docs/decisions.md:1034`) forbids *"measured numbers"* and says *"brackets and directions are registered in the measurement package's prereg, never in the design"*. An error rate copied from thirteen committed configs is neither measured nor a bracket, and D-537's own second condition requires the rule to be complete before any score is fitted. **I record the tension rather than hiding it**: a significance level is closer to a "direction" than to a mechanism, and D-518 registered its threshold in a *registration* document, not a design. On balance the design's reading is the better one, because a rule with two unchosen parameters is not a rule. **My numeral sweep confirms nothing else slipped in** — I stripped every inline-code span, D-number, `§`-reference, invariant/row/rule index and WP tag from all 1046 lines and inspected every surviving numeral: they are column indices (1–16), refusal-row indices (1–12), invariant indices (1–10), section numbers, and three cited facts about the code (twelve symmetries, 128-bit, 32 hex digits). **`0.05` and `0.95` at `:744` are the only numbers of the kind D-483 governs.** (The document's own sentence *"These are the only two numerals this document carries"* at `:748-749` is literally false — **NEW-m4**.)

**(d) Does the `key_full` disjointness claim hold, and is the WP-2.0b constraint stated correctly?** The claim that `key_full` is the coarsest holds (BLOCKING 2 above). The quoted candidate list is exact — `wp20_dispatches.md:184-185` reads *"the full-turn 128-bit key per D-8, or the canonical move-list prefix"*, and neither is `canonical_form` — so *"a constraint that package inherits and not an observation this one makes"* (`:792-793`) is honestly framed. **But the ground given is wrong twice**: see **NEW-M8** (the property is cited outside the scope §2.1 gives it) and **NEW-m9** (the stated reason — *"two firings on one position reached two ways would count as two"*, `:787-788` — is an argument for `key_pos`, the transposition key, and says nothing about why the symmetry fold must also rule).

**MAJOR 7, judged on the dispatch's two questions.** **Is the declaration honest?** **YES.** `:869-879` states plainly that INVARIANT 2 is the one no test pins, gives the reason (*"no in-process Rust test observes the absence of a `Command::new` or an `Instant::now` on a path it does not take"*), and cites `docs/process.md`'s criterion rule and WP-2.0-M's identical refusal — which `wp20m_design.md` does make (`:590-599` in revision 3, carried into revision 4). **Is the stated evidence sufficient?** **Adequate, not strong.** *"Its evidence is the diff: `labels.rs` reaches `pistol-core`, `crate::transcript`, `crate::exchange` and `pistol_cli::corpus::emit`, and nothing else; a reviewer reads the module's `use` list"* (`:875-878`) is checkable by a human at review time and by nothing else, and the design does not claim otherwise. INVARIANT 5's byte-identity test is correctly named as the thing that would catch a clock read **that changed an output** — which is the operative half. I do not raise this as a finding.

**MAJOR 8, the ground.** *"parity and pistol-core agree on every legal prefix"* (`:213-214`) is **correct**, and I re-derived it: game rule 3 makes the mover alternate strictly by turn; every asked prefix is a turn boundary at `Phase::First`; a `Turn::Single` is either turn 1 or a game's last (`transcript.rs:369-376` refuses turns after a win), and a won prefix is never asked. So the mutation changes no output and removing it is right. **The replacement is not real** — **NEW-M4**.

## MINOR

| # | finding | disposition | evidence |
|---|---|---|---|
| 1 | `transcript.rs:124-131` refuses an *empty* path, not a whitespace one | **APPLIED** by deletion | the sentence is gone; `:598-600` points at WP-2.0-M's §4.2 instead (D-423) |
| 2 | *"three of the fields are whitespace-bearing"* | **APPLIED BUT INTRODUCED A NEW DEFECT** | the figure is now "Two" (`:600`) and is wrong again — **NEW-m1** |
| 3 | the fake `"bestmove <turn>"` quotation | **APPLIED** | `:319-321` now says *"which `bestmove_line` builds from the `BESTMOVE_PREFIX` and the turn"*, matching `crates/pistol-cli/src/report.rs:107-108` |
| 4 | `--out` "before the mode match" | **APPLIED (first limb); NOT APPLIED (second)** | `:92-93` now reads *"after the mode is parsed and before it is dispatched (`:104-107`)"* — verified: the `match words` ends at `arena.rs:100`, `outpath::claim` is at `:103`, dispatch at `:104-107`. **The second limb is untouched**: `:290-293` still cites only `SearchInfo::search_nodes`'s doc-comment (`info.rs:162-166`) for the gate-off fallback and not the site that enforces it (`crates/pistol-search/src/search.rs:513-514`) |
| 5 | `pub mod labels;` and the usage paragraph | **APPLIED** | `:94-96`; verified `crates/pistol-arena/src/lib.rs:47-69` is the `pub mod` block |
| 6 | `score_value`'s type differs by kind | **APPLIED** | `:259-264`, verified against `crates/pistol-search/src/score.rs:53-61` (`Eval(i32)`, `MateIn(u16)`, `MatedIn(u16)`), with a new loader refusal, a test and §9 row 9 |
| 7 | `key_pos`'s rendering unspecified | **APPLIED** | `:606-607` quotes `crates/pistol-core/src/zobrist.rs:71-72` verbatim (*"The 32 hex digits of the key, high half first"*), and `a_key_pos_that_is_not_thirty_two_hex_digits_is_refused_by_name` pins it |
| 8 | `capture_sha256` carried on trust | **APPLIED** | `:645-650` re-derives it from WP-2.0-M §5's three inputs (verified buildable: the format version and `go` line are capture header params, `experiment_sha256` is on the `Transcript`), with a test and a mutant |
| 9 | a report game with no capture records | **APPLIED** | §9 row 7 (`:826`), a test (`:912`) and a mutant (`:944`) |
| 10 | the three unit properties as `note` lines | **APPLIED** | `:620-630` moves them to keyed `param`s. Verified: `Fixture::note` writes `# {line}` (`crates/pistol-cli/src/corpus/emit.rs:52-57`), identical in form to `Fixture::new`'s title lines (`:20-28`); `param` writes `# param {name} {value}` (`:36-38`) |
| 11 | nothing pins the header | **PARTIALLY APPLIED** | a test and a mutant now cover the three unit params (`:905,941`) and the schema version and body digest were already covered. The loader rule is *"a header missing any of its params"* (`:641`) but only the unit params have a test: a mutation dropping `experiment_sha256`, `source_sha256` or the label `go` line from the header kills no registered test |
| 12 | INVARIANT 3's *"never their sum"* | **APPLIED** | `:849-850` adds *"the gate-off case excepted, where the sum and its first term are the same number and §2.4 says so"* |

**Also still unpinned from revision 1's closure section 11(c), unnumbered there and untouched here:** §9 row 2 (the CAPTURE's schema-version refusal — the registered `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` pins the CORPUS's, a different check); §9 row 11 (*"a captured `moves` prefix is not a legal game under pistol-core"*); and §5's write-side *"A field carrying a TAB refuses the run by name"* (`:602`) — the only arity/emptiness tests are loader-side.

---

# PART 2 — NEW DEFECTS

## BLOCKING

### NEW-B1 — `key_full`'s spelling drops `Player`, so the column §8's non-loosenable denominator counts over cannot identify a position

`wp20s_design.md:605-609`, under the heading whose whole point is that a loader must be able to predict the token:

> **TWO COLUMNS' SPELLINGS ARE FIXED HERE BECAUSE A LOADER CANNOT CHECK A TOKEN IT CANNOT PREDICT.** `key_pos` is `Key128`'s own `Display` … `key_seq` and `key_full` are their values rendered as turn tokens and **as `q,r` cells** respectively, space-joined, in the order the canonicaliser returns them.

`pistol_core::canonical_form` returns **`Vec<(Coord, Player)>`** (`crates/pistol-core/src/symmetry.rs:165`), and `Coord`'s `Display` is *"`"q,r"` — the stone token of the line protocol"* (`crates/pistol-core/src/coord.rs:136-141`). **A `q,r` cell carries no colour.** Rendering `key_full` as cells discards the `Player` half of every element the key holds.

`key_seq` has no such problem — `canonical_sequence` returns `Vec<Turn>` and `Turn`'s `Display` carries the pairing (`"q,r"` or `"q,r/q,r"`, D-5). The defect is specific to the column revision 2 added.

**Why it is BLOCKING, not cosmetic.** The rendering is provably non-injective on the key's own value space. Concretely at prefix `k = 5` (turn 4's boundary: P1 holds three stones, P2 two), two positions over the same five cells with different colour partitions are both legal and both asked. Their canonical forms differ; their rendered cell lists can coincide, because the render reads only the first component and the list is sorted by coordinate. Three things then break:

- **§7:712-716** — *"Two records with the same `key_full` are the same position up to transposition AND symmetry"* — is false of the column as spelled.
- **§8:785-789** — D-537's denominator, which the document is forbidden to loosen, counts over a key that merges distinct positions. (The direction is conservative, which is luck, not design.)
- **§12.3 residue 3 and §8:789-793** hand WP-2.0b a constraint whose object is under-specified.

The design's own loader has no way to catch it: `:632-641` checks `key_pos`'s hex digits and the token sets, and nothing about `key_full`'s arity or content.

**FIX.** One clause: `key_full` is rendered as its `(Coord, Player)` pairs — the colour spelled in a fixed two-token set — space-joined in canonical order; and §2.10's non-collision argument is restated over that spelling. Fix `:605` to say three columns, and `:600` to say three carry spaces (**NEW-m1**, **NEW-m2**).

### NEW-B2 — the `book` boundary is off by one against the column's own definition, and the correct reading is registered as the MUTANT

The column is defined twice, and both definitions are about PROVENANCE:

- `:157` — *"| 14 | `book` | whether **neither engine chose this position** (§2.9) |"*
- `:455-456` — *"what is different is that **the MOVES leading to it** were drawn from a book rather than chosen"*

The rule is about the NEXT move:

> `:447-450` — the arena asks an engine at turn index `at` only when `at >= opening_turns` (`crates/pistol-arena/src/replay.rs:137-138`). A capture record whose prefix length is `k` is the position BEFORE turn index `k`, so **the record is `book` exactly when `k < opening_turns`** — which is precisely "neither engine chose to be here".

**The tree, verified.** `crates/pistol-arena/src/replay.rs:137-138` is `for (at, recorded) in game.moves.iter().enumerate() { if at >= opening_turns as usize {` — so turn indices `0 … opening_turns-1` are book turns and the engine is asked from index `opening_turns` onward. `opening_turns` is *"How many turns every opening has"* (`crates/pistol-arena/src/openings.rs:39`), uniform across the book (`:85`).

**The arithmetic.** The position at prefix length `k` is reached by turns `0 … k-1`. Every one of those is a book turn iff `k - 1 < opening_turns`, i.e. **`k ≤ opening_turns`**. The rule says `k < opening_turns`. **They differ at exactly `k = opening_turns`** — the position reached by playing the whole book and nothing else, which the column's own definition says is `book` and the rule flags `no`.

The document's "so" does no work: it derives *"is the next move a book move"* from the arena's ask-boundary while the column is defined as *"was this position chosen"*. Both are defensible columns; the document ships one definition and the other rule.

**And the design locks in the wrong one.** `:922` registers `the_first_position_after_the_book_is_flagged_not_book` and `:949` registers *"the book boundary read as `k <= opening_turns`"* as the **mutation that must die** — so the reading the column's own definition requires is the thing the test suite is built to reject. One record per game is mislabelled relative to the column's stated meaning, and a trainer weighting positions by how a game reached them (the design's own stated use, `:455-457`) gets the first fully-book position counted as engine-chosen.

**FIX, one sentence, and the operator can weigh which side.** Either (i) keep the boundary and redefine the column as *"whether the move played from this position came from the book"*, fixing `:157` and `:455-456`; or (ii) keep the definition and make the boundary `k <= opening_turns`, swapping the test name and the mutant. Not both.

## MAJOR

### NEW-M1 — the turn-zero sentinel is reversed at two sites of four

§2.10 decides (`:470-476`): *"**THEY ARE WRITTEN AS A SINGLE `-`, AND NO FIELD OF THIS RECORD IS EVER EMPTY.**"* §5 repeats it (`:602-603`, *"by §2.10 **no field is ever empty**"*), the loader refuses *"**any field that is empty**"* (`:638`), and a test and a mutant pin it (`:903,923,952`).

Two sites still say the opposite:

- `:146` — *"| 3 | `moves` | the move list (D-6), the turn tokens alone; **empty at `turn` zero** (§2.10) |"*. This is the schema table — the face of the record, the first thing an implementer reads — and it says the field is empty while pointing at the section that says it is `-`.
- `:823` — §9 refusal row 4: *"a field is **empty where §2.10 does not allow it**"*, which presupposes §2.10 allows an empty field somewhere. §2.10 allows none.

Rows 4 and 6 of the table also say nothing about turn zero at all, so `key_seq` and `key_full` — the other two sentinel columns — are undocumented on the face of the schema.

**This is D-544's second defect exactly**: a reversal applied to §2.10, §5, the loader and the test table, and not to the schema table or the refusal table. **FIX.** `:146` → *"the turn tokens alone; `-` at `turn` zero (§2.10)"*, the same on rows 4 and 6; `:823` → *"or a field is empty"*.

**The `-` sentinel itself is sound.** Its non-collision claim (`:471-473`) holds: `moves` and `key_seq` render as turn tokens, each *"one or two `q,r` cells"* (D-5, `crates/pistol-core/src/coord.rs:136-141`), never the single character; `key_full` as cells, likewise. `key_pos` genuinely needs no sentinel — `GameState::key` on a new game is `board.stones_key() ^ context_key(P1, First)` (`crates/pistol-core/src/state.rs:134-136`), a full 32-digit key.

### NEW-M2 — §12.2 says WP-2.0-M "did not have" the throughput obligation; WP-2.0-M assigned it in the commit immediately below this one, and the requirement is now discharged twice

`:1004-1006`:

> **Revision 1 had it in no section and no deferral list, and WP-2.0-M did not have it either** — the "falls between" defect on a requirement the split's own governing text names.

`docs/experiments/wp20m_design.md:864-872`, **at HEAD**:

> **AND THE DISPATCH'S THROUGHPUT OBLIGATION IS THE PILOT'S, NAMED HERE BECAUSE REVISION 3 LEFT IT IN NO PACKAGE AT ALL.** … The shape is: one label `go` at the label budget per asked position, plus one `newgame` per asked position, whose memset cost §12 names and does not guess. **The magnitudes — games per hour and labels per hour — are measured in the pilot and registered there.**

I dated it: `git show 1157dae:docs/experiments/wp20m_design.md | /usr/bin/grep -i throughput` returns **nothing** (exit 1); `git show a9a4a3a:…` returns it at `:864-872`, with the freeze table's own row *"| §11 | the throughput obligation assigned | m12 |"* at `:102`. `a9a4a3a` is committed at 19:18:21 and `f96593b` at 19:29:35 — **eleven minutes earlier, one commit below**. The author knew: `:1012` cites WP-2.0-M's refusal to guess the memset cost in the same paragraph.

**Two consequences.** (1) A claim about a governing document at the reviewed revision that the document contradicts — D-545's class, the third instance in this arc and the second in this document's own history (MAJOR 1 and MAJOR 12 were the first two). (2) The requirement is now **stated in both designs**, in different words, each claiming to own it — which is D-423's *"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING"*, on the requirement whose falling-between was BLOCKING 5. §0:68-71 promises the opposite discipline: *"WHAT IT INHERITS AND MAY NOT RE-OPEN … not restated here (D-423)."*

**FIX.** Delete the false clause. Either point §12.2 at `wp20m_design.md` §11 and add only what is genuinely this package's (pass 1's cost, the asked-position count, this transform's own cost), or move the whole shape here and have WP-2.0-M point back — one home, and the other section pointing.

### NEW-M3 — §3 says INVARIANT 4 pins byte-identity; INVARIANT 4 and its test pin per-game node counts

`:530-532`: *"**INVARIANT 4** pins that the SPRT report is **byte-identical** across the change."*

INVARIANT 4 (`:851-853`): *"**Splitting `totals_of` changes no output**: `totals_of`'s three lookups stay load-bearing, and the SPRT path **still bills each game's compute** from the totals line."*

Revision 1's INVARIANT 4 did say byte-identical (`1157dae:542-544`) and its test was `splitting_totals_of_leaves_the_sprt_report_byte_identical`. Revision 2 weakened both — the test is now `the_sprt_reports_per_game_node_counts_survive_the_totals_of_split` (`:895`) — **and left §3's sentence at the old strength**. Per-game node counts are strictly weaker than byte-identity of the report.

The weakening may well be the right call (a byte-identity assertion over an SPRT report needs a run, and the design's whole point is that no test here needs an engine). What is not right is a document that answers "what does INVARIANT 4 pin" two ways. **FIX.** Restate `:530` to what the invariant says, and say in one clause why the node counts are the checkable form.

### NEW-M4 — the mutant that replaced the no-op cannot die at its named test, and `to_move` now has no test at all

`:221-224`:

> **WHAT REPLACES THE MUTANT IS ONE THAT CHANGES AN ANSWER**: deriving `to_move` from the record's `turn` column WITHOUT replaying, which diverges the moment a capture record's `turn` and `moves` disagree — **a disagreement §9 now refuses by name**, and which the mutation would silently absorb.

Registered at `:948` against `a_record_whose_turn_and_moves_disagree_is_refused_by_name`.

**The mutation cannot kill that test.** The refusal is §9 row 5 (`:824`), a comparison between the record's `turn` field and its `moves` prefix length. Mutating the *source of the `to_move` column* does not remove or weaken that comparison. Under the mutation the transform still refuses the disagreeing record by name, and the test still passes. The design's own clause names the reason: the disagreement is the one input on which the two derivations differ, and it is refused before a record is emitted — so **there is no input on which the mutation changes an output**, which is the same property that killed the parity mutant one paragraph above.

**And revision 1's `side_to_move_comes_from_pistol_core_and_not_from_turn_parity` was removed** (`1157dae:585`), so column 7 now has **no test of its value anywhere** in the table at `:885-923`. The loader's token-set check (`:639`) has no named test either. A mutation writing the wrong side into `to_move` kills nothing: the round-trip test is write→read→compare and is self-consistent under a wrong value.

**FIX.** Either drop the mutant and say so (the honest answer, as §2.2 already does for parity: `to_move` is a rule-2 choice with no behavioural difference), or register a mutation that changes an output — deriving `to_move` from the record's `turn` **and** relaxing row 5's refusal is two changes, so the real single-change mutant is elsewhere. Either way, register one test that pins the column's value against a fixture whose expected side is computed from pistol-core.

### NEW-M5 — the key rename left `key_seq` with no test and no mutant

Revision 1 pinned its symmetry key: `two_positions_alike_up_to_a_symmetry_are_two_records_sharing_a_key_sym` (`1157dae:584`). Revision 2 renamed the column `key_sym → key_seq` (`:178-179`) and **replaced** that test with `two_positions_alike_up_to_a_symmetry_share_a_key_full_and_not_a_key_pos` (`:920`).

`/usr/bin/grep -n "key_seq" docs/experiments/wp20s_design.md` returns eleven lines; **not one is a test name or a mutant.** So:

- INVARIANT 9 (*"carries the three keys that let a consumer fold"*, `:864-865`) is pinned for two of its three keys.
- §7:706-708's claim — two records with the same `key_seq` are the same game prefix up to a lattice symmetry, *"has no false positives"* though it has false negatives — is pinned by nothing.
- A mutation computing `key_seq` from `canonical_form` instead of `canonical_sequence`, or from the raw move list, kills no registered test. (Only a mutation that *drops* the column dies, at the arity check.)

This is the rename half of D-544's second defect: the name was carried to every prose site and dropped from the test table. **FIX.** One test — two prefixes that are symmetry images share a `key_seq` while two transpositions of one stone set do not — and one mutant.

### NEW-M6 — §8 names the direction of its own bias backwards, and derives the "FLOOR" from the inversion

`:779-783`:

> **AND THE DIRECTION OF ITS BIAS IS NAMED, BECAUSE A RULE THAT HIDES ITS OWN ANTI-CONSERVATISM IS NOT A GUARD.** The bound's lower end is the **LARGEST effect size the arc licenses, so it yields the SMALLEST minimum**. **The number this rule produces is therefore a FLOOR**…

**The arithmetic is the other way round.** The alternative is the bound's lower end, **0.857** (`matrix_stage3_detector.md:613`); the bound's other end is **1.000** (`:614`). Against a fixed null the required sample size falls as the null-to-alternative gap grows. With the trigger-rich null 0.571, the lower end gives a gap of 0.286 and the upper end 0.429 — **so the lower end is the SMALLEST effect the arc licenses and yields the LARGEST minimum.** The same holds if the two frames are compared whole: 0.571→0.857 is a gap of 0.286 against 0.333→1.000's 0.667.

The choice is therefore **conservative**, which is the right choice — the guard exists to stop a successor registering a minimum small enough to open round 3 on thin evidence. The floor rule (*"may register a larger minimum with grounds and may never register a smaller one"*) survives and is safe. What does not survive is the paragraph's account of itself: the section whose heading is *"a rule that hides its own anti-conservatism is not a guard"* misidentifies which way it leans, in the one rule D-537 forbids a successor to loosen.

**FIX.** *"The bound's lower end is the smallest effect the arc licenses, so it yields the largest minimum: the rule is conservative by construction, and its number is a floor — a pre-registration may register a larger minimum with grounds and may never register a smaller one."*

### NEW-M7 — §8's two inputs come from a two-frame measurement and the rule names no frame, so four minima are still available to a successor

`:754-762` names the null as *"the incumbent recall … registered in the closed detector arc and cited there rather than restated here"* and the alternative as *"the lower end of the arc's measured column bound"*. The closed arc registers **two frames**, not one (`matrix_stage3_detector.md:612-615`):

| frame | best written ordering (null) | column bound (alternative) |
|---|---|---|
| trigger-rich (n = 14) | **0.571** | **0.857** |
| out-of-sample band 15 (n = 3) | **0.333** | **1.000** |

Nothing in §8 says which. A successor may pair 0.571 with 0.857 (the natural, frame-consistent reading), or 0.333 with 0.857 (which is what *"the lower end of the bound"* says literally, against the OOS null), or either with 1.000 by arguing the bound is the pair's maximum. **Four gaps: 0.286, 0.429, 0.524, 0.667 — and the minimum scales roughly as the inverse square of the gap, so the choice moves it by more than fivefold.** §8's own closing claim is *"Nothing in it is a choice a successor makes after seeing data"* (`:766`).

**A second under-specification in the same sentence.** *"a **two-proportion** test at that level and power separates those two recalls"* (`:764-766`) needs two arm sizes and solves for one. If the incumbent arm is the arc's own 14 firings, the test is bounded by 14 and can never reach power 0.95 against a gap of 0.286 — the rule would be unsatisfiable. If the incumbent recall is a registered constant, the test is a **one-sample** binomial test and the name is wrong. A successor must decide, which is the thing D-537 forbids.

**FIX.** Name the frame (trigger-rich is the one the arc's own gap argument is about, `matrix_stage3_detector.md:645-646`), and say whether the incumbent enters as a fixed `p0` or as a second arm with its registered `n`.

### NEW-M8 — §8 imposes `key_full` on WP-2.0b on a property §2.1 explicitly says does not hold there

§2.1 scopes the claim (`:198-199`): *"**On this corpus's records, and only on them**, `key_full` is a total position identity that folds both equivalences."*

§8 then uses it outside that scope (`:785-793`): *"`key_full` folds transposition and symmetry together and **loses nothing on an asked position (§2.1)**. **WP-2.0b's identity form must be consistent with it**."*

**WP-2.0b's rows are not this corpus's records and are not asked positions.** A census row is pushed inside `Run::solver_verdict`'s exit — *"One trigger firing, described by the O(1) facts a **per-node** detector could read at it"* (`crates/pistol-search/src/census.rs:1-9`, pushed at `crates/pistol-search/src/pvs.rs:737-751`) — an arbitrary node inside the search tree, mid-turn included. Read literally, §2.1 tells WP-2.0b's implementer that the key §8 forces on them is **not** a total identity at their sites.

**The property is in fact true there** — I checked: `GameState::key`'s doc says *"for an **ongoing game** the stone count fixes the turn, the phase and the mover together"* (`crates/pistol-core/src/state.rs:129-133`), with no boundary restriction, and the stone count is an invariant of `canonical_form` because `transform` preserves `Player` (`symmetry.rs:148-155`). **So §2.1's hedge is what is wrong, not §8's use.** But the document as written contains a scope claim and a use of it that contradict, on the one section D-537 forbids loosening and on a constraint imposed on another package.

**FIX.** Delete *"and only on them"* at `:198`, and ground the property where pistol-core grounds it: on an **ongoing** position, which every asked position and every searched node is.

### NEW-M9 — §11 says every committed config has the solver off the search path; three do not

`:962-968`:

> **THE VACUITY THIS PACKAGE MUST NOT WALK INTO** … including the solver spelling, **which no engine in this workspace can produce because every committed config has the solver off the search path** (`configs/gate_v0.toml:94`), and which a synthetic totals line produces for free.

`LC_ALL=C /usr/bin/grep -rn "on_search_path" configs/ | LC_ALL=C sort` returns three committed configs with the gate **on**:

- `configs/bench_wp18c_solver_on.toml:45` — `on_search_path = true`
- `configs/gate_staged_solver_v0.toml:47` — `on_search_path = true`
- `configs/play_staged_solver_v0.toml:75` — `on_search_path = true`

The cited line is real (`configs/gate_v0.toml:94` is `on_search_path = false`) and the universal built on it is false. This is the same shape as the previous round's MAJOR 1: **a sound conclusion resting on manufactured authority**, at a place the citation checker cannot see because the checker verifies the one line quoted and not the "every" in front of it. The conclusion (a synthetic totals line exercises the solver spelling for free, so no engine is needed) survives untouched on its own independent ground.

It also sits badly beside §2.5, which reconciles `depth_turns`'s two meanings precisely because the solver-proof record class is non-empty — a class only a solver-on config can produce.

**FIX.** Delete the false universal. *"…which a synthetic totals line produces for free, and which no fixture needs an engine to obtain."* Optionally name the three configs that can produce it, since the pilot's config is not yet written and the class is real.

### NEW-M10 — the corpus manifest row has a test in no test table, no invariant, and no reconciliation with the capture rows in the same file

`:999` registers `a_labels_run_prints_a_corpus_manifest_row_naming_its_digests`. `/usr/bin/grep -n` for that name returns **exactly one line in the document** — §11's test table (`:885-923`) does not contain it, no invariant of the ten covers the manifest row, and no mutant is registered for it. So the rule BLOCKING 6 forced into existence is the one rule in the document with a test that is not in the register and an obligation nothing pins.

**And the two row shapes are not reconciled.** WP-2.0-M defines the file as *"`docs/label_corpus_manifest.md` holds **one row per capture**"* with six fields (`wp20m_design.md:934-944`); this design adds a differently-shaped row per **corpus** to the same file (`:989-998`) and says only that it *"is added to `docs/label_corpus_manifest.md` in the commit that records the run"*. Nothing says how a reader tells the two apart, whether the file gains a second table, or what the column order is. `docs/label_corpus_manifest.md` does not yet exist (declared to the checker via `--proposes`), so the collision is entirely in the two designs' hands and neither resolves it.

**FIX.** Add the test to §11 against a new invariant (or against INVARIANT 6, widened), add a mutant, and state in one sentence how the corpus rows sit beside the capture rows — a second table under its own heading is the cheap answer, and it is WP-2.0-M's file to be told about.

## MINOR

1. **NEW-m1** — `:600`: *"Two of these columns carry spaces — `moves`, and `key_seq` rendered from a turn list"*. **Three do**: `:608-609` makes `key_full` *"space-joined"* eight lines later. MINOR 2's exact shape, reintroduced by the column MINOR 2's neighbour added.
2. **NEW-m2** — `:605`: *"**TWO COLUMNS' SPELLINGS** ARE FIXED HERE"*, followed by three (`key_pos`, `key_seq`, `key_full`). A leftover from the two-key revision.
3. **NEW-m3** — `:528`: *"**A field map** adds no lookup to `totals_of` at all"*, sixteen lines before *"THE RETURN TYPE IS THE WORD LIST AND NOT A KEY-VALUE MAP"* (`:512-513`). The argument is unaffected by the substitution; the phrase is a fix applied to three of four sites.
4. **NEW-m4** — `:748-749`: *"**These are the only two numerals this document carries**"*. My sweep (Part 1, MAJOR 3(c)) finds column indices, refusal-row indices, invariant indices, section numbers and three cited code facts. True of the class D-483 governs, false as written — and the document has now made a wrong claim about its own record three times (MINOR 2, NEW-m1, this).
5. **NEW-m5** — `:34`: *"AUTHORED UNDER THE SAME DISCIPLINE AS **WP-2.0-M REVISION 3**"*. WP-2.0-M is at **revision 4** in the commit below this one (`wp20m_design.md:3`).
6. **NEW-m6** — `:611-613` puts the corpus's `capture_sha256` under `param`. `Fixture::derived`'s own doc is *"A value the extraction computed. **Never a `param`**: a reader has to be able to tell a choice from a measurement"* (`crates/pistol-cli/src/corpus/emit.rs:40-44`), and §5 has the transform **re-derive** it (`:645-650`). WP-2.0-M puts it under `derived` (`wp20m_design.md:453-458`).
7. **NEW-m7** — `:614-618` drops the arena version from the corpus header *"for the reason WP-2.0-M's §5 gives once and this document points at"*. WP-2.0-M's §5 argues `arena_version` out of the **digest** (`wp20m_design.md:496-505`) while §4.3 keeps it as a capture **header param** (`:453-456`). The reason transfers; the attribution does not, and the two files' headers now differ for a reason neither states.
8. **NEW-m8** — `:1013-1015`: *"a **linear** pass over the capture with one pistol-core replay per record"*. The replay of prefix `k` is `O(k)`, so the transform is quadratic in game length (≈465 `make_turn` calls for a 30-turn game). Trivial in absolute terms, and wrong in a paragraph whose whole job is to state the cost shape.
9. **NEW-m9** — `:787-788`: *"**The coarsest must rule**, or two firings on one position **reached two ways** would count as two"*. "Reached two ways" is transposition, which `key_pos` already folds. The reason given argues for `key_pos`; the conclusion is `key_full`. The symmetry half of the fold gets no reason.
10. **NEW-m10** — `:1010-1011`: *"the `newgame` is a memset of the whole transposition table, whose size the committed seat sets"* — a claim about the tree with **no `path:line`**, against this document's own header rule (`:35-36`). The fact is right and lives at `wp20m_design.md:889-891` (and at `crates/pistol-engine/src/instance.rs:73-76` per WP-2.0-M's own site list).
11. **NEW-m11** — column 2 is named `turn` and holds the prefix length `k` (`:145`, §9 row 5 at `:824`). `GameState::turn()` at prefix `k` is `k + 1` (`crates/pistol-core/src/state.rs:111-115`, counting from `FIRST_TURN`), and the corpus carries `depth_turns` — a real turn count — three columns later. A consumer will read the two as the same unit.
12. **NEW-m12** — §9 rows 2, 4 and 11 have no registered test (rows 2 and 11 were named unnumbered in revision 1's closure and are untouched). §5's write-side TAB refusal (`:602`) likewise.
13. **NEW-m13** — the mutant *"the three unit params dropped from the header"* (`:941`) is paired with `a_corpus_missing_one_of_its_three_unit_params_is_refused_by_name` (`:905`), a **loader** test. Dropping the params on **write** leaves the loader check intact, so that test still passes; what dies is the round-trip test. Mis-paired.
14. **NEW-m14** — §4 widens `RecordedGame` with `forfeit_by` (`:568`) and §2.8 then says it is *"recoverable and **not useful**"* (`:396-399`). A second fatal `value()` lookup is added for a field nothing reads. (`RecordedGame` already carries `forfeit` off the `end` field, `crates/pistol-arena/src/transcript.rs:307`.)
15. **NEW-m15** — the dispatches preamble (`wp20_dispatches.md:12-18`) says the third's mutant list differs from the first's and names the seed qualification. It also **drops** *"census direction collapsed -> its test dies"* (present at `:114-115`, absent at `:318`) and adds a cold-label mutant. The preamble's claim holds as stated; the file whose purpose is to let a successor tell the lists apart does not name that difference.
16. **NEW-m16** — §8's ground is that *"a detector round is a decision of the same kind and at the same cost as the SPRT decisions this project already registers at those error rates"* (`:745-748`). `configs/random_openings_v1.toml:47-52` records that the nominal pair does not deliver those rates in this arena: *"at 500 pairs, `[sprt]` bounds of elo0 = 0, elo1 = 25 and alpha = beta = 0.05 achieve a **measured alpha of 0.030 and a power of 0.569**"*. The convention's standing is unaffected; the "same error rates" half of the ground is not what the tree records.
17. **NEW-m17** — no invariant states how `score_value` is read. INVARIANT 3 (`:846-850`) covers `score_kind`'s three tokens and the node columns; the `(tag, number)` pair — the thing BLOCKING 1 was about — is stated only in §3 prose and pinned by two tests mapped to INVARIANT 4 (`:898`), an invariant about `totals_of`'s outputs not changing.
18. **NEW-m18** — MINOR 4's second limb, carried forward: `:290-293` still cites `SearchInfo::search_nodes`'s doc-comment for the gate-off fallback and not `crates/pistol-search/src/search.rs:513-514`, the overwrite that enforces it.

## The citation check, and AUTHOR DEBT

`python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` → **104 citations checked, 0 unreproduced.**

**I hand-verified thirty-one citations' CONTENT, including every one new in revision 2.** New in revision 2 and verified to say what the document says: `configs/arena_smoke_v0.toml:66-67` ✓; `configs/gate_v0.toml:94` ✓ (the line is right, the universal built on it is not — NEW-M9); `crates/pistol-arena/src/bin/arena.rs:103` ✓ and `:94-99` ✓; `crates/pistol-arena/src/lib.rs:41-45` ✓ and `:47-69` ✓; `crates/pistol-arena/src/game.rs:103-107` ✓; `crates/pistol-arena/src/record.rs:16-22` ✓; `crates/pistol-arena/src/replay.rs:137-138` ✓ (the line supports the ask-boundary, not the book gloss — NEW-B2); `crates/pistol-arena/src/transcript.rs:17-18` ✓, `:39-40` ✓, `:152-158` ✓, `:359-379` ✓; `crates/pistol-arena/src/exchange.rs:163-168` ✓, `:176-188` ✓; `crates/pistol-cli/src/report.rs:20-29` ✓, `:55-61` ✓, `:62-81` ✓, `:69-78` ✓, `:82-84` ✓, `:105-108` ✓; `crates/pistol-cli/src/corpus/emit.rs:19-28` ✓; `crates/pistol-core/src/lib.rs:85-90` ✓; `crates/pistol-core/src/state.rs:128-133` ✓; `crates/pistol-core/src/symmetry.rs:157-165` ✓, `:181-193` ✓; `crates/pistol-core/src/turn.rs:34-45` ✓; `crates/pistol-core/src/zobrist.rs:70-76` ✓; `crates/pistol-search/src/info.rs:133-138` ✓, `:176-178` ✓, `:259-261` ✓; `crates/pistol-search/src/score.rs:53-61` ✓; `crates/pistol-search/src/search.rs:785-793` ✓. Also re-verified from revision 1: `info.rs:162-166`, `:167-172`, `:237-245`, `:250-267`, `protocol.rs:172-174`, `dedupe.rs:12-26`, `conclusion.rs:37-52`, `replay.rs:16-19`, `report.rs(arena):41-50`, `emit.rs:12-100`, `:92-99`, `:102-118`.

**One citation is weak rather than wrong**: `crates/pistol-core/src/turn.rs:25-29` (`:212`) is `Phase::index`'s match arms, cited for *"every asked position is a turn boundary at `Phase::First`"* — the citation shows `Phase` exists; the claim is WP-2.0-M's INVARIANT 2. Carried over from revision 1.

**AUTHOR DEBT the checker could have caught: none.** Every `path:line` in the document reproduces. **The debt the checker could NOT catch is the whole of Part 2**: three false universals about the tree and the governing documents (NEW-M2, NEW-M9, and `:1004-1006`'s claim about WP-2.0-M), all of them bare prose or a universal in front of a real citation. Two of them (`:1010-1011`'s memset claim, NEW-m10; the WP-2.0-M throughput claim) carry no line number at all, which is precisely the hole MAJOR 1 exposed last round and which the header's own rule (`:35-36`) was supposed to close.

## Closure over the ten invariants, thirty-seven tests and twenty-five mutants

**(a) Invariants with no test**: **one, declared** — INVARIANT 2 (`:869-879`), honestly and with its evidence named. Everything else maps.

**(b) Tests pinning nothing / mis-mapped**: three. `fields_of_gives_the_word_after_score_and_the_word_after_that` and `fields_of_reads_a_captured_line_that_has_no_time_field` are mapped to INVARIANT 4 (*"splitting `totals_of` changes no output"*) and pin `fields_of`'s **new capability**, which no invariant states (NEW-m17). `a_capped_game_and_a_forfeited_game_are_distinguishable_in_the_corpus` is mapped to INVARIANT 8 (the cross-check) and pins §2.7's two-column decision, which no invariant states.

**(c) Rules with neither test nor mutant**: `key_seq`'s semantics (NEW-M5); `to_move`'s value (NEW-M4); the corpus manifest row (NEW-M10); §9 rows 2, 4 and 11; §5's write-side TAB refusal; three of the header's five params (MINOR 11, partial).

**(d) Mutants that cannot die**: **one, and it replaced one** — `to_move` taken from the record's `turn` column (NEW-M4). One more is mis-paired rather than dead (NEW-m13). The rest I checked individually and they are real; the two best are *"the outcome relation gated on `a_is_p1`"* (dies at `the_outcome_check_holds_when_engine_b_takes_seat_one`) and *"the score read as the word after its key alone"* (dies at both its named tests).

**(e) Tests that would pass vacuously**: **none that I could find.** The one revision 1 registered is gone, and `the_derived_outcome_agrees_with_the_reports_own_result_field`'s fixture rule (`:970-974`) is stated with the right ground now that §2.7 claims the check for what it is.

## The dependency on WP-2.0-M, which is under review and has not passed

| # | what this document leans on | site here → there | what breaks if WP-2.0-M's review forces a change | risk |
|---|---|---|---|---|
| 1 | the capture header carries `source_sha256` | `:123-129` → `wp20m` §4.4 (`:475-481`) | §1's pairing check; §9 row 1. §4.4 is a verbatim lift of a passed paragraph | **low** |
| 2 | the normalisation removes ` nps <n> time <n>` and nothing else | `:486-489`, `:401-403` → `wp20m` §4.1 (`:373-398`) | §3's entire premise: if `time` stayed, `totals_of` would read a captured line and the split has no reason to exist. Lifted from a passed paragraph | **low** |
| 3 | the record's TAB grammar and five-field arity | `:598-603` → `wp20m` §4.2 (`:400-441`) | §5's TAB choice (which points rather than restates), §9 row 4, and the loader. §4.2 is **new in revision 3, unfrozen, and never passed** | **high** |
| 4 | the asked set: `k` from zero to `len` less a winning last turn; `k = 0` asked as bare `position start` | `:463-468`, `:1012-1014`, and silently by columns 2, 3, 7 | §2.10's whole existence, INVARIANT 1's one-record-per-capture-record claim, §6's no-sampling argument, §12.2's position count | **medium** — §2 is frozen but its range enumeration is a revision-3/4 edit |
| 5 | the capture schema version | `:821` | §9 row 2 only | low |
| 6 | `totals_of` stays the recogniser and rises to `pub(crate)` | `:541-544` → `wp20m` §8 (`:643-687`) | §3's "one recogniser" argument and the visibility both halves need. **§8's own rationale still says WP-2.0-S *"adds fields to one parser"* and *"adds `score` and `pv` as non-fatal `Option`s"* (`:651-652`, `:685-686`), which after this revision it does not do** — so a reviewer fixing WP-2.0-M's text could edit §3's premise out from under it | **high** |
| 7 | `capture_sha256` is a digest over exactly three inputs | `:645-650` → `wp20m` §5 (`:485-543`) | §5's re-derivation, §9 row 3, `a_capture_whose_header_identity_is_not_its_own_inputs_is_refused_by_name`, and its mutant. **§5 was REVERSED in revision 4** (the arena version taken out) and is new and unfrozen; restoring a fourth input breaks all four | **high** |
| 8 | `docs/label_corpus_manifest.md` and its row discipline | `:982-998` → `wp20m` §13(a) (`:934-944`) | §12.1 entire — and the two row shapes already collide (NEW-M10). New in revision 3/4 | **high** |
| 9 | the throughput obligation's owner | `:1004-1006` → `wp20m` §11 (`:864-872`) | already broken: the claim is false and the requirement is discharged twice (NEW-M2). New in revision 4 | **high** |
| 10 | *"the solver fields appear exactly when the engine printed them"* | §2.4 `:285-303` → `wp20m` §4.1 (`:384-388`) | the all-six-or-none refusal and both node columns | low |

**Four of the ten leans are on material that is new in WP-2.0-M revisions 3 and 4, unfrozen, and never passed by any reviewer.** That is the structural risk this package carries into implementation, and the document does not name it: §0:68-71 lists what it inherits without saying that most of it is under adjudication, and §1:114 names the outstanding review only as a reason not to add an output to another package's mode.

---

# PART 3 — THE VERDICT

## **FAIL** — 2 BLOCKING · 10 MAJOR · 18 MINOR, all new.

Prior-round disposition: **6 of 6 BLOCKING applied** (four cleanly, two with new defects); **11 of 12 MAJOR applied or partially applied** (MAJOR 4 moot); **11 of 12 MINOR applied**, one partial, one limb of MINOR 4 untouched. The work was real. The failure is the fix round's own output.

## "Could an implementer build from this without deciding something the design should have decided?"

**NO.** They would have to decide:

1. **Whether `key_full` carries the stone colours** — §5 says `q,r` cells and `canonical_form` returns `(Coord, Player)` pairs (**NEW-B1**). §8's disjointness count turns on this.
2. **Which of the two `book` definitions to implement** — the column's gloss says `k ≤ opening_turns`, the rule says `k <`, and the mutant table registers the gloss's answer as the mutation (**NEW-B2**).
3. **Whether the turn-zero record's three columns hold `-` or are empty** — §2.10 and §5 say `-`; the schema table and §9 row 4 say empty, and the loader refuses empty (**NEW-M1**).
4. **What INVARIANT 4's test asserts** — byte-identity of the SPRT report, or per-game node counts (**NEW-M3**).
5. **How to make the `to_move` mutant die**, and what test pins the column at all (**NEW-M4**).
6. **What pins `key_seq`** (**NEW-M5**).
7. **Which frame §8's null and alternative come from, and whether the test is one-sample or two-proportion** (**NEW-M7**) — this one is not the implementer's, but it is a successor's, and D-537 says it must not be.
8. **Where the corpus manifest row lives inside a file WP-2.0-M defines as one row per capture, and which test table its test belongs to** (**NEW-M10**).
9. Minor but real: whether `capture_sha256` is `param` or `derived` (**NEW-m6**); whether the corpus header carries the arena version when the capture header does (**NEW-m7**); which mutation kills the unit-param test (**NEW-m13**).

Everything else — the parser split and the two-token score read, the three keys and what each folds, the node pair and its gate-off fallback, the all-six-or-none solver refusal, `depth_turns`'s two meanings and their discriminator, the outcome relation, the seat-blind cross-check claimed for what it is, the no-dedup and no-seed policies, the twelve refusal rows, the TAB record and the loader, α and the power — **is decided, and decided well**.

## The strongest attack that did not land

**I set out to show that `key_full` cannot rule §8's disjointness, because `canonical_form` reads stones and not the mover, and a census firing is a mid-turn node where the mover is not recoverable.** If that had held, the document's central new decision — the third key and the denominator it settles — would have been unsound at the one place D-537 forbids loosening, and BLOCKING 2's remedy would have been worse than the defect.

**It failed, and the tree is why.** `GameState::key`'s own doc hedges to *"for an **ongoing game** the stone count fixes the turn, the phase and the mover together"* (`crates/pistol-core/src/state.rs:129-133`) — **not** to a turn boundary. I re-derived it: turn 1 places one stone and every later turn two (CLAUDE.md rule 3), so the stone counts `0, 1, 2, 3, 4, …` map one-to-one onto `(turn, phase, mover)` triples, mid-turn included. And the count is an invariant of `canonical_form`, because `transform` maps `(cell, player) -> (symmetry.apply(cell), player)` and preserves colour (`crates/pistol-core/src/symmetry.rs:148-155`). **So `key_full` is a total identity on every ongoing position — asked, searched, or mid-turn — and it really is the coarsest of the three.** The attack turned into NEW-M8: the document's own hedge is what is wrong, and deleting three words fixes it.

**A second attack also failed.** I tried to show that the `-` sentinel collides, since three columns share it and one of them (`key_full`) is a cell list. It cannot: every value those columns can hold is a space-joined list of tokens each containing a comma (`crates/pistol-core/src/coord.rs:136-141`, D-5), and `-` is one character with no comma. §2.10's argument is sound.

**A third.** I tried to break the claim that the `totals_of` split leaves the SPRT path byte-identical, on the theory that a word list changes the pairing. It does not: the current implementation **already** splits to words and looks up "the word after the key" (`crates/pistol-arena/src/exchange.rs:176-188`), so `fields_of` is a pure extraction of the first two statements and the three lookups are the same three expressions over the same `Vec<&str>`. That is the fix's real strength and it is better than the map revision 1 proposed.

## What I could not settle by reading, and the run that would

1. **Whether the `fields_of`/`totals_of` split compiles and leaves `arena_smoke` byte-identical.** The elision (`fn fields_of(line: &str) -> Option<Vec<&str>>`, one input lifetime, every element a subslice of `line`) and the three-expression identity both say yes. **The run**: `cargo test --workspace --locked` plus `tools/arena_smoke.sh` (gate 15) on the split, comparing verdict blocks. Refused here.
2. **Whether `crates/pistol-arena/src/bin/arena.rs` admits a fourth arm under gate 17.** `wc -l` is **283**; `tools/file_justification_check.sh:65` sets `SOFT_CAP=300`; WP-2.0-M extracts `USAGE` (`arena.rs:16-58`, 43 lines) to `usage.rs`. Arithmetic says yes with room. **The run**: `tools/file_justification_check.sh` at the post-implementation revision. I did not run it.
3. **Whether `a_rerun_over_one_capture_and_report_is_byte_identical` can kill its ordering mutant in one process.** It turns on whether two `HashMap`s in one thread get different `RandomState` keys. **The run**: the mutant in a `git worktree add --detach` with its own `CARGO_TARGET_DIR`.
4. **Whether the `key_full` cell-only rendering actually collides on a reachable pair.** I can show the rendering is non-injective on `canonical_form`'s value space by construction; whether two *specific* legal prefixes collide depends on which of the twelve images each selects. **The run**: a scratch test enumerating prefixes of depth 5 from two fixture games and comparing rendered `key_full` strings against the full `(Coord, Player)` forms. NEW-B1 does not depend on that run — the column loses information the key carries either way — but the run would price it.
5. **Whether any pilot or committed config puts `solver.on_search_path = true` on pass 1**, which is what makes §2.5's second record class non-empty in practice. Three committed configs can (NEW-M9); the pilot's config does not exist. **The run**: `LC_ALL=C /usr/bin/grep -rn "on_search_path = true" configs/ | LC_ALL=C sort` at the pilot's governing revision, against the config pass 1 names.

---

## One paragraph for the operator

The fix round did its job on the findings and then broke five new things applying them, which is the third round in a row this arc has spent that way — and it is the same three shapes every time: a rule reversed at some sites and not all, a claim about a governing document that document does not make, and a mutant registered against something that cannot change. Two of the new defects (`key_full`'s spelling, the `book` boundary) are one sentence each to fix and sit in the two decisions the previous BLOCKINGs forced into existence, which is to say the document is defective exactly where it was most recently repaired. **D-545 named the authoring method as the defect; this round is that diagnosis reproduced under supervision**, and I do not think a third fix round by the same method reaches a different place. The design's substance — the parser split, the three keys, the node pair, the outcome relation, the refusal table, the census rule's inputs — is sound and is worth keeping whole.
