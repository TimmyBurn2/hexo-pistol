# WP-1.5b U3 — Tier-T qualification and the config shape: DESIGN UNIT

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 1.** Carved from `docs/experiments/wp15b_design.md` §6, §10 and §12 items
4 and 5 at `6feb40a` (revision 7, never reviewed, CLOSED by D-309), plus the two
bullets of §7.2 that are not widening text, under the restructure selected as
option D by D-310. The carve's section-to-owner map is
`docs/experiments/section_owner_table.md`. The superseded document is not
in the tree: it is retrievable at `6feb40a` and nowhere else.

**WHAT IS NOT HERE.** MATRIX M2 — the widening schedule — and every sentence of
§7 that is about it are EXCISED to `WPQ_seed.md` with stage Q, per D-310. This
unit is Tier T only. `quiet_top_k` and `widen_schedule` still appear in §10's
config documents because a `deny_unknown_fields` document is complete or it is
nothing; **whether the D-scope shipped surface keeps those two keys at all is
OPEN and is the architect's, not the carve's** — see U3-Z.

**THE TEXT IS A VERBATIM CARVE** apart from cross-reference retargets and two
named repairs, each stated where it occurs: **B5** (§10's lead-in — the config
count, which the superseded document stated three different ways across four
sites, is stated once here and cited everywhere else) and **B7** (§6.2's
no-restatement claim, which was absolute and false, is stated at the strength the
pin actually has). Every **MEASURED** and **ESTIMATED** mark is the mark the
superseded text carried; no number moved, and none gained or lost a mark.

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes. A citation of
another unit names the unit AND the u-rev cited.

**THIS UNIT HAS NOT BEEN REVIEWED** at this u-rev.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This unit restates no theory; where it appears to, the calculus wins and
the disagreement is an ADR line.

---

## U3-A. Lineage — what has attacked this unit's content, and at which revision

| Round | Against | Verdict reaching M1 / §6 / §10 |
|---|---|---|
| DECISION-RED-TEAM, matrix M1 | revision 1, `ec8f7fb` | **M1 SURVIVES AMENDED.** The reviewer's own re-derivation, sharing no code with the census, disagreed on the READING — threshold against exact — and that disagreement is what killed revision 1's Tier-T option (§12 item 5) |
| REVIEW-design | revisions 2–6 | all FAIL; **M1 was never reopened on its merits.** §6.3's cost column was re-derived twice for transmission defects, and §6.2 became the instrument's output rather than a typed table |
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. **B5 and B7 are this unit's** and are repaired here. **MAJOR 12 is this unit's and is NOT repaired here** — it is a design question, recorded OPEN in U3-Z |
| DECISION-RED-TEAM, restructure | matrix at `eea480b` | F6: after M2's restoration unit 3 would have been the largest unit in option A's cut. Under D, M2 is not restored here at all |

**What this unit owes that no round has given it:** a REVIEW-design of THIS text
at THIS u-rev, and the census's registered replication and second instrument
(U3-Z, OPEN).

---

## 6. MATRIX M1 — Tier-T qualification — SURVIVES AMENDED

### 6.1 The reading, corrected

Revision 1's §10 said `tier_t_own_count = 2` was "mapped to the closed
`LiveCount`", whose `LiveTwo` is `own == 2`. Its §6.1 table was produced by a
census that unioned `LiveTwo ∪ LiveThree`. Re-derived independently over the same
24 corpus roots:

The census block renders both readings as adjacent rows — `option B — Tier T
(threshold, ADOPTED)` against `option B — Tier T (exact, NOT adopted)` — and
revision 1 printed the threshold figure while §10 committed the exact one.

**The option committed was not the option measured.** An implementer following
§10 literally would have shipped a generator the matrix never evaluated — and
under it the reviewer constructed a position where the mover has a forced win in
two own turns that option A finds and exact-C does not, with the pre-registered
fallback to B repairing nothing because exact-B has the same own half.

**ADOPTED: the THRESHOLD reading.** `tier_t_own_count = 2` means own windows at
count **≥ 2**; `tier_t_opponent_count = 3` means **≥ 3**. **MEASURED** cost of
the repair: **+0.17** cells/node for B, **+0.04** for C at corpus roots. There is
no cost argument for the exact spelling, and under the threshold reading B ⊇ C,
so the pre-registered fallback is coherent for the first time.

### 6.2 The measurements, with their sampling regime

**MEASURED** at `f317385`, release, by the census harness
`crates/pistol-solver/tests/wp15b_census.rs`, **committed at `7941775`** rather than deleting with its worktree — CLAUDE.md's
instrument clause, and D-287's rule that an artefact recording numbers is not
test-tree-only.

