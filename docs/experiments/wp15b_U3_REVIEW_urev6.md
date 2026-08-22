# REVIEW-design (re-review) — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md` u-rev 6

## Header

- **Pinned revision reviewed:** `f0ae14c` (`git rev-parse f0ae14c` →
  `f0ae14c7c7285280677816fd85d6b6905b70d82a`). Per the dispatch, U3's own content
  last changed at `13621d3` ("U3 reaches u-rev 6…"); `f0ae14c` is a later commit
  touching a different unit.
- **Matches HEAD:** **NO — HEAD advanced during the review, but the document under
  review did not move.** MEASURED: `git rev-parse HEAD` →
  `b9f4aead811b4f603fc7e9044b655d2670ab60d4`. MEASURED, `git diff f0ae14c HEAD --
  docs/experiments/U3_tier_t.md | wc -l` → **0**. `docs/experiments/U3_tier_t.md`
  is byte-identical at `f0ae14c` and at HEAD. One untracked file,
  `docs/experiments/matrix_U4R_REDTEAM_round2.md`, is present in the working tree
  (`git status --porcelain`) — unrelated, out-of-scope concurrent work; not
  U3's own file and not touched by this review.
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 6**, 952 lines (MEASURED, `wc -l`).
- **Prior review this repair answers:** `docs/experiments/wp15b_U3_REVIEW_urev5.md`,
  dispatched against `7473a6f` — FAIL, 0 BLOCKING / 1 MAJOR / 0 MINOR. Read in full
  before this review, as was `wp15b_U3_REVIEW_urev4.md`.
- **Reviewer context:** FRESH. I did not write this unit, its repair, or either prior
  review, and had not seen any of them before this dispatch.
- **Tree left clean:** MEASURED, `git status --porcelain` → only the pre-existing
  untracked, unrelated file noted above. No file edited, no git write command run,
  no worktree created.

## VERDICT: **FAIL**

**0 BLOCKING, 2 MAJOR, 0 MINOR.**

u-rev 6's headline repair — converting all nine sites u-rev 5's MAJOR C found bare,
and replacing the asserted universal with a rule plus a registered `grep` command —
is real and independently verified on both halves: SHA ancestry, cited-unit u-rev at
that SHA, and cited content at the cited section all check out for every one of the
nine (Named Check 3), and the one site deliberately left in the old bare form (§10's
"Revision 6's §8.3", now correctly re-attributed to the SUPERSEDED document rather
than to **U4**) reads correctly against `6feb40a` (Named Check 4). The pinned census
block remains byte-identical to `6f2dfe6` (Named Check 6), the B7 site table needed
no addition (Named Check 5), and the gate is green.

But the round's own central claim — "so the rule and the tree now agree" — is false,
and the derived command registered as the instrument that was supposed to make it
un-falsifiable is blind to the exact violation by construction: a tenth live,
content-bearing cross-unit citation, present unchanged since u-rev 2 and never
brought into the fold by any of the five prior sweeps, carries no u-rev at all and
does not match the command's pattern because it lacks the bold markdown the pattern
requires (MAJOR D). This is the fourth recurrence of the class this document has
now failed on for four consecutive rounds, and it is a live demonstration of the
exact instrument-narrowing failure the dispatch names
(`matrix_M4_REDTEAM_round2.md` R11): a command narrowed until it is blind over the
case it exists to catch.

Separately, the new OPEN bullet's claim that D-331 "does not reach" MAJOR A/B/C and
"is silent about them" is contradicted by D-331's own text, which explicitly names
MAJOR A and MAJOR B as grounding instances of the very class the CLAIM-HOME law is
built on (MAJOR E).

---

# FINDINGS

## MAJOR

### D. "So every cross-unit citation in this unit now reads `(u-rev N, landed <sha>)`… the rule and the tree now agree" is false — a tenth bare citation survives, and the registered derived command cannot see it

**Claim reviewed** (head, lines 70–82):

> **WHETHER EVERY SITE OBEYS THE RULE IS DERIVED, NOT ASSERTED HERE.** A live
> citation that does not obey it is what this command returns, and the command is
> the claim's instrument rather than a sentence in this head:
>
> ```
> $ grep -n '\*\*U[124]\*\* §' docs/experiments/U3_tier_t.md | grep -v "landed"
> ```
>
> **u-rev 5 asserted the universal instead, and the universal was false at nine
> sites**… The nine were the inherited sites three consecutive rounds had scoped
> out of repair; u-rev 6 converts all nine rather than re-introducing the scoping
> caveat, **so the rule and the tree now agree.**

