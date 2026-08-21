# WP-1.5b — staged threat-first candidate generation: DESIGN

**Revision 3.** Base revision `f317385`; revision 1 was `ec8f7fb`, revision 2
`182f389`. **Revision 2 FAILED its REVIEW-design** with two STOP findings in §5 —
the normative content of the whole work package — one BLOCKING finding that
voided §12.5's registered criterion, and seven MAJORs. §0.2 lists them. Revision
3 is the redesign, not a clarification.

**Revision 2.** Base revision `f317385`; revision 1 was `ec8f7fb`. Revision 1's
five matrices were each attacked by a fresh-context DECISION-RED-TEAM before
selection, as the Process section requires. **Three of the five FELL** (M0, M2,
M3) and two survive amended (M1, M4). Every finding below was reproduced — by
the reviewer, and independently by this session before it was accepted — and the
reproducers are cited rather than summarised.

An amendment reopens the review, so revision 2 owes a fresh REVIEW-design and
each amended matrix owes nothing further only where its red team is recorded as
having attacked the surviving option. Where an option is NEW in this revision —
M0's (f), M2's W-E, M3's S-E — it was supplied BY the red team that killed its
predecessor, which is the attack the Process section asks for, and revision 2
records that provenance rather than claiming an unattacked selection.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This document restates no theory; where it appears to, the calculus
wins and the disagreement is an ADR line.

---

## 0. What revision 1 got wrong

Listed first, because a reader who knows only revision 2 cannot see what the
review round bought, and because four of the nine are numeric claims this
document itself marked **MEASURED**.

| # | Revision 1 said | Revision 2 says | Found by |
|---|---|---|---|
| 1 | The link gate has "no discriminating answer left" post-linkage, so retire it | FALSE. Reproduced: a smuggled `include_str!` leaves the edge transcript byte-identical (md5 `5d0b6ee…`, `diff` empty) while the link gate names the file in 5 hit lines, 445 → 450 source inputs | M0 |
| 2 | Inverting the link gate "requires a `tools/` change" | FALSE. The live test already calls `link_check(&repo_root(), …)` and `repo_root()` canonicalises; stripping it is one `String::replace` test-side | M0 |
| 3 | "8 tests and 20" in the two gate suites | 9 and 19 — transposed | M0 |
| 4 | Tier T option B is 46.50 cells (**MEASURED**) | 46.50 is the count-**≥2** reading; §10 committed count-**==2**, which measures 46.3333. **The option committed was not the option measured** | M1 |
| 5 | "A live-2 window's empties reach up to 5 cells from a stone" (unmarked) | 4. Five is the arbitrary-cell bound | M1 |
| 6 | W-A "is the schedule that most directly meets 'never a bare hard cap'" | The root can never widen (`original_alpha = -INFINITY` is unreachable) and non-PV nodes can never truncate, so W-A caps only the root and the PV | M2 |
| 7 | A node returning "an exact score inside the window is unaffected" | An exact score over a SUBSET is a lower bound only, and it is stored as `Bound::Exact` and consumed unconditionally | M2 |
| 8 | The soundness gate costs "60–180 s, dominated by the reference at depth 3, whose cost is gate 10's today" | `configs/gate_v0.toml`'s own committed table measures radius 2 / depth 3 at **> 100 s for the engine alone**; the reference at radius 2 depth 3 on the corpus's cheapest branching position is **243 363 538 nodes in 554.2 s**. Gate 10 runs depth 3 at radius **1** | M3 |
| 9 | S-C is the instrument D-124's blindness demands | S-C is blind to D-124's own reproducer: `cells.pop()` after `order` leaves the whole class gate at **28 assertions, 0 RED** | M3 |

### 0.2 What revision 2 got wrong

Revision 2 fixed nine things and introduced four, three of which came straight
out of its own repairs. That is the pattern worth naming: **each STOP below is a
consequence of a revision-2 amendment that was not propagated.**

| # | Revision 2 said | Revision 3 says | Severity |
|---|---|---|---|
| 10 | §5.3's `Impossible` row: "At phase 0 this is step 2 and is not reached here", and generation there is "Tier T ∪ Tier Q" | FALSE, and it drops mates. Revision 2 gated step 2 on `!is_pv`; the root is always a PV node, so `Impossible` **is** reached at phase 0 — at the root, at every PV node, and whenever `can_win_this_turn` fires. Reproduced on a legal 10-turn position where the mover holds `WinWitness::Pair` and the row would have generated no winning cell | **STOP** |
| 11 | The survival filter applies on `Cover::Minimal` alone | `LAW-FORCE`'s antecedent is "the opponent has ≥1 plan **AND the mover cannot win this turn**". `Minimal` establishes only the first. Reproduced: a `Minimal` node where the mover wins by PAIR and `win_in_one_ply_cells` is EMPTY, so "plus own win-now cells" under its natural reading generates no winning cell | **STOP** |
| 12 | §12.5's second instrument agrees iff the two regimes rank A, B, C the same by staged-set size | Under the threshold reading `own≥3 ⊆ own≥2` and `them≥3 ⊆ them≥2`, so **A ⊆ C ⊆ B** as SETS and the ranking is a set-inclusion identity, invariant under every sampler. Verified over all 24 corpus roots. A criterion nothing can falsify | **BLOCKING** |
| 13 | D-263's discharge: the two threat queries are "0.6 %–3.7 % of a node" | The bracket omits its own worst cell. Recomputed from the same numbers: corpus-roots cost 347 ns against the fastest node's 3300 ns is **10.5 %**, and single-call maxima reach 2665 ns = **81 %** of that node. The bracket was the sole ground for declining D-263's three registered remedies | **MAJOR** |

An interim note this session wrote between rounds — that the gate's honest cost
was "ESTIMATED 25–45 s, whose dominant term is MEASURED at 17.89 s" — is
**withdrawn** with row 8. It read gate 10's radius-1 depth-3 number as if it
priced a radius-2 depth-3 sweep.

---

## 1. Preconditions, verified rather than assumed

### 1.1 What was checked

| Precondition | Verdict | Evidence |
|---|---|---|
| INTEG landed and its review round closed | HELD | D-287 through D-300; the round's findings at `6b03899` are each recorded with a reproducer and fixed in `c875016`..`f317385`. One item is recorded OPEN and not closed: D-295's structural residual (no `HitBudget`-shaped fixture separates `t = 3` from `t = 4`) — §11.6 is this WP's contact with it |
| The calculus is in-repo | HELD | `docs/research/threat_calculus_v1.md`, adopted verbatim by D-266 |
| The pattern fixture pack is green | HELD | `pattern_calculus_tests.rs`, 14 tests all `ok` in gate 3 of the base run |
| Arena digest-binding is in place | HELD | D-283, D-294(1); `binary_binding_tests.rs`, 7 tests all `ok` |
| CI green at the base revision | HELD | `tools/ci.sh` exit 0, `ci: all gates passed`, MEASURED 5 m 20 s (warm target) |

### 1.2 One precondition artefact does not exist

The prompt names "the prior 1.5b design report (its option matrices RESUME
here)". **There is no such document in this repository**, at any revision, and no
session scratchpad holds one. The matrices are authored here rather than
resumed; their prior is the ADR record written for this WP before it started —
D-243, D-249, D-255, D-257, D-261, D-263, D-267 — and `docs/ROADMAP.md`'s WP-1.5
entry.

### 1.3 One prompt claim is corrected against the artefact

The prompt says D-267's prose "said *eight* over what reads as nine IDs". D-267
contains no count word at all. Counted from the artefact: the absence paragraph
holds **nine entries naming ten IDs**, of which PROTO-PAIR is called out
separately as living in `pistol-core` and the remaining eight are the "no
counterpart" list. `eight` appears in this log at D-13, D-242 and D-247, none of
them about this ledger.

---

## 2. What this work package changes

### 2.1 The shape

`CandidatePolicy` gains a second variant, `Staged`. `pistol-search` takes a
normal dependency on `pistol-solver` — the first structural act, and the one
D-282 measured the gates' expiry against (§4).

`Position` already moves `GameState` and `Eval` from one place because they are
only correct together (D-41). It gains a third member, `Option<ThreatState>`,
moved from the same place. `Option`, not a bare field: under
`CandidatePolicy::Radius` nothing consults it, and a radius search paying for a
threat state it never reads would make the SPRT's incumbent slower for no reason
— a measurement hazard in the direction that flatters the change under test.
`reset_to` rebuilds it from scratch, O(stones × 18) once per search.

`pvs::visit` gains the node protocol (§5) and a batched candidate loop (§7).
`SearchInfo` gains stage counters (§12.3). Nothing in the line protocol's output
changes, so the D-209 golden transcripts — which are taken at `configs/gate_v0.toml`,
not at the instrument config (§9, F3) — are untouched either way.

**Adding the variant is not a local change.** MEASURED by grep at this revision:
**six** irrefutable `let CandidatePolicy::Radius { radius } = …` destructures
become compile errors the moment `Staged` lands — `tests/common/reference.rs:242`,
`tests/common/agreement.rs:131`, `src/search.rs:97`, `src/search.rs:311`,
`pistol-cli/src/bin/pistol.rs:131`, `pistol-engine/src/validate.rs:82` — plus two
match arms, across 30 mentions. Revision 1's "reuses `reference.rs` unchanged"
was false and is withdrawn.

### 2.2 What does NOT change

`configs/instrument_v0.toml` and `configs/play_v0.toml` stay at `kind =
"radius"`. Staged ships as three config-selectable documents (§10). The committed
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
  deferral takes an ADR line (§15) rather than living in this prose.

