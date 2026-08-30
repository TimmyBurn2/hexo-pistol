# WP-1.9b LANDING — REVIEW-impl

Fresh-context reviewer. I did not write this code. Attack report.

---

## Header

| | |
|---|---|
| **Revision reviewed** | `1703d25b3fa72e775dd1c75e78eacd2cea8fcacd`, tag `wp19b/landing`, a `git stash create` object |
| **Its parent** | `3e004e1424af616dc8aa50e71005a5385d1e1519` on `dev` |
| **Matches the working tree for `crates/`?** | **YES.** `git diff wp19b/landing -- crates/` printed nothing, checked twice — at the start of the review and again after the mutation work (`\| wc -l` → `0`) |
| **Matches HEAD?** | **NO, and it is not meant to.** HEAD is `3e004e1`; the change is STAGED in the index and not committed. `git status --short` shows `M crates/pistol-eval/src/handcrafted.rs`, `M crates/pistol-eval/src/lib.rs`, `D crates/pistol-eval/src/window_map.rs`, `M docs/rule9_justifications.md` in the index column |
| **Docs moved under me during the review** | Yes, and it is recorded rather than smoothed over. At review start `docs/decisions.md` and `docs/ROADMAP.md` were clean against HEAD; twelve minutes later `git diff HEAD --stat` showed `docs/ROADMAP.md \| 21 +-` and `docs/decisions.md \| 4 +` (D-506, D-507). `docs/experiments/wp19b_o3_design.md` in the tree carries an 11-line outcome banner the tag does not. `docs/experiments/wp19b_bench_results.md` is **untracked** — see MINOR 3 |

**Receipts for what I actually read** (`sha256sum`, 2026-08-30T18:11:24+02:00):

```
77b619382edd5b33188c89021a5ca230f121c8ace361807edb2f9f3d25be8a0c  crates/pistol-eval/src/handcrafted.rs
b630ebacd4bb829bd28a359440b31cd48645d10ed1d159c8a9a61f8fafddcd3e  crates/pistol-eval/src/lib.rs
abbce62ba57dcd8ee38c90a2b6e5faf8097d2b29f7eb5793b2ec90bd7729883e  crates/pistol-core/src/window.rs
99f3197799c610fcc770f40f47a3f893a3a2a2f8bf81ae79a6733477be5f25fc  docs/rule9_justifications.md
c9e829b7786289394587178d5f93163d2449515b62a6dd1d009083e07b569c82  docs/experiments/wp19b_bench_prereg.md
0794a40e5a077273653595e844b4b3678b85b115b962b075eced3c706ff4226c  docs/experiments/wp19b_bench_results.md
```

### Commands run, with their own output

**`cargo fmt --all -- --check`** — redirected to a file so the exit status is the formatter's and not a pipe's:

```
FMT_EXIT=0
0 …/fmt.txt          (zero bytes of complaint)
```

**`cargo clippy --workspace --all-targets --locked -- -D clippy::all`**:

```
    Checking pistol-arena v0.0.1 (/home/tom/Projects/HeXO-AlphaBeta/crates/pistol-arena)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.83s
```

No warning line of any kind.

**`tools/file_justification_check.sh`** — the gate's own last two lines:

```
file_justification_check: over the cap and registered: crates/pistol-eval/src/handcrafted.rs
file_justification_check: 329 tracked .rs/.sh files, 55 over the cap, all registered in docs/rule9_justifications.md (55 entries)
```

Exit 0. Note the gate reads the **index**, not the worktree (its own header says so at `tools/file_justification_check.sh:36-45`), so this verdict is against the staged landing.

**`cargo doc -p pistol-eval --no-deps --locked`, landed revision** (run in a scratch copy on `/home`):

```
warning: redundant explicit link target
  --> crates/pistol-eval/src/lib.rs:40:54
warning: `pistol-eval` (lib doc) generated 1 warning
```

**Same command at `3c9e28b`** (`git archive 3c9e28b` into a second scratch dir):

```
warning: unresolved link to `WINDOWS_PER_CELL`
   --> crates/pistol-eval/src/handcrafted.rs:200:48
warning: public documentation for `delta` links to private item `WindowMap::entry_or_default`
   --> crates/pistol-eval/src/handcrafted.rs:204:46
warning: public documentation for `delta` links to private item `WindowMap::get`
   --> crates/pistol-eval/src/handcrafted.rs:205:58
warning: unresolved link to `WindowMap::set`
   --> crates/pistol-eval/src/handcrafted.rs:207:24
warning: redundant explicit link target
  --> crates/pistol-eval/src/lib.rs:40:54
warning: `pistol-eval` (lib doc) generated 5 warnings
```

