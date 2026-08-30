# MATRIX WP-19-S — DECISION-RED-TEAM, round 1

**Target.** `docs/experiments/matrix_wp19_storage.md` (revision 1) at
`d9ddcc59cb2f5b2457d262865dbcf0a70faadb20`.

**Does that revision match HEAD?** Yes.
`git rev-parse HEAD` -> `d9ddcc59cb2f5b2457d262865dbcf0a70faadb20`; working tree
clean (`git status --short` empty).

**Verdict: FAIL / STOP.**

Two independent grounds, each sufficient, in the D-318 / D-324 shape:

1. **The only non-D-498 ground on which the matrix rejects the row it itself
   calls "the one most likely to dominate on speed alone" (O-4) is a requirement
   that exists nowhere in this repository.** The matrix puts it in quotation
   marks and attributes it to "the WP's own deliverable" and "the round". Every
   occurrence of it in the tree is inside the matrix file. (B1)
2. **A decisive cell does not reproduce.** Recommendation ground 3 — "It states
   a memory bound ... in live entries" — is false for O-2, measured: a
   `HashMap<u64, Counts>` unwound to zero entries retains **100 %** of its peak
   heap, where both `BTreeMap` rows release **99.9 %**. O-2's footprint is a
   high-water mark over the search's history, which is the *kind* of
   unboundedness the matrix uses to disqualify O-4. And every number in the
   `memory per live entry` row is wrong by 1.4x-2.4x and is unmarked under
   D-291. (B2, M4)

Separately and importantly: **there is real headroom above O-2 and the matrix
never computes it.** Fitting Amdahl to the matrix's own two MEASURED points
under D-249's own recorded `k = 4.4-4.9` gives `p ~ 0.60` and a ceiling of
**2.47x-2.58x** whole-engine — i.e. **~1.30x-1.36x still available above O-2**.
That is not "a claim about implementation quality" (§6). It is the exact
quantity the selection turns on, and the document does not ask it. (B3)

The matrix pre-accepted this disposition in §6: *"If the red team judges either
estimate load-bearing for the selection, the remedy is to measure it, not to
re-argue it."* This round judges both load-bearing.

**What reproduced exactly is listed in §R at the end. It is a lot.** The
performance cells, the node identity, the `size_of` figures, the raw line
counts, the `grep -c` figure, the D-498 pass for O-2, the artifact digests and
O-1's packing arithmetic all reproduce verbatim or exactly. This is not a
document that made its numbers up; it is a document whose *disposal of the two
unmeasured rows* has no surviving ground.

Environment: all adjudicated greps run with `/usr/bin/grep` or `git grep` pinned
to a revision, `LC_ALL=C sort`. Builds and tests ran in
`/home/tom/wp19-rt` (`git worktree add --detach dba05ea`) with its own
`CARGO_TARGET_DIR`; a standalone allocator probe in `/home/tom/wp19-rt-probe`
depends on `pistol-core` by path. Nothing in the live tree was built, tested or
modified. `tools/bench_delta.sh` was **not** re-run — no finding here needs it,
and the two committed artifacts were checked against their receipts instead.

---

## BLOCKING

### B1 — The requirement that rejects O-4 does not exist

**Claim attacked.** §5: *"this last one collides with the WP's own deliverable:
the round requires a memory footprint bound `stated as a number with its
derivation in a test`, and O-4 is the row that cannot supply one."* And §7
ground 3: *"It states a memory bound, which the round has to deliver as a number
with its derivation in a test, and which O-4 structurally cannot."*

```
$ git grep -n -i "memory bound\|footprint bound\|memory footprint\|bound.*live entries" \
    d9ddcc5 -- docs/ configs/ tools/ crates/ | LC_ALL=C sort
d9ddcc5:docs/experiments/matrix_wp19_storage.md:105:| **memory per live entry** | ...
d9ddcc5:docs/experiments/matrix_wp19_storage.md:106:| **memory bound derivable?** | ...
d9ddcc5:docs/experiments/matrix_wp19_storage.md:166:  footprint bound "stated as a number with its derivation in a test", and O-4 is
d9ddcc5:docs/experiments/matrix_wp19_storage.md:195:  is not speed: it is the only row that cannot state a memory bound on an
d9ddcc5:docs/experiments/matrix_wp19_storage.md:215:3. **It states a memory bound**, which the round has to deliver as a number with
d9ddcc5:docs/experiments/matrix_wp19_storage.md:232:them (D-498 and the memory bound) are both obligations this same session's

$ git grep -n "derivation in a test" d9ddcc5 | LC_ALL=C sort
d9ddcc5:docs/experiments/matrix_wp19_storage.md:166:...
d9ddcc5:docs/experiments/matrix_wp19_storage.md:216:...
```