Three regimes. The middle one is **re-sampled** in revision 2: revision 1 deepened
by uniform draws from the radius-**8** legal ball while the policy is radius **2**,
which inflated the ball 78.0 → 123.7 by the sampler rather than by depth.

<!-- BEGIN CENSUS TABLE — rendered by crates/pistol-solver/tests/wp15b_census.rs -->
| quantity | corpus roots | +1..3 turns, r2 draw (REPORTED) | +1..3 turns, r8 draw (SUPERSEDED) | playouts |
|---|---|---|---|---|
| own hot, mean | 0.0417 | 0.3559 | 0.3299 | 0.0833 |
| opponent hot, mean | 0.4583 | 0.2951 | 0.2101 | 0.0958 |
| live-2 own | 7.2083 | 11.1771 | 11.0694 | 23.7792 |
| live-2 opponent | 12.1667 | 12.4497 | 10.8976 | 25.4302 |
| live-3 own | 0.7500 | 1.7760 | 1.6059 | 1.7063 |
| live-3 opponent | 1.8750 | 1.8733 | 1.4253 | 1.8698 |
| radius-2 ball | 77.9583 | 94.4965 | 123.6615 | 376.4708 |
| cover union when FILTERED | 2.1667 | 2.1698 | 2.1899 | 2.2667 |
| WIN-NOW row | 4.2 % | 23.3 % | 21.7 % | 4.4 % |
| FILTERED row (`Cover::Minimal`) | 25.0 % | 18.4 % | 13.7 % | 3.1 % |
| `Cover::Impossible` | 4.2 % | 1.4 % | 1.2 % | 1.7 % |
| BATCHED nodes | 70.8 % | 61.5 % | 65.5 % | 92.5 % |
| option A — Tier T (threshold, ADOPTED) | 6.1250 | 8.2448 | 7.0382 | 6.6510 |
| option A — Tier T (exact, NOT adopted) | 6.1250 | 8.2205 | 7.0330 | 6.6510 |
| option A — staged, BATCHED only | 21.65 = 3.80x | 23.20 = 4.27x | 21.92 = 5.80x | 21.44 = 17.00x |
| option A — Tier T outside the r2 ball | 1.1250 | 1.0069 | 0.9236 | 0.0167 |
| option B — Tier T (threshold, ADOPTED) | 46.5000 | 54.6250 | 51.6649 | 88.1271 |
| option B — Tier T (exact, NOT adopted) | 46.3333 | 54.3854 | 51.4288 | 87.8708 |
| option B — staged, BATCHED only | 62.82 = 1.31x | 70.36 = 1.41x | 66.77 = 1.90x | 98.17 = 3.71x |
| option B — Tier T outside the r2 ball | 14.8750 | 14.5747 | 12.5851 | 6.1323 |
| option C — Tier T (threshold, ADOPTED) | 23.2917 | 31.4965 | 30.2622 | 48.7344 |
| option C — Tier T (exact, NOT adopted) | 23.2500 | 31.3194 | 30.0938 | 48.5812 |
| option C — staged, BATCHED only | 37.82 = 2.17x | 47.34 = 2.09x | 45.82 = 2.78x | 60.82 = 5.99x |
| option C — Tier T outside the r2 ball | 6.8333 | 7.4549 | 6.9392 | 2.9740 |
<!-- END CENSUS TABLE -->