**FACT, not a finding: this change CLOSES four of the five**, including the `WindowMap::set` unresolved link that was WP-1.9's stale-doc defect. It introduces none. The surviving one is at `lib.rs:40` and predates the change untouched (MINOR 6).

**`tools/ci.sh`** — run detached in the live tree against the staged landing, all 19 gates, **the gate's own final line and not a wrapper's exit status**:

```
=== gate  1/19: cargo fmt --all --check
=== gate  2/19: build from the git-tracked file set
=== gate  3/19: cargo test --workspace --locked
=== gate  4/19: cargo clippy --workspace --all-targets -- -D clippy::all
=== gate  5/19: artifact rejection
=== gate  6/19: config validation
=== gate  7/19: perft oracle
=== gate  8/19: tactical fixture at its pre-registered threshold
=== gate  9/19: cross-process determinism
=== gate 10/19: differential search oracle
=== gate 11/19: staged generator soundness (four parts)
=== gate 12/19: solver oracle (four gates)
=== gate 13/19: solver determinism
=== gate 14/19: movetime ceiling on the D-95 reproducer class
=== gate 15/19: arena self-match smoke
=== gate 16/19: sealbot anchor platform suite
=== gate 17/19: file-justification check
=== gate 18/19: decision-key uniqueness
=== gate 19/19: carve-document label consistency

ci: all gates passed
```

Gate 3 alone ran **942 tests, 0 FAILED**. Gate 17's own summary line:

```
file_justification_check: 329 tracked .rs/.sh files, 55 over the cap, all registered in docs/rule9_justifications.md (55 entries)
```

Gate 9 is `tools/determinism.sh`, the hard-rule-4 self-test, and it passed inside the run above. The full log is `sha256 cc4e9eadfd57e54a3098d804f045bbc21c93f51b4a777c101af4e78fab857761`, left at `/home/tom/wp19b-review-scratch/ci_landing_receipt.txt` for the operator to export into `artifacts/` if it is wanted as a receipt; I did not write into the live tree.

### Where my mutation work ran

Not in the live tree, and not on `/tmp`. `rsync -a --exclude=target --exclude=.git --exclude=artifacts --exclude=sessions` into `/home/tom/wp19b-review-scratch`, with its own `CARGO_TARGET_DIR` inside that copy. A second copy at `/home/tom/wp19b-review-base` from `git archive 3c9e28b` for the doc-warning comparison. The live tree was never edited; the only file I created in it is this report.

---

## VERDICT

**PASS** — **0 BLOCKING, 1 MAJOR, 8 MINOR.**

The store is right. I inlined-vs-module diffed every one of the three call sites against WP-1.9's module version, byte-compared all four panic-message literals, and could not construct an input on which the landed store answers differently. All three of D-502's registered singleton mutants still die on exactly one test each after the move, and the two rewritten container tests both kill the property they name — I broke each one in a scratch copy and watched them fail.

The one MAJOR is a false sentence in the results document, repeated into an ADR line that is **not yet committed** — so it can still be fixed for free rather than amended later.

---

## Findings

### MAJOR 1 — "on independent runs a day apart" is false: the two runs are 3h13m apart on the same day

**Where.** `docs/experiments/wp19b_bench_results.md:122`, and again — verbatim as "on runs a day apart" — in `docs/decisions.md:1082` (D-507), which is **uncommitted in the working tree** (`git show HEAD:docs/decisions.md | grep -c "D-507"` → `0`).

**What it says.**

> the run answers 1.171 against a predicted 1.185 and 1.205 against a predicted 1.208 — 0.014 and 0.003 away, on independent runs a day apart.

**Why it is wrong.** The "predicted" pair is the inverse of the head-to-head in `artifacts/wp19_bench_inline_vs_module_v1.txt`. Both artifacts were produced on 2026-08-30, three hours and thirteen minutes apart:

```
$ stat artifacts/wp19_bench_inline_vs_module_v1.txt | grep -E "Modify|Birth"
Modify: 2026-08-30 14:54:13.907274807 +0200
 Birth: 2026-08-30 14:53:26.021267166 +0200

$ stat artifacts/wp19b_bench_landing_v1.txt | grep -E "Modify|Birth"
Modify: 2026-08-30 18:07:09.050591316 +0200
 Birth: 2026-08-30 18:06:20.125483412 +0200
```

The sentence is load-bearing in the sense that matters to a reviewer: temporal separation is the only independence claim the composition check makes. Three hours on the same machine in the same working day is a weaker separation than a day, and the document asserts the stronger one.

**Against which rule.** CLAUDE.md's Process section — a number or a provenance claim in a reading document is checked against the artifact it cites, and D-374's "registered numbers never move" has its counterpart in "reported facts are the ones the artifact carries". Also D-423: the claim is made twice (results §3 and D-507), so it is one defect in two places.

