# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 7.** Carved from `docs/experiments/wp15b_design.md` §8, §9, §11.6 and §12
item 1 at `6feb40a` (revision 7, never reviewed, CLOSED by D-309) under the
restructure selected as option D by D-310. The carve's section-to-owner map is
`docs/experiments/section_owner_table.md`. The superseded document is not
in the tree: it is retrievable at `6feb40a` and nowhere else.

**THE CARVE FOUND BOTH OF THIS UNIT'S SELECTIONS OPEN AND DID NOT MAKE EITHER OF THEM
SO. AT u-rev 7 BOTH HAVE A SELECTION — M3's AT u-rev 6, M4's AXIS A AT u-rev 7 — AND
NEITHER IS BUILT.**

- **M3 (the soundness instrument) — SELECTED. THE OPTION IS S-M, AND IT IS NOT S-E.**
  `docs/experiments/matrix_M3_soundness_instrument_rev2.md` (revision 2, thirteen rows,
  authored `d48824f`) was attacked by a fresh-context DECISION-RED-TEAM at
  `docs/experiments/matrix_M3_REDTEAM_round2.md` (`809b5db`), and the selection landed
  at **`af8082a`**; the record is `docs/experiments/matrix_M3_selection.md` and the ADR
  line is **D-323**. **S-M is per-node EQUALITY of the emitted set against the LANDED
  referent R1** (`crates/pistol-solver/tests/common/reference.rs`), **REUSED by a
  `#[path]` include and NOT rewritten**. **S-E FELL in round 1 and is superseded**, and
  the matrix's own recommendation S-K is dead too. The gate ships **MARKED
  DEPENDS-OPEN-THEORY** under **D-321** — the `DEF-T` convention question is OPEN and
  the calculus is NOT amended — and carries five registered conditions. See the head of
  §8.
- **M4 (the snapshot's config seam) — AXIS A IS SELECTED AT u-rev 7. THE OPTION IS N-E,
  AND IT IS NOT THE ROW THE MATRIX RECOMMENDED (D-329).** Rounds 1 and 2 stopped
  (D-318) and round 3 stopped on the RECORDED TIE (D-324), whose cause was that **the
  field had two orthogonal axes and was being scored as one** — how the config is
  NAMED (axis A) against how many RECORDS one invocation produces (axis B). D-324's
  own stop clause authorised **a fourth round scoped to axis A alone with N-Q authored
  into it**; that round is `docs/experiments/matrix_M4_axisA_round4.md` (`7866bcf`,
  field {N-E, N-M, N-Q}, every MEASURED cell produced by a separate measurement agent
  under D-328), its attack is `docs/experiments/matrix_M4_axisA_REDTEAM.md`, and the
  selection record is `docs/experiments/matrix_M4_axisA_selection.md` (selected at
  `7e0a328`, landed at `d56a898`). **N-E is a REQUIRED `--config PATH`, NO DEFAULT,
  with a NEW WHOLE-PATH GUARD.** The matrix recommended **N-Q** and **its own red team
  killed that recommendation**; **N-M was eliminated before the ladder ran, on
  registered ground**; the ladder's first rung was **SILENT ACROSS THE WHOLE FIELD**,
  so the selection was taken at rung (b). **AXIS B IS NOT REOPENED** — D-324 records
  its flip clause as already fired toward N-K, and N-K COMPOSES with the naming
  mechanism selected here rather than rivalling it (D-329). **A SELECTION IS NOT AN
  IMPLEMENTATION:** no line of N-E is written, four conditions ride with it, and
  `configs/instrument_staged_v0.toml` still does not exist. See the head of §9.

**NEITHER SELECTION WAS THE CARVE'S TO MAKE, AND THE CARVE MADE NEITHER.** §8 and §9
below are carried as the RECORD of what was argued, and each is now read against a
selection that exists — **S-M** for §8 (D-323) and **N-E** for §9's axis A (D-329).
U4-Z's ADR lines are written on that footing. **What no selection covers, and what
U4-Z therefore still carries as OPEN:** the DIFFERENTIAL GATE's seam (D-323 records it
as a separate named decision it does not make), the four conditions riding with N-E,
and every part of IMPL for either.

**WHAT IS NOT A VERBATIM CARVE, u-rev BY u-rev — the change log, RE-READ at u-rev 6
rather than appended to.** The text is a verbatim carve apart from cross-reference
retargets and the following, each stated where it occurs.

- **u-rev 1** — **B4** (the tactical-suite gate's "all three staged tactical configs …
  not just the two gate ones", false in both halves, corrected against **U3** (u-rev 6, landed `13621d3`)
  §10's config table) and **MINOR 15** (§8.2 carried a paragraph spliced in twice
  mid-sentence; the duplicate is removed and the sentence closes).
- **u-rev 2** — **B3**, repaired by shape 2 under **D-316**: the gate letters are
  dropped, the four gates are named, and the S-E double-list dies with them. The
  two-shape comparison the architect selected from stands unedited in U4-Z beneath its
  selection record.
- **u-rev 3** — **MAJOR 8's repair**: M4's and M6's mutation witnesses rebuilt in §8.4
  as positions a legal game reaches and pinned by
  `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`.
- **u-rev 4** — **the M3 round-1 fold** at the head of §8: the matrix was authored and
  attacked and every option fell (**D-317**).
- **u-rev 5** — **the M4 round-2 fold** at the head of §9: two authored fields, two
  attacks, still nothing selected (**D-318**).
- **u-rev 6** — **the repair of `docs/experiments/wp15b_U4_REVIEW.md`**, finding by
  finding in the REVIEW STATUS block below and marked again at each site; **the M3
  SELECTION fold** (S-M, **D-323**, with D-321's mark and D-322's corrected figure);
  and **the M4 round-3 STOP fold** (**D-324**).
- **u-rev 7** — **the repair of `docs/experiments/wp15b_U4_REVIEW_urev6.md`** (1
  BLOCKING, 2 MAJOR, 4 MINOR), finding by finding in the REVIEW STATUS block below;
  **the M4 AXIS-A SELECTION fold** (**N-E**, **D-329**, with **D-330**'s amendment to
  D-328 recorded where it bears); the fold of **D-320** (B3's residual: breach
  recorded, retro-matrix WAIVED, debt paid by disclosure, flip clause carried) and of
  **D-325** (D-316's false diagnosis corrected in the log at `81180b8`). **D-320 landed
  at `0af32fb`, in the same commit as the D-321 u-rev 6 folds, and u-rev 6 missed it**;
  D-325 landed after u-rev 6. Also folded: the closure of the two
  shipped-instrument defects at `b067d47` and `a102c6a` with the REVIEW-impl PASS at
  `84ff8d7`. **U4-Z was RE-DERIVED IN FULL against the ADR log rather than patched at
  the sentences the review named**, because it is the section that has now twice
  shipped a claim falsified by a line landed in a commit this unit folds.

**The sentence this block replaces named three repairs and omitted the u-rev 4 and
u-rev 5 folds entirely — the two largest non-verbatim additions in the unit.** That is
BLOCKING 1 of the u-rev 5 review, at the surface a reader meets first. Every
**MEASURED** and **ESTIMATED** mark carried from the superseded text is the mark that
text carried; a mark added at u-rev 6 names the command that took it and pastes its
complete output.

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes. A citation of
another unit names the unit AND the u-rev cited.

**AND AT u-rev 8 IT NAMES THE REVISION AT WHICH THAT u-rev WAS CURRENT** — the form
`(u-rev N, landed <sha>)`, adopted here from **U3** (u-rev 6, landed `13621d3`), whose own
head states the ground. D-311's rule as written makes a citation a live claim about
another document's present state, which goes false the moment that document is bumped
and which nothing in this unit re-reads. **It had gone false at six sites in this unit
before u-rev 8** — five naming **U3** at u-rev 4 and one naming **U2** at u-rev 3,
both superseded — and no finding had named them, because the round that made them
stale was this same session's repair of those units (D-332, R17: this round repaired
serially, and this unit is last, which is why it is the one that can state the others
correctly). The landed-SHA form is a historical fact and cannot go stale.

**THIS UNIT DOES NOT CLAIM EVERY CITATION IS IN THE FORM, AND THE BOUND IS STATED
RATHER THAN THE UNIVERSAL.** What u-rev 8 converted is the six that named a u-rev and
named the WRONG one. Many further citations name a section without naming a u-rev at
all, and they fall in two kinds: those in live carve prose, which a later round may
convert, and **those inside blocks the record stamp at the head of §8 lists as RECORD,
which this unit may not edit at all** — a text an architect selected from is carried
unedited. **The set is DERIVED and is not enumerated here:**

```
$ grep -n '\*\*U[123]\*\*' docs/experiments/U4_soundness_instrument.md | grep -v 'landed'
```

**Converting the live ones is OPEN and is in U4-Z.** It is registered rather than
either done silently or claimed done: the sibling unit's u-rev 5 answered this same
finding by asserting a universal over exactly this kind of set, and the universal was
false at nine sites (`docs/experiments/wp15b_U3_REVIEW_urev5.md`, MAJOR C). A bounded
statement that names what it has not done is the alternative that round establishes.

Under D-331 the cited unit's CURRENT u-rev has exactly one home — that unit's own
head — and this unit points at it rather than keeping a second copy.

**REVIEW STATUS — u-rev 6 WAS REVIEWED AND FAILED; u-rev 7 IS THE REPAIR.**
`docs/experiments/wp15b_U4_REVIEW_urev6.md`, REVIEW-design, fresh context, dispatched
against the named revision **`7358a07`** (which was HEAD, tree clean, when it ran) —
**VERDICT FAIL**, **1 BLOCKING, 2 MAJOR, 4 MINOR**. **THIS u-rev — u-rev 7 — HAS NOT
BEEN REVIEWED**, and u-rev 6's review does not transfer to it: an amendment reopens
the review, however small the diff (D-311, and CLAUDE.md's own words). u-rev 7 also
folds M4's AXIS-A SELECTION (D-329), which landed after that report was written, so
it is a larger diff than the report alone would produce.

**THE FAILURE CLASS THE u-rev 6 REVIEWER NAMED IS THE SAME ONE u-rev 5's DID**, one
level in: u-rev 6's head claimed the fold-in was *"re-run across the whole unit"* and
then enumerated the sites re-read; U4-Z's B3 section was not in the enumeration, and
**D-320 — a landed line that disposes of exactly the residual U4-Z was still calling
the architect's open choice — went unread even though it landed in the same commit as
the D-321 u-rev 6 folds.** u-rev 7's answer is not a patch at the two sentences the
report names: **U4-Z is re-derived in full against `docs/decisions.md` as it now
stands**, which is how D-325 (landed after u-rev 6, correcting D-316's diagnosis in
the log) was caught in the same pass.

| Finding, `wp15b_U4_REVIEW_urev6.md` | Disposition at u-rev 7 |
|---|---|
| **BLOCKING 1** — D-320 has landed and closes B3's residual; U4-Z's residual paragraph and the OPEN list both describe the pre-D-320 state, and D-320 is named nowhere | **REPAIRED BY RE-DERIVATION, NOT BY PATCH.** U4-Z was re-read claim by claim against the ADR log. D-320 is now carried at both sites the report names and at the OPEN list: the breach is acknowledged, **the retro-matrix is WAIVED on its two stated grounds** (proportionality; and the independent verification the fresh-context REVIEW-design already supplied), the debt is recorded as **PAID by disclosure**, and **D-320's flip clause — a GATE-NAMING DEFECT surfacing — is carried in this unit, which is where such a defect would surface.** The re-derivation also caught **D-325**, which the report did not raise: it corrects D-316's false diagnosis in the log, so U4-Z's "carried to the architect" residual is DISCHARGED and is restated as closed |
| **MAJOR 2** — the record stamp at the head of §8 says everything to the end of §8.7 is record and "none of it is the state now", which is false of four blocks inside the span | **REPAIRED — THE STAMP IS SCOPED IN BOTH DIRECTIONS.** It now lists what IS record (the u-rev 1 stub and its five-row table, §8.1, §8.2's S-E body prose, the prose carried into §8.3 and §8.4) and what IS NOT (§8.2's `FOLDED AT u-rev 6` block, §8.3's four-gate table, §8.3's live **U3** §10 sentence, §8.4's `RE-READ AT u-rev 6` block and its u-rev 6 ledger marks, §8.7's wiring sentence and its fold). The prior stamp was widened to answer u-rev 5's BLOCKING 2, which found it too NARROW, and over-reached in the other direction |
| **MAJOR 3** — U4-Z's u-rev 2 SELECTION block still asserts "S-E **is** the differential gate" in present tense while its two siblings were retargeted at u-rev 6 | **REPAIRED.** The SELECTION block — which is carve prose recorded AFTER the comparison, not the selected-from text — now states that it records the u-rev 2 EXECUTION and that **the differential gate's instrument since D-323 is S-M**. *~~NO LIVE SENTENCE IN THIS UNIT IDENTIFIES THE DIFFERENTIAL GATE AS S-E.~~ **THAT UNIVERSAL IS WITHDRAWN AS FALSE AT u-rev 8** — MAJOR 1 of `wp15b_U4_REVIEW_urev7.md` found the sibling sentence at §8.7 saying exactly that, and the repair here had never reached it. §8.7 is corrected at u-rev 8. **NO UNIVERSAL REPLACES IT:** what this row records is that a named site was repaired, and whether any other site says it is a question for a reviewer's sweep, not a claim this table may assert about a body it does not own.* The two-shape comparison at the SHAPE 1 / SHAPE 2 cells is left unedited, on the unit's stated discipline, and the report agrees that is right |
| **MINOR 4** — U4-M item 1 says the amendment is carried "below"; §9.1 amendment 4 is above and carries no note at its own site | **REPAIRED AT BOTH ENDS.** The direction word is corrected to **ABOVE**, and §9.1 amendment 4 now carries a marked pointer at its own site, so a reader meeting it first does not read an unqualified MEASURED 34.5 s attributed to a script that has never existed |
| **MINOR 5** — §8.3's "this unit states no such number anywhere" is falsified by the record quotation four lines later | **REPAIRED BY BOTH REMEDIES the report offered.** The absolute claim narrows to *"states no such number as a live claim"*, AND the record quotation is replaced by a DESCRIPTION of the deleted clause, so the count is no longer restated one level up |
| **MINOR 6** — the REVIEW STATUS row for the prior reviewer's "one observation for the architect" describes a state `a0f241b` fixed | **REPAIRED, AND STATED SO IT CANNOT GO STALE AGAIN.** See the row at the foot of the u-rev 5 table below: the observation was discharged at `a0f241b`, and what is recorded now is the STANDING DUTY (a re-measurement is owed to the owner table after every u-rev bump of this unit, including this one) rather than any particular size or u-rev. **This unit still asserts no line count of itself** (rule 9) |
| **MINOR 7** — the selection SHA is cited two ways and only one of four sites disambiguates | **REPAIRED at the four sites the report named**, to the convention `matrix_M3_selection.md`'s own header uses: **"S-M SELECTED at `af8082a` (taken at `809b5db`)"**. The same convention is applied to the new M4 axis-A citations: **selected at `7e0a328`, landed at `d56a898`**. *~~Every abbreviated citation now reads …~~ **THAT UNIVERSAL IS WITHDRAWN AS FALSE AT u-rev 8** — MINOR 6 of `wp15b_U4_REVIEW_urev7.md` found §8.7:854 outside it. **THE UNIVERSAL IS NOT RE-ASSERTED AND IS NOT REPLACED BY A NARROWER ONE.** §8.7's site is corrected at u-rev 8; the set of sites is what a reader derives, not what this row certifies. **THIS IS THE SECOND UNIVERSAL IN THIS TABLE WITHDRAWN AS FALSE, AND THE THIRD IN THIS UNIT'S HISTORY** (BLOCKING 1's "across the whole unit" went at u-rev 7). A REVIEW STATUS row that certifies a property of the whole document is the shape all three had.* |

**THE PRIOR ROUND — u-rev 5's review, `docs/experiments/wp15b_U4_REVIEW.md`**,
REVIEW-design, fresh context, against **`35aab95`** — **VERDICT FAIL**, **3 BLOCKING,
3 MAJOR, 5 MINOR**; u-rev 6 was its repair. Its reviewer's summary of the failure class
is the one this unit keeps failing on: *"a change landed in one place with the claims
resting on it left un-re-read elsewhere."* The dispositions below are u-rev 6's and are
kept because each names a site.

| Finding, `wp15b_U4_REVIEW.md` | Disposition at u-rev 6 |
|---|---|
| **BLOCKING 1** — the head block, U4-A, the change log, §8's stub and §9's closing paragraph still describe the u-rev 1 state; six named sites | **REPAIRED at all six**, and the fold-in was re-run at the sites enumerated below. *~~"across the whole unit"~~ — **that claim is WITHDRAWN AS FALSE at u-rev 7.** The enumeration that follows is what actually happened, and **U4-Z's B3 section is not in it**; the u-rev 6 reviewer found D-320 disposing of a residual that section still called the architect's open choice. A claim of completeness beside an enumeration that is not complete is the same defect one level up, and it is deleted rather than argued.* The head's two bullets are rewritten to the current state; the change log is re-read u-rev by u-rev instead of appended to; U4-A gains a row for each of the five DECISION-RED-TEAM rounds and for the u-rev 5 review; §8's stub and its "fresh matrix" sentence are STAMPED as the u-rev 1 state and superseded; §9's pre-u-rev-5 closing paragraph is DELETED and replaced by the u-rev 6 fold. Sites beyond the six the report names — §8.2's heading, §8.3's gate table, §8.7's wiring, §11.6, U4-T, U4-M, U4-Z's lead-in, items 4 and 15, the OPEN list and the closing line — were re-read and repaired in the same pass |
| **BLOCKING 2** — U4-T registers S-E's two tests and U4-M prices the gate on S-E, with no caveat; the head's disclaimer does not reach them | **REPAIRED, and it is the same edit as the M3 fold.** S-E is superseded by **S-M** (D-323). U4-T's half-one row is restated against S-M with the referent REUSED not rewritten, the DEPENDS-OPEN-THEORY mark and the OPEN seam; U4-T's half-two row (the `visit` `assert!`) is marked **OPEN — not selected and not rejected**, because S-M is a criterion over the emitted set alone. U4-M's cost row is marked UNGROUNDED at its dominant term. §8.3's gate table, §8.7's wiring and §8.2's heading are retargeted with it. **Nothing is silently re-labelled: each site states what changed** |
| **BLOCKING 3** — the staged-config count restated at four sites, one of them inside the clause naming U3 §10 as the only place it may be stated | **REPAIRED at every one.** The change-log site now cites **U3** (u-rev 6, landed `13621d3`) §10 and states no cardinality; §8.3's B4 clause that stated the count *while* naming elsewhere as the only place it may be stated is DELETED, which the report's own fix shape says the derivation does not need; U4-Z's lead-in is rewritten without its cardinality; item 15 names `tactical_staged_v0.toml` as its own document instead of by ordinal. Two further staged-config cardinalities the scan reaches were removed in passing, at §8.3's derivation and at §8.3's config bullet. **Verified by re-running the reviewer's own line-break-tolerant scan: no live statement of the count survives** |
| **MAJOR 4** — U4-Z's lead-in blocks item 15 on "B3's unresolved wiring", which U4-Z records as CLOSED ninety lines above | **REPAIRED.** The false reason is DELETED, not argued; so is the stale "M3's fresh matrix". Item 4 is no longer blocked on M3 (selected) but on the SEAM; item 15 stays blocked on **D-324's** ground. *At u-rev 7 item 15's ground is RESTATED, not lifted: D-324's reason was that no seam was selected, and axis A now selects **N-E** (D-329), so what blocks it is that the seam is **SELECTED AND NOT BUILT** plus the missing `configs/instrument_staged_v0.toml`. Item 4's seam is a different decision and no selection has touched it.* **The reviewer's residual observation — that item 15's subject has no evident dependency on the snapshot's seam — is UNRECONCILED and is recorded in the OPEN list for the architect**, because this unit may not overrule a landed ADR line and may not hide the disagreement either |
| **MAJOR 5** — §8.4's M3 row declares its witness BUILT and names an abstract doc-comment shape, not a position | **REPAIRED.** The cell's "BUILT" is **WITHDRAWN AS FALSE**; the shape is restated as the REQUIRED PROPERTY of a witness; the row states that a position a legal game reaches is OWED, and it is named in the OPEN list rather than left inside a cell that says BUILT |
| **MAJOR 6** — U4-M item 1 registers the snapshot under "the amended script", which does not exist at HEAD or at any commit | **REPAIRED.** Item 1 now names **`tools/baseline_snapshot.sh` at `f317385`** — the pre-`--config` script, the only one that exists — as the BEFORE instrument, states that the AFTER is blocked twice over (no selected seam, D-324; and `configs/instrument_staged_v0.toml` does not exist, with the `ls` output pasted), and records that §9.1 amendment 4's MEASURED 34.5 s attributes a real wall time to an instrument that does not exist. *At u-rev 7 the first blocker is restated: the seam is **SELECTED (N-E, D-329) AND NOT BUILT**. The second is unchanged and re-measured. And the §9.1 note now sits at the amendment's own site too — **MINOR 4** of the u-rev 6 review* |
| **MINOR 7** — the stubbed matrix's column header is a live letter-address in the slot a future author copies | **REPAIRED** — the column reads "the differential gate's instrument", with the retirement noted |
| **MINOR 8** — the u-rev 2 correction's diagnosis is false, and the same false diagnosis is in the landed D-316 | **REPAIRED IN THIS UNIT ONLY.** The paragraph withdraws the diagnosis, states what the cell actually counted, and keeps the COUNT SIX. **`docs/decisions.md` is append-only and is not this unit's to edit: the D-316 residual is carried to the architect and is in the OPEN list**. *At u-rev 7 that residual is CLOSED: `D-325` landed at `81180b8` and corrects the log by a new line, which is the remedy this row asked for* |
| **MINOR 9** — "reinstate revision 1's four-part bar **verbatim**" is a paraphrase, at two sites | **REPAIRED at one site, RECORDED at the other.** §8.7's copy is corrected in place because it is carve prose. U4-Z's copy is inside the two-shape comparison the architect selected from, which this unit's own discipline leaves UNEDITED — so the correction is recorded immediately after it, beside the cost-cell correction, on that same discipline |
| **MINOR 10** — §9's fold (ii) puts D-316's residual into a list about a question D-316 is not about | **REPAIRED** — the conjunct is DELETED. The stop stands on the two conjuncts that are the red team's |
| **MINOR 11** — MAJOR 8's residual attributes the whole remaining gap to "the search is not built", but M6 owes a second construction independent of that | **REPAIRED** — the residual now has two parts, and part (ii) is the PARENT position from which M6's pinned witness is reached as a non-PV descendant at a null window, which is not gated on `staged.rs` existing |
| revision-7 review **MAJOR 12** (the unmarked `23.2`) and **MAJOR 9** (rule 5 / D-263) | **NOT REPAIRED, and not findings here.** The u-rev 5 reviewer verified both absent from this unit — `grep -n "23\\.2"` returns nothing, and the only rule-5 mention is game rule 5 at §8.4's M6 row — and recorded MAJOR 9's non-discharge as an IMPL gate, not a design defect |
| the reviewer's **"one observation for the architect"** — `section_owner_table.md` §11 recorded U4 at a stale size and u-rev | **DISCHARGED AT `a0f241b`, AND IT RE-OPENS WITH EVERY BUMP OF THIS UNIT — including this one.** The owner table re-measured its rows there and its U4 row was current for u-rev 6. u-rev 7 supersedes that measurement, so a re-measurement is owed to the owner table again. **What is recorded here is that STANDING DUTY, not a size and not a u-rev**, because a number stated here would be stale at the next bump — which is how the row this replaces went stale. The owner table is another document and this unit does not edit it, and **this unit asserts no line count of itself** (rule 9) |

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This unit restates no theory; where it appears to, the calculus wins and
the disagreement is an ADR line.

