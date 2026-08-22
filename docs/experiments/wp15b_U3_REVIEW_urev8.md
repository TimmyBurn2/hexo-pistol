# REVIEW-design (re-review) — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md` u-rev 8

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer (only HTML-escaped angle brackets restored: `&gt;`/`&lt;` to `>`/`<`).
The dispatching session repaired U3; it did not review it. Reports live in the
tree rather than in a scratchpad because finding IDs are cited across units.
-->

## Header

- **Pinned revision:** `1964026` (`git rev-parse 1964026` → `1964026c8efe89a4fea09f8e5c499cd40b7d9c42`).
- **Matches HEAD:** **YES.** MEASURED, `git rev-parse HEAD` → `1964026c8efe89a4fea09f8e5c499cd40b7d9c42`. `git status --porcelain` → empty.
- **Subject moved from pin:** **NO.** MEASURED, `git diff 1964026 HEAD -- docs/experiments/U3_tier_t.md | wc -l` → `0`. `git merge-base --is-ancestor 173397d 1964026` → true (exit 0): the u-rev-8 commit is an ancestor of the pin, and the pin's own last commit (`1964026`, U1 u-rev 4) is unrelated to U3.
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 8**, landed at `173397d`. MEASURED, `wc -l` → **1007 lines**.
- **Prior reports read:** `docs/experiments/wp15b_U3_REVIEW_urev6.md` (u-rev 6, `f0ae14c`, FAIL — 0 BLOCKING/2 MAJOR/0 MINOR, answered at u-rev 7) and `docs/experiments/wp15b_U3_REVIEW_urev5.md` (u-rev 5, `7473a6f`, FAIL — 0 BLOCKING/1 MAJOR/0 MINOR). No report exists for u-rev 7; u-rev 8 answers no review report — it is a `D-346` strike-and-dispose repair.
- **Context freshness:** FRESH. I did not write this unit, its u-rev 7/8 repairs, or any prior review of it, and had not read any of them before this dispatch.
- **Scope, as given:** narrower than a full sweep. The absence of struck text is not a finding; a pointer is not a finding; Gate 15's territory (head/foot u-rev agreement, self-counts) is not re-derived. In scope: (a) a false or drifted claim in surviving text, including a claim this round's own edits introduced; (b) a normative claim that lost its home to a strike; (c) a normative error.
- **Diff examined:** `git diff a2b50bf 173397d -- docs/experiments/U3_tier_t.md` (u-rev 7 → u-rev 8), full hunk-by-hunk, reproduced below where load-bearing.
- **Sibling u-revs checked against the tree:** MEASURED, U1 head → `**u-rev 4.**`; U2 head → `**u-rev 7.**`; U4 head → `**u-rev 9.**` — matching the dispatch's stated state.

## VERDICT

**FAIL — 0 BLOCKING, 2 MAJOR, 0 MINOR.**

