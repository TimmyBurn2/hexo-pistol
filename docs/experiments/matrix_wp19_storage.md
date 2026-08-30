# MATRIX WP-19-S — the eval window-map storage shape (revision 2)

**Revision 2 is the one fix round the dispatch's cap allows.** Revision 1
(`d9ddcc5`) was attacked by a fresh-context DECISION-RED-TEAM
(`matrix_wp19_storage_REDTEAM.md`) and **FAILED** on three BLOCKING findings.
Every one of them reproduced when this session re-ran it, and none is argued
with below. What changed is stated in §0 so a reader can check the fix against
the finding rather than re-reading two documents.

**The decision.** What container `pistol-eval`'s `HandcraftedV0` uses for its
per-window bookkeeping, currently `BTreeMap<Window, Counts>`
(`crates/pistol-eval/src/handcrafted.rs:86`). Owed by D-497.

**Nothing is selected in this document.** The selection record is written after
this revision is re-reviewed.

**Every number is marked MEASURED or ESTIMATED (D-291)**, and every MEASURED
number names the artifact and the revision it came from.

---

## 0. What revision 1 got wrong, and what replaced it

| Finding | What revision 1 said | What this revision does |
|---|---|---|
| **B1** — the requirement rejecting O-4 exists nowhere in the tree | Quoted "memory footprint bound stated as a number with its derivation in a test" and attributed it to "the round" | **The ground is deleted.** The requirement is real but it is a DISPATCH requirement, now archived and citable at `docs/experiments/wp19_storage_DISPATCH.md`; it is not registered scope and it rejects nothing. **O-4 is now rejected on a committed test it fails and on a measurement.** |
| **B2** — O-2 does not state a memory bound in live entries | Recommendation ground 3 claimed it did | **Ground 3 is withdrawn and the true figures are measured** (§4.2). O-2 is bounded by historical peak, not live entries. |
| **B3** — ~1.3x headroom above O-2 is never computed | §6 argued O-3/O-4 could not beat O-2, from structure | **O-4 is MEASURED** (§4.1). It is *slower* than O-2. The headroom estimate is falsified by measurement, not by argument. |
| **M1** — D-498 is not a hard filter | §2 headed "The hard filter", table said "FAILS BY CONSTRUCTION" | **Reframed to what it is: a small, measured code cost** (§3), stated once, in the section that owns it (D-423). |
| **M2** — "no fourth call site" is false | Counted `PartialEq`, missed `Debug` | **§2 is re-derived from the derive list plus the grep**, and `Debug` is row 5. |
| **M3/M4** — memory row wrong and unmarked | Estimated bytes, ranking wrong | **Re-measured with a counting allocator at 120/256/386** and marked. |
| **M5** — the "outside check" is a repeatability check | Claimed the two runs' baseline agreement was an outside check | **Withdrawn.** The baseline binary is bit-identical across runs (`8dc2f92…`), so that was repeatability. A genuine outside check is now available and is reported in §4.3 — a prediction that failed. |
| **M6** — both candidates fail `cargo fmt` | Counted 273 / 308 | **Both re-formatted, re-committed and RE-BENCHED at the fmt-clean revisions.** True counts are 277 / 315. All governing numbers below are from the fmt-clean revisions. |

---

## 1. What the container must do — the observable surface

Derived two ways, because revision 1 derived it one way and missed a reader:
`/usr/bin/grep -rn "windows" crates/pistol-eval/src/` for the named call sites,
**plus the derive list on the struct**, which produces readers no grep for the
field name can find.

| # | Reader | Site | Shape |
|---|---|---|---|
| 1 | `entry().or_default()`, then update | `handcrafted.rs:130` (`apply`) | insert-or-update |
| 2 | `entry()`, `Entry::Occupied` or desync; removes when emptied | `handcrafted.rs:151`, removal `:168-172` (`undo`) | update-or-remove |
| 3 | `get().copied().unwrap_or_default()` | `handcrafted.rs:226` (`delta`) | read-only lookup |
| 4 | derived `PartialEq` / `Eq` over the whole carried state | `handcrafted.rs:81` | whole-map comparison |
| 5 | **derived `Debug`** | `handcrafted.rs:81` | whole-map traversal |

