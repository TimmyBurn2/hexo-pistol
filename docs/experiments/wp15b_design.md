# WP-1.5b — staged threat-first candidate generation: DESIGN

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
if !is_pv && phase == First
   && can_win_this_turn(us, Two).is_none()
   && unblockable_double_threat(them, HitBudget::Two)
{ return -mate_in(turns_from_root + 2) }
```

- **The guard is step 1's answer**, evaluated first and unconditionally (D-243
  (3), D-257 (a)/(b)).
- **The distance is exact and is `k + 2`.** At a node `k` turns from the root it
  is our turn — the `(k+1)`-th — and the opponent completes six on the `(k+2)`-th.
  They cannot win sooner and cannot fail to win later: `t > 2` means no two cells
  hit every plan, and `LAW-HIT` says hitting is the only defence. Checked against
  `score.rs`: `visit` carries mate scores root-relative in distance and
  node-relative in sign, so the value is `-mate_in(k+2)`, and `k+2 <= 130` sits
  far inside `MAX_MATE_TURNS = 1000`.
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

### 5.3 Step 3 — the survival filter (`LAW-FORCE` + No-Counterattack)

With `budget = HitBudget::from(StonesLeft::from_state(state))`:

| `blocking_covers(us, budget)` | Meaning | Generation |
|---|---|---|
| `NothingToBlock` | `t = 0` (`PAT-RHOMBUS`, `PAT-O3`) | No filter. Tier F ∪ T ∪ Q, batched per §7 |
| `Minimal(covers)` | `1 ≤ t ≤ budget` | **Filtered.** Exactly Tier F: the union of cells over the inclusion-minimal covers, plus own win-now cells. No Tier T, no Tier Q, no batching |
| `Impossible` | `t > budget` | At phase 0 this is step 2 and is not reached here. At phase 1 the position is lost and the search must reach that through the generated moves: Tier T ∪ Tier Q, **batched per §7 like any unfiltered node** (revision 1 left this row a bare cap the matrix never considered — M2 MINOR-12), and counted |

**Why the filtered set is complete for non-losing moves.** `LAW-FORCE` is
`[PROVEN]`: if the opponent has ≥1 plan and the mover cannot win this turn, every
non-losing mover move hits ALL opponent plans; its corollary is that
counter-threats never substitute for hitting, except win-now. So the filtered set
contains every non-losing move and a further stage could only add losing ones.

**The two-ply realisation — VERIFIED on the shipped solver before implementation.**
`Cover::Minimal` carries SETS because the union is provably insufficient (D-257).
The search is ply-level, so a cross-window pair is reached across two plies. Run
at `f317385` against the shipped `ThreatState`, two disjoint sealed five-stone P1
rows, P2 to move with two stones:

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

**One licensed value change, stated rather than discovered.** Under the filter the
search no longer prefers the longest resistance among losing moves. `LAW-FORCE`
licenses this — those moves lose — so mate DISTANCES on already-lost positions
may shorten under Staged. Recorded in the ADR line.

### 5.4 Step 4 — staged generation

1. **Tier F** — never delta-ranked, emitted ascending `(q, r)`. Delta is not
   called for Tier F cells at all, which is what makes a filtered node cheap.
2. **Tier T** — `LAW-SUPPORT`-qualified per §6. Delta-ranked, stable sort.
3. **Tier Q** — the remaining cells of the `quiet_radius` ball, delta-ranked,
   batched per §7.

Tiers are disjoint by construction. Every staged cell is filtered through
`Board::is_legal_placement`, one cell at a time: D-243 proves every Tier F and
Tier T cell is inside rule 5's region, but D-77 forbids exactly the shortcut "the
radius is at most eight, so every policy cell is legal" and D-20 forbids
comparing the two radii. Because that proof says no Tier F cell can be dropped, a
Tier F cell the rules refuse is a **named refusal**, never a silent drop.

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

| option | Tier T cells | staged set | vs radius-2 full-width |
|---|---|---|---|
| A — count ≥3 both sides | 6.12 / 7.03 / 6.65 | 16.8 / 19.9 / 21.6 | 4.6× / 6.2× / 17.4× |
| B — count ≥2 both sides | 46.50 / 51.66 / 88.13 | 47.5 / 58.6 / 98.6 | 1.6× / 2.1× / 3.8× |
| **C — ≥2 for us, ≥3 for them** | 23.29 / 30.3 / 48.7 | **28.6 / 39.6 / 61.4** | **2.7× / 2.4× / 6.1×** |

### 6.3 The options

| Option | Theory standing | Cost | Failure modes |
|---|---|---|---|
| A — count ≥3 both sides | **No completeness licence.** `LAW-SUPPORT` k=2 licences windows at ≥2, and T10 adds that a window made hot this turn held ≥2 before — so count 3 misses every plan a PAIR creates from a count-2 window, which is the two-stone move this game is about | Cheapest, 4.6–17.4× | Provably k=2-incomplete. The reviewer built the position: P1 (0,0)(1,0)(2,1)(1,2)(0,3), pair {(2,0),(3,0)}, `t = 4`, `(2,0)` in own count-2 windows only |
| B — count ≥2 both sides | Full licence both sides | Only 1.6–3.8×; at corpus roots the staged set is 61 % of the whole ball | Its opponent half buys the least, per §6.4's lemma |
| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | **2.4–6.1×**; **MEASURED 29 % of C's Tier T lies OUTSIDE the radius-2 ball** (6.83 cells/node at corpus roots) | Asymmetric, so argued in §6.4. Residual: no cells blocking an opponent count-2 window; left to Tier Q's delta ranking, which is a set of 23.2 cells/node against a quiet allowance of 16 |
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
  because the filter fires or win-now applies. Any case the derivation does not
  cover is pre-registered **below** 100 % by name.
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
`quiet_top_k >= 1`; `widen_schedule` non-empty and strictly increasing, **no
sentinel admitted**; `tier_t_own_count` and `tier_t_opponent_count` in `{2, 3}`,
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
2. **Stage-share counters** — F/T/Q firing rates, the filtered-node rate, the
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
4. **D-263's registered hotspot, discharged.** **MEASURED**, release:
   `blocking_covers` 246 / 71 / 69 ns mean (max 1513 / 1252 / 2665 ns) and
   `unblockable_double_threat` 101 / 50 / 49 ns across the three regimes; a
   deliberately built family of 16 disjoint hot windows costs 1479 ns/call, and
   the maximum hot count observed anywhere is **5**. Against the 47 000–303 000
   nps of the BEFORE snapshot the two queries together are **0.6 %–3.7 % of a
   node**. D-263's three remedies are therefore **NOT implemented**, and that is
   the finding: the pre-registered hotspot is not hot at the counts the generator
   produces. Rule 5's "a measured structural floor is a finding, not a failure",
   in the other direction.
5. **The census is committed** (§6.2) at a named revision, with the
   radius-2-confined regime as the reported one and the radius-8 regime as the
   **second instrument**. Agreement criterion, registered before either runs: the
   two regimes must agree on the RANKING of options A, B and C by staged-set size
   at every stone band. **Registered consequence:** disagreement on the ranking
   voids §6.2's cost column and reopens M1; agreement with a magnitude difference
   is reported and changes nothing, because the ranking is what the matrix rests
   on. The stage under doubt is the DEEPENING SAMPLER, and the two regimes do not
   share it — they differ in exactly that stage and in nothing else.

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
   count-3-leg lemma's named gap.
3. W-E, naming the non-PV cut as a forward prune and the TT truncation rule.
4. S-E, and D-124's flip clause discharged.
5. `LAW-RIPOSTE` and `LAW-LEDGER` hand off to WP-1.6; `THM-WINDOW`'s exact-`t`
   eval terms registered; the Tier-Q ball scan stands (WP-1.5c); the fallback
   under Staged reuses the `quiet_radius` ball; the overload witness line
   (WP-1.6); D-295's `RULE-EXACT` residual (WP-1.10).
6. WP-1.5b does not complete ROADMAP WP-1.5's supersession — the operator's SPRT
   does. The ROADMAP changes only by ADR.
7. **D-255 is wrong on a number it states.** It says "the corpus shows own-side
   hot = 0.0 mean / 0 max at both stone counts". MEASURED: **0.0417 mean, max 1**,
   at corpus root index 16 (31 stones). D-255's deciding argument rests on the
   absolute count-1 traffic and not on this number, so the decision it carries is
   unaffected — but rule 10 says amend rather than drift past.
8. **D-299(2) is half false at this revision.** It records `assert_code` as lifted
   into `decision_key_check_tests.rs` AND `arena_smoke_gate_tests.rs`. MEASURED:
   `fn assert_code` exists only in `solver_link_check_tests.rs:151` and
   `decision_key_check_tests.rs:120`; `arena_smoke_gate_tests.rs` still asserts
   `!ran.status.success()` at lines 347, 410 and 441 — so the gate D-299(4) gave a
   void class in the same round still cannot tell a void from a failure in its own
   suite. WP-1.10.
9. **`tools/solver_edge_check.sh:103` uses bash's logical `pwd`** where its
   sibling uses `pwd -P`, so a symlinked root defeats its `<workspace>`
   substitution. Fixed here because (f) depends on it; recorded because it is a
   latent defect in a shipped gate that predates this WP.

---

*Revision 2. Reviews owed: a fresh REVIEW-design against this revision. The five
DECISION-RED-TEAM reports against revision 1 are the round of record; the three
options NEW in this revision — M0 (f), M2 W-E, M3 S-E — were each supplied by the
red team that killed their predecessor.*
