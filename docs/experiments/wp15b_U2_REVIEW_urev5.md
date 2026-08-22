# REVIEW-design (CONFIRMATION RE-REVIEW) — `docs/experiments/U2_node_protocol.md` u-rev 5

**Pinned revision: `f0ae14c`.** Full SHA `f0ae14c7c7285280677816fd85d6b6905b70d82a`.

**At review finish:** `git rev-parse HEAD` → `82733c8d2daefb396a22e25eac04ff023547d723`.
HEAD advanced several commits past the pin during this review (concurrent work on
U4: `d328d1d`, `75ae04e`, and at least one further commit reaching `82733c8`,
none of it touching U2). `git diff f0ae14c HEAD -- docs/experiments/U2_node_protocol.md`
is **empty** at every check I ran (twice, including immediately before writing
this report) — U2's own text is byte-identical at the pin and at current HEAD.
`git status --porcelain` is clean at review finish. Mid-review it showed a
modified `docs/experiments/U4_soundness_instrument.md` and an untracked
`docs/experiments/matrix_U4R_REDTEAM_round2.md`; both belonged to the concurrent
U4 work and were not touched by me, and both are gone from `git status` by the
time this report was written (the concurrent session committed them).

**Fresh context:** no prior turns this conversation. Read CLAUDE.md whole;
`docs/decisions.md` D-331, D-332, D-305, D-311; the whole of
`docs/experiments/U2_node_protocol.md`; then `wp15b_U2_REVIEW_urev4.md` and
`wp15b_U2_REVIEW_urev3.md`. Also read, to verify specific claims: the full diff
of every intermediate U2 revision (`38f21b9`→`56b0bec`→`d85b049`→`7dfd047`→
`7473a6f`→`f0ae14c`), `docs/experiments/wp15b_design_rev7_REVIEW.md`,
`docs/experiments/U3_tier_t.md` and `docs/experiments/U4_soundness_instrument.md`
(for cross-unit citation checks), `docs/experiments/section_owner_table.md`, and
`docs/experiments/U1_gate_supersession.md`.

**Scope:** judge u-rev 5's two choices — deleting both copies of the "four
already existed, one is new" count (G1) and registering, rather than repairing,
the `168 030` / `343 344` disagreement (G2) — on the merits, plus the seven
named checks in the dispatch brief, plus a full sweep for false
self-completeness claims. Per the REVIEWER SCOPE GUARD, a pointer is not
charged as a finding; only text carrying an independent, falsifiable claim is.

---

## VERDICT: FAIL — 1 MAJOR, 1 MINOR

---

## Finding H1 (MAJOR) — the carve-provenance sentence claims `6feb40a` was "never reviewed," which is false: it was reviewed, by name, and FAILED, and that failure is what D-309 cites as its own closing ground

**Claim, U2 line 15–16 (head, present unchanged since u-rev 1):** "**u-rev 5.**
Carved from `docs/experiments/wp15b_design.md` §2, §3, §5 and §14 at `6feb40a`
**(revision 7, never reviewed, CLOSED by D-309)** under the restructure selected
as option D by D-310."

