# WP-1.9 — eval window-map storage: DESIGN (revision 1)

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

1. **`window_key(Window) -> u64`** — `axis << 32 | (q ^ 0x8000) << 16 | (r ^ 0x8000)`,
   with `axis` the discriminant `ConstQ = 0, ConstR = 1, ConstS = 2`.
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

**The `windows` field doc comment is corrected, not carried.** It currently says
the map is "Ordered, so nothing in this crate can make a value depend on iteration
order". After this change the map is NOT ordered, and the true reason no value
depends on iteration order is that **nothing iterates it on a value path** — the
three operations are point lookups. Carrying the old sentence would leave the code
asserting a property it no longer has.

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
| **I6** | The footprint has a **stated bound with a derivation** | Dispatch requirement (`wp19_storage_DISPATCH.md`). Honoured because it is cheap and useful; **not used as a ground against any option**, which was finding B1 against the matrix. |
| **I7** | `newgame` **clears** the carried state | D-7. A map surviving a game boundary would make the first search of game *n+1* depend on game *n*. |

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

| Test | Pins | Shape |
|---|---|---|
| `a_packed_key_never_collides_for_two_distinct_windows` | I1 | Exhaustive over the three axes across the addressable coordinate range at the extremes and a swept interior, asserting distinct windows give distinct keys |
| `a_packed_key_orders_windows_the_way_the_window_type_does` | I2 | Same sweep, asserting `key(a) < key(b)` exactly when `a < b` |
| `an_unwound_eval_equals_a_fresh_one_whatever_order_it_is_taken_back_in` | I3, I5 | ALREADY EXISTS (`eval_incremental_tests.rs`); the rotated-unwind case is D-498's driving test and must keep passing unmodified |
| `a_probe_leaves_no_trace_in_the_carried_state` | I3 | ALREADY EXISTS (`eval_delta_tests.rs`) |
| `the_window_hasher_carries_no_seed_between_constructions` | I4 | Two independently constructed hashers give the same digest for the same key |
| `the_window_hasher_refuses_a_key_that_is_not_a_u64` | I4, rule 3 | The byte path panics with the `EVAL_DESYNC` token rather than degrading |
| `an_incremental_eval_agrees_with_a_rebuild_at_every_step` | Track E | Property test over random legal sequences INCLUDING turn-1 single stones and rule-4 truncations, comparing against `value_from_scratch` |
| `the_window_map_footprint_is_bounded_by_its_live_entries_and_its_peak` | I6 | Asserts the bound as a computed number from the map's own capacity and entry size — the derivation is the test body, not prose |
| `a_new_game_clears_the_carried_window_state` | I7 | Engine-level: state after `newgame` is indistinguishable from a fresh engine |

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
2. **Gate-off byte-identity of search output** over the 115-position receipt set, two
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

| Mutation | Must kill |
|---|---|
| Break the undo path — leave the emptied entry behind instead of removing it | `an_unwound_eval_equals_a_fresh_one_whatever_order_it_is_taken_back_in` |
| Skip one axis in the per-stone update | `an_incremental_eval_agrees_with_a_rebuild_at_every_step` |
| Skip the `newgame` clear | `a_new_game_clears_the_carried_window_state`, and the determinism seat |
| Collapse the packed key so two windows share one | `a_packed_key_never_collides_for_two_distinct_windows` |

A mutation that survives is a finding about the test, not a pass.

---

## 6. What this design does not decide

- **O-3 remains a registered flip trigger** (D-501). Nothing here forecloses it: a
  flip is another container behind the same `WindowMap` seam this design creates,
  which is the seam's second reason for existing.
- **No config surface changes.** `Budget`, the candidate policy and every committed
  config are untouched.
