# WP-1.5b — staged threat-first candidate generation: DESIGN

**Revision 5.** Base revision `f317385`; revisions 1–4 were `ec8f7fb`, `182f389`,
`7ad466b`, `f762c9a`. **Revision 4 FAILED its REVIEW-design** — 3 BLOCKING, 10
MAJOR — but with the shape changed: **no finding reopened a matrix, and there
were no STOPs.** M0 (f), M1 C-at-threshold, M2 W-E, M3 S-E, M4 N-A and M5-E all
survive attack on their merits. What failed was TRANSMISSION: the normative
section contradicted itself on one rule, the instrument clause was still
unsatisfied for this document's own instrument, and §6.2's table was not the
instrument's output. §0.5 lists it. One finding is a correctness error and is
redesigned, not edited.

**Revision 4.** Base revision `f317385`; revisions 1–3 were `ec8f7fb`, `182f389`,
`7ad466b`. **Revision 3 FAILED its REVIEW-design** (2 BLOCKING, 9 MAJOR) and
**matrix M5 FELL** to its DECISION-RED-TEAM, which supplied the dominating option.
§0.3 lists what changed. One item is a correction to this session's own record:
a defect the implementing session claimed to have self-caught was **refuted with a
reproducer**, and the fix it proposed would have deleted the option's entire
value.

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

### 0.3 What revision 3 got wrong

| # | Revision 3 said | Revision 4 says | Severity |
|---|---|---|---|
| 14 | §5.2's overload predicate and §5.3's `Impossible` row are two checks | They are the **same predicate**. `blocking_covers(us, b) == Impossible ⟺ unblockable_double_threat(them, b)`, proved from `cover.rs:145`/`:177` and MEASURED to agree on **145 158** playout positions and **343 344** side-and-budget comparisons with **zero** disagreements. Revision 3 computed a hitting set and a `can_win_this_turn` twice per node | **BLOCKING** |
| 15 | Revision 3 dropped `phase == First` from §5.2 while its turn-wholeness paragraph still argued from phase 0 | The branch is where **all** the value is — MEASURED, **455 177 of 455 201** firings at depth 3 are `Phase::Second` — and it generalises `LAW-OVERLOAD` beyond its `[PROVEN]` two-stone form. Adopted deliberately in revision 4, with the generalisation derived from `LAW-HIT` and `DEF-T` on the page | **BLOCKING** |
| 16 | The census "is committed at a named revision" | It existed at **no** revision; `git ls-files` named only this document. Now committed as `crates/pistol-solver/tests/wp15b_census.rs` with its registered values pinned beside it | **BLOCKING** |
| 17 | §8.3(a): the tactical threshold holds because "Tier Q's cut is not involved — the filter fires or win-now applies" | FALSE for 5 of the 20 cases. A *double three* is count 3, below D-243's hot threshold of 4, so those roots take the batched row. The derivation is redone in §8.3 on a different and better ground | **BLOCKING** |
| 18 | §12.4 implements D-263's three-disjoint-families early-out | It accelerates only the redundant call revision 4 **deletes**, at the budget that essentially never fires. Dropped; the saving comes from removing work, not speeding it up | **MAJOR** |
| 19 | §6.3's cost column | Re-derived from the committed census under revision 3's OWN generation rules, which revision 3 changed without re-deriving. And it is a MIXTURE of two node populations, now reported apart | **MAJOR** |
| 20 | "the saving is on the cheap majority … nodes that do not choose the answer" | MEASURED: the early return changes the **bestmove on 2 of 24** corpus roots at depth 2 and a **non-mate score on 3 of 24**. Non-PV values propagate into the root's argmax | **MAJOR** |
| 21 | The S-E observer is a `#[cfg(debug_assertions)]` hook on `Run` | Unbuildable as written: `Run` is `pub(crate)`, `plans.rs` is another crate's integration-test module, and D-115 forbids the in-source route. And D-129 forbids the `debug_assert` demotion for a correctness invariant | **MAJOR** |
| 22 | Tier T is a threshold "over `LiveCount`" | `LiveCount` is closed at `{Two, Three}` and cannot express ≥ 4; the ≥ 4 windows are `hot_windows`. Reachable at `Phase::Second` with an own hot-4 window | **MAJOR** |
| 23 | D-263's cover arithmetic is this WP's hotspot | It is not the dominant one. MEASURED, Tier-T cell extraction costs about **6×** both threat queries combined on one harness — and `pistol-solver` has **no public route** to a live-2 window's empties (D-261) | **MAJOR** |

### 0.5 What revision 4 got wrong

