# WP-1.5b — staged threat-first candidate generation: DESIGN

Revision 1. Base revision `f317385`, `dev`, tree clean, `tools/ci.sh` green (all
14 gates; log cited in §1.2). This document is the DESIGN artefact the Process
section requires: it carries the option matrices, and each matrix is attacked by
a fresh-context DECISION-RED-TEAM before its option is selected. Nothing in
`crates/` changes until REVIEW-design has passed against this revision.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This document restates no theory; where it appears to, the calculus
wins and the disagreement is an ADR line.

---

## 1. Preconditions, verified rather than assumed

### 1.1 What was checked

| Precondition | Verdict | Evidence |
|---|---|---|
| INTEG landed and its review round closed | HELD | `docs/decisions.md` D-287 (the §5 pattern pack) through D-300; the round's REVIEW-impl / RED-TEAM / DECISION-RED-TEAM findings at `6b03899` are each recorded with a reproducer and fixed in commits `c875016`..`f317385`. One item is recorded OPEN and not closed: D-295's structural residual (no `HitBudget`-shaped fixture can separate `t = 3` from `t = 4`). That is a recorded disposition, not an outstanding finding — see §11.6, which is this work package's contact with it. |
| The calculus is in-repo | HELD | `docs/research/threat_calculus_v1.md`, adopted verbatim by D-266. |
| The pattern fixture pack is green | HELD | `tests/pattern_calculus_tests.rs`, 14 tests, all `ok` in gate 3 of the base run — including `pattern_v0_fixture_matches_its_pinned_sha256`, `gap_trap_singleton_plan_detected`, `threat_number_is_never_additive` and `the_shipped_minimal_covers_are_the_hitting_sets_of_the_plan_family`. |
| Arena digest-binding is in place | HELD | D-283, D-294(1); `crates/pistol-arena/tests/binary_binding_tests.rs`, 7 tests all `ok`, including `every_committed_arena_config_binds_every_seat_by_content`. |
| CI green at the base revision | HELD | `tools/ci.sh` exit 0, `ci: all gates passed`, MEASURED 5 m 20 s wall on this machine (warm target directory). |

### 1.2 One precondition artefact does not exist, and this is recorded rather than worked around

The prompt names "the prior 1.5b design report (its option matrices RESUME
here)". **There is no such document in this repository**, at any revision:
`docs/` holds `wp13_prereg.md`, `wp13_results.md`, `delta_reprofile_prereg.md`
and `wp15a_prereg.md` and nothing for 1.5b, and no session scratchpad holds one
either. The matrices below are therefore authored here rather than resumed, and
their prior is the ADR record that WAS written for this work package before it
started: D-243 (the hot-window theorem, written once so two work packages could
not drift), D-249, D-255, D-257, D-261, D-263 (this WP's pre-registered
hotspot), D-267 (the calculus dictionary) and `docs/ROADMAP.md`'s WP-1.5 entry.
This is stated so that a reviewer looking for a predecessor does not conclude the
matrices dropped one.

### 1.3 One prompt claim is corrected against the artefact

The prompt says D-267's prose "said *eight* over what reads as nine IDs". D-267
contains no count word at all: the absence paragraph reads «`PROTO-PAIR`'s
intra-turn phase bit is `pistol-core`'s zobrist and NOT this crate's; `DEF-STAR`,
`DEF-TEMPO`/`ADOPT-TEMPO`, `LAW-RIPOSTE`, `LAW-LEDGER`, `LAW-DECOMP`, `ZONE-R`,
`THM-WINDOW` and `E-INIT` have NO counterpart anywhere in the shipped surface».
The count-vs-list hazard the prompt warns about is real and is handled by
counting the list (§3): **nine entries naming ten IDs**, of which PROTO-PAIR is
called out separately as living in `pistol-core` and the remaining eight are the
"no counterpart" list. `eight` appears in this log at D-13, D-242 and D-247, none
of them about this ledger. Trusting the artefact is what the prompt asked for;
this records what the artefact says.

---

## 2. What this work package changes

### 2.1 The shape

`CandidatePolicy` gains a second variant, `Staged`. `pistol-search` takes a
normal dependency on `pistol-solver` — the first structural act, and the one
D-282 measured the gates' expiry against (Item 0, §4).

`pistol_search::position::Position` already moves `GameState` and `Eval` from one
place because they are only correct together (D-41). It gains a third member,
`Option<ThreatState>`, moved from the same place. `Option`, not a bare field:
under `CandidatePolicy::Radius` nothing consults it, and a radius search that
paid for a threat state it never reads would make the SPRT's incumbent slower for
no reason — a measurement hazard in the direction that flatters the change under
test. One `if let Some` per placed stone is the whole cost of the conditional.

`pvs::visit` gains the node protocol (§5) and a batched candidate loop (§7).
`SearchInfo` gains stage counters (§9). Nothing in the line protocol's output
changes, so the D-209 golden transcripts stay green by construction.

### 2.2 What does NOT change

