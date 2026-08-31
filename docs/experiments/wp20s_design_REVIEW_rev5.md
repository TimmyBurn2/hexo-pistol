# SCOPED RE-REVIEW — `docs/experiments/wp20s_design.md` revision 5

## Header

- **Revision adjudicated**: `ea01fea90168ae336e64fe3a0ab7d0a4d7f3b41a` (`docs(wp20s): the guard table is checked against the diff instead of the fix list, and the fourth header param stops existing in two sections and not three`).
- **Matches HEAD**: **YES**. `git rev-parse HEAD` = `ea01fea90168ae336e64fe3a0ab7d0a4d7f3b41a`, branch `dev`.
- **Tree state**: no tracked file is modified. `git status --porcelain` at the start and at the end of this review prints exactly one line, `?? docs/experiments/wp20m_design_REVIEW_rev7.md` — **an untracked file I did not write and did not read**; a concurrent review of the sibling document is evidently in flight. The only file I wrote is this one.
- **What I ran**: `git` (`log`, `rev-parse`, `status`, `show`, `diff`, `diff -U0`, `merge-base`, `grep`), `/usr/bin/grep`, `sed`, `awk`, `wc`, `cat`, `ls`, `comm`, `diff`, `tr`, `LC_ALL=C sort`, `uniq`, and `python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` — **green, 121 citations checked, 0 unreproduced** (rev 4: 120; rev 3: 119; rev 2: 104; rev 1: 60).
- **What I refused to run**, per the dispatch's hard constraint: `cargo` in any form, `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Runs I could not make are named in Part 3.
- **Binding reading, all in full**: `docs/experiments/wp20s_design_REVIEW_rev4.md` (R-M1…R-M5, R-m1…R-m11, R-m13, and its Part 1 dispositions); `git show 4a12b46:docs/experiments/wp20s_design.md`; the complete `git diff 4a12b46 ea01fea` for this file, enumerated **twice** (`-U0` and default context); `CLAUDE.md`; `docs/process.md`; `docs/decisions.md` D-483, D-518, D-531, D-537, D-539…D-548; `docs/experiments/wp20_dispatches.md`; `docs/experiments/wp20m_design.md` **revision 7** (`4752192`) — §8, §9, §10's driver column, §0.2/§0.2a-c and its header; `docs/experiments/matrix_stage3_detector.md`.
- **Code read**: `crates/pistol-core/src/{symmetry,lib}.rs`, `crates/pistol-arena/src/{exchange,channel}.rs`, `crates/pistol-arena/tests/replay_chain_tests.rs`, `crates/pistol-arena/tests/common/mod.rs`, `crates/pistol-search/src/search.rs`, `configs/random_openings_v1.toml`, and every `configs/*.toml` carrying an `[sprt]` block.

---

## VERDICT: **FAIL**

**0 BLOCKING · 2 MAJOR · 9 MINOR.**

**This is much the best revision of this document, and the round's headline instrument half-worked.** I did the §0.1a audit independently — ran the diff, enumerated the hunks twice, mapped every row to a hunk and every hunk to a row — and **the class that failed revision 4 is nearly gone**: all five of the never-applied remedies (N-m8, N-m15, N-m16, N-m17, N-m19) are genuinely in the file this time, each with a hunk, each correct against the tree. R-M3, R-M4, R-M5, R-m1…R-m10 and R-m13 are all applied and I could not break any of them; R-M4's fixture conditions are not merely stated but **exactly necessary and sufficient**, and I could not construct the pair the previous reviewer built.

**It fails on two things, and the first is the one the dispatch predicted.**

- **§5's header enumeration is byte-identical to revision 4 and still says *"the three unit params below"*** (`:758-761`). That is the FIRST site R-M1 named, the site its FIX named by line number, and the site §0.1a's own R-M1 row records as *"pins NOW: §5's enumeration"* with a ✓ in the hunk column. **There is no hunk in that range.** So a writer built from §5's own header enumeration emits three unit params and `opening_turns`, and §5's own loader (`:800-801`) refuses *"a header missing any of its params, the four keyed meaning params included"* — **the run refuses its own output**, which is verbatim the defect R-M1 raised. R-m11's row (*"every 'three unit params' site now says four"*) is false in the same way. **A row with no hunk behind it, inside the column added to prevent exactly that.**
- **§6 still assigns INVARIANT 5 a mutant that §11 explicitly repudiated this round.** R-m13's remedy repointed the ordering mutant at the order test and wrote, in terms, that it is *"**not** the re-run test … which is not a thing this document can argue"* (`:1202`). `:849` — untouched — still reads *"**INVARIANT 5** is that a re-run … is byte-identical, **and its mutant is any ordering that depends on hash iteration**"*. An implementer building the registry from §6 registers the mutant the document's own §11 says cannot die.

**What is genuinely closed, and I attacked all of it**: the one-sample rule and every prescriptive site (N-B1 stays closed); INVARIANT 6's three key-column shapes; the colour test's fixture, which I could not falsify; the numeral self-claim, which is true for the first time in five revisions; the `key_seq` test and both key-shape mutants; INVARIANT 13; the `frame`/`band` completion; the §9/§5 two-enumeration split; the citation set, which grew by one occurrence and gained no new target.

---

# PART 1 — DISPOSITION OF EVERY REVISION-4 FINDING

**Summary: 14 APPLIED · 2 PARTIALLY APPLIED · 1 APPLIED BUT INTRODUCED A NEW DEFECT · 0 NOT APPLIED · 0 MOOT.** Seventeen findings, seventeen dispositions.

## MAJOR

