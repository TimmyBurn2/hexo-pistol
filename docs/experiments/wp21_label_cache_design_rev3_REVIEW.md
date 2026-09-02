# REVIEW-design ROUND 3 — `wp21_label_cache_design.md` revision 3. **VERDICT: FAIL.**

**REVISION READ**: `docs/experiments/wp21_label_cache_design.md` **revision 3**, at
the dispatched commit **`f87cbbe`**.

**DOES IT STILL MATCH HEAD? YES.** `git rev-parse HEAD` →
`f87cbbe1fb9b4ac021206b061ad7936fcfa3d4e3`; branch `dev`; the design file's last
touch is `b9ab9f9`, two commits back, so the file is byte-identical at both. Every
`file:line` below is read at `f87cbbe`.

**READ**: `CLAUDE.md`; `docs/process.md`; `tools/SHELL_CHECKLIST.md`;
`docs/decisions.md` D-540, D-553, D-568, D-570..D-585 (D-576, D-581, D-583,
**D-584**, D-585 read in full); `docs/experiments/wp21_throughput_prereg.md`
revision 3 (§2, §4.2, §4.4, §4.5, §7.1, §8); `docs/experiments/wp21_prereg.md`
revision 4 (§1, §2, §4, §5); `docs/experiments/matrix_label_cache_key.md` revision
3; `docs/experiments/arc3_ledger.md`;
`docs/experiments/wp21_label_cache_design_REVIEW.md` (round 1) and
`..._rev2_REVIEW.md` (round 2) **in full**;
`crates/pistol-arena/src/{capture.rs,capture_file.rs,passes.rs,channel.rs,seats.rs,outpath.rs,labels.rs,usage.rs}`;
`crates/pistol-arena/src/bin/{arena.rs,stub_engine.rs}`;
`crates/pistol-core/src/{symmetry.rs,coord.rs,board.rs}`;
`tools/{ci.sh,determinism.sh,cold_label_check.py,governing_citation_check.sh,file_justification_check.sh,opening_prefix_fold.py}`
(the last **read, never run**); `docs/rule9_justifications.md`;
`crates/pistol-cli/tests/fixtures/random_openings_v2.txt`.

**NO `cargo` COMMAND WAS RUN.** The tree was not modified except this file.

---

## RE-DERIVATION — commands I chose, each with its scope

`docs/process.md`: *a count reproduced only by running the document's own command
is NOT reproduced.* None of the below is the document's command. The sixteen fold
numbers are re-derived from **my own** transcription of `symmetry.rs` into Python,
with **my own** partition arithmetic taken from `wp21_prereg.md` §2 — not from
`tools/opening_prefix_fold.py`, which I read for its definition and never
executed, and not from round 2's report, which I did not consult until after my
numbers were printed.

| # | command / instrument, with its scope | returns | the document says |
|---|---|---|---|
| **S1** | `/usr/bin/grep -n 'GATE_TOTAL\|step "gate\|gate ' tools/ci.sh \| LC_ALL=C sort -t: -k1,1n` — scope = every gate declaration in the script, plus `awk 'NR>=100&&NR<=112'` for the raw neighbourhood | `:21 readonly GATE_TOTAL=20`; `:106 step "gate 8…"`; `:109 step "gate 9/$GATE_TOTAL: cross-process determinism"`; `:110 gate "determinism" tools/determinism.sh`; last `:202 gate 20`; `:103-105` is a three-line comment block | §2 `:184-186` *"CI gate 9 of 20 (`tools/ci.sh:109-110`; the total is `:21`'s `readonly GATE_TOTAL=20`, and `:104-105` is the comment above the gate)"* — **REPRODUCED on all three halves.** Round 2's BLOCKING A is CLOSED. One imprecision, minor 12 |
| **S2** | my own Python transcription of `symmetry.rs:138-142` (`rotate`), `:90-100` (`checked_apply`, reflect-then-rotate), `:149-156` (`transform`, map-then-sort) and `:166-180` (`canonical_form`, least image), with `Coord` ordered `(q,r)` (`coord.rs:18-24`) and `Player` `P1<P2` (`board.rs:13-19`); own parser of the book's move-list encoding; own partition loop `skip=13`, `takes=[218]*15+[217]`, asserting the final `skip == 3500` (`wp21_prereg.md:161-168`) | per-tranche `k=2` merges **42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46**, **sum 792**; exact-class counts 213 209 210 213 212 215 216 212 213 210 215 217 213 214 211 210; slices `[13:231) … [3283:3500)` | §1.1 `:93-96`'s table — **ALL SIXTEEN REPRODUCED, AND THE SUM.** See *WHAT SURVIVED* 1 |
| **S3** | `/usr/bin/grep -n 'X2' docs/experiments/wp21_label_cache_design.md` — scope = the whole document | exactly **two** hits: `:217` (§3's table row) and `:296` (§5's mutant row) | §5 `:296` *"X2 \| the emptiness condition negated \| **X2's own test**"* — **there is no X2 row in §6.** See **BLOCKING 1** |
| **S4** | `awk '/enum Behave/,/^\}/' crates/pistol-arena/src/bin/stub_engine.rs \| /usr/bin/grep -cE '^\s{4}[A-Z][A-Za-z]*,'`, then each variant's arm read at `:414-553` | **18** variants; the answer path writes census rows at `:414-422` and the session's answers at `:543-548`, so `bestmove` is always last; `Behave::Exit` fires on the **first** `go` (`:515`) | §6 T4 *"a stray engine line is refused at the same prefix in both runs"* — **no engine in this tree writes an unsolicited line.** See **BLOCKING 2** |
| **S5** | `git grep -n 'unsolicited' -- crates \| LC_ALL=C sort` — scope = every crate | four hits: `capture.rs:241`, `channel.rs:187/198`, `exchange.rs:34`. **No test file anywhere** | X3's guard has zero coverage today and §5 registers its call-removed mutant as dying at T4 |
| **S6** | `git grep -n -i 'ask_count\|n_asks\|go_count\|hit rate\|hit_rate' -- crates tools`; `awk 'NR>=70&&NR<=100' crates/pistol-arena/src/passes.rs`; `cat -n crates/pistol-arena/src/usage.rs` | the pass prints `records.len()`, the game count, a manifest row and two path lines at `:82-96`; **no ask counter anywhere**; the only `hit rate` in the tree is `tools/label_cache_count.py:181` | §6 `:321-322` *"`arena --capture` prints `asks <n> records <n>` with the hit rate"* — nothing prints it today and **§1 registers no field that would**. See **MAJOR 5** |
| **S7** | `for r in 239f21f f1acc57 fde1497 f87cbbe; do git show $r:tools/governing_citation_check.sh \| /usr/bin/grep -c wp21_label_cache_design; done`, plus `/usr/bin/grep -n` at HEAD | `0 0 2 2`; at HEAD the entries are `:53` (the document) and `:63-64` (its `--proposes` test file) | header `:32-34` *"**The design is not on `tools/governing_citation_check.sh`'s list**, and these three would have been caught if it were"* — **false on both halves**, and D-584 says so. See **BLOCKING 4** |
| **S8** | `sed -n '1236p' docs/decisions.md` — D-584 in full; `/usr/bin/grep -c 'D-583\|D-584' <design>` | D-584 corrects (1) the 42-per-tranche error, (2) D-576's gate citation, (3) the *"gate 20 would have caught three citation findings"* claim. Design cites **D-583 three times, D-584 zero** | §9 item 2 `:370-371` *"MEASURED at **42 searches a tranche**, 0.72% of its misses (D-583)"* — the corrected line, cited as authority, four sections after the correction. See **BLOCKING 3** |
| **S9** | `/usr/bin/grep -n 'partition\|required=True' tools/cold_label_check.py`; `git show f1acc57 --stat` | `:42` *"`--partition` IS REQUIRED AND HAS NO DEFAULT"*, `:253 parser.add_argument("--partition", required=True, …)`; landed at `f1acc57` | §4 `:255-258` still *"`tools/cold_label_check.py` **gains** `--partition`, and it is **required** rather than defaulted"* — registered as future work against §1 row 8's *"already landed at `f1acc57`"*. See **MAJOR 9** |
| **S10** | `/usr/bin/grep -n 'MIN_SAMPL\|ten \|< *10' tools/cold_label_check.py` | nothing; the ten-sampled-record floor is **not** in the instrument | §1 row 8 and §9 item 3 both say it is still owed — **true**, and it is a GUARD with no mutant in §5 and no test in §6. See **MAJOR 8** |
| **S11** | `wc -l crates/pistol-arena/src/capture.rs`; `/usr/bin/grep -n SOFT_CAP= tools/file_justification_check.sh`; `/usr/bin/grep -c 'rule 9' <design>`; `docs/rule9_justifications.md:82` read in full | **389**; `SOFT_CAP=300`; **0**; the entry reads *"one pass, and the census sink is not a second subject. **Every line here is the capture ask** — what is sent, what the engine may say back, and what each of those answers means to the pass"* | round 1's MAJOR 12, still unmentioned; the package adds a memo, an enum, a hoisted guard, two counters and a twelve-image symmetry fold to that file. See **MAJOR 12** |
| **S12** | `awk '/^SEATS=\(/,/^\)/' tools/determinism.sh \| /usr/bin/grep -c '^\s*"'`; `/usr/bin/grep -n 'BUDGETS=\|LAYOUT_BUDGET' tools/determinism.sh` | **5** seats; `BUDGETS=("depth_turns 4" "nodes 200000")` plus `depth_turns-2 nodes-10000` and two `LAYOUT_BUDGET`s | §2's *"across **five** seats"* — **REPRODUCED**. *"neither of its budgets is `nodes 400000`"* — true but still undercounts (round 1's m20, OPEN) |
| **S13** | `/usr/bin/grep -c 'materially\|search_nodes\|D-584' <design>` | `2 / 0 / 0` | round 1's M7(iii) and M15 unchanged |