`configs/instrument_v0.toml` and `configs/play_v0.toml` stay at `kind =
"radius"`. Staged ships as `configs/instrument_staged_v0.toml`, a
config-selectable policy exactly as `docs/ROADMAP.md` WP-1.5 words it ("radius
stays as a config-selectable fallback"), and the committed default moves only
when SPRT rules — which is the operator's run, after this session. This is the
CONSERVATIVE/INCUMBENT branch the session policy requires for a decision outside
the implementing mandate, and it is also what D-190/D-194 did: the H1 action that
moved the committed radius landed *after* the games, not before them.

Consequences worth naming so nobody rediscovers them:

- `tactical_v0.txt` keeps binding at the radius config (D-204). Staged gets its
  own sha-pinned `tactical_staged_v0.txt`, which is what `docs/ROADMAP.md`
  already schedules for this work package.
- D-204's flip clause — "when WP-1.5's threat-first generation supersedes the
  radius policy entirely" — is **not** fired by this work package. It fires when
  the operator's SPRT moves the committed config. Recorded because the clause
  names this WP by name and a reader would otherwise expect it to fire here.

---

## 3. MANDATORY DESIGN ITEM 1 — the D-267 ledger, dispositioned

**The count, taken from the artefact.** D-267's absence paragraph holds NINE
entries naming TEN IDs. Every one is dispositioned below; none is silent.

| # | ID(s) | Disposition | Where |
|---|---|---|---|
| 1 | `PROTO-PAIR` | **ALREADY IMPLEMENTED, mapping recorded.** Verified against shipped code, not against a doc comment: unordered-pair canonicalisation is `pistol_core::turn::canonical_pair` (`Turn::Pair(a, b)` with `a < b`, `Turn::is_canonical`), landed in WP-03/04; the intra-turn phase bit is `pistol_core::zobrist::phase_key`, mixed into `GameState::key` beside `side_key`. Both predate this WP and neither moves. | `crates/pistol-core/src/turn.rs`, `src/zobrist.rs` |
| 2 | `DEF-STAR` | **DEFERRED to WP-1.8 (df-pn).** The star S₅(c) is the relevance-zone primitive; nothing in staged generation needs a star, because the tiers are defined by window membership and not by geometric reach. Naming a WP rather than "later": WP-1.8 is where `ZONE-R`/`ADOPT-RZOP` land and the star is that construction's unit. | — |
| 3 | `DEF-TEMPO` / `ADOPT-TEMPO` | **DEFERRED to eval-terms.** τ is an eval-term candidate by the calculus's own §6 note, SPRT-gated, and this WP adds no eval term (non-goal). | — |
| 4 | `LAW-RIPOSTE` | **DEFERRED to WP-1.6 (quiescence).** The forced-reply plan check belongs to a forcing-line prover; this WP has no prover and no quiescence. Handoff recorded in one ADR line as the prompt requires. | — |
| 5 | `LAW-LEDGER` | **DEFERRED to WP-1.6.** t = 1 chain semantics and the 2 − t free-stone bank are quiescence-stand-pat questions. Same ADR line. | — |
| 6 | `LAW-DECOMP` | **DEFERRED to Stage 3 (TSS/DBS).** Additivity of t over star-disjoint regions is a proof-decomposition licence. Note the calculus's own warning is already respected here: it is FALSE for static evaluation, and this WP sums no regional eval. | — |
| 7 | `ZONE-R` | **DEFERRED to WP-1.8 / Stage 3.** Every claim this WP makes is bounded-depth and is therefore EVIDENCE and never PROOF (`REJ-DEPTHPROOF`, which the calculus says binds us too). §11 states that explicitly for the soundness gate, so no report from this WP can be read as a proof. | — |
| 8 | `THM-WINDOW` | **NOT CONSUMED; REGISTERED BY NAME.** The prompt's non-goal ("no eval terms from t or tau") is adopted, and the follow-up — exact-t counters as eval terms, which the calculus calls the cheapest fix and "free from PROTO-NODE" — takes one ADR line as a named registration. It becomes cheap exactly because this WP puts a `ThreatState` on the search's per-node path; that is the connection, and it is recorded rather than acted on. | ADR line |
| 9 | `E-INIT` | **DEFERRED to eval-terms.** `[CONJ]`, SPRT-gated by the calculus itself. | — |

**The two places the code is deliberately weaker than the law**, dispositioned
explicitly as the prompt requires. Both are CONSUMED by this WP, and each gets a
test that pins the composition:

- **`min_hitting_set_exceeds`'s completed-window reading.** The predicate reads
  "`t` exceeds the budget OR the family contains a won window" (D-267), because a
  `Completed` window has no empty cell and is unhittable rather than three-hard.
  Composition here: the overload check (§5, step 2) is reached only at a node the
  search has NOT already ended, and `visit` returns on `PlyOutcome::Win` before
  any child is entered, so no node the check runs at has a completed window for
  either side. That is an argument, so it is also an assertion and a test:
  `overload_composition_handles_completed_window_reading`.
- **`unblockable_double_threat` carrying LAW-OVERLOAD minus its guard.** The
  law's "defender cannot win this turn" clause lives in the caller (D-243
  consequence (3), D-257). Composition here: step 1 of the node protocol runs
  first and unconditionally, and step 2 is entered only when step 1 found no
  win. Pinned by `overload_check_guarded_by_own_win_now`. The exemplar in
  `threat_query_tests.rs::composed_win` is the recipe this follows, D-257's own
  amendment included: `GameState::outcome()` first, `StonesLeft::from_state`
  second, `state.to_move() != side` never.

---

## 4. MANDATORY DESIGN ITEM 0 — gate supersession

### 4.1 The expiry, re-measured at this revision

D-282 measured the expiry at `d6f6cbb`. Re-measured here at `f317385`, in a
git worktree on `/home`, by adding exactly the edge this WP adds
(`[workspace.dependencies] pistol-solver`, plus the member's own line —
**MEASURED**: `pistol-solver` is not in `[workspace.dependencies]` today, so the
linking commit touches three files, `Cargo.toml`, `Cargo.lock` and
`crates/pistol-search/Cargo.toml`, and a session that changed only the member
manifest gets `cargo` exit 2 from both gates for a stale lock rather than the
verdict it expected):

```
tools/solver_edge_check.sh  .  pistol-solver     -> exit 1, 6 lines
tools/solver_link_check.sh  .  crates/pistol-solver -> exit 1, 30 hits over 5 binaries
```

and the edge gate's whole printed answer, which matters for option (E):

```
solver_edge_check: inverted normal-edge tree for pistol-solver (6 lines)
solver_edge_check:   pistol-solver v0.0.1 (<workspace>/crates/pistol-solver)
solver_edge_check:   └── pistol-search v0.0.1 (<workspace>/crates/pistol-search)
solver_edge_check:       └── pistol-engine v0.0.1 (<workspace>/crates/pistol-engine)
solver_edge_check:           ├── pistol-arena v0.0.1 (<workspace>/crates/pistol-arena)
solver_edge_check:           └── pistol-cli v0.0.1 (<workspace>/crates/pistol-cli)
solver_edge_check:               └── pistol-arena v0.0.1 (<workspace>/crates/pistol-arena)
solver_edge_check: pistol-solver HAS normal reverse-dependencies — the lines above name them
```

**Two facts this measurement adds to D-282's framing**, both load-bearing below
and neither in D-282:

1. The gate SCRIPTS do not expire at all. Both take `<workspace-root>` and a
   subject as ARGUMENTS, and their suites drive them against scratch workspaces
   the tests control. Of `solver_edge_check_tests.rs`'s 8 tests and
   `solver_link_check_tests.rs`'s 20, exactly ONE each is an assertion about
   THIS workspace: `the_shipped_workspace_has_no_normal_edge_on_the_solver` and
   `no_solver_source_reaches_any_shipped_binary_of_this_workspace`. What expires
   is two test functions, not two gates. Every framing below is about those two.
2. The edge gate already substitutes `<workspace>` for the absolute root before
   printing (its own defence against a `mktemp` path reaching a record). **The
   link gate does not** — MEASURED, its 30 hit lines carry the absolute worktree
   path twice each. So an inverted edge assertion can pin the gate's literal
   output and an inverted link assertion cannot, without a `tools/` change.

### 4.2 MATRIX M0 — what happens to the two live-workspace assertions