**This block is the instrument's output and is not typed by hand.** It is
rendered by `crates/pistol-solver/tests/wp15b_census.rs` and pinned by
`the_carved_design_units_carry_this_censuss_table_verbatim`, which fails the
build if the two drift, and which reads **every carved unit and the seed** by an
enumerated path list rather than by one hard-coded path (travelling item T4').

**What the pin refuses, stated at the strength it actually has.** No FOUR-DECIMAL
figure from the block is restated anywhere outside it, in any carved unit or in
the seed, and the pin fails the build if one is. **That is narrower than "no
section restates a number from it" — and the superseded §6.2 made the wider claim
while the document contained four counter-examples to it** (revision-7 review
B7). The wider claim is not made here. Rounded and percentage renderings —
`70.8 %`, `6.83`, `23.2` — are the shape prose actually uses, the pin cannot see
them, and the sites that survive the carve are listed in U3-Z as OPEN. Everything
else cites the block.

Why it exists as a mechanism rather than a resolution: across four revisions this
document moved a number in one section nine times and left a copy of it in
another — §6.2 repaired while §6.3 was not, the instrument extended while its
registered SHA was not, the sampler figures replaced while §12 kept the withdrawn
ones. Writing the lesson down did not stop it. This is D-259's discipline, which
the project already applies to a derived fixture, applied to a design table for
the same reason: an edited number becomes a red test rather than a reviewer's
finding.

**How to read it.** The FILTERED and BATCHED rows are separate because **U2** §5.3 emits
different sets on them: a filtered node emits the cover union alone, a batched one
emits Tier T plus the quiet cut. `quiet_top_k` and `widen_schedule` govern only
the batched population, which is why the staged rows report it rather than a
blended mean — a blend flattered option B by half.

### 6.3 The options

| Option | Theory standing | Cost | Failure modes |
|---|---|---|---|
| A — count ≥3 both sides | **No completeness licence.** `LAW-SUPPORT` k=2 licences windows at ≥2, and T10 adds that a window made hot this turn held ≥2 before — so count 3 misses every plan a PAIR creates from a count-2 window, which is the two-stone move this game is about | The largest reduction of the three — see the census block's `option A` rows | Provably k=2-incomplete. The reviewer built the position: P1 (0,0)(1,0)(2,1)(1,2)(0,3), pair {(2,0),(3,0)}, `t = 4`, `(2,0)` in own count-2 windows only |
| B — count ≥2 both sides | Full licence both sides | The smallest reduction of the three — see the census block's `option B` rows, whose BATCHED figure is the one `quiet_top_k` governs | Its opponent half buys the least, per §6.4's lemma |
| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | see the census block; **MEASURED 29 % of C's Tier T lies OUTSIDE the radius-2 ball** (6.83 cells/node at corpus roots) | Asymmetric, so argued in §6.4. Residual: no cells blocking an opponent count-2 window; left to Tier Q's delta ranking, which is a set of 23.2 cells/node against a quiet allowance of 16 |
| D — a config knob instead of a choice | — | — | Rejected as a matrix answer. The knob exists (§10); what the matrix decides is what the config COMMITS |

### 6.4 The asymmetry, re-grounded

Revision 1's ground was "a defence against the opponent's two-turn win is what
SEARCH DEPTH and the filter are for". **That is falsified by this document's own
MEASURED `depth_at_500ms` = 2 / 2 / 1**: the opponent's second turn is depth 4,
and the engine reaches 2. The sentence is deleted.

The replacement is the reviewer's **count-3-leg lemma**, marked as a DERIVATION
and not a measurement: every k=2 win through `LAW-OVERLOAD` requires at least one
own window at count **3**. If every leg came from count 2, each leg contains both
new stones; by `LEM-CROSS` two windows on distinct axes share at most one cell,
so all legs lie on one axis — a same-line four, `PAT-4IFF`, `t ≤ 2`, not an
overload. Hence ≥1 leg at count 3. The attacker must generate **all** legs of its
own fork, so its half needs count ≥2; the defender need only break **one** leg,
and every fork has a count-3 leg, which C's opponent half carries.

**Its gap is named:** the lemma covers the `t ≥ 3` route only, not the
`LAW-LEDGER` t=2 forcing chain (four → forced blocks → win), whose pre-emption is
exactly the opponent count-2 cell C omits and whose refutation needs depth 4.
Both of the reviewer's constructed positions exhibit the lemma; it is not
exhaustively enumerated.

**Also stated, because revision 1 implied more than the law gives:** `LAW-SUPPORT`
at k=3 requires ≥0 own stones, i.e. no licence for any option. The licence
discriminates only inside a two-own-turn horizon, which is a horizon the engine
currently searches at depth 2.

### 6.5 ADOPTED: C at the threshold reading

**Pre-registered consequence, fixed before any gate runs.** If the soundness
instrument (**U4** §8) shows C dropping a cell a proven tactic needs, C is replaced by
B — which under the threshold reading is strictly wider — and the exchange is an
amendment with its own review, never a threshold move. **And the branch revision
1 omitted:** if the instrument is GREEN while mutation M7 (**U4** §8.4; Tier T at ≥3 for the
mover — option A) also SURVIVES, then the instrument has demonstrated it cannot
tell A from C, C's entire ground is unmeasured, and that is recorded as such in
the results rather than read as a confirmation of C.

**STRONGEST SURVIVING ATTACK** (abridged for the ADR line; the reviewer's full
paragraph is in the round record): *the matrix's MEASURED Tier-T column was
produced by a census reading count ≥2 while its config clause spelled count ==2 —
the threshold reading against the exact one — so the option committed was not the option measured;
and the reduction it is bought with shrinks from 3.1× to 2.4× the moment the
depth stand-in is re-sampled from the radius-2 ball the search actually uses, a
one-second run the document did not take.* Both halves are repaired in revision
2; what survives is that neither repair was found by the author.

---


---

## 7. What survives here of §7, and what does not

MATRIX M2 and the widening schedule are `WPQ_seed.md`'s. Two bullets of §7.2 are
**not** about the schedule — one scopes the whole `Staged` policy against
`Radius`, the other warns a reader of the SPRT verdict — and both are carried
here verbatim because a unit needs them and the seed is not reviewable.

- **THE CUT BINDS UNDER `CandidatePolicy::Staged` ONLY.** Stated because revision
  2 left it implied and an implementer would have had to invent it. Under
  `Radius` the candidate loop is byte-for-byte what ships today: no batching, no
  node protocol, no threat state (`Position::threats` is `None`). Three things
  depend on it — the D-209 golden transcripts are taken at
  `configs/gate_v0.toml`, which is `kind = "radius"`; `tools/determinism.sh`
  runs the same radius configs; and the SPRT's incumbent seat must be the
  committed engine, or the match measures two changes instead of one.
- **And the two SPRT seats therefore differ on a THIRD axis**, named here beside
  the other two (§10 withdrew one such claim already): not only in what they
  SELECT and in what they can SEE, but in SEARCH VALUE — the overload return and
  **U2** §5.3's licensed shortening of mate distances on lost positions both change what
  a node reports. A reader of the SPRT verdict must not read it as a pure
  generation experiment.

**The first of the two is what U2's
`a_radius_policy_search_is_byte_identical_to_the_committed_engine` watches**, and
it is why that test's claim has a reviewable home rather than a home in the seed.

---
## 10. The config shape

**FOUR** complete documents, `deny_unknown_fields`, no code-side default for
any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
and do not restate it** (B5, which found it stated three different ways across
four sites).

| document | mode | `quiet_radius` | `quiet_top_k` | `widen_schedule` | why |
|---|---|---|---|---|---|
| `configs/instrument_staged_v0.toml` | instrument | 2 | 16 | `[32]` | **the SPRT seat and the snapshot's AFTER.** The cut BINDS here, because a seat with the cut disabled would make the SPRT measure nothing about the prune (rule 6, `WPQ_seed.md` §7.2) |
| `configs/tactical_staged_v0.toml` | instrument | 2 | **1024** | `[2048]` | **NEW in revision 7.** The 15 `instrument_v0` tactical cases. The cut is DISABLED, which is what **U4** §8.3(a)'s derivation requires and what revision 6 asserted while committing `quiet_top_k = 16` for these cases |
| `configs/gate_staged_v0.toml` | instrument | 1 | **128** | `[256]` | the five `depth_turns 3` cases, at radius 1. Cut disabled — MEASURED balls 22/22/22/18/15 at 11 stones, bounded by 6 × 17 = 102 three turns deeper |
| `configs/play_staged_v0.toml` | play | 3 | 16 | `[32]` | the movetime measurement, whose incumbent is `play_v0.toml` at radius 3. Cut binds |

**The fourth document exists because three could not carry the requirement.**
Revision 6's **U4** §8.3(a) said "all three staged tactical configs disable the quiet cut" while §10
committed `quiet_top_k = 16` for two of them and §15 said "the two gate configs" —
three statements of one rule, none agreeing. The tension is real and needs a
document rather than a sentence: `instrument_staged_v0.toml` cannot be both the
tactical config (cut off, so `require 20`'s derivation holds) and the SPRT seat
(cut on, or the match measures nothing about the prune). `tactical_staged_v0.toml`
is that fourth document. The `1024` is not a guess: the radius-2 ball is MEASURED
in the census block's own `radius-2 ball` row, whose largest regime mean is under
400, and a bounded ball at 17 stones cannot exceed `6 × 17 = 102` at radius 1 or
`18 × 17 = 306` at radius 2.

Every other key is identical to the radius document it is the counterpart of, so
each is complete under rule 1 without restating the whole schema here; revision 4
promised "three complete documents" and printed the policy block of one.

**`widen_schedule` is defined against `quiet_top_k`, in QUIET CELLS, and both
ends are named.** Revision 2 left four questions an implementer would have had to
answer by invention.

- The **first** batch is `quiet_top_k` quiet cells. Tier F and Tier T are always
  emitted whole and are not counted against it (**U2** §5.4).
- The schedule's entries are **cumulative counts of QUIET cells**, not indices
  into the whole vector.
- A pool **shorter** than the first boundary never truncates, so the node is not
  counted in the widening schedule's registered denominator (`WPQ_seed.md`
  §7.2). Correct, and now stated.
- A pool **longer** than the last boundary is cut there permanently. That is what
  a finite last entry is FOR, and it is the forward prune the deferred schedule's ADR line names
  (`WPQ_seed.md`, item 3).
- Cross-field validation, which revision 2's validator lacked: every entry must
  exceed `quiet_top_k`. `quiet_top_k = 64` with `widen_schedule = [32]` passes
  revision 2's "non-empty and strictly increasing" and describes a widening that
  NARROWS — a named refusal under rules 1 and 3.

`schema_version` stays **2**: adding a `kind` to a tagged enum leaves every
existing document valid, and D-16's bump is for a change that invalidates one.
Recorded rather than left silent.

```toml
[search.candidate_policy]
kind = "staged"
quiet_radius = 2
quiet_top_k = 16
# Batch boundaries after the first. The LAST ENTRY IS FINITE: "all remaining"
# is what makes a widening schedule a rename of full width (WPQ_seed.md §7).
widen_schedule = [32]
# LAW-SUPPORT qualification, THRESHOLD reading: >= 2 for the mover, >= 3 for
# the opponent (§6).
tier_t_own_count = 2
tier_t_opponent_count = 3
```

Validation, in `pistol-engine`'s validator and again in `Searcher::new` (a
`SearchParams` can be built in code and never passes through a document):
`quiet_radius` in `1..=MAX_CANDIDATE_RADIUS` and representable as `i16`;
`quiet_top_k >= 1`; `widen_schedule` non-empty, strictly increasing, **every entry
greater than `quiet_top_k`**, and **no sentinel admitted**; `tier_t_own_count` and `tier_t_opponent_count` in `{2, 3}`; and **every
`widen_schedule` entry strictly greater than `quiet_top_k`**, which revision 3's
validator did not check — `quiet_top_k = 64` with `[32]` passed "non-empty and
strictly increasing" and described a widening that NARROWS.