---

# THE DIFF TEST

`git diff --stat fde1497 f87cbbe -- docs/experiments/wp21_label_cache_design.md` →
**67 insertions, 23 deletions**, and `git diff -U0` gives ten hunks. Mapped onto
`/usr/bin/grep -n '^#' <design>`:

| section | lines at rev3 | hunks? | what round 2 left it owing |
|---|---|---|---|
| header | 1-46 | **YES** (2 hunks) | — |
| §1 change list | 48-62 | **YES** — rows **1a** and **1b** restored | BLOCKING C |
| §1.1 counters | 63-123 | **YES** — the sixteen floors, and *"42 is tranche one's"* | MAJOR E (first half) |
| §2.0 memo-not-a-parameter | 125-145 | **no** | minor M |
| §2.1 normalise side | 146-166 | **no** | MAJOR H |
| §2 the key | 167-209 | **YES** — one hunk, the gate citation | BLOCKING A |
| §3 the refusals | 212-243 | **no** | MAJOR K, minor O |
| §4 the cold check | 246-272 | **NO HUNK AT ALL** | round 1's M16 and B2(d) — **untouched for the third revision running** |
| §5 the mutation set | 275-298 | **YES** — bullets replaced by a table | BLOCKING D, MAJOR 11 |
| §6 the tests | 300-326 | **YES** — T2 rewritten, T6 fixed, two paragraphs added | BLOCKING B |
| §7 the rejection | 328-350 | **no** | minor L, minor N |
| §8 what it owes | 352-363 | **no** | MAJOR J |
| §9 what it does not do | 365-376 | **no** | MAJOR E (second half) |

**The header's own account is accurate as far as it goes** — it claims four moves
(§2's gate citation, §6's T2, §1's change list, §5's mutation set) and the diff
shows exactly those four plus §1.1. That is an improvement on revision 2, whose
*"every finding is disposed of"* the diff refuted.

**But the diff test finds the same defect one section over.** §4 has no hunk for
the third revision running, and — new at this revision — **§9 has no hunk while
§1.1 was rewritten to contradict it**. Revision 3 fixed *"42 per tranche"* in §1.1
and left *"42 searches a tranche"* standing in §9, eleven lines from the end of the
same file. That is BLOCKING A's shape (the document knowing better in the same
file) reproduced in a revision whose header is about having caught it.

---

# DISPOSITION

## Round 1 — all 22 findings, at revision 3