Numeric claims are marked **MEASURED** or **ESTIMATED** per CLAUDE.md's
option-matrix clause (D-291, as corrected by D-293).

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| **(a) RETIRE both** | Delete both live assertions; keep both scripts and their 26 scratch-workspace tests. | One commit; **MEASURED** 2 test functions deleted, 0 lines of `tools/` changed. | Loses the standing guard against an ACCIDENTAL edge in a world where a deliberate one exists — e.g. `pistol-eval` or `pistol-core` taking a solver dependency, a layering inversion nothing else in CI sees. |
| **(b) INVERT both** to "linked exactly where declared" | Both live assertions become declared-list assertions. | Edge: test-only. Link: **requires a `tools/` change** — the printed paths are absolute (MEASURED), so pinning them needs `<workspace>` substitution added to `solver_link_check.sh`, which reopens SHELL_CHECKLIST for a script whose review round closed 4 commits ago. | D-282's own caveat: a declared list maintained by memory (D-275's lesson). Worse for the link gate specifically: post-linkage its answer is "all 6 solver sources reach all 5 binaries", which is ENTAILED by the edge and cannot distinguish a legitimate route from an accidental one — the gate exists to separate a graph claim from a codegen claim, and once the graph claim is YES it has nothing left to separate. |
| **(c) INVERT the edge, RETIRE the link** — as a declared list | Edge assertion becomes "the direct dependents of `pistol-solver` are exactly {`pistol-search`}"; link live assertion deleted. | Test-only; **MEASURED** 0 lines of `tools/`. | Still a list maintained by memory. A list of one name is a small memory, but it is memory, and it pins only depth 1: `pistol-cli` gaining a direct edge would be caught, `pistol-core` gaining one would not change the depth-1 set for the solver but WOULD change the tree. |
| **(d) KEEP BOTH RED and mark them expected-fail** | `#[ignore]`, or an inverted `assert!(!status.success())`. | Trivial. | A gate that asserts its own failure asserts nothing: `assert!(!success)` is satisfied by exit 2 — the VOID code — so a workspace `cargo` cannot resolve at all would pass it. That is the exact class `assert_code` was lifted across three suites to kill (D-299(2)). Rejected on this ground alone. |
| **(e) INVERT the edge as a GOLDEN TRANSCRIPT, RETIRE the link's live assertion** | The edge live assertion pins the gate's ENTIRE printed answer byte-for-byte (the 6-line block above, `<workspace>`-substituted, exit 1 asserted through `assert_code`). The link live assertion is deleted, and the script plus its 20 scratch tests stay as the standing crate-isolation instrument. | Test-only; **MEASURED** 0 lines of `tools/` changed, 1 test rewritten, 1 deleted. | The transcript must be updated by any legitimate graph change — a new crate, a new edge — which is a maintenance cost. It is NOT a memory cost: the expected text is the gate's own output and a mismatch prints the diff. Residual: it pins the graph and says nothing about codegen; the codegen question is the link gate's, and post-linkage that question has no discriminating answer left, which is why (e) retires it rather than pretending otherwise. |

**RECOMMENDATION: (e).** The reasons, in order of weight.

- It answers D-282's stated objection to inversion instead of accepting it. The
  objection is "an inverted gate must be told what 'as declared' means and
  becomes a list maintained by memory". A golden transcript of the gate's own
  output is not told anything: it is the output. Any graph change at all — a
  crate added, an edge added anywhere in the transitive cone, an edge removed —
  moves the text and turns the assertion red with the difference printed. That is
  strictly more than "the direct dependents are {pistol-search}", and it costs
  less to maintain than a hand-written list because it is regenerable by running
  the gate.
- It is the same instrument this project already trusts for exactly this shape of
  claim: the instrument golden transcripts (D-209) and the arena report's
  invariant block (D-161) both pin a tool's own output byte-for-byte, and the
  edge gate was already built to be pinnable — the `<workspace>` substitution
  exists because a per-run directory name in a byte-compared record voids every
  replication (its own header says so).
- It retires only what genuinely expired. The link gate's live assertion is the
  one thing here that becomes uninformative: after the edge exists, "solver
  sources reach shipped binaries" is entailed and unfalsifiable-in-the-useful-
  direction. Keeping it inverted would be a criterion the named defect class
  preserves, which is the vacuous-criterion shape D-269 put into CLAUDE.md.
- It costs no `tools/` change, so no `tools/` review round reopens for a
  work package whose subject is the search.