---

## U4-A. Lineage — what has attacked this unit's content, and at which revision

| Round | Against | Verdict reaching M3 / M4 / §8 / §9 |
|---|---|---|
| DECISION-RED-TEAM, matrix M3 | revision 1, `ec8f7fb` | **M3 FELL.** S-C was blind to D-124's own reproducer — `cells.pop()` after `order` leaves the class gate at 28 assertions, 0 RED. **S-E was supplied by that red team and had never itself been in a matrix** *(true until `d48824f`, where revision 2 carried it as an already-fallen row; the matrix then selected S-M — D-323)* |
| DECISION-RED-TEAM, matrix M4 | revision 1, `ec8f7fb` | **M4 SURVIVES AMENDED** — at a text three of whose cells the design has since MEASURED false. See §9 |
| REVIEW-design | revisions 2–6 | all FAIL. The tactical-suite gate's derivation (then §8.3(a)) was redesigned **three times** (rows 17, 32 and 27 of the superseded §0), and §8.4's ledger was rebuilt once |
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. **B1, B3, B4 and MINOR 15 are this unit's**, and MAJOR 8 (M4's and M6's witnesses are not positions a legal game reaches) is this unit's and is OPEN |
| DECISION-RED-TEAM, matrix M3, **round 1** | matrix revision 1, `f8e73e4` | **EVERY STATED OPTION FELL** (`docs/experiments/matrix_M3_REDTEAM.md`; **D-317**). Nine options; the decisive attack is the convention tension that makes an independent referent RED on a correct engine. Four missing rows named, two of them immune to the attack that killed S-E |
| DECISION-RED-TEAM, matrix M3, **round 2** | matrix revision 2, `d48824f` | **ATTACKED at `809b5db`; all eight facts reproduced; the recommendation S-K DIED on measurement.** **S-M SELECTED at `af8082a` (taken at `809b5db`)** (`docs/experiments/matrix_M3_selection.md`, whose own header reads *"Selected at `809b5db`"* — the revision carrying the attack; the record file exists only from `af8082a`; **D-323**), marked DEPENDS-OPEN-THEORY (D-321), with five registered conditions and an owed S-N flip trigger |
| DECISION-RED-TEAM, matrix M4, **round 1** | matrix revision 1, `77f7397` | the recommended option survives on repaired grounds; **no selection** (D-318) |
| DECISION-RED-TEAM, matrix M4, **round 2** | matrix revision 2, `cb16f7c` | **STOPPED.** Options survive; revision 2's reason for existing is false, one ground argues equally for five options, and the field omits the closed-enum selector that dominates the recommendation (D-318) |
| DECISION-RED-TEAM, matrix M4, **round 3** | matrix revision 3, `9ce863f` | **STOPPED — and this is the RECORDED TIE, not the every-option-fell stop** (`docs/experiments/matrix_M4_stop_round3.md`; **D-324**). Cause: the field has two orthogonal axes and has been scored as one. N-K's flip clause fires on axis B; N-Q is the missing row on axis A |
| DECISION-RED-TEAM, matrix M4, **round 4 — AXIS A ALONE** | `docs/experiments/matrix_M4_axisA_round4.md`, `7866bcf` | **THE MATRIX'S OWN RECOMMENDATION (N-Q) IS KILLED** (`docs/experiments/matrix_M4_axisA_REDTEAM.md`, at `7e0a328`; thirteen findings, nine of ten facts reproducing). The round was authored under the D-328 split — its author wrote no MEASURED cell — and the attack still broke FACT 5, its own headline, which pasted the wrong field of a three-field record line. **That is D-328's flip clause firing on the first round it governed (D-330)**, and the amended rule is three-part: author writes no cell, a stakeless measurer runs every command, **and the attacker RE-RUNS the inline evidence rather than reading it** |
| ARCHITECT-DELEGATED SELECTION, matrix M4, **axis A** | the field at `7866bcf` read against the attack at `7e0a328` | **N-E SELECTED at `7e0a328` (landed at `d56a898`)** (`docs/experiments/matrix_M4_axisA_selection.md`; **D-329**) — a required `--config PATH`, no default, with a NEW whole-path guard. **N-M eliminated on registered ground before the ladder ran** (`wp15b_sprt_prereg.md` §7A.2 registers `--config configs/gate_v0.toml` and N-M refuses it at exit 1); **rung (a) SILENT across the field**; selection taken at **rung (b)**. Four conditions ride with it, and its strongest surviving attack is **ASSEMBLED, NOT QUOTED** — a residual D-329 records, because no fresh context has been asked to break N-E |
| REVIEW-design, **this unit** | u-rev 5, `35aab95` | **FAIL** — 3 BLOCKING, 3 MAJOR, 5 MINOR (`docs/experiments/wp15b_U4_REVIEW.md`). BLOCKING 1 is D-305's class at this unit's most-read surfaces: the u-rev 4 and u-rev 5 folds landed without re-reading the head, this table, the change log, §8's stub, §9's closing paragraph, U4-T or U4-M. **Every finding is dispositioned in the head's REVIEW STATUS block; u-rev 6 was the answer to it** |
| REVIEW-design, **this unit** | u-rev 6, `7358a07` | **FAIL** — 1 BLOCKING, 2 MAJOR, 4 MINOR (`docs/experiments/wp15b_U4_REVIEW_urev6.md`). Both folds passed; the failure is the SAME class at the one section the u-rev 6 fold-in pass did not enumerate — **U4-Z**, which was open in the editor for two other findings and was not re-read against the ADR log it was citing. **u-rev 7 is the answer, and it re-derives U4-Z in full rather than patching the sentences the report names** |

**What this unit owes that no round has given it:** a REVIEW-design of THIS text at
THIS u-rev; **the differential gate's SEAM decision**, which D-323 records as separate
and OPEN and which D-115 makes a decision rather than a detail — *this is M3's seam and
is NOT the snapshot's config seam that D-329 selects*; **a fresh-context attack on N-E
in its own right**, which D-329 records as its own residual because the red team was
dispatched to break N-Q and was never asked to break the row it recommends; a POSITION
for §8.4's M3 witness and a PARENT position for M6's; and a SHELL_CHECKLIST review of
the `tools/` change §8.7 and §9 each carry, now with **four conditions riding on §9's**
(D-329). **What this unit no longer owes:** an architect selection on M4's recorded
tie, or a fourth M4 round scoped to axis A with N-Q authored into it — the round ran
at `7866bcf`, was attacked at `7e0a328`, and the selection landed at `d56a898`.

---

## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**

> ## **SELECTED — S-M, AND IT IS NOT S-E (D-323)**
>
> **THE OPTION IS S-M.** Field:
> `docs/experiments/matrix_M3_soundness_instrument_rev2.md` — revision 2, thirteen rows
> (the nine of revision 1 carried with the verdicts round 1 gave them, plus the four
> D-317 named as missing), authored `d48824f` under architect ruling R8 by a session
> that did not author revision 1. Attack: `docs/experiments/matrix_M3_REDTEAM_round2.md`,
> fresh context, at `809b5db` — **all eight of its facts reproduced.** Selection record,
> with the five registered conditions in full: `docs/experiments/matrix_M3_selection.md`
> (the selection was taken at `809b5db`, the revision carrying the attack, and landed at
> **`af8082a`**). **ADR line: D-323.**
>
> **WHAT S-M IS.** Per-node **EQUALITY** of the emitted set against the **LANDED**
> referent **R1** — `crates/pistol-solver/tests/common/reference.rs` — at every FILTERED
> node of the gate corpus, with the referent **REUSED by a `#[path]` include and NOT
> rewritten**.
>
> **S-E IS SUPERSEDED, AND SO IS THE MATRIX'S OWN RECOMMENDATION. THIS IS NOT A
> RE-LABELLING.** S-E is a row that **FELL in round 1**, on four independent attacks —
> the convention tension, D-115 applied asymmetrically, "forced by the tree" false, and
> a MEASURED cell that did not reproduce — and revision 2 carried it with that verdict
> and did not revive it. The recommended row of revision 2, **S-K, is dead too**, killed
> by its own attack: MEASURED, it fires on **NEITHER** of the two mutations §8.4
> registers for this gate's class — M4 survives it on M4's own pinned witness, and M3
> survives it on 20 of 20 differing nodes of the registered regime — while **S-M kills
> both**. An instrument for a gate that fires on none of that gate's registered
> mutations is the vacuity that killed S-C one round earlier. **What S-M changes from
> S-E is the REFERENT**: S-E promised a freshly written independent one, and its
> independence and its greenness were in tension; S-M reuses a landed,
> independently-written, already-gated one.
>
> **THE STRONGEST ATTACK SURVIVING AGAINST S-M — the red team's own words, which D-323
> quotes and which are carried here unparaphrased:**
>
> > S-M asserts the convention **D-321** records as OPEN: if the project settles toward
> > `DEF-T`'s minimum-cardinality reading, the gate turns RED on a correct engine, and
> > its referent cannot warn of it — R1 is independent of `cover.rs`'s CODE and shares
> > its CONVENTION, so fact 7's `0 of 3406` agreement is invariant under precisely the
> > defect in question, which is CLAUDE.md's two-instruments-blind-to-the-same-stage
> > clause read from inside. What survives the attack is that its advantage over the
> > immune-marked S-K is **not** confined to that open question: MEASURED, S-M kills
> > both registered S-E-class mutations — M4 on its own pinned witness, and M3 on 20 of
> > 20 differing nodes of the registered playout regime — where S-K kills neither, so R8
> > does not decide between them.
>
> **THE GATE SHIPS MARKED `DEPENDS-OPEN-THEORY` (D-321) AND THE CALCULUS IS NOT
> AMENDED.** Which convention a soundness criterion must assert — `DEF-T`'s
> minimum-cardinality reading or `cover.rs`'s inclusion-minimal enumeration — is OPEN
> THEORY. **The question's size, MEASURED and RE-TAKEN at u-rev 6 rather than carried:
> 22 of 174 FILTERED nodes, 12.6 %** — the figure **D-322** lands, and **NOT** D-321's
> original 84 of 300 / 28.0 %, which was taken over BOTH SIDES at a fixed
> `StonesLeft::Two` and `HitBudget::Two` instead of over the mover at the phase-derived
> budget **U2** (u-rev 5, landed `f0ae14c`) §5.3 specifies. Command and complete output at the foot of
> this block.
>
> **THE FIVE REGISTERED CONDITIONS BIND. Each is a red-team finding that would otherwise
> ride free:**
>
> 1. **THE REFERENT IS REUSED, NOT REWRITTEN.** IMPL takes R1 by `#[path]` include
>    (MEASURED, matrix fact 6: it compiles and answers from `pistol-search`'s test tree
>    for one include line plus a dev-dependency the WP lands anyway, at the cost of three
>    `dead_code` warnings). **A second, freshly written referent for this criterion is
>    FORBIDDEN without a registered agreement criterion and a registered consequence for
>    disagreement**, per CLAUDE.md's second-instrument clause. **This retires S-E's
>    "independently written plan-family referent in that crate's own `tests/common/`" as
>    an IMPL instruction** — §8.2 below still contains it, as record.
> 2. **`0 of 3406` MAY NOT BE CITED AS EVIDENCE ABOUT THE CONVENTION.** R1 and `cover.rs`
>    are blind to it together, so their agreement is invariant under the defect in
>    question. It is evidence about the ARITHMETIC and about nothing else. This unit
>    cites it nowhere and may not start.
> 3. **THE GATE SHIPS MARKED DEPENDS-OPEN-THEORY (D-321)** — §8.2 carries the mark and
>    the flip clause at the gate's own text, so no reader finds a gate whose criterion
>    looks settled.
> 4. **S-N IS OWED AND IS A FLIP TRIGGER, NOT A FOOTNOTE.** The rules-derived survival
>    row the red team found missing (F5) is registered as owed: if it is ever stated in a
>    form GREEN on a correct engine and affordable at a sampled population, **M3 REOPENS
>    as a two-row comparison** between it and S-M, because it would be immune to R8 by
>    construction rather than by taking a side, and it kills M4. Today it is unstated,
>    unpriced against S-M, RED on a correct engine in its only written form (a non-losing
>    pair may carry a free second stone outside any cover) and MEASURED at 1.76 M legal
>    turns per FILTERED node, mean; 2.61 M max.
> 5. **THE REGISTERED NUMBERS CARRY AN INSTRUMENT WITH A REVISION** — the probe behind
>    D-322's and D-323's figures is committed in full inside the selection record, which
>    is what gives it one. Red-team F7 found it had existed only as prose.
>
> **WHY THIS SELECTED WHERE M4 HAD NOT YET, on a field its own red team also found
> incomplete: DOMINANCE.** In M4 the missing closed-enum row satisfied the
> recommendation's own grounds identically while owing fewer guards — it dominated the
> recommended option outright. Here S-N dominates only the row that is **already dead**
> (S-K) and does not dominate the survivor. *(Written at u-rev 6, when M4 had stopped
> three times. **M4's axis A has since SELECTED N-E at `7e0a328`, D-329** — on a fourth
> round whose own recommendation its red team killed, which is the same shape this
> paragraph describes for M3. The contrast the paragraph draws is between the ROUNDS,
> not between the matrices' final states.)*
>
> **WHAT D-323 DOES NOT DECIDE, and this unit may not write as though it did.**
> **(a)** The **SEAM** by which a test observes the emitted set: every row needed one,
> D-115's constraint on widening `pistol_search::staged` applies to S-M exactly as
> round-1's F4 applied it to S-E, and it is **a separate named decision that is OPEN**.
> **(b)** The gate's **corpus and per-CI cost** — U4-M's ESTIMATED 40–90 s is CARRIED
> from S-E's pricing and is **not** this round's measurement. **(c)** The **convention**
> (condition 3). **(d)** S-E's **SECOND HALF**, the always-on `assert!` in `visit`: S-M
> is a criterion over the emitted set alone, so U4-T's
> `visit_searches_every_forced_candidate` row is **OPEN**, neither selected nor rejected.
>
> **THE FLIP CLAUSE (D-323).** Flips if S-N is stated green on a correct engine and
> affordable at a sampled population, at which point M3 reopens as a two-row comparison;
> or if the convention is settled toward `DEF-T`, at which point this gate is RED on a
> correct engine and **the criterion is re-derived rather than the engine changed**.
>
> **THE MEASUREMENT, RE-TAKEN AT u-rev 6 RATHER THAN CARRIED.** The selection record's
> own probe, copied verbatim into a throwaway `git worktree` on `/home` at `46c58ac` as
> `crates/pistol-solver/tests/zz_m3_phase.rs`; the worktree was removed and the live
> tree verified clean afterwards. **Complete output, untruncated:**
>
> ```
> $ cargo test -p pistol-solver --test zz_m3_phase -- --nocapture
>    Compiling pistol-core v0.0.1 (/home/tom/u4-repair-wt/crates/pistol-core)
>    Compiling pistol-solver v0.0.1 (/home/tom/u4-repair-wt/crates/pistol-solver)
>     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.77s
>      Running tests/zz_m3_phase.rs (target/debug/deps/zz_m3_phase-43a6603ed7b2c5b0)
>
> running 1 test
> mover positions (PROTO-NODE evaluated) = 1700
> FILTERED (U2 5.3, mover only, phase-derived left/budget) = 174
> conventions DIFFER = 22  (12.6 % of FILTERED)
> pop-mutant applicable = 154; S-K fires 132 (85.7 %), S-M fires 154 (100.0 %)
> test the_protocol_row_as_u2_states_it ... ok
>
> test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
> ```
>
> **It agrees with D-322 to the digit. It is the SAME instrument as the selecting
> session's, so it is a REPLICATION and not a second instrument** — stated so it is not
> mistaken for one, which is condition 2's own lesson one level up.

> ## THE RECORD OF THE EARLIER STATES — kept, STAMPED at u-rev 6, and RE-SCOPED at u-rev 7
>
> **THE STAMP IS SCOPED, IN BOTH DIRECTIONS, BECAUSE THE BLANKET ONE IT REPLACES WAS
> FALSE OF BLOCKS THE SAME u-rev WROTE.** Until u-rev 7 this read *"EVERYTHING FROM
> HERE TO THE END OF §8.7 IS RECORD … none of it is the state now"*, and that was
> **MAJOR 2** of `wp15b_U4_REVIEW_urev6.md`: the span it governed contains the u-rev 6
> folds and the carve-authored blocks this unit itself names, three paragraphs below,
> as RETARGETED. The prior round had found the older disclaimer too NARROW (it did not
> reach U4-T, U4-M or the U4-Z lead-in); widening it to a blanket over-reached the
> other way and told IMPL that the current specification of three of the four gates was
> superseded record.
>
> **WHAT IS RECORD BELOW — true at the revision that wrote it, superseded now, and to
> be read as the state AT THAT REVISION:** the u-rev 1 **SELECTION OPEN** heading, the
> stub and its five-row table; **§8.1**; **§8.2's BODY PROSE** from *"The stage under
> doubt, named"* to the end of §8.2 — S-E's criterion, its two halves, its seam
> respecification and the reduced S-C; the prose **§8.3** and **§8.4** carry from the
> superseded document; and every bare "S-E" inside all of that. The path since u-rev 1:
> the matrix was authored (u-rev 4), every option fell (D-317), a second field of
> thirteen rows was authored and attacked, and **S-M IS SELECTED** (D-323 — the block
> above).
>
> **WHAT IS NOT RECORD — these are the state now, and IMPL reads them as such:**
> **(i)** §8.2's opening `FOLDED AT u-rev 6` block, which states the selected
> instrument, the mark and the flip clause; **(ii)** §8.3's four-gate table, which is
> carve-authored at u-rev 2 under D-316 and retargeted at u-rev 6 — its differential-gate
> row states what that gate's instrument **IS**; **(iii)** §8.3's live sentence naming
> **U3** §10 as the one place the staged-config count is stated; **(iv)** §8.4's
> `RE-READ AT u-rev 6` block and the u-rev 6 marks inside the ledger's M3, M4 and M6
> cells; **(v)** §8.7's four-name wiring sentence and the `FOLDED AT u-rev 6` paragraph
> beneath it, **which is the live specification `tools/staged_soundness_check.sh` is
> taken from**. Each of the five says at its own site which it is.
>
> ### u-rev 1 — SELECTION OPEN — M3 HAS NO MATRIX, AND S-E IS NOT SELECTED HERE
>
> **B1 of the revision-7 review, and F7 of the restructure red team.** MEASURED,
> `grep -c "^| Option |"` over the superseded document returns **3** — **U1** §4.2 (M0),
> **U2** §5.6 (M5), **U3** §6.3 (M1). §8 has none. `S-A`, `S-B` and `S-D` occur once each in
> prose; the matrix that held them was deleted at revision 2 and never restored.
> **The adopted option S-E is not in it either:** MEASURED, `S-E` occurs **zero**
> times at `ec8f7fb`. S-E was supplied by the DECISION-RED-TEAM that killed S-C,
> so there is nothing to recover — M3 is a FRESH matrix that has never been
> authored and has never been attacked in the form it would be selected in.
> *(u-rev 1, SUPERSEDED: it was authored at `f8e73e4`, attacked, re-authored over
> thirteen rows at `d48824f`, attacked again, and selected from at `af8082a`.)*
>
> **CLAUDE.md:** "An option adopted without a matrix, or a matrix never attacked,
> is the same breach as silent architecture drift."
>
> **THE MATRIX SLOT, STUBBED. The carve does not fill it and does not select.**
>
> *(u-rev 1, SUPERSEDED. The slot was filled twice — `f8e73e4` with nine rows,
> `d48824f` with thirteen — attacked twice, and selected from at `af8082a`. The five
> rows below are the u-rev 1 STUB and are not the field that was attacked; that field
> is `docs/experiments/matrix_M3_soundness_instrument_rev2.md`.)*
>
> | Option | the differential gate's instrument *(this column head read `(b)'s instrument` until u-rev 6 — MINOR 7: a live letter-address in the one slot a future author copies its frame from, and the authored matrices at `f8e73e4` and `d48824f` sensibly did not use it)* | Cost | Failure modes |
> |---|---|---|---|
> | S-A — value agreement, as WP-1.1 | *to be authored* | *to be authored* | *to be authored* |
> | S-B — candidate-set containment | *to be authored* | *to be authored* | *to be authored* |
> | S-C — class-restricted answer agreement | *to be authored* | *to be authored* | *to be authored* |
> | S-D — S-C plus a mutation gate | *to be authored* | *to be authored* | *to be authored* |
> | S-E — per-node survival-set containment | *to be authored* | *to be authored* | *to be authored* |
> | *further options* | *the red team supplied the surviving option in four of six matrices in this WP (D-305); the slot is not closed at five rows* | | |
>
> **Every numeric claim entered in that matrix is marked MEASURED or ESTIMATED,
> and an estimate that could have been measured in seconds is a finding** —
> D-291, third occurrence in this work package (restructure red team F1).
>
> **THE MATRIX WAS AUTHORED AND ATTACKED AT u-rev 4, AND EVERY OPTION FELL.**
> `docs/experiments/matrix_M3_soundness_instrument.md` (authored `f8e73e4`) put
> nine options — the five above plus S-F, S-G, S-H and a null row — to a
> fresh-context DECISION-RED-TEAM, whose report is
> `docs/experiments/matrix_M3_REDTEAM.md`. **VERDICT: every stated option falls.**
> Under the architect's standing rule the matrix is STOPPED and no survivor is
> forced; the second authoring round is owed and is the architect's to schedule.
> D-317 records it.
>
> **THE DECISIVE ATTACK, and it lands on the text below.** S-E's criterion is an
> EQUALITY against the union over **inclusion-minimal** covers, checked by a
> referent "written against the plan-family definition". MEASURED: the plan-family
> definition D-266 makes canonical, `docs/research/threat_calculus_v1.md`,
> contains `inclusion` **zero** times, and its `DEF-T` states the OPPOSITE
> convention — *"exact **minimum hitting set**"*. On a legally reachable FILTERED
> node the two readings give `{(-1,0),(-1,5),(4,0)}` against `{(-1,0)}`. **So a
> referent that is genuinely independent makes the gate RED ON A CORRECT ENGINE,
> and one that is green has read `cover.rs` — D-295's defect.** Independence and
> greenness are in TENSION. §8.4's own mutation M4 is the ledger's admission that
> the two conventions differ on a reachable position, which is what the S-E text
> below needed to read and did not.
>
> **AND ONE ROW WAS NEVER NEEDED TO BE WRITTEN.**
> `crates/pistol-solver/tests/common/reference.rs:223` already implements
> `blocking_covers` *"by the definition: every subset within budget that covers,
> minus every one with a proper subset that also covers"* — the independent
> referent S-E proposes to build EXISTS, in the inclusion-minimal convention, and
> the matrix's own fact 4 ran the `ls` that printed the file without reading it.
>
> §8.1 through §8.7 below are carried verbatim as the RECORD of why S-C fell and
> of what S-E was written to do. **They are not a selection, and after this round
> they are not a recommendation either.** **No ADR line may cite S-E as adopted** —
> item 4 in U4-Z is written on that footing, and D-317 now gives that footing a
> measured ground rather than a procedural one.
>
> **THREE STAMPS ON THAT PARAGRAPH AT u-rev 6, because the fold above changed each
> of them.** (a) **S-E is still not adopted and never will be** — it fell, and what
> is adopted is S-M (D-323), so the sentence is true but no longer the reason
> anything is open. (b) **The disclaimer's scope was wrong and the u-rev 5 review
> found it so (BLOCKING 2):** §8.3's four-gate table and §8.7's wiring paragraph are
> **carve-authored at u-rev 2 under D-316**, not carried verbatim, and both named the
> differential gate as "S-E, with the reduced S-C beside it"; both are retargeted at
> u-rev 6. And **U4-T, U4-M and the U4-Z lead-in were never inside this disclaimer's
> stated scope at all**, which is how S-E survived at three uncaveated sites for four
> revisions. (c) **Item 4 in U4-Z is no longer written on this footing** — it is
> rewritten against S-M and is blocked on the SEAM instead.


### 8.1 Why S-C fell

**MEASURED, not argued.** D-124's own reproducer applied verbatim to
`pvs::visit` — `if cells.len() > 1 { cells.pop(); }` immediately after `order`,
confirmed applied by node counts falling 8794 → 8374, 10482 → 10045, 12260 →
11880 at depth 2 radius 2 — leaves S-C's entire class gate at **28 class
assertions, 0 RED**, identical to the pristine baseline.

D-124's defect is INTERIOR-node narrowing that never moves a ROOT maximum. S-C is
still a ROOT oracle; restricting *which roots* you assert on buys a root oracle no
interior visibility. **Revision 1 rejected S-A as "the criterion the defect class
preserves" and then adopted a criterion the same defect class preserves.**

Four more findings, each measured:

- **The forced-block class is EMPTY as written.** `ReferenceRun::argmax()` returns
  `Vec<Turn>` and a `Turn` is `Pair(Coord, Coord)`; the free second stone is never
  a hitting cell, so "the argmax set is a subset of the cells that hit every
  opponent plan" is unsatisfiable — 0 of 62 position-depth cases. The loose
  reading gives 4 positions at one depth. The reading that would admit `t = 2`
  positions IS `Cover::cells()`, i.e. mutation M3 itself.
- **M3 and M4 provably cannot fire.** Every forced-block member is
  `Minimal([One(c)])` with phase 1 `NothingToBlock` after the block, so
  inclusion-minimal and minimum-cardinality coincide and flattening changes
  nothing. M6 has no root witness: 0 corpus positions with `t ≥ 3` AND win-now.
  Three of seven mutations are killable.
- **The registered workload is not runnable.** `configs/gate_v0.toml`'s own
  committed table measures radius 2 at **depth 4** as "> 100 s" — depth 3 is 9.7 s
  there, and revisions 2–5 of this document read that table one column left; the full-width reference on the
  corpus's cheapest branching position at radius 2 depth 3 is **243 363 538 nodes
  in 554.2 s** — one position of thirty-one. Revision 1 attributed the cost to
  gate 10, which runs depth 3 at radius **1** on three fixtures.
- **Inside the forced-block class the argmax restraint is vacuous.** `|argmax| = 1`
  on all four members, so membership IS identity — which D-119 refuses by name —
  and the top entries differ only in the FREE second stone by 2 to 24 eval units,
  the quantity **U2** §5.3 explicitly licenses narrowing to move.

One half of the brief's own suspicion did **not** land, and the reviewer credits
it: `tactical_v0.txt`'s expectations are game facts, not generator output
(`expect cell` over `expect move`, deliberately), so a staged fixture carrying the
same rows is not the D-287/D-295 shape.

### 8.2 THE DIFFERENTIAL GATE — **S-M IS SELECTED (D-323)**; the text below is the RECORD of S-E

> **FOLDED AT u-rev 6.** The instrument for this gate is **S-M**: per-node **EQUALITY**
> of the emitted set against the **LANDED** referent **R1**
> (`crates/pistol-solver/tests/common/reference.rs`), at every FILTERED node of the gate
> corpus, with the referent **REUSED by a `#[path]` include and NOT rewritten** (D-323,
> condition 1). The heading said **SELECTION OPEN** from u-rev 1 to u-rev 5; it is not
> open any more.
>
> **WHAT DID AND DID NOT CHANGE.** The stage under doubt is unchanged and is still the
> right one — *does the staged generator ever drop a cell a proven tactic needs?* — and
> so are the FILTERED-row predicate (`can_win_this_turn` is `None` **AND**
> `blocking_covers` answers `Minimal`), the guard argument that makes it well-posed, and
> the EQUALITY-not-containment argument. Those are why S-M is the row that survived.
> **What changed is the REFERENT, and only the referent.**
>
> **THIS GATE SHIPS MARKED `DEPENDS-OPEN-THEORY` (D-321), and D-323's condition 3
> requires the mark and its flip clause to stand at this gate's own text, which is
> here.** The criterion asserts the INCLUSION-MINIMAL convention. Whether a sound
> candidate generator must carry that convention or `DEF-T`'s minimum-cardinality one is
> **OPEN THEORY**, and `docs/research/threat_calculus_v1.md` is **NOT amended** by this
> selection — the calculus changes by its own amendment rule and not by a matrix's
> convenience. **MEASURED, re-taken at u-rev 6 (command and complete output at the head
> of §8): the two conventions give different unions on 22 of 174 FILTERED nodes,
> 12.6 %.** *FLIP CLAUSE: if the convention is settled toward `DEF-T`, this gate is RED
> on a correct engine and **THE CRITERION IS RE-DERIVED, NOT THE ENGINE CHANGED**; or if
> S-N is stated in a form green on a correct engine and affordable at a sampled
> population, M3 REOPENS as a two-row comparison.*
>
> **THREE THINGS BELOW ARE RECORD AND ARE NOT INSTRUCTIONS TO IMPL.**
> **(1)** The *"independent from-scratch plan-family implementation in that crate's own
> `tests/common/`"* is **FORBIDDEN** by D-323 condition 1 without a registered agreement
> criterion and a registered consequence for disagreement — R1 is reused instead,
> because writing a second referent would create two instruments for arithmetic one
> already covers. **(2)** The *expensive half / cheap half* PAIR is S-E's shape. S-M is
> the emitted-set criterion **ALONE**; the always-on `assert!` in `visit` is **neither
> selected nor rejected — it is OPEN** (U4-T, U4-Z). **(3)** The **reduced S-C beside
> it** is not part of the selection either: the composite row that carried it, **S-J**,
> FELL in round 2 — it inherited every one of S-E's four kills, and pairing a wounded
> criterion with a cheap regression does not make the wounded half sound — so the
> `MEASURED 17.89 s` mate-class regression below is a costed proposal with no surviving
> matrix row and is **OPEN**.
>
> **AND THE SEAM IS STILL OPEN.** D-115's constraint on widening `pistol_search::staged`
> to `pub` — round-1 F4, which forbids S-E's primary mechanism BY NAME — applies to S-M
> exactly as it applied to S-E, and D-323 records the seam as a **separate named
> decision this selection does not make**. **The paragraph below beginning "Where S-E
> observes" answers a question the project has re-opened.**


