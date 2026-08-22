# WP-1.5b U2 — the normative node protocol: DESIGN UNIT

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 3.** Carved from `docs/experiments/wp15b_design.md` §2, §3, §5 and §14 at
`6feb40a` (revision 7, never reviewed, CLOSED by D-309) under the restructure
selected as option D by D-310. The carve's section-to-owner map is
`docs/experiments/section_owner_table.md`. The superseded document is not
in the tree: it is retrievable at `6feb40a` and nowhere else.

**u-rev 2 was a REPAIR, not a new carve.** It answered
`docs/experiments/wp15b_U2_REVIEW.md`'s FAIL against u-rev 1 (pinned `38f21b9`):
F2 (MAJOR, §5.3's unreachable fixture) and F1 (MINOR, §5.4's undisclosed
citation correction), plus a one-sentence architect ruling (R5, Tier Q).
Nothing else in that text changed.

**u-rev 3 is a REPAIR of u-rev 2's own review, not a new carve.** It answers
`docs/experiments/wp15b_U2_REVIEW_urev2.md`'s FAIL against u-rev 2 (pinned
`56b0bec`, byte-identical at HEAD `e3f0bc3` when that review ran): F3 (MAJOR,
the "exceptions" paragraph's own count went stale the moment F2 landed — it
omitted F2's own §5.3 rewrite, the largest exception) and F4 (MINOR, the D-257
quotation at §5.3 carried a second, unmarked elision). Nothing else in this
text changed.

**THE TEXT IS A VERBATIM CARVE.** Every change made to it is a CROSS-REFERENCE
RETARGET — a `§n` that pointed inside the superseded document now names the unit
that owns that section, or names `WPQ_seed.md` where the referent is stage Q —
plus one B5 repair in §2.2 (below). No sentence of §3, §5 or §14 was rewritten,
extended or re-derived, and no number moved. Every **MEASURED** and **ESTIMATED**
mark is the mark the superseded text carried.

**Four exceptions, all stated where they occur rather than only here. Each is a
change to this unit's CARVED CONTENT — prose carried over from the superseded
document's §2, §3, §5 or §14. This list, the u-rev label, and the rest of the
head's own apparatus are NOT carved content; they are new text the carve itself
writes and are expected to change every u-rev (LABEL DISCIPLINE, above) — so an
edit to this paragraph, including u-rev 3's addition of the fourth item below,
is not a further exception the list owes itself:** §2.2's config-count sentence
became a citation of U3 §10 (B5 — the count was stated three different ways and
is now stated once, there); §12 item 2's rate list hands the widening rate and
the declined-TT-entry count to `WPQ_seed.md` with stage Q, keeping the counter
SEAM here because the seam is what a later WP reads them through; §5.3's
`Run::salvage`-cost sentence silently repointed a dangling citation — the
superseded text read "§12 item 6", which never existed (§12 had five items at
`6feb40a`) — to its actual referent, "U2-Z item 20" (i.e. superseded §15 item
20), a content correction rather than a retarget, found by the u-rev 1 review
(`docs/experiments/wp15b_U2_REVIEW.md`, F1) and disclosed at the point of
occurrence in this u-rev; and §5.3's two-ply-illustration rewrite (F2, found by
the same u-rev 1 review) — roughly sixteen new lines of new prose, not a
`§n`-retarget: the "ILLUSTRATION ONLY; … WITHDRAWN" heading clause, the
withdrawal of the "VERIFIED on the shipped solver" sentence, a new D-257
blockquote, the rule-3 arithmetic (`1 + 2(k − 1) = 2k − 1`), and a rewritten
D-243(4)-discharge sentence — the largest of the four, and the one this list
omitted until the u-rev-2 confirmation re-review caught the omission (u-rev
2 → 3, F3).

**WHY THIS UNIT'S REVIEW IS A CONFIRMATION PASS.** Five fresh-context
REVIEW-designs and two DECISION-RED-TEAMs have run over this text and **none
broke it on the merits**. The revision-7 review's own Rejected list, with its
attempted reproducers, covers §2.1's six destructure sites (reproduced accurate
at `6feb40a`), §3's twenty-three calculus IDs (all exist), the solver query
surface §5 consumes (all public bar the one §5.35 correctly names as owed),
§5.2's cited source lines, §14's scope, and the hotspot registration. M5 FELL at
`7ad466b` and M5-E — supplied by the red team that killed M5-A — has survived
every round since. The confirmation pass is over THAT: that the carve moved this
text without changing it, and that its retargets resolve.

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes. A citation of
another unit names the unit AND the u-rev cited.