**What (e) deliberately does not claim.** It does not claim the accidental-edge
route is closed. An `include_str!` from `pistol-search` into `pistol-solver`
would change neither the graph nor the transcript, and the link gate — the only
instrument that ever saw that route (D-276's ground for choosing E over D) — is
the one whose live assertion (e) retires. That residual is real and is recorded
in the ADR line rather than argued away; the honest statement is that after
linkage there is no instrument that can distinguish a legitimate codegen route
from an accidental one, because both are legitimate now.

---

## 5. The normative node protocol, adapted to the two-ply turn

`PROTO-NODE`, with `E-PHASE` conditioning. Every step below cites the calculus ID
it implements; the adaptation is the two-ply structure and nothing else.

### 5.1 Step 1 — win-now

**`PROTO-NODE` step 1, `LAW-SUPPORT` k=1, `E-PHASE`.** Realised as GENERATION,
not as an early return, and the reason is the principal variation: `visit`
returns a score AND leaves a line at its ply, and a node that returns a mate
score without placing the stone leaves the parent promoting a line that stops
short. The existing `PlyOutcome::Win` arm already scores rule 4's truncation
exactly (`mate_in(turns_from_root + 1)`, D-72), so the whole of step 1 is
"generate the winning cells first and let one node of search find them".

Tier F therefore always contains, for the side to move:

- `win_in_one_ply_cells(us)` — every size-one plan, valid at either budget. This
  is where `PAT-GAP`'s Gap Trap lands from the attacking side.
- at `StonesLeft::Two` only, **both** empties of every own hot window holding
  exactly four stones — `WinWitness::Pair`'s class, generated in full rather than
  as `can_win_this_turn`'s single witness, because the witness is a witness and
  not advice (D-267). This is D-243 consequence (2): a delta ranking cannot be
  trusted to surface both halves of a pair whose first stone is worth little
  alone.

At `Phase::Second` the pair class is not generated: one stone is left and a
two-empty hot window is not a win now. D-243 states this conditioning as the
theorem's own; the type states it too (`StonesLeft`), which is why the code
cannot get it wrong silently.

### 5.2 Step 2 — overload at node entry

**`LAW-OVERLOAD` with its guard.** At `Phase::First` only:

```
if can_win_this_turn(us, Two).is_none()
   && min_hitting_set_exceeds(HitBudget::Two, hot_windows(them))   // via unblockable_double_threat(them, Two)
{ return -mate_in(turns_from_root + 2) }
```

- **The guard is step 1's answer**, per D-243 consequence (3) and D-257 clause
  (a)/(b). It is evaluated first and unconditionally.
- **The distance is exact and is `k + 2`.** At a node `k` turns from the root it
  is our turn — the `(k+1)`-th from the root — and the opponent completes six on
  the `(k+2)`-th. They cannot win sooner (they have no turn sooner) and they
  cannot fail to win later: `t > 2` means no two cells hit every plan, and
  `LAW-HIT` says hitting is the only defensive mechanism, so one plan survives
  our whole turn and they complete it. Negamax sign: the value returned is from
  the side to move's view, so a loss in `k + 2` is `-mate_in(k + 2)`.
- **Why phase 0 only.** An early return leaves this ply's line empty. At
  `Phase::First` that is exactly what the horizon return already does and is
  safe; at `Phase::Second` the phase-0 parent would promote a one-ply line and
  `turns_from_plies` refuses it by name at the root, far from the node that
  caused it — D-104's whole argument. The phase-1 case is handled by generation
  instead (§5.3), which costs one ply of search and keeps every line turn-whole.
- **The completed-window reading** is composed correctly by construction: see
  §3's disposition, plus a `debug_assert` that neither side owns a completed
  window at any node the protocol runs at.
- The node stores nothing in the transposition table on this path in revision 1.
  A bound could be stored and it would be sound; not storing is the conservative
  choice for a first landing, and it is one line to add under a later measurement.

### 5.3 Step 3 — the survival filter

**`LAW-FORCE` and its No-Counterattack corollary.** With `budget =
HitBudget::from(StonesLeft::from_state(state))`:

| `blocking_covers(us, budget)` | Meaning | Generation |
|---|---|---|
| `NothingToBlock` | `t = 0` — the opponent has no plan (`PAT-RHOMBUS`, `PAT-O3`) | No filter. Tier F ∪ Tier T ∪ Tier Q, with widening (§7). |
| `Minimal(covers)` | `1 ≤ t ≤ budget` | **Filtered.** The candidate set is exactly Tier F: the union of cells over the inclusion-minimal covers, PLUS own win-now cells. No Tier T, no Tier Q, no widening. |
| `Impossible` | `t > budget` | At phase 0 this is step 2 and never reached here. At phase 1 the position is lost and the search must reach that conclusion **through the generated moves**: generate Tier T ∪ Tier Q unfiltered, and count it. |

**Why the filtered set is complete for non-losing moves, and why widening past it
would add nothing.** `LAW-FORCE` is `[PROVEN]`: if the opponent has ≥ 1 plan and
the mover cannot win this turn, every non-losing mover move hits ALL opponent
plans. Its corollary is that counter-threats never substitute for hitting, except
win-now — which is why own win-now cells are in the set and why creating a new
plan gets no forced slot. So the filtered set contains every non-losing move, and
adding a stage could only add losing moves.

**The two-ply realisation, which is the mechanism the standing requirement names.**
`Cover::Minimal` carries SETS and not a flat cell union, because the union is
provably insufficient (D-257, `cover.rs`'s own counterexample). The search is
ply-level, so the cross-window pair `{a, b}` is reached across two plies rather
than emitted as a pair: at phase 0 both `a` and `b` are in the union and each is
tried; after `a` is placed the state is updated and phase 1 asks
`blocking_covers(us, HitBudget::One)` on what remains, where the windows `a` did
not kill are 1-coverable exactly when `{a, b}` was a minimal 2-cover — so `b`
comes back as `MinimalCover::One(b)`. The pairing obligation D-243 consequence
(4) calls "part of this requirement rather than a detail of implementing it" is
discharged by the phase-1 regeneration, and
`defensive_union_covers_nonminimum_two_stone_splits` pins it on a position with
two disjoint hot windows.

**One licensed value change, stated rather than discovered.** Under the filter
the search no longer prefers the longest resistance among losing moves: a
non-hitting move that delays a loss is not generated. `LAW-FORCE` licenses this
— those moves lose — and the soundness gate's charter allows narrowing to change
VALUES while forbidding it to lose a proven tactic. Mate DISTANCES on positions
that are already lost may therefore shorten under Staged. This is the honest
reading and it is recorded in the ADR line.

### 5.4 Step 4 — staged generation

Order within and across tiers, per D-7:

1. **Tier F (forced).** Never delta-ranked. Emitted in ascending `(q, r)`, which
   is the order every set the solver hands out already carries and the order the
   whole engine breaks ties in. Delta is not called for Tier F cells at all,
   which is also what makes a filtered node cheap (§8).
2. **Tier T (threats).** `LAW-SUPPORT`-qualified; the qualification is Matrix M1
   (§6). Delta-ranked, stable sort, so equal scores keep ascending `(q, r)`.
3. **Tier Q (quiet).** The remaining cells of the `quiet_radius` ball, delta-
   ranked, top-`quiet_top_k`, with the widening schedule of Matrix M2 (§7).

The three tiers are disjoint by construction (a cell in Tier F is not offered
again in T or Q). The final tie-break is lexicographic and costs nothing, because
the input to every sort is already ascending and every sort is stable — the same
argument `ordering.rs` carries today.

### 5.5 Step 5 — quiescence

**Out of scope.** WP-1.6's, together with `LAW-RIPOSTE` and `LAW-LEDGER` (§3,
rows 4 and 5). One ADR line records the handoff.

---

## 6. MATRIX M1 — Tier-T qualification

### 6.1 The measurements these options are chosen against

**MEASURED**, at base revision `f317385`, release build, by a census harness in a
`/home` git worktree (`crates/pistol-solver/tests/wp15b_census.rs`, run and then
removed with the worktree; it records no number this repository keeps except the
ones transcribed here, and it is named with its revision because it produced
them). Three sampling regimes, stated because the regime is most of what a
population number means:

- **corpus roots** — the 24 positions of `bench_positions_v1.txt`, at 15 and 35
  stones. These are the registered workload's own roots.
- **corpus + 1..3 sampled turns** — each root carried 1 to 3 turns deeper by
  uniform draws from the legal region, 8 seeds × 6 plies, n = 576. This is a
  stand-in for a search interior at depth 2–3 in STONE COUNT and is explicitly
  NOT the search's own distribution, which does not exist until this WP lands.
- **uniform playouts to 80 plies**, n = 960 — the tail the corpus does not reach.

| quantity | corpus roots | +1..3 turns | playouts |
|---|---|---|---|
| own hot windows, mean / max | 0.04 / 1 | 0.33 / 4 | 0.08 / 5 |
| opponent hot windows, mean / max | 0.46 / 4 | 0.21 / 4 | 0.10 / 5 |
| **survival filter applies** | **29.2 %** | **14.9 %** | **4.8 %** |
| of which `Impossible` (overload) | 4.2 % | 1.2 % | 1.7 % |
| candidate count when filtered, mean / max | **2.17 / 3** | **2.32 / 8** | **2.27 / 4** |
| live-2 windows, own / opponent | 7.21 / 12.17 | 11.07 / 10.90 | 23.78 / 25.43 |
| live-3 windows, own / opponent | 0.75 / 1.88 | 1.61 / 1.43 | 1.71 / 1.87 |
| radius-2 ball (the incumbent's whole universe), mean / max | 78.0 / 182 | 123.7 / 285 | 376.5 / 794 |

and the sets each option generates, and the whole node's candidate count at
`quiet_top_k = 16`:

| option | Tier T cells (mean) | **staged set, mean** | vs radius-2 full-width |
|---|---|---|---|
| **A** — count 3, both sides | 6.12 / 7.03 / 6.65 | **16.8 / 19.9 / 21.6** | **4.6× / 6.2× / 17.4×** narrower |
| **B** — count 2, both sides | 46.50 / 51.66 / 88.13 | **47.5 / 58.6 / 98.6** | 1.6× / 2.1× / 3.8× |
| **C** — count 2 for us, count 3 for them | 23.3 / 30.3 / 48.7 | **28.6 / 39.6 / 61.4** | **2.7× / 3.1× / 6.1×** |

(triples are corpus-roots / +1..3-turns / playouts.)

### 6.2 The options

| Option | Theory standing | Cost | Failure modes |
|---|---|---|---|
| **A — `NearHot::Three` both sides** | **No completeness licence.** `LAW-SUPPORT` k = 2 says a forced win in two own turns requires a window already holding ≥ 2 own stones, and T10 adds that a threat-window created THIS turn held ≥ 2 before. A count-3 qualification misses every plan a PAIR creates from a count-2 window — 2 + 2 = 4 — which is precisely the two-stone move this game is about. | Cheapest. **MEASURED** 4.6–17.4× narrower than the incumbent. | Loses `k = 2` completeness by construction, and `k = 2` is exactly what soundness gate (b) tests at depth 3 (= 2 own turns). Adopting a provably incomplete generator because a fixture set happens not to catch it is the gate-that-cannot-fail shape D-269 legislated against. |
| **B — `LiveCount::Two` both sides** | Full `LAW-SUPPORT` licence for both sides. | **MEASURED** only 1.6–3.8× narrower than the incumbent — and at corpus roots the staged set (47.5) is 61 % of the whole radius-2 ball (78.0). The depth this WP exists to buy is roughly `log(branching ratio)`, and a 1.6× ratio at the root band is close to buying nothing. | Two further costs, both measured. Tier T at count 2 is **not a subset of the radius-2 ball**: a live-2 window's empties reach up to 5 cells from a stone, so option B searches cells the incumbent never offered. That is a strength opportunity and a branching cost at once, and it makes "narrowing" the wrong word for what B does. And the opponent-side half buys the least: a defence against the opponent's two-turn win is what SEARCH DEPTH and the filter are for — their windows go hot on their move, and the filter then fires — whereas the attacker's half is a generation question no depth answers. |
| **C — `LiveCount::Two` for the side to move, `NearHot::Three` for the opponent** | Keeps `LAW-SUPPORT`'s k = 2 licence exactly where the law is about the mover's own forced win, and takes the opponent's contribution as the one-stone activation set — `BOUND-CONVERT`'s subject, the cells that would MAKE an opponent window hot. | **MEASURED** 2.7–6.1× narrower than the incumbent; **MEASURED** staged set 28.6 at corpus roots against 78.0. | Asymmetric, so it must be argued rather than assumed, and the argument is above. Residual: it does not offer cells that block an opponent COUNT-2 window, so a defence that must be played two of the opponent's turns early is left to Tier Q's delta ranking. The standing requirement ("defensive relevance never gated solely on opponent count ≥ 4") is met by the opponent-side count-3 kills plus Tier Q, and the colony family is the fixture that tests it. |
| **D — config knob, no design choice** | — | — | Rejected as a matrix answer: "make it configurable" is not a selection, and CLAUDE.md's rule is that the SURVIVING option's ADR line records the strongest surviving attack. The knob still exists (§9) so the operator can run variants; what the matrix decides is what `configs/instrument_staged_v0.toml` COMMITS. |

**RECOMMENDATION: C**, committed as `tier_t_own = 2`, `tier_t_opponent = 3` in
`configs/instrument_staged_v0.toml`, with A and B reachable by config for the
operator's variants.

Grounds: C is the only option that keeps a `[PROVEN]` completeness licence for
the half of the problem where generation is the only answer, while taking
**MEASURED** 2.7–6.1× of the branching reduction that this work package exists
to buy. A is cheaper and gives up the licence; B keeps the licence twice over
and gives up most of the reduction, for an opponent-side half that depth already
covers.

**Pre-registered consequence, before any gate runs**: if soundness gate (b)
(§11) is RED under C, the option is not repaired by moving a threshold — C is
replaced by B, which is the strictly wider licensed option, and the exchange is
recorded as an amendment with its own review. A red gate under B would be a
finding about the design and not about the option.

---

## 7. MATRIX M2 — the widening schedule

### 7.1 The problem, stated exactly

Tier Q's top-`K` is a forward-pruning cut, and it is unsound in one direction: a
node that **fails low** (`best_score <= original_alpha`) after searching a
TRUNCATED set has returned an upper bound that may be too low, because the move
that would have raised it was cut. A node that fails HIGH or returns an exact
score inside the window is unaffected — it found what it needed inside the set it
had.

The calculus already names this: the known unsoundness perimeter's item (ii) is
"Tier-Q interior". Widening is the mitigation; it is not a proof, and this
document does not claim one (`REJ-DEPTHPROOF`).

**Mechanism common to every option below.** The ordered universe (F, then T, then
the whole Q pool) is built ONCE per node. The candidate loop then runs over
BATCHES of it; after a batch, the next batch is entered only if the node has
failed low so far and a batch remains. Widening therefore searches only the
ADDITIONAL cells, at the same `alpha`/`beta` — every cell already searched scored
at or below `original_alpha ≤ alpha`, so re-searching them is provably useless.
The batch boundaries are a config'd list, so the set searched is a deterministic
function of (position, params) and of nothing else.

### 7.2 The options

| Option | Schedule | Cost | Failure modes |
|---|---|---|---|
| **W-A — one widening, to everything** | `[K, all]`. On a truncated fail-low, search the rest of the Q pool. | The bound is exactly the incumbent's: a node that widens fully has searched the radius-2 ball, so no node is ever more expensive than full-width plus the tier bookkeeping. | Fires at every fail-low, including the null-window scans PVS uses to CONFIRM a move is not better — which is the common case, and where a fail-low is the answer the parent wanted. **ESTIMATED, and it cannot be measured before the implementation exists** (no fail-low rate can be sampled from a search that does not run): the fraction of nodes that widen is unknown, and if it is high W-A degenerates toward full-width and buys no depth. |
| **W-B — PV nodes and the root only** | Widen where `beta - alpha > 1`, or at ply 0. | Cheapest. PV nodes are a small fraction of a PVS tree. | Non-PV nodes keep a bare hard cap in all but name, which the design brief forbids by name. Rejected on that ground: the objection to a hard cap is not that it is a cap but that nothing says when it bit, and W-B silently caps the majority of nodes. |
| **W-C — geometric schedule everywhere** | `[K, 2K, 4K, all]`. | Between the two; each widening step pays only its own increment. | More knobs, and the intermediate steps have no principle behind their sizes. A schedule whose entries are chosen because they are powers of two is a tuning artefact presented as a design. |
| **W-D — widen on fail-low, and also record what a full search WOULD have returned** | — | Doubles every fail-low node. | Rejected: it is the soundness gate as a run-time cost. What it wants is (b) in §11, which pays it once in CI instead of on every node forever. |

**RECOMMENDATION: W-A, `widen_schedule = [16, 0]`** (`0` spells "the rest"), for
three reasons.

1. It is the only option with a stated worst-case bound that a reader can check:
   a Staged node never searches more than the radius-2 policy would have, plus
   the tier arithmetic. Every other option trades that bound for a tuning guess.
2. The design brief's requirement is "deterministic WIDENING on fail-low by a
   config'd schedule — never a bare hard cap". W-A is the schedule that most
   directly meets it, and the ones that are cheaper are cheaper precisely by
   reintroducing the cap.
3. The unknown — how often it fires — is a MEASUREMENT this work package takes
   and reports (§12: stage-share counters include the widening rate), not a
   number to guess now. If the measured rate makes W-A degenerate, that is a
   finding with a named successor (W-C) and an amendment, which is cheaper than
   adopting W-C now on an estimate.

**Pre-registered reading of that measurement, fixed before it is taken**: a
widening rate at or below 25 % of unfiltered nodes leaves W-A committed. Above
50 %, W-A is recorded as degenerate, the finding is reported, and the successor
matrix is W-C's — not a change of `K`, which would be a threshold move. Between
25 % and 50 % the number is reported and W-A stands, because the bound in reason
(1) still holds and nothing in the band is a failure.

---

## 8. MATRIX M3 — the soundness instrument

D-124 is the debt this discharges. Its words: the value-differential oracle
cannot see a search that narrows at the TAIL — dropping the worst-ordered
candidate at every node left all ten of its assertions green — and "Stage 1's
very next candidate-set change is exactly this shape … so the work package that
lands it inherits the fact that this oracle will not judge its dominance rule,
and owes a check of its own".

The minimum bar is fixed by the brief: (a) the tactical suite at 100 % of its
pre-registered thresholds under Staged; (b) a differential gate against
full-width r2 at depths 1..=3 for mates and forced blocks; (c) a colony fixture
family of ≥ 6 cases; (d) the INTEG §5 pattern fixtures under Staged. The matrix
is about the INSTRUMENT for (b), which is the only part with real options.

| Option | (b)'s instrument | Cost | Failure modes |
|---|---|---|---|
| **S-A — value agreement, as WP-1.1** | Compare Staged's root score with the full-width reference's. | Reuses `tests/common/reference.rs` unchanged. | **This is the instrument D-124 proved blind to this exact change.** A narrowing that drops a move which is never the argmax cannot move a maximum. Rejected by name: adopting it would be adopting the criterion the defect class preserves. |
| **S-B — candidate-set containment** | Assert Staged's per-node candidate set ⊇ some property of the reference's. | — | Needs visibility into `pvs` that D-124 itself says the WP-1.1 non-goals forbade, and — decisively — it is the WRONG claim: Staged's set is deliberately NOT a superset, and a containment that held would mean the WP did nothing. |
| **S-C — class-restricted answer agreement** | Over a fixture corpus, at depths 1..=3, run the full-width reference at radius 2 and Staged. Wherever the REFERENCE's root answer is a **mate** (a forced win at that depth) or a **forced block** (the reference's argmax set is confined to cells that hit every opponent plan), assert Staged returns a move in the reference's argmax set and the same score kind and distance. On every other position, assert nothing. | Reuses the existing full-width reference and its `PairOrder::Deduped` mode; the cost is the reference's, which is the candidate count to the fourth power per turn — **MEASURED** as gate 10's own cost today, and the reason it runs last in `tools/ci.sh`. | The class restriction is the whole content, so it must be DERIVED from the reference and never asserted by hand: a fixture labelled "mate in 3" by its author and not by the reference would make the gate a test of the label. Mitigation: the class is computed from the reference run, per position, per depth. Residual: a position in neither class is not covered, which is correct — that is where narrowing is licensed to change values. |
| **S-D — S-C plus a mutation gate** | S-C, and additionally a recorded set of mutations to the staged generator that must each turn S-C red. | One worktree mutation run. | Cost only. Without it, S-C's own strength is a claim rather than a measurement — which is D-295's finding about the pattern pack, arriving one work package later, and it would be a poor look to repeat it inside the very WP that read it. |

**RECOMMENDATION: S-D.**

The class definitions, fixed here before any run so that neither can be
re-read after one:

- **Mate class.** The reference's root value at depth `d` is a mate score for the
  side to move. Staged must return a move in the reference's argmax set at that
  depth, and the same mate distance. This is the class D-124's mutation
  (`cells.pop()`) could not touch and that a narrowing genuinely endangers.
- **Forced-block class.** The reference is NOT in the mate class at depth `d`,
  the opponent's plan family at the root is non-empty, and the reference's argmax
  set at depth `d` is a non-empty subset of the cells that hit every opponent
  plan. Staged must return a move in that argmax set. Membership in the argmax
  set is the claim, never move identity — the same restraint `agreement.rs`
  already states, because which tie is taken is pinned by the determinism gate
  and not by this one.
- **Everything else**: no assertion. Recorded in the gate's own output as the
  count of positions in neither class, so the gate cannot silently become vacuous
  by classifying nothing.

**The mutations S-D must kill**, registered now (worktree, never the live tree):

| # | Mutation | Must turn red |
|---|---|---|
| M1 | Tier F drops the pair-completion class (own hot windows at count 4) | mate class |
| M2 | Tier F drops `win_in_one_ply_cells` | mate class |
| M3 | The survival filter uses `Cover::cells()` flattened at phase 0 and does NOT regenerate at phase 1 | forced-block class |
| M4 | The survival filter uses minimum-CARDINALITY covers instead of inclusion-minimal ones | forced-block class |
| M5 | Own win-now cells are dropped from the filtered set | mate class |
| M6 | The overload check drops its `can_win_this_turn` guard | mate class (it scores the winner as the loser) |
| M7 | Tier T qualifies at count 3 for the side to move (option A) | one of the two classes, or the mutation is recorded as SURVIVING and option A's incompleteness is recorded as unobserved on this corpus |

M7 is deliberately a mutation whose survival is INFORMATIVE: it is Matrix M1's
option A, and if it survives, the surviving-mutant list says so in full, with a
diagnosis, exactly as D-281 required of the last suite that reported one.

**Gate wiring.** (a)–(d) become one script, `tools/staged_soundness_check.sh`,
added to `tools/ci.sh`. It is a `tools/` change and is therefore reviewed against
`tools/SHELL_CHECKLIST.md` with every item answered by name, carries the coverage
rule's test driving the shipped script, and distinguishes RUN VOID from FAIL by
name (item 12) with a scratch preflight.

---

## 9. MATRIX M4 — the snapshot's config seam

The brief requires `tools/baseline_snapshot.sh` before and after, against pinned
operator numbers (`depth_at_500ms` opening 2 / early_mid 2 / late_mid 1 at
`050961d`). **MEASURED at `f317385` on this machine, BEFORE run: opening 2,
early_mid 2, late_mid 1 — the pinned triple reproduces exactly.** The script's
`CONFIG` is a literal, `configs/instrument_v0.toml`, with no flag.

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| **N-A — add `--config PATH`** | A workload-scope flag beside `--corpus`, `--ladder-depth` and `--binary`. The budget line still says `registered`; the record already carries `config <path> <sha>` and `engine_id config <path>` ABOVE the timing marker, so two records taken under two configs are already distinguishable and cannot be diffed as one. | A `tools/` change: SHELL_CHECKLIST answered item by item, plus at least one test driving the shipped script (item 10). **MEASURED** one snapshot run costs 34.0 s wall on this machine. | It reopens a script whose review round closed recently. Mitigated by the shape: the flag is the fourth of its exact kind, the record's invariance claim is untouched because the config path was already inside the invariant block, and the argument parser's `argument` helper already refuses an empty value. |
| **N-B — flip `configs/instrument_v0.toml` to staged** | No tools change; the standing instrument measures Staged by construction. | Zero. | Lands the strength claim before its judge, against rule 6 and against D-190/D-194's own precedent; breaks the D-209 instrument golden transcripts; fires D-204's flip clause on this session's authority rather than the operator's. Rejected. |
| **N-C — a scratchpad harness** | Measure Staged with a session-local script. | Zero repository change. | The number would come from an instrument with no governing revision, which CLAUDE.md's instrument clause exists to forbid, and it would not be comparable with the pinned operator triple because it is a different instrument. Rejected. |
| **N-D — take no Staged snapshot** | Report the radius numbers only. | Zero. | The brief's required measurement is not taken and the WP's whole depth claim goes unmeasured. Rejected. |

**RECOMMENDATION: N-A.** It is the only option that produces the required number
from the registered instrument. The deltas it yields are ADVISORY per the session
policy; the operator re-runs on their own hardware for the record.

---

## 10. The config shape

`configs/instrument_staged_v0.toml`, complete, `deny_unknown_fields`, no
code-side default for any value (CLAUDE.md rule 1):

```toml
[search.candidate_policy]
kind = "staged"
# Tier Q's ball. 2 so the quiet universe is the incumbent's whole universe and
# the SPRT's two seats differ in what they SELECT rather than in what they can
# see (docs/decisions.md D-190/D-194 committed radius 2).
quiet_radius = 2
# Tier Q's cut, before widening.
quiet_top_k = 16
# The batch boundaries after Tier Q's cut. 0 spells "everything that remains".
widen_schedule = [0]
# Tier T's LAW-SUPPORT qualification, per Matrix M1 option C.
tier_t_own_count = 2
tier_t_opponent_count = 3
```

Validation, in `pistol-engine`'s validator and again in `Searcher::new` (a
`SearchParams` can be built in code and never passes through a document):
`quiet_radius` in `1..=MAX_CANDIDATE_RADIUS` and representable as `i16`;
`quiet_top_k >= 1`; `widen_schedule` strictly increasing with `0` admissible only
as the last entry; `tier_t_own_count` and `tier_t_opponent_count` in `{2, 3}`,
mapped to the closed `LiveCount` — a value outside the set is a named refusal and
never a clamp.

