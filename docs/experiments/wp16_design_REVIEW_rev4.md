# REVIEW-design (Phase 1'', SCOPED, fourth reviewer) — `docs/wp16_quiescence_design.md` revision 4 (WP-1.6)

**Revision reviewed:** `82de3e4de3689aa8dd532c0fba04651a2f849dfb` (`82de3e4`),
"docs(wp16): design revision 4 -- completion stone closes B1/B2, D-392".
**Matches HEAD:** YES — `git rev-parse HEAD` = `82de3e4de3689aa8dd532c0fba04651a2f849dfb`.
Working tree otherwise clean; this report is untracked.

**Reviewer:** fresh-context, FOURTH reviewer of this document. I did not review
revisions 1, 2 or 3. I read `wp16_design_REVIEW_rev3.md` in full first, as the
dispatch instructed, to learn exactly what B1–B4 were.

**This is a SCOPED review.** Per the dispatch, IN SCOPE:

1. §3.5a — existence / totality of the two-tier completion chain, and the query
   composition ("live windows' support") and the `table_snapshot` refusal.
2. §3.5a — determinism (coordinate-order tie-break, `NEIGHBOUR_DIRECTIONS`,
   `Eval::delta`/`static_score_after` as the only score input).
3. §3.5a — the soundness paragraph (`LEM-MONO`, lower bound / under-claim).
4. §3.5's revision-4 correction (B2's fix): a completion stone placed before the
   `Cover2::Impossible` mate-band shortcut score.
5. §6 items 4–5 (quiescence `Record` contents; the accepted store-with-no-reader
   cost).
6. §5 (the corrected config checklist).

OUT OF SCOPE, and not re-reviewed, because revision 4 did not change them and the
rev-3 review passed them: §3.1–§3.4, §3.6, §3.7 (move-set structure), §4 (zones),
§6 items 1–3 (the TT byte-layout finding and its `flags: u8` resolution), §9 (the
cost derivation), the `LAW-RIPOSTE`/`LAW-LEDGER` discharge, and the citations the
rev-3 reviewer spot-checked. I confirmed by `git diff a3b9e37 82de3e4 --
docs/wp16_quiescence_design.md` that the delta touches only the header, §3.1 (a
parenthetical), §3.5, §3.5a (new), §5, §6 items 2/4/5, and §10 — nothing else
moved.

**Sources read directly, never through the document's or a prior review's
paraphrase:** `docs/wp16_quiescence_design.md` (whole);
`docs/experiments/wp16_design_REVIEW_rev3.md` (whole);
`crates/pistol-solver/src/{query.rs,state.rs,cover.rs,sets.rs}`;
`crates/pistol-search/src/{pvs.rs,staged.rs,pv.rs,position.rs,params.rs}`;
`crates/pistol-core/src/{axis.rs,coord.rs,board.rs}`;
`crates/pistol-engine/src/config.rs`;
`crates/pistol-engine/tests/{common/mod.rs,config_validate_tests.rs}`;
every `configs/*.toml`; `docs/research/threat_calculus_v1.md` (LEM-MONO,
LAW-HIT, LAW-FORCE, LAW-OVERLOAD); `docs/decisions.md` D-356, D-386.

---

## VERDICT: **FAIL**, on scope item 4 — and read the next paragraph before acting on it.

**The completion-stone mechanism itself is SOUND. Items 1, 2 and 3 PASS, and item
4's NAMED correction (place the stone, then assign the mate-band score) PASSES.**
Items 5 and 6 PASS. B1, B3 and B4 are closed.

