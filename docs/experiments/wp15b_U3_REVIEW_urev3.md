# REVIEW-design (re-review) — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md` u-rev 3

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session dispatched the repair; it did not review it.
Per the session's standing rule a re-review FAIL is COLLECTED AND REPORTED, not
looped on in-session: MAJOR 1, MAJOR 2 and MINOR 3-5 below are UNREPAIRED and
are the architect's.
-->

## Header

- **Pinned revision reviewed:** `7d5d39c`.
- **Matches HEAD:** **NO — HEAD advanced during the review, but the document under review did not move.** MEASURED: `git rev-parse HEAD` → `0af32fb` (it was `d48824f` when I started). `git diff 7d5d39c 0af32fb --stat` → 4 files, all additions: `docs/decisions.md` (+6), `matrix_M3_soundness_instrument_rev2.md`, `matrix_M4_snapshot_config_seam_rev3.md`, `wp15b_U2_REVIEW_urev2.md`. MEASURED: `git diff 7d5d39c 0af32fb -- docs/experiments/U3_tier_t.md | wc -l` → **0**. `docs/experiments/U3_tier_t.md` is byte-identical at `7d5d39c` and at HEAD.
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 3**, 818 lines (MEASURED, `wc -l`).
- **Reviewer context:** FRESH. I did not write this unit or its repair and had not seen either. Read for background but not reviewed: `wp15b_U3_REVIEW.md` (in full), `wp15b_U4_REVIEW.md`, `wp15b_design_rev7_REVIEW.md`, `U2_node_protocol.md`, `U4_soundness_instrument.md`, `section_owner_table.md`, `crates/pistol-solver/tests/wp15b_census.rs` (in full), D-304/D-310/D-312/D-315, and `git show 6feb40a:docs/experiments/wp15b_design.md`.
- **Scope:** the seven items of the dispatch. Cross-unit facts (U4's config counts, the owner table's stale sizes, Tier Q across units) are noted at the end and are not findings against U3.
- **Tree left clean:** MEASURED, `git status --porcelain` → empty. No file edited, no git write command run, no worktree created.

## VERDICT: **FAIL**

**0 BLOCKING, 2 MAJOR, 3 MINOR.**

Both BLOCKING findings of the prior round are genuinely repaired, and I verified BLOCKING 2 against the shipped test bodies rather than the prose — the three gates exist, they run, and U3's description of each is true of what it does, including the delicate part: U3 attaches the independence claim only to the companion gate that actually has an independent referent. Nine of the eleven findings are repaired to their stated fix shape, and MINOR 8's re-measurement reproduces exactly.

The unit fails on two numeric-provenance defects in §6 — the section that feeds U3-Z item 2's ADR line. One is a derivation the repair newly certifies **MEASURED** at u-rev 3 which does not reproduce, on the single cell the document itself records as possibly mis-attributed. The other is the fold-in law's own signature: the sampler-sensitivity figures were withdrawn and the repair landed at §12 item 5, while §6.5's copy — the copy destined for the ADR line — still carries them, contradicting the rule §12 item 5 states.

---

# FINDINGS

## MAJOR

### 1. U3-Z certifies **MEASURED** that "the derivations reproduce"; the `23.2` row does not, and two other block cells do

**Claim reviewed** (U3-Z, line 742, **new at u-rev 3**):

> The derivations reproduce — **MEASURED** at u-rev 3 from the block's own cells, and the values are read from the block rather than restated here because that is what the pin exists to enforce.