`instrument_r2_v0.toml` is value-identical to the committed `instrument_v0.toml`
(D-194) and is the SPRT's incumbent seat, so the two seats differ in exactly one
document key.

---

## 11. The test plan

Behaviour-named, calculus IDs in doc comments, and — per the INTEG lesson every
new fixture inherits — **each states what quantity it watches**, so a test whose
quantity cannot move is caught at authoring rather than by a later red team.

| Test | Watches |
|---|---|
| `overload_at_entry_scores_loss_without_expansion` | the returned SCORE and the NODE COUNT: the loss is `-mate_in(k+2)` and the node expanded no child. A version that searched to the loss would move the node count. |
| `overload_check_guarded_by_own_win_now` | the sign of the score on a position where BOTH sides have an unhittable family and the mover can win now: it must be a win, not a loss. |
| `overload_composition_handles_completed_window_reading` | that no node reached by the protocol carries a completed window for either side, and that on a DECIDED position the composition refuses (`StonesLeft::from_state` is `None`) rather than scoring the loser as winning. |
| `survival_filter_hits_all_plans_across_both_plies` | the generated CELL SET at phase 0 and again at phase 1, against the plan family computed independently — not the move played. |
| `defensive_union_covers_nonminimum_two_stone_splits` | that the cross-window pair `{a, b}` is REACHED: `a` at phase 0 and `b` among phase 1's forced cells. A flat-union generator offers both cells and never the pair, so the quantity is the phase-1 set after `a`, not the phase-0 set. |
| `mate_in_1_by_pair_generated_in_tier_f_not_ranked_in` | that both empties of the count-4 window are in the FORCED prefix, with an eval whose delta ranks them last — so a delta-ranked implementation fails. |
| `own_win_now_cells_survive_survival_filter` | the presence of the win-now cell in the filtered set on a position where it hits nothing. |
| `new_plan_creation_gets_no_forced_slot` | the absence of a plan-creating non-hitting cell from the forced prefix, with that cell's delta made maximal so only the tier boundary can exclude it. |
| `tier_t_qualification_matches_adopted_matrix_option` | the Tier T set against an independently computed `us@2 ∪ them@3`, on a position where `us@3 ∪ them@3` and `us@2 ∪ them@2` are both different sets. |
| `widening_schedule_fires_on_fail_low_and_is_deterministic` | the widening COUNTER and the node's answer: fires on a truncated fail-low, does not fire on a fail-high, and two runs agree. |
| `staged_ordering_deterministic_within_and_across_tiers` | the whole emitted order, twice, and with equal delta scores forced, so only the lexicographic tie-break can decide. |
| `stage_counters_reported_in_search_info` | that each counter is non-zero on a position constructed to fire that stage, and zero on one constructed not to. |
| `gap_trap_answered_in_tier_f` | that `PAT-GAP`'s singleton gap cell is in the FORCED prefix from the defender's side (`LAW-HIT`: the singleton plan must be hit) and among win-now from the attacker's. |
| `colony_family_passes_under_staged` | the move played on each of ≥ 6 built distant-cluster positions, attack and defence, where `LAW-DECOMP`'s star-disjointness means the right answer is in a cluster the delta ranking does not favour. |
| `tactical_suite_holds_at_thresholds_under_staged` | the `require` count of `tactical_staged_v0.txt` — 100 %, at the staged config. |
| `staged_matches_full_width_on_mate_and_block_classes_depths_1_to_3` | S-C's two classes, with the count of positions in NEITHER class printed, so the gate cannot become vacuous unnoticed. |