| # | Revision 4 said | Revision 5 says | Severity |
|---|---|---|---|
| 24 | §5.3: the table move is promoted to index 0 unconditionally. §5.4: only within its own tier | Two incompatible rules, and §11 registered a test that fails under one. The failing case — Tier F empty, Tier T non-empty, table move in Tier Q — is **70.8 %** of corpus roots. **One rule now** (§5.4) | **BLOCKING** |
| 25 | "The census is committed at a named revision" | Still no SHA anywhere, and the two cross-references resolved in a circle. Both instruments now carry theirs: census `7941775`, `tools/baseline_snapshot.sh` `e889b5b` | **BLOCKING** |
| 26 | §6.2's population table | Not the instrument's output. Ten cells differ, and a whole "playouts" column had no regime at all. §6.2 is now re-derived from the committed census, which gained that regime | **BLOCKING** |
| 27 | §8.3(a): "narrowing cannot invent a mate" | **FALSE, and this is the correctness error.** W-E's non-PV cut is a forward prune, and forward-pruning a defender's saving reply is exactly how a search proves a mate that is not there. Redesigned in §8.3 | **MAJOR** |
| 28 | §8.4's mutation M4's witness | `cover.rs`'s flat-list counterexample has **no** 1-cover, so inclusion-minimal and minimum-cardinality coincide there and the mutant is an identity. Replaced with the shape that discriminates, built and verified | **MAJOR** |
| 29 | §15 item 4 registers the `#[cfg(debug_assertions)]` seam | The seam §8.2 withdrew. The un-re-read claim, inside the ADR list itself — the fifth instance of this WP's own pattern, in the section that warns about it | **MAJOR** |

### 0.4 One correction to this session's own record

Between rounds the implementing session wrote into its notes that firing the
overload return at `Phase::Second` corrupts the root PV, and that "`!is_pv` does
not save it". **That is not reproducible.** With an assertion armed at every ply-0
promotion: depth 2, 2 619 phase-1 firings, **0 violations**, 24/24 bestmoves;
depth 3, **433 542** firings, `turns_from_plies` at every completed depth, **0
`PV_NOT_PLAYABLE`**.

The analysis stopped one link short. The parent at `Phase::First` does promote a
half-turn line — but its own parent reaches it through the NEGATING
`TurnComplete` link, and a promotion not preceded by a full-window re-search
implies `score >= beta`, so the grandparent sees `-v <= alpha` and does not
promote. The malformed line dies one ply above the firing node and never crosses
a flip link; at the root the fail-high branch is unreachable at all, `beta` being
`INFINITY`. Recorded because this project's own rule is that a finding is verified
with a minimal reproducer before its fix lands, and this one was written down on
an argument that had not been run — the same failure it exists to prevent, from
the other side.

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

**MEASURED saving**, arithmetic on §12.4's own figures: the registered per-node
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

That argument rests on machinery §7.2's batch cut touches, so it is not left as
prose: the named invariant asserted at the return site is what would fire if a
later change broke it, and §11's test is built on a **phase-1** firing — a phase-0
firing is 0.0053 % of firings and a test author following revision 3's argument
would have built the case that never runs.

#### What the early return costs elsewhere, stated because revision 3 denied it

Revision 3 claimed the saving "is on the cheap majority … nodes that do not choose
the answer". A non-PV subtree's value propagates into the root's argmax.
**MEASURED**, depth 2, 24 corpus roots: the bestmove differs on **2 of 24** and the
completed-depth score on **3 of 24**, including a case where neither score is a
mate score — so this is not only §5.3's licensed shortening of mate distances on
lost positions. It belongs beside the other two axes on which the SPRT's seats
differ (§7.2).

#### The completed-window reading

Composed correctly by construction: the protocol runs only at nodes the search has
not already ended, so no node it runs at has a completed window for either side —
a `debug_assert` and a test (§3), not an assumption.

### 5.3 The four generation rows

| Row | Reached when | Generation |
|---|---|---|
| **WIN-NOW** | `can_win_this_turn(us, left)` is `Some` | **Exactly** the win-now class of §5.1: every `win_in_one_ply_cells(us)`, plus at `StonesLeft::Two` both empties of every own hot window at exactly four stones. No filter, no Tier T, no Tier Q, no batching |
| **FILTERED** | `None`, and `Cover::Minimal` | The union of cells over the inclusion-minimal covers, and nothing below it |
| **BATCHED** | `None`, and `NothingToBlock` | Tier T ∪ Tier Q, batched per §7 |
| **BATCHED (lost)** | `None`, `Impossible`, and the node is a PV node or the root | Tier T ∪ Tier Q, batched per §7, and counted. The position IS lost; the search reaches that through the generated moves because a PV node must return the line that proves its score |

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
one rule, and §12 item 6 records what it costs `Run::salvage`.

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

**The two-ply realisation — VERIFIED on the shipped solver.** `Cover::Minimal`
carries SETS because the union is provably insufficient (D-257). Two disjoint
sealed five-stone P1 rows, P2 to move with two stones:

```
phase0 cover = Minimal([Two { first: (4,4), second: (5,0) }])
phase0 union = [(4,4), (5,0)]
phase1 cover after (4,4) = Minimal([One((5,0))])
```