**And the threshold is NOT "over `LiveCount`", which cannot express it.**
`LiveCount` is closed at `{Two, Three}` (D-255, a compile error otherwise), so it
cannot name `>= 4`; the `>= 4` windows are `hot_windows`, a different set. A count
of `n` therefore means the UNION:

```
n = 2  ->  live_cells_at_count(side, Two) ∪ live_cells_at_count(side, Three) ∪ threat_cells(side)
n = 3  ->                                   live_cells_at_count(side, Three) ∪ threat_cells(side)
```

Reachable, not pedantic: at `Phase::Second` with an own hot-4 window and no
win-in-one-ply, `can_win_this_turn` is `None`, the node takes a BATCHED row, and
that window's empties are in Tier T under the union reading and absent under the
`LiveCount`-only one. Revision 3 said "threshold over `LiveCount`" in §10 and
spelled the EXACT-count union in the test plan's referent (U3-T) — two different sets in one
document, which is §0 row 4's class recurring in the opposite direction. The
committed census implements the union reading, so §6.3's numbers are the union's.

`instrument_r2_v0.toml` is value-identical to the committed `instrument_v0.toml`
(D-194) and is the SPRT's incumbent seat.

**Revision 1's config comment is withdrawn.** It read "`quiet_radius = 2` so …
the SPRT's two seats differ in what they SELECT rather than in what they can
SEE". MEASURED, **29 % of option C's Tier T lies outside the radius-2 ball**
(6.83 cells/node at corpus roots), so the seats also differ in what they can see.
The comment now says so.