covering the eight-row table at lines 733–740, whose §6.3 row (line 735) attributes `23.2` to `option C — Tier T (threshold, ADOPTED)`, and restated at line 712 (U3-Z's MAJOR-12 bullet):

> `23.2` is the rounding of the block's own `option C — Tier T (threshold, ADOPTED)` row at corpus roots

**Contradicting evidence.** MEASURED:

```
$ python3 -c "
print('round(23.2917,1)=', round(23.2917,1))
print('round(23.2917,2)=', round(23.2917,2))
print('trunc 1dp 23.2917 =', int(23.2917*10)/10)
print('round(23.2500,1)=', round(23.2500,1))"
round(23.2917,1)= 23.3
round(23.2917,2)= 23.29
trunc 1dp 23.2917 = 23.2
round(23.2500,1)= 23.2
```

`23.2917` has no rounding equal to `23.2`. Two other cells of the same pinned block do produce `23.2` exactly (U3 lines 164, 161):

- `option C — Tier T (exact, NOT adopted)` at corpus roots = **23.2500**, which rounds to 23.2;
- `option A — staged, BATCHED only` at the r2-draw column = **23.20**, literally.

I re-ran the other seven rows and **all seven reproduce**: `6.83` ← 6.8333; `78.0 → 123.7` ← 77.9583 / 123.6615; `+0.17` ← 46.5000 − 46.3333 = 0.1667; `+0.04` ← 23.2917 − 23.2500 = 0.0417; `29 %` ← 6.8333/23.2917 = 29.34 %; `29.2 %` ← 100 − 70.8, and 4.2 + 25.0. Exactly one row fails, and it is the one the document flags as open.

**Why it breaks.** The claim is an **attribution** claim, newly stamped MEASURED by this repair, and attribution is the thing CLAUDE.md's dry-run clause singles out as what a synthetic check cannot catch. It is not bookkeeping: revision-7 MAJOR 12 is open precisely because `23.2` "may be mis-attributed", and the arithmetic says the nearest candidate is `option C — Tier T (exact, NOT adopted)` — the reading §6.1 rejected. If that is the provenance, the ADOPTED option's residual-risk figure in §6.3 was computed under the reading the matrix threw out and was never re-derived when §6.1 flipped exact → threshold, which is *the option committed was not the option measured* recurring inside the failure-mode cell. The u-rev-3 text asserts that question closed with a MEASURED mark instead of leaving it open, which is the opposite of what the OPEN bullet three lines above says it is doing. Recorded in the document's favour: under every candidate the figure is a Tier-T quantity, so the bullet's separate conclusion — "which is Tier T and not Tier Q" — survives.

**Fix scope.** Either drop the "the derivations reproduce — MEASURED" sentence's coverage of that row and say the `23.2` provenance is one of three candidates (23.2917 truncated, 23.2500 rounded, 23.20 exact), or state the derivation as a truncation and name the competing cell. Both are U3-local; deciding *which* cell it is remains the design act the bullet correctly refuses.

---

### 2. §6.5's ADR-bound attack still carries the withdrawn `3.1× → 2.4×`, against the rule §12 item 5 states in the same file

**Claim reviewed** (§6.5, line 286, verbatim carve, unrepaired at u-rev 3):

> *…and the reduction it is bought with shrinks from 3.1× to 2.4× the moment the depth stand-in is re-sampled from the radius-2 ball the search actually uses, a one-second run the document did not take.*

**Contradicting evidence.**

1. The instrument's counterpart quantity is in the pinned block. MEASURED, U3 lines 169 and 165: `option C — staged, BATCHED only` = `45.82 = 2.78x` (r8 draw) → `47.34 = 2.09x` (r2 draw). Under the Tier-T-only reading it is 123.6615/30.2622 = 4.09 → 94.4965/31.4965 = 3.00. Neither pair is `3.1× → 2.4×`. MEASURED, the multiplier is `ball / staged` over BATCHED-only rows (`crates/pistol-solver/tests/wp15b_census.rs:505–523`), so it is not recomputable from the block's own `radius-2 ball` row and a reader cannot reconcile the two.
2. The superseded document recorded these figures as **withdrawn**. MEASURED, `git show 6feb40a:docs/experiments/wp15b_design.md`, line 129 (§0 row 34):

```
| 34 | §12.5's sampler sensitivity, `3.1× → 2.4×`, marked MEASURED | The figures §6.2
had withdrawn two sections earlier, kept in the section whose job is to name the
instrument | **MAJOR** |
```

3. U3 carries the repair for that row — at §12 item 5 only. Lines 624–627:

> both regimes' figures are in the census block and the sensitivity is read from it **rather than restated here**, which is the rule the block exists to enforce.

MEASURED, `grep -n "3\.1×\|2\.4×" docs/experiments/*.md` → one hit, `U3_tier_t.md:286`. The §12.5 copy was repaired; the §6.5 copy, 338 lines earlier in the same file, was not re-read.

**Why it breaks.** This is D-305's class at the same distance and in the same file as BLOCKING 2 was — a repair landed in one section with the copy resting on it left standing in another — and it survived the u-rev that exists to answer that class. It is not inert history: U3-Z item 2 says the owed ADR line is written "with §6.5's surviving attack", and CLAUDE.md requires that line to record the strongest attack surviving against the option *as adopted*. As it stands the ADR line would carry two unmarked figures the committed instrument contradicts, permanently. The pair also carries neither **MEASURED** nor **ESTIMATED**, which is exactly the D-291 shape U3-Z keeps open for `23.2`, and U3-Z's OPEN list does not name it.

**Fix scope.** U3-local, and it is not a design act: mark the pair as revision 1's figures (as §0 row 34 records them withdrawn), or cite the block's `option C — staged, BATCHED only` row the way every other §6 site now does. Abridging a quote is already licensed by the sentence's own "(abridged for the ADR line)".

---

## MINOR

### 3. B7's class covers §10's "under 400"; the list omits it

**Claim reviewed** (U3-Z, line 721): "here is every site of it known at this u-rev", the class being *any rounded, percentage or otherwise derived rendering of a census cell*.

**Contradicting text** (§10, lines 344–347):

> The `1024` is not a guess: the radius-2 ball is MEASURED in the census block's own `radius-2 ball` row, whose largest regime mean is **under 400**, and a bounded ball at 17 stones cannot exceed `6 × 17 = 102` at radius 1 or `18 × 17 = 306` at radius 2.

**Reproducer.** MEASURED: `grep -n "400" docs/experiments/U3_tier_t.md` → line 346, and the cell it derives from is `radius-2 ball`, playouts = **376.4708** (U3 line 153); `python3 -c "print(376.4708<400)"` → `True`. The site is absent from the U3-Z table (lines 733–740) and from §6.2's four-site sentence (lines 202–205).

**Why it breaks.** It is a derived claim about a block cell that the four-decimal scan cannot see and that goes stale silently if the playouts regime is re-sampled — B7's hazard exactly. I derived this independently by rounding every block value to 0/1/2 dp and scanning the document outside the block; that scan found no site the list misses, so this is the one case, and it is at the class boundary (a one-sided bound rather than a restated value) with an independent second leg (`6 × 17` / `18 × 17`) carrying the same argument. That is why it is MINOR and not the MAJOR the prior round rated the enumeration.

**Fix scope.** One row in the U3-Z table, or drop "under 400" and let the combinatorial bound carry the sentence.

---

### 4. The inherited MAJOR-9 disposition truncates the U4 reviewer's sentence in the direction that changes its scope

**Claim reviewed** (head, REVIEW STATUS table, line 74):

> `wp15b_U4_REVIEW.md` records that "its non-discharge is an IMPL gate and is not reported as a design defect"

**Contradicting text.** MEASURED, `grep -n "IMPL gate" docs/experiments/wp15b_U4_REVIEW.md` → line 364; the full sentence (lines 360–364) reads:

> **MAJOR 9 (rule 5 / D-263 at §12 item 4, **U3's**).** … **No finding, and *per the brief* its non-discharge is an IMPL gate and is not reported as a design defect *anywhere in this report*.**

**Why it breaks.** Two clauses are dropped, and both bear on the disposition U3 rests on them. "Per the brief" makes the statement a consequence of that reviewer's dispatch, not a project ruling; "anywhere in this report" scopes it to U4. And the same sentence labels the item **U3's**. So U3 cites, as its authority for not discharging the item, a report that assigns the item to U3 and disclaims it only for its own scope. The disposition itself is defensible on other grounds — U3-M item 4 declares the hotspot substitution loudly under D-263's flip clause, and U2 does carry the item OPEN (MEASURED, `U2_node_protocol.md:783`: "**RULE 5 IS UNDISCHARGED FOR THE NODE PROTOCOL ITSELF** (revision-7 review MAJOR 9…)") — so this is the citation, not the call. U3-Z's OPEN section carries no corresponding entry, which is why the head row is doing all the work.

**Fix scope.** Quote in full and attribute the disposition to U2's OPEN item plus U3-M item 4's declared substitution.

---

### 5. New text introduced at u-rev 3 cites other units without their u-rev, against this unit's own LABEL DISCIPLINE rule

**Claim reviewed** (head, line 48–49, D-311 travelling item T5): "A citation of another unit names the unit AND the u-rev cited."

**Contradicting text — sites written by this repair.** MEASURED, `grep -no "\*\*U[124]\*\*[^.]\{0,25\}" docs/experiments/U3_tier_t.md` cross-referenced against `git diff 1b645ac 7d5d39c -- docs/experiments/U3_tier_t.md`:

| line | new text | carries u-rev? |
|---|---|---|
| 236 | `**U4** (u-rev 5) §12 item 1` | yes |
| **244** | `It is **U4**'s, and **U4** §9 amendment 1 records…` | **no** |
| 733 / 734 | `**U2** (u-rev 1) §5.3` / `**U4** (u-rev 5) §8.4` | yes |
| **749** | `u-rev 2 also named a 70.8 % in **U4** §8.5` | **no** |
| **763** | `the same defect as **U2** §2.2's "three" going stale` | **no** |
| 808 | `**U4** (u-rev 5) states the parallel duty` | yes |

**Why it breaks.** Lines 244 and 236 are the *same paragraph*: the MAJOR-5 repair applies the rule in one sentence and drops it in the next, so the reader cannot tell whether the omission means anything. D-311 exists because the label ambiguity across `d94dc0a`/`6feb40a` cost a review round; §9 amendment 1 is precisely the kind of citation that moves. (Nine further un-u-rev'd citations at lines 218, 274, 277, 314, 333, 338, 372, 491, 520 are inherited from u-rev 2 and were not flagged in the prior round; they are noted, not charged to this repair.)

**Fix scope.** Three insertions.

---

# Verified with no finding

Each of these was checked against the artefact, not against the document's prose.

- **BLOCKING 2 — fully discharged, and the vacuity trap the dispatch named is not present.** MEASURED, the dispatch's own two commands, run by me at `d48824f`:
  ```
  $ grep -n "fn the_carved_design_units_carry\|fn the_census_pin_reads_every\|fn the_pins_document_list" crates/pistol-solver/tests/wp15b_census.rs
  738:fn the_carved_design_units_carry_this_censuss_table_verbatim() {
  799:fn the_census_pin_reads_every_carved_document_it_names() {
  851:fn the_pins_document_list_is_the_set_of_carved_documents_on_disk() {

  $ cargo test -p pistol-solver --test wp15b_census
  running 5 tests
  test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
  test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
  test wp15b_census_reproduces_the_registered_populations ... ok
  test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
  test the_census_pin_reads_every_carved_document_it_names ... ok
  test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.22s
  ```
  Byte-identical to the block pasted at U3 lines 578–592 (which omits only the `Running …` and `finished in` lines). I then read all three bodies rather than the names. Every clause of U3's description is true of the code: `CARVE_DOCS` (`:645–652`) holds six paths; `carve_documents()` PANICS rather than skipping on unreadable **and** asserts non-empty; the main gate counts `BEGIN`/`END` across the whole set and asserts exactly one of each (`:748–762`); `restatements_outside` matches only fields with exactly four decimals (`:769–787`); `the_census_pin_reads_every_carved_document_it_names` runs an unplanted **control** first, asserts the planted string is still a rendered figure, then plants `77.9583` per file and requires the scan to name that file; `the_pins_document_list_…` reads the directory, filters on `CARVE_MARKER`, refuses a set smaller than 3 so two empty sets cannot certify each other, and compares against the constant.
- **The independence claim is correctly scoped — no vacuous companion.** `the_census_pin_reads_every_carved_document_it_names` *does* iterate `CARVE_DOCS` and therefore shares the suspect input; the source says so itself (`:840–847`, "checks the list against itself: shrink the list and the loop shrinks with it. MEASURED in a worktree … both of the tests above PASSED while blind to five carved documents"). **U3 never claims otherwise.** At both sites — §6.2 lines 180–187 and U3-M item 5 lines 554–560 — the phrase "a referent the constant/list does not share" is attached only to `the_pins_document_list_is_the_set_of_carved_documents_on_disk`, which is the gate that genuinely has one. That is the CLAUDE.md-conformant reading and the finding the dispatch anticipated is not there.
- **BLOCKING 1 — repaired, and no third recurrence.** MEASURED, `grep -n "\bfour\b\|\bFOUR\b\|\bfourth\b\|\bFour\b" docs/experiments/U3_tier_t.md` → 17 hits. Line 325 (`**FOUR** complete documents`) is the one authorised site and matches the table's four rows. Lines 337/344 are the derivation the owner table sanctions. Lines 338/339/351 quote revisions 6 and 4 as history. Line 646 and line 760 both cite §10 without a cardinality. Lines 36, 65, 178, 189–209, 546–553, 714–746 are about the carve's six documents, four-decimal figures, four revisions, or the B7 site count — not the config count. No site states, derives or implies it a second time.
- **MAJOR 3 — verified against the instrument, not the prose.** MEASURED, `crates/pistol-solver/tests/wp15b_census.rs:62–64` holds `const QUIET_TOP_K: usize = 16` with the quoted comment; `:257–261` computes the batched rows as `t.len() + quiet.min(QUIET_TOP_K)`; `:505–523` computes each `= N.NNx` as `ball / staged` over BATCHED-only rows. U3's "every staged figure and every `= N.NNx` multiplier … has the deferred stage inside it" is exactly right, including the multipliers. §6.3's option B cell (line 229) and option C residual (line 230) are quoted correctly.
- **MAJOR 5 — the retarget resolves and U4's treatment is stated correctly.** U4 is at u-rev 5 (`U4_soundness_instrument.md:15`); U4-M item 1 (`:658–666`) carries `depth_at_500ms` 2/2/1 as context with its dead band; U4 §9.1 amendment 1 (`:569–590`) carries the 32-lines-below-marker finding and the 1.96×/2.68×/18.7× resolution ladder; D-310 (`docs/decisions.md:665`) records "`depth_at_500ms` is demoted to below-marker CONTEXT, ADVISORY, and absent from the ROADMAP exit criterion". The item-numbering convention holds (U4-M's item 1 = superseded §12 item 1, as U3-M's items 4 and 5 are).
- **MAJOR 6 — present in both required places.** U3-A's owed-list bullet 2 (lines 96–103), U3-Z's OPEN bullet (lines 800–810), and U3-Z item 2 GATED (lines 669–675).
- **MAJOR 7 — repaired to the fix shape, and it did *not* move deferred-stage semantics into U3.** The lead-in (lines 357–369) marks the bullets as RECORDING; line 380 replaces the present tense with "the ADR line the seed records as OWED … which that file records as unwritable while M2 is an open selection". Its site list is complete: MEASURED, `grep -n "WPQ_seed" docs/experiments/U3_tier_t.md` returns exactly four normative reaches into the seed (332, 376, 380, 398) and all four are named at lines 363–365. Dispatch item 7 checked and clean — no design decision is made under cover of any repair; every added paragraph traces to a finding's stated fix shape or to text the prior reviewer itself recorded.
- **MINOR 8 — re-measured independently at `6feb40a`.** The block is at 797–824 (verified: BEGIN at 797, END at 824). `grep -n "70\.8\|6\.83\|23\.2"` outside it returns lines **139, 584, 853, 1260, 1442**, with line 853 carrying two (`6.83` and `23.2`) — **six restatements at five distinct lines**, exactly as U3 line 193–198 states.
- **MINOR 9 — verified.** MEASURED, `grep -n "70\.8" docs/experiments/U4_soundness_instrument.md` → one line, **412**; §8.4 spans 395–413 and §8.5 begins at 414. Inside §8.4, as claimed.
- **MINOR 10 / MINOR 11 — verified.** Line 434–437 says "the superseded §0" with `6feb40a` as the retrieval point; §6.2 names all six `CARVE_DOCS` entries and both companion gates.
- **revision-7 MAJOR 12's disposition quotes the prior report accurately**, ellipsis included (`wp15b_U3_REVIEW.md:652–655`).
- **`tools/baseline_snapshot.sh` at `e889b5b` is still current.** MEASURED, `git log -1 --format=%h -- tools/baseline_snapshot.sh` → `e889b5b`.
- **The pin is GREEN over the u-rev-3 text**, so the census block in this document is byte-identical to the instrument's render.

# Rejected, with the attempted reproducer

- **"A companion gate shares the suspect input, so U3's independence claim is vacuous."** REJECTED — see above. Both companion descriptions were read against the bodies; U3 attributes independence only to `the_pins_document_list_…`, which compares `CARVE_DOCS` to the on-disk `CARVE_MARKER` set.
- **"The `cargo test` block pasted at U3:583–592 is stale or invented."** REJECTED. Reproduced verbatim at HEAD.
- **"U3-Z's B7 list omits sites in the other carved documents."** REJECTED. `for f in U1… U2… U4… WPQ_seed.md section_owner_table.md; do grep -n "70\.8\|6\.83\|23\.2\|78\.0\|123\.7\|29 %\|29\.2\|376" $f; done` returns `U2:370` and `U4:412` — both listed — plus `section_owner_table.md:72`, which names rev-7's dropped line 139 as a *historical* site frozen at `6feb40a` and cannot go stale.
- **"There are further rounded restatements in U3 the list misses."** REJECTED beyond finding 3. I rounded all 105 distinct block values to 0/1/2 dp, generated every form, and scanned every line outside the block; and separately grepped all twelve `= N.NNx` multipliers and all twelve staged means. Every genuine hit is already listed; the rest are section numbers (§7.2, §6.1, §2.2) and unrelated bench figures.
- **"`about 2×` (line 246) is a new unmarked number."** REJECTED as a finding. It is a verbatim echo of U4-M's own sentence ("with a dead band of about 2×"), whose underlying ladder is MEASURED in U4 §9.1; U3 adds no number of its own there.
- **"The head's mark claim is false, because u-rev 3 adds new MEASURED marks."** REJECTED. The claim was rescoped by this repair to "no number **carried from the superseded text** moved, and none gained or lost a mark" (line 41–42), which is true; the new marks are all on numbers the repair itself measured.
- **"§10's fourth widening bullet still asserts settled design (`Correct, and now stated.`, line 377)."** REJECTED as a finding, recorded because it is close. The phrase remains, but the MAJOR-7 lead-in 19 lines above quotes it by name and states that at u-rev 3 it is "scoped, not withdrawn — the carve does not get to decide them either way", and explains why withdrawing it would itself be a design act. That is a disclosed choice, not an un-re-read one.
- **"M4 material has leaked into U3."** REJECTED, as in the prior round. `grep -n "M4\|N-A\|N-B\|N-C\|N-D\|§9" docs/experiments/U3_tier_t.md` returns nothing from the `N-*` family; the only `§9` is `**U4** §9 amendment 1`.

# Observations for the architect — not findings against U3

- **D-312's own text is looser than the code, and looser than U3.** `docs/decisions.md:669` says the renamed gate "now resolves its documents through the `CARVE_MARKER` line each carved file carries". It does not: `the_carved_design_units_carry_this_censuss_table_verbatim` resolves through the `CARVE_DOCS` constant, and it is the *third* gate that compares that constant to the marker set. U3 §6.2 and U3-M item 5 both describe the code correctly, so this is D-312's sentence to amend, not U3's.
- The cross-unit items the dispatch reserved are unchanged as of `7d5d39c`: U4's config-count restatements, the owner table's stale MEASURED sizes, and Tier Q's presence in the shipped node protocol (the last of which R5, landed at `0af32fb` after the pinned revision, appears to address).

---

*REVIEW-design of `docs/experiments/U3_tier_t.md` u-rev 3, at `7d5d39c`; HEAD advanced to `0af32fb` during the review and the document did not move. Fresh context. Every finding reproduced before reporting; every numeric claim marked MEASURED with its command. No file edited, no git write command run, `git status --porcelain` empty.*