The FAIL is on a **fifth defect, introduced by revision 4 itself**, in the other
new clause of §3.5 — the ply-2 `can_win_this_turn` check the document adds
("**This is a small addition this revision makes to close a hole neither the
win-set nor either prior review named**"). That clause is **B2's exact defect
class, reintroduced in the same subsection revision 4 corrected for B2**: a
terminal, zero-stone return at a `Phase::Second` node. It is also **provably dead
code** — the branch cannot fire — and its stated justification is provably false
and contradicts §3.3 of the same document.

So: B2 is not fully closed (one of the two `Phase::Second` zero-stone returns was
fixed; a new one was added), but the engine's runtime behaviour is unaffected
because the new branch is unreachable. **In this reviewer's judgment this is NOT
the architect-level hard stop the dispatch reserves for "a defect in the
completion mechanism" — it is a one-or-two-sentence correction to §3.5.** F1
below states the finding, the proof of unreachability, and both available fixes.
The orchestrating session has what it needs to weigh hard-stop-vs-correct-and-proceed.

---

## Scope item 1 — §3.5a existence / totality, and the query composition: **PASS**

**The query composition verifies exactly.** Read directly against
`crates/pistol-solver/src/query.rs` and `crates/pistol-solver/src/sets.rs`:

| §3.5a's Tier-1 term | code | what it names |
|---|---|---|
| `threat_cells(side)` | `query.rs:178–180` | empties of `Class::Hot` = live, `own >= 4` (`sets.rs:99,156`) |
| `win_in_one_ply_cells(side)` | `query.rs:165–167` | empties of `Class::WinInOnePly` = live, `own == 5` (`sets.rs:101,159`) |
| `live_cells_at_count(side, LiveCount::Two)` | `query.rs:206–208` | live, `own == 2` |
| `live_cells_at_count(side, LiveCount::Three)` | `query.rs:206–208` | live, `own == 3` |

`Class` is closed at `{LiveTwo, LiveThree, Hot, WinInOnePly, Completed}`
(`sets.rs:94–105`), and `ClassSet::of` (`sets.rs:145–166`) assigns nothing at
`own == 1` (D-255's unmaintained count) and puts every window with an opponent
stone in the empty set. So the eight calls, both sides, are **exactly the empties
of every maintained live window at own-count ≥ 2** — `Completed` is the only class
omitted and it means the game has ended. §3.5a's own totality wording ("no live
window anywhere carries either side's stones at count ≥ 2") is therefore an
accurate description of what Tier-1-empty means, not an approximation.

**The `table_snapshot` refusal holds.** `ThreatState::table_snapshot` is at
`state.rs:148–150` and its doc comment reads, verbatim, "Never on a choice path:
see `WindowTable::snapshot`, whose doc says why the table underneath may be hashed
at all" (`state.rs:146–147`). Quoted correctly, and the refusal is correct: a
completion pick is a choice path.

**Legality of both tiers verifies.** Tier 1 cells come from `fill_empties`
(`query.rs:268–276`), which fills from `empty_cells(window, masks)` — every cell is
empty by construction, and every cell of a window that holds a stone is within hex
distance 5 of that stone, hence inside rule 5's radius-8 region. Tier 2 cells are
`ply1_stone.offset(d)` for `d ∈ NEIGHBOUR_DIRECTIONS` — distance 1 from a stone
just placed — filtered by `Board::is_occupied` (`board.rs:90`). Nothing illegal can
be selected at either tier, exactly as §3.5a argues.

**The residual case is named, not assumed away, and that is the right disposition.**
Tier 1 empty AND all six neighbours of the ply-1 stone occupied is left explicitly
unproven-impossible, given a named token (`NO_COMPLETION_STONE`), a fail-loud
panic per rule 3, and a RED-TEAM fixture obligation. I could not construct the
state and I could not prove it unreachable either; the document claims neither. It
also answers the rev-3 review's demand that §3.5a "say what happens to D-104's
`assert!`": `no_candidates_at_a_turn_boundary` (`pvs.rs:479–486`) is called from
`visit`'s candidate path, which a dedicated `quiescence.rs` never enters, and the
quiescence path gets its own narrower named panic instead. Coherent.

**Two non-blocking observations.**

- **N1 (redundant call).** `Class::Hot` is `own >= HOT_MIN` (i.e. ≥ 4) and
  `Class::WinInOnePly` is `own == 5` — a window at 5 is in BOTH sets
  (`sets.rs:156–161`). So `win_in_one_ply_cells(side) ⊆ threat_cells(side)` for
  either side, and Tier 1's second term is redundant on the opponent's side too,
  not only (as §3.5a says) provably empty on the mover's. Eight calls could be
  six. Costs nothing but a query; worth one word if §3.5a is touched again.
- **N2 (no route into §3.5a at ply-1).** I checked for a third entry route the
  document does not enumerate and found none: a gate that fires trigger (b)
  produces `Cover::Minimal` containing a `MinimalCover::One`, so `Cover::cells()`
  (`cover.rs:108–116`) is non-empty by construction; a gate that fires trigger (c)
  fires *because* `cells_raising_to_hot` was non-empty. Ply-1 can never run dry.
  The two routes §3.5a names are the only two.

## Scope item 2 — §3.5a determinism: **PASS**, with two precision findings

**The score input is clean.** `Position::static_score_after` (`position.rs:130–133`)
is exactly `self.eval.delta(at, self.state.to_move())` — no clock, no hasher, no
thread, no board iteration. `Board`'s stone map is a `BTreeMap`
(`board.rs:70`), and Tier 2 only does `contains_key` membership tests
(`board.rs:90`), so there is no hash-iteration order anywhere on this choice path.
`NEIGHBOUR_DIRECTIONS` is a `const [Coord; 6]` in fixed ring order
(`axis.rs:64–77`) whose own doc says it is pinned by a test "for exactly the reason
this design needs it" — §3.5a quotes that doc accurately. `Coord::offset` is at
`coord.rs:72`. `staged.rs:336–339`'s `delta_rank` doc states the tie-break
convention §3.5a claims as precedent, verbatim: "a stable sort by score leaves
equal-scoring cells in the ascending coordinate order they arrived in". Every
citation in this item resolves exactly.

CLAUDE.md rule 4 is satisfied under either reading of the two ambiguities below —
both are deterministic — so neither is blocking. Both are, however, ambiguities a
determinism self-test can never catch, which is why they are worth naming.

- **N3 (Tier 1's union is not "already-sorted").** §3.5a says to "iterate the cell
  set in its already-sorted `(q, r)` order (`fill_empties` sorts and dedups)". Each
  of the eight queries sorts and dedups **its own buffer**, and `query.rs:10–12`'s
  module doc is explicit that every cell query CLEARS its `out` and never appends.
  The UNION of eight individually sorted vectors is not sorted, and `Vec::dedup`
  only removes *consecutive* duplicates, so IMPL must sort the combined vector
  itself — exactly what `tier_t_union` already does (`staged.rs:331–333`:
  `out.sort_unstable(); out.dedup();`). IMPL following `tier_t_union`'s shape gets
  this right by imitation; IMPL following §3.5a's sentence literally gets a
  concatenation order (still deterministic, but a different tie-break winner and a
  broken dedup). One clause fixes it.
- **N4 (Tier 2's order: ring or coordinate?).** Ruling 3 says "fixed
  coordinate-order tie-break"; §3.5a's Tier-2 paragraph says to use "the fixed-order
  six unit steps" of `NEIGHBOUR_DIRECTIONS` with "same tie-break rule". These are
  different orders. `NEIGHBOUR_DIRECTIONS` is ring order —
  `(+1,0), (+1,-1), (0,-1), (-1,0), (-1,+1), (0,+1)` — whereas ascending `(q, r)`
  over the same six offsets is `(-1,0), (-1,+1), (0,-1), (0,+1), (+1,-1), (+1,0)`.
  On a tie the two readings pick different cells. Both are reproducible, so no
  determinism-law breach; but D-5/D-7's convention is the lexicographic one and the
  document should say which it means in one word.

## Scope item 3 — §3.5a's soundness paragraph: **PASS**, re-derived independently, with the argument's real load-bearer named

`LEM-MONO` is quoted correctly and is where the document says it is
(`threat_calculus_v1.md:40`, verbatim: "stones are never removed; an own extra
stone never hurts (zugzwang-free)").

**Re-derived from scratch, and the conclusion holds — by a stronger route than the
document's.** The completion stone is always a member of the node's true legal move
set (item 1 above). A `Phase::Second` node is a MAX node over that set. Restricting
the set to one member can therefore only lower the node's value:
`value(completion) ≤ max over all legal second stones`. That is an under-claim for
that node's mover, unconditionally, and it does not need `LEM-MONO` at all —
`LEM-MONO` would be the load-bearer only if the design were comparing the
completion stone against *not placing a stone* (a null move), which rule 3 forbids
anyway. So the soundness claim is true, and the citation over-justifies rather than
under-justifies it. That is a strengthening, not a defect.

I also checked the case the argument is most exposed on — whether a crippled
`Phase::Second` set can manufacture a false MATE rather than a merely pessimistic
value — and it cannot fire immediately: on route (1) the union is empty, which
requires `Cover2 == NothingToBlock`, which by `blocking_covers`
(`cover.rs:202–205`) means the opponent has **no hot window at all**; an opponent
with no window at `own >= 4` cannot complete six on their next turn even with two
stones, so no `-mate_in` can be produced one turn out. Good.

- **N5 (the paragraph's parenthetical is false for route 2).** "the extension was
  granted because `Cover`/`Cover2` was not `Impossible`" — on route (2), `Cover2`
  IS `Impossible`; that is what route (2) is. The soundness paragraph therefore
  argues only route (1) while reading as if it covered both. Non-blocking, because
  route (2)'s justification is independent and correct, and is stated in §3.5's own
  bullet (see item 4). One clause.
- **N6 (the under-claim claim is node-local; the citation is an analogy, not a
  proof).** "never the direction that would corrupt a proof or a bound the rest of
  the tree relies on" is stronger than what follows from the node-local argument: a
  node-local under-claim at odd distance from the root is a root-level over-claim
  under negamax, and this holds for §3.3's offensive ply-1 restriction as much as
  for §3.5a. The cited precedent (`pvs.rs:406–423`) is a *bound with a known
  direction that the code then re-searches or prunes on*, not a heuristic value
  returned as exact. Non-blocking, and NOT new to this delta: §3.7's registered
  stance ("Quiescence is not a prover — it never claims a proof", rev-3-verified)
  is the design's answer to this whole class, and it is the correct one. Named so
  the record does not later read as if a proof had been given.

## Scope item 4 — §3.5's revision-4 corrections: **FAIL**

### The named correction (B2's fix) — **PASS**, verified including the question the dispatch asked

Placing §3.5a's completion stone *before* assigning `-mate_in(turns_from_root + 2)`
does produce a turn-whole PV: the stone goes through `Position::place`
(`position.rs:139`) like any other candidate, the ply-2 node promotes it at its own
ply (`pvs.rs:341`), and the line the gate node hands upward is two plies, so
`turns_from_plies` (`pv.rs:79–107`) finds `pending == None` and does not fire
`PV_NOT_PLAYABLE`. Correct, and it is the right shape: `is_pv` is not reintroduced,
which keeps §3.4's simplification intact.

**The dispatch's harder question — does a real stone under the pre-computed score
change the score's correctness? — answers NO, and I re-derived it rather than
trusting the document.** Three ways the placed stone could invalidate
`-mate_in(turns_from_root + 2)`:

1. *The stone itself wins.* Impossible. `can_win_this_turn(us, StonesLeft::One)`
   (`query.rs:231–247`) returns `Some` iff `win_in_one_ply_windows(us)` is
   non-empty, and any single stone completing six must complete some window holding
   five own stones and one empty — which is exactly `Class::WinInOnePly`
   (`sets.rs:159`), since such a window has no opponent stone and is live. Route (2)
   is reached only after that query returned `None` (§3.5 orders the win check
   before `Cover2`, and §3.5a's trigger paragraph restates the ordering
   explicitly). So no completion stone can win.
2. *The stone blocks the loss.* Impossible by the branch's own condition.
   `blocking_covers(us, HitBudget::One)` returning `Impossible` (`cover.rs:201–244`)
   means the attacker's hot family is non-empty and no single cell covers it. So
   for whatever cell `c` the completion picks, some hot window `W` with `c ∉ W`
   survives — `W` is still live, still at `own >= 4`, and the opponent completes it
   next turn with their two stones. The mate distance is therefore **invariant over
   the choice of completion stone**, which is a stronger property than "the stone
   does not break it".
3. *The stone creates a counter-threat that beats the loss.* Cannot help: the
   opponent moves next and completes six immediately; nothing the mover holds
   preempts a completed six (rule 4).

The score is sound with a real stone under it, for the same reason it was sound
without one.

### F1 (BLOCKING). §3.5's OTHER revision-4 addition — the ply-2 `can_win_this_turn` check — is a terminal, zero-stone return at `Phase::Second`: B2's exact defect class, reintroduced in the same subsection, on a justification that is provably false.

**The new text, verbatim** (`git diff a3b9e37 82de3e4` confirms every word of this
is added by revision 4; revision 3 went straight to "recompute BOTH queries"):

> at the `Phase::Second` node reached after ply-1 (one stone left,
> `left' = StonesLeft::One`), first re-run trigger (a)'s own check —
> `threats.can_win_this_turn(us, left')` — before anything else. **This is a small
> addition this revision makes to close a hole neither the win-set nor either prior
> review named** … `Some(witness)` → terminal, **zero-cost, exactly §3.1's
> shortcut** … so a position could have zero live-3 windows and zero remaining
> opponent plans **while still holding a stone that wins outright**, and the rest of
> this section would otherwise never look for it.

§3.1's shortcut is "`mate_in(turns_from_root + 1)`, **zero extra nodes**, no call
into `quiescence()` at all" — i.e. *no stone placed*. At the GATE that is fine
(`Phase::First`, the line ends on a turn boundary). Applied at the ply-2 node it is
**the identical mechanism revision 4 just fixed six lines below it**: the ply-2 node
clears its slot (`pvs.rs:194`) and returns without promoting, the gate node promotes
its own ply-1 cell and copies a zero-length child line, and the line that reaches
the root ends one ply short of a whole turn, where
`pv.rs:101–105`'s always-on `assert!(pending.is_none(), …)` fires
`PV_NOT_PLAYABLE`. A mate score at a `Phase::Second` node is precisely the score
most likely to be the node's best and to ride to the root, so the guard condition
that saves §3.4 does not save this.

**It is also DEAD CODE, and the justification quoted above is false.** Proof:

- Quiescence is entered only at a gate, at `Phase::First` of a later turn, so
  `left = StonesLeft::Two` (§2, §3.1).
- The gate runs trigger (a) first; an extension is granted only when
  `can_win_this_turn(us, StonesLeft::Two)` returned `None` (§3.1 — `Some` returns
  terminal and never calls `quiescence()`).
- `None` at `StonesLeft::Two` (`query.rs:231–266`) means **both** that
  `win_in_one_ply_windows(us)` is empty (no live window at `own == 5`) **and** that
  no hot window has `own_count == 4`. `Class::Hot` is `own >= 4` (`sets.rs:99,156`),
  and `own == 6` is `Class::Completed`, i.e. the game is already over. So at the
  gate `us` has **no live window at `own >= 4` at all**.
- One stone raises any window's own-count by at most one. After ply-1, the maximum
  own-count in any live window of `us` is therefore 4, so
  `win_in_one_ply_windows(us)` is **empty at the ply-2 node**, so
  `can_win_this_turn(us, StonesLeft::One)` returns `None` **always**.

The document makes this very argument itself, twice, about a different query: §3.3
("Tier F for `us` is PROVABLY EMPTY here — trigger (a) answered `None`, which at
`left` forbids both a win-in-one-ply cell and a hot four-stone window") and §3.5a
("for `us` this is PROVABLY EMPTY by the win-check just above"). §3.5's new clause
asserts the opposite of §3.3 in the same document. The chain case does not rescue
it either: every granted turn in a `q_depth_turns > 1` chain begins at its own
gate, which re-runs trigger (a) for the side to move.

**Why this is a FAIL and not a note.** The project's standing failure class across
revisions 2 and 3 is a document that licenses a path IMPL cannot build soundly.
This clause licenses a `Phase::Second` return with no stone — the very thing D-104
says has no score that fixes it, "because the missing thing is not a value but the
mover's second stone" — and it does so on an explicit, emphasised, false claim of
reachability, in the one subsection this review exists to certify. An IMPL that
writes what §3.5 says writes a branch that, if the gate's trigger-(a) precondition
is ever relaxed by a later WP, panics on a mate line.

**Why the orchestrating session may reasonably NOT treat this as the dispatch's
hard stop.** The branch cannot fire in the design as written, so no engine
behaviour, no score, no PV and no bench number depends on it. The completion-stone
mechanism — the thing the hard stop is reserved for — is sound. Two fixes are
available and both are one or two sentences:

- **(a) Delete the clause**, and replace it with the two lines of §3.3's own
  argument stating why a ply-2 win check is unreachable (this is also the more
  informative document: it records a proved property rather than a guard).
- **(b) Keep the clause and place the witness**: `Some(witness)` → place `witness`
  (which by `WinWitness::OnePly`'s own contract completes six, so `Position::place`
  returns `PlyOutcome::Win` and the turn is whole), then return
  `mate_in(turns_from_root + 1)` — the same "place, then score" shape revision 4
  already adopted for route (2) six lines below.

Whichever is chosen, §10's changelog line "closing B2" needs to become true of both
`Phase::Second` returns, not one.

## Scope item 5 — §6 items 4–5: **PASS**

**Item 4 (what a quiescence `Record` contains) is now unambiguous and matches the
code.** `visit`'s own end-of-node store (`pvs.rs:358–375`) constructs
`Record { depth_plies, score: best_score, static_eval: self.position.value(),
bound: <Upper if best_score <= original_alpha, else Lower if best_score >= beta,
else Exact>, best: best_cell }`. §6 item 4 names exactly that field set with
exactly two substitutions (`depth_plies: 1`, `from_quiescence: true`) and restates
the three-way bound rule correctly in the code's own direction. There is no field
left for IMPL to invent — B3's blocking half is closed. It also correctly names the
two edge constructions (a completion stone as `best`, and route (2)'s
placed-then-shortcut-scored stone with the mate-band value as `score`), which is
what makes "one reading only" actually true here.

`static_eval: self.position.value()` at a `Phase::Second` quiescence node is fine
and is not D-111's `STATIC_EVAL_MID_TURN`: `Position::value` (`position.rs:115–117`)
carries no phase assertion, the invariant lives in `pvs.rs:213` and is about
*returning* a static value at a horizon, and full-width `visit` already stores
mid-turn records with mid-turn `static_eval` at every `Phase::Second` node it
visits. Nothing new.

**Item 5 (the store nothing reads) is now stated as a cost, not as a foregone
optimization**, names the consequence on `ttd` (D-388's primary metric) by name,
and states the one-sentence retreat ("quiescence does not store") together with what
that retreat deletes. It is internally consistent with items 1–3: item 3's
victim-selection rule is what makes the unread store non-destructive (a quiescence
record can never evict a full-width one), which is exactly why accepting the cost is
coherent rather than merely tolerated. §6 item 2 also folds in rev-3's M2 correctly —
the flag is put on the public `Record` as well as on `Entry`, which is what
`Table::store`/`Entry::packed` actually require.

Minor citation nits, non-blocking: §6 item 4 cites `pvs.rs:359–377` and
`pvs.rs:367–373` for the store and the bound rule; the store block is `358–375` and
the bound expression is `366–372`. Both resolve to the right code.

## Scope item 6 — §5's corrected config checklist: **PASS**, verified exhaustively against the tree

Checked by enumerating the tree, not by reading the document's list:

- `grep -rn 'kind = "staged"' configs/` returns exactly five files:
  `gate_staged_v0.toml`, `instrument_staged_v0.toml`, `instrument_v0.toml`,
  `play_staged_v0.toml`, `tactical_staged_v0.toml`. **§5's five-file list is exactly
  right**, including `instrument_v0.toml` (staged since D-386 / `9282dd0`). The three
  other configs carrying `[search.candidate_policy]` (`gate_v0`, `play_v0`,
  `instrument_r2_v0`) are `kind = "radius"` and correctly excluded — the new field is
  on the `Staged` variant only (`config.rs:161–182`).
- The three `arena_wp15b_*` configs carry no `[search.candidate_policy]` section at
  all (their `kind` keys are budget kinds, `kind = "nodes"`). **§5 is right that they
  must NOT be edited**, and right about why (`deny_unknown_fields`). B4's
  wrong-in-the-other-direction half is closed.
- `VALID_STAGED` is at `crates/pistol-engine/tests/common/mod.rs:43–70` — the line
  range is exact — with `kind = "staged"` at `:53` and the five keys at `:54–58`. It
  is the only `kind = "staged"` document outside `configs/`, and it is consumed by
  `config_validate_tests.rs` through `replacing_staged`. §5's claim that it needs the
  field is correct.
- The `StagedParams` construction sites: `grep -rn 'StagedParams {' crates/` returns
  `pistol-engine/src/instance.rs:181`, `pistol-search/tests/common/mod.rs:74`,
  `staged_tests.rs:88`, `staged_colony_family_tests.rs:122` and `:151`,
  `staged_differential_gate_tests.rs:126`, `staged_pattern_fixture_tests.rs:51`,
  `staged_tier_t_threshold_tests.rs:96`. **§5's enumeration is complete and every
  line number is exact** (eight construction sites across seven files — the dispatch's
  phrase "seven sites" undercounts `staged_colony_family_tests.rs`, but the document's
  own list has both of its lines).
- `config.rs:159–160` does nominate `U3_tier_t.md` §10 as the schema document, in the
  words §5 quotes; §5's instruction that its key count move five → six alongside the
  landing commit is the right consequence.

The "twelve staged-validation cases" figure is D-356's own registered count
(`decisions.md:757`: "12 new cases covering every validation branch"); the file today
carries eight `#[test]` functions over the staged fixture, several of which loop over
multiple cases. The count is inherited from a registered ADR rather than invented, and
the load-bearing claim (the fixture must gain the field or those tests fail on a
missing-field error) is correct either way.

---

## Out-of-scope observations (recorded, not blocking, no normative contradiction found)

- **O1.** rev-3's **M1** (`MAX_PLY = 130` vs up to `2 × q_depth_turns = 16` extra
  plies past the horizon, and `SearchInfo::seldepth_turns`' doc asserting "Stage 0 has
  no extension that passes the horizon") is deliberately untouched by this delta and
  §10 says so. It is still open and is still an unnamed `index out of bounds` waiting
  for IMPL. Not a normative contradiction, but it is the one carried-forward finding
  most likely to bite Phase 2 on its first deep iteration, and it is cheap to answer
  before IMPL rather than during it.
- **O2.** rev-3's **M3** (`PROTO-NODE` step 5's `t >= 2` vs Ruling 2's `t <= 1` gate)
  and **M9** (`Cover::Impossible` means `t >= 2`, not `t >= 3`, at `HitBudget::One` —
  `cover.rs:225` skips the pair enumeration) are likewise untouched and recorded as
  known debt. I re-checked M9 against `cover.rs` and it is correct as the rev-3
  reviewer stated it; the SCORE is still right, only §3.2's table's label does not
  transfer. Neither is a normative contradiction.
- **O3.** `position.rs:75` (cited by §3.5 for the incremental threat update) is the
  start of `reset_to`'s doc comment; the threat-apply line is `position.rs:143`, which
  the same citation names. Trivial.
- **O4.** A stale agent worktree exists at `.claude/worktrees/agent-a383215a2f97832b8/`
  containing an older copy of `configs/` and `docs/`. It is not part of the live tree
  and `tools/config_check.sh` does not reach it, but a `grep -rn` over the repo root
  returns its files as if they were config documents. Housekeeping, not a finding
  against this design.

**No STOP-3-class normative contradiction found anywhere in or out of scope.**

---

## Explicitly verified, and NOT defects — do not re-litigate

1. **Tier 1's composition is exactly "empties of every maintained live window at own
   ≥ 2, both sides"** — checked against `Class`/`ClassSet::of`, not against the
   document's prose.
2. **`table_snapshot`'s doc really says "Never on a choice path"** and the refusal to
   use it is correct.
3. **Both tiers produce only legal, unoccupied cells**; the argument is sound and the
   residual `NO_COMPLETION_STONE` case is named rather than assumed away.
4. **Determinism holds under every reading**: `BTreeMap` board, `const` neighbour
   order, `Eval::delta` the only score input, no clock, no thread, no hash-iteration
   order on the choice path.
5. **The under-claim argument is true** — and by a stronger route (max-node
   restriction) than the `LEM-MONO` one the document gives.
6. **Route (2)'s mate-band score is invariant over the choice of completion stone**,
   for the reason `Cover::Impossible` at `HitBudget::One` gives; the placed stone can
   neither win nor block.
7. **B1 is closed. B3 is closed. B4 is closed** — all three verified against the tree,
   not against the document's or the rev-3 review's citation of it.
8. **Every `file:line` I spot-checked in the changed sections resolves**:
   `query.rs:165–167`, `:178–180`, `:206–208`, `:231`, `:268–276`;
   `state.rs:148–150`; `sets.rs:94–105`, `:145–166`; `cover.rs:108–116`, `:201`,
   `:216–239`; `axis.rs:64–77`; `coord.rs:72`; `board.rs:90`;
   `position.rs:115–117`, `:130–133`, `:139`, `:143`; `pvs.rs:194`, `:341`,
   `:358–375`, `:387–425`, `:429–431`, `:479–486`; `pv.rs:79–107`;
   `staged.rs:196–201`, `:294–334`, `:331–333`, `:336–339`;
   `config.rs:159–160`, `:161–182`; `params.rs:58–70`;
   `tests/common/mod.rs:43–70`; `threat_calculus_v1.md:40`; `decisions.md:757`.
   Two are off by one line (§6's `pvs.rs:367–373`/`:359–377`) and one points at a
   doc-comment line rather than the code (§3.5's `position.rs:75`). Nothing
   load-bearing.

---

## What would close this

One clause, in §3.5 only.

1. **F1**: either delete the ply-2 `can_win_this_turn` clause and replace it with
   §3.3's own two-line proof that it is unreachable (`can_win_this_turn(us, Two) ==
   None` at the gate ⇒ no live window of `us` at `own >= 4` ⇒ none at `own == 5` after
   one stone ⇒ `can_win_this_turn(us, One) == None` at every ply-2 node), or keep it
   and specify that the witness is PLACED before `mate_in(turns_from_root + 1)` is
   returned — the same "place, then score" shape route (2) already uses. Then make
   §10's "closing B2" true of both `Phase::Second` returns.
2. **N3, N4, N5** are one clause each and none moves a number: sort the Tier-1 union
   (as `tier_t_union` does) rather than calling it "already-sorted"; say whether Tier
   2 iterates ring order or ascending `(q, r)`; drop or qualify the "was not
   `Impossible`" parenthetical, which is false on route (2).
3. **N1, N6** are optional polish. **O1** (`MAX_PLY`) is the carried-forward finding
   most worth answering before IMPL rather than during it.

*Report written by the fresh-context, fourth-slot, SCOPED REVIEW-design (Phase 1'')
subagent against `82de3e4`. Left uncommitted for the orchestrating session.*