D-243 (4)'s pairing obligation is discharged by the phase-1 regeneration. The same
two rows UNSEALED give 8 hot windows and `Cover::Impossible` — correct, and why
the fixture seals both ends: a fixture that forgot them would test the overload
path while claiming to test the pairing path.

**One licensed value change.** Under the filter the search no longer prefers the
longest resistance among losing moves. `LAW-FORCE` licenses it — those moves lose
— so mate distances on already-lost positions may shorten. ADR line §15 item 11.

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
expensive half can drive it (§8.2), and its entry point must reach three things:
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
function and the search's actual candidate set diverge. Named in §15 item 21.

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
`Staged`. §15 item 20 takes the line, and §11 registers the test. A table move that is not a candidate is dropped, as
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

### 5.5 Step 5 — quiescence

Out of scope. WP-1.6's, with `LAW-RIPOSTE` and `LAW-LEDGER` (§15).

---

### 5.6 MATRIX M5 — how the overload verdict is realised — FELL

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| M5-A — early return, `!is_pv` gated, computed by its OWN query | Revision 3's | **MEASURED** `unblockable_double_threat` 49–101 ns — *the redundant half of a pair* | **FELL.** Strictly dominated by M5-E on the matrix's own cost axis |
| M5-B — early return everywhere, with a promoted WITNESS LINE | Fires at PV nodes too | **MEASURED** 22 PV firings against 455 177 non-PV over 14 roots at depth 3 — **0.005 %** more coverage | A line the search did not search, which `pvs.rs` refuses for the table cutoff. Rejected, now with its number |
| M5-C — no early return; let the search prove the loss | Delete step 2 | **MEASURED**: costs −17.27 % nodes at depth 2 and −47.25 % at depth 3, i.e. that is what the return buys | Correct and slower. Revision 3 rejected it with no number at all, which is the option-matrix clause's own finding |
| M5-D — store the verdict | A record beside the return | One store | Deferred. Its cell is corrected: it **CONTRADICTS** §7.2's stated rule rather than "interacting" with it, since §7.2 forbids storing a bound from a set that was not exhausted and this one is from a set never generated. Also `Record.best` has no answer, and the store depth has two conflicting honest choices |
| **M5-E — ONE `can_win_this_turn` and ONE `blocking_covers`; step 2 realised as the `!is_pv` early return on the `Impossible` row** | §5.2 | **MEASURED** equivalence over 145 158 positions and 343 344 comparisons, 0 disagreements. **ESTIMATED** from §12.4's own MEASURED figures: −29.1 % / −41.3 % / −41.5 % of the registered per-node threat cost | Identical verdict, value, soundness and gate to M5-A; `PROTO-NODE`'s ORDER is preserved textually, and only the order of QUERIES changes — a query is not a step |

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
> redundant half of a pair the design's own §12.4 already sums at 347 ns and
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

**MEASURED** at `f317385`, release, by the census harness
`crates/pistol-solver/tests/wp15b_census.rs`, **committed at `7941775`** rather than deleting with its worktree — CLAUDE.md's
instrument clause, and D-287's rule that an artefact recording numbers is not
test-tree-only.

Three regimes. The middle one is **re-sampled** in revision 2: revision 1 deepened
by uniform draws from the radius-**8** legal ball while the policy is radius **2**,
which inflated the ball 78.0 → 123.7 by the sampler rather than by depth.

| quantity | corpus roots | +1..3 turns, **r2 draw (REPORTED)** | +1..3 turns, r8 draw (SUPERSEDED) | playouts |
|---|---|---|---|---|
| own hot, mean | 0.0417 | 0.3559 | 0.3299 | 0.0833 |
| opponent hot, mean | 0.4583 | 0.2951 | 0.2101 | 0.0958 |
| live-2, own / opponent | 7.21 / 12.17 | 11.18 / 12.45 | 11.07 / 10.90 | 23.78 / 25.43 |
| live-3, own / opponent | 0.75 / 1.88 | 1.78 / 1.87 | 1.61 / 1.43 | 1.71 / 1.87 |
| **`Cover::Minimal` — the FILTERED row** | **25.0 %** | **18.4 %** | 13.7 % | 3.1 % |
| **`Cover::Impossible`** | **4.2 %** | **1.4 %** | 1.2 % | 1.7 % |
| **BATCHED nodes** | **70.8 %** | **61.5 %** | 65.5 % | 92.5 % |
| radius-2 ball, mean | 77.96 | 94.50 | 123.66 | 376.47 |

**Every cell above is the committed census's output**, and revision 4's was not:
ten cells differed, the "survival filter applies" row summed `Minimal` and
`Impossible` into one figure while the row below it averaged over `Minimal` only,
and a whole playouts column had no regime in the instrument at all. §5.3 routes
`Impossible` to a BATCHED row, so the two are now separate rows — a filtered node
and a lost node generate different sets and quoting them together was the mixture
defect this document convicted itself of one section later.

