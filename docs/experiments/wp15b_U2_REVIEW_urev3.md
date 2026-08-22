# REVIEW-design (CONFIRMATION RE-REVIEW) — `docs/experiments/U2_node_protocol.md` u-rev 3

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session repaired U2; it did not review it. Per the
session's standing rule a re-review FAIL is COLLECTED AND REPORTED, not looped
on in-session: F5 and F6 below are the architect's.
-->

**Pinned revision: `d85b049`.** HEAD advanced during this review to `3dd8ea0`.
`git diff d85b049 HEAD -- docs/experiments/U2_node_protocol.md` is **empty** —
the document under review is byte-identical at the pin and at that HEAD. The
only committed change between the two revisions is `docs/decisions.md` (+6
lines: D-326, D-327, D-328 — unrelated ADR lines, none touching U2 or D-257).
The findings below hold against both revisions. Working-tree modifications to
`crates/pistol-cli/tests/baseline_snapshot_tests.rs`, `docs/experiments/U3_tier_t.md`
and `tools/baseline_snapshot.sh` were present throughout (concurrent, unrelated
sessions, per dispatch note) — not touched, not reviewed.

**Fresh context:** no prior turns this conversation; read CLAUDE.md's Process
section, `docs/experiments/wp15b_U2_REVIEW_urev2.md` (the FAIL this repair
answers), then U2 u-rev 3 cold.

**Scope, per dispatch instructions:** confirmation-scope re-review of the F3/F4
repair and its blast radius only. Proportionality applied — no open-ended
re-review of settled design content conducted.

---

## VERDICT: FAIL — 1 MAJOR, 1 MINOR

---

## Finding F5 (MAJOR) — the "Four exceptions" list is still incomplete: it omits R5's own addition to §5.4, the same defect class F3 named and this repair did not close

**Claim, U2 lines 42–65 (the repaired "exceptions" paragraph):** "**Four
exceptions, all stated where they occur rather than only here.** Each is a
change to this unit's CARVED CONTENT — prose carried over from the superseded
document's §2, §3, §5 or §14. … " followed by exactly four items: §2.2's B5
citation (config count), §12 item 2's rate-list handoff, §5.3's F1 citation
repoint, and §5.3's F2 two-ply-illustration rewrite. The paragraph asserts this
enumeration is exhaustive of every non-retarget change to carved content.

**Contradicting evidence, reproducer:**
```
grep -n "Architect ruling R5" docs/experiments/U2_node_protocol.md docs/experiments/WPQ_seed.md
# → U2_node_protocol.md:504, WPQ_seed.md:24

grep -n "Architect ruling R5" /tmp/.../scratchpad/u2_urev1.md   # u-rev 1, pinned 38f21b9
# → no match (exit 1): the paragraph does not exist at u-rev 1

sed -n '504,507p' docs/experiments/U2_node_protocol.md
# → **Architect ruling R5 (settled).** Tier Q stays in this unit's node protocol,
#   SPECIFIED BUT UNARMED: the D-scope WP-1.5b ships is stages F and T only, this
#   unit's protocol scope for Tier Q is unchanged by that scope, and the
#   pre-registration registers F+T only.
```
This four-sentence paragraph sits inside `### 5.4 Step 4 — staged generation`
— squarely inside §5, one of the four sections the head declares CARVED
("§2, §3, §5 and §14"). It is absent from u-rev 1 (`38f21b9`) and first appears
at u-rev 2 (confirmed at `u2_urev2.md:479`), i.e. it landed in the exact same
u-rev-2 commit that produced F2's rewrite. It is not a `§n`-retarget — it is
wholly new prose making a new ruling, no different in kind from exception 4
(F2), which the paragraph's own text characterizes as "roughly sixteen new
lines of new prose, not a `§n`-retarget" — the same description applies to R5
verbatim. Nothing in the "Four exceptions" paragraph names it, and — unlike F1
and F2 — R5's own paragraph carries **no inline point-of-occurrence disclosure
marker** the way F1's fix does ("(Citation disclosure, u-rev 2 / F1: …)") and
F2's fix does ("(u-rev 2 / F2)"). Its sister document makes the gap visible by
contrast: `WPQ_seed.md`'s parallel R5 paragraph (line 24) is explicitly self-tagged
*"(u-rev 2, one sentence, per D-311.)"* — U2's copy of the identical ruling
carries no equivalent tag.

