# REVIEW-design — `docs/experiments/wp20s_design.md` revision 1

## Header

- **Revision adjudicated**: `1157dae963999d414fcfe1022a73b6d4550d03ef` (`docs(wp20s): the record schema decides what a label means …`).
- **Matches HEAD**: YES. `git rev-parse HEAD` = `1157dae963999d414fcfe1022a73b6d4550d03ef`.
- **Tree state**: clean at the start of this adjudication — `git status --porcelain` printed nothing. At the end of it the concurrent WP-2.0-M review round had appeared in the tree (` M docs/experiments/wp20m_design.md`, `?? docs/experiments/wp20m_design_REVIEW_rev3.md`); **none of it is mine and none of it was read for this review** — every claim below is adjudicated against `1157dae` as committed. The only file I wrote is this one.
- **What I ran**: `git` (`log`, `rev-parse`, `status`, `grep`, `log -S`, `log -L`), `/usr/bin/grep`, `awk`, `sed`, `wc`, `cat`, `ls`, `LC_ALL=C sort`, and `python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs docs/experiments/wp20s_design.md` (**green**, 60 citations checked, 0 unreproduced).
- **What I refused to run**, per the dispatch's hard constraint (a CI run is in flight in this tree): `cargo` in any form, `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. I also did not run `tools/file_justification_check.sh`; where a claim needed a run I could not make, I name the run instead.
- **Binding reading**: `CLAUDE.md` in full; `docs/process.md` in full; `docs/decisions.md` D-537…D-547 (plus D-6, D-80, D-423, D-424, D-483, D-518-by-reference, D-531, D-532, D-533); `docs/experiments/wp20_DESIGN_STOP_SPLIT.md`; `docs/experiments/wp20_design_REVIEW.md` (requirement tables, BLOCKING 1 and 2, MAJOR 6, finding index); `docs/experiments/wp20_design_REVIEW_rev2.md` (schema findings); `docs/experiments/wp20_dispatches.md`; `docs/experiments/wp20m_design.md` revision 3 (§0–§5, §8–§14); `docs/experiments/matrix_wp20_pipeline_shape.md` rows (g)/(h), §4, §5; `docs/experiments/matrix_wp20_shape_selection.md` §2–§5.
- **Code read**: `crates/pistol-arena/src/{exchange,transcript,conclusion,record,dedupe,game,replay,report,lib}.rs`, `crates/pistol-arena/src/bin/arena.rs`, `crates/pistol-cli/src/report.rs`, `crates/pistol-cli/src/protocol.rs`, `crates/pistol-cli/src/corpus/emit.rs`, `crates/pistol-cli/tests/workspace_shape_tests.rs`, `crates/pistol-search/src/{info,pvs,search,score}.rs`, `crates/pistol-core/src/{symmetry,state,turn,zobrist,lib}.rs`, `configs/arena_smoke_v0.toml`, `tools/file_justification_check.sh`.

---

## VERDICT: **FAIL**

**6 BLOCKING · 11 MAJOR · 12 MINOR.**

The document is a genuine advance on the two revisions that died: it does **not** repeat the provenance defect, and it does **not** repeat the summing defect — I attacked both hard and neither landed (see "The strongest attack that did not land"). It fails on four other grounds: one parser that cannot produce the column its own §2.3 defines (and which is the second limb of the review-1 BLOCKING that killed the schema in the first place); one claim about `pistol-core` that `pistol-core` contradicts on the very line the document cites for its neighbour; one cross-check rule that would refuse valid reports; and three requirements — **book-turn treatment**, **throughput shape**, **the corpus manifest** — that still fall between the packages, which is the defect the split was made to end.

---

# PART 1 — DOES IT REPEAT EITHER OF THE TWO DEFECTS THAT KILLED THE PREVIOUS ATTEMPT?

`docs/experiments/wp20_DESIGN_STOP_SPLIT.md` §2, lines 66–89.

## 1(a) The provenance remedy — **NOT REPEATED.** Verified, not accepted.

The dead defect: revision 2 "marked a record as solver-provenance when the totals line carried the solver fields", a condition that means *the solver was CONSULTED* rather than *the answer is a proof* (`wp20_DESIGN_STOP_SPLIT.md:66-74`).

**The claim to check** is `wp20s_design.md:260-261`: *"**So no column of this corpus can carry it honestly, and this package invents no proxy for it.**"* I checked it three ways.

1. **No provenance column exists.** §2's fourteen-column table (`wp20s_design.md:97-112`) carries none, and no column's definition reads the solver block's presence as a meaning. ✓
2. **The premise is true.** `SearchOutcome` carries `provenance` (`crates/pistol-search/src/info.rs:237-245`); the one site turning a `SearchOutcome` into protocol output writes `info_line`, `totals_line` and `bestmove_line` and never reads `outcome.provenance` (`crates/pistol-cli/src/protocol.rs:170-175`). `git grep -n "Provenance" -- crates/pistol-cli/src/` returns **nothing** (exit 1) — the document's own recorded grep reproduces. ✓
3. **Is the replacement discriminator a proxy in disguise?** §2.8:262-263 offers `search_nodes == 0`. This is **strictly finer** than the dead condition and is sound: `solver_proof_outcome` builds `search_nodes: 0, nodes: solver_nodes` (`crates/pistol-search/src/search.rs:795-797`), and on every other returned path `outcome.info.search_nodes = run.search_nodes` is overwritten from the run (`crates/pistol-search/src/search.rs:514`) where `search_nodes` is incremented per visited node (`crates/pistol-search/src/pvs.rs:212`, `crates/pistol-search/src/quiescence.rs:69,260`). Under a node budget the first iteration cannot be interrupted (D-74), so a non-proof answer has `search_nodes > 0`. **`search_nodes == 0` is the proof condition, not the consulted condition.** ✓

**Not repeated.** This is the finding the document handles best.

## 1(b) The two-sites-of-four defect — **PARTIALLY REPEATED.**

The dead defect: "§2 and INVARIANT 3 were rewritten … §7 still says 'non-forfeited' and §11 still registers a test that pins the NEGATION of the invariant above it" (`wp20_DESIGN_STOP_SPLIT.md:82-89`).

**No registered test pins the negation of an invariant.** I checked all 26 tests against all 9 invariants. The one that reads that way — `a_totals_line_without_solver_fields_yields_all_nodes_as_search_nodes` (`:565`) pinning INVARIANT 3's *"never their sum"* (`:538-541`) — is **not** a contradiction: when the solver block is absent, `solver_nodes == 0`, so `nodes == search_nodes` exactly (`crates/pistol-search/src/search.rs:513-514`, `crates/pistol-search/src/pvs.rs:150-152`), and nothing is summed. Cleared. (MINOR 12 records that INVARIANT 3 does not carry its own resolution.)

**But three rules ARE stated twice and differently, and one of them is fatal:**

- **§2.3 vs §3.** §2.3 defines `score_value` as *"the integer that goes with"* a three-way tag (`:154-162`) — a **two-word** read. §3 says the transform *"reads `score` … out of `fields_of`'s map"* (`:326-327`), where `fields_of` is typed `Option<Vec<(&str, &str)>>` — a **one-word-per-key** shape that cannot carry the number. **This is BLOCKING 1, and it is the second limb of `wp20_design_REVIEW.md` BLOCKING 2** (`:257-272`): *"One word after the key. `value("score")` returns the literal `cp`, `mate` or `-mate` — never a number … every label in the corpus is empty."* The naming half of that finding was fixed; the parser half was reintroduced under a new name.
- **§2.5 vs §2.8.** §2.5 states `depth_turns`'s meaning as *"always a depth that was actually COMPLETED"* (`:216-218`). §2.8 admits the record class where that is false — the solver-proof answer, whose `depth_turns` is `tree.win_depth_turns()`, a proof depth (`crates/pistol-search/src/search.rs:785,793`; `Provenance::SolverProof`'s own doc, `crates/pistol-search/src/info.rs:259-261`: *"`depth_turns` is the proof's depth in turns"*). MAJOR 9.
- **§5's "three of the fields are whitespace-bearing"** (`:386-387`) against §2's own fourteen columns, of which at most two can be. MINOR 2.

**Verdict on Part 1: (a) not repeated; (b) repeated in the "one rule, two answers" form, once fatally.**

---

# PART 2 — THE FOURTEEN CHECKS

### 1. Repeat of the killing defects
Answered in full above. (a) NO. (b) YES, in the stated-twice form; §2.3/§3 is BLOCKING 1.

### 2. §2.4, the node columns

**(a) Is `nodes` really the sum, at the site cited?** YES. `wp20s_design.md:193-196` cites `crates/pistol-search/src/info.rs:167-172` (*"ONE OF THE TWO INDEPENDENT COUNTERS — `nodes` is their derived sum at report time"* — verbatim at `:169-170`) and `crates/pistol-search/src/pvs.rs:148-152` (`total_nodes` = `self.search_nodes + self.solver_nodes` at `:151`). Both citations support their claims. ✓

**(b) Is the solver block emitted only under the stated condition, and are the fields exactly the ones named?** The **condition** is right — `let solver_field = if info.solver_nodes > 0` (`crates/pistol-cli/src/report.rs:62`). The **fields are not**: the block is **six** fields, not two — `search_nodes`, `solver_nodes`, `solver_firings`, `solver_invocations`, `solver_proofs`, `solver_root_nodes` (`crates/pistol-cli/src/report.rs:69-78`). The document never names the other four anywhere, does not carry them as columns, and does not name them in §2.8's list of deliberate absences. **MAJOR 6.**

