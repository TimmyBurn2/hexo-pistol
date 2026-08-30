# WP-1.9 — eval window-map storage: DESIGN (revision 2)

**Revision 2 is the one fix round the cap allows.** Revision 1 (`2cda4f5`) was
attacked by a fresh-context REVIEW-design (`wp19_storage_design_REVIEW.md`) and
**FAILED** on six MAJOR findings — none of them a correctness defect in the
proposed mechanism. The review confirmed that **§1's mechanism, §1.1's gate
refusal, §2.1's I1/I2 disposition and §4's Track E claim need no change**; what
failed was the test-and-receipt layer, which named tests that do not exist,
registered an invariant against a mechanism that does not exist, and registered a
test that could not fail. Those are fixed below.

**Selected shape:** O-2, per D-501 and `matrix_wp19_storage_selection.md`.

**This document carries no measured numbers (D-483).** Mechanisms, invariants and
tests only. Every bracket and every figure this package registers is produced
post-implementation by a named instrument in one run.

**Scope, from the registered scope memo:** a CONTAINER SWAP behind an unchanged
`Eval` contract. `apply`/`undo`/`delta` already exist and are accepted (D-214,
D-220); this package does not introduce them, does not add eval terms, does not
touch search, and does not touch the solver.

---

## 1. Mechanism

Replace `HandcraftedV0`'s `BTreeMap<Window, Counts>` with a hash map over an
integer key, in a module of its own.

**New module `crates/pistol-eval/src/window_map.rs`**, holding three things and
nothing else:

1. **`window_key(Window) -> u64`** — with `axis` the discriminant
   `ConstQ = 0, ConstR = 1, ConstS = 2`:

   ```
   axis << 32 | u64::from((q as u16) ^ 0x8000) << 16 | u64::from((r as u16) ^ 0x8000)
   ```

   **The `as u16` is load-bearing and revision 1's prose dropped it.** Taken
   literally without it, a negative coordinate sign-extends, smears into the axis
   field, and collides `(ConstQ, -1, 0)` with `(ConstS, -1, 0)` — two distinct
   windows sharing one entry, which is invariant I1's exact failure. The narrowing
   cast is what makes each field disjoint.
2. **`WindowHasher`** — a seedless multiply-xor `Hasher`. It implements `write_u64`
   and refuses everything else by panicking with the crate's named `EVAL_DESYNC`
   token, because a hasher that silently degrades to a byte path is a wrong answer
   nobody sees (rule 3).
3. **`WindowMap`** — a thin newtype over
   `HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>` exposing exactly the three
   operations the eval performs, keyed by `Window` so no caller outside this module
   ever handles a packed key.

**Why a module and not a wider `handcrafted.rs`.** Rule 9's soft cap, and rule 9's
own remedy of first resort: a hasher and a key packing are not the evaluation.
This is condition 1 of the selection record.

**`handcrafted.rs` changes in three places only** — the field type, and the three
call sites at `apply`, `undo` and `delta`. The bodies keep their shape: `apply`
still inserts-or-updates, `undo` still removes an emptied window, `delta` still
reads without mutating.

**Two doc comments are corrected, not carried — the field's and the crate's.** It currently says
the map is "Ordered, so nothing in this crate can make a value depend on iteration
order". After this change the map is NOT ordered, and the true reason no value
depends on iteration order is that **nothing iterates it on a value path** — the
three operations are point lookups. Carrying the old sentence would leave the code
asserting a property it no longer has.

The same defect sits one level up, in the crate's most-read documentation:
`crates/pistol-eval/src/lib.rs`'s Determinism section says *"no hasher: the window
bookkeeping is a `BTreeMap`"*. That is the same claim in the same wrong direction,
and it is corrected in the same commit — the determinism argument becomes the two
things that are actually true, a seedless hasher and nothing iterating the map on
a path that reaches a value.

### 1.2 Interaction with `Eval::delta` (D-220), with the sites quoted

The dispatch requires this stated rather than assumed, and it matters more than
the other two call sites because **`delta` is the one that does not go through the
`Entry` API**, and because D-214 accepted its equivalence in terms of the two
`std` methods this change replaces.

The accepted argument reads, at `crates/pistol-eval/src/handcrafted.rs`:

> The `before` values are equal too: `entry().or_default()` reads the same counts
> `get().copied().unwrap_or_default()` reads, and the empty entry `apply` inserts
> is removed again by `undo`, so the roundtrip leaves no residue for this body to
> miss.

Both named methods cease to exist at those sites under this change, so **the
doc comment is retargeted rather than left standing**: `WindowMap::entry_or_default`
is what `apply` reads through and `WindowMap::get` is what `delta` reads, and the
two agree because `WindowMap::set` removes an entry it empties — so an absent
window and an emptied one are ONE observation to the map, which is exactly the
premise the original sentence needed from `BTreeMap`. An equivalence argument
whose sites no longer exist is not checkable, which is why this is a code change
and not only a design note.

