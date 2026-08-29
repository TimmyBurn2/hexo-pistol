# WP-1.5d — design: the safety-net candidate cap, T-BELOW-T scope

**REVISION 5 — THE SPLIT APPLIED. WHICH SECTIONS GOVERN WHICH SESSION, AND WHY
THE DEFERRED ONES ARE FROZEN RATHER THAN FIXED.** `docs/decisions.md` **D-482**
splits this work package after D-481, and **D-483** makes it a standing law that
a design document carries no measured numbers at all.

| section | governs |
|---|---|
| §1 the quoted facts, §2 the mechanism, §3 the gate, §4 the change surface, §6 the store rule, §7 the tests | **SESSION (A)**, the implementation. Cleared twice with zero correctness findings and unchanged across revisions 3 and 4 |
| **§5 the calibration, §8 the bench** | **DEFERRED TO SESSION (B)** — every number they carry is re-taken there, in one run by one registered instrument into one digest-named artifact, and **no number in them is admissible as data** (D-482) |

**§5 AND §8 ARE LEFT EXACTLY AS THEY STAND AND ARE NOT REPAIRED.** They are the
record of the practice D-483 abolishes — five review rounds' worth of numbers
assembled across runs, budgets, radii and artifacts — and rewriting them now
would destroy the evidence for the law while changing nothing anyone may use.
They are cited by (B) as motivation and scoping history, never as measurement.
**Every measured figure elsewhere in this document is under the same rule**,
including the ones in the revision headers below: they record what a review found
and when, not what a run may conclude.

**REVISION 4.** The scoped re-review FAILED revision 3
(`docs/experiments/wp15d_design_REVIEW_rev3.md`: 3 BLOCKING, 4 MAJOR, 3 MINOR,
**none a correctness finding**, five of twelve closure rows introducing something
new) and the design gate's cap was exhausted. **`docs/decisions.md` D-480 is an
architect grant of ONE capped continuation round, scoped to that report's ten
enumerated remedies and nothing else**, under a single scoped re-review where any
NEW finding is a STOP and a split. This revision is that round. It touches no
option, no selection and no mechanism — none of those was ever in question — and
every change below is one of the ten remedies applied in the words the report
states it in.

**THE DEFECT CLASS, NAMED ONCE SO THE FIXES READ AS ONE THING.** Every finding in
both design-review rounds was the same shape: **a number quoted away from the run
that produced it** — a `movetime` cell under a `Stop::Nodes` bench, an incumbent
from a different artifact, a depth vector from a different radius, a fraction
assembled from two budgets, a counter registered against an instrument that
cannot read it. D-479 is that lesson and this revision applies it mechanically:
**every number below now names the artifact, the seat, the budget and the radius
it comes from, or it is deleted.**

**REVISION 3.** Revision 2 FAILED its fresh-context REVIEW-design
(`docs/experiments/wp15d_design_REVIEW.md`) with 3 BLOCKING, 5 MAJOR and 4 MINOR
— **and none of them a correctness finding**: the reviewer attacked §6.3's store
rule five ways and §2.2's guard at every turn number, and both held. All three
BLOCKINGs are REGISTRATION failures and this revision is the one fix round the
round allows. What changed:

- **§8's spread expectation was a `movetime` cell registered against a
  `Stop::Nodes` bench** — the exact substitution `artifacts/wp15d_turn_axis_v1.txt`'s
  own header and D-478 forbid, committed by the session that wrote both. The
  claim "expected exactly inert — MEASURED 0 prune events" is **false at the
  registered budget**, and §8 now separates the two claims it was conflating.
- **No test could kill the `ply > 1` mutant** — revision 2's own headline
  correction shipped with no falsifier. §7 row 5 is replaced by one that can.
- **§4's file list was short by six files**, because `kind = "staged"` cannot see
  a `StagedParams` struct literal. The enumeration is now the one that can.
- Five MAJORs are closed with it: the corpus wall ratios' provenance, the
  handshake change (DROPPED, and why), §8's missing aggregating instrument and
  bench configs, §5's flatness anchor, and `lift`'s treatment of a
  mate-terminated search.

**Revision 2.** Revision 1 was written against matrix M2 revision 3, which
**FELL** to its DECISION-RED-TEAM (`docs/experiments/matrix_M2_REDTEAM_round3.md`,
`bc003c9`). Two of that report's findings land on this document directly and both
are CORRECTNESS findings, which D-424 states are never overruled, only fixed:

1. **The `ply > 1` guard does not express what D-478 selected.** Rule 3 gives
   turn 1 ONE stone (`crates/pistol-core/src/rules.rs`, `stones_in_turn`), so at a
   turn-1 root ply 1 is already the OPPONENT's first stone and `ply > 1` caps half
   the opponent's reply — the turn-incoherence the matrix struck two rows for,
   moved one turn down. **The guard is now `turns_from_root() > 0`**
   (`crates/pistol-search/src/pvs.rs:523-525`), which is turn-coherent by
   construction at every turn number. §2.2.
