# REVIEW-design (re-review) — `docs/experiments/U4_soundness_instrument.md`, u-rev 9

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer (only HTML-escaped angle brackets and ampersands restored:
`&gt;`/`&lt;`/`&amp;` to `>`/`<`/`&`). The dispatching session repaired U4; it did not
review it. Reports live in the tree rather than in a scratchpad because finding
IDs are cited across units.
-->

## Header

- **Pinned revision:** `1964026`. **At entry:** `git rev-parse HEAD` → `1964026c8efe89a4fea09f8e5c499cd40b7d9c42`. **At exit:** identical. **The pin matches HEAD.**
- **Subject moved?** No. `git diff 1964026 HEAD -- docs/experiments/U4_soundness_instrument.md` → empty (0 lines). Worktree carries only two untracked sibling review reports (`wp15b_U1_REVIEW_urev4.md`, `wp15b_U3_REVIEW_urev8.md`) from parallel rounds; the subject is unmodified.
- **Document + u-rev:** `docs/experiments/U4_soundness_instrument.md`, u-rev 9, landed at `3ced890`.
- **MEASURED size:** `wc -l` → **2197** lines; `wc -c` → **190151** bytes.
- **Prior report answered:** `docs/experiments/wp15b_U4_REVIEW_urev8.md`, against `a2b50bf` — FAIL, 1 BLOCKING / 7 MAJOR / 7 MINOR.
- **Context freshness:** fresh. I did not author this document, any of its u-revs, any ADR line, or the prior report. Everything I score below was re-measured in this session; every number I state is marked MEASURED or ESTIMATED.
- **Scope as given, and I held to it.** Three finding kinds only: (a) a false or drifted claim in **surviving** text, including text this round introduced; (b) a **normative** claim that lost its home to a strike (D-346's flip clause); (c) a normative error. The absence of struck text is not a finding; a pointer is not a finding; gate 15's subject (head/foot u-rev agreement, summand/group self-counts) was not re-derived and label agreement is not reported. I read `tools/label_consistency_check.sh`'s header only to establish that its second check covers **summand lines, group-count lines and summands-against-table-rows** — so prose self-counts such as the head's *"Eleven such claims"* fall outside it and inside my scope.

## VERDICT

**FAIL — 0 BLOCKING, 6 MAJOR, 4 MINOR**

Counts derived from this report's own finding headings:

```
$ awk '/^## MAJOR/,/^## MINOR/' report.md | grep -c '^### '
6
$ awk '/^## MINOR/,0' report.md | grep -c '^### '
4
```

**The one-line diagnosis.** The strike policy worked where it was applied: I checked every strike this round made for D-346's flip clause and **found no normative claim homeless** — that negative result is recorded in full below and it is the round's real achievement. What failed is the same generator D-346 was landed to stop, displaced one level: **the round wrote eleven new claims about its own state into the REVIEW STATUS block, the change log and the record stamp, and six of them are false.** Two rows misdescribe the operation performed at the site they name (MAJOR 4, MAJOR 5); one head number is falsified by the document's own table (MAJOR 3); one is a MEASURED figure that was already stale in an ancestor commit of the round that copied it (MAJOR 2); one names the wrong home for a review whose 432-line report is in the tree (MAJOR 1); and one classification tells IMPL that a superseded-defect narrative is the state now (MAJOR 6). D-346 forbids rewording a false self-state claim; it does not forbid *authoring a new one*, and that is where all six live.

---

# FINDINGS

## BLOCKING

None.

---

## MAJOR

### 1. The head's carve-provenance clause, rewritten this round, says D-309 **"is that review's home"** — the revision-7 REVIEW-design is a 432-line report landed verbatim in the tree, and D-309's own text says it reproduces only two of that review's findings

**The claim reviewed — U4:15–16, verbatim (the u-rev 9 replacement for the clause `wp15b_U4_REVIEW_urev8.md` MINOR 5 charged):**

```
$ sed -n '15,16p' docs/experiments/U4_soundness_instrument.md
**u-rev 9.** Carved from `docs/experiments/wp15b_design.md` §8, §9, §11.6 and §12
item 1 at `6feb40a` (revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it, and is that review's home) under the
```

**Contradicting evidence — the report is in the tree, landed verbatim, at the same SHA D-309 closes:**

```
$ ls docs/experiments/ | grep rev7
wp15b_design_rev7_REVIEW.md
$ wc -l docs/experiments/wp15b_design_rev7_REVIEW.md
432 docs/experiments/wp15b_design_rev7_REVIEW.md
$ sed -n '1,18p' docs/experiments/wp15b_design_rev7_REVIEW.md
<!--
PROVENANCE — this file is a LANDED EPHEMERAL REPORT.

- Report:            REVIEW-design of WP-1.5b design revision 7.
...
- SHA the report examined: 6feb40af1f1c12c1977d7a2030509dd98cbdc8ac
                     (`docs/experiments/wp15b_design.md`, revision 7, 1975 lines;
                     that file is DELETED from the tree at cf74594 by the carve and
                     is retrievable at 6feb40a and nowhere else)
- Landed at:         2026-08-22, tree at cf74594.
- Cited by:          D-309 (closes revision 7 on this report), and the eleven-finding
                     summary in docs/experiments/restructure_selection_15b.md.
$ grep -n "VERDICT" docs/experiments/wp15b_design_rev7_REVIEW.md | head -1
39:## VERDICT: **FAIL**
$ git log --oneline --diff-filter=A -- docs/experiments/wp15b_design_rev7_REVIEW.md
7d0418e docs(experiments): the revision-7 review and the restructure red team land verbatim under docs/, each with the session, the tmpfs path it would have died on and the SHA it examined, so D-309 and D-310 cite a retrievable report instead of a scratchpad
```

**And D-309 disclaims being the home, in its own words:**

```
$ sed -n '663p' docs/decisions.md | cut -c1-560
D-309: WP-1.5b's DESIGN REACHED REVISION 7 AND FAILED ITS FRESH-CONTEXT REVIEW-design AT `6feb40a` — 7 BLOCKING, 7 MAJOR, 9 MINOR — ... The review's own ground for restructure rather than another pass is two findings taken together, and both are reproduced here because the report file is session-scoped: B1 — ... and B7 — ...
```

D-309 reproduces **two** of twenty-three findings, on a premise (*"the report file is session-scoped"*) that `7d0418e` has since falsified. D-346's own home list is *"`docs/experiments/wp15b_*_REVIEW*.md` for findings and verdicts"*; `wp15b_design_rev7_REVIEW.md` matches that glob and is the home.

**Why it breaks.** This is not a struck claim and not a pointer — it is a **new claim** written at u-rev 9 into the surface a reader meets first, and it names the wrong home under the very law (D-331, restated by D-346) this round is executing. Its consequence is concrete and reachable from inside this unit: U4-A:272 tells a reader that *"B1, B3, B4 and MINOR 15 are this unit's, and MAJOR 8 … is this unit's and is OPEN"*. A reader who follows the head to D-309 to read B3, B4, MINOR 15 or MAJOR 8 finds none of them — D-309 carries B1 and B7 only. The 432-line report that carries all of them is two directory entries away and the head steers past it. D-346's clause is *"a pointer and never a claim"*; the replacement clause is a claim, and it is false.

**Fix scope.** Replace *"and is that review's home"* with a pointer to `docs/experiments/wp15b_design_rev7_REVIEW.md`, or delete the trailing clause and let the D-309 reference stand alone.

---

### 2. The new OPEN-list D-311 bullet states **"`u-rev 8` name six texts 289 lines apart"** — MEASURED at HEAD it is **seven** texts and **291** lines, and it was already seven when u-rev 9 was authored

**The claim reviewed — U4:2170–2175, verbatim (new at u-rev 9, the repair of `wp15b_U4_REVIEW_urev8.md` MAJOR 7):**

```
$ sed -n '2170,2176p' docs/experiments/U4_soundness_instrument.md
- **THE u-rev LABEL BUMPS PER LANDED COMMIT, AND WHETHER A ROUND-LABEL MAY REPLACE
  THAT IS THE ARCHITECT'S.** u-rev 8 read D-311 as permitting one label across a
  multi-commit round; `wp15b_U4_REVIEW_urev8.md` MAJOR 7 MEASURED that reading against
  D-311's own recorded ground — one label naming two texts 69 lines apart is the
  ambiguity D-311 removes, and the round-label made `u-rev 8` name six texts 289 lines
  apart. The reading is struck and the per-commit bump is restored.
```

**Contradicting evidence — MEASURED, the span the head label `u-rev 8` actually governs:**

```
$ git show bb64501:docs/experiments/U4_soundness_instrument.md | sed -n '15p'
**u-rev 8.** Carved from `docs/experiments/wp15b_design.md` §8, §9, §11.6 and §12
$ git log --oneline --reverse 0f49c90..bb64501 -- docs/experiments/U4_soundness_instrument.md | wc -l
7
$ git diff 0f49c90 bb64501 --stat -- docs/experiments/U4_soundness_instrument.md
 docs/experiments/U4_soundness_instrument.md | 291 ++++++++++++++++++++++++----
 1 file changed, 255 insertions(+), 36 deletions(-)
```

255 + 36 = **291** changed lines across **seven** commits, the seventh being `bb64501`, which appended to the unit and left the head label at `u-rev 8`.

**And `bb64501` is an ancestor of the commit that wrote this bullet:**

```
$ git merge-base --is-ancestor bb64501 3ced890 && echo "bb64501 IS an ancestor of the u-rev 9 commit"
bb64501 IS an ancestor of the u-rev 9 commit
```

**Why it breaks.** *(i)* It is a MEASURED number, presented without qualification, that is **false at HEAD** and was false at the moment it was written — the exact defect class D-305 names and the reason D-331 makes a report the home of its own findings. The report's `six / 289` was correct at its pinned revision `a2b50bf`; copying it forward without re-taking it is the "second copy that is true only at the revision it is written" D-331's diagnosis describes. *(ii)* The subject of the sentence is **the labelling rule itself**, so the error falsifies the bullet's own argument in its own favour: the round-label's harm is one line *larger* and one commit *worse* than the bullet claims. *(iii)* The unit's own head rule (U4:132–134) is that *"a mark added at u-rev 6 or later names the command that took it and pastes its complete output"*; this figure carries neither and is not marked MEASURED or ESTIMATED at its site.

**Fix scope.** Either re-take the two figures at HEAD (seven texts, 291 lines) with the commands pasted, or drop them entirely and point at `wp15b_U4_REVIEW_urev8.md` MAJOR 7, which is their home and which carries them correctly for its own pinned revision.

---

### 3. The change log's u-rev 9 entry says **"Eleven such claims went at this u-rev"** — the unit's own REVIEW STATUS block records **twenty** struck items across ten rows, and no reading of "such claims" yields eleven

**The claim reviewed — U4:126–131, verbatim (new at u-rev 9):**

```
$ sed -n '126,132p' docs/experiments/U4_soundness_instrument.md
  block below says where each is answered. **WHAT IS NEW IN THE INSTRUMENT:** a claim
  this unit made about ITS OWN STATE and that a reviewer found false is now DELETED
  rather than reworded, with a pointer to the external home where its removal strands
  navigation. Eleven such claims went at this u-rev, and the largest of them — this
  unit's own review lineage, kept in U4-A and again in the OPEN list — is gone from
  both, because keeping it in two places is what put it one round behind three times
  running.
```

**Contradicting evidence — the u-rev 9 table's own accounting of what it struck:**

```
$ sed -n '210,226p' docs/experiments/U4_soundness_instrument.md | grep -c 'STRUCK'
10
```

Enumerating the objects those ten rows name as struck, from the rows verbatim:

| row | struck objects named | count |
|---|---|---|
| BLOCKING 1 | *"The other three are STRUCK"* | 3 |
| MAJOR 1 | *"The change-log clause, STRUCK"* | 1 |
| MAJOR 3 | *"Both halves STRUCK: the claim itself, and the six surviving universals"* | 7 |
| MAJOR 4 | *"The locality claim STRUCK"* | 1 |
| MAJOR 5 | *"STRUCK, replaced with nothing"* | 1 |
| MAJOR 6 | *"The row's clause STRUCK"* | 1 |
| MAJOR 7 | *"The self-exemption paragraph STRUCK"* | 1 |
| MINOR 1 | *"All three STRUCK"* | 3 |
| MINOR 3 | *"The count STRUCK"* | 1 |
| MINOR 5 | *"the … clause STRUCK"* | 1 |
| | **total** | **20** |

**I tried every restriction the sentence's own wording licenses and none reaches eleven.** Restricting to claims *about this unit's own state* removes MINOR 1's two claims about **other** documents (the `## Travelling items` heading, the "fiftieth line"), MINOR 3's count of **another report's** enumeration, and MAJOR 1's attribution to an **ADR line** → **16**. Counting rows rather than claims → **10**. Counting only the six universals plus their parent claim plus MAJOR 4/5/7 → **10**. Eleven is not a value this document's own record produces.

**Why it breaks.** It is a fresh, non-derivable, false claim about the document's own state, written in the sentence that announces the policy against exactly that shape — D-335's generator (2), *"a universal about the document's own state, false in the commit that asserts it"*, committed by the commit executing D-346. And it is outside gate 15's subject: gate 15's second check covers summand lines (`20 + 4 + 5 = **32**`), group-count lines (`**U2 (20):**`) and summands against table rows, none of which this is —

```
$ sed -n '44,60p' tools/label_consistency_check.sh | grep -n "THE SUMMAND LINE\|THE GROUP-COUNT LINE\|THE SUMMANDS AGAINST"
11:#   1. THE SUMMAND LINE — `20 + 4 + 5 + 3 = **32**`. The arithmetic must hold,
15:#   2. THE GROUP-COUNT LINE — `**U2 (20):**` introducing a backtick-quoted list.
18:#   3. THE SUMMANDS AGAINST THE TABLE ABOVE THEM. Where a summand line sits under
```

so nothing mechanized will catch it either.

**Fix scope.** Delete the cardinal. The sentence works without it (*"Claims of that shape went at this u-rev, and the largest of them …"*), and the REVIEW STATUS block below is the derivable record.

---

### 4. The REVIEW STATUS row for MAJOR 3 says **"Both halves STRUCK: the claim itself"** — the claim was **reworded in place**; there is no strikethrough, no `WITHDRAWN` token and no deletion at the site the row names

**The claim reviewed — U4:216, verbatim:**

```
$ sed -n '216p' docs/experiments/U4_soundness_instrument.md
| **MAJOR 3** — *"every universal … is withdrawn and none is replaced"* is false by enumeration | Both halves STRUCK: the claim itself, and the six surviving universals the report enumerated |
```

**Contradicting evidence — the site, before and after:**

```
$ git show a2b50bf:docs/experiments/U4_soundness_instrument.md | sed -n '1935,1937p'
  D-331. **What u-rev 8 does about it is a REMEDY and not a rule:** every universal in
  this unit's REVIEW STATUS table is withdrawn and none is replaced by a narrower one,
  and the head's citation claim states its bound instead of asserting a set.
$ sed -n '2013,2017p' docs/experiments/U4_soundness_instrument.md
  properties of a body it does not own). **What u-rev 8 and u-rev 9 do about it is a
  REMEDY and not a rule:** the universals `wp15b_U4_REVIEW_urev8.md` MAJOR 3 enumerated
  are struck under D-346 and none is replaced by a narrower one, and this bullet
  certifies no set — which universals survive anywhere in this unit is a reviewer's
  sweep, not a claim this list may assert.
$ sed -n '1998,2018p' docs/experiments/U4_soundness_instrument.md | grep -c '~~'
0
```

The sentence is intact in structure and position; only its quantifier moved, from *"every universal in this unit's REVIEW STATUS table"* to *"the universals `wp15b_U4_REVIEW_urev8.md` MAJOR 3 enumerated"*. That is a **narrowing reword** — the operation D-346 exists to forbid for this class — and the row reports it as a strike. The contrast is visible against the round's own genuine strikes, which all carry `~~…~~` plus a disposition token:

```
$ grep -c '~~' docs/experiments/U4_soundness_instrument.md
16
$ sed -n '303p' docs/experiments/U4_soundness_instrument.md | cut -c1-140
| **BLOCKING 3** — the staged-config count restated at four sites, one of them inside the clause naming U3 §10 as the only place it may be stated | *~~
```

**Why it breaks.** *(i)* The row is the round's headline instrument and its lead-in is *"THE ROWS BELOW SAY WHERE EACH FINDING IS ANSWERED"*; a row that reports a reword as a strike is the same defect the prior round graded MAJOR 6 (*"a status row that misdescribes the state at the site it names is worse than the unmarked number, because it is the surface a next round reads to decide the finding is closed"*). *(ii)* It matters more here than in the general case, because **D-346's entire ground is that a reworded status claim is a new claim that can go false while a deleted one cannot**. The one site where this round chose to reword rather than delete is reported to the next reviewer as a deletion, which conceals the round's only departure from its own policy. *(iii)* Half two of the row **is** accurate — I verified all six enumerated universals are gone or struck-through:

```
$ for p in "asserts no line count of itself" "Nothing is silently re-labelled" "no live statement of the count survives" "returns nothing, and the only rule-5"; do grep -c "$p" docs/experiments/U4_soundness_instrument.md; done
0
0
0
0
```

so the row is wrong about exactly one of its two halves.

**Fix scope.** Correct the row: *"the six enumerated universals STRUCK; the claim itself narrowed at its home to the enumerated set."* Or strike the parent claim as the row says and let the bullet's *"this bullet certifies no set"* carry it alone.

---

### 5. The REVIEW STATUS row for MAJOR 1 says **"U4-Z's four-conditions bullet is that residual's one home"** — D-331 clause (1) gives the home to the selection record, and the bullet's own words say so

**The claim reviewed — U4:212, verbatim:**

```
$ sed -n '212p' docs/experiments/U4_soundness_instrument.md
| **MAJOR 1** — the head's change log re-attributes the relative-base residual to `D-329` | The change-log clause, STRUCK. U4-Z's four-conditions bullet is that residual's one home and already attributes it correctly |
```

**Contradicting evidence — D-331's binding clause (1), at HEAD:**

```
$ sed -n '707p' docs/decisions.md | grep -o "where a claim has landed in \`docs/decisions.md\` or in a selection record, THAT is its home and no unit may restate it"
where a claim has landed in `docs/decisions.md` or in a selection record, THAT is its home and no unit may restate it
```

**And the site the row names says the same thing about itself:**

```
$ sed -n '2036p' docs/experiments/U4_soundness_instrument.md | grep -o "it is recorded by the SELECTION RECORD and not by D-329\*\* — \`docs/experiments/matrix_M4_axisA_selection.md\` condition 4"
it is recorded by the SELECTION RECORD and not by D-329** — `docs/experiments/matrix_M4_axisA_selection.md` condition 4
```

The residual landed in `matrix_M4_axisA_selection.md` condition 4 — a selection record. Under D-331 that record **is** its home, and U4-Z's bullet is a pointer, not a home.

**Why it breaks.** This is the third consecutive round in which this one residual's attribution has been got wrong in this unit: `wp15b_U4_REVIEW_urev7.md` MAJOR 3 (attributed to D-329 at the bullet), `wp15b_U4_REVIEW_urev8.md` MAJOR 1 (re-attributed to D-329 in the change log), and now the row answering MAJOR 1 assigns it a home D-331 forbids a unit to hold. The consequence is not cosmetic: a bullet declared a claim's *home* is licensed to carry the claim's content, and this bullet does carry it in full (*"a relative `--config` resolved against `$ROOT` while a relative `--out` resolved against `$CALLER_PWD`…"*). Calling that a home converts a restatement the round should be retiring into one it has ratified.

**Fix scope.** *"the residual's home is `matrix_M4_axisA_selection.md` condition 4; U4-Z's four-conditions bullet points at it and attributes it correctly."*

---

### 6. The §8 record stamp's new item **(vi)** is added under the header **"these are the state now, and IMPL reads them as such"** — the block it admits is a narrative of a defect repaired at u-rev 2, and it contains the present-tense *"so S-E **is** gate (b)'s instrument"*

**The claim reviewed — U4:513–525, verbatim (item (vi) is new at u-rev 9):**

```
$ sed -n '513,525p' docs/experiments/U4_soundness_instrument.md
> **WHAT IS NOT RECORD — these are the state now, and IMPL reads them as such:**
> **(i)** §8.2's opening `FOLDED AT u-rev 6` block, which states the selected
...
> taken from**; and **(vi)** §8.7's opening `THE DEFECT` blockquote, which is CARVE
> PROSE this unit may edit and did edit at u-rev 6 (MINOR 9) and at u-rev 8 (MAJOR 1),
> and which the stamp did not classify in either direction until u-rev 9
> (`wp15b_U4_REVIEW_urev8.md` MINOR 6). Each of the six says at its own site which it
> is.
```

**Contradicting evidence — what the admitted block actually contains:**

```
$ sed -n '968,978p' docs/experiments/U4_soundness_instrument.md
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
$ sed -n '979,980p' docs/experiments/U4_soundness_instrument.md
> and its matrix was headed `| Option | (b)'s instrument |` — **so S-E *is* gate
> (b)'s instrument** *(reported speech, revision 1)*, and "(a)–(d) plus S-E" counted it once as (b) and once as
```

**Why it breaks.** The stamp is a **binary classification with an IMPL-facing consequence stated in its own headers**: RECORD = *"true at the revision that wrote it, superseded now, and to be read as the state AT THAT REVISION"* (U4:503); NOT-RECORD = *"these are the state now, and IMPL reads them as such"* (U4:513). The §8.7 `THE DEFECT` blockquote is neither: it is **editable** carve prose (which is item (vi)'s stated reason) but it is **not the state now** — its subject is a wiring sentence B3 repaired at u-rev 2 under D-316, and its live sentence at U4:979 is a present-tense identification of the differential gate as **S-E**, a row that FELL in M3 round 1 and which D-323 superseded with S-M. Item (vi) therefore admits into the "state now" list the one block the same round had to mark *(reported speech, revision 1)* precisely because it is **not** the state now.

Note also that item (vi) does not use the header's criterion at all — items (i)–(v) are justified by currency (*"states the selected instrument"*, *"states what that gate's instrument IS"*, *"the live specification `tools/staged_soundness_check.sh` is taken from"*), while (vi) is justified by editability. The header's universal is now false of its own newest member, and the two criteria are silently merged under one heading.

This is a normative error, not a status error: the stamp exists so an IMPL session knows what to trust, and MINOR 6 of the prior report named that consequence exactly — *"a reader deciding whether to trust or to correct the paragraph consults the stamp"*. The reviewer's suggested fix scope (add it to NOT-RECORD) is not binding on the truth of the header it lands under.

**Fix scope.** Either qualify the header so it states the two criteria it now carries (*"editable by this unit; items (i)–(v) are additionally the state now"*), or give (vi) a third classification — editable-but-historical — which is what it is, and which the block already says at its own site.

---

## MINOR

### 1. The MAJOR 5 repair site says the struck universal **"is the fourth of its shape struck from this unit"** — six siblings of the same shape were struck in the same commit

**The claim reviewed — U4:1655–1662, verbatim (new at u-rev 9):**

```
$ sed -n '1655,1662p' docs/experiments/U4_soundness_instrument.md
> **BLOCKING 1** of `wp15b_U4_REVIEW_urev6.md`. *~~So at u-rev 7 every claim below
> whose truth depends on an ADR line was re-read against `docs/decisions.md` as it now
> stands, item by item, rather than the two sentences the report names being edited.~~
> **STRUCK AT u-rev 9 AND REPLACED WITH NOTHING** (`wp15b_U4_REVIEW_urev8.md` MAJOR 5).
> Re-derivation was u-rev 7's INSTRUMENT, and `wp15b_U4_REVIEW_urev7.md` MAJOR 2, 3 and
> 4 are three claims it did not reach; a completeness universal standing at the head of
> the section whose re-derivation was incomplete is the defect one level up, and it is
> the fourth of its shape struck from this unit.*
```

**Contradicting evidence — the same commit struck six more of the same shape, and the round's own sibling row says so:**

```
$ sed -n '303p' docs/experiments/U4_soundness_instrument.md | grep -o "a completeness universal beside an enumeration is the shape three earlier rows in these tables were struck for"
a completeness universal beside an enumeration is the shape three earlier rows in these tables were struck for
$ git log --oneline -1 3ced890 --format=%s
docs(experiments): U4 reaches u-rev 9 — its 1 BLOCKING and 14 other findings answered under STRIKE-AND-DISPOSE, eleven self-state claims deleted rather than reworded, and the round-label self-exemption from D-311 struck
```

Both this site and the BLOCKING 3 row take the same three pre-u-rev-9 strikes (U4:230, :234, :245 — the instance base D-346 cites) as their baseline, and both land in commit `3ced890`. There is no fact of the matter about which of the round's simultaneous strikes is "the fourth"; by the enumeration in MAJOR 3 above there are ten of the shape in this unit, seven of them at this u-rev.

**Why it breaks.** It is a self-state ordinal, non-derivable, asserted in the disposition of a strike — small, but it is the same generator as MAJOR 3 above and it will be copied forward by the next round exactly as `six texts / 289 lines` was.

**Fix scope.** Delete *"and it is the fourth of its shape struck from this unit"*. The sentence before it carries the whole argument.

---

### 2. The REVIEW STATUS row for MINOR 6 credits **"the u-rev 9 ruling"** as item (vi)'s reason — the carve-prose ruling is u-rev 8's, stated inside the block, and item (vi) cites it as such

**The claim reviewed — U4:225, verbatim:**

```
$ sed -n '225p' docs/experiments/U4_soundness_instrument.md
| **MINOR 6** — the §8 record stamp does not classify the §8.7 blockquote | Added to the NOT-RECORD list as item **(vi)**, with the u-rev 9 ruling as its reason |
```

**Contradicting evidence — where the ruling was actually made, and what item (vi) gives as its reason:**

```
$ sed -n '995,999p' docs/experiments/U4_soundness_instrument.md
> *(CORRECTED AT u-rev 8 — MAJOR 1 of `wp15b_U4_REVIEW_urev7.md`. Until u-rev 7 this
...
$ sed -n '1000,1002p' docs/experiments/U4_soundness_instrument.md | grep -o "It is corrected IN PLACE, not annotated, because this block is carve prose"
It is corrected IN PLACE, not annotated, because this block is carve prose
$ sed -n '522p' docs/experiments/U4_soundness_instrument.md
> PROSE this unit may edit and did edit at u-rev 6 (MINOR 9) and at u-rev 8 (MAJOR 1),
```

Item (vi)'s stated reason is that the block is carve prose edited at **u-rev 6** and **u-rev 8**. The carve-prose ruling itself is made at u-rev 8, inside the block. The prior report's own fix scope reads *"Add it to the NOT-RECORD list, with the **u-rev 8** ruling as its reason."* Under the alternative reading — that "the u-rev 9 ruling" means the classification made at u-rev 9 — the row states the classification as its own reason, which is circular.

**Why it breaks.** A status row naming the wrong u-rev for a ruling is small, but this table is the surface a next round reads to decide a finding is closed, and the whole class this unit keeps failing on is u-rev attribution.

**Fix scope.** *"with the u-rev 8 carve-prose ruling as its reason."*

---

### 3. The new finding aid's second stage (`grep -v 'landed'`) drops a live old-form `U3 §10` citation, and the document's account of the aid's blindness names only the pattern

**The claim reviewed — U4:166–178, verbatim (new at u-rev 9):** the paragraph beginning *"WHAT IS OFFERED IS A FINDING AID, LABELLED AS ONE, AND IT IS WIDER THAN THE CITATION FORM"*, its command block `grep -nE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md | grep -v 'landed'`, and the disclaimer *"THIS UNIT DOES NOT CLAIM THAT COMMAND IS COMPLETE EITHER. A reference shape it does not match would be invisible to it as well…"*

**Credit where it is due — the widening works.** MEASURED, the new pattern surfaces all three sites the prior report named as blind spots (they have moved from 665/1476/1477 to 725/1539/1540):

```
$ grep -n 'U[123]-T' docs/experiments/U4_soundness_instrument.md | cut -c1-70
171:`U2-T`/`U3-T` and to every unbolded reference, which is the blindness
725:generated cell set at both phases (U2-T). D-124's flip clause reads *"flips when that
1539:Carried from the superseded §11. The rows this unit does not own are in U2-T,
1540:U3-T and `WPQ_seed.md`, and no row is in two places.
$ for L in 725 1539 1540; do grep -nE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md | grep -v 'landed' | grep -c "^${L}:"; done
1
1
1
```

**Contradicting evidence — the residual blindness is in the second stage, not the pattern:**

```
$ grep -cnE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md
36
$ grep -nE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md | grep -v 'landed' | wc -l
29
$ grep -nE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md | grep 'landed' | cut -c1-118 | sed -n '3p'
303:| **BLOCKING 3** — the staged-config count restated at four sites, one of them inside the clause naming U3 §10 as the only place it may
$ sed -n '303p' docs/experiments/U4_soundness_instrument.md | grep -o 'U3 §10\|U3\*\* (u-rev 6, landed `13621d3`)'
U3 §10
U3** (u-rev 6, landed `13621d3`)
```

Line 303 — a line this round edited — carries a **bare `U3 §10`**, exactly the old-form citation the aid exists to surface, and the aid discards the whole line because the *same* line also carries a converted citation containing the word `landed`.

**Why it breaks.** The row for MINOR 2 claims the aid *"says what it is and is not blind to"*, and the disclaimer attributes the aid's incompleteness solely to *"a reference shape it does not match"*. Here the shape **is** matched; the loss is at the filter, which the disclaimer does not name. Since *"Converting the live ones is OPEN"* and this aid is how a future round will enumerate what is left to convert, a reader running it will conclude line 303 is already converted.

**Fix scope.** One clause naming the filter (*"and a line carrying both an old-form and a converted citation is dropped by the `landed` filter"*), or drop the filter and let the reader triage the seven extra lines.

---

### 4. The D-334 bullet asserts what a revision 3 must contain **"IS NOT COUNTED OR RESTATED HERE"** and then names one of the enumerated items two clauses later

**The claim reviewed — U4:1988–1995, verbatim (new at u-rev 9):**

```
$ sed -n '1988,1995p' docs/experiments/U4_soundness_instrument.md
  `wp15b_U4_REVIEW_urev7.md`. **WHAT A REVISION 3 MUST CONTAIN IS ENUMERATED IN
  `docs/experiments/matrix_U4R_REDTEAM_round2.md`, under *"What a revision 3 would have
  to contain"*, AND IS NOT COUNTED OR RESTATED HERE** — the count that stood here said
  two and the report's own items say more (`wp15b_U4_REVIEW_urev8.md` MINOR 3). The one
  item this unit names, because two independent rounds MEASURED it to be the only
  candidate whose prevention claim is a mechanism rather than an assertion, is **the
  DERIVED-ENUMERATION row on the landed `7dfd047` precedent**; D-345 records that it is
  now missing from three consecutive fields and that it is the architect's.
```

**Contradicting evidence.** The pointer target resolves and the count is genuinely gone —

```
$ grep -n "What a revision 3 would have to contain" docs/experiments/matrix_U4R_REDTEAM_round2.md | cut -c1-90
1000:**What a revision 3 would have to contain**, stated so the next round is not a fourth
$ sed -n '735p' docs/decisions.md | grep -o "R3 — the derived-enumeration row, the only candidate two independent rounds have MEASURED to be a mechanism rather than an assertion — is now missing from three consecutive fields"
R3 — the derived-enumeration row, the only candidate two independent rounds have MEASURED to be a mechanism rather than an assertion — is now missing from three consecutive fields
```

— but item 2 of that enumeration (*"The DERIVED-ENUMERATION row (K5)"*) is restated by name in the next sentence. The universal *"IS NOT … RESTATED HERE"* quantifies over the enumeration and one member of the enumeration is restated immediately below it.

**Why it breaks.** This is the shape the prior round graded MAJOR 4 (*"STATED AT §9.1 AND NOWHERE ELSE"*, falsified by the two sentences after it in the same bullet), re-authored one bullet away, at u-rev 9. It is graded MINOR rather than MAJOR only because the text acknowledges the exception in the same breath (*"The one item this unit names"*), so no reader is misled — but the universal is still false of its own bullet, and the next round will read it as a certification.

**Fix scope.** *"…AND IS NOT COUNTED HERE; this unit names one of its items."*

---

# VERIFIED WITH NO FINDING

**The D-346 flip clause does not fire. I hunted for it deliberately, strike by strike, and found no normative claim homeless — this is the strongest result of the round and it should be recorded as such.** For each strike I asked whether what went was status matter or something IMPL, a design decision, a binding rule or a statement of what is owed depends on:

1. **U4-A's two REVIEW-design rows and the OPEN list's review-history enumeration (BLOCKING 1).** What went was pinned revisions, verdicts and counts — status matter, home = the reports. What IMPL needs and what is **retained** at U4:2179–2181: *"No REVIEW-design has run against this text at THIS u-rev, and a review of a superseded revision does not transfer — an amendment reopens the review, however small the diff."* The binding rule survives; only the history left.
2. **The u-rev 5 table's owner-table row (BLOCKING 1).** The struck sentence carried *"a re-measurement is owed to the owner table again"* — a statement of what is owed. It is **retained** in the same cell: *"**DISCHARGED AT `a0f241b`, AND IT RE-OPENS WITH EVERY BUMP OF THIS UNIT — including this one.** **What is recorded here is that STANDING DUTY, not a size and not a u-rev**"*. No loss.
3. **The change-log's R19 clause (MAJOR 1).** *"CLOSED … by architect ruling R19 at `63eac4c`, whose REVIEW-impl PASSED at `d59f0de`"* — IMPL depends on this (N-E inherits nothing). **Retained in full** at U4:2036: *"**CLOSED AT `63eac4c`, BEFORE N-E IS BUILT** — architect ruling R19 … its REVIEW-impl PASSED with 0 BLOCKING and 0 MAJOR at `d59f0de`. So N-E has nothing here to inherit."*
4. **The `tools/`-review bullet's restatements (MAJOR 4).** This was my strongest flip-clause candidate: the struck text said *"§9.1 amendment 2's 'eight of twelve' was counted for **N-A**; D-329 … **REMOVES item 11**"*. Both pointers resolve and both preserve the content: §9.1's own section heading is *"### 9.1 The five amendments the design made to **N-A** after that attack"* (U4:1448), so the N-A qualification survives at the target; and the F13 paragraph at U4:1091–1093 names item 11 by number and states why item 9 governs instead. An IMPL session following the two pointers reaches everything the struck sentences carried. **No finding.**
5. **The MAJOR 12 / MAJOR 9 row's verification (MAJOR 3, item 5 of six).** The struck text was `grep` evidence; what survives is the normative half — *"recorded MAJOR 9's non-discharge as an IMPL gate rather than a design defect"* — plus a pointer to `wp15b_U4_REVIEW.md` where the commands live. No loss.
6. **The self-exemption paragraph (MAJOR 7).** Its one quasi-normative sentence (*"the revision a review is dispatched against is named … by its SHA, and that is the binding identifier"*) is redundant against the LABEL DISCIPLINE paragraph at U4:145–150, which survives and which the REVIEW STATUS blocks continue to honour with named SHAs.
7. **MINOR 5's struck clause, MINOR 3's struck count, MINOR 1's three struck errors.** All status or false-about-other-documents; nothing normative in any of them.

**Further verifications that produced no finding:**

- **The repaired D-331 bullet in U4-Z attributes every finding to the right report.** MEASURED:
  ```
  $ grep -n '^### ' docs/experiments/wp15b_U3_REVIEW_urev4.md | cut -c1-95
  32:### A. The MAJOR-2 repair's own new citation in §6.5 is an unlisted B7/MAJOR-4-class site
  59:### B. The MINOR-5 repair's "U2 is at u-rev 2" is false; U2 was already at u-rev 3 before this
  $ grep -n '^### ' docs/experiments/wp15b_U3_REVIEW_urev5.md | cut -c1-95
  59:### C. "Every cross-unit citation in this unit now reads `(u-rev N, landed <sha>)`" is false
  ```
  MAJOR A and B are `wp15b_U3_REVIEW_urev4.md`'s; MAJOR C is `wp15b_U3_REVIEW_urev5.md`'s — exactly as U4:2007–2009 now states. The characterisations hold too: B is a stale second copy of U2's u-rev (a D-331 restatement, clauses 1–4 reach it); A is the B7 site-table completeness claim and C is a completeness universal over the citation set (both originally-authored, non-copied). The re-framing from *"D-331 … is silent about"* to *"D-331 HAS NO BINDING CLAUSE FOR"* is correct against D-331's ground section, which names A and B by review citation.
- **MAJOR 6's repair is real at both ends.** `grep -n "91 test lines"` returns only U4:248 (the disposition row) and U4:1116 (the measurement itself); U4-Z's condition (3) now points at *"§9's condition 3"*, and that condition does paste `git show b067d47 --numstat` with its complete output at U4:1110–1116.
- **MINOR 4's two D-333 pointers are accurate against D-333.** `docs/decisions.md:711` reads *"R18 RULES THAT THIS DOES NOT REOPEN THE SELECTION"* — which is what U4:1157–1159 and U4:288–289 now say.
- **MINOR 1's two surviving `file:line` pointers resolve and carry what the struck text claimed.** `restructure_matrix_15b.md:35` → `T2. M4 ADR line (B2).`; `restructure_selection_15b.md:50` → `T2. M4 ADR line (B2). T3. …`.
- **MINOR 7's repair is complete at both halves.** U4:979 now carries the inline mark *(reported speech, revision 1)*; and the parenthetical's emphasis now balances — it terminates `MINOR 7.**)*` (bold close, paren, italic close) where at `a2b50bf` it terminated `twice.*)` with the `**` opened at *"The paragraph above it"* unclosed.
- **BLOCKING 1's `bb64501` claim is true.** `git show bb64501 -- <subject>` is a one-line change to U4:2105, rewriting the closing line, and `bb64501` is an ancestor of `3ced890`.
- **The MINOR 3 pointer target exists** at `matrix_U4R_REDTEAM_round2.md:1000`, under the heading the unit quotes verbatim.
- **Gate 15's subject was not re-derived** and no head/foot or summand claim is reported, per scope.

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

### RJ1. *"The `tools/`-review bullet's new `this bullet points at both and restates neither` is falsified by the `eight of twelve` still inside the bullet"* — REJECTED

**Attempted reproducer:**

```
$ sed -n '2069,2085p' docs/experiments/U4_soundness_instrument.md | grep -n "eight of twelve"
12:  *(Until u-rev 8 this bullet said the reopening engages "eight of twelve … plus D-329's
```

The cardinal does still appear inside the bullet. **But D-331 exempts it explicitly:** *"it does not forbid QUOTATION — a marked verbatim quotation carrying its source and revision is a pointer that brings its referent with it"* (`docs/decisions.md:707`). The occurrence is inside `*(Until u-rev 8 this bullet said …)*` — a marked quotation of the bullet's own withdrawn wording, carrying its revision. It asserts nothing about the current engaged set. Not a restatement; rejected.

### RJ2. *"`THE LABEL BUMPS PER LANDED COMMIT AGAIN` is falsified by `bb64501`, which appended without bumping"* — REJECTED

**Attempted reproducer:**

```
$ git show bb64501 --stat --format= -- docs/experiments/U4_soundness_instrument.md
 docs/experiments/U4_soundness_instrument.md | 2 +-
$ git show bb64501:docs/experiments/U4_soundness_instrument.md | sed -n '15p' | cut -c1-14
**u-rev 8.**
```

`bb64501` is indeed a landed commit that appended to the unit and left the head label unbumped. But the claim is *"BUMPS … **AGAIN**"* and the OPEN bullet's companion sentence is *"the per-commit bump is **restored**"* — both forward-looking, and `bb64501` predates the restoration. u-rev 9 itself is one commit (`3ced890`) and honours the rule. The claim is normative, not a history claim, and nothing at HEAD falsifies it. Rejected. *(The seventh-commit fact is not lost: it is the falsifier of MAJOR 2 above, where it does bear.)*

### RJ3. *"Striking the MAJOR 4 restatements left the engaged-checklist set homeless for IMPL"* — REJECTED

**Attempted reproducer:**

```
$ sed -n '1448p' docs/experiments/U4_soundness_instrument.md
### 9.1 The five amendments the design made to N-A after that attack
$ sed -n '1489p' docs/experiments/U4_soundness_instrument.md
**SHELL_CHECKLIST items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve**,
$ sed -n '1091,1093p' docs/experiments/U4_soundness_instrument.md
> **N-Q's extra lines are required by no rule in this tree (F13):** item 11's scope is
> *"any binding consumed by `rm`, `mv`, or a write"* and `$CONFIG` is a READ, so item 9
> governs it and is discharged by the whole-path guard **both** rows owe.
```

I expected to find that the strike removed the *"counted for N-A"* qualification and the identification of item 11 with nothing carrying them. Both survive at the pointer targets — the qualification in §9.1's own section heading, item 11 by number in the F13 paragraph. D-346's flip clause does not fire here. Rejected.