**(c) Is the fallback TRUE, not merely documented? Is there any path where the gate is off and `nodes != search_nodes`?** **TRUE, and I could not find such a path.** The document rests on `SearchInfo::search_nodes`'s doc (`crates/pistol-search/src/info.rs:162-166`, *"Zero difference from `nodes` whenever the gate is off"* — verbatim at `:165`). The doc is backed by code, and it took looking: `search.rs:451-465` and `:487-501` construct salvage/fallback `SearchInfo`s with `nodes: 0, search_nodes: 0`, which would break the identity — but **both are overwritten** at `crates/pistol-search/src/search.rs:513-514` (`outcome.info.nodes = run.total_nodes(); outcome.info.search_nodes = run.search_nodes;`), with a comment at `:515-518` recording that the overwrite exists for exactly this reason. Since `total_nodes()` is `search_nodes + solver_nodes` and the block's absence means `solver_nodes == 0`, `nodes == search_nodes` on every path that reaches `totals_line`. The remaining construction, `solver_proof_outcome`, sets `nodes: solver_nodes` with `search_nodes: 0` (`:795-796`) and is reached only with `solver_nodes > 0`, so the block is present and the fallback does not fire. **The claim holds.** (MINOR 4: the document cites the doc-comment and not the site that enforces it.)

**(d) Is "one solver field present and not the other" unreachable from `render_info`?** YES — the six fields are one `format!` and are all-or-nothing (`crates/pistol-cli/src/report.rs:69-78`). The refusal is therefore not dead in the test sense (a hand-written fixture reaches it, and §11 registers both the test and the mutant), and hard rule 3 licenses it. **But its coverage is arbitrary**: it guards a 2-subset of a 6-field atomic block, so a capture carrying `search_nodes`+`solver_nodes` and missing `solver_proofs` — equally impossible from `render_info`, equally a sign that something else produced the line — is accepted silently. The document's own ground (*"a corpus is the wrong place to find out that something else produced it"*, `:211-212`) covers all six. **MAJOR 6.**

### 3. §2.7 and §4, the outcome

**(a) Is `result` genuinely unrecoverable for a forfeited game?** **YES, verified.** On a forfeit the arena writes `GameResult::loser_of(mover_is_p1)` (`crates/pistol-arena/src/game.rs:64,77-85,92-100`) and records the move list as it stood. `transcript::read`'s `replays` guarantees a recorded list is a legal prefix with at most a terminal win (`crates/pistol-arena/src/transcript.rs:359-379`), so a forfeited game's replay ends `Ongoing` and cannot name the awarded side. ✓

**(b) Is widening `RecordedGame` safe?** **YES, and this was worth checking hard.** Every consumer of `Transcript`/`RecordedGame` is `crates/pistol-arena/src/bin/arena.rs:173`, `crates/pistol-arena/src/replay.rs:12,20,87,114,216`, `crates/pistol-arena/src/replay_report.rs:7,94`, and two test files — the struct has exactly one construction site (`crates/pistol-arena/src/transcript.rs:303`), so added fields break no caller. The two new **fatal** `value()` lookups are safe because `conclusion.rs` writes `result` and `forfeit_by` on **every** game record unconditionally, `forfeit_by` taking the literal `none` when there was no forfeit (`crates/pistol-arena/src/conclusion.rs:27-31,39`). I checked the document's schema-guard argument independently: `forfeit_by` has been in the game record since the arena's first commit (`git log -S "forfeit_by {by}"` → `1ad4070`, schema 1), and `REPORT_SCHEMA` is 4 (`crates/pistol-arena/src/report.rs:25`), so no schema-4 report lacks either field. **No report the replay mode can currently accept becomes refused.** ✓

**(c) Is "not on the SPRT path" true?** **YES.** `git grep` for `transcript::`/`Transcript`/`RecordedGame` across `crates/` returns no hit in `schedule.rs`, `game.rs`, `seats.rs`, `score.rs`, `sprt.rs` or `conclusion.rs` — the generation path writes reports and never reads one. ✓ INVARIANT 4's byte-identity claim is untouched by the widening.

**(d) Is the cross-check an "externally derived referent" in `docs/process.md`'s sense?** **NO — and the document's stated ground is false.** `wp20s_design.md:245-247` says *"pistol-core does not share the report's own arithmetic, so the check is one the defect could falsify rather than one it preserves."* The report's `result` **is** pistol-core's arithmetic: `game.rs:89-107` takes `state.make_turn(turn)`'s `Outcome` and maps `Player::P1 => GameResult::P1Win`, `Player::P2 => GameResult::P2Win` one-for-one. The cross-check re-runs the **same judge** over the **same move list**, and `transcript::read` already runs it a third time at read time (`transcript.rs:359-379`). `docs/process.md:49-55` names this shape: *"A criterion that is a property the named defect class PRESERVES — internal agreement between components sharing an input … passes vacuously"*; `:69-72`: *"two instruments blind to the same stage are one instrument reported twice."* The check retains a narrow real value — it can falsify a serialization defect between `game.rs`'s in-memory record and the report file, and it can falsify a defect in the transform's own replay — but calling it an externally derived referent is a mis-citation of `docs/process.md`, and §11's vacuity remedy (`:613-615`) rests on the same false ground. **MAJOR 2.** Compounding it, the rule as written is wrong (**BLOCKING 3**).

### 4. §3, the `totals_of` split

**(a) Can `totals_of` read a captured line?** **NO — the document is right.** `totals_of` is a `?`-chain over `nodes`, `TIME_FIELD` and `depth_turns` (`crates/pistol-arena/src/exchange.rs:169-188`, lookups at `:185-187`), and WP-2.0-M removes ` nps <n> time <n>` from every captured totals line (`wp20m_design.md:300-304`). `value(TIME_FIELD)?` returns `None`, so `totals_of` returns `None` on every corpus input. ✓ The premise is sound and well-found.

**(b) Is the SPRT path byte-identical, and is the borrow shape workable?** **The lifetime shape is fine**: `fn fields_of(line: &str) -> Option<Vec<(&str, &str)>>` elides to one input lifetime, and every `&str` in the Vec is a subslice of `line`, so nothing borrows a local — `rest` at `exchange.rs:175` is already such a subslice. **Byte-identity holds for the three SPRT lookups**: the totals tail is strictly alternating key/value from `depth_turns` through `hashfull` (`crates/pistol-cli/src/report.rs:82-84`, with the six-field solver block inserted as three whole pairs at `:69-78`), so `nodes`, `time` and `depth_turns` resolve to the same words under either pairing scheme. ✓ **But the same fact kills the label half**: the tail stops alternating at `score cp 42` — `score_token` emits two words (`crates/pistol-cli/src/report.rs:153-158`) — and `pv` is a variable-length tail with no value. Under index-pairing the pairs after `hashfull` are `("score","cp"), ("42","pv"), (pv0,pv1)…`; under adjacent-pairing, `value("score") == "cp"`. **Under both readings the score's number is unreachable.** **BLOCKING 1.**

**(c) Does it depart from D-542's branch-B text, and is the departure handled under hard rule 10?** It departs twice. D-542 records branch B as `totals_of` *"gains `score` and `pv` as **non-fatal `Option`s** with the three existing lookups kept load-bearing"* (`docs/decisions.md:1152`). The design adds **neither** lookup and splits the function instead. §2.8:267-274 flags the `pv` half and promises an ADR amendment at closure; §3:315-324 states the shape change openly and argues it is safer. That is acceptable rule-10 handling **for the mechanism**. It is **not** acceptable for the fourth mode and the third pass (MAJOR 5).

**(d) Does WP-2.0-M's recogniser survive?** **YES.** WP-2.0-M's pass 2 identifies the totals line by `totals_of(&line).is_some()` (`wp20m_design.md:533-537`), and pass 2 sees **live** engine output, which carries `time`. `wp20s_design.md:328-330` states exactly this and it is correct. ✓ Two riders: WP-2.0-M's own ground for the widening — *"so that package adds fields to one parser instead of writing a second"* (`wp20m_design.md:519-520`) — no longer describes what WP-2.0-S builds, and WP-2.0-M's registered mutant is justified there as *"the guard WP-2.0-S inherits when it adds `score` and `pv` as non-fatal `Option`s"* (`wp20m_design.md:553-554`), which it now does not do. Both belong in check 13.

### 5. §2.1 and §7, the keys and dedup

**(a) What each key folds.** `canonical_sequence` returns `Vec<Turn>`, the least of twelve images of a **sequence** (`crates/pistol-core/src/symmetry.rs:206-219,219-233`), with the document's quoted sentences verbatim at `:206,209-211` and *"It has no false positives"* at `:213`. `GameState::key` is `self.board.stones_key() ^ context_key(self.to_move, self.phase)` (`crates/pistol-core/src/state.rs:134-136`) with *"two positions this key cannot tell apart are the same position"* at `:131-132`. **Both of the document's precise claims are right.** ✓