| # | round-1 finding | r2 | **r3** | evidence |
|---|---|---|---|---|
| **B1** | memo as a `&mut` parameter | CLOSED | **CLOSED** | §2.0 unchanged and still right |
| **B2(a)** | counters have no route out / no print site | PARTIAL | **PARTIAL** | §1 rows 3-4 give the route and the site; S6 shows the *ask count* and *hit rate* §6 depends on are registered in **no** §1 row. **MAJOR 5** |
| **B2(b)** | `Mode::Capture` arity / arm count | PARTIAL | **OPEN** | §1 row 5 byte-identical: *"the `--label-cache` word, and X1's refusal arms"*. No arity, no count. **MAJOR 11** |
| **B2(c)** | no test site | CLOSED | **CLOSED** | §1 row 7, §6's six rows |
| **B2(d)** | row 6 already landed | PARTIAL | **PARTIAL** | S9: row 8 is right; §4's prose still registers the landed `--partition` as future work. **MAJOR 9** |
| **B3** (normalise) | side unstated | PARTIAL | **PARTIAL** | §5 gains the raw-side mutant (the half round 2 asked for). §2.1 unchanged, so MAJOR H's two halves stand |
| **B3** (`no_tab`) | — | REJECTED, rightly | **REJECTED, rightly** | re-verified at `capture.rs:359`; see *ATTACKS REJECTED* 1 |
| **B4** | two guards stop running on 53% | CLOSED / argued | **PARTIAL** | X3 stands; its registered mutant is now shown untakeable (**BLOCKING 2**); census half still MAJOR K |
| **B5** | wrong gate named twice | OPEN | **CLOSED** | S1. Both sites correct at rev3 |
| **B6** | superseded registration revision | CLOSED | **CLOSED** | header names revision 3, §2.0, §2.1 |
| **M6** | `key_pos` is a zobrist | CLOSED | **CLOSED** | `labels.rs:202-203` re-verified |
| **M7(a)** | D-581's second conjunct | CLOSED | **CLOSED** | §1.1 `:107-110` |
| **M7(b)** | the `key_full` answer is measured | CLOSED | **CLOSED, and improved** | S2 reproduces all sixteen |
| **M7(iii)** | *"materially"* undefined | OPEN | **OPEN** | S13: 2 occurrences, both bare |
| **M8** | *"settles D-562(2)"* false | CLOSED | **CLOSED** | §1.1 `:112-116`, §9 item 4 |
| **M9** | play-order claim | CLOSED | **CLOSED** | §2 `:200-204` |
| **M10(a)** | counter cost unmeasurable | OPEN | **OPEN** | §1.1 `:86-87` unchanged; the dry run is `--label-nodes 2000` and the counters are unconditional, so no referent exists |
| **M10(b)** | cheaper formulation uncosted | OPEN | **OPEN** | unchanged |
| **M10(c)** | matrix says *"no extra search"* | OPEN | **OPEN** | MAJOR G unchanged; two documents on gate 20's own list still contradict each other |
| **M10 res.** | replay's error path must be `unreachable!` | OPEN | **OPEN** | unchanged |
| **M11** | mutation set incomplete | PARTIAL | **PARTIAL** | §5 rewritten; X2's mutant added but its test does not exist (**BLOCKING 1**), X3's cannot be taken (**BLOCKING 2**), the cold-check floor has none (**MAJOR 8**) |
| **M12** | hard rule 9 | OPEN | **OPEN** | S11: 389 lines, cap 300, zero mentions. **MAJOR 12** |
| **M13** | a cached capture leaves no trace | PARTIAL | **PARTIAL** | the printed ask count would be a trace — on stdout, which no artifact keeps; §1.1 says the run log carries the *counters*, not the ask count |
| **M14** | X1 enforced in the wrong crate | OPEN | **OPEN** | no in-library refusal anywhere in the document |
| **M15** | §6's disclaimers incomplete | PARTIAL | **PARTIAL** | S13: `search_nodes` still 0 against `wp21_throughput_prereg.md:551-556` |
| **M16** | §4 is in the wrong document | OPEN | **OPEN** | no hunk, third revision |
| **M17** | design on no citation gate | CLOSED in tree / FALSE in doc | **still FALSE in doc** | S7. **BLOCKING 4** |
| **m18** | *"the one shape gate 9 never takes"* | OPEN | **OPEN** | `:195-196` unchanged |
| **m19** | *"~885 ms"* priced against a hit | OPEN | **OPEN** | `:85`, `:208` unchanged |
| **m20** | *"neither of its budgets"* | OPEN | **OPEN** | S12 |
| **m21** | counters' memory uncosted | OPEN | **OPEN** | unchanged |

**Round 1 tally at revision 3: 9 CLOSED (+1 rightly rejected), 6 PARTIAL, 6 OPEN.**
Two moved (B5 CLOSED, M7(b) improved); one regressed in classification (B2(b)
PARTIAL → OPEN, because revision 3 rewrote §5 around a row 5 it did not touch, so
the arm count is now load-bearing for a mutant).

## Round 2 — all 15 findings, at revision 3

