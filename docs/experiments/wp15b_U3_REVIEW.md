# REVIEW-design — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md`

## Header

- **Pinned revision reviewed:** `1b645ac02f20a81d4583e8bbfda2133865f2a285`.
- **Matches HEAD:** YES. `git rev-parse HEAD` → `1b645ac02f20a81d4583e8bbfda2133865f2a285`;
  `git status --porcelain` → empty. Nothing moved during the review.
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 2**, 603 lines (MEASURED,
  `wc -l`). u-rev 2 confirmed as the B3 repair: `git diff cf74594 9421d19 --
  docs/experiments/U3_tier_t.md` is 3 hunks — the u-rev label, the closing line, and
  two `§8.3(a)` → `§8.3 TACTICAL SUITE gate` retargets. D-311's label rule is honoured.
- **Reviewer context:** FRESH. I did not author this unit and had not seen it.
  Background read but not reviewed: `section_owner_table.md`,
  `restructure_selection_15b.md`, `wp15b_design_rev7_REVIEW.md`, `WPQ_seed.md`,
  D-304/D-309–D-316, and `git show 6feb40a:docs/experiments/wp15b_design.md`.
- **M4 leak check (the dispatch's ownership correction):** performed and NEGATIVE.
  `grep -n "M4\|N-A\|N-B\|N-C\|N-D\|§9\|--config" docs/experiments/U3_tier_t.md`
  returns nothing from the `N-*` family and no `§9`. The only `baseline_snapshot.sh`
  mentions (lines 445, 461) are §12 item 5's own verbatim text, which U3 owns. **No
  M4 material has leaked into U3.**

## VERDICT: **FAIL**

**2 BLOCKING, 5 MAJOR, 4 MINOR.**

Both BLOCKING findings are the WP's standing defect class — a repair landed in one
place with the claim resting on it left un-re-read in another (D-305) — reproduced
*inside the carve that exists to stop it*. One is B5 recurring at a site the carve
itself wrote; the other is a discharge of CLAUDE.md's instrument clause that names a
test D-312 deleted, and which green-passes as `running 0 tests`.

The unit's substance is sound. M1's selection is coherent, the threshold repair is
right, §6 and §10 are faithful verbatim carves, the §7 split is exact to the line,
and the pin note states the pin's strength honestly rather than overclaiming. The
failures are in the *completeness of what the unit says is open* and in two stale
references.

---

## SCOPE ITEM 1 — Tier-T only; nothing adopts the deferred quiet stage as settled

**What I did.** Read all 603 lines. Enumerated every mention of the deferred stage
(`grep -n "widen\|quiet_top_k\|Tier Q\|Tier-Q\|stage Q\|W-E\|quiet cut\|quiet
allowance" docs/experiments/U3_tier_t.md` → 27 hits). Read the census instrument to
see whether the matrix's cost evidence depends on the deferred stage. Read
`WPQ_seed.md`'s header and its ADR list to see what may be cited from it.

**Evidence.** U3 is Tier-T and the config shape. The header's "WHAT IS NOT HERE"
paragraph and U3-Z's D-scope bullet both flag the openness, and §7's lead-in
correctly assigns M2 and the widening schedule to `WPQ_seed.md`. But three
dependencies on the deferred stage stand inside the unit and are **not** in the OPEN
statement:

- §6.3's ADOPTED option C mitigates its own residual with *"left to Tier Q's delta
  ranking, which is a set of 23.2 cells/node against a quiet allowance of 16"* —
  Tier Q is deferred to WP-1.5c by D-315, and `16` is `quiet_top_k`, a key U3-Z says
  may not survive.
- §6.3's option B cost cell points the reader at *"whose BATCHED figure is the one
  `quiet_top_k` governs"*.
- MEASURED, `crates/pistol-solver/tests/wp15b_census.rs:64` and `:258–260`: the
  block's `option X — staged, BATCHED only` rows are computed as
  `t.len() + quiet.min(QUIET_TOP_K)` with `const QUIET_TOP_K: usize = 16`, the
  source comment reading *"The design's committed quiet cut"*. M1's own cost
  evidence therefore includes the deferred stage.

And §10 states the deferred schedule's semantics and validator rules as settled, on
the seed's authority. See findings 3 and 7.

**Result: FAIL** — content is Tier-T, but the adopted matrix option and its cost
evidence rely on the deferred stage and the OPEN statement does not say so.

---

## SCOPE ITEM 2 — the widening text's ABSENCE, verified mechanically

**What I did.** Retrieved the superseded document (`git show
6feb40a:docs/experiments/wp15b_design.md`, 1975 lines). §7 spans lines 905–1006
(102 lines), matching the owner table. Extracted every line of §7, trimmed it, and
tested each line longer than 25 characters for literal presence in U3:

```
n=905; while IFS= read -r line; do
  t="$(echo "$line" | sed 's/^ *//;s/ *$//')"
  [ ${#t} -gt 25 ] && grep -qF -- "$t" U3_tier_t.md && echo "HIT $n: $t"
  n=$((n+1)); done < rev7_s7.txt
```

**Output:** exactly 13 HITs, lines **959–971** contiguous, and nothing else. Line 972
(`generation experiment.`) is under the length filter; the block is **959–972 = 14
lines**, exactly the owner table's count. Those 14 lines are the two bullets the
owner table names.

Diffed the block byte-for-byte:

```
$ diff rev7_two_bullets.txt u3_two_bullets.txt
12c12
<   §5.3's licensed shortening of mate distances on lost positions both change what
---
>   **U2** §5.3's licensed shortening of mate distances on lost positions both change what
```

One difference, and it is a sanctioned cross-reference retarget.

I also diffed the whole of §10 (rev7 1347–1446) against U3's §10: the only changes
are the B5 lead-in repair, four cross-reference retargets, and the derivation's
rewording. No §7 widening material entered §10 by the back door.

**Result: PASS.** The split happened exactly as the owner table records. No widening
material beyond the two bullets is in U3, and §7's lead-in does not present deferred
text as adopted. (The *semantics* of the deferred schedule in §10 is a separate
matter — finding 7.)

---

## SCOPE ITEM 3 — B5, the config count stated exactly once

**What I did.** Enumerated every occurrence of `four|FOUR|fourth|Four` in U3 (16
hits) and every occurrence near `config|document|selectable|complete`. Counted §10's
own table rows. Grepped U1, U2, U4 and the seed.

**Evidence.**
- §10's table has **4** rows (`configs/instrument_staged_v0.toml`,
  `tactical_staged_v0.toml`, `gate_staged_v0.toml`, `play_staged_v0.toml`) — matches
  the stated FOUR.