| # | finding | disposition | evidence |
|---|---|---|---|
| R-M1 | `depth_meaning` in no enumeration, test or mutant | **PARTIALLY APPLIED** | test ✓, mutants ✓, meanings paragraph ✓; **§5's header enumeration is untouched** — **R2-M1** |
| R-M2 | five never-applied remedies | **APPLIED**, all five, verified against the tree | see below |
| R-M3 | INVARIANT 6 pinned no key shape | **APPLIED** | `:1070-1074` |
| R-M4 | the colour test can be red on correct code | **APPLIED**, and I could not falsify the fix | `:1228-1237`; see the attack section |
| R-M5 | the numeral self-claim is false in two ways | **APPLIED**, and the replacement is true | `:920-925`; see below |

**R-M1 — the three sites that landed, and the one that did not.**

| site | revision 4 | revision 5 |
|---|---|---|
| §5's **meanings paragraph** (`:777-781`) | *"**FOUR** … keyed params"*, grammar broken | *"**FOUR** properties … **keyed params whose values a loader can check**: … — and **`depth_meaning`, that …**"* ✓ and it parses (R-m2) |
| §5's **loader** (`:800-801`) | *"the four keyed meaning params included"* | unchanged — **it was already correct in revision 4 and has no hunk**, so the row's ×4 cannot be the four sites it names |
| the **test name** (`:1128`) | `…_three_unit_params_…` | `a_corpus_missing_one_of_its_four_meaning_params_is_refused_by_name` ✓ |
| the **mutants** (`:1178`, `:1183`) | *"the three unit params dropped"* | *"the four keyed meaning params dropped **on write**"* ✓ plus a new *"`depth_meaning` dropped from the header on write"* ✓ |
| **§5's header enumeration** (`:758-761`) | *"the three unit params below"* | **byte-identical** — `diff` over `git show 4a12b46:…` `752,756p` against HEAD `758,762p` is empty |

**R-M2, all five verified against the tree and not against the row.**

- **N-m8** ✓ — `:1228-1237`, an eleven-line paragraph, hunk `@@ -1208,0 +1228,11 @@`.
- **N-m15** ✓ — `:388-390` now *"overwrites `nodes` and `search_nodes` from the run"*. `crates/pistol-search/src/search.rs:513-514` is `outcome.info.nodes = run.total_nodes();` / `outcome.info.search_nodes = run.search_nodes;`. **The remedy names exactly the two fields the cited lines assign.** ✓
- **N-m16** ✓ and **the reading is now the passage's own**. `:915-919` says the config *"records the same nominal pair falling well short at the smaller of two sample sizes and MEETING them at the larger … so a nominal pair is a registration and not a guarantee, and the gap closes with n"*. `configs/random_openings_v1.toml:47-52` reproduces: 500 pairs → alpha 0.030, power 0.569; *"At 2000 pairs the same bounds achieve alpha 0.048 and power 0.945"*. **The design's conclusion is now the passage's conclusion**, which is what N-m16 asked for. One clause in it is not the passage's (**R2-m7**).
- **N-m17** ✓ — `:882-883` now *"loses nothing on any ONGOING position and therefore on every record here (§2.1)"*, which matches §2.1 `:270` and §8 `:997` exactly. `/usr/bin/grep -n "loses nothing"` returns two lines and they agree.
- **N-m19** ✓ — `:1242-1247` names *"the real `arena` binary driven against the stub engine into a scratch directory, the shape `crates/pistol-arena/tests/replay_chain_tests.rs:12-31` already uses … and no match log is committed (hard rule 8)"*. The citation reproduces: `stub_config`, `binary_a: STUB`, `binary_b: STUB`, `run(scratch, &spec, "chain")`. **The hard-rule-8 route is now in §11, where an implementer looks.** One new sentence in the same paragraph is false (**R2-m4**).

**R-M5, and I tried hard to break the replacement.** `:920-925` now reads: *"this document makes no claim about how many numerals it carries — it has made one wrongly in four revisions running, including the replacement. What is true and worth saying is narrower: **the level and the power are the only numbers this document FIXES**, and they are conventions adopted from a committed config rather than measurements."* Both of the previous claim's falsifiers are gone: the claim is no longer about thresholds (limb 1 died on 0.05/0.95 being consumed by the census rule), and it is no longer about restatement (limb 2 died on *"five hundred pairs"*, which **N-m16's rewrite deleted**, and on *"fourteen"*, which the new claim does not touch because "fourteen" is cited, not fixed). I could not find a number this document FIXES besides the pair — see "the strongest attack that did not land". **First true version of this sentence in five revisions.**

## MINOR