Six hits, all six inside the matrix. Checked the four places it could have come
from and it is in none of them:

- `docs/ROADMAP.md:290-298` (WP-1.9's entry) names exactly three obligations —
  *"its own option matrix, its own pre-registration and its own
  `tools/bench_delta.sh` run"*. No memory bound. (`sed -n '289,297p'`)
- `D-225` (`docs/decisions.md:492`) — no memory obligation; its only forward
  obligation is *"its own pre-registration under rule 5 — named hotspot,
  expected-gain bracket, abort threshold"*.
- `D-249` (`:540`), `D-496` (`:1060`), `D-497` (`:1062`), `D-498` (`:1064`),
  `D-499` (`:1066`) — none mentions memory.
- `docs/experiments/wp19_storage_scope_memo.md` — `git grep -n -i
  "memory\|bound"` against it returns **nothing at all**.

**What it means for the selection.** The matrix's own §7 closing attack says
*"the grounds that defeat them (D-498 and the memory bound) are both obligations
this same session's rulings created hours ago."* For D-498 that is true and
traceable: D-498 is a committed ADR line with standing. For "the memory bound"
it is false in a worse way — no ruling created it, it has no ADR line, no
ROADMAP line, no memo line. Describing it as an obligation a ruling created
lends it standing it does not have. Strip it and O-4's rejection rests on D-498
alone, which M1 below shows is worth about five lines of code. This is the
D-318 pattern exactly: the recommendation turns on a ground that was
manufactured rather than found, and the row it defeats is the one the document
concedes is most likely to win on the matrix's own trigger.

### B2 — O-2 does not state a memory bound in live entries. Measured.

**Claim attacked.** Table row `memory bound derivable?`: O-0 *"yes, in live
entries"*, O-1 *"yes"*, O-2 *"yes"*; O-4 *"no static bound"*. And §7 ground 3.

Probe (`/home/tom/wp19-rt-probe`, counting `GlobalAlloc`, release, real
`pistol_core::window::Window`): insert 4000 distinct windows, then remove all
4000.

```
== retained heap after full unwind (4000 entries inserted, all removed) ==
BTreeMap<Window,Counts>  peak 77984 B  held-at-zero 104 B  len 0
BTreeMap<u64,Counts>     peak 93920 B  held-at-zero 128 B  len 0
HashMap<u64,Counts>      peak 139280 B  held-at-zero 139280 B  len 0 capacity 7010
```

O-0 releases 99.87 %, O-1 releases 99.86 %, **O-2 releases nothing.** `std`'s
`HashMap` never shrinks on `remove`.

**What it means for the selection.** O-0 and O-1 really are bounded in live
entries; O-2 is bounded in the *historical peak* of live entries and holds that
peak for the life of the eval. The eval is long-lived — one `HandcraftedV0` is
built per `Searcher` (`crates/pistol-engine/src/instance.rs:140`) and is
`apply`/`undo`-ed across an entire search — so the retained figure is the
deepest line the search ever walked, not the current position. On an unbounded
lattice (rule 1) with no rule-level draw and a cap that is *"an evaluation
horizon, never a game rule"* (rule 6), the number of stones is unbounded, so
O-2's peak is unbounded too. It is asymptotically better than O-4 (linear in
stones versus quadratic in the played radius), but the matrix does not say that
— it states a categorical yes/no, and the categorical is false. Under D-424,
a distinction whose two sides license the same conclusion is not a distinction;
here the *stated* distinction does not even hold.

Recommendation ground 3 therefore does not stand as written, and it is the
second of the only two grounds that dispose of O-3 and O-4.

### B3 — The headroom above O-2 is material and is never computed

**Claim attacked.** §6: *"O-3's remaining claim is that a hand-rolled probe beats
hashbrown's, which is a claim about implementation quality, not about
structure."* And §7 ground 1's implicit premise that 1.788/1.900 is near the
ceiling.