2. **Revision 1's §6 soundness argument was scoped to a single search and is
   false across the searches of a game.** The transposition table is KEPT between
   searches — `crates/pistol-search/src/search.rs:52-56`, *"successive searches in
   one game share what they learned … `Searcher::clear` is what a new game
   does"* — so a node truncated deep in search *N* is search *N+1*'s root turn,
   where this design promises full width. MEASURED by the red team over 40
   self-play games with a warm table: **41 cutoffs inside the played turn on
   bounds the capped node never proved**, against 0 for the incumbent, and
   **41 → 0** with a cold table, which isolates the cause exactly. Revision 1's
   truncate-before-promote ordering does not touch this class. **§6.3 adopts
   `WPQ_seed.md` §7.2's store rule**, which revision 1's matrix wrongly dismissed.

Everything else stands. **What is implemented is still the option D-478 selects
— a cap that never reaches the node where the move is chosen — and these are two
defects in how revision 1 spelled it, not a different option.** D-478's own words
are *"the turn-coherent `ply > 1` scope"*; where those two halves conflict,
turn-coherent is the operative one and `ply > 1` was the defective spelling.

**WHAT THIS DESIGN DOES NOT RE-ARGUE.** The field, its eleven rows and their kill
conditions are the matrix's; the selection is D-478's operator ruling; the price
of that selection — 153 fewer book openings at depth ≥ 3 than the deferred
T-ROOT row, a 1.47× wall-time swing against it, and the D-95 depth debt left
unpaid in the channel it is defined in (§8, §9) — is stated on the matrix's face
at §4 and is not restated here beyond this sentence (D-423: a claim a document
makes twice is a defect waiting).

---

## 1. THE TWO FACTS THIS DESIGN QUOTES BEFORE IT PROPOSES ANYTHING

- **The row it modifies.** `crates/pistol-search/src/staged.rs:222-236`: when
  `tier_t_union` comes back empty the batched row falls back to
  `within_radius(board, params.quiet_radius)`, delta-ranks it, and emits it whole
  with `out.forced = 0`. Tier F is provably empty on every row `batched` is
  called from (`staged.rs:209-213`) and Tier T is empty by the branch condition,
  **so a cap inside that branch can exclude no Tier-F and no Tier-T cell — a
  GUARD, not a construction**, and §7 tests it as one.
- **The unit the scope is expressed in.** `ply` increments once per STONE
  (`crates/pistol-search/src/pvs.rs:397-399`,
  `PlyOutcome::TurnContinues => self.child(depth_plies - 1, alpha, beta, ply + 1,
  index == 0, true)`) and the engine's answer is the first `Turn` of the
  ply-indexed principal variation (`crates/pistol-search/src/search.rs:328-329`).
  The move that gets played is a TURN, and **the number of plies it spans is not
  a constant**: rule 3 gives turn 1 one stone and every later turn two
  (`stones_in_turn`), and rule 4 truncates a turn whose first stone wins. So no
  `ply` threshold expresses "the played turn" at every turn number, and the guard
  is written in the unit the answer is spelled in: `turns_from_root()`
  (`pvs.rs:523-525`), which is `state().turn() - root_turn`. D-477 requires the
  quotation at the line where the unit is SPENT, and `search.rs:328-329` is that
  line.

- **The table outlives the search.** `search.rs:52-56`: *"The table is kept:
  successive searches in one game share what they learned … `Searcher::clear` is
  what a new game does"*, and `Table::new_generation` (`tt/mod.rs:116-118`) only
  advances a counter. **So `turns_from_root` is not a function of the key either
  — across searches the same position sits at a different distance from the root
  — and §6.3 is what makes that safe rather than an argument about keys.**

---

## 2. THE MECHANISM

### 2.1 Where the truncation goes, and why not in `staged.rs`

**In `pvs.rs`, immediately after the empty-set check and BEFORE
`set.promote_table_move(table_move)` (`pvs.rs:324-328`).** Three reasons, in
order of weight:

1. **It is where the search's own distance-from-root lives.**
   `turns_from_root()` is a `Run` method (`pvs.rs:523-525`) reading `root_turn`,
   which `staged_candidates` has no access to and no business having: its own doc
   (`staged.rs:104-107`) states that the entry point reaches exactly three things
   — the board, the threat state and a delta ranking. The guard is a property of
   the SEARCH, not of the position, which is the whole content of §6.3.
2. **It is where the existing ply-scoped set modification already lives.** The
   root's zone restriction (`pvs.rs:336-347`) is a candidate-set edit guarded on
   `ply == 0`. This is the same shape one ply-band over.
3. **It is exactly where every measurement in the matrix was taken — the SITE,
   and only the site.** The evidence instrument truncates at this point
   (`artifacts/wp15d_m2_evidence_instrument_v2.txt` PART 6), so shipping the
   truncation anywhere else would ship a site the receipts do not describe.
   **It says nothing about the SCOPE**: every matrix cell was taken under a
   ply-indexed predicate, and `turns_from_root() > 0` is measured nowhere in
   those artifacts. §5's calibration and §8's bench re-take every number that
   governs anything, at the implementation revision, under this scope.

### 2.2 The guard, stated as it will read