**THIS UNIT HAS BEEN REVIEWED TWICE, AND BOTH REVIEWS FAILED.** u-rev 1 (pinned
`38f21b9`) was reviewed by `docs/experiments/wp15b_U2_REVIEW.md`, a CONFIRMATION
PASS: **VERDICT FAIL**, 1 MAJOR (F2, §5.3's unreachable fixture) and 1 MINOR (F1,
§5.4's undisclosed citation correction). u-rev 2 — the repair answering those two
findings, plus one architect ruling (R5) — was reviewed by
`docs/experiments/wp15b_U2_REVIEW_urev2.md`, again a CONFIRMATION PASS:
**VERDICT FAIL**, 1 MAJOR (F3, the exceptions paragraph did not name F2's own
§5.3 rewrite) and 1 MINOR (F4, the D-257 quotation's unmarked second elision).
This u-rev (3) is the repair answering F3 and F4. It is unreviewed at ITS OWN
revision — a repair reopens the review exactly as any other amendment does — and
a WP is not landable while a review is outstanding.

**BUILD ORDER.** This unit's IMPL is the commit that gives `pistol-search` a
normal dependency on `pistol-solver` — MEASURED at `08cf4f7`, `pistol-solver` is
absent from `[workspace.dependencies]` and from `crates/pistol-search/Cargo.toml`,
so the edge does not exist yet. **U2 lands before U1's gates are armed** (U1 §U1-B,
travelling item T7). Review order is free; IMPL order is not.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This unit restates no theory; where it appears to, the calculus wins and
the disagreement is an ADR line.

---

## U2-A. Lineage — what has attacked this unit's content, and at which revision

| Round | Against | Verdict reaching §2, §3, §5, §14 |
|---|---|---|
| DECISION-RED-TEAM ×5, one per matrix | revision 1, `ec8f7fb` | none of the five was M5; M5 did not yet exist |
| REVIEW-design | revision 2, `182f389` | **FAIL** — **2 STOP, both in §5**, the normative content: §5.3's `Impossible` row dropped mates, and the survival filter's antecedent was half-established. Redesigned in revision 3, not clarified |
| DECISION-RED-TEAM, matrix M5 | revision 3, `7ad466b` | **M5 FELL** → M5-E, supplied by the red team. §5.2's two queries proved to be one predicate |
| REVIEW-design | revisions 3–6 | all FAIL; **no STOP after revision 2 and no reopening of M5**. §5's residual findings were transmission, not merits |
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. **None of the seven BLOCKING is §2's, §3's, §5's or §14's**, except B5's §2.2 site, repaired here |

**What this unit owes that no round has given it:** a REVIEW-design of THIS text
at THIS u-rev, and rule 5's bracket for the node protocol itself (U2-Z, OPEN).

---

## 2. What this work package changes

### 2.1 The shape

`CandidatePolicy` gains a second variant, `Staged`. `pistol-search` takes a
normal dependency on `pistol-solver` — the first structural act, and the one
D-282 measured the gates' expiry against (**U1** §4).

`Position` already moves `GameState` and `Eval` from one place because they are
only correct together (D-41). It gains a third member, `Option<ThreatState>`,
moved from the same place. `Option`, not a bare field: under
`CandidatePolicy::Radius` nothing consults it, and a radius search paying for a
threat state it never reads would make the SPRT's incumbent slower for no reason
— a measurement hazard in the direction that flatters the change under test.
`reset_to` rebuilds it from scratch, O(stones × 18) once per search.

`pvs::visit` gains the node protocol (§5) and a batched candidate loop (**the
batched loop is stage Q, DEFERRED with the widening schedule — see
`WPQ_seed.md`**).
`SearchInfo` gains stage counters (U2-M item 2). Nothing in the line protocol's output
changes, so the D-209 golden transcripts — which are taken at `configs/gate_v0.toml`,
not at the instrument config (**U4** §9) — are untouched either way.

**Adding the variant is not a local change.** MEASURED by grep at this revision:
**six** irrefutable `let CandidatePolicy::Radius { radius } = …` destructures
become compile errors the moment `Staged` lands — `tests/common/reference.rs:242`,
`tests/common/agreement.rs:131`, `src/search.rs:97`, `src/search.rs:311`,
`pistol-cli/src/bin/pistol.rs:131`, `pistol-engine/src/validate.rs:82` — plus two
match arms, across 30 mentions. Revision 1's "reuses `reference.rs` unchanged"
was false and is withdrawn.

### 2.2 What does NOT change

`configs/instrument_v0.toml` and `configs/play_v0.toml` stay at `kind =
"radius"`. Staged ships as config-selectable documents; **U3 §10 states how many,
and is the only place that states it** (B5). The committed
default moves only when SPRT rules — the operator's run, after this session.
This is the CONSERVATIVE/INCUMBENT branch, and it is what D-190/D-194 did: the
H1 action that moved the committed radius landed *after* the games.

Consequences named so nobody rediscovers them:

- `tactical_v0.txt` keeps binding at the radius config (D-204). Staged gets its
  own sha-pinned `tactical_staged_v0.txt`, which `docs/ROADMAP.md` already
  schedules.
- D-204's flip clause is **not** fired by this WP. It fires when the operator's
  SPRT moves the committed config.
- **WP-1.5b does not complete `docs/ROADMAP.md` WP-1.5's supersession.** The
  ROADMAP says staged generation "SUPERSEDES the radius policy as the primary
  candidate source (radius stays as a config-selectable fallback)"; this WP
  performs the parenthetical only. The ROADMAP changes only by ADR, so the
  deferral takes an ADR line (U2-Z item 10) rather than living in this prose.

---

## 3. MANDATORY DESIGN ITEM 1 — the D-267 ledger, dispositioned

**The count, taken from the artefact:** NINE entries naming TEN IDs. Every one is
dispositioned; none is silent.

| # | ID(s) | Disposition |
|---|---|---|
| 1 | `PROTO-PAIR` | **ALREADY IMPLEMENTED, mapping recorded.** Verified against shipped code, not a doc comment: unordered-pair canonicalisation is `pistol_core::turn::canonical_pair` (`Turn::Pair(a, b)` with `a < b`), landed in WP-03/04; the intra-turn phase bit is `zobrist::phase_key`, mixed into `GameState::key`. Neither moves |
| 2 | `DEF-STAR` | **DEFERRED to WP-1.8.** The star is the relevance-zone primitive; staged tiers are defined by window membership, not geometric reach |
| 3 | `DEF-TEMPO` / `ADOPT-TEMPO` | **DEFERRED to eval-terms.** τ is an eval-term candidate by **U3** §6's own note, SPRT-gated |
| 4 | `LAW-RIPOSTE` | **DEFERRED to WP-1.6.** Forced-reply plan checks belong to a prover; this WP has none |
| 5 | `LAW-LEDGER` | **DEFERRED to WP-1.6.** t=1 chain semantics and the 2−t bank are stand-pat questions |
| 6 | `LAW-DECOMP` | **DEFERRED to Stage 3.** Its warning is already respected: this WP sums no regional eval |
| 7 | `ZONE-R` | **DEFERRED to WP-1.8 / Stage 3.** Every claim this WP makes is bounded-depth and is therefore EVIDENCE and never PROOF (`REJ-DEPTHPROOF`, which binds us too). **U4** §8.6 states this for the soundness gate — revision 1 claimed §11 did and it did not (M3 F12) |
| 8 | `THM-WINDOW` | **NOT CONSUMED; REGISTERED BY NAME** (U2-Z item 8). Exact-`t` counters as eval terms become cheap precisely because this WP puts a `ThreatState` on the per-node path |
| 9 | `E-INIT` | **DEFERRED to eval-terms.** `[CONJ]`, SPRT-gated by the calculus |

**The two places the code is deliberately weaker than the law.** Both are
CONSUMED here, and each gets a test pinning the composition:

- **`min_hitting_set_exceeds`'s completed-window reading.** It reads "`t` exceeds
  the budget OR the family contains a won window" (D-267). Composition: the
  overload check (§5.2) runs only at nodes the search has not already ended —
  `visit` returns on `PlyOutcome::Win` before entering a child — so no node it
  runs at has a completed window for either side. That is an argument, so it is
  also a `debug_assert` and a test:
  `overload_composition_handles_completed_window_reading`.
- **`unblockable_double_threat` carrying LAW-OVERLOAD minus its guard.** The
  law's "defender cannot win this turn" clause lives in the caller (D-243 (3),
  D-257). Composition: step 1 runs first and unconditionally; step 2 is entered
  only when step 1 found no win. Pinned by
  `overload_check_guarded_by_own_win_now`. The exemplar this follows is
  `threat_query_tests.rs::composed_win`, D-257's amendment included:
  `GameState::outcome()` first, `StonesLeft::from_state` second,
  `state.to_move() != side` never.