| # | finding | disposition | evidence |
|---|---|---|---|
| R-m1 | the duplicated `key_seq` test row | **APPLIED** | the name now appears twice in the file — once at `:1145` (test register) and once at `:1184` (mutant target). A `uniq -d` over the register's 51 rows returns nothing |
| R-m2 | §5's params paragraph did not parse | **APPLIED** | `:777-781`; the three `that …` clauses hang off *"keyed params whose values a loader can check:"* and `depth_meaning` is joined with an em-dash |
| R-m3 | the `Phase` fact attributed to WP-2.0-M's INVARIANT 2 | **APPLIED**, and I verified the target | `:295` now says INVARIANT 1. At HEAD `wp20m_design.md:800-803` is *"The asked set is every turn boundary of every recorded game at which the engine can legally be asked"* and `:804-805` is *"No asked position is decided"*. **The turn-boundary fact is INVARIANT 1** ✓ |
| R-m4 | two surviving "frame"s | **APPLIED** | `/usr/bin/grep -n "frame"` over the whole file returns **one** line, `:112`, which is §0.1a's own row narrating the fix |
| R-m5 | the bias paragraph's "lower end" | **APPLIED** | `:978-983` — *"The trigger-rich band's bound is the SMALLEST effect the arc licenses — it is also the lower of the two bands' bounds, **which is the fact revision 2 mistook for the rule**"*. The conservatism conclusion and the FLOOR rule are unchanged and the phrasing item 3 repudiated is gone |
| R-m6 | §12.1's "only claim" | **APPLIED** | `:1286-1288`; the "only" is deleted and replaced by *"the division of labour above is a description of what the two designs each specify rather than an instruction to the other one"* |
| R-m7 | §9's "one place" claim | **APPLIED** | `:1024-1029` — *"This table enumerates every refusal on the READ side … §5 enumerates the loader's own refusals and its one write-side TAB refusal … Two enumerations, two sides"*. I checked the partition: every §5 input-side refusal has a §9 row (source digest → 1, capture schema → 2, capture identity → 3, TAB/arity/empty → 4, score tag → 8, negative mate → 9, solver block → 10, legality → 11) |
| R-m8 | `key_seq`'s check unguarded | **APPLIED** | `:1151` adds `a_key_seq_field_that_is_not_turn_tokens_is_refused_by_name`; `:1181-1182` add both shape-check mutants. Both die at their own tests |
| R-m9 | the write-side TAB test mapped to 6 | **APPLIED** | `:1150` → 7. INVARIANT 7 is *"Any failure refuses the whole run"*, which is the rule the test's name asserts |
| R-m10 | `a_capped_game_and_a_forfeited_game_…` mapped to 8 | **APPLIED** | INVARIANT 13 added at `:1089-1090` (*"`result` and `end` are separate columns, so a capped game and a forfeited one are distinguishable (§2.7)"*), the test remapped at `:1141`, and the mutant *"`result` and `end` collapsed into one column"* (`:1196`) dies there |
| R-m11 | *"three unit params"* at the test name and the mutant | **PARTIALLY APPLIED** | test ✓ `:1128`, mutant ✓ `:1178`; **`:759-760` still says it** — **R2-M1**. One further residue at `:1179` — **R2-m8** |
| R-m13 | the ordering mutant's death turned on hash draws | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:1202` is right and its reasoning is right; **`:849` was left behind** — **R2-M2** |

---

# PART 2 — THE §0.1a AUDIT, AND NEW DEFECTS

## The §0.1a audit — done independently, from the diff

**Completeness of rows: PASS.** 17 rows — R-M1…R-M5, R-m1…R-m11, R-m13 — one per revision-4 finding, matching the review's 5 MAJOR + 12 MINOR exactly. No finding is missing and no row is invented.

**The hunk count the table asserts is wrong under every reading of its own command.** `:97-100` says every row *"was checked against `git diff 4a12b46 -- docs/experiments/wp20s_design.md`, which carries **twenty-one hunks**"*. I ran that literal command:

| command | hunks |
|---|---|
| `git diff 4a12b46 -- docs/experiments/wp20s_design.md \| /usr/bin/grep -c "^@@"` | **18** |
| `git diff -U0 4a12b46 -- docs/experiments/wp20s_design.md \| /usr/bin/grep -c "^@@"` | **26** |

Neither is 21. The table's own arithmetic sums to 26, not 21: R-M1's ✓×4 plus R-M2's ✓×5 plus fifteen single-✓ rows is 24, plus the *"two hunks [that] carry no finding"* is 26 — the `-U0` count. **21 is 26 less the five hunks in the header-and-§0.1a region**, which is a defensible thing to count but is not what the sentence says, and the sentence then names one of those five (the revision header) as one of the two unrowed hunks. **R2-m1.**

**Row → hunk: 15 of 17 rows are fully backed. Two are not.**

| row | claimed | found |
|---|---|---|
| **R-M1** | *"at every site — **the enumeration**, the loader, the test name and the mutant"*, ✓ ×4, *"pins NOW: **§5's enumeration**"* | the meanings paragraph (2 hunks), the test name (1), the mutants (2) — **the header enumeration has no hunk and is byte-identical to `4a12b46`**, and **the loader has no hunk either because it needed none** |
| **R-m11** | *"every 'three unit params' site now says four"*, ✓ | `:759-760` still says it |

Every other row maps to at least one hunk that does what the row says, and I checked each against the tree rather than against the row: R-M2 → hunks 7, 10, 11, 24, 25 (five, as claimed); R-M3 → 15; R-M4 → 24; R-M5 → 11; R-m1 → 19; R-m2 → 8, 9; R-m3 → 6; R-m4 → 12; R-m5 → 13; R-m6 → 26; R-m7 → 14; R-m8 → 20, 22; R-m9 → 20; R-m10 → 16, 18; R-m13 → 23.

**Hunk → row: four unrowed hunks are undeclared, and the one declared unrowed hunk is rowed.** `:122-123` says *"**Two hunks carry no finding** and are recorded rather than hidden: the revision header, and §2.4's `search_nodes` sentence, which N-m15's remedy reached."*

- The revision header (`@@ -3,5 +3,21 @@`) ✓ genuinely carries no finding.
- **§2.4's hunk (`@@ -383,2 +388,3 @@`) IS N-m15's remedy**, which the same table counts inside R-M2's *"✓ ×5 … the five edits, each with a hunk"*. The sentence says so in its own subordinate clause. **It is double-booked**: either it is one of R-M2's five or it carries no finding, not both.
- **Four hunks are genuinely unrowed and undeclared**: `@@ -72 +88 @@`, `@@ -74,7 +90,6 @@`, `@@ -82,30 +97,4 @@` and `@@ -113,6 +102,22 @@` — §0.1a's own heading, intro, table replacement and closing note. They are self-evidently the table, but the table's closure claim is *"two"*, and it is five. **R2-m2.**