---


---

## U3-T. The tests this unit registers

Carried from the superseded §11. The rows this unit does not own are in U2-T,
U4-T and `WPQ_seed.md`, and no row is in two places.

| Test | Watches |
|---|---|
| `tier_t_qualification_matches_adopted_matrix_option` | the Tier T set against an independent **`us@{2,3} ∪ threat_cells(us) ∪ them@{3} ∪ threat_cells(them)`** — the UNION reading §10 establishes, since `LiveCount` cannot express ≥ 4. On a position where exact-2 and ≥2 DIFFER, and one where the `LiveCount`-only and union readings differ. Revision 4 spelled the `LiveCount`-only referent here while §10 corrected it two sections earlier |
| `the_fallback_under_staged_answers_from_the_quiet_radius_ball` | the turn `fallback_turn` returns under a Staged policy, and that it reads no threat state — the bounded, pure property WP-1.4's movetime ceiling rests on |
| `no_candidates_under_staged_is_refused_by_a_policy_agnostic_error` | the error variant at a root the policy cannot serve; `SearchError::NoCandidates { turn, radius }` names a `radius` a Staged policy has three of |
| `tier_t_cells_match_an_independent_window_walk` | the emitted Tier T against a from-scratch enumeration, on a position where the `LiveCount`-only reading and the union reading DIFFER |