---

## 3. MANDATORY DESIGN ITEM 1 — the D-267 ledger, dispositioned

**The count, taken from the artefact:** NINE entries naming TEN IDs. Every one is
dispositioned; none is silent.

| # | ID(s) | Disposition |
|---|---|---|
| 1 | `PROTO-PAIR` | **ALREADY IMPLEMENTED, mapping recorded.** Verified against shipped code, not a doc comment: unordered-pair canonicalisation is `pistol_core::turn::canonical_pair` (`Turn::Pair(a, b)` with `a < b`), landed in WP-03/04; the intra-turn phase bit is `zobrist::phase_key`, mixed into `GameState::key`. Neither moves |
| 2 | `DEF-STAR` | **DEFERRED to WP-1.8.** The star is the relevance-zone primitive; staged tiers are defined by window membership, not geometric reach |
| 3 | `DEF-TEMPO` / `ADOPT-TEMPO` | **DEFERRED to eval-terms.** τ is an eval-term candidate by §6's own note, SPRT-gated |
| 4 | `LAW-RIPOSTE` | **DEFERRED to WP-1.6.** Forced-reply plan checks belong to a prover; this WP has none |
| 5 | `LAW-LEDGER` | **DEFERRED to WP-1.6.** t=1 chain semantics and the 2−t bank are stand-pat questions |
| 6 | `LAW-DECOMP` | **DEFERRED to Stage 3.** Its warning is already respected: this WP sums no regional eval |
| 7 | `ZONE-R` | **DEFERRED to WP-1.8 / Stage 3.** Every claim this WP makes is bounded-depth and is therefore EVIDENCE and never PROOF (`REJ-DEPTHPROOF`, which binds us too). §8.6 states this for the soundness gate — revision 1 claimed §11 did and it did not (M3 F12) |
| 8 | `THM-WINDOW` | **NOT CONSUMED; REGISTERED BY NAME** (§15). Exact-`t` counters as eval terms become cheap precisely because this WP puts a `ThreatState` on the per-node path |
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

## 4. MANDATORY DESIGN ITEM 0 — gate supersession

### 4.1 The expiry, re-measured

D-282 measured it at `d6f6cbb`. Re-measured at `f317385` in a `/home` worktree by
adding exactly the edge this WP adds:

```
tools/solver_edge_check.sh  .  pistol-solver         -> exit 1, 6 tree lines
tools/solver_link_check.sh  .  crates/pistol-solver  -> exit 1, 30 hits over 5 binaries
```

**MEASURED, and not in D-282:** `pistol-solver` is absent from
`[workspace.dependencies]` today, so the linking commit touches **three** files —
`Cargo.toml`, `Cargo.lock`, `crates/pistol-search/Cargo.toml`. A session editing
only the member manifest gets `dependency.pistol-solver was not found in
workspace.dependencies` and **exit 2 from both gates** (a stale-lock VOID), not
the exit 1 the expiry predicts.

**MEASURED, and it is what killed revision 1's option:** the gate SCRIPTS do not
expire. Both take the workspace root and the subject as ARGUMENTS. Of
`solver_edge_check_tests.rs`'s **9** tests and `solver_link_check_tests.rs`'s
**19** (revision 1 said 8 and 20 — transposed), exactly ONE each is an assertion
about this workspace. What expires is two test functions.

### 4.2 MATRIX M0 — the two live-workspace assertions

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| (a) RETIRE both | Delete both live assertions; keep both scripts and their 26 scratch tests | **MEASURED** 2 test functions, 0 lines of `tools/` | Loses the standing guard against an accidental edge in a world where a deliberate one exists |
| (b) INVERT both as declared lists | Both become "linked exactly where declared" | Edge test-only; link needs `<workspace>` substitution in the script | D-282's caveat: a list maintained by memory (D-275's lesson) |
| (c) INVERT the edge, RETIRE the link | Edge becomes "direct dependents are exactly {`pistol-search`}" | **MEASURED** 0 lines of `tools/` | Still a memory list, and pins only depth 1 |
| (d) KEEP BOTH RED, marked expected-fail | `#[ignore]`, or `assert!(!status.success())` | Trivial | `assert!(!success)` is satisfied by exit **2** — the VOID code — which is the class `assert_code` exists to kill (D-299(2)). *Rejected, but see §4.4: revision 1 rejected the option's weakest formulation and never considered `assert_code(&ran, 1, …)`, which is not satisfied by exit 2* |
| (e) Edge as a GOLDEN TRANSCRIPT, RETIRE the link's live assertion | *Revision 1's recommendation* | — | **FELL — see §4.3** |
| **(f) Edge as an AMENDED golden transcript, LINK INVERTED to a DERIVED hit-set invariant** | Edge: pin the gate's stdout, with colour neutralised, `pwd -P`, the cargo version recorded, and a D-209-shaped regeneration discipline. Link: strip `repo_root()` test-side, discard the preflight and `N source inputs` lines as machine-variant, and assert the set of `crates/pistol-solver/` files in the hit lines is EXACTLY the set of `*.rs` files the test enumerates from `crates/pistol-solver/src/` | Edge: a small `tools/` change (`--color never`, `pwd -P`) — revision 1's "0 lines of `tools/`" is **withdrawn**. Link: **MEASURED** 0 lines of `tools/` | The edge transcript pins only `pistol-solver`'s reverse cone and its members' version strings — see the surviving attack |

### 4.3 Why (e) fell, measured

**Reproduced by the reviewer and again independently by this session.** With the
linkage in place, add a codegen route into a file under `crates/pistol-solver/`
that is not already a compiled input of the solver:

```
printf 'NOT A COMPILED SOURCE.\n' > crates/pistol-solver/NOTES.txt
# in crates/pistol-core/src/lib.rs:
pub const _SMUGGLED: &str = include_str!("../../pistol-solver/NOTES.txt");
```

- edge transcript: **byte-identical**, md5 `5d0b6eeedb6e3907464472b7e812c108` with
  and without, `diff` empty.
- link gate: names `NOTES.txt` in **5 hit lines**, `source inputs` 445 → **450**.

Revision 1 generalised from a single example — `include_str!` from
`pistol-search` into `pistol-solver`, where the link gate is genuinely blind
*because that file is already in the legitimate compile set*. That instance does
not generalise. **Option (e) would have deleted the only instrument in CI that
sees the residual class revision 1 admitted to losing.**

And the cost that excluded every link-keeping option does not exist: the live
link test already calls `link_check(&repo_root(), …)` and `common::repo_root()`
returns a canonicalised absolute path, so removing it from captured stdout is one
`String::replace` **in the test**.

### 4.4 ADOPTED: (f)

Grounds. The link half is kept because it demonstrably discriminates (§4.3) and
because its inverted form uses an **externally derived referent** — the solver's
own source directory, enumerated by the test — rather than the gate's own output,
which is the operationalisation CLAUDE.md says a reviewer looks for first. The
edge half is kept as a transcript because it is strictly more than a depth-1
declared list and it caught the accidental `pistol-cli → pistol-solver` edge in
the reviewer's construction; it is amended because it is not machine-invariant as
shipped:

- **Colour.** REPRODUCED by this session: `CARGO_TERM_COLOR=always cargo tree …`
  piped to a file still emits `^[[2m` around the tree glyphs. `--color never`
  goes into the script.
- **`pwd` vs `pwd -P`.** `tools/solver_edge_check.sh:103` uses bash's LOGICAL
  `pwd` where its sibling `tools/solver_link_check.sh:67` uses `pwd -P`. Cargo
  prints physical paths, so a symlinked root defeats the `<workspace>`
  substitution — a latent defect in a shipped gate, found by this round.
- **Version.** The transcript records the cargo version, as D-209 records the
  revision, the profile and a sha256.
- **Regeneration.** D-209's discipline, quoted: "Regenerating this fixture
  compares post to post and certifies nothing, so a regeneration is legitimate
  only for a deliberate, ADR-recorded instrument-behavior change naming the new
  revision." Revision 1 offered regenerability as the *reason* the transcript was
  not a memory cost, which is that precedent cited against itself.

**THE STRONGEST SURVIVING ATTACK, recorded verbatim for the ADR line:**

> A golden transcript relocates the memory D-282 objected to rather than removing
> it: every red it can ever show — a legitimate crate added inside the cone, an
> accidental `pistol-cli` edge, a workspace version bump that is not a graph
> change at all — arrives as the same-looking diff with the same one-command
> repair, so the judgement a declared list would have forced a maintainer to
> write down is instead deferred to whoever is looking at a red suite and least
> wants to be delayed; and its silence is scoped, because it pins only
> `pistol-solver`'s reverse dependency cone and was MEASURED byte-identical
> across the addition of a whole new workspace member, across `pistol-core`
> acquiring a non-std dependency in breach of rule 2, and across an out-of-graph
> `include_str!` that put a non-source file from `crates/pistol-solver/` into all
> five shipped binaries.

The last clause is what (f) answers and (e) did not: the link half goes red on
exactly that construction. The first two clauses stand against (f) unrepaired,
and the regeneration discipline is the only thing between them and a maintainer
pasting a diff.

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

### 5.2 Step 2 — overload at node entry (`LAW-OVERLOAD` with its guard)

```
let left = StonesLeft::from_state(state)?;          // never a hardcoded budget
if !is_pv
   && can_win_this_turn(us, left).is_none()
   && unblockable_double_threat(them, HitBudget::from(left))
{ return -mate_in(turns_from_root + 2) }
```