---

## 5. The normative node protocol, adapted to the two-ply turn

### 5.1 Step 1 — win-now (`PROTO-NODE` 1, `LAW-SUPPORT` k=1, `E-PHASE`)

Realised as GENERATION, not an early return, because `visit` returns a score AND
leaves a line at its ply: a node that returns a mate score without placing the
stone leaves its parent promoting a line that stops short. The existing
`PlyOutcome::Win` arm already scores rule 4's truncation exactly
(`mate_in(turns_from_root + 1)`, D-72).

Tier F therefore always contains, for the side to move:

- `win_in_one_ply_cells(us)` — every size-one plan, at either budget. `PAT-GAP`
  from the attacking side.
- at `StonesLeft::Two` only, **both** empties of every own hot window holding
  exactly four stones — `WinWitness::Pair`'s class in full, not
  `can_win_this_turn`'s single witness. D-243 (2): a delta ranking cannot be
  trusted to surface both halves of a pair whose first stone is worth little
  alone.

At `Phase::Second` the pair class is not generated; the type says so
(`StonesLeft`), so the code cannot get it wrong silently.

### 5.2 Steps 2 and 3 — ONE match, because they are one predicate

Revision 3 wrote step 2 as a standalone check and step 3 as a `match` on
`blocking_covers`. A DECISION-RED-TEAM proved they are the same question:

> `blocking_covers(us, b) == Cover::Impossible ⟺ unblockable_double_threat(them, b)`

from the source — `blocking_covers` (`cover.rs:177`) answers `Impossible` iff
`hot_windows(them)` is non-empty and no cover of size ≤ b exists, and
`min_hitting_set_exceeds` (`cover.rs:145`) answers `false` on the empty family and
otherwise the same "no cover of size ≤ b" over the same universe and the same
`covers` predicate — and MEASURED over **145 158** playout positions and **343 344**
side-and-budget comparisons with **zero** disagreements.

So revision 3 paid a hitting-set computation and a `can_win_this_turn` twice at
every node. Revision 4 computes each once:

```
// PROTO-NODE steps 1-3, in the calculus's own order, with one query each.
let left = StonesLeft::from_state(state)
    .unwrap_or_else(|| panic!("{OVERLOAD_ON_A_DECIDED_POSITION}: …"));
match threats.can_win_this_turn(us, left) {
    // Step 1. The node's value is mate_in(k+1), the ceiling.
    Some(_) => WIN-NOW ROW,
    None => match threats.blocking_covers(us, HitBudget::from(left)) {
        Cover::NothingToBlock   => BATCHED ROW,
        Cover::Minimal(covers)  => FILTERED ROW,
        // Step 2. LAW-OVERLOAD's verdict, reached without a second query.
        Cover::Impossible if !is_pv => return -mate_in(turns_from_root + 2),
        Cover::Impossible           => BATCHED ROW,
    },
}
```

**MEASURED saving**, arithmetic on **U3** §12 item 4's own figures: the registered per-node
threat cost falls 347 → 246 ns at corpus roots (−29.1 %), 121 → 71 (−41.3 %) and
118 → 69 (−41.5 %), and D-263's recomputed ceiling falls from **10.51 % to
7.45 %** of a fast node — a larger cut in the registered hotspot than any remedy
D-263 names, taken by deleting work rather than accelerating it.

`StonesLeft::from_state` is `None` only on a decided position, which `visit`
cannot reach — `place` returns `PlyOutcome::Win` and the parent scores without
recursing, and `check_root` refuses a decided root. So the honest spelling is a
NAMED PANIC and not a silent `?` (rule 3); revision 3 wrote `?` inside a function
returning `i32`, which does not compile and hid the question.

#### The overload return

- **The guard is step 1's answer**, established by the `None` arm rather than
  re-asked (D-243 (3), D-257 (a)/(b)).
- **The distance is exact and is `k + 2`.** At a node `k` turns from the root it
  is our turn — the `(k+1)`-th — and the opponent completes six on the `(k+2)`-th.
  Checked against `score.rs`: `visit` carries mate scores root-relative in
  distance and node-relative in sign, so the value is `-mate_in(k+2)`; `k <= 65`,
  so `k+2 <= 67`, far inside `MAX_MATE_TURNS = 1000`. Verified by exhaustive
  escape search: 24 firing positions, **2 008 879 legal turns tried**, both
  orders where a pair is legal in only one, **0 escapes**.
- **`!is_pv` is the gate**, for the reason `pvs.rs` already gives its table
  cutoff: a PV node must return the line that proves its score. Ply 0 is always a
  PV node — `iterate` opens at `(-INFINITY, INFINITY)` — so the root can never
  take it and `NO_MOVE_FROM_A_COMPLETED_ITERATION` is unreachable through this
  path. **That is a property of TODAY's construction, not of the search**: Stage 4
  schedules aspiration windows, which would narrow the root. It is therefore an
  `assert!` at the return site under a named invariant, in this project's own
  idiom (`NO_CANDIDATES_MID_TURN`, `STATIC_EVAL_MID_TURN`), rather than a sentence
  in a document.

#### Both phases, and the generalisation stated rather than assumed

The check runs at `Phase::First` **and** `Phase::Second`, with the budget from
`StonesLeft::from_state`. This is deliberate in revision 4; revision 3 dropped the
phase conjunct silently, which is why its own author briefly mistook the branch
for a defect.

**Where the value is.** MEASURED, 14 corpus roots at depth 3: **455 177 of
455 201** firings are `Phase::Second`. Restoring a `phase == First` conjunct takes
the node saving from **−47.25 % to −0.065 %** and leaves the move played identical
on all eight roots tested. The phase-1 branch is the option.