---

## U3-M. What this unit measures

ADVISORY on this machine; the operator re-runs for the record. (A standing
condition of every measurement in every unit, stated per unit so a unit is
readable alone; it is a condition, not a datum.)

4. **D-263's registered hotspot — the bracket is recomputed, and the pre-registered
   hotspot turns out not to be the dominant one.** **MEASURED**, release:
   `blocking_covers` 246 / 71 / 69 ns mean (max 1513 / 1252 / 2665 ns),
   `unblockable_double_threat` 101 / 50 / 49 ns; a deliberately built family of 16
   disjoint hot windows costs 1479 ns/call, and the maximum hot count observed
   anywhere is **5**.

   **The bracket, corrected.** Revision 2 printed "0.6 %–3.7 % of a node" and
   declined all three of D-263's remedies on it; the bracket omitted its own worst
   cell. Per-regime sums are 347 / 121 / 118 ns against node times of 21 277 ns
   (47 knps) and 3 300 ns (303 knps), so the true ceiling is **10.51 %** — about
   3× what was printed — and one worst-case call is **81 %** of a fast node.

   **The remedies stay unimplemented, and revision 3's adoption of the first one is
   WITHDRAWN.** Revision 3 implemented the three-pairwise-disjoint-families
   early-out. Two measurements retire it: it needs three families and **1 of 24**
   corpus roots has them (17 roots have none, five have one, one has two); and it
   accelerates `min_hitting_set_exceeds`, which under M5-E (**U2** §5.2) is **no longer
   called per node at all**. M5-E delivers **−29.1 % / −41.3 % / −41.5 %** of the
   registered per-node threat cost by deleting the redundant query — a larger cut
   in the same hotspot than any remedy D-263 names, and it needs no new code in
   `pistol-solver`.

   **THE REAL HOTSPOT IS TIER-T EXTRACTION, and it is registered here BEFORE the
   change that touches it**, which is what rule 5 asks. **MEASURED** on one
   harness over the 24 corpus roots (see the population caveat below): extracting
   Tier T's cells costs about **6×**
   both threat queries combined (533 ns with a reused buffer, 662 ns fresh,
   against 86 ns for the pair on the same harness). D-263 named the cover
   arithmetic and the measurement says otherwise — which is a pre-registration
   doing its job, not failing it. **Registered rule-5-shaped**, which revision 4's registration was not — it named a
   mechanism where a bracket belongs. HOTSPOT: Tier-T cell extraction on the
   per-node path. EXPECTED GAIN BRACKET: the honest answer is that **no bracket can be
   derived before the IMPL measures it**, and revision 5's `[1.10×, 1.35×]` was
   anchored on the wrong comparison — 662/533 = 1.24× is the cost of NOT reusing a
   buffer, a saving the search gets free with one scratch `Vec` on `Run` and
   without any accessor, since `query.rs`'s cell queries already fill a
   caller-supplied `&mut Vec<Coord>`. So the accessor's own gain is the per-window
   public-boundary crossing alone, which nothing has measured. The registration is
   therefore: **BASELINE = the in-search mask walk with a reused buffer, MEASURED
   first, in its own commit**; the accessor is then a second commit whose bracket
   is set from that baseline before it is written. ABORT THRESHOLD: below 1.05×, or any
   regression in whole-search nps. INSTRUMENT: one IQR-gated bench reporting
   **nps AND time-to-depth**, per rule 5, not the snapshot — which reports
   `depth_turns` and `nodes` only. ONE CHANGE = ONE COMMIT.
   **And the number is re-taken on the right population**: 533/662 ns were
   measured over all 24 corpus roots, but **U2** §5.3 does not extract Tier T on the
   **29.2 %** of them that take a forced row, so the registered figure is a
   blended mean over two populations — the same mixture defect §6.3 was corrected
   for. The IMPL re-takes it on BATCHED nodes only.

   **The surface gap behind it.** Tier T needs the empty cells of live-2 and live-3
   windows, and after D-261 `pistol-solver` offers **no convenience accessor** for
   them — `live_windows_at_count`, `masks()` and `Window::cells()` are all public
   and are the route the committed census takes, so the claim is about ergonomics
   and per-node cost rather than reachability:
   `threat_cells` covers hot only, `cells_raising_to_hot` is closed at
   `NearHot::Three`, and `empty_cells` is crate-private. The committed census had
   to walk `masks()` bits against `Window::cells()` per window, which is exactly
   what the search would have to do per node. D-261's flip clause — "Flips when a
   consumer outside this crate needs one of these names — additive, one `pub use`
   each. WP-1.9's instrument is the nearest candidate and is NOT one" — names this
   WP as that consumer, and item 16 (U3-Z) takes the line.


