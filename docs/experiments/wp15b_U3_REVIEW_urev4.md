# REVIEW-design (re-review) — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md` u-rev 4

## Header

- **Pinned revision reviewed:** `6f2dfe6`.
- **Matches HEAD:** **NO — HEAD advanced during the review, but the document under review did not move.** MEASURED: `git rev-parse HEAD` → `b067d47` (it was `6f2dfe6` when I started; the pinned revision matched HEAD exactly for the bulk of this review). `git diff 6f2dfe6 HEAD --stat` → 2 files, both additions, `crates/pistol-cli/tests/baseline_snapshot_tests.rs` and `tools/baseline_snapshot.sh` — the concurrent, out-of-scope work the dispatch anticipated. `git diff 6f2dfe6 HEAD -- docs/experiments/U3_tier_t.md | wc -l` → **0**. `docs/experiments/U3_tier_t.md` is byte-identical at `6f2dfe6` and at HEAD.
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 4**, 847 lines.
- **Prior review this repair answers:** `docs/experiments/wp15b_U3_REVIEW_urev3.md`, dispatched against `7d5d39c` — FAIL, 0 BLOCKING / 2 MAJOR / 3 MINOR. Read in full before this review.
- **Reviewer context:** FRESH. I did not write this unit, its repair, or the prior review, and had not seen any of them before this dispatch.
- **Scope:** the five repairs named in the dispatch and their blast radius, per the dispatch's proportionality instruction. U3's settled design content (M1's substance, the nine already-confirmed-repaired findings from u-rev 2) was not re-opened.
- **Tree left clean:** MEASURED, `git status --porcelain` → empty at the end of this review. No file edited, no git write command run, no worktree created. (Two files — `crates/pistol-cli/tests/baseline_snapshot_tests.rs` and `tools/baseline_snapshot.sh` — showed as modified in git status at the start of this session; they were clean by the end, are unrelated to `U3_tier_t.md`, and are noted per the dispatch as another agent's concurrent, out-of-scope work.)

## VERDICT: **FAIL**

**0 BLOCKING, 2 MAJOR, 0 MINOR.**

The two MAJORs named in the u-rev-3 review are genuinely repaired on their own narrow terms: the `23.2` attribution is re-scoped to three named, undecided candidates rather than falsely certified MEASURED, and `10.51 %`... rather, `2.78x`/`2.09x` replace the withdrawn `3.1× to 2.4×` pair in §6.5, which is correctly marked and dated everywhere the withdrawn figures survive. All three MINORs are also genuinely repaired to their stated fix shape.

But the fold-in sweep the dispatch called "the highest-yield check in this review" finds it twice more, freshly, inside this very u-rev's own repair:

1. The MAJOR-2 repair itself creates a new instance of the B7/MAJOR-4 restatement class in §6.5 (`45.82 = 2.78x`, `47.34 = 2.09x`) that is not folded into the U3-Z site table the same u-rev keeps current for every other site, immediately falsifying that table's "every site of it known at this u-rev" claim on the day it is written.
2. The MINOR-5 repair's own correction — stated as fact, three times in the disposition table and twice more in body text — that "U2 is at u-rev 2" is wrong. U2 reached u-rev 3 at `d85b049`, which is an ANCESTOR of `6f2dfe6` (this document's own u-rev-4 commit); U2's u-rev 3 had even already been reviewed and FAILED (`e1f5dc2`) before `6f2dfe6` landed. The repair meant to close exactly this class of staleness introduces it.

Both are D-305's shape — a repair landing in one place with a claim resting on it left un-re-read (or, here, un-re-checked) elsewhere — recurring inside the u-rev whose entire content is repairs of that shape.

---

# FINDINGS

## MAJOR

### A. The MAJOR-2 repair's own new citation in §6.5 is an unlisted B7/MAJOR-4-class site

**Claim reviewed** (U3-Z, lines 738–739, unchanged in scope by this repair): "B7's residual is a CLASS, and here is every site of it known at this u-rev (MAJOR 4)," followed by a ten-row table the u-rev-4 repair itself extended (MINOR 3's `under 400` row) and corrected (MAJOR 1's `23.2` row).

**Contradicting text — written by the MAJOR-2 repair itself** (§6.5, lines 290–292, new at u-rev 4):

> …see the census block's own `option C — staged, BATCHED only` row, `45.82 = 2.78x` on the r8 draw against `47.34 = 2.09x` on the r2 draw…

The identical pair recurs at U3-M item 5 (line 69's disposition table entry and the census-block row citation) and the disposition table itself (head, line 69): `2.78x` r8 draw → `2.09x` r2 draw.

**Reproducer.** MEASURED:

```
$ grep -n "45\.82\|47\.34\|2\.78x\|2\.09x" docs/experiments/U3_tier_t.md
170:| option C — staged, BATCHED only | 37.82 = 2.17x | 47.34 = 2.09x | 45.82 = 2.78x | 60.82 = 5.99x |
291:block's own `option C — staged, BATCHED only` row, `45.82 = 2.78x` on the r8
292:draw against `47.34 = 2.09x` on the r2 draw, a one-second run the document did
```

Line 170 is inside the pinned census block (`BEGIN CENSUS TABLE` at 145, `END` at 172). Lines 291–292 are outside it, in §6.5's STRONGEST SURVIVING ATTACK — the paragraph U3-Z item 2 says the owed ADR line quotes. `sed -n '748,759p'` (the U3-Z B7 site table) contains no row naming §6.5, `2.78x`, `2.09x`, `45.82` or `47.34`.

**Why it breaks.** The values are a literal copy of a census-block cell, exactly the shape (a rendered multiplier attached to a `staged, BATCHED only` row) that the block already prints outside four-decimal precision — the same shape as `70.8 %` (`BATCHED nodes`), which the table lists twice (once per citing unit). The document's own class definition — "any rounded, percentage or otherwise derived rendering of a census cell" outside the block — covers it without strain; nothing about being a multiplier rather than a percentage takes it out of the class, and the document treats `70.8 %`, `29 %`, `29.2 %` and `under 400` as the same class regardless of shape. This is not a hypothetical gap: it is the MAJOR-2 repair's own new text, landed in the same commit as the MINOR-3 repair that added a row for a different, older gap in the same table two lines below where this one belongs. The U3-Z table's completeness claim is false again, on the day this u-rev writes it, over a site this u-rev itself created — the exact "repair landing in one section with the copy resting on it left standing in another" shape the dispatch names as the cause of both prior MAJORs.

**Fix scope.** One row in the U3-Z table (site: §6.5's STRONGEST SURVIVING ATTACK; rendering: `2.78x` / `2.09x`; cell: `option C — staged, BATCHED only`, r8 and r2 draw columns), and §6.2's "five further sites" sentence becomes six. U3-local, not a design act — the same shape as MINOR 3's fix.

---

### B. The MINOR-5 repair's "U2 is at u-rev 2" is false; U2 was already at u-rev 3 before this commit landed

**Claim reviewed** (head, REVIEW STATUS table, line 72, new at u-rev 4):

> Separately found while fixing this: U4 is at u-rev 6 and **U2 is at u-rev 2**, not the u-rev 5 / u-rev 1 this unit's existing labels said — every existing cross-unit u-rev citation is corrected to the cited unit's current u-rev

The correction is applied at four sites: line 74 (`**U2** (u-rev 2)`), line 750 (U3-Z table, `**U2** (u-rev 2) §5.3`), and line 790 (`**U2** (u-rev 2) §2.2`) — plus the claim itself at line 72.

**Contradicting evidence.** MEASURED:

```
$ grep -n "u-rev" docs/experiments/U2_node_protocol.md | head -5
15:**u-rev 3.** Carved from `docs/experiments/wp15b_design.md` §2, §3, §5 and §14 at
21:**u-rev 2 was a REPAIR, not a new carve.** It answered
...
27:**u-rev 3 is a REPAIR of u-rev 2's own review, not a new carve.** It answers
```

U2's own header states it is at u-rev 3, not u-rev 2. This is not a case of U2 advancing after `6f2dfe6` was cut:

```
$ git merge-base --is-ancestor d85b049 6f2dfe6 && echo "YES d85b049 is ancestor of 6f2dfe6"
YES d85b049 is ancestor of 6f2dfe6
$ git log --oneline 6f2dfe6 -- docs/experiments/U2_node_protocol.md | head -3
d85b049 docs(experiments): U2 reaches u-rev 3 — the exceptions list finally counts the largest exception it always had...
56b0bec docs(experiments): U2 reaches u-rev 2 — ...
cf74594 docs(experiments): WP-1.5b's design is carved into four units...
```

`d85b049` ("U2 reaches u-rev 3") is an ancestor of `6f2dfe6` (U3's own u-rev-4 commit). Worse, U2's u-rev 3 had already been reviewed and had already FAILED before `6f2dfe6` landed — `git log --oneline 6f2dfe6` shows `e1f5dc2` ("U2's u-rev 3 re-review FAILS on the class its own repair was written to close…") between `d85b049` and `6f2dfe6`.

**Why it breaks.** This is D-311's own rule ("A citation of another unit names the unit AND the u-rev cited," to remove exactly the label ambiguity that "cost a review round") violated by the repair whose entire job was to enforce that rule. It is not a downstream-drift case the "matches HEAD" convention forgives — the correct value (`u-rev 3`) was already committed, and already reviewed, in this same repository's history before the commit under review was made; a `grep -n "u-rev" docs/experiments/U2_node_protocol.md` at authoring time would have shown it. A reader following `**U2** (u-rev 2) §5.3` to check the `70.8 %` figure, or `**U2** (u-rev 2) §2.2` to check the `quiet_top_k`/`widen_schedule` cardinality claim MINOR 5 itself flags as historically fragile, is pointed at a superseded revision of a sibling unit that has already failed its own re-review — precisely the scenario D-311 exists to foreclose.

**Fix scope.** Four sites (lines 72, 74, 750, 790): `u-rev 2` → `u-rev 3`. U3-local, not a design act.

---

# Verified with no finding

- **MAJOR 1 (u-rev-3 review) — repaired.** MEASURED, python3: `round(23.2917,1) = 23.3`; `int(23.2917*10)/10 = 23.2` (truncation); `round(23.2500,1) = 23.2`. The U3-Z table (line 753) now attributes `23.2` to three named candidates — `option C — Tier T (exact, NOT adopted)` at corpus roots (rounds, 23.2500→23.2), `option A — staged, BATCHED only` at the r2-draw column (23.20, literal), and `option C — Tier T (threshold, ADOPTED)` at corpus roots (truncates only, rounds to 23.3) — and does not decide between them. The MEASURED sentence (lines 761–771) and the MAJOR-12 OPEN bullet (lines 722–737) state the same three candidates in the same terms; all three agree. Every candidate is a Tier-T census row, so "which is Tier T and not Tier Q" holds regardless of which is the true provenance — verified directly against the census block, which has no Tier-Q row at all.
- **The other nine rows of the U3-Z site table reproduce.** MEASURED, python3 against the pinned block's cells: `6.83` ← 6.8333; `78.0 → 123.7` ← round(77.9583,1)/round(123.6615,1); `+0.17`/`+0.04` ← 46.5000−46.3333 / 23.2917−23.2500; `29 %` ← round(6.8333/23.2917×100,1) = 29.3, consistent with the printed integer `29 %`; `29.2 %` ← 100−70.8, and independently 4.2+25.0; `under 400` ← 376.4708 < 400 = True (MINOR 3's new row). All reproduce.
- **MAJOR 2 (u-rev-3 review) — repaired on its own terms.** §6.5 (lines 285–297) and U3-M item 5 (lines 630–641, the text that now occupies superseded §12 item 5's slot) read side by side without contradiction: both state that the withdrawn `3.1× to 2.4×` pair is not restated and that the census block's `option C — staged, BATCHED only` row is cited instead. MEASURED, table line 170: r8-draw column = `45.82 = 2.78x`, r2-draw column = `47.34 = 2.09x` — an exact match to what §6.5 and the disposition table cite. `grep -n "3\.1×\|2\.4×" docs/experiments/*.md` returns only the two occurrences inside `U3_tier_t.md` (lines 293, 635), both explicitly marked WITHDRAWN and attributed to revision 1, with the fuller citation (`the superseded §0 row 34, 6feb40a`) given at first occurrence (line 294). The attack is not blunted into vacuity: `2.78x → 2.09x` is still a real, stated shrink under re-sampling, and the "option committed was not the option measured" half of the attack is untouched.
- **MINOR 3 — repaired.** §6.2 (lines 201–210) now reads "five further sites," and §10's `1024`-derivation "under 400" bound has its own row in the U3-Z table (line 759). MEASURED, `376.4708 < 400` = True (`radius-2 ball`, playouts column). No other B7-class site among the pre-existing text was found missing (spot-checked: all percentage/multiplier figures outside the block that are not in the table — the bench-timing percentages in U3-M item 4, e.g. `10.51 %`, `81 %`, `−29.1 % / −41.3 % / −41.5 %` — derive from `blocking_covers`/`unblockable_double_threat` timing measurements, not from any census-block cell, so they are correctly outside the class).
- **MINOR 4 — repaired.** MEASURED, `docs/experiments/wp15b_U4_REVIEW.md:360–364`: the full sentence — "No finding, and *per the brief* its non-discharge is an IMPL gate and is not reported as a design defect *anywhere in this report*" — matches U3's quotation at line 74 verbatim, both clauses ("per the brief", "anywhere in this report") now present. The disposition is grounded on U2's OPEN item (MEASURED, `U2_node_protocol.md:811`, "RULE 5 IS UNDISCHARGED FOR THE NODE PROTOCOL ITSELF") and on U3-M item 4's declared hotspot substitution, not on the (disclaimed) U4 sentence alone.
- **MINOR 5 — the three newly-added labels are correct.** MEASURED via `git diff 7d5d39c 6f2dfe6`: the three sites the repair states it newly labeled (§6.4's `**U4** §9 amendment 1` cross-reference, U3-Z's MINOR-9 `**U4** §8.5` note, U3-Z's D-scope bullet's `**U2** §2.2` citation) each gained a u-rev label exactly where claimed. U4's label is also correct throughout: MEASURED, `U4_soundness_instrument.md:15`, "**u-rev 6.**" — every `**U4** (u-rev 6)` citation in `U3_tier_t.md` matches. (The U2 half of the same repair is MAJOR finding B above.)
- **The pin is GREEN.** MEASURED, `cargo test -p pistol-solver --test wp15b_census`:
  ```
  running 5 tests
  test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
  test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
  test wp15b_census_reproduces_the_registered_populations ... ok
  test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
  test the_census_pin_reads_every_carved_document_it_names ... ok
  test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.35s
  ```
- **The census block itself is unchanged since before the carve's last content commit.** MEASURED: `diff` between the block at `369d43a` (a later, unrelated commit) and the block at HEAD is empty (exit 0); `git log --oneline -- docs/experiments/U3_tier_t.md` shows the block has not moved since `9421d19`, the commit before U3's carve.
- **Fold-in law sweep — u-rev label and lineage table.** The head's u-rev banner (line 15), REVIEW STATUS block (lines 54–64), disposition table header (line 66), U3-A's lineage table (adds a `u-rev 3, 7d5d39c — FAIL` row at line 91, correctly has no `u-rev 4` row since it is unreviewed), the owed-list bullet (lines 95–96), U3-Z's "no review has run at this u-rev" bullet (lines 838–842), and the closing line (846, "U3, u-rev 4… plus the repairs answering `wp15b_U3_REVIEW.md` and `wp15b_U3_REVIEW_urev3.md`") are all internally consistent and correctly reflect u-rev 4 as unreviewed. No stale u-rev-3-as-current language found anywhere in this sweep.
- **`git status --porcelain` is empty; no file was edited and no git write command was run** by this review.

# Rejected, with the attempted reproducer

- **"MAJOR 1's repair decides which of the three `23.2` candidates is correct."** REJECTED. Read lines 722–737 and 761–771 in full: both explicitly state "deciding which of the three candidate cells is the true provenance remains MAJOR 12's open design act" and "the carve does not choose." No decision is made.
- **"The withdrawn `3.1× to 2.4×` pair survives unmarked somewhere outside `U3_tier_t.md`."** REJECTED. `grep -rn "3\.1×\|2\.4×" docs/experiments/*.md` returns only the two marked, dated occurrences inside `U3_tier_t.md` itself (lines 293, 635); the prior review's own copy in `wp15b_U3_REVIEW_urev3.md` is historical record of a finding, not a live restatement.
- **"The U3-Z table's row count regression (eight rows at u-rev 3 per the prior review vs. ten now) hides a second silently-dropped or duplicated site."** REJECTED as a distinct finding. The ten rows at u-rev 4 are exactly the prior nine (the `70.8 %` U2/U4 pair counted as two rows, `6.83` counted at its two citing sites, plus `23.2`, the sampler sentence, the threshold-repair deltas, `29 %`, `29.2 %`) plus MINOR 3's new `under 400` row; no row's content was found duplicated or dropped. (The apparent discrepancy with the prior report's "eight-row" phrasing traces to how rows were counted in that report's prose, not to a defect in the current table.)
- **"§6.5's abridged attack is blunted to a bare citation and no longer states a real attack, breaching CLAUDE.md's 'strongest surviving attack' requirement."** REJECTED. The abridged paragraph still asserts a concrete, numeric shrink (`2.78x → 2.09x`) under re-sampling and the unchanged "option committed was not the option measured" clause; both are load-bearing claims, not a mere pointer.
- **"The census multiplier column (`= N.NNx`) is independently re-derivable from the block's other rows, and the document's citation of it should be checked against a from-scratch recomputation."** REJECTED as out of scope for this check. The multiplier is the test harness's own rendered output (part of the pinned, CI-gated cell), not a document-side derivation; the relevant check is transcription accuracy (document text vs. table cell), which was verified exactly, not re-derivation of the harness's internal formula.

---

*REVIEW-design (re-review) of `docs/experiments/U3_tier_t.md` u-rev 4, at `6f2dfe6`; HEAD advanced to `b067d47` during the review and the document did not move. Fresh context. Every finding reproduced before reporting; every numeric claim marked MEASURED with its command. No file edited, no git write command run, `git status --porcelain` empty.*