- Line 253 states it: `**FOUR** complete documents`. This is the one authorised site.
- Line 265/272 keep the derivation (`The fourth document exists because three could
  not carry the requirement` … `is that fourth document`) — the owner table sanctions
  this as explanation, not restatement. Line 266/279 quote revision 6's and revision
  4's false claims as history. All correct.
- U3-Q cites without restating: *"Staged ships as the selectable documents §10 lists,
  and §10 is the one place their number is stated (B5)."* Correct.
- **Line 594, in U3-Z, restates it.** See BLOCKING 1.

**Result: FAIL.**

---

## SCOPE ITEM 4 — M1 (§6.3), its marks, and its selection

**What I did.** Diffed §6 against `6feb40a` (rev7 761–904) to confirm the carve is
verbatim; read every cell of the M1 table; checked the adopted option against §6.1,
§6.4, §6.5, §10's config block, §10's union reading, and U3-T's registered test row;
searched `docs/decisions.md` for an M1 ADR line; checked U4 §8.4 for mutation M7.

**Evidence.**

*Marks.* Inside the M1 table the only marked numeric claim is option C's cost:
`**MEASURED** 29 % of C's Tier T lies OUTSIDE the radius-2 ball (6.83 cells/node at
corpus roots)`. Option A's and B's cost cells carry no bare number — they cite the
census block by row name, which is the right discipline. **The one unmarked numeric
claim is `23.2` (and the `16` beside it) in option C's failure-mode cell** — MAJOR 12
of the revision-7 review, which U3-Z records OPEN with an honest reason (adding a
mark would decide a question the review says may be a mis-attribution; a repair is a
design act, not a carve act). Per the dispatch's rule, a correctly scoped OPEN is not
a defect. Its scoping is nonetheless incomplete — finding 3.

*Verbatim.* `diff` of §6: three cross-reference retargets (`**U2** §5.3`, `**U4**
§8`, `**U4** §8.4`) and the B7 repair paragraph. Every MEASURED/ESTIMATED mark is
unchanged. The header's claim that no number moved and none gained or lost a mark
holds.

*Selection.* ADOPTED = C at the threshold reading (own ≥2, opponent ≥3). It is
supported:
- §6.1's threshold repair is MEASURED (`+0.17` for B, `+0.04` for C) and makes
  `B ⊇ C`, so §6.5's pre-registered fallback to B is coherent.
- §6.4's count-3-leg lemma grounds the asymmetry, is marked a DERIVATION and not a
  measurement, and **names its own gap** (the `LAW-LEDGER` t=2 chain, whose refutation
  needs depth 4). §6.5 registers the consequence *and* the branch where a GREEN
  instrument with M7 surviving is recorded as "the instrument cannot tell A from C",
  not as confirmation. That is the right shape.
