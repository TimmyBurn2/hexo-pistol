# WP-1.9b — O-3, the hand-rolled probing table: REVIEW-impl

**Revision reviewed.** `3a351ea319874d8dfdcb755fde02bcb0a71f15d7`, tag
`wp19b/o3-impl`, a `git stash create` object whose parent is `e299b0e`.

**Does it match the working tree and HEAD?** The working tree MATCHES for
`crates/` — `git diff wp19b/o3-impl -- crates/` printed nothing, and
`git diff --stat wp19b/o3-impl` (all paths) also printed nothing, so the whole
stash content is present in the working tree. HEAD is `b60094e`
(`docs(wp19b): O-3's design and the pre-registration of all three runs land
before any of them is taken…`), which is a docs-only commit; `e299b0e` is an
ancestor of HEAD (`git merge-base --is-ancestor e299b0e HEAD` → YES). The five
engine/registry changes are UNCOMMITTED and staged in the index:

```
M  crates/pistol-core/src/window.rs
M  crates/pistol-eval/src/handcrafted.rs
M  crates/pistol-eval/src/lib.rs
D  crates/pistol-eval/src/window_map.rs
M  docs/rule9_justifications.md
```

**Date.** 2026-08-30.

**Reviewer discipline.** No file in the live tree was edited. Every mutation was
applied to a COPY of `crates/pistol-core` + `crates/pistol-eval` under
`…/scratchpad/mut/` with its own `CARGO_TARGET_DIR` inside that copy; the live
tree's `git status` is byte-for-byte what it was at the start of the review. No
git worktree was created.

## Commands run, with their output

| Command | Where | Outcome (quoted) |
|---|---|---|
| `cargo fmt --all --check` | live tree | no output — clean |
| `cargo clippy --workspace --all-targets --locked -- -D clippy::all` | live tree | `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 2.99s` |
| `cargo test -p pistol-eval --locked --lib` | live tree | `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` |
| `cargo test -p pistol-eval --locked` | live tree | seven binaries, every one `test result: ok.` — 8 + 6 + 3 + 4 + 2 + 9 + 7 = 39 tests, 0 failed |
| `cargo test --workspace --locked` | live tree | see **Workspace suite** below |
| `tools/file_justification_check.sh` (CI gate 17) | live tree | `file_justification_check: 329 tracked .rs/.sh files, 55 over the cap, all registered in docs/rule9_justifications.md (55 entries)` |
| `tools/artifact_check.sh` (gate 5) | live tree | `artifact_check: ok (569 tracked files, none of them artifacts)` |
| `tools/config_check.sh` (gate 6) | live tree | `validate_random_openings_config: 1 document(s) ok` |
| `tools/decision_key_check.sh` (gate 18) | live tree | `decision_key_check: 507 decision keys in docs/decisions.md, no repeat outside the exemption` |
| `tools/label_consistency_check.sh` (gate 19) | live tree | `label_consistency_check: 6 documents, 2 summand line(s), 5 summand row(s), 4 group count(s) — every document agrees with itself` |
| `cargo doc -p pistol-eval --no-deps` | scratch copy | `warning: pistol-eval (lib doc) generated 5 warnings` — all five are PRE-EXISTING at `e299b0e` (same link text), not introduced here. Not a CI gate. |

**Workspace suite.** `cargo test --workspace --locked` in the live tree, run
detached with its output captured to a file: **155 test binaries, 890 tests
passed, 0 failed, ZERO occurrences of the string `FAILED` anywhere in the log**,
ending on `all doctests ran in 0.19s; merged doctests compilation took 0.16s`.
Counted from the log's own `test result: ok.` lines rather than from any wrapper
status, per CLAUDE.md's closure rule. For context only, and not relied on here:
the implementer's own full-CI artifact `artifacts/wp19b_ci_base_v1.txt:2654`
reads `ci: all gates passed`; gates 5, 6, 17, 18 and 19 were re-run
independently for this review with their output quoted above.

## Independent instruments the reviewer built

Three, because two of the review brief's questions (B and A) cannot be settled by
reading:

1. **A verbatim transcription of `WindowMap` into a standalone Rust program**,
   fuzzed against a `std::collections::HashMap` reference: 300 seeds × 4000
   mixed insert/decrement operations over a deliberately small key space so
   clusters and wrap-around actually occur, checking after every removal that
   `live` equals the occupied-slot count, that the load factor never passes one
   half, that a free slot exists, that every reference key is reachable with the
   right counts, that the table holds no key the reference does not, and that no
   hole ever sits on a live key's probe path. Plus, per seed: a shuffled rebuild
   must compare EQUAL, a drained table must compare equal to a fresh one and hold
   no residue, and a table one entry short must compare UNEQUAL. Output:
   `FUZZ OK; largest table reached 4096 slots`.