**Contradicting evidence, reproducer:**
```
$ sed -n '655,663p' docs/decisions.md | head -1
D-309: WP-1.5b's DESIGN REACHED REVISION 7 AND FAILED ITS FRESH-CONTEXT
REVIEW-design AT `6feb40a` — 7 BLOCKING, 7 MAJOR, 9 MINOR — SO THE STANDING
NO-REVISION-8 RULE FIRES AND THE DOCUMENT IS RESTRUCTURED RATHER THAN REVISED
AGAIN IN PLACE. Revision 7 is CLOSED. ...

$ head -30 docs/experiments/wp15b_design_rev7_REVIEW.md
- Report:            REVIEW-design of WP-1.5b design revision 7.
- SHA the report examined: 6feb40af1f1c12c1977d7a2030509dd98cbdc8ac
                     (`docs/experiments/wp15b_design.md`, revision 7, 1975 lines;
                     ...)
- Landed at:         2026-08-22, tree at cf74594.
- Cited by:          D-309 (closes revision 7 on this report), ...
## Header
- **Revision reviewed:** `6feb40af1f1c12c1977d7a2030509dd98cbdc8ac`
- **Matches HEAD:** YES. Working tree clean at review time.

$ sed -n '157,166p' docs/experiments/U2_node_protocol.md
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR,
9 MINOR. **None of the seven BLOCKING is §2's, §3's, §5's or §14's**, except
B5's §2.2 site, repaired here |
```
Three independent sources agree with each other and disagree with U2's own
head: `6feb40a` was reviewed, by a fresh-context REVIEW-design, and it FAILED
with 7 BLOCKING / 7 MAJOR / 9 MINOR — the exact numbers D-309 cites and the
exact numbers U2's own U2-A lineage table (14 lines below the "never reviewed"
sentence, in the same document) reports for "revision 7, `6feb40a`". D-309's
own ground for CLOSING revision 7 and forcing the restructure **is** that
failed review — "SO THE STANDING NO-REVISION-8 RULE FIRES" follows directly
from "FAILED ITS FRESH-CONTEXT REVIEW-design AT `6feb40a`" in the same
sentence. "Never reviewed" and "CLOSED by D-309" cannot both be true of the
same SHA when D-309's own closing clause is the review's failure.

I checked whether "reviewed" might be a term of art in this document family
meaning "reviewed and passed" (which would rescue the sentence by reading
"never reviewed" as "never *passed* review"). It is not: U2's own REVIEW STATUS
apparatus uses "reviewed" to mean "a review ran," independent of verdict —
"u-rev 1 (pinned `38f21b9`) **was reviewed** by `docs/experiments/wp15b_U2_REVIEW.md`
... **VERDICT FAIL**" (u-rev 2's history paragraph, still legible via
`git show 56b0bec:docs/experiments/U2_node_protocol.md`) and the current head's
own "**u-rev 4 WAS REVIEWED AND FAILED** ON TWO MINORS" (line 125) use exactly
this convention. By the document's own usage, "reviewed" does not mean
"passed," so there is no reading under which "never reviewed" survives contact
with a review that ran and failed.

**This is not local to U2.** The identical templated phrase — "(revision 7,
never reviewed, CLOSED by D-309)" — appears verbatim in `U1_gate_supersession.md:16`,
`U3_tier_t.md:16`, `U4_soundness_instrument.md:16` and
`section_owner_table.md:6`. It has existed since the original carve commit
(`cf74594`) predating D-331 entirely, and no fresh-context REVIEW-design of any
of the five carved documents, across every u-rev of all four units, has ever
raised it (`grep -rn "never reviewed" docs/experiments/wp15b_*REVIEW*.md`
returns nothing). I flag this here because it is squarely inside U2's own text
and inside this round's named check 1 ("sweep for exactly this shape"); the
same defect in the sibling documents is outside my scope to charge but is
recorded here so it is not independently re-discovered five times.

**Why it breaks:** this is precisely the class named check 1 asked me to hunt
— a claim about the document family's own state, asserted by its authors, that
a landed ADR line (D-309) and the document's own internal table falsify. It
predates D-331 and none of the four post-carve repair rounds (u-rev 2, 3, 4, 5)
touched this sentence (each repair only ever changed the u-rev number in it),
so it is not a *restatement* D-331 generates — it is an original error, never
caught, silently reproduced into every sibling document at the same carve
commit. A reader who takes the carve-provenance sentence at face value would
believe `6feb40a` reached CLOSED status by some route other than failing
review, which is false and undersells the very finding (D-309's own 7
BLOCKING) that justified restructuring the document at all.