```rust
// The safety-net cap (docs/decisions.md D-478): on the row where Tier F and
// Tier T are both empty the emitted set is the whole quiet ball, which is
// unbounded in the stone count. The exemption is the ROOT TURN, and it is
// spelled in turns because the played turn is one stone at turn 1 and two
// after it (rule 3) -- no ply threshold names it at every turn number.
let truncated = params.safety_net_top_k > 0
    && self.turns_from_root() > 0
    && set.used_quiet_safety_net
    && set.cells.len() > params.safety_net_top_k as usize;
if truncated {
    set.cells.truncate(params.safety_net_top_k as usize);
}
```

`used_quiet_safety_net` is already on `StagedSet` (`staged.rs:65-69`) and already
written by `batched`; nothing new is computed to decide whether the guard fires.
The `len() >` test is not an optimisation — `truncated` is read again at the
store (§6.3), and a node whose pool was already at or below K searched its whole
set and has nothing unsound to withhold.

### 2.3 Truncate first, then promote — and what that buys

The truncation is placed BEFORE `promote_table_move`, so the table's move is
promoted **within** the truncated set and can never re-admit a cut cell. The
emitted set is then a function of (position, `quiet_radius`, K,
`turns_from_root`) and of nothing else — no transposition contents and no search
history.

**AND THAT LAST TERM IS NOT A FUNCTION OF THE POSITION**, which revision 1 got
wrong and which §6.3 exists for: the same position sits at a different distance
from the root in successive searches of one game, so this ordering makes the
emitted set well defined WITHIN a search and does nothing at all across searches.
Revision 1 offered this ordering as the soundness argument; it is not one. It is
kept because it is worth having on its own terms — it is the ordering the matrix's
F11 measurement was taken under — and because a set that depends on the table's
contents would make §6.3's rule harder to state, not because it closes anything.
The cost is real and named: a best move found at a deeper iteration is not
re-searched at this node if it has fallen outside the top K.

### 2.4 K counts CELLS of the quiet ball

`StagedSet::cells` is a `Vec<Coord>`, a turn is two sequential same-side plies
(D-9), and a *pair* is never a unit the candidate set holds. There is nothing at
this seam for K to count but cells, and on this row the set is exactly the ball,
so K counts ball cells.

### 2.5 Tie-break, and why rule 4 holds by construction

`within_radius` returns cells ascending (it drains a `BTreeSet`);
`staged::delta_rank` sorts by `Reverse(Eval::delta)` with `sort_by_key`, which is
stable, so equal-scoring cells keep ascending coordinate order (D-5, D-7). The
K-th and (K+1)-th cells are therefore separated by a total order containing no
clock, no thread and no hash iteration. §7 seats it anyway.

---

## 3. THE GATE

**One new key on `CandidatePolicy::Staged`: `safety_net_top_k: u64`, where `0`
disables the cap and is the committed value in every one of the twelve
documents.**

