# REVIEW-design (SCOPED, fifth reviewer) — `docs/wp16_quiescence_design.md` revision 5 (WP-1.6)

**Revision reviewed:** `96c856c05603438a95970c67ee1832a61a0a0700` (`96c856c`),
"docs(wp16): revision 5 -- fix a defect revision 4 introduced, D-393".
**Matches HEAD:** YES — `git rev-parse HEAD` = `96c856c05603438a95970c67ee1832a61a0a0700`.
Working tree clean at review start; this report is untracked and uncommitted.

**Reviewer:** fresh-context, FIFTH reviewer of this document. I did not review
revisions 1–4. I read `wp16_design_REVIEW_rev4.md` in full first, as the dispatch
instructed, to learn exactly what F1 and N3/N4/N5/N6 were, then read revision 5's
document directly (not only the diff) and re-derived every claim below against the
tree rather than against either document's prose.

**This is a SCOPED review**, matching the size of the delta. IN SCOPE, and nothing
else:

1. §3.5's "No ply-2 win check" replacement — the unreachability PROOF that replaces
   revision 4's runtime check, and whether deletion (vs. the rev-4 reviewer's
   alternative (b), keep-the-check-and-place-the-witness) loses coverage.
2. N3's fix — Tier 1's union sorted after concatenation, against `tier_t_union`.
3. N4's fix — Tier 2 iterated in ascending `(q, r)`, and the worked example.
4. N5/N6's restated soundness paragraph — the max-node under-claim, and route (2)'s
   invariance claim.

OUT OF SCOPE and not re-reviewed (revision 5 does not touch them and the fourth
review passed them): §3.5a's existence/totality/query-composition and determinism
arguments, the `is_pv`/B2 correction's own soundness, §6 items 1–3, §9, §5, §3.7's
calculus citations, §3.1–§3.4/§3.6. I confirmed by `git diff 82de3e4 96c856c --
docs/wp16_quiescence_design.md` that the delta touches only the header, §3.5's
opening paragraph, §3.5a's trigger paragraph, §3.5a's Tier-1 sort paragraph, Tier 2,
the soundness paragraph, and §10 — nothing else moved, exactly as §10 claims.

**Sources read directly:** `docs/wp16_quiescence_design.md` (whole);
`docs/experiments/wp16_design_REVIEW_rev4.md` (whole);
`crates/pistol-solver/src/{query.rs,sets.rs,cover.rs}`;
`crates/pistol-search/src/{pvs.rs,staged.rs,position.rs}`;
`crates/pistol-core/src/{axis.rs,coord.rs}`.

---

## VERDICT: **PASS** — the design gate is clear; WP-1.6 proceeds to IMPL.

All four scope items pass. The proof in §3.5 is correct and I re-derived it
independently; deletion is the right disposition and is strictly safer than the
alternative; N3's and N4's fixes are correct against the code they cite; and both
halves of the restated soundness paragraph — the max-node under-claim and route
(2)'s invariance — re-derive from the code exactly as stated, the second one being
even a little stronger than the document claims.

Five precision residuals are recorded below (R1–R5). **None is blocking**, none moves
a number, none licenses a path IMPL cannot build, and none is a normative
contradiction. R1 and R2 are one-clause wording repairs inside the proof itself and
are the only two I would bother making if this document is opened again; R3 is a
misattributed `file:line`; R4 and R5 are stale referents and a §10 overstatement.

---

## Scope item 1 — §3.5's "No ply-2 win check" proof: **PASS**, re-derived from scratch

**The proof holds.** Re-derived against `query.rs` without using the document's or
the rev-4 review's steps:

- `can_win_this_turn(side, left)` (`query.rs:231–266`) returns `Some` on exactly two
  routes: (i) `win_in_one_ply_windows(side)` non-empty — checked first and at every
  budget; (ii) only when `left == StonesLeft::Two` (the `if left == StonesLeft::One
  { return None; }` guard at `:245–247`), a hot window whose
  `masks(window).own_count(side) == 4`.