**What survives correction.** Everything substantive. The two runs ARE independent — separate `bench_delta.sh` invocations, separate throwaway worktrees, separate `--release --locked` builds, and the head-to-head's candidate side (`16c6b70`) is not even the landing's baseline binary lineage. The composition still closes to 0.014 / 0.003. **Only the qualifier is false.**

**Minimal reproducer.**

```bash
stat -c '%n %y' artifacts/wp19_bench_inline_vs_module_v1.txt artifacts/wp19b_bench_landing_v1.txt
# both dated 2026-08-30; delta 3h13m, not a day
```

**Fix.** Strike "a day apart" (or replace with "three hours apart, on separately built binaries") in `wp19b_bench_results.md:122` and in D-507 before `docs/decisions.md` is committed. D-507 is append-only once landed.

---

### MINOR 1 — the ONE-PROBE contract in `undo` is pinned by no test; only the bench can catch its regression

**Where.** `crates/pistol-eval/src/handcrafted.rs:215-248`. The comment at `:216-220` asserts:

> ONE probe, not two: the entry resolves the slot once and every edit below goes through it, where a lookup then a store would hash each window twice … Measured, and the reason this is not spelled as a get and a set.

**What I found.** I replaced the `Entry::Occupied` body with an honest two-probe `get` then `insert`/`remove`, preserving both desync messages and the removal rule exactly. **Every test in the crate still passes:**

```
########## MUTANT: M5_two_probe_undo
test result: ok. 6 passed; 0 failed …   (lib, incl. all six store guards)
test result: ok. 6 passed; 0 failed …   (eval_delta_tests)
test result: ok. 3 passed; 0 failed …   (eval_incremental_tests)
test result: ok. 4 passed; 0 failed …   (eval_invariant_tests)
test result: ok. 2 passed / 9 passed / 7 passed …
########## END M5_two_probe_undo
```

**Why it is only MINOR.** This is not a regression this change introduces — WP-1.9's `WindowMap::update` had the same exposure, and D-502 records that the two-probe form is exactly what REVIEW-impl caught last time and cost 0.003/0.024. But the module version at least made the one-probe operation a NAMED API with a doc saying "do not reintroduce a `set`"; inline, the shape is a `match` a future refactor can flatten with nothing red to stop it. The bench is the only detector, and a bench is not run on every change.

**Minimal reproducer.** In a scratch copy, replace the `let (before, after) = { … }` block in `undo` with the two-probe form and run `cargo test -p pistol-eval --locked --no-fail-fast`; nothing fails.

**Suggested disposition.** Accept as a named residue, or add one line to the comment pointing at the bench artifact so the next reader knows no test guards it. Do not manufacture a probe-counting test for it.

---

### MINOR 2 — the rule-9 entry cites a document it does not name

**Where.** `docs/rule9_justifications.md:45`:

> The numbers live in D-502 and in this file's own landing bench, both of which resolve without a path under the gitignored `artifacts/`.

**What is wrong.** `D-502` resolves — a reader greps `docs/decisions.md`. "This file's own landing bench" names nothing: not `docs/experiments/wp19b_bench_results.md`, not `docs/experiments/wp19b_bench_prereg.md`, not D-507. The clause asserts that BOTH citations resolve; one of them is not a citation.

**Why it matters.** The entry's whole job is to survive its author. The memory of this package's own history is explicit that a handoff a successor cannot resolve is the defect it has already hit twice (commit `6812ddc`: *"the closure handoff lands on a tracked path, because a session summary a successor cannot resolve is the defect this package already hit twice"*).

**Fix.** Name the document: `docs/experiments/wp19b_bench_results.md` §3, or D-507.

---

### MINOR 3 — the landing bench's reading document is UNTRACKED at the reviewed revision

**Where.** `docs/experiments/wp19b_bench_results.md`.

```
$ git status --short docs/experiments/wp19b_bench_results.md
?? docs/experiments/wp19b_bench_results.md
$ git diff --stat 3e004e1 wp19b/landing | grep -c bench_results
0
```

`git stash create` does not capture untracked files, so the tag `wp19b/landing` — the revision this review, D-507 and the rule-9 entry all point at — **does not contain the document that reads the landing bench.** A reviewer handed only the tag cannot see it; I read it from the worktree, which is why my receipt above carries its sha256 and a timestamp.

**Fix.** `git add` it in the landing commit. Same class as MINOR 2 and the same precedent (`6812ddc`).

---

### MINOR 4 — the footprint test's sweep floor is 19x looser than the value it guards