- MEASURED cross-check: U4 line 383 carries mutation M7 (*"Tier T qualifies at ≥3 for
  the mover (option A) … survival is a recorded finding under **U3** §6.5's second
  branch"*). The two units agree.

*Test row vs selection.* `tier_t_qualification_matches_adopted_matrix_option` names
the referent `us@{2,3} ∪ threat_cells(us) ∪ them@{3} ∪ threat_cells(them)`. That is
exactly own ≥2 / opponent ≥3 under §10's union reading, and exactly what §10's TOML
block commits (`tier_t_own_count = 2`, `tier_t_opponent_count = 3`, comment "THRESHOLD
reading: >= 2 for the mover, >= 3 for the opponent"), and exactly what the census
instrument implements (`OPTIONS[2] = { name: "C", own: 2, opponent: 3 }`,
`wp15b_census.rs:88–92`). **The row and the selection agree.**

*ADR line.* `grep -n -i "tier_t_own_count\|Tier-T option C\|TIER-T QUALIFICATION"
docs/decisions.md` → **no match**. U3-Z item 2 records it as owed, and U3-Z's lead-in
says *"items 2, 7 and 16 are this unit's own and have not landed"*. **The document
honestly says the ADR has not landed** — acceptable per the dispatch.

**Result: PASS on the marks, the selection and the test row; FAIL on completeness** —
the owed-list omits a fresh DECISION-RED-TEAM against M1 as amended (finding 6), and
the OPEN statement omits the adopted option's reliance on the deferred stage
(finding 3).

---

## SCOPE ITEM 5 — B7's residual, the census pin

**What I did.** Read `crates/pistol-solver/tests/wp15b_census.rs` (951 lines), ran it,
and independently scanned every carved document for rounded and derived restatements
of census cells.

**The shipped test's own output**, not a wrapper's status:

```
$ cargo test -p pistol-solver --test wp15b_census
     Running tests/wp15b_census.rs (target/debug/deps/wp15b_census-395e5ba3ce82a23a)
running 5 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.30s
```

The rebuild is real: `CARVE_DOCS` names six files, `carve_documents()` PANICS rather
than skipping on an unreadable or empty file, `the_census_pin_reads_every_carved_
document_it_names` plants `77.9583` per file and requires the scan to name that file,
and `the_pins_document_list_is_the_set_of_carved_documents_on_disk` compares the
constant against the `CARVE_MARKER` set on disk — a referent the constant does not
share. T4' is discharged.

**Is U3's note honest?** On the *pin's strength*, yes, and this is the unit's best
work: §6.2's "What the pin refuses, stated at the strength it actually has" states the
four-decimal limit explicitly, says it is narrower than the claim the superseded §6.2
made, and refers the surviving sites to U3-Z as OPEN. U3-M item 5's residual —
*"the pin covers the TABLE. Prose claims about the census … remain judged, and so does
`tools/baseline_snapshot.sh`"* — is verbatim consistent with D-304 as amended by
D-312 and does not narrow it. **No overclaim about coverage.**

**But the enumeration of what survives is wrong** (finding 4), and U3-M item 5's
mechanism sentence names a test that no longer exists (BLOCKING 2).

I also verified the instrument revision U3 registers: `tools/baseline_snapshot.sh` at
**`e889b5b`** — MEASURED correct, `git log -1 --format=%h -- tools/baseline_snapshot.sh`
→ `e889b5b`.

**Result: FAIL** — the pin note's strength claim is honest, but its residual site
list is incomplete and its mechanism names a deleted test.

---

# FINDINGS

## BLOCKING

### 1. B5 recurs — the config count is stated a second time, in U3-Z

**Claim (line 253, §10):** *"**FOUR** complete documents … **This is the one place the
count is stated; U2 §2.2 and U3-Q cite it and do not restate it** (B5 …)."*

**Contradicting text (line 594, U3-Z):**

> `- **The D-scope of \`quiet_top_k\` and \`widen_schedule\`.** §10's four documents each`
> `  commit both keys, and D-310 defers the stage they govern.`

**Reproducer.**
```
$ grep -no "[^.]\{0,70\}\b\(four\|FOUR\|fourth\|Four\)\b[^.]\{0,50\}" docs/experiments/U3_tier_t.md
253:**FOUR** complete documents, `deny_unknown_fields`, no c
265:**The fourth document exists because three could not carry the
272:is that fourth document
594:** §10's four documents each
```
Lines 265 and 272 are the derivation the owner table §10 explicitly sanctions
("it explains the count, it does not restate it"). Line 594 is neither the
authorised site nor a citation: it asserts a cardinality.

**Why it breaks.** `docs/experiments/section_owner_table.md` §10 rules: *"**The count
is FOUR**, and `docs/experiments/U3_tier_t.md` §10 is the only place in the carve that
says so."* §10's own lead-in enumerates the citing sites as "U2 §2.2 and U3-Q" —
U3-Z is not among them, and U3-Q does cite correctly. Falsifiability test, which is
what makes this the same defect and not bookkeeping: if §10 gained a fifth document,
line 594 would be false and nothing would catch it — which is exactly how §2.2's
"three" went stale when §10 gained a fourth (B5). The site is text the **carve
itself wrote** (it is about D-310, which postdates the superseded document), so this
is not inherited; it was introduced by the repair.

**Fix shape.** "§10's config documents each commit both keys".

---

### 2. U3-M item 5 discharges the instrument clause by naming a test D-312 deleted

**Claim (lines 452–455, §12 item 5):**

> *"The instrument clause asks that a change to the instrument reopen the review. A
> recorded SHA does that only if someone re-reads it … `the_design_document_carries_
> this_censuss_table_verbatim` does it mechanically instead: change the instrument and
> the build fails until the document is re-rendered."*

**Contradicting text.** D-312: *"D-304's gate `the_design_document_carries_this_
censuss_table_verbatim` — one hard-coded path, one file — **is renamed**
`the_carved_design_units_carry_this_censuss_table_verbatim`."* And U3's own §6.2,
line 134, names the new one correctly.

**Reproducer, and it is the load-bearing one:**
```
$ grep -n "the_design_document_carries\|the_carved_design_units_carry" crates/pistol-solver/tests/wp15b_census.rs
738:fn the_carved_design_units_carry_this_censuss_table_verbatim() {

$ cargo test -p pistol-solver --test wp15b_census the_design_document_carries_this_censuss_table_verbatim
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

**Why it breaks.** This paragraph is U3's *whole* answer to CLAUDE.md's instrument
clause — the argument for why a stale SHA is acceptable is that a test discharges it
mechanically. The named test does not exist at `1b645ac`, and the command that would
verify it exits 0 reporting `ok` while running nothing: `tools/SHELL_CHECKLIST.md`'s
EXIT-0-WRONG-ANSWER, in the exact form D-312 says the rename existed to prevent. The
sentence also keeps the pre-carve singular scope ("until **the document** is
re-rendered"), which is the one-path framing D-312 calls the defect.

This is the WP's recurring class at its purest: the repair landed in §6.2 (line 134,
correct new name, plus the "reads every carved unit" clause), and the claim resting on
it 320 lines later in §12 item 5 was not re-read. Both sections are U3's; both are in
one file; the restructure's stated ground was that such pairs are found by grepping
one file (D-310).

**Fix shape.** Rename the citation, and restate the mechanism at the carve's scope
(the six documents `CARVE_DOCS` names), mentioning the two companion gates D-312
landed.

---

## MAJOR

### 3. The OPEN statement of the D-scope omits M1's own dependence on the deferred stage

**Claim (U3-Z, lines 594–598):** *"§10's four documents each commit both keys, and
D-310 defers the stage they govern. Whether the shipped `Staged` surface keeps them …
**changes the config documents, the validator and the SPRT seat.** The carve does not
choose."*

**What it omits, MEASURED.**

1. **The adopted option's residual mitigation.** §6.3 option C: *"Residual: no cells
   blocking an opponent count-2 window; left to **Tier Q's delta ranking**, which is a
   set of 23.2 cells/node against a **quiet allowance of 16**."* D-315 schedules Tier Q
   into WP-1.5c. If the shipped surface "narrows to Tier F ∪ Tier T with no quiet tier
   at all" — U3-Z's own second branch — the ADOPTED option's stated mitigation does not
   exist in the shipped engine, and the cells C omits are generated by nothing. That
   changes the B-vs-C comparison, not just the config documents.
2. **Option B's cost cell** directs the reader to *"whose BATCHED figure is the one
   `quiet_top_k` governs"*.
3. **The matrix's cost evidence itself.** MEASURED, `crates/pistol-solver/tests/
   wp15b_census.rs`:
   ```
   64:  const QUIET_TOP_K: usize = 16;
   258:            // The batched rows. Tier T whole, then the quiet cut.
   259:            let quiet = pool.iter().filter(|cell| !t.contains(cell)).count();
   260:            t.len() + quiet.min(QUIET_TOP_K)
   ```
   with the source comment at :62 reading *"The design's committed quiet cut, so the
   staged-set column is the set the config actually produces"*. Every
   `option X — staged, BATCHED only` row and every `= N.NNx` multiplier in the pinned
   block is computed with the deferred stage in it.

**Why it breaks.** The dispatch's rule and CLAUDE.md's: an OPEN item is acceptable, an
*incomplete* one is not, because a reader takes the list as the boundary of the
question. Under D-310 the shipped engine's candidate count is not the `staged,
BATCHED only` figure, and the adopted option's risk cell points at a package outside
this WP.

**Mitigation, recorded because it errs toward the document:** the *ranking* survives.
U3-M item 5 establishes `A ⊆ C ⊆ B` as a set-inclusion identity under the threshold
reading, so the Tier-T rows order the options identically to the staged rows and the
selection of C over A and B does not turn on the quiet cut. What does not survive is
the magnitude of the reduction the matrix reports and C's stated mitigation.

**Fix shape.** Extend the OPEN bullet: name M1's cost column, the `staged, BATCHED
only` rows and `QUIET_TOP_K = 16` in the instrument, and option C's residual, as
things the D-scope decision also moves.

---

### 4. B7's residual list is incomplete — four further restatement sites inside U3 itself

**Claim (U3-Z, lines 578–581):** *"**B7's residual, four sites the pin cannot see.**
The pin refuses four-decimal restatements only. Rounded and percentage renderings
survive the carve at: **U2** §5.3's `70.8 %` (**U2**), §6.3's `6.83` and `23.2`, §10's
`6.83`, and §8.4/§8.5's `70.8 %` (**U4**)."* And §6.2, line 144: *"Rounded and
percentage renderings — `70.8 %`, `6.83`, `23.2` — are the shape prose actually uses."*

**Contradicting text — four further sites in U3, all outside the pinned block (lines
103–130), none named by either enumeration:**

| U3 line | Text | Census cell it restates |
|---|---|---|
| **101** | "inflated the ball **78.0 → 123.7** by the sampler rather than by depth" | `radius-2 ball` = **77.9583** (corpus roots) and **123.6615** (r8 draw) |
| **88** | "**MEASURED** cost of the repair: **+0.17** cells/node for B, **+0.04** for C" | `option B — Tier T` **46.5000 − 46.3333**; `option C — Tier T` **23.2917 − 23.2500** |
| **169, 350** | "**29 %** of C's Tier T lies OUTSIDE the radius-2 ball" | **6.8333 / 23.2917 = 29.34 %** |
| **426** | "does not extract Tier T on the **29.2 %** of them that take a forced row" | `BATCHED nodes` **70.8 %**; 100 − 70.8 = **29.2**, and `WIN-NOW` 4.2 + `FILTERED` 25.0 = **29.2** |

**Reproducer.**
```
$ python3 -c "print(round(77.9583,1), round(123.6615,1), round(46.5000-46.3333,4),
              round(23.2917-23.2500,4), round(6.8333/23.2917*100,2), 100-70.8)"
78.0 123.7 0.1667 0.0417 29.34 29.2
$ grep -n "78\.0\|123\.7" docs/experiments/U3_tier_t.md
101:which inflated the ball 78.0 → 123.7 by the sampler rather than by depth.
```
The pin is GREEN with all four standing (test output cited in scope item 5).

**Why it breaks.** The pin's *strength claim* is stated honestly — that is not the
finding. The finding is that U3-Z presents an enumeration as the OPEN set, and the
enumeration is short by at least four sites in this unit alone; line 101 sits **two
lines above** the `BEGIN CENSUS TABLE` marker, inside the very paragraph that repairs
B7. Each is drift-prone in exactly B7's way: if the `radius-2 ball` row or the
`option B — Tier T` rows move, lines 88 and 101 go stale silently. §6.2's own list of
"the shape prose actually uses" is three shapes; there are at least five.

**Note on U3-Z's arithmetic:** "four sites" is *correct* if a site is a distinct
document location among the ones it lists (U2 §5.3, U3 §6.3, U3 §10, U4 §8.4) — see
MINOR 9 on `§8.5`. The defect is the omission, not the count.

**Fix shape.** Either extend the list, or state the residual as a class ("any rounded
or derived rendering of a census cell") rather than as an enumeration — the class form
cannot go stale.

---

### 5. §6.4's load-bearing falsifier claims a measurement this unit does not carry

**Claim (§6.4, lines 175–177):** *"Revision 1's ground was 'a defence against the
opponent's two-turn win is what SEARCH DEPTH and the filter are for'. **That is
falsified by this document's own MEASURED `depth_at_500ms` = 2 / 2 / 1**: the
opponent's second turn is depth 4, and the engine reaches 2. **The sentence is
deleted.**"*

**Contradicting text.** After the carve U3 carries no such measurement:
```
$ grep -n "depth_at_500ms" docs/experiments/U3_tier_t.md
176:MEASURED `depth_at_500ms` = 2 / 2 / 1**: the opponent's second turn is depth 4,
```
One occurrence — the claim itself. The measurement lives in **U4** (lines 467, 484,
605, 608), whose §9 amendment 1 states it *"sits BELOW the record's own 'excluded from
every comparison' marker with a dead band of about 2×, so it is context and not the
registered quantity"* (U4:607–610), and D-310's own record calls it *"demoted to
below-marker CONTEXT, ADVISORY, and absent from the ROADMAP exit criterion"*.

**Why it breaks.** "This document's own" is false at u-rev 2, and it is the phrase
that licenses the reader not to look elsewhere. The carve's header rule retargets a
`§n` naming another unit's section; it did not catch a *self-attribution* the carve
made false. The claim is load-bearing: it is the whole ground for deleting revision
1's justification of the asymmetry, and §6.4's replacement lemma is what option C
rests on. A reviewer of U3 alone cannot check it and is told they need not.

Recorded in the document's favour: depth 2 vs depth 4 is not within a 2× dead band on
the depth *number*, so the argument itself is not obviously wrong — the defect is the
attribution, not the conclusion.

**Fix shape.** `**U4** §12 item 1's MEASURED depth_at_500ms = 2 / 2 / 1`, and note
that U4 treats it as below-marker context.

---

### 6. The owed-list omits a fresh DECISION-RED-TEAM against M1 as amended

**Claim (U3-A, lines 61–63):** *"**What this unit owes that no round has given it:** a
REVIEW-design of THIS text at THIS u-rev, and the census's registered replication and
second instrument (U3-Z, OPEN)."*

**Contradicting text, from U3-A's own table three lines above:**
- Row 1: *"DECISION-RED-TEAM, matrix M1 | revision 1, `ec8f7fb` | **M1 SURVIVES
  AMENDED.**"*
- Row 2: *"REVIEW-design | revisions 2–6 | all FAIL; **M1 was never reopened on its
  merits.** §6.3's cost column was re-derived twice for transmission defects, and
  §6.2 became the instrument's output rather than a typed table."*
- And §6.1: the ADOPTED reading itself flipped from **exact** to **threshold** after
  that attack, changing what the config commits and re-deriving every option row.

**Why it breaks.** CLAUDE.md: *"Reviews of superseded revisions do not transfer — an
amendment reopens the review, however small the diff"*, and *"a matrix never attacked
is the same breach as silent architecture drift"*. The matrix that was attacked at
`ec8f7fb` is not the matrix in §6.3 at u-rev 2, and the *selection* (C at the threshold
reading) postdates the attack. U3-Z item 2 is the ADR line owed for that selection and
CLAUDE.md requires it to record the strongest attack surviving against **the option as
adopted**; §6.5 records the attack against the option as it stood at revision 1.

The parallel case is handled explicitly elsewhere in the same carve, which is why this
reads as an omission rather than a scope decision: U4 line 68 (*"M4 SURVIVES AMENDED —
at a text three of whose cells the design has since MEASURED false"*) and U4 line 73
(*"at THIS u-rev, **two fresh DECISION-RED-TEAMs**, and a SHELL_CHECKLIST review"*),
and `WPQ_seed.md` for M2.

**Fix shape.** Add the fresh DECISION-RED-TEAM against M1-as-amended to U3-A's
owed-list and to U3-Z's OPEN section, gating item 2's ADR line on it.

---

### 7. §10 states the deferred schedule's semantics as settled, on the seed's authority

**Claim (§10, lines 281–298).** Five bullets and a validator rule, asserted as settled
design: *"The **first** batch is `quiet_top_k` quiet cells"*; *"The schedule's entries
are **cumulative counts of QUIET cells**"*; *"A pool **shorter** than the first
boundary never truncates, so the node is not counted in the widening schedule's
registered denominator (`WPQ_seed.md` §7.2). **Correct, and now stated.**"*; *"A pool
**longer** than the last boundary is cut there permanently. That is what a finite last
entry is FOR, and **it is the forward prune the deferred schedule's ADR line names**
(`WPQ_seed.md`, item 3)."*; and *"Cross-field validation … every entry must exceed
`quiet_top_k` … **a named refusal under rules 1 and 3**"*, restated in the validator
paragraph at lines 320–325.

**Contradicting text.** `docs/experiments/WPQ_seed.md`, its own header: *"**THIS IS NOT
A DESIGN AND IS NOT REVIEWABLE** … **Nothing here is selected, and nothing here may be
cited as adopted.** A reviewer asked to review this file should decline."* And its ADR
list, line 208: *"**Neither may be written while M2 is an open selection**; they are
recorded here as debt."* — of which item 3 is *"W-E, naming the non-PV cut as a forward
prune, the TT truncation rule, and the cut's binding under `Staged` only."* And D-315:
*"`WPQ_seed.md` as its input and **nothing in that seed inherited as settled**"*, with
W-E recorded as never having been attacked as an option among options.

**Reproducer.**
```
$ grep -n "WPQ_seed" docs/experiments/U3_tier_t.md
 23:§7 that is about it are EXCISED to `WPQ_seed.md` with stage Q, per D-310.
226:MATRIX M2 and the widening schedule are `WPQ_seed.md`'s.
260:...would make the SPRT measure nothing about the prune (rule 6, `WPQ_seed.md` §7.2)
290:  counted in the widening schedule's registered denominator (`WPQ_seed.md`
293:  (`WPQ_seed.md`, item 3).
310:# is what makes a widening schedule a rename of full width (WPQ_seed.md §7).
```
Lines 260, 290, 293 and 310 are normative citations INTO the non-reviewable file.
Line 293 uses the present tense for an ADR line the seed records as unwritable.

**Why it breaks.** The carve's own stated principle, from the owner table §3, is the
one being violated: *"Leaving them in the seed would put a claim a registered test
rests on inside a file that is not reviewable."* The two §7.2 bullets were moved into
U3 for exactly this reason; §10's four bullets rest the other way. A reviewer asked to
check "every entry must exceed `quiet_top_k`" has to reach into a document they are
instructed to decline to review, for an option (W-E) that D-315 records as unattacked.
U3-Z's OPEN item covers whether the *keys* survive; it does not say their *semantics*
are unattacked deferred content.

**Fix shape.** Either mark §10's widening bullets as recording the seed's unselected
text rather than stating settled semantics, or move the semantics they depend on into
U3 where they can be attacked. Line 293's present tense should read "the ADR line the
seed records as owed".

---

## MINOR

### 8. §6.2 undercounts the finding it repairs, citing that finding

**Claim (line 141–143):** *"**That is narrower than 'no section restates a number from
it' — and the superseded §6.2 made the wider claim while the document contained **four
counter-examples** to it** (revision-7 review B7)."*

**Contradicting text.** B7's own heading: *"Contradicting text — **six sites** outside
the pinned block"*, and D-309 repeats *"six census figures stand restated outside the
block it pins"*.

**Reproducer**, MEASURED against the superseded document (block at 797–824):
```
$ grep -n "70\.8\|6\.83\|23\.2" rev7.md   # outside 797..824
139:  ... **70.8 %** of corpus roots ...
584:  70.8 % of corpus roots**, the common case rather than a corner ...
853:  ... (6.83 cells/node at corpus roots) ... a set of 23.2 cells/node ...
1260: ... on the 70.8 % BATCHED population `forced == 0` ...
1442: (6.83 cells/node at corpus roots), so the seats also differ ...
```
Six restatements at five distinct lines. Not four, on any reading.

(U3-Z's *"four sites the pin cannot see"* is a different and correct number: after
§0's line 139 is dropped, four distinct locations survive.)

---

### 9. U3-Z names a restatement site that does not exist

**Claim (line 581):** *"and §8.4/**§8.5**'s `70.8 %` (**U4**)"*.

**Reproducer.**
```
$ grep -n "70\.8" docs/experiments/U4_soundness_instrument.md
383:| **M8** | ... on the 70.8 % BATCHED population `forced == 0` ...
$ awk '/^### 8\.5/,/^### 8\.6/' docs/experiments/U4_soundness_instrument.md | grep -c "70\.8"
0
```
U4's only occurrence is line 383, inside §8.4 (heading at U4:366). §8.5 carries none.
Harmless in direction (it over-lists rather than under-lists), but it is a claim about
another unit's text that is false, which the dispatch names as a finding class.

---

### 10. A citation into DROPPED text with no retrieval pointer

**Claim (line 342):** *"…two different sets in one document, which is **§0 row 4's**
class recurring in the opposite direction."*

§0 is DROPPED by the carve (owner table §2), owned by no unit, and absent from the
tree; it is retrievable only at `6feb40a`. The header's resolution rule covers a `§n`
"that names a section this unit does not own" by prefixing the owning unit — there is
no owning unit here, so the rule does not reach it. U3 handles the analogous cases
correctly elsewhere by saying so in words: *"the superseded §11"* (line 361), *"the
superseded §15"* (line 523), *"the superseded §17's own list"* (line 587). Line 342
does not.

---

### 11. §6.2's description of the pin understates its coverage by one file and two gates

**Claim (lines 134–137):** *"pinned by `the_carved_design_units_carry_this_censuss_
table_verbatim` … which reads **every carved unit and the seed** by an enumerated path
list."*

`CARVE_DOCS` (wp15b_census.rs:645–652) holds **six** entries — the four units, the
seed, **and `section_owner_table.md`**. §6.2 also does not mention the two companion
gates D-312 landed (`the_census_pin_reads_every_carved_document_it_names`,
`the_pins_document_list_is_the_set_of_carved_documents_on_disk`), the second of which
is what answers "the list is not self-certifying". Understating coverage is the safe
direction and is not an overclaim, but a reader reconciling U3 against D-312 will find
the description short.

---

## Rejected, with the attempted reproducer

Recorded so they are not re-found.

- **"M4 material has leaked into U3."** REJECTED. `grep -n "M4\|N-A\|N-B\|N-C\|N-D\|
  §9\|--config\|SHELL_CHECKLIST" docs/experiments/U3_tier_t.md` returns five hits, all
  of them §10's and §12 item 5's own verbatim text (the snapshot's AFTER as a config
  role; `tools/baseline_snapshot.sh` at `e889b5b` as §12 item 5's registered
  companion instrument; the SHELL_CHECKLIST sentence inside §15 item 23, which the
  owner table assigns to U3). No `N-*` option, no §9, no matrix.
- **"The census figures in U3's table have drifted."** REJECTED. `cargo test -p
  pistol-solver --test wp15b_census` →
  `the_carved_design_units_carry_this_censuss_table_verbatim ... ok`, which compares
  the block byte-for-byte against the instrument's render.
- **"`e889b5b` is a stale instrument SHA."** REJECTED.
  `git log -1 --format=%h -- tools/baseline_snapshot.sh` → `e889b5b`. Current.
- **"The test row disagrees with the adopted option."** REJECTED. U3-T's referent
  `us@{2,3} ∪ threat_cells(us) ∪ them@{3} ∪ threat_cells(them)`, §10's union block,
  §10's TOML (`tier_t_own_count = 2` / `tier_t_opponent_count = 3`) and the census's
  `TierOption { name: "C", own: 2, opponent: 3 }` all name the same set.
- **"§10's `1024` and `128` allowances are unsupported."** REJECTED, as at revision 7:
  `6 × 17 = 102` and `18 × 17 = 306` are correct radius-1/radius-2 hex union bounds at
  17 stones, and the census's `radius-2 ball` playouts mean (376.4708) is a denser
  population.
- **"The carve is not verbatim / a mark moved."** REJECTED. `diff` of §6 (rev7
  761–904) and §10 (rev7 1347–1446) against U3 shows only the two named repairs (B5's
  lead-in, B7's strength paragraph) and cross-reference retargets. Every MEASURED and
  ESTIMATED mark is unchanged; no numeric value differs.
- **"U3 exceeds a reviewable size."** NO FINDING. 603 lines. Recorded by the owner
  table §11 as the architect's question, and out of this review's scope by the
  dispatch.
- **"MAJOR 12 (the unmarked `23.2`) is unrepaired and should fail the unit."** NO
  FINDING as such. It is recorded OPEN in U3-Z with a reason that survives scrutiny —
  adding a mark would decide whether the figure is measured, and the review's finding
  is that it may be mis-attributed. Its *scoping* is incomplete, which is finding 3.

---

## Observations for the architect — not findings against U3

Reported because scope item 3 asked me to check the other units, and because these
are cross-unit facts a U3-only fix would not reach.

- **B5's class survives in U4.** U4 restates the config count three times:
  line 39–40 *"corrected to the **four-config** reality"*, line 743 *"stands on §10's
  **four** configs"*, line 753 *"the **FOURTH** config, `tactical_staged_v0.toml`"*.
  Under the owner table §10's ruling that U3 §10 is the only place the count is
  stated, these are the same defect as BLOCKING 1. U4's own reviewer may or may not
  have been asked to check it.
- **The owner table §11's measured unit sizes are stale.** It records U4 at **701**
  lines; MEASURED at `1b645ac`, `wc -l docs/experiments/U4_soundness_instrument.md` →
  **800**. `git show cf74594:…U4_soundness_instrument.md | wc -l` → 701, so the B3
  repair at `9421d19` added 99 lines and the table (u-rev 1) was not bumped with them.
  D-311 binds the table too. The size question the table hands the architect is
  therefore posed on a number that has moved.
- **Tier Q is still in the shipped node protocol.** U2 lines 336–337 route BATCHED
  rows to *"Tier T ∪ Tier Q, batched per the deferred schedule (`WPQ_seed.md`)"* and
  U2 line 440 defines Tier Q. The D-scope question U3-Z leaves open is therefore open
  across at least U2, U3 and the census instrument, and no unit owns closing it.

---

*REVIEW-design of `docs/experiments/U3_tier_t.md` u-rev 2, at `1b645ac`. Fresh
context. Every finding reproduced before reporting; every numeric claim above marked
MEASURED where it was measured. This report is not committed and modifies no other
file.*