- `Class::WinInOnePly` is `own == WIN_IN_ONE_PLY` (= 5) and `Class::Hot` is
  `own >= HOT_MIN` (= 4), both conjoined with liveness `opp == 0`
  (`sets.rs:145–166`); the `const` assertion at `sets.rs:86–90` pins
  `WIN_IN_ONE_PLY == HOT_MIN + 1`.
- Therefore `None` at the gate, at `StonesLeft::Two`, gives BOTH: no live window of
  `us` at `own == 5`, and no live window of `us` at `own == 4`.
- A single ply-1 stone raises the own-count of each window containing its cell by
  exactly one, and of every other window by zero. Liveness is monotone in the right
  direction here: `us` placing a stone cannot make a dead window live (`opp` never
  decreases; stones are never removed), and cannot lower any own-count. So every live
  window of `us` at the ply-2 node was live at the gate with own-count one lower or
  equal.
- Hence at the ply-2 node no live window of `us` reaches `own == 5`: it would have had
  to be at `own == 4` at the gate, which the gate's `None` excludes. So
  `win_in_one_ply_windows(us)` is empty there, and
  `can_win_this_turn(us, StonesLeft::One)` takes route (i) never and route (ii) never
  (guarded off at `One`) — it returns `None`, always. ∎

**The proof's premise that the gate ran at `StonesLeft::Two` also checks out**, and is
in fact over-determined: `visit`'s `depth_plies == 0` branch carries a `debug_assert!`
that the horizon lands at `Phase::First` (`pvs.rs:211–216`, `STATIC_EVAL_MID_TURN`),
§2 states the same as the design's own invariant, and — independently of both — a
`Phase::Second` node of `us` EXISTS only in a turn that owes two stones, so any gate
that has a ply-2 node below it was necessarily at `StonesLeft::Two`. The only gate
that could read `StonesLeft::One` is a turn-1 gate (rule 3's one-stone turn), whose
ply-1 stone ends the turn and produces no ply-2 node at all. The chain case
(`q_depth_turns > 1`) does not escape either: §2's "once granted it completes as a
whole turn (both plies) before the next gate decision" and §3.7's "re-run at the new
position by the recursion" both put every granted turn behind its own gate, which
re-runs trigger (a) for the side to move.

**Deleting the check loses nothing, and is strictly safer than the alternative
(b).** Three separate ways of asking the question, all agreeing:

1. *Today, at runtime:* the branch is provably unreachable, so keeping it (fixed) and
   deleting it are behaviourally identical — same nodes, same scores, same PV. The
   choice is a documentation choice, and a recorded proof is worth more than a guard
   whose test can never be true.
2. *Under a hypothetical future relaxation of the gate's trigger (a)* — the scenario
   the rev-4 review used to argue the clause was dangerous — the two options fail
   DIFFERENTLY, and deletion fails better. With the clause deleted, a ply-2 node that
   could win would simply not have the winning cell generated (it is the empty of an
   `own == 5` window; neither `Cover2::cells()` nor `cells_raising_to_hot(us,
   NearHot::Three)` — empties of `own == 3` windows — produces it), so the search
   would miss the win and report a value below the truth: an UNDER-claim, the error
   direction §3.7 has already registered as the one this design tolerates. With
   revision 4's clause as written, the same relaxation produces a `Phase::Second`
   return with no stone placed and a mate score — `PV_NOT_PLAYABLE` at
   `pv.rs:101–105`, i.e. a panic on the most-likely-to-be-best line. Deletion degrades;
   the broken clause detonates. Alternative (b) (place the witness, then score) would
   also be correct, but it is dead code carrying a live cost, and it would have to
   restate the "place, then score" contract a second time.
3. *Coverage:* nothing else in the design depended on the check. §3.5a's Tier-1 bullet
   cites it for "`win_in_one_ply_cells(us)` is PROVABLY EMPTY", and that claim survives
   unchanged — it is now discharged by the same proof rather than by a runtime check
   (see R4 for the stale wording).

The document's own characterization of the trade ("Recording the proof rather than the
check is both correct and more informative") is one I confirm.

**R1 (precision, one clause, inside the proof).** "Ply-1 raises at most one window's
own-count by one" is, read literally, false: a stone lies in many windows (up to three
axes × six offsets) and raises each of them by one. The load-bearing statement is the
per-window one — *any* window's own-count rises by at most one — which is what the very
next clause ("so after it the maximum reachable is `own == 4`") actually uses, and which
is what the rev-4 review wrote. The conclusion is unaffected; but this paragraph is
presented as a proof, and a proof should not contain a sentence a careful reader must
silently repair.

