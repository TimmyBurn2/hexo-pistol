# SCOPED RE-REVIEW — `docs/experiments/wp20m_design.md` revision 5

## Header

- **Named revision adjudicated:** `41a52f0ce841902259e4b17371248bfbe3cb8cd6`
  (`docs(wp20m): the round that closes the class no instrument guards — every
  remedy now names what it pinned before and what pins it now, and the
  normalisation's call gets its test back`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` → `41a52f0…`, branch `dev`.
- **Tree state:** `git status --porcelain` → one untracked file,
  `?? docs/experiments/wp20s_design_REVIEW_rev2.md`. Nothing this review
  adjudicates is uncommitted; HEAD did not move during the review.
- **Prior revisions compared against:** `a9a4a3a` (revision 4, the reviewed
  revision), `7af62e7` (revision 2) and `5064b05` (revision 1), all read via
  `git show <sha>:docs/experiments/wp20m_design.md`.
- **Read as binding:** `docs/experiments/wp20m_design_REVIEW_rev4.md` in full
  (N1–N8 and both Part-2 sweeps), `wp20m_design_REVIEW.md`,
  `wp20m_design_REVIEW_rev2.md`, `wp20m_design_REVIEW_rev3.md`, CLAUDE.md,
  `docs/process.md`, D-539 … D-548 in full, and
  `docs/experiments/wp20_dispatches.md` at HEAD — where the third dispatch and
  the *"a document quoting 'the dispatch' must say which"* rule (`:15`) now live,
  and **which, unlike at revision 4, PRE-DATE the subject and therefore govern
  it** (`f96593b` is 41a52f0's parent).
- **What I ran:** `git`, `git grep`, `/usr/bin/grep`, `sed`, `awk`, `diff`,
  `comm`, `sort`, `tr`, `wc`, `cat`, and `python3 tools/design_citation_check.py`
  with the four `--proposes` paths. Every recorded grep is `/usr/bin/grep` or
  `git grep` (D-265).
- **What I refused to run, per the dispatch:** `cargo` in any form,
  `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Where a claim
  needs a run I name the run in Part 4.

---

## VERDICT: **PASS**

**0 BLOCKING, 0 MAJOR, 9 MINOR.**

**Both MAJORs are APPLIED and I verified each against the tree rather than
against the document's word.** N1's remedy is real: I traced
`a_captured_record_carries_the_normalised_totals_line` end to end — the stub
emits `nps: 1` / `time_ms: 0` (`crates/pistol-arena/src/bin/stub_engine.rs:125-126`)
through `pistol_cli::Session` (`stub_engine.rs:266`) and therefore through
`render_info`'s one format literal (`crates/pistol-cli/src/report.rs:82-84`), so
the literal substring ` nps 1 time 0` is in the written line, the assertion has
something to bite on, and **deleting the call while leaving the function makes
the test fail**. The mutant table now separates the two sites by name (`:872`,
`:873`). N2 is closed at both ends: `/usr/bin/grep -n "arena_version"` over the
document returns five hits and **not one of them writes it anywhere** — three are
§5's own argument for its absence and two are change-log rows.

**The round's stated purpose is achieved, and I checked it the hard way.** I
diffed `a9a4a3a` → `41a52f0` completely (172 changed lines, sixteen hunks) and asked of
every hunk whether it traded something away. **None does.** Every quoted lift is
byte-identical to revision 4's (`/usr/bin/grep "^>"` over both revisions: 118
lines each, `diff` empty), §12 is word-identical to revision 1's §4, all six
verbatim lifts still reduce to the same word stream as their sources in
`7af62e7`, and the one edit to a frozen section (§6's new TAB row) is listed with
its ground. `design_citation_check.py` is green over **116** citations; I read
**thirty-four** by hand, including every one carrying new prose, and every one
supports the claim built on it. **AUTHOR DEBT: none** — the third consecutive
round with zero.

**Why this passes rather than failing a sixth time.** Not one finding below is a
way the package produces a wrong answer, a requirement it leaves in no package,
or a mechanism decision the design should have taken and did not. The nine are
one clause each and they fall in two families: **two rows of the new §0.2b table
over-claim what pins what** (M-1, M-2), and **the test/mutant registry has four
rows whose site or driver is not workable as written** (M-4 … M-7) — none of
which leaves a mechanism unguarded, which is the line I held throughout and state
explicitly in each finding.

---

# PART 1 — DISPOSITION OF N1 … N8

## N1 (MAJOR) — nothing pinned that pass 2 CALLS the normalisation — **APPLIED**

The dispatch named four sub-questions. All four check out.

**(a) Does the stub emit those constants through `render_info`, so the assertion
has something to bite on? — YES, and I traced the whole path.**
`crates/pistol-arena/src/bin/stub_engine.rs:117-131` builds the `SearchInfo` with
`nps: 1` (`:125`) and `time_ms: 0` (`:126`); `:266` is
`let mut session = pistol_cli::Session::new(engine).identify(vec![weights_line]);`,
so every answer is produced by `pistol-cli`'s own formatter; `report.rs:45` is
`render_info(info, true)` and `report.rs:83-84` is the ONE format literal
`… nodes {}{solver_field} {NPS_FIELD} {} {TIME_FIELD} {} hashfull {} score {} pv`
with `NPS_FIELD = "nps"` (`:16`) and `TIME_FIELD = "time"` (`:18`). **The literal
` nps 1 time 0` is in the stub's totals line.** The design's citation
`stub_engine.rs:120-131` contains both constants exactly (`:125`, `:126`).

**(b) Does the test really die when the call is deleted and the function is left
intact? — YES.** Without the call, field 4 is the raw totals line and carries
` nps 1 time 0`; the design's assertion is that it does **not** (`:1078-1082`).
**And the design closed the vacuity route I went looking for**: it asserts the
record *"does carry every other field the stub emitted"* as well, so a run that
wrote an empty or absent field 4 cannot pass the negative half. That second half
is not decoration — without it the test would pass on a capture that wrote
nothing.

**(c) Does the mutant table separate the two sites? — YES, correctly.**
`:872` is *"the normalisation's FUNCTION broken"* → the unit test; `:873` is
*"**the normalisation not APPLIED on the write path** — the call deleted, the
function left intact"* → the new integration test. The two widening mutants
(`:875`, `:876`) stay on the unit tests, which is right: a widened function is a
function-site mutation.

**(d) Is any OTHER registered test still claimed to kill something it cannot? —
YES, two, and they are M-4 and M-5 below.** I walked all twenty-eight mutant rows
and all thirty-seven test rows. Twenty-five of the twenty-eight rows survive the
check. **M-4** (*"the `bestmove` line normalised too"*) is a mutation that cannot
change any output this package can see. **M-5** (*"an unrecognised totals line
treated as an ordinary `info` line…"*) is registered against a test whose blank
driver column means the stub, and no stub behaviour can produce its named
condition. Neither leaves a mechanism unguarded — see the findings for why.

**One imprecision on the face of §0.2b's N1 row, noted and not graded.** The
BEFORE column reads *"rev 3: one stub-driven integration assertion, killing both
the function and the CALL"*. Revision 3's assertion killed the function's
**non-solver** mutation only — the solver spelling died nowhere, which was
revision 4's own M4 finding. The row flatters revision 3 by one word. The NOW
column is correct and strictly stronger, so nothing turns on it.

**§14.3's rewording is a strengthening, not a spend.** Revision 4 said the re-run
test *"cannot observe the normalisation"*; revision 5 narrows it to *"cannot
observe the normalisation as a DIFFERENCE BETWEEN TWO RUNS"* and then says what
the integration test can do with the same constants (`:1116-1124`). Both
statements are true; the limit is still declared.

**Disposition: APPLIED. This is the strongest fix in the round and I could not
break it.**

## N2 (MAJOR) — §4.3 still wrote `arena_version` into the header — **APPLIED**

**Swept the whole document, not just §4.3.**
`/usr/bin/grep -n "arena_version" docs/experiments/wp20m_design.md` returns five
lines: `:115` and `:136` (change-log rows), `:540`, `:546`, `:548` (§5's argument
for its absence). **No section writes it, names it as a param, or relies on it.**

§4.3's `param` list is now the capture format version, the source report's
`experiment_sha256` and `source_sha256`, and the label `go` line (`:497-500`),
and **every one of the four has a source pass 2 holds**, which I verified against
`Transcript` rather than against the document: `experiment_sha256` is
`crates/pistol-arena/src/transcript.rs:47` (*"The experiment this report is of,
by content"*) and `source_sha256` is `:50`; the format version is this package's
own constant; the label `go` line is its own flag. `Transcript` still carries no
`arena_version` field (`:29-53`), and `git grep -n "arena_version" -- crates/`
still returns two writers (`report.rs:130`, `replay_report.rs:99`, both `env!`)
and no reader. **The decision M1/N2 kept open across three revisions is taken and
the document answers the question once.**

**Disposition: APPLIED.**

## N3 (MINOR) — §0.2a's frozen-edit count — **APPLIED**

`:101-103`: *"**FOUR of revision 4's edits landed in sections frozen by an
earlier review — §1, §2, §6 and §7** — and revision 4's own header said two"*.
Correct, and it says whose count it is correcting.

**But the same block now carries a NEW stale count one row down — that is M-3.**

**Disposition: APPLIED.**

## N4 (MINOR) — INVARIANT 10's evidence names one file where two are touched — **APPLIED, and the argument is sound**

`:778-793` now reads *"the only file whose SPRT-path BEHAVIOUR this package
changes is `crates/pistol-arena/src/exchange.rs`"*, names `bin/arena.rs` and its
five edits explicitly, and argues *"§1 shows the two existing arms and
`outpath::claim` untouched, so a third arm beside them changes no path pass 1
takes."*

**I attacked that argument against the file itself and it holds.**
`crates/pistol-arena/src/bin/arena.rs:82-100` is the arg match: `:83-86` the
`--help` arm, `:87-89` the `["--config", config, "--out", out] | ["--out", out,
"--config", config]` arm, `:90-93` the `--replay` arm, `:94-99` the `_ =>`
fallback. A `--capture … --out … --label-nodes …` arm is a slice pattern of a
different length over different literals — **it cannot shadow the `--config`
arm**, and Rust's slice patterns give it no way to. `outpath::claim` is at `:103`,
after the arg match and **before** `let outcome = match &mode` at `:104`, so the
O_EXCL claim is inherited by structure exactly as the design's bullet at `:244-247` says.
The `Mode` enum at `:78-81` gains a variant; `Mode::Play(config) => run(…)` at
`:105` is untouched.

**The other two edits do not touch the SPRT path either**: `USAGE` is printed
only on `--help` (`:84`) and inside the `_ =>` refusal (`:97`), and the fallback
refusal's mode list (m14's remedy) is on that same unknown-mode path.

One sentence-level looseness I checked and will not grade: a visibility keyword
changes no behaviour, so the set of files whose SPRT-path behaviour changes is
strictly empty and the sentence names one member of it. That understates in the
safe direction and licenses the same conclusion (D-424).

**But §0.2b's own N4 row contradicts this paragraph — that is M-1.**

**Disposition: APPLIED.**

## N5 (MINOR) — §3.1 says the check needs no new predicate and then requires one — **PARTIALLY APPLIED (M-2)**

The prose is fixed and fixed well (`:346-351`): *"**detection is `identities[0] ==
identities[1]` and needs no new predicate; the REFUSAL MESSAGE then walks the
four fields to name the one that differed.**"* That is exactly the FIX the rev-4
review proposed, and it states which of the two revision 4 conflated and why it
mattered.

**The mutant was not touched.** `:882` is byte-identical to revision 4's `:820`:
*"`id_lines` dropped from **the identity comparison**"*. So is the driver column
at `:820`: *"**unit, over the identity comparison**"*. Under §3.1's own new
sentence *"the identity comparison"* is the derived `PartialEq`
(`crates/pistol-arena/src/identity.rs:12`, verified: `#[derive(Debug, Clone,
PartialEq, Eq)]`), from which `id_lines` cannot be dropped without rewriting the
derive — which is the exact objection N5 raised.

The finding is small because the TEST still dies: the test name is
`…_refused_naming_that_line`, so a walk that no longer names `id_lines` fails it
whatever the mutant row calls the site. **What is wrong is §0.2b's claim that the
mutant was fixed** — see M-2.

**Disposition: PARTIALLY APPLIED — graded MINOR as M-2.**

## N6 (MINOR) — `experiment_sha256` had no test that varies it and no mutant — **APPLIED, and both are workable**

Test registered at `:844`, mutant at `:894`.

**Can the mutant die?** The identity is a function of exactly three inputs
(`:531-539`; INVARIANT 12 at `:770-772`): the format version, the source report's
`experiment_sha256`, the label `go` line. Two captures of DIFFERENT experiments
at one format version and one label budget differ in `experiment_sha256` alone.
Drop it and the two identities are equal → the test fails. **It dies.**

**Can the test pass?** It needs two reports with different `experiment_sha256`.
`report.rs:131` writes `experiment_sha256 {experiment_digest(written)}` and
`config.rs:64` records that the digest is over what *"changes which games are
played"*, so two scratch arena configs give two values. **Constructible in-crate,
and it does not disturb `two_reports_of_one_experiment_share_a_capture_identity`,
which needs two runs of ONE config — same `experiment_sha256`, different
`source_sha256`.** The pair covers substitution and omission between them.

**Disposition: APPLIED.**

## N7 (MINOR) — §6 owns the failure modes and carried no TAB row — **APPLIED**

`:610`: *"| **a field about to be written carries a TAB** (§4.2 owns the grammar)
| **refuse the run**, by name, naming the game and turn |"*.

**And D-423 is honoured rather than breached by the addition**: §4.2 owns the
grammar and states the rule (`:475-478`); §6's row and INVARIANT 6 (`:756-759`)
both point at §4.2 rather than restating it. That is the shape D-423 asks for.

**Disposition: APPLIED.** But this is an edit to a section a reviewer PASSED, and
§0.2's own freeze row for §6 was not updated — that is **M-3**.

## N8 (MINOR) — §14.4 counted a one-time pilot receipt as a standing gate — **APPLIED, and the replacement argument is stronger than the FIX asked for**

`:1141-1149` now prices the declined `tools/` route against **gate 9**, and I
verified both limbs at the file: `tools/determinism.sh:153` is the comment
*"`nps` and `time` are the only fields two runs may disagree about."* and `:154`
is `normalize() { sed -E 's/ nps [0-9]+ time [0-9]+//'; }` — **the exact rule and
the exact expression**. The *"same `newgame`/`position`/`go` loop shape"* claim is
also true: `tools/determinism.sh:216-217` is *"One session over every position and
every budget. `newgame` before each so that a position's answer does not depend on
the ones before it."*

**I attacked *"What it would buy is already bought"* as an over-claim and it
survives, because revision 5 supplies the missing half.** Gate 9 buys the PREMISE
(nps and time are the only fields two runs may disagree about); §14.1's new
integration test buys the CONCLUSION's other half (pass 2 removes exactly those
two from what it writes); the composition is INVARIANT 8 against a real engine.
Revision 4 could not make this argument because it had no test for the call.
**The finding and its remedy compose, which is the first time in this arc two
fixes have reinforced each other rather than traded.**

The one strengthening left on the table: the loop-shape claim carries no
citation, and `tools/determinism.sh:216-217` is the line that proves it.

**Disposition: APPLIED.**

---

# PART 2 — THE §0.2b AUDIT AND THE FREEZE AUDIT

## (a) Complete diff `a9a4a3a` → `41a52f0`, hunk by hunk

`git diff a9a4a3a 41a52f0 -- docs/experiments/wp20m_design.md` → **172 changed
lines, sixteen hunks.** Each mapped by hand to a section, then to §0.2, §0.2a or
§0.2b.

| hunk | § | frozen by an earlier PASS? | listed? |
|---|---|---|---|
| `-1,23 +1,37` | header / revision banner | no | disclosure, not an edit to a claim |
| `-84,9 +98,12` | §0.2a lead-in | no (revision-4 material; its review FAILed) | **N3's remedy — not on §0.2b (M-9)** |
| `-104,6 +121,29` | §0.2b lead-in and table, **new** (§0.3 heading is the hunk's tail) | — | the block itself |
| `-302,8 +342,12` | §3.1 | **no** — §0.2 puts §3's rev-2-BLOCKING-A material outside the freeze, and the rev-4 review's own table graded the same section `no` | §0.2b **N5** ✓ |
| `-451,9 +495,9` | §4.3 prose BELOW the lift (`:495-500`; the lift is `:489-493`) | no | §0.2b **N2** ✓ |
| `-493,7 +537,12` | §5 | no (rev-2 MAJOR 5 NOT APPLIED) | §0.2b **N2** ✓ |
| `-558,6 +607,7` | **§6 failure table** | **YES** (rev-2 review MAJOR 1 APPLIED) | §0.2b **N7** ✓ — but §0.2's own row is now stale (**M-3**) |
| `-722,14 +772,21` | §9 preamble + INVARIANT 10 | no (rev-2 MAJOR D) | §0.2b **N4** ✓ |
| `-767,6 +824,8` | §10 test table (+2) | no | §0.2b **N1**, carried ✓ |
| `-782,6 +841,7` | §10 test table (+1) | no | §0.2b **N6** ✓ |
| `-809,7 +869,9` | §10 mutant table (1 rename, +2) | no | §0.2b **N1**, carried ✓ |
| `-823,12 +885,13` | §10 mutant table (m8 clause, + the `experiment_sha256` mutant) | no | §0.2b **m8 residual**, **N6** ✓ |
| `-991,12` `-1006,6` `-1031,11` `-1054,8` | §14.1 heading, §14.1 new paragraph, §14.3, §14.4 | no | §0.2b **N1**, **N8** ✓ |

**One edit lands in a frozen section (§6) and it is listed with a ground.
D-547's substance is met; no silent edit.**

## (b) Are the LIFTED VERBATIM blocks still byte-exact?

**YES, mechanically proved twice.** First: `/usr/bin/grep "^>"` over `a9a4a3a`
and over HEAD returns **118 lines each** and `diff` between them is **empty** —
**revision 5 changed no quoted line at all.** Second, independently against the
sources, by whitespace-normalised word stream:

| block | rev 5 | source | result |
|---|---|---|---|
| §3.2 engine verification | `:394-397` | `7af62e7:110-113` | **IDENTICAL** (48 words) |
| §4.1 THE ONE NORMALISATION | `:422-432` | `7af62e7:121-131` | **IDENTICAL** (122 words) |
| §4.3 THE FILE'S SHAPE | `:489-493` | `7af62e7:133-137` | **IDENTICAL** (49 words) |
| §4.4 THE SOURCE IS NAMED | `:521-525` | `7af62e7:139-143` | **IDENTICAL** (60 words) |
| §7 the budget's kind | `:636-641` | `7af62e7:178-183` | **IDENTICAL** (73 words) |
| §2 which positions are asked | `:263-284` | `7af62e7:73-94` | **IDENTICAL** (233 words) |
| §0.3 D-544 premise | `:167-171` | `7af62e7:39-43` | **IDENTICAL** (53 words) |
| §0.3 capture-decisions lead-in | `:151-155` | `7af62e7:27-31` | differs by the word *"four"* (disclosed at `:92`) and one `:` → `.` — carried from revision 3, correctness-neutral, not graded |

**§12 is word-identical to revision 1's §4**: `:944-960` against `5064b05:80-97`,
**152 words each, `diff` empty** — D-540's fresh-process criterion and its
defect-class clause included.

§8's block (`:698-705`) is a PARTIAL lift and is declared one — its lead-in says
*"with its one unapplied consequence taken"* and the prose below explicitly
overturns revision 2's omitted sentence *"Pass 2 does not call it"*. Disclosed,
not a lift claim, unchanged this round.

## (c) Is §0.2b's "what pins it now" column true in every row? — **NO. Two of nine fail.**

I attacked each of the nine rows.

| row | NOW column | verdict |
|---|---|---|
| **N1** | unit tests + `a_captured_record_carries_the_normalised_totals_line` + its mutant row | **TRUE** — traced end to end in Part 1 |
| **N2** | nothing is owed; §5 covers both; provenance is the pilot's governing revision | **TRUE** — swept, nothing writes it |
| **N4** | *"the diff, **INVARIANT 7's test**, and gate 15 — unchanged"* | **FALSE — M-1.** §9 of the same document withdraws that leg by name (`:789-790`) |
| **N5** | *"…and **the mutant names the walk**"* | **FALSE — M-2.** `:882` is byte-identical to revision 4's and names *"the identity comparison"* |
| **N6** | `two_captures_of_different_experiments_do_not_share_an_identity` | **TRUE** — dies on omission, constructible |
| **N7** | §6's table pointing at §4.2 | **TRUE** — `:610` verified |
| **N8** | — (a pricing sentence) | **TRUE** |
| carried | `a_totals_line_with_no_score_at_all_is_captured_as_written` | **NOT VERIFIABLE AS WRITTEN — M-6.** The test is registered; whether it pins the rule depends on a unit the document does not name |
| m8 residual | the §10 clause that makes the swap mutant able to die | **TRUE** — `:888` carries *"whose fixture must give the game index and the prefix length DIFFERENT values"* |

## (d) Is §0.2b COMPLETE? Did revision 5 change anything that should have a row and does not?

**On the substantive question the dispatch put — did a remedy spend a true thing
and escape the table — the answer is NO.** I walked all sixteen hunks looking
for a trade and found none:

- §14.1 **adds** a fourth registered test; nothing is removed.
- The mutant rename *"the normalisation removed"* → *"the normalisation's FUNCTION
  broken"* **narrows** one row and **adds** the row that covers what the old name
  ambiguously claimed. Net coverage strictly greater.
- §14.3's rewording keeps the declared limit and adds what the new test covers.
- §14.4 replaces one pricing argument with a stronger one; §14.2 survives as its
  own subsection with its obligation intact.
- §4.3's removal of `arena_version` removes a param nothing read and nothing
  tested (§0.2b's N2 BEFORE column, verified).
- §9's rewrite keeps both legs revision 4 had (the diff, gate 15).
- §3.1's rewrite keeps *"needs no new predicate"* and adds the separation.
- The header rewrite drops revision 4's self-assessment sentence; §14.4 carries
  the argument, so nothing is lost.

**One completeness gap, graded MINOR as M-9:** N3's remedy — the §0.2a lead-in
correction — is a change revision 5 made against a rev-4 finding and it has **no
§0.2b row**, while **N8, which also moves no test, DOES have one** with dashes in
both columns. The table is internally inconsistent about its own scope: its title
says *"What REVISION 5 changed"* and its lead-in says *"every test this round
moves, retires or re-drives"*. One row of dashes closes it.

---

# PART 3 — FINDINGS

## M-1 (MINOR) — §0.2b's N4 row names a leg the document itself withdraws

`:137` (NOW column): *"rev 5: the diff, **INVARIANT 7's test**, and gate 15 —
**unchanged**; only the sentence is true now"*.

`:789-790`: *"**Revision 3 also leaned on INVARIANT 7's test here and that leg is
withdrawn**: read as a cross-build comparison it was the very thing this
paragraph calls impossible"*.

The row is wrong twice: it names a leg §9 withdraws, and it calls that leg
*"unchanged"* when it was withdrawn at revision 3 → 4. §9 owns INVARIANT 10 and a
reader who follows the pointer gets the right answer, so nothing an implementer
does turns on it — but this is a **coverage claim inside the block D-548 granted
the round to make trustworthy**, and it over-states coverage.
**FIX:** *"rev 5: the diff and gate 15 — unchanged; only the sentence is true
now."*

## M-2 (MINOR) — §0.2b's N5 row says the mutant was fixed and it was not

`:138`: *"rev 5: `==` detects, a field walk reports, **and the mutant names the
walk**"*. `:882` is byte-identical to `a9a4a3a:820` and reads *"`id_lines`
dropped from **the identity comparison**"*; `:820`'s driver column likewise says
*"unit, over the identity comparison"*. Under §3.1's own new sentence the
comparison is the derived `PartialEq` (`identity.rs:12`, verified), which has no
separable limbs — the objection N5 made. **N5 is PARTIALLY applied: the prose is
fixed, the two registry rows are not, and §0.2b claims otherwise.**
The test still dies (its name asserts the refusal names the line), so nothing is
unguarded.
**FIX:** two words in the mutant row and the driver column — *"dropped from the
refusal message's field walk"* — or delete the claim from §0.2b.

## M-3 (MINOR) — §0.2's freeze table now understates §6's edits, which is N3's shape one round on

`:88` still reads *"**EDITED**: three rows added, **and row 1 is reworded** … The
third row is the malformed totals line (**m9**)"*. Revision 2's §6 table had five
rows (`7af62e7:166-170`, verified); revision 5's has nine (`:603-611`). **Four are
added.** §6's own lead-in at `:598` likewise says *"with two rows added"* — that
one is scoped to the two `Received` outcomes and is defensible; `:88` is not,
because it enumerates.

The TAB row IS disclosed, in §0.2b's N7 row, so D-547's substance holds. What is
wrong is the count on the face of the freeze block — **the exact defect N3 named
one round ago, in the same block, one row down.** The harm D-547 exists to prevent
is a reviewer of revision 6 reading §0.2 and re-deriving three §6 edits when four
were made.
**FIX:** *"four rows added … the third is the malformed totals line (m9) and the
fourth is the write-side TAB refusal (rev-4 review N7)."*

## M-4 (MINOR) — a registered mutant that cannot die: "the `bestmove` line normalised too"

`:877`: *"| the `bestmove` line normalised too |
`a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote` |"*.

**The normalisation is the removal of ` nps <n> time <n>` (§4.1, `:423-424`), and
no `bestmove` line can contain that substring.** `bestmove_line` is
`format!("{BESTMOVE_PREFIX} {best}")` (`crates/pistol-cli/src/report.rs:106-108`,
verified) over a turn token (`"q,r"` or `"q,r/q,r"`, D-5/D-49). Applying the
normalisation to it is the identity function on every input this package can see,
so **the mutation changes no output and the test passes**. Four reviews have
carried this row.

**Why it is MINOR and not the N1 class.** Nothing is left unguarded: the test
asserts byte-identity against the engine's own line and therefore kills every
mutation that actually touches the bestmove — re-rendering it from a parsed
`Turn`, trimming it, reordering the pair. **The invariant is pinned; only the
registered mutation is a no-op.**
**FIX:** replace the row with a mutation the code can take — *"the `bestmove`
field written from the parsed turn rather than the engine's line"*.

## M-5 (MINOR) — `an_unrecognised_totals_line_…` has a blank driver column and no stub behaviour can produce its condition

`:838` registers the test with an **empty** driver column, which §10's own lead-in
(`:806-809`) defines as the arena's stub: *"**Where a test is driven by something
other than the arena's stub, the driver is named**, because the vacuity this arc
has paid for four times comes from a test whose driver cannot produce the thing
it is testing."*

**No stub behaviour can emit a malformed `info totals` line.** I read every one:
`Behave` has eleven variants (`stub_engine.rs:13-54`); the only line rewrites are
in `deviate`, which touches `id protocol` alone (`:353-361`); every other answer
comes from `pistol_cli::Session` and therefore from `render_info`, which is always
well-formed. `Garbage` (`:314-318`) writes a non-protocol line and **never a
`bestmove`**, so it produces a watchdog, not this refusal.

**Why it is MINOR.** The mutant's observable second clause — *"and the run
completed"* — IS reachable: `BadBestmove` (`:319-324`) answers `bestmove
not-a-turn` with no `info` lines at all, so the search closes with no totals line
captured and §6's rule must fire. **Every route the implementer can take yields a
real test that kills the registered mutant**, which is what distinguishes this
from rev-3's M3, where the mutant died nowhere. The gap is that the design commits
itself to naming the driver and here does not.
**FIX:** one clause in the driver column — *"unit, over the totals-line
recogniser, synthetic"* — which is the same unit M-6 needs.

## M-6 (MINOR) — the new no-score test's UNIT is unnamed, and its mutant is a refusal the normalisation cannot express

`:828` registers `a_totals_line_with_no_score_at_all_is_captured_as_written` as
*"**unit, synthetic**"*; `:874` registers the mutant *"a score-less totals line
refused instead of captured"*.

**The two sibling rows spelled "unit, synthetic" (`:825`, `:826`) are over the
normalisation, which is a `fn(&str) -> String` (§14.1, `:1060-1062`) and cannot
refuse anything.** A test over that function cannot kill a refusal added
elsewhere. The design names the unit for the two tests where it matters
elsewhere — *"unit, over the record writer"* (`:830`), *"unit, over the identity
comparison"* (`:820`) — so the standard is the document's own.

**The site the mutant would live at is real and identifiable**: `totals_of`
(`crates/pistol-arena/src/exchange.rs:169-188`, verified) has three `?` lookups —
`nodes`, `time`, `depth_turns` — and **no `score` lookup**, so a score-less totals
line is recognised today and *"capture it as written"* is the natural behaviour;
the mutation is an ADDED check on the recognise-and-capture path.

**Why it is MINOR, not the N1 class.** The test's own name — *"is captured as
written"* — and the mutant's wording — *"refused instead of captured"* — between
them tell an implementer that the test must exercise the path that could refuse.
The natural implementation kills the mutant. **This is a naming gap, not a fork
with a wrong branch**; and no `pistol` engine can emit a score-less totals line
(`render_info` always writes `score {}`, `report.rs:83-84`), so the rule guards
only a third-party speaker.
**FIX:** name the unit — the same clause as M-5.

## M-7 (MINOR) — §6's `Received::Overlong` row has neither test nor mutant, and §9 declares only two unpinned things

`:608` is the only occurrence of *"overlong"* in the document
(`/usr/bin/grep -ni "overlong"` → one line). No registered test and no registered
mutant names it. INVARIANT 9's five tests (`:834-838`) cover eight of §6's nine
rows; this is the ninth. §9 promises *"TWO THINGS NO TEST PINS, BOTH DECLARED
HERE"* (`:774-776`) and this is an undeclared third — which is the criticism N1
made of revision 4.

**Why it is MINOR.** It fails safe, and I checked the mechanism rather than
assuming it. `crates/pistol-arena/src/channel.rs:96-106` sends
`FromEngine::Overlong` and then **`return`s** — the reader thread stops — so an
implementer who forgets the Overlong arm gets no further lines and the run is
refused under the `Closed` or watchdog name instead. **The refusal still fires;
what is lost is hard rule 3's one-name-per-reason, not the refusal.** And unlike
M-5, the driver problem is the same one: no stub behaviour writes ≥`MAX_LINE_BYTES`
without a newline, so this too wants a unit over the failure mapping.
**FIX:** one test row, *"`an_overlong_non_line_refuses_the_run_by_name` | 9 | unit,
over the failure mapping"*, or declare it unpinned in §9 with its reason.

## M-8 (MINOR) — the design quotes "the dispatch" five times and never says which, under a rule that now governs it

`docs/experiments/wp20_dispatches.md:15`: *"**A document quoting "the dispatch"
must say which**"*. That rule arrived in `f96593b`, which the rev-4 reviewer
correctly refused to grade against because it post-dated `a9a4a3a`. **`f96593b` is
41a52f0's parent, so it governs revision 5.**

Sites: `:516`, `:662`, `:928`, `:1010`, `:1106`. Four say *"the standing
dispatch"* and one (`:662`) says *"the dispatch's own"*.

**I checked every quote against the file and every one is exact and every one is
from the FIRST dispatch**: *"a documented, versioned schema with a loader test"*
(`wp20_dispatches.md:88`), *"throughput expectation stated as a shape, measured in
the pilot, never guessed (D-500's class)"* (`:106`), *"ledger overwrite -> append
test dies"* (`:117`, and the design names its section — *"Development round item
2"*), *"a re-run receipt proves byte-identical output on a small range"* (`:92`,
the design's bracketed *"[that]"* correctly marked), *"games at the standing"*
(`:83`, truncated before the number — correct D-483 hygiene).

**The finding is thin and I say so.** *"Standing"* is the third dispatch's own
word for the first (*"Obligations from the standing WP-2.0 dispatcher unchanged"*,
`:317-318`), and only the first has numbered requirements, so *"the standing
dispatch's requirement 4"* resolves uniquely. The bare *"the dispatch's own"* at
`:662` does not carry that word.
**FIX:** *"the WP-2.0 dispatch's own"* at `:662`, and the same qualifier on the
four *"standing"* sites for the rule's literal terms.

**A strengthening the design leaves on the table, noted rather than graded:**
§14.2 argues the real-binary receipt belongs to the pilot. **The THIRD dispatch
registers it there by name** — *"the determinism re-run receipt on a sub-range"*,
`wp20_dispatches.md:331`, inside §4 *"Pilot (registered before it runs)"*. Citing
that would turn §14.2's argument into a transcription of a governing text.

## M-9 (MINOR) — §0.2b has a row for N8, which moves no test, and none for N3, which also moves no test

`:131-143`. Eight of the nine rev-4 findings are rowed; N3 is not, and its remedy
(`:101-103`) is a real edit revision 5 made against a real finding. Either the
table's scope is *"every test this round moves"* — in which case N8's row does not
belong either — or it is *"what revision 5 changed"*, which its own title says, in
which case N3's is missing. **This is the completeness question the dispatch put,
and it is the only gap I found in it.**
**FIX:** one row, *"| **N3** | §0.2a's frozen-edit count corrected from two to
four | — | — (a disclosure count, no test moves) |"*.

## Two citation imprecisions, noted and NOT graded (D-424: neither changes a conclusion)

- `:459-461`: *"`crates/pistol-arena/src/transcript.rs:124-131` **refuses a path
  containing whitespace** *"because the format is whitespace-delimited and does
  not quote"*"*. The quoted comment is exact; the code at those lines refuses an
  **empty** path (`if word.is_empty()`), the whitespace case having already been
  split away upstream. The point being made — this crate has been bitten by
  whitespace-delimited formats, so a TAB and not a space — is true and the cited
  lines are where it is recorded. Carried from revision 3.
- `:206-209`: *"`crates/pistol-arena/src/lib.rs:47-69` **is a plain `pub mod`
  list**"*. Twenty-two of the twenty-three lines are; `:69` is `mod validate;`,
  private. The conclusion — a new `pub mod usage;` is reachable exactly as the
  others are — is unaffected. Carried.

## Sweeps run and clean

- **A rule stated twice and differently (D-544's second shape).** Six candidates
  checked: `arena_version` (now stated once — nowhere); the TAB refusal (§4.2 owns
  it, §6 and INVARIANT 6 point at it — D-423's shape, correct); the digest inputs
  (§5's table at `:531-539` and INVARIANT 12 at `:770-772` agree, three inputs, identical); §14.1 and §14.3
  (consistent); §3.1's detection-versus-message (fixed in prose, stale in two
  registry rows — **M-2**); INVARIANT 10's evidence (**M-1**). **Two hits, both
  graded.**
- **Closure of the sets.** **12 invariants, 37 registered tests, 28 mutants**
  (34/25 at revision 4 — the deltas are exactly the three new tests and the three
  new mutant rows). I extracted every test name and every mutant target and
  `comm`'d them under `LC_ALL=C sort`. **Every mutant target resolves to a
  registered test**; the one non-name token is `demands_newgame_per_ask`, which is
  the stub behaviour in that row's parenthetical, not a test.
  **Invariants with no test: 1 and 3's slot-zero limb, both declared** (`:774-800`).
  **No test pins nothing.** **No two invariants are in tension.**
  **Mutants that cannot die: one — M-4.** **Tests whose declared driver cannot
  produce their condition: one — M-5.** **Tests whose unit is unnamed and whose
  mutant is therefore undecided: one — M-6.** **Rules with neither test nor
  mutant: one — §6's Overlong row (M-7);** revision 4's one (the no-score row) is
  closed, subject to M-6.
- **Vacuous passes.** I looked hardest at the new integration test and could not
  make it pass vacuously: the design pairs the negative assertion (no ` nps 1 time
  0`) with a positive one (every other field the stub emitted), which closes the
  empty-field route. `two_captures_of_different_experiments_do_not_share_an_identity`
  is a strict inequality over a value the two fixtures must differ in, so it
  cannot pass by construction either.
- **D-483 numeral sweep — PASS.** After stripping citations, D-keys, section
  numbers and sha-shaped tokens, the numerals in this document are: 0-15 and 17.
  The largest is **17** (CI gate 17). What remains is invariant numbers, section
  numbers, pass numbers, field indices, gate numbers, revision numbers and rule
  numbers. **No budget value, no node count, no threshold, no range, no line
  count.** The one spelled-out count — *"all thirty-four registered tests"* — is a
  fact about revision 4's own test table and matches the rev-4 review's count.
- **The citation checker.**
  ```
  $ python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/capture.rs \
      --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md \
      --proposes configs/arena_wp20_label_pilot.toml docs/experiments/wp20m_design.md
  docs/experiments/wp20m_design.md: 116 citation(s) checked, 0 unreproduced
  ```
  Exit 0. All four proposed paths confirmed absent from the tree. **The distinct
  citation TARGETS are unchanged from revision 4 — 98 in each, `comm` empty in
  both directions — so revision 5's three new citation occurrences all reuse
  targets a prior reviewer already resolved, and every NEW claim is built on a
  line I re-read below.**
- **The half the checker cannot do.** I read the cited lines for **thirty-four**
  citations, **including every one carrying new revision-5 prose**:
  `stub_engine.rs:120-131` (nps 1 at `:125`, time_ms 0 at `:126`),
  `stub_engine.rs:262-266` / `:267-289` / `:43-53`, `tools/determinism.sh:153-154`
  (and `:216-217` for the loop shape the design asserts uncited),
  `report.rs (cli):15-18` / `:62-84` / `:82-97` / `:106-108`,
  `identity.rs:11-22` / `:12` / `:74-82` / `:84`, `validate.rs:243-250`,
  `transcript.rs:29-53` / `:47` / `:50` / `:124-131` / `:164-170` / `:189-194`,
  `exchange.rs:76-79` / `:154-161` / `:169-188`,
  `bin/arena.rs:82-100` / `:94-99` / `:103` / `:124-143` / `:173-174`,
  `config.rs:120-134`, `channel.rs:96-106`, `corpus/emit.rs:36-44` / `:102-118`,
  `lib.rs (arena):41-45` / `:47-69`, `lib.rs (cli):36`, `corpus/mod.rs:6`,
  `replay.rs:216-241`, `seats.rs:47`, `tt/mod.rs:105-112` and `:114-118`,
  `position.rs:68-73`, `pistol.rs:142-166` (`:163` is
  `format!("config {}", path.display())`), `configs/gate_v0.toml:94`,
  `configs/arena_smoke_v0.toml:54-58`, `docs/book_v2_ledger.md:16`,
  `tools/arena_smoke.sh:22-27` and `:29-33`, and five `wp20_dispatches.md` quotes.
  **Every one supports the claim built on it. No off-by-one.** The two
  imprecisions are recorded above and neither changes a conclusion.
- **AUTHOR DEBT: none.** All nine findings are in the half the instrument
  explicitly disclaims. **Third consecutive round with zero.**

---

# PART 4 — THE VERDICT

## **PASS — 0 BLOCKING, 0 MAJOR, 9 MINOR.**

## Could an implementer build from this without deciding something the design should have decided?

## **YES — for every question about what the package DOES. The nine findings leave three test-AUTHORING choices, and no branch of any of them is wrong.**

The mechanism is decided end to end and I checked each decision against the tree:
which positions are asked and why only the last prefix can be decided; which
engine answers and from which slot, with the config-path precondition; the record
grammar (five TAB-separated fields, in order); the one normalisation and what it
does not touch; the file's shape, its header's four `param`s — all four now
sourceable from `Transcript` plus this package's own constants — and its
`derived`s; the loader's four refusals; the digest's exactly three inputs; the
nine failure modes; the budget kind and the constraint it puts on pass 1's config;
who lands the pilot config; who prints the manifest row; the ADR act owed at
landing.

The three authoring choices are **which unit drives
`a_totals_line_with_no_score_at_all_is_captured_as_written` (M-6)**, **which
drives `an_unrecognised_totals_line_refuses_the_run_and_names_the_game_and_turn`
(M-5)**, and **whether §6's Overlong row gets a test or a declaration (M-7)**. All
three want the same one clause — a unit over the totals-line recogniser and the
failure mapping — and in every branch the registered mutant dies. That is why they
are MINOR and why this is a PASS: **no branch of any of them ships an unguarded
mechanism**, which is the line that separated M3 and N1 (both MAJOR, both leaving
a real site unpinned) from these.

Revision 4's list was two. **Both are retired against the tree and not by
assertion.**

## The strongest attack that did not land

**I attacked §0.2b on the hypothesis that it is the arc's failure mode wearing a
table — that a round granted to close "a remedy that spends a true thing" would
add a block asserting nothing was spent, and that the assertion would be the
spend.** The dispatch pre-declared that as the thing to hunt. **It came back
sound.** I diffed all sixteen hunks against the table and then, independently of
the table, asked of every hunk whether anything true was removed:

1. **Was the integration test bought by giving something up?** No. §14.1 keeps all
   three unit tests verbatim and adds a fourth registered test. The solver-spelling
   coverage revision 4 bought — which died nowhere before it (`report.rs:62`'s
   `if info.solver_nodes > 0`, `stub_engine.rs:123`'s `solver_nodes: 0`,
   `gate_v0.toml:94`'s `on_search_path = false`, all re-verified) — is untouched.
   **Revision 5 has strictly more coverage of INVARIANT 6 than any prior
   revision.**
2. **Did the mutant rename narrow a row without a replacement?** No — the rename
   and the new row together cover both sites the old ambiguous name claimed, which
   is the fix N1 asked for in either of its two allowed shapes, and the design took
   the second.
3. **Did §14.4's new argument shed §14.2's obligation?** No. §14.2 survives whole,
   and the gate-9 comparison is checkable at the file (`determinism.sh:153-154`,
   `:216-217`) where the §14.2 comparison was a judgement.
4. **Did removing `arena_version` from the header orphan a provenance
   obligation?** No — I looked for something now owed by no package and found
   nothing: the provenance it offered is `docs/process.md`'s *"named in the
   pre-registration WITH ITS REVISION"*, which §5 puts in the pilot's
   pre-registration as pass 2's own commit SHA, and `Transcript` never carried the
   field for pass 2 to write honestly in the first place.
5. **Did the freeze hold?** Yes, and mechanically: **not one quoted line changed**
   (118 lines each at `a9a4a3a` and HEAD, `diff` empty), and the single edit to a
   passed section is listed with its ground.

**Three more attacks failed and are recorded so nobody re-runs them:**

- **That the new integration test is unwritable in-crate.** It needs a `nodes`-budget
  stub report, which the arena's test helper can build in a scratch dir the way
  `artifact_gate_tests.rs:35-39` already builds report fixtures, and `BudgetSection::Nodes`
  spells the config (`config.rs:127-131`). The stub ignores the budget
  (`go_reporting`'s `_budget`), so its constants arrive at any budget.
- **That the new integration test can pass vacuously.** It cannot: the positive
  half ("carries every other field the stub emitted") forecloses the empty-field
  route, and the design wrote both halves.
- **That N4's "a third arm changes no path pass 1 takes" is a story.** It is a
  fact about slice patterns and about `outpath::claim` sitting at `arena.rs:103` (the design's bullet at `:244-247`),
  before `match &mode` at `:104`. I read the whole dispatch function.

## What I could not settle by reading, and the run that would

- **M-4's exact fate.** That *"the `bestmove` line normalised too"* changes no
  output is read off `report.rs:106-108`'s format literal and §4.1's definition of
  the normalisation; I did not observe it. **The run:** in a worktree, apply the
  normalisation to field 5 as well as field 4 and run `cargo test -p pistol-arena`
  — it should stay green. Refused here per the dispatch.
- **M-5 and M-6's fate.** That no stub behaviour produces a malformed or
  score-less totals line is read off all eleven `Behave` variants and `deviate`;
  the confirming run is `cargo test -p pistol-arena` with a test that asks the
  stub for one. Refused.
- **The headroom after the edit** (carried from all four prior reviews, still
  open, and the design is correctly silent about the number).
  **The run:** write the arm, extract `USAGE` to `usage.rs`, run
  `tools/file_justification_check.sh` (CI gate 17); `bin/arena.rs` carries no
  entry in `docs/rule9_justifications.md` today.
- **The pilot's throughput and the label budget's value.** Correctly not in this
  document (D-483); the pilot's pre-registration measures and registers them.

---

## A closing note for the architect, since this verdict releases the package

**The class D-548 named is closed, and closed by evidence rather than by
assertion.** No remedy in this round spends a true thing — I looked hunk by hunk,
independently of the table that says so, and found nothing traded. **§0.2b earns
its place**: it is what let me check the round in an hour instead of re-deriving
it, and its BEFORE column is the first artefact in this arc that makes a spend
detectable rather than discoverable. **Two of its nine rows over-claim and that is
worth fixing before it becomes standing practice** (M-1, M-2), because a ledger
that over-states coverage is a ledger a future reviewer will stop trusting.

**The nine findings are one clause each and none of them is a mechanism.** Four
are registry rows — a mutant that is a no-op, two tests wanting a named unit, one
§6 row wanting a test or a declaration — and they cluster on exactly the family of
conditions no engine in this tree can produce, which the design already knows how
to handle: it names the unit for the TAB test and the identity test, and it should
do the same three more times. **Three are change-log rows.** One is a
dispatch-naming word. **None of the nine can make a capture wrong, leave a
requirement in no package, or force an implementer to invent a mechanism.**

**And the good part, which is most of the document.** The headline normalisation
now has both limbs and I traced the integration one to the stub's own bytes.
`arena_version` is answered once. INVARIANT 10's evidence is true as written and
its argument survives reading `bin/arena.rs`. The digest has three inputs, three
tests and three mutants, one for each. The freeze held with not a single quoted
line moved. 116 citations resolve, thirty-four hand-checked, zero author debt for
the third round running. **Five reviews said this package's problem was the
authoring method; the sixth says the method is working.**