**Severity:** MAJOR. It does not touch the soundness of §5's design content,
the F5/F6/G1/G2 repair chain, or the CARVE-EXCEPTION apparatus this round
verified clean — but it is a checkable, self-contradicted factual claim
sitting in the document's own provenance sentence, propagated into five
documents, and it has evaded review since the carve's first commit.

**Fix scope:** one clause, in five documents. Replace "never reviewed" with an
accurate characterization, e.g. "reviewed and FAILED (7 BLOCKING, 7 MAJOR,
9 MINOR)" or simply drop the clause and let the U2-A table (which already
states the true verdict) carry it, per D-331's own STATUS-CARRIES-POINTERS-ONLY
rule. Not confined to U2 — U1, U3, U4 and `section_owner_table.md` carry the
same sentence and need the same fix, though only U2's copy is chargeable to
this review.

---

## Finding H2 (MINOR) — G2's stated reason for not repairing invokes "carved content" and "the carve['s] standing discipline," but the disputed figure does not live in carved content by the document's own definition, and the unit's own repair history contradicts "transcription only"

**Claim, U2 line 135 (REVIEW STATUS table, G2 row) and lines 877–881 (OPEN
bullet, near-identical wording):** "**Reconciling two population figures for
the M5-E equivalence is a design act on carved content, not a carve act**, and
this unit's standing discipline is that the carve repairs transcription only."

**Contradicting evidence, reproducer:**
```
$ sed -n '54,58p' docs/experiments/U2_node_protocol.md
**THE EXCEPTIONS ARE STATED AT THEIR OWN SITES AND NOWHERE ELSE...** An
exception is a change to this unit's CARVED CONTENT — prose carried over from
the superseded document's §2, §3, §5 or §14 — that is not a `§n`-retarget.
```
The document's *only* formal definition of "CARVED CONTENT" scopes it to §2,
§3, §5 and §14 — the four sections named in the carve-provenance sentence
itself (line 15). The `168 030` figure the OPEN bullet is about lives under
`## U2-Z`, in "### The handoff this unit carries to WP-1.6" — a section the
document's own ADR-lines preamble (line 752) says is "**Carried from the
superseded §15**," not §2/§3/§5/§14. Tracing further, the exact sentence
predates even that: `git show 6feb40a:docs/experiments/wp15b_design.md | sed -n '1900,1965p'`
shows the "168 030 comparisons" line sits in the superseded document's
**§18.4, "Handoff to WP-1.6"** (`## 18. OPERATOR-QUEUE`), a section outside
both the declared carve scope (§2/3/5/14) and the "§15" provenance the U2-Z
preamble states for the ADR-lines list. By the document's own precise,
capitalized definition, this text was never "CARVED CONTENT" in the first
place — so "reconciling ... is a design act on carved content, not a carve
act" invokes a rule that does not, by the document's own terms, govern the
site in question.