**R2 (precision, one clause, inside the proof).** "which means `us` holds no live
window at `own >= 4` at all" is slightly stronger than trigger (a)'s `None` alone
delivers. `ClassSet::of` puts `own == 6` in BOTH `Hot` and `Completed`
(`sets.rs:156–164`), and `can_win_this_turn`'s hot scan tests `own_count == 4`
exactly (`query.rs:253`), so a live `own == 6` window would not be found by the query.
It is excluded instead by the game being over (rule 2/4 — such a position is decided,
`StonesLeft::from_state` answers `None` there, and no gate is reached). Nothing in the
conclusion depends on it — an `own == 6` window cannot become `own == 5` — so the proof
is sound as it stands; the sentence just claims one thing more than the cited query
proves.

## Scope item 2 — N3's fix (sort the Tier-1 union after concatenation): **PASS**

**`tier_t_union` really is structured the way the document says.** `staged.rs:294–308`:
it fills `cells` from `tier_t_side(us, …)`, fills a separate `opponent` vector from
`tier_t_side(us.opponent(), …)`, then `cells.extend(opponent); cells.sort_unstable();
cells.dedup();` — concatenate, then sort, then dedup, over inputs that were each
already sorted and deduped by `tier_t_side` itself (`staged.rs:332–333`). Both
functions exhibit the pattern; the document's description of it is accurate.

**The stated fix produces a correct set.** `Coord`'s derived ordering is lexicographic
by `q` then `r`, and `coord.rs:29–39`'s own doc says so and names it "the same total
order … the search uses as its final tie-break (docs/decisions.md D-5, D-7)" with
"Field order is therefore load bearing." So `sort_unstable()` puts the concatenation
into ascending `(q, r)`, which makes every duplicate adjacent, which is exactly the
precondition `Vec::dedup` needs. The result is a fully deduplicated, ascending-`(q,r)`
set. `sort_unstable`'s instability cannot affect determinism here: two `Coord`s that
compare equal are the same value, so no observable ordering depends on which copy
survives — and `dedup` removes the copies anyway.

The diagnosis the document gives for revision 4's wording is also right: each query
CLEARS its own `out` (`query.rs:10–12`, verbatim), each sorts and dedups only its own
buffer (`fill_empties`, `query.rs:268–276`), so a union of eight such buffers is
neither sorted nor deduplicated, and a "already sorted, just iterate" reading would
give call-order (deterministic, but a different tie-break winner) plus a `dedup` that
misses every non-adjacent duplicate — and duplicates across these eight sets are the
normal case, not a corner one (`win_in_one_ply_cells ⊆ threat_cells` for either side,
since `own == 5` is in both classes).

**R3 (citation, non-load-bearing).** The document attributes the pattern to
`tier_t_union` but cites `staged.rs:331–333`, which is inside `tier_t_side`
(`out.extend_from_slice(&scratch); out.sort_unstable(); out.dedup();`).
`tier_t_union`'s own sort/dedup is `staged.rs:304–306`. The pattern is identical in
both, so nothing normative rides on it; the rev-4 review made the same slip and the
document inherited it. If it is corrected, cite BOTH: `tier_t_side` (`:321–334`) is
in fact the more instructive precedent for IMPL, because it also shows the one thing
the document's prose leaves implicit — a single reusable `scratch` buffer whose
contents are copied out with `extend_from_slice` after EACH query, which is precisely
what the "every query clears its own `out`" contract forces. Following
`tier_t_union` alone (two vectors, one `extend`) does not model the eight-call case.

## Scope item 3 — N4's fix (Tier 2 iterated in ascending `(q, r)`): **PASS**

