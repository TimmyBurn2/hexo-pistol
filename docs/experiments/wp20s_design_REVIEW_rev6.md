# SCOPED RE-REVIEW — `docs/experiments/wp20s_design.md` revision 6

## Header

- **Revision adjudicated**: `d4b1915f49a706dadfc6617d3aa65885ac755e3f` (`docs(wp20s): the last granted round — the header says four params in every section, invariant 5 gets a mutant that can die, and the sibling's correction reaches this document too`).
- **Matches HEAD**: **YES**. `git rev-parse HEAD` = `d4b1915f49a706dadfc6617d3aa65885ac755e3f`, branch `dev`.
- **Tree state**: **clean**. `git status --porcelain` printed nothing at the start of this review and nothing at the end. The only file I wrote is this one.
- **What I ran**: `git` (`log`, `rev-parse`, `status`, `show`, `diff`, `diff -U0`, `grep`), `/usr/bin/grep`, `sed`, `awk`, `wc`, `cat`, `ls`, `comm`, `diff`, `tr`, `LC_ALL=C sort`, `uniq`, `fold`, and `python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` — **green, 122 citations checked, 0 unreproduced** (rev 5: 121; rev 4: 120; rev 3: 119; rev 2: 104; rev 1: 60).
- **What I refused to run**, per the dispatch's hard constraint: `cargo` in any form, `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Runs I could not make are named in Part 3.
- **Binding reading, all in full**: `docs/experiments/wp20s_design_REVIEW_rev5.md` (R2-M1, R2-M2, R2-m1…R2-m9 and its Part 1 dispositions); `docs/experiments/wp20s_design.md` at HEAD, all 1406 lines; the complete `git diff ea01fea d4b1915` for this file, enumerated twice (`-U0` and default context); `CLAUDE.md`; `docs/decisions.md` D-423, D-424, D-483, D-518, D-537, D-542…D-548; `docs/experiments/wp20m_design.md` **revision 7** (`4752192`) — its header, §0.2's freeze table, §4.1, §4.2, §8's two lean sentences, §13(a), §14.1.
- **Code and configs read**: `crates/pistol-core/src/{symmetry,lib}.rs`, `crates/pistol-arena/src/{exchange,channel,lib}.rs`, `crates/pistol-arena/src/bin/arena.rs`, `configs/random_openings_v1.toml`, `configs/bench_wp18c_solver_on.toml`, `configs/gate_staged_solver_v0.toml`, `configs/play_staged_solver_v0.toml`, every `configs/*.toml` carrying `alpha`/`beta`.

---

## VERDICT: **PASS**

**0 BLOCKING · 0 MAJOR · 6 MINOR.**

**Both of revision 5's MAJORs are closed, and I swept for each of them the way the dispatch asked rather than at the site the change table names.** `depth_meaning` now reads FOUR at every one of the six sites in the document that counts the header's keyed meaning params, and there is no `unit param` residue anywhere in the file. INVARIANT 5 has a mutant, §6 and §11 agree about what it is, and the ordering mutant sits on INVARIANT 1's own order test. R2-m3's and R2-m4's two false claims about other documents and about the tree are replaced by claims I verified against `wp20m_design.md` at HEAD and against `exchange.rs`/`channel.rs`.

**The design is decided.** I could not find a column, a spelling, a refusal, a boundary, a key, an invariant or a rule input that an implementer would have to choose for themselves, and I looked for one specifically rather than reading for consistency. The census-minimum rule is complete on all six of its axes and unmoved; the three keys and exactly what each folds are pinned to pistol-core's own docs; the record's sixteen columns, their order, their token sets and their spellings are fixed; the two-sided refusal enumeration partitions; and the manifest row's six fields agree between §12.1 and INVARIANT 11.

**What remains is six MINORs, and five of them are in the document's own bookkeeping rather than in the package.** The one that is about the package — the read-side TAB refusal against WP-2.0-M's frozen §4.2 — has a reading available that makes it consistent, and both readings ship the same loader. **None of the six names a way the package produces a wrong answer, a requirement left in no package, or a decision an implementer must make.**

**One thing must be said plainly to the architect, because it is the round's real finding.** §0.1a's third instrument in three rounds is **structurally incapable of returning a negative**: a row's phrase always greps, because the row itself contains it. For four of the twelve rows — R2-M1's among them, the exact row the rebuild was for — the row is the *only* line the phrase matches, because the body's copy is hard-wrapped across a newline. The rows are nonetheless all true this time, which I established by reading the sites, not by running the check. **The document passes on its content; its guard is not what got it there.**

---

# PART 1 — DISPOSITION OF EVERY REVISION-5 FINDING

**Summary: 10 APPLIED · 1 APPLIED BUT INTRODUCED A NEW DEFECT · 1 NOT APPLIED IN SUBSTANCE · 0 PARTIALLY APPLIED · 0 MOOT.** Eleven findings, eleven dispositions, plus the sibling's cross-document finding the author added unprompted.

## MAJOR

| # | finding | disposition | evidence |
|---|---|---|---|
| R2-M1 | §5's header enumeration said "three unit params" | **APPLIED**, and I swept every site | `:780-781`; sweep below |
| R2-M2 | §6 gave INVARIANT 5 a mutant §11 repudiated | **APPLIED**, with one over-broad conjunct | `:867-875`, `:1228-1229`; **R3-m2** |

### R2-M1 — the full sweep, which is what the dispatch asked for

I did not check the site the row names. I extracted **every** occurrence of `three`/`four` and **every** occurrence of `param` in the file (`/usr/bin/grep -n`, 44 and 17 lines respectively) and read each in context. Every site that counts the corpus header's keyed meaning params:

| site | text | count |
|---|---|---|
| `:779-781` §5's header enumeration | *"THE HEADER CARRIES, as `param`: … **the four keyed meaning params below**; and `opening_turns` …"* | **4** ✓ |
| `:798-802` §5's meanings paragraph | *"**FOUR** properties … keyed params whose values a loader can check: [units], [sign], [mate counting] — and **`depth_meaning`** …"* — four items, and they enumerate | **4** ✓ |
| `:821-822` §5's loader | *"a header missing any of its params, **the four keyed meaning params included**"* | **4** ✓ |
| `:434-436` §2.5 | *"§5's header carries it as the keyed param `depth_meaning`, beside the score's **three** properties"* — three **score** properties plus `depth_meaning` = four | **4** ✓ |
| `:1154` test register | `a_corpus_missing_one_of_its_four_meaning_params_is_refused_by_name` | **4** ✓ |
| `:1204` mutant | *"the **four keyed meaning params** dropped on write"* | **4** ✓ |
| `:1205` mutant | *"the loader's **keyed-meaning-param** check removed"* | ✓ (R2-m8) |

**`/usr/bin/grep -n "unit param\|unit-param"` over the whole file returns nothing.** The three surviving `"three"`s near this material — `:9`, `:35`, `:806-807` — are all past-tense narration of the defect in the revision header and in §5's own history sentence. **The finding is closed at every site, and there is no site left that a writer could build three params from.**

### R2-M2 — INVARIANT 5's mutant, and the three things I checked

- **§6 and §11 agree.** `:869-875` now reads: *"**INVARIANT 5** is that a re-run over one capture and one report is byte-identical … **Its mutant is a clock or an environment read entering a written field** … **It is NOT the ordering mutant**: a reordering that is DETERMINISTIC is invisible to a comparison of two runs, and that mutant belongs to INVARIANT 1's order test (§11)."* `:1229`'s remedy text is unchanged and says the same thing from the other end. **The two sites no longer dissent.**
- **INVARIANT 5 has a mutant that can die**, and it is new this round: `:1228` *"a clock or environment value written into any field | `a_rerun_over_one_capture_and_report_is_byte_identical`"*, added by the single `-U0` hunk `@@ -1201,0 +1228 @@`. Two runs read different clocks, the files differ, the test is red. ✓
- **The ordering mutant sits on INVARIANT 1's test.** `:1229` names `every_capture_record_produces_one_corpus_record_in_order`, and `:1136` maps that test to invariant **1**. ✓

**One conjunct of the new mutant cannot die** — see **R3-m2**. That is a wording defect inside a closed finding, not a reopening of it.

## MINOR

