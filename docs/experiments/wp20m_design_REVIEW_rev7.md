# REGRESSION RE-CHECK — `docs/experiments/wp20m_design.md` revision 7

## Header

- **Named revision adjudicated:** `4752192bc63a1cd29dc488c4020239eec96e6cb7`
  (`docs(wp20m): the corrections completed — three dispatch references named, the
  bestmove mutant given a driver that can kill it, and the solver-config claim
  restated as ambiguous rather than false`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` → `4752192bc63a…`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty**. Nothing adjudicated here
  is uncommitted; HEAD did not move during the check.
- **Prior revisions compared against:** `ff1c575` (revision 6, the revision the
  four findings were raised on), `41a52f0` (revision 5, the PASSED revision),
  `a9a4a3a` (revision 4) and `5064b05` (revision 1, for the §12 freeze).
- **Read as binding:** `docs/experiments/wp20m_design_REVIEW_rev6.md` in full
  (F-1 … F-4 and all five Parts), `docs/experiments/wp20_dispatches.md` at HEAD
  (all three transcriptions), CLAUDE.md, D-424, D-441, D-465, D-520, D-543,
  D-547, and the four tree files the findings turn on
  (`crates/pistol-arena/src/bin/stub_engine.rs`, `crates/pistol-cli/src/report.rs`,
  and the three `configs/*solver*` files).
- **What I ran:** `git`, `git grep`, `/usr/bin/grep`, `sed`, `diff`, `sort`,
  `uniq`, `tr`, `wc`, `cat`, and `python3 tools/design_citation_check.py` with the
  four `--proposes` paths. Every recorded grep is `/usr/bin/grep` or `git grep`
  (D-265).
- **What I refused to run, per the dispatch:** `cargo` in any form,
  `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`.
- **Scope:** the seven questions the dispatch put and nothing wider. I re-opened
  nothing revision 5's reviewer passed.

---

## VERDICT: **PASS** — **0 BLOCKING, 0 MAJOR, 0 MINOR.**

**F-1, F-2, F-3 AND F-4 ARE APPLIED, EACH IS CORRECT AGAINST THE TREE, AND
REVISION 7 SPENT NOTHING.** Ten hunks, every one of them on the four findings or
on the revision banner. The freeze is intact to the byte on all three limbs. The
citation checker is green at 123/0 and the three citations the round introduces —
the config self-descriptions, the ones F-4 turns on — are each exact at the line
cited.

**The one substantive correction of the four is F-3**, and it is right: the
`bestmove` mutant is now registered against a driver that can produce the byte
difference, which is what revision 6 could not do. F-1 and F-2 are ledger repairs
and both are complete. **F-4 is the round's real content** — the reclassification
of a front-page "It is false" into an AMBIGUITY — and it is accurate about the
three files, about D-441/D-520's vocabulary, and about why D-424 still says
delete rather than refine.

I record **four observations** below. **None is a finding**, and I say so on each
one with the reason. The dispatch asked me not to manufacture a finding to
justify the round, and the honest answer is that this round did what it was sent
to do.

---

## Q1. F-1 — are all five dispatch quotations qualified, and each to the RIGHT dispatch? — **YES, all five, and every quotation is exact and from the first dispatch.**

`/usr/bin/grep -n "dispatch" docs/experiments/wp20m_design.md` gives five
quotation sites, and **all five now name the WP-2.0 dispatch**:

| site | text | quoted phrase found at | inside |
|---|---|---|---|
| `:574` | *"the WP-2.0 dispatch's requirement 2 asks for"* | `wp20_dispatches.md:88` | first dispatch, Scope item **2** |
| `:720` | *"it agrees with the WP-2.0 dispatch's own "games at the standing" node budget"* | `wp20_dispatches.md:83` | first dispatch, Scope item **1** |
| `:988` | *"The WP-2.0 dispatch asks for "throughput expectation stated as a shape…""* | `wp20_dispatches.md:106` | first dispatch, **Design decides and records** |
| `:1070` | *"The WP-2.0 dispatch registers "ledger overwrite → append test dies""* | `wp20_dispatches.md:117` | first dispatch, **Development round item 2** |
| `:1182` | *"The WP-2.0 dispatch's requirement 4 asks for…"* | `wp20_dispatches.md:92` | first dispatch, Scope item **4** |

**Three of these are the sites revision 6 skipped** (`:697`, `:1047`, `:1152` at
revision 6's numbering → `:720`, `:1070`, `:1182` here), and the one the rev-6
finding said mattered most — the bare *"the dispatch's own"*, which carried no
word that resolved it — is among them.

**Each names the right one, checked by exclusion, not by assumption.**
`/usr/bin/grep -c -F` over `wp20_dispatches.md` returns **exactly 1** for each of
the five quoted strings, and each single hit falls inside lines 35–163, which is
the `## WP-2.0 — the label + census pipeline` section. The WP-2.0b dispatch
(165–243) and the M-design-by-quotation dispatch (245–398) contain none of them.
`:1070`'s parenthetical *"Development round item 2"* is right: line 117 is inside
that item's mutant list. `:1182`'s bracketed *"[that]"* is a marked editorial
insertion into `wp20_dispatches.md:92`, and the rest is verbatim.

**"the WP-2.0 dispatch" resolves uniquely** because the file's own three headings
are `WP-2.0`, `WP-2.0b` and `WP-2.0 finish`; the third dispatch itself calls the
first *"the standing WP-2.0 dispatcher"* (`wp20_dispatches.md:318`), which is why
the old "standing" phrasing resolved at all and why replacing it loses nothing.

**No residual bare reference remains.** `/usr/bin/grep -n "dispatcher\|standing"`
over the document returns three lines, none of them a dispatch quotation (`:157`
prose, `:605` a quotation of this document's own revision 3, `:721` the wrapped
tail of `:720`'s already-qualified quote). The two bold lead-ins that say *"THE
DISPATCH'S"* (`:987`, `:1069`) are not quotations, and the sentence carrying the
quotation immediately beneath each one names the dispatch — the rule at
`wp20_dispatches.md:15` binds *"a document quoting "the dispatch""*, and every
quotation now says which.

---

## Q2. F-2 — does §0.2b have an N3 row, is it true, and is §0.2c's summary now accurate? — **YES, TRUE, and yes.**

**The row exists**, at `:169`, in §0.2b (the table F-2 named), not in §0.2c:

> `| **N3** | §0.2a's frozen-edit count corrected from two to four | — | — (a count on the freeze block's own face; revision 5 applied the fix and left this row out) |`

**Every clause of it is true, verified at both ends:**

- **"corrected from two to four"** — `git show a9a4a3a:…` line 87 reads *"**Two of
  this round's edits land in sections frozen by an earlier review**"*; the
  document at `:130-131` now reads *"**FOUR of revision 4's edits landed in
  sections frozen by an earlier review — §1, §2, §6 and §7**"*, and its table
  lists all four.
- **"revision 5 applied the fix"** — `git show 41a52f0:…:102-103` already carries
  the corrected "FOUR" sentence. TRUE.
- **"and left this row out"** — `git show 41a52f0:… | /usr/bin/grep -n "N3"`
  returns **nothing**. TRUE.

**The empty before/after columns are correct**, not a breach of §0.2b's own rule:
that rule (`:155-157`) binds *"every test this round moves, retires or re-drives"*,
and N3 moves no test. §0.2b already carries `—`/`—` rows of exactly this kind
(N8), as does §0.2c (M-3, M-8, M-9).

**§0.2c's summary is now accurate on the limb F-2 named.** The header at `:11`
no longer says *"corrects three counts"*; it says *"corrects two counts; revision
7 then completed three of those corrections and restated the fourth"*, and that
second clause is exactly right (F-1, F-2, F-3 completed; F-4 restated). See
**OBSERVATION 1** for the residue on the word *"two"*, which I do not grade a
finding.

M-9's row at `:191` (*"this table has a row for every change, N3's included"*)
is now satisfiable: the table M-9's finding was about — §0.2b — has the row. That
it took revision 7 to put it there is stated openly eight lines below, in F-2's
own row at `:199` (*"§0.2b gains the N3 row **revision 6 claimed was there**"*),
so the block does not assert a completeness it lacks.

---

## Q3. F-3 — does the new driver clause make the mutation die, and is its claim about the stub true? — **YES, and the claim is true over the domain the sentence governs.**

**The driver column at `:887` is no longer blank:**

> `| a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote | 6 | **unit, over the record writer, synthetic** — the stub's bestmove is always one canonical turn after one space, so a field re-rendered from the parsed Turn would write identical bytes and the mutation would be invisible |`

**The mutation is now observable, and the chain is short.** The registered mutant
(`:934`) is *"the `bestmove` field written from the parsed turn rather than from
the engine's own line"*. §4.2 (`:520-521`) fixes field 5 as *"**the engine's own
bytes off the channel**"*, and `crates/pistol-arena/src/channel.rs:96-106` trims
only a trailing newline/CR run — so an interior double space survives to the
writer. A **unit over the record writer** can therefore hand it `bestmove  q,r`;
under the mutant, `rest.trim().parse::<Turn>()`
(`crates/pistol-arena/src/exchange.rs:59-68`) absorbs the extra space and
`bestmove_line` (`crates/pistol-cli/src/report.rs:107-108`,
`format!("{BESTMOVE_PREFIX} {best}")`) re-renders one space. **The bytes differ,
the test fails, the mutant dies.** The clause also matches the section's own
established vocabulary — `a_captured_field_containing_a_tab_refuses_the_run_by_name`
at `:888` carries the identical *"unit, over the record writer"* driver for the
same reason (no engine in this tree can produce the input).

**The claim about the stub is true of everything the capture can write.** Ten of
the eleven `Behave` variants (`crates/pistol-arena/src/bin/stub_engine.rs:13-54`)
answer through `pistol_cli::Session` (`:266`) and therefore through
`bestmove_line` — one space, one canonical `Turn` — and `Behave::Illegal`
(`:325-333`) writes `format!("{} {}", BESTMOVE_PREFIX, Turn::single(…))`, also one
space and canonical. `deviate` (`:354-361`) rewrites only `id protocol `.
`Garbage`, `Hang` and `Exit` write no `bestmove` at all. The single exception,
`Behave::BadBestmove` (`:319-322`), writes `bestmove not-a-turn` — which fails the
parse and forfeits, so it is never a line the record writer is handed. See **the
strongest attack that did not land**, where I take this as far as it goes.

**The `pins BEFORE / pins NOW` columns on F-3's own row (`:200`) are filled and
correct**, which is the one row of the four where §0.2b's discipline actually has
work to do.

---

## Q4. F-4 — is the restatement accurate on all four limbs? — **YES, on all four.**

### (a) The three configs' self-descriptions are quoted exactly at the cited lines

`sed -n "<n>p"` on each file, verbatim:

| cited | line's actual text | design's quotation |
|---|---|---|
| `configs/bench_wp18c_solver_on.toml:15` | `# NOT an SPRT arm and never a committed engine config: the committed configs` | *"NOT an SPRT arm and never a committed engine config"* ✔ |
| `configs/gate_staged_solver_v0.toml:8` | `# strength seat: it is never an SPRT arm and never the committed config.` | *"never the committed config"* ✔ |
| `configs/play_staged_solver_v0.toml:8` | `# THIS IS NOT A DEPLOYMENT CONFIG. D-441's "gate OFF in every committed config` | *"THIS IS NOT A DEPLOYMENT CONFIG"* ✔ |

Each quoted fragment is wholly contained on the single line cited — which matters,
because the checker resolves a citation at its line and revision 6 cited these
same declarations as ranges. All three are on point: each of the three files
declares, in its own header, that it is not the thing the deleted clause was
about.

### (b) D-441 / D-520's vocabulary is represented correctly

- `docs/decisions.md:948` (D-441) is titled *"THE SOLVER IS ON THE SEARCH PATH
  BEHIND A GATE THAT IS **FALSE IN EVERY COMMITTED CONFIG**"* — the universal the
  design attributes to it.
- `docs/decisions.md:1108` (D-520) reads *"D-441's **"gate OFF in every committed
  config" binds DEPLOYMENT configs; this is a measurement seat**, on the precedent
  of `configs/gate_staged_solver_v0.toml` and
  `configs/bench_wp18c_solver_on.toml`."* The design's front page (`:22-24`) says
  D-441's phrase *"binds what pistol SHIPS, which D-520 restates as the same
  distinction"* — a faithful paraphrase, and the quoted fragment is D-520's own
  quotation of D-441, word for word. `configs/play_staged_solver_v0.toml:8-9`
  quotes it identically.
- **The ambiguity claim itself is measured, not asserted.** `git grep -n
  "on_search_path" -- configs/` gives 20 assignments across 20 engine configs:
  **three `true`** (`bench_wp18c_solver_on.toml:45`,
  `gate_staged_solver_v0.toml:47`, `play_staged_solver_v0.toml:75`) and
  **seventeen `false`**, including every deployment seat
  (`play_v0.toml:89`, `play_staged_v0.toml:64`, `gate_v0.toml:94`). So §14.1's
  *"true of deployment configs, false of files committed under `configs/`"*
  (`:1165-1171`) is exactly right, and `configs/gate_v0.toml:94`, still cited at
  `:1165`, is still `on_search_path = false`.

### (c) "stub-driven" now appears in both places §14.1 needed it

`/usr/bin/grep -n "test in this crate"` returns two sites and **both carry the
qualifier**: `:1156` (*"no stub-driven test in this crate can produce the
spelling"*) and `:1173` (*"the whole reason **no stub-driven test in this crate**
can reach the spelling"*). The second is the one revision 6 left narrow at one end
and universal at the other, and it is the site that would otherwise have been
false of §10's own
`the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line`
(`:884`), which is a test in this crate that reaches the spelling synthetically.
**One rule, one scope, both sites.**

### (d) The deletion is still the right call under D-424

**Yes, and the ground is stronger than the clause ever was.** `render_info` emits
the solver block only `if info.solver_nodes > 0`
(`crates/pistol-cli/src/report.rs:62`) and the stub sets `solver_nodes: 0`
(`crates/pistol-arena/src/bin/stub_engine.rs:123`). That fact governs an in-crate
mutation run whichever sense of *"committed"* a reader takes, so both readings of
the deleted clause license the identical conclusion — which is D-424's own test
verbatim (*"where both sides of a distinction license the same conclusion it is
not a distinction, and it is DELETED rather than refined"*), and the document now
says so in those terms at `:1174`. The clause is prose that constrains nothing;
deleting it is the fix, and refining it would have been the error.

**And the paragraph's tail survived the rewrite in better shape.** Revision 6's
*"the universal's falsity cuts the other way anyway"* is now *"the ambiguity cuts
toward the synthetic test rather than against it: **a pilot naming a measurement
seat** would produce solver-bearing totals lines"* (`:1175-1178`) — the same
argument, with the config class that produces the spelling now named instead of
left as *"a pilot config"*.

---

## Q5. Did revision 7 spend anything? — **NO. Ten hunks, walked one by one, independently of §0.2c.**

`git diff -U0 4a12b46 4752192 -- docs/experiments/wp20m_design.md` gives exactly
**ten** hunks. (`4a12b46` is the intervening WP-2.0-S commit; `ff1c575 → 4752192`
also carries that commit's 241 lines in `wp20s_design.md`, which are not this
document.) The commit touches **two files**: this design and the rev-6 review
report it lands alongside.

| hunk | what it does | which finding | removes anything? |
|---|---|---|---|
| `-3,2 +3,5` | revision banner rewritten to REVISION 7 | disclosure | no — revision 5's PASS and its 0/0/9 counts are carried forward at `:6-7` |
| `-8 +11,2` | *"corrects three counts"* → *"corrects two counts; revision 7 then completed…"* | F-2 | see OBSERVATION 1 |
| `-11,6 +15,15` | the session's-own paragraph restated as an ambiguity | F-4(a) | see OBSERVATIONS 2 and 3 |
| `-155,0 +169` | **pure insertion** of the N3 row | F-2 | no |
| `-178 +192,10` | *"false universal"* → *"ambiguous universal"* in §0.2c's last row; revision 7's own four-row table appended | F-4, disclosure | no |
| `-697 +720` | *"the dispatch's own"* → *"the WP-2.0 dispatch's own"* | F-1 | no |
| `-864 +887` | blank driver column filled | F-3 | no |
| `-1047 +1070` | *"The standing dispatch"* → *"The WP-2.0 dispatch"* | F-1 | no |
| `-1140,9 +1163,16` | §14.1's universal-paragraph rewritten | F-4(a),(b) | see OBSERVATIONS 2 and 3 |
| `-1152 +1182` | *"The standing dispatch's"* → *"The WP-2.0 dispatch's"* | F-1 | no |

**Nothing that was pinning a rule was removed.** Specifically, and checked rather
than assumed:

- **No test, mutant, invariant or driver was retired.** The only line in §10 that
  moves is the driver column at `:887`, which goes from empty to filled; the test
  table and the mutant table are otherwise byte-identical to revision 6's. The set
  closure revision 6 was checked on — 38 tests, 29 mutants, one stub-behaviour
  token — is untouched.
- **No mechanism, invariant or decision moved.** Seven of the ten hunks are one
  line each.
- **§1's clause that §14.1 used to lean on is still where it lives** — `:227`,
  inside the lifted block, *"both engine sections naming the same committed
  config"*. The rev-6 reviewer's strongest attack was that this clause would go
  down with the universal; it did not go down at revision 6 and it has not moved
  at revision 7.
- **The replacement ground is unchanged and still cited**:
  `crates/pistol-cli/src/report.rs:62-81` and
  `crates/pistol-arena/src/bin/stub_engine.rs:120-131` are both still in §14.1 at
  `:1153-1155`, and both resolve.

---

## Q6. Is the freeze intact? — **YES, mechanically, on all three limbs.**

1. **No verbatim-lift block altered.** `/usr/bin/grep "^>"` over
   `git show ff1c575:…` and over HEAD returns **118 lines each**, and `diff`
   between the two streams is **empty**. Revision 7 changed no quoted line.
2. **§12 is still word-identical to `5064b05`'s §4.** §12 runs `:999-1047`; its
   quoted stream, `> `-stripped and whitespace-normalised, is **152 words**, and
   `diff` against `5064b05:79-97` normalised the same way is **empty**. D-540's
   fresh-process criterion and its defect-class clause are intact.
3. **No numbered section moved and no edit is unlisted.**
   `diff <(git show ff1c575:… | /usr/bin/grep "^## \|^### ") <(/usr/bin/grep "^## \|^### " …)`
   is **empty** — the heading list is identical, so §0.4's stability rule
   (`:211-214`) holds and revision 7's new table is a sub-block inside §0.2c. All
   ten hunks are rowed: five to F-1/F-3's single-line edits, two to F-2, two to
   F-4, and the banner, which every reviewer in this arc has treated as
   disclosure.

---

## Q7. The citation checker, and every citation NEW in revision 7 — **green, and all three new ones hand-verified.**

```
$ python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/capture.rs \
    --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md \
    --proposes configs/arena_wp20_label_pilot.toml docs/experiments/wp20m_design.md
docs/experiments/wp20m_design.md: 123 citation(s) checked, 0 unreproduced
DESIGN_CITATION_CHECK_DONE
EXIT=0
```

**The set delta is exactly three swaps and nothing else.** Extracting every
`file:line` token from both revisions and diffing the sorted multisets:

```
<  2 configs/bench_wp18c_solver_on.toml:45     >  2 configs/bench_wp18c_solver_on.toml:15
<  2 configs/gate_staged_solver_v0.toml:47     >  2 configs/gate_staged_solver_v0.toml:8
<  2 configs/play_staged_solver_v0.toml:75     >  2 configs/play_staged_solver_v0.toml:8
```

No other citation was added, removed or moved, which is why the total stays at
123. The three new ones (twice each — front page and §14.1) are the config
self-descriptions, and I read all three at the file rather than trusting the
instrument: the table in **Q4(a)** is that reading. **They are the important ones
and they are exact.**

The checker's own disclaimer applies and I did the work it disclaims: it cannot
tell whether a true quotation supports the claim built on it. The claim built on
these three — that each file classifies itself outside the clause's set — is what
each line actually says.

---

## OBSERVATIONS — four, and none of them is a finding

I name each, with the reason it is not graded, so the next round does not
re-derive them.

### OBSERVATION 1 — the header says *"corrects two counts"*; the governing review measured one

`:11`. The rev-6 review's Q3 (`wp20m_design_REVIEW_rev6.md:274-278`) adjudicated
this explicitly: *"**The counts are one**: §0.2's §6 row, three → four. The N4
row's leg list is a list, not a count"*, and its FIX offered two wordings —
*"drop it"* or *"corrects one count and two coverage claims"*. The author took a
third. The second correction being counted is M-1, the N4 row's three legs
becoming two.

**Not a finding.** The disputed word is *"count"* versus *"list"*, and the N4 row
the author is counting uses the word *"third leg"* in its own corrected text
(`:167`), so calling it a count is a defensible reading rather than a new claim.
Nothing downstream reads the header's summary adjectives, so both sides license
the same conclusion — D-424's own test, applied to the sentence rather than by it.
**The substantive half of F-2 — the missing N3 row, which was a real completeness
gap — is applied and true.** If the operator wants the ledger to agree with its
own review's arithmetic, the edit is one word at `:11`.

I checked the header's other three summary clauses for the same class and they
survive. *"deletes two false clauses"* is still satisfiable without the clause
revision 7 reclassified: revision 6 also deleted §14.1's *"WHY THIS IS THE ONLY
WAY THE SOLVER SPELLING CAN BE REACHED"* heading and N4's third leg, which is two.
*"replaces two mutants"* (M-2, M-4) and *"names three test drivers"* (M-2's driver
column, M-5, M-6) are exact.

### OBSERVATION 2 — three value citations were dropped while the claim they supported was kept

Revision 6 cited `configs/bench_wp18c_solver_on.toml:45`,
`configs/gate_staged_solver_v0.toml:47` and `configs/play_staged_solver_v0.toml:75`
for *"all arm it"*. Revision 7 keeps the claim — *"the three files under
`configs/` that arm the solver"* (`:1167-1168`), *"those same three files are the
counterexamples"* (`:1171`) — and points its citations at the self-descriptions
instead.

**Not a finding, and not a spend.** The claim is TRUE and I verified it
independently at the three lines by `git grep -n "on_search_path" -- configs/`;
each is `on_search_path = true`. No conclusion in the document rests on it — the
paragraph's whole point is that the clause did no work — and the checker cannot
flag a claim carried as bare prose, which it says of itself. If the operator wants
belt-and-braces, the fix is to keep both citations in the one sentence.

### OBSERVATION 3 — the correction's provenance is no longer on the document's face

Revision 6's header said the §14.1 correction *"was found by the concurrent
WP-2.0-S review, not by this document's own"*. Revision 7's replacement keeps
*"NOT THE REVIEWER'S AND IS RECORDED AS THIS SESSION'S OWN"* (`:14-15`) but drops
the sibling's name; `/usr/bin/grep -n "WP-2.0-S"` shows nine remaining mentions,
none of them about this correction.

**Not a finding.** D-547 requires an edit to a frozen section to be listed **with
its ground**, and the ground is stated at greater length than before; who found it
is disclosure, and the disclosure that matters — that the correction came from
outside this document's review — survives verbatim, as does §0.2c's *"this
session's own"* row at `:192`.

### OBSERVATION 4 — the sibling document still carries the claim F-4 corrected (OUTSIDE THIS DISPATCH'S SCOPE)

`docs/experiments/wp20s_design.md:1200-1206`, at HEAD, reads: *"Revision 2
grounded that on a universal — "every committed config has the solver off the
search path" — and **the universal is false**: **three committed configs arm it**
(`configs/bench_wp18c_solver_on.toml:45`, `configs/gate_staged_solver_v0.toml:47`,
`configs/play_staged_solver_v0.toml:75`)"*.

That is the flat falsity claim revision 7 just restated as an ambiguity in this
document, and it uses *"committed configs"* in precisely the tracked-file sense
the three files' own headers and D-520 contest. **I am not reviewing WP-2.0-S and
I do not grade this**, but the two sibling designs now say different things about
the same sentence, which is D-423's class across documents rather than within one.
The operator should know that F-4's remedy landed in one of the two documents the
correction came from.

---

## THE STRONGEST ATTACK THAT DID NOT LAND

**I attacked F-3's new driver clause on the hypothesis that it repeats the exact
defect F-4 was raised to fix: a universal about the tree stated on the document's
face with a counterexample in the tree.** The clause says *"the stub's `bestmove`
is **always** one canonical turn after one space"*, and `Behave::BadBestmove`
(`crates/pistol-arena/src/bin/stub_engine.rs:319-322`) writes
`bestmove not-a-turn` — one of the eleven behaviours, writing a `bestmove` line
that is not a canonical turn. The rev-6 reviewer had enumerated that variant by
name in the finding the author was applying, so the author had the counterexample
in hand and wrote *"always"* anyway.

**It does not land, on the sentence's own domain.** The clause is not a claim
about what the stub can print; it is the second half of a conditional whose first
half is *"a field re-rendered from the parsed `Turn` would write identical
bytes"*. A field is only re-rendered if it was parsed, and `bestmove not-a-turn`
fails `rest.trim().parse::<Turn>()` (`crates/pistol-arena/src/exchange.rs:59-68`)
and forfeits the game before any record is written — so it is never a line the
record writer holds. **Over the domain the sentence governs — `bestmove` lines the
capture writes — the universal is exact and has no counterexample**, and the
conclusion the clause exists to license (that the stub cannot drive this test, so
the driver must be a synthetic unit) is true by both routes rather than by one.

Three further attacks failed and are recorded so nobody re-runs them:

- **That F-1's qualification could have mis-attributed a quote to the first
  dispatch when the third also registers it.** The third does list *"ledger
  overwrite"* among its mutants (`wp20_dispatches.md:321`), but not in the form
  quoted; `/usr/bin/grep -c -F` returns exactly one hit for each of the five
  quoted strings, all in the first dispatch. And the design's claim at `:1074-1078`
  — that this is *"a departure from governing dispatch text"* — holds a fortiori
  if both dispatches register it.
- **That adding a table inside §0.2c pushed a section a reviewer had adjudicated.**
  It did not: the heading lists at `ff1c575` and HEAD are identical.
- **That §14.1's rewrite would drop the "cuts the other way" clause** the
  synthetic test's justification uses. It is retained and improved at
  `:1175-1178`, now naming the measurement seat as the config class that produces
  the spelling.

---

## WHAT I COULD NOT SETTLE BY READING, AND THE RUN THAT WOULD

- **F-3's fix is a design registration, not code**, so nothing here is
  observable yet. When the implementation lands, the run that confirms it is: in a
  separate `git worktree add --detach` with its own `CARGO_TARGET_DIR`, write
  field 5 from the parsed `Turn` and run `cargo test -p pistol-arena` — with the
  new driver the registered test must go RED, where under revision 6's blank
  driver it would have stayed green. Refused here per the dispatch.
- **Everything else in this check was settled by reading**, because every claim
  the round makes is about the document's own text or about a line in the tree,
  and I read every one of them at the file.

---

## A closing note for the architect

**The four corrections are complete and the round cost nothing.** Ten hunks,
seven of them a single line, no quoted byte touched, no test or mutant retired,
and the citation set changed by exactly the three swaps F-4 required. The ledger
that failed twice — §0.2b's row set, §0.2c's row truth, and the header's summary —
is now accurate on every limb that carries a conclusion, and revision 7 does the
thing the last two rounds did not: it records its own four corrections in a table
of their own rather than folding them into the previous round's.

**The correction worth naming is F-4.** The round found a front-page claim of its
own making, discovered the tree's vocabulary contradicted its classification
rather than its values, and restated it as an ambiguity instead of quietly
softening it — while reaching the same deletion, for a better reason, under
D-424's own test. That is the arc's method working rather than the arc's method
failing, and it is the first round in this sequence where the thing corrected was
the document's confidence rather than its content.

**This document is what revision 5's reviewer said it was, and now its change log
says so too.**