**The worked example is arithmetically correct.** `NEIGHBOUR_DIRECTIONS`
(`axis.rs:70–77`) is, in source order, `(1,0), (1,-1), (0,-1), (-1,0), (-1,1),
(0,1)` — the document's ring-order list, exactly. Sorting those same six offsets
lexicographically by `(q, r)`: `q = -1` gives `(-1,0)` then `(-1,1)`; `q = 0` gives
`(0,-1)` then `(0,1)`; `q = 1` gives `(1,-1)` then `(1,0)` — i.e. `(-1,0), (-1,+1),
(0,-1), (0,+1), (+1,-1), (+1,0)`, the document's second list, exactly. The two
sequences are different (they disagree from the first element onward), so the
document's claim that the choice is observable on a tie is correct, not a
hypothetical.

**And the example transfers to the actual candidates, which is the step the document
does not spell out but which holds.** Tier 2 sorts CELLS (`ply1_stone.offset(d)`), not
offsets. `Coord::offset` is a pure translation that panics rather than wraps on
overflow (`coord.rs:72–78`, `COORD_OVERFLOW`), and lexicographic `(q, r)` order is
invariant under translation: `q₁ < q₂ ⟺ q₁+a < q₂+a`, and on the tie `q₁ = q₂` the same
holds of `r`. So the ascending order of the six translated cells is the ascending order
of the six offsets, and the occupancy filter (`board.rs:90`) only deletes members
without reordering the survivors. The worked example is therefore a valid statement
about the candidate set and not merely about the direction constants.