5. **The census is `crates/pistol-solver/tests/wp15b_census.rs`, coupled to this
   document by a TEST rather than by a SHA**, and `tools/baseline_snapshot.sh` is
   at **`e889b5b`**.

   The instrument clause asks that a change to the instrument reopen the review.
   A recorded SHA does that only if someone re-reads it: revision 5 named the
   census at `7941775`, a revision emitting THREE regimes, while a whole column of
   its own tables came from a fourth added in the same commit as the document —
   the SHA went stale in the commit that wrote it.
   `the_design_document_carries_this_censuss_table_verbatim` does it mechanically
   instead: change the instrument and the build fails until the document is
   re-rendered. That is a stronger discharge than a SHA, not a weaker one, and it
   is the same substitution D-284 made for this log's own integrity — a property
   nobody was checking became a gate.

   The residual, stated: the pin covers the TABLE. Prose claims about the census —
   its sampling regimes, what a column means — remain judged, and so does
   `tools/baseline_snapshot.sh`, whose output this document does not carry.

   **The second-instrument framing of revision 2 is WITHDRAWN, and what actually
   happened is recorded instead.** Revision 2 registered "the two regimes must
   agree on the RANKING of options A, B and C by staged-set size". Under the
   adopted threshold reading `own≥3 ⊆ own≥2` and `them≥3 ⊆ them≥2`, so
   **A ⊆ C ⊆ B as SETS** and `|A| ≤ |C| ≤ |B|` holds in every position under every
   sampler — verified over all 24 corpus roots, strict on 23 and 24 of them. The
   ranking is a set-inclusion identity, so the criterion could not have been
   falsified by the sampler defect it named, nor by anything else. That is the
   vacuity CLAUDE.md forbids, registered as the answer to a doubt.

   **The real second instrument already ran, and it already disagreed.** The
   fresh-context DECISION-RED-TEAM on M1 independently re-derived §6.2's
   population columns from the same corpus, sharing no code with the census. Its
   agreement was exact on every population number — and its DISAGREEMENT was
   the exact reading against the threshold one, which is what exposed that defect that killed revision 1's Tier-T option. The stage
   under doubt was the census's READING of `tier_t_own_count`, the two instruments
   did not share it, and the consequence was that M1 reopened. Nothing needs
   registering after the fact; what needs recording is that the criterion which
   worked was independent re-derivation by a fresh context, and the criterion this
   document invented was an identity.

   **What is registered forward**, for the deepening sampler: the radius-2 regime
   is the reported one, and the radius-8 regime's numbers are retained as
   SUPERSEDED with the delta stated — both regimes' figures are in the census block and the
   sensitivity is read from it rather than restated here, which is the rule the
   block exists to enforce. No verdict in this document turns on a quantity that
   moves by less than the sampler does between those two columns.

---

### Cost

| Item | DECLARED | MEASURED |
|---|---|---|
| The census harness | ~1 min | **< 1 s** per run after a 1.3 s build |

The census is the reason CLAUDE.md's proportionality clause bites here rather than
being argued around: a run measured in seconds is answered by REPLICATION and a
SECOND INSTRUMENT, never by a margin. Neither is registered — U3-Z.

---

## U3-Q. The conservative branches this unit records

- **The committed configs do not move.** `instrument_v0.toml` and `play_v0.toml`
  stay at `kind = "radius"`; Staged ships as the selectable documents §10 lists,
  and §10 is the one place their number is stated (B5). The SPRT is the judge and
  it is the operator's run — D-190/D-194's own order.
- **D-263's three remedies stay unimplemented**, on a corrected bracket and a
  measured firing rate — and the measurement says D-263 named the wrong hotspot
  (§12 item 4).
- **The census is WP-1.6's to extend**: it renders population figures for any
  regime added to it, and the pin makes a unit that cites them unable to drift.

---

## U3-Z. ADR lines this unit owes, and what is OPEN

### ADR lines

Carried from the superseded §15. Its item numbers are retained exactly so an
existing cross-reference to "§15 item n" still resolves; this unit invents none
and renumbers none. The superseded §15's preamble does not travel (MAJOR 10
measured it false on both clauses); this is U3's lead-in instead: **items 2, 7
and 16 are this unit's own and have not landed; items 12 and 23 are corrections
to LANDED lines and have themselves LANDED, which the superseded list said of
only one of them** (MAJOR 13).