| # | round-2 finding | **r3** | evidence |
|---|---|---|---|
| **A** | §2 carries the wrong gate citation | **CLOSED** | S1. Both the fix and D-584's ADR line landed |
| **B** | T2 names a non-existent instrument | **PARTIAL** | T2 now reads a real stream (`arena`'s stdout), and §6 argues correctly that no other row can fail on a dead cache. But S6: nothing prints the count, **§1 registers no field or increment site for it**, and the design never says the count is taken at the ask call site rather than derived. **MAJOR 5** |
| **C** | §1 dropped the lookup and the insert | **PARTIAL** | rows 1a/1b restored with their position relative to `ask` ✓. Their position relative to the record literal at `capture.rs:352-358` — which round 2 asked for by name — is still absent, and two of §5's *"dies at"* cells point at test properties §6 does not register (**MAJOR 6**, **MAJOR 7**) and one at a test that does not exist (**BLOCKING 1**) |
| **D** | §5 byte-identical; X3 has no mutant | **PARTIAL** | §5 rewritten as a table; X3, X2, the normalise side and the mode all gain mutants ✓. Two of them cannot be taken (**BLOCKING 1**, **BLOCKING 2**) and the package's fifth guard has none (**MAJOR 8**) |
| **E** | the 42 is tranche one's | **PARTIAL** | §1.1 CLOSED and reproduced (S2). **§9 item 2 unchanged** — *"42 searches a tranche, 0.72% of its misses"*. **BLOCKING 3** |
| **F** | D-581's flip clause silently amended; *"materially"* | **OPEN** | §1.1 `:107-110` unchanged; no ADR amends D-581's *"materially above zero across the sweep"* to the floor-relative per-tranche form the design applies |
| **G** | contradicts the matrix and D-581 ×3 | **OPEN** | unchanged; `matrix_label_cache_key.md` is still revision 3 and still on `governing_citation_check.sh:51` beside this design at `:53` |
| **H** | §2.1 diverges from §4.2 unrecorded | **OPEN** | §2.1 unchanged, including the *"the bad case is a memo storing raw and normalising on the way out"* argument round 2 showed does not hold once the record is built at one place |
| **I** | X1's arm count unregistered | **OPEN** | §1 row 5, §3, §6 T3 all unchanged. **MAJOR 11** |
| **J** | §8 and the header false about the gate | **OPEN**, and now contradicted by an ADR of its own arc | S7, S8. **BLOCKING 4** |
| **K** | §3's mootness uses the premise §2.0 demolishes | **OPEN** | §3 unchanged |
| **L** | §7 cites `:355-360` for lines `:356-359` | **OPEN** | re-verified: `:355` is `position,`, `:360` is `out.push(record);` |
| **M** | §2.0's *"inside the closure"* at `:333` | **OPEN** | §2.0 unchanged; `:333` is outside the closure, which opens at `:338` |
| **N** | §7's *"registered mutant"* uncited | **OPEN** | §7 unchanged |
| **O** | X3's end-of-game residual unstated | **OPEN** | §3 unchanged |

**Round 2 tally at revision 3: 1 CLOSED, 4 PARTIAL, 10 OPEN.**

---

# BLOCKING

## BLOCKING 1 — §5's X2 MUTANT DIES AT A TEST THAT DOES NOT EXIST, AND X2's CONDITION IS UNREACHABLE FROM ANY INPUT `ask` CAN PRODUCE

§5, `:296`:

> | X2 | the emptiness condition negated | **X2's own test** |

S3: `X2` occurs **twice** in the whole document — `:217` (the §3 refusal table) and
`:296` (this cell). §6's rows are T1 (byte-identity), T2 (asks), T3 (X1's refusal),
T4 (X3's guard), T5 (the counters), T6 (order and fields). **There is no X2 row.**

This is round 2's BLOCKING D applied to four of its five bullets and not the fifth.
Round 2's exact words: *"X2 still appears exactly once in the document … with no
test in §6 and no mutant in §5. **A refusal with neither is prose.**"* Revision 3
added the mutant and not the test, so the refusal is now prose with a dangling
forward reference — strictly worse than revision 2, because §5 now asserts a
referent that a REVIEW-impl will look for and not find.

**AND THE MISSING TEST IS NOT AN OVERSIGHT — IT MAY NOT BE WRITABLE AS SPECIFIED.**
X2 refuses *"a hit whose cached `bestmove` or `totals` is empty"*. Read what can
enter the map. `ask` returns `Ok((totals, line))` at `capture.rs:273` only from
inside the `bestmove` arm at `:266`, so `bestmove` always begins
`"{BESTMOVE_PREFIX} "` and is never empty; `totals` is `Some(line)` from
`Step::Totals` at `:276-277`, and `classify` (`:186-203`) returns `Step::Totals`
only when `exchange::totals_of(line).is_some()`, so it is never empty either. **No
engine — honest, deviant or hostile — can cause an empty pair to be inserted.**
D-553's standing law is that a call-removed mutant *"must die at a test that drives
the call site with reachable input"*; X2's call site has no reachable input at all
through the pass's own front door.

That does not make X2 wrong to have — a guard over the map's own invariant is
defensible — but it makes the design owe one of three things it does not say:
(a) a `label_cache_tests.rs` row that drives an internal insert helper directly,
which requires the memo's insert to be a named function rather than an inline
statement (a shape §1 row 1b does not register); (b) a `stub_engine` behaviour that
makes an empty answer reachable, which requires a §1 row for `src/bin/stub_engine.rs`
that does not exist; or (c) an explicit statement that X2 is a `debug_assert`-class
invariant with no registered mutant, which contradicts §5's own row.

**FIX.** Add a §6 row — T7 — pinning X2, name the driver in it, and change §5's
*"X2's own test"* to *"T7"*. If (c) is the answer, say so in §3 and delete §5's row;
a mutant nothing can kill is the vacuous-criterion class at the level of the
mutation set.

## BLOCKING 2 — T4 REQUIRES AN ENGINE THAT WRITES AN UNSOLICITED LINE, NO SUCH ENGINE EXISTS IN THIS TREE, AND §1 REGISTERS NO STUB CHANGE — SO X3's MUTANT SURVIVES THE WHOLE SUITE

§6, `:309`, and §5, `:297`:

> | T4 | a **stray engine line** is refused **at the same prefix** in both runs — X3's guard, which is the one a hit would otherwise skip |
> | X3 | the hoisted guard REMOVED | **T4**: a stray line refused at the same prefix in both runs |

X3 is the guard **this package adds**, and T4 is the only thing registered to
notice its removal. Neither can be taken.

**(a) NOTHING IN THE TREE WRITES AN UNSOLICITED LINE.** S4: `stub_engine.rs` has
**18** `Behave` variants. I read every arm of the answer path. The census-row
deviations write at `:414-422`, *before* the session's own answers at `:543-548`,
so `bestmove` is always the last line of an ask; `Garbage` (`:516-519`),
`BadBestmove` (`:521-525`) and `Illegal` (`:527-538`) each write **one** line and
`continue`; `Hang` writes nothing; `Exit` (`:515`) returns on the **first** `go`.
`channel.rs:180-197` names the reachable case the guard exists for — *"every engine
that answers twice in one breath"* — and **no stub answers twice in one breath.**

**(b) THE GUARD HAS NEVER BEEN TESTED.** S5: `git grep -n 'unsolicited' -- crates`
returns four hits, all in `src/`, **none in any `tests/` directory**. The stray-line
refusal at `capture.rs:241-246` is uncovered today; hoisting it does not make it
covered.

**(c) EVEN GIVEN SUCH AN ENGINE, T4 AS WRITTEN NEED NOT KILL THE MUTANT.** With the
hoist removed, the guard reverts to `ask`'s first statement and still runs on every
**miss**. The cached and uncached runs then differ only if the stray line arrives at
a prefix that is a **cache hit**. §3's own X3 row says exactly this — *"a guard that
silently stops running on **53% of prefixes**"* — and §6's T4 does not require the
fixture to place the stray line in that 53%. A T4 whose stray line precedes a miss
passes with the mutant in place.

**This is BLOCKING B's class, unfixed one row over.** Revision 3's §6 explains at
`:313-319` why a test specified against a non-existent instrument is fatal, and
registers T4 against a non-existent instrument in the table three lines above.

**FIX.** §1 gains a row for `crates/pistol-arena/src/bin/stub_engine.rs`: a
`Behave` variant that writes a second line after `bestmove` (the tree's own
precedent for exactly this reasoning is at `stub_engine.rs:44-51`, `:52-58` — two
variants that exist *because* a guard's call was otherwise unreachable from any
test). §6's T4 states that the stray line arrives at a prefix the cached run serves
as a **HIT**. §5's cell then names T4 with that property.

## BLOCKING 3 — §9 STILL CITES THE NUMBER §1.1 SPENT ELEVEN NEW LINES CORRECTING, AND CITES AS ITS AUTHORITY THE ADR LINE THIS ARC CORRECTED IN THE SAME COMMIT

§1.1, `:93-105` (new at this revision):

> ```
> tranche  1  2 … 16   sum
> merges  42 51 … 46   792
> ```
> **EACH TRANCHE HAS ITS OWN FLOOR** … **REVISIONS 1 AND 2 SAID "at least 42 per
> tranche" AND 42 IS TRANCHE ONE'S** — the minimum across the sixteen … **D-583
> carries the same error and is corrected by its own successor line.**

§9 item 2, `:369-371` (unchanged since revision 1):

> what the folds are worth is **MEASURED at 42 searches a tranche, 0.72% of its
> misses (D-583)**.

S2 reproduces the sixteen floors independently: 42 is the value at exactly one
tranche, the mean is 49.5, the sum is 792. **Both numbers in §9's clause are
tranche-one figures presented as sweep facts**: `42/5819 = 0.722%` is tranche one's
miss ratio, and the sweep-wide figure is `792/93 076 = 0.851%`.

**AND THE CITATION IS TO THE CORRECTED LINE.** S8: D-584 (`docs/decisions.md:1236`)
opens *"FIRST, D-583's '42 SEARCHES A TRANCHE' IS TRANCHE ONE'S NUMBER"* and prints
the same sixteen values. D-584 landed at **`b9ab9f9` — the commit that carries this
revision of the design**. The design cites **D-583 three times and D-584 zero
times**, and §1.1's *"corrected by its own successor line"* does not name the
successor.

CLAUDE.md, D-423: *"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING — state it
once, in the section that owns it, and have every other section point there
instead."* This document now makes the claim twice, in two directions, seventy
lines apart — which is the precise defect its own header is about having caught in
revision 2's §2.

**FIX.** §9 item 2 → *"what the folds are worth is MEASURED per tranche at 42-60
merges, 792 across the sweep, under 1% of a tranche's misses (D-583 as corrected by
D-584, §1.1)"*, and §1.1's *"its own successor line"* names D-584.