(Counts by section: `## BLOCKING` has zero `### ` entries; `## MAJOR` has two, `### F` and `### G`; `## MINOR` has zero — grep-derived against this report's own final markdown: `grep -c '^### '` under each heading equals the stated count.)

u-rev 8's three explicit strikes are each individually sound: the head's "the counts are D-309's and are not restated here" is false against U3-A's own revision-7 row (which prints `7 BLOCKING, 7 MAJOR, 9 MINOR` verbatim); the §6.5 "and are not restated here" is false against the same sentence, which prints `3.1× to 2.4×` two clauses earlier; and the MAJOR-D row's "this head asserts no universal about its own citations at all" is self-refuting exactly as u-rev 8 diagnoses. No normative claim was stranded by any of the three strikes, and the §6.5 replacement correctly uses a pointer (§12 item 5; the MAJOR-2 finding) because removing the struck clause there would otherwise orphan the "which rule" reference.

But the repair manufactured two fresh defects of the exact shape this document has now failed on for four consecutive prior rounds. **First (MAJOR F):** the head's strike of "the counts are D-309's and are not restated here" was not replaced with nothing (navigation was not stranded — the sentence is complete without it) but with a new, false claim: "and is that review's home." D-309 is not that review's home; the review's home is the landed file `docs/experiments/wp15b_design_rev7_REVIEW.md`, which D-309 itself says it only partially reproduces ("because the report file is session-scoped"), and which D-346 — the very policy this repair executes under — names as the home for "findings and verdicts" as opposed to `docs/decisions.md`'s rulings. **Second (MAJOR G):** the round's own preamble claims three claims were struck and "nothing replaces them," but a fourth, materially identical instance of the same false universal — "it asserts no universal about its own citations" — survives unstruck four lines into U3-Z's OPEN list (line 854), by the round's own stated reasoning for why the MAJOR-D-row instance is false.

---

# FINDINGS

## BLOCKING

None.

## MAJOR

### F. The u-rev 8 replacement clause "and is that review's home" is a new, false claim about D-309 — introduced by this round's own strike, not required to preserve navigation, and contradicts D-346's own homes taxonomy

**Claim reviewed** (head, lines 15–16, new at u-rev 8):

> **u-rev 8.** Carved from `docs/experiments/wp15b_design.md` §6, §10 and §12
> items 4 and 5 at `6feb40a` (revision 7, CLOSED by D-309 — which records the
> fresh-context REVIEW-design that FAILED it, **and is that review's home**),
> plus the two bullets of §7.2 that are not widening text…

**What u-rev 7 said, for contrast** (`git show a2b50bf:docs/experiments/U3_tier_t.md` lines 15–16, the text this round struck):

> …(revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design
> that FAILED it; **the counts are D-309's and are not restated here**), plus
> the two bullets…

The struck clause ("the counts are D-309's and are not restated here") was correctly identified as false — MEASURED, U3-A's own revision-7 row (line 192) prints `**FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR`, restating exactly those counts. But striking it and replacing it with nothing would have left a complete, self-sufficient sentence: *"(revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it)"* — no navigation is stranded, since D-309 is still named for the reader to follow. `D-346` licenses a pointer only "if and only if its removal strands navigation." No such stranding exists here, yet the repair inserted a new claim instead of nothing.

**Contradicting evidence — the claim is false.** MEASURED, the actual review's provenance header (`docs/experiments/wp15b_design_rev7_REVIEW.md`, lines 1–17):

```
PROVENANCE — this file is a LANDED EPHEMERAL REPORT.

- Report:            REVIEW-design of WP-1.5b design revision 7.
- Original path:     /tmp/.../scratchpad/wp15b_design_rev7_REVIEW.md
                     (session-scoped tmpfs; does not survive the session — this
                     landing is the only retrievable copy)
- SHA the report examined: 6feb40af1f1c12c1977d7a2030509dd98cbdc8ac
- Landed at:         2026-08-22, tree at cf74594.
- Cited by:          D-309 (closes revision 7 on this report), and the
                     eleven-finding summary in
                     docs/experiments/restructure_selection_15b.md.
```

This file — not D-309 — is the review's landed, verbatim home; its own header states D-309 *cites* it, not the reverse. MEASURED, D-309's own text (`docs/decisions.md:663`) says of the two findings it reproduces: *"both are reproduced here **because the report file is session-scoped**"* — D-309 explicitly frames itself as a partial, redundant reproduction taken *from* a separate report file, not as that report's home.

MEASURED, `D-346` (`docs/decisions.md:737`, the policy this very u-rev-8 repair is executed under) states the homes taxonomy directly: *"The homes are the ones D-331 already names: `docs/decisions.md` for rulings, `docs/experiments/wp15b_*_REVIEW*.md` for findings and verdicts, git for what a revision contained."* The revision-7 review's verdict and findings live at `wp15b_design_rev7_REVIEW.md`, which matches the `wp15b_*_REVIEW*.md` glob D-346 names for exactly that content; D-309 is `docs/decisions.md`, the ruling home, not the findings/verdict home.

**Why it breaks.** This is a same-commit recurrence of the exact class the round exists to remove: a false claim about this document's own state (here, about another document's provenance), asserted in the very sentence that struck the previous false claim of that class. It also breaches the strike policy procedurally, not just factually: `D-346`'s pointer clause is conditional ("if and only if… strands navigation"), and no stranding existed here to license adding anything at all.

**Fix scope.** U3-local, not a design act: strike "and is that review's home" and replace with nothing (the sentence is complete without it), or, if a pointer to the review's home is wanted, name `docs/experiments/wp15b_design_rev7_REVIEW.md` directly rather than misattributing that role to D-309.

---

### G. The u-rev 8 preamble claims three self-state claims were struck and "nothing replaces them," but a fourth, materially identical instance of the same false universal survives unstruck in U3-Z

**Claim reviewed** (head, lines 98–109, new at u-rev 8):

> **u-rev 8 ANSWERS NO REVIEW REPORT.** … u-rev 8 is a repair under `D-346` of
> three claims this unit made about its own state and that were false at u-rev
> 7: the head's *"the counts are D-309's and are not restated here"*… §12's
> *"and are not restated here"*… and the MAJOR D row's *"this head asserts no
> universal about its own citations at all"*, which is itself a universal
> about this document's own state and is false of a head that states which two
> citations are deliberately not in the form. **All three are STRUCK and
> nothing replaces them.**

**Contradicting text — a fourth instance, unstruck, in U3-Z's OPEN list** (line 853–854, unchanged by the u-rev 7→8 diff — MEASURED, `git diff a2b50bf 173397d -- docs/experiments/U3_tier_t.md` does not touch this hunk):

> **What this unit does about it is a remedy and not a rule:** **it asserts no
> universal about its own citations**, the B7 list is explicitly not a
> boundary, and the instrument it offers is labelled a finding aid rather than
> a proof — because u-rev 6 tried the derivation and the derivation was blind.

MEASURED, `grep -n "asserts no universal" docs/experiments/U3_tier_t.md`:

```
103:earlier; and the MAJOR D row's *"this head asserts no universal about its own
107:LEFT** — *"this head asserts no universal about HOW MANY SITES OBEY THE RULE"* — because
139:...*~~This head asserts no universal about its own citations at all~~* — **WITHDRAWN AT u-rev 8**...
854:  and not a rule:** it asserts no universal about its own citations, the B7 list is
```

Line 107 is the deliberately-kept, narrower, true sentence (scoped to "HOW MANY SITES OBEY THE RULE" — not in scope here). Line 139 is the struck instance. **Line 854 is a fourth, live, unstruck instance of the unscoped universal** ("it asserts no universal about its own citations" — no "HOW MANY SITES" qualifier), and it is false for exactly the reason u-rev 8 itself gives for line 139: MEASURED, the disposition text at line 139 states *"a claim that this document asserts no universal is itself a universal about this document's own state, which is the class this row is about"* — that reasoning applies to line 854's sentence word-for-word, since it makes the identical unscoped claim in a different location (the OPEN list's own remedy-not-a-rule bullet, answering `wp15b_U3_REVIEW_urev6.md` MAJOR E).

**Why it breaks.** This is the same propagation failure D-305 and D-331 are built on — a repair corrects a claim in one section (the MAJOR-D disposition row) and leaves an identical copy standing in another (U3-Z's OPEN list) — occurring inside the very commit whose own preamble claims completeness ("three claims… nothing replaces them"). It is the fifth instance of this document's now-five-round pattern where a repair's own self-description of what it fixed is itself incomplete (previously: "four sites" undercounting eight in MINOR 8/MINOR 11; "5" vs the actual six counter-examples in B7; the MAJOR-D blind grep).

**Fix scope.** U3-local, not a design act: strike "it asserts no universal about its own citations" at line 854 (or narrow it the same way line 107 was narrowed — e.g. "about how many sites obey the rule" — if the narrower, true claim is what was meant), and correct the preamble's count from "three claims" to whatever the true count is once this fourth instance is accounted for.

---

## MINOR

None.

---

# VERIFIED WITH NO FINDING

- **The three explicit strikes are individually correct.** (1) Head's struck claim: MEASURED, U3-A line 192 prints `7 BLOCKING, 7 MAJOR, 9 MINOR`, falsifying "not restated here." (2) §6.5's struck claim: MEASURED, the same sentence (lines 402–408) prints `3.1× to 2.4×` two clauses before "are not restated here." (3) MAJOR-D row's struck claim: reasoning verified sound — the head does state a positive, enumerated claim about exactly which two citations are exceptions (lines 93–96, verified below), which is itself "a universal about its own citations," contradicting "asserts no universal… at all."
- **No normative claim was stranded by any of the three strikes.** Struck claim (1) was purely self-referential (which document restates counts), not a rule. Struck claim (2)'s replacement correctly preserves the normative pointer — MEASURED, the replacement text still names "§12 item 5" and `wp15b_U3_REVIEW_urev3.md` MAJOR 2 as where the rule and the finding live, so the reader is not left without a route to the rule. Struck claim (3) was self-referential, not a rule.
- **The "two citations deliberately NOT in the form" claim (lines 93–96) still holds.** MEASURED, `grep -n '\*\*U2\*\* (u-rev 2)' docs/experiments/U3_tier_t.md` → exactly 2 hits, lines 59 and 61, matching "the two in the paragraph above."
- **The finding-aid `grep` command (lines 86–88) still behaves as documented.** MEASURED, `grep -nE '\*\*U[1234]\*\*|(^|[^A-Za-z])U[1234] §' docs/experiments/U3_tier_t.md | grep -v 'landed'` returns 7 lines (8, 59, 61, 96, 354, 361, 461); all seven fall into the document's own documented noise categories — quotations of the old bare form (59, 61), the explicitly-excepted superseded-document reference (96), or "prose that names a unit without citing it" (8, 354, 361, 461, none of which carry a `§`). No new, uncategorized hit.
- **"NEITHER u-rev 7 NOR u-rev 8 HAS BEEN REVIEWED" (REVIEW STATUS block, replacing u-rev 7's single-u-rev version of the same sentence) is accurate.** MEASURED, no `wp15b_U3_REVIEW_urev7.md` or `..._urev8.md` file exists in `docs/experiments/`, and U3-A's table has no row past u-rev 6.
- **Cross-unit citation SHAs, unchanged by this round's diff, still check out.** MEASURED, `git show 7dfd047:docs/experiments/U2_node_protocol.md | grep -n "^\*\*u-rev"` → `**u-rev 4.**`; `git show 0f49c90:docs/experiments/U4_soundness_instrument.md | grep -n "^\*\*u-rev"` → `**u-rev 7.**`. Both SHAs are ancestors of HEAD (`git merge-base --is-ancestor`, exit 0 both). These are historical-fact citations (`u-rev N, landed <sha>`) per this document's own u-rev-5 rule, correctly not required to track U2's/U4's current u-rev (U2 now at u-rev 7, U4 now at u-rev 9) — not a finding.
- **The footer's u-rev bump is self-consistent.** MEASURED, final line reads "*U3, u-rev 8. …*" matching the head's "**u-rev 8.**" — no head/foot mismatch (Gate 15's territory, not re-derived, but the surface text agrees on its face).
- **The census block and gate-relevant test names are untouched by this diff.** MEASURED, `git diff a2b50bf 173397d -- docs/experiments/U3_tier_t.md` touches only lines in the head paragraph, the REVIEW STATUS block, the MAJOR-D/E disposition table, and the §6.5 strongest-surviving-attack paragraph, plus the footer; the `BEGIN CENSUS TABLE`/`END CENSUS TABLE` block (lines 253–280) and `U3-M`/`U3-T` sections are byte-unchanged.

# REJECTED, WITH THE ATTEMPTED REPRODUCER

- **"U3-A's revision-7 row itself violates D-331 by restating D-309's counts a second time."** Attempted: checked whether printing `7 BLOCKING, 7 MAJOR, 9 MINOR` in U3-A is a forbidden restatement under the claim-home law. REJECTED: D-331 explicitly carves out lineage/status tables — *"STATUS AND SUMMARY MATTER CARRIES POINTERS ONLY — REVIEW STATUS tables, change logs, **lineage tables** and OPEN lists may say that a finding exists, that it is repaired and where the repair lives"* — and U3-A is headed "Lineage." Every other row in the same table carries the same shape (e.g. u-rev 2's "FAIL — 2 BLOCKING, 5 MAJOR, 4 MINOR"), established practice unchanged by this round and not flagged by any of the four prior reviews. Not a defect, and not introduced by u-rev 8.
- **"The MAJOR-D and §6.5 strike-and-dispose edits violate D-346 because they replace struck text with an explanation rather than literally nothing."** Attempted: compared against D-346's "STRUCK: deleted, and replaced with nothing" language read maximally literally. REJECTED: D-346's own "INSTANCE BASE" cites `U4_soundness_instrument.md`'s three prior executions as "struck through in the REVIEW STATUS tables, **each disposed** and none reworded" — disposal notes accompanying a strikethrough are the established, already-executed convention D-346 is itself built on (also matching D-338's original C-prime language: "striking the sentence through and appending a fixed... token"). Only finding F's addition ("and is that review's home") is a *new claim* rather than a disposal note explaining what happened to the old one, which is why F is charged and the disposal-note pattern generally is not.
- **"Line 8's `**U1**–**U4**` and lines 354/361/461's bare `**U4**` mentions (returned by the finding-aid grep) are additional undetected bare citations beyond finding G."** Attempted: checked each against the `§`-bearing citation shape the head's own rule and command target. REJECTED: none carries a `§` reference; each is either the generic unit-name list in the "HOW TO RESOLVE" preamble (line 8) or an anaphoric back-reference to a citation already fully given earlier in the same paragraph (354, 361, 461) — the same shape the u-rev-6 review already rejected on identical grounds ("Rejected, with the attempted reproducer," first bullet), unchanged by this diff.
- **"D-309's own text is itself defective for calling the report file 'session-scoped' when it has since been landed at `docs/experiments/wp15b_design_rev7_REVIEW.md`."** Attempted: checked whether this is a live defect chargeable here. REJECTED as out of this review's scope — that is a claim inside `docs/decisions.md`, not inside `docs/experiments/U3_tier_t.md`, and D-309 is an append-only ADR line this review is not dispatched against; it is evidence used against finding F, not a finding against U3 in its own right.