**Did any remedy spend a true thing?** One did, and it is R2-M2: R-m13's repointing of the ordering mutant left `:849` asserting the old assignment. That is the D-548 class — a remedy trading away what a sentence elsewhere was standing on — and it is the class §0.1a exists to catch. **The hunk column cannot catch it**, because the spend is at a site with no hunk; only the "pinned BEFORE / pins NOW" columns could, and R-m13's BEFORE column is filled with the finding's history rather than with the site the remedy moved off.

---

## MAJOR

### R2-M1 — §5 still states the corpus header's params two ways, and the row that says otherwise is the guard table's own

Two sites, twenty lines apart, in the section that owns the header:

| site | text | count |
|---|---|---|
| `:758-761` | *"**THE HEADER CARRIES**, as `param`: the corpus schema version; the source report's `experiment_sha256` and `source_sha256`; the label `go` line; **the three unit params below**; and `opening_turns` …"* | **3** |
| `:777-781` | *"**FOUR** properties a column name cannot carry are therefore **keyed params whose values a loader can check**: … and **`depth_meaning`** …"* | **4** |
| `:800-801` | the loader refuses *"a header missing any of its params, **the four keyed meaning params included**"* | **4** |
| `:1128` | `a_corpus_missing_one_of_its_four_meaning_params_is_refused_by_name` | **4** |
| `:1178` | *"the **four keyed meaning params** dropped **on write**"* | **4** |

**The dissenting site is the one that points at the agreeing one.** *"the three unit params below"* forward-references the paragraph that opens *"**FOUR** properties"*. An implementer building the writer from §5's header enumeration writes three; §5's loader refuses a header missing four. **The run refuses its own output** — the identical wrong answer R-M1 raised, at the identical line, after a round whose commit subject is *"the fourth header param stops existing in two sections and not three"*.

**Why this is MAJOR and not a count.** It is the only finding in this report that names a way the package produces a wrong answer, and it is the third consecutive round in which §0.1a asserts a remedy at a site the diff does not touch (rev 3: NEW-B1's row naming a loader check that existed in no rule; rev 4: five rows for work never done; rev 5: this one). The hunk column narrowed the class from five rows to one; it did not close it, because **the column records that the ROW's finding has hunks somewhere, not that each SITE the row names has one.**

**FIX.** One edit: `:759-760` → *"the four keyed meaning params below"*. Then the residue at `:1179` (**R2-m8**). Nothing else changes.

### R2-M2 — §6 registers a mutant against INVARIANT 5 that §11 says cannot die, and INVARIANT 5's test now has no mutant at all

`:846-851`, untouched this round:

> the transform is a pure function of two files, so **INVARIANT 5** is that a re-run over one capture and one report is byte-identical, **and its mutant is any ordering that depends on hash iteration rather than on the capture's own record order** — which is hard rule 4's own concern and is the property a seed mutant would have been guarding.

`:1202`, rewritten this round by R-m13's remedy:

> records emitted in any order but the capture's | `every_capture_record_produces_one_corpus_record_in_order` — **not** the re-run test, which a DETERMINISTIC reordering satisfies: both runs would produce the same reordered file. Revision 2 pointed this mutant at the re-run test, **whose death would have turned on whether two `HashMap`s in one process draw different keys, which is not a thing this document can argue**.

**§11 is right and §6 is the sentence it repudiates.** The two sites now disagree about which invariant the ordering mutation guards, and §6's version is precisely the one the document elsewhere calls unarguable. Its own §11 closes with *"**A mutant that cannot die and a test that cannot fail are the same defect from two ends**, and this arc has registered both before"* (`:1208-1209`) — and §6 still registers one.

**A second consequence, checked mechanically.** I extracted every test name from the mutant table and `comm`ed it against the 51-row register: every mutant names a registered test ✓, and **`a_rerun_over_one_capture_and_report_is_byte_identical` is now named by no mutant.** INVARIANT 5 has a test and no mutation, while §6 asserts it has one. That is a decision the design makes twice and differently, and an implementer building the registry from §6 registers a mutation the design's other half says will survive.

**FIX.** `:849` → *"and what would break it is a clock read or an ordering that is not the capture's own; the registry names the mutation and the test that dies (§11)"*, or simply delete the clause and let §11 own it — which is D-423's shape and the shape §9 was just corrected into.

---

## MINOR

1. **R2-m1** — `:98-99` says the diff *"carries **twenty-one hunks**"*. Its own named command yields **18**; `-U0` yields **26**; the table's own arithmetic (4 + 5 + 15 + 2) yields **26**. 21 is the `-U0` count less the five header-and-table hunks, which the next sentence contradicts by naming the revision header as one of the two unrowed hunks. **The sentence that makes the instrument mechanical carries a number the instrument's own command falsifies.**
2. **R2-m2** — `:122-123`: *"Two hunks carry no finding"*. **Five do** (the revision header and §0.1a's four self-rewrite hunks), and the second one named — §2.4's `search_nodes` sentence — is N-m15's remedy, which R-M2's row counts among *"the five edits, each with a hunk"*. Double-booked in one sentence.
3. **R2-m3** — `:1327-1328`: *"the record's TAB grammar, the capture identity's three inputs, the corpus manifest file and the throughput shape are all NEW in WP-2.0-M's revisions 3-5, **unfrozen, and passed by no reviewer**"*. **False at HEAD, and false at revision 4 too.** `wp20m_design.md:6-8` reads *"revision 5 (`41a52f0`) **PASSED** — 0 BLOCKING, 0 MAJOR, 9 MINOR. **The design is therefore under D-547's freeze in full**"*, and `git merge-base --is-ancestor 41a52f0 4a12b46` returns true, so revision 5 had already passed when revision 4 was written. The paragraph overstates the risk rather than understating it, so **the conclusion (*"this document changes with it"*) survives** — but it is a false claim about a governing document at HEAD, which is the D-545 class, and it has now travelled three rounds unraised. Carried; no hunk.
4. **R2-m4** — `:1246`, **new this round**: *"**That is the one test in this package that runs a binary**; the rest read files."* **False.** `the_sprt_reports_per_game_node_counts_survive_the_totals_of_split` (INVARIANT 4) pins that *"the SPRT path still bills each game's compute from the totals line"*. `compute.add` is reached from exactly one site, `crates/pistol-arena/src/exchange.rs:77`, inside `pub fn ask(channel: &mut Channel, …)`; `Channel` is a concrete struct (`crates/pistol-arena/src/channel.rs:46`) whose only constructor is `Channel::start`, which runs `Command::new(binary)`. **There is no file seam into that path**, and WP-2.0-M's own §10 test table carries a *"driver, where it is not the stub"* column precisely because its default driver is a spawned binary. `a_labels_run_prints_a_corpus_manifest_row_naming_its_digests` is a second likely falsifier, since §12.1 pins printing **on stdout**. The clause is a reassurance about hard rule 8 that nothing downstream reads, which is why it is MINOR — but it is the fifth consecutive round with a false universal, and it was introduced by a remedy.
5. **R2-m5** — INVARIANT 6's enumeration (`:1070-1074`) names *"schema version, body digest, record arity, header params, token set or key-column shape"*, and **two tests mapped to 6 pin refusals it does not name**: `a_corpus_record_with_an_empty_field_is_refused_by_name` (§2.10's rule, §5's loader) and `a_negative_mate_value_is_refused_by_name` (§2.3's rule, which §2.3 says *"the 'not a number this format writes' rule does not cover"*). R-M3's remedy widened the enumeration for the key shapes and did not sweep the other two. Carried through five revisions and never raised.
6. **R2-m6** — §8 item 1 states one claim twice, four lines apart. `:925-926`: *"The two recalls §8's rule consumes are measured, are owned by the closed arc, and are cited there rather than restated here."* `:929-930`: *"The two recalls it is applied to ARE measured, and neither is restated here."* Carried (revision 4 had the same pair in different wording), and D-423 is explicit that *"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING"*.
7. **R2-m7** — `:916-918`, new this round inside N-m16's remedy: *"MEETING them at the larger — **which is why that book was sized as it was**"*. The cited passage says the opposite of the nearest reading: `configs/random_openings_v1.toml:47-52` says *"the LLR needs 569 pairs to reach a boundary on drift alone, and **the book ran out at 500**"* — the book was **not** sized to meet the rates, it fell short of them and is retired for governed use (D-518). If "that book" means `book_v2`, that one was sized at 4500 by a different registered rule (D-518's `ceil_to_500(P + 500)`), not by the 2000-pair figure. **The rest of the sentence is exactly right** and closes N-m16; this one clause is an inference the passage does not carry.
8. **R2-m8** — `:1179`'s mutant is still *"the loader's **unit-param** check removed"* while its own test is `a_corpus_missing_one_of_its_four_meaning_params_…` and the row above it says *"the four **keyed meaning** params"*. Terminology residue of R-M1's rename; recorded separately so the fix is complete at every site.
9. **R2-m9** — `the_turn_zero_record_writes_a_dash_for_its_three_empty_columns` is the one row of the 51-test register whose "pins" column names a **section** (`§2.10`) rather than an invariant, because no invariant states the `-` sentinel rule. The document is honest about it and the rule is stated (twice, correctly, at `:585` and `:734-735` with §5 pointing at §2.10). Recorded as the register's one structural exception, not as a gap.