The rest of `delta`'s contract is untouched: it still mutates nothing, still
reads every `before` from an unmutated map, still sums into one local and clamps
once, and still panics on the same first window with the same token.

### 1.1 No feature gate, and why that is not a hedge

The dispatch says to put the implementation behind a gate "where the design says
one is meaningful". **It is not meaningful here and none is added.**

A gate exists to let a behaviour be turned off. This change has no behaviour: it is
claimed bit-identical on every input, and that claim is verified by harness before
it binds (D-495). A config knob selecting a storage backend would add a hard-rule-1
surface, a second code path to test, and a permanent invitation to run the engine in
a shape no strength claim was ever taken in — to switch between two containers that
by construction answer identically. If the equivalence harness finds any mismatch,
the correct response is not a gate: it is the mechanical flip to Track S that D-495
requires.

---

## 2. Invariants, and which test pins each

| # | Invariant | Why it is load-bearing |
|---|---|---|
| **I1** | **`window_key` is INJECTIVE** over every addressable `Window` | This is the operative property. A collision silently merges two windows' counts into one entry — a wrong evaluation with no panic and no symptom. |
| **I2** | `window_key` is additionally ORDER-PRESERVING with respect to `Window`'s derived `Ord` | Not relied on by this shape. Recorded and pinned because it is the property that makes the key reusable if an ordered container ever returns, and because it implies I1. |
| **I3** | The map's equality is **canonical**: iteration-order-independent and history-independent | D-498. The `Eval` contract makes whole-state equality observational equivalence (D-214); a non-canonical comparison would make the two equality tests mean something weaker than they say. |
| **I4** | The hasher is **seedless by construction** | Rule 4 and D-32. No `RandomState`, nothing environment-derived, so two runs of the same position hash identically on any machine. |
| **I5** | An emptied window **leaves no entry behind** | The incumbent's `undo` removes it. Preserving this is what keeps an unwound eval equal to a fresh one, which is I3's driving case. |
| **I6** | The footprint has a **stated PEAK bound with a derivation** | Dispatch requirement (`wp19_storage_DISPATCH.md`). It is a bound on the table's CAPACITY, not on its live entries: a hash table does not shrink on removal, so a live-entries bound would be false — the selection record measures ~66.6 % of peak still held at zero entries. Because `Position::reset_to` unwinds rather than reconstructs, that peak is process-lifetime. Honoured because it is cheap and useful; **not used as a ground against any option**, which was finding B1 against the matrix. |
| **I7** | A game boundary leaves the eval **indistinguishable from a fresh one** | D-7. Revision 1 said `newgame` *clears* the map. **It does not, and no such mechanism exists**: `Searcher::clear()` (`crates/pistol-search/src/search.rs:194-203`) never touches the `Position` that owns the eval. What empties it is `Position::reset_to` (`crates/pistol-search/src/position.rs:55-70`), which UNWINDS at the head of every search. So this invariant is a consequence of I3 and I5 rather than a mechanism of its own, and giving it the mechanism revision 1 named would be a `pistol-search` change the dispatch's scope forbids. |

### 2.1 The correction I1/I2 record

The selection record's condition 2 asks for an order-preservation test. **For this
shape that is the wrong operative property, and the design says so rather than
satisfying the condition literally.** A `HashMap` needs the key to be injective; it
does not care about order. Order-preservation is a strictly stronger property that
happens to hold, and it implies injectivity, so the test suite pins both — I1 as the
property the code depends on, I2 as the stronger statement recorded for reuse. A
suite that pinned only I2 would leave a reader believing the shape needs an ordering
it does not need.

---

## 3. Tests

Behaviour-named, deterministic, no wall-clock (rule 7, Code style).