Staged-set sizes at `quiet_top_k = 16`, threshold reading:

| option | Tier T cells | staged set, **all nodes** | staged set, **BATCHED only** |
|---|---|---|---|
| A — ≥3 both sides | 6.125 / 8.245 / 6.651 | 15.96 = 4.89× / 15.17 = 6.23× / 20.07 = 18.76× | 21.65 = 3.80× / 23.20 = 4.27× / 21.44 = 17.00× |
| B — ≥2 both sides | 46.500 / 54.625 / 88.127 | 45.12 = 1.73× / 44.15 = 2.14× / 91.04 = 4.14× | 62.82 = **1.31×** / 70.36 = **1.41×** / 98.17 = 3.71× |
| **C — ≥2 us, ≥3 them** | 23.292 / 31.497 / 48.734 | 27.42 = 2.84× / **30.00 = 3.15×** / 56.49 = 6.66× | **37.82 = 2.17× / 47.34 = 2.09× / 60.82 = 5.99×** |

*(triples are corpus-roots / radius-2-deepened / playouts, all from
`crates/pistol-solver/tests/wp15b_census.rs` at **`7941775`**.)*

**Re-derived, because revision 3 changed the generation rules and revision 4 did
not re-derive §6.2 from the instrument it had just committed** — instances four
and five of this work package's recurring pattern.

**The column is a MIXTURE and both populations are reported.** Batched nodes are
**70.8 % / 61.5 % / 92.5 %** of each regime; the rest take a forced row emitting
two or three cells. `quiet_top_k` and `widen_schedule` govern only the batched
population, and on it **option B's reduction is 1.31–1.41×** at the corpus depths
— its blended cell was flattering it by half. **Option C's honest figure is
2.09–2.17×** there, and the case for C over B is STRONGER on the disaggregated
numbers than on the blended ones this document quoted through three revisions.

**Sampler sensitivity, re-taken from the instrument.** The superseded radius-8
deepening regime gives C **4.01× (all nodes)** against the reported regime's
**3.15×**, and a ball mean of 123.66 against 94.50 — a **21.4 %** move in the
ratio and **23.6 %** in the ball. Revision 4 stated 3.1× → 2.4× and 23 % / 24 %
from the pre-instrument harness. No verdict in this document turns on a quantity
that moved by less than the sampler does.

### 6.3 The options

| Option | Theory standing | Cost | Failure modes |
|---|---|---|---|
| A — count ≥3 both sides | **No completeness licence.** `LAW-SUPPORT` k=2 licences windows at ≥2, and T10 adds that a window made hot this turn held ≥2 before — so count 3 misses every plan a PAIR creates from a count-2 window, which is the two-stone move this game is about | Cheapest, 4.6–17.4× | Provably k=2-incomplete. The reviewer built the position: P1 (0,0)(1,0)(2,1)(1,2)(0,3), pair {(2,0),(3,0)}, `t = 4`, `(2,0)` in own count-2 windows only |
| B — count ≥2 both sides | Full licence both sides | Only 1.6–3.8×; at corpus roots the staged set is 61 % of the whole ball | Its opponent half buys the least, per §6.4's lemma |
| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | **2.09–2.17× on the batched population**, 2.84–3.15× blended, from the committed census; **MEASURED 29 % of C's Tier T lies OUTSIDE the radius-2 ball** (6.83 cells/node at corpus roots) | Asymmetric, so argued in §6.4. Residual: no cells blocking an opponent count-2 window; left to Tier Q's delta ranking, which is a set of 23.2 cells/node against a quiet allowance of 16 |
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
- ~~`Cover::Impossible` at phase 1 inherits the same schedule~~ — **inert under
  revision 4 onward and struck**: §5.3 routes `Impossible` to the candidate loop
  only at a PV node or the root, and §7.2 exempts both from every schedule. A
  leftover from when `Impossible` could reach a non-PV loop.
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

**Where S-E observes, and against what — respecified, because revision 3's seam
cannot be built.** A REVIEW-design closed all three of its doors: `pvs` is
`pub(crate)` so `Run` is unreachable from an integration test;
`crates/pistol-solver/tests/common/plans.rs` is another crate's integration-test
module and no `src/` can `use` it; and D-115 forbids widening either to let a test
reach it, while D-129 forbids putting a CORRECTNESS invariant behind
`debug_assertions` — "never a quiet demotion". Revision 4 splits the claim in two
and gives each the mechanism its own class calls for.

- **The expensive half is a test over a PUBLIC generator.** `pistol_search::staged`
  becomes public with one entry point returning the ordered candidate vector and
  its forced count. An integration test in `crates/pistol-search/tests/` walks
  positions with the existing reference walker and, at every one, compares the
  forced prefix against a hitting set computed by an **independent from-scratch
  plan-family implementation in that crate's own `tests/common/`** — the pattern
  rule 7 already names for movegen (`bruteforce.rs`) and the search
  (`reference.rs`). That is NOT D-295's defect: D-295's defect was checking a
  shipped oracle against a test-side oracle DERIVED FROM THE SAME FIXTURE; this
  checks a shipped generator against an independently written referent.