**(b) Is `GameState::key` reachable and meaningful for a replayed prefix?** YES. `GameState` is exported at `crates/pistol-core/src/lib.rs:85`, `Key128` at `:90`, and `key()` is `pub`. Its own doc hedges the guarantee to an **ongoing** game (`state.rs:129-131`), which every asked prefix is (WP-2.0-M INVARIANT 2). ✓ (MINOR 7: the column's spelling is not fixed; `Key128`'s `Display` is 32 hex digits high-half-first, `crates/pistol-core/src/zobrist.rs:70-76`, but the design never says the column uses it.)

**(c) Is "deduplicate nothing and carry two keys" a legitimate reading of "dedup policy for transposed positions (by canonical move list, stated)"?** **LEGITIMATE, not an evasion.** D-6 makes the plain move list the canonical position encoding (`docs/decisions.md:24`), so "by canonical move list" names the `moves` column, and the document answers it directly at `:452-454`: two records with the same `moves` cannot occur within one corpus because each prefix is asked once. The policy is stated, the key is carried, the asymmetry-of-loss argument (`:442-447`) is sound, and D-483 keeps the training decision out of a design. ✓

**(d) Does "no pistol-core key folds both" survive a search of pistol-core's public surface?** **NO. IT IS FALSE.** `pistol_core::canonical_form` (`crates/pistol-core/src/symmetry.rs:157-165`) takes a **stone set**: *"The canonical spelling of a position: the least of its twelve images … two positions with the same canonical form are the same position up to a symmetry of the lattice … The input need not be sorted and its play order is not read."* Order-independence is transposition-folding; the twelve-image minimum is symmetry-folding. **It folds both, and it is exported on the very line the document cites for its neighbour** — `crates/pistol-core/src/lib.rs:86` reads `pub use symmetry::{Symmetry, canonical_form, canonical_sequence};`. **BLOCKING 2.**

### 6. §2.3, the score

**(a) Three spellings, three arms?** YES. `score_token` has exactly three arms over `ScoreKind::{Eval,MateIn,MatedIn}` (`crates/pistol-cli/src/report.rs:153-158`; `classify` at `crates/pistol-search/src/score.rs:64-72`; the enum at `:54-61`). The design's mapping table (`:158-162`) is correct in all three rows, including that `mated_in`'s value is the positive `<t>` with the sign carried by the tag. ✓ This is the section that discharges `wp20_design_REVIEW.md` BLOCKING 2's first limb and the STOP-SPLIT's owed item 4 (*"plain `mate` has no fixture"*) — three fixtures, three tests (`:562-564`). ✓

**(b) Sign convention?** RIGHT. `crates/pistol-search/src/info.rs:153-155`: *"The score of the position from the point of view of the side to move at the root."* ✓

**(c) Mate counting?** RIGHT. `crates/pistol-cli/src/report.rs:147-148`: *"counts every turn from the root, both sides', so a win for the side to move is always an odd distance and a loss an even one"* — quoted verbatim. ✓

**(d) Does renaming `cp` to `eval` lose anything, and is the header sufficient?** Nothing a consumer needs is lost — the units live nowhere but in `pistol-eval`, exactly as the quoted doc says (`report.rs:149-152`), and no tool parses a corpus column. The header carrying the three properties is the right call. **Two riders**: the header's three properties are emitted through `Fixture::note`, which renders as a bare `# <text>` line indistinguishable from a title line (`crates/pistol-cli/src/corpus/emit.rs:51-58,20-28`) — no machine can find them, and no registered test pins their presence (MINOR 10, MINOR 11); and `score_value`'s **type** differs by kind (`i32` signed for `eval`, `u16` non-negative for the mates), which the design never states and the loader's *"a number spelled a way this format does not write"* (`:406-407`) does not settle (MINOR 6).

### 7. §8, the census-minimum rule

**(a) Does it honour D-537's two fixed conditions?** **Condition (1), the disjoint-positions denominator: named, but not made countable.** `:485-487` quotes it correctly, and D-537's own words are at `docs/decisions.md:1142`. But the rule never says **what makes two positions disjoint** — and this is the one document in the arc that just decided there are two incompatible notions of sameness (§2.1, §7:461-463) and that neither folds the other's equivalence. A rule whose denominator has two candidate definitions and no ruling is not countable. **Condition (2), the power-style rule fixed before any score is fitted: HALF-HONOURED.** The **form** lands now. **Two of the four inputs are deferred to a pre-registration that does not exist, has no named owner and appears in no deferral list** — and they are precisely D-518's item 3, the component `wp20_design_REVIEW.md` MAJOR 6 (`:500-514`) killed the last version for deferring: *"the threshold, `power ≥ 0.90` … registered here, **before the sweep**, precisely because moving it afterwards is the post-hoc threshold move CLAUDE.md forbids."*