2. Tier-T option C at the threshold reading, with §6.5's surviving attack and the
   count-3-leg lemma's two named gaps.

7. **D-263's three remedies stay unimplemented, and revision 3's adoption of the
   first is WITHDRAWN** — it needs three disjoint families and 1 of 24 corpus
   roots has them, and it accelerates a query M5-E deletes. The larger cut in the
   same hotspot comes from removing the duplicated query (−29 % to −42 %).
   **AND D-263 NAMED THE WRONG HOTSPOT**: Tier-T cell extraction is MEASURED at
   about 6× both threat queries combined. Registered here with its own bracket and
   abort threshold, before the change that touches it (rule 5).

12. **D-255 is wrong on a number it states.** It says "the corpus shows own-side
    hot = 0.0 mean / 0 max at both stone counts". The census block's `own hot,
    mean` row at corpus roots refutes it, at index 16 / 31 stones, which sits in
    the 35-band. **LANDED as D-301** at `68a28c8`.

16. **D-261 gains a query.** Tier T needs the empty cells of live-2 and live-3
    windows and the public surface has no route to them; D-261's flip clause names
    this WP as the consumer. `ThreatState::live_cells_at_count(side, LiveCount,
    &mut Vec<Coord>)`, additive, with the map entry D-267 requires of a new query
    naming its calculus ID (`LAW-SUPPORT`'s k=2 qualification), and the recorded
    coincidence that it equals `cells_raising_to_hot(side, NearHot::Three)` at
    count 3 while MEANING something different.

23. **The census stays in the test tree while recording numbers**, which is the
    case D-287's clause reserves for a future ADR ("promotion is a FUTURE ADR,
    owed the day anything records a number from it"). The line records why it does
    not move to `tools/`: `tools/` membership pulls in SHELL_CHECKLIST's coverage
    rule and the shell instrument rules, and this artefact is a Rust test driven by
    `cargo test` with its own pinning test — the coverage rule's intent, met by a
    different mechanism.
    **LANDED as D-304**

### OPEN — carried forward, not closed by the carve

- **MAJOR 12 — the unmarked `23.2` in §6.3's option-C failure-mode cell.** It
  carries neither **MEASURED** nor **ESTIMATED**, on the cell that states the
  ADOPTED option's residual risk, and the review found it is not a census row at
  all: the block renders no Tier-Q quantity, and `23.2` is the rounding of the
  block's own `option C — Tier T (threshold, ADOPTED)` row at corpus roots —
  cited by name, because restating its four decimals here is what the pin
  refuses — which is Tier T and not Tier Q. **The carve
  preserves marks and does not add them** — adding one would decide whether the
  figure is measured, and the review's finding is that it may be mis-attributed.
  Either the figure is wrong or it is an unmarked estimate with a committed
  instrument standing beside it, which is D-291's clause. **A repair here is a
  design act, not a carve act.**
- **B7's residual, four sites the pin cannot see.** The pin refuses four-decimal
  restatements only. Rounded and percentage renderings survive the carve at:
  **U2** §5.3's `70.8 %` (**U2**), §6.3's `6.83` and `23.2`, §10's `6.83`, and
  §8.4/§8.5's `70.8 %` (**U4**). Each is carried verbatim; none is repaired,
  because deciding whether `70.8 %` is the same quantity as the block's `BATCHED
  nodes` row is a design question and not a transcription one. Widening the pin's
  scan past four decimals is likewise an instrument change, not a re-target, and
  T4' does not license it.
- **The census owes a registered REPLICATION and a SECOND INSTRUMENT** (the
  superseded §17's own list). It runs in under a second, so CLAUDE.md's clause
  gives it no room to argue: the agreement criterion is registered before either
  runs, it names the stage under doubt and how the second instrument does not
  share it, and it carries a registered consequence for disagreement. §12 item 5
  records that the criterion which WORKED was independent re-derivation by a
  fresh context and the one this document invented was a set-inclusion identity —
  that is a record of what happened, not a registration for next time.
- **The D-scope of `quiet_top_k` and `widen_schedule`.** §10's four documents each
  commit both keys, and D-310 defers the stage they govern. Whether the shipped
  `Staged` surface keeps them (validated, inert, and set wide), narrows to Tier F
  ∪ Tier T with no quiet tier at all, or something else, changes the config
  documents, the validator and the SPRT seat. **The carve does not choose.**
- **No REVIEW-design has run against this text at this u-rev** (U3-A).

---

*U3, u-rev 1. A carve, not a revision. IMPL has not started.*