### 11.6 One thing this WP does NOT close, named because it is adjacent

D-295's residual — that `RULE-EXACT`'s "never derived by weight algebra" is
unpinned in `src`, because no `HitBudget`-shaped fixture separates `t = 3` from
`t = 4` — is NOT closed here. D-295 names `blocking_covers` as the differently-
shaped surface that could close it, and this WP puts `blocking_covers` on the
per-node path, so a reader will reasonably ask. The answer: closing it is a
`pistol-solver` test-coverage change with its own scope, and this WP's soundness
gate exercises `blocking_covers` for its ANSWERS rather than for its
arithmetic's exactness. Registered for WP-1.10, one ADR line.

---

## 12. What this work package measures

All ADVISORY on this machine; the operator re-runs `tools/baseline_snapshot.sh`
on theirs for the record.

1. **Snapshot before / after.** BEFORE is taken and recorded above. AFTER is
   taken with `--config configs/instrument_staged_v0.toml` (Matrix M4 option A).
   The pinned triple to move: `depth_at_500ms` opening 2 / early_mid 2 /
   late_mid 1.
2. **Stage-share counters** — F/T/Q firing rates, the filtered-node rate, the
   `Cover::Impossible` rate, the overload-return rate and the WIDENING rate, over
   the fixture corpus. This is the empirical footing WP-1.6 and WP-1.7 inherit,
   and it is what Matrix M2's pre-registered reading is read against.