**The stage under doubt, named** — revision 1 named the defect and never the
stage: **does the staged generator ever drop a cell a proven tactic needs?**

**S-E — per-node survival-set containment. The primary instrument.** At every node where **`can_win_this_turn` is `None` AND** `blocking_covers`
answers `Minimal` — which is **U2** §5.3's FILTERED row and not `blocking_covers` alone —
assert the emitted set **EQUALS** the union of cells over the inclusion-minimal
covers, against a hitting set computed
**independently of the generator**. **Equality, not containment** — **U2** §5.3 says the
filtered row emits the cover union "and nothing below it", and a containment
criterion is preserved by every mutation that OVER-generates, which is §8.1's own
charge against S-C arriving at S-E. Mutation M3 is exactly such a mutation:
phase-1 cover cells are provably a SUBSET of the phase-0 union, so failing to
regenerate can only over-generate, and containment would have let it live — the test-side
exact reference, not `ThreatState`. At every node, assert the emitted set is
non-empty and its tiers disjoint.

This is D-124's own named remedy, quoted in revision 1 and then not implemented:
*"What would help is comparing the engine's per-node candidate set with the
reference's"*. Revision 1 rejected its nearest relative (S-B) as a superset claim
D-124 never made, and priced it at a visibility cost that belonged to **WP-1.1's**
non-goal list, not this WP's — whose test plan already registers a test watching the
generated cell set at both phases (U2-T). D-124's flip clause reads *"flips when that
check lands, which is where the visibility question gets settled"*; S-E settles
it.

**Where S-E observes, and against what — respecified, because revision 3's seam
cannot be built.** A REVIEW-design closed all three of its doors: `pvs` is
`pub(crate)` so `Run` is unreachable from an integration test;
`crates/pistol-solver/tests/common/plans.rs` is another crate's integration-test
module and no `src/` can `use` it; and D-115 forbids widening either to let a test
reach it, while D-129 forbids putting a CORRECTNESS invariant behind
`debug_assertions` — "never a quiet demotion". Revision 4 splits the claim in two
and gives each the mechanism its own class calls for.

- **The expensive half is a test over a PUBLIC generator.** `pistol_search::staged`
  becomes public with one entry point returning the ordered candidate vector and
  its forced count. An integration test in `crates/pistol-search/tests/` walks
  positions with the existing reference walker and, at every one, compares the
  forced prefix against a hitting set computed by an **independent from-scratch
  plan-family implementation in that crate's own `tests/common/`** — the pattern
  rule 7 already names for movegen (`bruteforce.rs`) and the search
  (`reference.rs`). That is NOT D-295's defect: D-295's defect was checking a
  shipped oracle against a test-side oracle DERIVED FROM THE SAME FIXTURE; this
  checks a shipped generator against an independently written referent.
- **The cheap half is an always-on `assert!` in `visit`.** The test above sees what
  the generator EMITS; D-124's reproducer (`cells.pop()` after `order`) is a drop
  AFTER generation, which no test of the generator can see. So `visit` carries a
  named invariant — the first `forced` candidates are searched unless a cutoff or
  an abort intervened — as an `assert!`, not a `debug_assert!`: its violation
  makes the answer wrong, which is D-129's own criterion for the macro. Cost is
  one comparison per node. It is a PRIVATE invariant guard, which is precisely
  what D-115 permits in-source.

Two consequences worth stating. The gate then runs in whatever profile CI uses
without a `release-checked` argument, because an `assert!` survives release —
revision 3's `debug_assertions` seam would have been compiled out of
`tools/search_oracle_check.sh`, `tactical_check.sh`, `determinism.sh` and
`movetime_check.sh`, all of which drive release binaries. And §13's estimate is
re-based: the traversal cost is the test's, once per fixture, not "at every node".

**The guard is not decoration, and revision 6 dropped it.** **U2** §5.3 selects the row
on `can_win_this_turn` FIRST; `blocking_covers` is consulted only in the `None`
arm. A node where the mover can win now AND the opponent holds a coverable hot
window takes the WIN-NOW row and emits the win class — while `blocking_covers`
still answers `Minimal`. MEASURED on the shipped fixture case
`mate_in_1_own_win_beats_blocking_the_opponent`: `can_win_this_turn = Some(OnePly
{ at: (5,0) })` and `blocking_covers = Minimal([One((5,3))])`, so revision 6's S-E
would assert `{(5,0)} == {(5,3)}` and fail a CORRECT implementation on a corpus
position — while passing mutation M5, whose mutant emits exactly `{(5,3)}`. The
instrument was inverted on that class. Revision 5's wording was red on the same
node for the same reason; revision 6 removed the win-now half of the criterion and
did not add the guard that made it well-posed.

It also dissolves three of §8.1's findings at once: no argmax type confusion, no
verdict decided by a free stone's 2–24 eval units, and no comparison across two
universes the adopted Tier-T option makes non-comparable.

**Reduced S-C — a cheap mate-class regression only.** Radius 1, the three fixtures
plus one built mate-in-3 that gate 10 already affords at **MEASURED 17.89 s**. Not
a corpus sweep, not radius 2, not depth 3 at radius 2. Its mate class must state
whether `LossInTurns` roots are included or excluded **by name** — `compact_mated_in_2`
falls in neither class today.

### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED

**THE LETTERS ARE GONE. B3 IS REPAIRED HERE, BY SHAPE 2 (D-316).** The soundness
gate has FOUR parts and each is named by what it is, so that no part can lose its
definition the way `(b)` did — deleted with revision 2's enumeration and then
wired into CI for five revisions as a letter naming nothing:

| The four gates | Where it is specified |
|---|---|
| **THE TACTICAL SUITE UNDER STAGED** | §8.3 below, first bullet |
| **THE DIFFERENTIAL GATE** — **S-M** (D-323): per-node EQUALITY against the LANDED referent R1, REUSED not rewritten; marked DEPENDS-OPEN-THEORY (D-321). *This cell read "S-E, with the reduced S-C beside it" from u-rev 2 to u-rev 5; S-E fell in M3 round 1 and the composite carrying the reduced S-C (S-J) fell in round 2* | §8.2 |
| **THE COLONY FAMILY**, ≥ 6 built cases | §8.3 below |
| **THE PATTERN FIXTURES UNDER STAGED** | §8.3 below |

**LEGACY CITATIONS RESOLVE THROUGH THIS TABLE AND ARE NOT ORPHANED**, which is
the one cost shape 2 was charged with: `§8.3(a)` is the tactical suite,
`(b)` is the differential gate and is §8.2's subject, `(c)` is the colony family,
`(d)` is the pattern fixtures. The letters are retired as an ADDRESSING SCHEME;
they are kept here as a lookup so an existing "§8.3(a)" in another unit, in the
superseded document's history or in a landed ADR line still lands on the right
gate. Nothing below is addressed by letter.

- **THE TACTICAL SUITE UNDER STAGED** *(the superseded `(a)`)*. **Revision 5's derivation rested on set
  containment; a negamax value is not monotone in the candidate set, so it is
  redesigned again — this time on where the two searches can differ at all.**

  Revision 5 argued: with the cut disabled the staged set is a SUPERSET of the
  radius policy's, so anything the radius search finds Staged finds. The
  containment half is true and survived attack. The inference is not: enlarging
  the set at the OPPONENT's nodes LOWERS the value, so a superset can refute a
  mate as easily as find one. Revision 5 also claimed none of the fifteen
  `instrument_v0` cases takes a batched row; MEASURED, the phase-1 node
  immediately below each `must_block` root answers `NothingToBlock` with a 48–56
  cell quiet pool against `quiet_top_k = 16`, so the cut binds inside all four.

  **The derivation that does work, and what it costs.** **BOTH staged TACTICAL
  configs disable the quiet cut** (`quiet_top_k` above the whole pool):
  `tactical_staged_v0.toml` for the fifteen `instrument_v0` cases and
  `gate_staged_v0.toml` for the five gate cases. The remaining staged documents —
  the SPRT seat and the play config — keep the cut. **U3** (u-rev 6, landed `13621d3`) §10 is the one
  place the number of staged config documents is stated, and **this unit states no such
  number as a live claim**. *The clause that stood here until u-rev 6 gave that count as
  a cardinal and, in the same breath, cited **U3** §10 as the only place the count may
  be stated — the count stated inside the very clause naming elsewhere as its only home.
  That is **BLOCKING 3** of `wp15b_U4_REVIEW.md`, and it is B5's class reproduced inside
  B4's own repair for the second consecutive revision of this sentence. It is deleted:
  the sentence above already names both tactical staged documents and what each covers,
  which is all the derivation below needs. **The clause was QUOTED here until u-rev 7
  and is now DESCRIBED instead** — quoting it restated the deleted count four lines
  after the sentence saying the unit states it nowhere, which is **MINOR 5** of
  `wp15b_U4_REVIEW_urev6.md`: the same defect one level up, in the third consecutive
  revision of this one sentence.* *Revision 7
  repaired §10 and §15 item 15 and left this sentence saying "all three staged
  tactical configs … not just the two gate ones", which was false in both halves
  and had no referent for "the two gate ones" — B4 of the revision-7 review, the
  recurring defect reproduced inside the repair that names it. The DERIVATION
  below is unaffected: the twenty cases run at `tactical_staged_v0.toml` (15) and
  `gate_staged_v0.toml` (5), both with the cut disabled, which is what it needs.* Then Staged and the radius policy can differ at a
  node in exactly three ways, and each is a `[PROVEN]` law rather than a
  heuristic:

  0. **The `!is_pv` OVERLOAD RETURN** — a fourth way, and revision 6's "exactly
     three" was wrong to omit it: it GENERATES NOTHING and returns
     `-mate_in(k+2)`, so it is not a generation row at all. **U3** §7 names it as the SPRT's third axis, and **U2** §5.2 MEASURES it changing the
     bestmove on 2 of 24 corpus roots and a non-mate score on 3 of 24. It fires
     inside a `require 20` case: enumerating every P1 turn from the radius-1 ball
     at `mate_in_3_double_three_becomes_double_four` gives **8** resulting P2
     nodes answering `Impossible`, all reached at a null window and therefore
     non-PV. **Its direction argument**: the return replaces a search with a
     verdict that is exact when its premises hold, so it can only shorten a mate
     distance on a position that is already lost — never invent one, since the
     premise includes `can_win_this_turn` being `None` for the side it condemns.
     **Its licence is not yet landed**: 455 177 of 455 201 firings are the
     `t >= 2` / one-stone form, which `LAW-OVERLOAD` does not state, and **U2** item
     22 records the calculus amendment as OWED. Until that lands, `require 20`
     rests partly on theory this project has derived and not yet adopted, and
     this document says so rather than counting it among the `[PROVEN]` three.
  1. **WIN-NOW row** — the emitted set is the argmax set, because nothing beats
     `mate_in(k+1)`. The value is identical to full-width's.
  2. **BATCHED row** — the emitted set is a SUPERSET (Tier T ∪ the whole ball).
     The value can differ, and the direction is not fixed — but it differs only by
     the search seeing MORE, which is the direction the fixture's expectations are
     written in: they are game facts (`expect cell`, `expect mate n`), and its
     header says "A case states only what the game decides".
  3. **FILTERED row** — the emitted set is the cover union. **It is a SUBSET of
     the radius policy's set at radius 2 and NOT at radius 1**, which revision 6
     asserted unconditionally. A cover cell is an empty of an opponent hot window,
     hence within distance 2 of an own stone — inside a radius-2 ball, but able to
     fall outside a radius-1 one. MEASURED at the gate case
     `mate_in_3_double_three_becomes_double_four` (radius 1): of **182** FILTERED
     descendants, **157** have a cover cell outside the ball. MEASURED at
     `must_block_p2_five_in_a_row` (radius 2): **0 of 10**. So at the gate configs
     the two sets INTERSECT rather than nest — and the direction still holds, for
     a different reason: the cells Staged adds are exactly the blocks `LAW-HIT`
     says are the only defence, which the radius-1 policy never offered. Staged
     sees more of the truth there, not less. `LAW-FORCE`
     is `[PROVEN]` that every excluded move LOSES. So a value difference here can
     only be Staged declining a move that loses, and if full-width's answer rested
     on such a move, full-width was wrong about the game and right only about its
     horizon.

  **`require 20` is pre-registered on that**, and on nothing weaker: every case's
  root and every node of its proof line is one of those three, and in each the
  divergence is either nil or in the direction of the game fact the case asserts.
  A failure is a red gate to investigate — and, specifically, evidence that one of
  the three laws is being composed wrongly, which is what makes it worth gating.

  **What this deliberately gives up.** With the cut disabled the tactical suite is
  silent about the quiet prune. That is the right division — the prune is a
  strength knob and rule 6 makes SPRT its judge — but it must be said, because a
  green tactical suite under Staged is then NOT evidence about the cut. The cut is
  judged by the movetime measurement (`WPQ_seed.md`, §12 item 3), by the SPRT,
  and by the differential gate (§8.2 — **S-M** since D-323, S-E until u-rev 6).

  **And it is an honest weakening from revision 5**, which claimed the suite ran
  fifteen of twenty cases at the committed `quiet_top_k = 16`. It does not.

