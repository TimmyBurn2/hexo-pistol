# REVIEW-design (re-review) — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md` u-rev 5

## Header

- **Pinned revision reviewed:** `7473a6f` (`git rev-parse 7473a6f` →
  `7473a6fb6c775e0e87cd7dd9821b459be6cc41f5`).
- **Matches HEAD:** **NO — HEAD advanced during the review, but the document under
  review did not move.** MEASURED: `git rev-parse HEAD` → `a1d425cbe3dbd42a3c7a4c3e3f739cd07a65a608`
  (it moved from `7473a6f` through at least `97aa4d6` to `a1d425c` over the course of
  this review — three commits landed: `docs/experiments/matrix_U4R_REDTEAM.md`,
  `docs/experiments/matrix_U4R_restructure_rev2.md`, `docs/experiments/wp15b_trackC_R19_REVIEW_impl.md`,
  and a 4-line append to `docs/decisions.md`). MEASURED, `git diff 7473a6f HEAD --
  docs/experiments/U3_tier_t.md | wc -l` → **0**. `docs/experiments/U3_tier_t.md` is
  byte-identical at `7473a6f` and at HEAD. All concurrent work is unrelated to U3 and
  none of it is U3's own file.
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 5**, 906 lines (MEASURED, `wc -l`).
- **Prior review this repair answers:** `docs/experiments/wp15b_U3_REVIEW_urev4.md`,
  dispatched against `6f2dfe6` — FAIL, 0 BLOCKING / 2 MAJOR / 0 MINOR. Read in full
  before this review, as was `wp15b_U3_REVIEW_urev3.md`.
- **Reviewer context:** FRESH. I did not write this unit, its repair, or either prior
  review, and had not seen any of them before this dispatch.
- **Tree left clean:** MEASURED, `git status --porcelain` → empty at the end of this
  review. No file edited, no git write command run, no worktree created.

## VERDICT: **FAIL**

**0 BLOCKING, 1 MAJOR, 0 MINOR.**