**Ascending `(q, r)` is the codebase's tie-break convention, on three independent
statements of it:** `coord.rs:29–33` (the derived order IS the search's final
tie-break, D-5/D-7); `query.rs:3–5` (every query returns cells "in the derived
`(axis, start)` / `(q, r)` lexicographic order the protocol and the search tie-break
already use"); `staged.rs:336–339` (`delta_rank`: "a stable sort by score leaves
equal-scoring cells in the ascending coordinate order they arrived in"). Specifying
ascending `(q, r)` for Tier 2 makes Tier 2, Tier 1, `delta_rank` and the ordering
module all one rule, which is what Ruling 3's "fixed coordinate-order tie-break"
says. Confining `NEIGHBOUR_DIRECTIONS`' ring order to "enumerate the six candidate
cells, never as the tie-break order" is the correct division: the pinned ring order
still does the job its own doc claims (a fixed, non-drifting enumeration), it just is
not the comparator.

I also checked that the document's scan rule is equivalent to the cited precedent:
scanning an ascending-`(q,r)` list and replacing the running best only on STRICT
improvement yields the highest-scoring cell, ties broken by least coordinate — which
is exactly what `delta_rank`'s stable descending sort over an ascending input puts in
position 0. Same winner, no new mechanism.

## Scope item 4 — N5/N6's restated soundness paragraph: **PASS**, both arguments re-derived

### (a) The max-node under-claim — holds, and the `LEM-MONO` demotion is correct

The claim needs two things, and both check out against the code:

1. *The ply-2 node is a MAX node for `us` in the same sign convention as its parent.*
   Confirmed at `pvs.rs:323–325`: `PlyOutcome::TurnContinues` calls
   `self.child(..., same_side = true)`, and `child` (`pvs.rs:387–410`) applies
   `visit(depth, alpha, beta, ply)` with NO negation when `same_side` — "The same side
   owes another stone: same window, no flip." The node's loop then keeps the maximum
   over its candidate cells (`pvs.rs:336–338`). So the ply-2 node maximizes over the
   mover's second-stone choices, in the mover's own sign.
2. *The completion stone is a member of the true legal move set at that node.* Tier-1
   cells are empties of live windows (`fill_empties`), hence empty by construction and
   within hex distance 5 of a stone of that window, hence inside rule 5's radius-8
   region; Tier-2 cells are at distance 1 from the stone just placed and are filtered
   through `Board::is_occupied`. (This is §3.5a's totality argument, which the fourth
   review already verified; I re-checked only the membership consequence I needed.)

Given both, `value({c}) ≤ max over the full legal set` is ordinary max-over-a-subset
arithmetic — an under-claim for the mover at that node, unconditionally. No premise
about zugzwang, tempo, or stone monotonicity enters anywhere. The document's demotion
of `LEM-MONO` to context is therefore right, and its stated reason is right too: a
zugzwang-freedom premise is what you need to compare a placed stone against NOT
placing one, and rule 3 (a turn is two stones) makes that comparison unavailable
in this game, so the lemma was never carrying the weight. Keeping the citation as
context costs nothing and preserves the calculus trail.

### (b) Route (2)'s invariance — holds, and is if anything stronger than stated

Re-derived from `blocking_covers` (`cover.rs:201–245`) rather than from the
document. `Cover::Impossible` at `HitBudget::One` is returned on exactly two routes,
both of which give the same thing:

- `three_pairwise_disjoint_families` (`:213–215`): three hot-window empty-families
  pairwise disjoint, which no single cell can meet more than one of; or
- `minimal.is_empty()` after the one-cell scan (`:220–224, :240–242`): no cell `at` in
  the universe satisfies `covers(&families, at, None)`.

In both cases, and in both, `windows = hot_windows(us.opponent())` was non-empty
(`:202–205`). So for EVERY cell `c` — whether or not it is in the universe; a cell
outside the universe lies in no hot window's empties at all — some opponent hot window
`W` exists with `c ∉ empties(W)`. Since `c` is empty (both tiers filter to empty
cells), `c ∉ empties(W)` means `c ∉ W`. Therefore `W` receives no stone of `us`, stays
live (`opp == 0` for the opponent's reading is unchanged), and stays at own-count ≥ 4.

The opponent then moves with two stones against a live window holding ≥ 4 of their own
in a length-6 window: at own 4 it has exactly two empties, at own 5 exactly one — in
either case fillable within one turn, and every such empty is legal (distance ≤ 5 from
a stone of `W`). So the opponent completes six on their very next turn, whichever
completion stone `us` played.

And `us` cannot pre-empt it: the completion stone cannot itself win, because a winning
single stone must be the empty of a live `own == 5` window and item 1's proof shows no
such window exists at this node. So the position's value is not merely bounded by
`-mate_in(turns_from_root + 2)` — it IS that, exactly, for every legal completion
stone. The document's "not merely a lower bound — it is INVARIANT over the choice of
completion stone" is correct, and the one-sentence argument it gives (`LAW-OVERLOAD`'s
criterion via `blocking_covers`) is the right one-sentence argument.

I also checked the distance arithmetic while I was here, since the invariance claim is
about a specific number: `turns_from_root()` is `state.turn() - root_turn`
(`pvs.rs:429–431`) and both plies of a turn share it, so at the ply-2 node it reads the
same `t` a `Phase::First` gate would; our turn completes at `t+1` and the opponent's
overload win lands at `t+2` — the same convention `visit`'s own `OverloadReturn`
already uses (`pvs.rs:277–279`). Consistent.

**R5 (§10 overstates one half of what was folded in).** §10 lists "N5/N6" as folded
into the soundness restatement. N5 is genuinely fixed — the false "the extension was
granted because `Cover`/`Cover2` was not `Impossible`" parenthetical is gone, replaced
by an explicit route (1) / route (2) split. N6 is fixed only in its first half (the
real load-bearer is now named as max-over-subset rather than as an analogy). N6's
actual flagged sentence survives verbatim at the paragraph's end: "never the direction
that would corrupt a proof or a bound the rest of the tree relies on". Under negamax a
node-local under-claim at odd ply-distance from the root arrives at the root as an
over-claim, so that sentence is still stronger than the node-local argument above it
supports. This is NOT blocking and NOT new to this delta: the fourth review classified
it the same way, noted it applies equally to §3.3's offensive ply-1 restriction, and
identified §3.7's registered stance ("Quiescence is not a prover — it never claims a
proof") as the design's standing and correct answer to the whole class. I record only
that §10's changelog reads as if the sentence had been repaired, and it was not.

**R4 (two stale referents, wording).** Revision 5 deleted the ply-2 win-check but two
sentences still point at it as an existing check: §3.5a's Tier-1 bullet — "for `us`
this is PROVABLY EMPTY by the win-check just above" (the referent is now §3.5's PROOF,
which discharges the claim just as well) — and the soundness paragraph's "neither the
win-check just above nor the completion pick can change that". Both claims remain
TRUE; only their referent no longer exists. Worth two words if the document is opened
again, because "the win-check just above" invites an IMPL reader to go looking for a
check the same revision removed.

