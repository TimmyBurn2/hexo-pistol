# REVIEW-impl — pre-Phase-2 sweep, GROUP C (comments and doc text only)

**Revision under review: `cb6e853`.** `git rev-parse HEAD` → `cb6e8532e163bcb41497993da3e9518c719cf448`
— **it still matches HEAD**, at the start of this review and at its end.

**The working tree does not, and did not stay still under me.** At the start it
carried one untracked file (`docs/experiments/pre_phase2_sweep_CLOSURE.md`) and
no tracked modification. By the time I finished, another session had staged
uncommitted edits to thirteen tracked files, several of them the exact sites this
review reports findings against — `crates/pistol-core/src/symmetry.rs`,
`crates/pistol-search/src/position.rs`, `crates/pistol-cli/src/corpus/{bench,mod}.rs`,
`crates/pistol-cli/src/random_openings/error.rs`,
`crates/pistol-solver/src/{solver,state,config}.rs` and
`crates/pistol-solver/src/bin/solver-cost.rs`. Spot-checking two, they address
MAJOR-3 (`Symmetry::IDENTITY` gains a `///`) and MINOR-1 (the D-42 citation is
dropped).

**This changes nothing about this report.** Every finding, count and machine
check below is taken at `cb6e853` and is true of `cb6e853`. Under the Process
section's rule that reviews of superseded revisions do not transfer, the
uncommitted fix round is a different revision and **owes its own review**; I have
not read it, have not verified it, and it must not be treated as discharging
anything here. I flag it only so the operator is not surprised that `git status`
in the live tree disagrees with this document's premise.

**Scope: commit `f4f4a5c`** ("the comment rows close"), 69 files, +577 / −508,
claiming closure of audit rows **A-11 … A-16** of
`docs/audit/repo_audit_2026-09.md`. Its parent is `c586837`; the audit itself was
taken at `d83ac01`. Every count below is re-derived at the revision it names,
with `/usr/bin/grep` / `git grep` and `LC_ALL=C sort` per CLAUDE.md's Environment
section. Reviewer did not write this code.

Verification that required building a mutated tree ran in a detached worktree at
`/home/tom/pistol-runs/groupC-review`, never the live tree. It was restored
(`git checkout -- crates/`, `git diff --quiet` asserted) and then removed; it
produced no gitignored `artifacts/` of its own and its `sessions/` was tracked
content from the checkout, so removal took nothing that owed an export receipt.

---

## Half one — machine checks

| check | result |
|---|---|
| `cargo fmt --all --check` | **clean**, no output, `FMT_EXIT=0` |
| `cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings` | **clean**, `CLIPPY_EXIT=0`, `Finished dev profile … in 2.29s` |
| `cargo doc --workspace --no-deps` (all eight `crates/*/src/lib.rs` `touch`ed first, so all eight re-emit) | **0 warnings**, `DOC_EXIT=0`. All eight crates re-`Documenting`-ed: core, api, solver, eval, search, engine, cli, arena |
| `cargo test --workspace --locked` | **181 suites, 1142 passed, 0 failed, 21 ignored**, `TEST_EXIT=0`. No `FAILED`, no `failures:`, no `panicked at` anywhere in the log. |
| `cargo test --workspace --doc` | **clean**, `DOCTEST_EXIT=0`; ten doctests ran across eight crates (see below) |

`CARGO_TARGET_DIR` was **not** exported around `cargo test` in the live tree.

**This package's byte-identity gate, quoted from the gate's own log output:**

```
     Running tests/instrument_golden_tests.rs
running 1 test
test instrument_behavior_byte_identical_pre_post ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.21s

     Running tests/census_protocol_tests.rs
running 14 tests
… test the_census_line_field_order_is_the_documented_one ... ok
… test a_census_row_carries_the_canonical_key_of_the_position_it_fired_at ... ok
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 42.09s
```

**The four moved doctests all run and all pass**, quoted from the doctest log:

```
test crates/pistol-search/src/params.rs - params::SearchParams (line 160) - compile fail ... ok
test crates/pistol-solver/src/state.rs - state::ThreatState (line 59) ... ok
test crates/pistol-solver/src/state.rs - state::ThreatState (line 67) - compile fail ... ok
test crates/pistol-solver/src/state.rs - state::ThreatState (line 75) - compile fail ... ok
```

They are registered as doctests after the move — the `- compile fail` suffix is
rustdoc's own, so the fences are live and not inert prose.

### The four `compile_fail` doctests — verified non-vacuous, and failing for the right reason

Group C moved one `compile_fail` out of `crates/pistol-search/src/params.rs`'s
`//!` onto `SearchParams`, and three out of `crates/pistol-solver/src/lib.rs`'s
`//!` onto `ThreatState`. A `compile_fail` that starts failing for a *different*
reason is vacuous, and the solver's own doc says so in as many words — so I did
not reason about it, I measured it. In the worktree I flipped every
```` ```compile_fail ```` fence to a plain ```` ``` ```` and read the errors:

```
crates/pistol-search/src/params.rs:162
  error[E0599]: no associated function or constant named `default`
                found for struct `SearchParams` in the current scope
crates/pistol-solver/src/state.rs:73  (doctest at line 67)
  error[E0603]: module `table` is private   --> crates/pistol-solver/src/lib.rs:31:1
crates/pistol-solver/src/state.rs:77  (doctest at line 75)
  error[E0603]: module `line` is private    --> crates/pistol-solver/src/lib.rs:29:1
crates/pistol-solver/src/state.rs   (doctest at line 59, NOT compile_fail)
  test result: … 1 passed
```

All three intended reasons hold: **E0599** is rule 1's "no `Default`", and both
**E0603**s are the store privacy the block exists to mechanise. The plain
example at line 59 still compiles, which is the whole of the doc's own
non-vacuity argument for the second `compile_fail`. Worktree restored, `git diff
--quiet` asserted.

Path resolution was the risk and it is discharged: `SearchParams` is re-exported
at `crates/pistol-search/src/lib.rs` (`pub use params::{… SearchParams …}`), so
the failure is the missing `default` and not an unresolved path; `mod line;` and
`mod table;` are private in the solver root and `empty_cells` / `unpack` are
`pub(crate)`, so E0603 is the first error either snippet can reach.

---

## Half two — the twenty-comment sample

Twenty comment or doc blocks **changed by `f4f4a5c`**, spread across all eight
crates, judged against CLAUDE.md's Code style section.

| # | site (at `cb6e853`) | verdict | one-line reason |
|---|---|---|---|
| 1 | `crates/pistol-api/src/lib.rs:1-6` (root, 8→6) | KEPT-CORRECTLY | Drops the duplicated "intentionally empty" sentence; the rule-11 claim and the "do not add code without a spec" instruction both survive. |
| 2 | `crates/pistol-core/src/lib.rs:1-15` (root, 59→11) | KEPT-CORRECTLY | The deleted bullets were a hand-maintained module list rustdoc builds itself; the rule-2 and rule-5 claims and the D-37 std-only pin survive in the 11 lines. |
| 3 | `crates/pistol-core/src/board.rs:102-104` — `apply`'s `# Errors` | KEPT-CORRECTLY | Names `CoreError::OccupiedCell`, which is the only variant the body returns. |
| 4 | `crates/pistol-core/src/state.rs:164-168` — `place`'s `# Errors` | KEPT-CORRECTLY | Names four variants; `GameDecided` is returned directly, the other three arrive through `board.check_placement(at)?` / `board.apply(…)?`. Verified against both callees. |
| 5 | `crates/pistol-core/src/play.rs:63-70` — `unmake_turn`'s `# Panics` + `# Errors` | KEPT-CORRECTLY | `HISTORY_DESYNC` is the panic the body raises; `TurnInProgress` is direct and `NothingToUndo` propagates from `self.undo()?`. Both sections earn their place — a caller can handle one and not the other. |
| 6 | `crates/pistol-eval/src/lib.rs:1-10` (root, 41→7) | KEPT-CORRECTLY | Every deleted claim has a home: D-3's mate band at `eval.rs:5-9`, D-21's load-time refusal at `weights.rs:42`, the seedless-hasher determinism at `handcrafted.rs:101`, `EVAL_DESYNC`'s rule-3 argument at `handcrafted.rs:23-28`. |
| 7 | `crates/pistol-engine/src/lib.rs:1-10` (root, 26→7) | SHOULD-NOT-HAVE-CHANGED (in part) | The module list rightly goes, but the deleted `# Determinism` block is a **crate-wide** rule-4 invariant ("reads a clock in exactly one place") that no item can restate — see MINOR-3. |
| 8 | `crates/pistol-engine/src/budget.rs:54-58` — `validate`'s `# Errors` | KEPT-CORRECTLY | `EngineError::Config` is right: both refusals go through the `EngineError::config(…)` constructor, and the section names the two conditions (zero, past `MAX_MOVETIME_MS`). |
| 9 | `crates/pistol-engine/src/budget.rs:95-100` — `resolve`'s `# Errors` | KEPT-CORRECTLY | "Whichever of `require`, `validate` and `check_supported` refuses first" is exactly what the body does, and naming the delegates beats re-listing their variants. |
| 10 | `crates/pistol-search/src/lib.rs:17-20` — the A-15 trailing comment | KEPT-CORRECTLY | The D-353 comment moves above `pub mod search;`; CLAUDE.md's "own line" is satisfied and nothing is lost. |
| 11 | `crates/pistol-search/src/params.rs` — the `//!` block (A-11) | KEPT-CORRECTLY | Both paragraphs moved onto the items they are about, the `compile_fail` with them; `params` was the only module in the crate carrying a `//!`, so its removal makes the crate uniform rather than uneven. |
| 12 | `crates/pistol-search/src/params.rs:155-163` — `SearchParams`' no-`Default` paragraph | KEPT-CORRECTLY | Rule 1's "no code-side default" is a property of *this type*, and the doctest that enforces it now sits on it. Verified E0599 above. |
| 13 | `crates/pistol-search/src/position.rs:76-78` — `state()` (was `/// The game.`) | NOW-WRONG (citation) | The claim is right and worth making, but it cites **D-42**, which is about `GameState::from_plies` replaying a move list — it says nothing about the game state being the authority caches are maintained against. See MINOR-1. |
| 14 | `crates/pistol-search/src/position.rs:82-83` — `board()` (was `/// The stones.`) | KEPT-CORRECTLY | Replaces a name-restatement with the rule-2 reason the board is reached *through* the game rather than kept beside it. |
| 15 | `crates/pistol-search/src/search.rs:105-113` — `new`'s `# Errors` | KEPT-CORRECTLY | Four refusal classes named, and all four exist: `…radius`, `…tier_t_own_count`/`…tier_t_opponent_count`, `…q_depth_turns`, `solver.on_search_path`. |
| 16 | `crates/pistol-search/src/pvs.rs:965-971` — `should_stop`'s mask comment | KEPT-CORRECTLY | The dropped MEASURED receipt (mean 156,313 nodes against a 50,000 budget) is not lost — it stands at `stop.rs:25` and in full in D-463. |
| 17 | `crates/pistol-solver/src/lib.rs:1-15` (root, 110→15) | KEPT-CORRECTLY | Both retracted claims are independently false at HEAD (see "Claims 4a/4b" below); the D-243 conditioning paragraph — the one thing here that is a correctness argument — is kept verbatim. |
| 18 | `crates/pistol-solver/src/state.rs:43-85` — the privacy block on `ThreatState` | SHOULD-NOT-HAVE-CHANGED (in part) | The move is right and the doctests survive, but the **third** example's own non-vacuity sentence was dropped from a paragraph whose entire subject is that a bare `compile_fail` proves nothing. See MAJOR-2. |
| 19 | `crates/pistol-solver/src/sets.rs:20-32` — the class-assert rationale (22→14) | KEPT-CORRECTLY | Compresses to the three claims and points at D-262 for the argument — the D-423 "state it once" shape, done right, and it keeps the honest admission that the third conjunct is vacuous at today's constants. |
| 20 | `crates/pistol-arena/src/identity.rs:59-66` — `capture`'s `# Errors` + the binding comment | KEPT-CORRECTLY | `EngineBinaryDigestMismatch` is the variant the body returns; the compressed comment keeps D-252's reproducer, D-21's offline-validation reason and the before-the-spawn reason. |