2. **An EXHAUSTIVE model check of `remove`'s arithmetic**, copied verbatim, over
   an injected hash so every home assignment is visited: all `8^4` home
   assignments for 4 keys in 8 slots (the shipped ½ load bound) × all 4! build
   orders × all 4! deletion orders, asserting after every single removal that
   every surviving key is still reachable and that no hole sits on any live key's
   probe path. Output:
   `EXHAUSTIVE OK: 9437184 removal states checked over all 8^4 home assignments x 4! builds x 4! deletion orders`.
   This visits wrap-around clusters, clusters that reach the end of the array,
   removal of a cluster's last entry, and repeated removals — the four cases the
   brief names.
3. **An independent Python model of the placement function**, written from the
   design's §1.1/§1.2 prose, used to recompute the golden table and to sweep
   probe-step and hash mutations.

---

## VERDICT

**FAIL** — 0 BLOCKING, 3 MAJOR, 4 MINOR.

**The store itself is sound.** I could not construct a key sequence that strands
an entry, could not make two tables holding the same live entries compare
unequal, could not make two holding different entries compare equal, and could
not find an input on which this store answers differently from the `HashMap` it
replaces. `remove` is Knuth 6.4R and its shift condition is exactly right,
including the wrap-around arithmetic. Sections A, B, C, D and G of the brief are
all clean, and I say so with the instruments above rather than by inspection.

The FAIL is entirely in the **evidence layer**: two of the design's invariant
pins do not pin what §2 says they pin (I demonstrate each with a mutant that
survives the whole suite), and the rule-9 registry — the ONE home hard rule 9
gives the why, adjudicated by a CI gate — asserts a measured causal claim that
this package's own design document disclaims in as many words. All three fixes
are small and local.

---

# Findings

## MAJOR 1 — the golden placement test does not pin the hash, and the retirement of `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key` therefore loses a pin nothing else carries

**Severity.** MAJOR (a test does not pin what it claims; a design clause is
false; a registered mutation receipt has a live survivor).

**Where.** `crates/pistol-eval/src/handcrafted.rs:106-109` (the subject),
`:610-690` (the golden test), `docs/experiments/wp19b_o3_design.md:123` (the I4'
row), `:169-174` (the retirement argument), `:190` (the §4 receipt row).

**What the design claims.** I4' is *"The placement function is seedless and
**fixed** — same key, same slot count, same slot, on every machine and every
run"*, pinned by `the_table_places_a_fixed_window_set_in_a_fixed_pattern` and by
nothing else. §4 registers *"The probe step or the hash constant moves"* as a
mutation that MUST kill that test, on the ground that *"nothing else in the
workspace can see it."* §3 retires the WP-1.9 digest golden because *"the golden
placement test pins the surviving half (the fold) through the code that uses
it."*

**What is wrong.** The golden observes only the LOW SIX BITS of
`window_hash`'s output, for 24 keys, at one slot count. That is far less than
the fold. A ONE-BIT change to the multiply constant survives the entire
`pistol-eval` suite while observably relocating entries.

**MINIMAL REPRODUCER.** In a copy of the crate, change one hex digit:

```
crates/pistol-eval/src/handcrafted.rs:107
-    let mixed = key.wrapping_mul(0x9e37_79b9_7f4a_7c15);
+    let mixed = key.wrapping_mul(0x9e37_79b9_7f4a_7c95);   // bit 7 flipped
```

`cargo test -p pistol-eval --no-fail-fast` then reports, verbatim:

```
test handcrafted::tests::the_table_places_a_fixed_window_set_in_a_fixed_pattern ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 6 passed; 0 failed; ...
test result: ok. 3 passed; 0 failed; ...
test result: ok. 4 passed; 0 failed; ...
test result: ok. 2 passed; 0 failed; ...
test result: ok. 9 passed; 0 failed; ...
test result: ok. 7 passed; 0 failed; ...
```

All 39 tests pass. This is not an equivalent mutant: at the shipped 64 slots it
moves the home slot of **2916 of the 43200 windows in the ±60 coordinate box**
(6.75 %), e.g. `Window { axis: ConstQ, start: (-60, 17) }` goes from home slot 41
to home slot 46; at 128 slots `(-60, -60)` goes from 84 to 20; the divergence
persists at every table size from 64 through 8192.