## BLOCKING 4 — THE HEADER AND §8 TELL A SUCCESSOR THAT AN OBLIGATION IS OUTSTANDING WHICH IS DISCHARGED, AND CREDIT A GATE WITH A PROTECTION D-584 SAYS IT DOES NOT PROVIDE

Header, `:32-34`:

> **The design is not on `tools/governing_citation_check.sh`'s list**, and these
> three would have been caught if it were; adding it is §8's first obligation.

§8, `:356`:

> | **this document joins `tools/governing_citation_check.sh`'s list** | the IMPL
> commit — revision 1 carried three wrong citations **that the gate would have
> refused** |

Both halves of both sentences are false, and this revision's own arc says so.

**(a) IT IS ON THE LIST.** S7: `git show <rev>:tools/governing_citation_check.sh |
grep -c wp21_label_cache_design` returns `0` at `239f21f`, `0` at `f1acc57`, **`2`
at `fde1497`**, `2` at `f87cbbe`. At HEAD the entries are `:53` (the document) and
`:63-64` (its `--proposes` entry for `label_cache_tests.rs`). It joined **two
commits before this revision**, and §8 registers it as work the IMPL commit owes.
An implementer following §8 adds a duplicate line to a shell array.

**(b) THE GATE WOULD HAVE CAUGHT NONE OF THE THREE.** D-584's third correction, in
full: *"the claim that gate 20 'would have caught three of the review's citation
findings' is **FALSE**. `tools/design_citation_check.py` refuses a path the tree
does not hold and a line number past end-of-file; `tools/ci.sh:104-105` is **in
range** in a 205-line file, so the gate returns exit 0 on it."* And of the three,
*"CI gate 6"* is bare prose with no `file:line` for the checker to see at all.

**WHY THIS IS BLOCKING RATHER THAN A STALE SENTENCE.** §8 is the IMPL contract —
it is the table an implementer works from and a REVIEW-impl checks against. And the
header's claim is not merely stale, it is a **false statement about what protects
citations in this repository**, made in a document that exists inside the arc whose
whole subject (D-582, D-583, D-584) is that only a reviewer catches a
wrong-but-in-range citation. This design has now carried a wrong citation into two
consecutive reviews; the sentence that explains why says the mechanism that will
stop the next one is a CI gate, and it is not.

**FIX.** Header → *"the design joined the list at `fde1497`; gate 20 was green on
§2's wrong lines throughout, because `design_citation_check.py` checks only that a
range lies inside the file (D-584) — a reviewer found them, and D-582's reviewer
clause is the only thing that can."* §8's row → **discharged at `fde1497`**, and
replace it with the obligation that is actually outstanding.

---

# MAJOR

## MAJOR 5 — THE ASK COUNT IS THE HINGE OF THE WHOLE SUITE AND §1's "COMPLETE LIST" REGISTERS NEITHER A FIELD FOR IT NOR THE SITE THAT INCREMENTS IT

§6, `:321-326`:

> **SO THE ASK COUNT IS A REGISTERED OUTPUT AND NOT A DIAGNOSTIC.** `arena
> --capture` prints `asks <n> records <n>` with the hit rate … **The only way to
> pass T2 without a working cache is to lie about the count.**

§1 is headed *"WHAT CHANGES, AND **THE COMPLETE LIST IS SHORT**"*. Its two relevant
rows:

> | 3 | `capture.rs` | **the two coarser-fold counters**, and a `CaptureCounts` the pass returns beside its records |
> | 4 | `passes.rs` | `capture` threads the mode in and **prints the counts** — `:82-96` … |

Row 3 enumerates the struct's contents and the enumeration is *"the two
coarser-fold counters"*. **Neither the ask count nor the hit rate appears in any
§1 row.** S6: `passes.rs:82-96` today prints `records.len()`, the game count, a
manifest row and two path lines; `usage.rs` documents none of it; the only *hit
rate* in the tree is `tools/label_cache_count.py:181`, a reader of the finished
file.

**AND THE ARGUMENT AT `:324-326` HAS AN UNSTATED PREMISE THAT THE SIBLING TOOL
VIOLATES.** *"A cache that is dropped really does ask every time, so it reports
`asks == records`"* is true **only if `asks` is a counter incremented at the `ask`
call site (`capture.rs:343`)**. Derive it instead — count distinct `position` keys,
which is precisely what `tools/label_cache_count.py:167,175,179-181` does for the
same quantity, in the same package, and is the obvious thing to reach for — and a
dead cache reports `asks = 347 < records = 742` and **T2 passes green on a cache
that never ran**. That is not *"lying about the count"*; it is the natural
implementation, and it reinstates BLOCKING B exactly.

**WHAT THE CHANGE MUST ADD**, said concretely because the design does not: a `u64`
field on `CaptureCounts` incremented **immediately before the `ask(...)` call at
`capture.rs:343`, inside the miss branch only**, never derived from `out.len()` or
from the memo's length; a `records` figure that is `out.len()`; a `println!` in the
`:82-96` block; and a line in `usage.rs` saying what the numbers mean.

**FIX.** §1 row 3 enumerates the fields of `CaptureCounts` including `asks`; a new
row registers the increment site by line; §6's paragraph says *"counted at the ask
call site, never derived from the record list"* and names why (`label_cache_count.py`
computes the same name a different way).