Rows 4 and 5 are both from the single `#[derive(Debug, Clone, PartialEq, Eq)]`.
Row 5 is **not** a correctness surface — it reaches only assertion output and
cannot influence move choice — but revision 1's claim that rows 1-3 were "the
whole observable surface" was false and is withdrawn.

**Nothing here is iterated on a choice path**, which is the half of D-225 that
survives and which D-498 confirms: rows 1-3 are point operations, rows 4-5 are
test and diagnostic. D-32 does not reach this map.

**Load.** `WINDOWS_PER_CELL` is 18 (3 axes x 6 offsets): one stone touches 18
windows on `apply`, on `undo` and on `delta`. Live occupancy is the 120-386
windows D-249 records (**quoted from the record, not measured here**).

**The key.** `Window { axis: Axis, start: Coord { q: i16, r: i16 } }`, derived
`Ord` = `(axis, q, r)` lexicographic; `crates/pistol-core/src/coord.rs:14-18`
makes `Coord`'s field order load-bearing and says so.

**Sizes, MEASURED** by `size_of` at `dba05ea`: `Window` **6**, `Coord` **4**,
`Axis` **1**, `(u64, Counts)` **16** (padded from 10).

---

## 2. The field

- **O-0 INCUMBENT** — `BTreeMap<Window, Counts>`.
- **O-1 PACKED-KEY TREE** — `BTreeMap<u64, Counts>`, key
  `axis << 32 | (q + 32768) << 16 | (r + 32768)`. **The option D-225 named and
  recommended evaluating first.**
- **O-2 HASHED** — `HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>`, same
  key, seedless multiply-xor hasher in-crate, no new dependency.
- **O-3 HAND-ROLLED OPEN ADDRESSING** — the packed key in a hand-written probing
  table. **D-225's `k = 4.1` row.**
- **O-4 DIRECT ADDRESSING** — a dense `Vec<Cell>` indexed by axis and by offset
  in a bounding box over window starts, rebased as the box grows. **No hashing
  and no probing: one bounds check, one multiply-add, one load.**

---

## 3. D-498's canonical-equality obligation — a cost, not a filter

Revision 1 called this a hard filter and wrote "FAILS BY CONSTRUCTION" against
two rows. **That was wrong and the red team was right.** D-498's failure clause
is conditional on the derive being retained:

> A store retaining tombstones that reach **the derived `PartialEq`** fails the
> obligation BY CONSTRUCTION.

D-498 nowhere requires the comparison to be derived. So **every row can satisfy
the obligation**; what differs is whether it is free or written.

**The cost is now MEASURED rather than asserted**, because this session wrote
one: O-4's `impl PartialEq` at `crates/pistol-eval/src/grid.rs` is **12 lines**
(live-count equality plus a per-live-entry lookup), and with it O-4 passes both
equality tests. `std`'s `HashMap` supplies the same shape for free.

**This is the only place in this document that states the obligation** (D-423).
The table below carries FREE or the measured line cost, and no other section
re-argues it.

---

## 4. The matrix

### 4.1 Performance — whole-engine, all MEASURED