**(b) Is a rule with four registered inputs and no number a real discharge?** **It would be — if the inputs were registered. They are not, and the document's own anti-fitting argument does not cover them.** `:492-495` grounds the rule's protective force on *"a rule whose four inputs come from a CLOSED arc cannot be tuned by the corpus it will be applied to."* That is true of input 1 (incumbent recall — the closed arc's `0.571`, `docs/decisions.md:1130,1132`, `docs/experiments/matrix_stage3_detector.md:613,634,645`). It is **false of inputs 3 and 4**: α and power are free choices a later session makes, and a session that wants round 3 open picks a weaker pair and gets a smaller minimum. It is **false of input 2** for a stronger reason: **there is no registered "target recall the detector must beat for round 3 to be worth opening" anywhere in the tree.** The closed arc registers an incumbent (`0.571`) and a *bound* (`0.857` / `1.000`); a bound is not a target, and D-531 records that on trigger-rich per-search *"the gap is ZERO and the barrier is arithmetic"* (`matrix_stage3_detector.md:645-646`). §8's input 2 is *"taken from the same closed arc"* (`:480-481`) with **no citation at all** — bare prose the citation checker cannot see. **MAJOR 3.** D-483 does not license this: α and power are choices, not measured numbers, and D-518 registered its threshold in a registration document exactly as CLAUDE.md's no-post-hoc-threshold rule requires.

**(c) Is the handling of D-544's sequencing question honest?** **Partly, and the honest half is undermined by the dishonest half.** The argument at `:503-505` — that the rule's protective force is being committed and dated before any census row exists, which git shows — is a good argument, and D-539's *"the pilot carries no census and is not corpus"* is correctly quoted (`:507-508`). But it is an argument about **the form**, and the document has just deferred the two inputs whose post-hoc movement is the entire defect D-537 forecloses. Declaring an operator's open question *"does not need [to be] answered"* (`:503`) while the rule's own anti-fitting guarantee covers two of four inputs is a dodge. Answered by MAJOR 3; the sequencing question stays open and the operator should be told so rather than told it is moot.

### 8. §6, the label policy and the absent seed

**The decision is right and the ground is fabricated.** ALL-and-therefore-no-seed is honest and is **not** a package narrowing its own scope: WP-2.0-M asks every legal turn boundary of every recorded game (`wp20m_design.md:177-190,201-205`, INVARIANT 1 at `:560-563`), the transform writes one record per capture record, pass 1's book range is chosen and ledgered rather than sampled, and D-540 fixes that *"seeds attach to pipeline SAMPLING only"* (`docs/decisions.md:1148`). With nothing sampled, D-542's owed item 2 (*"the design says what is sampled"*) is discharged by saying nothing is. ✓ Requirement 4 survives without a seed: INVARIANT 5 covers the transform, WP-2.0-M §14 covers the capture's re-run receipt at gate 15, and the end-to-end receipt is the pilot's.

**But `:426-427` attributes to the dispatch a sentence the dispatch does not contain.** It reads: *"The dispatch's own wording is conditional — "seed ignored **where the pipeline samples**"."* The dispatch's registered mutant list is `docs/experiments/wp20_dispatches.md:105-107` and reads, in full: *"schema field dropped -> loader test dies; seed ignored -> determinism receipt dies; ledger overwrite -> append test dies; census direction collapsed -> its test dies."* `/usr/bin/grep -n "sampl\|seed" docs/experiments/wp20_dispatches.md` returns exactly three lines — `:80`, `:91`, `:105` — and **none carries the quoted clause**. The mutant is unconditional. **MAJOR 1** — this is the sharpest instance in the document of the exact class D-545 diagnoses, applied to a governing text instead of to code, and the citation checker cannot see it because the attribution carries no line number.

### 9. §1, the fourth mode

**(a) Consistent with D-542's row (g)?** **NO, and the departure is unamended.** D-542 selects *"a labelling mode of the existing `arena` binary, **two-pass**"* whose mechanism is *"**a third arm** in `bin/arena.rs`'s mode match beside `--config` and `--replay` … and writing one record per position"* (`docs/decisions.md:1152`; the matrix's row (g) at `matrix_wp20_pipeline_shape.md:243-252`). WP-2.0-M + WP-2.0-S together ship **two** new arms and **three** passes, and the record is written by a pass D-542 does not describe. §1's three grounds (`:71-78`) are good grounds — I would take the same decision — but the document never notes that it is departing from the selected shape and never promises the ADR amendment, while §2.8 invokes hard rule 10 for the far smaller `pv` clause (`:271-274`). **MAJOR 5.**

**(b) New crate, manifest change, `workspace_shape_tests.rs`?** **None needed, verified.** `pistol-arena` already depends on `pistol-core` and `pistol-cli` (`crates/pistol-arena/src/dedupe.rs:3`, `exchange.rs:155`), the shape tests police manifest dependency **names** (`crates/pistol-cli/tests/workspace_shape_tests.rs:16-69,78-80`), and no name changes. ✓ The design never states this inheritance; MINOR 5 records that it also never states the required `pub mod labels;` in `crates/pistol-arena/src/lib.rs:47-69`.

**(c) Does the file cap admit a fourth arm?** **Almost certainly yes, by arithmetic I can show.** `wc -l crates/pistol-arena/src/bin/arena.rs` = **283**; `tools/file_justification_check.sh:65` sets `SOFT_CAP=300`; `USAGE` occupies `:16-58` (43 lines) and WP-2.0-M moves it to `crates/pistol-arena/src/usage.rs` (`wp20m_design.md:151-156`). Post-extraction the binary is ~240 lines plus two mode variants, two dispatch patterns and two calls — comfortably inside. **The run that settles it** is `tools/file_justification_check.sh`, CI gate 17, at the post-implementation revision; I did not run it. One gap the design leaves: it never says the `--labels` usage paragraph goes in `usage.rs` rather than back in the binary (MINOR 5).

### 10. Requirement coverage

**Dispatch Scope (`wp20_dispatches.md:69-83`):**

| # | requirement | status |
|---|---|---|
| 1 | Plays self-play games | **CORRECTLY ELSEWHERE** — WP-2.0-M §1 pass 1 |
| 2 | One record per position: canonical move list, side to move, score+best+depth+nodes, outcome, versioned schema with loader test | **DELIVERED WITH TWO HOLES** — §2, §5. The score column is unbuildable (BLOCKING 1); the `moves` column is wrong for the `k = 0` record WP-2.0-M guarantees (MAJOR 10) |
| 3 | Census logging | **CORRECTLY ELSEWHERE** — D-539, §8, §12 ✓ |
| 4 | Deterministic end-to-end given (seed, book range, config, SHA) | **DELIVERED for this package** — INVARIANT 5; seed correctly declined (MAJOR 1 is the ground, not the answer) |
| 5 | Ledgers: book ranges, **corpus manifest with digests**, census manifest | **FALLS BETWEEN.** WP-2.0-M §13(a) defines `docs/label_corpus_manifest.md` as *"one row per **capture**"*; the **corpus** file is a distinct uncommitted artifact with its own digest and no manifest row anywhere. Hard rule 8 asks for one. **MAJOR 4** |

**"Design decides and records" (`wp20_dispatches.md:88-96`):**

| item | status |
|---|---|
| storage format and schema version | **DELIVERED** — §5 ✓ |
| label policy (all, or a registered sampling rule) | **DELIVERED** — §6 ✓ |
| dedup policy for transposed positions (by canonical move list, stated) | **DELIVERED** — §7, on a false ground (BLOCKING 2) |
| the census minimum rule per D-53a | **PARTIAL** — MAJOR 3 |
| **throughput expectation stated as a shape, measured in the pilot, never guessed (D-500's class)** | **FALLS BETWEEN — NOWHERE.** `/usr/bin/grep -i "throughput\|per hour\|games per\|labels per"` returns **nothing** in `wp20s_design.md` **or** `wp20m_design.md`. It is in neither §12's deferral list nor WP-2.0-M §11's. The dead design **had** it (`wp20_design_REVIEW.md:88`: *"DECIDED — §9 … Clean"*). **The split lost it.** **BLOCKING 6** |

**D-544's own charge list**, quoted by the document at `:4-6`: provenance ✓, score representation ✓, node columns ✓, **book** ✗ **and** forfeit ✓ treatment, the board key ✓, capped-versus-forfeited ✓, census-minimum rule ~. **Book-turn treatment is never decided** — and WP-2.0-M explicitly hands it here: *"Book turns and forfeited games are asked like any other, because those ARE exclusions by meaning and **they belong to WP-2.0-S**"* (`wp20m_design.md:189-190`). Both prior reviews flagged it (`wp20_design_REVIEW.md:421-450`, `wp20_design_REVIEW_rev2.md:597`). **BLOCKING 4.**

**Matrix §5's open decisions** (`matrix_wp20_pipeline_shape.md:352-359`): 6 deferred ✓; 9 partial (MAJOR 9); 10 ✓; 11 inherited ✓; **12 (the corpus manifest's digest boundary) falls between** — MAJOR 4; storage/label/dedup ✓.

### 11. Closure of the sets

**(a) Invariants with no test**: none — all 9 are pinned by name (`:558-585`). ✓
**(b) Tests pinning nothing**: none — 25 pin invariants, 1 pins §2.2. ✓
**(c) Rules with neither test nor mutant**: §5's header contents (the schema version aside, nothing pins the three unit `note` lines or the five header digests — a mutation dropping them kills nothing: MINOR 11); §5's write-side *"A field carrying a TAB refuses the run by name"* (`:389`) — the only arity test is loader-side; §9's *"a captured `moves` prefix is not a legal game"*; §9's *capture* schema-version refusal (the registered test `a_corpus_whose_schema_version_is_unknown_is_refused_by_name` pins the **corpus's** version, a different check); §2.5's *"Neither is re-derived"*; §1's `capture_sha256` is copied to the corpus header and never re-derived from its four inputs (MINOR 8).
**(d) Mutants that cannot die**: **one, and it is real.** *"`to_move` computed from turn parity"* → `side_to_move_comes_from_pistol_core_and_not_from_turn_parity` (`:603,585`). Under game rule 3 the mover alternates strictly by turn, and every asked position is a turn boundary with `Phase::First`, so parity is **exactly correct at every prefix** — including `k = 0` and including a first turn that is `Turn::Single`. The mutation is a **no-op**; its test cannot fail. §2.2's argument is a good **rule-2** argument (`:143-145`) and a bad behavioural one, and registering it as a behavioural mutant is registering a mutant that cannot die. **MAJOR 8.**
**(e) Tests that would pass vacuously**: `the_transform_spawns_no_process_and_reads_no_clock` (`:561`) pinning INVARIANT 2 (`:536-537`). No mechanism is given, and no in-process Rust test can observe the absence of a `Command::new` or an `Instant::now` on a code path it does not take. As registered it passes whatever the code does. WP-2.0-M faced the same question and answered it properly — *"Registering a further test here would be registering one that passes whatever the code does, which `docs/process.md` calls a criterion that is not one"* (`wp20m_design.md:597-599`). **MAJOR 7.**

**Is §11's own vacuity paragraph's remedy sufficient?** **NO, on its own terms.** It names one at-risk test and prescribes *"its fixture reports are produced by the arena itself and their `result` fields are the arena's own"* (`:613-615`). But the arena's `result` field **is** pistol-core's `Outcome` (`game.rs:103-107`), so the fixture's referent and the check's referent are the same judge over the same moves — the remedy relocates the shared input rather than removing it. The remedy does retain real value (it can falsify a defect in the transform's own replay and mapping — in fact it is the one test that would have caught BLOCKING 3), and the paragraph is right that no test here needs a real engine. But it identifies the wrong vacuity risk, misses the two above, and its stated ground is the same false one as §2.7's. **MAJOR 2.**

### 12. D-483 and the citation checker

**The checker is green**: 60 citations, 0 unreproduced. **The half it cannot do**: I checked 24 citations' content against the cited lines. **Twenty-one support their claims exactly** — `info.rs:167-172`, `info.rs:162-166`, `info.rs:153-155`, `info.rs:133-138`, `info.rs:237-245`, `info.rs:250-267`, `pvs.rs:148-152`, `report.rs(cli):15-18`, `:20-29`, `:62-81`, `:145-158`, `:147-149`, `:149-152`, `protocol.rs:172-174`, `symmetry.rs:206-219`, `:213-218`, `state.rs:124-136`, `:128-133`, `turn.rs:34-45`, `lib.rs:86`/`:90`/`:85`, `dedupe.rs:12-26`, `conclusion.rs:37-52`, `record.rs:16-22`/`:25-28`, `transcript.rs:48-50`, `:152-158`, `:270-313`, `replay.rs:16-19`, `exchange.rs:163-168`, `:169-188`, `:76-79`, `arena/report.rs:41-50`, `emit.rs:12-100`/`:92-99`/`:102-118`. **Three do not:**

- `:388` — *"this crate already refuses a whitespace-bearing path "because the format is whitespace-delimited and does not quote"" (`crates/pistol-arena/src/transcript.rs:124-131`)*. The **comment** says that; the **function** refuses only an **empty** path (`transcript.rs:128-131`). MINOR 1.
- `:219-220` — *"which the protocol writes as "`bestmove <turn>`"" (`crates/pistol-cli/src/report.rs:105-108`)*. `/usr/bin/grep -rn "bestmove <turn>" crates/` returns **nothing**. The substance is true (`report.rs:107-108`); the quotation is not a quotation. MINOR 3.
- `:66-69` — *"the exclusive output claim is inherited from `crates/pistol-arena/src/bin/arena.rs:103`, which claims `--out` **before the mode match**"*. `:103` is **after** the `match words` at `:82-100`; it is before the mode **dispatch** at `:104-107`. The substance holds. MINOR 4.

Two further claims are **bare prose with no citation at all**, both load-bearing: §3's *"the ONLY place in this workspace that tells `info totals …` from `info …`"* (**MAJOR 11**, false) and §8's *"taken from the same closed arc"* for the target recall (**MAJOR 3**, no such registered figure).

**Numeral sweep / D-483**: I stripped citations and swept the remainder. Every surviving numeral is a column index, a section number, a D-number, or a cited fact about the code (twelve symmetries, four `Provenance` variants, 128-bit key). **No measured number, no bracket, no threshold. D-483 compliant** — and §8's refusal to write a minimum is correctly grounded (`:490-492`). One numeral is a claim about the document's own record and is wrong: *"three of the fields are whitespace-bearing"* (MINOR 2).

### 13. The premise on WP-2.0-M

| # | what is depended on | site | what breaks if WP-2.0-M's review forces a change |
|---|---|---|---|
| 1 | **The capture header carries `source_sha256`** | `:80-81` → `wp20m_design.md` §4.4 (`:380-386`) | §1's whole pairing check and its refusal; `:516`'s first refusal row. This is the **lowest-risk** dependency — §4.4 is a verbatim lift of a paragraph a reviewer passed (`wp20m_design.md:49`) |
| 2 | **The normalisation removes ` nps <n> time <n>` and nothing else** | `:297-298`, `:281-283` | §3's entire premise. If the normalisation changed to keep `time`, `totals_of` would read a captured line and §3's split would have no reason to exist; if it removed more, §2.4's node columns could vanish. Also a **passed** paragraph (`wp20m_design.md:48`), so low risk |
| 3 | **The record's TAB grammar and five-field arity** | `:384-389` → `wp20m_design.md` §4.2 (`:322-351`) | §5's own TAB choice (which points at §4.2 rather than restating), §9's field-count refusal, and the loader. §4.2 is **NEW in revision 3** (*"rev-2 MAJOR E, and it is decided here"*) and is **not** in the freeze table — **this is the highest-risk premise in the document** |
| 4 | **The asked set** (every legal turn boundary; `k = 0` asked as bare `position start`; the decided final prefix excluded) | `:417-419`, and silently by §2's `turn`/`moves`/`to_move` columns | INVARIANT 1's one-record-per-capture-record claim, §6's no-sampling argument, and the `moves` column's spelling. §2 is a **frozen** section per `wp20m_design.md:46`, but the **range enumeration** at `:201-205` is a revision-3 edit. MAJOR 10 already fires on the `k = 0` case as WP-2.0-M has it |
| 5 | **The capture schema version** | `:517` | The capture-version refusal row; nothing structural |
| 6 | **`totals_of` stays the recogniser and rises to `pub(crate)`** | `:328-330` → `wp20m_design.md` §8 (`:511-554`) | §3's "one recogniser" argument and the `pub(crate)` visibility both halves need. §8's own rationale for the widening (*"so that package adds fields to one parser"*) and for its mutant (*"the guard WP-2.0-S inherits when it adds `score` and `pv`"*) **already do not describe what WP-2.0-S builds**, so a reviewer editing §8 could break §3's premise while fixing WP-2.0-M's own text |
| 7 | **`docs/label_corpus_manifest.md` and the ledger row** | not depended on — and that is MAJOR 4 | — |
| 8 | **The freeze status of WP-2.0-M §1 and §4** | `:74-76` | §1's ground (2) for a separate mode. This is **already wrong** — MAJOR 12 |

### 14. Implementability

**NO.** See the list below.

---

# PART 3 — FINDINGS

## BLOCKING

### BLOCKING 1 — `fields_of` cannot produce `score_value`, so §2.3's column is unbuildable from §3's parser — and this is the second limb of the BLOCKING that killed revision 1

`wp20s_design.md:305-307` and `:326-327`:

> **`fields_of(line) -> Option<Vec<(&str, &str)>>`** — recognises the totals marker and returns the line's key-value tail.

> The corpus transform reads `score`, `depth_turns`, `nodes` and, when present, `search_nodes` and `solver_nodes` out of `fields_of`'s map.

The score is **three** tokens on the wire, not two: `render_info` writes `… hashfull {} score {} pv` (`crates/pistol-cli/src/report.rs:83-84`) and `score_token` expands to `cp 42` / `mate 7` / `-mate 7` (`:153-158`). A `Vec<(&str, &str)>` keyed by field name yields `value("score") == "cp"` under adjacent-pairing and `("score","cp"), ("42","pv")` under index-pairing. **Under both, the number is unreachable**, and `pv`'s variable-length tail is unpairable at all.

`docs/experiments/wp20_design_REVIEW.md:257-272` is the same finding against the same helper: *"One word after the key. `value("score")` returns the literal `cp`, `mate` or `-mate` — never a number … Implemented as the helper stands, `.parse::<i32>()` fails on **every** line and every label in the corpus is empty."* Its FIX was explicit: *"state the read as `(tag, number)` with the tag deciding the sign"* (`:282-283`). §2.3 takes the naming half of that fix and §3 rebuilds the parser the other half condemned. D-542's branch-B text chose to widen `totals_of` *"so `score` and `pv` come out of the one parser"* precisely because `score` needs bespoke two-token handling; the field-map replaces that with a shape that provably cannot do it.

**FIX.** Type the split's product as the recognised **word list** (`Option<&[&str]>` or the existing `Vec<&str>`), keep `totals_of`'s three `value()` lookups over it unchanged, and state in §3 that `score` is read as a `(tag, number)` pair — the tag at the word after `score`, the number at the word after that — with an unparseable or absent pair refusing the run by name per §9.

---

### BLOCKING 2 — `pistol-core` DOES define a key that folds transpositions and symmetries together, on the line the document cites for its neighbour

`wp20s_design.md:461-463`:

> **No pair of these folds the other's equivalence.** A consumer wanting both folds at once needs a key this design does not carry and **pistol-core does not define**, and inventing one is out of scope by rule 2.

Repeated as residue at `:634-635`: *"**A key folding transpositions AND symmetries together does not exist in pistol-core** (§7), so no column carries one."*

`crates/pistol-core/src/symmetry.rs:157-165`:

> /// The canonical spelling of a position: the least of its twelve images.
> /// … two positions with the same canonical form are the same position up to a symmetry of the lattice.
> /// The input need not be sorted and its play order is not read.
> `pub fn canonical_form(stones: &[(Coord, Player)]) -> Vec<(Coord, Player)>`

Play-order independence over a stone set **is** transposition folding; the twelve-image minimum **is** symmetry folding. `transform` sorts for exactly this reason (`symmetry.rs:143-148`), and `transform_sequence`'s doc contrasts the two head-on (`:181-193`). It is **public and exported at `crates/pistol-core/src/lib.rs:86`** — the same line the document cites, three identifiers away from `canonical_sequence`.

Three consequences: §7's closing bullet is false; §12's residue 3 tells a successor to look for something that is already there; and §8's disjointness denominator loses the obvious answer to the question MAJOR 3 says it leaves open. §2.1's *"a third notion of 'the same position' written in this crate would be a second judge of sameness"* (`:134-136`) is right about **this** crate — but pistol-core already owns the third notion, so carrying it costs nothing under rule 2.

**FIX.** Delete both false sentences. Either carry a third column from `canonical_form` over the replayed prefix's stones, or state the ground for not carrying it (a stone-set key discards side-to-move, which `key_pos` folds in) — and point §8's disjointness at whichever key rules.

---

### BLOCKING 3 — §2.7's cross-check maps the winner through `a_is_p1`, which is not what `result` is about; as written it refuses valid reports

`wp20s_design.md:239-244`:

> and requires the derived outcome to agree with the report's own `result`: a `Win` by the side **`a_is_p1`** names, or `capped` when the list ends `Ongoing`. **Disagreement refuses the run by name.**

`GameResult` is about **seats**: `P1Win => "p1_win"`, `P2Win => "p2_win"` (`crates/pistol-arena/src/record.rs:16-22`), and it is written straight off pistol-core's `Outcome` — `pistol_core::Player::P1 => GameResult::P1Win` (`crates/pistol-arena/src/game.rs:103-107`). `a_is_p1` is about **which engine holds seat one** (`crates/pistol-arena/src/transcript.rs:17-18`, and `seat_of`'s doc at `game.rs:113-119`). It plays no part in the agreement relation, which is simply `Outcome::Win{winner: P1} ⟺ result == "p1_win"`.

An implementer who follows the sentence writes a comparison gated on `a_is_p1` and inverts it on every game where engine B holds seat one — roughly half of a paired-openings run — and §9:522 makes disagreement **refuse the whole run**. The result is a transform that refuses the reports it was built to read. §11's `the_derived_outcome_agrees_with_the_reports_own_result_field` is the test that would catch it, which is why the reviewer's remedy in §11 matters (MAJOR 2).

**FIX.** Replace the clause with: *the derived `Outcome::Win{winner}` must map to `p1_win` for `Player::P1` and `p2_win` for `Player::P2`, and to `capped` when the replay ends `Ongoing`; `a_is_p1` is not read.* Note in §2.7 that `transcript::read` already replays every list (`transcript.rs:359-379`), so this check adds the verdict comparison and not the replay.

---

### BLOCKING 4 — book-turn treatment is this package's charge, is handed here by name, and is never decided

The document quotes its own charge at `:4-6`: D-544 cuts out WP-2.0-S with *"provenance, score representation, node columns, **book and forfeit treatment**, the board key, capped-versus-forfeited, and the census-minimum rule."* WP-2.0-M hands it over explicitly: *"**Book turns and forfeited games are asked like any other**, because those ARE exclusions by meaning and **they belong to WP-2.0-S**"* (`docs/experiments/wp20m_design.md:189-190`).

`/usr/bin/grep -n -i "book\|opening" docs/experiments/wp20s_design.md` returns ten lines: the D-544 quotation at `:5`, the `openings_*` records at `:337,340,364,366`, the word "opening" as a verb at `:480`, `book_v2_registration.md` as a methodology precedent at `:495,497`, and the pilot's `book_v2` range at `:622`. **Not one decides whether a book position is labelled, flagged, or distinguishable.** The corpus carries no `is_book` column; `opening_turns` is on `Transcript` (`crates/pistol-arena/src/transcript.rs:39-40`) and is not read; §2.8's list of deliberate absences does not name it; §12's deferral list does not name it.

Forfeit treatment **is** decided (§2.7, §4, §2.8's `forfeit_by`); book treatment is the other half of the same clause and is missing. Both prior reviews raised it — `wp20_design_REVIEW.md` MAJOR 4 (*"the book exclusion is stated once, pinned by nothing, and its boundary is not fixed"*, `:421-450`, with the tree's own boundary at `replay.rs:137`, `at >= opening_turns`) and `wp20_design_REVIEW_rev2.md:597` (*"Whether book turns are labelled. §7 says no; INVARIANT 3 says every position"*). It is unanswered in a third round.

A trainer reading this corpus cannot tell a position neither engine chose from one both searched. That is a meaning question, and meaning is this package.

**FIX.** One paragraph in §2: either a `book` column (`opening_turns` is on the report and its boundary is `at >= opening_turns`) or a stated decision that book positions are labelled and indistinguishable, with the ground. Add the invariant and one test either way.

---

### BLOCKING 5 — the throughput expectation is in no package and in no deferral list

`docs/experiments/wp20_dispatches.md:95-96`, the last item of "Design decides and records":

> throughput expectation stated as a shape, measured in the pilot, never guessed (D-500's class).

`/usr/bin/grep -n -i "throughput\|per hour\|games per\|labels per"` returns **nothing** in `docs/experiments/wp20s_design.md` and **nothing** in `docs/experiments/wp20m_design.md`. `wp20s_design.md:621-625` (§12, "Not decided here, and correctly elsewhere") does not name it. `wp20m_design.md:675-680` (§11) does not name it. The pilot's pre-registration cannot inherit an obligation no design states, and D-483 does not excuse it: a **shape** is a mechanism, not a number — the dispatch's own wording draws that line, and the dead design satisfied it (`wp20_design_REVIEW.md:88`: *"**DECIDED** — §9, and §6 refuses to guess the memset cost. Clean"*).

This is the "falls between" defect the split exists to prevent, on a requirement the split's own governing text lists.

**FIX.** One paragraph in §12 or §5 stating the shape — records per capture record is 1:1 and the transform is a linear file pass whose cost is the replay per record; the pass-2 cost is WP-2.0-M's and is dominated by a `newgame` memset per position — and stating that the numbers are the pilot's. Or, if it is WP-2.0-M's, say so there and point here.

---

### BLOCKING 6 — the corpus is an artifact with no manifest, and requirement 5 / matrix decision 12 fall between the two packages

The dispatch's requirement 5 asks for a *"corpus manifest with digests"* (`wp20_dispatches.md:82-83`) and the matrix leaves *"the corpus manifest's digest boundary"* open as decision 12 (`matrix_wp20_pipeline_shape.md:358`). WP-2.0-M discharges it **for the capture**: *"`docs/label_corpus_manifest.md` holds one row per **capture**"* (`wp20m_design.md:742-752`).

`wp20s_design.md` produces a **different uncommitted artifact** — the corpus file — with its own body digest and its own identity, and says nothing about a manifest row for it. §12's deferral list does not name it. Hard rule 8: *"Nets, books, match logs, bench outputs are never committed; a committed manifest may sha-index them"* — and the corpus is the deliverable the whole arc exists to produce.

**FIX.** One paragraph: the labels mode prints a manifest row for the corpus (its body digest, the capture's `capture_sha256`, the report's two digests, the corpus schema version, the artifact path), on the same D-543 ground WP-2.0-M gives for printing rather than retyping, added to `docs/label_corpus_manifest.md` in the commit that records the run. Add a test on the row's shape, as WP-2.0-M does.

---

## MAJOR

### MAJOR 1 — §6 declines a registered mutant on a quotation the dispatch does not contain

`wp20s_design.md:426-428`: *"The dispatch's own wording is conditional — "seed ignored **where the pipeline samples**" — and the honest reading is that the clause does not fire on a pipeline that samples nothing."*

`docs/experiments/wp20_dispatches.md:105-107` reads: *"Mutants: schema field dropped -> loader test dies; **seed ignored -> determinism receipt dies**; ledger overwrite -> append test dies; census direction collapsed -> its test dies."* Unconditional. `/usr/bin/grep -n "sampl\|seed" docs/experiments/wp20_dispatches.md` returns `:80`, `:91`, `:105` and no such clause exists on any of them.

**Not BLOCKING, and I say why**: the conclusion survives on an independent and sound ground the same paragraph gives — D-540's *"seeds attach to pipeline SAMPLING only"* plus a pipeline that samples nothing (`:417-422`). The finding is that the document manufactures textual authority it does not need, in the exact class D-545 names, at the one place the citation checker cannot see it (the attribution carries a path and no line).

**FIX.** Delete the fabricated quotation. Say plainly: the dispatch registers the mutant unconditionally; this pipeline has no sampling site to host it; under D-540 a seed here would be a knob with nothing to choose; INVARIANT 5 and its ordering mutant replace it.

### MAJOR 2 — the pistol-core cross-check is not an externally derived referent, and §11's vacuity remedy rests on the same false ground

`:245-247`: *"This is `docs/process.md`'s "externally derived referent" — **pistol-core does not share the report's own arithmetic**, so the check is one the defect could falsify rather than one it preserves."*

It does share it: `game.rs:88-107` derives `result` from `state.make_turn(turn)`'s `Outcome` one-for-one, and `transcript::read` replays every list through the same `GameState::make_turn` at read time (`transcript.rs:359-379`). `docs/process.md:49-55` calls internal agreement between components sharing an input a criterion that passes vacuously, and `:69-72` names two-instruments-blind-to-one-stage explicitly. §11's remedy — *"its fixture reports are produced by the arena itself and their `result` fields are the arena's own"* (`:613-615`) — moves the shared input rather than removing it.

The check is **not worthless**: it can falsify a defect in `conclusion.rs`'s serialization and, more usefully, in the transform's own replay and mapping (it is the test that catches BLOCKING 3). But it must be claimed for what it is.

**FIX.** Restate §2.7 and §11: the referent is the arena's **recorded verdict**, the defect class excluded is a defect in **this transform's** replay, mapping and record assembly, and the check is explicitly **not** an independent check of pistol-core's win detection.

### MAJOR 3 — §8's rule defers exactly the two inputs D-518 fixed in advance, and one of its four inputs does not exist

`:474-488` registers four inputs and computes the minimum from them. Inputs 3 and 4 are *"the **significance level**"* and *"the **power**"* — the direct analogue of D-518's item 3, which `wp20_design_REVIEW.md:500-502` quotes: *"The threshold is registered here, **before the sweep**, precisely because moving it afterwards is the post-hoc threshold move CLAUDE.md forbids."* Deferring them to an unnamed, unowned future pre-registration is the same deferral MAJOR 6 killed the last §8 for, and the document's own anti-fitting ground — *"a rule whose four inputs come from a CLOSED arc cannot be tuned by the corpus it will be applied to"* (`:493-495`) — is simply false of a level and a power nobody has chosen. Neither is a measured number, so D-483 does not shelter them.

Input 2, *"the **target recall** the detector must beat for round 3 to be worth opening, taken from the same closed arc"* (`:480-482`), **has no referent**. The closed arc registers an incumbent (`0.571`) and a **bound** (`0.857` / `1.000`) — `docs/decisions.md:1130,1132,1144`, `docs/experiments/matrix_stage3_detector.md:613,634,645`. A bound on what a perfectly-fitted score could reach is not a target a detector must beat, and D-531 records that on trigger-rich per-search the two coincide (*"the gap is ZERO and the barrier is arithmetic"*). The clause carries no citation.

And the denominator is not countable: D-537 fixes *"win-proving firings on **DISJOINT POSITIONS**"* (`docs/decisions.md:1142`), the document defines two incompatible notions of position sameness (§2.1) and states that neither folds the other's equivalence (`:461-463`), and §8 never says which one disjointness uses. (BLOCKING 2 restores the key that would settle it.)

**FIX.** Fix α and the power **here**, with grounds, as D-518 fixed its own; replace input 2 with the registered figure that exists, or state that the target is a choice and fix it here too; name the key that decides disjointness and note that WP-2.0b's identity form must be consistent with it; add D-518's off-the-end clause (*if no candidate clears, the rule is amended in one reviewed amendment, never extrapolated*). Then §8's answer to D-544's sequencing question becomes true rather than asserted.

### MAJOR 4 — the corpus manifest
Folded into BLOCKING 6 above; recorded here only so the requirement table's row 5 has an owner.

### MAJOR 5 — a fourth mode and a third pass depart from D-542's recorded two-pass, three-arm mechanism with no ADR amendment

D-542 (`docs/decisions.md:1152`) selects *"row (g) — a labelling mode of the existing `arena` binary, **TWO-PASS**"* whose mechanism is *"a **third arm** in `bin/arena.rs`'s mode match beside `--config` and `--replay` … and writing one record per position"*. The matrix's row (g) says the same (`matrix_wp20_pipeline_shape.md:243-252`), and its coldness and seam arguments are scored on that shape.

`wp20s_design.md:53-55` adds a **fourth** arm and a **third** pass, and the record is written by a pass D-542 does not describe. The three grounds at `:71-78` are sound and I would take the same decision. What is missing is the acknowledgement: the document invokes hard rule 10 for the much smaller `pv` clause (*"hard rule 10 wants that amended rather than left describing something nobody ships"*, `:271-274`) and says nothing about the shape change. CLAUDE.md: *"Silent architecture drift is a breach; amend the ADR instead."*

**FIX.** One sentence in §1 naming the departure, and one more item in §12's residue so the closure's ADR line covers the pass count and the arm count, not only `pv`.

### MAJOR 6 — the wire carries six solver fields and two more totals fields the record neither takes nor names, and §2's own closure sentence says it does

`:200-201` describes the block as a pair. `crates/pistol-cli/src/report.rs:69-78` emits **six**: `search_nodes`, `solver_nodes`, `solver_firings`, `solver_invocations`, `solver_proofs`, `solver_root_nodes`. Together with `seldepth` and `hashfull` (`:83-84`), **eight** fields the capture preserves verbatim are neither columns nor named in §2.8's list of deliberate absences — while `:94-95` claims *"every column below answers a clause of it or is named in §2.8 as deliberately absent."* `solver_proofs` in particular is the numerator of the precision the detector arc exists to measure (`crates/pistol-search/src/info.rs:203-206`), and this is the package that also owns D-537's census-minimum rule.

The same fact makes §2.4's half-present refusal arbitrary: the block is atomic, so a line carrying `search_nodes` and `solver_nodes` without `solver_proofs` is exactly as impossible from `render_info` and is accepted in silence.

**FIX.** Name the eight in §2.8 with the reason each is not a column (one sentence covers `seldepth`/`hashfull`; the four call counters need their own, given §8). Restate §2.4's refusal over the block as a whole: *the solver block is all six fields or none; any other subset refuses the run by name.* Fix `:94-95` to say what it means.

### MAJOR 7 — `the_transform_spawns_no_process_and_reads_no_clock` is a test that cannot fail

`:561` registers it against INVARIANT 2 (`:536-537`). No mechanism is given, and no in-process Rust test observes the absence of a `Command::new` or an `Instant::now` on a path it does not take. `docs/process.md:47-52`: *"Recording without a criterion is a dry run nothing can fail."* WP-2.0-M met the identical situation and refused to register such a test, saying so on the document's face (`wp20m_design.md:590-599`).

**FIX.** Either replace it with a checkable proxy that a mutation kills (a byte-identity test between two runs separated by a controlled clock change, plus a source-level check in the tools/ suite), or delete it and say — as WP-2.0-M does — that INVARIANT 2's evidence is the diff, naming the modules the transform may reach.

### MAJOR 8 — the `to_move` mutant is a no-op and its test cannot die

`:603` registers *"`to_move` computed from turn parity"* against `side_to_move_comes_from_pistol_core_and_not_from_turn_parity` (`:585`). Game rule 3 makes the mover alternate strictly by turn; every asked prefix is a turn boundary at `Phase::First` (WP-2.0-M INVARIANT 1/2, and `Phase` at `crates/pistol-core/src/turn.rs:25-29`); the first turn being `Turn::Single` does not break alternation, and a winning `Turn::Single` can only be a game's last (`transcript.rs:369-376`). **Parity is exactly correct at every prefix**, so the mutation changes no output and the test passes under it.

§2.2's argument (`:143-145`) is a **rule-2** argument and a good one. It is not a behavioural one, and registering it as a behavioural mutant is registering a mutant that cannot die — the class `wp20m_design.md:546-549` names by example (*"which is a no-op because all three lookups already carry `?`"*).

**FIX.** Say plainly that parity and pistol-core agree on every legal prefix, that the reason to use pistol-core is rule 2 and not a difference in answers, and drop the mutant; or replace it with one that does change an answer (deriving `to_move` from the record's `turn` column without replaying, which diverges the moment a capture's `turn` and `moves` disagree — a condition §9 should then refuse).

### MAJOR 9 — §2.5 states a meaning for `depth_turns` that is false on the record class §2.8 admits exists

`:216-218` gives `depth_turns` a single meaning, quoting *"always a depth that was actually COMPLETED"* (`crates/pistol-search/src/info.rs:133-138`). On a solver-proof answer `depth_turns` is `tree.win_depth_turns()` — a **proof** depth (`crates/pistol-search/src/search.rs:785,793`) — and `Provenance::SolverProof`'s own doc says so: *"`depth_turns` is the proof's depth in turns"* (`crates/pistol-search/src/info.rs:259-261`). §2.8 concedes the class exists and gives the consumer `search_nodes == 0` to find it (`:262-263`), so the document contains both readings and reconciles neither.

This is the substance of `wp20_design_REVIEW.md` BLOCKING 1 (`:166-228`, memo decision 9): *"a `depth` column that is sometimes a search depth and sometimes a proof depth."* The **discriminator** half is now discharged and discharged well (Part 1(a)); the **meaning** half is not.

**FIX.** One clause in §2.5: `depth_turns` is a completed search depth except on a record with `search_nodes == 0`, where it is the proof's depth in turns — and the header's `note` block says so beside the three properties already there.

### MAJOR 10 — the `moves` column's spelling is wrong for the `k = 0` record WP-2.0-M guarantees every capture contains

`:101`: *"| 3 | `moves` | the canonical move list (D-6), as `position start moves …` spells it |"*.

WP-2.0-M asks the initial position and asks it differently: *"**The initial position is asked as bare `position start`**, never `position start moves` — which the engine refuses by name when no turns follow it. `exchange::position_line` produces the refused form for an empty slice, so **pass 2 does not use it for the empty case**"* (`wp20m_design.md:180-183`, cited there to `crates/pistol-engine/src/position_token.rs:84-98` and `crates/pistol-arena/src/exchange.rs:154-161`), and the range is *"`k` from zero to `len` inclusive"* (`:203`). So every capture holds a `k = 0` record whose position field is `position start`, and a corpus column defined as *"`position start moves …`"* does not describe it. The same record's `key_sym` is `canonical_sequence(&[])` — an empty `Vec<Turn>` (`crates/pistol-core/src/symmetry.rs:219-233`) — hence an empty TAB field, which the design never addresses.

**FIX.** One sentence in §2 fixing the `k = 0` record's spelling for `moves` and `key_sym`, and one loader rule saying an empty field is legal in exactly those two columns and nowhere else.

### MAJOR 11 — "the ONLY place in this workspace that tells `info totals …` from `info …`" is false

`:308-310`. `git grep -n "TOTALS_MARKER\|\"totals\"\|info totals" -- crates/ tools/`, `LC_ALL=C` sorted, finds recognisers in **`tools/sealbot/matchserver/src/pistol_client.rs:41,241`** (`const TOTALS_MARKER: &str = " totals ";` … `rest.contains(TOTALS_MARKER)`), `tools/bench_block.sh:260,264`, `tools/determinism.sh:170`, `tools/movetime_check.sh:125`, `tools/baseline_snapshot.sh:487`, and in `crates/pistol-cli/tests/{protocol_tests.rs:182,227, determinism_tests.rs:51}`. `crates/pistol-cli/src/report.rs:60` names the sealbot parser in so many words: *"the one substring parser in the tree (`tools/sealbot`)"*.

The claim scoped to `crates/pistol-arena` would be true, and that is the scope row (b)'s kill condition is about (`matrix_wp20_pipeline_shape.md:264-267`). As written it is a workspace-wide fact the workspace contradicts, and it is load-bearing for §3's "does not create a second reader" argument. It carries no citation.

**FIX.** Scope it: *"the only place in `pistol-arena` that tells `info totals …` from `info …`, which is the scope row (b)'s kill condition is drawn in."*

### MAJOR 12 — §1's ground (2) overstates WP-2.0-M's review state and its freeze

`:74-76`: *"It leaves WP-2.0-M's design untouched — **that document is reviewed and its §1 and §4 are frozen** (D-547), and adding an output to its mode would edit a frozen section from outside the package that owns it."*

WP-2.0-M revision 3 is **under review and has not passed**; revisions 1 and 2 both FAILED (D-545, `docs/decisions.md:1158`). D-547 freezes *a section a reviewer has PASSED*. WP-2.0-M's own freeze table (`wp20m_design.md:42-53`) freezes **paragraphs**, not whole sections: revision 2's §1 *"WHERE THE CODE LIVES"*, revision 2's §4 *"THE ONE NORMALISATION"* and *"THE SOURCE IS NAMED ON THE FACE OF THE FILE"*. §4.2 — the record's grammar, which §5 of this document leans on — is **new in revision 3** and is not frozen at all; `:55-58` lists further unfrozen material.

The decision survives on grounds (1) and (3), which are good. The ground as stated is a claim about a governing document that document does not make, and it is the same class as MAJOR 1.

**FIX.** Replace with what is true: WP-2.0-M owns its mode, its review is outstanding, and adding an output to another package's mode couples two design rounds — grounds (1) and (3) carry the decision without it.

---

## MINOR

1. **`:388`** — *"this crate already refuses a whitespace-bearing path"* citing `transcript.rs:124-131`. `path()` refuses an **empty** path (`:128-130`); the whitespace sentence is the comment's explanation of why the format cannot carry one. WP-2.0-M makes the same claim at `:339-340`; restating it here makes it this document's to own. FIX: *"…refuses a path this format cannot round-trip, on the ground that…"*.
2. **`:386-387`** — *"three of the fields are whitespace-bearing"*. Of the fourteen columns at most two are (`moves`, and `key_sym` rendered from a `Vec<Turn>`); `best` is a single turn token and `key_pos` 32 hex digits. The figure is WP-2.0-M's, about its own five-field record (`wp20m_design.md:336-337`). The same sentence claims not to restate §4.2 and then restates it (D-423). FIX: point and stop.
3. **`:219-220`** — *"which the protocol writes as "`bestmove <turn>`""*. `/usr/bin/grep -rn "bestmove <turn>" crates/` returns nothing; the substance is true at `report.rs:107-108`. FIX: drop the quotation marks or quote the doc that is there.
4. **`:66-69`** — *"claims `--out` **before the mode match**"*. `arena.rs:103` is after the match at `:82-100` and before the dispatch at `:104-107`. FIX: *"before the mode is dispatched"*. Same slip at `wp20m_design.md:158-161`. Relatedly, `:205-208` cites `SearchInfo::search_nodes`'s doc-comment for the fallback and not the site that enforces it (`crates/pistol-search/src/search.rs:513-514`) — the claim is true either way, but the enforcing site is the checkable one.
5. **`:62-63`** — the module is named but `crates/pistol-arena/src/lib.rs:47-69` gaining `pub mod labels;` is not stated, and neither is where the `--labels` usage paragraph lives once WP-2.0-M has moved `USAGE` to `usage.rs`.
6. **`:106`** — `score_value`'s type differs by kind (signed `i32` for `eval`, non-negative `u16` for the two mate kinds, `crates/pistol-search/src/score.rs:54-61`). The loader's *"a number spelled a way this format does not write"* (`:406-407`) does not settle it.
7. **`:103`** — `key_pos`'s rendering is unspecified. `Key128`'s `Display` is 32 hex digits, high half first (`crates/pistol-core/src/zobrist.rs:70-76`); the design should name it so the loader's token check is definable.
8. **`:391-393`** — the corpus header carries the capture's `capture_sha256` on trust; §9's refusal table never re-derives it from the capture's four inputs (`wp20m_design.md:392-400`), so a corpus can name a capture identity its body does not have.
9. **`:514-523`** — a report game with **no** capture records is not refused. §9's own ground, quoted from `replay.rs:16-19` (*"a criterion over SOME of a report's games is a criterion over a sample nobody registered"*), argues it should be.
10. **`:394-398`** — the three unit properties go in as `Fixture::note` lines, which render as bare `# <text>` indistinguishable from title lines (`crates/pistol-cli/src/corpus/emit.rs:20-28,51-58`). A machine reader cannot find them.
11. **§11** — nothing pins the header at all: a mutation dropping the schema version, the five digests or the three `note` lines kills no registered test (`a_corpus_file_round_trips_through_its_own_loader` is a body test; the schema-version test is the only header pin).
12. **`:538-541`** — INVARIANT 3's *"never their sum"* reads against `a_totals_line_without_solver_fields_yields_all_nodes_as_search_nodes` (`:565`). They agree (§2.4's arithmetic), but the invariant does not carry the resolution, and D-544's second defect was exactly a test whose name pinned an invariant's negation. FIX: *"…never their sum, the gate-off case excepted, where the sum and the first term are the same number."*

---

## "Could an implementer build from this without deciding something the design should have decided?"

**NO.** They would have to decide:

1. **How to read the score out of `fields_of`** — the specified return type cannot carry a two-token value (BLOCKING 1). Every label in the corpus turns on this.
2. **Whether a book position is a corpus record, a flagged record, or indistinguishable** (BLOCKING 4).
3. **What the outcome cross-check actually compares** — the sentence as written is wrong and an implementer must invent the correct relation (BLOCKING 3).
4. **Whether to carry a both-folds key** now that `canonical_form` is known to exist (BLOCKING 2).
5. **What the `k = 0` record's `moves` and `key_sym` fields contain** (MAJOR 10).
6. **What to do with `seldepth`, `hashfull` and the four solver call counters**, and whether a 3-, 4- or 5-field solver block refuses (MAJOR 6).
7. **How to write `the_transform_spawns_no_process_and_reads_no_clock`** so it can fail (MAJOR 7).
8. **How `key_pos` and `score_value` are spelled**, and whether an empty field is ever legal (MINOR 6, 7, 2).
9. **Whether `pub mod labels;` goes in `lib.rs`** and where the `--labels` usage paragraph lives (MINOR 5).
10. **Whether the corpus gets a manifest row, and what it holds** (BLOCKING 6).
11. **Whether a report game with no capture records refuses the run** (MINOR 9).
12. **Whether `capture_sha256` is verified or copied** (MINOR 8).

Everything else — the two-key answer, the score's three-way mapping, the two node columns and their fallback, the outcome's two columns, the schema/loader shape, the no-dedup policy, the no-seed policy, the refusal table — is decided, and decided well.

---

## The strongest attack that did not land

**I set out to show that §2.4's fallback is unsound and that §2.8's discriminator is the dead provenance proxy under a new name.** Both attacks failed, and it took reading the code rather than the docs to establish it.

The fallback attack: `crates/pistol-search/src/search.rs:451-465` and `:487-501` build salvage and fallback `SearchInfo`s with `nodes: 0, search_nodes: 0`. If either reached `totals_line`, a gate-off line could carry `nodes > 0` with `search_nodes` meaning something else, and §2.4's *"`search_nodes` is the line's `nodes`"* would be false. It cannot: `:513-514` overwrites both from the run before the outcome returns, and `:515-518` carries a REVIEW-impl note recording that the overwrite exists to keep the sum law. `total_nodes()` is `search_nodes + solver_nodes` (`crates/pistol-search/src/pvs.rs:150-152`), the block's absence means `solver_nodes == 0`, and `protocol.rs:173` renders the post-overwrite outcome. **`nodes == search_nodes` exactly, on every path.**

The proxy attack: the dead revision's condition was the block's presence, meaning *the solver was CONSULTED*. `search_nodes == 0` is strictly finer, and `solver_proof_outcome` is the only construction that produces it (`search.rs:795-796`); `search_nodes` is incremented per visited node (`pvs.rs:212`, `quiescence.rs:69,260`), and under a node budget the first iteration cannot be interrupted (D-74), so any non-proof answer has `search_nodes > 0`. **The discriminator is the proof condition, not the consulted condition.**

A third attack also failed: that widening `RecordedGame` with two **fatal** `value()` lookups would refuse reports the replay mode accepts — the D-545 shape, where a refusal rejects a hundred per cent of its inputs. `conclusion.rs:27-31,39` writes `forfeit_by` as the literal `none` when there was no forfeit, and `git log -S "forfeit_by {by}"` puts both fields in the game record since the arena's first commit (`1ad4070`, schema 1) against today's `REPORT_SCHEMA = 4`. **Safe.**

These three are the document's best work, and they are the findings the previous two rounds died on. The package has moved.

---

## What I could not settle by reading, and the run that would

1. **Whether `crates/pistol-arena/src/bin/arena.rs` admits a fourth arm under gate 17.** `wc -l` gives 283 against `SOFT_CAP=300` (`tools/file_justification_check.sh:65`), and WP-2.0-M's `USAGE` extraction frees `:16-58`. The arithmetic says yes with room. **The run**: `tools/file_justification_check.sh` (CI gate 17) at the post-implementation revision. I did not run it.
2. **Whether the `Vec<(&str,&str)>` split compiles and leaves the SPRT report byte-identical.** The lifetime elision and the alternating-tail analysis both say yes. **The run**: `cargo test --workspace --locked` plus `tools/arena_smoke.sh` (gate 15) on the split, comparing two runs' verdict blocks. Refused here — a CI run is in flight.
3. **Whether `a_rerun_over_one_capture_and_report_is_byte_identical` can actually kill its ordering mutant in one process.** It turns on whether two `HashMap`s in one process get different `RandomState` keys. **The run**: the mutant in a worktree with `cargo test`.
4. **Whether any committed or pilot config puts `solver.on_search_path = true` on pass 1**, which is what makes MAJOR 9's record class non-empty in practice. `wp20_design_REVIEW.md:200-205` recorded three such configs at that revision. **The run**: `LC_ALL=C /usr/bin/grep -rn "on_search_path = true" configs/ | LC_ALL=C sort` at the pilot's governing revision, against the config pass 1 names.