## MAJOR 6 — §5's KILL CRITERION FOR THE ASK-COUNT MUTANT NAMES THE ONE ARM OF T2 THAT CANNOT KILL IT

§5, `:294`:

> | 4, the count | reports `records` rather than the asks actually made | **T2 in
> its uncached arm, where the two are equal**, and a second fixture where they are
> not |

Walk it. The mutant makes the printed `asks` equal `records.len()`. In T2's
**uncached** arm the true values are already equal, so the mutated and unmutated
runs print identical numbers and every assertion passes — **the uncached arm is
exactly where the mutant survives.** It dies in the **cached** arm, where T2
asserts `asks < records` and the mutant prints `742 < 742`. The cell names the
wrong half of its own test.

And *"a second fixture where they are not"* is not registered anywhere: §6 has six
rows and none of them is a second fixture. This is round 2's BLOCKING C complaint
(*"§5's kill criteria point at a test §6 does not have"*) surviving into the table
that was written to fix it.

**FIX.** → *"T2's **cached** arm: the mutant prints `asks == records` where T2
requires `asks < records`."* Delete the second clause or give it a §6 row.

## MAJOR 7 — §5 REQUIRES A PROPERTY OF T1's FIXTURE THAT §6's T1 DOES NOT REGISTER

§5, `:290`:

> | 1a, the lookup | keyed on something other than the asked `position` | **T1 on a
> report holding a transposition** |

§6's T1, `:306`, in full: *"a cached capture of a report is byte-identical to an
uncached one — §4.4's criterion in miniature, at a toy budget"*. **No transposition
is required of it.** A wrong-key mutant — keyed on the stone set, say — returns
another position's answer **only where two asked prefixes transpose**; on a fixture
without one it is behaviourally identical to the correct key and T1 passes.

The property is real and buildable (S2: the book's own prefixes fold at `k=2`, 42
times in tranche one alone), which is why this is MAJOR and not BLOCKING — but a
mutation receipt written at IMPL against this table would record a survivor and
have no registered fixture to blame.

**FIX.** §6's T1 states that its report holds at least one transposing pair and one
mirrored pair among its asked prefixes — or a new row T1b carries that population
and §5 cites it.

## MAJOR 8 — §1 ROW 8 REGISTERS A GUARD AS THIS PACKAGE'S WORK AND §5 GIVES IT NO MUTANT, §6 NO TEST, AND §1 ROW 7 NO TEST FILE

§1 row 8 and §9 item 3 both register the same outstanding change:

> What is still owed there is `wp21_prereg.md` revision 4's **ten-sampled-record
> floor**, which neither the tree nor revision 1's list carries

S10 confirms it is absent from `tools/cold_label_check.py`. §4 `:268-271` states
what it is for: *"**AN EMPTY CLASS IS A VOID AND NOT A PASS** … A filter over an
already-read list would otherwise print `0 of 0 … agree` and exit 0, which is the
vacuous pass `docs/process.md` forbids."*

**That is a guard, and it is the guard that stops T-A from passing vacuously on a
whole tranche.** D-553 is standing law: *"for every guard or invariant, the mutation
set includes a call-REMOVED mutant, and it must die at a test that drives the call
site with reachable input."* §5's table has eleven rows and none is this one. §6's
six rows do not reach it. §1 row 7 names `capture_tests.rs` and `label_cache_tests.rs`
and not `crates/pistol-arena/tests/cold_label_check_tests.rs`, which is where the
shipped checker's suite already lives (`docs/rule9_justifications.md:87`).

**FIX.** §5 gains *"the ten-record floor REMOVED — dies at a `cold_label_check_tests.rs`
case sampling a class with nine records and asserting VOID (exit 2), not exit 0"*;
§1 row 7 names that file; §6 either carries the row or says explicitly that this
mutant's test lives in the checker's own suite.

## MAJOR 9 — §4 REGISTERS AS FUTURE WORK THE THING §1 ROW 8 SAYS ALREADY LANDED, IN THE SAME DOCUMENT

§4, `:255-258`, unchanged for three revisions:

> `tools/cold_label_check.py` **gains** `--partition`, and it is **required**
> rather than defaulted

§1 row 8, `:61`:

> `tools/cold_label_check.py` | **already landed** at `f1acc57`.

S9: `f1acc57` added `--partition` with `required=True` (`:253`) and a header
paragraph at `:42` saying it has no default. Both statements are about the same
change; one is present tense and one is past. Round 2 flagged this as B2(d)'s
residual; §4 still has no hunk.

D-423 again: one claim, two sections, two directions. And it compounds MAJOR 16 —
§4 remains argument for an amendment to a *different* document
(`wp21_prereg.md`'s T-A), sitting in a design.

**FIX.** §4's paragraph → past tense with the commit, or §4 collapses to a pointer
at §1 row 8 and at `wp21_prereg.md` §4.

## MAJOR 10 — THE DESIGN PROVES THE GOVERNING REGISTRATION'S DRY-RUN LIMB 4 IS VACUOUS AND REGISTERS NO OBLIGATION TO AMEND IT

§6, `:316-319`:

> **The registration's own fallback cannot fail either**: `tools/label_cache_count.py`
> reads the capture file and returns the same number whether the run was cached or
> not … which is the criterion's whole point and makes it **useless as an existence
> check**.

That is correct and well argued. `wp21_throughput_prereg.md:770-771` registers dry
run limb **4** as *"`tools/label_cache_count.py` reports a hit rate above zero on
that capture, so the cache was actually exercised rather than merely present"* — a
limb the design has just shown cannot fail. And `:775-777` says *"limbs 2-4 [are
taken] when the cache package lands"*, so the vacuous limb is scheduled to be run
and recorded as evidence **after** this package lands.

§8's obligations table has no row for it. CLAUDE.md: *"an amendment reopens the
review however small the diff"* — so this is not a thing IMPL can quietly do; it is
a registration change with a review attached, and a design that discovers it owes
the successor that fact.

**FIX.** §8 gains a row: *"`wp21_throughput_prereg.md` §7.1 limb 4 is amended to
read the ask count `arena --capture` prints, and the amendment reopens that
document's review"* — or, if the intent is that T2 replaces limb 4, say that.

## MAJOR 11 — X1's ARM COUNT IS STILL UNREGISTERED AND §5's NEW X1 MUTANT NOW DEPENDS ON IT

Round 2's MAJOR I, unchanged: §1 row 5 says only *"the `--label-cache` word, and X1's
refusal arms"*; §6 T3 says *"refused naming both words"* without saying **in both
orders**. `bin/arena.rs:39-86` is a positional slice pattern (re-verified), so
`--census --label-cache` and `--label-cache --census` are two distinct spellings,
and the catch-all at `:79-85` refuses the unwritten one with a message naming
neither word and advising a reorder that cannot help.