u-rev 5's two substitutions for the u-rev-4 MAJORs hold up well under direct attack.
The B7 site table (MAJOR A's fix) now correctly carries the new §6.5 row, and an
exhaustive sweep of the whole document for the class — any rounded, percentage or
otherwise derived rendering of a census cell outside the pinned block — finds **no
site the table misses**, breaking the three-rounds-running pattern the dispatch
named. The completeness claim itself is correctly removed from the B7 bullet, and
§6.2/U3-Z no longer contradict each other about whether the table is a boundary. The
deleted u-rev-4 disposition table cost nothing load-bearing: every one of its five
REPAIRED rows' content is still findable at its one home, and the two OPEN items are
correctly carried forward as live bullets. The cross-unit citations that now carry
`(u-rev N, landed <sha>)` are correct on both halves — SHA ancestry and cited content
both verified.

But the fix for MAJOR B overclaims. The document asserts, in two places, that
**"every cross-unit citation in this unit now reads `(u-rev N, landed <sha>)`."**
That is false: nine cross-unit references to other units — most of them the exact
nine sites the u-rev-3 review's MINOR 5 already named and every round since has
explicitly scoped OUT of its own repair — still carry no u-rev at all, bare or
otherwise. u-rev 4's disposition table (now deleted) at least stated the true, bounded
claim ("the nine sites inherited from u-rev 2 are unchanged, per the review's own
scope"); u-rev 5 drops that caveat and replaces it with an unqualified claim that is
false on the day it is written — the same shape as MAJOR A and MAJOR B, now inside
the very repair that exists to close that shape.

---

# FINDINGS

## MAJOR

### C. "Every cross-unit citation in this unit now reads `(u-rev N, landed <sha>)`" is false — nine citations carry no u-rev at all

**Claim reviewed** (head, lines 63–64, new at u-rev 5):

> **So every cross-unit citation in this unit now reads `(u-rev N, landed <sha>)`,
> which is a historical fact and cannot go stale.**

Restated at the REVIEW STATUS disposition table (line 84, MAJOR B's answer):

> **ANSWERED BY CHANGING THE CITATION FORM, NOT ONLY THE FOUR VALUES.** Every
> cross-unit citation in this unit now reads `(u-rev N, landed <sha>)` — a historical
> fact that cannot go stale — … The four sites are corrected, and so are the four
> `**U4** (u-rev 6)` citations the u-rev-4 round left standing…

**Contradicting text.** MEASURED, `grep -no '\*\*U[124]\*\*[^.,;)|]\{0,60\}'
docs/experiments/U3_tier_t.md` cross-checked against
`grep -no '\*\*U[24]\*\* (u-rev[^)]*)' docs/experiments/U3_tier_t.md`:

Live citations carrying the full `(u-rev N, landed <sha>)` form (7 total): lines 106,
803, 848 (`**U2** (u-rev 4, landed \`7dfd047\`)`) and 276, 284, 804, 893 (`**U4**
(u-rev 7, landed \`0f49c90\`)`). (Lines 58, 60 and 84 quote the *old, bare* form
historically, describing the bug being fixed — not live citations.)

Live citations carrying **no u-rev at all** (9 total):

```
258:  **How to read it.** The FILTERED and BATCHED rows are separate because **U2** §5.3 emits
314:instrument (**U4** §8) shows C dropping a cell a proven tactic needs, C is replaced by
317:1 omitted:** if the instrument is GREEN while mutation M7 (**U4** §8.4; Tier T at ≥3 for the
359:  **U2** §5.3's licensed shortening of mate distances on lost positions both change what
378:| ... **U4** §8.3's TACTICAL SUITE gate derivation requires ...
383:Revision 6's **U4** §8.3 TACTICAL SUITE gate said "all three staged tactical configs disable the quiet cut" while §10
417:  emitted whole and are not counted against it (**U2** §5.4).
536:   accelerates `min_hitting_set_exceeds`, which under M5-E (**U2** §5.2) is **no longer
565:   measured over all 24 corpus roots, but **U2** §5.3 does not extract Tier T on the
```

These are content-bearing cross-unit citations — each attributes a specific fact to a
named section of U2 or U4 (what §5.3 emits, what §5.4 covers, what §8.3's gate
requires, what mutation M7 is) — exactly the shape the document's own lead-in
paragraph (lines 56–59) says is the hazard: *"a bare … citation is a live claim about
another document's present state, it goes false the moment that document is bumped,
and nothing in this unit re-reads it."* Every one of these nine is bare in exactly
that sense, only without even a stale number to be wrong about.

**These are not a fresh omission — they are the exact nine sites the project already
named and explicitly scoped out three rounds running.** MEASURED,
`wp15b_U3_REVIEW_urev3.md` MINOR 5: *"Nine further un-u-rev'd citations at lines 218,
274, 277, 314, 333, 338, 372, 491, 520 are inherited from u-rev 2 and were not flagged
in the prior round; they are noted, not charged to this repair."* MEASURED, the
u-rev-4 disposition table for MINOR 5 (`git show 6f2dfe6:docs/experiments/U3_tier_t.md`,
line 72): *"the nine sites inherited from u-rev 2 are unchanged, per the review's own
scope."* Nine then, nine now — the count survives three repair rounds unchanged
while the document's own claim about it goes from an honest, bounded statement to an
unqualified false one.

**Why it breaks.** This is the identical defect class MAJOR A and MAJOR B were rated
for — a completeness claim false in the commit that writes it — occurring a third
time in the same document, and this repair is the one D-331 and D-332 were landed to
prevent recurring. It is worse than a mere omission: **the u-rev-4 disposition table
carried the true, scoped statement of this exact gap, and u-rev 5 deleted that
caveat** (per D-331's own license to drop history once it is superseded) **while
promoting the claim it used to bound into an unqualified universal.** A reader who
trusts line 63–64 or line 84 and goes looking for a stale u-rev number to distrust at
line 258 or 417 will find none — but they will also not be told that the sentence
they are reading was never checked against those nine sites at all.

**Fix scope.** Either (a) add `(u-rev N, landed <sha>)` to the nine sites listed above
— U3-local, not a design act, the same fix shape as MAJOR B's original four; or (b)
scope the claim honestly, as u-rev 4's disposition table did: "every citation that
previously named a u-rev now also names the SHA; N further sites cite a section
without naming any u-rev and are unchanged." Recommend (a): the nine sites are
exactly as content-bearing as the ones already fixed, and (b) reintroduces the
inherited-debt caveat this u-rev's own restructuring reasonably wanted to retire.

---

# Verified with no finding

- **Check 2 — the B7 site table, exhaustively re-swept.** The new §6.5 row is
  correct: MEASURED, table line 206 (census block), `option C — staged, BATCHED only`
  = `37.82 = 2.17x | 47.34 = 2.09x | 45.82 = 2.78x | 60.82 = 5.99x` (corpus roots | r2
  draw REPORTED | r8 draw SUPERSEDED | playouts). §6.5 (lines 328–329) cites `45.82 =
  2.78x` on the r8 draw against `47.34 = 2.09x` on the r2 draw — an exact match, and
  the U3-Z table's new row (line 805) cites the same pair against the same cell.
  **No further missing site was found.** I rounded/derived every one of the block's
  105 cells to the shapes the document actually uses (`%`, `x`-multiplier, `+N.NN`,
  one-sided bound) and grepped the whole document outside `BEGIN`/`END CENSUS TABLE`
  (lines 181–208) for every hit: `70.8 %` (2 sites: U2 §5.3, U4 §8.4 — both tabled),
  `6.83` (2 sites: §6.3, §10 — both tabled), `78.0 → 123.7` (1 site: §6.2 — tabled),
  `+0.17`/`+0.04` (1 site: §6.1 — tabled), `29 %` (2 sites: §6.3, §10 — one combined
  row), `29.2 %` (1 site: U3-M item 4 — tabled), `under 400` (1 site: §10 — tabled),
  `2.78x`/`2.09x` (2 sites: §6.5, table's own citation of itself — tabled). All other
  percentage/multiplier figures found outside the block (`10.51 %`, `81 %`, `−29.1 %
  / −41.3 % / −41.5 %`, `1.10×`/`1.35×`/`1.24×`/`1.05×`, `0.6 %–3.7 %`, `6×`, `3×`,
  `6 × 17 = 102`, `18 × 17 = 306`, `22/22/22/18/15`) derive from bench-timing
  measurements or independent combinatorial bounds, not from any census-block cell —
  correctly outside the class, matching the u-rev-4 reviewer's own disposition of the
  same figures. `3.1×`/`2.4×` occur twice (lines 330, 672), both explicitly marked
  WITHDRAWN and dated — historical record, not a live restatement.
- **Check 3(ii)/(iii) — the seven full-form citations are correct on both halves.**
  MEASURED: `git merge-base --is-ancestor 7dfd047 HEAD` → true; `git merge-base
  --is-ancestor 0f49c90 HEAD` → true. `git show 7dfd047:docs/experiments/U2_node_protocol.md
  | grep -n "^\*\*u-rev"` → `**u-rev 4.**`, matching U3's citation exactly; `git show
  0f49c90:docs/experiments/U4_soundness_instrument.md | grep -n "^\*\*u-rev"` →
  `**u-rev 7.**`, matching. Both SHAs are also U2's/U4's *current* head u-rev
  (`docs/experiments/U2_node_protocol.md:15` → u-rev 4; `U4_soundness_instrument.md:15`
  → u-rev 7), so none of the seven has gone stale since. Content spot-checks: U2 line
  854 carries `RULE 5 IS UNDISCHARGED FOR THE NODE PROTOCOL ITSELF` (matches line 106's
  citation); U2 line 429 carries `70.8 % of corpus roots` inside §5.3 (matches line
  803); U4 line 797, inside §8.4 (770–799), carries the `70.8 %` BATCHED-population
  reference (matches line 804); U4 lines 1032/1414–1416 (§9.1/§9 amendment 1) carry
  the `depth_at_500ms` dead-band-of-`about 2×` finding cited at line 284, and U4 line
  1032 carries the pinned triple `2 / 2 / 1` cited at line 276.
- **Check 4 — the deleted u-rev-4 disposition table cost nothing load-bearing.**
  MEASURED, `git show 6f2dfe6:docs/experiments/U3_tier_t.md` lines 66–74: the table
  held 7 rows (MAJOR 1, MAJOR 2, MINOR 3, MINOR 4, MINOR 5 = REPAIRED; revision-7
  MAJOR 12 = STILL OPEN; revision-7 MAJOR 9 = IMPL-GATE ITEM). The two OPEN/gate rows
  are carried forward verbatim as live bullets at u-rev 5 (lines 101–109). Every
  REPAIRED row's substantive content is independently findable at its stated home:
  MAJOR 1's re-attribution → U3-Z's MAJOR-12 bullet and table row (759–774, 807);
  MAJOR 2's §6.5-as-carrier → §6.5 itself (311–334) and U3-M item 5 (656–677); MINOR
  3's `under 400` row → U3-Z table (813); MINOR 4's full-quote attribution → U2's OPEN
  item and U3-M item 4, both still cited at lines 106–109. The one loss found is
  finding C above: MINOR 5's row stated the *bounded* scope of the citation-form gap
  ("nine sites inherited … unchanged, per the review's own scope"); u-rev 5 keeps no
  pointer to that bound and instead states the unqualified claim finding C falsifies.
- **Check 5 — the pinned census block is byte-identical to `6f2dfe6`.** MEASURED,
  `diff` between `awk '/BEGIN CENSUS TABLE/,/END CENSUS TABLE/'` extracted from
  u-rev 5 (HEAD) and from `6f2dfe6` → empty, exit 0. MEASURED, no four-decimal figure
  from the block (`grep -noE '[0-9]+\.[0-9]{4}'` restricted to the block's own values)
  recurs outside lines 181–208.
- **Check 6 — §6.2 and U3-Z do not contradict each other, and neither asserts the
  table is a boundary.** §6.2 (lines 226–247) states the pin's narrow (four-decimal
  only) refusal, then states the CLASS and points at the U3-Z table with "THIS
  PARAGRAPH POINTS AT IT AND STATES NO COUNT." U3-Z's B7 bullet (lines 775–799)
  states "THIS BULLET NO LONGER CLAIMS THE TABLE IS COMPLETE," gives the reason
  (false at every u-rev it was made), and states the class sentence is what a reader
  tests a site against, not the list. No sentence elsewhere in the document (swept
  via `grep -n "site of it known\|table is complete\|every site"`) asserts the
  table is the boundary.
- **Pin mechanics — the three census tests that exercise U3's own content are GREEN.**
  MEASURED, `cargo test -p pistol-solver --test wp15b_census` at HEAD
  (`a1d425c`): `the_carved_design_units_carry_this_censuss_table_verbatim ... ok`,
  `the_census_pin_reads_every_carved_document_it_names ... ok`,
  `wp15b_census_reproduces_the_registered_populations ... ok`. (Fourth gate's failure
  is recorded separately below — it is not caused by U3.)

# GATE — cited verbatim, with an out-of-scope failure flagged

```
$ cargo test -p pistol-solver --test wp15b_census
running 5 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... FAILED
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

failures:

---- the_pins_document_list_is_the_set_of_carved_documents_on_disk stdout ----
thread 'the_pins_document_list_is_the_set_of_carved_documents_on_disk' (58795) panicked at crates/pistol-solver/tests/wp15b_census.rs:878:5:
assertion `left == right` failed: this pin's CARVE_DOCS list and the carved documents on disk disagree. Files carrying the marker but not read by the pin are green-over-unread; files listed but not on disk are already a panic in carve_documents().
  left: ["U1_gate_supersession.md", "U2_node_protocol.md", "U3_tier_t.md", "U4_soundness_instrument.md", "WPQ_seed.md", "section_owner_table.md"]
 right: ["U1_gate_supersession.md", "U2_node_protocol.md", "U3_tier_t.md", "U4_soundness_instrument.md", "WPQ_seed.md", "matrix_U4R_REDTEAM.md", "matrix_U4R_restructure_rev2.md", "section_owner_table.md"]

test result: FAILED. 3 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.45s
```

**Not a finding against U3.** `git diff 7473a6f HEAD -- docs/experiments/U3_tier_t.md`
is empty (verified above): U3's own content is unchanged. The failure is caused by
two files landed after the pinned revision — `matrix_U4R_REDTEAM.md` and
`matrix_U4R_restructure_rev2.md`, U4-restructure red-team/matrix documents that quote
the `CARVE_MARKER` Rust constant's string literal verbatim inside a code block, which
causes the scanner's `.contains(CARVE_MARKER)` check to treat them as carved units —
without being added to `CARVE_DOCS`. This is the same "concurrent, out-of-scope work"
shape both prior U3 reviews noted and left to the architect. Flagged here as an
**observation for the architect, not a finding against U3.**

# Rejected, with the attempted reproducer

- **"The class-statement paragraph at §6.2 (lines 237–247), which names `70.8 %`,
  `6.83` and `23.2` as example shapes, is itself an uncredited restatement outside the
  table's one home."** REJECTED. The paragraph explicitly states it "STATES NO COUNT"
  and the three values are quoted as illustrative shapes of the CLASS definition, not
  attributed to any specific site — this is the quotation D-331 licenses ("a marked
  verbatim quotation carrying its source … is a pointer"), not a restated finding.
- **"The u-rev-4 round's now-deleted disposition table's MINOR-4 full quote of
  `wp15b_U4_REVIEW.md`'s MAJOR 9 sentence is a loss — u-rev 5 no longer quotes it in
  full."** REJECTED as a distinct finding beyond what MINOR-4's shape requires. The
  live bullet at lines 105–109 preserves the *conclusion* MINOR 4 required (the
  disposition attributed to U2's OPEN item and U3-M item 4's substitution, not to the
  bare U4 sentence) and paraphrases the scoping accurately ("scoped to that report and
  is not a project ruling" ≈ "per the brief … anywhere in this report"). No load-bearing
  content is lost, only the exact wording, and D-331 does not require verbatim
  preservation of an already-answered finding's quote.
- **"§10's 'every other key is identical to the radius document it is the counterpart
  of' (line 394) is an unchecked completeness claim."** Attempted: no reproducer
  available inside this document or repo scan (the claim compares against
  `configs/*.toml`, external files this review did not open) and the sentence is
  unchanged since at least revision 4 per the u-rev-3/u-rev-4 reviews' silence on it —
  not new to u-rev 5. REJECTED as out of this round's scope; noted, not charged.
- **"The `matrix_U4R_*` census-gate failure is a defect this review must charge
  against U3."** REJECTED — `git diff 7473a6f HEAD -- docs/experiments/U3_tier_t.md`
  is empty; the failure's cause (two unrelated files landed after the pinned
  revision) has nothing to do with U3's content, and the three tests that exercise
  U3's own pinned block are green.

---

*REVIEW-design (re-review) of `docs/experiments/U3_tier_t.md` u-rev 5, at `7473a6f`;
HEAD advanced to `a1d425c` during the review and the document did not move. Fresh
context. Every finding reproduced before reporting; every numeric claim marked
MEASURED with its command. No file edited, no git write command run, `git status
--porcelain` empty.*