3. **The WP-1.4 adversarial-spread debt at `movetime 500` under Staged** —
   completed depth on the `spread_v1.txt` class (D-210).
4. **D-263's registered hotspot, discharged.** Rule 5 wants the hotspot measured
   at the counts THIS generator produces rather than at counts chosen to make a
   curve, and that is now done. **MEASURED**, release, at base revision:
   `blocking_covers` costs 246 / 71 / 69 ns mean (max 1513 / 1252 / 2665 ns) and
   `unblockable_double_threat` 101 / 50 / 49 ns mean across the three regimes; a
   deliberately built family of 16 disjoint hot windows costs 1479 ns/call, and
   the game does not reach 16 — the maximum hot count observed anywhere in the
   three regimes is 5. Against a per-node budget of roughly 8–20 µs implied by
   the BEFORE snapshot's 47 000–303 000 nps, the two queries together are
   **MEASURED at 0.6 %–3.7 % of a node**. D-263's three remedies — the
   three-disjoint-families early-out, lifting `empty_families` off the per-call
   path, and a different enumeration — are therefore **NOT implemented**, and
   that is the finding: the pre-registered hotspot is not hot at the counts the
   generator produces. Rule 5's "a measured structural floor is a finding, not a
   failure", in the other direction.

---

## 13. Costs