---

## Out-of-scope observations (recorded, not blocking, no normative contradiction)

- **O1 (carried forward, unchanged).** The rev-3/rev-4 `MAX_PLY` finding (`MAX_PLY =
  130` against up to `2 × q_depth_turns` extra plies past the horizon, and
  `SearchInfo::seldepth_turns`' doc asserting Stage 0 has no extension past the
  horizon) is still open and still deliberately untouched, as §10 says. It remains the
  carried-forward item most likely to bite Phase 2's first deep iteration, and it is
  cheaper to answer before IMPL than during it.
- **O2 (rev-4's N1, untouched by design).** `win_in_one_ply_cells(side) ⊆
  threat_cells(side)` for either side (`own == 5` is in both `Hot` and `WinInOnePly`,
  `sets.rs:156–161`), so Tier 1's eight calls could be six. §10 explicitly leaves N1 as
  optional polish; recorded only so the eight-call count is not later read as
  load-bearing.
- **O3 (a rev-4 note that DID land).** §3.5 now cites `position.rs:143` alone for the
  incremental threat update — the stale `position.rs:75` (a doc-comment line, rev-4's
  O3) is gone. Verified: `position.rs:143` is `threats.apply(at, mover)`.
- **O4.** `position.rs:130–133` (`static_score_after`) and `staged.rs:336–339`
  (`delta_rank`) both resolve exactly as cited; `axis.rs:64–77` covers the
  `NEIGHBOUR_DIRECTIONS` doc plus const; `query.rs:10–12`, `:268–276`, `:231`,
  `cover.rs:201–244`, `pvs.rs:194,341` all resolve. The only citation that lands in
  the wrong function is R3's.

**No STOP-3-class normative contradiction found, in scope or out.** Nothing in this
delta contradicts a pinned game rule, the threat calculus, or an ADR.

---

## What I explicitly verified, and which should not be re-litigated

1. `can_win_this_turn(us, StonesLeft::One)` is `None` at every ply-2 node this design
   reaches — derived from `query.rs:231–266` and `sets.rs:145–166`, not from the
   document.
2. The gate's `StonesLeft::Two` premise is over-determined (the design's §2 invariant,
   `pvs.rs:211–216`'s assert, and the structural fact that a `Phase::Second` node
   exists only in a two-stone turn), and holds for chained turns too.
3. Deleting the check is behaviourally identical to fixing it today, and fails in the
   tolerated direction (a missed win, i.e. an under-claim) rather than in
   `PV_NOT_PLAYABLE` under any future relaxation of trigger (a).
4. `tier_t_union`/`tier_t_side` are concatenate-then-sort-then-dedup; `Coord`'s order
   is lexicographic `(q, r)`; sort-then-dedup yields a complete, ascending,
   duplicate-free set.
5. Ring order and ascending `(q, r)` over `NEIGHBOUR_DIRECTIONS` are the two sequences
   the document prints, they differ, and the ordering transfers from offsets to
   translated cells.
6. The ply-2 node is a max node in the mover's own sign (`same_side = true`, no flip),
   so restricting its candidate set to one legal member is an under-claim by
   max-over-a-subset; `LEM-MONO` is genuinely not load-bearing.
7. Route (2)'s score is invariant over the completion stone, and is exact rather than
   merely a bound: some opponent hot window survives every single cell, and the
   completion stone cannot win first.

## What, if anything, would be worth changing

Nothing that blocks IMPL. If §3.5/§3.5a is opened again for another reason:
**R1** ("any window's own-count by at most one", not "at most one window's"), **R2**
(the `own == 6`/`Completed` qualification on "no live window at `own >= 4` at all"),
**R3** (cite `staged.rs:304–306` for `tier_t_union`, and `:321–334` for the
scratch-buffer pattern IMPL actually needs across eight calls), **R4** (two stale
"the win-check just above" referents), **R5** (§10 should say N6 was folded in only in
part). Each is one clause; none changes a claim, a number, or a code path.

*Report written by the fresh-context, fifth-slot, SCOPED REVIEW-design subagent
against `96c856c`. Left uncommitted for the orchestrating session.*