**Contradicting text.** MEASURED, the registered command exactly as printed:

```
$ grep -n '\*\*U[124]\*\* §' docs/experiments/U3_tier_t.md | grep -v "landed"
```

returns **no output, exit 0** — reads as confirmation. But a live, bare,
content-bearing cross-unit citation exists that this pattern cannot match because
it carries no bold markdown at all:

```
$ grep -n 'U[124] §' docs/experiments/U3_tier_t.md
402:any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
```

Full sentence (§10, lines 401–403): *"**This is the one place the count is stated;
U2 §2.2 and U3-Q cite it and do not restate it** (B5, which found it stated three
different ways across four sites)."* `U2 §2.2` here is unbolded — not `**U2**
§2.2` — so it cannot match `\*\*U[124]\*\* §` under any value of the `grep -v
"landed"` filter; the base pattern excludes it before the filter is even applied.

**This is not a fresh omission — it predates every round that has swept this
document for the class, including the round that just finished sweeping it.**
MEASURED, the identical sentence at every prior revision back to u-rev 2:

```
$ git show 1b645ac:docs/experiments/U3_tier_t.md | grep -n "U2 §2.2"
254:any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
$ git show 7d5d39c:docs/experiments/U3_tier_t.md | grep -n "U2 §2.2"
326:any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
$ git show 6f2dfe6:docs/experiments/U3_tier_t.md | grep -n "U2 §2.2"
334:any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
$ git show 7473a6f:docs/experiments/U3_tier_t.md | grep -n "U2 §2.2"
371:any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
```

Unchanged, byte-for-byte, since at least u-rev 2 (`1b645ac`). It is not one of the
nine MINOR-5-tracked sites (those are all bold `**U2**`/`**U4**` citations; this one
never appeared in that list at any round, MEASURED against
`wp15b_U3_REVIEW_urev3.md`'s named line numbers and `wp15b_U3_REVIEW_urev5.md`'s
nine-line dump — neither contains a line matching this sentence). The u-rev-5
reviewer's own exhaustive sweep (its "Check 2") swept for rounded/percentage/derived
census-cell renderings, a different class, and did not sweep for bare citations at
all beyond quoting MAJOR C's nine; no round's citation sweep, human or mechanized,
has ever caught this one.

**Attack on the instrument, not just the output (per the dispatch).** Constructed
minimal reproducer, run against the exact three-line shape the document's own class
covers — bold-bare, unbold-bare, and full-form:

```
$ printf '%s\n' '**U2** §5.3 test bare bold' 'U2 §5.3 test bare unbold' \
  '**U2** (u-rev 4, landed `abc`) §5.3 test full form' \
  | grep -n '\*\*U[124]\*\* §' | grep -v landed