**Why one key with an off-value and not a separate boolean.** The `q_depth_turns`
precedent is exact — `config.rs`'s own doc for it reads *"`0` disables the
extension"* — and here `0` is the semantically true off-value: the cap is a
ceiling, and no ceiling is `0` meaning "do not truncate". This is the opposite of
`quiet_top_k`, where a LARGE value means disabled (`U3_tier_t.md` §10, and the
reason WP-1.9's design needed a separate boolean); that asymmetry is why the two
knobs cannot share a shape.

**THERE IS NO VALIDATION CHECK, AND THAT IS THE ANSWER RATHER THAN AN OMISSION.**
Revision 2 said "the bound lives here" and named no bound; revision 3 named a
`u64`-representable-as-`usize` check, **which on this target can never fail** —
and it did so in the paragraph that deletes another check for never firing
(re-review MINOR 10). Both are now gone, on one principle. **The type is the
domain**: `safety_net_top_k: u64` admits exactly `0` (off) and `n > 0` (cap at
`n`), `deny_unknown_fields` and serde's own typing refuse everything else with a
named error, and there is no invalid value left for a validator to catch. **No
upper bound** either: a K above every pool is a no-op, so refusing it would
refuse a document spelling the OFF behaviour a second way. §7's first row pins
that `0` reproduces the pre-change engine, which is the only property of this
key a test can carry.

**`quiet_top_k` and `widen_schedule` are NOT re-purposed and NOT retired.** They
carry `U3_tier_t.md` §10's committed semantics — a quiet tier ADDED beyond Tier
T — in twelve documents, `validate.rs:94-121` validates them and the search does
not read them (`params.rs:47-57`, D-353). Silently inverting them is the defect
that stopped WP-1.5c. This design states their status once, here, and no other
section restates it.

---

## 4. WHERE THE CODE CHANGES — the enumerated set, not a glob

**THE ENUMERATION, AND THE COMMAND THAT CAN ACTUALLY SEE THE SITES.** Revision 2
verified this list with `/usr/bin/grep -l 'kind = "staged"'`, which reads TOML and
**cannot see a `StagedParams` struct literal in Rust** — REVIEW-design BLOCKING 3,
which found six files that way. The enumerations are:

```
/usr/bin/grep -l 'kind = "staged"' configs/*.toml | LC_ALL=C sort     # 12 documents
/usr/bin/grep -rln 'kind = "staged"' crates/ --include=*.rs           # 1 embedded TOML
/usr/bin/grep -rln 'StagedParams {' crates/ --include=*.rs            # 11 files, 18 sites
```

| file | change |
|---|---|
| `crates/pistol-engine/src/config.rs` | `safety_net_top_k: u64` on `CandidatePolicy::Staged` |
| `crates/pistol-engine/src/validate.rs:81-92` | destructures all fields with no `..` — **will not compile** without the new one; §3 names the check it adds |
| `crates/pistol-engine/src/instance.rs` | destructures and passes through (`StagedParams` literal) |
| `crates/pistol-search/src/params.rs` | `StagedParams` gains `safety_net_top_k: u64` |
| `crates/pistol-search/src/search.rs` | the SAME check again, because a `SearchParams` can be built in code and never pass through a document (rule 1) — the two-crate pattern `radius` and `q_depth_turns` already follow |
| `crates/pistol-search/src/pvs.rs` | §2.2's guard AND §6.3's store rule — one binding, `truncated`, read at both sites; also a `StagedParams` literal site |
| `crates/pistol-search/src/quiescence.rs` | `StagedParams` literal site |
| `crates/pistol-search/src/info.rs` | `StageCounters` gains `safety_net_capped_rows`, `safety_net_emitted_cells`, `safety_net_pool_cells`, and the withheld count SPLIT BY BOUND KIND — `safety_net_upper_withheld` and `safety_net_exact_withheld`, because one total cannot tell a rule that wrongly stores `Exact` from one that wrongly stores `Upper` (REVIEW-impl MAJOR 1) — §5's and §8's instruments read these |
| `crates/pistol-search/tests/common/mod.rs` | `staged_params` / `staged_searcher` gain the parameter (literal site) |
| `crates/pistol-search/tests/staged_tests.rs` | literal sites |
| `crates/pistol-search/tests/staged_colony_family_tests.rs` | literal sites |
| `crates/pistol-search/tests/staged_differential_gate_tests.rs` | literal sites |
| `crates/pistol-search/tests/staged_pattern_fixture_tests.rs` | literal sites |
| `crates/pistol-search/tests/staged_tier_t_threshold_tests.rs` | literal sites |
| `crates/pistol-search/tests/wp18b_solver_path_tests.rs` | literal sites |
| `crates/pistol-search/tests/search_determinism_tests.rs` | `staged_params` call sites |
| the **twelve** `kind = "staged"` documents | each gains `safety_net_top_k = 0` |
| `crates/pistol-engine/tests/common/mod.rs` | the one embedded staged TOML |
| `crates/pistol-engine/tests/{config_validate_tests,config_schema_tests}.rs` | destructures / schema rows |
| `configs/gate_staged_snk_v0.toml` | **NEW** — `gate_staged_v0.toml` + `safety_net_top_k = 8`, the fifth determinism seat. **Its 8 is not §5's calibrated K and does not become it** (REVIEW-design MINOR 10): a determinism seat's job is to exercise the mechanism reproducibly, and any value that makes the cap bind on that fixture does it — MEASURED on that seat's own fixture and configs, **every** safety-net row carries a pool of 33–52, so 8 binds on all of them — the counts are **151 of 151** at one of the seat's two budgets and **153 of 153** at the other, and revision 3's "151 of 153" was those two runs' numerators and denominators crossed (re-review MINOR 9). If §5 selects a different K this document does not change |
| `tools/determinism.sh:65-75` | the fifth seat, reviewed against `tools/SHELL_CHECKLIST.md` per `docs/process.md`'s tools/ coverage rule |
| `crates/pistol-search/tests/wp15d_calibration.rs` | **NEW** — §5's instrument, with its own governing revision named in the artifact it writes |
| `crates/pistol-search/tests/wp15d_bench_counters.rs` | **NEW** — §8's COUNTER leg, calling `Searcher::search` directly and reading `SearchOutcome::info.stages`, because `StageCounters` is not on the line protocol (`info.rs:10-12`) and `tools/bench_block.sh` therefore cannot read it. Its own governing revision is named in the receipt beside the timing legs' |

**`crates/pistol-cli/src/bin/pistol.rs` IS NOT ON THIS LIST, AND ITS ABSENCE IS A
DECISION.** Revision 2 listed it as a forced-compile site; it is not — it
destructures with `..` (`pistol.rs:154-158`) — and adding the key to the
handshake's `candidate_policy` line would change a protocol line **D-356 and
`U2_node_protocol.md` §U2-M item 2 own**, which is an ADR's business and not a
side effect of this WP. The seat a transcript needs is identified by the config
path the handshake already prints and by the config digest the pre-registration
pins. If a later package wants the key on that line it takes its own ADR line.

---

## 5. CALIBRATION — the rule, the direction and the undefined case, all before any number

`sessions/WP-1.9/wp19_design_REVIEW_rev2.md` BLOCKING N1 (a)(b)(c) is this
section's checklist, and each of its three parts is answered by name.

**THE INSTRUMENT.** `crates/pistol-search/tests/wp15d_calibration.rs`, driving
`Searcher::search` directly over `crates/pistol-cli/tests/fixtures/random_openings_v1.txt`
(all 2 000 openings) at **`Stop::Nodes(50_000)`**, `quiet_radius 2`, ordering
heuristics off, one seat per K. **Reproducible by construction (D-22, D-478):
instrument mode refuses `MovetimeMs`, so no cell of this calibration is
wall-clock-derived.** The counters it reads are §4's `StageCounters` fields, and
the dry run verifies they are readable from this harness before the sweep runs —
not assumed.

**THE GRID.** `K ∈ {4, 8, 16, 32, 48, 64}`. The upper end is chosen against the
MEASURED pool: the book's safety-net pool mean is **77.7 cells**, so at K = 64 the
cap barely binds and the benefit must decay toward the incumbent's — the grid
has to contain that decay or the rule below cannot have an interior answer.

**THE CHANNEL, AND ITS DIRECTION.** `lift(K)` = the number of the 2 000 openings
whose **completed** `depth_turns` is ≥ 3. It is a COUNT and **larger is better**.
It is defined for every opening at this budget — MEASURED, the incumbent's
histogram has **zero** openings at completed depth 0 — and §5's undefined clause
below states what happens if that ever stops holding.

**THE SELECTION RULE, stated before the sweep and not movable (D-374):**

> **K is the LARGEST value on the grid whose `lift(K)` is at least 90 % of
> `max_K lift(K)`. Ties break toward the larger K.**

**Why that rule and not "maximise lift".** F10 of the matrix: completed depth is
monotonically improved by narrowing, so a rule that maximises it selects the grid
minimum by construction — the mirror image of the defect that failed WP-1.5c's
re-review. The MEASURED shape here is different and is stated before the rule is
applied: on the book `lift` is nearly FLAT across the low grid (535 / 524 / 514
at K = 4 / 8 / 16) while the cost — in-pool count-two cells excluded — falls
monotonically (79.6 % / 69.3 % / 57.7 %). **So the weakest prune that keeps the
benefit is the right selection, and "largest K within 90 % of the best" is that
rule.** It selects a grid extreme only if the benefit truly does not decay, which
the grid's upper end is chosen to make falsifiable.

**THE UNDEFINED CASE.** `lift` is a count of openings reaching completed
`depth_turns >= 3`, so an opening that completes depth 0 is simply not counted.
**Revision 3 registered a 1 %-of-openings VOID threshold on that case and argued
the store rule could fire it. The code forbids it** (re-review MAJOR 6): under
`Stop::Nodes` the first iteration is not abortable (`search.rs:323`,
`pvs.rs:730`'s *"a non-abortable iteration completes"*, D-74), so no amount of
slowing produces completed depth 0. **The threshold is DELETED**, on §3's own
principle and D-424's: a threshold that cannot fire protects nothing and is prose
a reviewer must still attack. What remains is the reporting, which costs nothing
and can be read: the artifact prints the completed-depth histogram for every
seat, so a depth-0 opening would be visible rather than thresholded.

**THE MATE-TERMINATED SET IS FIXED ACROSS SEATS, WHICH IS THE WHOLE POINT**
(re-review BLOCKING 3). A search that stops short because it PROVED a mate has
not failed to reach depth 3, so those openings are excluded — but revision 3
excluded them PER SEAT, which makes the population move with K and lets a larger
K flatter itself by shrinking its own denominator. D-395, the precedent revision 3
cited, did the opposite: it compared *"19 of the 24 … on BOTH sides"*, a
seat-invariant set. So:

> **`lift(K)` is a COUNT over the FIXED population**
> **`2 000 − |{openings mate-terminated on ANY seat, the incumbent control
> included}|`.** That excluded set is computed once over all seats, printed once
> in the artifact by opening index, and is identical at every K.

`lift` is a count and has no denominator; revision 3's "numerator and
denominator" sentence is deleted with the per-seat rule that produced it.

**THE FLATNESS ANCHOR IS ADVISORY AND SAYS SO** (REVIEW-design MAJOR). The
535 / 524 / 514 figures were measured WITHOUT §6.3's store rule, which withholds
records and therefore changes the tree; and round 3 measured 490 at K = 32, so
the curve is not flat across the whole grid — it is flat across the LOW grid and
decays above it, which is exactly the shape the rule needs and the grid is sized
to expose. The registered calibration re-takes every cell at the implementation
revision, store rule included. **If the re-taken curve is monotone across the
whole grid with no decay, the 90 % rule selects the grid maximum and the design
says so now rather than discovering it then**: that outcome is recorded as a
finding, the grid is NOT extended after the fact (D-374), and K is taken at the
maximum with the monotonicity stated beside it.

**THE COST OF THE SWEEP, on this document's face.** 6 seats × 2 000 openings +
one incumbent control = 14 000 searches at 50 000 nodes. MEASURED anchor: a
2 000-opening seat took **377–430 s** in the matrix's own runs, so **ESTIMATED
44–50 minutes**, single-threaded, one launch and one read.

---

## 6. THE TRANSPOSITION STORE — AND THE RULE THAT MAKES THE SCOPE MEAN WHAT IT SAYS

### 6.1 The hazard, stated exactly

A truncated node stores a `Record` over a SUBSET of its legal candidate set
(`pvs.rs:449-467`) and the probe takes a cutoff on `!is_pv` alone
(`pvs.rs:245-256`). The bound is sound for a later visitor **iff the visitor
searched no more than the storer did**. Two of the three bound kinds fail that
test from a truncated node:

- `Bound::Lower` (`best_score >= beta`): a move IN THE SUBSET reached `beta`, and
  the full set can only do better. **Sound.**
- `Bound::Upper` (`best_score <= original_alpha`) and `Bound::Exact`: both claim
  that NOTHING did better, over a set that was not exhausted. **Unsound.**

### 6.2 Why revision 1's argument did not close it

Revision 1 argued that a key cannot recur at a different ply band, because the
stone count is a function of the key and increases down the tree. That is true
DOWNWARD and it is the wrong direction. The table is kept across the searches of
a game (`search.rs:52-56`), and a node truncated at distance ≥ 1 turn in search
*N* is exactly search *N+1*'s ROOT TURN — where this design promises full width.
`turns_from_root` is a property of the search, not of the position, so nothing
about the key prevents it.

**MEASURED by the DECISION-RED-TEAM, 40 self-play games, one `Searcher` per game
so the table is warm as it is in play**: 41 cutoffs inside the played turn taken
on bounds the capped node never proved, against **0** for the incumbent and **0**
for the deferred T-ROOT row; and the same games with a cold table give **41 → 0**,
which isolates the cause and is why no earlier experiment in this work package
could have seen it — every one of them builds a fresh `Searcher` per call.

### 6.3 THE STORE RULE — `WPQ_seed.md` §7.2's, adopted

> **A node whose emitted set was truncated stores its record only when the bound
> is `Bound::Lower`. A fail-low or an exact score from a set that was not
> exhausted stores nothing at all.**

In code, at the store site, using the `truncated` binding §2.2 already computes:

```rust
let bound = if best_score <= original_alpha {
    Bound::Upper
} else if best_score >= beta {
    Bound::Lower
} else {
    Bound::Exact
};
// A truncated node proved a lower bound and nothing else: the moves it did not
// search are exactly the ones an Upper or Exact bound claims to have refuted.
if !truncated || bound == Bound::Lower {
    self.table.store(key, from_root, Record { .. });
}
```

**What this costs, named rather than hidden**: the lost entries are ordering and
cutoff information a later visit would have reused, so the capped search gets
slower. `WPQ_seed.md` §12.3 calls the lost-entry count a rule-5 measurement and
§8's bench reports it as context beside the ratio.

**What it does NOT claim.** It does not make the capped search's values equal to
the uncapped search's — nothing does; that is what a forward prune is. It makes
the ROOT TURN's search see no bound that a truncated node failed to prove, which
is precisely what D-478 selected this scope for.

### 6.4 The verification, registered here and not left to argument

§7's `no_cutoff_inside_the_played_turn_rests_on_an_unproved_bound` re-takes the
red team's census on the SHIPPED code with a WARM table across a game, and must
read **0**. A cold-table variant is run beside it as the control that the census
can see the class at all, exactly as the red team isolated it.

---

## 7. TESTS, AND THE MUTANT EACH MUST KILL

| test | pins | the mutation it kills |
|---|---|---|
| `the_gate_off_search_is_byte_identical_to_the_pre_change_engine` | `safety_net_top_k = 0` reproduces a sha-pinned expectation fixture, cross-REVISION | the guard's `> 0` deleted |
| `the_cap_admits_exactly_k_cells_on_a_safety_net_row` | emitted width == K at a node one turn from the root with pool > K | — |
| `k_and_k_plus_one_differ_by_exactly_the_next_ranked_cell` | the boundary as a set difference | `truncate(k)` → `truncate(k + 1)` |
| `the_root_turn_emits_the_same_set_with_the_cap_armed` | the emitted set is identical gate-on and gate-off at every node of the root turn | `turns_from_root() > 0` → `>= 0` |
| `at_a_turn_one_root_the_cap_binds_at_ply_one_because_that_ply_is_a_new_turn` | **THE `ply > 1` FALSIFIER** (REVIEW-design BLOCKING 2). From an EMPTY-board root, rule 3 makes turn 1 one stone, so the ply-0 stone COMPLETES the root turn and the ply-1 node is already at `turns_from_root() == 1`; its pool is the radius-2 ball around the origin **minus the occupied origin, so 18 cells** (re-review MINOR 8 — `within_radius` filters by `is_legal_placement`), and the shipped guard truncates it while `ply > 1` leaves it whole. **The assertion is stated in the observable §4 actually provides** (re-review MAJOR 4 — `info.rs`'s counters are whole-search totals, so "the emitted width at that node" is not expressible): at an empty-board root, `depth_turns 2`, `safety_net_top_k = 8`, **`safety_net_capped_rows == 9` shipped and `== 18` under the mutant.** The constant is **9 and not 19** (re-review rev 4, and D-481's derivation): 19 is the predicate's count on the UNCAPPED tree, and capping is what changes the tree — ply 0 is a one-cell safety-net row at `turns_from_root() == 0` and exempt, ply 1 is turn 2 at `turns_from_root() == 1` with pool `19 − 1 = 18` truncated to 8, and its 8 children are capped rows too, so `1 + 8 = 9`; the mutant leaves ply 1 whole and caps its 18 children, `0 + 18 = 18`. **The receipt is the fixture itself** — the test builds the capped tree and reads the counter, so the derivation is embodied rather than asserted. This is the only position at which the two spellings differ at all | `turns_from_root() > 0` → `ply > 1` |
| `the_cap_never_fires_off_a_safety_net_row` | a batched row with Tier T non-empty emits its full Tier T with the cap armed | the `used_quiet_safety_net` guard deleted |
| `a_truncated_fail_low_stores_no_transposition_record` | `safety_net_upper_withheld > 0` | also storing a truncated node's `Upper` |
| `a_truncated_exact_score_stores_no_transposition_record` | `safety_net_exact_withheld > 0` | also storing a truncated node's `Exact` |
| `a_truncated_fail_high_still_stores_its_lower_bound` | withheld < truncated rows: the sound half is not thrown away with the unsound half | withholding every bound |
| `the_store_rule_fires_across_the_searches_of_one_warm_game` | **§6.4's class INSTANTIATED**: one `Searcher` across six searches of a game, so the table is warm as it is in play, and the rule fires there. **It does NOT take §6.4's cutoff census** — that needs provenance on `Record` which the shipped type does not carry, and the shortfall is recorded here rather than papered over | the store rule deleted |
| `the_tie_break_is_stable_across_runs` | two runs agree cell for cell | `sort_by_key` → `sort_unstable_by_key` |
| `tools/determinism.sh` fifth seat | cross-process, gate-on | any of the above |

**Three of these are the dispatch's named mutants** — cap-boundary off-by-one,
tie-break order dependence, tier-exclusion violation — and each has a test that
dies at a named case rather than at a suite. **Three more are this WP's own
review history made executable**: the turn-1 root case, the truncated-store case,
and the warm-table census, each of which is a defect a review found in a document
that had argued it away.

---

## 8. THE RULE-5 BENCH, REGISTERED BEFORE MEASURING

**HOTSPOT.** The additional INTERIOR nodes the cap creates: capping the branching
factor below the root turn deepens the tree at a fixed node budget, so the ratio
of ball-paying safety-net nodes to cheap leaves rises. MEASURED on the book:
1.131× wall at fixed nodes at K = 8 before the store rule. **This is a cost
channel, not a gain channel**, and it is registered as one.

**THE SEATS, NAMED** (REVIEW-design MAJOR: revision 2 named none, so its radius
was undetermined and its spread claim unreadable). Baseline
`configs/instrument_staged_v0.toml` **as committed** — `quiet_radius = 2`,
`safety_net_top_k = 0` — digest recorded at the run. Candidate: **the same
document with `safety_net_top_k` set to §5's calibrated K and nothing else**,
digest recorded, **not committed to the tree** (rule 8: an uncommitted bench
variant is not an artifact). This is D-395's and D-398's own two-seat shape.

**TWO INSTRUMENTS, AND WHICH LEG EACH ONE CARRIES** (re-review BLOCKING 1:
revision 3 registered a per-position counter and named only an instrument that
cannot read it — `StageCounters` is not on the line protocol, by
`info.rs:10-12`'s own design). **The timing legs are
`tools/bench_block.sh`'s; the counter leg is a harness in the
`crates/pistol-search/` test tree, `crates/pistol-search/tests/wp15d_bench_counters.rs`,
calling `Searcher::search` directly and reading `SearchOutcome::info.stages` —
the same shape §5's `wp15d_calibration.rs` takes and the shape `info.rs:10-12`
points at.** Both are named in the receipt with their governing revisions, and a
number from one is never quoted beside a number from the other without saying
which produced it (D-479).

**THE TIMING INSTRUMENT, AND WHAT IT DOES AND DOES NOT DO.**
`tools/bench_block.sh` at its committed revision, over
`crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` (`--grammar tail`) and
`crates/pistol-cli/tests/fixtures/spread_v1.txt` (`--grammar line`),
`--budget 'nodes 50000'`, `--reps 5`, one invocation per seat. **The script emits
one record line per (entry, rep) and aggregates nothing** — its own header says
medians, IQR gating and ratios are the caller's — so the arithmetic is registered
here and is not the script's: per position, the MEDIAN of its five `time` values;
per band, `Σ nodes / Σ median time` for nps and `Σ median time` for
time-to-depth; the IQR gate is per position at 10 % of that position's median
(D-215/D-362). The receipt carries the raw record lines AND the derived table, so
the derivation is checkable rather than asserted.

**BRACKET AND DIRECTION, in the house convention (D-388, D-395, D-398).**
Time-to-depth ratio is **ON/OFF and LARGER IS WORSE**; nps ratio is ON/OFF,
larger is better, reported as context and never as the gate (D-374: across seats
with different candidate policies nps is not a like-for-like unit).

- **Corpus, `bench_positions_v1`:** ttd ratio **≤ 1.10**; **ABORT if > 1.25**.
- **Spread, `spread_v1`:** REPORTED, NOT GATED.

**THE CORPUS EXPECTATION, WITH EVERY TERM FROM ONE ARTIFACT AND ONE RUN**
(re-review BLOCKING 2: revision 3 quoted an incumbent of 4 825 that lives in
`docs/experiments/matrix_M2.md:236` and belongs to a DIFFERENT run, listed a
`scope1` seat that is not the selected row, and derived three ratios below 1.000
— contradicting this section's own cost-channel hotspot. **The wrong baseline
flattered the change**, which is why D-479 exists).

Every cell below is from `artifacts/wp15d_turn_axis_v1.txt`'s `S2/CORPUS` block,
`Stop::Nodes(50_000)`, `quiet_radius 2`, one run:

| seat | Σ nodes | Σ wall ms | ratio to that block's own incumbent |
|---|---|---|---|
| incumbent, `scope0/K0` | 1 104 026 | **4 776** | 1.000 |
| `scope4/K4` | 1 104 026 | 4 828 | **1.011** |
| `scope4/K8` | 1 104 026 | 4 795 | **1.004** |
| `scope4/K16` | 1 104 026 | 4 790 | **1.003** |

**All three are at or above 1.000, which AGREES with this section's registered
hotspot** — the cap is a cost channel on the corpus, a small one. The block's
fourth seat is `scope1/K8`, a different row entirely (the rejected `ply > 0`
scope), and it is not quoted here.

What that block records, exactly: **Σ nodes identical on every seat**,
**per-position completed depths identical seat for seat** (the `depths=` field,
24 values), and the Σ wall figures above. **Per-position node counts are NOT
recorded and no claim rests on them.** The ratios are derived here from those
sums and are marked as derived.

**THE SPREAD FIXTURE — ONE RADIUS, ONE BUDGET, ONE HARNESS, AND THE TWO CLAIMS
KEPT APART.** Revision 2 registered "expected exactly inert — MEASURED 0 prune
events", which was a `movetime` cell governing a `Stop::Nodes` bench — forbidden
by D-478 and by `artifacts/wp15d_turn_axis_v1.txt`'s own header. Revision 3 fixed
the budget and imported the replacement numbers from the revision-2 reviewer's
throwaway probe, with no instrument, no artifact, no digest and no K, at two
radii one of which has no config in the tree (re-review MAJOR 5); and it quoted
an r = 3 depth vector under the r = 2 seat it had just named (re-review MAJOR 7).
Both are corrected by **dropping the r = 3 cells entirely and keeping only the
radius the named seats actually use**:

1. **The cap FIRES on this fixture at this budget, and the count is the ON seat's
   own.** No inertness is expected or registered. `wp15d_bench_counters.rs`
   reports `safety_net_capped_rows` per position on the ON seat's own tree — not
   inferred from the OFF seat's, which is what the imported anchor was. The
   ADVISORY anchor for what to expect, and it is labelled as one: measured on the
   UNCAPPED tree at `configs/instrument_staged_v0.toml` (`quiet_radius = 2`),
   `go nodes 50000`, the guard's predicate `turns_from_root() > 0 && pool > K` is
   satisfied **95 / 5 / 152 / 0** times at 11 / 21 / 51 / 99 stones, K-invariant
   over K ∈ {4, 8, 16}. That figure governs nothing; the registered number is the
   ON seat's counter.
2. **The cap does not restore completed depth on the D-95 class**, which is the
   claim §9 and the preamble rest on. At the r = 2 seat §8 names, the incumbent's
   completed-depth vector on `spread_v1` at `movetime 500` is **1 / 1 / 0 / 0**
   (`artifacts/wp15d_turn_axis_v1.txt`, `S1/r2/nocap`) — **not the 1 / 0 / 0 / 0
   revisions 2 and 3 quoted, which is the r = 3 reading.** It is a SCOPING
   measurement at the budget the debt is defined in (D-22, D-478) and it governs
   no threshold here; it is why the fixture is reported and not gated.

**THE STORE RULE'S OWN COST IS REPORTED, NOT BRACKETED** (§6.3,
`WPQ_seed.md` §12.3): `safety_net_stores_withheld` and the resulting change in
`hashfull` are printed beside the ratio as context. It is a cost this design pays
for correctness and there is no threshold at which it would be traded away, so a
bound on it would be a threshold that can never fire — prose a reviewer must
still attack (D-424).

**THE BOOK'S 1.131× IS NOT IN THIS BRACKET, and that is deliberate**: the book is
not a bench fixture, and the honest confirmation of a whole-game cost is the
SPRT report's own per-side compute (rule 6), registered with the
pre-registration rather than here.

---

## 9. WHAT THIS DESIGN DOES NOT SETTLE

- **K's value.** §5's rule decides it from a sweep taken at the implementation
  revision. No value is chosen here.
- **The pre-registration.** Its own document, its own fresh-context review at the
  revision that governs the run, fresh `openings_skip` slice with consumed ranges
  receipted, warm-replay Criterion 1'', second-instrument agreement criterion,
  and the slot pass (D-427).
- **The D-95 depth debt.** OPEN, re-pointed by D-478 at a package of its own.
  **MEASURED untouched by this one in the channel the debt is defined in** —
  completed depth at `movetime 500` on `spread_v1` is **1 / 1 / 0 / 0 at the
  r = 2 seat §8 names** (`S1/r2/nocap`; the 1 / 0 / 0 / 0 revisions 2 and 3 quoted
  is the r = 3 reading — re-review MAJOR 7), with the cap armed and without it. That is a scoping measurement (D-22, D-478) and it
  governs nothing here; §8 states what the cap does do on that fixture, which is
  fire hundreds of times without moving the channel the debt is about.