**Why it breaks:** this is the exact FOLD-IN LAW defect F3 already named once
in this document — "the paragraph was actively edited this round … yet still
failed to account for [a change to carved content] disclosed extensively
[elsewhere] in the same file" — recurring in the very repair meant to close it.
The repair widened "Three exceptions" to "Four" and added a scoping preamble
explaining why editing the list itself is not a fifth exception, but it did not
re-audit the u-rev-1→u-rev-2 diff for OTHER changes to §3/§5/§14 beyond F1/F2 —
which is precisely check 1 of this round's brief ("Is anything ELSE changed in
§3/§5/§14 that the list of four still does not name? … do the diff, do not
take the list's word for it"). R5 is disclosed exactly once, in the head's
"u-rev 2 was a REPAIR …, plus a one-sentence architect ruling (R5, Tier Q)"
sentence (line 24) — a historical summary, not a claim about carved-content
exhaustiveness, and not "where it occurs." The prior confirmation review
(`wp15b_U2_REVIEW_urev2.md`, check 3) examined R5 for cross-document
consistency and explicitly declined to raise its absence from the exceptions
list as a finding — that was a miss this round was dispatched to catch, and it
recurred rather than being caught.

**Note on check 2 (the self-reference/scoping test):** the paragraph's new
carved-content/apparatus boundary is not what lets R5 escape — R5 is squarely
"carved content" by the paragraph's own definition (new prose inside §5), not
reclassifiable as "head apparatus." It escapes by simple omission, not by
exploiting the new scoping language. The scoping paragraph's internal logic is
otherwise sound (see Verified section below); this finding is a coverage gap
in the enumeration, not a defect in the boundary rule itself.

**Fix scope:** one sentence — add a fifth exception naming R5's §5.4 addition
(or rephrase to "Five exceptions"), and/or an inline disclosure marker at
§5.4's R5 paragraph matching the F1/F2/WPQ_seed convention. Confined to U2's
head and one paragraph in §5.4; no design content changes.

---

## Finding F6 (MINOR) — "roughly sixteen new lines" is an unmarked numeric claim, inconsistent with this document's own convention for grep/diff-derived counts

**Claim, U2 line 59:** "…roughly sixteen new lines of new prose, not a
`§n`-retarget…" — describing F2's §5.3 rewrite, in the newly repaired fourth
exception item.

**Contradicting/supporting evidence, reproducer:**
```
sed -n '371,385p' u2_urev1.md | wc -l   # u-rev 1 span, "two-ply realisation" … up to "One licensed value change"
# → 15
sed -n '391,421p' <current U2>          # u-rev 3 (== u-rev 2) span, same boundaries
# → 31
# net = 31 - 15 = 16   (exact, not "roughly")
```
The claim is factually TRUE and precisely exact (16, not merely "roughly"
16). But it is a numeric claim about the diff, unmarked MEASURED or ESTIMATED,
in a document that marks exactly this class of claim elsewhere: §2.1's "MEASURED
by grep at this revision: **six** irrefutable `let CandidatePolicy::Radius …`
destructures become compile errors" is a structurally identical claim (a
grep/diff-derived count about the document/codebase itself, not a search-behavior
measurement) and carries an explicit MEASURED tag. "Roughly sixteen new lines"
is the same kind of claim and carries none.

**Severity note:** this is a marking-convention gap, not a content error — the
number is correct and does not affect any argument the paragraph makes. Rated
MINOR, on a par with F4's quotation-fidelity defect.

**Fix scope:** prefix with "MEASURED" (the count is exact and grep/diff-derivable,
so ESTIMATED would undersell it), e.g. "**MEASURED** sixteen new lines of new
prose."

---

## Verified with no finding

