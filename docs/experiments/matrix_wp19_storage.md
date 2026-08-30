# MATRIX WP-19-S — the eval window-map storage shape (revision 1)

**The decision.** What container `pistol-eval`'s `HandcraftedV0` uses for its
per-window bookkeeping, currently `BTreeMap<Window, Counts>`
(`crates/pistol-eval/src/handcrafted.rs:86`). Owed by D-497, which added this
step to the WP-1.9 round and struck `docs/ROADMAP.md:291-292`'s claim that
WP-1.5a's matrix already selected one.

**Nothing is selected in this document.** It is the field and the costs. The
selection record is written after a fresh-context DECISION-RED-TEAM has attacked
it (CLAUDE.md's matrix law), and the surviving option's ADR line records the
strongest surviving attack.

**Every number is marked MEASURED or ESTIMATED (D-291).** MEASURED numbers name
their run and its artifact. Two rows carry ESTIMATED performance cells and say
so in the open; §6 states exactly what it would cost to convert them and why
this revision did not.

---

## 1. What the container must do — the whole observable surface

Verified at `a5c5661` by `/usr/bin/grep -rn "windows" crates/pistol-eval/src/`.
`windows` is a private field, so this list is closed — **there is no fourth call
site.**

| # | Operation | Site | Shape |
|---|---|---|---|
| 1 | `entry().or_default()`, then update | `handcrafted.rs:130` (`apply`) | insert-or-update |
| 2 | `entry()`, `Entry::Occupied` or desync; removes when emptied | `handcrafted.rs:151`, removal at `:168-172` (`undo`) | update-or-remove |
| 3 | `get().copied().unwrap_or_default()` | `handcrafted.rs:226` (`delta`) | read-only lookup |
| 4 | **derived `PartialEq` over the whole carried state** | `handcrafted.rs:81` | **whole-map comparison** |

Row 4 is the one D-225 missed and D-498 narrowed the licence around. It is not
decorative: `eval_incremental_tests.rs:118-140` and `eval_delta_tests.rs:407`
are that comparison, and the `Eval` trait makes it contractual — *"a backend
whose whole state is comparable pins it as equality"* (D-214).

**Load.** `WINDOWS_PER_CELL` is 18 (3 axes x 6 offsets), so one stone touches
18 windows on each of `apply`, `undo` and `delta`. Live occupancy is the
120-386 windows D-249 records. The map is **never iterated on any choice path**
— rows 1-3 are point operations and row 4 is test-only — so D-32's determinism
argument is untouched by any row below (D-498 confirms this half of D-225
survives).

**The key.** `Window { axis: Axis, start: Coord { q: i16, r: i16 } }`, derived
`Ord` = `(axis, q, r)` lexicographic (`crates/pistol-core/src/coord.rs:14-18`
makes `Coord`'s field order load-bearing and says so).

**Sizes, MEASURED** by `size_of` at `dba05ea`: `Window` **6**, `Coord` **4**,
`Axis` **1**, `(u64, Counts)` **16** (padded from 10).

---

## 2. The hard filter: D-498's canonical-equality obligation

D-498 is not a preference cell, it is a **binding obligation** on every row:

> any store shape must yield iteration-order-independent and TOMBSTONE-independent
> comparison of carried state, pinned by the rotated-unwind test
> (`eval_incremental_tests.rs:118-140`) as the driving test; a store retaining
> tombstones that reach the derived `PartialEq` fails the obligation BY
> CONSTRUCTION.

The driving test unwinds a position twice — once in reverse, once rotated
(`stones.iter().skip(3).chain(stones.iter().take(3))`) — and both times compares
against a **fresh** eval, "emptied windows included". A row satisfies the
obligation either **FREE** (its `PartialEq` is already semantic) or **AT COST**
(a hand-written `PartialEq`, which is new code that itself needs a test, and
which removes the derive that currently makes the contract self-evident).

---

## 3. The field

- **O-0 INCUMBENT** — `BTreeMap<Window, Counts>`. The do-nothing row.
- **O-1 PACKED-KEY TREE** — `BTreeMap<u64, Counts>`, key
  `axis << 32 | (q + 32768) << 16 | (r + 32768)`. **This is the option D-225
  named and recommended evaluating first.** Order-preserving by construction, so
  iteration order and `PartialEq` semantics are identical to O-0.
- **O-2 HASHED** — `HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>`, same
  packed key, with a seedless multiply-xor hasher written in-crate (no new
  dependency).
- **O-3 HAND-ROLLED OPEN ADDRESSING** — the packed key in a hand-written probing
  table. **This is D-225's `k = 4.1` row.**
- **O-4 DIRECT ADDRESSING** — a dense `Vec<Counts>` indexed by axis and by
  offset within a bounding box over window starts, rebased as the box grows.

---

## 4. The matrix

Performance cells are **whole-engine nps and time-to-depth**, which is the
instrument `docs/ROADMAP.md:296` requires ("its bracket IS a whole-engine one")
and which D-499 binds to the shipped structure with the comparand in the same
run. Ratios are candidate/baseline, larger is better.

| | **O-0 incumbent** | **O-1 packed tree** | **O-2 hashed** | **O-3 open addressing** | **O-4 direct** |
|---|---|---|---|---|---|
| **nps ratio, early band** | 1.000 by definition | **1.202** MEASURED | **1.788** MEASURED | ESTIMATED ~O-2 | ESTIMATED >= O-2 |
| **nps ratio, late band** | 1.000 by definition | **1.239** MEASURED | **1.900** MEASURED | ESTIMATED ~O-2 | ESTIMATED >= O-2 |
| **time-to-depth-2 ratio** | 1.000 | 1.244 / 1.273 MEASURED | 1.892 / 2.008 MEASURED | ESTIMATED ~O-2 | ESTIMATED >= O-2 |
| **node identity held** | n/a | **yes** MEASURED | **yes** MEASURED | expected yes | expected yes |
| **D-498 canonical equality** | FREE (tree is canonical) | **FREE** MEASURED — both equality tests pass | **FREE** MEASURED — both equality tests pass; `HashMap`'s `PartialEq` is len + per-key lookup, order-independent | **FAILS BY CONSTRUCTION** — tombstones reach a derived `PartialEq`; costs a hand-written equality + its test | **FAILS BY CONSTRUCTION** — a grown box never compares equal to a fresh one; costs the same |
| **memory per live entry** | 8 B (`Window` 6 + `Counts` 2), tree nodes of 11 | 16 B padded — **larger than O-0** | 16 B padded + 1 control byte per bucket, at <= 87.5% load | ~16 B + tombstone slack | **unbounded in the played region**, not in live entries |
| **memory bound derivable?** | yes, in live entries | yes | yes | yes | **no static bound** — the lattice is unbounded (rule 1) and the box grows with the played region |
| **`handcrafted.rs` lines** | 255 MEASURED | 273 MEASURED | **308 MEASURED — over the ~300 soft cap** | est. 380+ | est. 380+ |
| **new dependency** | none | none | **none** (hasher is ~20 lines in-crate) | none | none |
| **rules-truth risk** | none | key packing is arithmetic over a pinned `Ord` | same | hand-written probing + deletion is new correctness surface | rebasing on growth is new correctness surface |

### The measured runs behind the performance cells

One instrument, `tools/bench_delta.sh`, which is on D-289's DRIVEN list and
which D-499's obligations are satisfied by: it measures the **shipped structure**
(the real `pistol` binary), builds **both sides in the same run** from named
revisions in throwaway worktrees, digests every binary it measures, hoists all
construction out of the timed region, and asserts per-position **node identity**
between the two binaries under both budgets in every rep.

- **O-1**: `tools/bench_delta.sh rev:a5c5661 rev:ecb2247 5` — artifact
  `artifacts/wp19_mx_bench_O1_v1.txt`, exit 0. Early nps 245960.8 -> 295732.8
  (IQR 906.1 / 869.8), late 203616.2 -> 252344.9 (IQR 906.8 / 1011.9).
- **O-2**: `tools/bench_delta.sh rev:a5c5661 rev:dba05ea 5` — artifact
  `artifacts/wp19_mx_bench_O2_v1.txt`, exit 0. Early nps 245960.8 -> 439818.8
  (IQR 1407.8 / 3212.8), late 203947.2 -> 387578.4 (IQR 744.4 / 1787.5).

**The two runs cross-check each other and the agreement is stated rather than
assumed:** the O-0 baseline is a separate build and a separate set of reps in
each, and its early-band median is 245960.8 in both, its late-band median
203947.2 against 203616.2 — **0.16 %** apart. That is the outside check D-499
carries over from D-258 ("check the instrument against something OUTSIDE it"),
and it is the only cross-check this document claims.

**What these runs do NOT establish.** `bench_delta.sh` prints its own
`VERDICT PASS` / `VERDICT BELOW-BRACKET` wording against **its own** `[1.4,
2.5]` thresholds, which descend from D-220's `Eval::delta` package. **Those are
not WP-1.9's bracket.** WP-1.9's bracket is registered post-implementation on
the selected option per D-483, and D-258's headline stands: the `k = 4.4-4.9`
figure may not be quoted as one. The verdict words are reproduced here only
because they are part of the instrument's own cited output.

---

## 5. Failure modes, row by row

- **O-0** — none new; the failure mode is the WP not happening. Its cost is the
  status quo the package exists to move.
- **O-1** — the packing arithmetic is a second statement of `Window`'s derived
  order. If `Coord`'s field order ever changes, `coord.rs:14-18` says the derived
  order changes with it and the packing silently disagrees. Mitigation is a test
  asserting the packing is order-preserving over a spread of windows, not a
  comment.
- **O-2** — same packing risk as O-1, plus: a hasher is a determinism surface.
  This one is **seedless by construction** (no `RandomState`, no
  environment-derived seed), which is what keeps it clear of rule 4 and D-32;
  the risk is a future edit reintroducing a seed. Mitigation is that the hasher
  refuses any key that is not a `u64` by panicking with the crate's named token
  rather than degrading quietly.
- **O-3** — two new correctness surfaces (probing and deletion) plus the D-498
  cost, and D-498's failure is the *quiet* kind if someone "repairs" it by
  comparing `p1_score` alone, which would make both equality tests and the
  trait's clause vacuous together.
- **O-4** — the rebasing path is new correctness surface, the D-498 cost is the
  same, and its memory has no static bound on an unbounded lattice. This last
  one collides with the WP's own deliverable: the round requires a memory
  footprint bound "stated as a number with its derivation in a test", and O-4 is
  the row that cannot supply one.

---

## 6. What is ESTIMATED, why, and what it would cost to convert

**O-3 and O-4 carry ESTIMATED performance cells. This is stated plainly because
D-291 makes an unmarked or lazily-estimated cell a finding.**

Neither could be measured "in seconds": each needs a correct implementation
first — O-3 a probing table with deletion, O-4 a growable rebasing index — at an
estimated 120-160 lines apiece plus their own correctness tests. That is the
stated ground for not measuring them in revision 1.

**The reasoning behind the estimates, offered so it can be attacked rather than
believed:**

- **O-3 is not a different structural row from O-2 — it is the same row
  hand-rolled.** `std::collections::HashMap` *is* an open-addressing table
  (hashbrown, SIMD group probing). So D-225's `k = 4.1` "open addressing against
  a packed btree" is, structurally, the O-2-vs-O-1 comparison, and this document
  measures that comparison whole-engine at 1.788/1.202 = **1.49x** early and
  1.900/1.239 = **1.53x** late. O-3's remaining claim is that a hand-rolled probe
  beats hashbrown's, which is a claim about implementation quality, not about
  structure.
- **O-4 is a genuinely distinct structural row and is the one most likely to
  dominate on speed alone** — no hashing, one indexed load. It is estimated at or
  above O-2 for exactly that reason. It is not recommended below, and the reason
  is not speed: it is the only row that cannot state a memory bound on an
  unbounded lattice, and it pays D-498 at cost.

**If the red team judges either estimate load-bearing for the selection, the
remedy is to measure it, not to re-argue it.** That is the disposition D-318
imposed and it is accepted here in advance.

---

## 7. Recommendation

**O-2, the hashed row.** The grounds, in the order they bear weight:

1. **It is the fastest thing measured**, by a wide margin over the row D-225
   recommended evaluating first — 1.788/1.900 against O-1's 1.202/1.239, on one
   instrument, both bands, node identity holding, IQR far inside the 10 % gate.
2. **It pays D-498 for free.** MEASURED, not argued: the whole eval suite
   including both equality tests passes at `dba05ea` unmodified. O-3 and O-4 both
   fail the obligation by construction and buy it back with hand-written
   equality that displaces the derive.
3. **It states a memory bound**, which the round has to deliver as a number with
   its derivation in a test, and which O-4 structurally cannot.
4. **It adds no dependency** and no rules-truth surface beyond the packing
   arithmetic O-1 already carries.

**The cost the recommendation does not hide:** O-2 takes
`crates/pistol-eval/src/handcrafted.rs` to **308 lines**, past hard rule 9's
~300 soft cap, and there is **no entry for that file in
`docs/rule9_justifications.md` today** (MEASURED: `grep -c handcrafted` returns
0). The answer should be a small dedicated module for the key packing and the
hasher rather than a justification entry — a hasher is not the evaluation, and
rule 9's own remedy of first resort is single responsibility. That is a design
decision the design step owns, not this matrix.

**The attack this author expects to be strongest**, named here so the red team
does not have to spend its round finding it: *the two rows that could beat O-2
on speed are the two rows that were not measured, and the grounds that defeat
them (D-498 and the memory bound) are both obligations this same session's
rulings created hours ago.* The honest answer is in §6 — if that estimate is
load-bearing, measure it.