- **The cheap half is an always-on `assert!` in `visit`.** The test above sees what
  the generator EMITS; D-124's reproducer (`cells.pop()` after `order`) is a drop
  AFTER generation, which no test of the generator can see. So `visit` carries a
  named invariant — the first `forced` candidates are searched unless a cutoff or
  an abort intervened — as an `assert!`, not a `debug_assert!`: its violation
  makes the answer wrong, which is D-129's own criterion for the macro. Cost is
  one comparison per node. It is a PRIVATE invariant guard, which is precisely
  what D-115 permits in-source.

Two consequences worth stating. The gate then runs in whatever profile CI uses
without a `release-checked` argument, because an `assert!` survives release —
revision 3's `debug_assertions` seam would have been compiled out of
`tools/search_oracle_check.sh`, `tactical_check.sh`, `determinism.sh` and
`movetime_check.sh`, all of which drive release binaries. And §13's estimate is
re-based: the traversal cost is the test's, once per fixture, not "at every node".

It also dissolves three of §8.1's findings at once:It also dissolves three of §8.1's findings at once: no argmax type confusion, no
verdict decided by a free stone's 2–24 eval units, and no comparison across two
universes the adopted Tier-T option makes non-comparable.

**Reduced S-C — a cheap mate-class regression only.** Radius 1, the three fixtures
plus one built mate-in-3 that gate 10 already affords at **MEASURED 17.89 s**. Not
a corpus sweep, not radius 2, not depth 3 at radius 2. Its mate class must state
whether `LossInTurns` roots are included or excluded **by name** — `compact_mated_in_2`
falls in neither class today.

### 8.3 The other three parts, re-scoped

