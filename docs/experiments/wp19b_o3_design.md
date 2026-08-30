# WP-1.9b — O-3, the hand-rolled probing table: DESIGN

> **OUTCOME, added after the run this document was written for: O-3 DID NOT
> FLIP.** It was implemented, reviewed (`wp19b_o3_impl_REVIEW.md`), its mutants
> taken, and measured on D-501's fixed terms at **1.518 early / 1.594 late**
> against the registered comparand of 1.783 / 1.909 — below it in BOTH bands, so
> not even the one-band finding the terms contemplate
> (`artifacts/wp19b_bench_flip_v1.txt`, `wp19b_bench_results.md`). The shape that
> landed is O-2 moved inline. **This document therefore describes a revision that
> is tagged and NOT on `dev`: `wp19b/o3-fixed` (`6ea88b2`).** It is kept because a
> rejected option measured is the only thing that discharges a flip trigger, and
> because a successor should reproduce O-3 rather than re-implement it.

**Scope.** Implement O-3 — the open-addressing probing table D-225 named and
D-501 registered as WP-1.9's flip trigger — over the SAME packed `u64` key, and
land the flip bench's winner INLINE in `crates/pistol-eval/src/handcrafted.rs`.
Two debts discharge together because they interact (WP-1.9 closure §6.1/§6.2): if
O-3 wins it should never have been written in a module, so it is written inline
from the start.

**This document carries no measured number (D-483).** Mechanisms, invariants and
tests only. Every number this package consumes is registered in
`wp19b_bench_prereg.md` and produced by the instrument named there.

**Not in scope.** The `Eval` trait, the search, the config, the weights, and the
enumeration `windows_through` performs. This is a container swap behind an
unchanged seam, exactly as D-501 says a flip would be.

---

## 1. Mechanism

### 1.1 The key is unchanged

`window_key(window) = axis << 32 | (q ^ 0x8000) << 16 | (r ^ 0x8000)`, the packed
`u64` D-225 named and WP-1.9 shipped, moved verbatim. Its injectivity (I1) is
what every store shape here depends on and it is not re-derived.

**One new consequence is load-bearing and is pinned rather than argued.** `axis`
takes three values, so every key the packing can produce is below `3 << 32`, and
`u64::MAX` is therefore a value NO window has. That is what lets an empty slot be
spelled in the key itself rather than in a second array of control bytes — one
load per probe instead of two, and no way for the two arrays to disagree.

### 1.2 The table

Open addressing, LINEAR probing, power-of-two slot count, `EMPTY_KEY` in the key
field marking a free slot:

- **Probe sequence:** `i = hash(key) & mask`, then `i = (i + 1) & mask`, where
  `hash` is the same seedless multiply-xor fold WP-1.9 shipped as `WindowHasher`,
  now a plain function because there is no `std` trait left to satisfy. Linear
  and not quadratic: the table is small enough to live in L1 at every occupancy
  this eval reaches, so a probe that walks the next cache line beats one that
  jumps.
- **Load factor:** at most one half. Growth doubles and rehashes. The bound is
  what makes the probe loop terminate — a full table has no empty slot to stop
  at — so it is an invariant and not a tuning knob (I8).
- **A fresh table allocates its initial slots eagerly**, so `slots` is never
  empty and no operation carries an is-it-allocated branch. One `Position` owns
  one eval for the life of a search (`crates/pistol-search/src/position.rs:13`,
  `reset_to` unwinds rather than rebuilding), so this allocation is not on any
  hot path.

### 1.3 Deletion is EAGER COMPACTION, not a tombstone

**A window whose counts reach zero has its entry removed, and the removal leaves
no residue of any kind.** Removal is Knuth's backward-shift deletion (Algorithm
6.4R): the hole walks forward while each following entry whose home lies on the
hole's side is moved down into it, stopping at the first empty slot.

Two things this buys, both of which the alternative (a DELETED marker) loses:

1. **No tombstone can exist**, so the D-498 obligation cannot be violated by
   residue — there is no residue to exclude. This is the "eager compaction"
   branch of the obligation, chosen over "tombstone-excluding equality" because
   an invariant that holds by construction needs no reader to remember it.
2. **No unbounded probe growth.** This eval empties and refills its table at
   every `Position::reset_to`, and a store that left a marker per removal would
   fill with markers and lengthen every probe until a rehash it has no other
   reason to perform.

**The hazard this introduces, named because it is O-3's whole defect class:** a
removal that punches a hole WITHOUT shifting silently orphans every later entry
in the same cluster — each is still in the table and no lookup can reach it,
because probing stops at the hole. It is invisible to a store that is only ever
grown, and it produces wrong evaluations with no panic. §3 gives it a
deterministic test with a constructed cluster, not a random one.