- **The five gate_v0 cases need a staged config — A CONFIG STATEMENT, NOT A FIFTH GATE**, and it was the unlabelled bullet whose presence beside three lettered ones helped hide that a fourth letter had gone missing. MEASURED: `tactical_v0.txt`
  is 15 cases at `configs/instrument_v0.toml` and **5 at `configs/gate_v0.toml`**
  (radius 1, the `depth_turns 3` cases, because gate_v0's table measures radius 2
  at depth 4 as > 100 s, and depth 3 at 9.7 s). Earlier revisions shipped fewer staged
  documents than this derivation needs; **U3** (u-rev 6, landed `13621d3`) §10 states the number that
  ships and is the only place that states it (B5).
- **THE COLONY FAMILY, ≥ 6 built cases** *(the superseded `(c)`)*, distant-cluster attack and defence,
  where `LAW-DECOMP`'s star-disjointness puts the right answer in a cluster the
  delta ranking does not favour.
- **THE PATTERN FIXTURES UNDER STAGED** *(the superseded `(d)`)*, **re-scoped so it is about the stage.** As written it never ran the search:
  D-295 measured the pattern pack's whole contact with `crates/pistol-solver/src`
  as 33 booleans plus four `hot_windows` assertions. Revision 2 runs the **U2** §5
  pattern positions **through the staged generator** and asserts `PAT-GAP`'s
  singleton gap cell is in Tier F — `LAW-HIT`, the singleton plan must be hit.
  Confirmed available from the fixture's own data: PAT-GAP's plans are
  `{-1,0 1,0} {1,0} {1,0 6,0} {6,0 7,0}`, so `{1,0}` is a singleton and every
  minimal cover contains it.

### 8.4 The mutation ledger, with witnesses