- **F4 fully discharged (check 3).** `grep -o "three hot windows with empties.*covers nothing in the middle" docs/decisions.md` returns: *"three hot windows with empties {a,b}, {b,d}, {d,e} have no one-cell cover **and three minimal two-covers, and** {a,e} **is drawn from the same union and** covers nothing in the middle"*. U2's repaired quote (lines 421–422) now reads "…have no one-cell cover**…**" / "{a,e} **…**covers nothing in the middle" — two ellipses, each landing exactly at the boundary of a real elision (the first spans "and three minimal two-covers, and", the second spans "is drawn from the same union and"), with nothing else altered. No third, unmarked omission exists in the quoted span. F4 is correctly and completely repaired.
- **F3's own named item is accurate (check 1).** The fourth exception's five-part enumeration of F2's §5.3 rewrite (heading clause, VERIFIED-sentence withdrawal, D-257 blockquote, rule-3 arithmetic, rewritten D-243(4)-discharge sentence) matches `diff -u u2_urev1.md docs/experiments/U2_node_protocol.md`'s actual F2 hunk; every named component is present in the diff and no named component is fabricated. (Its incompleteness relative to R5 is F5, above — a coverage gap, not an inaccuracy in what it does say.)
- **The self-reference scoping boundary is internally consistent (check 2).** The paragraph's carved-content/apparatus split ("This list, the u-rev label, and the rest of the head's own apparatus are NOT carved content") is consistent with how the rest of the head uses the terms: the "Carved from … §2, §3, §5 and §14" line (line 15) and every disclosed exception name a §-numbered body section, never head-region administrative prose; conversely every edit this repair makes to the head itself (the u-rev label, the "u-rev 3 is a REPAIR…" paragraph, "Four exceptions"'s own wording, "REVIEWED TWICE", the closing line) is administrative/review-status text with no §-number claim of its own. No instance was found where a real change to §2/§3/§5/§14 body prose is reclassified as "apparatus" to dodge the list — where a real change escapes (F5), it escapes by plain omission, not by exploiting this boundary.
- **U2-A lineage table, deliberately not appended (check 4).** Same call as the prior round, and it is still right: the table's rows are keyed to superseded-document SHAs and scopes itself to pre-carve attacks on design content (per its own header, "what has attacked this unit's content"); post-carve confirmation-pass lineage is tracked in the head's "REVIEWED TWICE" paragraph (lines 85–95) and the U2-Z handoff bullet (lines 793–799), both of which now correctly enumerate both post-carve FAILs (u-rev 1 and u-rev 2). No structural change to that split occurred this repair, so the prior round's rejection stands unchanged.
- **Lines ~397/417 "(u-rev 2 / F1)" / "(u-rev 2 / F2)" markers, deliberately left as history (check 4).** Correct: these mark WHEN and BY WHICH FINDING each fix landed, not the current u-rev state; changing them to "(u-rev 3 / …)" would misattribute the fix's provenance. Consistent with how the head itself treats history (e.g. "found by the u-rev 1 review … (`wp15b_U2_REVIEW.md`, F1)" is not rewritten to name u-rev 2 or u-rev 3 either).
- **Fold-in / stale-state sweep (check 4).** `grep -n "u-rev [0-9]"`, `grep -n "u-rev 2 / F"`, and `grep -n "VERDICT\|CONFIRMATION PASS\|HAS BEEN REVIEWED\|UNREVIEWED"` across the whole file: every live self-referential label reads "u-rev 3" or names u-rev 3 as current; every u-rev 1/u-rev 2 mention is correctly framed as past history (both in the head's "was a REPAIR"/"is a REPAIR" pair and in the "REVIEWED TWICE" paragraph); the U2-Z handoff bullet's "attacked twice post-carve" and its FAIL/repair chronology match the head; the closing line correctly cites `wp15b_U2_REVIEW_urev2.md` and "u-rev 2". No stale u-rev-2 self-label survives.
- **Fold-in scope discipline (check 4, negative check).** Isolated `diff -u u2_urev2.md docs/experiments/U2_node_protocol.md` (u-rev 2 → u-rev 3, this repair's actual diff) touches exactly: the u-rev label, the two head history paragraphs, the exceptions paragraph, the review-status paragraph, the D-257 quote's second ellipsis, the U2-Z handoff bullet, and the closing line. Nothing outside those seven spans changed — the repair did not silently touch unrelated design content while it was in the file.
- **Marks, elsewhere (check 5).** No other numeric claim in the repaired spans (u-rev numbers, finding IDs F1–F4, revision SHAs, "1 MAJOR"/"1 MINOR" finding-severity counts) is the kind of claim this document marks MEASURED/ESTIMATED — review-verdict and finding-count tallies are consistently left unmarked throughout the document (e.g. the pre-existing "7 BLOCKING, 7 MAJOR, 9 MINOR" in the U2-A table), so their being unmarked here is convention-consistent, not an omission. Only "roughly sixteen new lines" (F6) breaks that convention.

## Rejected, with the attempted reproducer

- **None additional this round.** The one item the prior round rejected (U2-A lineage table staleness) was re-examined under this round's check 4 and is listed under "Verified with no finding" above rather than re-litigated, since nothing about the repair changed the basis for that call.

---

## VERDICT: FAIL — 1 MAJOR (F5), 1 MINOR (F6)
