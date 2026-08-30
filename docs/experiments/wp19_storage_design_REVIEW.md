# WP-1.9 storage DESIGN — REVIEW-design (round 1)

**Target:** `docs/experiments/wp19_storage_design.md` (revision 1).
**Named revision reviewed:** `2cda4f5d8c760874dfbf14b8c68a47116c410015`, branch `dev`.
**Match to HEAD:** YES — `git rev-parse HEAD` = `2cda4f5d8c760874dfbf14b8c68a47116c410015`.
**Tree state:** clean at the first check; by the second check the concurrently
implementing session had added one untracked file,
`crates/pistol-eval/src/window_map.rs`, and edited `crates/pistol-eval/src/lib.rs`.
By the time the report was written the session had also modified
`crates/pistol-eval/src/handcrafted.rs`. **Every file:line citation below is
pinned to `2cda4f5`** (verified with `git show 2cda4f5:<path>`), not to the drifting
working tree.

**That implementation is NOT the review target and was not read as one** — every
finding below is taken against the design text and against the committed code at
`2cda4f5`.

**VERDICT: FAIL.**

The mechanism survives and so does the package's central technical claim: the
packed key is injective and order-preserving over the whole addressable range,
and Track E holds (see §T and §R). What fails is the layer that turns the design
into work. Three of the nine rows in §3 name tests that do not exist under those
names or present an already-committed test as new; two of the four mutation
receipts in §5 therefore point at nothing; one invariant (I7) asserts a mechanism
the code does not have and cannot be given inside the registered scope; one test
(I4) cannot fail for the failure it is registered against; and one dispatch
deliverable is absent outright. An implementer following this document would
write hollow tests, duplicate accepted ones, or breach scope.

The findings are enumerated so one fix round closes them. None of them touches
§1's mechanism or §4's claim.

---

## Findings

### BLOCKING — none

No finding names a way the proposed code produces a wrong answer. Track E is not
flipped by this review.

---

### MAJOR 1 — §3 marks two tests "ALREADY EXISTS" under names that exist nowhere in the tree, and presents a third already-committed test as new. §5's receipts inherit the error.

**Claim attacked.** §3 rows 3, 4 and 7:

| row | design's name | design's status |
|---|---|---|
| 3 | `an_unwound_eval_equals_a_fresh_one_whatever_order_it_is_taken_back_in` | "ALREADY EXISTS (`eval_incremental_tests.rs`)" |
| 4 | `a_probe_leaves_no_trace_in_the_carried_state` | "ALREADY EXISTS (`eval_delta_tests.rs`)" |
| 7 | `an_incremental_eval_agrees_with_a_rebuild_at_every_step` | listed with no "ALREADY EXISTS" mark |

**Command run.**

```
$ /usr/bin/grep -n "^fn " crates/pistol-eval/tests/eval_incremental_tests.rs
29:fn eval_incremental_matches_from_scratch_on_random_playouts() {
94:fn eval_apply_undo_roundtrip() {
144:fn eval_is_independent_of_the_order_stones_were_applied() {

$ /usr/bin/grep -rn "an_unwound_eval_equals_a_fresh_one\|a_probe_leaves_no_trace\|an_incremental_eval_agrees_with_a_rebuild" crates/
(no output)
```

The real tests are `eval_apply_undo_roundtrip` (`eval_incremental_tests.rs:94`,
whose rotated-unwind block at `:118-140` is the one D-498 names by `file:line`)
and `delta_leaves_the_eval_indistinguishable` (`eval_delta_tests.rs:396`).

Row 7 is worse than misnamed. `eval_incremental_matches_from_scratch_on_random_playouts`
(`eval_incremental_tests.rs:29-91`) already does exactly what row 7 describes,
including both properties row 7 emphasises in capitals:

```
crates/pistol-eval/tests/eval_incremental_tests.rs:44
        // A playout that stumbles into a completed line stops there: a decided
        // game accepts no further stone (rule 4). The eval still saw the winning
        // stone, and still has to be right about it.
```

and it drives every ply through `GameState`, so turn-1 single stones are the
game's own rule-3 handling, not something a new test must add. It checks against
`value_from_scratch` on the way in AND on the way back, and asserts the unwind
equals a fresh eval.

