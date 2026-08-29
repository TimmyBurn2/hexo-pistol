# WP-1.5d SESSION (A) — REVIEW-impl

**REVISION UNDER REVIEW:** `5279cae47e8292381d9e893787c287b1893f65f7`.
**HEAD MATCH:** yes. `git rev-parse HEAD` → `5279cae47e8292381d9e893787c287b1893f65f7`;
`git status --short` → empty. The revision under review IS `dev`'s tip and the
tree is clean.

**IMPLEMENTATION RANGE READ:** `git diff bd1faf4..5279cae` (34 files,
+1241/−18), commit by commit (`186c626`, `5845b61`, `e4bb5bf`, `5896579`,
`fc02655`, `5279cae`).

**GOVERNING DOCUMENTS READ IN FULL:** `docs/experiments/wp15d_design.md`
(revision 5 — §1, §2, §3, §4, §6, §7 only; §5 and §8 are DEFERRED per D-482 and
are not reviewed), `CLAUDE.md`, `docs/decisions.md` D-473–D-483 plus D-5, D-7,
D-9, D-22, D-74, D-88, D-353, D-356, D-374, D-424, `docs/experiments/WPQ_seed.md`
§7.2, `tools/SHELL_CHECKLIST.md`, and the review history
(`wp15d_design_REVIEW.md`, `_rev3.md`, `_rev4.md`).

**CODE READ:** `crates/pistol-search/src/{pvs.rs, info.rs, params.rs,
staged.rs, quiescence.rs, search.rs, score.rs, tt/mod.rs}`,
`crates/pistol-engine/src/{config.rs, validate.rs, instance.rs}`,
`crates/pistol-search/tests/wp15d_safety_net_cap_tests.rs`,
`tools/determinism.sh`, `tools/file_justification_check.sh`, all thirteen
`kind = "staged"` documents.

**WORKTREE.** `/home/tom/Projects/pistol-wt-ri` (detached at `5279cae`, created
with `git worktree add --detach`, on `/home` and not `/tmp`), with
`CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-ri-target`. **NOT REMOVED** per
D-469. It holds no uncommitted change — every mutation was reverted with
`git checkout -- .` and both scratch probe files were deleted;
`git -C /home/tom/Projects/pistol-wt-ri status --short` is empty. The live tree
was never edited; the only file this review writes is this one.

**BUILT AND RAN**