- **(a) tactical suite under Staged. Revision 4's derivation contained a
  CORRECTNESS ERROR and is redesigned.** It claimed "the `expect not-mated` cases
  are value claims that narrowing cannot break in the dangerous direction — a mate
  score requires a proven line, and narrowing cannot invent one."

  **That is false, and §7.2 says so two sections earlier**: the non-PV cut is "a
  forward prune of the LMR family", and forward-pruning a DEFENDER's saving reply
  is exactly how a search proves a mate that is not there. The mechanism is
  concrete: at an opponent PV node every reply is generated, but each non-argmax
  reply is scanned at a null window — a non-PV, capped search. If that reply's
  saving continuation lies past the last batch boundary, the scan returns a losing
  value, `scan <= alpha`, no re-search fires, and the opponent's true best is
  discarded. The root then reports a mate. **A forward prune can fabricate a
  mate, and this design's own §7.2 named the prune.**

  **The redesign: the two gate configs disable the cut, and the derivation becomes
  a containment argument.** With `quiet_top_k` at or above the whole quiet pool no
  batch boundary ever binds, so on a BATCHED row the staged set is
  `Tier T ∪ (the whole radius ball)`, which is a **SUPERSET** of what the radius
  policy offers. Anything the radius search finds at a given depth, Staged finds.
  The other two rows are complete on their own terms: the FILTERED row by
  `LAW-FORCE` (every non-losing move hits all plans), the WIN-NOW row by the
  argmax property (nothing beats `mate_in(k+1)`). So `require 20` follows from the
  radius suite's own 20 of 20, which D-204 already binds.

  **The number is measured, not guessed.** The five `gate_v0` cases hold 11 stones
  and their radius-1 balls are MEASURED at 22 / 22 / 22 / 18 / **30** cells. Three
  turns deeper is at most 17 stones, and a radius-1 ball is bounded by
  `7 × stones − stones = 6 × 17 = 102`. `quiet_top_k = 128` therefore cannot bind
  on any position these cases reach, and `configs/gate_staged_v0.toml` commits it
  with that arithmetic in the file.

  **What this deliberately gives up, said plainly.** The tactical suite under
  Staged then tests the THREAT MECHANISMS and not the quiet cut. That is the right
  division — the cut is a strength knob and rule 6 makes SPRT its judge — but it
  means gate (a) is silent about the prune, and the prune is judged instead by
  §12 item 3's movetime measurement, by the SPRT, and by the differential in §8.2.
  Recorded so no reader takes a green tactical suite for evidence about the cut.

  **The 15 `instrument_v0` cases keep the cut and need no exemption**: MEASURED,
  eleven are `mate_in_1` (WIN-NOW row) and four are `must_block` against a
  five-in-a-row (the opponent holds a hot window, so the FILTERED row). None takes
  a batched row, so `configs/instrument_staged_v0.toml` runs them at the committed
  `quiet_top_k = 16`.

  **The threshold, pre-registered here: `require 20`.** A case the derivation does
  not cover would have had to be named and pre-registered below 20 before the
  first run; the derivation now covers all twenty, by containment on five and by
  completeness on fifteen. A failure is a red gate to investigate, never a licence
  to re-read the threshold (D-204, inherited).

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
| M3 | The FILTERED row emits `Cover::cells()` flattened at phase 0 and does not regenerate at phase 1 | S-E | **BUILT**: §5.3's two sealed five-stone rows, whose only cover is `Two{(4,4),(5,0)}` |
| M4 | Minimum-cardinality covers instead of inclusion-minimal | S-E | **BUILT, and revision 4's witness was inert.** The shape must have a 1-cover COEXISTING with a minimal 2-cover; `cover.rs`'s flat-list counterexample has no 1-cover, so the two notions coincide there and the mutant is an identity. VERIFIED on the shipped solver: P1 at (1,0)(2,0)(3,0)(4,0)(0,1)(0,2)(0,3)(0,4) sealed by P2 at (-1,0)(6,0)(0,-1)(0,6) gives `Minimal([One((0,0)), Two{(0,5),(5,0)}])`, and minimum-cardinality drops the pair |
| M5 | The WIN-NOW row emits the cover union instead of the win class | **mate** | `mate_in_1_own_win_beats_blocking_the_opponent`. Revision 4 named "own win-now cells dropped from the FILTERED set", a path §5.3 deleted — on that position `can_win_this_turn` is `Some`, so the node takes the WIN-NOW row and `blocking_covers` is never called |
| M6 | The overload return drops its `can_win_this_turn` guard | **mate**, not S-E | **BUILT**: P1 with one sealed five-run (win in one ply at (5,0)) and P2 with three disjoint sealed five-runs at rows 8 / 16 / 24 — 8 apart keeps every placement legal under rule 5 and 8 > 5 guarantees no shared window. VERIFIED: `can_win_this_turn(P1,Two) = Some(OnePly{(5,0)})` while `unblockable_double_threat(P2,Two) = true`. Its class is mate and not S-E, because the mutant RETURNS rather than emitting and S-E is blind at an `Impossible` node |
| M7 | Tier T qualifies at ≥3 for the mover (option A) | informative | survival is a recorded finding under §6.5's second branch, with a diagnosis, per D-281 |
| **M8** | **`visit` drops the last candidate after generation** — D-124's own reproducer, `if cells.len() > 1 { cells.pop(); }` | **the `assert!`** | **A FILTERED root**, where `forced` is the whole set and `beta = INFINITY` guarantees the loop exhausts. Revision 4 registered no mutation for the `assert!` half at all, while §8.4 opened by quoting D-295's finding that asserting an instrument's strength rather than measuring it is the defect. Registered because the honest reading is that on the 70.8 % BATCHED population `forced == 0` and the assertion is VACUOUS there — it earns its place only on forced rows, and the mutation is what shows which |

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

| document | mode | `quiet_radius` | `quiet_top_k` | `widen_schedule` | why |
|---|---|---|---|---|---|
| `configs/instrument_staged_v0.toml` | instrument | 2 | 16 | `[32]` | the SPRT seat and the snapshot's AFTER; the incumbent is radius 2 |
| `configs/gate_staged_v0.toml` | instrument | 1 | **128** | `[256]` | `tactical_v0.txt`'s five `depth_turns 3` cases run at radius 1, and 128 **disables the cut** on them — MEASURED balls 22/22/22/18/30 at 11 stones, bounded by 6 × 17 = 102 three turns deeper (§8.3) |
| `configs/play_staged_v0.toml` | play | 3 | 16 | `[32]` | the movetime measurement's incumbent is `play_v0.toml` at radius 3 |