The RETIRED test would have killed it. For `key = 1` the shipped fold answers
`0x9e3779b9e17d05ac` — which is literally the second row of WP-1.9's `GOLDEN`
array in the deleted `window_map.rs` — and the mutant answers
`0x9e3779b9e17d052c`. So the retirement does not merely move the pin, it drops
it.

**Scope of the gap.** A single-bit sweep of all 64 constant bits finds **30
survivors of the golden, 12 of them observably relocating entries** (bits 7, 8,
23, 24, 38-45); the remaining 18 (bits 46-63) are genuine equivalent mutants,
because a bit at that height cannot reach a mask any reachable table uses.

**Why it matters, stated without inflation.** No wrong answer follows from the
shipped code, and rule 4 is not engaged — the shipped hash is fixed and seedless.
What is lost is exactly the property WP-1.9 paid for and D-502 recorded: *a
reseeded hasher moves no search output, so `tools/determinism.sh` cannot see it
and an agreement test would pass — a golden is its only pin*. After this change
that sentence is still true and the golden is no longer a sufficient golden. The
implementer's own receipt M7 (`artifacts/wp19b_mutations_v1.txt`) used a
WHOLESALE constant replacement (`0x9e3779b97f4a7c15 → 0xff51afd7ed558ccd`), which
is a coarse mutant of the class; the fine mutant above is the one the receipt's
claim needs to survive and does not.

**What would close it.** Either widen the golden's key set / slot counts until a
one-bit constant change cannot hide in it, or — cheaper and exactly what the
predecessor had — keep a two-line digest golden over `window_hash` beside the
placement golden. `window_hash` is a private free function in this file, so
D-115 already licenses an in-source guard for it, and the retirement's stated
ground ("both pinned the `std::hash::Hasher` impl") is true of
`..._refuses_a_key_that_is_not_a_u64` but NOT of the digest test, whose subject
(the fold) survived the refactor intact.

---

## MAJOR 2 — the rule-9 registry entry states a measured causal claim that the cited run does not establish, and attributes it to an arrangement that run never contained

**Severity.** MAJOR (a doc is false, in the one document hard rule 9 makes
authoritative and a CI gate adjudicates).

**Where.** `docs/rule9_justifications.md:45`.

**The text.**

> `crates/pistol-eval/src/handcrafted.rs`: the evaluation and the store it reads
> are one artefact BY MEASUREMENT and not by taste — **the same code split across
> a module boundary was benched head to head against this arrangement and lost**
> (`artifacts/wp19_bench_inline_vs_module_v1.txt`), which is the measured reason
> rule 9 calls its cap soft.

**Both halves of the emphasised clause are wrong, and this package's own design
document says so.**

*(a) Not "the same code."* `docs/experiments/wp19b_o3_design.md:200-208` — §5,
"What this design does not decide" — reads:

> The two revisions that measurement compared differ in more than a file boundary
> — a newtype wrapper, a type parameter and a closure argument all appear on one
> side and not the other — so "the module split" names the CHANGE that was
> measured and not an isolated cause. **… Recorded so a later reader does not
> take a mechanism the run never separated.**

D-502 says the same thing. The registry entry is the later reader taking exactly
that mechanism.

*(b) Not "this arrangement."* The cited artifact's own header (verified by
reading it) is:

```
bench_delta: baseline revision 9a986c6 -> 9a986c6b05bb7c1984d507e918ee6f39d68c1311
bench_delta: candidate revision 16c6b70 -> 16c6b700a28f1d3c76d1ebc057b64c568d1115c7
```

`9a986c6` is the tag `wp19/mx-O2` — and `git ls-tree wp19/mx-O2 crates/pistol-eval/src/`
shows `error.rs eval.rs handcrafted.rs lib.rs weights.rs window.rs` and **no
`window_map.rs`**. So the run compared O-2, the `HashMap`, INLINE against O-2 in
a module. "This arrangement" — the O-3 open-addressed probing table — was in
neither side. It has never been benched at all: `docs/experiments/wp19b_bench_prereg.md`
registers the flip bench and it has not been run.

**Reproducer.** `git ls-tree wp19/mx-O2 crates/pistol-eval/src/` (no
`window_map.rs`), and `head -6 artifacts/wp19_bench_inline_vs_module_v1.txt` (the
two revisions), against `docs/experiments/wp19b_o3_design.md:200-208`.

