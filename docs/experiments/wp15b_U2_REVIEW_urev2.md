# REVIEW-design (CONFIRMATION RE-REVIEW) — `docs/experiments/U2_node_protocol.md` u-rev 2, `docs/experiments/WPQ_seed.md` u-rev 2

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session repaired U2; it did not review it. Per the
session's standing rule a re-review FAIL is COLLECTED AND REPORTED, not looped
on in-session: F3 and F4 below are UNREPAIRED and are the architect's.
-->

**Pinned revision: `56b0bec`.** Current HEAD at review time: `e3f0bc3` — HEAD had advanced past the pin, but `git diff 56b0bec HEAD --stat` showed only a new file, `docs/experiments/wp15b_U1_REVIEW_urev2.md` (+102 lines, unrelated document). `git diff 56b0bec HEAD -- docs/experiments/U2_node_protocol.md docs/experiments/WPQ_seed.md` is empty. **Both documents under review are byte-identical at the pin and at that HEAD.** The findings below hold against both.

**Fresh context:** no prior turns this conversation; read CLAUDE.md's Process section, the prior confirmation-pass review (`wp15b_U2_REVIEW.md`, FAIL on u-rev 1), then U2 u-rev 2 and WPQ_seed.md u-rev 2 cold.

**Scope, per dispatch instructions:** confirmation-scope re-review only, five checks named in the brief. Proportionality applied — no open-ended re-review conducted.