The matrix has two MEASURED whole-engine points and the project has a recorded
micro-instrument `k`. D-249 (`docs/decisions.md:540`), quoted:

> k = 4.4-4.9 for its storage winner over the incumbent `BTreeMap<Window, Counts>`
> shape, over the real corpus at the real occupancy of 120-386 windows, three
> replications

Fitting `1/(1 - p + p/k)` to the late band's MEASURED 1.900:

```
$ python3 ...
  D-249 k=4.4 : implied p=0.6130  ceiling(k=inf)=2.584x  headroom over O-2 = 1.360x  implied k_O1=1.459
  D-249 k=4.65: implied p=0.6035  ceiling(k=inf)=2.522x  headroom over O-2 = 1.327x  implied k_O1=1.470
  D-249 k=4.9 : implied p=0.5951  ceiling(k=inf)=2.470x  headroom over O-2 = 1.300x  implied k_O1=1.480
```

The fit is self-consistent: at `p = 0.60, k = 4.65` Amdahl predicts **1.890x**
against O-2's measured 1.900x, and the same `p` implies `k_O1 ~ 1.47` for O-1,
squarely inside D-225's own standalone bracket for the packing change
(*"1.54x-1.89x on the lookup path and 1.25-1.30x on surgery"*). Two independent
MEASURED points and one recorded prior all land on the same `p`.

*(Marked ESTIMATED: `p` is inferred, not profiled. The lower bound is hard —
1.900 measured with `k` finite forces `p >= 1 - 1/1.900 = 0.4737`. If instead
`p` were D-223 §0.4's top of 0.48, the ceiling is 1.923x and the headroom is
1.2 %.)*

**What it means for the selection.** The project's own two recorded priors for
`p` disagree by a factor of thirty in what headroom remains: D-223 §0.4's 34-48 %
band says ~1 %, D-249's `k` band says ~30 %. The matrix consults neither. A row
that captures even half of a 30 % headroom beats O-2 by more than O-1 beats O-0.
Whether O-3 or O-4 can capture it is therefore a load-bearing question, not an
implementation-quality footnote, and it is unanswered.

The structural half of §6's argument is also weaker than stated. `std`'s
`HashMap` is open addressing, but hashbrown reads **two** arrays per lookup — a
control byte and a bucket — which my own measurement confirms exactly: at 386
live entries the probe reports `HashMap<u64> = 8720 B`, and
`512 buckets x (16 B pair + 1 B ctrl) + 16 B group tail = 8720`. A hand-rolled
linear-probe table over a dense `u64` key needs no control array and touches one
cache line. "The same row hand-rolled" understates a real structural difference.

---

## MAJOR

### M1 — D-498 is not a hard filter, and "FAILS BY CONSTRUCTION" contradicts §2

**Claim attacked.** §2's heading *"The hard filter: D-498's canonical-equality
obligation"*, the table cells *"FAILS BY CONSTRUCTION"* for O-3 and O-4, and §7
ground 2.

D-498's own words (`/usr/bin/grep -n "^D-498:" docs/decisions.md`), quoted:

> THE SURVIVING OBLIGATION IS CANONICAL EQUALITY: any store shape must yield
> iteration-order-independent and TOMBSTONE-independent comparison of carried
> state ... A store retaining tombstones that reach **the derived `PartialEq`**
> fails the obligation BY CONSTRUCTION.

The failure clause is conditional on the derive being retained. D-498 nowhere
requires the comparison to be derived. §2 of the matrix reads it correctly —
a row satisfies the obligation *"either FREE ... or AT COST"* — and then the
table says the same rows *"FAIL BY CONSTRUCTION"*. Those are different claims
about the same cells and both are in the document. This is D-423's defect
arriving on schedule: the D-498 obligation is stated five times (§2, the table
row, §5's O-3 and O-4 bullets, §7 ground 2, §7's closing attack) and two of the
five disagree.

What the cost actually is, measured. `std`'s `HashMap` passes D-498 free because
`std` wrote roughly five lines for it — `len` equality plus a per-key lookup.
The probe confirms both halves of the property:

```
== canonical equality probes ==
insertion-order-independent PartialEq: true
iteration order identical across the two histories: false
emptied-after-4000 == fresh: true  (grown capacity 7010, fresh capacity 0)
```