**Why it matters.** Rule 9 makes this file the single home for the why precisely
so a justification cannot drift from the gate that adjudicates it, and the file's
own header says an entry that does no work "is a claim the gate refuses." An
entry whose load-bearing sentence is a measured cause the run did not separate is
the D-291/D-318/D-324 pattern the log has already recorded three times — a
measured-sounding cell over a run that does not support it — now inside the
paragraph that licenses a file to be four times the cap. **This is a
prose-only defect and the code is untouched by it**, which is exactly the shape
CLAUDE.md's OPERATOR OVERRULE contemplates if the operator judges the claim does
no work; but as written it is not a claim that does no work — it is a claim that
does the WRONG work, so it should be corrected rather than overruled. The honest
version is available and short: the head-to-head measured O-2 inline against O-2
in a module at 0.844/0.828 (D-502), the mechanism was not isolated, and the
inline landing of O-3 is what the registered flip bench will decide.

**A MINOR rides with it:** the entry's only citation is
`artifacts/wp19_bench_inline_vs_module_v1.txt`, a path under the gitignored
`/artifacts/` that rule 8 forbids committing and that no committed manifest
sha-indexes. The numbers themselves survive in D-502, so the chain resolves — but
the entry does not say so, and this repository has now twice recorded a
successor unable to resolve a cited path (commit `6812ddc`; MEMORY note "Review
reports live in ephemeral scratchpads"). Citing D-502 alongside the path costs
nothing.

---

## MAJOR 3 — `the_table_never_fills` does not pin I8's stated bound, and the failure mode it does guard manifests as a hang rather than a failure

**Severity.** MAJOR (a test does not pin what it claims).

**Where.** `crates/pistol-eval/src/handcrafted.rs:796-816` (the test), `:208`
(the subject), `docs/experiments/wp19b_o3_design.md:127` (the I8 row), `:47`
(the "invariant and not a tuning knob" sentence).

**What the design claims.** I8: *"The table is never full — load factor at most
one half"*, pinned by `the_table_never_fills`. §1.2: the ½ bound *"is what makes
the probe loop terminate … so it is an invariant and not a tuning knob."*

**(a) The "at most one half" half is not pinned.** The test asserts
`map.live * 2 <= map.slots.len()` ONCE, after a 64×64 insert sweep has finished.
It never looks at the load factor DURING the sweep, so the growth condition can
be off by one in the "grow too late" direction and the assert still passes.

**MINIMAL REPRODUCER:**

```
crates/pistol-eval/src/handcrafted.rs:208
-            if (self.live + 1) * 2 > self.slots.len() {
+            if self.live * 2 > self.slots.len() {
```

`cargo test -p pistol-eval --no-fail-fast` on that copy reports, verbatim:

```
test handcrafted::tests::the_table_never_fills ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

— and every one of the other seven binaries is `test result: ok.` too; 39 of 39
pass. The mutant's peak load factor is **0.5156** (33 live in 64 slots), above
the bound the design calls an invariant, while its load factor at the exact
moment the assertion fires is 0.5000, because 4096 into 8192 is the sweep's
resting point either way.

**(b) An outright I8 break hangs instead of failing.** Weaken the condition so
the table can actually fill —

```
-            if (self.live + 1) * 2 > self.slots.len() {
+            if (self.live + 1) * 2 > self.slots.len() * 2 {
```

— and `entry_or_default` spins forever inside `probe` on the 65th insert. My run
was killed at the 120 s tool timeout with the test binary still spinning; it
never prints a failure. `tools/ci.sh` has no per-test timeout, so gate 3 would
hang rather than fail. That is not a wrong answer, but a gate that hangs is a
worse signal than a gate that fails, and the design's I8 row implies a red test.

**A second, smaller point in the same test.** Both of its assertions read
`map.live` — the very counter the growth decision consumes — rather than counting
occupied slots. `live`'s agreement with reality IS cross-checked elsewhere
(`an_emptied_window_leaves_no_entry_behind` at n=1;
`the_window_table_holds_its_peak_capacity_after_every_entry_is_gone` asserts
`map.live == 64 * 64` against an independent number), so this is not a hole — but
it is why the test's second assertion (`any(EMPTY_KEY)`) is implied by its first
rather than independent of it.

**What would close it.** One assertion inside the sweep loop —
`assert!(map.live * 2 <= map.slots.len())` after each insert — kills the (a)
mutant, and a `debug_assert!` on the same expression at the top of `probe` turns
the (b) hang into a named failure. Both are one line.

---

## MINOR 4 — the golden's non-vacuity self-check certifies only the FIRST probe step, so a real alternative probing scheme produces a bit-identical golden

**Where.** `crates/pistol-eval/src/handcrafted.rs:681-689`;
`docs/experiments/wp19b_o3_design.md:139-143`, `:190`.

The self-check counts keys placed away from home and asserts the count is
positive, with the failure message *"no key in the golden set was displaced from
its home slot, so this test pins the hash and says nothing about probing."* The
implicature is that `displaced > 0` makes it pin probing. It does not: in the
shipped golden **exactly two of the 24 keys are displaced, and both by exactly
one slot** (slot 44 from home 43, slot 61 from home 60). Any probe scheme whose
first step is +1 reproduces the golden byte for byte.

**Reproducer.** Replace linear probing with triangular (quadratic) probing —
`at = (start + step * (step + 1) / 2) & mask` — in a copy. Result:

```
test handcrafted::tests::the_table_places_a_fixed_window_set_in_a_fixed_pattern ... ok
```

The mutant is nonetheless amply killed — ten tests fail, led by
`a_removal_keeps_every_other_entry_reachable` (a triangular probe makes the
linear backward shift unsound, and the constructed cluster catches it) —
so **no pin is lost**; what is inaccurate is §4's receipt row naming the golden
as the killer for "the probe step … moves". The implementer's own note against M5
already concedes that its `+1 → +2` mutant is "a coarse mutant". Adding one key
to `STARTS` that lands two or more slots from home would make the self-check mean
what its message says.

For completeness, the golden DOES pin the initial slot count as a singleton: a
`INITIAL_SLOTS: 64 → 128` mutant in a copy gives
`test handcrafted::tests::the_table_places_a_fixed_window_set_in_a_fixed_pattern ... FAILED`
with `test result: FAILED. 7 passed; 1 failed`, and nothing else fails.

---

## MINOR 5 — the removal test's constructed cluster never wraps

**Where.** `crates/pistol-eval/src/handcrafted.rs:712-744`, `:819-835`.

`home_sharing_windows` starts its search at `(0, 0)`, whose key `0x80008000`
homes at slot **47** of 64, so the three-entry cluster occupies slots 47, 48, 49
and the removal never crosses the array end. The design's §1.3 hazard paragraph
names wrap-around implicitly ("the hole walks forward"), and `remove` uses
`wrapping_sub` precisely for it, but the shipped test never exercises it.

I verified the arithmetic myself instead (instrument 2 above: all `8^4` home
assignments × 4! builds × 4! deletions, 9 437 184 removal states, every one
sound), so this is a COVERAGE note and not a defect. It is worth recording
because the cheapest strengthening is free: `home_sharing_windows` could search
for a cluster whose home is `slots.len() - 1`, and the same test body would then
cover the wrap.

---

## MINOR 6 — `the_table_never_fills`'s and the golden's stated subjects both name "the placement function" with different members

`crates/pistol-eval/src/lib.rs:29-31` says the placement function is *"the hash
and the probe step"*; `docs/experiments/wp19b_o3_design.md:131-133` defines I4'
over *"the hash AND the probe step AND the initial slot count."* The
determinism claim in `lib.rs` only needs seedlessness, so nothing false is said —
but the two definitions of one named term differ, which is the drift D-423 asks
to be resolved by stating a thing once and pointing at it.

---

## MINOR 7 — pre-existing rustdoc warnings, recorded so they are not mistaken for new

`cargo doc -p pistol-eval --no-deps` on the reviewed content emits five warnings:
one unresolved `[\`WINDOWS_PER_CELL\`]`, one redundant explicit link target, and
three "public documentation for `delta` links to private item
`WindowMap::{entry_or_default,get,update}`". **All five are present at `e299b0e`
in the same shape** (the same link text, with `set` in place of `update`), so
WP-1.9b introduces none of them, and there is no rustdoc gate in `tools/ci.sh`.
Recorded only so the next reader does not attribute them here.

---

# What I checked and found SOUND

Listed so the next reader knows the coverage, and so a later reviewer does not
re-derive it.

### A — the D-498 canonical-equality obligation: SOUND, both mechanisms real

- **Eager compaction is real.** `remove` (`:262-282`) writes `Slot::EMPTY` and
  never a marker; there is no DELETED state in the `Slot` type, so "tombstone-
  independent" holds by construction rather than by an exclusion a reader has to
  remember. `an_emptied_window_leaves_no_entry_behind` (`:693-709`) asserts
  `map.slots.iter().all(|slot| slot.key == EMPTY_KEY)`, which is the residue
  check the predecessor did not have.
- **`PartialEq` (`:315-323`) is canonical.** `self.live == other.live` plus
  "every live entry of self is in other with the same counts" is set equality
  **given** `live == the occupied-slot count`, and that identity is maintained
  correctly on every path (`entry_or_default` increments only on a real insert;
  `remove` decrements exactly once; `grow` preserves). I checked the identity
  after every operation across 300 × 4000 fuzz operations and it never drifted.
- **Neither direction fails.** Same live set built in a different order ⇒ EQUAL
  (asserted per seed against a shuffled rebuild); one entry removed ⇒ UNEQUAL
  (also asserted per seed). A drained table of up to 4096 slots compares equal to
  a fresh 64-slot one.
- **No path can leave a key in a non-live slot.** The only writers of a non-EMPTY
  key are `entry_or_default`'s insert, `remove`'s shift, and `grow`'s re-place;
  the exhaustive model confirms no hole ever sits on a live key's probe path
  after any removal sequence.
- **I independently re-took the two D-498 receipts.** A derived-shape `PartialEq`
  (zip over the slot arrays plus a length check) gives
  `test eval_apply_undo_roundtrip ... FAILED` (plus 3 more); a tombstone removal
  (`self.slots[hole].counts = Counts::default()` instead of `= Slot::EMPTY`)
  gives `test handcrafted::tests::an_emptied_window_leaves_no_entry_behind ... FAILED`
  and `test eval_apply_undo_roundtrip ... FAILED` (7 in total). Both receipts
  hold.

### B — the open-addressing defect class: SOUND, and I could not strand an entry

`remove`'s shift condition at `:275`,
`(probe.wrapping_sub(hole) & mask) <= (probe.wrapping_sub(home) & mask)`, is
exactly Knuth 6.4R. Writing `d_hole = (probe − hole) mod m` and
`d_home = (probe − home) mod m`, Knuth keeps an entry in place iff its home lies
cyclically in `(hole, probe]`, which is `d_home < d_hole`; the code moves iff
`d_hole <= d_home`, the exact complement. The `usize::wrapping_sub` then `& mask`
is a correct mod-m reduction because `m` is a power of two dividing `2^64`. The
loop's `hole` is re-read each iteration, so a moved entry correctly becomes the
new hole. The exhaustive check (9 437 184 removal states over ALL home
assignments, including every wrap-around and end-of-array configuration, every
build order and every deletion order) found no stranding. I have no key sequence
to offer, and I looked hard for one.

The off-by-one that would have been the classic bug — `<=` to `<` — IS caught:
in a copy it gives `test handcrafted::tests::a_removal_keeps_every_other_entry_reachable ... FAILED`
plus 8 more.

### C — termination and the load factor: SOUND

- **A free slot always exists.** `entry_or_default` grows when
  `(live + 1) * 2 > slots.len()`, so after any insert `live * 2 <= slots.len()`,
  i.e. free slots `= len − live >= live >= 0`, and at `len = 64` the first insert
  already leaves 63. Traced: peak load factor is exactly 0.5000, reached at
  live = 32 of 64.
- **The growth condition is NOT off by one in the dangerous direction.** At
  `live = 31` inserting the 32nd, `(31+1)*2 = 64` is not `> 64`, so it does not
  grow and lands at exactly ½; at `live = 32` it grows first. There is no way to
  insert without growing when it should have grown.
- **`remove` terminates** because at entry `live >= 1` and `len >= 2*live`, so at
  least one EMPTY slot exists elsewhere in the array; the walk from `at+1` reaches
  it before it can return to `at` (whose stale key is still in place). The fuzz
  harness carried a `probe`-iteration guard of `len + 1` and it never fired.
- **`grow` terminates**: the doubled table holds at most `old_len/2 = new_len/4`
  entries, and the key being re-placed is distinct from every key already there.

### D — the `EMPTY_KEY` sentinel: SOUND, and airtight by TYPE rather than by sweep

`Window` has two public fields: `axis: Axis`, a three-variant fieldless enum
(`crates/pistol-core/src/axis.rs:8-15`), and `start: Coord { q: i16, r: i16 }`
(`crates/pistol-core/src/coord.rs:19-24`). `window_key` therefore ranges over
`[0, 0x2_FFFF_FFFF]` for EVERY value the type admits — including values
`Window::new` would refuse, since the fields are public and the struct is
constructible directly (which is what the sweep helper does). `u64::MAX` is
outside that range unconditionally, not merely for the sweep's 243 windows.
`a_window_key_is_never_the_empty_slot_marker` (`:594-607`) asserts the structural
bound `key < 3 << 32` rather than only `key != EMPTY_KEY`, which is the right
assertion — it fails if the packing's field layout moves, not just if it happens
to hit the sentinel.

### E — the golden placement pin: the GOLDEN ITSELF IS CORRECT and independently reproduced

I re-derived all 24 `(slot, key)` pairs from the design's prose with a Python
model written without reading the array, and they match the committed `GOLDEN`
exactly, including the two displaced entries. So the golden is not a
copied-from-output tautology, and there is no way for it to pass for the wrong
reason as a HASH-VALUE record. Its weaknesses are the two scoped above (MAJOR 1:
it observes only 6 bits; MINOR 4: it observes only the first probe step) — the
*table* is right, the *coverage* is not what §2 claims. The
`assert_eq!(map.slots.len(), INITIAL_SLOTS)` guard at `:660-664` does correctly
tie the golden to the initial slot count (a smaller `INITIAL_SLOTS` grows the
table and trips that assert; a larger one changes every home and trips the
comparison).

### F — the other tests

| Test | Non-vacuous? | Evidence |
|---|---|---|
| `a_packed_key_never_collides_for_two_distinct_windows` | yes | fails on any field-overlap change |
| `a_packed_key_orders_windows_the_way_the_window_type_does` | yes | D-502 records a q/r-swap mutant killing only this one |
| `a_window_key_is_never_the_empty_slot_marker` | yes | structural bound, see D above |
| `the_table_places_a_fixed_window_set_in_a_fixed_pattern` | partially — see MAJOR 1, MINOR 4 | kills the `INITIAL_SLOTS` singleton and coarse hash/probe mutants |
| `an_emptied_window_leaves_no_entry_behind` | yes | tombstone mutant → FAILED |
| `a_removal_keeps_every_other_entry_reachable` | yes | hole-punch, `<=`→`<`, and triangular mutants all → FAILED |
| `the_window_table_holds_its_peak_capacity_after_every_entry_is_gone` | **yes, and the WP-1.9 vacuity is fixed** | it now asserts `slots.len() == peak` and `map == WindowMap::default()`; the predecessor's `capacity() >= len()` (a std invariant) is gone. It is the test that kills the derived-`PartialEq` mutant even in the unit binary. |
| `the_table_never_fills` | **no — see MAJOR 3** | |

**On the two RETIRED tests.** Retiring
`the_window_hasher_refuses_a_key_that_is_not_a_u64` is SOUND: its subject was
`Hasher::write`, `std`'s byte path is gone, and `window_hash(key: u64)` makes the
case unreachable by type, so keeping it would pin nothing.
`the_window_hasher_answers_a_fixed_digest_for_a_fixed_key` is a DIFFERENT case
and its retirement is not sound — see MAJOR 1. The design treats them as one
("both pinned the `std::hash::Hasher` impl"); the digest test's real subject was
the fold, which survived the refactor unchanged as `window_hash`.

### G — bit-identity: no input found on which this store answers differently

- The evaluation body is unchanged apart from prose. `git diff e299b0e wp19b/o3-impl -- crates/pistol-eval/src/handcrafted.rs`
  touches `apply`, `undo`, `value` and `delta` only in comments and doc text; the
  arithmetic, the iteration over `windows_through(at)`, and the accumulation into
  `p1_score` are byte-identical.
- The three operations answer identically to `std`'s. `entry_or_default` matches
  `HashMap::entry().or_default()` (both insert a default when absent and return a
  `&mut` to it); `update` matches the `Entry::Occupied` / `Entry::Vacant` match
  (both return `None` without mutating when absent, both remove when the edit
  leaves the default); `get` matches `.get().copied().unwrap_or_default()`. The
  fuzz harness asserted `update`'s full `Option<(before, after)>` return against
  the reference on every one of 1.2 M operations and it never diverged.
- **The desync paths fire on the same window, in the same order, with the same
  token.** `apply`'s check reads `before` from an entry `entry_or_default` has
  just inserted-or-found — identical to before, including the fact that a
  desync panic leaves a spurious zero-count entry behind (a panic path, not a
  value path, and unchanged). `undo`'s check lives inside the closure `update`
  runs on a COPY, so the table is unmutated when it fires — also unchanged; and
  the `None` arm's second desync fires on exactly the windows `Entry::Vacant`
  used to reach, because `probe` finds a key iff it is present. `delta`'s check
  mirrors `apply`'s over `get`, and `get`'s "absent and emptied are ONE
  observation" premise now rests on eager compaction, which is precisely what the
  rewritten `delta` doc says.
- The eval is a `Box<dyn Eval>` built once at `crates/pistol-engine/src/instance.rs:140`
  and never cloned on a search path (`Position` owns it; `reset_to` unwinds), so
  the design's §1.2 claim that eager allocation of 64 slots is off every hot path
  is TRUE — I checked rather than assumed, because a per-node clone of a 4096-slot
  `Vec` would have been a real regression the bench might not have separated.

### H — rules and style

- **Hard rule 3.** No silent fallback. `get`'s `Counts::default()` for an absent
  window is the contract (an absent window holds no stones), not a swallowed
  error; both desync tokens are unchanged and both still panic.
- **Hard rule 4.** Nothing nondeterministic reaches move choice. The hash is
  seedless; slot arrangement is a deterministic function of the key sequence.
  `Debug` (`:330-341`) does iterate the slot array — I checked whether it is
  reachable from any value: `git grep` over `pistol-search`, `pistol-engine` and
  `pistol-cli` finds the eval formatted nowhere (`pistol.rs:164` prints
  `config.eval.backend.token()`, a config string). Its only consumers are
  assertion messages. `PartialEq` also walks the slots but its RESULT is
  order-independent, which is the property that matters.
- **Hard rule 9.** The registry has exactly one entry for `handcrafted.rs`, it
  states no line count, `window_map.rs`'s entry is gone with its file, and gate 17
  passes with 55/55 registered. The entry's TRUTH is MAJOR 2.
- **Hard rule 10.** No new ADR is owed by the implementation: D-501 fixed the flip
  trigger's terms in advance and the selection ADR is the bench's, not this
  package's. Worth stating plainly for the record: the code is landed in the tree
  as a CANDIDATE, and design §5 is explicit that if the flip bench does not flip,
  this storage is not what lands.
- **Style.** `cargo fmt --all --check` clean, `cargo clippy … -D clippy::all`
  clean. No file-top narrative header (the file opens on `use`); no `//!` outside
  the crate root; comments are WHY-shaped throughout. Every public item carries a
  `///`.
- **Stale docs — the WP-1.9 defect class is CLOSED.** `WindowMap::set` no longer
  appears anywhere; `delta`'s equivalence doc now names `entry_or_default`, `get`
  and `update`, all of which exist. `crates/pistol-eval/src/lib.rs` and
  `crates/pistol-core/src/window.rs` were both retargeted. `git grep` for
  `window_map` / `WindowHasher` across `crates`, `tools`, `configs` and
  `Cargo.toml` returns exactly one hit, and it is unrelated:
  `crates/pistol-solver/tests/threat_oracle_tests.rs:237 fn window_map_ordering_is_unobservable()`,
  a solver test about its own sets. Every remaining mention in `docs/` is in a
  historical WP-1.9 document describing the past, or in WP-1.9b's own design
  describing what it replaced.

### I — design promised vs code delivered

Every test §3 tables is present and named as promised; the three operations keep
their signatures and their one-probe contract (`update` still resolves the slot
once and edits through it — no `set` was reintroduced); the map is concrete over
`Counts` as §1.5 requires; a fresh table allocates eagerly as §1.2 requires;
`EMPTY_KEY` lives in the key field with no second control array as §1.1 requires;
deletion is compaction with no marker as §1.3 requires. The design carries no
measured number (D-483 satisfied — I grepped it). I found nothing the code does
that the design does not cover.

---

# Summary of what should change before this lands

1. **Restore a digest pin for `window_hash`** (or widen the golden until a
   one-bit constant change cannot hide in it), and correct §3's retirement
   paragraph, which is currently false about which subject the digest test
   pinned. — MAJOR 1
2. **Correct `docs/rule9_justifications.md:45`** so it does not attribute an
   isolated cause to a run that did not separate one, and does not attribute the
   measurement to an arrangement it never contained. Cite D-502 beside the
   gitignored artifact path. — MAJOR 2
3. **Move `the_table_never_fills`'s load assertion inside the sweep**, and add a
   `debug_assert!` in `probe` so an I8 break fails instead of hanging. — MAJOR 3
4. Optionally: one more `STARTS` key displaced by ≥2 slots (MINOR 4), and a
   wrap-around cluster in the removal test (MINOR 5). Both are one line and both
   would make the receipts in §4 true as written.

None of these touch the store. The store is right.