### 1.4 Equality is over the live set (D-498, the driving obligation)

`PartialEq` is HAND-WRITTEN and compares the live `(key, counts)` set: equal live
counts, and every live entry of one found in the other with the same counts. It
reads NO slot index, NO capacity, and NO empty slot.

**Why a derived `PartialEq` is wrong here, and would be wrong even with eager
compaction.** Linear probing's arrangement depends on insertion order: two keys
sharing a home slot land in the order they arrived, so a map grown one way and a
map grown another hold the same entries in swapped slots. A derived comparison
calls those two maps different. Capacity differs too — an unwound map keeps the
peak it grew to, a fresh one has its initial slots — and D-498's obligation is
that an unwound eval is INDISTINGUISHABLE from a fresh one. The driving test is
the rotated unwind at
`crates/pistol-eval/tests/eval_incremental_tests.rs:136-140`, which takes the
same stones off in an order no search would use.

`Debug` is likewise written by hand, printing live entries only: a derived one
would dump every slot of a grown table into an assertion message. Neither
`PartialEq` nor `Debug` is on a value path, so neither can reach move choice
(D-498's own reading, which this design does not widen).

### 1.5 The three operations, unchanged in contract

`get` (delta, read-only), `entry_or_default` (apply, insert-or-update) and
`update` (undo, edit-and-maybe-remove) keep the signatures and the ONE-PROBE
contract WP-1.9 measured and REVIEW-impl defended. `update` still resolves the
slot once and edits through the resolved index; a `get` then a `set` would hash
twice on the path a search walks for every stone it unwinds. **Do not
reintroduce a `set`** — the closure says so and this design does not reopen it.

The map is CONCRETE over `Counts` rather than generic over `V`. The generic
bought reuse the crate never took, and the whole point of landing inline is to
leave the optimiser nothing to see across.

---

## 2. Invariants, and what pins each

| # | Invariant | Pinned by |
|---|---|---|
| I1 | `window_key` is injective | `a_packed_key_never_collides_for_two_distinct_windows` |
| I2 | `window_key` is order-preserving (not relied on; kept for reuse) | `a_packed_key_orders_windows_the_way_the_window_type_does` |
| I3 | Equality is canonical — order-, capacity- and residue-independent | `eval_apply_undo_roundtrip` (the rotated unwind), `delta_leaves_the_eval_indistinguishable` |
| **I4'** | **The placement function is seedless and fixed** — same key, same slot count, same slot, on every machine and every run. Its members are the FOLD, the PROBE STEP and the INITIAL SLOT COUNT, named here and nowhere else (D-423) | `the_window_hash_answers_a_fixed_digest_for_a_fixed_key` (GOLDEN, over the fold) and `the_table_places_a_fixed_window_set_in_a_fixed_pattern` (GOLDEN, over the placement the fold feeds) |
| I5 | An emptied window leaves no entry behind, and no residue | `an_emptied_window_leaves_no_entry_behind`, `a_removal_keeps_every_other_entry_reachable` |
| I6 | The footprint is bounded by PEAK, not by live entries | `the_window_table_holds_its_peak_capacity_after_every_entry_is_gone` |
| I7 | A game boundary leaves the eval indistinguishable from fresh | `new_game_forgets_the_position_and_everything_learned` (`pistol-engine`) |
| **I8** | **The table is never full** — load factor at most one half | `the_table_never_fills`, which asserts the bound AFTER EVERY INSERT of a saturating sweep (once at the end passes for a table that spent the sweep over the bound), plus a `debug_assert!` in `probe` so a full table fails by name instead of spinning |

**I4' replaces WP-1.9's I4 and is WIDER than it.** I4 said the HASHER was
seedless. What matters to an open-addressed table is the whole placement
function, because any of its three members moving relocates an entry. So it takes
TWO goldens, and the reason is a REVIEW FINDING rather than a preference: the
placement golden observes only the low bits of the fold, for one slot count, so a
ONE-BIT change to the multiply constant passes it while moving the home slot of a
twentieth of the windows in a ±60 box — reproduced by REVIEW-impl
(`wp19b_o3_impl_REVIEW.md` MAJOR 1). WP-1.9's digest golden is therefore KEPT, not
retired: its subject is the fold, which survived the refactor intact. What IS
retired with `std::hash::Hasher` is `..._refuses_a_key_that_is_not_a_u64`, whose
subject was a trait method no caller has any more — the key is a `u64` by type
and there is no byte path to guard. Both goldens carry the reseed-singleton
lesson WP-1.9 recorded: a moved placement moves no search output, so
`tools/determinism.sh` cannot see it and an agreement test would pass.

**The placement golden certifies its own non-vacuity, and it says how far that
reaches.** A golden over a key set that never collides says nothing about
probing, and one whose deepest key sits a single slot from home is reproduced by
any scheme that steps by one first — triangular probing included, which
REVIEW-impl demonstrated. So the set is chosen to place a key FOUR slots from its
home, and the test asserts that depth before it compares anything.

**I7 still has no mechanism of its own** (WP-1.9 closure §4): `Searcher::clear()`
does not touch the eval, and what empties it is `Position::reset_to`'s unwind. I7
is a consequence of I3 and I5. Nothing here changes that, and no `newgame` mutant
is registered.

---

## 3. Tests

Moved verbatim where the subject did not change; new where it did. They live in
`handcrafted.rs`'s own `#[cfg(test)] mod tests`, as WP-1.9's lived in
`window_map.rs`'s, because every one of them pins a PRIVATE item (D-115): the key
packing, the placement, the slot array, the live count.

| Test | Pins | Status |
|---|---|---|
| `a_packed_key_never_collides_for_two_distinct_windows` | I1 | MOVED |
| `a_packed_key_orders_windows_the_way_the_window_type_does` | I2 | MOVED |
| `a_window_key_is_never_the_empty_slot_marker` | §1.1's consequence — the sentinel is outside the key's range | NEW |
| `the_window_hash_answers_a_fixed_digest_for_a_fixed_key` | I4', the fold | KEPT from WP-1.9, its subject unchanged |
| `the_table_places_a_fixed_window_set_in_a_fixed_pattern` | I4', the placement | NEW, golden, self-certifying against vacuity |
| `an_emptied_window_leaves_no_entry_behind` | I5 | MOVED, and now also asserts no residue in the slots |
| `a_removal_keeps_every_other_entry_reachable` | I5, §1.3's hazard | NEW, over a CONSTRUCTED cluster |
| `the_window_table_holds_its_peak_capacity_after_every_entry_is_gone` | I6 | MOVED |
| `the_table_never_fills` | I8 | NEW |
| `eval_apply_undo_roundtrip`, `eval_incremental_matches_from_scratch_on_random_playouts` | I3, and the whole store against a rebuild | UNCHANGED, in the suite |

`the_window_hasher_refuses_a_key_that_is_not_a_u64` is RETIRED WITH ITS SUBJECT:
the byte path it guarded belongs to `std::hash::Hasher`, and a table whose key is
a `u64` by type cannot reach one. Its sibling digest golden is KEPT — see I4'
above for why the placement golden does not subsume it.

---

## 4. Mutation receipts owed

Each is a deliberate break in a SEPARATE worktree (never the live tree), and each
must kill the named test. A survivor is a finding about the test.

| Mutation | Must kill | Why this one |
|---|---|---|
| `PartialEq` compares the slot arrays instead of the live set | `eval_apply_undo_roundtrip` | D-498's obligation directly: an unwound map and a fresh one differ in capacity and in arrangement |
| Removal leaves the key in the slot (a tombstone) instead of compacting | `eval_apply_undo_roundtrip` | The residue class the obligation names |
| Removal punches a hole and skips the backward shift | `a_removal_keeps_every_other_entry_reachable` | §1.3's hazard, and the reason that test constructs its cluster rather than sampling one |
| `update` stores a zeroed value instead of removing the entry | `eval_apply_undo_roundtrip` | I5 — WP-1.9's own receipt, re-taken against the new store |
| The hash constant moves, by one bit or wholesale | `the_window_hash_answers_a_fixed_digest_for_a_fixed_key` | I4', and the reseed-singleton lesson: nothing else in the workspace can see it |
| The initial slot count moves | `the_table_places_a_fixed_window_set_in_a_fixed_pattern` | I4' again, and the placement golden is its ONLY pin |
| The probe step moves | the placement golden for a step it can see; a deeper scheme change (triangular) is killed by the removal and delta tests, NOT by the golden alone | stated as measured rather than claimed: the golden's reach is the depth of its own set |
| `apply` skips one axis | `eval_incremental_matches_from_scratch_on_random_playouts` | The store against an independent rebuild |

---

## 5. What this design does not decide

**Which shape ships.** The flip bench decides that, on terms D-501 fixed and this
document may not touch: O-3 flips only if it exceeds the registered comparand in
BOTH bands by more than the within-run IQR, and one band is a finding. If it does
not flip, the landing candidate is O-2 moved inline and this design's storage is
not landed — its measurement discharges D-501 either way.

**Whether the module boundary is the cause of the cost the WP-1.9 closure
measured.** The two revisions that measurement compared differ in more than a file
boundary — a newtype wrapper, a type parameter and a closure argument all appear
on one side and not the other — so "the module split" names the CHANGE that was
measured and not an isolated cause. This package does not need the cause: it lands
the whole change and re-measures it. Recorded so a later reader does not take a
mechanism the run never separated.