**Note (not a finding against these documents):** the working tree carried an unrelated, uncommitted modification to `docs/experiments/U3_tier_t.md` (u-rev 2→3, the same session's in-flight repair). Not touched, not reviewed — out of scope, flagged here only for the record.

---

## VERDICT: FAIL — 1 MAJOR, 1 MINOR

---

## Finding F3 (MAJOR) — the "verbatim carve" claim's own exception count is now wrong

**Claim, U2 lines 27–32:** "**THE TEXT IS A VERBATIM CARVE.** Every change made to it is a CROSS-REFERENCE RETARGET … plus one B5 repair in §2.2 (below). **No sentence of §3, §5 or §14 was rewritten, extended or re-derived**, and no number moved." Followed immediately (lines 34–44) by "**Three exceptions, all stated where they occur rather than only here:**" listing (1) B5/§2.2, (2) §12 item 2's rate list → `WPQ_seed.md`, (3) §5.3's `§12 item 6` → `U2-Z item 20` citation fix (F1).

**Contradicting evidence, reproducer:**
```
git show 38f21b9:docs/experiments/U2_node_protocol.md > u2_urev1.md
diff -u u2_urev1.md docs/experiments/U2_node_protocol.md
```
The diff shows F2's repair added ~16 new lines to §5.3 well beyond a citation retarget: a new heading clause ("ILLUSTRATION ONLY; … WITHDRAWN"), a withdrawal of the "VERIFIED on the shipped solver" sentence, a new D-257 blockquote, new rule-3 arithmetic (`1 + 2(k−1) = 2k−1`), and a rewritten D-243(4)-discharge sentence replacing the old one. This is unambiguously new prose in §5.3 — a rewrite/extension, not a `§n`-retarget — and it is **not counted among the "Three exceptions."**

**Why it breaks:** the head's own integrity claim — "no sentence of §3, §5 or §14 was rewritten, extended or re-derived" — is literally false for §5.3 as the document itself stands, and the "Three exceptions" carve-out that exists specifically to reconcile that claim with reality omits the fourth, larger exception (F2). This is the exact FOLD-IN LAW defect this round exists to catch: the paragraph was *actively edited this round* (bumped "Two exceptions" → "Three exceptions", added the F1 bullet) yet still failed to account for F2, the larger of the two repairs landing in the same u-rev bump, disclosed extensively 350 lines below in the same file. It is a live instance of the same "undisclosed exception to the verbatim-carve claim" defect class F1 itself was.

**Fix scope:** one sentence — add a fourth bullet to the "exceptions" paragraph naming F2's §5.3 rewrite (or rephrase the paragraph to say "four exceptions"). Confined to U2's head; no design content changes.

---

## Finding F4 (MINOR) — D-257 quotation carries an unmarked second elision

**Claim, U2 §5.3 (lines 396–397):**
```
> three hot windows with empties {a,b}, {b,d}, {d,e} have no one-cell cover…
> {a,e} covers nothing in the middle
```

**Contradicting evidence, reproducer:**
```
grep -o "three hot windows with empties.*covers nothing in the middle" docs/decisions.md
```
returns: *"three hot windows with empties {a,b}, {b,d}, {d,e} have no one-cell cover **and three minimal two-covers, and** {a,e} **is drawn from the same union and** covers nothing in the middle"*.

**Why it breaks:** the single `…` marks only the first omission ("and three minimal two-covers, and"). A second omission — "is drawn from the same union and", between "{a,e}" and "covers" — is silently dropped with no ellipsis marker, so the quote reads as continuous text that D-257 does not contain verbatim at that join. Severity note: the substantive point ({a,e}, built from cells in the union, still fails to cover the middle window) survives the elision intact — this is a quotation-fidelity defect, not a content error, and does not undermine the argument the quote supports.

**Fix scope:** add a second `…` between `{a,e}` and `covers`, or requote in full.

---

## Verified with no finding

- **Rule 3 constants (check 1a).** `grep -n "FIRST_TURN_STONES\|TURN_STONES" crates/pistol-core/src/rules.rs` → `FIRST_TURN_STONES: u32 = 1`, `TURN_STONES: u32 = 2`. U2's arithmetic (`1 + 2(k−1) = 2k−1`, always odd, P1=10 unreachable) is correct.
- **D-257 pairing-obligation discharge (check 1d).** D-257's quoted example ("the union is provably insufficient … {a,e} covers nothing in the middle") does support the same union-insufficiency thesis D-243(4) needs, even though its framing (a `Cover` computation example) differs in shape from D-243(4)'s framing (a generator-behavior example about cross-window pairs). Judged adequate; not a reproducible defect.
- **Retained phase0/phase1 output (check 1c).** No longer functions as evidence for a design claim — the text explicitly states "the claim it was offered to support does not need it" and relocates the load-bearing ground to D-257. Its description ("real solver arithmetic over that unreachable board") is a provenance claim, not a re-verification claim, and is honest given nobody re-ran it this u-rev.
- **F1 repair correctness (check 2).** `git show 6feb40a:docs/experiments/wp15b_design.md | sed -n '1498,1659p' | grep -nE "^[0-9]+\. \*\*"` → exactly 5 items in superseded §12, confirming "§12 item 6" never existed. `sed -n '1685,1812p' … | grep -n "^[0-9]\+\. \*\*"` confirms item 20 = "`Run::salvage`'s documented ground does not hold under `Staged`" — the correct referent, and U2-Z item 20 (lines 723–738) reproduces it. Disclosed both at point of occurrence (§5.3, lines 371–375) and as the head's third stated exception (lines 34, 40–44) — factually correct as far as it goes (see F3 for the incompleteness of the surrounding paragraph).
- **Architect ruling R5 (check 3, not raised as a scope finding).** U2 §5.4 (lines 479–482) and WPQ_seed.md (lines 24–27) both state, in one sentence each: Tier Q stays in U2's protocol SPECIFIED BUT UNARMED, WP-1.5b's D-scope ships F+T only, pre-registration registers F+T only. Consistent with each other and with D-310 ("RESTRUCTURE OPTION D … SHIPS STAGES F AND T ONLY") and D-315 ("D-310 cut WP-1.5b to stages F and T"). No finding.
- **The one edit beyond the three named items (check 4).** U2-Z's WP-1.6 handoff bullet (lines 768–772) changed "no round has attacked the carve" to an accurate account of the u-rev-1 CONFIRMATION PASS FAIL. This is a legitimate fold-in consequence — the prior review's own existence falsified the old sentence, and it sits in the same section (U2-Z) as the material it's repairing — not out-of-scope content.
- **u-rev labels, review-status text, closing lines (check 4).** Grepped for `u-rev 1`, `u-rev 2`, `VERDICT FAIL`, `CONFIRMATION PASS`, `UNREVIEWED` across U2 — all instances consistent, no stale pre-repair state found (`THIS UNIT HAS NOT BEEN REVIEWED` correctly replaced with `THIS UNIT HAS BEEN REVIEWED, AND THAT REVIEW FAILED`; closing line correctly reads "u-rev 2. A repair…").
- **MEASURED/ESTIMATED marks (check 5).** No numeric claim in the repaired spans lost or lacks its mark. The new rule-3 constants and arithmetic are code constants, not measurements, and are correctly left unmarked (consistent with how rule constants are treated elsewhere in the document).

## Rejected with attempted reproducer

- **U2-A lineage table possibly stale.** The table (lines 84–96) lists only the superseded document's pre-carve revision history (1–7) and does not add a row for the u-rev-1 CONFIRMATION PASS that failed with F1/F2, despite the table's header claiming to show "what has attacked this unit's content, and at which revision." Attempted reproducer: compared table contents against `wp15b_U2_REVIEW.md`'s existence — confirmed the review is absent from the table. **Rejected as a definite finding**: the table's rows are keyed to superseded-document SHAs (`ec8f7fb`, `182f389`, `7ad466b`, `6feb40a`) and plausibly scopes itself to pre-carve attacks on the *design content*, while post-carve confirmation-pass lineage is tracked separately in the head (lines 64–70) and the U2-Z handoff bullet (lines 768–772) — both of which disclose it prominently. The document does not state the table's scope boundary explicitly enough to call this a contradiction rather than a legitimate structural split.