S-D's discipline is kept — asserting an instrument's strength rather than
measuring it is D-295's finding one WP earlier — but the ledger is rebuilt,
because three of revision 1's seven mutations could not fire and one had no
witness. **Each mutation names the position it dies on, and where the corpus
cannot produce one it is BUILT** (D-260's precedent and its remedy).

**RE-READ AT u-rev 6, AND THAT CLAIM IS NOT TRUE OF EVERY ROW.** **M3's witness is
NOT built** — the cell said BUILT and named an abstract shape from a doc comment, not
a position; MAJOR 5 of `wp15b_U4_REVIEW.md`, corrected in the row itself and carried
OPEN in U4-Z. **M4's and M6's ARE built** and are pinned by
`crates/pistol-solver/tests/wp15b_mutation_witnesses.rs` (u-rev 3, MAJOR 8), and M6
owes a second construction on top of that (MINOR 11, U4-Z). **And the class column
moved:** what this table calls the `S-E` class is the DIFFERENTIAL GATE's class, whose
instrument since D-323 is **S-M** — M3 and M4 are the two mutations of it, which is the
measurement that killed the matrix's own recommendation.

| # | Mutation | Class | Witness |
|---|---|---|---|
| M1 | Tier F drops the pair-completion class | mate | `mate_in_1_two_stones_complete_a_row` (corpus) |
| M2 | Tier F drops `win_in_one_ply_cells` | mate | the **nine** single-stone `mate_in_1` cases (corpus; eleven `mate_in_1` cases in all, two of which are two-stone and belong to M1) |
| M3 | The FILTERED row emits `Cover::cells()` flattened at phase 0 and does not regenerate at phase 1 | the differential gate (**S-M** since D-323; registered as S-E's class until u-rev 6) | **NOT BUILT. THE "BUILT" THIS CELL CARRIED UNTIL u-rev 6 IS WITHDRAWN AS FALSE** — MAJOR 5 of `wp15b_U4_REVIEW.md`. What follows is a REQUIRED PROPERTY of a witness and not a witness, and §8.4's lead-in promises a named position or a built one. Revision 6's witness was inert under EQUALITY too: with a single two-cell cover the stale union minus the played cell EQUALS the correct phase-1 set, so nothing separates them. **The property a witness must have:** a phase-0 union of **three or more** cells — `cover.rs`'s own `{a,b} {b,d} {d,e}` shape, whose union is `{a,b,d,e}` while the phase-1 set after any one cell is strictly smaller. **That shape is not a position.** It is the abstract window-empties example in `crates/pistol-solver/src/cover.rs`'s module doc comment: no coordinates, no stone counts, no parity, no legality, no pin. **A POSITION A LEGAL GAME REACHES IS OWED**, on the `wp15b_mutation_witnesses.rs` pattern MAJOR 8's repair established for M4 and M6 — a row MAJOR 8's literal scope (§17 named M4 and M6) did not reach. It is in U4-Z's OPEN list rather than left inside this cell. *Separately, and it does NOT discharge the witness: M3 round 2 MEASURED the mutation's class on a proxy — S-M fires on 20 of 20 differing nodes of the registered playout regime and the matrix's recommended row S-K on 0 of 20 (D-323), which is what killed S-K.* |
| M4 | Minimum-cardinality covers instead of inclusion-minimal | the differential gate (**S-M** since D-323; registered as S-E's class until u-rev 6) | **BUILT, and revision 4's witness was inert.** The shape must have a 1-cover COEXISTING with a minimal 2-cover; `cover.rs`'s flat-list counterexample has no 1-cover, so the two notions coincide there and the mutant is an identity. **REBUILT AT u-rev 3 AS A POSITION A LEGAL GAME REACHES (MAJOR 8).** The witness this row carried until u-rev 2 held P1 = 8 stones with no stone on the origin — MEASURED refused by the rules on three counts at once, so it was a `ThreatState::apply` construction and never a position the SEARCH could be at. The rebuilt witness, with P2 to move: **P1** `(0,0)(1,0)(2,0)(3,0)` and `(-1,1)(-1,2)(-1,3)(-1,4)` and `(0,7)`, **P2** `(-2,0)(5,0)(-1,-1)(-1,6)` and `(4,-4)(5,-4)(-4,4)(-5,5)`. Nine P1 stones and eight P2 stones is rule 3's parity; the two arms share the empty corner `(-1,0)` and each is sealed at both far ends, so exactly one window per arm is hot. **MEASURED by replaying every ply through `GameState` and then querying the shipped solver:** `can_win_this_turn(P2,Two) = None` and `blocking_covers(P2,Two) = Minimal([One((-1,0)), Two{(-1,5),(4,0)}])` — the 1-cover coexisting with the minimal 2-cover, and minimum-cardinality drops the pair. Pinned by `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs` |
| M5 | The WIN-NOW row emits the cover union instead of the win class | **mate** | `mate_in_1_own_win_beats_blocking_the_opponent`. Revision 4 named "own win-now cells dropped from the FILTERED set", a path **U2** §5.3 deleted — on that position `can_win_this_turn` is `Some`, so the node takes the WIN-NOW row and `blocking_covers` is never called |
| M6 | The overload return drops its `can_win_this_turn` guard | **mate**, not the differential gate's class ("not S-E" until u-rev 6) | **BUILT, AND REBUILT AT u-rev 3 AS A POSITION A LEGAL GAME REACHES (MAJOR 8).** The shape is unchanged and was never the defect: P1 holds one five-run sealed at one end, so exactly one cell completes it, and P2 holds three disjoint five-runs at rows 8 / 16 / 24 — 8 apart keeps every placement legal under rule 5 and 8 > 5 guarantees no shared window. What was wrong was the COUNT: P2 held 15 stones, and rule 3 gives P2 an even number at every turn boundary. The rebuilt witness, with P1 to move: **P1** `(0,0)(1,0)(2,0)(3,0)(4,0)`, the three seals `(-1,8)(-1,16)(-1,24)`, and seven further stones `(0,4)(3,4)(0,12)(3,12)(0,20)(3,20)(7,4)` placed where no window reaches four — fifteen in all; **P2** the seal `(-1,0)` and the three runs `(q,8)(q,16)(q,24)` for `q` in `0..5` — sixteen. The seven fillers are not decoration: P2's sixteen stones force P1's fifteen, and a witness that cannot be counted to cannot be replayed. **MEASURED by replaying every ply through `GameState` and then querying the shipped solver:** `can_win_this_turn(P1,Two) = Some(OnePly{ at: (5,0) })` while `unblockable_double_threat(P2,Two) = true`. Pinned by `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`. Its class is mate and not S-E, because the mutant RETURNS rather than emitting and S-E is blind at an `Impossible` node. **The witness is driven as a NON-PV DESCENDANT, never as a root**: the overload return is `!is_pv`-gated and ply 0 is always a PV node, so as a root the mutant does not fire at all and survives. Revision 5 changed this mutation's class and did not re-read the gate it then leaned on |
| M7 | Tier T qualifies at ≥3 for the mover (option A) | informative | survival is a recorded finding under **U3** §6.5's second branch, with a diagnosis, per D-281 |
| **M8** | **`visit` drops the last candidate after generation** — D-124's own reproducer, `if cells.len() > 1 { cells.pop(); }` | **the `assert!`** | **A FILTERED root**, where `forced` is the whole set and `beta = INFINITY` guarantees the loop exhausts. Revision 4 registered no mutation for the `assert!` half at all, while §8.4 opened by quoting D-295's finding that asserting an instrument's strength rather than measuring it is the defect. Registered because the honest reading is that on the 70.8 % BATCHED population `forced == 0` and the assertion is VACUOUS there — it earns its place only on forced rows, and the mutation is what shows which |

### 8.5 Floors, not printed counts

Revision 1 printed the count of positions in neither class "so the gate cannot
silently become vacuous". Printing is not a criterion: the defect it names —
classifying nothing — **preserves** it. Revision 2 registers a per-class floor
that names its witness positions, so a class whose members all disappear turns
the gate RED rather than green with a large number beside it.

### 8.6 REJ-DEPTHPROOF, stated where it belongs

Every claim this instrument makes is bounded-depth with no zone argument and is
therefore EVIDENCE and never PROOF. `REJ-DEPTHPROOF` binds us as it binds the
community. Revision 1 asserted the test plan said this; it contained no occurrence of
`proof`, `evidence` or `DEPTHPROOF`, and §8 cited neither.

### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**

> **THE DEFECT, kept because the wiring below is only legible against it.** B3 of
> the revision-7 review: the superseded wiring sentence read "(a)–(d) plus S-E
> become one script", which named a component with no definition anywhere and
> listed S-E twice. MEASURED: `grep "(b)"` over the superseded document returns
> two hits, §4.2's matrix row `(b) INVERT both as declared lists` and **U2** §5.2's
> citation `D-257 (a)/(b)` — neither is a soundness gate. §8.3 was titled "The
> other three parts" and defined **(a)**, **(c)**, **(d)** plus one unlabelled
> config bullet. Revision 1 DID define it (`ec8f7fb:502`), and this is that line
> VERBATIM, corrected at u-rev 6 (**MINOR 9** of `wp15b_U4_REVIEW.md`, which found
> text presented as a verbatim quotation was a paraphrase): "(a) **the** tactical suite **at 100 % of its** pre-registered
> thresholds under Staged; **(b) a differential gate** against …",
> and its matrix was headed `| Option | (b)'s instrument |` — **so S-E *is* gate
> (b)'s instrument**, and "(a)–(d) plus S-E" counted it once as (b) and once as
> itself. Revision 2 deleted the enumeration and the matrix together, and five
> revisions then shipped a CI wiring sentence addressing a gate by a letter that
> named nothing.
>
> **REPAIRED by shape 2 (D-316): the letters are dropped and the four gates are
> named.** The double-list dies with the letters — the row was not listed beside the
> letters it was one of, because **the differential gate is named ONCE, in §8.2**. The
> two-shape comparison the selection was made from stands unedited in U4-Z, with the
> selection recorded beneath it.
>
> *(CORRECTED AT u-rev 8 — MAJOR 1 of `wp15b_U4_REVIEW_urev7.md`. Until u-rev 7 this
> sentence read "because **it IS the differential gate** and is named once", in the
> present tense and in this unit's own voice, of **S-E** — a row that FELL in M3 round
> 1 and has not been the differential gate's instrument since D-323 selected S-M. It
> is the same sentence MAJOR 3 of `wp15b_U4_REVIEW_urev6.md` charged at U4-Z's copy;
> u-rev 7 repaired that copy and did not reach this one. It is corrected IN PLACE, not
> annotated, because this block is carve prose — the unit edited inside this very
> blockquote at u-rev 6 under MINOR 9, four lines above — and the record discipline
> that leaves the two-shape comparison unedited does not reach it. **The paragraph
> above it, which reads "so S-E *is* gate (b)'s instrument", is REPORTED SPEECH about
> revision 1's own matrix heading and is left as it stands; it is marked here so a
> wording-independent sweep does not have to decide that twice.*)

**THESE FOUR — the tactical suite under Staged (§8.3), the differential gate
(§8.2: S-M — per-node EQUALITY of the emitted set against the LANDED referent R1,
REUSED not rewritten, D-323, marked DEPENDS-OPEN-THEORY under D-321), the colony
family (§8.3) and the
pattern fixtures under Staged (§8.3) — become one script**,
`tools/staged_soundness_check.sh`, added to `tools/ci.sh`. **Four parts, four
names, each defined in exactly one place, and the script's own enumeration is
this sentence's.** A `tools/` change: reviewed against `tools/SHELL_CHECKLIST.md`
with every item answered by name, carrying the coverage rule's test driving the
shipped script, and distinguishing RUN VOID from FAIL by name (item 12) with a
scratch preflight.

**FOLDED AT u-rev 6 — WHAT THIS SENTENCE SAID AND WHAT IT CAN AND CANNOT SPECIFY
NOW.** From u-rev 2 to u-rev 5 the differential gate above read *"S-E with the reduced
S-C beside it"*; **S-E fell in M3 round 1 and the composite that carried the reduced
S-C (S-J) fell in round 2**, and the instrument **S-M SELECTED at `af8082a` (taken at
`809b5db`)** is the one this sentence now names (D-323) — the convention MINOR 7 of
`wp15b_U4_REVIEW_urev6.md` settled, applied here at u-rev 8 to the one live site its
repair had not reached (**MINOR 6**, `wp15b_U4_REVIEW_urev7.md`). This paragraph is the sentence `tools/staged_soundness_check.sh`'s own
enumeration is taken from, so the retarget is not cosmetic. **THREE OF THE FOUR PARTS
CAN BE SPECIFIED AND ONE CANNOT:** the differential gate's CRITERION is selected, but
the SEAM by which a test observes the emitted set is a separate named decision D-323
leaves OPEN (D-115, round-1 F4), so **the script's differential part cannot be written
until that decision is made.** The other three named gates are unaffected, which is
what D-316's naming was for.

---

## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**

> ## **SELECTED — N-E, AND IT IS NOT THE ROW THE MATRIX RECOMMENDED (D-329)**
>
> **THE OPTION IS N-E: a REQUIRED `--config PATH`, NO DEFAULT, with a NEW WHOLE-PATH
> GUARD.** Field: `docs/experiments/matrix_M4_axisA_round4.md` — round 4, **scoped to
> axis A alone with N-Q authored into it**, which is the disposition D-324's own stop
> clause authorised; three rows, {N-E, N-M, N-Q}; authored `7866bcf` by the session
> dispatching WP-1.5b design closure, **under D-328 (R11): its author wrote NO MEASURED
> CELL and every one was produced by a separate measurement agent with no stake.**
> Attack: `docs/experiments/matrix_M4_axisA_REDTEAM.md`, fresh context, at `7e0a328` —
> thirteen findings, **nine of ten facts reproducing**. Selection record, with the
> ladder run in full: `docs/experiments/matrix_M4_axisA_selection.md` (**selected at
> `7e0a328`**, the revision carrying the attack, **landed at `d56a898`**). **ADR line:
> D-329.**
>
> **AXIS B IS NOT REOPENED.** D-324 records that axis B's flip clause has already FIRED
> toward **N-K**, measured at 8 added lines, 0 removed, ~2× wall (2 × 33 s), and D-329
> states that **N-K COMPOSES with the naming mechanism selected here rather than
> rivalling it**. This round decided axis A and nothing else.
>
> **THE MATRIX RECOMMENDED N-Q AND ITS OWN RED TEAM KILLED THAT RECOMMENDATION.** The
> ground round 4 selected on — hard rule 1's *"one schema home"* — is a rule about
> **DEFAULTS**, and **MEASURED, none of the three rows has one**: all three refuse an
> absent `--config` by name at exit 1. The round-3 red team had already ruled that
> clause *"a WOUND, not a KILL"* for exactly that reason, and `configs/` is not one
> schema — it holds four engine configs, six arena match configs and a weights table,
> with the engine exiting 2 on two of three sampled. **So RUNG (a) OF THE LADDER IS
> SILENT ACROSS THE WHOLE FIELD**, and this unit does not repeat the reading that made
> it look decisive.
>
> **N-M IS ELIMINATED BEFORE THE LADDER RUNS, AND NOT BY IT.** Red-team F10, MEASURED:
> `docs/experiments/wp15b_sprt_prereg.md` §7A.2 registers this script as DOUBT 2's
> instrument and names the invocation `tools/baseline_snapshot.sh --config
> configs/gate_v0.toml`, and §10 registers that document's flip *"if
> `tools/baseline_snapshot.sh` lands `--config` in a shape the §7A.2 criterion cannot
> be taken under."* Driven against all three rows, N-E and N-Q take that invocation at
> exit 0 and **N-M refuses it at exit 1** — ``--config takes `instrument` or `staged`,
> got `configs/gate_v0.toml` ``. **That is an elimination on REGISTERED GROUND, not a
> tiebreak preference**, and the matrix never states it.
>
> **THE SELECTION IS TAKEN AT RUNG (b) — fewest MEASURED added lines — and it is not
> close.** N-E 22 added / **7 CODE** against N-Q's 32 / 12; both owe the same **4**
> whole-path guard lines; N-Q owes **5** containment lines on top; the item-10 driving
> tests owed are **2 classes** for N-E against 5 classes plus an unpinned normalisation
> for N-Q. Every cell was produced by a stakeless measurer under D-328 and
> independently re-derived by the attacker, which reproduced all twelve cells of the
> line-count table exactly. **Rung (c), recorded-arbitrary, is not reached.**
>
> **AND THREE MEASURED FINDINGS MAKE RUNG (b)'s ANSWER RIGHT ON SUBSTANCE RATHER THAN
> MERELY CHEAP.** *(i)* **N-Q's own selling property is FALSE (F4):** `.gitignore:7` is
> `*.bin` under rule 8, so `configs/ghost_v0.bin` is invisible to `git status
> --porcelain`, passes containment, and reaches the invariant block at **exit 0** beside
> a revision at which no commit contains it — containment bounds by DIRECTORY, never by
> COMMIT. *(ii)* **N-Q is the only row that ADDS a defect (F5):** `ROOT` is bash's
> logical `pwd` while `realpath -m` is physical, so through a symlinked checkout N-Q
> refuses the repository's own `configs/instrument_v0.toml` on an invocation N-E
> completes — a VOID reported as a FAIL, `SHELL_CHECKLIST` items 8 and 12. *(iii)*
> **N-Q's extra lines are required by no rule in this tree (F13):** item 11's scope is
> *"any binding consumed by `rm`, `mv`, or a write"* and `$CONFIG` is a READ, so item 9
> governs it and is discharged by the whole-path guard **both** rows owe.
>
> **THE FOUR CONDITIONS THAT RIDE WITH THIS SELECTION BIND, and none of them was costed
> for any row:**
>
> 1. **THE `config` LINE'S DIGEST IS `$3`, NOT `$4`.** That line is `config <path>
>    <sha>` — three fields. The four-token reasoning belongs to the differently shaped
>    `corpus` line. Any future guard, test or reader of the `config` line uses `$3` and
>    re-derives the index for the line it is actually reading.
> 2. **N-E's WHOLE-PATH GUARD MAY NOT BE SPELLED AS A REUSE OF THE LINE-289 BASENAME
>    LOOP.** MEASURED twice — by the measurer and again by the attacker — that spelling
>    leaves `configs/spaced dir/instrument_v0.toml` reaching the record at exit 0.
> 3. **AN ITEM-10 DRIVING TEST IS OWED for both new refusal classes**, in two halves
>    with a control, per the coverage rule. The precedent is **MEASURED** — the mark is
>    added at u-rev 8, MINOR 7 of `wp15b_U4_REVIEW_urev7.md`, which found the figure
>    unmarked at two sites and measurable in one command:
>
>    ```
>    $ git show b067d47 --numstat -- crates/pistol-cli/tests/baseline_snapshot_tests.rs tools/baseline_snapshot.sh
>    91	0	crates/pistol-cli/tests/baseline_snapshot_tests.rs
>    54	4	tools/baseline_snapshot.sh
>    ```
>
>    **MEASURED 91 test lines for ONE guard arm at `b067d47`.**
> 4. **AN ITEM-12 SENTENCE IS OWED** in the usage block saying that a config refusal is
>    a **FAIL**, this script having declared no void class.
>
> **WHAT D-329 SUPERSEDES IN D-324, stated because this unit carried D-324's version:**
> D-324's remedy *"closes with THREE LINES copying the guard already at
> `tools/baseline_snapshot.sh:289`"* is **wrong in COUNT — four, measured at
> `b067d47`** — **and wrong in KIND: the line-289 loop guards a BASENAME while the
> record writes a WHOLE PATH**, so it is a new guard and not a copy. D-329 also corrects
> D-324's framing of the missing `configs/instrument_staged_v0.toml` as an N-M cost: it
> **blocks all three rows equally**.
>
> **THE STRONGEST SURVIVING ATTACK AGAINST N-E IS ASSEMBLED, NOT QUOTED, AND THAT IS
> ITSELF A RECORDED RESIDUAL.** The red team was dispatched to break N-Q and the tied
> set's interaction with it; it recommends N-E and **was never asked to break N-E**. The
> selection record assembles the paragraph from the round's own measured findings and
> records the difference rather than smoothing it:
>
> > N-E is selected because it is the cheapest row that adds no defect — not because it
> > delivers what the record's `config` line exists for. That line is provenance: a
> > reader re-runs the run from it. N-E bounds the admissible set nowhere, so a record
> > may name a document outside the repository that no other reader can obtain, and its
> > provenance rests on caller discipline plus a digest of bytes nobody else holds. The
> > round measured that the one row which tried to fix this does not fix it either — a
> > gitignored file inside `configs/` reaches the invariant block at exit 0 — so the
> > weakness is not resolved by this selection, only left where it was. N-E therefore
> > ships a seam whose provenance guarantee is exactly as strong as the caller, and this
> > round chose it knowing that and having measured that the alternative bought nothing
> > for its five extra lines except a new false refusal.
>
> *(COMPLETED AT u-rev 8 — MINOR 8 of `wp15b_U4_REVIEW_urev7.md`. Until u-rev 7 this
> quotation stopped at "only left where it was" and dropped the record's closing
> sentence with no elision mark, while D-323's fold in this same document marks its
> elisions with `…`. The dropped sentence is the ROUND'S OWN DEFENCE, so the omission
> ran in the attack's favour and softened nothing — which is why the finding is MINOR
> and why the remedy is to quote it rather than to mark the gap. Text presented as a
> verbatim quotation should be one.)*
>
> **RESIDUAL, OPEN AND THE ARCHITECT'S: N-E HAS NOT BEEN ATTACKED BY A FRESH-CONTEXT
> DECISION-RED-TEAM IN ITS OWN RIGHT.** The matrix law is satisfied for the FIELD — a
> matrix was authored and attacked before selection — and D-329 claims no more than
> that. It is in U4-Z's OPEN list.
>
> **A SELECTION IS NOT AN IMPLEMENTATION, AND THIS UNIT MAY NOT WRITE AS THOUGH IT
> WERE.** No line of N-E is written; `tools/baseline_snapshot.sh` still has no
> `--config` flag; the four conditions above are unpaid; and **`configs/instrument_staged_v0.toml`
> STILL DOES NOT EXIST** (MEASURED, output pasted at U4-M item 1), which blocks the
> snapshot's AFTER independently of the seam. **What u-rev 6 recorded as blocked ON THE
> SEAM is now blocked on the seam's IMPLEMENTATION**, which is a narrower and different
> statement, and U4-Z carries it that way.
>
> **AND ONE PROCESS LINE CAME OUT OF THIS ROUND, recorded because it bears on every
> number this unit registers: D-330.** D-328's flip clause fired on the very first round
> it governed. Round 4's own headline FACT 5 pasted *"4th token a reader would take as
> the digest"* for a line that has three fields; the author wrote no cell and the
> measurer had no stake, and **both were reading the same mislabelled probe.** What
> caught it was the third mechanism — **a fresh-context adversary RE-RUNNING the inline
> evidence rather than reading it (R7)**. D-330 amends D-328 to a three-part rule and
> records that the split is NECESSARY AND NOT SUFFICIENT. The concrete lesson: **a field
> index is not portable between record lines of different shapes.**

> ## THE RECORD OF THE EARLIER STATES OF §9 — kept, STAMPED, and SUPERSEDED IN PART AT u-rev 7
>
> **WHAT IS RECORD BELOW AND WHAT IS NOT.** RECORD, true at the revision that wrote it:
> the T1' recovery and its two DIFFs; the recovered matrix and §9.1's five amendments;
> the u-rev 5 update (D-318); and the u-rev 6 update (D-324) **in everything it says
> about rounds 1–3** — which stop each was, the two orthogonal axes, N-K's fired flip,
> the author's-own-defects paragraph, and the two shipped-instrument defects as they
> stood then. **NOT RECORD, and superseded by the block above:** every sentence below
> that says nothing is selected on axis A, that no ADR line may cite N-E, that the guard
> residue closes with three lines, or that item 15 and B2 are blocked because no seam is
> selected. Each such sentence carries a u-rev 7 mark at its own site; none is deleted,
> because the ROUNDS are the record and the record is what D-329 was selected from.

> ## THE RECOVERY (T1'), THE DIFF, AND WHY THE SELECTION DOES NOT STAND
>
> **The recovery.** B1 found §9 contains no matrix. Unlike M2 and M3, M4's does
> exist and holds the adopted option — MEASURED, `N-A` occurs at `ec8f7fb` and
> the matrix there recommends it — so T1' makes M4 a MECHANICAL RECOVERY. The
> text below the fold is `ec8f7fb:558–578`, recovered verbatim, sha256 prefix
> `5e8c5e4a1e7ad416`.
>
> ### DIFF 1 — recovered text vs. the text the DECISION-RED-TEAM attacked
>
> **IDENTICAL — and identical BY CONSTRUCTION, which is why it is reported and
> not relied on.** The five-matrix DECISION-RED-TEAM round ran against revision 1,
> `ec8f7fb` (superseded §16, row 1). Recovering from `ec8f7fb` and comparing to
> what was attacked at `ec8f7fb` reads one blob twice. **A criterion the defect
> class preserves is not a criterion** — CLAUDE.md's own words — and "did the
> recovery corrupt the bytes" is preserved by every defect that matters here.
> Stated so a reader does not mistake it for a check that passed.
>
> ### DIFF 2 — recovered text vs. an EXTERNALLY DERIVED referent
>
> The referent that does not share the suspect input: **the design's own §9 at
> `6feb40a`**, five amendments produced by measurements taken AFTER the attack, by
> a process that is not the matrix. `diff -u` is non-empty, and three of the four
> differences are MEASURED falsifications of matrix cells:
>
> | # | Recovered cell | The design's §9 says | Mark |
> |---|---|---|---|
> | 1 | N-A failure modes: "the flag is the **fourth of its exact kind**", and "the `argument` helper already refuses an empty value" | Amendment 2: *"The fourth flag of its exact kind" is withdrawn* — `--corpus` reaches the record through `$(basename …)` while `--config` would reach it as a whole path on TWO invariant lines; **four guards are owed and the `argument` helper is none of them** | MEASURED |
> | 2 | N-B failure modes: "**breaks the D-209 instrument golden transcripts**" | Amendment 3: **false** — `grep -c instrument_v0` on that fixture is **0**; the golden is taken at `configs/gate_v0.toml` | MEASURED |
> | 3 | N-A cost: "**MEASURED** one snapshot run costs **34.0 s**" | Amendment 4: N-A **is** a change to the instrument, so BEFORE is re-taken under the amended script — **MEASURED 34.5 s** | MEASURED |
> | 4 | Preamble: the pinned triple `depth_at_500ms` 2 / 2 / 1 "reproduces exactly" — the quantity the matrix is about | Amendment 1: `depth_at_500ms` sits **32 lines below the `# timing` marker** whose text reads *excluded from every comparison*, is demoted to CONTEXT, and the registered quantity becomes per-position `depth_turns` and `nodes` | MEASURED |
>
> Difference 1 removes N-A's mitigation; difference 2 removes a ground on which
> the matrix REJECTED a rival; difference 4 moves the matrix's SUBJECT.
>
> ### VERDICT: **DIFFERS. SELECTION OPEN. The carve does not select N-A.**
>
> *(u-rev 1's verdict, and it STANDS as to N-A: N-A was never selected and is not what
> D-329 selects. **"SELECTION OPEN" is superseded at u-rev 7** — axis A selects **N-E**,
> a required path with no default, which is a different row from N-A's optional one.)*
>
> **UPDATE AT u-rev 5 — THE FRESH ROUNDS RAN, TWICE, AND STILL SELECT NOTHING
> (D-318).** `docs/experiments/matrix_M4_snapshot_config_seam.md` was authored
> fresh (revision 1, `77f7397`), attacked, re-authored over a ten-option field
> (revision 2, `cb16f7c`) and attacked again. Options SURVIVED both rounds, so
> this is not M3's "every option fell" stop — it stops for three other reasons,
> each sufficient. **(i)** Revision 2's stated reason for existing is FALSE: it
> claimed D-252's matrix selected the document seam and D-283 landed it, and
> **D-288 exists in this tree solely to relabel D-252's option (c) "DEFERRED …
> NO OPTION SELECTED"**, warning against a successor who reads it as adopted and
> finds "the matrix already spent, and no red-team owed, on a choice that had
> never been attacked". **(ii)** Every precedent ground is therefore void — the
> tree holds NO attacked selection for how an instrument binds a per-run input,
> since D-252 selected nothing and D-283 states its own choices were never attacked —
> leaving one ground that argues equally for five options and discriminates between
> none. *(A third conjunct stood here until u-rev 6 — "and D-316's residual says the
> same of B3's shape comparison" — and is DELETED: **MINOR 10** of
> `wp15b_U4_REVIEW.md`. D-316 is about whether the soundness gate's four parts are
> addressed by letters or by names; it is not a selection about how an instrument binds
> a per-run input, so it did not support the claim it was placed inside. It was the
> author's own addition and appears in neither M4 red-team report. The stop stands on
> the two conjuncts that are R1's.)*
> **(iii)** The field is STILL incomplete at revision 2: a required
> `--config {instrument|staged}` **closed-enum selector** — rule 1's own mechanism,
> which the recommendation invokes — dominates the recommended option on the
> matrix's own guard trigger and was excluded by the framing "lets a caller name
> the path" for two revisions running.
>
> **So B2 is not discharged and the reason is now measured rather than
> procedural.** No ADR line may cite N-A, N-A′ or N-E as adopted. Item 15 in U4-Z
> stays blocked, and `tools/baseline_snapshot.sh` has no config seam, so the
> registered above-marker quantity has a BEFORE and no AFTER. *(u-rev 5's list. The
> u-rev 6 update immediately below EXTENDS it to N-K, N-M, N-L and N-Q, which the
> third round's field added, and adds a second, independent blocker for the AFTER.
> **SUPERSEDED IN PART AT u-rev 7 as to N-E**: a fourth round was authored and attacked
> and **D-329 cites N-E as adopted**, so the prohibition survives only for the rows
> that round did not select. `tools/baseline_snapshot.sh` still has no config seam
> BUILT, which is why the AFTER is still missing.)*
>
> **UPDATE AT u-rev 6 — A THIRD ROUND RAN AND STOPPED AGAIN, NOTHING IS SELECTED
> (D-324), AND THIS SLOT STAYS `SELECTION OPEN`.** *(**SUPERSEDED AT u-rev 7 as to the
> slot**: the fourth round D-324's own stop clause authorised has since run, and axis A
> selects **N-E** (D-329, the block at the head of §9). Everything this update states
> about round 3 itself is RECORD and stands.)* Revision 3 is
> `docs/experiments/matrix_M4_snapshot_config_seam_rev3.md` (authored `9ce863f` under
> R9, by a session that authored neither prior revision); its attack is
> `docs/experiments/matrix_M4_REDTEAM_round3.md`; the stop record, with the tie stated
> in full, is `docs/experiments/matrix_M4_stop_round3.md`. **THREE authored revisions,
> THREE fresh-context DECISION-RED-TEAMs, NO SELECTION.**
>
> **WHICH STOP THIS IS, because it is not M3's round-1 stop and the difference is the
> point.** It is the **RECORDED-TIE** stop and **not** the every-option-fell stop:
> **N-E, N-J, N-K, N-F and N-L all survive**, and the ground rounds 2 and 3 both aimed
> at N-E and N-J is now MEASURED away rather than argued away. The round was
> pre-authorised to reach this disposition: *if no ground discriminates after the
> field, the TIE ITSELF IS THE FINDING* — record which options are equivalent and
> under what measurement they would separate, stop, and let the architect select on
> the recorded tie or order the measurement.
>
> **THE CAUSE, which is the round's real finding and what three revisions of argument
> were unknowingly about: THE FIELD HAS TWO ORTHOGONAL AXES AND HAS BEEN SCORED AS
> ONE.**
>
> - **AXIS A — how the config is NAMED:** N-A′ (optional path), N-E (required path),
>   N-M (closed-enum token), N-N (through the fixture), **N-Q (containment-guarded
>   path — MISSING)**, N-G (environment), N-L (a literal, re-pinned by ADR).
> - **AXIS B — how many RECORDS one invocation produces:** N-K (two configs, two
>   records, one invocation) against everything else's one.
>
> **N-K is not a rival of N-E, N-M, N-Q or N-L; it COMPOSES with one of them** — the
> red team's own strongest-attack line, that N-K *"owes on its second config exactly
> the caller-named-path obligation N-E owes on its first"*. A matrix that puts a
> two-record MODE in the same column as three naming MECHANISMS cannot select, because
> its rows are not alternatives. **That is why every round has found its recommendation
> resting on a ground that argues equally for four or five rows: the rows were never
> competing for the same slot.**
>
> **N-K'S FLIP CLAUSE HAS FIRED, AND IT SETTLES AXIS B ONLY.** Three revisions declined
> to recommend N-K solely because its cost was unmeasured, and revision 3 registered
> the clause *"The measurement of N-K's cost is taken and is small. Remedy: flip to
> N-K"* **before** the measurement, which is the whole point of registering it. The red
> team then measured it: **8 added lines, 0 removed, no re-indentation, two complete
> records at two policies from one invocation, ~2× wall (2 × 33 s on the registered
> corpus)**. The clause fires. **It does not settle axis A**, because N-K still has to
> name its two configs somehow.
>
> **ON AXIS A NOTHING DISCRIMINATES N-E FROM N-M FROM N-Q.** The one differential
> ground revision 3 offered — the guard surface — is **MEASURED away**: D-232's newline
> line-injection class is already refused on the config path by `digest()`'s hex-shape
> check, reproduced at exit 1 with a named refusal, and the residue (TAB, ESC, U+2028)
> closes with **three lines** copying the guard already at
> `tools/baseline_snapshot.sh:289`. What survives is a **JUDGEMENT and not a
> measurement**: whether a token→path map in a script satisfies hard rule 1's MECHANISM
> as well as its VOCABULARY, given that `Budget` is a closed enum of KINDS whose values
> live in the schema while N-M is a closed enum of VALUES living in the script. **N-Q
> IS THE MISSING ROW ON AXIS A** — `--config PATH` refused unless it resolves under
> `configs/` — absent from all three revisions and all three attacks, dominating N-M on
> N-M's own failure mode and N-E on guard surface, and it is `tools/SHELL_CHECKLIST.md`
> **ITEM 11** under this project's own name, in a round bound to that checklist that
> answered no item of it by name.
>
> *(**THIS PARAGRAPH IS SUPERSEDED AT u-rev 7 IN THREE PLACES, and each was a MEASURED
> claim that a later measurement falsified — they are marked rather than deleted,
> because D-329 was selected against this text.** **(1) The tie is broken.** Round 4
> ran on axis A alone; **N-M was eliminated on registered ground** — `wp15b_sprt_prereg.md`
> §7A.2 registers `--config configs/gate_v0.toml` and N-M refuses it at exit 1 — and
> rung (b) then separated N-E from N-Q on MEASURED added lines. The judgement this
> paragraph leaves standing was therefore never reached: **N-M did not survive to be
> judged.** **(2) The "three lines copying the guard at `tools/baseline_snapshot.sh:289`"
> remedy is SUPERSEDED by D-329, in COUNT and in KIND** — four lines measured at
> `b067d47`, and the line-289 loop guards a **BASENAME** while the record's `config`
> line writes a **WHOLE PATH**, so copying it leaves `configs/spaced dir/instrument_v0.toml`
> reaching the record at exit 0. It is a NEW guard, and N-E and N-Q owe the same four
> lines of it. **(3) N-Q is no longer "missing", and the two claims made for it here are
> both MEASURED false by round 4:** it does not dominate N-E on guard surface — both owe
> the identical whole-path guard — and it is **not `SHELL_CHECKLIST` item 11**, whose
> scope is a binding consumed by `rm`, `mv` or a write, where `$CONFIG` is a READ
> governed by item 9 and discharged by the guard both rows owe. N-Q was authored into
> round 4, recommended by it, and its recommendation was killed by that round's own red
> team.)*
>
> **THIS UNIT SELECTS NOTHING FOR §9 AND MAY NOT WRITE AS THOUGH ANYTHING WERE
> SELECTED.** The architect either selects on the recorded tie — N-K on axis B plus one
> of {N-E with three guard lines, N-M, N-Q} on axis A — or orders a fourth round scoped
> to axis A ALONE with N-Q authored into it. **No ADR line may cite N-A, N-A′, N-E,
> N-K, N-M, N-L or N-Q as adopted.**
>
> *(**SUPERSEDED AT u-rev 7.** The second of the two dispositions this paragraph offered
> is the one that was taken: **a fourth round scoped to axis A alone, with N-Q authored
> into it**, at `7866bcf`, attacked at `7e0a328`. **D-329 cites N-E as adopted**, so the
> prohibition no longer binds for N-E; it still binds for N-A, N-A′, N-M, N-L and N-Q,
> none of which any ADR line adopts. N-K is not adopted either — D-324 records its flip
> clause as fired and D-329 records that it COMPOSES with N-E rather than rivalling it,
> and neither is a selection of N-K. **This unit still selects nothing itself**; it
> carries what D-329 selected.)*
>
> **WHAT STAYS BLOCKED, unchanged from D-318 and now for a third round.**
> `tools/baseline_snapshot.sh` still has no config seam, so no Staged snapshot can be
> taken and the registered above-marker quantity still has a BEFORE and no AFTER.
> **U4-Z item 15 stays blocked and B2 — M4 has no ADR line — stays open.** **AND
> INDEPENDENTLY OF THE SEAM, THE AFTER IS BLOCKED ON
> `configs/instrument_staged_v0.toml`, WHICH DOES NOT EXIST** (MEASURED, revision 3
> fact 6; re-taken at u-rev 6 with the `ls` output pasted at U4-M item 1) **and which
> no row of any revision produces.**
>
> *(**RE-DERIVED AT u-rev 7, and two of these three change while the practical state
> does not.** **(a) The BLOCKAGE'S GROUND is narrower.** The seam is no longer
> unselected — it is **SELECTED AND NOT BUILT** (N-E, D-329). `tools/baseline_snapshot.sh`
> still resolves its config from the literal `CONFIG="configs/instrument_v0.toml"` at
> `tools/baseline_snapshot.sh:182` and takes no `--config` argument of its own, so no
> Staged snapshot can still be taken — but what is owed is now IMPL against a named
> option plus its four conditions, not another matrix round. **(b) B2's reason is gone**: an ADR line for M4
> exists and it is D-329 — see U4-Z, where B2 is re-derived. **(c) The
> `configs/instrument_staged_v0.toml` blocker is UNCHANGED and re-MEASURED at u-rev 7**
> (output at U4-M item 1), and D-329 corrects D-324's framing of it: it is not an N-M
> cost, it **blocks all three axis-A rows equally**, and no row of round 4 produces it
> either.)*
>
> **TWO SHIPPED-INSTRUMENT DEFECTS the round found in passing, owned by no row and
> OPEN** — `tools/` findings, not matrix questions: a SPACE in a caller-named
> `--corpus` path reaches the record unescaped at exit 0 under the COMPLETE kind token,
> breaking the record's own leading-tokens parse rule and shifting the digest field;
> and `crates/pistol-cli/src/report.rs:151-162` rewrites control characters to `?` in
> the engine's handshake while the script writes its own copy raw, so one record's two
> config lines can disagree for a reason that has nothing to do with the config.
>
> *(**BOTH ARE CLOSED AT u-rev 7.** The `--corpus` space is fixed at **`b067d47`** —
> which also swaps `basename` for `${X##*/}`, and which is why round 4's measurements
> are pinned there — and the control-character canonicalisation at **`a102c6a`**, where
> the engine now refuses a config path its handshake cannot echo verbatim. A
> fresh-context **REVIEW-impl covering both as one defect class PASSED at `84ff8d7`**,
> 0 BLOCKING / 0 MAJOR / **3 MINOR**, on mutation-checked controls — it restored the
> pre-fix sources and re-ran both tests, then mutated both guards to refuse everything
> and confirmed each control half kills the mutant. **THE THREE MINORS (F1, F2, F3) ARE
> OPEN** and were deliberately not repaired in that commit, because an amendment
> reopens the review that passed; they are in U4-Z's OPEN list.)*
>
> **AND THE ROUND RECORDS ITS OWN AUTHOR'S DEFECTS, first-person**, because D-318 had
> already ruled that the recurrence and not the instance is the finding: fact 4's
> "verbatim output" was truncated by one line — killing, under R7, the only MEASURED
> support in the recommended row's cost cell — fact 1's READING was false, fact 7's
> paste was elided toward the thesis, and the row that session added on its own
> measurement falls with it. **Three distinct authoring sessions have now produced this
> pattern in one work package.**
>
> *(THE PARAGRAPH THAT STOOD HERE UNTIL u-rev 6 IS DELETED: "T1' is explicit —
> 'identical = attack stands, differs = fresh round'. It differs. **The fresh
> DECISION-RED-TEAM is the architect's dispatch, not the carve's**, and until it runs
> no ADR line may cite N-A as adopted …". It ran, three times, per everything above it;
> it was left in place BENEATH the u-rev 5 update that said so, which is **BLOCKING 1**
> of `wp15b_U4_REVIEW.md` — a fold appended without re-reading what it superseded.)*
>
> <details><summary>THE RECOVERED MATRIX, verbatim from <code>ec8f7fb:558–578</code>
> — a record of what was attacked, NOT a selection</summary>
>
> The brief requires `tools/baseline_snapshot.sh` before and after, against pinned
> operator numbers (`depth_at_500ms` opening 2 / early_mid 2 / late_mid 1 at
> `050961d`). **MEASURED at `f317385` on this machine, BEFORE run: opening 2,
> early_mid 2, late_mid 1 — the pinned triple reproduces exactly.** The script's
> `CONFIG` is a literal, `configs/instrument_v0.toml`, with no flag.
>
> | Option | What it does | Cost | Failure modes |
> |---|---|---|---|
> | **N-A — add `--config PATH`** | A workload-scope flag beside `--corpus`, `--ladder-depth` and `--binary`. The budget line still says `registered`; the record already carries `config <path> <sha>` and `engine_id config <path>` ABOVE the timing marker, so two records taken under two configs are already distinguishable and cannot be diffed as one. | A `tools/` change: SHELL_CHECKLIST answered item by item, plus at least one test driving the shipped script (item 10). **MEASURED** one snapshot run costs 34.0 s wall on this machine. | It reopens a script whose review round closed recently. Mitigated by the shape: the flag is the fourth of its exact kind, the record's invariance claim is untouched because the config path was already inside the invariant block, and the argument parser's `argument` helper already refuses an empty value. |
> | **N-B — flip `configs/instrument_v0.toml` to staged** | No tools change; the standing instrument measures Staged by construction. | Zero. | Lands the strength claim before its judge, against rule 6 and against D-190/D-194's own precedent; breaks the D-209 instrument golden transcripts; fires D-204's flip clause on this session's authority rather than the operator's. Rejected. |
> | **N-C — a scratchpad harness** | Measure Staged with a session-local script. | Zero repository change. | The number would come from an instrument with no governing revision, which CLAUDE.md's instrument clause exists to forbid, and it would not be comparable with the pinned operator triple because it is a different instrument. Rejected. |
> | **N-D — take no Staged snapshot** | Report the radius numbers only. | Zero. | The brief's required measurement is not taken and the WP's whole depth claim goes unmeasured. Rejected. |
>
> **RECOMMENDATION: N-A.** It is the only option that produces the required number
> from the registered instrument. The deltas it yields are ADVISORY per the session
> policy; the operator re-runs on their own hardware for the record.
>
> ---
>
> </details>

### 9.1 The five amendments the design made to N-A after that attack

Carried verbatim. They are what DIFF 2 is measured against, and they are the
reason the selection is open rather than the reason it is closed.


N-A (add `--config PATH`) remains the only option that produces a Staged number
from the registered instrument; N-B (flip the committed config), N-C (a scratchpad
harness) and N-D (measure nothing) are rejected. Five amendments.

**1. The registered quantity changes.** `timing depth_at_500ms` sits at lines
89/93/97 of a 97-line record — **32 lines below the `# timing` marker** whose own
emitted text reads *excluded from every comparison*. Its resolution, MEASURED
from this session's own BEFORE ladder:

| rung | to move UP one unit | to move DOWN one unit |
|---|---|---|
| opening (d2 102 ms, d3 9339 ms) | 18.7× faster | 4.9× slower |
| early_mid (d2 118 ms, d3 1340 ms) | 2.68× faster | 4.24× slower |
| late_mid (d1 30 ms, d2 982 ms) | **1.96× faster** | 16.7× slower |

and the reviewer measured the triple **unchanged at 2 / 2 / 1** under a deliberate
16-way load that stretched the same run from 34.5 s to 66.3 s. The agreement
revision 1 reported as a reproduction is invariant under a ~2× defect in the
quantity it is made of — a criterion its own defect class preserves.

**Revision 2 registers the ABOVE-MARKER quantity**: per-position `depth_turns`
and `nodes` at the registered 50 000-node budget, plus the `ladder … nodes`
counts. That is D-190's own mechanism statistic ("radius 2 completes a second
turn-iteration in 17 of the 24 bench positions"), it is inside the invariant
block, and it is byte-invariant by construction. `depth_at_500ms` is demoted to
below-marker CONTEXT and its dead band is stated so an unmoved triple is not read
as a null result.

**2. "The fourth flag of its exact kind" is withdrawn.** `--corpus` reaches the
record through `$(basename …)`; `--config` would reach it as a whole path on TWO
invariant lines. Four guards are owed and the `argument` helper is none of them:
caller-relative resolution (as `--out` and `--binary` each got), the printable
allow-list extended to the whole `$CONFIG` path, three named refusals
(directory / missing / not a regular file), and an assertion that the script's
`config` line and the engine's `engine_id config` line name the same document.
**SHELL_CHECKLIST items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve**,
answered by name in the IMPL commit.

**3. N-B's rejection loses a cost that does not exist.** "Breaks the D-209
instrument golden transcripts" is **false**: `grep -c instrument_v0` on that
fixture is **0**; the golden is taken at `configs/gate_v0.toml`. The real
exposure is `tactical_v0.txt`'s **15** `instrument_v0`-bound cases under D-204.
The rejection stands on its three surviving grounds — rule 6's judge, the
D-190/D-194 precedent, and D-204's flip being the operator's to fire.

**4. The instrument is named with its revision, and BEFORE is re-taken.** Revision
1 invoked the instrument clause against N-C without satisfying it for N-A:
`tools/baseline_snapshot.sh` was named twice and never with a revision. And N-A
**is** a change to that instrument, so the BEFORE run — taken under the
pre-`--config` script — is re-taken under the amended one. **MEASURED 34.5 s.**
Not worth an argument.

> **MARKED AT u-rev 7, AT THIS SITE AND NOT ONLY ELSEWHERE (MINOR 4 of
> `wp15b_U4_REVIEW_urev6.md`).** The amendment above is RECORD, and its
> **MEASURED 34.5 s is ATTRIBUTED TO AN INSTRUMENT THAT HAS NEVER EXISTED AT ANY
> COMMIT**: there is no amended `tools/baseline_snapshot.sh`, and there is none at
> HEAD. **The seconds are real; the attribution is not.** The BEFORE run was taken
> under the pre-`--config` script at **`f317385`**, which is the only script that
> exists. **U4-M item 1 carries the full note**; until u-rev 7 it sat a hundred lines
> below this text and pointed at it as though it were below, so a reader meeting this
> amendment first met an unqualified MEASURED figure with nothing against it.

**5. Replicate.** The run is 34.5 s and CLAUDE.md says a cheap doubt is answered
by replication, never by a margin defending a single sample. The below-marker
triple is taken three times.

---


---

### 11.6 One thing this WP does NOT close

D-295's residual — `RULE-EXACT`'s "never derived by weight algebra" is unpinned in
`src`, because no `HitBudget`-shaped fixture separates `t = 3` from `t = 4` — is
not closed here. D-295 names `blocking_covers` as the differently-shaped surface
that could close it, and this WP puts `blocking_covers` on the per-node path, so a
reader will ask. **S-M** — the selected instrument (§8.2, D-323; S-E until u-rev 6) —
exercises `blocking_covers` for its ANSWERS, not for its
arithmetic's exactness. Registered for WP-1.10 (item 8, **U2**).

---

## U4-T. The tests this unit registers

Carried from the superseded §11. The rows this unit does not own are in U2-T,
U3-T and `WPQ_seed.md`, and no row is in two places.

| Test | Watches |
|---|---|
| `gap_trap_answered_in_tier_f` | that `PAT-GAP`'s singleton gap cell is in the FORCED prefix from the defender's side |
| `colony_family_passes_under_staged` | the move played on ≥ 6 built distant-cluster positions |
| `tactical_suite_holds_at_its_rederived_thresholds_under_staged` | the `require` count of `tactical_staged_v0.txt`, at the **two tactical** staged configs — `tactical_staged_v0.toml` for fifteen cases and `gate_staged_v0.toml` for five |
| `staged_filtered_set_equals_the_minimal_cover_union` | **THE DIFFERENTIAL GATE'S CRITERION — AND IT IS NOW S-M, NOT S-E (D-323).** The emitted set at every FILTERED node, asserted **EQUAL** — not contained; containment is exactly what an over-generating mutation preserves, which §8.2 established and revision 6 then left this row registering — to the inclusion-minimal cover union computed by the **LANDED** referent **R1**, `crates/pistol-solver/tests/common/reference.rs`, **REUSED by a `#[path]` include and NOT rewritten**. *Until u-rev 6 this row read "the public generator's forced prefix against an independently written plan-family referent in pistol-search's own test tree", which was **S-E half one**; S-E FELL in M3 round 1 — its referent's independence and its greenness were in tension (F2) — and D-323 condition 1 now **FORBIDS** a second, freshly written referent for this criterion without a registered agreement criterion and a registered consequence for disagreement. This is a change of referent, not a re-labelling of S-E.* **THE ROW SHIPS MARKED `DEPENDS-OPEN-THEORY` (D-321)** — MEASURED, the two conventions differ on 22 of 174 FILTERED nodes, 12.6 % (§8, re-taken at u-rev 6) — **and `0 of 3406` may not be cited as evidence about the convention** (condition 2). **ITS SEAM IS OPEN:** how a test observes the emitted set is a separate named decision D-323 does not make, and D-115's constraint on widening `pistol_search::staged` (round-1 F4) applies to S-M as it applied to S-E |
| `visit_searches_every_forced_candidate` | **OPEN AT u-rev 7, AS AT u-rev 6 — NOT SELECTED AND NOT REJECTED, AND IMPL MAY NOT READ IT AS REGISTERED.** *(Re-checked at u-rev 7: D-329 decides the SNAPSHOT's config seam and touches nothing here, and no ADR line since D-323 carries this half.)* This was **S-E, half two**: the always-on `assert!` in `visit`, which is what sees a drop made AFTER generation — D-124's own reproducer, and §8.2's own words are that no test of the generator can see it. **S-E fell, and S-M is a criterion over the EMITTED SET ALONE**, so nothing in D-323 carries this half; the selection record's "what this selection does not decide" is where it lands. The gap is live rather than academic: M3 round 2's F1 measured that the recommended row's 72 % against D-124's `pop()` mutant is 0 % in the shape its own cost ground pays for, and §8.4 keeps **M8** registered against exactly this half. It is in U4-Z's OPEN list |

---

## U4-M. What this unit measures

ADVISORY on this machine; the operator re-runs for the record. (A standing
condition of every measurement in every unit, stated per unit so a unit is
readable alone; it is a condition, not a datum.)

1. **Snapshot BEFORE / AFTER. Registered quantity: per-position `depth_turns` and
   `nodes` at 50 000 nodes** (above the marker); `depth_at_500ms` reported as context
   with its dead band. **THE INSTRUMENT, NAMED WITH ITS REVISION — AND THIS IS THE
   REPAIR OF MAJOR 6.** Until u-rev 6 this item registered both runs *"both under the
   amended script"*. **There is no amended script, and there has never been one at any
   commit:** `tools/baseline_snapshot.sh` carries a literal
   `CONFIG="configs/instrument_v0.toml"` and no `--config` flag, at HEAD and at
   `f317385` — the commit this unit names as the one the BEFORE numbers were taken at.
   So:
   - **BEFORE** is taken under `tools/baseline_snapshot.sh` **at `f317385`**, the
     pre-`--config` script, which is the only script that exists. That is the
     instrument, and it now has a revision, which is what CLAUDE.md's instrument clause
     asks for and what "the amended script" could not supply.
   - **AFTER DOES NOT EXIST, AND IT IS BLOCKED TWICE OVER — AND AT u-rev 7 THE FIRST
     BLOCKER HAS CHANGED SHAPE WITHOUT LIFTING.** Once on M4's seam: at u-rev 6 there
     was no selected config seam after three rounds (D-324). **A fourth round has since
     run on axis A alone and N-E IS SELECTED (D-329)** — a required `--config PATH`, no
     default, with a new whole-path guard — **so the seam is now SELECTED AND NOT
     BUILT.** MEASURED at `8690ad6` and **RE-TAKEN at u-rev 8** because the script has
     moved since (`63eac4c`, architect ruling R19, one base for every caller-supplied
     path): `tools/baseline_snapshot.sh:182` still reads
     `CONFIG="configs/instrument_v0.toml"` and the script still takes no `--config`
     argument of its own. **THE FOUR `--config` OCCURRENCES ARE NOT ALL THE SAME THING,
     and u-rev 7 said they were** — MINOR 5 of `wp15b_U4_REVIEW_urev7.md`. Three are the
     ENGINE's flag on invocation lines; the fourth is inside a COMMENT, and it is the
     one line in the tree that binds the next `--config` change, so it is worth citing
     correctly in the unit that owes that change. RE-TAKEN:

     ```
     $ grep -n -- "--config" tools/baseline_snapshot.sh
     543:printf 'pistol\nquit\n' | timeout "$HANDSHAKE_TIMEOUT" "$BINARY" --config "$CONFIG" >"$WORK/hs" 2>/dev/null || HANDSHAKE_RC=$?
     559:	# caller-named and the guard above does not cover it. If a `--config` flag is
     592:timeout "$CORPUS_TIMEOUT" "$BINARY" --config "$CONFIG" <"$WORK/corpus.session" >"$WORK/corpus.out" || CORPUS_RC=$?
     659:		timeout "$LADDER_CAP_S" "$BINARY" --config "$CONFIG" >"$out" 2>/dev/null || rc=$?
     $ grep -n 'CONFIG="configs' tools/baseline_snapshot.sh
     182:CONFIG="configs/instrument_v0.toml"
     ```

     The comment at 559 says a `--config` flag would join the guard above it in the same
     commit, which STRENGTHENS this item's conclusion rather than weakening it: the
     script's own text records that the flag does not exist. Four conditions ride with the selection and
     none is paid. **So no Staged snapshot can be taken, for a narrower reason than at
     u-rev 6: what is owed is IMPL against a named option, not another matrix round.**
     And independently of the seam, on the config document itself — **MEASURED at
     `46c58ac` and RE-TAKEN at `8690ad6`, identical output:**

     ```
     $ ls configs/instrument_staged_v0.toml
     ls: cannot access 'configs/instrument_staged_v0.toml': No such file or directory
     ```

     No row of any M4 revision produces it, round 4's three included — **and D-329
     corrects D-324's framing of this blocker as an N-M cost: it blocks every axis-A
     row equally** (D-324, revision 3 fact 6; D-329).
   - **THEREFORE §9.1 AMENDMENT 4's "the BEFORE run … is re-taken under the amended one
     — MEASURED 34.5 s" ATTRIBUTES A REAL WALL TIME TO AN INSTRUMENT THAT DOES NOT
     EXIST.** The seconds are real; the attribution is not. The amendment is carried
     **ABOVE**, at §9.1, as RECORD, **and at u-rev 7 it carries a marked note at its own
     site as well as this one** — until then this sentence said "below", pointing a
     reader the wrong way past the amendment it was meant to qualify (**MINOR 4** of
     `wp15b_U4_REVIEW_urev6.md`). Both the amendment and this item are verbatim carries
     from `6feb40a`, so the defect is inherited rather than created — but the u-rev 5
     fold re-read §9 and did not re-read this item, which is BLOCKING 1's class.

The BEFORE numbers taken at `f317385` reproduce the pinned operator triple
(`depth_at_500ms` 2 / 2 / 1) — **and §9 amendment 1 establishes that the triple
sits BELOW the record's own "excluded from every comparison" marker with a dead
band of about 2×, so it is context and not the registered quantity.** No AFTER
exists: no engine code was written, **the config seam is SELECTED (N-E, D-329) and
NOT BUILT**, and `configs/instrument_staged_v0.toml` does not exist. *(Until u-rev 7
this sentence read "no config seam is selected after three rounds (D-324)", which the
fourth round falsified.)*

### Cost

| Item | DECLARED | MEASURED |
|---|---|---|
| One baseline snapshot | ~35 s | **34.0 / 34.5 s** |
| The soundness gate per CI run | **ESTIMATED 40–90 s — UNGROUNDED AT ITS DOMINANT TERM since u-rev 6, and still ungrounded at u-rev 7.** It was priced on **S-E's** one traversal per fixture plus the reduced S-C's **MEASURED 17.89 s**, and neither survives: S-E FELL in M3 round 1, and the composite row that carried the reduced S-C beside it (**S-J**) fell in round 2, inheriting every one of S-E's four kills. **D-323 selects S-M and explicitly does NOT decide the gate's corpus or its per-CI cost** — the selection record states that this 40–90 s figure is CARRIED from this table and is not that round's measurement, so it is not independent corroboration of itself. Revision 1's 60–180 s priced a workload that is days | **RE-DERIVED FOR S-M FIRST, then MEASURED when it lands, and reconciled here.** S-M's per-node cost is a walk against R1 reused by `#[path]` include, which is a different shape from S-E's per-fixture traversal; U4-Z carries the re-derivation as OPEN |

**The proportionality clause, and what it now says here.** The snapshot run is
34.5 s, so a doubt about it is answered by REPLICATION and by a SECOND INSTRUMENT
whose agreement criterion is registered before either runs — never by a margin
derived to defend one sample. §9 amendment 5 registers the replication (the
below-marker triple is taken three times). **It registers no second instrument,
no agreement criterion, no stage under doubt and no consequence for
disagreement** — U4-Z, OPEN. (D-307 rules WP-1.5b's SPRT run EXPENSIVE and lapses
the replication clause *for that run*; it does not reach this 34.5 s one, and
D-307 keeps the second-instrument duty in either case.)

---

## U4-Z. ADR lines this unit owes, the OPEN decision it stops on, and what is OPEN

> **RE-DERIVED IN FULL AT u-rev 7, NOT PATCHED.** This section is where the u-rev 6
> fold-in pass stopped: the head claimed the fold was *"re-run across the whole unit"*
> and then enumerated the sites it re-read, and U4-Z's B3 section was not among them —
> **BLOCKING 1** of `wp15b_U4_REVIEW_urev6.md`. So at u-rev 7 every claim below whose
> truth depends on an ADR line was re-read against `docs/decisions.md` as it now
> stands, item by item, rather than the two sentences the report names being edited.
> **Two landed lines were found disposing of residuals this section still carried as
> the architect's open choice: D-320 (B3's residual, landed at `0af32fb` — in the SAME
> COMMIT as the D-321 that u-rev 6 folded) and D-325 (D-316's false diagnosis, landed
> at `81180b8`, after u-rev 6).** The report raised the first; the second came out of
> the re-derivation and is the reason a re-derivation was the instruction rather than a
> patch.

### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.

**THE TWO-SHAPE COMPARISON BELOW IS THE TEXT THE ARCHITECT SELECTED FROM AND IS
LEFT UNEDITED**, on the same discipline the restructure matrix landed under: a
comparison corrected after the decision it fed is a comparison the decision was
never made against. Its "the carve does not choose" is the state AT SELECTION
TIME. The selection, and one MEASURED correction to a cost cell that execution
falsified, are recorded AFTER it, not inside it.

*(At selection time:)* The carve does not choose because both shapes are
coherent, and choosing between them is a design act.

**SHAPE 1 — RESTORE THE ENUMERATION, and put S-E inside (b).**
Reinstate revision 1's four-part bar verbatim from `ec8f7fb:502` — "(a) tactical
suite at pre-registered thresholds under Staged; **(b) a differential gate**
against full-width r2 at depths 1..=3 for mates and forced blocks; (c) a colony
fixture family of ≥ 6 cases; (d) the INTEG **U2** §5 pattern fixtures under Staged" —
and state that S-E, with the reduced S-C beside it, **is (b)'s instrument**, which
is what revision 1's own matrix heading said (`| Option | (b)'s instrument |`).
§8.7's wiring then reads "(a)–(d) become one script", which is revision 1's
sentence unchanged, and the double-list is gone because S-E is no longer listed
beside the letters it is one of.
*Cost, MEASURED:* four sentences reinstated; **zero** cross-references outside §8
change; §8.3's title "The other three parts" becomes true again, since (a), (c)
and (d) are the other three and (b) is §8.2's subject.
*What it keeps:* a lettering scheme that has already lost its own definition once,
in a document that then shipped four revisions without noticing.

**SHAPE 2 — DROP THE LETTERS, name the four gates by what they are.**
The gate is: the tactical suite under Staged; the differential half (S-E plus the
reduced S-C); the colony family; the pattern fixtures under Staged. §8.7 reads
"these four become one script". No component can go undefined, because none is
named by a letter that a later edit can delete.
*Cost, MEASURED:* **three** cross-references outside §8 retarget — `configs/tactical_staged_v0.toml`'s
"why" cell and §10's B5 paragraph, both **U3** §10, and item 15 in this unit's
list, which says "gate (a)". Inside §8, `(a)`, `(c)` and `(d)` become named
bullets.
*What it costs:* "§8.3(a)" is cited from three units and from the superseded
document's history; after shape 2 those citations name nothing.

**Neither shape is a repair the carve may make**, because each decides how the
gate is addressed from the other units, and the count of retargets is not the
argument — the argument is whether a lettered enumeration is worth having at all.

#### SELECTION — SHAPE 2, by architect ruling. Landed at u-rev 2. ADR line D-316.

**Selected: SHAPE 2.** The letters are dropped; the four gates are named by what
they are; §8.7's wiring enumerates the four names; **the double-list dies with the
letters, because the differential gate is named ONCE, in §8.2.** Executed in this unit
at §8.2, §8.3 and §8.7, and in **U3** §10 at the two sites named below.

**MARKED AT u-rev 7.** This block records the u-rev 2 EXECUTION, and at that execution
the gate named once in §8.2 was **S-E**. It read "because S-E **is** the differential gate" in the present
tense until u-rev 7 — **MAJOR 3** of `wp15b_U4_REVIEW_urev6.md`, which found it standing
unmarked six hundred lines after the head says S-M is selected, while its two sibling
sentences (§8.3's table cell and §8.7's wiring) were both retargeted at u-rev 6.
**S-E FELL in M3 round 1 and the differential gate's instrument since D-323 is S-M.**
It is corrected in place, not merely annotated, because this block is carve prose
recorded AFTER the comparison — the discipline that leaves the comparison unedited does
not reach it, which is the same reading applied to §8.7's copy under MINOR 9 below.
The shape-2 selection is untouched by the change: **it decided how the gate is
ADDRESSED, never which instrument it carries.**

**The ground, which is the option statement's own last clause read the way it
asks to be read:** the argument is not the retarget count, it is whether a
lettered enumeration is worth having. It measurably was not. `(b)` lost its
definition when revision 2 deleted the enumeration, and the document then shipped
FIVE further revisions wiring `(b)` into CI as a letter naming nothing, through
five REVIEW-design rounds that did not catch it. Shape 1 reinstates exactly the
scheme that failed that way and its own cost cell concedes the point — *"a
lettering scheme that has already lost its own definition once, in a document
that then shipped four revisions without noticing."* A name cannot go undefined
while remaining in the sentence; a letter can, and did.

**MINOR 9, RECORDED HERE AND NOT FIXED INSIDE THE COMPARISON, BECAUSE THE COMPARISON
IS LEFT UNEDITED.** Shape 1 above presents revision 1's bar as quoted *verbatim* from
`ec8f7fb:502` and renders it *"(a) tactical suite at pre-registered thresholds under
Staged"*. The actual text is *"(a) **the** tactical suite **at 100 % of its**
pre-registered thresholds under Staged"*. The line citation is correct and nothing
substantive turns on the difference, but text presented as a verbatim quotation should
be one, in a document whose whole discipline is that a carried quotation is unedited.
The correction is recorded AFTER the comparison, on the same discipline as the
cost-cell correction below: a comparison corrected after the decision it fed is a
comparison the decision was never made against. §8.7's copy of the same quotation IS
repaired in place, because that one is carve prose and not the selected-from text.

**MEASURED CORRECTION TO SHAPE 2's COST CELL, recorded because the cell was wrong
in the direction that favoured the option selected.** The cell says *three*
cross-references outside §8 retarget. Executing it, the count is **SIX**:

| # | Site | Was |
|---|---|---|
| 1 | **U3** §10, `configs/tactical_staged_v0.toml`'s "why" cell | "what **U4** §8.3(a)'s derivation requires" |
| 2 | **U3** §10, the B5 paragraph | "Revision 6's **U4** §8.3(a) said …" |
| 3 | This unit's §15 item 15 | "so gate (a) tests the threat mechanisms" |
| 4 | This unit's head, the B4 sentence | "**B4** (§8.3(a)'s …)" |
| 5 | This unit's U4-A lineage row | "§8.3(a)'s derivation was redesigned three times" |
| 6 | This unit's U4-Z lead-in | "item 15 on §8.3(a)'s derivation" |

**THE DIAGNOSIS THIS PARAGRAPH CARRIED UNTIL u-rev 6 IS FALSE AND IS WITHDRAWN**
(MINOR 8 of `wp15b_U4_REVIEW.md`). It read: *"Sites 4–6 are outside §8 and inside this
unit, and the cell counted only what was outside the unit."* The cell's own text, three
paragraphs above, counts *"item 15 in this unit's list"* — which is site 3 and is
INSIDE the unit. So the cell did not count only what was outside the unit: **it counted
one inside site and missed three others.** The **COUNT SIX stands** — the u-rev 5
reviewer reconstructed it independently from the superseded document's own letter
occurrences and found no seventh — and only the explanation was wrong, in a paragraph
whose subject is the accuracy of cost cells. **THE SAME FALSE DIAGNOSIS WAS IN THE
LANDED D-316** (*"the cell counted only the sites outside the unit and missed three
inside it"*), and `docs/decisions.md` is append-only and is not this unit's to edit, so
u-rev 6 carried the residual to the architect. **IT HAS SINCE BEEN DISPOSED OF, AND
THIS UNIT IS CATCHING UP TO IT: `D-325` LANDED AT `81180b8`** — after u-rev 6 — and
corrects the log by a new line rather than an edit, which is the remedy this unit asked
for. D-325 records that the cell *"counted one inside site and missed three others"*,
that **THE COUNT SIX STANDS** and there is no seventh site, and that D-316's
conclusion — the correction errs toward the selected option and does not move the
selection — is untouched. It names this unit's own repaired copy as the reason a line
was still worth its space: *"the paragraph the false sentence sits in is ABOUT THE
ACCURACY OF COST CELLS … and a mis-diagnosis of how a cell went wrong is how the next
cell goes wrong the same way."* **So nothing here is owed to the architect any more**,
and the OPEN list below records it CLOSED rather than carried.
**The correction does not move the selection** — the cell's own
sentence is that the count is not the argument — but it is recorded rather than
quietly fixed, because a cost cell understating the selected option's cost is the
defect class this work package has been failing on, and the reviewer of this unit
should see it stated rather than discover it.

**The stated cost of shape 2 is DISCHARGED, not paid.** The cell's charge was
that "§8.3(a)" cited from elsewhere would after shape 2 "name nothing". §8.3 now
opens with a lookup table mapping each retired letter to its gate, so every
legacy citation — in another unit, in the superseded document's history, or in a
landed ADR line — still resolves. The letters are retired as an ADDRESSING
SCHEME, not erased as a HISTORY.

**THE RESIDUAL, AND IT IS NO LONGER OPEN: D-320 DISPOSED OF IT, AND THE DEBT IS
RECORDED AS PAID.** This selection was made by architect ruling on the comparison
above. That comparison states both options, both costs MEASURED, and each option's
failure mode, but it carries no recommendation and **it was never put to a
fresh-context DECISION-RED-TEAM**. CLAUDE.md's Process section wants a named design
decision with more than one viable option settled by an attacked matrix, and this one
was not attacked, so **adopting shape 2 without a matrix IS A BREACH OF THE MATRIX
LAW** — which is how **D-320** (`docs/decisions.md`, landed at `0af32fb`) opens, in
those terms, *"recorded here rather than left implicit"*.

**WHAT D-320 RULES, in its own load-bearing parts, because this unit is the document it
is about and the document where its flip clause would fire:**

- **THE BREACH IS ACKNOWLEDGED, NOT ARGUED AWAY**, and D-316's own naming of the
  residual is quoted back into it.
- **THE RETRO-MATRIX IS WAIVED — "on two grounds stated together because neither
  carries it alone".** *(i)* **PROPORTIONALITY:** the decision is a naming scheme for
  four gates that already exist as gates, and the whole of what a matrix would add is a
  red team's attack on a comparison whose two cost cells were already MEASURED and
  whose LOSING option's own cell conceded the ground the selection turned on. *(ii)*
  **INDEPENDENT VERIFICATION ALREADY HAPPENED**, which D-320 calls "the part that makes
  this a waiver rather than an excuse": the fresh-context REVIEW-design at
  `docs/experiments/wp15b_U4_REVIEW.md` verified the six-site retarget the correction
  predicted, independently and against the tree, and its scope-1 verdict on the
  named-gate wiring is **PASS** — so the thing a retro-matrix would most plausibly have
  caught, a gate silently lost between the lettered bar and the named one, was checked
  by a session that did not make the choice.
- **WHAT D-320 DOES NOT DO**, stated because a reader may not take the waiver wider
  than it goes: *"it does not license adoption-without-matrix as a practice, and it does
  not convert D-316's residual into a clean record."*
- **THE DEBT IS PAID.** D-320's words: it *"converts an unattacked selection that nobody
  wrote down into a debt the architect has now **paid** in the only currency available
  after the fact, which is **disclosure**."*
- **THE FLIP CLAUSE, AND THIS UNIT IS WHERE IT WOULD FIRE.** D-320 *"flips if a
  **GATE-NAMING DEFECT** surfaces — a part of the soundness gate that the four names do
  not reach, a citation that resolves under the retired letters and not under the names,
  or a fifth gate appended as a letter — at which point the waiver was wrong, the matrix
  is owed retroactively, and this line is superseded by the round that runs it."* **Any
  reviewer or IMPL session that finds one of those three things in §8.3, §8.7 or a
  citing unit has fired D-320's flip**, and the retro-matrix becomes owed.

**THE TWO DOCUMENTS ARE LOAD-BEARING ON EACH OTHER, AND UNTIL u-rev 7 ONLY ONE OF THEM
KNEW IT.** D-320's waiver rests on THIS unit's review history; this unit told the
architect the opposite — that the debt was theirs *"to choose to pay or to accept"* and
that the residual was *"not closed"* — in the list an architect reads to learn what is
owed. That is **BLOCKING 1** of `wp15b_U4_REVIEW_urev6.md`, and D-320 landed in the
same commit as the D-321 this unit folds. **The reviewer of this unit is not asked to
ratify the selection; it is asked to look for the gate-naming defect D-320's flip names.**

### ADR lines

Carried from the superseded §15. Its item numbers are retained exactly so an
existing cross-reference to "§15 item n" still resolves; this unit invents none
and renumbers none. The superseded §15's preamble does not travel (MAJOR 10
measured it false on both clauses); this is U4's lead-in instead: **both items below
are this unit's own and neither has landed.** *(Until u-rev 6 this lead-in said BOTH
were blocked on a selection that is OPEN, and gave item 15 three mutually
inconsistent reasons — MAJOR 4 of `wp15b_U4_REVIEW.md`. One of the three, "B3's
unresolved wiring", is measurably CLOSED: this same section records B3 SETTLED at
u-rev 2 under D-316 and struck through in the OPEN list below, and M3's own fold says
the other three named gates are unaffected. It is deleted rather than argued. The
stale phrase "M3's fresh matrix" is deleted with it — the matrix was authored twice,
attacked twice and selected from.)*

- **Item 4 is NO LONGER BLOCKED ON M3.** M3 is SELECTED — S-M, D-323 — so the
  instrument exists and the item is rewritten against it. It is blocked instead on the
  **SEAM**, which D-323 records as a separate named decision it does not make.
  **RE-CHECKED AT u-rev 7, AND THIS SEAM IS NOT THE ONE D-329 SELECTS.** Item 4's seam
  is *how a TEST observes the staged generator's emitted set*, constrained by D-115's
  bar on widening `pistol_search::staged` to `pub`. D-329's seam is *how
  `tools/baseline_snapshot.sh` is told which CONFIG to run under*. The two share a word
  and nothing else, and **no selection has touched item 4's**.
- **Item 15 STAYS BLOCKED, AND AT u-rev 7 THE GROUND IS NARROWER AND IS NOT THE ONE
  D-324 GAVE.** D-324 blocked it because **no config seam was selected**. That is no
  longer true: **axis A selects N-E (D-329)**. What blocks item 15 now is that **the
  seam is SELECTED AND NOT BUILT** — no line of N-E is written, four registered
  conditions ride with it unpaid — and, independently, that
  `configs/instrument_staged_v0.toml` does not exist, which D-329 records as blocking
  every axis-A row equally rather than being N-M's cost. **A SELECTION IS NOT AN
  IMPLEMENTATION, and this unit does not write item 15 as though the snapshot were
  takeable.** **UNRECONCILED and recorded rather than resolved, and UNCHANGED by the
  selection:** MAJOR 4 of `wp15b_U4_REVIEW.md` observes that item 15's own subject — the
  two TACTICAL staged configs and THE TACTICAL SUITE gate (§8.3, the superseded `(a)`) —
  has no evident dependency on the *snapshot's* config seam at all, so a selection on
  that seam neither creates nor removes the dependency in question. This unit may not
  overrule a landed ADR line and may not hide the disagreement either; it is in the OPEN
  list for the architect.

4. **The differential gate's instrument, and D-124's flip clause discharged.
   REWRITTEN AT u-rev 6: THE OPTION IS S-M (D-323), NOT S-E.** Per-node **EQUALITY**
   of the emitted set against the **LANDED** referent R1
   (`crates/pistol-solver/tests/common/reference.rs`), **REUSED by a `#[path]` include
   and not rewritten**, at every FILTERED node; MARKED **DEPENDS-OPEN-THEORY**
   (D-321); carrying D-323's five registered conditions and its flip clause. **The
   line still cannot be written, and the reason has CHANGED**: it is no longer that no
   option is selected — one is — it is that the SEAM by which a test observes the
   emitted set is a separate named decision D-323 leaves OPEN, and D-115 (round-1 F4,
   which forbids S-E's primary mechanism by name) is what makes it a decision. *That
   remains true at u-rev 7: D-329 selects the SNAPSHOT's config seam and decides nothing
   about how a test reaches the emitted set.*
   *Until u-rev 6 this item read "S-E, and D-124's flip clause discharged. Its seam is
   the PAIR of §8.2 — a public generator driven by a test in pistol-search's own tree
   against an independently written referent, plus an always-on `assert!` in `visit`
   for the drop a generator test cannot see." Every clause of that has moved: S-E fell,
   the independently written referent is now FORBIDDEN by condition 1 in favour of
   reuse, and the `assert!` half is OPEN rather than registered. Revision 4's line had
   already registered the `#[cfg(debug_assertions)]` observer that §8.2 had withdrawn
   two sections earlier — the un-re-read claim, inside the ADR list itself — so this is
   the second time this same item has carried a superseded mechanism.*

15. **The two TACTICAL staged configs disable the quiet cut** — which needs its own
    document, `tactical_staged_v0.toml`, because the SPRT seat must keep the cut.
    **U3** (u-rev 6, landed `13621d3`) §10 is the one place the number of staged config documents is
    stated and this line does not restate it. *Until u-rev 6 this line
    named that document by an ordinal and so stated the count — BLOCKING 3 of
    `wp15b_U4_REVIEW.md`, B5's class.* So THE
    TACTICAL SUITE gate (the superseded `(a)`) tests the threat mechanisms rather than
    the prune, and the prune is judged by SPRT, by the movetime measurement and by the
    differential gate (§8.2 — **S-M** since D-323, S-E until u-rev 6). The line records
    what a green tactical suite under Staged does NOT evidence.

**AND B2 IS ANSWERED AT u-rev 7 — BY A LANDED ADR LINE THAT IS NOT THIS UNIT'S, AND
THE ANSWER CARRIES A RESIDUAL.** The revision-7 review found **M4 has no ADR line at
all**: §15 contained zero occurrences of `N-A`, `baseline_snapshot`, `--config` or
`snapshot`, while §9 adopted N-A, changed a shipped `tools/` instrument and changed the
registered quantity of the snapshot. Rule 10 requires one line; the Process section
requires it to record the strongest surviving attack. **The carve could not write that
line for six u-revs, because §9's selection was OPEN and a line citing an unattacked
selection is the breach it exists to prevent** — three fresh DECISION-RED-TEAM rounds
ran and all three stopped (rounds 1 and 2 under D-318, round 3 under D-324's RECORDED
TIE).

**THAT REASON IS GONE. A FOURTH ROUND WAS AUTHORED AND ATTACKED, AND `D-329` IS M4's
ADR LINE.** It records the field (`7866bcf`, axis A alone, N-Q authored in), the attack
(`7e0a328`), the selection (**N-E**, rung (b), after N-M's elimination on registered
ground and rung (a) found silent), four binding conditions, and what it supersedes in
D-324. **What rule 10 asked for exists in the log.**

**WHAT IS NOT DISCHARGED, AND IT IS D-329's OWN RESIDUAL RATHER THAN THIS UNIT'S
EVASION: THE STRONGEST SURVIVING ATTACK AGAINST N-E IS ASSEMBLED AND NOT QUOTED.** The
red team was dispatched to break **N-Q**; it recommends N-E and **was never asked to
break it**, and no fresh context has been. D-329 records the difference rather than
smoothing it. The Process section's requirement that the line record the strongest
surviving attack is therefore met by an assembled paragraph, which is weaker than a
quoted one, and **a fresh-context attack on N-E in its own right is OPEN and is in the
list below**.

**AND B2's PRACTICAL CONSEQUENCE IS UNCHANGED:** nothing of N-E is built, so the
snapshot still has a BEFORE and no AFTER. *(Travelling item T2 is carried under this
head, **and at u-rev 8 it gets the disposition u-rev 7 withheld.** T2 is named in the
tree at `docs/experiments/restructure_matrix_15b.md:35` and
`docs/experiments/restructure_selection_15b.md:50`, both under
`## Travelling items (bind to every option, cost is common)`, and both give it the
same content — **"M4 ADR line (B2)"**. That is the head this paragraph carries it
under, and B2 is ANSWERED by D-329 with the assembled-not-quoted attack recorded as
its residual and RULED ON by D-333. **T2 is therefore DISCHARGED**, on the same two
documents from whose fiftieth line this unit already resolves its sibling T5, at
U4:108. Until u-rev 8 this parenthetical said T2 was *"named in no document now in the
tree"* and withheld the disposition on that ground — **MAJOR 4** of
`wp15b_U4_REVIEW_urev7.md`. A travelling item is precisely the thing a carve must not
silently drop, and the false premise was new at u-rev 7, in the section that u-rev
re-derived in full.)*

### OPEN — carried forward, not closed by the carve

- **CROSS-UNIT CITATIONS IN LIVE CARVE PROSE THAT NAME NO u-rev ARE NOT CONVERTED.**
  u-rev 8 adopted the `(u-rev N, landed <sha>)` form (head, beside D-311's label
  discipline) and converted the six citations that named a u-rev and named a
  SUPERSEDED one. It did not convert the citations that name a section without naming
  a u-rev at all. The set is derived by the command in that head paragraph, and it
  contains two kinds — live carve prose, which a later round may convert, and text
  inside blocks the §8 record stamp lists as RECORD, which this unit may not edit.
  **What is owed: a pass that converts the live ones and states, per site, which of
  the two kinds each is.** Registered rather than claimed done, because the sibling
  unit answered the same finding with a universal and the universal was false at nine
  sites (`docs/experiments/wp15b_U3_REVIEW_urev5.md`, MAJOR C).

- ~~**B1 / M3 — no matrix, then a matrix in which every option fell.**~~ **CLOSED AT u-rev 6 BY SELECTION: S-M, D-323** (the block at the head of §8; record at `docs/experiments/matrix_M3_selection.md`). Round 1 (`f8e73e4`, D-317) stopped with every stated option fallen and named four missing rows; revision 2 (`d48824f`) authored thirteen rows including those four, was attacked at `809b5db` — all eight facts reproduced — and **S-M was selected at `af8082a` (taken at `809b5db`, the revision carrying the attack; the record file exists only from `af8082a`). The matrix's own recommendation, S-K, is dead, killed by its own attack.** **THE RESIDUALS ARE NOT CLOSED and each is named where it lives:** the gate ships MARKED **DEPENDS-OPEN-THEORY** (D-321, condition 3) and turns RED on a correct engine if the convention settles toward `DEF-T`, at which point the CRITERION is re-derived and not the engine; **S-N** — the rules-derived row the red team found missing — is OWED and is a FLIP TRIGGER (condition 4); the SEAM is a separate named decision and is OPEN; the gate's corpus and per-CI cost are not decided; `0 of 3406` may not be cited as evidence about the convention (condition 2); and a second freshly-written referent is FORBIDDEN without a registered agreement criterion and a registered consequence for disagreement (condition 1). Each has its own bullet below.
- ~~**B2 / M4 — no ADR line, and after THREE authored revisions and THREE fresh-context DECISION-RED-TEAMs the selection is STILL OPEN.**~~ **ANSWERED AT u-rev 7 BY SELECTION ON AXIS A: N-E, D-329** (the block at the head of §9; record at `docs/experiments/matrix_M4_axisA_selection.md`, selected at `7e0a328`, landed at `d56a898`). Rounds 1 and 2 stopped (D-318); round 3 stopped on the RECORDED TIE (D-324), whose cause was that **the field had TWO ORTHOGONAL AXES and was being scored as one** — axis A is how the config is NAMED, axis B is how many RECORDS one invocation produces, and N-K composes with a naming row rather than rivalling it. D-324's own stop clause authorised **a fourth round scoped to axis A alone with N-Q authored into it**; that round ran at `7866bcf` under the D-328 split, was attacked at `7e0a328`, and **its own recommendation (N-Q) was killed by its red team.** **N-M was eliminated on registered ground** — `wp15b_sprt_prereg.md` §7A.2 registers `--config configs/gate_v0.toml` and N-M refuses it at exit 1 — **rung (a) was SILENT across the whole field**, and the selection was taken at **rung (b)**: N-E 22 added / 7 CODE against N-Q's 32 / 12, both owing the same 4 whole-path guard lines and N-Q owing 5 containment lines on top. **AXIS B IS NOT REOPENED** (D-329): D-324 records its flip clause already fired toward N-K, and no ADR line adopts N-K. **THE RESIDUALS ARE NOT CLOSED, and each has its own bullet below:** N-E has never been attacked by a fresh-context DECISION-RED-TEAM in its own right, so **D-329's strongest surviving attack is ASSEMBLED, not quoted**; the four conditions riding with the selection are unpaid; and **the seam is SELECTED AND NOT BUILT**, so the snapshot still has a BEFORE and no AFTER.
- **N-E HAS NOT BEEN ATTACKED IN ITS OWN RIGHT, AND AT u-rev 8 THAT IS RULED ON RATHER THAN MERELY CARRIED — `D-333` (architect ruling R18).** R18 holds that this does NOT reopen the selection, on two grounds it states rather than assumes: the matrix law binds the FIELD, which was authored and attacked before selection; and the selection was taken at a REGISTERED LADDER RUNG whose twelve cells the red team itself re-derived exactly. **It flips if N-E's implementation debt materially exceeds the 22 added / 7 CODE ground rung (b) was taken on**, at which point axis A reopens as a two-row comparison between N-E as built and N-Q; and independently if a fresh context is ever dispatched against N-E and breaks it. **R18 does not certify N-E sound, does not discharge the four conditions, and does not close the weakness below.** The round-4 red team was dispatched to break **N-Q** and the tied set's interaction with it; it recommends N-E and was never asked to break it. D-329 claims no more than the field. The assembled paragraph's own point is the one to attack first: **N-E bounds the admissible set nowhere**, so a record may name a document outside the repository that no other reader can obtain, and its provenance rests on caller discipline plus a digest of bytes nobody else holds. The round MEASURED that N-Q does not fix this either (a gitignored `configs/*.bin` passes containment into the invariant block at exit 0), so the weakness is left where it was rather than resolved.
- **THE FOUR CONDITIONS RIDING WITH N-E ARE UNPAID, AND EACH BINDS IMPL** (D-329). **(1)** The `config` line's digest is **`$3`, not `$4`** — that line is `config <path> <sha>`, three fields, and the four-token reasoning belongs to the differently shaped `corpus` line. **(2)** N-E's whole-path guard **may NOT be spelled as a reuse of the `tools/baseline_snapshot.sh:289` basename loop** — MEASURED twice, that spelling leaves `configs/spaced dir/instrument_v0.toml` reaching the record at exit 0. **(3)** An **item-10 driving test** is owed for both new refusal classes, in two halves with a control, against a precedent of 91 test lines for one arm at `b067d47`; no row of round 4 was costed for it. **(4)** An **item-12 sentence** is owed saying a config refusal is a **FAIL**, this script having declared no void class. *(A fifth thing is recorded, it is not a condition, and **it is recorded by the SELECTION RECORD and not by D-329** — `docs/experiments/matrix_M4_axisA_selection.md` condition 4, from red-team F6. The relative-base inconsistency: a relative `--config` resolved against `$ROOT` while a relative `--out` resolved against `$CALLER_PWD`, which N-E would have inherited without making it load-bearing for a refusal. **CLOSED AT `63eac4c`, BEFORE N-E IS BUILT** — architect ruling R19 gave `tools/baseline_snapshot.sh` ONE base for every caller-supplied path, stated it in the script's own usage text, and made the two readings' disagreement a named refusal; its REVIEW-impl PASSED with 0 BLOCKING and 0 MAJOR at `d59f0de`. So N-E has nothing here to inherit. **This is the one condition-adjacent residual the round did not have to carry to IMPL.** Until u-rev 8 this parenthetical attributed the residual to "recorded by D-329", and D-329 records nothing of the kind — zero occurrences of `relative`, `CALLER_PWD` or the F6 residual — so a reader who checked the ADR, which is what the words instructed, found nothing: **MAJOR 3** of `wp15b_U4_REVIEW_urev7.md`, an ATTRIBUTION defect, which is the class D-322 and D-330 both landed lines about in this same work package.)*
- ~~**B3 — gate (b), the two shapes above.**~~ **CLOSED at u-rev 2** by the architect's selection of shape 2, recorded above and in D-316. ~~**Its RESIDUAL is not closed**~~ — **THE RESIDUAL IS DISPOSED OF TOO, BY `D-320` (landed `0af32fb`), AND THIS UNIT SAID OTHERWISE UNTIL u-rev 7.** The selection was not put to a fresh-context DECISION-RED-TEAM, which D-320 records as **a breach of the matrix law**, acknowledged and not argued away. **D-320 WAIVES the retro-matrix on two grounds stated together because neither carries it alone** — PROPORTIONALITY (a naming scheme for four gates that already exist, over a comparison whose two cost cells were already MEASURED and whose losing option's own cell conceded the ground the selection turned on) and **INDEPENDENT VERIFICATION THAT ALREADY HAPPENED** (the fresh-context REVIEW-design at `wp15b_U4_REVIEW.md` verified the six-site retarget against the tree and returned PASS on the named-gate wiring, so a session that did not make the choice checked the one thing a retro-matrix would most plausibly have caught). **The debt is recorded as PAID "in the only currency available after the fact, which is disclosure."** **NOTHING HERE IS THE ARCHITECT'S TO DECIDE ANY MORE.** **WHAT IS STILL LIVE IS D-320's FLIP CLAUSE, AND THIS UNIT IS WHERE IT WOULD FIRE:** it flips if a **GATE-NAMING DEFECT** surfaces — a part of the soundness gate the four names do not reach, a citation that resolves under the retired letters and not under the names, or a fifth gate appended as a letter — at which point the waiver was wrong, **the matrix is owed retroactively**, and D-320 is superseded by the round that runs it. *(Until u-rev 7 this bullet said the residual was not closed and the section above told the architect the debt was theirs "to choose to pay or to accept" — **BLOCKING 1** of `wp15b_U4_REVIEW_urev6.md`, on a line that had already landed in the same commit as the D-321 this unit folds.)*
- ~~**MAJOR 8 — M4's and M6's mutation witnesses are not positions a legal game
  reaches.**~~ **CLOSED at u-rev 3.** Both witnesses are rebuilt in §8.4 as
  positions reached by replaying every ply through `GameState`, which is the
  referee rule 2 names, and both are pinned by
  `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`. The pin is not
  vacuous: MEASURED, the superseded M4 witness is REFUSED by that replay on three
  independent counts — P1 holds an even 8, its first stone is not the origin, and
  P2's 4 is neither one more nor one fewer than 8. §8.4's old "VERIFIED on the
  shipped solver" is replaced by a verification that goes through the rules
  first, which is the distinction MAJOR 8 drew and the reason the old claim was
  true and worthless. **THE RESIDUAL IS NAMED AND IS NOT CLOSED, AND IT HAS TWO
  PARTS AND NOT ONE (MINOR 11).** (i) A legal position is not yet a position the
  mutation DIES on in the SEARCH — that needs the search, and the search is not
  built (`crates/pistol-search/src/staged.rs` does not exist; MEASURED at `46c58ac`
  and RE-TAKEN at `8690ad6`, `ls` returns *No such file or directory* at both). (ii) **M6 owes a SECOND construction
  that does not depend on the search existing at all.** §8.4's own M6 row states that
  the witness is driven as a NON-PV DESCENDANT and never as a root, because the
  overload return is `!is_pv`-gated and ply 0 is always a PV node — so as a root the
  mutant does not fire and survives. The pinned witness is a POSITION, and a position
  handed to a search is a ROOT. M6 therefore additionally owes a PARENT position from
  which the pinned witness is reached as a non-PV descendant at a null window: not
  built, not pinned, and not gated on `staged.rs` existing. Until u-rev 6 this
  residual attributed the whole remaining gap to the search not being built, which is
  incomplete for one of the two witnesses it covers. What is discharged is the
  reachability half, which was the half MAJOR 8 raised; the ledger's "dies here"
  claim stays owed to IMPL. **And M3's witness is not built at all — see MAJOR 5 in
  the list below.**
- **The snapshot's SECOND INSTRUMENT is unregistered** (U4-M). Replication is
  registered; the second instrument, its agreement criterion, the stage under
  doubt, how the second instrument does not share that stage, and the registered
  consequence of disagreement are not.
- **The `tools/` changes this unit implies have had no SHELL_CHECKLIST review.**
  §8.7's `staged_soundness_check.sh` is a new script; §9's `--config` — **now a
  SELECTED shape, N-E under D-329** — reopens `tools/baseline_snapshot.sh`. Both reviews
  are owed at IMPL, and the coverage rule binds each. **WHICH CHECKLIST ITEMS N-E's
  `--config` ENGAGES IS STATED AT §9.1 AND NOWHERE ELSE (D-331), AND D-329 CHANGES THAT
  SET IN BOTH DIRECTIONS.** §9.1 amendment 2's *"eight of twelve"* was counted for
  **N-A**; D-329 ADDS four conditions and **REMOVES item 11**, whose scope is a binding
  consumed by `rm`, `mv` or a write, while `$CONFIG` is a READ — so item 9 governs it
  and is discharged by the whole-path guard both rows owe. **§9's own head carries that
  measurement (red-team F13) and this bullet cites it rather than re-counting.**
  *(Until u-rev 8 this bullet said the reopening engages "eight of twelve … plus D-329's
  four conditions ON TOP", which told an IMPL session that D-329 only adds. It both
  adds and subtracts, the subtraction was one of the three measured findings the
  selection turned on, and the falsifying line is the one the same u-rev existed to
  fold — **MAJOR 2** of `wp15b_U4_REVIEW_urev7.md`, the third recurrence of the class
  D-331 was landed for.)*
- ~~**TWO SHIPPED-INSTRUMENT DEFECTS ARE OPEN AGAINST `tools/baseline_snapshot.sh`.**~~
  **BOTH CLOSED, AND THE FIX IS REVIEWED.** They were found in passing by M4's round-3
  red team and owned by no matrix row (D-324): a SPACE in a caller-named `--corpus` path
  reaching the record unescaped at exit 0 under the COMPLETE kind token, breaking the
  record's own leading-tokens parse rule and shifting the digest field; and
  `crates/pistol-cli/src/report.rs:151-162` rewriting control characters to `?` in the
  engine's handshake while the script wrote its own copy raw. **The first is fixed at
  `b067d47`** — which also replaced `basename` with `${X##*/}`, which is why round 4's
  measurements are pinned there — **and the second at `a102c6a`**, where the engine now
  refuses a config path its handshake cannot echo verbatim. **A fresh-context REVIEW-impl
  covering both as ONE defect class PASSED at `84ff8d7`**, 0 BLOCKING / 0 MAJOR / 3
  MINOR (`docs/experiments/wp15b_trackC_REVIEW_impl.md`), and it is a real pass rather
  than a restated one: the reviewer restored the pre-fix sources and re-ran both tests,
  then **mutated both guards to refuse everything and confirmed each test's CONTROL half
  kills the mutant**, so neither control is vacuous.
- **THE THREE MINORS FROM THAT REVIEW-impl ARE OPEN** (F1, F2, F3 of
  `docs/experiments/wp15b_trackC_REVIEW_impl.md`), and they were deliberately NOT
  repaired in the reviewed commit, because an amendment reopens the review however small
  the diff. **F1** — the usage block `b067d47` added asserts a universal that one command
  falsifies: it says every way of not writing a record is *"a 1 with a named reason on
  stderr"*, and a scratch failure is a fourth way with no named reason (`WORK="$(mktemp
  -d)"` carries no `|| fail`), which is `SHELL_CHECKLIST` item 12 obligation 2's own
  defect asserted against in the paragraph citing item 12. **F2** — the `$CONFIG` comment
  states the guard's criterion as *"caller-named"*, which is not the criterion the loop
  applies (`$OPENINGS` is equally not caller-named and IS guarded); the real criterion is
  *"reaches the record as a whitespace-delimited field"*. **F3** — the same usage block
  says a test driving the script *"asserts the CODE and not merely `!success`"*; the new
  test does, but `Run::refusal()` does not, and it backs 18 of the suite's 19 refusal
  assertions. **F1 bears directly on D-329's condition 4**, which owes an item-12
  sentence for the config refusal in this same block.
- **§8.4's M3 WITNESS IS NOT BUILT** (MAJOR 5). Until u-rev 6 the cell said BUILT and
  named an abstract window-empties shape from `crates/pistol-solver/src/cover.rs`'s
  module doc comment — no coordinates, no stone counts, no parity, no legality, no pin.
  It is a REQUIRED PROPERTY of a witness, not a witness. M3 is one of the two mutations
  the ledger classes to the differential gate, so this is the half MAJOR 8 raised for M4
  and M6, at a row MAJOR 8's literal scope did not reach. **A position a legal game
  reaches is OWED**, on the `wp15b_mutation_witnesses.rs` pattern.
- **THE DIFFERENTIAL GATE'S SEAM IS A SEPARATE NAMED DECISION AND IS OPEN.** D-323
  selects the CRITERION (S-M) and explicitly does not decide how a test observes the
  emitted set; D-115's constraint on widening `pistol_search::staged` to `pub` —
  round-1 F4, which named S-E's primary mechanism by name — applies to S-M as it
  applied to S-E. **`tools/staged_soundness_check.sh` cannot be specified for this gate
  until that decision is made**, and the other three named gates are unaffected.
  **RE-CHECKED AT u-rev 7 AND STILL OPEN — AND IT IS NOT THE SEAM D-329 SELECTED.** Two
  different decisions in this unit are called a seam: this one is *how a TEST observes
  the emitted set*; D-329's is *how `tools/baseline_snapshot.sh` is told which CONFIG to
  run under*. **No matrix has ever been authored for THIS one**, and no ADR line
  decides it.
- **S-E's SECOND HALF — the always-on `assert!` in `visit` — IS NOT SELECTED AND NOT
  REJECTED.** S-M is a criterion over the emitted set alone. §8.4 still registers M8
  against it and U4-T still names the test; **no attacked matrix row carries it at
  u-rev 7 either**, and IMPL may not read it as registered. *(Re-checked against the
  log at u-rev 7: D-329 decides the snapshot's config seam, not this.)*
- **THE GATE'S PER-CI COST IS UNGROUNDED AT ITS DOMINANT TERM.** U4-M's ESTIMATED
  40-90 s was priced on S-E's traversal plus the reduced S-C's MEASURED 17.89 s; S-E
  fell, and the composite row that carried the reduced S-C (S-J) fell with it. D-323
  does not re-price the gate — the selection record says this figure is CARRIED from
  here and is not that round's measurement — so a re-derivation for S-M is owed.
- **`configs/instrument_staged_v0.toml` DOES NOT EXIST**, so the snapshot's AFTER is
  blocked independently of M4's seam and no row of any M4 revision produces it — **round
  4's three rows included** (D-324; MEASURED at u-rev 6 and RE-TAKEN at u-rev 7, output
  at U4-M item 1). **D-329 corrects D-324's framing of this blocker as an N-M cost: it
  blocks every axis-A row equally.**
- **THE SNAPSHOT'S CONFIG SEAM IS SELECTED AND NOT BUILT.** N-E is the selected shape
  (D-329) and no line of it is written: MEASURED at `8690ad6`,
  `tools/baseline_snapshot.sh:182` still reads `CONFIG="configs/instrument_v0.toml"` and
  the script takes no `--config` argument of its own. **A SELECTION IS NOT AN
  IMPLEMENTATION**, and nothing in this unit may be read as though the Staged snapshot
  were takeable.
- **ITEM 15's BLOCKAGE IS UNRECONCILED BETWEEN A LANDED ADR AND A LANDED REVIEW, and
  it is the architect's. THE SELECTION DID NOT RESOLVE IT.** D-324 (and D-318 before it)
  states that U4-Z item 15 stays blocked on the snapshot's config seam. MAJOR 4 of
  `wp15b_U4_REVIEW.md` observes that item 15's subject — the two TACTICAL staged configs
  and THE TACTICAL SUITE gate — has no evident dependency on the *snapshot's* seam, and
  that on this document's own evidence item 15 may not be blocked at all. **D-329
  selects that seam and changes neither side of the disagreement**: the blockage's
  ground narrows from *"no seam is selected"* to *"the seam is selected and not built"*,
  and the question MAJOR 4 raises — whether item 15 depends on that seam at all — is
  untouched by which option was chosen. This unit records the ADR's ruling as binding
  and the disagreement as open; it overrules neither.
- ~~**D-316 CARRIES A FALSE DIAGNOSIS AND `docs/decisions.md` IS NOT THIS UNIT'S TO
  EDIT.**~~ **CLOSED BY `D-325`, LANDED AT `81180b8` — AFTER u-rev 6, AND THIS UNIT IS
  CATCHING UP TO IT AT u-rev 7.** D-316 said *"the cell counted only the sites outside
  the unit and missed three inside it"*; the cell counted item 15, which is inside the
  unit (MINOR 8 of `wp15b_U4_REVIEW.md`, corrected in this unit's own copy above).
  **D-325 is the correction, and it took exactly the remedy this bullet asked for — a
  NEW LINE in an append-only log, not an edit.** It records that the cell *"counted one
  inside site and missed three others"*, that **the COUNT SIX stands** with no seventh
  site, and that D-316's selection, its residual and its flip clause are all unaffected.
  Nothing here is owed to the architect any more.
- **No REVIEW-design has run against this text at THIS u-rev** (U4-A). **u-rev 6 was
  reviewed and FAILED** (`docs/experiments/wp15b_U4_REVIEW_urev6.md`, pinned revision
  `7358a07`, **1 BLOCKING / 2 MAJOR / 4 MINOR**), as u-rev 5 had before it
  (`docs/experiments/wp15b_U4_REVIEW.md`, `35aab95`, 3 BLOCKING / 3 MAJOR / 5 MINOR).
  **u-rev 7 is the repair of the u-rev 6 report plus the M4 axis-A selection fold**, and
  a review of a superseded revision does not transfer — an amendment reopens the review,
  however small the diff.

---

*U4, u-rev 7. A carve, not a revision. **BOTH MATRICES NOW HAVE A SELECTION AND NEITHER IS BUILT.** M3: attacked once with every option fallen (D-317), then re-authored over thirteen rows, attacked again, and **S-M SELECTED at `af8082a` (taken at `809b5db`; D-323)** — marked DEPENDS-OPEN-THEORY under D-321, with five registered conditions, an owed S-N flip trigger and an OPEN SEAM that no selection has touched. M4: stopped three times over three authored fields (D-318, then D-324's RECORDED TIE, whose cause was two orthogonal axes scored as one), then a fourth round scoped to **AXIS A** alone with N-Q authored into it — attacked at `7e0a328`, its own recommendation killed by that attack, **N-E SELECTED (landed `d56a898`; D-329)** after N-M's elimination on registered ground and rung (a) found silent, with four conditions riding and its strongest surviving attack ASSEMBLED rather than quoted. Axis B is not reopened. B3 CLOSED at u-rev 2 and **its residual DISPOSED OF by D-320's waiver, whose flip clause — a gate-naming defect — this unit is where it would fire**; D-316's false diagnosis CLOSED in the log by D-325. MAJOR 8's reachability half CLOSED at u-rev 3 for M4 and M6 and NOT for M3, whose witness is not built. **IMPL has not started, and this u-rev has not been reviewed; u-rev 6's review FAILED (1 BLOCKING / 2 MAJOR / 4 MINOR)** and this u-rev is its answer.*