| what | result |
|---|---|
| `cargo test -p pistol-search --locked` (worktree) | ok — including all 8 of `wp15d_safety_net_cap_tests` |
| `cargo test --workspace --locked` (worktree) | one failure, **RUN VOID of my instrument, not a finding** — see NOT REPRODUCED 1 |
| `env -u CARGO_TARGET_DIR tools/solver_determinism.sh` | exit 0, `solver_determinism: PASS — 61 cases, byte-identical transcripts` |
| `tools/determinism.sh` (worktree) | exit 0, `determinism: ok — 5 seat(s), no difference outside nps/time in any of them`, 148 s wall |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D clippy::all` | clean |
| `tools/file_justification_check.sh` | **exit 1** — see BLOCKING 1 |
| 6 receipt mutations re-taken + 5 of my own | see below |

---

# VERDICT: **FAIL**

Two BLOCKING findings, **both requirement findings, neither a correctness
finding**. I attacked the mechanism hard and did not find a way to make it
produce a wrong answer: the guard names exactly the intended node set at every
turn number I could construct, and `Bound::Lower` from a truncated node is
genuinely sound on every path I could trace. The package does **not** go back to
the architect. What fails is a red CI gate and one registered verification that
was never written.

---

# FINDINGS

## BLOCKING 1 — `tools/ci.sh` gate 17/19 is RED: the new test file is over rule 9's cap with no registry entry. **Requirement finding.**

`crates/pistol-search/tests/wp15d_safety_net_cap_tests.rs` is **307 lines**.
`tools/file_justification_check.sh:65` sets `SOFT_CAP=300` and `:115` skips only
files at or under it. `docs/rule9_justifications.md` has no entry for the path —
`git diff bd1faf4..5279cae -- docs/rule9_justifications.md` prints nothing.

**REPRODUCED.**

```
$ cd /home/tom/Projects/pistol-wt-ri && bash tools/file_justification_check.sh; echo "exit=$?"
...
file_justification_check: crates/pistol-search/tests/wp15d_safety_net_cap_tests.rs: over the cap with no entry in docs/rule9_justifications.md
file_justification_check: FAIL: 1 finding(s) against CLAUDE.md rule 9's soft cap
exit=1
```

`tools/ci.sh:176-177` runs this as `gate 17/19: file-justification check`, so
`tools/ci.sh` cannot pass at this revision. CLAUDE.md rule 9 is explicit that
the why lives in `docs/rule9_justifications.md` and that it "never states a line
count".

**The fix is one line of registry or one line out of the file** — the eight
tests plainly are one seat (one `searcher` helper, one empty-board root, one
spread position) and a justification is easy to write honestly. I flag it
BLOCKING only because a red gate is a red gate and the closure rule requires a
gate claim cited from the gate's own log.

## BLOCKING 2 — §6.4's registered verification does not exist, and NO shipped test exercises the warm-table class the store rule was written for. **Requirement finding.**

Design §6.4 (`wp15d_design.md:459-466`), a section that governs (A):

> §7's `no_cutoff_inside_the_played_turn_rests_on_an_unproved_bound` re-takes the
> red team's census on the SHIPPED code with a WARM table across a game, and must
> read **0**. A cold-table variant is run beside it as the control that the census
> can see the class at all, exactly as the red team isolated it.

and §7's table row (`wp15d_design.md:480`) registers it against the mutants "the
store rule deleted; the guard widened to `ply > 1`".

**REPRODUCED — it is absent.**

```
$ git grep -n "no_cutoff_inside_the_played_turn\|unproved_bound" -- . | LC_ALL=C sort
crates/pistol-search/tests/wp15d_safety_net_cap_tests.rs:200:fn a_truncated_node_withholds_its_unproved_bounds_and_keeps_its_proved_one() {
docs/experiments/wp15d_design.md:461:...
docs/experiments/wp15d_design.md:480:...
docs/experiments/wp15d_design_REVIEW.md:216:...
```

Only the design and a prior review name it. Nothing in `crates/` does.

**The attack, and why this is more than a missing row.** §6.2 is the whole
reason §6.3 exists, and it rests on one measured fact: the table outlives the
search, so a node truncated at distance ≥ 1 turn in search *N* is search *N+1*'s
root turn. The red team measured that class at **41 cutoffs warm against 0
cold**, and §6.2 says in terms *why no earlier experiment could see it*: "every
one of them builds a fresh `Searcher` per call."

**Every one of the eight shipped tests builds a fresh `Searcher` per call.**
`crates/pistol-search/tests/wp15d_safety_net_cap_tests.rs:10-32` is a `searcher()`
factory, and each of `:47`, `:76`, `:95`, `:127`, `:158`, `:181`, `:201` calls it
anew for a single search. The suite therefore reproduces, exactly, the blind spot
§6.2 names — the one condition under which the hazard is invisible. The store
rule ships with its motivating defect class never once instantiated.

This is a requirement finding, not a correctness finding: I traced the class by
hand (see WHAT I CHECKED AND FOUND SOUND, §3) and the shipped rule holds. But
§6.4's own sentence is *"registered here and not left to argument"*, and it has
been left to argument.

## MAJOR 1 — the two halves of §6.3's store rule are each individually unpinned: two non-equivalent mutants of the condition survive the whole suite.

`crates/pistol-search/src/pvs.rs:489`:

```rust
            if truncated && bound != Bound::Lower {
```

§6.1 names **both** `Bound::Upper` and `Bound::Exact` unsound from a truncated
node. The single shipped test of the rule
(`wp15d_safety_net_cap_tests.rs:200-220`) asserts only
`0 < safety_net_stores_withheld < safety_net_capped_rows`. That criterion cannot
see *which* bound kinds are withheld.

**REPRODUCED.** Two mutants, each of which stores one of the two bound kinds
§6.1 calls unsound, and each of which changes the search measurably:

```
$ mut.sh M9  pvs.rs "if truncated && bound != Bound::Lower {" "if truncated && bound == Bound::Upper {"
MUTANT M9: SURVIVED
$ mut.sh M10 pvs.rs "if truncated && bound != Bound::Lower {" "if truncated && bound == Bound::Exact {"
MUTANT M10: SURVIVED
```

They are **not** equivalent mutants — the tree itself moves (probe run on an
empty-board root, `depth_turns 3`, K = 8, `quiet_radius 2`):

| variant | `capped_rows` | `stores_withheld` |
|---|---|---|
| shipped | 102 | 51 |
| M9 (truncated `Exact` stored) | 97 | 37 |
| M10 (truncated `Upper` stored) | 90 | 9 |

The design's *own* two registered mutants for this rule do die — I re-took both
(M4 `→ if false`, M5 `→ if truncated`; both DIED at
`a_truncated_node_withholds_...`). So the letter of §7 is met. What is not met is
`docs/process.md`'s vacuous-criterion standard applied to the mechanism's central
soundness rule: the design registered **two separate tests**,
`a_truncated_fail_low_stores_no_transposition_record` and
`a_truncated_fail_high_still_stores_its_lower_bound`, and the implementation
merged them into one aggregate inequality. The merge is exactly what lost the
discrimination. Two tests that name a bound kind — assert the table holds nothing
for a truncated fail-low key, and holds a `Lower` record for a truncated
fail-high key — kill M9 and M10 and cost nothing.

## MAJOR 2 — three of §7's eleven registered rows are absent, and the byte-identity claim has no substitute for the STAGED path.

§7 registers eleven rows. Eight tests ship. Missing by name:

1. `the_gate_off_search_is_byte_identical_to_the_pre_change_engine` — registered
   as reproducing "a sha-pinned expectation fixture, cross-REVISION".
2. `the_cap_admits_exactly_k_cells_on_a_safety_net_row`.
3. `no_cutoff_inside_the_played_turn_rests_on_an_unproved_bound` (BLOCKING 2).

Row 2 is genuinely subsumed —
`at_a_turn_one_root_...:60-68` asserts `safety_net_emitted_cells == 9 * 8` with
`pool_cells > emitted_cells`, which is "emitted width == K at a node one turn
from the root with pool > K" summed. No finding there.

Row 1 is **not** subsumed, and the natural assumption that it is, is wrong. The
project's cross-revision byte-identity gate is
`crates/pistol-cli/tests/instrument_golden_tests.rs` over
`crates/pistol-cli/tests/fixtures/instrument_golden_v1.txt` — and that fixture's
own header (`:6-11`) says it was generated **`at configs/gate_v0.toml`**, the
**RADIUS** policy. It pins nothing about the staged path this WP edits. The
substitute that shipped, `the_off_value_truncates_nothing_and_records_nothing`
(`:74-88`), asserts the four new counters read 0 at K = 0; it kills the mutant
§7 assigned to row 1 (I re-took it as M7 below), but it is not a cross-revision
byte-identity claim and the design's cell should not be read as closed.

Mitigating, and I say so plainly: the gate-off path is byte-identical **by
inspection** — with `safety_net_top_k = 0` the guard's first conjunct is false at
every node, `truncated` is never set, and `if truncated && …` always takes the
`else` arm, which is the pre-change `store` call verbatim. The risk here is
bookkeeping, not behaviour.

## MINOR 1 — `info.rs`: the new recorder was inserted between an existing function's doc comment and its body, so the doc now describes the wrong function.

`crates/pistol-search/src/info.rs:107-121`:

```rust
    /// Record that a BATCHED or BATCHED-lost row just recorded used the
    /// quiet-ball safety net (`StagedSet::used_quiet_safety_net`) rather than
    /// a non-empty Tier T. Called separately from [`StageCounters::record`]
    /// because the safety net is this D-scope's own IMPL choice and not a
    /// `StagedRow` of the node protocol itself (`crate::staged`'s doc).
    /// One safety-net row the cap truncated, with the widths either side of it.
    pub(crate) fn record_safety_net_cap(&mut self, pool: u64, emitted: u64) {
        ...
    }

    pub(crate) fn record_quiet_safety_net(&mut self) {
```

`record_safety_net_cap` now carries five lines that describe
`record_quiet_safety_net`, and `record_quiet_safety_net` carries none. Not a
style dispute rustfmt can settle: it is a false statement about the item it sits
on. Two lines to fix.

## MINOR 2 — `configs/gate_staged_snk_v0.toml`'s copied header states a role the document does not have.

`configs/gate_staged_snk_v0.toml:1-2`:

```
# pistol — the staged configuration THE COLONY FAMILY and TACTICAL SUITE
# UNDER STAGED gates run their gate_v0-class cases at.
```

**REPRODUCED — nothing but the determinism gate reads it:**

```
$ git grep -n "gate_staged_snk" -- . | LC_ALL=C sort
docs/experiments/wp15d_design.md:290:...
docs/experiments/wp15d_design_REVIEW*.md:...
tools/determinism.sh:78:	"staged-safety-net-cap configs/gate_staged_snk_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"
```

The file's own later comment contradicts its header — *"it is a DETERMINISM SEAT
and nothing else"* — which is the accurate sentence. On the dispatch's direct
question, **the copy itself is faithful**: `diff configs/gate_staged_v0.toml
configs/gate_staged_snk_v0.toml` shows exactly one hunk, the `safety_net_top_k`
comment block and its value `0` → `8`, and nothing else.

## MINOR 3 — `tools/determinism.sh`'s header now says FOUR SEATS and the gate runs five.

`tools/determinism.sh:19-20` still reads `# FOUR SEATS, RADIUS, STAGED,
STAGED-WITH-HEURISTICS AND / # STAGED-WITH-SOLVER`, and `:45` still reads
`THREE SEATS` (that one was already stale before this WP). The summary line at
`:287` uses `${#SEATS[@]}` and printed `5 seat(s)` correctly. Two counts asserted
in comments beside one derived correctly — the same shape rule 9 forbids for line
counts.

## MINOR 4 — `params.rs`'s doc says "the root turn's two", which is false at exactly the case the guard exists for.

`crates/pistol-search/src/params.rs:65-66`: *"at every node but the root turn's
two"*. Game rule 3 gives turn 1 **one** stone, and rule 4 truncates a turn whose
first stone wins — the two cases §1 spends a paragraph on and
`at_a_turn_one_root_...` exists to pin. The doc's next paragraph states the rule
correctly, so this is one phrase, not a misunderstanding.

## MINOR 5 — design §3 and §4 contradict each other about a validation check, and the code follows §3.

§4's table row for `crates/pistol-search/src/search.rs` says *"the SAME check
again … §3 names the check it adds"*, while §3 says **"THERE IS NO VALIDATION
CHECK, AND THAT IS THE ANSWER RATHER THAN AN OMISSION."** The implementation
follows §3: `search.rs` is untouched and `validate.rs:84-87` destructures the
field to `_` with a comment saying why. §4 also lists
`crates/pistol-search/tests/search_determinism_tests.rs` and
`crates/pistol-engine/tests/config_schema_tests.rs` as change sites; neither
needed one (`common::staged_params`'s signature is unchanged, and
`config_schema_tests.rs` names no policy field —
`git grep -n quiet_top_k crates/pistol-engine/tests/config_schema_tests.rs`
returns nothing).

**On §3's substantive claim** — that "the type is the domain" and no validator is
owed — I attacked it and agree for this target: `0` is the off-value, every other
`u64` is a legal ceiling, a K above every pool is a no-op that spells OFF a second
way, and `deny_unknown_fields` plus serde's typing refuse everything else by name.
Rule 1 asks for completeness and no code-side default; both hold. Rule 3 asks for
a named error on wrong-kind input; serde gives it.

## MINOR 6 — `safety_net_top_k as usize` narrows on a 32-bit target, where the narrowing is a panic and not a no-op.

`crates/pistol-search/src/pvs.rs:339-348`:

```rust
                let cap = params.safety_net_top_k as usize;
                if params.safety_net_top_k > 0
                    && self.turns_from_root() > 0
                    && set.used_quiet_safety_net
                    && set.cells.len() > cap
                {
```

On a 32-bit `usize`, `K = 2^32` gives `cap == 0` while `safety_net_top_k > 0`
holds and `set.cells.len() > 0` holds, so `set.cells.truncate(0)` empties the set
and `let mut best_cell = cells[0];` (`pvs.rs:395`) panics on an empty vector.
§3 deleted revision 3's representability check on the ground that it *"on this
target can never fail"*, and that is true of the one workstation CLAUDE.md's
Design point names — but CLAUDE.md's Environment section pins **no toolchain file
and no target**. **NOT REPRODUCED** on this machine and unreproducible on it:
`usize == u64` here, so `cap == K > 0` always. Recorded because the design made
the target an explicit premise and nothing in the repository pins it.

---

# NOT REPRODUCED

## NOT REPRODUCED 1 — a `cargo test --workspace` failure that is a RUN VOID of my instrument, not a defect.

My first full-workspace run failed:

```
---- the_shipped_solver_determinism_script_passes_and_says_so stdout ----
the shipped script must exit 0; stderr:
   ...
solver_determinism: RUN VOID: no binary at target/release/solver-selftest after a green build
test result: FAILED. 0 passed; 1 failed
```

This is `SHELL_CHECKLIST` item 12's own distinction, and the void was mine: I had
exported `CARGO_TARGET_DIR` around `cargo test`, which is precisely what
CLAUDE.md's Environment section forbids, so the script's `target/release/`
lookup pointed at a directory cargo never wrote. Re-taken cleanly:

```
$ env -u CARGO_TARGET_DIR bash tools/solver_determinism.sh; echo "exit=$?"
solver_determinism: PASS — 61 cases, byte-identical transcripts
exit=0
```

Not a finding against `5279cae`.

## NOT REPRODUCED 2 — §2.3's truncate-before-promote ordering has no test, but I could not construct a seat where reversing it changes anything.

§2.3 registers the ordering as buying a property: the emitted set is a function
of (position, `quiet_radius`, K, `turns_from_root`) "and of nothing else — no
transposition contents and no search history". Nothing in the suite pins it, so
I mutated it — moved `set.promote_table_move(table_move)` (`pvs.rs:353`) to
before the cap block:

```
--- M8 (promote before truncate) ---
suite: 8 passed; 0 failed
SEARCH K=8 d2: capped_rows=9 pool_cells=221 emitted_cells=72 withheld=9   (identical to shipped)
SEARCH K=8 d3: capped_rows=102 withheld=51                                 (identical to shipped)
SPREAD d2: best=Pair((6,2),(7,1)) nodes=33527 capped=4411 withheld=6 score=-56   (identical)
SPREAD d3: best=Pair((9,1),(11,0)) nodes=947842 capped=9810 withheld=561 score=892 (identical)
```

Behaviour-identical on both seats at two depths, so I cannot call the missing
test a defect. The likely reason is worth recording for (B): §6.3's store rule
withholds most truncated nodes' records, so a truncated node usually has no table
move to promote at all — the store rule has largely absorbed the property §2.3
was protecting. **Not reproduced; recorded as an observation.**

---

# WHAT I CHECKED AND FOUND SOUND

## 1. The guard names exactly the intended set, at every turn number.

`crates/pistol-search/src/pvs.rs:339-349`. Placement is §2.2's, verbatim in
effect: after the empty-set check (`:325-328`) and **before**
`set.promote_table_move` (`:353`), which is §2.3's ordering.

- **Root turn, ordinary (2 stones).** `Run::turns_from_root()` is
  `state().turn() - root_turn` (`pvs.rs:560-562`) and `check_root`
  (`search.rs:463+`) refuses a root at `phase() != Phase::First`, so ply 0 and
  ply 1 both sit at `turns_from_root() == 0` and are exempt. Ply 2 opens turn
  *r+1* → `== 1` → capped.
- **Turn-1 root (1 stone).** Ply 0 completes the turn, so ply 1 is already
  `== 1` and IS capped — the case no `ply` threshold expresses. Verified by
  running, §2 below.
- **Rule-4-truncated turn.** `PlyOutcome::Win` returns `mate_in(...)` at
  `pvs.rs:405-415` without descending, so no node exists to classify; the
  guard is not reached and cannot be wrong there.
- **`cap >= 1` whenever the guard fires**, because `safety_net_top_k > 0` is the
  first conjunct — so `set.cells` is never emptied and `cells[0]` (`:395`) is
  safe.
- **Nothing else reads `set.cells` in a way truncation invalidates.**
  `promote_table_move` (`staged.rs:86-96`) only *rotates within* `cells[forced..]`
  and **never inserts** — I read it specifically for the re-admission hazard; it
  cannot grow the set, so §2.3's "can never re-admit a cut cell" holds and
  `emitted == cap` is exact. `root_restrict` (`pvs.rs:355-370`) is gated on
  `ply == 0`, where `turns_from_root() == 0`, so it and the cap can never
  co-occur. `set.forced` is `0` on this row by construction
  (`staged.rs:213`, `:235`), so the truncation cannot cut a Tier-F cell and
  `forced_bound`/`best_index >= forced` are unaffected. The ordering heuristics
  reorder without resizing.
- **Quiescence is correctly untouched.** `quiescence.rs:26-27` states in terms
  that it does not fall through to "`staged_candidates`'s quiet ball,
  `within_radius`", so no safety-net row exists inside quiescence and the cap has
  nothing to do there. Not a scope gap.

## 2. The constant **9** is right, verified two independent ways.

**By running** (worktree, `cargo test -p pistol-search --test wp15d_safety_net_cap_tests`):
all 8 tests pass, `at_a_turn_one_root_...` included.

**By derivation**, with a scratch probe that drives `staged_candidates` directly
rather than reading the assertion under test (probe since deleted):

```
ply0 (empty board):                     row=Batched safety_net=true n=1
ply1 (turn 2, one stone on board):      row=Batched safety_net=true n=18
ply2 after -2,0 … 0,-2 (the 8 kept):    n = 27 26 27 26 22 22 26 27   (all > 8)
children whose pool exceeds K=8: 8
SEARCH K=8 d2: capped_rows=9 pool_cells=221 emitted_cells=72 withheld=9 sn_rows=11
derived = 1 + 8 = 9
SEARCH K=0 d2: sn_rows=21
```

Three independent cross-checks fall out of that: `1 + 8 = 9`;
`emitted_cells = 72 = 9 × 8`; and `pool_cells = 221 = 18 + (27+26+27+26+22+22+26+27)`
**exactly**, which confirms both that the cap keeps precisely those eight cells
and that each child node is visited once — no PVS re-search inflates the counter
at this seat. D-481's "19" reconciles too: the uncapped tree's 21 safety-net rows
are its `1 + 18` plus the ply-0 row of each of the two deepening iterations.

## 3. `Bound::Lower` from a truncated node is sound — traced, not assumed.

The dispatch calls this the finding that stopped an earlier round. It has not
come back. The trace:

- **Direction.** Truncation removes moves at a node, so the node's value over the
  subset is ≤ its value over the full set. `best_score >= beta` therefore implies
  the full-width value is `>= beta`. The probe's cutoff
  (`pvs.rs:245-256`) returns `record.score` under `Bound::Lower => record.score >= beta`,
  which is a fail-soft lower bound used as a fail-high. Sound.
- **The cross-search case §6.2 raises.** Within one search, stone count fixes
  distance-from-root, so a key cannot recur at two `turns_from_root` values.
  Across searches it can, and that is the hazard. The compositional check: a node
  X's descendants beyond its own turn are capped in *every* search (they are at
  `turns_from_root >= 1` from any root), so the capping environment **below** X
  is identical in search *N* and *N+1*. Only X's own set differs — capped at
  distance ≥ 1, full inside the root turn — which is exactly the subset-vs-full
  comparison above. Stored ≤ intended. Sound.
- **Which root-turn nodes can even take a table cutoff.** The root call is
  `self.visit(depth_plies, -INFINITY, INFINITY, 0)` (`pvs.rs:153`), so `is_pv` is
  true at ply 0 and the probe's `!is_pv` conjunct blocks a root cutoff outright.
  The only root-turn node that can cut from the table is a ply-1 node, and its
  children are at `turns_from_root == 1` — capped in every search. The "untruncated
  ancestor stores `Exact` over truncated children" attack therefore has no
  landing site inside the root turn.
- **Mate re-basing.** `Table::store`/`probe` (`tt/mod.rs:149`, `:129`) apply
  `score::to_table`/`from_table`, which shift only outside `MATE_THRESHOLD`
  (`score.rs:87-106`). Node-relative storage is unchanged by this WP and the
  subset-vs-full direction argument is independent of magnitude.
- **`Run::salvage` / `root_score` / `Provenance::PartialRoot`.** `salvage`
  (`pvs.rs:174-190`) reads a ply-0 promotion, which happens only on a completed
  child subtree; the score it reports is the capped-game value, which is what a
  forward prune means and what §6.3 says it does not claim to fix. `search.rs:457`
  overwrites `outcome.info.stages = run.stages` on **every** arm, so the
  `PartialRoot` and `Fallback` arms (which build `StageCounters::default()` at
  `:397` and `:432`) do not lose the new counters — see §4.
- **Quiescence.** Stores with `from_quiescence: true` and `depth_plies: 1`
  (`quiescence.rs:122-133`) from an uncapped, threat-only set; no truncated node
  contributes a bound there.

## 4. The counters are whole-search totals and survive every answer path.

`safety_net_capped_rows`, `_pool_cells`, `_emitted_cells` are written once per
truncating node at `pvs.rs:346-347`, before the truncate; `_stores_withheld` at
`:490`, on the one path that declines a store. `Run` is constructed once per
`Searcher::search`, so the totals span iterative deepening exactly as `nodes`
does, matching `info.rs`'s own doc. Salvage and fallback preserve them via the
unconditional overwrite at `search.rs:457` (whose neighbouring comment is
REVIEW-impl W-1's, written for the identical class on `solver_nodes`). Aborted
work counts, as it does elsewhere. No path records a truncation that did not
happen: the counter call sits inside the same `if` as `set.cells.truncate`.

One semantic note, not a finding: `_emitted_cells` records `cap`, not
`set.cells.len()` after the truncate. For shipped code these are equal (nothing
between the call and the store resizes the set — see §1), so the doc line "the
cells those rows emitted after truncation" is true; it is only under mutation
that the counter would stop tracking reality, and the mutation that exploits
that (`truncate(cap + 1)`) is caught by `capped_rows` in the same test.

## 5. The config surface is complete, and the fifth determinism seat is NOT vacuous.

- **Twelve committed documents all carry `safety_net_top_k = 0`**: verified file
  by file in the diff — `bench_wp18c_solver_{off,on}`, `gate_staged_{heuristics,solver,}_v0`,
  `instrument_staged_{h,q_defensive_and_offensive,q_defensive_only,}_v0`,
  `instrument_v0`, `play_staged_v0`, `tactical_staged_v0`. Plus the embedded TOML
  at `crates/pistol-engine/tests/common/mod.rs:64`.
- **The thirteenth is the new seat at `8`**, and the copy is faithful (MINOR 2).
- `config.rs:225-231` adds the field with a doc; `validate.rs:84-87` destructures
  it (the no-`..` destructure would not compile otherwise);
  `instance.rs:202-205` passes it through; `params.rs:64-71` carries it;
  `config_validate_tests.rs:204` asserts it is `0` in the committed document.
  `tools/config_check.sh:43` globs `configs/**/*.toml`, so the new document is
  validated with no gate edit.
- **The seat exercises the mechanism.** I replayed all twenty positions of
  `tactical_staged_v0.txt` under the snk parameters (`quiet_radius 1`, K = 8) at
  both standing budgets:
  `depth_turns 4` → **151 safety-net rows, 151 capped**;
  `nodes 200000` → **153 safety-net rows, 153 capped**; pools averaging 34–44,
  inside the 33–52 band §4 states. Those are §4's own two figures, reproduced
  from a separately written instrument. Concentrated in three positions (cases
  13, 14, 19); the other seventeen resolve tactically before a safety-net row
  appears, which is why a seat and not a bare assertion was the right call.
- `tools/determinism.sh` passes with the seat: `determinism: seat
  staged-safety-net-cap: ok — 40 searches, 20 positions, no difference outside
  nps/time`, whole gate `5 seat(s)` in **148 s**.

## 6. The mutation receipts hold. Six re-taken, all die where claimed; one of my own dies too.

Re-taken in my own worktree with my own harness (not the receipt's), suite
`wp15d_safety_net_cap_tests`:

```
MUTANT M1: DIED at: at_a_turn_one_root_the_cap_binds_at_ply_one_because_that_ply_is_a_new_turn
MUTANT M2: DIED at: at_a_turn_one_root_the_cap_binds_at_ply_one_because_that_ply_is_a_new_turn
MUTANT M3: DIED at: the_cap_never_fires_off_a_safety_net_row
MUTANT M4: DIED at: a_truncated_node_withholds_its_unproved_bounds_and_keeps_its_proved_one
MUTANT M5: DIED at: a_truncated_node_withholds_its_unproved_bounds_and_keeps_its_proved_one
MUTANT M6: DIED at: equal_scoring_safety_net_cells_are_emitted_in_ascending_coordinate_order
MUTANT M7: DIED at: the_off_value_truncates_nothing_and_records_nothing     [mine: the guard's `> 0` deleted]
```

`artifacts/wp15d_a_mutation_receipts_v1.txt` names the same six and the same
death sites. **All six reproduce.** M7 is the mutant §7's first row registers and
which no shipped test is named for; it dies anyway, by panic on the emptied set —
which is why MAJOR 2's row 1 is a bookkeeping finding and not a behavioural one.

## 7. The tie-break test earns its keep — and its own history is the evidence.

`equal_scoring_safety_net_cells_are_emitted_in_ascending_coordinate_order`
(`:238-306`) asserts the ORDER, not run-to-run agreement, and says why:
"Two runs agree under an unstable sort too … a property the defect PRESERVES,
which `docs/process.md`'s vacuous-criterion clause forbids as a criterion." That
is the vacuous-criterion clause applied correctly and unprompted. Its
`ties > 0 && groups > 1` guard (`:294-298`) is a real falsifiability guard: the
receipt records that two earlier fixtures let M6 survive (an 18-cell ball, where
`sort_unstable` insertion-sorts and is stable in practice; and a 36-cell
single-stone ball where every cell scored 36 — a single tie group, the one input
an unstable sort provably leaves alone), and the guard is what made the third
failure visible. M6 dies on the shipped fixture. Note this test needs no cap at
all — it drives `staged_candidates` directly at K = 0 — which is correct: the
boundary the cap rests on is `delta_rank`'s stability, not the truncate.

## 8. `tools/determinism.sh` against `tools/SHELL_CHECKLIST.md`, answered by item.

The change is four lines: one `SEATS` entry and three comment lines.

- **1 (discarded command substitution).** None added.
- **2 (pipeline in a `then` body).** None added.
- **3 (`grep` under `pipefail`).** None added. The existing `|| true` sites the
  new seat flows through (`:157`, `:164`, `:168`, `:209`) are unchanged and
  already carry the reason.
- **4 (`LC_ALL` and guard width).** Not engaged; no new character class.
- **5 (index vs worktree).** Not engaged; the seat names paths the preflight at
  `:102-107` stats, and both exist.
- **6 (a sweep by prefix owns the prefix).** Not engaged. `$WORK` is `mktemp -d`
  and per-seat files are `$name.`-prefixed (`:176`), so the new seat name
  `staged-safety-net-cap` cannot collide with the other four.
- **7 (traps).** Unchanged single `trap … EXIT` at `:112`.
- **8 (one spelling per number, one refusal per reason).** The seat entry has
  **three** fields and no budget override. That is the documented path: `:282`
  reads `extra_budgets` empty, `:284` expands it unquoted to nothing, `:190`
  sees `$# == 0` and takes `BUDGETS`. The comment at `:185-188` explains exactly
  why `"${@:-}"` was wrong here. Correct, and the run confirms it — the seat
  reports "40 searches" (20 positions × 2 standing budgets).
- **9 (caller-controlled values reaching a record).** The seat's fields are
  literals in the script.
- **10 (THE COVERAGE RULE).** `tools/determinism.sh` produces a verdict, not a
  recorded number, and it is driven by `tools/ci.sh` gate 9/19 plus
  `crates/pistol-cli/tests/determinism_tests.rs`. No new number is produced and
  no per-seat test is owed; the seat itself is the test.
- **11 (destructive path containment).** No new `rm`/`mv`/write. `$WORK` is
  script-created.
- **12 (RUN VOID vs FAIL).** Not degraded. Worth naming for the record: my own
  instrument hit item 12's exact failure mode this session (NOT REPRODUCED 1).
- **Findings:** MINOR 3 only (the stale `FOUR SEATS` count). The seat costs the
  gate ~30 s of its 148 s.

## 9. Session (A) took NO measurement — D-482 and D-483 respected.

```
$ git diff bd1faf4..5279cae -- crates/ configs/ tools/ | grep -nE "nps|time_ms|ms=|[0-9]+\.[0-9]+ *(x|×)|ratio|bench|elapsed|seconds|MEASURED"
```

returns three substantive lines, none of which is a measurement in D-482's sense:

- `configs/gate_staged_snk_v0.toml:62` — `MEASURED balls 22/22/22/18/15 at eleven
  stones (configs/gate_v0.toml's own header)`, copied verbatim from
  `gate_staged_v0.toml` along with the rest of the file. A pre-existing committed
  figure carried by a faithful copy, not a new reading.
- Two comments in the tie-break test recording **mutation-testing outcomes**
  ("`sort_by_key` → `sort_unstable_by_key` survived every suite"; "the
  single-stone fixture gave 36 cells all scoring 36"). These are test-discipline
  observations of the kind CLAUDE.md's Process section requires a reviewer
  finding to carry, not calibration, bench, ratio, nps or timing.

No benchmark, no timing, no ratio, no nps, no node-count-per-second anywhere in
the range. `crates/pistol-search/tests/wp15d_calibration.rs` and
`wp15d_bench_counters.rs` — §5's and §8's instruments — are correctly **absent**.
The design's revision-5 header and D-481/D-482/D-483 are the only documentation
changes, and §7's falsifier row was edited from 19 to 9 exactly as D-482
authorises ("capped-rows constant 9 by derivation receipt").

## 10. Other gates.

`cargo fmt --all --check` exit 0. `cargo clippy --workspace --all-targets
--locked -- -D clippy::all` clean. `cargo test -p pistol-search --locked` ok.
`tools/solver_determinism.sh` PASS. `tools/config_check.sh`'s glob covers the new
document. `tools/ci.sh` gate 17/19 is the one red gate, and it is BLOCKING 1.

---

# SUMMARY OF WHAT THE NEXT ROUND OWES

| # | grade | one-clause remedy |
|---|---|---|
| BLOCKING 1 | requirement | register `wp15d_safety_net_cap_tests.rs` in `docs/rule9_justifications.md` (no line count), or bring it under 300 lines |
| BLOCKING 2 | requirement | write §6.4's `no_cutoff_inside_the_played_turn_rests_on_an_unproved_bound` — one `Searcher` across a game, warm table, census reads 0, cold-table control beside it |
| MAJOR 1 | requirement | split the store-rule test back into §7's two registered rows, each naming a bound kind, so M9 and M10 die |
| MAJOR 2 | requirement | either write §7's row 1 against a staged sha-pinned fixture or record in the design that `instrument_golden_v1.txt` is radius-only and the row is closed by inspection |
| MINOR 1 | — | move the doc comment back onto `record_quiet_safety_net` and give `record_safety_net_cap` its own |
| MINOR 2 | — | replace the copied first two lines of `gate_staged_snk_v0.toml` with what the file's own later comment already says |
| MINOR 3 | — | delete the seat counts from `tools/determinism.sh:19` and `:45` rather than update them |
| MINOR 4 | — | "the root turn's" rather than "the root turn's two" in `params.rs:66` |
| MINOR 5 | — | one clause in the design reconciling §4's `search.rs` row with §3 |
| MINOR 6 | — | architect's call: pin a target, or restore the representability refusal |

**None of the ten is a correctness finding**, so none returns the package to the
architect and all are inside D-424's reach if the operator judges any of them to
constrain nothing. I record explicitly that I looked for a correctness finding
against the mechanism and did not manufacture one: the guard is right at every
turn number I could construct, `9` is right by two derivations, and the store
rule's `Bound::Lower` half is sound on every path I could trace.