| Test | Pins | Status and shape |
|---|---|---|
| `a_packed_key_never_collides_for_two_distinct_windows` | I1 | NEW, in-source (D-115: a guard on a private item). Sweeps both lattice ends, the sign boundary and an interior run across all three axes, asserting distinct windows give distinct keys |
| `a_packed_key_orders_windows_the_way_the_window_type_does` | I2 | NEW, in-source. Same sweep, asserting `key(a) < key(b)` exactly when `a < b` |
| `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key` | I4 | NEW, in-source, **GOLDEN**. Revision 1 registered "two fresh hashers agree", which **cannot fail for the failure it guards**: a hasher seeded once per process from a clock or the environment passes it, and nothing else would catch that — `tools/determinism.sh` diffs search output, which a hash seed cannot move, precisely because §4's argument is right. Golden digests move under any seed |
| `the_window_hasher_refuses_a_key_that_is_not_a_u64` | I4, rule 3 | NEW, in-source. The byte path panics with the `EVAL_DESYNC` token rather than degrading |
| `an_emptied_window_leaves_no_entry_behind` | I5 | NEW, in-source. A window set empty loses its entry, and the map then equals a fresh one |
| `the_window_map_footprint_is_bounded_by_its_capacity_and_its_entry_size` | I6 | NEW, in-source. The bound is COMPUTED in the test body from `size_of` and the table's own capacity — the derivation is the test, not a quoted number |
| `eval_apply_undo_roundtrip` (`eval_incremental_tests.rs:94`) | I3, I5 | **ALREADY EXISTS.** The rotated-unwind case D-498 cites by `file:line`; it must keep passing unmodified |
| `delta_leaves_the_eval_indistinguishable` (`eval_delta_tests.rs:396`) | I3 | **ALREADY EXISTS.** |
| `eval_incremental_matches_from_scratch_on_random_playouts` (`eval_incremental_tests.rs:29-91`) | Track E leg 1 | **ALREADY EXISTS, and revision 1 wrongly registered it as new.** It already does what the dispatch asks: random legal sequences through `GameState` (so plies are coloured by the rules and never by `i % 2`, D-499), turn-1 single stones, and rule-4 truncation, checked against `value_from_scratch` forward and on the way back |
| `new_game_forgets_the_position_and_everything_learned` (`crates/pistol-engine/tests/engine_tests.rs:213`) | I7 | **ALREADY EXISTS.** A fresh engine and a reused-then-`newgame` engine must agree on bestmove, nodes, PV, score and hashfull |

**Three tests revision 1 listed as new already exist**, and the design says so
rather than shipping duplicates of them: a second test asserting what a landed one
asserts is a claim stated twice (D-423), and it makes a mutation receipt
uninformative because the mutant dies to the older test first.

**What is NOT re-tested.** The `Eval` contract, `delta`'s equivalence to the
roundtrip, and the desync tokens are already pinned and accepted (D-214, D-220).
This package must not break them and does not restate them.

---

## 4. Track E — the claim and its proof

**The claim: every eval output is bit-identical to the incumbent on every input.**
Justified by construction — the value path never iterates the map (`value` reads the
running `p1_score`; `apply`, `undo` and `delta` are point operations over the
unchanged `windows_through` enumeration, accumulating in the unchanged order) — and
therefore verified, not assumed.

**The proof, in the order it must be taken:**

1. **Incremental-vs-rebuild agreement at every node**, over the determinism fixtures
   and a governed-shape game set, both seats and both budgets.
2. **Byte-identity of search output** over the 115-position receipt set, two
   `--release --locked` binaries built in their own detached worktrees and compared
   directly — the WP-1.5d(A) precedent (D-484), which does not route through
   `search_oracle_check.sh`.
3. **The rule-5 bench bracket**, registered BEFORE it is taken and then taken, on
   `tools/bench_delta.sh` against the baseline this package started from.

**A single mismatch anywhere in 1 or 2 flips the package to Track S mechanically, no
discretion, recorded (D-495).** The design does not reserve the right to argue about
a mismatch.

**Evidence already in hand, and its limit:** node identity held per position under
both budgets in every rep for all three benched candidates. That is evidence the
searches are identical and it is NOT the proof — it compares node counts, not eval
outputs, and `bench_delta.sh` is the instrument the bracket also comes from, so the
two are not independent. Step 1 is what actually establishes the claim.

---

## 5. Mutation receipts owed

Each mutation is a deliberate break made in a **separate worktree**, never the live
tree, and each must kill the named test:

| Mutation | Must kill | Why this one |
|---|---|---|
| `undo` leaves the emptied entry behind instead of removing it | `eval_apply_undo_roundtrip` | I5, and the driving case of D-498 |
| Skip one axis in the per-stone update | `eval_incremental_matches_from_scratch_on_random_playouts` | I1's consumer: the agreement against a rebuild |
| **Swap the `q` and `r` fields in the packed key** | `a_packed_key_orders_windows_the_way_the_window_type_does`, and NOT the collision test | Isolates I2 from I1. Revision 1 registered "collapse the key so two windows share one", which kills three tests at once and never separates the two invariants — a receipt that cannot tell which property it pinned |
| Drop the `as u16` narrowing from the key | `a_packed_key_never_collides_for_two_distinct_windows` | I1 directly, and it is the real defect the prose bug would have shipped |
| Reintroduce a per-process seed in the hasher | `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key` | I4, and the reason that test is golden rather than self-comparing |

**No `newgame` mutant is registered**, because I7 has no mechanism of its own to
break — see the invariant table. Breaking `Position::reset_to` would be a
`pistol-search` mutation, out of scope.

A mutation that survives is a finding about the test, not a pass.

---

## 6. What this design does not decide

- **O-3 remains a registered flip trigger** (D-501). Nothing here forecloses it: a
  flip is another container behind the same `WindowMap` seam this design creates,
  which is the seam's second reason for existing.
- **No config surface changes.** `Budget`, the candidate policy and every committed
  config are untouched.
