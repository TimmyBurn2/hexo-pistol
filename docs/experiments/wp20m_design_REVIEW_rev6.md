# REGRESSION RE-CHECK — `docs/experiments/wp20m_design.md` revision 6

## Header

- **Named revision adjudicated:** `ff1c575beaf93b056a39e0a9d3ed3f101bf676d4`
  (`docs(wp20m): the correction round on a passed design — two mutants that
  could not die are replaced, three drivers named, and the false universal about
  committed configs is deleted`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` → `ff1c575…`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty** at the start of this
  check. Nothing adjudicated here is uncommitted; HEAD did not move.
- **Prior revision compared against:** `41a52f0` (revision 5, the PASSED
  revision), read via `git show 41a52f0:docs/experiments/wp20m_design.md`; and
  `5064b05` (revision 1) and `7af62e7` (revision 2) for the freeze checks.
- **Read as binding:** `docs/experiments/wp20m_design_REVIEW_rev5.md` in full
  (M-1 … M-9, all four Parts), `git show 41a52f0:docs/experiments/wp20m_design.md`,
  CLAUDE.md, `docs/experiments/wp20_dispatches.md` at HEAD, and D-543, D-544,
  D-546, D-547, D-548 (plus D-441, D-465 and D-520, which turned out to govern
  finding F-4).
- **What I ran:** `git`, `git grep`, `/usr/bin/grep`, `sed`, `awk`, `diff`,
  `comm`, `sort`, `tr`, `wc`, `cat`, and `python3 tools/design_citation_check.py`
  with the four `--proposes` paths. Every recorded grep is `/usr/bin/grep` or
  `git grep` (D-265).
- **What I refused to run, per the dispatch:** `cargo` in any form,
  `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Where a claim
  needs a run I name the run at the end.
- **Scope:** the seven questions the dispatch put and nothing wider. I did not
  re-open anything revision 5's reviewer considered and passed.

---

## VERDICT: **FAIL** — **0 BLOCKING, 0 MAJOR, 4 MINOR.**

**THE ROUND'S BINDING QUESTION IS ANSWERED FIRST AND THE ANSWER IS NO.
**REVISION 6 BROKE NOTHING REVISION 5 HAD, AND SPENT NOTHING.** I diffed
`41a52f0` → `ff1c575` completely — **thirteen hunks at `-U0`** — and asked of
each whether something that was pinning a rule was removed. **Nothing was.**
The freeze held mechanically: **not one quoted line moved** (118 `^>` lines at
each revision, `diff` empty), **§12 is still word-identical to `5064b05`'s §4**
(152 words each, `diff` empty), **every hunk has a §0.2c row**, and the
mechanism, invariant and decision sets are untouched but for one ADDED test and
one ADDED mutant. `design_citation_check.py` is green over **123** citations
(116 at revision 5); the four new citation TARGETS are hand-verified below.
**No mechanism is unguarded that was guarded, and no test lost a driver.**

**Why this is nevertheless a FAIL under the dispatch's own calibration —
*"PASS unless a correction is wrong, incomplete, or spent something."*** Two of
the nine corrections did not land, and **§0.2c records both as landed**:

- **M-8 is applied at two of five sites** and `:176` says five — and the site it
  skipped is the one the rev-5 reviewer singled out as the only one that does
  **not** resolve uniquely.
- **M-9's remedy is nowhere in the document** — `/usr/bin/grep -n "N3"` returns
  two lines and neither is a table row — and `:177` says *"this table has a row
  for every change, N3's included."*

and one landed against a mutation its own registered driver cannot produce:

- **M-4's replacement mutant cannot die under the stub**, which is the driver
  the row carries, for a structural reason I traced to `Turn`'s own parser.

**This is the M-1/M-2 defect of one round ago, in the block that replaced the
block M-1/M-2 were found in.** §0.2b over-stated two of nine rows; §0.2c
over-states two of nine rows. That is the whole of the FAIL. **All four findings
are one clause each, none is a mechanism, and the remedy is four edits.**

---

# PART 1 — THE SEVEN QUESTIONS, ANSWERED BY NAME

## Q1. Is each of M-1 … M-9 applied, and correct against the tree?

**Six of nine are applied and correct. Two are not applied (M-8 partially, M-9
not at all). One is applied but ineffective (M-4).**

### M-1 — §0.2b's N4 row drops the withdrawn leg — **APPLIED, CORRECT**

`:152` now reads *"rev 5: the diff and gate 15 — unchanged; only the sentence is
true now. (Revision 5's own row named INVARIANT 7's test as a third leg, which §9
withdraws by name; the row over-stated its own coverage and is corrected here)"*.

Checked against §9 rather than against the row: `:813-828` gives INVARIANT 10's
evidence as **the diff** (*"the only file whose SPRT-path BEHAVIOUR this package
changes is `crates/pistol-arena/src/exchange.rs`"*) **plus CI gate 15**
(`tools/arena_smoke.sh:22-27`), and then says *"**Revision 3 also leaned on
INVARIANT 7's test here and that leg is withdrawn**"* (`:825-826`). **The row now
names exactly the two legs §9 names, and the parenthetical states the correction
truthfully.** The rev-5 FIX was *"the diff and gate 15 — unchanged; only the
sentence is true now"* and that is verbatim what landed, with a disclosure added.

### M-2 — the mutant and driver name the refusal message's field walk — **APPLIED, CORRECT, AND IT MAKES A THIRD SENTENCE TRUE**

Mutant `:918`: *"`id_lines` dropped from **the refusal message's field walk**"*.
Driver `:855`: *"**unit, over the refusal message's field walk**"*. Both were
byte-identical to revision 4's *"the identity comparison"* before this round;
`/usr/bin/grep -n "identity comparison"` over the document now returns **zero**
occurrences outside §0.2b's and §0.2c's own descriptions of the correction
(`:153`, `:171`).

**Can the replacement die? YES, and it is a mutation the code can take.** §3.1
(`:381-385`) separates the two: *"detection is `identities[0] == identities[1]`
and needs no new predicate; the REFUSAL MESSAGE then walks the four fields to name
the one that differed."* Deleting the `id_lines` limb of that walk is a one-branch
deletion, and the test's own name —
`two_identities_differing_only_in_an_id_line_are_refused_naming_that_line` —
asserts the message names that line, so it fails. **It dies.**

**Could the replaced one? NO, and I verified the ground rather than taking it.**
`crates/pistol-arena/src/identity.rs:11-22` is
`#[derive(Debug, Clone, PartialEq, Eq)] pub struct EngineIdentity` over four
fields. *"Dropping `id_lines` from the identity comparison"* is not a mutation of
that: it requires replacing a `derive` with a hand-written `impl PartialEq`, or
removing the field, which does not compile against `identity.rs:74-82`'s
constructor. **The old row named a site with no separable limbs; the new one names
a site made of limbs.**

**And a bonus the round did not claim:** §3.1's own closing sentence
(`:384-385`) — *"the mutant table registers a comparison with separable limbs"* —
was **false at revision 5** and is **true at revision 6**. The correction repaired
a claim outside the row it was aimed at.

### M-3 — §0.2's §6 row says four rows, not three — **APPLIED, CORRECT, COUNT VERIFIED BOTH ENDS**

`:103` now reads *"**EDITED**: **four rows added**, and row 1 is reworded … The
third row is the malformed totals line (**m9**) and the fourth is the write-side
TAB refusal (rev-4 review **N7**). Revision 5 said three, which is the count
defect N3 named one round earlier in this same block"*.

**Both ends counted at the file, not read off the row.** Revision 2's §6 table
(`git show 7af62e7:… | sed -n '160,175p'`) has **five** condition rows; HEAD's
§6 table (`:638-646`) has **nine**. **Nine minus five is four.** The row is
correct, it names which four, and it says whose count it is correcting — the
shape N3 asked for.

### M-4 — the `bestmove` mutant becomes one the code can take — **APPLIED, BUT THE REPLACEMENT CANNOT DIE UNDER ITS REGISTERED DRIVER — F-3**

Applied at `:913`. The row is a strict improvement on what it replaced. **It still
does not die under the stub**, and the stub is what the row's driver column says.
Full evidence under F-3.

### M-5 — `an_unrecognised_totals_line_…`'s driver named — **APPLIED, CORRECT**

`:874`: *"**unit, over the totals-line recogniser, synthetic** — no stub
behaviour can emit a malformed totals line, because every stub answer goes
through `pistol_cli::Session`"*. That is the rev-5 FIX verbatim plus its reason,
and the reason is true: I re-read all eleven `Behave` variants
(`crates/pistol-arena/src/bin/stub_engine.rs:13-54`) and `deviate`
(`:354-361`, which rewrites `id protocol ` and nothing else); every non-deviant
answer is produced by `pistol_cli::Session` (`:266`) and therefore by
`render_info`'s one format literal (`crates/pistol-cli/src/report.rs:82-84`),
which is always well-formed.

**The site is real and identifiable.** `totals_of`
(`crates/pistol-arena/src/exchange.rs:169-188`) is the recogniser; today a line
that strips the `info totals ` prefix but fails a lookup returns `None` and falls
through to `exchange.rs:76-79`'s *"an ordinary `info` line → `continue`"*, which
is exactly the mutant at `:923`. **Pass 2's own classifier already has the game
and turn in scope** — `channel.receive(rules.hang_timeout_ms, opening, turn)`
(`exchange.rs:46`) threads both — so a unit over the recogniser-plus-refusal can
assert the naming half of the test's name. **No branch of the authoring choice is
wrong.**

### M-6 — the no-score test's unit named — **APPLIED, CORRECT**

`:863`: *"**unit, over the totals-line recogniser, synthetic** — §6's last row,
unpinned across three revisions. **Not over the normalisation, which is a
`fn(&str) -> String` and cannot refuse anything**"*. The clause the finding asked
for, plus the reason the finding gave. `totals_of` has three `?` lookups —
`nodes`, `time`, `depth_turns` — and **no `score` lookup**
(`exchange.rs:186-188`), so a score-less line is recognised today and the mutant
(*"a score-less totals line refused instead of captured"*, `:910`) is an ADDED
check on that path. **It dies at a unit over that path.**

### M-7 — `Received::Overlong` gains a test and a mutant — **APPLIED, CORRECT, AND THE MUTANT DIES**

Test `:873`: `an_overlong_non_line_refuses_the_run_by_name`, pins **9**, driver
*"**unit, over the failure mapping** — no stub behaviour writes past
`MAX_LINE_BYTES` without a newline"*. Mutant `:922`: *"an overlong non-line
mapped to the `Closed` refusal rather than its own"*.

**The mutation is takeable and the test dies on it.** `exchange::ask` has two
adjacent arms: `Received::Closed => closed(channel, ForfeitReason::EngineExited, …)`
(`exchange.rs:46-48`) and `Received::Overlong => … ForfeitReason::ProtocolError`
with its own message (`:49-56`). Collapsing the second into the first is a
one-line edit, and a test asserting the refusal is named for the overlong line
fails on it. **The driver claim is true too:** no `Behave` variant writes
≥ `MAX_LINE_BYTES` without a newline — every write is a `writeln!`.

**And §9 is now consistent without being edited.** `:809-811` promises *"TWO
THINGS NO TEST PINS"*; the Overlong row was rev-5's undeclared third, and it is
now pinned, so the count is right where it stands. **This is the one correction
that closed a rule with neither test nor mutant.**

### M-8 — five quotations name the WP-2.0 dispatch — **APPLIED AT TWO OF FIVE — F-1**

### M-9 — §0.2b gains an N3 row — **NOT APPLIED — F-2**

---

## Q2. Did any correction SPEND A TRUE THING? — **NO. I looked hunk by hunk, independently of §0.2c.**

`git diff -U0 41a52f0 ff1c575 -- docs/experiments/wp20m_design.md` → **thirteen
hunks**. Each mapped by hand, then asked the D-544/D-548 question.

| hunk | § | what it does | anything removed that was pinning a rule? |
|---|---|---|---|
| `-3,2 +3,17` | header | revision banner + the session's own disclosure | no — added text; revision 5's D-548 paragraph is kept below it |
| `-88 +103` | §0.2 freeze table, §6 row | *three* → *four rows added*, with which four | no — the row gains an enumeration |
| `-137,2 +152,2` | §0.2b, N4 and N5 rows | M-1, M-2 | **N4 loses the words "INVARIANT 7's test"** — and that leg was already withdrawn at §9 `:825-826`, so nothing pinning was removed; the two legs that remain are the two §9 names |
| `-146,0 +162,20` | §0.2c, **new** | the block itself | no |
| `-516 +551` | §4.3 | *the standing* → *the WP-2.0* dispatch | no |
| `-820 +855` | §10 driver | M-2 | no — a driver column gains precision |
| `-828 +863` | §10 driver | M-6 | no — a driver column gains a unit |
| `-838 +873,2` | §10 test table | M-7 (**+1 test**), M-5 (driver) | no — strictly additive |
| `-877 +913` | §10 mutant | M-4 replacement | **the old row is removed**; it was the identity function on every input, so nothing was pinned by it. See F-3 for what the replacement does and does not buy |
| `-882 +918` | §10 mutant | M-2 replacement | the old row named a site with no limbs; the new one names a site with limbs. **Net coverage strictly greater** |
| `-885,0 +922` | §10 mutant | M-7 (**+1 mutant**) | no — additive |
| `-928 +965` | §11 | *the standing* → *the WP-2.0* dispatch | no |
| `-1091,12 +1128,21` | §14.1 | the universal deleted, the paragraph rewritten | **the sentence *"and §1 requires pass 1's engine sections to name a committed config"* goes with it.** Nothing turned on it: §1 still carries the requirement in its own lifted block (`:226-227`), and §14.1's conclusion is now grounded on the stub alone, which is the stronger ground. See Q5 |

**The two replaced mutants, checked in both directions, which the dispatch asked
for by name.**

| row | could the REPLACED one die? | can the REPLACEMENT die? |
|---|---|---|
| `id_lines` | **NO** — `EngineIdentity` derives `PartialEq` (`identity.rs:11-22`); the limb cannot be dropped without replacing the derive | **YES** — one branch of the refusal message's four-field walk (§3.1 `:381-385`); the test's name asserts the message names that line |
| `bestmove` | **NO** — the normalisation is the removal of ` nps <n> time <n>` (§4.1 `:457-458`) and no `bestmove` line contains it; the identity function on every input | **NO under the driver the row carries** — see **F-3**. It is a real mutation for some inputs, which the old row was not, but the stub cannot produce one |

**The one added test, checked for vacuity.**
`an_overlong_non_line_refuses_the_run_by_name` cannot pass vacuously: its unit is
the failure mapping, whose two arms differ by the refusal's NAME, and the test
asserts the name. It also does not disturb
`an_engine_that_closes_its_pipe_refuses_the_run_by_name`, which exercises the
other arm.

**Set closure re-derived, not carried.** **38 registered tests, 29 mutant rows**
(37/28 at revision 5 — the deltas are exactly M-7's one test and one mutant, the
two replacements being one-for-one). Extracted every test name and every mutant
target, `LC_ALL=C sort -u`, `comm`: **every mutant target resolves to a registered
test.** The one non-name token is `demands_newgame_per_ask`, the stub behaviour in
that row's parenthetical — identical to revision 5's result. **No test pins
nothing. No mechanism lost a guard.**

---

## Q3. Is §0.2c complete and true?

**COMPLETE: yes, mechanically. TRUE: no — two of its nine rows over-claim.**

**Completeness.** All thirteen `-U0` hunks map to a §0.2c row or to the block
itself; the header rewrite is a disclosure, which is how every prior reviewer in
this arc has treated the revision banner. **There is no unlisted edit** — the
thing M-9 was about one round ago, and the thing D-547 exists to prevent.

**Truth, row by row.**

| row | claim | verdict |
|---|---|---|
| **M-1** `:170` | *"the diff and gate 15, which is what §9 actually names"* | **TRUE** — §9 `:813-828` names exactly those two |
| **M-2** `:171` | *"the same test, now against a mutation the code can take"* | **TRUE** — verified at `identity.rs` and §3.1 |
| **M-3** `:172` | *"§0.2's §6 row says four rows, not three"* | **TRUE** — 9 rows now, 5 at `7af62e7` |
| **M-4** `:173` | *"the same test, **which already killed every real bestmove mutation**"* | **FALSE of the row's own replacement — F-3.** The test does kill a dropped, trimmed or re-prefixed field; it does not kill the mutation now registered against it |
| **M-5, M-6** `:174` | *"the named unit, which is the one site that can refuse"* | **TRUE** — `totals_of` is the recogniser and has no `score` lookup |
| **M-7** `:175` | `an_overlong_non_line_refuses_the_run_by_name` | **TRUE** — registered at `:873`, mutant at `:922`, both workable |
| **M-8** `:176` | *"**five** quotations name the WP-2.0 dispatch"* | **FALSE — F-1.** Two do |
| **M-9** `:177` | *"this table has a row for every change, **N3's included**"* | **FALSE — F-2.** No N3 row exists in §0.2b or §0.2c |
| **this session's own** `:178` | *"the stub's own `solver_nodes: 0`, which was always the real ground"* | **TRUE as a "what pins it now" claim** — but the deletion's stated GROUND is contested; see **F-4** |

**And the header's summary sentence (`:8`) inherits the same over-claim.**
*"…names three test drivers, and **corrects three counts**."* The drivers are
three (M-5, M-6, and M-7's new row). **The counts are one**: §0.2's §6 row,
three → four. The N4 row's leg list is a list, not a count, and the third would
have been M-9's — which did not land.

---

## Q4. Is the freeze honoured? — **YES, and mechanically, on all three limbs.**

1. **No verbatim-lift block altered.** `/usr/bin/grep "^>"` over
   `git show 41a52f0:…` and over HEAD returns **118 lines each**, and `diff`
   between them is **empty**. **Revision 6 changed no quoted line at all.**
2. **§12 is still word-identical to `5064b05`'s §4.** `:977-1024`'s quoted stream
   against `5064b05:79-97`, whitespace-normalised: **152 words each, `diff`
   empty.** §12 is also byte-identical to revision 5's (`diff` empty over the
   raw block). D-540's fresh-process criterion and its defect-class clause are
   intact.
3. **No unlisted edit to any section a reviewer passed.** D-547 freezes revision
   5 in full and every hunk is rowed (Q3). The two frozen-block EDITS this round
   makes are to §0.2 (`:103`, M-3) and §0.2b (`:152-153`, M-1/M-2) — the
   change-log blocks themselves — and both are listed in §0.2c with their ground.
   §0.2c is inserted as a new sub-block between §0.2b and §0.3, so **no numbered
   section moved**; §0.4's stability rule (`:211-214`) is honoured.

---

## Q5. The author's own added correction

**The deletion is RIGHT. The three configs carry the cited lines. But the
document's reason for deleting is contested by the tree's own vocabulary, and
§14.1 now scopes one rule two ways — F-4.**

**(a) Is the deletion right? YES, and independently of whether the universal was
false.** The clause was propping up *"a normalisation widened to strip
`solver_root_nodes` would have died nowhere."* That conclusion's real ground is
two facts I verified at the files: `render_info` emits the solver block only
`if info.solver_nodes > 0` (`crates/pistol-cli/src/report.rs:62-81` — `:62` is
the `if`, `:81` the closing `};`, so the cited range is exact), and the arena's
stub sets `solver_nodes: 0` (`crates/pistol-arena/src/bin/stub_engine.rs:123`,
inside the cited `:120-131`). **A clause that changes no conclusion is deleted
rather than refined — CLAUDE.md's own test, D-424.** §14.1's argument stands
without it, and stands better: the synthetic-line route is now justified by the
one fact that actually governs an in-crate mutation run.

**(b) Are the three configs what the document says?** They carry the cited
values, exhaustively:
`git grep -n "on_search_path" -- configs/ | LC_ALL=C sort` returns **twenty-three
lines across sixteen files, of which exactly three are `= true`** —
`configs/bench_wp18c_solver_on.toml:45`, `configs/gate_staged_solver_v0.toml:47`,
`configs/play_staged_solver_v0.toml:75`. **The document names all three and no
fourth exists.** Every other config is `= false`, `configs/gate_v0.toml:94`
included.

**(c) Does §14.1's argument still stand without the universal? YES** — see (a).
The three unit tests over synthetic lines and the one integration test are
unchanged, INVARIANT 6 is unchanged, and the closing clause (*"the solver-bearing
totals line is a real class a pilot config could produce, which is a reason for
the synthetic test and not against it"*) is TRUE under either reading of
"committed" and is the strongest form of the point.

**(d) What does NOT hold: the falsity claim itself. See F-4.**

---

## Q6. `design_citation_check.py`, and every citation NEW in revision 6

```
$ python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/capture.rs \
    --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md \
    --proposes configs/arena_wp20_label_pilot.toml docs/experiments/wp20m_design.md
docs/experiments/wp20m_design.md: 123 citation(s) checked, 0 unreproduced
```
Exit **0**. (116 at revision 5.) All four proposed paths still absent from the
tree.

**The NEW citation targets are exactly four**, found by extracting every
backticked `path:line` token from both revisions and `diff`ing the sorted
multisets. **I read every one at the file.**

| new citation | occurrences | read at the file | supports its claim? |
|---|---|---|---|
| `configs/bench_wp18c_solver_on.toml:45` | 2 (`:12`, `:1142`) | `on_search_path = true` | **the VALUE, yes** — the classification, see F-4 |
| `configs/gate_staged_solver_v0.toml:47` | 2 (`:13`, `:1143`) | `on_search_path = true` | **the VALUE, yes** — same |
| `configs/play_staged_solver_v0.toml:75` | 2 (`:13`, `:1144`) | `on_search_path = true` | **the VALUE, yes** — same |
| `docs/experiments/wp20_dispatches.md:15` | 1 (`:176`) | *"standard. **A document quoting "the dispatch" must say which**, because the third"* | **YES** — the rule is on that line, and `f96593b` pre-dates `41a52f0`, so it governs |

**The one citation the round DROPPED** is `configs/gate_v0.toml:94`, which
survives inside §14.1's quotation of the deleted sentence (`:1141`) and is still
resolved by the checker. Nothing else lost a citation.

**Two adjacent claims I re-read because the round touched their paragraph, and
both hold:** `crates/pistol-cli/src/report.rs:62-81` is the whole
`if info.solver_nodes > 0 { … } else { String::new() }`, and
`crates/pistol-arena/src/bin/stub_engine.rs:120-131` contains `solver_nodes: 0`
at `:123` — the design's narrowed range is tighter than revision 5's and still
exact.

**D-483 numeral sweep over the ADDED lines only — PASS.** After stripping
citations, D-keys, section numbers and sha-shaped tokens, the numerals introduced
this round are **0–10 and 15**. The largest is 15 (CI gate 15 / the dispatch rule
line). **No budget value, no node count, no threshold, no line count.**

---

## Q7. Any rule now stated twice and differently as a result of these edits?

**One — F-4(b) — plus one benign duplication and one phrase I checked and cleared.**

- **The graded one.** §14.1 `:1132-1133`: *"**no stub-driven test in this crate**
  can produce the spelling"*. §14.1 `:1145-1146`, eight lines later and new this
  round: *"the whole reason **no test in this crate** can reach the spelling"*.
  Same rule, two scopes, and the second is false of the section's own registered
  test. **F-4(b).**
- **A duplication, consistent, not graded.** The false-universal correction is
  stated in full twice — the header `:10-16` and §14.1 `:1139-1148` — with the
  same three citations in each. D-423's shape would be one statement plus a
  pointer. **But the two agree**, and this document's whole §0.2/§0.2a/§0.2b/§0.2c
  apparatus is a deliberate restatement layer that four reviewers have passed. I
  record it and do not grade it.
- **Cleared.** `:938` — *"returns `None` for **every committed, solver-off
  config**"* — survived the §14.1 deletion. It is a conjunction, not a universal
  over committed configs, so it is true as written and does not conflict with the
  deletion. The mutant it supports (*"a fourth load-bearing lookup added to
  `totals_of`"*) still dies under the stub, whose totals line carries no
  `solver_nodes` field at all.
- **Swept and clean:** the TAB refusal (§4.2 owns it, §6 `:645` and INVARIANT 6
  point at it), the digest's three inputs (§5 and INVARIANT 12 agree),
  `arena_version` (still stated once — nowhere it is written), §14.1 vs §14.3
  (consistent), §3.1's detection-versus-message (**now consistent in prose AND in
  both registry rows** — M-2's doing), INVARIANT 10's evidence (**now consistent
  with §0.2b** — M-1's doing).

---

# PART 2 — FINDINGS

## F-1 (MINOR) — M-8 landed at two of five sites, and it skipped the only one the finding said does not resolve

`:176` claims *"**five** quotations name the WP-2.0 dispatch."*

`/usr/bin/grep -n "dispatch"` over the document. The five sites rev-5 listed are
now `:551`, `:697`, `:965`, `:1047`, `:1152`. **Two carry the qualifier:**
`:551` (*"the WP-2.0 dispatch's requirement 2"*) and `:965` (*"The WP-2.0
dispatch asks for"*). **Three do not:**

- `:697` — *"it agrees with **the dispatch's own** *"games at the standing"*"*
- `:1047` — *"**The standing dispatch** registers *"ledger overwrite → append test
  dies"*"*
- `:1152` — *"**The standing dispatch's** requirement 4 asks for…"*

**And `:697` is the one that matters.** The rev-5 reviewer's FIX read: *"**"the
WP-2.0 dispatch's own" at `:662`**, and the same qualifier on the four
"standing" sites for the rule's literal terms"* — and it explained why the two
halves differ: *"'Standing' is the third dispatch's own word for the first …, so
'the standing dispatch's requirement 4' resolves uniquely. **The bare "the
dispatch's own" at `:662` does not carry that word.**" **The round applied the
optional half at two sites and skipped the necessary one.**

The rule is live and governing: `docs/experiments/wp20_dispatches.md:15`,
*"**A document quoting "the dispatch" must say which**"*, in `f96593b`, which is
`41a52f0`'s parent. **Nothing an implementer builds turns on it** — I re-checked
all five quotations against the file and every one is exact and every one is from
the first dispatch — but §0.2c asserts a completeness the document does not have,
which is the class this round existed to stop asserting.

**FIX:** qualify `:697`, `:1047` and `:1152`, **or** correct `:176` to *"two of
five quotations named; `:697`, `:1047`, `:1152` still read 'the standing
dispatch'."*

## F-2 (MINOR) — M-9's remedy is not in the document, and §0.2c says it is

`:177`: *"| **M-9** | **this table has a row for every change, N3's included** |
— | — |"*.

`/usr/bin/grep -n "N3" docs/experiments/wp20m_design.md` returns **two lines**:
`:103` (§0.2's §6 ground, which mentions *"the count defect N3 named"*) and
`:177` (the claim itself). **Neither table has an N3 row.** §0.2b's table
(`:150-158`) is still N1, N2, N4, N5, N6, N7, N8, *carried*, *m8 residual* —
nine rows, no N3. §0.2c's table (`:170-178`) is M-1 … M-9 plus *this session's
own* — no N3.

The rev-5 FIX was one row, quoted in full: *"| **N3** | §0.2a's frozen-edit count
corrected from two to four | — | — (a disclosure count, no test moves) |"*.
Neither reading of *"this table"* rescues the claim: §0.2b has no N3 row, and
§0.2c's own scope is revision 6's changes, in which N3 has no place. **The row
claims a completeness neither table has.**

**And the header's `:8` inherits it** — *"corrects three counts"* is supported by
one (M-3); the second is the N4 row's leg list; the third would have been this.

**FIX:** add the row to §0.2b, and either drop *"corrects three counts"* or say
*"corrects one count and two coverage claims."*

## F-3 (MINOR) — M-4's replacement is a real mutation, and the stub cannot produce a line that distinguishes it

`:913`: *"| the `bestmove` field written from the parsed turn rather than from
the engine's own line | `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote` |"*.

**The test's driver column at `:864` is EMPTY**, and §10's own lead-in
(`:841-843`) defines that: *"**Where a test is driven by something other than the
arena's stub, the driver is named**, because the vacuity this arc has paid for
four times comes from a test whose driver cannot produce the thing it is
testing."* **So the registered driver is the stub.**

**Under the stub, the mutation writes identical bytes.** Four steps, each read at
the file:

1. **The engine's line is a `Turn` rendered once.**
   `crates/pistol-cli/src/report.rs:106-108` is
   `pub fn bestmove_line(best: Turn) -> String { format!("{BESTMOVE_PREFIX} {best}") }`.
2. **`Turn`'s parse and render are mutually inverse on everything the parser
   accepts.** `Display` is `"q,r"` or `"q,r{PAIR_SEPARATOR}q,r"`
   (`crates/pistol-core/src/turn.rs:177-186`); `FromStr` **refuses** an
   uncanonical pair rather than reordering it (`:219-241`, *"An uncanonical pair
   is refused rather than reordered … one turn has one spelling"*). So
   `parse` → `to_string` is the identity on every accepted token.
3. **The arena's parse absorbs the one thing that could have differed.**
   `crates/pistol-arena/src/exchange.rs:59-68` is
   `rest.trim().parse::<Turn>()` — `.trim()` removes exactly the leading and
   trailing whitespace that would have made the raw line and the re-render
   differ.
4. **No stub behaviour writes such a line.** Eleven `Behave` variants
   (`crates/pistol-arena/src/bin/stub_engine.rs:13-54`). The honest ones answer
   through `pistol_cli::Session` (`:266`) and therefore through `bestmove_line`.
   `Behave::Illegal` writes `format!("{} {}", BESTMOVE_PREFIX, Turn::single(Coord::new(0,0)))`
   (`:325-333`) — one space, canonical. `Behave::BadBestmove` writes
   `bestmove not-a-turn` (`:319-322`), which **fails the parse and forfeits**, so
   it is never captured. `Garbage`, `Hang` and `Exit` write no `bestmove` at all.
   `deviate` (`:354-361`) rewrites `id protocol ` and nothing else.

**Therefore the mutant survives the test it is registered against.**

**This is a real improvement on the row it replaced, and I say so.** The old
mutant (*"the `bestmove` line normalised too"*) was the identity function on
**every possible input**, because the normalisation strips ` nps <n> time <n>`
and no `bestmove` line contains it. The new one is a genuine behavioural
difference — for a third-party speaker writing `bestmove  q,r` with two spaces,
which §4.2 explicitly contemplates (*"a third-party speaker satisfying the
handshake could emit anything"*, `:507-509`). **It is unreachable only through
the driver the row carries.**

**Why MINOR and not the N1 class**, on revision 5's own line: the test still
kills every mutation that changes the field observably under the stub — dropping
it, trimming it, dropping its prefix, writing the token alone — so **INVARIANT
6's `bestmove` limb is pinned**. What is wrong is the registered mutation and
§0.2c `:173`'s claim that the test *"already killed every real bestmove
mutation"*. **And the design decided the mechanism correctly**: §4.2 `:505-506`
says field 5 is *"the engine's own bytes off the channel"*, so an implementer
reading the design plumbs the raw line; only the mutant fails to catch one who
does not.

**FIX:** the same clause M-5 and M-6 got this round — a named driver, e.g.
*"unit, over the record writer, synthetic — a `bestmove` line whose spacing the
turn parser normalises away"*.

## F-4 (MINOR) — the session's own correction: the three configs are ones the tree classifies OUT of the universal, and §14.1 now scopes one rule two ways

**(a) The falsity claim.** `:11-13` (front page) and `:1140-1144` (§14.1) both
assert that *"every committed engine config has the solver off the search path"*
is **false**, on the ground that `configs/bench_wp18c_solver_on.toml:45`,
`configs/gate_staged_solver_v0.toml:47` and `configs/play_staged_solver_v0.toml:75`
arm it. **The values are right. The classification is what those files deny, in
their own headers, by name:**

- `configs/bench_wp18c_solver_on.toml:15-17` — *"**NOT an SPRT arm and never a
  committed engine config**: the committed configs keep `on_search_path = false`
  (D-441)"*.
- `configs/gate_staged_solver_v0.toml:8` — *"**NOT a strength seat: it is never an
  SPRT arm and never the committed config**"*.
- `configs/play_staged_solver_v0.toml:8-12` — *"**THIS IS NOT A DEPLOYMENT
  CONFIG.** D-441's "gate OFF in every committed config until an SPRT says
  otherwise" binds what pistol SHIPS; **the measurement seats
  `configs/gate_staged_solver_v0.toml` and `configs/bench_wp18c_solver_on.toml`
  are committed with the gate on for the same reason this one is**"*.

**And the ADR log settles the vocabulary rather than leaving it to reading.**
`docs/decisions.md:1108` (D-520): *"**D-441's "gate OFF in every committed
config" binds DEPLOYMENT configs; this is a measurement seat**, on the precedent
of `configs/gate_staged_solver_v0.toml` and `configs/bench_wp18c_solver_on.toml`."*
D-465 (`:996`) states the same set as *"THE GATE STAYS `false` IN EVERY COMMITTED
CONFIG"* while all three of these files already existed.

So the sentence is **false under a "tracked in the repository" reading of
"committed" and TRUE under the project's own**, which is the reading D-441,
D-465, D-520 and the three files themselves use — and which the rev-5 sentence's
own second half (*"§1 requires pass 1's engine sections to name a committed
config"*, where §1's `:226-227` means a config file that exists in the tree)
pulled the other way. **The clause was ambiguous, not simply false**, and the
document now states one side of the ambiguity as settled fact on its front page,
citing the three VALUES and none of the three lines that contest the
classification.

**Nothing turns on it and the deletion is still right** — a clause that licenses
the same conclusion either way is deleted, not refined (D-424), which is exactly
what §14.1 does and why *"the argument never needed it"* is the true sentence in
that paragraph. But this is the one correction that came from outside this
document's review, it is recorded prominently as *"THIS SESSION'S OWN"*, and
**a front-page finding that a tracked file contradicts in its own header is the
kind of claim that becomes precedent.**

**(b) One rule, two scopes, eight lines apart, both new or rewritten this round.**
`:1132-1133`: *"**no stub-driven test in this crate** can produce the spelling"*.
`:1145-1146`: *"the whole reason **no test in this crate** can reach the
spelling"*. The second is false of the section's own registered test
`the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line`
(`:861`), which **is** a test in this crate that reaches the solver spelling —
synthetically, which is the entire point of the paragraph three above it. The
narrow form is correct and the section already uses it.

**FIX:** for (a), either name the sense — *"false of the tracked configs; the
three that arm it are measurement seats their own headers and D-520 put outside
D-441's deployment set"* — or delete the falsity claim and keep only *"the
argument never needed it"*, which is sufficient and uncontested. For (b), the
word *"stub-driven"*.

---

# PART 3 — WHAT I CHECKED AND FOUND SOUND

Recorded so the next round does not re-derive it.

- **The freeze, on all three limbs**: 118 quoted lines unmoved, §12 word-identical
  to `5064b05:79-97` at 152 words, every hunk rowed, no numbered section moved.
- **The set closure**: 38 tests, 29 mutants, every mutant target a registered
  test, one stub-behaviour token as at revision 5.
- **M-1, M-2, M-3, M-5, M-6, M-7**: applied, and each verified against the tree
  rather than against §0.2c — `identity.rs:11-22`, `exchange.rs:169-188`,
  `exchange.rs:46-56`, `stub_engine.rs:13-54` and `:354-361`,
  `7af62e7:160-175` vs `:638-646`, §9 `:813-828`.
- **M-2's side effect**: §3.1's closing sentence (`:384-385`) was false at
  revision 5 and is true at revision 6.
- **M-7's side effect**: §9's *"TWO THINGS NO TEST PINS"* (`:809-811`) is now
  accurate without being edited.
- **The citation checker**: 123 / 0 unreproduced, exit 0; four new targets, all
  four read at the file.
- **D-483**: numerals introduced this round are 0–10 and 15. Clean.
- **AUTHOR DEBT: none.** All four findings are inside the half the citation
  instrument explicitly disclaims. **Fourth consecutive round with zero.**

---

# PART 4 — THE STRONGEST ATTACK THAT DID NOT LAND

**I attacked the round on the hypothesis the dispatch named: that a correction
round on a PASSED document is where this arc spends a true thing, and that the
§14.1 rewrite — the biggest hunk, the one correction not asked for by a
reviewer, arriving from a sibling review — was where it would happen.** The
specific attack was that deleting *"every committed engine config has the solver
off the search path"* would take with it the clause beside it, *"and §1 requires
pass 1's engine sections to name a committed config"*, and that some other
paragraph was leaning on that clause.

**It came back sound, and I checked it three ways.**

1. **Is the clause still available where it is needed?** Yes — §1's lifted block
   carries it verbatim (`:226-227`, *"both engine sections naming the same
   committed config"*), and it is the ground §3 is rebuilt on (`:231-238`,
   with `validate.rs:243-250` cited). §14.1 was quoting §1, not owning the rule.
2. **Does any other section lean on the deleted universal?**
   `/usr/bin/grep -n "committed engine config\|committed config"` over the
   document returns five lines: `:11` and `:1140` (the quotation of the deleted
   text), `:178` (the §0.2c row), `:227` (§1's lift), `:938` (the
   `totals_of` mutant's *"every committed, solver-off config"*, a conjunction
   that is true as written). **No section's conclusion rests on it.**
3. **Is the replacement ground strictly stronger?** Yes, and this is the part
   worth recording: the old ground was a claim about the DEPLOYMENT fleet, which
   is irrelevant to whether a mutation dies under `cargo test`; the new ground —
   `render_info` gates on `solver_nodes > 0` and the stub sets it to `0` — is
   the fact that actually governs an in-crate mutation run. **The correction
   improved the argument's ground while I was attacking it for weakening it.**

**Three further attacks failed and are recorded so nobody re-runs them:**

- **That M-7's new test is unwritable.** It is: `exchange.rs:46-56` has both arms
  side by side and a unit over the failure mapping needs no process at all.
- **That the M-2 replacement mutant is as unkillable as the one it replaced.** It
  is not — the refusal message's four-field walk is a walk, and dropping one limb
  is a one-branch deletion that the test's own name catches.
- **That adding §0.2c pushed a section number a reviewer had adjudicated.** It did
  not: §0.2c is a sub-block inside §0, and §0.4's own rule (`:211-214`) is
  honoured — `/usr/bin/grep -n "^## "` gives the same numbered sections at both
  revisions.

---

# PART 5 — WHAT I COULD NOT SETTLE BY READING, AND THE RUN THAT WOULD

- **F-3's exact fate.** That *"the `bestmove` field written from the parsed turn"*
  writes identical bytes under every stub behaviour is read off `Turn`'s
  `Display`/`FromStr` pair, `exchange.rs:59-68`'s `.trim()`, and all eleven
  `Behave` variants; **I did not observe it. The run:** in a separate
  `git worktree add --detach` with its own `CARGO_TARGET_DIR`, write field 5 from
  the parsed `Turn` and run `cargo test -p pistol-arena` — it should stay green.
  Refused here per the dispatch.
- **F-4(a)'s adjudication is the operator's, not a run's.** Whether *"committed
  engine config"* means "tracked in `configs/`" or D-441's deployment set is a
  vocabulary question this document cannot settle for itself; D-520 settles it
  for the ADR log, and the design should either follow it or say it is using the
  other sense.
- **Everything else in this check was settled by reading**, because every claim
  the round makes is about the document's own text or about a line in the tree.

---

## A closing note for the architect

**The answer to the question this check was dispatched to ask is NO: revision 6
broke nothing, spent nothing, and honoured the freeze to the byte.** Six of the
nine corrections are applied and correct, two of them repaired sentences outside
the rows they were aimed at, and the one that mattered most — M-7 — closed the
last rule in the document with neither test nor mutant. **The mechanism, the
invariants and the decisions are exactly what revision 5's reviewer passed.**

**What failed is the ledger, in the same shape as last round.** §0.2b over-stated
two of nine rows; §0.2c over-states two of nine rows, and one more (M-4) records
a mutant as killable that its own registered driver cannot kill. **A block whose
entire purpose is to let the next reviewer trust the round without re-deriving it
cannot be the block that is wrong** — that is the argument the rev-5 reviewer
made for fixing M-1 and M-2 *"before it becomes standing practice"*, and it is
now standing practice.

**The remedy is four edits and none touches a frozen block's substance**: three
words at `:697`, `:1047`, `:1152`; one row in §0.2b; one driver clause at `:864`;
and either a sense or a deletion at `:11-13` and `:1140-1144`, plus the word
*"stub-driven"* at `:1146`. **None of them is a mechanism, and after them this
document is what revision 5's reviewer said it was.**