**The budget comes from `StonesLeft::from_state`, never from the phase.**
Revision 2 wrote `HitBudget::Two` under `phase == First`, which is wrong at turn
1: `stones_owed` is 1 there and `Phase::First` does not imply two stones. D-257's
whole ground for the two closed types is that conflating "stones the turn owes"
with "cells a cover may spend" is the class they exist to prevent, and revision 2
committed exactly that class in the one place the law is applied.

- **The guard is step 1's answer**, evaluated first and unconditionally (D-243
  (3), D-257 (a)/(b)).
- **The distance is exact and is `k + 2`.** At a node `k` turns from the root it
  is our turn — the `(k+1)`-th — and the opponent completes six on the `(k+2)`-th.
  They cannot win sooner and cannot fail to win later: `t > 2` means no two cells
  hit every plan, and `LAW-HIT` says hitting is the only defence. Checked against
  `score.rs`: `visit` carries mate scores root-relative in distance and
  node-relative in sign, so the value is `-mate_in(k+2)`. The band: `k` is a TURN
  count bounded by `MAX_DEPTH_TURNS = 64`, so `k+2 <= 66`, far inside
  `MAX_MATE_TURNS = 1000`. (Revision 2 wrote 130, which is `MAX_PLY`; conservative
  but a plies-for-turns confusion in a numeric claim.)
- **`!is_pv` is load-bearing and is new in revision 2.** An early return leaves
  this ply's line empty. At ply 0 that is fatal — `Searcher::search` panics with
  `NO_MOVE_FROM_A_COMPLETED_ITERATION` on an empty root PV, and a lost root still
  has to play a move. At an interior PV node it truncates the reported variation,
  which is the objection `pvs.rs` already states for the table cutoff. The root
  window is `(-INFINITY, INFINITY)`, so ply 0 is always a PV node and the check
  can never fire there — the root case is closed by construction rather than by a
  special case. Where it does fire, turn-wholeness is automatic: a phase-0 node
  is at ply ≥ 2, its parent at ply−1 is phase 1 and promotes `[parent_cell]`, its
  grandparent at ply−2 is phase 0 and promotes two plies — one whole turn.
- **Placement:** after the transposition probe and its cutoff, under the same
  `!is_pv` guard the cutoff already carries. It stores nothing in revision 1 of
  the code; the value is exact and depth-independent so storing would be sound,
  and not storing is the conservative first landing.
- Registered refinement, NOT taken: promoting a witness line would let it fire at
  PV nodes too (§15).

### 5.3 Step 3 — win-now, then the survival filter

**Revision 3 restores `PROTO-NODE`'s own ordering, which revision 2 inverted.**
The calculus puts win-now at step 1 and the survival filter at step 3; revision 2
wrote the filter as a `match` on `blocking_covers` whose arms then tried to
re-admit the win-now class, and two of the three arms lost it. The ordering is
the fix, and it is the calculus's, not a repair invented here.

```
let left = StonesLeft::from_state(state)?;
match can_win_this_turn(us, left) {
    Some(_) => WIN-NOW ROW,
    None    => match blocking_covers(us, HitBudget::from(left)) { … }
}
```

**The WIN-NOW row.** Generate **exactly** the win-now class of §5.1 — every
`win_in_one_ply_cells(us)`, plus at `StonesLeft::Two` both empties of every own
hot window at exactly four stones. No filter, no Tier T, no Tier Q, no batching.

Sound, and the argument is short: at a node `k` turns from the root the best
score any move can reach is `mate_in(k+1)`, because a mate at that distance means
completing six on this turn; so the win-now class is exactly the argmax set and
nothing outside it can beat it. And the class is COMPLETE — every way to complete
six this turn is a window that held five own stones (one stone) or four (two
stones), since a completing stone forms a fully-own length-6 window and overlines
contain one. `LAW-FORCE`'s antecedent fails here, so the filter is not licensed
and is not applied: that is STOP-2's repair.

**The filter rows**, reached only when `can_win_this_turn` is `None` — which is
`LAW-FORCE`'s second conjunct, now established rather than assumed:

| `blocking_covers(us, budget)` | Meaning | Generation |
|---|---|---|
| `NothingToBlock` | `t = 0` (`PAT-RHOMBUS`, `PAT-O3`) | Tier F (empty here) ∪ Tier T ∪ Tier Q, batched per §7 |
| `Minimal(covers)` | `1 ≤ t ≤ budget` | **Filtered**: the union of cells over the inclusion-minimal covers. Tier F's win-now class is empty on this row by construction, so revision 2's "plus own win-now cells" is not a patch but a tautology, and it is dropped |
| `Impossible` | `t > budget` | The position **is** lost — `LAW-OVERLOAD` in full, both conjuncts, the second one established by the `None` arm. Generate Tier T ∪ Tier Q batched per §7 so the search reaches the conclusion through the generated moves, and count it |

**Where `Impossible` is actually reachable**, stated rather than asserted away —
revision 2 claimed phase 0 could not reach it and was wrong three ways:

- **At the root**, always: `iterate` opens at `(-INFINITY, INFINITY)`, so ply 0
  is a PV node and §5.2's `!is_pv` gate can never fire there. A lost root still
  has to play a move.
- **At every interior PV node**, for the same gate.
- **At phase 1**, where §5.2 does not run at all.

Revision 2's third claim — that step 2 covers the phase-0 case whenever the mover
can win — is now vacuous rather than false: the win-now row takes that node
first.

**Why the filtered set is complete for non-losing moves.** `LAW-FORCE` is
`[PROVEN]`: if the opponent has ≥1 plan **and the mover cannot win this turn**,
every non-losing mover move hits ALL opponent plans; its corollary is that
counter-threats never substitute for hitting, except win-now. Both conjuncts now
hold on the `Minimal` row by construction, so the filtered set contains every
non-losing move and a further stage could only add losing ones.

**The two-ply realisation — VERIFIED on the shipped solver before implementation.**
`Cover::Minimal` carries SETS because the union is provably insufficient (D-257).
Run at `f317385`, two disjoint sealed five-stone P1 rows, P2 to move with two
stones:

```
P1 hot windows: 2
phase0 cover = Minimal([Two { first: (4,4), second: (5,0) }])
phase0 union = [(4,4), (5,0)]
phase1 cover after (4,4) = Minimal([One((5,0))])
```

Both cells are in the phase-0 union, and after the first stone the survivor comes
back as a one-cell cover at `HitBudget::One`. D-243 (4)'s pairing obligation is
discharged by the phase-1 regeneration.

Incidental and load-bearing for the fixture: the same two rows UNSEALED give 8
hot windows and `Cover::Impossible` — correct, since a five-run open at both ends
cannot be stopped by two stones. A fixture that forgot the seals would test the
overload path while claiming to test the pairing path.

**One licensed value change.** Under the filter the search no longer prefers the
longest resistance among losing moves. `LAW-FORCE` licenses this — those moves
lose — so mate DISTANCES on already-lost positions may shorten under Staged. It
changes what a printed score means (D-3, D-72) and it takes its own ADR line
(§15 item 10), which revision 2 promised without naming.

### 5.4 Step 4 — staged generation

1. **Tier F** — never delta-ranked, emitted ascending `(q, r)`. Delta is not
   called for Tier F cells at all, which is what makes a filtered node cheap.
2. **Tier T** — `LAW-SUPPORT`-qualified per §6. Delta-ranked, stable sort.
3. **Tier Q** — the remaining cells of the `quiet_radius` ball, delta-ranked,
   batched per §7.

**The forced prefix and `ordering::order` — specified, because they conflict as
shipped.** `order` stable-sorts the WHOLE candidate vector by `Eval::delta` and
then rotates the table's move to index 0. Both would destroy a forced prefix.
Under `Staged` the generator therefore produces the vector already ordered and
`order` is **not called**: Tier F is emitted ascending `(q, r)` with no delta call
at all, Tiers T and Q are delta-scored and stable-sorted WITHIN their own ranges,
and the table's move is promoted only **within the tier it belongs to** — never
across a tier boundary, so the forced prefix stays a prefix and S-E's containment
assertion keeps its subject. A table move that is not a candidate is dropped, as
today. The deadline check `order` performs under a wall-clock stop moves into the
staged scorer at the same `ORDER_CHECK_INTERVAL` stride, so the play-mode
overshoot bound is unchanged.

**Tier F is exempt from the batch cut.** §5.4's whole argument for the pair class
is that it must not depend on ranking, and a batch boundary is a ranking cut by
another name. Tier F is emitted whole on every row of §5.3, and the first batch
boundary is counted from the END of Tier F, never from the start of the vector.

Tiers are disjoint by construction. Every staged cell is filtered through
`Board::is_legal_placement`, one cell at a time: D-243 proves every Tier F and
Tier T cell is inside rule 5's region, but D-77 forbids exactly the shortcut "the
radius is at most eight, so every policy cell is legal" and D-20 forbids
comparing the two radii. Because that proof says no Tier F cell can be dropped, a
Tier F cell the rules refuse is a **named refusal**, never a silent drop.

### 5.6 MATRIX M5 — how the overload verdict is realised