Stated on this document's own face, per the proportionality rule (D-228, as
amended by D-245, D-277 and corrected by D-292). This work package has **no
governed run**: the SPRT is a DELIVERABLE here and the operator's run later.

| Item | DECLARED | MEASURED |
|---|---|---|
| `tools/ci.sh`, once | ~5 min | **5 m 20 s** (warm target) |
| One baseline snapshot | ~35 s | **34.0 s** |
| The census harness behind §6.1 | ~1 min | **< 1 s** per run after a 1.3 s build |
| The gate-expiry re-measurement (§4.1) | ~3 min | **~2 min** including a cold worktree build |
| The soundness gate (a)–(d), per CI run | ESTIMATED 60–180 s, dominated by the full-width reference at depth 3, whose cost is gate 10's today | to be MEASURED when it lands, and reconciled here |
| The operator's SPRT run | see the pre-registration | the operator's |

---

## 14. Non-goals and registered follow-ups

Non-goals, adopted verbatim: no quiescence (WP-1.6); no killers/history/
countermove (WP-1.7); no df-pn (WP-1.8); no eval terms from `t` or `τ`; no
dominance pruning beyond the staged scheme; no `LEGAL_RADIUS` change; no
ball-scan optimisation; no `pistol-eval` refactor.

Registered by name, one ADR line each:

- `THM-WINDOW` follow-up — exact-`t` counters as eval terms, cheap because this
  WP puts a `ThreatState` on the per-node path.
- The Tier-Q **ball scan** stands: building the ordered universe costs one delta
  per ball cell even at a filtered node's neighbours, and D-207's epsilon domain
  is untouched. WP-1.5c candidate.
- `LAW-RIPOSTE` and `LAW-LEDGER` hand off to WP-1.6.
- The fallback (`fallback_turn`) under Staged reuses the `quiet_radius` ball and
  does not consult the threat state: bounded, pure, and identical to today's
  play-mode behaviour at that radius. WP-1.6 candidate.
- D-295's `RULE-EXACT` residual → WP-1.10 (§11.6).

---

*Revision 1. Reviews owed against this revision: one DECISION-RED-TEAM per matrix
(M0, M1, M2, M3, M4) and one REVIEW-design. An amendment to any matrix reopens
its review.*