Separately, "this unit's standing discipline is that the carve repairs
transcription only" is asserted here for the first time — `grep -n
"transcription\|standing discipline" docs/experiments/U2_node_protocol.md`
returns only this OPEN bullet and its REVIEW STATUS twin; no prior u-rev states
any such discipline. And the unit's own CARVE-EXCEPTION history contradicts it
on the merits: F2's repair (u-rev 1→2, marked `CARVE-EXCEPTION, u-rev 2 / F2`
at line 469) withdrew a FALSE verification claim ("VERIFIED on the shipped
solver") and substituted a different load-bearing justification (D-257's
abstract, position-free example) as the ground for the same conclusion — that
is a design-level correction of a wrong claim, not transcription, and it was
done as a disclosed carve act. If F2 could reconcile a false claim inside
carved content via a marker, the precedent set by the unit's own history does
not support treating "reconciling two conflicting numbers" as categorically
outside what a marked carve act may do.

**Why it breaks:** the OPEN bullet borrows the CARVE-EXCEPTION apparatus's
vocabulary and authority ("design act," "carve act," "carved content") to
justify inaction on text the apparatus's own scope excludes, and cites a
"standing discipline" that is not documented anywhere prior to this bullet and
is arguably falsified by the unit's own F2 precedent. The practical decision —
register OPEN rather than reconcile, because IMPL has not started and no code
exists yet to re-run either count — is separately defensible on its own
grounds (stated at line 732: "This unit has no governed run... IMPL has not
started"). But the STATED reason is a category error, not that grounding.

**Severity:** MINOR. It does not change the practical disposition (G2 is, and
should remain, OPEN rather than silently reconciled without a re-run) and it
does not affect landability of the F5/F6/G1 repair this round's substitution
rests on. It is a soundness-of-reasoning defect in the OPEN item's own
justification, which the next repair should not inherit uncorrected.

**Fix scope:** one sentence. Either drop the "carved content / carve act"
framing and state the real ground directly (no instrument exists yet to
re-derive either MEASURED figure, so reconciling now would itself be an
unmeasured claim), or, if the carved-content framing is intentional, extend the
document's own CARVED CONTENT definition (line 55–56) to explicitly cover U2-Z
content and reconcile the "transcription only" claim against F2's own history.

---

## Verified with no finding

- **G1's deletion (named check 2).** `grep -n "four\|already existed\|one is
  new" docs/experiments/U2_node_protocol.md` finds no surviving copy of the
  "four markers already existed, one is new" count anywhere in the document;
  the two former sites (head paragraph, F5 status row) now read "Which markers
  pre-dated u-rev 4 and which was new is not stated here or anywhere else in
  this unit" and "Every disclosure in the unit carries the same token."
  Nothing else in the document depends on the deleted count — the F5 status
  row's remaining claim ("one command enumerates them") does not require
  knowing which markers are old vs. new. Both copies are gone; nothing was
  load-bearing on the specific 4-vs-1 breakdown.