Owed because §5.2 itself names a second viable option, and the Process section
makes an option adopted without a matrix the same breach as silent architecture
drift. Revision 2 adopted the early return, gated it `!is_pv`, and recorded the
alternative as "registered refinement, NOT taken" — a deferral written as if it
were a selection.

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| **M5-A — early return, `!is_pv` gated** *(revision 2's, recommended here)* | Return `-mate_in(k+2)` without expanding, at non-PV nodes only | One threat query per non-PV node after the table probe. **MEASURED** `unblockable_double_threat` at 49–101 ns mean | Fires nowhere at the root or on the PV, which is where the answer is chosen, so the saving is on the cheap majority. Leaves the reported PV silent about WHY a line is lost |
| M5-B — early return everywhere, with a WITNESS LINE promoted | Also fire at PV nodes, promoting the opponent's uncoverable completion so the PV stays turn-whole | Constructing a witness line at every firing; a claim about the opponent's play nothing else in this WP produces | The witness is a line the search did not search, which is exactly what `pvs.rs` refuses for the table cutoff — "the score would come back without the line that proves it". A NEW class of unproved PV, not an extension of an existing one |
| M5-C — no early return; let the search prove the loss | Delete step 2; the `Impossible` row already generates | Zero new code; the loss is proved rather than asserted | Pays a full subtree at every overloaded node to reach a conclusion `LAW-OVERLOAD` proves in one query, and forgoes the one place a `[PROVEN]` law replaces search outright — which is why the calculus has a step 2 |
| M5-D — M5-A, and STORE the verdict | A `Bound::Exact` record beside the return | One store | The value is exact and depth-independent so the store is sound, but it interacts with §7.2's truncation rule, which is new and unmeasured, and the interaction has no test. Deferred, not rejected |

**RECOMMENDATION: M5-A**, with M5-D registered as a follow-up once §7.2's
truncation rule has been measured. Grounds: it is the only option that takes the
law's saving without inventing a PV the search did not walk, and its cost is
MEASURED at the same 49–101 ns §12.4's discharge already accounts for. M5-C is the
honest fallback if a reviewer finds the early return unsound; nothing in it is
wrong, it is only slower.

**This matrix has NOT yet been attacked.** It goes to a fresh-context
DECISION-RED-TEAM against this revision, and the surviving option's ADR line
records the strongest surviving attack. Until that report lands M5-A is a
RECOMMENDATION and not a selection, and this document says so rather than letting
a reader take the table for a decision.

### 5.5 Step 5 — quiescence

Out of scope. WP-1.6's, with `LAW-RIPOSTE` and `LAW-LEDGER` (§15).

---

## 6. MATRIX M1 — Tier-T qualification — SURVIVES AMENDED

### 6.1 The reading, corrected

Revision 1's §10 said `tier_t_own_count = 2` was "mapped to the closed
`LiveCount`", whose `LiveTwo` is `own == 2`. Its §6.1 table was produced by a
census that unioned `LiveTwo ∪ LiveThree`. Re-derived independently over the same
24 corpus roots:

```
option B, EXACT count-2 reading:   46.3333
option B, THRESHOLD (>=2) reading: 46.5000
revision 1's table printed:        46.50
```

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

**MEASURED** at `f317385`, release, by a census harness that revision 2 **commits
at a named revision** (§12.4) rather than deleting with its worktree — CLAUDE.md's
instrument clause, and D-287's rule that an artefact recording numbers is not
test-tree-only.

Three regimes. The middle one is **re-sampled** in revision 2: revision 1 deepened
by uniform draws from the radius-**8** legal ball while the policy is radius **2**,
which inflated the ball 78.0 → 123.7 by the sampler rather than by depth.

| quantity | corpus roots | +1..3 turns (r8 draw) | +1..3 turns (r2 draw) | playouts |
|---|---|---|---|---|
| own hot, mean / max | 0.04 / 1 | 0.33 / 4 | — | 0.08 / 5 |
| opponent hot, mean | 0.46 | 0.21 | **0.30** | 0.10 |
| opponent live-3 | 1.88 | 1.47 | **1.92** | 1.87 |
| survival filter applies | 29.2 % | 15.1 % | **19.5 %** | 4.8 % |
| candidates when filtered | 2.17 / 3 | 2.32 / 8 | — | 2.27 / 4 |
| radius-2 ball, mean | 78.0 | 123.5 | **93.7** | 376.5 |
| **C vs full-width** | **2.7×** | 3.1× | **2.4×** | 6.1× |

The radius-2-confined regime is the reported one; the radius-8 regime is retained
as the **second instrument**, with its agreement criterion and registered
consequence fixed before either runs (§12.4).

Staged-set sizes at `quiet_top_k = 16`, threshold reading:

| option | Tier T cells | staged set | vs full-width, **all three against the reported r2 ball** |
|---|---|---|---|
| A — count ≥3 both sides | 6.12 / 7.03 / 6.65 | 16.8 / 19.9 / 21.6 | 4.6× / **4.7×** / 17.4× |
| B — count ≥2 both sides | 46.50 / 51.66 / 88.13 | 47.5 / 58.6 / 98.6 | 1.6× / **1.6×** / 3.8× |
| **C — ≥2 for us, ≥3 for them** | 23.29 / 30.3 / 48.7 | **28.6 / 39.6 / 61.4** | **2.7× / 2.4× / 6.1×** |

**The middle column is recomputed, and revision 2's was not one instrument.**
Revision 2 printed 6.2× / 2.1× / 2.4×. Recomputed: 19.9 and 58.6 divide the
radius-**8** ball (123.5) to give 6.206 and 2.108, while 39.6 divides the
radius-**2** ball (93.7) to give 2.366 — so A's and B's middles came from the
regime this section's own prose calls superseded, and C's from the reported one.
Against the reported r2 ball throughout, B's middle reduction is **1.6×, not
2.1×**: a 32 % overstatement inside an option matrix's cost column, and §0 exists
for exactly this class. The A/C/B ordering is unaffected; the magnitudes are not.

### 6.3 The options

| Option | Theory standing | Cost | Failure modes |
|---|---|---|---|
| A — count ≥3 both sides | **No completeness licence.** `LAW-SUPPORT` k=2 licences windows at ≥2, and T10 adds that a window made hot this turn held ≥2 before — so count 3 misses every plan a PAIR creates from a count-2 window, which is the two-stone move this game is about | Cheapest, 4.6–17.4× | Provably k=2-incomplete. The reviewer built the position: P1 (0,0)(1,0)(2,1)(1,2)(0,3), pair {(2,0),(3,0)}, `t = 4`, `(2,0)` in own count-2 windows only |
| B — count ≥2 both sides | Full licence both sides | Only 1.6–3.8×; at corpus roots the staged set is 61 % of the whole ball | Its opponent half buys the least, per §6.4's lemma |
| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | **2.4–6.1×** against the reported r2 ball; **MEASURED 29 % of C's Tier T lies OUTSIDE the radius-2 ball** (6.83 cells/node at corpus roots) | Asymmetric, so argued in §6.4. Residual: no cells blocking an opponent count-2 window; left to Tier Q's delta ranking, which is a set of 23.2 cells/node against a quiet allowance of 16 |
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
instrument (§8) shows C dropping a cell a proven tactic needs, C is replaced by
B — which under the threshold reading is strictly wider — and the exchange is an
amendment with its own review, never a threshold move. **And the branch revision
1 omitted:** if the instrument is GREEN while mutation M7 (Tier T at ≥3 for the
mover — option A) also SURVIVES, then the instrument has demonstrated it cannot
tell A from C, C's entire ground is unmeasured, and that is recorded as such in
the results rather than read as a confirmation of C.

**STRONGEST SURVIVING ATTACK** (abridged for the ADR line; the reviewer's full
paragraph is in the round record): *the matrix's MEASURED Tier-T column was
produced by a census reading count ≥2 while its config clause spelled count ==2 —
46.5000 against 46.3333 — so the option committed was not the option measured;
and the reduction it is bought with shrinks from 3.1× to 2.4× the moment the
depth stand-in is re-sampled from the radius-2 ball the search actually uses, a
one-second run the document did not take.* Both halves are repaired in revision
2; what survives is that neither repair was found by the author.

---

## 7. MATRIX M2 — the widening schedule — FELL

### 7.1 Why W-A fell

Three structural facts, each confirmed by this session by reading `pvs.rs`:

1. **The root can never widen.** `iterate` opens at
   `visit(depth, -INFINITY, INFINITY, 0)` (`pvs.rs:134`) with `INFINITY = MATE+1`
   (`score.rs:39`); `best_score` starts at `-INFINITY` (`pvs.rs:266`) and the
   first completed child raises it, so `best_score <= original_alpha` is
   unsatisfiable. MEASURED: 0 root widenings in 101 root iterations.
2. **A non-PV node can never truncate.** A child opens at `(alpha, alpha+1)`
   (`pvs.rs:378/380`), so any `score > alpha` sets `alpha >= beta` and breaks
   (`pvs.rs:325`). Such a node either fails high early or exhausts its batch
   having failed low — and W-A then widens it to full width. MEASURED: 0 of
   2 022 904 non-PV interior nodes truncated.
3. **A PV node that truncates stores `Bound::Exact`**, which the probe consumes
   unconditionally at every later non-PV hit. An exact score over a SUBSET is a
   lower bound only.

**So W-A caps exactly where it must not — the root and the PV, where the move is
chosen and where D-124 says no oracle will catch a mistake — and widens exactly
where widening restores full width.** Revision 1's sentence "a node that … returns
an exact score inside the window is unaffected" is what concealed it.

The reviewer's prior question, which the matrix never asked: **in this recursion a
widening trigger that is SOUND cannot narrow anything.** Narrowing requires
accepting an unsound cut somewhere; the design must name where and defend it.

Two further claims of revision 1 fell with it. The worst-case bound — "no node is
ever more expensive than full-width plus the tier bookkeeping", offered as the
reason a reader could CHECK the option — is **MEASURED false**: a fully widened
node searches a mean **84.79** cells against the incumbent's **77.96**, 1.088×,
each extra cell a subtree. And the fail-low rate marked ESTIMATED and argued
unmeasurable took **23.5 s** to measure on the shipped recursion — D-291's clause,
the second instance this round.

### 7.2 ADOPTED: W-E

- **The root (ply 0) and every PV node (`beta − alpha > 1`) are NEVER capped.**
  They search the full staged universe. This closes both structural defects at
  once: no root cap, so the engine can always play any candidate it generates;
  and no PV node ever returns a value it has not proved, so no unsound
  `Bound::Exact` is ever written.
- **Non-PV nodes carry the cut**, as an escalating batch schedule whose **last
  entry is FINITE**. Revision 1's `[0]` / "all remaining" is precisely what made
  the widening vacuous. This is a forward prune of the LMR family and the ADR
  line calls it one — scoped by a stated rule, counted per node class, and
  SPRT-judged, which is what makes it not BARE.
- **The transposition store gains a truncation rule**, removing the poisoned-entry
  class rather than living with it. A subset maximum `>= beta` is a genuine lower
  bound, so **fail-high stores `Bound::Lower` as today**; a fail-low or exact
  score from a set that was **not exhausted** is unsound in the bound it claims,
  so **it stores nothing**. The lost TT entries are a rule-5 measurement (§12.3).
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
  §5.3's licensed shortening of mate distances on lost positions both change what
  a node reports. A reader of the SPRT verdict must not read it as a pure
  generation experiment.
- **Mechanism.** The ordered universe is built ONCE per node; the loop runs over
  batches; a further batch is entered only on a fail-low so far. Widening searches
  only the ADDITIONAL cells at the same window — every already-searched cell
  scored at or below `original_alpha ≤ alpha`. (The argument is per direction: a
  cell scoring ≤ alpha did so via a child fail-high, whose `Bound::Lower` is valid
  even from a truncated set. Revision 1 asserted it undirected.)
- **`Cover::Impossible` at phase 1 inherits the same schedule** (M2 MINOR-12).
- Revision 1 recommended `[16, 0]` in §7 and committed `[0]` in §10, and its own
  validator rejects `[16, 0]`. Both are withdrawn; §10 commits one schedule.

**Registered reading, with its denominator, depth and table size pinned before
the run** (revision 1's spanned 0.84 %–99.95 % on one instrument and one corpus
depending on an unpinned denominator, crossing both its thresholds — and a LOW
rate signified not health but that the cut never bound, so the criterion was
anti-correlated with the property it certified): the widening rate is reported
**per node class** (root / PV / non-PV), denominator = non-PV interior nodes that
ran the candidate loop and whose quiet pool exceeded the first batch, at depths 2
and 3, at the committed `tt_bytes`. **The falsifiable criterion**, whose defect
class is "the cap never bites": the count of non-PV nodes that exhausted the
schedule while still truncated must be non-zero, and it is reported.

**Recorded against this WP's own conclusion**, as the reviewer recorded it
against theirs: the incumbent's root argmax sat inside the first 16 ordered
candidates in 47/47 root iterations at depth 2 and 18/18 at depth 3, and only 1
PV node in 255 had its argmax past index 16. On this corpus a root cap would not
have changed the move played. That is not a licence — it is D-124's blindness
restated from the other side, and it is why the root exemption rests on the
argument above and not on that measurement.

---

## 8. MATRIX M3 — the soundness instrument — FELL

### 8.1 Why S-C fell

**MEASURED, not argued.** D-124's own reproducer applied verbatim to
`pvs::visit` — `if cells.len() > 1 { cells.pop(); }` immediately after `order`,
confirmed applied by node counts falling 8794 → 8374, 10482 → 10045, 12260 →
11880 at depth 2 radius 2 — leaves S-C's entire class gate at **28 class
assertions, 0 RED**, identical to the pristine baseline.

D-124's defect is INTERIOR-node narrowing that never moves a ROOT maximum. S-C is
still a ROOT oracle; restricting *which roots* you assert on buys a root oracle no
interior visibility. **Revision 1 rejected S-A as "the criterion the defect class
preserves" and then adopted a criterion the same defect class preserves.**

Four more findings, each measured:

- **The forced-block class is EMPTY as written.** `ReferenceRun::argmax()` returns
  `Vec<Turn>` and a `Turn` is `Pair(Coord, Coord)`; the free second stone is never
  a hitting cell, so "the argmax set is a subset of the cells that hit every
  opponent plan" is unsatisfiable — 0 of 62 position-depth cases. The loose
  reading gives 4 positions at one depth. The reading that would admit `t = 2`
  positions IS `Cover::cells()`, i.e. mutation M3 itself.
- **M3 and M4 provably cannot fire.** Every forced-block member is
  `Minimal([One(c)])` with phase 1 `NothingToBlock` after the block, so
  inclusion-minimal and minimum-cardinality coincide and flattening changes
  nothing. M6 has no root witness: 0 corpus positions with `t ≥ 3` AND win-now.
  Three of seven mutations are killable.
- **The registered workload is not runnable.** `configs/gate_v0.toml`'s own
  committed table measures radius 2 at depth 3 as **"> 100 s" for the engine
  alone**, which is why that config exists; the full-width reference on the
  corpus's cheapest branching position at radius 2 depth 3 is **243 363 538 nodes
  in 554.2 s** — one position of thirty-one. Revision 1 attributed the cost to
  gate 10, which runs depth 3 at radius **1** on three fixtures.
- **Inside the forced-block class the argmax restraint is vacuous.** `|argmax| = 1`
  on all four members, so membership IS identity — which D-119 refuses by name —
  and the top entries differ only in the FREE second stone by 2 to 24 eval units,
  the quantity §5.3 explicitly licenses narrowing to move.

One half of the brief's own suspicion did **not** land, and the reviewer credits
it: `tactical_v0.txt`'s expectations are game facts, not generator output
(`expect cell` over `expect move`, deliberately), so a staged fixture carrying the
same rows is not the D-287/D-295 shape.

### 8.2 ADOPTED: S-E, with a reduced S-C beside it

**The stage under doubt, named** — revision 1 named the defect and never the
stage: **does the staged generator ever drop a cell a proven tactic needs?**

**S-E — per-node survival-set containment. The primary instrument.** At every node
where `blocking_covers` answers `Minimal`, assert the emitted FORCED PREFIX
contains every cell of every inclusion-minimal cover and every own win-now cell,
against a hitting set computed **independently of the generator** — the test-side
exact reference, not `ThreatState`. At every node, assert the emitted set is
non-empty and its tiers disjoint.

This is D-124's own named remedy, quoted in revision 1 and then not implemented:
*"What would help is comparing the engine's per-node candidate set with the
reference's"*. Revision 1 rejected its nearest relative (S-B) as a superset claim
D-124 never made, and priced it at a visibility cost that belonged to **WP-1.1's**
non-goal list, not this WP's — whose §11 already registers a test watching the
generated cell set at both phases. D-124's flip clause reads *"flips when that
check lands, which is where the visibility question gets settled"*; S-E settles
it.

**Where S-E observes, and against what — specified, because either answer left
implicit reproduces a defect this project has already shipped once.**

- **The observation point is the vector the candidate loop actually iterates**,
  after tiering and after any batch cut, not the generator's return value. D-124's
  reproducer is `cells.pop()` *immediately after `order`*; a check reading the
  generator's output would be blind to it for exactly the structural reason S-C
  was, and §8.1's whole argument for the replacement would be unspent. Since
  §5.4 makes the staged generator produce the ordered vector itself and `order`
  is not called under `Staged`, the two points are one place — but the design
  names it rather than relying on that coincidence surviving a later refactor.
- **The seam is a `#[cfg(debug_assertions)]` observer on `Run`**, not test-side
  scaffolding in release and not a re-implementation in the test tree. The
  alternatives are the ones D-295 rules out one work package earlier: a test-tree
  re-implementation of the staged generator would mutation-gate the test-side
  oracle and not the shipped one, which is D-295's finding verbatim. The observer
  is `None` in release, so the shipped search is unchanged; the gate runs the
  suite under `cargo test`, whose profile has `debug_assertions` on (D-128,
  D-129's taxonomy).
- **The referent is `crates/pistol-solver/tests/common/plans.rs`**, which computes
  `plan_family` and exact unbounded `t` from a `&Board`, enumerating windows by
  position and sharing no code with `ThreatState`. Deriving all inclusion-minimal
  covers of size ≤ budget from that family is new test-side work this design
  scopes here: it is a ≤2-cell vertex cover over ≤2-cell sets, the same finite
  enumeration `cover.rs` performs, written independently.

It also dissolves three of §8.1's findings at once: no argmax type confusion, no
verdict decided by a free stone's 2–24 eval units, and no comparison across two
universes the adopted Tier-T option makes non-comparable.

**Reduced S-C — a cheap mate-class regression only.** Radius 1, the three fixtures
plus one built mate-in-3 that gate 10 already affords at **MEASURED 17.89 s**. Not
a corpus sweep, not radius 2, not depth 3 at radius 2. Its mate class must state
whether `LossInTurns` roots are included or excluded **by name** — `compact_mated_in_2`
falls in neither class today.

### 8.3 The other three parts, re-scoped

- **(a) tactical suite under Staged.** The 100 % threshold is **re-derived, not
  carried**: the fixture's own pre-registration derives 100 % from "a Stage-0
  search is full-width and exhaustive inside its horizon", and the same paragraph
  says the Stage-1 class "will pre-register below 100 %". The re-derivation:
  every case is a win-now or must-block position, so Tier F carries it by
  `LAW-SUPPORT` k=1 or `LAW-FORCE`; the depth-3 cases need Tier T's k=2 licence,
  which option C at the threshold reading has; Tier Q's cut is not involved
  because the filter fires or win-now applies. The threshold this derives, pre-registered HERE because this design is the only
  candidate document for it: **`require 20`, all twenty cases**, at the three
  staged configs — with the standing rule that a case the derivation does not
  cover would have had to be pre-registered below 20 BY NAME before the first run,
  and none is: the derivation covers all twenty. If a case nevertheless fails, it
  is a red gate to investigate and never a licence to re-read the threshold
  (D-204's rule, applied to the suite that inherits it).
- **The five gate_v0 cases need a staged config.** MEASURED: `tactical_v0.txt`
  is 15 cases at `configs/instrument_v0.toml` and **5 at `configs/gate_v0.toml`**
  (radius 1, the `depth_turns 3` cases, because gate_v0's table measures radius 2
  at depth 3 as > 100 s). Revision 1 shipped one staged config. Revision 2 ships
  **three** (§10).
- **(c) colony family, ≥ 6 built cases**, distant-cluster attack and defence,
  where `LAW-DECOMP`'s star-disjointness puts the right answer in a cluster the
  delta ranking does not favour.
- **(d) re-scoped so it is about the stage.** As written it never ran the search:
  D-295 measured the pattern pack's whole contact with `crates/pistol-solver/src`
  as 33 booleans plus four `hot_windows` assertions. Revision 2 runs the §5
  pattern positions **through the staged generator** and asserts `PAT-GAP`'s
  singleton gap cell is in Tier F — `LAW-HIT`, the singleton plan must be hit.
  Confirmed available from the fixture's own data: PAT-GAP's plans are
  `{-1,0 1,0} {1,0} {1,0 6,0} {6,0 7,0}`, so `{1,0}` is a singleton and every
  minimal cover contains it.

### 8.4 The mutation ledger, with witnesses

S-D's discipline is kept — asserting an instrument's strength rather than
measuring it is D-295's finding one WP earlier — but the ledger is rebuilt,
because three of revision 1's seven mutations could not fire and one had no
witness. **Each mutation names the position it dies on, and where the corpus
cannot produce one it is BUILT** (D-260's precedent and its remedy).

| # | Mutation | Class | Witness |
|---|---|---|---|
| M1 | Tier F drops the pair-completion class | mate | `mate_in_1_two_stones_complete_a_row` (corpus) |
| M2 | Tier F drops `win_in_one_ply_cells` | mate | the ten single-stone mate-in-1s (corpus) |
| M3 | The filter flattens `Cover::cells()` at phase 0 and does not regenerate at phase 1 | S-E | **BUILT**: §5.3's two sealed five-stone rows, whose only cover is `Two{(4,4),(5,0)}` |
| M4 | Minimum-cardinality covers instead of inclusion-minimal | S-E | **BUILT**: three hot windows with empties `{a,b} {b,d} {d,e}` — `cover.rs`'s own counterexample |
| M5 | Own win-now cells dropped from the filtered set | mate | `mate_in_1_own_win_beats_blocking_the_opponent` (corpus) |
| M6 | The overload check drops its `can_win_this_turn` guard | S-E | **BUILT**: a position with opponent `t ≥ 3` AND mover win-now; 0 corpus positions have both |
| M7 | Tier T qualifies at ≥3 for the mover (option A) | informative | survival is a recorded finding under §6.5's second branch, with a diagnosis, per D-281 |

### 8.5 Floors, not printed counts

Revision 1 printed the count of positions in neither class "so the gate cannot
silently become vacuous". Printing is not a criterion: the defect it names —
classifying nothing — **preserves** it. Revision 2 registers a per-class floor
that names its witness positions, so a class whose members all disappear turns
the gate RED rather than green with a large number beside it.

### 8.6 REJ-DEPTHPROOF, stated where it belongs

Every claim this instrument makes is bounded-depth with no zone argument and is
therefore EVIDENCE and never PROOF. `REJ-DEPTHPROOF` binds us as it binds the
community. Revision 1 asserted §11 said this; §11 contained no occurrence of
`proof`, `evidence` or `DEPTHPROOF`, and §8 cited neither.

### 8.7 Gate wiring

(a)–(d) plus S-E become one script, `tools/staged_soundness_check.sh`, added to
`tools/ci.sh`. A `tools/` change: reviewed against `tools/SHELL_CHECKLIST.md`
with every item answered by name, carrying the coverage rule's test driving the
shipped script, and distinguishing RUN VOID from FAIL by name (item 12) with a
scratch preflight.

---

## 9. MATRIX M4 — the snapshot's config seam — SURVIVES AMENDED

N-A (add `--config PATH`) remains the only option that produces a Staged number
from the registered instrument; N-B (flip the committed config), N-C (a scratchpad
harness) and N-D (measure nothing) are rejected. Five amendments.

**1. The registered quantity changes.** `timing depth_at_500ms` sits at lines
89/93/97 of a 97-line record — **32 lines below the `# timing` marker** whose own
emitted text reads *excluded from every comparison*. Its resolution, MEASURED
from this session's own BEFORE ladder:

| rung | to move UP one unit | to move DOWN one unit |
|---|---|---|
| opening (d2 102 ms, d3 9339 ms) | 18.7× faster | 4.9× slower |
| early_mid (d2 118 ms, d3 1340 ms) | 2.68× faster | 4.24× slower |
| late_mid (d1 30 ms, d2 982 ms) | **1.96× faster** | 16.7× slower |

and the reviewer measured the triple **unchanged at 2 / 2 / 1** under a deliberate
16-way load that stretched the same run from 34.5 s to 66.3 s. The agreement
revision 1 reported as a reproduction is invariant under a ~2× defect in the
quantity it is made of — a criterion its own defect class preserves.

**Revision 2 registers the ABOVE-MARKER quantity**: per-position `depth_turns`
and `nodes` at the registered 50 000-node budget, plus the `ladder … nodes`
counts. That is D-190's own mechanism statistic ("radius 2 completes a second
turn-iteration in 17 of the 24 bench positions"), it is inside the invariant
block, and it is byte-invariant by construction. `depth_at_500ms` is demoted to
below-marker CONTEXT and its dead band is stated so an unmoved triple is not read
as a null result.

**2. "The fourth flag of its exact kind" is withdrawn.** `--corpus` reaches the
record through `$(basename …)`; `--config` would reach it as a whole path on TWO
invariant lines. Four guards are owed and the `argument` helper is none of them:
caller-relative resolution (as `--out` and `--binary` each got), the printable
allow-list extended to the whole `$CONFIG` path, three named refusals
(directory / missing / not a regular file), and an assertion that the script's
`config` line and the engine's `engine_id config` line name the same document.
**SHELL_CHECKLIST items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve**,
answered by name in the IMPL commit.

**3. N-B's rejection loses a cost that does not exist.** "Breaks the D-209
instrument golden transcripts" is **false**: `grep -c instrument_v0` on that
fixture is **0**; the golden is taken at `configs/gate_v0.toml`. The real
exposure is `tactical_v0.txt`'s **15** `instrument_v0`-bound cases under D-204.
The rejection stands on its three surviving grounds — rule 6's judge, the
D-190/D-194 precedent, and D-204's flip being the operator's to fire.

**4. The instrument is named with its revision, and BEFORE is re-taken.** Revision
1 invoked the instrument clause against N-C without satisfying it for N-A:
`tools/baseline_snapshot.sh` was named twice and never with a revision. And N-A
**is** a change to that instrument, so the BEFORE run — taken under the
pre-`--config` script — is re-taken under the amended one. **MEASURED 34.5 s.**
Not worth an argument.

**5. Replicate.** The run is 34.5 s and CLAUDE.md says a cheap doubt is answered
by replication, never by a margin defending a single sample. The below-marker
triple is taken three times.

---

## 10. The config shape

Three complete, `deny_unknown_fields`, no code-side default for any value:

| document | mode | `quiet_radius` | why |
|---|---|---|---|
| `configs/instrument_staged_v0.toml` | instrument | 2 | the SPRT seat and the snapshot's AFTER; the incumbent is radius 2 |
| `configs/gate_staged_v0.toml` | instrument | 1 | `tactical_v0.txt`'s five `depth_turns 3` cases run at radius 1 (§8.3) |
| `configs/play_staged_v0.toml` | play | 3 | the movetime measurement's incumbent is `play_v0.toml` at radius 3 |

**`widen_schedule` is defined against `quiet_top_k`, in QUIET CELLS, and both
ends are named.** Revision 2 left four questions an implementer would have had to
answer by invention.

- The **first** batch is `quiet_top_k` quiet cells. Tier F and Tier T are always
  emitted whole and are not counted against it (§5.4).
- The schedule's entries are **cumulative counts of QUIET cells**, not indices
  into the whole vector.
- A pool **shorter** than the first boundary never truncates, so the node is not
  counted in §7.2's registered denominator. Correct, and now stated.
- A pool **longer** than the last boundary is cut there permanently. That is what
  a finite last entry is FOR, and it is the forward prune the ADR line names.
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
# is what makes a widening schedule a rename of full width (§7).
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
greater than `quiet_top_k`**, and **no sentinel admitted**; `tier_t_own_count` and `tier_t_opponent_count` in `{2, 3}`,
mapped to a **threshold** over `LiveCount` — a value outside the set is a named
refusal, never a clamp.

`instrument_r2_v0.toml` is value-identical to the committed `instrument_v0.toml`
(D-194) and is the SPRT's incumbent seat.

**Revision 1's config comment is withdrawn.** It read "`quiet_radius = 2` so …
the SPRT's two seats differ in what they SELECT rather than in what they can
SEE". MEASURED, **29 % of option C's Tier T lies outside the radius-2 ball**
(6.83 cells/node at corpus roots), so the seats also differ in what they can see.
The comment now says so.

---

## 11. The test plan

Behaviour-named, calculus IDs in doc comments, and each states **what quantity it
watches** — the INTEG lesson every new fixture inherits.

| Test | Watches |
|---|---|
| `overload_at_entry_scores_loss_without_expansion` | the SCORE and the NODE COUNT: `-mate_in(k+2)` with no child expanded |
| `overload_check_is_not_taken_at_a_pv_node_or_the_root` | that the root's PV is non-empty on an overloaded root, and that the reported line is turn-whole where the check does fire |
| `overload_check_guarded_by_own_win_now` | the SIGN, on a position where both sides have an unhittable family and the mover can win now |
| `overload_composition_handles_completed_window_reading` | that no node reached by the protocol carries a completed window, and that a DECIDED position refuses via `StonesLeft::from_state` |
| `survival_filter_hits_all_plans_across_both_plies` | the generated CELL SET at phase 0 and again at phase 1, against an independently computed plan family |
| `defensive_union_covers_nonminimum_two_stone_splits` | the phase-1 set AFTER `a` — a flat-union generator offers both cells and never the pair, so the phase-0 set cannot discriminate |
| `mate_in_1_by_pair_generated_in_tier_f_not_ranked_in` | that both empties of the count-4 window are in the FORCED prefix, with an eval whose delta ranks them LAST |
| `own_win_now_cells_survive_survival_filter` | the win-now cell's presence where it hits nothing |
| `new_plan_creation_gets_no_forced_slot` | the absence of a plan-creating non-hitting cell from the forced prefix, with its delta made maximal |
| `tier_t_qualification_matches_adopted_matrix_option` | the Tier T set against an independent `us@{2,3} ∪ them@{3}`, on a position where exact-2 and ≥2 DIFFER — without that, the test cannot catch the defect that killed revision 1 |
| `widening_schedule_fires_on_fail_low_and_is_deterministic` | the per-class widening counters AND that a non-PV node exhausting a finite schedule while truncated exists. Two runs agreeing is a property the defect preserves and is not the criterion |
| `the_root_and_every_pv_node_search_the_full_staged_universe` | the emitted set size at ply 0 and at PV nodes against the unbatched universe |
| `a_truncated_fail_low_stores_no_transposition_record` | the table's contents after a node that stopped truncated |
| `staged_ordering_deterministic_within_and_across_tiers` | the whole emitted order, twice, with equal delta scores forced |
| `stage_counters_reported_in_search_info` | each counter non-zero on a position built to fire it, zero on one built not to |
| `gap_trap_answered_in_tier_f` | that `PAT-GAP`'s singleton gap cell is in the FORCED prefix from the defender's side |
| `colony_family_passes_under_staged` | the move played on ≥ 6 built distant-cluster positions |
| `tactical_suite_holds_at_its_rederived_thresholds_under_staged` | the `require` count of `tactical_staged_v0.txt`, at the three staged configs |
| `staged_forced_prefix_contains_every_minimal_cover_cell` | **S-E**: the per-node forced prefix against an independently computed hitting set |
| `a_win_now_node_generates_only_the_win_now_class` | the emitted set on a `Minimal` node where the mover wins by PAIR — revision 2's STOP-2, where the natural reading generated no winning cell |
| `cover_impossible_at_phase_zero_still_generates_the_win_now_class` | the emitted set at a ROOT (always a PV node, so §5.2 cannot fire) with `Cover::Impossible` and a mover win available — revision 2's STOP-1 |
| `cover_impossible_at_phase_one_with_a_win_in_one_ply_cell` | that the win cell is generated even when it lies in no Tier-T window (the reviewer's R1 construction) |
| `the_overload_budget_comes_from_stones_left_not_from_the_phase` | the budget at turn 1, where `Phase::First` carries ONE stone |
| `tier_f_is_never_truncated_by_a_batch_boundary` | the emitted Tier F against the batch count on a node whose Tier F exceeds `quiet_top_k` |
| `a_staged_table_move_is_promoted_only_within_its_own_tier` | the position of the table move when it is a Tier-Q cell and Tier F is non-empty |
| `a_radius_policy_search_is_byte_identical_to_the_committed_engine` | bestmove, nodes and PV under `Radius` before and after this WP — the scoping claim of §7.2 |
| `the_threat_state_stays_in_step_with_the_game_and_the_eval` | `THREAT_DESYNC` never fires across `place`/`undo`/`reset_to` over seeded playouts — the D-41 seam's third member |
| `the_fallback_under_staged_answers_from_the_quiet_radius_ball` | the turn `fallback_turn` returns under a Staged policy, and that it reads no threat state — the bounded, pure property WP-1.4's movetime ceiling rests on |
| `no_candidates_under_staged_is_refused_by_a_policy_agnostic_error` | the error variant at a root the policy cannot serve; `SearchError::NoCandidates { turn, radius }` names a `radius` a Staged policy has three of |
| `the_disjoint_family_early_out_agrees_with_the_exact_enumeration` | `min_hitting_set_exceeds` with and without the early-out, over the pattern pack and seeded playouts |

### 11.6 One thing this WP does NOT close

D-295's residual — `RULE-EXACT`'s "never derived by weight algebra" is unpinned in
`src`, because no `HitBudget`-shaped fixture separates `t = 3` from `t = 4` — is
not closed here. D-295 names `blocking_covers` as the differently-shaped surface
that could close it, and this WP puts `blocking_covers` on the per-node path, so a
reader will ask. S-E exercises `blocking_covers` for its ANSWERS, not for its
arithmetic's exactness. Registered for WP-1.10 (§15).

---

## 12. What this work package measures

All ADVISORY on this machine; the operator re-runs for the record.

1. **Snapshot before / after**, both under the amended script. **Registered
   quantity: per-position `depth_turns` and `nodes` at 50 000 nodes** (above the
   marker). `depth_at_500ms` reported as context with its dead band.
2. **Stage-share counters.** `SearchInfo` gains `stages: StageCounters`, all zero
   under `CandidatePolicy::Radius`. `SearchInfo` has THREE construction sites in
   `search.rs` — the completed-depth report, the `PartialRoot` salvage and the
   `Fallback` answer — and the last two zero most fields today. The counters are
   WHOLE-SEARCH totals like `nodes`, so they are written from the `Run` at the
   same point `nodes`, `nps`, `time_ms`, `seldepth_turns` and `hashfull_permille`
   are, i.e. on every path including the two salvage ones. Stated because
   revision 2 did not say, and a counter that silently reads zero on the
   wall-clock paths would make the play-mode stage shares unreadable.
   The rates: F/T/Q firing rates, the filtered-node rate, the
   `Cover::Impossible` rate, the overload-return rate, and the **widening rate
   per node class** with its denominator, depth and `tt_bytes` pinned (§7.2).
   Plus the TT entries the truncation rule declines to store.
3. **The WP-1.4 adversarial-spread debt at `movetime 500` under Staged.**
   **BASELINE MEASURED** at `f317385`, release, `configs/play_v0.toml`:

   | stones | completed depth_turns | nodes | time_ms |
   |---|---|---|---|
   | 11 | **1** | 180246 | 499 |
   | 21 | **0** | 170259 | 499 |
   | 51 | **0** | 160321 | 499 |
   | 99 | **0** | 149839 | 499 |

   Depth 0 means no iteration completed. On this class no length-6 window holds
   two stones of either colour, so Tier F and Tier T are both EMPTY and Staged
   reduces to Tier Q batching over the same ball — the cleanest possible test of
   the cut alone, with every threat mechanism inert.
4. **D-263's registered hotspot — the bracket is RECOMPUTED and the verdict
   changes.** **MEASURED**, release: `blocking_covers` 246 / 71 / 69 ns mean
   (max 1513 / 1252 / 2665 ns) and `unblockable_double_threat` 101 / 50 / 49 ns
   across corpus-roots / deepened / playout regimes; a deliberately built family
   of 16 disjoint hot windows costs 1479 ns/call, and the maximum hot count
   observed anywhere is **5**.

   Revision 2 printed "**0.6 %–3.7 % of a node**" and declined all three of
   D-263's registered remedies on it. **The bracket omitted its own worst cell.**
   Per-regime sums are 347 / 121 / 118 ns; node times from the BEFORE snapshot are
   21 277 ns (47 knps) and 3 300 ns (303 knps). Recomputed:

   | regime | sum | % of a 21 277 ns node | % of a 3 300 ns node |
   |---|---|---|---|
   | corpus roots | 347 ns | 1.63 % | **10.51 %** |
   | +1..3 turns | 121 ns | 0.57 % | 3.67 % |
   | playouts | 118 ns | 0.55 % | 3.58 % |

   Revision 2's bracket paired the two CHEAPEST regimes with the extreme node
   times and silently dropped the corpus-roots regime — which is the one the bench
   corpus itself measures. The honest mean ceiling is **10.5 %**, about 3× what
   was printed, and a single worst-case call (2665 ns) is **81 %** of a fast node.

   **So the verdict changes.** D-263 names three remedies "in order of
   directness", and revision 3 implements the **first**: a
   three-pairwise-disjoint-families early-out, which decides `HitBudget::Two`
   outright because two cells cannot meet three disjoint families. It is
   one-sided and therefore sound — a greedy scan that finds three disjoint
   families proves `t > 2`; finding fewer falls through to the exact enumeration
   unchanged — and it is O(|H|) against the enumeration's O(|H|³). The other two
   (lifting `empty_families` off the per-call path; a different enumeration) stay
   unimplemented, and the re-measurement after the early-out is reported here
   rather than assumed.

   Rule 5 wants a bracket a reader can check. Revision 2's could not be
   reproduced from its own printed numbers, and it was the sole ground for
   declining a registered remedy.

5. **The census is committed** (§6.2) at a named revision, with the
   radius-2-confined regime as the reported one.

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
   `46.3333` against the census's `46.5000`, which is what exposed the
   threshold-versus-exact defect that killed revision 1's Tier-T option. The stage
   under doubt was the census's READING of `tier_t_own_count`, the two instruments
   did not share it, and the consequence was that M1 reopened. Nothing needs
   registering after the fact; what needs recording is that the criterion which
   worked was independent re-derivation by a fresh context, and the criterion this
   document invented was an identity.

   **What is registered forward**, for the deepening sampler: the radius-2 regime
   is the reported one, and the radius-8 regime's numbers are retained as
   SUPERSEDED with the delta stated — C's reduction moved 3.1× → 2.4× and the ball
   mean 123.5 → 93.7, a MEASURED sampler sensitivity of 23 % in the ratio and 24 %
   in the ball. No verdict in this document turns on a quantity that moved less
   than that, and §6.3's cost column now states which regime each number is from.

---

## 13. Costs

No governed run: the SPRT is a deliverable here and the operator's run later.

| Item | DECLARED | MEASURED |
|---|---|---|
| `tools/ci.sh`, once | ~5 min | **5 m 20 s** (warm) |
| One baseline snapshot | ~35 s | **34.0 / 34.5 s** |
| The census harness | ~1 min | **< 1 s** per run after a 1.3 s build |
| The gate-expiry re-measurement | ~3 min | **~2 min** including a cold worktree build |
| The five DECISION-RED-TEAM rounds | ESTIMATED ~15 min each | **10.5 / 18.5 / 20.8 / 24.3 / 65.7 min** — the last is 4.4× the declaration, and it is the round that found the two blocking defects |
| The soundness gate per CI run | **ESTIMATED 40–90 s**, dominated by S-E's one traversal per fixture plus the reduced S-C's **MEASURED 17.89 s**. Revision 1's 60–180 s priced a workload that is days | to be MEASURED when it lands, and reconciled here |
| The operator's SPRT run | see the pre-registration | the operator's |

---

## 14. Non-goals

Adopted verbatim: no quiescence (WP-1.6); no killers/history/countermove
(WP-1.7); no df-pn (WP-1.8); no eval terms from `t` or `τ`; no dominance pruning
beyond the staged scheme; no `LEGAL_RADIUS` change; no ball-scan optimisation; no
`pistol-eval` refactor.

---

## 15. ADR lines this work package owes

Registered by name so none is silent. The first six are this WP's own; the last
three are corrections to LANDED lines that this review round falsified, and they
are recorded rather than fixed, because they are outside this WP's mandate.

1. Item 0's gate supersession — option (f), with §4.4's surviving attack.
2. Tier-T option C at the threshold reading, with §6.5's surviving attack and the
   count-3-leg lemma's two named gaps.
3. W-E, naming the non-PV cut as a forward prune, the TT truncation rule, and the
   cut's binding under `Staged` only.
4. S-E, its `#[cfg(debug_assertions)]` observation seam and its independent
   referent; D-124's flip clause discharged.
5. **M5-A**, the overload realisation, with the strongest surviving attack from
   its DECISION-RED-TEAM; M5-D (storing the verdict) registered as a follow-up
   once §7.2's truncation rule is measured.
6. **The `Position` seam gains a third member.** D-41's content is that
   `GameState` and `Eval` move from one place BECAUSE they are only correct
   together; `Option<ThreatState>` joins them, and `THREAT_DESYNC` is the same
   failure class as `POSITION_DESYNC` and `EVAL_DESYNC`. An amendment to D-41,
   with the desync test §11 now registers.
7. **D-263's first remedy IS implemented** — the three-pairwise-disjoint-families
   early-out — because the recomputed bracket (§12.4) reaches 10.5 % of a fast
   node rather than the 3.7 % revision 2 printed. The other two remedies stay
   unimplemented, with the re-measurement recorded.
8. `LAW-RIPOSTE` and `LAW-LEDGER` hand off to WP-1.6; `THM-WINDOW`'s exact-`t`
   eval terms registered; the Tier-Q ball scan stands (WP-1.5c); the fallback
   under Staged reuses the `quiet_radius` ball and `SearchError::NoCandidates`
   gains a policy-agnostic shape; D-295's `RULE-EXACT` residual (WP-1.10).
9. **The dominance-pruning half of ROADMAP WP-1.5 is deferred, not dropped.**
   §14 says "no dominance pruning beyond the staged scheme"; the ROADMAP sentence
   that item 10 takes a line for reads "staged pair generation **with dominance
   pruning** SUPERSEDES the radius policy". The ROADMAP changes only by ADR, so
   both halves take a line, not one.
10. WP-1.5b does not complete ROADMAP WP-1.5's supersession — the operator's SPRT
    does.
11. **The licensed value change**: mate distances on already-lost positions may
    shorten under the filter (§5.3). It changes what a printed score means
    (D-3, D-72) and is attributable here rather than to an unnamed line.
12. **D-255 is wrong on a number it states.** It says "the corpus shows own-side
    hot = 0.0 mean / 0 max at both stone counts". MEASURED: **0.0417 mean, max 1**
    — at corpus root index 16, 31 stones, which is in the **35-band** (that
    fixture's "band centre 35 width 5"), so the line's "both stone counts" is
    false for the 35-band specifically. D-255's deciding argument rests on the
    absolute count-1 traffic, not on this number, so the decision it carries is
    unaffected; rule 10 says amend rather than drift past.
13. **D-299(2) is half false at this revision.** It records `assert_code` as
    lifted into `decision_key_check_tests.rs` AND `arena_smoke_gate_tests.rs`.
    MEASURED: `fn assert_code` exists only in `solver_link_check_tests.rs:151` and
    `decision_key_check_tests.rs:120`; `arena_smoke_gate_tests.rs` still asserts
    `!ran.status.success()` at 347, 410 and 441 — so the gate D-299(4) gave a void
    class in the same round still cannot tell a void from a failure. WP-1.10.
14. **`tools/solver_edge_check.sh:103` uses bash's logical `pwd`** where its
    sibling uses `pwd -P`, so a symlinked root defeats its `<workspace>`
    substitution. Fixed here because (f) depends on it; recorded because it is a
    latent defect in a shipped gate that predates this WP.
15. **Rule 9 and `pvs.rs`.** The file is 552 lines and carries a
    `RULE9-JUSTIFICATION` ending "Stage 1 moves candidate generation out entirely
    (D-117, WP-1.5)". This WP adds the node protocol and the batch loop INTO
    `visit` while moving generation OUT to a new `staged` module, so that
    justification's forward-looking clause is discharged and its text must be
    re-read at the landing commit rather than left asserting a future that has
    arrived.

---

## 16. The review record

| Round | Against | Verdict |
|---|---|---|
| DECISION-RED-TEAM ×5, one per matrix | revision 1, `ec8f7fb` | M0 **FELL** → (f); M1 survives amended; M2 **FELL** → W-E; M3 **FELL** → S-E; M4 survives amended |
| REVIEW-design | revision 2, `182f389` | **FAILS** — 2 STOP, 1 BLOCKING, 7 MAJOR, 10 MINOR, 8 findings REJECTED with their attempted reproducers |
| DECISION-RED-TEAM, matrix M5 | revision 3 | dispatched |
| REVIEW-design | revision 3 | dispatched |

Three of revision 2's four new defects were consequences of revision 2's own
repairs that were not propagated — the `!is_pv` gate not reaching §5.3's
reachability claim, the threshold reading not reaching §12.5's criterion, and the
re-sampled regime not reaching §6.3's cost column. That is the pattern this
document records for its successor: **a repair is not done until every claim that
rested on the broken thing has been re-read.**

What the REVIEW-design attacked and could NOT break, which the implementer may
rely on: the `-mate_in(k+2)` distance and sign; the `!is_pv` gate's sufficiency
for a non-empty root PV and a turn-whole reported line; `blocking_covers`'s side
convention and `hot_windows` as `DEF-PLAN`'s plan set; the `Minimal` row's two-ply
pair completeness; the TT's fail-high `Bound::Lower` from a truncated set, and
the further result that `Bound::Exact` is UNREACHABLE at a non-PV node (`beta =
alpha+1` leaves no integer strictly between), which makes §7.2's soundness
argument complete for the whole table rather than only for PV nodes; determinism
across the staged tiers; and every checkable MEASURED count in §0, §4, §7.1, §9
and §15.

---

*Revision 3. Reviews owed: a DECISION-RED-TEAM against matrix M5, and a fresh
REVIEW-design against this revision. IMPL does not start until both pass.*