**What is new at revision 3** is that §5 `:295` now registers *"X1 | **the arm
REMOVED**"* with the both-words kill criterion. Under two arms, removing **one** is
a mutant that T3 kills only if T3 exercises the spelling that was removed. With the
order unregistered, the mutation receipt is a coin flip.

**FIX.** §1 row 5 enumerates the five reachable spellings and marks the three legal
ones; §6 T3 asserts both refusal orders; §5's X1 row becomes one mutant per arm.

## MAJOR 12 — HARD RULE 9 IS STILL UNMENTIONED, AND THIS PACKAGE FALSIFIES `capture.rs`'s OWN JUSTIFICATION RATHER THAN MERELY LENGTHENING IT

S11: `capture.rs` is **389** lines against `tools/file_justification_check.sh`'s
`SOFT_CAP=300`; the design mentions rule 9 **zero** times. Round 1's MAJOR 12,
OPEN through three revisions.

The sharper half is not the count. `docs/rule9_justifications.md:82` justifies the
file on a subject claim: *"one pass, and the census sink is not a second subject.
**Every line here is the capture ask** — what is sent, what the engine may say back,
and what each of those answers means to the pass."* Into that file §1 puts: a
`LabelCache` enum, a `BTreeMap` memo, a lookup, an insert, a hoisted guard, a
`CaptureCounts` type, **and two counters that replay each miss's position and fold
it over twelve lattice symmetries**. A twelve-image symmetry fold is not *what is
sent, what the engine may say back, or what that answer means*. It is a second
subject by the entry's own test, and rule 9 says the justification is where that
argument lives — not a comment, and not silence.

**FIX.** §8 gains the rule-9 obligation, and §1.1 states in one sentence why the
counters belong in `capture.rs` rather than beside `labels.rs`'s existing
`canonical_form` call (`labels.rs:203`), which is where the tree already does this
arithmetic.

---

# minor

**minor 13.** §2 `:185`: *"`:104-105` is the comment above the gate"*. The block is
`:103-105` and it sits directly above **gate 8**'s `step` line at `:106`, not gate
9's at `:109` — its *content* describes the determinism gate, which is how the
mis-citation arose. The load-bearing half (it is prose, not the gate) is right.
D-584 inherits the same slight imprecision.

**minor 14.** §5's *"the counters | REMOVED, **and** INVERTED | T5"* is two mutants
in one row, and §5's own header promises *"one mutant per defect class per site"*.
Cosmetic against the table's other rows, which are one apiece.

**minor 15.** §6's T2 requires a fixture in which some `position` repeats. Within a
single game every asked prefix is distinct (`capture.rs:341-342`), so a
one-game report gives `asks == records` cached and T2 fails for a reason that is not
the cache. The fixture needs ≥2 games sharing an opening; §6 does not say so. Cheap
to discover (the test goes red immediately), which is why it is minor.

**minor 16.** §5 names `bin/arena.rs:79-85` inside a *"dies at"* cell. Verified
exact. But that makes §5 the only section of the document carrying a `file:line`
that `design_citation_check.py` will now track through rot, while §1 rows 1a and 1b
— the sites the whole table is specified against — carry none.

---

# WHAT SURVIVED ATTACK

1. **THE SIXTEEN FOLD NUMBERS ARE RIGHT, ALL OF THEM, AND SO IS THE SUM.** S2. I
   transcribed `rotate`, `checked_apply`, `transform` and `canonical_form` from
   `symmetry.rs` myself, took `Coord`'s `(q,r)` order from `coord.rs:18-24` and
   `Player`'s from `board.rs:13-19`, wrote my own move-list parser, and derived the
   partition from `wp21_prereg.md:161-168` with an assertion that the sixteenth
   slice ends exactly on `3500`. My output is
   `42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46`, sum **792** — the design's
   table, value for value. I did not run `tools/opening_prefix_fold.py`. The exact
   classes behind them (213, 209, 210, 213, 212, 215, 216, 212, 213, 210, 215, 217,
   213, 214, 211, 210) are also consistent with the design's *"213 `k=2` classes to
   171"* for tranche one. **This is the strongest thing in revision 3**: a number
   that was asserted for two revisions is now measured, per tranche, and it holds.

2. **THE GATE CITATION IS FIXED AND FIXED CORRECTLY, AT THE SITE THAT MATTERED.**
   S1. `:109-110` is the gate, `:21` is the total, `:104-105` is prose. Round 2's
   BLOCKING A named §2's body specifically, and §2's body is what changed. The ADR
   line round 2's FIX also asked for exists (D-584, second correction).

3. **§6's CENTRAL ARGUMENT IS SOUND, AND I TRIED TO BREAK IT.** *"T2 is the only row
   a dead cache fails"* — I walked all six rows against the motivating mutant
   (`bin/arena.rs` parses the word, `passes::capture` drops it) and reached the same
   answer round 2 did: T1 and T6 pass on the uncached bytes, T3 is a CLI arm, T4's
   guard is hoisted unconditionally, T5's counters are unconditional. And the
   demolition of the registration's own limb 4 is correct: `label_cache_count.py`
   reads the finished file, which T1 requires to be identical either way. **Reading
   the count off `arena`'s own stdout is the right instrument** — the second of the
   two options round 2 offered, and the better one, because it also puts a trace of
   the cache into the operator's transcript (round 1's MAJOR 13). What it lacks is
   the increment site (**MAJOR 5**), not the idea.

4. **§1 ROWS 1a AND 1b DO WHAT ROUND 2 ASKED, ON THE HALF THEY ADDRESS.** The
   lookup is *"before `ask`"*, the insert *"after a miss's `ask`"*, and 1b fixes the
   post-`normalise` side so it agrees with §2.1. The hit path's rejoin at the record
   literal is still not drawn — but it is now **derivable** in one step rather than
   two: §2.1 fixes the memo on the normalised side and `capture.rs:69-71` errors on
   a second `normalise`, so an implementer who routes a hit through
   `capture.rs:356` gets a loud failure at the first hit, not a silent one. Round
   2's BLOCKING C is genuinely reduced.

5. **THE MUTATION TABLE'S SHAPE IS RIGHT, AND SEVEN OF ITS ELEVEN ROWS ARE
   NON-VACUOUS.** 1a-REMOVED, 1a-INVERTED, 1b-REMOVED, 1b-RAW, the mode, X1 and the
   counters each name a defect that a named §6 row actually distinguishes. The
   1b-RAW row is the best of them: a raw-side memo writes ` nps <n> time <n>` into
   53% of records and T1's byte-identity fails on the first hit. The table is a real
   improvement on revision 1's bullets, which named two tests that did not exist.