Every other key is identical to the radius document it is the counterpart of, so
each is complete under rule 1 without restating the whole schema here; revision 4
promised "three complete documents" and printed the policy block of one.

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
spelled the EXACT-count union in §11's referent — two different sets in one
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
| `the_table_move_ordering_under_staged_is_within_tier` | that a Tier-Q table move does NOT reach index 0 when Tier T is non-empty, and that `Run::salvage` still returns a completed root subtree's exact score — the property that survives, as against the ordering claim that does not |
| `new_plan_creation_gets_no_forced_slot` | the absence of a plan-creating non-hitting cell from the forced prefix, with its delta made maximal |
| `tier_t_qualification_matches_adopted_matrix_option` | the Tier T set against an independent **`us@{2,3} ∪ threat_cells(us) ∪ them@{3} ∪ threat_cells(them)`** — the UNION reading §10 establishes, since `LiveCount` cannot express ≥ 4. On a position where exact-2 and ≥2 DIFFER, and one where the `LiveCount`-only and union readings differ. Revision 4 spelled the `LiveCount`-only referent here while §10 corrected it two sections earlier |
| `widening_schedule_fires_on_fail_low_and_is_deterministic` | the per-class widening counters AND that a non-PV node exhausting a finite schedule while truncated exists. Two runs agreeing is a property the defect preserves and is not the criterion |
| `the_root_and_every_pv_node_search_the_full_staged_universe` | the emitted set size at ply 0 and at PV nodes against the unbatched universe |
| `a_truncated_fail_low_stores_no_transposition_record` | the table's contents after a node that stopped truncated |
| `staged_ordering_deterministic_within_and_across_tiers` | the whole emitted order, twice, with equal delta scores forced |
| `stage_counters_reported_in_search_info` | each counter non-zero on a position built to fire it, zero on one built not to |
| `gap_trap_answered_in_tier_f` | that `PAT-GAP`'s singleton gap cell is in the FORCED prefix from the defender's side |
| `colony_family_passes_under_staged` | the move played on ≥ 6 built distant-cluster positions |
| `tactical_suite_holds_at_its_rederived_thresholds_under_staged` | the `require` count of `tactical_staged_v0.txt`, at the three staged configs |
| `staged_forced_prefix_contains_every_minimal_cover_cell` | **S-E, half one**: the public generator's forced prefix against an independently written plan-family referent in pistol-search's own test tree |
| `visit_searches_every_forced_candidate` | **S-E, half two**: the always-on `assert!` in `visit`, which is what sees a drop made AFTER generation — D-124's own reproducer |
| `a_win_now_node_generates_only_the_win_now_class` | the emitted set on a `Minimal` node where the mover wins by PAIR — revision 2's STOP-2, where the natural reading generated no winning cell |
| `cover_impossible_at_phase_zero_still_generates_the_win_now_class` | the emitted set at a ROOT (always a PV node, so §5.2 cannot fire) with `Cover::Impossible` and a mover win available — revision 2's STOP-1 |
| `cover_impossible_at_phase_one_with_a_win_in_one_ply_cell` | that the win cell is generated even when it lies in no Tier-T window (the reviewer's R1 construction) |
| `stones_left_and_hit_budget_are_read_from_core_at_both_phases` | the budget at `Phase::Second`, which is the reachable class — turn 1 exists only at ply 0, always a PV node, on an empty board where the predicate is `false` at either budget, so revision 3's stated justification named an unreachable case |
| `the_protocol_runs_after_the_horizon_return_and_the_table_cutoff` | the node count and the table hit rate against a build with the protocol placed earlier — §5.35's placement, which is otherwise unattributable |
| `a_forced_row_emits_no_tier_t_or_tier_q_cell` | that Tier F is the WHOLE set on the WIN-NOW and FILTERED rows, which is what makes the batch cut structurally unable to touch it |
| `the_two_predicates_agree_everywhere` | `blocking_covers(us,b) == Impossible` against `unblockable_double_threat(them,b)` at ALL THREE budgets over built hard cases and seeded playouts, with a NON-VACUITY assertion that the `Impossible` branch is reached at each budget — M5-E's soundness, and a sweep that never reached a budget would agree there by not testing it |
| `a_radius_policy_search_is_byte_identical_to_the_committed_engine` | bestmove, nodes and PV under `Radius` before and after this WP — the scoping claim of §7.2 |
| `the_threat_state_stays_in_step_with_the_game_and_the_eval` | `THREAT_DESYNC` never fires across `place`/`undo`/`reset_to` over seeded playouts — the D-41 seam's third member |
| `the_fallback_under_staged_answers_from_the_quiet_radius_ball` | the turn `fallback_turn` returns under a Staged policy, and that it reads no threat state — the bounded, pure property WP-1.4's movetime ceiling rests on |
| `no_candidates_under_staged_is_refused_by_a_policy_agnostic_error` | the error variant at a root the policy cannot serve; `SearchError::NoCandidates { turn, radius }` names a `radius` a Staged policy has three of |
| `tier_t_cells_match_an_independent_window_walk` | the emitted Tier T against a from-scratch enumeration, on a position where the `LiveCount`-only reading and the union reading DIFFER |

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
   §9's `--config` puts it inside the snapshot's INVARIANT block. Under `Staged`
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
   accelerates `min_hitting_set_exceeds`, which under M5-E (§5.2) is **no longer
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
   per-node path. EXPECTED GAIN BRACKET: **[1.10×, 1.35×]** on that path, derived
   from the only two numbers available — 662 ns fresh against 533 ns with a reused
   buffer is 1.24×, and the accessor removes the same allocation plus the
   per-window public-boundary crossing. ABORT THRESHOLD: below 1.05×, or any
   regression in whole-search nps. INSTRUMENT: one IQR-gated bench reporting
   **nps AND time-to-depth**, per rule 5, not the snapshot — which reports
   `depth_turns` and `nodes` only. ONE CHANGE = ONE COMMIT.
   **And the number is re-taken on the right population**: 533/662 ns were
   measured over all 24 corpus roots, but §5.3 does not extract Tier T on the
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
   WP as that consumer, and §15 takes the line.

5. **The census is `crates/pistol-solver/tests/wp15b_census.rs` at `7941775`**,
   and `tools/baseline_snapshot.sh` is at **`e889b5b`** — both named with their
   revisions, which CLAUDE.md's instrument clause requires and revisions 1–4 did
   not do. Revision 4's two cross-references resolved in a circle (§6.2 pointed at
   §12, §12 pointed back at §6.2) and neither carried a SHA. The
   radius-2-confined regime is the reported one.

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
4. S-E, and D-124's flip clause discharged. Its seam is the PAIR of §8.2 — a
   public generator driven by a test in pistol-search's own tree against an
   independently written referent, plus an always-on `assert!` in `visit` for the
   drop a generator test cannot see. Revision 4's line still registered the
   `#[cfg(debug_assertions)]` observer that §8.2 had withdrawn two sections
   earlier — the un-re-read claim, inside the ADR list itself.
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
7. **D-263's three remedies stay unimplemented, and revision 3's adoption of the
   first is WITHDRAWN** — it needs three disjoint families and 1 of 24 corpus
   roots has them, and it accelerates a query M5-E deletes. The larger cut in the
   same hotspot comes from removing the duplicated query (−29 % to −42 %).
   **AND D-263 NAMED THE WRONG HOTSPOT**: Tier-T cell extraction is MEASURED at
   about 6× both threat queries combined. Registered here with its own bracket and
   abort threshold, before the change that touches it (rule 5).
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
16. **D-261 gains a query.** Tier T needs the empty cells of live-2 and live-3
    windows and the public surface has no route to them; D-261's flip clause names
    this WP as the consumer. `ThreatState::live_cells_at_count(side, LiveCount,
    &mut Vec<Coord>)`, additive, with the map entry D-267 requires of a new query
    naming its calculus ID (`LAW-SUPPORT`'s k=2 qualification), and the recorded
    coincidence that it equals `cells_raising_to_hot(side, NearHot::Three)` at
    count 3 while MEANING something different.
17. **`pistol_search::staged` is public**, because S-E's expensive half is a test
    over the generator and D-115 forbids widening `pvs` to reach `Run`.
18. **The `!is_pv` gate's root safety is an `assert!`, not a sentence.** "Ply 0 is
    always a PV node" is a property of today's construction; Stage 4's aspiration
    windows would narrow the root. Named invariant at the return site.
15. **The two gate configs disable the quiet cut**, so gate (a) tests the threat
    mechanisms rather than the prune, and the prune is judged by SPRT, by the
    movetime measurement and by S-E. The line records what a green tactical suite
    under Staged does NOT evidence.
20. **`Run::salvage`'s documented ground does not hold under `Staged`.** Its doc
    says "the first root candidate is the table's move … so a salvaged answer is
    never worse-informed than the last completed depth's"; under `Staged` on a
    batched root Tier T comes first. The salvage stays SOUND — it is a completed
    root subtree's exact score — but the ordering claim must be restated. Amends
    D-207's salvage rationale.
21. **`pistol_search::staged`'s entry point and `candidate_cells`' divergence.**
    The public entry point takes `(&GameState, &ThreatState, &mut dyn Eval,
    StagedParams)`, putting a `pistol-solver` type in `pistol-search`'s public
    API; and `candidate_cells` keeps a signature that cannot express `Staged`, so
    the public function and the search's real candidate set diverge under it.
19. **Rule 9 and `pvs.rs`.** The file is 552 lines and carries a
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
| DECISION-RED-TEAM, matrix M5 | revision 3, `7ad466b` | **FELL** → M5-E, supplied by the red team |
| REVIEW-design | revision 3, `7ad466b` | **FAILS** — 2 BLOCKING, 9 MAJOR, 10 MINOR, 6 findings REJECTED with reproducers |
| REVIEW-design | revision 4, `f762c9a` | **FAILS** — 3 BLOCKING, 10 MAJOR, 10 MINOR, 8 findings REJECTED with reproducers. **No matrix reopened and no STOP**: every option selection survived attack on its merits, and what failed was transmission |
| REVIEW-design | revision 5 | dispatched |

**Four matrices in one work package recommended an option a fresh context then
dominated** — M0 (f), M2 W-E, M3 S-E, M5 E — and in three of the four the
dominating option cost LESS rather than more. D-276 recorded one such instance as
notable; four is a base rate, and it is a fact about self-review rather than about
any one matrix. Recorded here rather than in a single matrix's line.

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

*Revision 5. Reviews owed: a fresh REVIEW-design against this revision. Every
matrix has been attacked by a fresh context and four of the six changed their
selection; revision 4's review reopened none of them, which is the first round
where the SELECTIONS held and only the transmission failed. IMPL does not start
until REVIEW-design passes.*