**Where.** `crates/pistol-eval/src/handcrafted.rs:494-498`:

```rust
let live = eval.windows.len();
assert!(
    live > WINDOWS_PER_CELL,
    "{live} windows is too thin a peak to bound a footprint against"
);
```

**Measured.** I instrumented the test in the scratch copy:

```
MEASURED live=347 capacity_at_peak=448
MEASURED capacity_after_removal=427 WINDOWS_PER_CELL=18
```

So the assertion is `347 > 18`. The predecessor asserted the exact count (`assert_eq!(live, 64 * 64)` at `3c9e28b:crates/pistol-eval/src/window_map.rs`), which fails if the sweep degenerates at all; the replacement only fails if the sweep loses more than 94 % of its entries.

**Why only MINOR.** The test's NAMED property — the map declines to shrink — is genuinely pinned (see MINOR-free section below, mutant M2). The floor is a sanity guard on the fixture, not the property, and it is not vacuous (a `windows_through` that yielded one window per cell would give `live = 64`, still passing, but an `apply` that inserted nothing gives `live = 0` and it fires). It is simply much weaker than what it replaced, and the loss is not noted anywhere.

**For the record, the real assertion has comfortable margin:** `capacity_after_removal (427) >= live (347)` is 1.23x, not a knife edge.

---

### MINOR 5 — the unit-test weight fixture copies the operator-confirmed committed table verbatim

**Where.** `crates/pistol-eval/src/handcrafted.rs:376-391`, `test_weights()`: `1 = 2, 2 = 12, 3 = 60, 4 = 300, 5 = 1500`. That is `configs/eval_v0_weights.toml`'s `[table]` byte for byte — the block the config marks `OPERATOR-CONFIRM`.

**Why it is worth a line.** The tests are structural: nothing in either of them reads a weight, and the values could be `1..=5`. Copying the confirmed table puts a second, unmarked copy of operator-confirmed numbers inside `src/`, where a Stage-4 SPSA retune will silently diverge from it. The sibling fixture does this correctly — `crates/pistol-eval/tests/eval_delta_tests.rs:97-115`'s `steep_weights()` deliberately differs in one entry and its doc says why.

**Not a rule-1 breach.** Rule 1 bans a *code-side default*; this is a `#[cfg(test)]` fixture, not a fallback, and `Weights::parse` is public API (`crates/pistol-eval/src/weights.rs:58`). The test comment's justification for parsing rather than loading a path — *"reading a path would make them about the deployment layout as well"* — is correct and is the right call for a unit test inside `src/`.

**Answering the question directly: does the inline document weaken anything the tests claim?** No. Both tests assert on `len()`, `is_empty()` and `capacity()`, none of which depends on a weight. The committed table's own parse is already pinned by `eval_weights_committed_table_holds_the_operator_confirmed_values`.

---

### MINOR 6 — `cargo doc -p pistol-eval` is not clean (one surviving warning, pre-existing)

`crates/pistol-eval/src/lib.rs:40`, `redundant explicit link target`: `[`EVAL_DESYNC`](handcrafted::EVAL_DESYNC)`. Present identically at `3c9e28b`. `lib.rs` IS a file this change edits, so a one-character fix would have cost nothing; it is untouched. Four warnings closed, one left.

---

### MINOR 7 — the workspace emits a release-only build warning that no gate sees

```
$ cargo build --release --locked -p pistol-solver
warning: unused import: `generate_turns`
 --> crates/pistol-solver/src/policy.rs:1:44
warning: `pistol-solver` (lib) generated 1 warning
```

`generate_turns` is used only inside a `#[cfg(debug_assertions)]` block at `crates/pistol-solver/src/policy.rs:262-266`, so the debug build is clean (`cargo build --workspace --locked` → 0 warning lines) and CI never sees it.

**Not this package's defect** — `git show 3c9e28b:crates/pistol-solver/src/policy.rs | head -1` is the same line — but it is reported here for two reasons. It appears twice inside **this package's own governed artifact**, `artifacts/wp19b_byte_identity_run_v1.log`, once per side of the identity leg, so an artifact meant to demonstrate a clean identity run carries two compiler warnings in its header. And it appears **inside CI's own log**, at gate 9, where the determinism script builds release:

```
=== gate 9/19: cross-process determinism
determinism: building the engine (release, locked)
warning: unused import: `generate_turns`
 --> crates/pistol-solver/src/policy.rs:1:44
```

No gate refuses it, because gate 1 is `fmt`, gate 4 is `clippy` on the **dev** profile, and gate 2 builds the tracked file set without `-D warnings`. It is a standing hole, not a WP-1.9b hole — material for WP-1.10's `tools/` hardening rather than for this landing.