6. **X3's HOIST IS STILL THE RIGHT MECHANISM.** I re-read `ask` (`capture.rs:231-319`)
   and reached round 2's conclusion independently: `where_`'s two captures are in
   scope at `run`'s `:340-341`, the guard is `ask`'s first statement, `ask` is
   private with exactly one call site (`:343`), and everything else inside `ask`
   fires only in response to lines written for that ask. The design's *"the uncached
   and cached passes must refuse the same input at the same place"* is the correct
   requirement. What fails is that nothing registered can check it (**BLOCKING 2**).

7. **§7's REJECTION OF THE `no_tab` FINDING IS STILL RIGHT AT REVISION 3.** Verified
   at the file, not from the quote: `capture.rs:352-358` is the record literal,
   `:359` is `no_tab(&record)?`, `:360` is `out.push(record)`, and `ask` ends at
   `:319`. A hit that produces a `(totals, bestmove)` pair before the literal passes
   through `:359` like any miss. **The rejection holds and its reproducer is the
   right one.** Only its citation is off by one at each end (round 2's minor L,
   still open) and the *"registered mutant"* it leans on is still uncited (minor N).

8. **`742 / 347 / 53% / 47% / 79 / five seats`** — I re-checked the two that are
   cheap from source: `determinism.sh` has exactly 5 seats (S12), and `turn_cap = 40`
   gives `1 + 39×2 = 79`. The pilot figures I did not re-take; round 2 reproduced
   them from the artifact under its own parser.

9. **THE ARGUMENT THAT `BTreeMap` OWES NOTHING TO RULE 4** is right and is now the
   registration's own reason too (`wp21_throughput_prereg.md` §4.2 corrects revision
   2's wrong reason for the same choice). Nothing to attack.

---

# ATTACKS I ATTEMPTED AND REJECTED

1. **"§7's rejection fails because the hit path must skip the record literal to
   avoid double-normalising."** REJECTED. §2.1 stores the post-`normalise` pair, so
   the hit path supplies `totals` already stripped; the implementer moves
   `normalise` to the miss branch and the literal keeps a plain `totals` binding.
   `no_tab` at `:359` is downstream of the literal either way and is not skipped.

2. **"T5's zero-counter arm is unachievable because every game shares the `k=1`
   prefix."** REJECTED, on my own fold rather than on the design's word: at `k=1`
   there is one class under every key (every opening starts `0,0`), and the counters
   count collisions among **misses**, of which there is exactly one at that depth.
   S2's `k=2` exact-class counts (213 of 218, etc.) confirm the fold is nontrivial
   only from `k=2`.

3. **"The ask counter breaks `wp21_throughput_prereg.md:384`'s registered claim that
   `arena` prints one line per capture pass, not per label."** REJECTED. That
   sentence is about there being no per-label wall-clock quantity; a second summary
   line per pass is still one line per pass, and the registered mean is unaffected.

4. **"Adding a stdout line breaks a downstream parser."** REJECTED. `git grep` over
   `tools/` and `crates/*/tests` finds three consumers of capture-pass stdout
   (`capture_tests.rs:455`, `census_capture_tests.rs:160`, `labels_tests.rs:869`),
   all of which search for a manifest row rather than assert the full stream.

5. **"`Behave::Exit` can drive T4, so BLOCKING 2 is wrong."** REJECTED — and the
   attempt is what makes BLOCKING 2 stronger. `unsolicited()` does map a
   disconnected pipe to `Some(Received::Closed)` (`channel.rs:203`), but `Exit`
   fires on the **first** `go` (`stub_engine.rs:515`), and the first asked prefix of
   the first game is always a **miss**. There is no stub that answers N asks and
   then dies, so the disconnect can never be reached at a hit prefix either.

6. **"`--label-cache` is a code-side default and breaches hard rule 1."** REJECTED,
   as both prior rounds rejected it: `usage.rs:14`'s `[--census]` is the tree's
   convention for a capture-mode switch, and a mode selector is not a tunable.

7. **"The counters' per-miss replay is a second `GameState` construction the design
   hides."** REJECTED as a *correctness* attack — the design states the cost openly
   at §1.1 `:83-87` and prices it. It survives here only as MAJOR G's unrecorded
   contradiction with the matrix and D-581, which say *"no extra search"*.

8. **"§1 row 1's `:333` placement cannot work because `with_seats` needs an
   `FnMut`."** REJECTED. `seats.rs:25` takes `impl FnOnce(&mut [Channel; N])`,
   called once at `:55`. Both placements compile; only §2.0's prose names two
   (minor M).

---

## THE SHORT VERSION

Revision 3 moves five sections and every move is in the right direction. The gate
citation is fixed at the site that mattered; the sixteen fold floors are measured
and **all sixteen reproduce under an implementation I wrote from the Rust source**;
the change list has its lookup and insert back; the mutation set is a table with
named kill criteria instead of two references to tests that never existed; and T2
is now specified against a real stream, with §6 carrying the clearest paragraph in
the document about why it is the only row that can fail.

It fails on four things, and three of them are the same defect this revision's
header is about having caught in revision 2 — **a claim the document makes twice,
in two directions**. §5's X2 mutant dies at a §6 row that does not exist, and X2's
condition is unreachable from any input `ask` can produce (**1**). T4 needs an
engine that writes an unsolicited line, no such engine exists among
`stub_engine.rs`'s eighteen behaviours, `channel::unsolicited` has no test anywhere
in the tree, and §1 registers no stub change — so X3, the guard this package adds,
has a registered mutant that survives the entire suite (**2**). §9 still prints the
per-tranche number §1.1 spent eleven new lines correcting, and cites as its
authority the ADR line that this very commit's D-584 corrected (**3**). The header
and §8 still say the design is not on the citation gate's list and that the gate
would have caught its wrong citations; it joined the list two commits ago and D-584
says the gate catches nothing wrong-but-in-range (**4**). Around those, the ask
count that the whole suite now hinges on has no field and no increment site in a
section headed *"the complete list"*, and deriving it the way the package's own
sibling tool does would make T2 green on a dead cache.

Of round 1's 22 findings, 9 are CLOSED and 12 are still PARTIAL or OPEN; of round
2's 15, one is CLOSED and 14 are PARTIAL or OPEN. **§4 has had no hunk in three
revisions.**

**VERDICT: FAIL** — 4 BLOCKING, 8 MAJOR, 4 minor. **This is round 3 of five
(D-585)**, so this is not a stop. What remains for round 4 is small and mechanical
in three of the four blocking cases: §9's sentence, the header's and §8's two
sentences, and a §6 row for X2 are each one edit. **BLOCKING 2 is the only one that
needs a decision** — whether the stub gains a behaviour (a §1 row and a `Behave`
variant) or X3's mutant is retired with an argument. Take that decision first; the
rest of this report is text.
