# WP-1.9 storage IMPLEMENTATION — REVIEW-impl (round 1)

**Target:** commit `07f518bf7024363810cba90099397e2755cb3e29`, branch `dev`
("feat(eval): the window map moves to a seedless hashed store behind an unchanged
`Eval` contract, with the key packing and hasher in their own module").

**Match to HEAD:** YES at the start of the review, NO at the end. The
concurrently working session committed twice on top while this review ran, so
`git rev-parse HEAD` is now `0c6cce8`. **Both are documents only** — `562c8eb`
(the rule-5 bench prereg) and `0c6cce8` (its run-1 results). `git diff --stat
07f518b HEAD` touches no `.rs` file, so **the three reviewed source files are
byte-identical at HEAD** and every finding below stands against HEAD as well as
against the named revision. The review was taken against `07f518b`.

**Tree state:** clean (`git status --porcelain` empty at the start of the review).
Nothing was written to the live tree except this report; all building and testing
happened in detached worktrees, and the two worktrees belonging to the other
session (`/home/tom/wp19-base`, `/home/tom/wp19-cand`) were left untouched.

**Governing design:** `docs/experiments/wp19_storage_design.md` **revision 2**
(`723758b`), whose round-1 review is `wp19_storage_design_REVIEW.md` (FAIL on
revision 1, six MAJOR).

**Baseline compared against:** tag `wp19/mx-base` = `a5c5661`.

**Work done in:** four detached worktrees under `/home/tom/wp19rev/` (`impl`,
`base`, `diff`, `mut`), each with its own `CARGO_TARGET_DIR`, removed on
completion. Nothing was built or tested in the live tree.

---

## VERDICT: FAIL

**No correctness defect was found, and Track E is NOT flipped.** Every eval output
I could reach is bit-identical to the incumbent's, panic messages included, on the
value path and on the post-panic wreckage path (§E). The reason for the FAIL is
elsewhere, and it is the same class of defect the design review already failed the
design on once:

1. **The shipped `undo` is not the shape the selection's only surviving ground was
   measured on.** It does **two** table probes per window where the benched O-2
   revision `9a986c6` did one — MEASURED, 36 table hashes per stone against 18.
   The design says the bodies keep their shape. They do not, on the one hot path
   the design review warned about by name (MINOR 1). The implementing session's
   own rule-5 bracket, taken after this finding was written, lands **below** at
   1.505 / 1.555 and names the same cause — two instruments, one defect.
2. **A MAJOR from the governing review is not closed.** `pistol-core/src/window.rs:29-30`
   still asserts the eval bookkeeping lives "in an ordered map with no hasher".
   That is now false, in the crate CLAUDE.md calls the one source of game truth.
3. **The footprint test still cannot fail for the invariant it is registered
   against.** MAJOR 6 was closed by renaming the test to the shape the review
   itself called vacuous "in letter only", not by changing it. MEASURED: the map
   retains 97.1 % of peak capacity at zero live entries, and the test never
   drains, so nothing pins that.

One fix round closes all three; none of them touches the mechanism, the key
packing, the hasher or the `Eval` contract. MAJOR 1 is already in hand — the
implementing session found it independently and registered the repair as bench
RUN 2 — but the repair has not landed at the reviewed revision and MAJORs 2 and 3
are untouched, so the package is not landable as it stands (a WP is not landable
while its reviews are outstanding).

---

## Findings

### BLOCKING — none

No finding names an input on which this eval answers differently from the
incumbent, and none names a way the shipped code produces a wrong answer. D-495's
mechanical flip to Track S is **not** triggered by this review.

---

### MAJOR 1 — the shipped `undo` doubles the table work per window against the revision the 1.783/1.909 nps ground was measured on, and the design says it would not

**Claim attacked.** Design §1: *"`handcrafted.rs` changes in three places only —
the field type, and the three call sites at `apply`, `undo` and `delta`. **The
bodies keep their shape:** `apply` still inserts-or-updates, `undo` still removes
an emptied window, `delta` still reads without mutating."*

**What the benched revision did.** The selection record's one ground is
`tools/bench_delta.sh` on O-2 at `9a986c6`. That revision's `undo` reaches the
table exactly once per window, through a single `Entry`:

```
$ git show 9a986c6:crates/pistol-eval/src/handcrafted.rs | sed -n '/fn undo/,/^    fn value/p'
                let Entry::Occupied(mut slot) = self.windows.entry(window_key(window)) else {
                ...
                slot.get_mut().remove(player);
                let after = *slot.get();
                if after.is_empty() {
                    slot.remove();
                }
```

**What ships.** `07f518b`'s `undo` reads through `WindowMap::get` and writes back
through `WindowMap::set` — two independent hashes and two independent table
probes per window:

```rust
let before = self.windows.get(window);      // hash + probe 1
...
let mut after = before;
after.remove(player);
self.windows.set(window, after);            // hash + probe 2
```

**Command run.** A counter added to `WindowHasher::write_u64` in the scratch
worktree `/home/tom/wp19rev/diff`, over a 300-ply `GameState` playout:

```
WINDOWS_PER_CELL = 18
apply : 18 table hashes per stone
delta : 18 table hashes per hypothetical stone
undo  : 36 table hashes per stone (mean over 300 stones)
undo/apply ratio: 2.000
```

`apply` and `delta` are at parity with the incumbent (one `entry`, one `get`).
**Only `undo` regressed, and it regressed by exactly 2x.**

**Consequence.** The nps figure the whole option selection rests on — D-501 and
`matrix_wp19_storage_selection.md` §1, "O-2 is the fastest shape measured", the
record's *one* surviving ground — was taken on a shape whose hot unwind path costs
half what the shipped one costs in table operations. The rule-5 bracket the design
owes (§4.3) will therefore be a measurement of a shape the matrix never evaluated,
against an O-4 row that was. I am NOT quoting a speed number for this: D-499 binds
D-258's "measure at the level of the shipped structure" and its "table-only figures
may not be quoted as a bracket", and the probe count above is a structural count,
not a timing claim. The magnitude of the nps effect is **UNMEASURED** and is the
bracket's job — the finding is that the bracket can no longer be read as confirming
or refuting the ground D-501 records, because it measures a different `undo`.

The design review predicted this precisely (MINOR 1: *"the `undo` path is the one
where a coarser API silently costs a probe against the measurement the whole
selection rests on"*). Revision 2 did not close it, and the implementation walked
into it.

**Independently corroborated, at the whole-engine level, after this finding was
written.** While this review was running the implementing session took the rule-5
bracket and reached the same conclusion from the other direction —
`docs/experiments/wp19_storage_bench_results.md` (`0c6cce8`),
`tools/bench_delta.sh rev:723758b rev:07f518b 5`, artifact
`artifacts/wp19_bench_shipped_v1.txt`:

| band | nps ratio | registered bracket |
|---|---|---|
| early | **1.505** | BELOW [1.60, 2.10] |
| late | **1.555** | BELOW [1.60, 2.10] |

against the matrix's inline 1.783 / 1.909 for the same storage shape. That
document names the cause as *"an extra hash lookup per window in `undo`,
introduced by the shipped `WindowMap` API and not by the split"* and registers the
single-lookup repair as RUN 2. Two independent instruments — a probe count and a
whole-engine bench — agree, the bracket was registered before the run and is not
moved (D-374), and the finding is therefore MEASURED rather than inferred. **This
does not soften the finding; it confirms it.** It is recorded here because a
REVIEW-impl that reached the same defect independently is evidence about the
defect, not a reason to drop it: the shipped `undo` still stands at `07f518b`, and
the fix is owed before the package can land.

**Fix.** One additional `WindowMap` operation that performs the read-modify-remove
through a single `Entry` — `take_one(window, f) -> (V, V)` or an
`entry`-shaped accessor — restoring 18 probes per stone. `apply` and `delta` need
no change. Then take the bracket.

---

### MAJOR 2 — design-review MAJOR 5 is not closed: `pistol-core` still asserts the eval bookkeeping is an ordered map with no hasher

**Claim attacked.** Design §1: *"**Two doc comments are corrected, not carried —
the field's and the crate's.**"* Two. The governing review's MAJOR 5 named
**three** sites and its Fix said *"Extend §1's correction paragraph to name all
three sites"*, quoting the third verbatim.

**Command run.**

```
$ git grep -n -i "ordered map\|no hasher" 07f518b -- crates/ | LC_ALL=C sort
07f518b:crates/pistol-core/src/window.rs:29:/// ordered map with no hasher anywhere near a value the engine plays on
```

In full, at `crates/pistol-core/src/window.rs:25-31`:

```rust
/// A window: [`WINDOW_LEN`] cells from `start`, stepping along `axis`.
///
/// The ordering is `(axis, start)` with `start` lexicographic by `(q, r)` —
/// derived, and deterministic, which is what lets the bookkeeping live in an
/// ordered map with no hasher anywhere near a value the engine plays on
/// (CLAUDE.md rule 4, docs/decisions.md D-32).
```

**Consequence.** After `07f518b` the bookkeeping does **not** live in an ordered
map and there **is** a hasher in `pistol-eval`. The sentence is false as written,
and it is false in `pistol-core` — the crate CLAUDE.md names as THE one source of
game truth, and the crate a reader consults first for what `Window` is for. This
is exactly the defect design §1 exists to prevent: *"Carrying the old sentence
would leave the code asserting a property it no longer has."* The determinism
conclusion survives; the stated reason for it does not.

`crates/pistol-core/src/lib.rs:44-49` was checked and is **fine** — its `BTreeMap`
sentence is about board occupancy (D-32), which is untouched.

**Fix.** The review's own remedy: a one-line trim of the subordinate clause. It is
a comment-only edit in `pistol-core`, so it changes no rule, no geometry and no
win detection, and it does not engage hard rule 2.

---

### MAJOR 3 — the footprint test closes MAJOR 6 by renaming, and the shape it keeps is the one the review called un-failable

**Claim attacked.** Design §3 row: `the_window_map_footprint_is_bounded_by_its_capacity_and_its_entry_size`
— *"The bound is COMPUTED in the test body from `size_of` and the table's own
capacity — the derivation is the test, not a quoted number."*

The governing review's MAJOR 6 identified the contradiction between the row's NAME
(bound in live entries and peak — can fail) and its SHAPE (bound computed from the
map's own capacity — *"a claim about the map's arithmetic against itself, which
cannot [fail] ... it asserts that multiplication works"*). Its Fix was explicit:
*"Pick one and make the other a pointer: assert `capacity <= f(peak live entries)`
with `f` written out, **drive the map to a stated peak, drain it, and assert the
retained footprint against `f(peak)` — a test that fails if the container's
retention behaviour changes.**"*

Revision 2 resolved the contradiction by adopting the **name that matches the
un-failable shape**, and shipped that.

**What the shipped test actually asserts**, at `crates/pistol-eval/src/window_map.rs:231-270`:

| # | assertion | can it fail? |
|---|---|---|
| 1 | `capacity() * (PAIR+1) == 0` on a fresh map | yes — pins that `Default` does not pre-allocate |
| 2 | `live == 64*64` after the sweep | yes — this is an injectivity assertion, already owned by `a_packed_key_never_collides_for_two_distinct_windows` |
| 3 | `capacity() >= live` | **no** — `HashMap::capacity()` is by definition the element count the map holds without reallocating, so it is `>= len()` for every std `HashMap` |
| 4 | `capacity*(PAIR+1) <= live*8*(PAIR+1)` | only for a container overshooting by more than 8x |

**Command run.** A probe added beside the test in `/home/tom/wp19rev/mut`, same
workload as the test:

```
PAIR+1 = 17
live = 4096, capacity = 7168
assert 3: capacity >= live  -> 7168 >= 4096  (std guarantees this)
assert 4: capacity <= 8*live -> 7168 <= 32768   headroom x4.57
after drain: live = 0, capacity = 6957 (RETAINED 97.1% of peak capacity)
the shipped footprint test never drains, so this number is unpinned
```

**Consequence.** The two assertions that carry the word "footprint" are the two
that cannot discriminate: one is a std invariant, the other has 4.57x headroom at
the registered workload. The property the selection record measured and D-501
records as O-2's accepted cost — that the table **retains** most of its peak after
draining, "~66.6 % of peak bytes at zero entries" — is the only non-vacuous
footprint claim in the package and **the shipped test never drains the map**, so
nothing pins it. I6 is registered as *"a stated PEAK bound with a derivation"* and
*"a bound on the table's CAPACITY, not on its live entries: a hash table does not
shrink on removal"*; the shipped test exercises only the growth half.

This also leaves selection-record condition 4 (*"The memory figure is stated as a
number with its derivation in a test"*) discharged in letter only — no number is
stated anywhere in the shipped code, and none is stated in the design either
(correctly, under D-483). The number therefore has no home at all.

**Fix.** The review's own remedy, unchanged: drive to a stated peak, drain, assert
the retained capacity against `f(peak)`. That is a three-line addition to the test
already in the file, and it is the assertion M1 and a future container swap would
both have to clear.

---

### MINOR 1 — the order-preservation test restates `Window`'s field order instead of reading its `Ord`, so it passes when the invariant it names is false

**Claim attacked.** I2, and the test's own name:
`a_packed_key_orders_windows_the_way_the_window_type_does`. I2 is stated as
order-preservation *"with respect to [`Window`]'s derived `Ord`"*.

**What the test compares** (`window_map.rs:192-197`):

```rust
assert_eq!(
    window_key(a) < window_key(b),
    (a.axis, a.start) < (b.axis, b.start),
    ...
);
```

`Window` derives `Ord` (`crates/pistol-core/src/window.rs:31`), so `a < b` was
available; the test writes out the field order by hand instead.

**Command run.** In `/home/tom/wp19rev/mut`, `Window`'s two fields were swapped so
its derived `Ord` becomes `(start, axis)` — which makes I2 as stated FALSE, since
the key still orders by `(axis, q, r)`:

```
32:pub struct Window {
33-    pub start: Coord,
35-    pub axis: Axis,

test window_map::tests::a_packed_key_orders_windows_the_way_the_window_type_does ... ok
test result: ok. 6 passed; 0 failed; ...
```

**Consequence.** The test is green for a `Window` whose ordering the key does not
preserve. It pins the key against a tuple the test writes, not against the type
whose ordering I2 is stated in terms of — and I2's whole registered purpose is
that the key *"stays reusable if an ordered container ever returns"*, which is a
claim about `Window: Ord` and nothing else. **Fix:** `a < b`.

---

### MINOR 2 — the `windows` field doc comment over-claims in the same direction the doc correction existed to fix

**Claim attacked.** `crates/pistol-eval/src/handcrafted.rs:83-85`:

> The windows holding at least one stone, and what they hold. Never iterated on a
> value path, **which is what keeps iteration order out of every answer this crate
> gives.**

**Command run.** `/home/tom/wp19rev/diff`, two evals holding identical stones
inserted in opposite orders, new store against the incumbent:

```
Debug identical across insertion order -- NEW: false  BASE: true
unwound == fresh (PartialEq): true
unwound Debug == fresh Debug: true
clone == original: true
```

**Consequence.** `Debug` is an answer this crate gives — it is a derive on a public
type — and it is now order-dependent where the incumbent's was canonical. "Every
answer this crate gives" is false. `lib.rs`'s replacement wording is careful and
**correct** ("nothing in this crate iterates the map on a path that reaches a
*value*"); the field's is the loose one. This is the design review's MINOR 7, which
revision 2 did not put on the I-list, now written into the code as a claim the code
does not have — the precise failure mode design §1 names. The behavioural
consequence is nil (see §E), so this is MINOR, but it is a comment asserting a
false property in the file the correction was aimed at.

---

### MINOR 3 — the `delta` doc comment's closing sentence documents the review history, not the code (D-423 / CLAIM-HOME)

`crates/pistol-eval/src/handcrafted.rs:210-217` now ends:

> ... **This sentence names the two operations because D-214 accepted the
> equivalence in terms of the two it replaced, and an argument whose sites no
> longer exist is not checkable.**

That is a fact about why the paragraph was rewritten, and it already has a home:
design §1.2 makes exactly this argument, at length, in the section that owns it.
Code style says a comment says WHY the *code* is as it is, and that *"if a comment
needs a paragraph, the code or the design doc is the wrong shape"*. The retarget
itself is right and required; the meta-sentence justifying it is the claim stated
twice. **Fix:** delete the last sentence.

---

## §E — The Track E bit-identity claim

**It holds. I found no input on which the two differ, and I looked for one rather
than reading for one.**

The design's argument (§4: the value path never iterates the map; `windows_through`
is untouched; accumulation order is the enumeration's) is correct, and the review's
§T strengthening (the container is unreachable outside `pistol-eval`'s own test
crate, because `Eval` has no supertrait and every consumer holds `Box<dyn Eval>`)
is correct. I did not rely on either.

**Method.** The incumbent `handcrafted.rs` from `wp19/mx-base` was vendored beside
the new one in a scratch worktree as `BaseV0` (identical code, renamed type), so
both run in one process against one `pistol-core` and can be driven in lockstep.

**What was compared, and the result:**

| probe | inputs | result |
|---|---|---|
| `value(P1)`, `value(P2)`, `delta(c, p)` over 40 seeded `GameState` playouts of up to 200 plies, with ~40 hypothetical `delta` probes per ply on both colours, then the full unwind | ~10^6 paired comparisons | identical |
| the same, at 8 anchors including `(-1,-1)`, `(i16::MIN+8, i16::MIN+8)`, `(i16::MAX-8, i16::MAX-8)` and `(i16::MIN+8, i16::MAX-8)` — the sign boundary and both lattice ends, where the `as u16` narrowing is load-bearing | 8 anchors x 60 stones x probes | identical |
| whole-state `PartialEq` against a fresh eval after each unwind | every playout | equal, both stores |
| desync panic **messages**: `undo` of an absent window, `undo` of a colour the window never held, `apply` that overfills, `delta` that would overfill | 4 cases | byte-identical strings, all carrying `EVAL_DESYNC` |
| **post-panic wreckage**: after a caught `apply` panic at each of 6 window positions, every `undo` and every `delta` over a 17x17 neighbourhood on both colours, compared message-for-message | ~20 000 paired calls | identical |
| the same after a caught `undo` panic | 2 wreckage shapes x 578 calls | identical |

**Non-vacuity of that harness, demonstrated rather than asserted.** Two deliberate
breaks were made to the shipped store and the harness was re-run:

```
# key collapsed to 4 bits of q  ->  collisions inside a playout
assertion `left == right` failed: seed 1 fwd: delta -7,13 p1
test reviewer_track_e_random_playouts_are_bit_identical ... FAILED

# `set` keeps emptied entries (M1)
test reviewer_track_e_extreme_coordinates_are_bit_identical ... FAILED
test reviewer_track_e_random_playouts_are_bit_identical ... FAILED
```

Both restored; the harness is green on `07f518b`.

### The dispatch's three sharp questions, answered

**(a) Is "absent" still distinguishable from "present but empty", and can a
present-but-empty entry ever exist?**

**It cannot exist, and it could not exist in the incumbent either, so the
substitution is exact.** The only insertion sites are `entry_or_default` and
`set`. `set` removes an entry it empties (`value == V::default()`, which for
`Counts{p1: u8, p2: u8}` is exactly `Counts::is_empty()` — `total() == 0` — since
both fields are zero in `Default`). `entry_or_default` inserts a `Counts::default()`
and is followed **unconditionally** by `counts.add(player)`; the only escape between
them is `desync`, which fires on `before.total() >= 6` and a just-inserted default
has `total() == 0`. So the newly inserted entry is always made non-empty, and a
window that *does* trip the full-window check was already present and already full.
The incumbent's `entry(window).or_default()` in `apply` had the identical shape.
The 20 000-call post-panic sweep above is the empirical half of this: no divergence
at any of the six panic sites.

**(b) Does `apply` still insert a default entry before the full-window check, and
does the post-panic state still match?**

Yes and yes. Both versions insert-then-check, and — per (a) — the entry the check
can fire on is never the one just inserted. The post-panic sweep found the two
stores answer identically to every `undo` and every `delta` in the neighbourhood,
and `value` matches on both seats. The `Eval` doc already licenses post-panic state
to differ under `catch_unwind`; here it does not.

**(c) Any path where `HashMap` vs `BTreeMap` changes an observable?**

| observable | changed? |
|---|---|
| `value` / `delta` / panic messages | **no** (above) |
| `PartialEq` (D-498 canonical equality) | **no** — `HashMap` compares by length and per-key lookup; order-independent and history-independent, verified: an unwound eval equals a fresh one |
| `Clone` | **no** |
| iteration | not performed anywhere on any path |
| capacity | changed, and unobservable outside the crate's own `#[cfg(test)]` accessors |
| **`Debug`** | **YES** — the single observable the swap changes: order-dependent where the incumbent was canonical (MINOR 2) |

**The strongest reason to doubt Track E, stated plainly.** It is `Debug`, and it is
the same one the design review named. It does not flip the package: `Eval` has no
supertrait, so `pistol-search`, `pistol-engine`, `pistol-cli` and `pistol-arena`
cannot reach the concrete type's derives at all; the only renderings are the failure
branches of two `assert_eq!`s inside `pistol-eval`'s test crate; and for a *fixed
history* the rendering is still deterministic, so rule 4 is untouched. The
second-strongest is MAJOR 1 — which is a doubt about whether the shipped shape is
the *benched* shape, not about whether it gives the same answers. **Track E
survives, and the D-495 flip to Track S is not triggered.**

**Legs 2 and 3, which this review does not supply, have since been taken by the
implementing session and were checked here against their artifacts:**

- **Leg 2 — gate-off byte-identity — is DISCHARGED.**
  `artifacts/wp19_byte_identity_v1.txt`: two `--release --locked` binaries at
  `723758b` and `07f518b`, 44 positions across `tactical_staged_v0.txt` and
  `bench_positions_v1.txt`, both determinism-gate budgets, 422 output lines each,
  88 bestmove lines each, 0 error lines, `RESULT: IDENTICAL`, one digest for both.
  This is the D-484 two-binary diff, not a route through
  `search_oracle_check.sh`, as D-496 requires. **The design §4.2 phrase "the
  115-position receipt set" is a misnomer inherited from the scope memo, not a
  shortfall in the receipt**: `crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt`
  has 115 non-comment lines but its own header says it is *"the same twenty
  positions as tactical_v0.txt"* — 115 is a line count. The receipt covers the
  whole fixture. Worth correcting in the design so the next package does not go
  looking for 71 missing positions.
- **Leg 3 — the rule-5 bracket — is TAKEN and BELOW BRACKET**, which is MAJOR 1.

My leg-1 harness is independent evidence for leg 1's claim but it is not the
package's registered instrument, and the incremental-vs-rebuild agreement leg 1
registers is separately pinned by
`eval_incremental_matches_from_scratch_on_random_playouts`, which passes.

---

## §R — What I checked that held

- **`window_key` injectivity (I1) HOLDS over the whole addressable range**, and
  the shipped code has the narrowing cast the design calls load-bearing
  (`window_map.rs:28-29`, `(window.start.q as u16) ^ 0x8000`). `Coord`'s fields are
  `i16` (`crates/pistol-core/src/coord.rs:21-23`), so `as u16` is exact, not
  truncating, and the key is base-2^16 positional notation over
  `{0,1,2} x [0,2^16) x [0,2^16)` — three disjoint bit fields, invertible by
  construction. Verified independently in Python, not by re-reading the Rust:

  ```
  injectivity sample: 400143 distinct windows, 400143 distinct keys, collisions=0
  exhaustive q-sweep (3 axes x 65536 q x 2 extreme r): 393216 keys for 393216 windows
  order-preserving along the (axis,q,r) walk: True
  no-cast (ConstQ,-1,0) == 0xffffffff7fff8000  (ConstS,-1,0) == 0xffffffff7fff8000
  ```

  The last line reproduces the design's own claim: **without** the cast,
  `(ConstQ,-1,0)` and `(ConstS,-1,0)` collide exactly. The cast is load-bearing and
  it is present.

- **I2 (order-preservation) HOLDS** for the shipped arithmetic (same run). The
  *test* for it is weaker than the property — MINOR 1 — but the property is true.

- **The `WindowHasher` is sound, and `std`'s `HashMap` never reaches the panicking
  byte path.** `HashMap<u64, V, S>` hashes its key through `BuildHasher::hash_one`
  → `u64::hash` → `write_u64`, on every path: `get`, `entry`, `insert`, `remove`,
  resize/rehash, and `PartialEq` (which probes `other` per key). Nothing hashes a
  slice, a `str`, or a length prefix, so `write` — and with it every defaulted
  `write_i32` / `write_usize` / `write_length_prefix` — is unreachable while the key
  type is `u64`. Tested rather than argued: the shipped panicking `write` is its own
  detector, and a workload of 12 000 stones (216 000 window operations, forcing many
  resizes) plus 12 000 `delta` probes plus the full unwind ran clean and unwound to
  equality with a fresh eval. The `write` panic is a correct rule-3 guard against a
  future key-type change, not dead code.

- **The golden digests are the seedless function's own output.** All four
  reproduced independently in Python from `mixed = k * 0x9e3779b97f4a7c15;
  mixed ^ (mixed >> 32)`:

  ```
  0x0000000000000000 -> 0x0000000000000000  expected 0x0000000000000000  OK
  0x0000000000000001 -> 0x9e3779b9e17d05ac  expected 0x9e3779b9e17d05ac  OK
  0xffffffffffffffff -> 0x61c88646e17d05ad  expected 0x61c88646e17d05ad  OK
  0x9e3779b97f4a7c15 -> 0xdf442d22110c749b  expected 0xdf442d22110c749b  OK
  ```

  The hasher is seedless: `#[derive(Default)]` on a `u64` newtype, no `RandomState`,
  nothing read from the clock or the environment (I4, rule 4, D-32, D-498).

- **Mutation receipts M3 and M5 REPRODUCE exactly as `artifacts/wp19_mutations_v1.txt`
  claims**, both re-run from scratch in my own worktree at `07f518b` with
  `cargo test -p pistol-eval --locked --no-fail-fast`:

  - **M3, swap `q` and `r` in the packed key** (`axis << 32 | r << 16 | q`) — kills
    **only** `a_packed_key_orders_windows_the_way_the_window_type_does`, and **not**
    the collision test. This is right and it is the point: swapping the two fields
    is still a bijection, so nothing that depends on injectivity can see it. The
    receipt's claim that M3 isolates I2 from I1 is correct.
  - **M5, a per-process seed** — implemented as the design review's own shape, a
    `OnceLock` seeded from `SystemTime::now().subsec_nanos()` XORed into the key
    before the multiply. Kills **only**
    `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key`. Every other test in
    `pistol-eval` — the from-scratch agreement, the roundtrip, the delta oracle, the
    equality tests — is blind to it. **The golden form is load-bearing and design
    MAJOR 3's reasoning is confirmed**: a self-comparing "two fresh hashers agree"
    test would have passed this mutant.

- **Scope is clean.** `git show --stat 07f518b` touches three files, all in
  `crates/pistol-eval/src/`. No search change, no solver contact, no new eval term,
  no learned weight, no config, no committed document, no `pistol-api`. Hard rule 11
  untouched.

- **Hard rule 9.** `window_map.rs` 289 lines, `handcrafted.rs` 255 — both under the
  ~300 soft cap, so no `docs/rule9_justifications.md` entry is owed and none was
  added. Selection condition 1 (the storage lands in its own module, and inline
  would have taken `handcrafted.rs` to 315) is **discharged**.

- **D-115 compliance holds for all six in-source tests.** Every one is a guard on a
  private item: `window_key` and `WindowHasher` are `pub(crate)`, `WindowMap` is
  `pub(crate)`, and `len`/`capacity` are new `#[cfg(test)]`-only accessors — an
  addition, not a widening of an existing signature, and no item was made public to
  let a test reach it. `an_emptied_window_leaves_no_entry_behind` is the one at the
  boundary, since I5's observable consequence is already pinned in `tests/` by
  `eval_apply_undo_roundtrip`; it stays inside D-115 because what it asserts —
  that no *entry* survives — is invisible from outside the crate, which is the gap
  D-115 exists to close.

- **The `undo` desync-check substitution is exact.** `get()` + `before.is_empty()`
  answers what `Entry::Occupied` answered, for the reason given in §E(a), and the
  panic messages are byte-identical.

- **Mechanical law is clean**, in a worktree at `07f518b`:
  `cargo fmt --all --check` — no output, exit 0.
  `cargo clippy --workspace --all-targets --locked -- -D clippy::all` — clean
  through all seven crates.

- **`cargo test -p pistol-eval --locked --no-fail-fast`** at `07f518b`: green,
  `EXIT=0`, all six new in-source tests present and passing alongside the existing
  suites, including the three the design registers as already-existing pins
  (`eval_apply_undo_roundtrip`, `delta_leaves_the_eval_indistinguishable`,
  `eval_incremental_matches_from_scratch_on_random_playouts`).

- **`cargo test --workspace --locked --no-fail-fast`** at `07f518b`: 881 passed,
  7 failed — and **all seven failures are the documented environment hazard, not
  the change**. They are the `pistol-cli` suites that build their own scratch cargo
  workspaces (`solver_link_check_tests`, `solver_determinism_gate_tests`), which
  CLAUDE.md's Environment section says a shared `CARGO_TARGET_DIR` breaks: *"several
  gate-test suites build their own scratch cargo workspaces, and a shared target
  directory makes one fixture read another's dep-info."* The symptom matches
  exactly — `solver_link_check: NO source under crates/subject is an input to any
  shipped binary`, and the determinism script's stderr showing a rebuild of a
  *different* worktree's crates. **Control run**, the same two suites at
  `wp19/mx-base` under the same exported `CARGO_TARGET_DIR`:

  ```
  test result: FAILED. 11 passed; 8 failed; ...   (solver_link_check_tests)
  test result: FAILED. 0 passed; 1 failed; ...    (solver_determinism_gate_tests)
  EXIT=101
  ```

  The baseline fails a strict **superset** of what `07f518b` fails, so the change
  introduces no regression here. These suites are gated properly by `tools/ci.sh`,
  which does not export `CARGO_TARGET_DIR`; I did not re-run the full CI, and this
  review makes no claim about the 19 gates.

- **Selection conditions 2 and 3 are discharged.** The order-preservation test
  exists (weakly — MINOR 1) and the seedless-hasher test exists in the only form
  that can fail. The `windows` field's "Ordered" sentence and the crate-level
  "no hasher: the window bookkeeping is a `BTreeMap`" sentence are both corrected,
  as revision 2 required. Condition 1 is discharged (rule 9 above). Condition 4 is
  discharged in letter only (MAJOR 3). Condition 5 is Track E (§E).

- **Design §1.1's refusal of a feature gate is honoured**, and the implementation
  adds no config surface: hard rule 1 is untouched, `Budget` is untouched, no
  committed config changed.

---

## What one fix round must close

1. **MAJOR 1** — restore `undo` to one table probe per window, then re-take the
   bracket. Already independently found and registered as RUN 2 by the implementing
   session; run 1's below-bracket numbers stay recorded whatever run 2 says.
2. **MAJOR 2** — trim the false clause at `crates/pistol-core/src/window.rs:29-30`.
3. **MAJOR 3** — make the footprint test drain and assert retained capacity against
   a written-out `f(peak)`, per the governing review's own Fix.
4. **MINOR 1** — compare `a < b`, not `(a.axis, a.start) < (b.axis, b.start)`.
5. **MINOR 2** — narrow the `windows` field comment to what `lib.rs` already says
   correctly, or name `Debug` as the exception.
6. **MINOR 3** — delete the `delta` comment's closing meta-sentence.

None of these touches the mechanism, the key packing, the hasher, or the `Eval`
contract, and none of them is a correctness defect. **Track E holds.**