---

### MINOR 8 — the rule-9 entry attributes a rationale to rule 9 that rule 9 does not state

`docs/rule9_justifications.md:45`: *"…because a module boundary was MEASURED to cost, and rule 9 calls its cap soft for exactly that."* CLAUDE.md rule 9 reads *"Single responsibility, ~300-line soft cap; exceeding requires a why-justification entry"* — it never says why the cap is soft. The claim is unfalsifiable decoration on an otherwise carefully hedged paragraph. Under CLAUDE.md's own overrule test (a clause that changes no conclusion is deleted rather than refined), delete the half-sentence.

The same paragraph's framing of the file as a "store's half" plus an "invariant guards" rest omits the evaluation itself, which is the majority of the file — but the entry's first sentence presupposes the evaluation, so I do not raise it as a separate finding.

---

## What I checked and found SOUND

### A — semantic identity with the predecessor: I could not break it

**The strongest single fact, and it is better than the claim the package makes.** The landed non-test code is not merely *equivalent to* WP-1.9's module version — it is **the very revision that was measured at 1.783 / 1.909**. `git show 9a986c6:crates/pistol-eval/src/handcrafted.rs` (`wp19/mx-O2`, the matrix's inline O-2) already carries:

```
142:    windows: HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>,
186:        let counts = self.windows.entry(window_key(window)).or_default();
207:        let Entry::Occupied(mut slot) = self.windows.entry(window_key(window)) else {
```

`diff -u` of that file against the landed one shows, outside doc text and the appended `mod tests`, exactly four changes: `Counts::is_empty` deleted and its one call site spelled `after.total() == 0`; the `type WindowMap = …` alias introduced and used at the field and in `new`; the `window_key` / `WindowHasher` items reordered; and `#[inline]` dropped from `Hasher::write` (the panic path, never called). **The arithmetic, the iteration, the entry API and both desync branches are byte-identical to the revision the comparand number was taken on.** That is what makes the landing bench's composition reading (1.783 / 0.844 ≈ the measured 1.171) a statement about the same code rather than an analogy.

- **`apply`.** `WindowMap::entry_or_default(window)` was `entries.entry(window_key(window)).or_default()`. The landed line 196 is that expression. Identical, including the fact that a desync panic leaves a spurious zero-count entry behind (a panic path, not a value path, unchanged from both predecessors).
- **`delta`.** `WindowMap::get(window)` was `entries.get(&window_key(window)).copied().unwrap_or_default()`. The landed lines 304-308 are that expression. Identical.
- **`undo`.** `WindowMap::update` matched `Entry::Occupied` / `Entry::Vacant`, ran `edit` on a **copy**, and wrote back or removed. The landed body matches the same `Entry`, checks the same predicate against the same `before`, mutates through `slot.get_mut()`, and removes through the same resolved slot. The `Entry::Vacant` arm never inserted in the predecessor and does not here (a `VacantEntry` is dropped unused in both).
- **Desync order.** In the predecessor a vacant window returns `None` **without running the closure**, so the "holds nothing" message wins; an occupied window with zero stones of `player` hits the in-closure check. The landed code checks vacancy first, then `before.of(player) == 0`. Same order, same first-failing window (`windows_through` is unchanged, and `apply`/`undo`/`delta` all walk it identically).
- **Messages.** I collapsed Rust's `\`-newline continuations and compared the literals mechanically. All four are byte-identical:

```
OLD/NEW: "{player} stone taken off {at}, but the window at {} along {:?} holds nothing"
OLD/NEW: "{player} stone taken off {at}, but the window at {} along {:?} holds no {player} stone"
OLD/NEW: "{player} stone on {at} would make {} stones in the {WINDOW_LEN}-cell window at {} along {:?}"
OLD/NEW: "a hypothetical {player} stone on {at} would make {} stones in the {WINDOW_LEN}-cell window at {} along {:?}"
```

The hasher message differs only in spelling — the module version could not use an inline capture and passed `crate::handcrafted::EVAL_DESYNC` positionally; the landed one writes `{EVAL_DESYNC}`. Both render the same bytes, and both are pinned by `the_window_hasher_refuses_a_key_that_is_not_a_u64`.