Note the middle line: O-2's iteration order *does* differ between two histories
with identical contents, and its equality is order-independent anyway, purely
because the comparison is written rather than derived. That is precisely what an
O-3 or O-4 `impl PartialEq` would do. A "hard filter" that every row passes, and
whose cost differential is a five-line `impl` plus its test, is not a filter.
Under D-424 it does not do the work the recommendation asks of it.

*(Note also that the stated reason is under-general: a tombstone-free
open-addressing table with backward-shift deletion still compares unequal under
a derive, because capacities differ. The conclusion is right; the reason given
is not the operative one.)*

### M2 — §1's "the whole observable surface ... there is no fourth call site" is false at HEAD

**Claim attacked.** §1: *"Verified at `a5c5661` by `/usr/bin/grep -rn "windows"
crates/pistol-eval/src/`. `windows` is a private field, so this list is closed —
**there is no fourth call site.**"*

`crates/pistol-eval/src/handcrafted.rs:81` at `a5c5661` is
`#[derive(Debug, Clone, PartialEq, Eq)]`. That derive contains **two** readers of
the map, not one. The matrix counts `PartialEq` (row 4) and does not count
`Debug`.

```
$ git grep -n "windows" a5c5661 -- crates/pistol-eval/src/ | LC_ALL=C sort
a5c5661:crates/pistol-eval/src/handcrafted.rs:101:            windows: BTreeMap::new(),
a5c5661:crates/pistol-eval/src/handcrafted.rs:130:                let counts = self.windows.entry(window).or_default();
a5c5661:crates/pistol-eval/src/handcrafted.rs:151:                let Entry::Occupied(mut slot) = self.windows.entry(window) else {
a5c5661:crates/pistol-eval/src/handcrafted.rs:226:            let before = self.windows.get(&window).copied().unwrap_or_default();
a5c5661:crates/pistol-eval/src/handcrafted.rs:84:    /// The windows holding at least one stone, ...
a5c5661:crates/pistol-eval/src/handcrafted.rs:86:    windows: BTreeMap<Window, Counts>,
   (+ window.rs / lib.rs re-export lines)
```

Line 81 is not in that output and cannot be — a derive macro reads every field
without naming one. **The cited instrument structurally cannot produce the
cited closed list.** The matrix knows this, because row 4 cites `:81` directly;
it nevertheless presents the grep as establishing closure.

Measured consequence of the missed reader:

```
== derived Debug over the map ==
fwd Debug == rev Debug: false          <- HashMap, same contents, two histories
btree fwd Debug == rev Debug: true     <- BTreeMap, same two histories
```

O-2 makes `HandcraftedV0`'s `Debug` output a function of insertion history.

**What it means for the selection.** Not a correctness break and not a rule-4
breach: `pub trait Eval` carries no `Debug` bound
(`crates/pistol-eval/src/eval.rs:44`), so `Box<dyn Eval>` cannot print one, and
the only reachable consumers are `assert_eq!`/`assert_ne!` failure messages in
`eval_incremental_tests.rs:116/129/140` and `eval_delta_tests.rs:407`. The cost
is that a *failing* equality test under O-2 prints a nondeterministic map. What
is blocking-adjacent is the method: this is the same enumeration technique, on
the same source line, that produced D-225's error — the scope memo's F3 even
quotes the whole derive, `Debug` included, and then analyses only `PartialEq`.
The design step must re-derive the observable surface from the **derive list
plus** the grep, and D-225's flip clause (*"Flips if `self.windows` ever gains an
iterating caller"*) should be recorded as having fired twice, not once.

### M3 — The `memory per live entry` row does not reproduce, and its ranking inverts at the document's own occupancy

**Claim attacked.** Table row: O-0 *"8 B (`Window` 6 + `Counts` 2), tree nodes of
11"*, O-1 *"16 B padded — larger than O-0"*, O-2 *"16 B padded + 1 control byte
per bucket, at <= 87.5 % load"*.