| # | finding | disposition | evidence |
|---|---|---|---|
| R2-m1 | the "twenty-one hunks" figure is false under its own command | **NOT APPLIED IN SUBSTANCE** — one wrong number replaced by two | `:120-122`; **R3-m1(b)** |
| R2-m2 | *"Two hunks carry no finding"* was five, and one was double-booked | **APPLIED** | `/usr/bin/grep -n "carry no finding\|unrowed"` returns only §0.1a's own row `:129`; the sentence is gone |
| R2-m3 | *"passed by no reviewer"* was false about WP-2.0-M | **APPLIED**, and I verified the replacement | `:1366-1369`; verification below |
| R2-m4 | *"the one test in this package that runs a binary"* was false | **APPLIED**, and I verified the replacement | `:1281-1286`; verification below |
| R2-m5 | INVARIANT 6 did not name two refusals its tests pin | **APPLIED**, with one word changed from every sibling site | `:1097`; **R3-m3** |
| R2-m6 | §8 item 1 stated one claim twice | **APPLIED** | `/usr/bin/grep -n "recall"` returns four lines: `:133` (the change row), `:951` (the single statement), `:961` (a different claim about revision 1's referent), `:992` (the WHY-a-rule paragraph's own use). The duplicate at old `:929-930` is deleted |
| R2-m7 | *"which is why that book was sized as it was"* was an inference the passage does not carry | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:942-945`; **R3-m5** |
| R2-m8 | the mutant said *"unit-param"* | **APPLIED** | `:1205` |
| R2-m9 | the §2.10 register row is the one section-anchored one | **MOOT / recorded** | `:1186` still reads `§2.10`, which is the document saying so; the row was recorded as an exception, not a gap |
| **the sibling's finding** | §11 carried a claim WP-2.0-M had already corrected | **APPLIED**, and the sibling reproduces | `:1245-1256`; verification below |

**R2-m3, verified against the sibling at HEAD.** `:1366-1369` now says *"**They are no longer unadjudicated** — WP-2.0-M revision 5 PASSED its scoped re-review, so under D-547 they are frozen"*. `wp20m_design.md:6-8` at HEAD reads *"revision 5 (`41a52f0`) **PASSED** — 0 BLOCKING, 0 MAJOR, 9 MINOR. **The design is therefore under D-547's freeze in full**"*. ✓ **TRUE.** The narrower dependency the sentence keeps also checks out: §4.2 is *"The record's grammar"* (`wp20m_design.md:502`), §5 is *"WHAT COUNTS AS THE SAME CAPTURE"* (`:587`), and §13(a) is the corpus manifest (`:1057`), so the three targets are the right ones for §5's re-derivation, §9's rows 2-3 and §12.1. And *"a paragraph passed twice"* for the normalisation is supportable: `wp20m_design.md:114` rows revision 2 §4 as passed at the rev-2 review and **LIFTED VERBATIM**, and revision 5 then passed the whole document.

**R2-m4, verified against the tree.** `:1281-1286` now says *"It is not the only test here that runs a binary — `the_sprt_reports_per_game_node_counts_survive_the_totals_of_split` drives the SPRT path, whose `compute.add` sits inside `exchange::ask` behind a spawned `Channel`"*. `crates/pistol-arena/src/exchange.rs:76-79` is `if let Some(totals) = totals_of(&line) { compute.add(totals.0, totals.1, totals.2); continue; }` ✓, and `crates/pistol-arena/src/channel.rs:46-52` is `pub struct Channel { label, child: Child, stdin, lines, stderr }` ✓, whose only constructor `Channel::start` runs `Command::new(binary)` at `:56-57`. **The claim is now existential where it was universal, and the existential witness is real.** The false universal is gone.

**The sibling's cross-document finding, verified.** `:1245-1256` restates the solver universal as **AMBIGUOUS** and quotes the three configs' own disclaimers. All three citations are new this revision and all three reproduce: `configs/bench_wp18c_solver_on.toml:15` = *"NOT an SPRT arm and never a committed engine config"* ✓; `configs/gate_staged_solver_v0.toml:8` = *"it is never an SPRT arm and never the committed config"* ✓; `configs/play_staged_solver_v0.toml:8` = *"THIS IS NOT A DEPLOYMENT CONFIG"* ✓. `/usr/bin/grep -rln "on_search_path = true" configs/` returns **exactly those three files**, so *"the three files under `configs/` that arm the solver"* is complete ✓. And it matches `wp20m_design.md:14-27`, which reaches the same reading in the same words. **The reversal now stands at both sites.**

---

# PART 2 — THE §0.1a AUDIT, AND NEW DEFECTS

## The §0.1a audit — done independently

**Completeness of rows: PASS.** 12 rows at `:126-137` — R2-M1, R2-M2, R2-m1…R2-m9, plus the sibling's finding — one per revision-5 finding (2 MAJOR + 9 MINOR = 11) plus one the author added. No finding is missing and no row is invented.

**Every phrase greps: PASS, but see the qualification below.** I ran `/usr/bin/grep -c -F` for all ten quoted phrases and then located each by line number. Every phrase stands somewhere in the document.

**Every phrase discharges its finding: NINE of ten do.** Row by row:

| row | the phrase's site in the body | discharges? |
|---|---|---|
| R2-M1 | `:780-781`, §5's header enumeration | **YES** — the site the finding named, and the whole sweep above |
| R2-M2 | `:871`, `:873`, §6 | **YES** — §6 and §11 agree, and INVARIANT 5 gains a killable mutant |
| **R2-m1** | `:120-122`, §0.1a's own hunk sentence | **NO — the replacement figures are false under the command the same sentence names.** See **R3-m1(b)** |
| R2-m2 | — (deletion) | **YES** — verified by grep, the sentence is gone |
| R2-m3 | `:1366-1367`, §12.3 | **YES** — and I verified it against `wp20m_design.md` at HEAD |
| R2-m4 | `:1281`, §11 | **YES** — and I verified the witness in `exchange.rs`/`channel.rs` |
| R2-m5 | `:1097`, INVARIANT 6 | **YES** for the enumeration; one word dissents from four sibling sites (**R3-m3**) |
| R2-m6 | — (deletion) | **YES** |
| R2-m7 | `:942`, §8 item 1 | **YES** for the flagged inference; a new one replaces it (**R3-m5**) |
| R2-m8 | `:1205`, the mutant table | **YES** |
| R2-m9 | `:1186` | **YES** (recorded, no change) |
| sibling | `:1245`, §11 | **YES** — three new citations, all verified |

**The qualification, and it is the round's instrument finding.** `:114-118` says *"Every NOW cell below quotes the phrase that now stands in the document, and every one was confirmed with `/usr/bin/grep -c` … A row whose phrase does not grep is a defect, and it is checkable by a reader in one command rather than by trusting a tick."* **A row's phrase always greps, because the row itself contains it.** The check cannot return zero. For four of the ten phrases the row is the *only* line that matches, because the body's copy is hard-wrapped across a newline:

| phrase | lines `/usr/bin/grep -n -F` matches |
|---|---|
| *"the four keyed meaning params below"* | **`:126` only** — §5's copy breaks after *"the four"* at `:780` |
| *"Its mutant is a clock or an environment read entering a written field"* | **`:127` only** — §6's copy breaks after *"read"* at `:871` |
| *"They are no longer unadjudicated"* | **`:130` only** — §12.3's copy breaks after *"longer"* at `:1366` |
| *"The passage's own sequel is that the book in question ran out"* | **`:134` only** — §8's copy breaks after *"sequel is"* at `:942` |
| the other six | row **and** body ✓ |

So a reader running the document's own command on R2-M1's phrase — the row the whole rebuild exists for — gets `1` whether or not §5 was ever fixed. **This is the third consecutive round in which §0.1a's guarantee is weaker than its sentence**: revision 4's rows reported work rather than checking it, revision 5's hunk column certified the finding rather than the site, and revision 6's grep certifies the row rather than the body. **The rows are nonetheless all true this time**, which I established by opening each site, not by running the check.

## MINOR

### R3-m1 — §0.1a's new rule cannot fail, and its own R2-m1 row is false under the command it names

**(a) The rule is vacuous**, for the reason set out above: the phrase is in the row.

**(b) The hunk figures are wrong, again.** `:120-122`:

> `git diff ea01fea -- docs/experiments/wp20s_design.md` carries **9 hunks** with default context and **10** with `-U0`; both figures are given because revision 5 gave a third that matched neither.

I ran the literal command at the clean tree, HEAD = `d4b1915`:

| command | hunks |
|---|---|
| `git diff ea01fea -- docs/experiments/wp20s_design.md \| /usr/bin/grep -c "^@@"` | **11** |
| `git diff -U0 ea01fea -- docs/experiments/wp20s_design.md \| /usr/bin/grep -c "^@@"` | **16** |

Neither is 9 and neither is 10. The figures **are** 11 − 2 and 16 − 6, the counts less the revision header and §0.1a's own self-rewrite (2 hunks with context, 6 at `-U0`) — the same defensible-but-unstated arithmetic that made revision 5's *"twenty-one"* wrong. **R2-m1 asked for the sentence to stop carrying a number its own command falsifies; the remedy replaced one such number with two.** Third round running.

**Class**: the document's own bookkeeping. It changes no reading of any rule and constrains no implementer, which is why it is MINOR and not a FAIL.

**FIX**: either state the counts the command actually yields, or say which hunks are excluded — *"9 of its 11 hunks, and 10 of its 16 at `-U0`, are outside this section and the revision header"*.

### R3-m2 — INVARIANT 5's new mutant is half-unkillable: an environment read is identical across two runs

`:871-873`, §6, and `:1228`, the mutant table, register the same mutation:

> **Its mutant is a clock or an environment read entering a written field**, which two runs disagree about and which the byte-identity comparison therefore kills.

**The clock half dies.** Two runs read different instants, the two corpus files differ, `a_rerun_over_one_capture_and_report_is_byte_identical` is red. ✓

**The environment half does not.** `std::env::var`, the hostname, the working directory, the user, the process id — every one of them is the **same value in both runs of a re-run comparison**, in one process and in two. A mutation that writes one into a field produces two byte-identical corpus files, the test passes, and the mutant survives. §6's *"which two runs disagree about"* is asserted of both conjuncts and is false of one.

This is exactly the class §11's own closing sentence names at `:1235-1236`: *"A mutant that cannot die and a test that cannot fail are the same defect from two ends, and this arc has registered both before."* It is registered in the round whose purpose was to give INVARIANT 5 a mutant that can die.

**Why MINOR and not MAJOR.** INVARIANT 5 **does** now have a mutant that dies, which is what R2-M2 asked for; the surviving conjunct is an over-broad word inside it, not a missing mutation. An implementer running the suite finds the environment mutation surviving and drops it — a mechanical outcome, not a design decision, and no output of the package changes either way. There is also a charitable reading under which *"environment"* means *"anything outside the two input files"*, of which the clock is the killable exemplar — but D-424's own test cuts against it here, because the two readings license **different** conclusions about whether a registered mutation dies, so the word is doing work and one of its readings is wrong.

**FIX**: delete *"or an environment read"* at `:871` and *"or environment value"* at `:1228`. One word at two sites, and INVARIANT 5's mutant is unambiguous.

### R3-m3 — INVARIANT 6 says "out-of-range mate value" where four other sites say "negative"

R2-m5's remedy widened INVARIANT 6's enumeration to *"**empty field, out-of-range mate value or key-column shape**"* (`:1097`). The rule everywhere else is **negative**:

| site | text |
|---|---|
| `:377-378` §2.3 | *"a `mate_in`/`mated_in` value is a turn count and is **never negative**. The loader refuses a **negative** value under either mate kind by name"* |
| `:816` §5's loader | *"a **negative** `score_value` under either mate kind"* |
| `:1066` §9 row 9 | *"a `mate_in` or `mated_in` value is **negative**"* |
| `:1156` the test | `a_negative_mate_value_is_refused_by_name` |
| `:1097` INVARIANT 6 | *"**out-of-range** mate value"* |

`ScoreKind::MateIn(u16)`/`MatedIn(u16)` (`crates/pistol-search/src/score.rs:53-61`, cited at `:375-376`) has an upper bound too, so *"out-of-range"* is a strictly wider rule than *"negative"*: an implementer reading INVARIANT 6 could add a ceiling check that no other site asks for and that no registered test or mutant covers. **A rule stated twice and differently — the class this document has been failed for in three consecutive rounds — narrowed to one word.**

**Why MINOR.** The four concrete sites agree, the test's name fixes the behaviour, and a ceiling check on a `u16` parse is a no-op in practice. Nothing the package writes is wrong under either reading.

**FIX**: `:1097` → *"empty field, negative mate value or key-column shape"*.

### R3-m4 — the read-side TAB refusal contradicts WP-2.0-M's frozen §4.2, and the reconciliation is never stated

This document draws the read/write TAB line correctly for the corpus it **writes**: `:755` registers the TAB refusal as a write-side one, `:1051` says §5 owns *"its one write-side TAB refusal"*, and `:1214`'s mutant says *"the **write-side** TAB check removed — the corpus this transform writes"*. That is exactly WP-2.0-M's own reasoning.

It does not draw the same line for the capture it **reads**:

- `:1061` §9 row 4 — *"a capture record's field count is wrong, **or a field carries a TAB**, or any field is empty"*, under a preamble (`:1049-1050`) that says *"This table enumerates every refusal on the READ side"*.
- `:1175` — `a_capture_record_with_a_tab_in_a_field_is_refused_by_name`, registered separately from `a_capture_record_with_the_wrong_field_count_is_refused_by_name` at `:1178`.
- `:1215` — the mutant *"the **read-side** TAB refusal removed — the capture this transform reads"*.

`wp20m_design.md:537-541`, in §4.2 — a section passed at the rev-2 review and frozen in full under D-547 since revision 5 — says the opposite in terms:

> **THIS IS A WRITE-SIDE REFUSAL AND IT IS PINNED AS ONE** — INVARIANT 6, not INVARIANT 11. **On READ a TAB inside a field is indistinguishable from an extra field, so the loader's own guard is the arity check and nothing finer, and saying otherwise would register a loader behaviour no loader can have.**

The sibling is right on the mechanism: the capture is five TAB-separated fields, so a TAB inside one makes six, and a reader cannot tell the two apart. This document never reconciles the two positions, and it is the same cross-document class §0.1a's last row (`:139-144`) records the arc as having just produced.

**Why MINOR and not MAJOR.** A reading is available that makes both true, and every competent implementer lands on it: §9 groups the three triggers into **one row**, so one named refusal covers all three, the arity check is what fires, and the registered mutant *"the read-side TAB refusal removed"* is that arity check — which kills both tests. Under that reading nothing is unimplementable, the two `..._is_refused_by_name` tests are two inputs to one refusal, and no output differs. But the reading is inferred rather than stated, and the two documents disagree on their faces about what a loader can do.

**FIX**: one clause at `:1061` — *"or a field carries a TAB (which the arity check is what sees, WP-2.0-M §4.2)"* — and the same qualifier on `:1215`'s mutant.

### R3-m5 — R2-m7's replacement swaps one inference the sources do not carry for another

`:942-945`, new this round:

> (The passage's own sequel is that **the book in question ran out** before either figure could be reached, **which is why it is retired for governed use, D-518**; the sizing of its successor followed a different registered rule and is not this citation's subject.)

The flagged clause (*"which is why that book was sized as it was"*) is gone ✓, and the scope-limiting second half is correct and useful. Two limbs of the replacement are still beyond the sources:

1. **The book that ran out is not the artifact D-518 retires.** `configs/random_openings_v1.toml:45` reads *"**2000 AND NOT 500**, and the number is a power calculation rather than a taste"* — this config sets the book at 2000; *"the book ran out at 500"* (`:50`) describes the 500-opening state it replaced. D-518 retires **`random_openings_v1.txt`**, the 2000-opening artifact, and its stated ground is *"`book_v2` IS GENERATED, COMMITTED AND SIZED FROM A MEASUREMENT … This is D-505's and D-513's flip condition firing"* — not a 500-pair shortfall. *"which is why it is retired"* is a causal attribution D-518 does not make.
2. **"before either figure could be reached" is not right of the alpha.** At 500 pairs the measured figures are *"alpha of 0.030 and a power of 0.569"* (`:47-48`): the alpha is **at or below** nominal 0.05 and only the power falls short, which is why the config's own next sentence is *"A declared beta the design cannot deliver is worse than a smaller claim"*.

**Why MINOR.** The parenthesis is an aside inside item 1, and the conclusion it serves — *"a nominal pair is a registration and not a guarantee, and the gap closes with n"* — is the passage's own and is untouched by either limb. **Nothing in the census rule reads it.** It is recorded because it is a claim about a governing ADR, which is the D-545 class, and because it is the second consecutive round in which this one parenthesis has carried an inference its citation does not.

**FIX**: *"(The passage's own sequel is that the 500-opening book it replaced ran out before the LLR could reach a boundary — the shortfall is in the power, not the level — which is why the size was raised.)"*

### R3-m6 — the `-` sentinel's interaction with the key-shape checks is left to inference

§2.10 writes `moves`, `key_seq` and `key_full` as a single `-` when the prefix is empty (`:606-609`). §5's loader refuses *"a `key_seq` whose elements are not turn tokens, or a `key_full` whose elements are not `q,r:p1` / `q,r:p2` pairs"* (`:818-819`), and INVARIANT 6 repeats it (`:1098-1100`). **`-` is neither a turn token nor a `q,r:p1` pair, and the document never says the shape check exempts the sentinel.**

The answer is forced rather than open: `a_corpus_file_round_trips_through_its_own_loader_field_by_field` plus `the_turn_zero_record_writes_a_dash_for_its_three_empty_columns` together require the loader to accept `-` in those columns, and *"whose **elements** are not …"* reads naturally as vacuous over a field with no elements. Recorded because §2.10's own argument for choosing `-` is that it spares the loader a per-column conditional (`:610-613`), and the shape checks added in revision 5 reintroduce exactly one — which is a sentence §2.10 would want to own.

**FIX**: one clause at `:818` — *"…are not turn tokens (the `-` sentinel aside, §2.10)"*.

---

## Full closure over the thirteen invariants, fifty-one tests and thirty-eight mutants

**(a) Invariants with no test**: **one, declared** — INVARIANT 2, at `:1118-1128`, with the reason (*"no in-process Rust test observes the absence of a `Command::new` or an `Instant::now` on a path it does not take"*) and the diff named as its evidence. I extracted the register's "pins" column and counted: 1×1, 3×7, 4×3, 5×1, 6×12, 7×11, 8×4, 9×4, 10×3, 11×1, 12×2, 13×1, §2.10×1 = **51**, and every invariant except 2 appears. ✓ **Closed.**

**(b) Tests pinning nothing / mis-mapped**: **none.** Revision 5's two carried items are closed by R2-m5's widening: `a_corpus_record_with_an_empty_field_is_refused_by_name` → INVARIANT 6's *"empty field"* ✓ and `a_negative_mate_value_is_refused_by_name` → its *"out-of-range mate value"* ✓ (with R3-m3's wording note). `a_corpus_missing_its_opening_turns_param_is_refused_by_name` → *"header params"* ✓; `a_key_seq_field_…` and `a_key_full_field_…` → *"key-column shape"*, spelled out in the invariant's own tail ✓. A `uniq -d` over the 51 rows returns nothing — **no duplicate test names**.

**(c) Rules with neither test nor mutant**: **none.**

**(d) Mutants that cannot die**: **one conjunct, R3-m2** — the environment half of `:1228`. Otherwise closed. I extracted every identifier from the 38-row mutant table and `comm`ed it against the 51-row register: **every test name a mutant points at is registered**, and the residue of the comm is field and function names (`a_is_p1`, `book`, `canonical_form`, `canonical_sequence`, `capture_sha256`, `cp`, `depth_meaning`, `end`, `key_full`, `key_seq`, `mate`, `nodes`, `result`, `score_kind`, `search_nodes`, `time`, `to_move`, `totals_of`) — no invented test. Every invariant with a test now also has a mutant: **INVARIANT 5's gap, which was revision 5's R2-M2, is closed at `:1228`.** The write-side TAB mutant remains the one I cannot argue from the text alone, carried unraised from revisions 4 and 5 for their reason — a validating write seam is the ordinary shape.

**(e) Tests that can fail on correct code**: **none.** I re-attacked R-M4's fixture conditions at `:1263-1272` independently and they hold — see the attack section. **Tests that pass vacuously**: **none**; the one candidate class (`the_transform_spawns_no_process_and_reads_no_clock`) is declared removed with its reason at `:1234-1236`.

**(f) Rules stated twice and differently** — the sweep, aimed at this round's five remedies plus everything with history:

| where | verdict |
|---|---|
| **§5's header params** | **clean** — six sites, all four, no `unit param` residue (R2-M1 closed) |
| **INVARIANT 5's mutant** | **clean** — §6 and §11 agree (R2-M2 closed); one over-broad conjunct, R3-m2 |
| **the mate-value refusal** | **NOT clean — R3-m3** (four sites "negative", INVARIANT 6 "out-of-range") |
| **the read-side TAB refusal** | **NOT clean across documents — R3-m4** (§9 row 4 vs `wp20m_design.md` §4.2, frozen) |
| §8's test form | **clean** — `/usr/bin/grep -n "two-proportion\|one-sample\|ONE-SAMPLE"` returns seven lines: `:43-45` and `:987` narrate the closed defect in the past tense; **every prescriptive use (`:976`, `:980`, `:984`) is one-sample** ✓ |
| §8's measured recalls | **clean** — R2-m6's duplicate deleted; `:951` states it once and `:992` makes a different point |
| `key_full`'s scope | **clean** — `:291`, `:906-907`, `:1022` all say "any/an ongoing position"; revision 2's *"and only on them"* is gone |
| INVARIANT 4's strength | **clean** — `:674-676` and `:726-729` agree, and neither claims byte-identity |
| §9 vs §5's enumerations | **clean** — the read/write split holds and the partition is complete row by row |
| the book boundary | **clean** — prose `k <= opening_turns` (`:573`), the two mutants (`:1223-1224`) and the three tests all agree |
| §12.1's manifest row | **clean** — six fields in §12.1 (`:1304-1306`) and six in INVARIANT 11 (`:1110-1112`), the same six |
| §1 vs residue 2(b) | **clean** — *"a FOURTH arm and a THIRD pass"* (`:202`) and *"a third pass and a fourth arm"* (`:1401`) |
| §2.10's `-` sentinel | **clean as a rule**, unstated against the shape checks — R3-m6 |
| the solver universal | **clean, and now agrees with the sibling** — `:1245` and `wp20m_design.md:24-25` |

**(g) The false-universal sweep** (`every`, `no`, `none`, `only`, `always`, `never`, `all`, `any`, `each`). I stripped inline-code spans and inspected every surviving instance across 1406 lines. **The two false ones revision 5 found are both gone** — `:1246`'s *"the one test … that runs a binary"* is now existential (R2-m4) and `:1327`'s *"passed by no reviewer"* is corrected (R2-m3). **I re-verified every remaining universal with a code or document referent against the tree, not against the previous review:**

- `:932-934` *"**Every** committed arena experiment config in this repository carries `alpha = 0.05` and `beta = 0.05`"* — `/usr/bin/grep -rn "^alpha = \|^beta = " configs/` gives 13 `alpha = 0.05` and 13 `beta = 0.05` and **nothing else** ✓.
- `:642-645` *"it is the **only** place **in `pistol-arena`** that tells `info totals …` from `info …`"* — `git grep -n "TOTALS" -- crates/pistol-arena/src/` returns one line, `exchange.rs:173` ✓.
- `:498-499` *"A `git grep` for `Provenance` under `crates/pistol-cli/src/` names the type on **no** line at all"* — returns nothing ✓.
- `:307-308` *"**All three** are exported from `crates/pistol-core/src/lib.rs:85-90`"* — `pub use state::GameState;` (`:85`) and `pub use symmetry::{Symmetry, canonical_form, canonical_sequence};` (`:86`) ✓.
- `:1247` *"**the three** files under `configs/` that arm the solver"* — `/usr/bin/grep -rln "on_search_path = true" configs/` returns exactly those three ✓.
- `:1323` *"`docs/label_corpus_manifest.md` does not exist yet"* — `ls` says no such file ✓.
- `:1327-1329` *"WP-2.0-M's §13 states what its own rows are and **says nothing about headings**"* — `/usr/bin/grep -n "heading" docs/experiments/wp20m_design.md` returns nothing ✓.
- `:775-777` *"A colon appears in **no** turn token, **no** cell token and **no** score spelling"* ✓ (carried, re-verified against `coord.rs:136-141` and `report.rs:145-158`).
- `:18-19` / `:114-116` *"**every** phrase was confirmed by `/usr/bin/grep -c`"* — **true only vacuously**, R3-m1(a). This is the one universal in the file that does not do what it says, and it is about the document, not the package.

**Five consecutive rounds carried a false universal about the tree or a sibling document. This round carries none.**

**(h) The D-483 numeral sweep.** Stripping inline code, D-numbers, and §/WP/INVARIANT/revision/item/row/rule/gate indices leaves: section, column and refusal-row indices; **three code facts** — *"twelve"* images (`symmetry.rs:157-165`), *"six"* solver fields (`report.rs:62-81`), *"thirty-two"* / *"32"* hex digits (`zobrist.rs:70-76`) — all pinned to citations that reproduce; the `0.05` / `0.95` pair at `:936`, argued as a convention adopted from a committed config; **one measured figure from the closed arc**, *"the arc's own **fourteen** firings"* (`:979`), used to argue the two-proportion form unsatisfiable and **not consumed by the rule**; and, new this round, the **9 / 10 hunk counts** at `:120-121`, which are a report about this document's own diff rather than a number any rule reads — and are wrong (R3-m1(b)). **No measured number enters the census rule**, and the self-claim at `:948-951` — *"the level and the power are the only numbers this document FIXES, and they are conventions adopted from a committed config rather than measurements"* — survives: I tried to falsify it with the hunk counts and could not, because the document reports them rather than fixing them and no rule consumes them. **The D-483 position is sound for the second consecutive round.**

---

## The citation check, and AUTHOR DEBT

`python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/labels.rs --proposes crates/pistol-arena/src/capture.rs --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md docs/experiments/wp20s_design.md` → **122 citations checked, 0 unreproduced.** Green before the review, per D-546's condition.

**The citation TARGET set moved, and I hand-verified every change.** A `comm` over both revisions' extracted `path:line` multisets:

| | |
|---|---|
| **added** | `configs/bench_wp18c_solver_on.toml:15`, `configs/gate_staged_solver_v0.toml:8`, `configs/play_staged_solver_v0.toml:8`, `crates/pistol-arena/src/channel.rs:46-52`, and a second occurrence of `crates/pistol-arena/src/exchange.rs:76-79` |
| **removed** | `configs/bench_wp18c_solver_on.toml:45`, `configs/gate_staged_solver_v0.toml:47`, `configs/gate_v0.toml:94`, `configs/play_staged_solver_v0.toml:75` |

**All five new citations hand-verified against the tree** (quoted above under the sibling's finding and R2-m4). The three config citations moved from the `[solver]` blocks to the files' own disclaimer headers, which is the right move: the new claim is about what each file **says it is**, and the new lines are where each says it.

**I also re-verified every cross-document claim, which the checker cannot see:**

- **WP-2.0-M's freeze state** (R2-m3's new claim) — `wp20m_design.md:6-8` ✓.
- **WP-2.0-M's §4.2, §5 and §13(a)** as the dependency targets — `:502`, `:587`, `:1057` ✓, and §13(a)'s row carries the six fields §12.1 attributes to it ✓.
- **§12.3's `totals_of` lean** — `wp20m_design.md:759` (*"The visibility change is for WP-2.0-S, so that package adds fields to one parser"*) and `:793-794` (*"the guard WP-2.0-S inherits when it adds `score` and `pv` as non-fatal `Option`s (D-542)"*) **both still reproduce verbatim at HEAD** ✓, so the sentence's assignment — a prose-and-ADR correction WP-2.0-M owes — is still right at revision 7.
- **§1's `usage.rs` claim** — `wp20m_design.md:296` names *"The extraction target is `crates/pistol-arena/src/usage.rs`"* ✓.
- **§1's arena.rs mechanism citations** — `crates/pistol-arena/src/bin/arena.rs:82-100` is the mode match with two arms plus help and the fallback ✓; `:103` is `let claimed = outpath::claim(&out_path)` ✓, before the dispatch at `:104-107` ✓; `:94-99` is the `_ =>` refusal ✓; `crates/pistol-arena/src/lib.rs:47-69` is the `pub mod` list ✓.

**AUTHOR DEBT the checker could have caught: NONE. Six consecutive rounds with zero.**

**The debt the checker cannot catch** is R3-m1(b) (a number about a `git` command), R3-m5 (an attribution beyond a true quotation and beyond D-518), R3-m3 (one word wider than four sibling sites), R3-m4 (a disagreement with a frozen sibling section), R3-m2 (an over-broad conjunct in a mutant), and R3-m6 (an unstated interaction). **Not one of this round's six findings is a `path:line` that does not reproduce, and not one is a claim about the tree.** The failure class has moved entirely into wording and self-description.

---

# PART 3 — THE VERDICT

## **PASS** — 0 BLOCKING · 0 MAJOR · 6 MINOR

Prior-round disposition: **2 of 2 MAJOR applied; 7 of 9 MINOR applied, 1 applied with a new defect, 1 not applied in substance. Zero not applied at all** — against revision 4's five.

## "Could an implementer build from this without deciding something the design should have decided?"

**YES.** There is no item on the list.

I built the list the way the previous two reviewers did — walking every decision the dispatch, D-544, D-537 and the matrix put in this package and asking of each whether the document settles it — and every one is settled: the sixteen columns and their order; the three keys and exactly what each folds, each pinned to pistol-core's own doc; the `q,r:p1` spelling and its non-collision; the `-` sentinel and the no-empty-field rule; the parser split, its return type and the two-token score read; the node pair and its gate-off fallback; the all-six-or-none solver refusal; `depth_turns`'s two meanings, their discriminator, and the **four** keyed header params that carry them; the `to_move` / `result` / `end` / `book` / `score_kind` token sets; the outcome relation and its seat-blindness; the `RecordedGame` widening at `result` and only `result`; the no-dedup and no-seed policies with their grounds; the book boundary in prose, in three tests and in two mutants; the twelve refusal rows and the two-sided enumeration split; §8's complete one-sample rule with all five inputs fixed and its `key_full` denominator; INVARIANT 6's three key shapes; the colour test's two fixture conditions; the stub-driven outcome fixture and its hard-rule-8 route; the manifest row's six fields and the two-table file; the throughput shape's own half; and §12.3's four leans with the two ADR acts the landing owes.

**The six MINORs, tested against the same question, each come back "no".** R3-m1 and R3-m5 are claims about a `git` command and about a config's history that no rule reads. R3-m3 is one word where four other sites fix the behaviour and a test name pins it. R3-m2 leaves a mutation that will survive its own suite run and be dropped, with INVARIANT 5's killable mutant intact beside it. R3-m4 has a stated grouping in §9 that reconciles it, and both readings ship the same arity check. R3-m6 is forced by two registered tests. **An implementer building `labels.rs`, the loader, the `fields_of` split and the manifest row from this document writes the same code under every one of them.**

## The strongest attack that did not land

**I set out to break §0.1a a fourth time, and I could not.** The instrument's *rule* is vacuous (R3-m1a) — a row's phrase always greps, and for four rows the row is the only match — so I expected the same failure the last three rounds produced: a row asserting a fix at a site the fix never reached. **I checked all twelve rows by opening the site rather than by running the check, and every substantive remedy is genuinely in the file, at the site named, doing what the row says.** R2-M1's is at `:780-781`, the exact line revision 5 failed the document over, and I confirmed it by an independent sweep of all 44 `three`/`four` occurrences and all 17 `param` occurrences rather than by reading §5. R2-M2's is at `:871-875` with a matching mutant at `:1228`. R2-m3's and R2-m4's replacements are true against the sibling at HEAD and against `exchange.rs`/`channel.rs`. **The one row that does not discharge is R2-m1's, and what it gets wrong is a hunk count.** After three rounds of a guard that reported rather than checked, the guard is still not the reason — but the work under it is finally right.

**A second attack failed.** I tried to re-open R-M4's fixture conditions at `:1263-1272`, on the theory that a two-clause prose condition cannot be exactly the negation of a collision. It is. `transform` maps `|&(cell, player)| (symmetry.apply(cell), player)` and then sorts (`crates/pistol-core/src/symmetry.rs:148-155`, read at HEAD) — **colour is carried through untouched** — and `canonical_form` is *"the least of its twelve images"* under an origin-fixing group (`:157-165`), so `canonical_form(A) == canonical_form(B)` **iff** some symmetry carries A's coloured stone list onto B's. The paragraph's *"are **not** symmetry images of each other"* is therefore the exact negation, and the shared-cell-set clause is separately needed for the *"`key_full` rendered as bare cells"* mutant to die. **Two conditions, both necessary, jointly satisfiable, and I could not construct a legal pair that meets them and still makes the test red.**

**A third failed.** I tried to show R3-m4 was MAJOR — that §9's read-side TAB refusal is a loader behaviour no loader can have, so its test is red on correct code or its mutant cannot die. It is not, because §9 row 4 groups three triggers under **one** refusal and the arity check is what fires; both tests are then two inputs to one refusal and the mutant dies at either. The finding survives only as the unstated reconciliation with a frozen sibling section, which is why it is MINOR.

**A fourth failed.** I tried to falsify the numeral self-claim at `:948-951` a sixth time, using the two hunk counts revision 6 introduced. It holds: the document *reports* them about its own diff and no rule consumes them, so they are not numbers it FIXES — the same distinction the sentence itself draws for 0.05/0.95.

## What I could not settle by reading, and the run that would

1. **Whether `a_corpus_field_carrying_a_tab_refuses_the_run_by_name` is constructible.** No corpus field the transform computes can carry a TAB on the shipped path, so the test must sit at a validating `write_record` seam the design does not name. **The run**: write it in a `git worktree add --detach` and see whether it needs an entry point the design has not specified. **Refused by the hard constraint.** Carried unraised from revisions 4 and 5 for their reason — a validating write seam is the ordinary shape.
2. **Whether the environment half of `:1228`'s mutant survives in practice as well as in argument.** I argue it from the semantics of a two-run byte comparison; the settling run is to write `a_rerun_over_one_capture_and_report_is_byte_identical`, mutate a field to `std::env::var("USER")`, and watch it stay green. **Refused.** This is R3-m2's reproducer and it is a five-line job.
3. **Whether the `fields_of`/`totals_of` split compiles and leaves the SPRT report's node counts intact.** Three identical expressions over the same `Vec<&str>` with elided lifetimes; I expect yes. **The run**: `cargo test --workspace --locked` plus gate 15. **Refused.**
4. **Whether the one-sample rule yields a finite, sensible `n`** at the arc's incumbent against its trigger-rich bound, level 0.05, power 0.95. **The run**: a `scipy`/`statsmodels` power calculation. **D-483 forbids the answer entering the design**, which is the design's own position and is correct.
5. **Whether gate 17's soft cap holds once the fourth arm, `labels.rs`, the loader and `usage.rs` land.** **The run**: `tools/file_justification_check.sh` at the post-implementation revision. Not run.

---

## One paragraph for the architect

**This document passes, and it passes on the design rather than on patience.** Both of revision 5's MAJORs are closed at every site rather than at the site the change table names — I swept all 44 `three`/`four` occurrences and all 17 `param` occurrences to settle `depth_meaning`, and INVARIANT 5 now has a mutant that dies, with the ordering mutant on INVARIANT 1's own test and §6 and §11 saying the same thing. Two false claims about other documents are replaced by claims I verified against `wp20m_design.md` at HEAD and against `exchange.rs`/`channel.rs`; the citation set reaches 122 with zero author debt for the sixth consecutive round; and for the first time in six rounds **there is no false universal about the tree or about a sibling anywhere in the file**. **The six remaining findings are all in one class and it is not the design's**: two are numbers and attributions in the document's own bookkeeping (R3-m1, R3-m5), one is a single word wider than the four sites that fix the same rule (R3-m3), one is an over-broad conjunct in an otherwise-killable mutant (R3-m2), one is an unstated reconciliation with a frozen sibling paragraph that both readings implement identically (R3-m4), and one is an interaction two registered tests already force (R3-m6). Every one of them is a one-line edit and none of them changes what an implementer builds — which is exactly the standard the dispatch set for a PASS. **The one thing worth carrying forward is the guard, not the document**: §0.1a's third instrument in three rounds is the first that *cannot* return a negative, because a row's quoted phrase is satisfied by the row itself, and for four of the twelve rows — R2-M1's included — the row is the only line the phrase matches, since the body's copy is hard-wrapped. The rows are all true this time, and I established that by opening every site. **A fourth revision of the same table would not close anything the fixes did not already close; if anything replaces it, let it be a rule that names a `file:line` outside §0.1a, which is the one form of the check that can fail.**