- **`after.total() == 0` vs `after == Counts::default()`: EQUIVALENT, and I checked both ways.** `Counts` is `{p1: u8, p2: u8}` and `total()` is `p1 + p2`. Wrapping to zero would need `p1 + p2 == 256` exactly, which `apply`'s own guard (`before.total() >= WINDOW_LEN_STONES` → desync, `WINDOW_LEN_STONES == 6`) makes unreachable — that guard is itself pinned by `eval_apply_that_overfills_a_window_panics`. Under `total() <= 6`, `total() == 0 ⟺ p1 == 0 && p2 == 0 ⟺ Counts::default()`. Empirically, substituting `after == Counts::default()` back in leaves the entire suite green (mutant M4: 6/6/3/4/2/9/7 all `ok`). **`total()` cannot overflow or wrap on any reachable state**, in debug (where `+` would panic) or release (where it would wrap).
- **ONE probe held.** `HashMap::entry` hashes once and resolves the bucket; `get`, `get_mut` and `remove` on the resulting `OccupiedEntry` all act on that bucket pointer with no second hash. No `set` was reintroduced. (That the property is untested is MINOR 1.)
- **`Counts::is_empty` is gone and nothing wants it.** `git grep` over the workspace finds no `Counts::is_empty`; the only `is_empty()` hits in `pistol-eval` are on `str`, `Vec`, `Board` and `HashMap`. Clippy is silent, which it would not be if a private method were dead.

### B — determinism (hard rule 4): clean

- `type WindowMap = HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>` at `:139`. `BuildHasherDefault` constructs `WindowHasher(0)` every time; there is no `RandomState`, no seed, nothing from the clock or the environment.
- **Every shipped use of `self.windows`** is a point lookup: `:196` (`entry`), `:222` (`entry`), `:305` (`get`). Nothing iterates. The test-only uses (`:458`, `:464`, `:479`, `:494`, `:503`, `:512`, `:515`) are `len`, `is_empty`, `capacity`.
- The derived `PartialEq` on `HandcraftedV0` does reach the map, and `HashMap`'s `PartialEq` iterates one side — but its RESULT is length-plus-per-key-lookup and therefore order-independent, which is exactly what D-498 narrowed the licence to. The type alias's own doc at `:135-138` states this correctly.
- `crates/pistol-eval/src/lib.rs:31-33` still says, unqualified, *"nothing in this crate iterates the map on a path that reaches a value"*. Strictly, `PartialEq` and `Debug` iterate; neither result depends on order, and D-498 already records that the conclusion survives. The sentence is unchanged by this package (the only `lib.rs` edit is deleting `mod window_map;`), so I do not raise it — but a future editor should prefer the alias doc's phrasing.
- Nothing reads a clock, a thread count or an env var. `tools/determinism.sh` runs as CI gate 12 and passed (see VERDICT).

### C — the two rewritten container tests: BOTH pin what they claim, proven by breaking them

Mutations applied in `/home/tom/wp19b-review-scratch` (never the live tree), each against the pristine file, `cargo test -p pistol-eval --locked --no-fail-fast`:

| Mutant | What I changed | Result |
|---|---|---|
| **M1** | deleted `if after.total() == 0 { slot.remove(); }` from `undo` | **`an_emptied_window_leaves_no_entry_behind` FAILED** at `:460`, plus `the_window_map_holds_its_peak_capacity_after_every_entry_is_gone`, `eval_apply_undo_roundtrip`, `eval_incremental_matches_from_scratch_on_random_playouts`, `eval_windows_stop_at_the_edge_of_the_addressable_lattice` — **5 dead** |
| **M2** | added `self.windows.shrink_to_fit()` at the end of `undo` | **only `the_window_map_holds_its_peak_capacity_after_every_entry_is_gone` FAILED** at `:512` — a singleton, exactly as D-502 recorded for the module version |
| **M3** | `new()` builds the map with `with_capacity_and_hasher(64, …)` | **only the footprint test FAILED**, at `:479` — the `capacity() == 0` line |
| **M4** | `after.total() == 0` → `after == Counts::default()` | all green (equivalence, see A) |
| **M5** | `undo` rewritten as `get` then `insert`/`remove` | all green (MINOR 1) |
| **M6** | `q` and `r` swapped in `window_key` | **only `a_packed_key_orders_windows_the_way_the_window_type_does` FAILED** at `:409` — singleton, as D-502 recorded |
| **M7** | hasher seeded with a constant xor | **only `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key` FAILED** at `:434` — singleton, as D-502 recorded |

**All three of D-502's registered singletons survive the move to the inline file, and each still kills exactly one test.** That is the strongest evidence that the guards were carried across rather than re-written into something weaker.

Answering the specific questions put to me:

- *If `undo` stopped removing the emptied entry, does `an_emptied_window_leaves_no_entry_behind` fail?* **Yes — M1.** And so do four integration tests, so the removal of WP-1.9's in-test `assert_eq!(map, WindowMap::default())` costs nothing: D-498's canonical-equality obligation is carried by its own named driving test, `eval_apply_undo_roundtrip` at `crates/pistol-eval/tests/eval_incremental_tests.rs:118-140`, which asserts `eval == fresh` after both a reverse and a rotated unwind, and which M1 kills at `:127`.
- *If the map started shrinking on removal, does the footprint test fail?* **Yes — M2, and only it.**
- *Is `assert_eq!(eval.windows.capacity(), 0)` on a fresh map a real property or an accident of `HashMap::default()`?* **A real property, and M3 proves it.** It is not the `capacity() >= len()` std-invariant vacuity the previous reviewer named: it constrains `HandcraftedV0::new`, and a constructor that pre-sized the table — a plausible future optimisation given the measured 120-386 window occupancy — fails it immediately.
- *Do the tests pin their own reimplementation?* **No.** Both drive the shipped `Eval::apply` / `Eval::undo`; neither inserts or removes an entry itself. That is the previous reviewer's defect class, and it is closed here — as the rule-9 entry's last sentence claims, correctly.

### D — rule 9 and the registry

One entry, for `crates/pistol-eval/src/handcrafted.rs`, at `docs/rule9_justifications.md:45`. `window_map.rs`'s entry is gone with its file, so the gate's staleness arm has nothing to refuse. The entry states **no line count** — the count-ban regex at `tools/file_justification_check.sh:129` finds nothing, and the gate agrees. The clauses I could check against evidence are true:

- *"a module boundary was MEASURED to cost"* — `artifacts/wp19_bench_inline_vs_module_v1.txt`, `nps ratio 0.844` / `0.828`, `VERDICT ABORT` on the script's own bracket. ✅
- *"its two sides differ in more than that boundary — a newtype, a type parameter and a closure argument"* — **verified directly.** Baseline `9a986c6` has a bare `HashMap` field and an `Entry::Occupied` `undo`; candidate `16c6b70` has `struct WindowMap<V>` with `update(window, edit: impl FnOnce(&mut V))`. All three named differences are real, and the narrowing is honest. ✅
- *"the defect this log has recorded three times (D-291, D-318, D-324)"* — all three exist and D-500 uses the same framing verbatim. ✅
- *"the map's refusal to shrink on removal, which is the property a footprint bound rests on"* — M2. ✅
- *"the hasher's seedless digest, which nothing else in the workspace can catch"* — M7 killed only the golden test; the byte-identity leg would not move under a reseed. ✅
- *"The two guards that read the map drive the shipped `apply` and `undo` rather than the container"* — true by inspection. ✅

The previous reviewer's MAJOR 2 (an isolated cause asserted for a run that did not separate one) is **closed**: the entry now says explicitly that the run "measured the CHANGE and did not isolate the boundary as the cause".

### E — stale documentation