Those are element sizes, not per-live-entry costs. Measured with a counting
allocator at the occupancies §1 itself names (*"Live occupancy is the 120-386
windows D-249 records"*):

```
== live heap bytes per entry (real allocator) ==
n=  120  BTree<Window>=  2368B (19.73 B/e)  BTree<u64>=  2848B (23.73 B/e)  HashMap<u64>=  4368B (36.40 B/e)
n=  256  BTree<Window>=  4944B (19.31 B/e)  BTree<u64>=  5952B (23.25 B/e)  HashMap<u64>=  8720B (34.06 B/e)
n=  386  BTree<Window>=  7520B (19.48 B/e)  BTree<u64>=  9056B (23.46 B/e)  HashMap<u64>=  8720B (22.59 B/e)
n= 1000  BTree<Window>= 19672B (19.67 B/e)  BTree<u64>= 23680B (23.68 B/e)  HashMap<u64>= 34832B (34.83 B/e)
```

- O-0 is **19.3-19.7 B/entry**, not 8. `BTreeMap` nodes are about half full;
  the *"tree nodes of 11"* parenthetical gestures at this and does not pay for it.
- O-1 is **23.3-23.7 B/entry**, not 16.
- O-2 is **22.6-36.4 B/entry** depending on where in the doubling cycle the
  occupancy lands, not 17.

The row's implied ranking (O-0 8 < O-1 16 = O-2 16) inverts at 386, the top of
the document's own stated range: **O-0 19.48 < O-2 22.59 < O-1 23.46**. O-1 —
the row D-225 recommended evaluating first — is the *worst* of the three on
memory, and the matrix says it ties O-2.

D-291's second clause bites: I measured all of this in about thirty seconds. An
estimate that could have been measured in seconds is a finding, and these cells
are not even marked as estimates (M4).

### M4 — D-291 breach: the memory rows carry unmarked numbers

**Claim attacked.** The matrix's own preamble: *"Every number is marked MEASURED
or ESTIMATED (D-291)."*

Rows 1-4 and rows 7-8 of §4's table carry MEASURED/ESTIMATED markers. Rows 5-6
(`memory per live entry`, `memory bound derivable?`) and row 9 (`new dependency`,
*"~20 lines in-crate"*) carry numbers with **no marker at all**: 8, 6, 2, 11, 16,
16, 87.5, ~20. D-291's stated harm is exactly this shape:

> what the sentence forbids is a number that LOOKS measured and is not, because
> that is what a red-team cannot attack

Three of those eight are wrong by 1.4x-2.4x (M3) and one of the rows they sit in
is recommendation ground 3 (B2).

### M5 — The "outside check" is a repeatability check inside the instrument

**Claim attacked.** §4: *"That is the outside check D-499 carries over from D-258
(`check the instrument against something OUTSIDE it`), and it is the only
cross-check this document claims."*

The two runs used the same script, the same config, the same fixture, and — per
the artifacts' own digest lines — **the same baseline binary, byte for byte**:

```
$ /usr/bin/grep "baseline  rev" artifacts/wp19_mx_bench_O1_v1.txt artifacts/wp19_mx_bench_O2_v1.txt
O1: bench_delta: baseline  rev:a5c5661 -> .../base.pistol (8dc2f92249022ea76fe4a886e25d1bcd77924a57305d748351959efcf760a328)
O2: bench_delta: baseline  rev:a5c5661 -> .../base.pistol (8dc2f92249022ea76fe4a886e25d1bcd77924a57305d748351959efcf760a328)
```

One instrument measuring one binary twice and agreeing with itself is
**repeatability**, and it is a good thing to have shown. It is not a check
against anything outside the instrument. D-499 "BINDS" for WP-1.9 and carries
that obligation explicitly; it is unsatisfied, and the matrix's sentence says it
is the *only* cross-check claimed, so nothing else covers it.

An outside check is cheaply available and B3 is half of it: the Amdahl fit of
1.890x predicted against 1.900x measured, from D-249's independently-recorded
`k` and D-223's independently-recorded `p`, is a genuine external agreement the
document could have claimed and did not.

*(The identical early-band baseline median of 245960.8 in both runs is not
itself suspicious. `nps = 1000*nodes/time` with `time` an integer-ms band sum
(`tools/bench_delta.sh:401-412`), the observed IQR of 906 nps corresponds to a
~9 ms spread over ~2440 ms, so the five reps take on roughly ten distinct
values and two independent medians coinciding is unremarkable. The late-band
figures differ, as expected. The 0.16 % arithmetic is correct:
(203947.2 - 203616.2)/203616.2 = 0.1626 %.)*

### M6 — Both candidates fail `cargo fmt --check`, so the MEASURED line counts are of unshippable source

**Claim attacked.** *"`handcrafted.rs` lines | 255 MEASURED | 273 MEASURED | 308
MEASURED — over the ~300 soft cap"*, and §7's *"O-2 takes ... to 308 lines"*.

The raw counts reproduce exactly. But CLAUDE.md makes rustfmt *"mechanical law"*,
and neither candidate is rustfmt-clean:

```
$ cargo fmt --all -- --check          # in /home/tom/wp19-rt at dba05ea
Diff in .../handcrafted.rs:99:   (the panic! line in WindowHasher::write, over 100 cols)
Diff in .../handcrafted.rs:276:  (the self.windows.get(&window_key(window)).copied()... line in delta)
FMT_EXIT=1
```

Re-measured after `rustfmt --edition 2024`:

```
a5c5661 raw=255 rustfmt_clean=255      <- baseline is clean; the measurement method is sound
ecb2247 raw=273 rustfmt_clean=277
dba05ea raw=308 rustfmt_clean=315
```

The shippable figures are **277** and **315**, not 273 and 308. O-1's over-width
`delta` line is shared, so O-1 is affected too. `cargo clippy --workspace
--all-targets -- -D clippy::all` at `dba05ea` is clean (`CLIPPY_EXIT=0`).

Magnitude is small and the direction does not rescue any row — O-2 is still over
the cap and O-1 is still under it — but a MEASURED cell measured against the
wrong definition is a MEASURED cell that does not reproduce, and it means both
candidate commits would fail CI gate 1 as they stand.

---

## MINOR

### m1 — O-2 requires an edit in `pistol-core` that the `rules-truth risk` row prices at "same"

`crates/pistol-core/src/window.rs:29-30` at `dba05ea`, quoted:

> The ordering is `(axis, start)` with `start` lexicographic by `(q, r)` —
> derived, and deterministic, which is what lets the bookkeeping live in an
> **ordered map with no hasher anywhere near a value the engine plays on**
> (CLAUDE.md rule 4, docs/decisions.md D-32).

O-2 puts a hasher exactly there. That doc lives in the rules-truth crate and
asserts a rule-4 property O-2 falsifies; it needs an edit, and the matrix's
`rules-truth risk` cell for O-2 says only *"same"* (i.e. the packing arithmetic
O-1 already carries). Small, but it is a `pistol-core` edit the row does not
disclose.

### m2 — O-2's `windows` field doc still claims the map is ordered

```
$ git show dba05ea:crates/pistol-eval/src/handcrafted.rs | sed -n '137,139p'
    /// The windows holding at least one stone, and what they hold. Ordered, so
    /// nothing in this crate can make a value depend on iteration order.
    windows: HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>,
```

False as written at `dba05ea`, and it is the one comment in the file that
asserts the property D-498 governs. A candidate-patch defect, not a matrix
defect, but it is what a reader of `dba05ea` will meet.

### m3 — `WindowHasher::write`'s panic is unreachable and untested

The panic path is correct under rule 3 (loud, named token) and I could not reach
it: `std`'s `impl Hash for u64` calls `write_u64`, and the probe exercised
`entry`/`get`/`remove`/`clone`/`PartialEq`/`iter` over 500 keys with no panic,
while the full `pistol-eval` suite passes at `dba05ea`. It is therefore dead
code with no test, and `git grep -rn "window_key\|WindowHasher\|order_preserv"
dba05ea -- crates/pistol-eval/tests/` returns **nothing** — §5's own stated
mitigation for O-1 (*"a test asserting the packing is order-preserving over a
spread of windows, not a comment"*) is also unshipped in both candidates. That
is a design-step obligation, correctly located, and it is recorded here so the
design step inherits it.

### m4 — `docs/ROADMAP.md:292` still asserts what D-497 struck

D-497 says it *"strikes `docs/ROADMAP.md:291-292`'s ... claim as stale"*, and the
matrix's opening repeats that. `/usr/bin/grep -n "WP-1.5a's matrix selects"
docs/ROADMAP.md` returns line 292 at HEAD. The strike is an ADR-level one and
the text is unedited, so a reader arriving at the ROADMAP still finds the false
claim. Not this matrix's defect to fix, but it should not be described in the
past tense.

---

## Per-row verdict

| Row | Survives? | On what |
|---|---|---|
| **O-0 incumbent** | **Yes** | Its own cells are the ones that reproduce best in *relative* terms (it is the cheapest per live entry at every occupancy measured, 19.3-19.7 B/e, and it releases 99.87 % of peak on unwind). It is not selectable — the WP exists to move it — but it is the only row whose memory story the matrix tells accurately in direction, and it is a legitimate abort target if the round runs out of budget. |
| **O-1 packed tree** | **Yes, wounded** | Speed cells (1.202 / 1.239 / 1.244 / 1.273) reproduce verbatim. **Its packing arithmetic is sound and I verified it exhaustively, not by sampling**: zero order mismatches over the full `i16::MIN..=i16::MAX` sweep on `q` and on `r`, for all three axes, with the other coordinate pinned at `MIN`/-1/0/1/`MAX`, including the cross-axis boundary `ConstQ(MAX,MAX)=4294967295 < ConstR(MIN,MIN)=4294967296`. `(q as u16) ^ 0x8000` is the standard order-preserving bijection and it holds over negatives. Wounds: it is the **worst** row on memory at the document's own occupancy (M3), its `rustfmt`-clean size is 277 not 273 (M6), and its order-preservation test does not exist yet (m3). |
| **O-2 hashed** | **Yes on speed, no on its other two grounds** | Ground 1 reproduces exactly and is the strongest thing in the document. Ground 2 (D-498 free) survives as a real but ~5-line advantage, not a filter (M1) — I confirmed the property itself: order-independent `PartialEq`, `emptied-after-4000 == fresh: true` at capacity 7010 vs 0, and 31/31 `pistol-eval` tests green at `dba05ea`. Ground 3 (memory bound) is **falsified** (B2). Selectable only under the conditions below. |
| **O-3 open addressing** | **Not disposed of** | Its exclusion rests on (a) *"the same row hand-rolled"*, which understates hashbrown's two-array control-byte structure — confirmed by the layout arithmetic `512 x 17 + 16 = 8720 B` exactly matching the measured allocation — and (b) *"FAILS BY CONSTRUCTION"*, which contradicts §2 and D-498's own conditional wording (M1). With ~1.3x headroom on the table (B3), it is not disposed of. |
| **O-4 direct** | **Not disposed of; this is the stop** | The matrix calls it *"the one most likely to dominate on speed alone"* and rejects it on a requirement that exists nowhere in the tree (B1) plus a memory distinction the recommended row partly shares (B2). Its genuine disadvantage — `O(R^2)` in the played radius versus `O(stones)` — is real and is never stated; the matrix states a categorical instead. Must be measured or rejected on the asymptotic ground stated properly. |

---

## What to do

The matrix pre-committed to the remedy and it is the right one. Before a
selection record is written:

1. **Measure O-4**, whole-engine, on `tools/bench_delta.sh` against the same
   `a5c5661` baseline, and O-3 as a plain linear-probe table over the dense key
   (no control array), which is the variant §6's structural argument does not
   cover. If the matrix instead wants to reject O-4 without measuring, the ground
   must be the asymptotic one — `O(R^2)` versus `O(stones)` on an unbounded
   lattice — stated as such, with the manufactured "deliverable" deleted.
2. **Delete the invented requirement** in §5 and §7 ground 3, or produce the ADR
   line that creates it. It cannot stay in quotation marks attributed to "the
   round".
3. **Re-measure the memory row** per live entry with a real allocator at 120/256/386,
   mark every number MEASURED or ESTIMATED, and add the retained-capacity fact for
   O-2 (139280 B held at zero entries).
4. **Reconcile §2 and the table** on D-498 — one statement, in the section that
   owns it (D-423). "FAILS BY CONSTRUCTION" is not what D-498 says.
5. **Re-derive §1's observable surface** from the derive list plus the grep, and
   add `Debug` as row 5.
6. **Replace the "outside check"** with one that is outside — the Amdahl fit in
   B3 is available for free and agrees to 0.5 %.
7. `cargo fmt` both candidate commits before any line count is quoted again.

---

## §R — What reproduced exactly

Stated plainly, because most of the document does reproduce and a reader should
not infer otherwise from the length of the findings above.

- **Every performance cell**, verbatim from the artifacts: O-1 early nps
  245960.8 -> 295732.8 (IQR 906.1 / 869.8), late 203616.2 -> 252344.9
  (906.8 / 1011.9), ratios 1.202 / 1.239, ttd 1.244 / 1.273; O-2 early
  245960.8 -> 439818.8 (1407.8 / 3212.8), late 203947.2 -> 387578.4
  (744.4 / 1787.5), ratios 1.788 / 1.900, ttd 1.892 / 2.008. Both artifacts
  `EXIT=0`, both *"node identity holds per position, both budgets, all reps"*.
- **The artifact digests match their receipts.** `sha256sum` of both files equals
  `artifacts/wp19_mx_receipts_v1.txt` lines 1-2.
- **The instrument caveat is honest.** The `VERDICT PASS` / `VERDICT
  BELOW-BRACKET` wording is genuinely the script's own against its own
  `[1.4, 2.5]` (`tools/bench_delta.sh:452-455`), and §4 correctly refuses to read
  it as WP-1.9's bracket. This is D-483-compliant: no bracket is registered here.
- **The `size_of` figures**, measured against the real `pistol-core` types:
  `Window` 6, `Coord` 4, `Axis` 1, `(u64, Counts)` 16. (`Counts` 2,
  `(Window, Counts)` 8, both consistent.)
- **The raw line counts** 255 / 273 / 308 (see M6 for the rustfmt caveat).
- **`grep -c handcrafted docs/rule9_justifications.md` returns 0**, at HEAD and at
  `d9ddcc5`. There is no justification entry for that file.
- **D-498 is FREE for O-2, and it is measured, not argued.** `cargo test -p
  pistol-eval` at `dba05ea`: 6 + 3 + 4 + 2 + 9 + 7 = 31 tests, all pass, including
  `eval_incremental_tests` (3/3, the rotated-unwind test among them) and
  `eval_delta_tests` (6/6). `TEST_EXIT=0`. Independently, `HashMap`'s `PartialEq`
  is order- *and* capacity-independent as claimed.
- **O-1's packing is order-preserving over the whole `i16` range**, exhaustively
  (see the O-1 row above). Zero mismatches. No finding.
- **The hashbrown structural description is right**: `16 B padded + 1 control byte
  per bucket` is exactly the measured layout.
- **`WINDOWS_PER_CELL` is 18** (`crates/pistol-core/src/window.rs:9`,
  `Axis::ALL.len() * WINDOW_LEN`), and the *"120-386 windows"* figure is genuinely
  D-249's (`docs/decisions.md:540`, *"the real occupancy of 120-386 windows"*).
- **§6's derived ratios are correct**: 1.788/1.202 = 1.4875 -> 1.49;
  1.900/1.239 = 1.5335 -> 1.53.
- **The 0.16 % baseline agreement arithmetic is correct** (see M5 for what it is
  not).

---

## Strongest surviving attack

For quotation verbatim into the ADR line of whichever row is ultimately selected.
If O-2 is selected after the conditions above are met, this is the attack it
survives:

> O-2 is the fastest of the three rows that were measured, and it is the *only*
> row whose speed was ever compared to anything. Of the three grounds the matrix
> gave for preferring it, only that one survived attack: the D-498
> canonical-equality ground reduces to `std` having written a five-line
> `PartialEq` that O-3 and O-4 would each have to write for themselves — D-498's
> own text conditions its BY-CONSTRUCTION failure on the derive being retained,
> so no row actually fails it — and the memory-bound ground was measured false in
> the recommended row's own favour: a `HashMap<u64, Counts>` unwound from 4000
> entries to zero retains 139280 B, 100 % of its peak, where both `BTreeMap` rows
> release 99.9 %, so O-2's footprint is bounded by the historical peak and not
> "in live entries" as the matrix's table claims. The requirement that footprint
> bound was said to serve — quoted in the matrix as the round's own deliverable,
> "stated as a number with its derivation in a test" — exists nowhere in this
> repository outside the matrix that cites it. Strip both and nothing in the
> document disposes of O-4, which the matrix itself calls the row most likely to
> dominate on speed alone; and the headroom is not nominal — fitting Amdahl to
> the matrix's own two measured points under D-249's recorded `k = 4.4-4.9` gives
> `p ~ 0.60` and a ceiling of 2.47x-2.58x, leaving roughly 1.3x still on the
> table above O-2's 1.900x. O-2 is therefore selectable as the best *measured*
> shape and never as the best shape, and the record should say which of those it
> means.