The instrument is `tools/bench_delta.sh` (on D-289's DRIVEN list), which
satisfies D-499's carried obligations: it measures the **shipped structure**,
builds **both sides in one run** from named revisions in throwaway worktrees,
digests every binary, hoists construction out of the timed region, and asserts
per-position **node identity** under both budgets in every rep. Ratios are
candidate/baseline, larger is better. Baseline is `a5c5661` in every run.

| | **O-0** | **O-1 packed tree** | **O-2 hashed** | **O-3 open addressing** | **O-4 direct** |
|---|---|---|---|---|---|
| governing revision | `a5c5661` | `abf3d5d` | `9a986c6` | not implemented | `22bbd96` |
| **nps ratio, early** | 1.000 by definition | **1.198** M | **1.783** M | ESTIMATED, bounded above by O-4 | **1.737** M |
| **nps ratio, late** | 1.000 by definition | **1.242** M | **1.909** M | ESTIMATED, bounded above by O-4 | **1.837** M |
| **time-to-depth-2, early / late** | 1.000 | 1.228 / 1.263 M | 1.915 / 2.034 M | — | 1.805 / 1.920 M |
| **node identity** | n/a | **holds** M | **holds** M | — | **holds** M |
| artifact | — | `wp19_mx_bench_O1_fmt_v1.txt` | `wp19_mx_bench_O2_fmt_v1.txt` | — | `wp19_mx_bench_O4_v1.txt` |

`M` = MEASURED. All four runs exited 0.

**The ordering is O-2 > O-4 > O-1 > O-0, and the gap between O-2 and O-4 is the
finding of this revision.** O-4 performs no hashing and no probing at all; its
lookup is the cheapest arithmetic any keyed store can do. It is **slower than
O-2**. Per-lookup arithmetic is therefore not where the remaining time is, and
the storage layer has little left to give.

### 4.2 Memory — MEASURED with a counting global allocator

Revision 1's memory row was estimated and wrong. Re-measured at the occupancies
this document itself cites, on the real key and value types, `--release`:

| n live | O-0 peak / B-per-entry / retained | O-1 peak / B-per-entry / retained | O-2 peak / B-per-entry / retained |
|---|---|---|---|
| 120 | 2368 / 19.7 / 104 (4.4 %) | 2848 / 23.7 / 128 (4.5 %) | 6560 / 54.7 / **4368 (66.6 %)** |
| 256 | 4944 / 19.3 / 104 (2.1 %) | 5952 / 23.2 / 128 (2.2 %) | 13088 / 51.1 / **8720 (66.6 %)** |
| 386 | 7520 / 19.5 / 104 (1.4 %) | 9056 / 23.5 / 128 (1.4 %) | 13088 / 33.9 / **8720 (66.6 %)** |
| 4000 | 77984 / 19.5 / 104 (0.1 %) | 93920 / 23.5 / 128 (0.1 %) | 208928 / 52.2 / **139280 (66.7 %)** |

**Three things this says, stated plainly because revision 1 got the direction
wrong:**

1. **O-2 is the most expensive row per live entry**, 1.7x-2.8x O-0. O-1 is worse
   than O-0 too (the padded `u64` key costs more than the 6-byte `Window`).
   Revision 1's claim that O-1 was tied is withdrawn.
2. **O-2 does not release on unwind.** It holds ~66.6 % of peak BYTES at zero
   entries, at every occupancy, where both tree rows release ~99 %.
3. **The red team's figure and this one are the same measurement of different
   quantities, and both are right.** Its "139280 B, 100 % of peak" is *capacity*
   retention — the bucket array is never shrunk. This table's 66.7 % is *bytes*
   retained against peak BYTES, and peak bytes are higher because a resize holds
   the old and new arrays at once. `139280` appears in both, independently
   derived.

**Absolute magnitude, which is the part that decides:** at the real occupancy of
120-386 windows, O-2's peak is **6.5-13 KB**. The ratio is real; the quantity is
negligible against a 268 MB transposition table
(`tt_bytes 268435456`, from the bench's own identity block).

### 4.3 The outside check — a prediction that failed

Revision 1 claimed its two runs' baseline agreement was a check "outside" the
instrument. **It was not**: the baseline binary digest is `8dc2f92…` in both
runs, so that was the same binary measured twice — repeatability, withdrawn.

A genuine outside check now exists, and it is worth more than an agreement.
The red team fitted Amdahl to revision 1's two measured points under D-249's
recorded `k = 4.4-4.9`, obtained `p ~ 0.60`, and predicted **a ceiling of
2.47x-2.58x with ~1.3x still available above O-2**. That prediction was made
before O-4 was measured. **O-4 then measured 1.837, below O-2's 1.909.**

The prediction is falsified, and its failure is informative rather than
embarrassing: it inherited `k = 4.4-4.9`, which is exactly the table-only
figure **D-258 says may not be used this way**. D-258's warning is now
corroborated by a measurement in the direction it predicted.

### 4.4 Code cost — MEASURED at the fmt-clean revisions

| | O-0 | O-1 `abf3d5d` | O-2 `9a986c6` | O-4 `22bbd96` |
|---|---|---|---|---|
| `handcrafted.rs` lines | 255 | 277 | **315 — over rule 9's ~300 soft cap** | **267** |
| separate storage module | — | — | — | `grid.rs`, 184 |
| new dependency | — | none | none | none |

There is **no entry for `handcrafted.rs` in `docs/rule9_justifications.md`**
(MEASURED: `grep -c` returns 0). **O-4 demonstrates the remedy**: moving the
storage into its own module keeps `handcrafted.rs` at 267, under the cap,
without a justification entry. Whatever row is selected should land the same
way — a hasher and a key packing are not the evaluation. That is a design
decision, and the design step owns it.

---

## 5. Why O-4 is not selectable, on grounds that exist

Revision 1 rejected O-4 on an invented requirement. Both real grounds are
measured:

1. **It fails a committed test.** `eval_windows_stop_at_the_edge_of_the_addressable_lattice`
   (`crates/pistol-eval/tests/eval_invariant_tests.rs:40`) places stones at the
   corners of the `i16` lattice. O-4's box must then span the lattice: `w * h`
   **overflows `i32`** at `grid.rs:112` and the run panics. Even without the
   overflow the allocation is `3 x 65536^2 x 2 B` = **~25.8 GB**. The test's own
   comment says why it exists: *"an eval that panicked there would be a crash the
   type system invites, so it is checked."* This is not a tuning failure — it is
   `O(R^2)` in the played radius against `O(stones)` for every other row, on a
   lattice rule 1 declares unbounded.
2. **It is slower than O-2 anyway** (§4.1), so the speed case that motivated the
   row does not survive its own measurement.

O-4 was implemented, formatted, and run against the full suite to establish
this: it passes every `pistol-eval` test **except** the lattice-edge one, both
equality tests included.

## 6. Why O-3 is not implemented, stated as an argument with its limits

**This is the one substantive cell that remains ESTIMATED, and it is the
weakest thing in this document.**

The argument: O-3 must compute a hash and then probe. O-4 does neither and is
already below O-2. If the cheapest possible per-lookup arithmetic does not beat
O-2, arithmetic is not the binding cost, and a hand-rolled table that adds
hashing back cannot recover it.

**Where that argument is not airtight, said plainly:** O-4's per-lookup work is
cheapest but its *locality* is not — it scatters 18 indices across a sparse box,
where a probing table concentrates them in a smaller array. A hand-rolled table
could plausibly beat O-4. What it would have to beat is **O-2**, and O-2 is
hashbrown: SIMD group probing over a control array, which is a well-optimised
instance of exactly the structure O-3 proposes to hand-write.

**The remedy if this is judged load-bearing is to measure it, not to re-argue
it** (D-318's disposition, accepted). The cost is a probing table with deletion,
~120-160 lines plus its tests.

---

## 7. Recommendation

**O-2, the hashed row** — on one ground, because only one survived attack.

**It is the fastest shape measured**, 1.783 / 1.909, against the row D-225
recommended evaluating first at 1.198 / 1.242 and against a zero-hash direct
store at 1.737 / 1.837. Node identity holds in all three runs, IQR is far
inside the instrument's 10 % gate, and the fmt-clean re-runs reproduce the
first pair to within 0.5 %.

**The two grounds revision 1 also claimed are withdrawn**, and neither is
replaced:

- **D-498 is not a filter.** It is a ~12-line `impl` that O-3 and O-4 would each
  have to write and `std` supplies free. Real, small, and not decisive.
- **The memory ground is measured false in O-2's own disfavour.** O-2 is the
  *most* expensive row per live entry and the only one that does not release on
  unwind. It is selected **despite** its memory, not because of it, and the
  reason that is affordable is absolute magnitude — 6.5-13 KB at real
  occupancy — not a bound in live entries.

**What the record should say it means:** O-2 is the best **measured** shape.
It is not established as the best shape, because O-3 was never implemented.
§4.1 is the reason to think the gap is small — the zero-arithmetic floor is
already below O-2 — and §6 is honest that this is an argument.

**The strongest attack this recommendation must survive** is §6's, and it is
recorded here so the ADR line can carry it rather than a paraphrase: *O-3 is the
row D-225 actually named, it is the only row in the field never implemented, and
it is excluded by an argument from O-4's measurement rather than by a
measurement of its own.*