1:**U2** §5.3 test bare bold
```

Line 2 — the unbold-bare shape, the exact shape of the real violation — does not
match. The command is not narrow by accident of this one document's phrasing; it is
narrow by construction, because it anchors on `\*\*U[124]\*\*`, and nothing in the
rule as stated (D-311/D-332: "a citation of another unit names the unit AND the
u-rev cited") requires the citation to be bold in the first place. A bare citation
typed without markdown emphasis is exactly as live and exactly as capable of going
stale as a bolded one, and the class sentence the head itself states ("a live
cross-unit citation… names the unit, the u-rev, and the revision…") makes no mention
of formatting. This is `matrix_M4_REDTEAM_round2.md` R11's failure mode: a command
narrowed until it is blind over the cases it is about.

**Why it breaks.** This is the fourth consecutive occurrence of the class this
document has now failed on for four rounds running — a completeness claim about this
document's own state, false in the commit that writes it. It is a materially worse
instance than the first three: MAJOR A, B and C were each an author's unchecked
assertion; this one is an author's assertion **backed by a registered instrument
explicitly built to make the class impossible**, and the instrument passes over the
exact violation it exists to catch. The document's own reasoning for adopting the
derived-command design (lines 105–112) is that "converting alone leaves an asserted
universal that the next citation added without the form falsifies silently" — the
derived command was supposed to close exactly that gap, and it does not: a citation
added (or, here, inherited) without the *bold* form falsifies the universal
silently, and the command reports clean while doing so.

**Fix scope.** Two independent fixes are available and neither is a design act:
(a) widen the pattern to `grep -noE 'U[124] §|U[124]-[A-Z]? §'` (or similar,
covering both bold and unbold forms) so the instrument actually matches its own
class description; and (b) convert the tenth site itself, `U2 §2.2` →
`**U2** (u-rev N, landed <sha>) §2.2`. Recommend both: (b) alone repeats the same
mistake finding C attacked (converting sites while leaving the detector narrow);
(a) alone leaves the tenth site converted-by-luck but still undetectable if an
eleventh unbold site is added later.

---

### E. The new OPEN bullet claims D-331 "does not reach" MAJOR A/B/C and "is silent about them" — contradicted by D-331's own text, which names MAJOR A and MAJOR B as grounding instances of the class it is built on

**Claim reviewed** (U3-Z, OPEN list, lines 790–804):

> **D-331 DOES NOT REACH THE DEFECT THAT HAS NOW FAILED THIS UNIT THREE ROUNDS
> RUNNING, AND THAT IS FOR THE ARCHITECT AND NOT FOR THIS UNIT.** MAJOR A, MAJOR B
> and MAJOR C are one class… D-331 (R15) requires every claim to have ONE HOME and
> every other occurrence to be a pointer. **All three of these were at their home.**
> They were not restatements of anything; they were assertions about a set the
> author had not enumerated. So the CLAIM-HOME law, as landed, is silent about them…

**Contradicting text — D-331 itself** (`docs/decisions.md`, D-331):

> The class is not confined to U4: U2's *"Four exceptions"* list has now been found
> incomplete twice (F3, then F5 in `wp15b_U2_REVIEW_urev3.md`), **and U3's u-rev-4
> repair falsified its own B7 site-table completeness claim on the day it wrote it
> and simultaneously stated *"U2 is at u-rev 2"* against a u-rev 3 that was already
> an ancestor commit and had already failed its own review
> (`wp15b_U3_REVIEW_urev4.md`, MAJOR A and B).** **THE DIAGNOSIS, AND IT IS WHAT
> MAKES THIS A LAW RATHER THAN MORE CARE:** every one of those defects is a SECOND
> COPY of content whose FIRST copy is correct and lives somewhere else — in an ADR
> line, in a selection record, in another section of the same unit.

D-331 does not merely fail to exclude MAJOR A and MAJOR B — it names them by review
citation, in its own ground section, as two of the instances the rule is built on.
"D-331… is silent about them" is not an accurate description of a law whose own text
quotes them.

**Why it breaks, and where the OPEN bullet's argument does and does not hold.**
Splitting the three:

- **MAJOR B ("U2 is at u-rev 2") is a restatement, and the bullet's own reasoning
  says so if applied consistently.** The claim asserts a fact — U2's current
  u-rev — whose home is U2's own header (`docs/experiments/U2_node_protocol.md:15`,
  `**u-rev N.**`), a fact this document's own lead-in paragraph (lines 57–60)
  independently states is exactly the hazard: *"a bare `**U2** (u-rev 2)` is a live
  claim about another document's present state… and nothing in this unit re-reads
  it."* D-331's diagnosis — "a SECOND COPY of content whose FIRST copy is correct
  and lives somewhere else" — describes MAJOR B exactly: the second copy is U3's
  stale "u-rev 2," the first, correct copy lives at U2's own head. The OPEN bullet's
  claim that MAJOR B "was not a restatement of anything; it was an assertion about a
  set the author had not enumerated" does not fit MAJOR B at all — there is no "set"
  being enumerated in "U2 is at u-rev 2," and the fact restated is not
  self-originated, it is copied (wrongly) from another document.
- **MAJOR A and MAJOR C are more defensibly "at their home"** — each is an original,
  self-generated enumeration attempt (the B7 site list; the citation-form sweep)
  with no other section stating the same completeness fact first, so calling them
  "second copies" is a stretch even in D-331's own diagnosis. But D-331's ground
  section still explicitly discusses MAJOR A (via the same parenthetical citation
  that names MAJOR B), so "silent about them" overstates the case for A as well —
  D-331 discusses it, even if the binding mechanism arguably does not cover it.

**Consequence, and why this matters more than a wording nitpick.** The dispatch
names this the ground on which a project-level rule may later be written. An
architect reading "D-331… is silent about [MAJOR A/B/C]" without re-reading D-331's
own text could conclude a new rule is needed to cover a gap that, for at least
MAJOR B, D-331 already claims to cover — either amending D-331 redundantly, or
missing that D-331's diagnosis itself needs correcting (since it lumps a genuine
restatement, MAJOR B, together with two claims — A and C — that are not
restatements in the same sense). The correct framing is narrower than what the
bullet states: D-331's *ground section* discusses all three; what is genuinely open
is whether D-331's *binding mechanism* (clauses 1–4, "WHAT R15 BINDS") extends to an
originally-authored, non-copied completeness claim — which is true only of MAJOR A
and MAJOR C, not MAJOR B.

**Fix scope.** U3-local, not a design act: narrow the bullet's claim to MAJOR A and
MAJOR C (drop MAJOR B from the "not a restatement" grouping, or state plainly that
MAJOR B is a restatement D-331 already covers and the open question is only about
A/C-shaped self-completeness claims), and correct "D-331… is silent about them" to
something like "D-331's ground section names all three; its binding clauses,
addressed to restating a claim whose home is elsewhere, do not obviously reach an
originally-authored, uncopied completeness claim (MAJOR A, MAJOR C) — MAJOR B is a
restatement D-331 already covers."

---

# Verified with no finding

- **Check 3 — the nine converted sites, both halves.** MEASURED, `git merge-base
  --is-ancestor 7dfd047 HEAD` → true (exit 0); `git merge-base --is-ancestor
  0f49c90 HEAD` → true (exit 0). `git show 7dfd047:docs/experiments/U2_node_protocol.md
  | grep -n "^\*\*u-rev"` → `**u-rev 4.**`, matching every `**U2** (u-rev 4, landed
  \`7dfd047\`)` citation; `git show 0f49c90:docs/experiments/U4_soundness_instrument.md
  | grep -n "^\*\*u-rev"` → `**u-rev 7.**`, matching every `**U4** (u-rev 7, landed
  \`0f49c90\`)` citation. Content spot-checks at the cited sections: U2 §5.3 at
  `7dfd047` states FILTERED emits the cover union alone and BATCHED emits "Tier T ∪
  Tier Q" (matches U3 line 289's "cover union alone… Tier T plus the quiet cut"); U2
  §5.4 states "Tier F and Tier T are emitted whole" (matches U3 line 448's "emitted
  whole and are not counted against it"); U2 §5.2 references
  `min_hitting_set_exceeds` (matches U3 line 567); U4 §8 heading is "MATRIX M3 — the
  soundness instrument" (matches U3 line 345's "the instrument"); U4 §8.4's mutation
  table row M7 reads "Tier T qualifies at ≥3 for the mover (option A)" (matches U3
  line 348's citation exactly, word for word).
- **Check 4 — the one site deliberately left bare (§10, "Revision 6's §8.3").**
  MEASURED: "Revision 6" of the superseded document is `2d07ff6`
  (`git log --oneline --follow -- docs/experiments/wp15b_design.md`, the commit
  labeled "WP-1.5b reaches revision 6"). Its §8.3 states: *"**All three staged
  tactical configs disable the quiet cut** (`quiet_top_k` above the whole pool), not
  just the two gate ones."* — matching U3's quoted "all three staged tactical
  configs disable the quiet cut" verbatim. MEASURED, the same sentence is still
  present, unchanged, at `6feb40a`'s §8.3 (the SHA U3 actually names), so citing
  `6feb40a` for it is accurate; §10 and §15 of `6feb40a` also carry the "two of
  them"/"the two gate configs" disagreement U3 quotes, MEASURED at lines
  1354–1360 of that file. Confirmed this is genuinely NOT true of **U4** as it now
  stands: MEASURED, `docs/experiments/U4_soundness_instrument.md:690`, "**BOTH
  staged TACTICAL configs disable the quiet cut**" — "both," not "all three" —
  so attributing the "all three" sentence to U4 would be a content error, and U3's
  disclaimer ("NOT **U4** as it now stands") correctly avoids making it.
- **Check 5 — the B7 site table, re-swept independently.** MEASURED, the table is
  byte-identical to u-rev 5's (`diff` between the table blocks at `7473a6f` and
  HEAD → empty). MEASURED, no new rounded/percentage/multiplier rendering of a
  census cell was introduced by u-rev 6's diff: `diff 7473a6f HEAD --
  docs/experiments/U3_tier_t.md | grep '^>' | grep -oE '[0-9]+\.[0-9]+|[0-9]+ %'`
  returns only section numbers (§8.3, §5.3, etc.) and one recurrence of `70.8 %` at
  an already-tabled site (MINOR 9's sentence, gaining only its SHA/u-rev, not a new
  occurrence). No untabled site found.
- **Check 6 — the pinned census block, byte-identical to `6f2dfe6`.** MEASURED,
  `diff <(sed -n '212,239p' docs/experiments/U3_tier_t.md) <(git show
  6f2dfe6:docs/experiments/U3_tier_t.md | sed -n '145,172p')` → empty, exit 0.
  (Note: a naive `awk '/BEGIN CENSUS TABLE/,/END CENSUS TABLE/'` extraction is
  unsafe on this document — the phrase "`BEGIN CENSUS TABLE` marker" recurs in prose
  at line 875/772 in the two revisions respectively, outside the real markers,
  and pulls in ~570 extra lines if used naively. Exact line-range extraction from
  the literal HTML-comment markers was used instead.)
- **Check 1 (partial) — "two citations deliberately NOT in the form."** MEASURED,
  `grep -n '\*\*U2\*\* (u-rev 2)' docs/experiments/U3_tier_t.md` → exactly 2 hits
  (lines 59, 61), matching "the two in the paragraph above." The count of *named*
  exceptions is accurate as far as it goes; it does not (and, given finding D,
  cannot) account for the tenth, undeclared site — that is finding D, not a defect
  in this specific sentence's arithmetic.
- **The gate.** MEASURED, `cargo test -p pistol-solver --test wp15b_census`:
  ```
  running 6 tests
  test a_document_quoting_the_carve_marker_is_not_a_carve_member ... ok
  test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
  test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
  test wp15b_census_reproduces_the_registered_populations ... ok
  test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
  test the_census_pin_reads_every_carved_document_it_names ... ok

  test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.61s
  ```
  All green, including a new test (`a_document_quoting_the_carve_marker_is_not_a_carve_member`)
  that resolves the `matrix_U4R_*` false-positive the u-rev-5 review flagged as an
  out-of-scope observation — not caused by U3 and not this review's to charge either
  way, and now fixed.

# Rejected, with the attempted reproducer

- **"The bare `**U4**`/`**U4**'s` occurrences at lines 313 and 320 ('the measurement
  went to **U4**'; 'must read it in **U4**') are additional bare citations beyond
  the tenth site found in finding D."** REJECTED. Both are anaphoric back-references
  to the fact already fully cited two sentences earlier in the same paragraph
  (`**U4** (u-rev 7, landed \`0f49c90\`) §9 amendment 1`, line 315) — not
  independent claims capable of going stale on their own, the same way "Smith's
  finding is Y" following "(Smith 2020) argues X" does not need its own citation.
  Neither carries a `§` reference, so neither matches the class the head's own rule
  and the derived command are stated against.
- **"Line 62's 'U2 had already reached u-rev 3' and line 794's quoted 'U2 is at
  u-rev 2' are additional live, uncited claims."** REJECTED. Both are inside the
  explicitly historical/quoted description of the bug being fixed (line 57–68's
  lead-in, and the OPEN bullet's marked quotation at line 794), not live claims
  about present state — the same carve-out the head itself names for lines 59/61.
- **"The census multiplier/staged rows should be independently re-derived from the
  block's raw cells rather than transcription-checked."** Attempted: out of this
  round's scope. The nine conversions and the §10 correction are citation-form and
  attribution changes only; no census number moved at u-rev 6 (confirmed under
  Check 6), so re-deriving the harness's own arithmetic is not this round's
  question.
- **"`docs/experiments/matrix_U4R_REDTEAM_round2.md`, present untracked in the
  working tree, is in scope for this review."** REJECTED. It is unrelated,
  out-of-scope concurrent work on a different unit (U4's restructure), not staged,
  not committed, and not referenced by `U3_tier_t.md`. `git diff f0ae14c HEAD --
  docs/experiments/U3_tier_t.md` is empty, confirming U3's own content is
  untouched by it.

---

*REVIEW-design (re-review) of `docs/experiments/U3_tier_t.md` u-rev 6, at `f0ae14c`
(U3's own content unchanged since `13621d3`); HEAD advanced to `b9f4aea` during the
review and the document did not move. Fresh context. Every finding reproduced
before reporting; every numeric claim marked MEASURED with its command. No file
edited, no git write command run; `git status --porcelain` shows only a
pre-existing, unrelated untracked file.*