Sample composition: api 1, core 3, eval 1, engine 3, search 7, solver 3, arena 1,
cli 1 (row 20 doubles as the arena entry; `crates/pistol-cli/src/flags.rs:9-11`
was also read and is KEPT-CORRECTLY — "the usage text, for a word that is not a
`--flag` or a flag with no value" is exactly what the body returns).

---

## Re-derived counts

Every number below is mine, not the closure's, with the command that produced it.

### A-12 — crate-root `//!` lines: **CONFIRMED, 344 → 76**

`for f in crates/*/src/lib.rs; do git show <rev>:$f | grep -c '^//!'; done`

| crate | `c586837` | `cb6e853` |
|---|---|---|
| pistol-api | 8 | 6 |
| pistol-arena | 45 | 10 |
| pistol-cli | 33 | 10 |
| pistol-core | 59 | 11 |
| pistol-engine | 26 | 7 |
| pistol-eval | 41 | 7 |
| pistol-search | 22 | 10 |
| pistol-solver | **110** | **15** |
| **total** | **344** | **76** |

The closure's 344 → 76 and its "largest `pistol-solver` 110 → 15" both reproduce
exactly, and the audit's list does omit `pistol-api` as the closure says.

### A-11 — the only non-root `//!`: **CONFIRMED**

`git grep -l '^//!' cb6e853 -- crates/` returns, under `src/`, only the eight
crate roots plus `crates/pistol-solver/src/bin/solver-cost.rs` — a binary crate
root, which A-11 itself records as permitted. `params.rs` is gone. (See MINOR-4
for what that bin root still is.)

### A-13 — blocks of ≥ 8 consecutive `//` lines: the closure's detector is **not the audit's**

Two detectors, both over `crates/*/src/**.rs`, both counting maximal runs:

- **Detector A** — breaks a run at a bare `//` line.
- **Detector B** — does not.

| revision | detector A | detector B |
|---|---|---|
| `d83ac01` (the audit's own revision) | **16** | 15 |
| `c586837` (pre-group-C) | 17 | **19** |
| `cb6e853` (HEAD) | **4** | **8** |

**Detector A reproduces the audit exactly** — 16 blocks, and every one of the six
the audit names by line with the length it gives: `pvs.rs:350` (13), `pvs.rs:227`
(11), `dfpn.rs:534` (11), `conclusion.rs:135` (9), `validate.rs:196` (9),
`pvs.rs:783` (9). Detector B gives 15 at that revision — i.e. it *cannot* be the
audit's, because it does not reproduce the audit's number at the audit's
revision. The closure's "16 → **19** at HEAD; now **8**, longest 22 → 14" is
detector B throughout. See MINOR-2. The row's substance survives either reading:
17 → 4, or 19 → 8. **The class is closed; only the reported magnitude of the
"grown" claim is an artefact of the detector swap.**

### A-14 — public `fn` returning `Result` without `# Errors`: **0 remain, CONFIRMED; the closure's "46 without" is not reproducible**

My detector parses the actual **return type** of every `pub fn` in non-test
`src/` (bodies past the first `#[cfg(test)]` excluded), walks back over
attributes to the doc block, and looks for a `# Errors` heading:

| | `c586837` | `cb6e853` |
|---|---|---|
| `pub fn` returning `Result` | 96 | 96 |
| …lacking `# Errors` | **74** | **0** |

**The load-bearing claim — 0 remain — is confirmed.** The closure's population
count (100) is close to mine (96; the gap is `pub(crate)`, which I exclude and a
`# Errors`-side count does not). But the closure's "**46 without**" is
contradicted by the commit's own diff:

```
$ git grep -c '/// # Errors' c586837 -- crates/*/src   →  24
$ git grep -c '/// # Errors' cb6e853  -- crates/*/src   → 102
$ git show f4f4a5c | grep -c '^+.*# Errors'             →  78
```

102 − 24 = 78 added, 0 removed. If only 46 functions had lacked a section, 32 of
the 78 additions would have landed on functions that already had one — which
cannot happen. See MAJOR-1.

### `# Errors` accuracy — sampled by hand and then checked mechanically: **clean**

Hand-sampled eight: `Board::apply`, `Board::undo`, `Board::check_placement`,
`GameState::place`, `GameState::unmake_turn`, `Budget::{require,validate,
check_supported,resolve}`, `Channel::start`, `identity::{digest_of,capture}`,
`handshake::shake`, `tt::Table::new`, `Searcher::{new,search}`,
`SolverSection::validate`, `flags::{pairs,one}`. None names a variant the
function cannot return.

Then mechanically, over all of non-test `src/`: of the 100 functions carrying a
`# Errors` section, 42 name a concrete `…Error::Variant` in it. Every named
variant is reachable from its function's body — the 19 my first pass flagged are
all constructor-helper spellings (`EngineError::config`, `ArenaError::io`,
`SearchError::params`) or `?`-propagation from a named callee, each confirmed by
reading. **Zero `# Errors` sections name the wrong variant.** That was the
failure mode worth hunting and it is not present.

One weak-but-not-wrong case: `SolverSection::validate`'s section names
`SolverConfigError::Epsilon` and `::ZoneOrders` and then says "or whichever bound
refuses first", where the body also returns `::FreeStoneRadius` and `::TtEntries`
by name. Nothing is false; two nameable variants are left unnamed.

### A-15 — "32 trailing inline comments": **the closure is right, and its arithmetic is off by one**

The audit's own pattern, `grep -E '[^ ]\s+// [A-Za-z]'` over `crates/*/src/`:

| revision | hits |
|---|---|
| `d83ac01` | **36** (the audit says 32) |
| `c586837` | 36 |
| `cb6e853` | 35 |

Reading all 36 at HEAD-before:

| where | count | what it is |
|---|---|---|
| `crates/pistol-solver/src/zone.rs` lines 245-372 | 30 | inside `#[cfg(test)] mod tests`, which opens at `zone.rs:176` — every hit is past it |
| `crates/pistol-solver/src/tt.rs:305` | 1 | inside `#[cfg(test)]`, which opens at `tt.rs:178` |
| `crates/pistol-solver/src/cover.rs:78-81` | 4 | rows of an ASCII table inside a ` ```text ` block inside a `///` doc — the `//` is doc TEXT, not a line comment |
| `crates/pistol-search/src/lib.rs:31` | 1 | the one genuine trailing comment in non-test code |

**The conclusion is correct and I verified it independently and more strictly.**
A broader detector — any `//` after non-whitespace on a line that does not start
with `//`, skipping string literals and `://`, over every `crates/*/src/**.rs`
outside `#[cfg(test)]` regions — returns **0 hits at `cb6e853`**. There is no
trailing inline comment left in non-test code, under a pattern wider than the
audit's.

The closure's split is **30 + 4 + 1 = 35**, one short of 36: it names `tt.rs:178`
as opening a test module in the same breath as the `zone.rs` ones but does not
add `tt.rs`'s single hit to the 30. The correct split is **31 + 4 + 1 = 36**. See
MINOR-5.

### A-16 — name-restating docs: **CONFIRMED, 9 → 0**

`git grep -nE '^\s*/// The [A-Za-z]+\.$' <rev> -- crates/*/src`: **9** at both
`d83ac01` and `c586837`, **0** at `cb6e853`. The audit's row says "2 hits;
sampled" — the closure's correction to 9 is right, and the audit under-reported
by sampling. Both hits the audit *does* name (`position.rs:76`, `:81`) are among
the 9 and both were replaced with something to say. The other seven were deleted
— which is where MAJOR-3 comes from.

### Claim 4a — "Nothing links this crate yet": **the retraction is correct**

```
crates/pistol-engine/Cargo.toml:20:pistol-solver = { workspace = true }
crates/pistol-search/Cargo.toml:22:pistol-solver = { workspace = true }
```

Both present at `d83ac01` too, so the sentence was already false when the audit
ran. It is called on the search path — `crates/pistol-search/src/pvs.rs:337`
(`&& let Some(verdict) = self.solver_verdict()`) and `:830` — and the engine
constructs `pistol_solver::SolverParams` at `instance.rs:211,220`. **Deleting it
was right.**

### Claim 4b — "the TWELVE QUERIES … NONE OF THE ELEVEN": **twelve is right, eleven was a contradiction**

Counted at `cb6e853`. Nine on `ThreatState` in `query.rs`: `hot_windows`,
`win_in_one_ply_windows`, `completed_windows`, `live_windows_at_count`,
`win_in_one_ply_cells`, `threat_cells`, `cells_raising_to_hot`,
`live_cells_at_count`, `can_win_this_turn` (`query.rs:38`'s `from_state` is a
constructor on `StonesLeft`, not a query). Three on `ThreatState` in `cover.rs`:
`min_hitting_set_exceeds`, `blocking_covers`, `unblockable_double_threat`
(`cover.rs:27,60` are `cells()` on the answer types). **9 + 3 = 12.** The
paragraph did contradict itself two sentences after stating the number.
**Deleting it was right.**

---

## Findings

### BLOCKING — none.

### MAJOR-1 — the closure's A-14 count (46) is contradicted by the commit's own diff (78)

The closure's A-14 row makes a point of having *corrected* a bad detector: "the
first count this package took was 69, by a detector that matched 'Result'
anywhere in the next four lines … **Corrected by parsing the RETURN TYPE. 0
remain**". The corrected figure — "100 such functions, **46 without**" — is
itself wrong, and the refutation is in the commit it describes.

Reproducer:

```
$ cd /home/tom/Projects/HeXO-AlphaBeta
$ git grep -c '/// # Errors' c586837 -- crates/*/src | \
    grep -E '^c586837:crates/[^/]+/src/' | awk -F: '{s+=$NF} END {print s}'
24
$ git grep -c '/// # Errors' cb6e853 -- crates/*/src | \
    grep -E '^cb6e853:crates/[^/]+/src/' | awk -F: '{s+=$NF} END {print s}'
102
$ git show f4f4a5c | grep -c '^+.*# Errors'
78
```

78 sections added, none removed. A population of "46 without" cannot absorb 78
additions. My own return-type-parsing detector puts the figure at **74**, which
78 comfortably accommodates (the remaining 4 land on `pub(crate)` functions my
detector excludes).

**This is a record defect, not a code defect** — the code outcome ("0 remain") is
independently confirmed. But the row is the one place the package asserts it
measured this carefully, and the number it publishes is not the number its own
diff shows. **Fix:** correct the row to the derivable figures (78 sections added;
0 of 96 public `Result`-returning functions now lack one), or state the detector
that yields 46 so the reading can be reproduced.

### MAJOR-2 — the third `compile_fail`'s non-vacuity argument was deleted from the paragraph that exists to supply it

At `b60c3d3`/`c586837` the solver root said:

> A bare `compile_fail` passes on ANY compilation error … What makes the second
> example non-vacuous is the FIRST: every line of it appears there and compiles,
> so the only line it can fail on is the one that differs. **The third is one
> line whose only other outcome is a type error.**

The bolded sentence is gone at `cb6e853`
(`crates/pistol-solver/src/state.rs:79-85`). What survives argues for example
two and says nothing about example three — inside a paragraph whose stated
premise is that a `compile_fail` with no argument behind it proves nothing.

The claim itself is **true**: I measured example three and it fails E0603
(`module line is private`), which is the intended reason. So this is a deleted
*justification*, not a broken mechanism — but it is deleted reasoning that now
lives nowhere (not in D-254, not in D-261, not in a design doc; `git grep 'type
error' -- crates docs` finds no successor), and it is the one sentence in the
block a future reader would need to know whether example three still earns its
place.

**Fix:** restore the sentence, or replace it with the measured reason (`unpack`
is `pub(crate)` behind a private `mod line`, so E0603 is the only error a
one-line call can reach).

### MAJOR-3 — closing A-16 by deletion left seven public items with no doc at all

A-16's class is "docs that restate the name", and the fix applied to
`position.rs`'s two hits — replace with something to say — is right. The other
seven were **deleted**, leaving public items undocumented, which trades one
clause of CLAUDE.md's Code style ("no doc that restates the signature") for
another ("Public items get `///` docs"). The same commit's justification for
cutting the crate roots is that "a module's name and its **public item docs**
carry its purpose" — so it removed seven of the things it is leaning on.

Mechanical reproducer, run in the worktree (never the live tree):

```
$ cd /home/tom/pistol-runs/groupC-review
$ export CARGO_TARGET_DIR=$PWD/target RUSTFLAGS="-W missing_docs"
$ for rev in c586837 cb6e853; do
    git checkout -q --detach $rev; touch crates/*/src/lib.rs
    cargo build --workspace --locked 2>&1 |
      grep -oE 'missing documentation for an? [a-z ]+' | LC_ALL=C sort | uniq -c
  done
```

```
--- c586837 ---            --- cb6e853 ---
  1 … for a function         1 … for a function
  1 … for a method           1 … for a method
108 … for a module         109 … for a module
 23 … for a struct field    29 … for a struct field
                             1 … for an associated constant
```

**+6 struct fields, +1 associated constant** — exactly the seven A-16 deletions:

| item | kind |
|---|---|
| `BenchPosition::position` (`crates/pistol-cli/src/corpus/bench.rs:60`) | `pub` field on a `pub struct` |
| `crates/pistol-cli/src/corpus/mod.rs:39` `pub path: PathBuf` | `pub` field on a `pub struct` |
| `RandomOpeningsError::RadiusPastCeiling::ceiling` (`error.rs:51`) | variant field on a `pub enum` |
| `RandomOpeningsError::CountPastCeiling::ceiling` (`error.rs:57`) | variant field on a `pub enum` |
| `RandomOpeningsError::…::path` (`error.rs:140`) | variant field on a `pub enum` |
| `Symmetry::IDENTITY` (`crates/pistol-core/src/symmetry.rs:24`) | `pub const` on a `pub struct` |
| `SolveResult::outcome` (`crates/pistol-solver/src/solver.rs:36`) | `pub` field on a `pub struct` |

The two `RandomOpeningsError` variants are now half-documented — `max_radius` and
`n_openings` keep their `///` while their `ceiling` sibling has none, so rustdoc
renders one field of a two-field variant blank. `Symmetry::IDENTITY` is the
public constant naming the identity symmetry and now carries nothing.

The **+1 module** in the same table is *not* a finding: it is `pistol_search::
params`, whose `//!` A-11 removed by design, and `params` was the only module in
that crate that had one — so the crate is now uniform rather than uneven, which
is what CLAUDE.md's "no `//!` in ordinary modules" asks for.

**Fix:** give the seven a `///` that says something (`ceiling` → what the ceiling
is *for*; `IDENTITY` → the symmetry that fixes every cell), or record an ADR line
that a field whose name is its whole content is exempt. Deleting is the one
option that satisfies neither clause.

### MINOR-1 — `Position::state()`'s new doc cites D-42 for something D-42 does not say

`crates/pistol-search/src/position.rs:76-78`:

> /// The authority. Everything else this type holds — the eval's running sum,
> /// the threat state — is a cache maintained against it, so a reader deciding
> /// what is true asks here (docs/decisions.md D-42).

D-42 (`docs/decisions.md:100`) is entirely about `GameState::from_plies`
replaying a move list through `place` being the only way to reach a position
other than by playing to it. It does not say the game state is the authority
caches are maintained against. The claim is a good one and worth making; the
citation is borrowed from a *different* file's deleted line (pistol-**engine**'s
root said "[`position`] — … the replay through the rules that is the only way one
becomes a game (D-42)"), and it landed on a new claim in pistol-**search**.

This is the shape D-253 names as the drift rule 10 exists to prevent — citing an
ADR for a proposition it does not carry. **Fix:** drop the citation, or point at
the ADR that does say it.

### MINOR-2 — A-13's "16 → 19, PRESENT, grown" is partly a detector swap

Table under "A-13" above. The audit's detector (breaking runs at a bare `//`)
returns exactly 16 at `d83ac01` and reproduces all six of the audit's cited
`file:line (length)` pairs; it returns **17** at `c586837`. The closure's
detector returns **15** at `d83ac01` — so it contradicts the audit's own number
at the audit's own revision — and 19 at `c586837`.

So the growth the closure reports as "16 → 19" is really "16 → 17" plus a
detector change, and the "longest 22 → 14" figures are detector-B lengths that
do not correspond to any number the audit published. The row's *closure* is
unaffected: under the audit's detector the count went **17 → 4**, under the
closure's **19 → 8**, and both are large reductions.

docs/process.md's re-derivation clause asks that a re-derived count be produced
by an instrument calibrated against the original. **Fix:** state the detector, or
report the audit's.

### MINOR-3 — pistol-engine's crate-wide clock invariant is now only in the ADR log

The deleted `# Determinism` block said the crate "reads a clock in exactly one
place — translating a wall-clock budget into the instant it expires at, which
instrument mode refuses to be given at all (D-22, D-73) — and holds no other
nondeterministic state". That is a **whole-crate** rule-4 invariant; no single
item's `///` can carry it, and CLAUDE.md permits a crate-root `//!` for exactly
what rustdoc genuinely needs.

It is not lost — D-73 (`docs/decisions.md:170`) states it in full, and
`crates/pistol-engine/Cargo.toml:18` carries "the only place a `Budget` becomes a
`Stop` (D-73)". The single `Instant::now()` is at `instance.rs:301` and has no
comment saying it is the only one. So a reader who checks the ADR is fine and a
reader who reads the crate is not told. Downgraded to MINOR because the claim has
a home; raised above nothing because the home is not the code.

**Fix:** one line at `instance.rs:301`'s `stop_for`, or two lines back in the
root — a determinism invariant is what a root `//!` is for.

### MINOR-4 — every *library* root was cut to a few lines; the binary crate root was not

`crates/pistol-solver/src/bin/solver-cost.rs` is a crate root and still carries
**16** `//!` lines at `cb6e853`, untouched by this commit, while all eight
library roots went to 6–15. A-11 records it as "permitted" (it is a root, so the
`//!` is legal), and A-12's own command is scoped to `crates/*/src/lib.rs`, so
this is strictly outside both rows as written. But CLAUDE.md's clause is "`//!` is
permitted only at a crate root, **at most a few lines**", and 16 is what the
package just spent 69 files declaring is not a few.

Reproducer: `git grep -c '^//!' cb6e853 -- 'crates/*/src/bin/*.rs'`.

**Fix:** cut it in the same spirit, or record in the closure that binary roots
were deliberately out of scope.

### MINOR-5 — the A-15 split is 35 where the total is 36

The closure's paragraph reads "**30 are inside `#[cfg(test)] mod tests`**
(`zone.rs:176` opens it, and every `zone.rs` hit is past that line; `tt.rs:178`
likewise) … **4 are not line comments at all** … **1 is a genuine trailing
comment**". 30 + 4 + 1 = 35, against a stated total of 36. `tt.rs`'s single hit
(line 305, past the `#[cfg(test)]` at 178) is named in the parenthesis but not
counted: the split is **31 + 4 + 1 = 36**.

Reproducer:

```
$ git grep -nE '[^ ][[:space:]]+// [A-Za-z]' cb6e853 | sed 's/^cb6e853://' |
    grep -E '^crates/[^/]+/src/' | awk -F: '{print $1}' | LC_ALL=C sort | uniq -c
      4 crates/pistol-solver/src/cover.rs
      1 crates/pistol-solver/src/tt.rs
     30 crates/pistol-solver/src/zone.rs
```

The row's conclusion — one in non-test code, now zero — is unaffected and
independently confirmed.

### MINOR-6 — one `# Errors` names two of four variants and hand-waves the rest

`crates/pistol-solver/src/config.rs:86-91`, `SolverSection::validate`: the
section names `SolverConfigError::Epsilon` and `::ZoneOrders` "or whichever bound
refuses first", where the body also returns `::FreeStoneRadius` and `::TtEntries`
by name. Nothing false; two nameable variants unnamed, in a package whose whole
claim is that every public `Result` now says what it refuses.

---

## What I verified clean

- **`cargo fmt --all --check`** — no output, exit 0.
- **`cargo clippy --workspace --all-targets --locked -D clippy::all -D warnings`**
  — no diagnostics, exit 0.
- **`cargo doc --workspace --no-deps`** with all eight roots `touch`ed — **0
  warnings**, all eight crates re-documented. The closure's "clean for the first
  time" holds.
- **The four `compile_fail` doctests fail for the intended reason** — E0599 for
  `SearchParams::default`, E0603 ×2 for the private `table` / `line` modules —
  measured by flipping the fences in a worktree, not inferred. The plain
  companion example still compiles, which is what makes the second non-vacuous.
- **A-11** — `params.rs`'s `//!` is gone and it was the only non-root one under
  `src/` besides the permitted bin root. Nothing rustdoc needs was lost: both
  paragraphs and the doctest moved onto public items, and `params` was the only
  module in the crate with a `//!` to begin with.
- **A-12** — 344 → 76 reproduces exactly, per crate.
- **A-14's substance** — 0 of 96 public `Result`-returning functions in non-test
  `src/` lack a `# Errors` section, down from 74.
- **`# Errors` accuracy** — 42 sections name a concrete variant; every one is
  reachable from its function. **Zero name a wrong variant.**
- **A-15's substance** — 0 trailing inline comments in non-test code at HEAD,
  under a detector wider than the audit's.
- **A-16** — 9 → 0 by the audit's own pattern.
- **Both retracted solver-root claims are independently false**: `pistol-engine`
  and `pistol-search` both declare `pistol-solver` (and did at `d83ac01`), and it
  is called from `pvs.rs:337`; the query count is 9 + 3 = 12, and "eleven" was a
  self-contradiction.
- **Deleted crate-root prose has homes.** I traced every non-list claim removed
  from the eight roots. `pistol-core`: panic tokens documented at `coord.rs:6-9`,
  `win.rs:6`, `state.rs:9-13`; D-253's window-length history at `window.rs:14-19`;
  D-137 at `symmetry.rs:188,217,245`. `pistol-eval`: D-3 at `eval.rs:5-9`, D-21
  at `weights.rs:42`, `EVAL_DESYNC` at `handcrafted.rs:23-28`, the seedless
  hasher at `handcrafted.rs:101`. `pistol-solver`: the determinism argument at
  `line.rs:107-114` and `sets.rs:138-142` and in full in D-254; `THREAT_DESYNC`
  at `state.rs:13-15`; the store-privacy block moved intact onto `ThreatState`.
  `pistol-arena`: the never-writes-in-repo rule at `usage.rs:28`, D-408 at
  `game.rs:140`. `pistol-search`: D-20 at `params.rs:12` and `search.rs:128`.
  **The finding this review was most likely to have — load-bearing text deleted
  into nowhere — is present only as MAJOR-2 (one sentence) and MINOR-3 (one
  crate-wide invariant now only in the ADR).**
- **`docs/rule9_justifications.md` needs nothing new**: `f4f4a5c` is net −508
  lines and no file it touches grew past a threshold it was not already past.

## What I could not verify

- **The closure's "46 without" for A-14 and its A-13 counts** — I could not
  construct a detector that yields 46, nor one that yields 15-at-`d83ac01` *and*
  agrees with the audit. Both are recorded as findings rather than as
  disagreements I resolved.
- **Whether the seven bare public items of MAJOR-3 were a considered trade.**
  Nothing in the closure or in `docs/decisions.md` records a decision that a
  field whose name is its content is exempt from "public items get `///` docs",
  so I judged it against the rule as written. If such a decision exists, it is an
  ADR line and MAJOR-3 becomes a documentation gap instead.
- **Byte-identity of engine output across `f4f4a5c`.** The commit is
  comment-and-doc-only and D-578 rules that this package TRACES its digests
  rather than demanding identity (line shifts move symbol hashes), so I did not
  re-derive a baseline snapshot. The workspace suite standing in for it is
  reported above.
- **Whether `solver-cost.rs`'s 16-line bin root was deliberately excluded**
  (MINOR-4) — no scope statement in the closure settles it.

---

## Summary of findings

| severity | count | ids |
|---|---|---|
| BLOCKING | 0 | — |
| MAJOR | 3 | MAJOR-1 (A-14's published count refuted by its own diff), MAJOR-2 (third `compile_fail`'s non-vacuity argument deleted), MAJOR-3 (seven public items left with no doc) |
| MINOR | 6 | MINOR-1 (D-42 cited for what it does not say), MINOR-2 (A-13 detector swap), MINOR-3 (engine's clock invariant only in the ADR), MINOR-4 (16-line bin root untouched), MINOR-5 (A-15 split sums to 35 of 36), MINOR-6 (two of four variants named) |

**None of the nine names a way the code can produce a wrong answer**, so none is
in D-424's never-overruled class. All three MAJORs are small fixes: seven
one-line `///` docs, one restored sentence, and three corrected numbers in the
closure. Nothing here asks for the package to be redone.

## Verdict

**FAIL**

On two grounds, and neither is about the six rows failing to close — every one of
A-11 … A-16 I re-derived independently and every one is closed in substance, with
every machine check green.

1. **MAJOR-3**: the package exists to satisfy CLAUDE.md's Code style section, and
   it introduces a mechanically demonstrable breach of that same section on seven
   public items. A comments-only package that leaves the style rules net-worse in
   one clause has not closed its rows; it has moved them.
2. **MAJOR-1**: the closure publishes a count ("46 without") that the commit it
   describes refutes (78 sections added, none removed). D-291's discipline and
   the Closure rule that a claim cites its own output both bite here. The row is
   the one place the package asserts it re-measured this carefully.

MAJOR-2 alone would have been a MINOR-weight fix; it is listed as MAJOR because
the deleted sentence is the only argument in the tree for one of three live
mechanisms, in a paragraph whose whole subject is that such a mechanism without
an argument proves nothing.

Re-review is owed only on the diff that fixes these; the machine checks, the
doctest verification and the twelve re-derivations above do not need retaking
unless the fix touches them.