---

## The citation check, and AUTHOR DEBT

`python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` → **121 citations checked, 0 unreproduced.** Green before the review, per D-546's condition.

**The citation TARGET set did not change.** A `comm` over both revisions' extracted `path:line` sets gives **74 distinct targets in each, added: none, removed: none.** The occurrence count rose by one because `crates/pistol-core/src/symmetry.rs:143-155` is now cited twice — at `:279` (carried) and at `:1233` (R-M4's new fixture paragraph).

**I hand-verified every citation that is new in revision 5's TEXT** — that is, every `path:line` appearing inside one of the 26 hunks — and the ones the round's remedies lean on:

- `crates/pistol-core/src/symmetry.rs:143-155` ✓ — `pub fn transform(stones: &[(Coord, Player)], symmetry: Symmetry)` mapping `|&(cell, player)| (symmetry.apply(cell), player)` then `image.sort_unstable()`. **Colour is preserved and the origin is fixed**, exactly as R-M4's paragraph says.
- `crates/pistol-search/src/search.rs:513-514` ✓ — `outcome.info.nodes = run.total_nodes();` / `outcome.info.search_nodes = run.search_nodes;`. N-m15's remedy names precisely these two fields.
- `crates/pistol-arena/tests/replay_chain_tests.rs:12-31` ✓ — `stub_config`, `binary_a: STUB`, `binary_b: STUB`, `run(scratch, &spec, "chain")`. It is the stub-into-scratch harness §11 now says it is, and it is **no longer cited only inside the change table** (R-M2's N-m19 substance, closed).
- `configs/random_openings_v1.toml:47-52` ✓ — reproduces in full; the design's new reading matches its conclusion (one clause aside, R2-m7).
- `crates/pistol-core/src/symmetry.rs:157-165` ✓ (§7's widened scope), `crates/pistol-search/src/info.rs:162-166` ✓, `configs/arena_smoke_v0.toml:66-67` ✓ (`alpha = 0.05` / `beta = 0.05`).

**I also re-verified the three cross-document claims, which the checker cannot see.**

- **WP-2.0-M's INVARIANT 1** (R-m3's new target): `wp20m_design.md:800-803` at HEAD is the turn-boundary claim ✓, `:804-805` is *"No asked position is decided"*. The remedy corrects the number in the right direction.
- **§12.3's `totals_of` lean**: `wp20m_design.md:759` (*"**The visibility change is for WP-2.0-S**, so that package adds fields to one parser"*) and `:794` (*"it is the guard WP-2.0-S inherits when it adds `score` and `pv` as non-fatal `Option`s (D-542)"*) **both still reproduce at revision 7** (`4752192`). §12.3's claim survives another two WP-2.0-M revisions and its assignment as *"a prose-and-ADR correction WP-2.0-M owes"* is still right.
- **§12.1's six-field claim about WP-2.0-M's rows** ✓ (verified at rev 4, unchanged range).

**AUTHOR DEBT the checker could have caught: NONE. Five consecutive rounds with zero.**

**The debt the checker cannot catch** is R2-m3 (a stale claim about the sibling's freeze state), R2-m4 (a claim about the tree's test seams built on no citation), and R2-m7 (an inference beyond a true quotation). Of this round's eleven findings, **none is a `path:line` that does not reproduce**; the failure class is now entirely internal consistency and self-description.

---

## Closure over the thirteen invariants, fifty-one tests and thirty-seven mutants

**(a) Invariants with no test**: **one, declared** — INVARIANT 2 (`:1092-1102`), with the reason and the diff named as its evidence. Every other invariant (1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13) has at least one test; I extracted the register's "pins" column and counted: 1×1, 3×7, 4×3, 5×1, 6×12, 7×11, 8×4, 9×4, 10×3, 11×1, 12×2, 13×1, §2.10×1 = 51. ✓ **Closed.**

**(b) Tests pinning nothing / mis-mapped**: **two, both carried and both narrow** — `a_corpus_record_with_an_empty_field_…` → 6 and `a_negative_mate_value_…` → 6, whose rules INVARIANT 6 does not enumerate (R2-m5). **Revision 4's three are all closed**: the `key_full` shape check is now in INVARIANT 6 (R-M3), the write-side TAB test is on INVARIANT 7 (R-m9), and the capped/forfeited test has INVARIANT 13 (R-m10).

**(c) Rules with neither test nor mutant**: **none.** `depth_meaning` now has a test (`:1128`) and two mutants (`:1178`, `:1183`) ✓ — revision 4's one gap closed. The `key_seq` shape check has a test and a mutant ✓, the `key_full` shape check has both ✓, §2.7's two-column decision has an invariant, a test and a mutant ✓.

**(d) Mutants that cannot die**: **none in §11's registry — and one in §6's prose** (R2-M2). I checked all 37 individually and `comm`ed every mutant's named test against the register: **every mutant names a registered test.** The three added this round die: *"the loader's `key_full` shape check removed"* and *"the loader's `key_seq` shape check removed"* at their own refusal tests; *"`depth_meaning` dropped from the header on write"* at the round-trip test, because the loader's four-param check is intact and the written header then fails its own load. Revision 4's one unsettled mutant — the hash-ordering one (R-m13) — **is resolved correctly**: a deterministic reordering is invisible to a re-run comparison and visible to an order assertion, which is exactly the argument `:1202` makes. **The mutant I still cannot argue from the text is the write-side TAB one**, carried from revision 4's own Part 3 item 1 and for the same reason: no corpus field the transform computes can carry a TAB on the shipped path, so the test must sit at a validating write seam the design does not name. I do not raise it, for revision 4's reason — a validating write seam is the ordinary shape.

**(e) Tests that can fail on correct code**: **none.** R-M4 is closed and I attacked it hardest — see below. **Tests that pass vacuously**: **none**; the one candidate class (`the_transform_spawns_no_process_and_reads_no_clock`) is declared removed with its reason at `:1207-1209`.

**(f) Rules stated twice and differently** — the sweep, aimed at this round's five remappings plus everything with history:

| where | verdict |
|---|---|
| **§5's header params** | **NOT clean — R2-M1** (three vs four, one dissenting site) |
| **INVARIANT 5's mutant** | **NOT clean — R2-M2** (§6 vs §11) |
| §8's test | **clean** — `/usr/bin/grep -n "two-proportion\|one-sample\|ONE-SAMPLE"` returns eight lines: `:26-28` and `:962` narrate the closed defect in the past tense, `:953` quotes revision 2; **every prescriptive use (`:951`, `:955`, `:959`) is one-sample** ✓ N-B1 stays closed |
| `key_full`'s scope | **clean** — `:270`, `:882`, `:997` all say "any/an ongoing position", and revision 2's "and only on them" is gone |
| INVARIANT 4's strength | **clean** — `:653-654` and `:705-707` agree; `/usr/bin/grep -n "byte-identit"` returns four lines and none dissents |
| "frame" → "band" | **clean** — one occurrence in the whole file and it is §0.1a's own row |
| §9 vs §5's enumerations | **clean** — R-m7's split holds; I checked the partition row by row |
| §12.1's file claims | **clean** — the "only" is gone and the two-table description is framed as description |
| the book boundary | **clean** — prose, two tests and two mutants all agree at `k <= opening_turns` |
| §8's alternative and its bias | **clean** — R-m5's rewrite removes the repudiated phrasing and the number does not move |
| §2.10's `-` sentinel | **clean** — `:585` states it, `:734-735` points at it |

**(g) The false-universal sweep** (`every`, `no`, `none`, `only`, `always`, `never`, `all`, `any`). I stripped inline-code spans and inspected every surviving instance across 1365 lines. **One is false** — `:1246` (R2-m4). **One is false about another document** — `:1327-1328` (R2-m3). Everything else holds, and I re-checked the ones with history against the tree rather than against the previous review:

- `:908-910` *"**Every** committed arena experiment config in this repository carries `alpha = 0.05` and `beta = 0.05`"* — **verified**: `/usr/bin/grep -rn "^alpha = \|^beta = " configs/ | LC_ALL=C sort` returns 26 lines across 13 files and **every one is 0.05** ✓.
- `:621-624` *"the **only** place in `pistol-arena` that tells `info totals …` from `info …`"* — `git grep -n "TOTALS" -- crates/pistol-arena/src/` returns one line, `exchange.rs:173` ✓.
- `:477-478` *"A `git grep` for `Provenance` under `crates/pistol-cli/src/` names the type on **no** line at all"* — returns nothing ✓.
- `:286-287` *"**All three** are exported from `crates/pistol-core/src/lib.rs:85-90`"* — `pub use state::GameState;` and `pub use symmetry::{Symmetry, canonical_form, canonical_sequence};` ✓.
- `:1216-1221` the three-config counterexample ✓ (re-verified).
- `:754-756` *"A colon appears in **no** turn token, **no** cell token and **no** score spelling"* ✓ (carried, re-verified against `coord.rs:136-141` and `report.rs:145-158`).

**(h) The D-483 numeral sweep.** Stripping inline code, D-numbers, and §/WP/INVARIANT/revision/item/row/rule/gate/finding indices leaves: section and column and refusal-row indices; **three code facts** — *"twelve"* images (`symmetry.rs`), *"six"* solver fields (`report.rs:62-81`), *"thirty-two"* / *"32"* hex digits (`zobrist.rs:70-76`) — all pinned; the `0.05` / `0.95` pair at `:912`, argued as a convention adopted from a committed config; and **one measured figure from the closed arc**, *"the arc's own **fourteen** firings"* (`:954`), which is used to argue that the two-proportion form is unsatisfiable and is **not consumed by the rule**. **`"five hundred pairs"` is gone** — N-m16's rewrite deleted it, which is the one measured value the previous three rounds flagged as restated. **No measured number enters the rule, and the document's claim about its own numerals is true for the first time.** The D-483 position is sound.

---

# PART 3 — THE VERDICT

## **FAIL** — 0 BLOCKING · 2 MAJOR · 9 MINOR

Prior-round disposition: **3 of 5 MAJOR applied cleanly, 1 partially, 1 fully in substance; 10 of 12 MINOR applied, 1 partially, 1 with a new defect. Zero not applied** — against revision 4's five.

**Three things are worth recording as movement, and they are not small.**

1. **The never-applied class is closed.** All five remedies revision 4 recorded and did not make are in the file, each with a hunk, each correct against the tree — I checked all five against `git show 4a12b46:` and against the code they cite, not against the row.
2. **Revision 4 had one defect that made the package produce a wrong answer; revision 5 has one, and it is the same one at one site instead of three.** §5 now dissents from itself at exactly one line out of five.
3. **The self-descriptive claims are finally true.** The numeral self-claim — false in four consecutive revisions — is true; the citation set has zero author debt for the fifth round; the one-sample rule is complete on all six axes and unmoved; and both of the arc's hardest decisions survived another fresh attack.

**What fails it is that the guard table's new column does not check the thing it was added to check.** A ✓ in "in the diff?" means the row's finding produced hunks somewhere, not that each SITE the row names has one — and R-M1's row names *"§5's enumeration"* as what pins it now, over a passage byte-identical to the revision it claims to have fixed. That is the third consecutive round in which §0.1a asserts a remedy at a site the diff does not touch, and it is the sentence a fifth-round author would build from.

## "Could an implementer build from this without deciding something the design should have decided?"

**NO.** They would have to decide:

1. **How many keyed params the corpus header carries** — three (§5's header enumeration) or four (§5's meanings paragraph, §5's loader, the test and both mutants). A writer built from the first emits a corpus the loader built from the second refuses (**R2-M1**).
2. **What mutation guards INVARIANT 5** — §6 says the hash-ordering one; §11 says that mutation guards INVARIANT 1's test and that its death against the re-run test *"is not a thing this document can argue"*. INVARIANT 5's test is named by no mutant (**R2-M2**).

**That is the whole list.** Everything else — the sixteen columns and their order, the three keys and exactly what each folds, the `q,r:p1` spelling and its non-collision, the `-` sentinel and the no-empty-field rule, the parser split and the two-token score read, the node pair and its gate-off fallback, the all-six-or-none solver refusal, `depth_turns`'s two meanings and their discriminator, the outcome relation and its seat-blindness, the no-dedup and no-seed policies, the book boundary in prose and in both tests and both mutants, the twelve refusal rows and the two-sided enumeration split, §8's complete one-sample rule and its `key_full` denominator, INVARIANT 6's three key shapes, the colour test's two fixture conditions, the stub-driven outcome fixture and its hard-rule-8 route, the manifest row and the two-table file, and §12.3's four leans — **is decided, and decided well.**

## The strongest attack that did not land

**I set out to break R-M4's fixture conditions**, because the previous reviewer had built a legal counterexample and the round's answer was two clauses of prose rather than a mechanism. **It held, and the conditions turn out to be exactly necessary and sufficient.**

- `canonical_form` is *"the least of its twelve images"* under a group that **fixes the origin** and whose `transform` **preserves colour** (`symmetry.rs:143-155`, `:157-165`, read). So `canonical_form(A) == canonical_form(B)` **if and only if** some symmetry carries A's coloured stone list onto B's — the canonical form is a complete invariant of the orbit, not merely a non-colliding one. The condition §11 states, *"are **not symmetry images of each other**"*, is therefore the exact negation of the collision. **There is no legal pair that satisfies it and still makes the test red**, and the previous reviewer's five-stone pair (A and B related by the 180° rotation) is precisely what it excludes.
- **And the conditions are jointly satisfiable**, which is the other half — a condition that cannot be met would be a test that cannot be written. Take a five-cell set containing the origin whose setwise stabiliser in the twelve is trivial, e.g. `{(0,0), (1,0), (2,0), (3,0), (0,1)}`, with **A**: P1 `{(0,0),(1,0),(2,0)}` / P2 `{(3,0),(0,1)}` and **B**: P1 `{(0,0),(3,0),(0,1)}` / P2 `{(1,0),(2,0)}`. Both are legal 3-turn prefixes with rule 3's required 3-P1 / 2-P2 split and the origin in P1; both are far inside the radius-8 region; neither wins. A's P1 is three collinear stones and B's is not, and a lattice symmetry preserves collinearity, **so no symmetry carries A to B** — while the shared cell set makes *"`key_full` rendered as bare cells"* die, which is the condition the same paragraph adds for the mutant. **Both conditions hold at once and both are needed.**
- The paragraph's first sentence says *"an image of **itself**"* where it means "of the other", and *"consistently with the colour swap"* where `transform` swaps nothing — but the operative sentence three lines down (*"are **not symmetry images of each other**"*) is unambiguous and correct, so no reading is lost and I do not raise it.

**A second attack failed.** I tried to show that R-M5's replacement self-claim is false the way its four predecessors were, by finding a number this document FIXES besides the level and the power. The nearest candidate is the loader's *"thirty-two hex digits"* — but the document does not fix it: it quotes `Key128`'s own `Display` doc at `:740-741` and checks against pistol-core's rendering, which is the same relation the sentence itself describes for 0.05/0.95 (*"adopted from a committed config"*). The column count, the four-param count and the schema version are structural decisions, not numeric values the rule reads. **The claim is narrow enough to be true and I could not falsify it.**

**A third failed.** I tried to show §12.3's `totals_of` lean had gone stale across two further WP-2.0-M revisions (rev 6 `ff1c575`, rev 7 `4752192`, both landed since revision 4). It has not: `wp20m_design.md:759` and `:794` still carry both quoted phrases verbatim at HEAD, inside a §8 that revision 5 passed and D-547 froze, so the sentence's assignment — a prose-and-ADR correction WP-2.0-M owes, not a change to §3 — is still the right one.

## What I could not settle by reading, and the run that would

1. **Whether `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` is constructible.** No corpus field the transform computes can carry a TAB on the shipped path, so the test must sit at a validating `write_record` seam the design does not name. **The run**: write it in a `git worktree add --detach` and see whether it needs an entry point the design has not specified. **Refused by the hard constraint.** Carried unraised from revision 4 for the same reason.
2. **Whether `the_sprt_reports_per_game_node_counts_survive_the_totals_of_split` can be written without spawning.** I established that `compute.add` sits inside `exchange::ask(&mut Channel, …)` and that `Channel::start` spawns `Command::new(binary)` — so on the shipped shape it cannot, which is what R2-m4 rests on. What I could not settle is whether an implementer would introduce a seam rather than drive the stub. **The run**: write the test in a worktree. **Refused.**
3. **Whether the `fields_of`/`totals_of` split compiles and leaves the SPRT report's node counts intact.** Three identical expressions over the same `Vec<&str>` with elided lifetimes; I expect yes. **The run**: `cargo test --workspace --locked` plus gate 15. **Refused.**
4. **Whether the one-sample rule yields a finite, sensible `n`** at the arc's incumbent against its trigger-rich bound, level 0.05, power 0.95. **The run**: a `scipy`/`statsmodels` power calculation. **D-483 forbids the answer entering the design**, which is the design's own position and is correct.
5. **Whether gate 17's soft cap holds once the fourth arm, the loader and `usage.rs` land.** **The run**: `tools/file_justification_check.sh` at the post-implementation revision. Not run.

---

## One paragraph for the operator

**Two one-line edits stand between this document and a pass, and neither touches a decision.** §5 says the corpus header carries *"the three unit params below"* in the sentence that points at a paragraph declaring FOUR, so a writer built from the enumeration emits a corpus this same section's loader refuses — the identical defect revision 4 was failed for, now at one site instead of three, and §0.1a's own R-M1 row records that very site as fixed when the diff shows the passage is byte-identical. And §6 still tells an implementer that INVARIANT 5's mutant is the hash-ordering one, eleven lines after §11 rewrote that assignment and said in terms that its death *"is not a thing this document can argue"*. **Everything else in the round worked, and the parts that worked are the hard parts**: all five never-applied remedies are genuinely applied and correct against the code they cite, INVARIANT 6 now pins all three key shapes, the colour test's fixture conditions turn out to be exactly necessary and sufficient and I could not build the pair the last reviewer built, the numeral self-claim is true for the first time in five revisions, and the citation set reaches 121 with zero author debt for the fifth consecutive round. **The instrument itself improved and did not close**: the "in the diff?" column certifies that a row's finding produced hunks somewhere, not that each site the row names has one, which is why one false row got through it. **The fix for the last grant round is three edits and one rule**: `:759-760` → *"the four keyed meaning params below"*; `:1179` → *"keyed meaning param"*; delete or repoint `:849`'s mutant clause; and make the NOW column name a `file:line` rather than a section, so that "pins NOW: §5's enumeration" cannot be written without opening §5's enumeration.