**Consequence.** (a) An implementer either creates three duplicate tests or
renames three accepted, D-498-cited tests — a rename is a change to the pinned
referent of a landed ADR line and is not something a design may request silently.
(b) §5 receipt 1 ("Break the undo path … must kill
`an_unwound_eval_equals_a_fresh_one_whatever_order_it_is_taken_back_in`") names a
test that does not exist, so the receipt is not executable as written. (c) §5
receipt 2 ("Skip one axis … must kill `an_incremental_eval_agrees_with_a_rebuild_at_every_step`")
would in fact be killed by the *existing* `eval_incremental_matches_from_scratch_on_random_playouts`,
so as a receipt for a new test it is uninformative — which is precisely the defect
the dispatch's own question about receipts asks about.

**Fix.** Name the three tests by the identifiers in the tree; mark row 7 ALREADY
EXISTS; and state for each new-vs-existing row whether the package extends it or
leaves it untouched.

---

### MAJOR 2 — I7 asserts a `newgame` clear the code does not have, and the only way to give it one is a search change the registered scope forbids.

**Claim attacked.** §2 I7: "`newgame` **clears** the carried state". §3 row 9:
"Engine-level: state after `newgame` is indistinguishable from a fresh engine."
§5 receipt 3: "Skip the `newgame` clear".

**Command run.**

```
$ /usr/bin/grep -n -A4 "fn new_game" crates/pistol-engine/src/instance.rs
73:    fn new_game(&mut self) {
74:        self.state = GameState::new_game();
75:        self.searcher.clear();
76:    }

$ sed -n '194,203p' crates/pistol-search/src/search.rs
    pub fn clear(&mut self) {
        self.table.clear();
        self.heuristics.clear();
        // Wholesale (design wp18b §1): epoch isolation already makes
        // earlier solves read as absent, so this is memory hygiene and
        // defence-in-depth, stated once and honestly there.
        if let Some(solver) = self.solver.as_mut() {
            solver.reset();
        }
    }
```

`Searcher::clear` never touches `self.position`, and `Position` is what owns the
`Box<dyn Eval>` (`crates/pistol-search/src/search.rs:65`,
`crates/pistol-search/src/position.rs:13`). **There is no newgame clear of the
eval to skip.** What actually empties the eval is `Position::reset_to`
(`crates/pistol-search/src/position.rs:55-70`), which unwinds stone by stone at
the head of every search:

```
crates/pistol-search/src/position.rs:56
        let stones: Vec<(Coord, Player)> = self.state.board().stones().collect();
        for (at, player) in stones {
            self.eval.undo(at, player);
        }
```

**Consequence.** The invariant is real — the eval must not carry game *n* into
game *n+1* — but it is discharged by the unwind contract (I5 plus the `Eval`
trait's inverses clause), not by `newgame`. As written, an implementer either
adds a clear to `Searcher::clear`/`Position` — a change to `pistol-search`, which
the dispatch's scope paragraph rules out ("no search changes", `wp19_storage_DISPATCH.md:88`)
— or writes a test named for a mechanism that does not exist, which passes for
the wrong reason. The mutation receipt cannot be made at all. Note also that this
is *not* a property the container swap puts at risk: the reproducer in §R shows a
`HashMap` drained to zero entries compares equal to a fresh one, so the
observable is unchanged either way.

**Fix.** Restate I7 as what it is — "an eval unwound to empty is
indistinguishable from a fresh one, and that is what carries across a game
boundary, because `Position::reset_to` unwinds rather than reconstructs" — cite
the two sites, and replace the receipt with one that can be made (e.g. suppress
the emptied-entry removal and show the cross-game seat of `tools/determinism.sh`
still reading identical, which is the honest statement that this invariant's
guard is I5's test and not a newgame seat).

---

### MAJOR 3 — the I4 test cannot fail for the failure I4 is registered against. A reintroduced process-wide seed passes it, and nothing else in the tree would catch one.

**Claim attacked.** §2 I4: "The hasher is **seedless by construction** … No
`RandomState`, nothing environment-derived, so two runs of the same position hash
identically on any machine." §3 row 5:
`the_window_hasher_carries_no_seed_between_constructions` — "Two independently
constructed hashers give the same digest for the same key."

**Command run.** Standalone crate on real disk (`/home/tom/wp19_review_repro`,
removed after the run), a hasher seeded once per process from the clock:

```rust
static SEED: OnceLock<u64> = OnceLock::new();
impl Default for Seeded { fn default() -> Self { Seeded(*SEED.get_or_init(|| ...subsec_nanos()...)) } }
```

Output:

```
SEEDED hasher: two independent constructions agree in-process: true (digest 0x418017143d120d2d)
```

**Consequence.** The registered test compares two hashers *within one process*. A
seed that is per-process rather than per-instance — a `OnceLock`, a `static mut`,
a lazily-initialised value read from the environment or the clock — is identical
for both, so the assertion passes while the invariant is dead. And the second
line of defence is absent too: `tools/determinism.sh` diffs bestmove, nodes and
pv across two processes, but nothing on the value path iterates the map (§4's own
argument, which §T confirms), so a varying hash seed changes no search output and
the determinism gate reads green. **I4 would have no pin at all.**

**Fix.** Pin a GOLDEN DIGEST — assert `WindowHasher::default()` fed a named
literal key finishes at a literal `u64` written into the test. That is the only
form that fails for every seed, per-instance or per-process. Keep the
two-constructions assertion if desired; it is not sufficient on its own.
(Registering a literal digest in the *test* is not a measured number in a
*design*, so D-483 is untouched — the design registers the test's shape, not the
value.)

---

### MAJOR 4 — the dispatch deliverable "interaction with `Eval::delta` (D-220) stated with quoted sites" is absent, and it is the one call site the new API shape can silently change.

**Claim attacked.** §3's "What is NOT re-tested" paragraph disposes of `delta` in
one sentence: "`delta`'s equivalence to the roundtrip … already pinned and
accepted (D-214, D-220). This package must not break them and does not restate
them."

**Command run.**

```
$ sed -n '92,97p' docs/experiments/wp19_storage_DISPATCH.md
Design decides and records, from the memo: storage layout; incremental
update and UNDO paths (the undo path is where incremental state rots:
it gets its own tests and its own mutant); rebuild-from-scratch path as
the internal oracle; interaction with Eval::delta (D-220) stated with
quoted sites; memory footprint bound stated as a number with its
derivation in a test, not prose (D-483).
```

**Consequence.** The dispatch asks for quoted sites and gets none. This is not
bookkeeping: `delta` is the only one of the three call sites that reads the map
*without* the `Entry` API, and the accepted equivalence argument in
`crates/pistol-eval/src/handcrafted.rs:214-217` is stated in terms of the two
concrete `std` methods —

```
crates/pistol-eval/src/handcrafted.rs:214
    /// The `before` values are equal too: `entry().or_default()` reads the
    /// same counts `get().copied().unwrap_or_default()` reads, and the empty
    /// entry `apply` inserts is removed again by `undo`, so the roundtrip
    /// leaves no residue for this body to miss.
```

— both of which the design replaces with unnamed `WindowMap` operations (§1's
"exactly the three operations the eval performs"). The design never says what
those three operations are, so it never says whether that quoted argument still
type-checks as an argument. See MINOR 1.

**Fix.** Add the deliverable: quote the three call sites (`handcrafted.rs:130`,
`:151`, `:226`), give the `WindowMap` operation each becomes, and state in one
line why `handcrafted.rs:214-217`'s equivalence argument survives the rename.

---

### MAJOR 5 — the doc-comment correction is scoped to one sentence and leaves two others asserting the property the swap removes. One of them is the crate-level determinism claim.

**Claim attacked.** §1: "**The `windows` field doc comment is corrected, not
carried.**" That discharges selection-record condition 3 as literally worded
(round-1 m2 named the field comment). It is not the only site.

**Command run.**

```
$ git grep -n "BTreeMap\|no hasher\|ordered map" -- crates/pistol-eval/src crates/pistol-core/src/window.rs
crates/pistol-core/src/window.rs:29:/// derived, and deterministic, which is what lets the bookkeeping live in an
crates/pistol-core/src/window.rs:30:/// ordered map with no hasher anywhere near a value the engine plays on
crates/pistol-eval/src/handcrafted.rs:1:use std::collections::BTreeMap;
crates/pistol-eval/src/handcrafted.rs:84:    /// The windows holding at least one stone, and what they hold. Ordered, so
crates/pistol-eval/src/lib.rs:30:    windows: BTreeMap<Window, Counts>,
```

and the crate root, `crates/pistol-eval/src/lib.rs:26-31`:

```
//! # Determinism
//!
//! Integer arithmetic throughout, no interior mutability, no hasher: the window
//! bookkeeping is a `BTreeMap`, so no iteration order in this crate can differ
//! between two runs of the same position (CLAUDE.md rule 4, D-7, D-32). Nothing
//! here reads a clock, a thread count, or an environment variable.
```

**Consequence.** After the swap, "no hasher" and "the window bookkeeping is a
`BTreeMap`" are both false, and the *reason* the determinism conclusion still
holds changes from "ordered container" to "seedless hasher, and nothing iterates
on a value path" — which is exactly the substitution §1 makes for the field
comment and does not make here. Leaving it is the same defect round-1 m2 caught,
in the crate's most-read doc. `pistol-core/src/window.rs:29-30` is the second
instance; it is a `pistol-core` sentence describing a `pistol-eval` choice, so
its repair is a one-line trim rather than a rewrite, but it must be named.

**Fix.** Extend §1's correction paragraph to name all three sites.

---

### MAJOR 6 — I6's test row contradicts I6's own test name, and in the shape the row states, the test cannot fail.

**Claim attacked.** §3 row 8, `the_window_map_footprint_is_bounded_by_its_live_entries_and_its_peak`:
"Asserts the bound as a computed number from the map's own capacity and entry
size — the derivation is the test body, not prose."

**The contradiction.** The NAME says the bound is in terms of *live entries and
peak* — a claim about the workload, which can fail. The SHAPE column says the
bound is computed *from the map's own capacity* — a claim about the map's
arithmetic against itself, which cannot. `bytes <= capacity * size_of::<(u64, Counts)>() + overhead`
is true of any capacity the map happens to hold, including a pathological one; it
asserts that multiplication works. The dispatch's requirement (`wp19_storage_DISPATCH.md:95-96`, "memory
footprint bound stated as a number with its derivation in a test, not prose") is
satisfied by the vacuous form only in letter.

**Consequence, sharpened by the selection record's own measurement.** The
selection record's §1 records that O-2 "retains ~66.6 % of peak bytes at zero
entries". So a bound in *live* entries is FALSE for this shape, and the only
non-vacuous bound is one in PEAK entries. Worse, `Position::reset_to`
(`crates/pistol-search/src/position.rs:55-70`) unwinds rather than reconstructing
the eval, and `Pistol::new_game` never reconstructs it either (MAJOR 2), so the
eval object lives for the whole process and its peak is a **process-lifetime**
high-water mark across every game and every `set_position`, not a per-search one.
That is affordable on the selection record's absolute-magnitude ground, but the
design must say which peak it means or the test's constant is unanchored.

**Fix.** Pick one and make the other a pointer: assert `capacity <= f(peak live
entries)` with `f` written out, drive the map to a stated peak, drain it, and
assert the retained footprint against `f(peak)` — a test that fails if the
container's retention behaviour changes. Say "peak", not "live entries and its
peak".

---

### MINOR 1 — §1 never says what `WindowMap`'s three operations are, and the `undo` path is the one where a coarser API silently costs a probe against the measurement the whole selection rests on.

**Claim attacked.** §1: "exposing exactly the three operations the eval performs,
keyed by `Window` so no caller outside this module ever handles a packed key" and
"The bodies keep their shape".

**What is at stake.** At `wp19/mx-O2` — the revision whose 1.783/1.909 IS the
selection's one ground — `undo` uses a single `Entry::Occupied` probe that reads,
mutates and removes:

```
$ git show wp19/mx-O2:crates/pistol-eval/src/handcrafted.rs | sed -n '207,226p'
                let Entry::Occupied(mut slot) = self.windows.entry(window_key(window)) else { ... };
                let before = *slot.get();
                ...
                slot.get_mut().remove(player);
                let after = *slot.get();
                if after.is_empty() { slot.remove(); }
```

A `WindowMap` that exposes `get` / `insert` / `remove` instead turns that into two
or three probes. **The tag is inline; the design's module split is not the
measured shape** (the dispatch's own framing). Nothing in §1 forbids the coarser
API, and the design registers no obligation to preserve the probe count.

**Consequence.** Not a correctness gap — the answers are the same either way, so
Track E is untouched — but the §4.3 bracket would then be taken against a
structure nobody measured, and a regression against `wp19/mx-O2` would be
indistinguishable from noise in the design's own record. Also, whether the desync
checks in `apply`/`undo` stay in `handcrafted.rs` (where `at` and `player` are in
scope for the message) or migrate into the module is undetermined, and the design
asserts "the bodies keep their shape" without saying which.

**Fix.** Write the three signatures. State that the `undo` path stays a single
probe, and that the desync checks stay in `handcrafted.rs` (the messages name
`at` and `player`, which the storage module has no business knowing).

---

### MINOR 2 — §1's key formula, taken literally, is not injective over negative coordinates. The working reference has a narrowing cast the design drops.

**Claim attacked.** §1 item 1: "`window_key(Window) -> u64` — `axis << 32 | (q ^ 0x8000) << 16 | (r ^ 0x8000)`".

`q` and `r` are `i16` (`crates/pistol-core/src/coord.rs:21-23`). The reference at
`wp19/mx-O2` narrows first:

```
$ git show wp19/mx-O2:crates/pistol-eval/src/handcrafted.rs | sed -n '127,129p'
    let q = u64::from((window.start.q as u16) ^ 0x8000);
    let r = u64::from((window.start.r as u16) ^ 0x8000);
```

The design's text omits `as u16`. Widening a negative `i16` straight to `u64`
sign-extends and the fields stop being disjoint. Reproducer output:

```
naive key(axis=0,q=-1,r=0)  = 0xffffffff7fff8000
naive key(axis=2,q=-1,r=0)  = 0xffffffff7fff8000
naive collide (0,-1,0)==(2,-1,0): true
```

**Consequence.** The axis field is annihilated for any negative `q`; three
distinct windows share one key, which is I1's own stated failure mode ("a wrong
evaluation with no panic and no symptom"). It is MINOR only because the sweep in
§3 row 1 includes the extremes and would catch it, and because the reference
implementation is right. It is on the record because the design is the document
an implementer follows and this is the exact "negative coordinates" trap.

**Fix.** Write the formula with the narrowing cast, or say in words that each
field is a 16-bit unsigned quantity occupying bits `[0,16)`, `[16,32)`, `[32,34)`.

---

### MINOR 3 — §5's key-collapse receipt does not discriminate I1 from I2, so the one test §2.1 argues hardest for has no receipt of its own.

**Claim attacked.** §5 row 4: "Collapse the packed key so two windows share one →
must kill `a_packed_key_never_collides_for_two_distinct_windows`."

Any collapse that makes two distinct windows share a key also breaks
order-preservation (`a < b` while `key(a) == key(b)` falsifies "`key(a) < key(b)`
exactly when `a < b`"), and it merges two windows' counts, which falsifies the
incremental-vs-rebuild test and probably the unwind test as well. So the receipt
kills three or four tests at once and says nothing about the one it names — which
is the dispatch's stated criterion for an uninformative receipt.

§2.1 goes out of its way to argue that I1 rather than I2 is "the operative
property"; §5 then gives it no receipt that isolates it. The isolation cannot be
had from a collapse, because §2.1's own reasoning is right that I2 ⟹ I1 (see §R).
It can be had from the other direction: a key that is injective but NOT
order-preserving — swap the `q` and `r` fields, or use a multiplicative hash of
`(axis, q, r)` — kills `a_packed_key_orders_windows_the_way_the_window_type_does`
alone and leaves everything else green. That mutation is the one that shows the
two tests are two tests.

**Fix.** Add it. Keep the collapse receipt if desired, but say which tests it is
expected to kill together.

---

### MINOR 4 — reusing `EVAL_DESYNC` for the hasher's byte path makes the token stop discriminating the invariant its own doc defines, and drags the eval's vocabulary into the module §1 justifies by saying a hasher is not the evaluation.

**Claim attacked.** §1 item 2: the hasher "refuses everything else by panicking
with the crate's named `EVAL_DESYNC` token". §3 row 6 pins that.

`EVAL_DESYNC`'s home defines a different thing:

```
crates/pistol-eval/src/handcrafted.rs:19
/// Named invariant: the eval was told about a stone that contradicts what it
/// already holds — a cell applied twice, or a stone taken back that was never
/// applied.
```

A hasher fed bytes is neither. Three committed tests match the token by substring:

```
$ git grep -n "should_panic(expected = \"EVAL_DESYNC\")" -- crates/
crates/pistol-eval/tests/eval_invariant_tests.rs:9:#[should_panic(expected = "EVAL_DESYNC")]
crates/pistol-eval/tests/eval_invariant_tests.rs:22:#[should_panic(expected = "EVAL_DESYNC")]
crates/pistol-eval/tests/eval_invariant_tests.rs:31:#[should_panic(expected = "EVAL_DESYNC")]
```

Secondarily: `window_map.rs` must then import `EVAL_DESYNC` from `handcrafted`,
which is the storage module depending on the evaluation module — mild friction
against §1's own ground for the split ("a hasher and a key packing are not the
evaluation"). Rule 3 asks for a named error per kind; this reuses a name across
two kinds without recording the trade.

**Fix.** Either give the module its own token, or amend `EVAL_DESYNC`'s doc to
cover the second kind and say in §1 that the design chose reuse and why. Note the
panic is unreachable in production — `impl Hash for u64` calls `write_u64` and
nothing else — so this is defence-in-depth naming, not a live path.

---

### MINOR 5 — §4.2 says "gate-off byte-identity" in a design that has no gate (§1.1). The two sections describe different comparisons.

"Gate-off byte-identity" is D-495's and WP-1.5d(A)'s phrase for *the new feature
disabled reproduces the old binary*. With no gate (§1.1's own decision), there is
no "off". What §4.2 actually describes — "two `--release --locked` binaries built
in their own detached worktrees and compared directly" — is a baseline-vs-candidate
diff, which is a STRONGER claim than gate-off identity, since the new container is
live in the candidate binary. The design should say so rather than inherit a phrase
whose premise it has removed; as written an implementer could look for a gate to
turn off.

---

### MINOR 6 — D-423 / CLAIM-HOME (D-331): the Track-S flip rule is stated twice, and §4 restates a measured result whose home is D-501 and the selection record.

Two instances.

(a) §1.1: "the correct response is not a gate: it is the mechanical flip to Track
S that D-495 requires." §4: "A single mismatch anywhere in 1 or 2 flips the
package to Track S mechanically, no discretion, recorded (D-495)." §4 owns the
rule; §1.1 restates rather than points. D-423 in CLAUDE.md is explicit: "state it
once, in the section that owns it, and have every other section point there
instead."

(b) §4's "Evidence already in hand": "node identity held per position under both
budgets in every rep for all three benched candidates." That claim's home is
D-501 / `matrix_wp19_storage_selection.md` §1. Under D-331 ¶2 every other
occurrence is a pointer that names WHERE and does not repeat WHAT. §4 repeats the
what and cites no artifact — which is also the shape D-483 asks for when a design
touches a measured result at all. The paragraph's *point* (that node identity is
evidence and not proof) is the design's own and belongs here; the restated result
does not.

**Fix.** In (a), have §1.1 point at §4. In (b), keep the "evidence, not proof"
sentence and replace the restated result with a reference to the selection
record's §1 row.

---

### MINOR 7 — the I-list has no entry for `Debug`, which is the one observable the swap actually changes.

`HandcraftedV0` derives `Debug` (`crates/pistol-eval/src/handcrafted.rs:81`), so
`WindowMap` must implement it or the derive stops compiling — the design does not
mention it. And unlike `PartialEq`, `HashMap`'s `Debug` is NOT canonical.
Reproducer, two maps with identical contents inserted in opposite orders:

```
PartialEq order-independent: true
Debug(a)==Debug(b) with same contents, different insertion order: false
```

**Consequence is diagnostic, not behavioural**, and it is bounded: the only
places `HandcraftedV0`'s `Debug` renders are the failure messages of
`assert_eq!(eval, fresh, …)` at `eval_incremental_tests.rs:140` and
`eval_delta_tests.rs:407`. Nothing on a value path and nothing in
`pistol-search`/`pistol-engine` can reach it (see §R). It belongs on the I-list
as the one place I3's canonicality does not extend, together with the note that a
`WindowMap: Debug` rendering packed `u64`s instead of `Window`s makes those two
assertion messages harder to read than the ones they replace.

---

## §T — The Track E claim

**It holds.** §4's "by construction" argument is correct, and it is correct for a
stronger reason than §4 gives: the container is not merely un-iterated on the
value path, it is **unreachable** outside `pistol-eval`'s own test crate.

1. `value` reads the running scalar and never the map
   (`crates/pistol-eval/src/handcrafted.rs:179-187`).
2. `apply`, `undo` and `delta` iterate `windows_through(at)` — a function this
   package does not touch (`crates/pistol-core/src/window.rs:110-118`) — and
   perform one point operation per window. Accumulation order into `p1_score` is
   therefore fixed by the enumeration, not by the container. Unchanged.
3. The desync panics fire on the same first window for the same reason: the check
   is inside the per-window loop body and the loop order is the enumeration's.
   The `Entry` API question the dispatch raises is answered by that: `Entry` is
   how a window is *reached*, not *which* window is reached. Swapping
   `btree_map::Entry` for `hash_map::Entry` cannot reorder the loop.
4. Every consumer outside `pistol-eval` holds the eval as `Box<dyn Eval>`
   (`crates/pistol-engine/src/instance.rs:140`,
   `crates/pistol-search/src/position.rs:13`), and `Eval`
   (`crates/pistol-eval/src/eval.rs:44`) has **no supertrait** — no `Debug`, no
   `PartialEq`, no `Clone`. The concrete type's derives are invisible to
   `pistol-search`, `pistol-engine` and `pistol-cli`.
5. `PartialEq` — reachable only from `pistol-eval`'s tests — is canonical for
   `HashMap`, order-independent AND capacity/history-independent. Verified rather
   than assumed:

```
PartialEq order-independent: true
drained==fresh: true  drained.capacity=6700 fresh.capacity=0
```

   A map grown to 4096 entries and drained to zero compares equal to a fresh one
   despite retaining 6700 slots of capacity. That is I3 and I5's driving case,
   and it is the property D-498 requires.

**The strongest reason to doubt it, stated plainly.** Not iteration — `Debug`
(MINOR 7). `HashMap`'s `Debug` renders in bucket order, so two evals holding the
same stones can print differently. It is the single observable the swap changes.
It flips nothing: it is reachable only from the failure branch of two assertions
inside `pistol-eval`'s test crate, it cannot influence a move choice, and it
cannot differ between two runs of the same history. **Track E survives.** The
second-strongest is MINOR 1 — not a doubt about identity of *answers*, but about
whether the module-split shape is the shape the 1.783/1.909 ground was measured
on, which is a §4.3 bracket concern rather than a Track E one.

---

## §R — What reproduced or held

Stated so the record shows what passed.

- **I1 (injectivity) HOLDS, and I2 (order-preservation) HOLDS**, for the packing
  as implemented at `wp19/mx-O2`. `Coord` derives `Ord` lexicographically with
  `q` declared before `r` and the field order called load-bearing
  (`crates/pistol-core/src/coord.rs:14-24`); `Axis` derives `Ord` in declaration
  order `ConstQ < ConstR < ConstS` matching the discriminants `0 < 1 < 2`
  (`crates/pistol-core/src/axis.rs:7-15`); `Window` derives `Ord` over
  `(axis, start)` (`crates/pistol-core/src/window.rs:31-37`). The key is
  `(axis, q+32768, r+32768)` in three disjoint bit fields. Reproducer over the
  full `i16` `q` range × all three axes:

  ```
  ref key strictly increasing along the (axis,q) walk: true
  ref key fields disjoint at all extremes: true
  ```

- **§2.1's reasoning is CORRECT.** For a total order, `a < b ⟺ key(a) < key(b)`
  gives `a ≠ b ⟹ key(a) ≠ key(b)`; I2 implies I1, exactly as the design says.
  And the design is right that injectivity is what a `HashMap` needs while
  order-preservation is not — declining condition 2's literal wording while
  pinning both properties, and recording the reasoning in a numbered subsection
  rather than dropping it, is the right disposition of that condition.

- **§1.1's refusal of a feature gate is SOUND, and is not a hedge.** The dispatch
  conditions the gate on the design finding one meaningful, so declining is
  compliant on its own terms. The substantive grounds check out: a storage-backend
  knob is a hard-rule-1 config surface requiring a schema entry and a default in
  every committed config; the dispatch's "default = incumbent behaviour" would put
  the *slow* container on the default path, so the §4.3 bracket and every future
  strength claim would be taken in a shape the operator does not run; and the
  revert path for a three-site diff behind an unchanged trait is a single-commit
  `git revert`, which is strictly cheaper than a permanent second code path. The
  post-landing case §1.1 does not spell out does not change this: if the harness
  ran clean and a mismatch appeared later, the harness was insufficient, and a gate
  would not have caught it either.

- **I3 and I5 HOLD** — verified against `std`'s `HashMap`, above and in §T.5.

- **The `Eval` contract's observation surface is closed**, verified by
  `git grep -n "HandcraftedV0" -- crates/` over the whole workspace: every
  construction outside `pistol-eval`'s tests immediately coerces to
  `Box<dyn Eval>`, and the trait has no supertrait.

- **D-483 HOLDS.** Every numeral in the design is a structural constant, an
  identifier or a cross-reference. The only borderline is "the 115-position
  receipt set" (§4.2), which is a fixture cardinality quoted from
  `wp19_storage_DISPATCH.md:104`, not a measurement. No bracket, no nps figure, no
  memory figure appears. This is a clean instance of the discipline D-483 exists
  to install.

- **Rule 9 HOLDS and condition 1 is carried.** `handcrafted.rs` is 255 lines at
  `2cda4f5`; the selection record measured O-2 inline at 315, over the ~300 soft
  cap with no entry in `docs/rule9_justifications.md`
  (`/usr/bin/grep -n "handcrafted" docs/rule9_justifications.md` returns nothing
  for the source file). A module is rule 9's own remedy of first resort and the
  design takes it.

- **Rule 1 HOLDS.** §6's "No config surface changes" is consistent with the whole
  design; nothing proposed reads config or introduces a tunable.

- **Selection-record conditions 1, 3 (in part), 4 and 5 are carried**, and
  condition 2 is declined *with its reasoning recorded* in §2.1 rather than
  silently dropped — which is the disposition CLAUDE.md's process asks for.
  Condition 3 is carried in substance but under-scoped on the doc-comment half
  (MAJOR 5).

- **The stale field comment is real and still present at both candidate
  revisions**, as the selection record says (round-1 m2):
  `crates/pistol-eval/src/handcrafted.rs:84` at HEAD, and
  `git show wp19/mx-O2:crates/pistol-eval/src/handcrafted.rs` line 140 carries the
  same sentence. §1's decision to correct rather than carry it is correct, and its
  replacement reason ("nothing iterates it on a value path") is the true one.

- **§6 holds.** Nothing in the design forecloses O-3; the `WindowMap` seam is a
  genuine second reason for the module, and D-501's flip trigger survives
  untouched.

---

## Summary of what one fix round must close

1. §3 rows 3, 4, 7 — use the identifiers in the tree; mark row 7 ALREADY EXISTS.
   Fix §5 receipts 1 and 2 accordingly. (MAJOR 1)
2. I7 — restate as the unwind invariant with `Position::reset_to` cited; replace
   the un-makeable receipt. (MAJOR 2)
3. I4 — add a golden-digest assertion; the two-constructions test does not pin it.
   (MAJOR 3)
4. Add the `Eval::delta` interaction with quoted sites. (MAJOR 4)
5. Extend the doc-comment correction to `lib.rs:26-31` and
   `pistol-core/src/window.rs:29-30`. (MAJOR 5)
6. I6 — decide peak-vs-live, and make the test able to fail. (MAJOR 6)
7. Minors 1-7 as recorded.

Mechanism (§1), the gate refusal (§1.1), the I1/I2 disposition (§2.1) and the
Track E claim (§4) need no change.