- **The CARVE-EXCEPTION rule (named check 3).** I ran the head's registered
  command myself: `grep -n "(CARVE-EXCEPTION" docs/experiments/U2_node_protocol.md`
  returns exactly 6 lines (207, 446, 469, 559, 726, plus the command's own
  quoted self-match at line 66) — 5 real markers, 1 self-match, matching the
  head's own characterization exactly. I then independently diffed **every**
  intermediate revision named in the brief: `git diff 38f21b9 56b0bec`,
  `56b0bec d85b049`, `d85b049 7dfd047`, `7dfd047 7473a6f` (empty), and
  `7473a6f f0ae14c` (this round's own diff) against
  `docs/experiments/U2_node_protocol.md`. Every hunk in every diff either (a)
  touches only head apparatus (u-rev label, u-rev table, history paragraphs,
  REVIEW STATUS blocks, OPEN list — none of it §2/§3/§5/§14 body prose) or (b)
  is one of the five marked CARVE-EXCEPTION sites (B5/§2.2, F1/§5.3, F2/§5.3,
  R5/§5.4, D-310-seam/§12), each carrying its marker at the point of change.
  u-rev 5's own diff (`7473a6f`→`f0ae14c`) touches **zero** carved-content
  lines — every hunk is inside the head, the two REVIEW STATUS tables, or the
  U2-Z OPEN list. No untagged change to §2/§3/§5/§14 exists at any revision.
  This is the strongest part of the repair chain and I could not falsify it.
- **G2's numbers and marks (named check 4).** `grep -n "145 158\|343 344\|168
  030"` confirms §5.2 (line 301), the M5-E matrix row (line 622) and the
  quoted red-team sentence (lines 634–635) all state **145 158** positions /
  **343 344** comparisons, all tagged **MEASURED**; the U2-Z handoff bullet
  (line 848) states **168 030** comparisons with no MEASURED/ESTIMATED tag —
  exactly as the OPEN bullet describes. The OPEN bullet's description of the
  two figures is accurate (setting aside H2's separate finding about its
  stated *reason* for not reconciling them). What is owed is stated clearly
  enough for a later session: "either an instrument re-run that reconciles
  them and marks both, or an ADR line recording which population the M5-E
  claim rests on."
- **The u-rev table and both REVIEW STATUS blocks (named check 5).** All four
  historical SHAs (`38f21b9`, `56b0bec`, `d85b049`, `7473a6f`) resolve as
  commits; all four report paths exist on disk; each report's own `Pinned
  revision:` line and `## VERDICT` line match the table's row exactly (FAIL,
  FAIL, FAIL, FAIL — 0 BLOCKING/0 MAJOR/2 MINOR for u-rev 4's row, matching
  `wp15b_U2_REVIEW_urev4.md`'s own verdict line). Row 5 ("this text | NOT YET
  REVIEWED") is accurate — this review is that row's first entry. The two
  REVIEW STATUS blocks (u-rev 3→4 transition, u-rev 4→5 transition) describe
  disjoint transitions and do not restate each other's content.
- **Cross-unit citations (named check 6).** `grep -n "^\*\*u-rev [0-9]*\.\*\*"`
  on each unit file confirms, live: U1 at u-rev 2, U2 at u-rev 5, U3 at u-rev
  6, U4 at u-rev 7 — matching the dispatch brief exactly. U2 cites other units
  by section only (`**U1** §4`, `**U3** §6/§7/§10/§6.2/§12 item 4`, `**U4**
  §8.2/§8.6/§9`), never by another unit's u-rev number, so D-311/D-332's
  stale-citation defect class cannot occur inside U2 — there is nothing to go
  stale. On the D-332 (R17) ordering disclosure: `git log --oneline` confirms
  this round's actual commit order was `7dfd047` (U2→4), `7473a6f` (U3→5),
  `13621d3` (U3→6), `f0ae14c` (U2→5) — U2's final state genuinely was written
  after U3's, as the brief states. I checked the reverse direction (U3/U4
  citing U2's u-rev) since the brief explicitly invited it: `U3_tier_t.md` and
  `U4_soundness_instrument.md` cite U2 exclusively in the D-332-fixed
  `**U2** (u-rev N, landed <sha>)` historical form (e.g. `**U2** (u-rev 4,
  landed `7dfd047`)`, and `U4_soundness_instrument.md:291` already carries
  `**U2** (u-rev 5, landed `f0ae14c`)`, written in a commit that landed after
  `f0ae14c`). This form states a fact about what landed at a SHA, which cannot
  go stale by construction, so U2's own advancement to u-rev 5 after U3/U4
  wrote their citations does not make any of them wrong.
- **The "Five fresh-context REVIEW-designs and two DECISION-RED-TEAMs" claim
  (named check 1, investigated, not confirmed false).** This sentence (line
  93–94) is unchanged since u-rev 1 (verified via `git log -p` and the
  `56b0bec`/`182f389` diffs) and is head apparatus, not carved content, but I
  swept it anyway per check 1. Counting DISPATCH ROUNDS (not individual
  matrix/revision targets) from the pre-carve commit history —
  `ec8f7fb`→`182f389` ("after five decision red-teams — three matrices fell"),
  `182f389`→`7ad466b` ("after REVIEW-design failed revision 2 on two STOPs"),
  then the four further REVIEW-design rounds implied by `7ad466b`→`f762c9a`→
  `64af80c`→`2d07ff6`→`d94dc0a` (matching D-305's independently-stated "four
  fresh-context REVIEW-designs" across "six revisions"), plus the M5-specific
  DECISION-RED-TEAM at `7ad466b` — the totals (5 REVIEW-design rounds, 2
  DECISION-RED-TEAM rounds) are consistent with this claim under the "count
  dispatch rounds, not per-matrix or per-revision instances" reading. I could
  not fully pin an exact 1:1 count for the "revisions 3–6" table row (it may
  represent 3 or 4 discrete rounds), so I am not certifying this claim true,
  only that I could not falsify it and the available evidence is consistent
  with it. Not charged as a finding.
- **The "eight prior review rounds" figure in the OPEN bullet (line 876,
  investigated, not confirmed false).** Two plausible countings both land on
  eight (5 pre-carve REVIEW-design + 3 post-carve confirmation passes,
  excluding the 2 DECISION-RED-TEAMs as a different review kind; or the
  breakdown `wp15b_U2_REVIEW_urev4.md`'s own summary gives, "5 REVIEW-design +
  2 DECISION-RED-TEAM + this document's own history" read as 5+2+1). Given the
  ambiguity and that both readings reach the same number, not charged.

## Rejected, with the attempted reproducer

- **The gate itself, as a finding against U2.** `cargo test -p pistol-solver
  --test wp15b_census` (see GATE below) passes clean at current HEAD, 5/5 not
  ignored. No finding to reject here, but recording for completeness since
  `wp15b_U2_REVIEW_urev4.md` had to isolate a false-positive failure at its
  pin — that failure mode no longer reproduces: a new test,
  `a_document_quoting_the_carve_marker_is_not_a_carve_member`, now exists in
  the suite and the census pin is robust to a document merely quoting the
  marker string. Not U2's own change (U2's diff at `f0ae14c` never touches
  `wp15b_census.rs`) so not credited to this repair, just noted since it
  removes a source of gate flakiness the last review had to work around.

---

## GATE

```
$ cargo test -p pistol-solver --test wp15b_census
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running tests/wp15b_census.rs (target/debug/deps/wp15b_census-395e5ba3ce82a23a)

running 6 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test a_document_quoting_the_carve_marker_is_not_a_carve_member ... ok
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.62s
```
Clean pass, no isolation needed this round.

---

## Summary

u-rev 5's two named choices are both defensible on their own terms: G1's
deletion removes a genuine duplication (D-331-class) cleanly and leaves
nothing dangling, and the CARVE-EXCEPTION apparatus continues to hold exactly
as u-rev 4's review found it — I re-verified every intermediate diff myself
rather than trusting either the head's or the prior review's word, per the
brief's explicit instruction, and found no untagged change to carved content
at any revision, including this one's own (which touches zero carved-content
lines).

Two findings survive. **H1 (MAJOR)** is new to this round's sweep, not a
regression in u-rev 5's own diff: the carve-provenance sentence's "(revision
7, never reviewed, CLOSED by D-309)" is false — `6feb40a` was reviewed, by a
fresh-context REVIEW-design, and FAILED with the exact numbers D-309 and U2's
own U2-A table both report, and that failure is D-309's own stated ground for
closing revision 7. This sentence has existed unchanged since the original
carve, is repeated verbatim in three sibling unit documents and the
section-owner table, and has evaded every fresh-context review across all
four units to date. **H2 (MINOR)** finds that G2's stated reason for
registering rather than repairing — "a design act on carved content, not a
carve act," under "this unit's standing discipline" — invokes a rule (the
CARVE-EXCEPTION apparatus's CARVED CONTENT definition) that, by the document's
own precise scoping to §2/§3/§5/§14, does not cover the U2-Z site where the
disputed figure actually lives, and asserts a "standing discipline" that is
undocumented before this bullet and arguably contradicted by the unit's own
F2 precedent. The practical disposition (register OPEN, do not silently
reconcile without a re-run) is sound; the stated justification for it is not.

Neither finding touches the soundness of §5's node-protocol design content,
which five REVIEW-designs and two DECISION-RED-TEAMs have now examined without
a merits break, and neither reopens F5/F6/G1's repairs. Both are new findings
this round produced by independently re-deriving every check rather than
accepting the head's or the prior review's characterization, per this
document's own repeatedly-stated discipline that a claim is verified, not
trusted.