- **`git grep -n window_map`** over the repo returns no live code reference to the deleted module. The hits are: `crates/pistol-eval/src/handcrafted.rs:471` (the test's own name), `crates/pistol-solver/tests/threat_oracle_tests.rs:237 fn window_map_ordering_is_unobservable()` (a solver test about its own sets, unrelated), and `docs/` files describing WP-1.9 or WP-1.9b history. `git grep -n WindowMap` in `crates/` returns only the three lines of the new type alias.
- Every doc comment in `handcrafted.rs` names a live item. `delta`'s equivalence paragraph was retargeted from `WindowMap::{entry_or_default,get,set}` to `apply` / `undo` and the ambient "entry"/"lookup", and — importantly — keeps the sentence explaining WHY it names operations at all (D-214 accepted the equivalence in terms of two sites that no longer exist). `[`WINDOWS_PER_CELL`]` was fixed to `[`crate::WINDOWS_PER_CELL`]`, which is why one `cargo doc` warning closed.
- `crates/pistol-core/src/window.rs` is unchanged by this package; its `Window` doc already points the determinism argument at "where that store lives", which is now true of a file rather than a module — still correct.
- The `#[inline]` on `Hasher::write` present at `9a986c6` is absent here and at `3c9e28b`; it is the panic path, so it is not a perf change.

### F — the bench and identity claims: every number is in the artifact it cites

| Claim in `wp19b_bench_results.md` | Artifact | Verified |
|---|---|---|
| dry run 1.195 / 1.242, exit 0 | `wp19b_bench_dryrun_v1.txt` | `nps ratio 1.195` / `nps ratio 1.242`, `EXIT=0` ✅ |
| referent 1.198 / 1.242 | `wp19_mx_bench_O1_fmt_v1.txt` | `nps ratio 1.198` / `1.242`, baseline `a5c5661`, candidate `abf3d5d` ✅ |
| flip 1.518 / 1.594, IQR 1804 / 1827 on 367590 / 318676 nps | `wp19b_bench_flip_v1.txt` | `1.518` / `1.594`; `candidate median 367589.7 (IQR 1804.2)`, `318675.6 (IQR 1826.8)` ✅ |
| flip t2d 1.538 (dev 0.020) / 1.586 (dev 0.009) | same | verbatim ✅ |
| landing 1.171 / 1.205, t2d 1.226 (0.055) / 1.238 (0.032) | `wp19b_bench_landing_v1.txt` | verbatim, `EXIT=0` ✅ |
| landing candidate is `1703d25b…` | same | `candidate revision 1703d25b… -> 1703d25b…` ✅ |
| binaries `ddbae8f3…` vs `b66f88c9…`, differ | `wp19b_byte_identity_v1.txt` | verbatim ✅ |
| 88 searches, 422 lines, 88 bestmoves, 0 errors, digest `0b1cb805…` | same | verbatim ✅ |
| "the same digest WP-1.9's own leg recorded" | `wp19_byte_identity_v2.txt` | `digest of both: 0b1cb8054857e8a4a877297733d284b23efaeaad8ccd76f0a6a65d34b5512edf` — identical ✅ |
| §2.1's three-way table 1.783/1.909, 1.508/1.579, 1.518/1.594 | D-501, D-502, this run | ✅ |
| grounds 1/0.844 = 1.185 and 1/0.828 = 1.208 | `wp19_bench_inline_vs_module_v1.txt` | `0.844` / `0.828`; inverses check ✅ |

**No bracket moved, and I hunted for it.** The prereg was committed at `b60094e` (2026-08-30 **17:11:26**) and amended once at `3e004e1` (**17:17:45**). The first governed run's artifact was born at **17:54:06**. `git diff b60094e 3e004e1 -- docs/experiments/wp19b_bench_prereg.md` touches **only** §1.3's dry-run rep count (`1` → `5`) and adds the amendment paragraph; it changes nothing in §1.1, §2, §2.1 or §3. `git show b60094e:…` carries the landing bracket **`[1.10, 1.30]`** and the comparand **1.783 / 1.909** already, identical to the current text. `git diff 3e004e1 -- docs/experiments/wp19b_bench_prereg.md` is empty, so the prereg has not been touched since. The amendment is genuinely stricter (5 reps costs more than 1) and its stated cause is verifiable: `tools/bench_delta.sh:136` does read `[ "$REPS" -ge 5 ] || fail "REPS must be an integer >= 5 (pre-registered)"`.

**A concurrency worry I raised and then dismissed on data.** The three artifacts' birth→modify spans are 65 s, 61 s and 49 s, which cannot be whole `bench_delta.sh` runs including two `--release` builds — so the mtimes do not measure run duration and cannot be used to prove the runs were sequential. The measurements themselves settle it: the dry run and the flip run share the baseline binary `a5c5661`, and answered `244363.6` / `200284.9` and `242103.7` / `199886.1` nps for it — 0.9 % and 0.2 % apart, and both within 2 % of another session's `248089.0` / `204445.6` for the same binary. A contended machine does not reproduce a baseline to 0.2 %.

**One instrument-fidelity note, in the package's favour.** The landing bench ran against a `git stash create` object, and `bench_delta.sh` printed its own warning — `NOTE candidate revision 1703d25b… is a git stash create commit — files never git add'ed are NOT in it`. The `crates/` content IS staged, which is why `git diff wp19b/landing -- crates/` is empty, so the binary measured is the code under review. (What is *not* in the object is the results document — MINOR 3.)

### G — rules, style, gates

- **Hard rule 3.** Both `undo` desync branches diverge through `desync(…) -> !`; `Hasher::write` panics and is pinned; `value` and `delta` use `.expect` on a clamp that cannot fail. No silent fallback anywhere. `get`'s `Counts::default()` for an absent window is the contract, not a swallowed error.
- **Hard rule 7.** Tests are behaviour-named, deterministic, no wall-clock waits.
- **Hard rule 9.** 55/55 registered, gate 17 green.
- **Hard rule 10.** No new ADR is owed by the code; D-507 is the closure line and it is where MAJOR 1's fix belongs.
- **Hard rule 11.** `pistol-api` untouched.
- **Style.** No file-top narrative header (the file opens on `use`); no `//!` outside the crate root; every private item that needed one carries a `///`; comments are WHY-shaped. `rustfmt` and `clippy -D clippy::all` both clean.
- **The O-3 work is correctly OFF `dev`.** `git rev-parse wp19b/o3-fixed` → `6ea88b2bad1008ce87d262639fd999b23f20f718`, tagged and not an ancestor of the landing. `docs/experiments/wp19b_o3_design.md` carries its outcome banner in the worktree (though not in the tag — see the header table). I did not review O-3 and make no recommendation about it.