**The generalisation, derived rather than attributed.** `LAW-OVERLOAD` is stated
for `t >= 3` against two stones. The phase-1 case is `t >= 2` against one, which
`LAW-OVERLOAD` does not say — so it is derived here from two things that do:
`DEF-T` makes `t` the minimum hitting set over the plan family, and `LAW-HIT`
makes hitting the ONLY defensive mechanism ("Kill = hit; no other defensive
mechanism exists"). With `left` stones remaining this turn and `t > left`, some
plan is unhit when our turn ends, and the attacker completes it on their next
turn. The two-stone instance is `LAW-OVERLOAD`; the general statement is
`LAW-HIT` + `DEF-T` + counting. A defending stone can only REMOVE windows from
the attacker's hot class — `ClassSet::of` returns the empty set the moment
`opp != 0` — so no defence outside the hitting set exists, and D-243's legality
lemma closes the unreachable-plan escape.

**Turn-wholeness, argued for the branch that actually fires.** Revision 3's
argument covered phase 0 only. At `Phase::Second` the firing node's parent P is at
`Phase::First` of the same turn and promotes a HALF-TURN line `[c_{p-1}]` — and
that line dies one ply above P and never reaches the root. A node promotes only on
`score > alpha`; for any candidate after the first, `child` returns the
null-window scan unchanged unless `alpha < scan < beta`, where it re-searches at
the real window — and that re-search node is a PV node, which cannot fire. So a
promotion not preceded by a full-window re-search implies `score >= beta`. P is
`Phase::First`, so P's parent G reaches it through the NEGATING `TurnComplete`
link where `beta_P = -alpha_G`; G therefore sees `-v <= alpha_G` and does not
promote. At the root, `beta = INFINITY` makes the fail-high branch unreachable, so
every improving root scan is re-searched at the full window.

**MEASURED**, with an assertion armed at every ply-0 promotion: depth 2, 2 619
phase-1 firings, **0 violations**, 24/24 bestmoves; depth 3, **433 542** firings,
`turns_from_plies` at every completed depth, **0 `PV_NOT_PLAYABLE`**.

That argument rests on machinery the deferred batch cut touches (`WPQ_seed.md`
§7.2), so it is not left as prose: the named invariant asserted at the return site is what would fire if a
later change broke it, and U2-T's test is built on a **phase-1** firing — a phase-0
firing is 0.0053 % of firings and a test author following revision 3's argument
would have built the case that never runs.

#### What the early return costs elsewhere, stated because revision 3 denied it

Revision 3 claimed the saving "is on the cheap majority … nodes that do not choose
the answer". A non-PV subtree's value propagates into the root's argmax.
**MEASURED**, depth 2, 24 corpus roots: the bestmove differs on **2 of 24** and the
completed-depth score on **3 of 24**, including a case where neither score is a
mate score — so this is not only §5.3's licensed shortening of mate distances on
lost positions. It belongs beside the other two axes on which the SPRT's seats
differ (**U3** §7).

#### The completed-window reading

Composed correctly by construction: the protocol runs only at nodes the search has
not already ended, so no node it runs at has a completed window for either side —
a `debug_assert` and a test (§3), not an assumption.

### 5.3 The four generation rows

| Row | Reached when | Generation |
|---|---|---|
| **WIN-NOW** | `can_win_this_turn(us, left)` is `Some` | **Exactly** the win-now class of §5.1: every `win_in_one_ply_cells(us)`, plus at `StonesLeft::Two` both empties of every own hot window at exactly four stones. No filter, no Tier T, no Tier Q, no batching |
| **FILTERED** | `None`, and `Cover::Minimal` | The union of cells over the inclusion-minimal covers, and nothing below it |
| **BATCHED** | `None`, and `NothingToBlock` | Tier T ∪ Tier Q, batched per the deferred schedule (`WPQ_seed.md`) |
| **BATCHED (lost)** | `None`, `Impossible`, and the node is a PV node or the root | Tier T ∪ Tier Q, batched per the deferred schedule (`WPQ_seed.md`), and counted. The position IS lost; the search reaches that through the generated moves because a PV node must return the line that proves its score |

**Tier F is either the WHOLE candidate set or empty**, which revision 3 stated
three different ways and defined nowhere. On the WIN-NOW row the set IS the
win-now class; on the FILTERED row it IS the cover union; on both BATCHED rows
`can_win_this_turn` is `None`, which at `StonesLeft::Two` forbids both a
win-in-one-ply window and a hot window at exactly four, and at `One` forbids the
former while §5.1 withholds the pair class — so the win-now class is provably
empty there. Two things follow and each closes a finding: "Tier F is exempt from the batch cut"
is automatic rather than a rule, and S-E's "forced prefix" has an unambiguous
subject.

**A third thing does NOT follow, and revision 4 claimed it did.** Revision 4 wrote
that the table's move can therefore be promoted to index 0 unconditionally,
"which preserves `Run::salvage`'s documented ground". That is a non-sequitur: Tier
F being EMPTY is exactly what puts Tier T first, not the table move. The failing
case — Tier F empty, Tier T non-empty, table move in Tier Q — is **MEASURED at
70.8 % of corpus roots**, the common case rather than a corner. §5.4 states the
one rule, and U2-Z item 20 records what it costs `Run::salvage`. **(Citation
disclosure, u-rev 2 / F1:** the superseded text cited "§12 item 6" here, which
never existed — §12 had five items at `6feb40a` — and the carve silently
repointed it to its actual referent, "U2-Z item 20" (superseded §15 item 20);
disclosed as this unit's third stated exception in the head.)

**Soundness of the WIN-NOW row.** At a node `k` turns from the root the best score
any move can reach is `mate_in(k+1)`, because a mate at that distance means
completing six this turn. So the win-now class is exactly the argmax set. It is
COMPLETE: every way to complete six this turn is a window that held five own
stones (one stone) or four (two stones), since a completing stone forms a
fully-own length-6 window and overlines contain one. `LAW-FORCE`'s antecedent
fails here, so the filter is not licensed and is not applied.

**Soundness of the FILTERED row.** `LAW-FORCE` is `[PROVEN]`: if the opponent has
≥1 plan **and the mover cannot win this turn**, every non-losing mover move hits
ALL opponent plans. Both conjuncts hold on this row by construction — the second
one established by the `None` arm rather than assumed, which is the repair
revision 3 made and revision 4 keeps.

**The two-ply realisation — ILLUSTRATION ONLY; "VERIFIED on the shipped solver"
is WITHDRAWN (u-rev 2 / F2).** `Cover::Minimal` carries SETS because the union
is provably insufficient. The load-bearing ground for that claim is D-257's own
abstract, position-free example, which needs no reachability claim at all:

> three hot windows with empties {a,b}, {b,d}, {d,e} have no one-cell cover…
> {a,e} …covers nothing in the middle

The board below is kept only as a concrete illustration of the same shape. It is
**not a position a legal game reaches**: "two disjoint sealed five-stone P1
rows" puts P1 at 10 stones, and CLAUDE.md rule 3 makes that impossible at any
turn boundary — `crates/pistol-core/src/rules.rs` fixes `FIRST_TURN_STONES = 1`
and `TURN_STONES = 2`, so P1's cumulative count after its *k*-th turn is
`1 + 2(k − 1) = 2k − 1`, always odd, never 10. The phase0/phase1 output below is
real solver arithmetic over that unreachable board, not a verification of
anything a legal game produces, and the claim it was offered to support does not
need it:

```
phase0 cover = Minimal([Two { first: (4,4), second: (5,0) }])
phase0 union = [(4,4), (5,0)]
phase1 cover after (4,4) = Minimal([One((5,0))])
```

D-243 (4)'s pairing obligation does not rest on this fixture; it is discharged by
D-257's position-free ground quoted above. The same two rows UNSEALED illustrate
`Cover::Impossible`'s 8-hot-window contrast — equally unreachable, for the same
reason — which is why the illustration seals both ends: an illustration that
forgot them would show the overload path while claiming to show the pairing
path.

**One licensed value change.** Under the filter the search no longer prefers the
longest resistance among losing moves. `LAW-FORCE` licenses it — those moves lose
— so mate distances on already-lost positions may shorten. ADR line U2-Z item 11.

### 5.35 Where the protocol sits in `visit`, and what the generator's signature is

Both were left for the implementer to invent. `pvs::visit` has four candidate
insertion points with materially different behaviour, and the MEASURED firing
counts in §5.2 are unattributable without one being named.

**The protocol runs AFTER the transposition probe and its cutoff, and AFTER the
`depth_plies == 0` horizon return.** After the horizon return, because before it
an overloaded leaf would return `-mate_in(k+2)` instead of `position.value()` —
that is a mate-detection EXTENSION at the horizon, which changes every node count
and is not in this WP's scope. After the probe, because a table hit is cheaper
than a threat query and because a node that fires would otherwise never take a
cutoff it is entitled to. It runs after `should_stop()`, like everything else.

**The generator's signature.** `pistol_search::staged` is public so S-E's
expensive half can drive it (**U4** §8.2), and its entry point must reach three things:
the board, the threat state, and a delta ranking. `Position` is `pub(crate)`
(`lib.rs:37`), so an entry point taking `&mut Position` would be unbuildable by
the very test it exists for. It therefore takes what it needs and no more:

```
pub fn staged_candidates(
    state:   &GameState,
    threats: &ThreatState,
    eval:    &mut dyn Eval,      // the delta seam, not a Position
    params:  StagedParams,
    out:     &mut StagedSet,     // cells, and the count that is forced
)
```

`visit` calls it through a `pub(crate)` accessor on `Position` that hands out
`(&GameState, &ThreatState, &mut dyn Eval)` together — one accessor, so the three
cannot be taken apart at a call site and drift, which is D-41's whole reason for
the seam existing.

**Two consequences, recorded rather than discovered.** The entry point puts a
`pistol-solver` type (`ThreatState`) in `pistol-search`'s PUBLIC API, which is a
permanent surface commitment made for a test — the trade D-115 refuses in
general. It is taken here because the alternative is the in-source cross-crate
comparison D-115 refuses more specifically, and because the generator is a
legitimate public capability rather than a test hook. And `candidate_cells`
remains public with a `(&Board, CandidatePolicy)` signature that cannot express
`Staged`; under `Staged` it can only answer about the quiet ball, so the public
function and the search's actual candidate set diverge. Named in U2-Z item 21.

### 5.4 Step 4 — staged generation

1. **Tier F** — never delta-ranked, emitted ascending `(q, r)`. Delta is not
   called for Tier F cells at all, which is what makes a filtered node cheap.
2. **Tier T** — `LAW-SUPPORT`-qualified per **U3** §6. Delta-ranked, stable sort.
3. **Tier Q** — the remaining cells of the `quiet_radius` ball, delta-ranked,
   batched per the widening schedule — **stage Q, DEFERRED; see `WPQ_seed.md`**.

**Architect ruling R5 (settled).** Tier Q stays in this unit's node protocol,
SPECIFIED BUT UNARMED: the D-scope WP-1.5b ships is stages F and T only, this
unit's protocol scope for Tier Q is unchanged by that scope, and the
pre-registration registers F+T only.

**The forced prefix and `ordering::order` — specified, because they conflict as
shipped.** `order` stable-sorts the WHOLE candidate vector by `Eval::delta` and
then rotates the table's move to index 0. Both would destroy a forced prefix.
Under `Staged` the generator therefore produces the vector already ordered and
`order` is **not called**: Tier F is emitted ascending `(q, r)` with no delta call
at all, Tiers T and Q are delta-scored and stable-sorted WITHIN their own ranges,
and the table's move is promoted only **within the tier it belongs to** — never
across a tier boundary, so the forced prefix stays a prefix and S-E's containment
assertion keeps its subject. **This is the one table-move rule; revision 4's
second one is withdrawn.**

**And it breaks a documented ground of shipped code, which is an ADR line and not
a footnote.** `Run::salvage`'s doc says "The first root candidate is the table's
move, which is the previous iteration's best, so a salvaged answer is never
worse-informed than the last completed depth's (WP-1.4's decision, verified
line-by-line by its decision-red-team)." Under `Staged` on a batched root that is
false: Tier T comes first. The salvage remains SOUND — it is still a completed
root subtree's exact score, which is the property `pvs.rs` proves — but the
"never worse-informed" claim rests on the ordering and must be restated for
`Staged`. U2-Z item 20 takes the line, and U2-T registers the test. A table move that is not a candidate is dropped, as
today. The deadline check `order` performs under a wall-clock stop moves into the
staged scorer at the same `ORDER_CHECK_INTERVAL` stride, so the play-mode
overshoot bound is unchanged.

**Tier F and Tier T are emitted whole, and the schedule counts QUIET CELLS ONLY.**
Revision 5 said this in the config shape (**U3** §10) and said something else here — that the boundary is
"counted from the END of Tier F" — which at a typical batched node would put ZERO
quiet cells in the first batch and truncate Tier T. The committed census
implements the quiet-cells rule (`t.len() + quiet.min(QUIET_TOP_K)`), so every
number in **U3** §6.2 assumes it, and the deferred schedule's registered
denominator (`WPQ_seed.md` §7.2) says "whose quiet
pool exceeded the first batch". **U3 §10's rule is the one.** Tier F is then exempt
from the cut automatically rather than by a further rule — §5.3 proves it is the
whole set or empty.

Tiers are disjoint by construction. Every staged cell is filtered through
`Board::is_legal_placement`, one cell at a time: D-243 proves every Tier F and
Tier T cell is inside rule 5's region, but D-77 forbids exactly the shortcut "the
radius is at most eight, so every policy cell is legal" and D-20 forbids
comparing the two radii. Because that proof says no Tier F cell can be dropped, a
Tier F cell the rules refuse is a **named refusal**, never a silent drop.

### 5.5 Step 5 — quiescence

Out of scope. WP-1.6's, with `LAW-RIPOSTE` and `LAW-LEDGER` (U2-Z item 8).

---

### 5.6 MATRIX M5 — how the overload verdict is realised — FELL

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| M5-A — early return, `!is_pv` gated, computed by its OWN query | Revision 3's | **MEASURED** `unblockable_double_threat` 49–101 ns — *the redundant half of a pair* | **FELL.** Strictly dominated by M5-E on the matrix's own cost axis |
| M5-B — early return everywhere, with a promoted WITNESS LINE | Fires at PV nodes too | **MEASURED** 22 PV firings against 455 177 non-PV over 14 roots at depth 3 — **0.005 %** more coverage | A line the search did not search, which `pvs.rs` refuses for the table cutoff. Rejected, now with its number |
| M5-C — no early return; let the search prove the loss | Delete step 2 | **MEASURED**: costs −17.27 % nodes at depth 2 and −47.25 % at depth 3, i.e. that is what the return buys | Correct and slower. Revision 3 rejected it with no number at all, which is the option-matrix clause's own finding |
| M5-D — store the verdict | A record beside the return | One store | Deferred. Its cell is corrected: it **CONTRADICTS** the deferred schedule's stated TT rule rather than "interacting" with it (`WPQ_seed.md` §7.2), since that rule forbids storing a bound from a set that was not exhausted and this one is from a set never generated. Also `Record.best` has no answer, and the store depth has two conflicting honest choices |
| **M5-E — ONE `can_win_this_turn` and ONE `blocking_covers`; step 2 realised as the `!is_pv` early return on the `Impossible` row** | §5.2 | **MEASURED** equivalence over 145 158 positions and 343 344 comparisons, 0 disagreements. **ESTIMATED** from **U3** §12 item 4's own MEASURED figures: −29.1 % / −41.3 % / −41.5 % of the registered per-node threat cost | Identical verdict, value, soundness and gate to M5-A; `PROTO-NODE`'s ORDER is preserved textually, and only the order of QUERIES changes — a query is not a step |

**ADOPTED: M5-E**, supplied by the DECISION-RED-TEAM that killed M5-A. That is
the fourth time in this work package's review round that the surviving option was
one the matrix did not contain (M0 (f), M2 W-E, M3 S-E, M5 E) — a base rate worth
recording beside D-276's single precedent.

**THE STRONGEST SURVIVING ATTACK**, recorded verbatim for the ADR line:

> M5-A's recommendation is priced on the wrong call and its value lives in a
> branch the matrix never names: `blocking_covers(us, b) == Cover::Impossible`
> and `unblockable_double_threat(them, b)` are the same predicate — provable from
> `cover.rs:145` and `cover.rs:177`, and MEASURED to agree on 145 158 playout
> positions and on 343 344 side-and-budget comparisons with zero disagreements —
> so §5.2 recomputes at every node the query and the `can_win_this_turn` that
> §5.3 pays for one line later, and the 49–101 ns the matrix quotes is the
> redundant half of a pair the design's own §12 item 4 (**U3**) already sums at 347 ns and
> calls 10.5 % of a fast node; and of the 455 201 firings MEASURED over fourteen
> corpus roots at depth 3, 455 177 are at `Phase::Second`, so restoring the
> `phase == First` conjunct revision 3 silently dropped takes the node saving from
> 47.25 % to 0.065 % and leaves the move played identical on all eight roots — the
> recommended option therefore stands or falls on a generalisation of
> `LAW-OVERLOAD` to `HitBudget::One` that §5.2's turn-wholeness paragraph does not
> cover, that the calculus states nowhere, and that the matrix does not mention,
> while the realisation it chose is strictly dominated by one the matrix does not
> contain.

Both halves are answered in §5.2 — the duplication by M5-E, the generalisation by
deriving it from `LAW-HIT` and `DEF-T` rather than attributing it to
`LAW-OVERLOAD`. What survives unrepaired is the base rate itself: four matrices in
one work package recommended an option a fresh context then dominated, and in
three of the four the dominating option cost less rather than more.


---

## U2-T. The tests this unit registers

Behaviour-named, calculus IDs in doc comments, and each states **what quantity it
watches** — the INTEG lesson every new fixture inherits. Carried from the
superseded §11; the rows this unit does not own are in U3-T, U4-T and
`WPQ_seed.md`, and no row is in two places.

| Test | Watches |
|---|---|
| `overload_at_entry_scores_loss_without_expansion` | the SCORE and the NODE COUNT: `-mate_in(k+2)` with no child expanded |
| `overload_check_is_not_taken_at_a_pv_node_or_the_root` | that the root's PV is non-empty on an overloaded root, and that the reported line is turn-whole where the check does fire |
| `overload_check_guarded_by_own_win_now` | the SIGN, on a position where both sides have an unhittable family and the mover can win now |
| `overload_composition_handles_completed_window_reading` | that no node reached by the protocol carries a completed window, and that a DECIDED position refuses via `StonesLeft::from_state` |
| `survival_filter_hits_all_plans_across_both_plies` | the generated CELL SET at phase 0 and again at phase 1, against an independently computed plan family |
| `defensive_union_covers_nonminimum_two_stone_splits` | the phase-1 set AFTER `a` — a flat-union generator offers both cells and never the pair, so the phase-0 set cannot discriminate |
| `mate_in_1_by_pair_generated_in_tier_f_not_ranked_in` | that both empties of the count-4 window are in the FORCED prefix, with an eval whose delta ranks them LAST |
| `the_table_move_ordering_under_staged_is_within_tier` | that a Tier-Q table move does NOT reach index 0 when Tier T is non-empty, and that `Run::salvage` still returns a completed root subtree's exact score — the property that survives, as against the ordering claim that does not |
| `new_plan_creation_gets_no_forced_slot` | the absence of a plan-creating non-hitting cell from the forced prefix, with its delta made maximal |
| `staged_ordering_deterministic_within_and_across_tiers` | the whole emitted order, twice, with equal delta scores forced |
| `stage_counters_reported_in_search_info` | each counter non-zero on a position built to fire it, zero on one built not to |
| `a_win_now_node_generates_only_the_win_now_class` | the emitted set on a `Minimal` node where the mover wins by PAIR — revision 2's STOP-2, where the natural reading generated no winning cell |
| `cover_impossible_at_phase_zero_still_generates_the_win_now_class` | the emitted set at a ROOT (always a PV node, so §5.2 cannot fire) with `Cover::Impossible` and a mover win available — revision 2's STOP-1 |
| `cover_impossible_at_phase_one_with_a_win_in_one_ply_cell` | that the win cell is generated even when it lies in no Tier-T window (the reviewer's R1 construction) |
| `stones_left_and_hit_budget_are_read_from_core_at_both_phases` | the budget at `Phase::Second`, which is the reachable class — turn 1 exists only at ply 0, always a PV node, on an empty board where the predicate is `false` at either budget, so revision 3's stated justification named an unreachable case |
| `the_protocol_runs_after_the_horizon_return_and_the_table_cutoff` | the node count and the table hit rate against a build with the protocol placed earlier — §5.35's placement, which is otherwise unattributable |
| `a_forced_row_emits_no_tier_t_or_tier_q_cell` | that Tier F is the WHOLE set on the WIN-NOW and FILTERED rows, which is what makes the batch cut structurally unable to touch it |
| `the_two_predicates_agree_everywhere` | `blocking_covers(us,b) == Impossible` against `unblockable_double_threat(them,b)` at ALL THREE budgets over built hard cases and seeded playouts, with a NON-VACUITY assertion that the `Impossible` branch is reached at each budget — M5-E's soundness, and a sweep that never reached a budget would agree there by not testing it |
| `a_radius_policy_search_is_byte_identical_to_the_committed_engine` | bestmove, nodes and PV under `Radius` before and after this WP — the scoping claim of §7.2 |
| `the_threat_state_stays_in_step_with_the_game_and_the_eval` | `THREAT_DESYNC` never fires across `place`/`undo`/`reset_to` over seeded playouts — the D-41 seam's third member |

---

## U2-M. What this unit measures

ADVISORY on this machine; the operator re-runs for the record. (A standing
condition of every measurement in every unit, stated per unit so a unit is
readable alone; it is a condition, not a datum.)

2. **Stage-share counters, and the seam by which anyone reads them.** `SearchInfo`
   gains `stages: StageCounters`, all zero under `CandidatePolicy::Radius`.
   **The line protocol does not carry them.** `report.rs` renders an explicit
   field list, `tools/baseline_snapshot.sh` parses the handshake and the `info`
   lines, and §2.1's claim that no protocol output changes is worth more than a
   convenient print. So the rates are produced by a COMMITTED harness in the
   pistol-search test tree that calls `Searcher::search` directly and reads
   `SearchInfo.stages` — the same shape as the census, named with its revision
   when it lands. Revision 4 registered the rates as a deliverable with no path
   by which the operator could ever see them.
   **And the handshake line must still say something.** `pistol.rs:131`
   destructures the policy to emit `id candidate_policy radius <n>`, which
   `tools/bench_delta.sh` guards and `baseline_snapshot_tests.rs` requires, and
   **U4** §9's `--config` puts it inside the snapshot's INVARIANT block. Under `Staged`
   it emits `id candidate_policy staged quiet_radius <n> quiet_top_k <k>` — one
   line, whitespace-delimited, multi-token value, which is the form D-230 records
   the record already tolerating (`id budgets depth_turns nodes`). `SearchInfo` has THREE construction sites in
   `search.rs` — the completed-depth report, the `PartialRoot` salvage and the
   `Fallback` answer — and the last two zero most fields today. The counters are
   WHOLE-SEARCH totals like `nodes`, so they are written from the `Run` at the
   same point `nodes`, `nps`, `time_ms`, `seldepth_turns` and `hashfull_permille`
   are, i.e. on every path including the two salvage ones. Stated because
   revision 2 did not say, and a counter that silently reads zero on the
   wall-clock paths would make the play-mode stage shares unreadable.
   The rates: F/T/Q firing rates, the filtered-node rate, the
   `Cover::Impossible` rate and the overload-return rate. **The widening rate
   per node class — with its denominator, depth and `tt_bytes` — and the TT
   entries the truncation rule declines to store are stage Q's quantities and
   DEFER with it** (`WPQ_seed.md` §7.2); the counter seam described above is
   what a later WP reads them through, so the seam is not deferred.

### Cost

This unit has no governed run. Its IMPL's own rule-5 bench is OPEN (below), and
what that bench costs cannot be stated before the baseline it is set from is
taken.

---

## 14. Non-goals

Adopted verbatim: no quiescence (WP-1.6); no killers/history/countermove
(WP-1.7); no df-pn (WP-1.8); no eval terms from `t` or `τ`; no dominance pruning
beyond the staged scheme; no `LEGAL_RADIUS` change; no ball-scan optimisation; no
`pistol-eval` refactor.

---


## U2-Z. ADR lines this unit owes, the handoff it carries, and what is OPEN

### ADR lines

Carried from the superseded §15. Its item numbers are retained exactly so an
existing cross-reference to "§15 item n" still resolves; this unit invents none
and renumbers none. Listing them in ascending order here dissolves MINOR 19
mechanically — the superseded list ran 1–14, 16, 17, 18, **15**, 20, 21, 22, 23,
**19**, so every cross-reference by number had to be resolved by search.

The superseded §15's PREAMBLE does not travel. MAJOR 10 measured it false on both
of its clauses (it said twenty-three items were nine, and said "recorded rather
than fixed" of an item that said "Fixed here"). Each unit writes its own lead-in
instead, and this is U2's: **the eleven lines below are this unit's own; none is
a correction to a landed line, and none has landed.**

5. **M5-E**, the overload realisation — one `can_win_this_turn` and one
   `blocking_covers` per node, step 2 realised as the `!is_pv` early return on the
   `Impossible` row — with §5.6's strongest surviving attack. It records the
   PHASE decision explicitly and the generalisation of the overload verdict to
   `HitBudget::One`, derived from `LAW-HIT` + `DEF-T` rather than attributed to
   `LAW-OVERLOAD`, which states only the two-stone instance. M5-B and M5-D
   registered as not-taken, M5-D's cell corrected to "contradicts §7.2's stated
   rule".

6. **The `Position` seam gains a third member.** D-41's content is that
   `GameState` and `Eval` move from one place BECAUSE they are only correct
   together; `Option<ThreatState>` joins them, and `THREAT_DESYNC` is the same
   failure class as `POSITION_DESYNC` and `EVAL_DESYNC`. An amendment to D-41,
   with the desync test §11 now registers.

8. `LAW-RIPOSTE` and `LAW-LEDGER` hand off to WP-1.6; `THM-WINDOW`'s exact-`t`
   eval terms registered; the Tier-Q ball scan stands (WP-1.5c); the fallback
   under Staged reuses the `quiet_radius` ball and `SearchError::NoCandidates`
   gains a policy-agnostic shape; D-295's `RULE-EXACT` residual (WP-1.10).

10. WP-1.5b does not complete ROADMAP WP-1.5's supersession — the operator's SPRT
    does.

11. **The licensed value change**: mate distances on already-lost positions may
    shorten under the filter (§5.3). It changes what a printed score means
    (D-3, D-72) and is attributable here rather than to an unnamed line.

17. **`pistol_search::staged` is public**, because S-E's expensive half is a test
    over the generator and D-115 forbids widening `pvs` to reach `Run`.

18. **The `!is_pv` gate's root safety is an `assert!`, not a sentence.** "Ply 0 is
    always a PV node" is a property of today's construction; Stage 4's aspiration
    windows would narrow the root. Named invariant at the return site.

19. **Rule 9 and `pvs.rs`.** The file is 552 lines and carries a
    `RULE9-JUSTIFICATION` ending "Stage 1 moves candidate generation out entirely
    (D-117, WP-1.5)". This WP adds the node protocol and the batch loop INTO
    `visit` while moving generation OUT to a new `staged` module, so that
    justification's forward-looking clause is discharged and its text must be
    re-read at the landing commit rather than left asserting a future that has
    arrived.

---

20. **`Run::salvage`'s documented ground does not hold under `Staged`.** Its doc
    says "the first root candidate is the table's move … so a salvaged answer is
    never worse-informed than the last completed depth's"; under `Staged` on a
    batched root Tier T comes first. The salvage stays SOUND — it is a completed
    root subtree's exact score — but the ordering claim must be restated.
    **AND IT APPEARS TWICE.** The second site is `search.rs`'s comment at the
    resolution point — "the aborted iteration's completed root prefix where one
    exists (it starts from the table's move, so it is never worse-informed than
    the last completed depth)" — which is not a doc comment but the entire
    justification for PREFERRING `PartialRoot` over `CompletedDepth`. Under
    `Staged` that preference may hand back an answer worse-informed than a depth
    already completed: a play-mode BEHAVIOUR change, not a doc drift. Amends
    D-207, whose own flip clause ("flips when WP-1.5's threat-first generation
    shrinks the first iteration to a rounding error") is dispositioned here as
    NOT YET FIRED — the first iteration is not a rounding error at the measured
    reductions.

21. **`pistol_search::staged`'s entry point and `candidate_cells`' divergence.**
    The public entry point takes `(&GameState, &ThreatState, &mut dyn Eval,
    StagedParams)`, putting a `pistol-solver` type in `pistol-search`'s public
    API; and `candidate_cells` keeps a signature that cannot express `Staged`, so
    the public function and the search's real candidate set diverge under it.

22. **`docs/research/threat_calculus_v1.md` gains the generalised overload
    verdict.** MEASURED, 455 177 of 455 201 firings are the `t >= 2` / one-stone
    form, which `LAW-OVERLOAD` does not state — the engine's mate scores would come
    almost entirely from a statement the theory source of record does not contain.
    The calculus's own footer makes a new law an ADR line, and D-266 makes that
    file the source. The derivation from `LAW-HIT` + `DEF-T` is in §5.2; the
    amendment is owed there, not only here.

### The handoff this unit carries to WP-1.6 (threat-only zone-bounded quiescence)

- **`LAW-RIPOSTE` and `LAW-LEDGER` are yours** (§3 rows 4 and 5). The riposte
  check on every forced reply is what makes a forcing-line prover sound; the
  ledger's `2 − t` bank is what decides whether a `t = 1` chain terminates.
- **The node protocol's shape is settled and attacked** even though unimplemented:
  win-now before overload before filter, one `can_win_this_turn` and one
  `blocking_covers` per node, and the identity that makes that possible
  (`blocking_covers == Impossible ⟺ unblockable_double_threat`) is verified over
  168 030 comparisons with the `Impossible` branch reached at every budget.
- **The generalised overload verdict (`t > left`, either phase) is derived from
  `LAW-HIT` + `DEF-T` and the calculus amendment is OWED** (item 22 above). A
  quiescence that reuses it inherits that debt.
- **Quiescence stands pat and extends in TURNS, never plies** — D-111, unchanged.
- **What WP-1.6 must not inherit:** this text is UNREVIEWED at this u-rev (u-rev
  3, this repair). Five rounds failed before the restructure, and the carve
  itself has now been attacked twice post-carve — a CONFIRMATION PASS at u-rev 1
  (`docs/experiments/wp15b_U2_REVIEW.md`, pinned `38f21b9`) returned **FAIL**, 1
  MAJOR + 1 MINOR, repaired at u-rev 2; a second CONFIRMATION PASS at u-rev 2
  (`docs/experiments/wp15b_U2_REVIEW_urev2.md`, pinned `56b0bec`) again returned
  **FAIL**, 1 MAJOR (F3) + 1 MINOR (F4), repaired here. u-rev 3 has not yet been
  reviewed.

### The conservative branch this unit records

- **WP-1.5b does not complete `docs/ROADMAP.md` WP-1.5's supersession**, and the
  ROADMAP changes only by ADR — **item 10 above, and item 9 in `WPQ_seed.md`**.
  (The superseded §18.3 cited "§15 item 6" for this, which is the `Position`
  seam. MINOR 17; retargeted.)

### OPEN — carried forward, not closed by the carve

- **RULE 5 IS UNDISCHARGED FOR THE NODE PROTOCOL ITSELF** (revision-7 review
  MAJOR 9, and the superseded §17's own list). D-263 pre-registered the hotspot
  precisely so the first per-node caller would not discover it, and its flip
  clause is "Flips when WP-1.5b measures it". §12 item 4 (**U3**) measures the two
  queries' cost and registers a bracket for **Tier-T cell extraction**, taking
  item 7's ADR line for the substitution — which is the registration doing its
  job — but the change that puts `can_win_this_turn` and `blocking_covers` on
  EVERY NODE still has no expected-gain bracket, no abort threshold and no
  IQR-gated bench reporting nps AND time-to-depth. **The carve does not close
  this and does not narrow it.** It is a rule-5 registration the architect must
  place before U2's IMPL, not a repair a carve may write.
- **No REVIEW-design has run against this text at this u-rev** (U2-A).

---

*U2, u-rev 3. A repair, answering `docs/experiments/wp15b_U2_REVIEW_urev2.md`'s
FAIL against u-rev 2. IMPL has not started.*
