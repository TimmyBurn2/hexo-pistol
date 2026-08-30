# MATRIX WP-19-S — DECISION-RED-TEAM, round 2 (capped round)

**Target.** `docs/experiments/matrix_wp19_storage.md` **revision 2** at
`c9befe6b8deba59bc7d7fcfc3390adb4867803f2`.

**Does that revision match HEAD?** Yes.
`git rev-parse HEAD` -> `c9befe6b8deba59bc7d7fcfc3390adb4867803f2`, branch `dev`.
`git status --porcelain` prints nothing: the working tree is clean.

**Author.** This session did not author the matrix and did not write round 1's
report. Round 1 was read as evidence and three of its own claims were re-checked
against the code and the allocator; where round 1 was wrong that is recorded too
(§0 M3, §0 M5).

---

## VERDICT: **FAIL**

**One BLOCKING finding, stated narrowly.** It is not about the measurements.
Every measured cell in this document reproduced — the three bench runs to the
last digit, the memory table byte-for-byte on two of its three rows against an
independently written counting allocator, the O-4 test failure, the five line
counts, the `size_of` figures, the `grep -c` zero, and `cargo fmt --check` exit 0
at all three governing revisions. The measurement work in revision 2 is the best
in this series and §R says so at length.

The failure is this: **the recommendation rests on one ground — "it is the
fastest shape measured" — and the only row that could contest that ground on its
own terms, O-3, is disposed of by a table cell that the document's own §6
contradicts and by an argument whose central premise is false against the code
it cites.** Strip both and nothing in the document disposes of O-3.

The matrix registered the disposition for exactly this case, in its own words,
before the attack:

> **The remedy if this is judged load-bearing is to measure it, not to re-argue
> it** (D-318's disposition, accepted).

This round judges it load-bearing, on evidence rather than on assertion, and
takes the registered remedy. Under the dispatch's cap (D-481 precedent) a
document failing twice STOPS and returns to the architect. **The package stops.**

**What the architect is being handed is a live two-way choice, not a broken
document** — see "What the stop hands back" at the end.

**Environment.** Everything adjudicated here used `/usr/bin/grep` or `git grep`
pinned to a revision. Builds and tests ran in throwaway worktrees under
`/home/tom` (`wp19-rt2` at `22bbd96`, `wp19-rt2-abf3d5d`, `wp19-rt2-9a986c6`),
each with its own `CARGO_TARGET_DIR`; the allocator probe is a standalone crate
at `/home/tom/wp19-rt2-probe` depending on `pistol-core` by path. Nothing in the
live tree was built, tested or modified, and `/home/tom/wp19-mx` was not touched.
`tools/bench_delta.sh` was **not** re-run: no finding here needs it, and the
three committed artifacts were re-derived from their own text instead.

---

## §0 audit — is revision 2's fix table honest?

Finding by finding, against round 1's report and against the tree.

| Finding | Status | Why |
|---|---|---|
| **B1** the invented requirement | **CLOSED** | Genuinely deleted, and the archiving is a real fix rather than a relabelling. Verified below. |
| **B2** O-2's "memory bound in live entries" | **CLOSED** | Ground 3 withdrawn; direction now stated against the recommended row; figures reproduce. |
| **B3** the uncomputed headroom above O-2 | **PARTIAL** | A measurement replaced an argument, which is right — but it does not answer B3's question, and the answer it is presented as giving is the BLOCKING finding below. |
| **M1** D-498 is not a hard filter | **PARTIAL** | Reframed correctly as a cost. But §3's CLAIM-HOME sentence is false in two ways (MAJOR-2). |
| **M2** "no fourth call site" | **CLOSED** | §1 re-derived from the derive list plus the grep; `Debug` is row 5; every cited line verified at `a5c5661`. |
| **M3/M4** the memory row | **CLOSED, one label slip** | Re-measured with a real allocator and marked; reproduces against my own probe. The prose calls a peak-per-entry figure "per live entry" (MINOR-1). |
| **M5** the "outside check" | **REOPENED-DIFFERENTLY** | The false half is withdrawn. The replacement is not an outside check either, and D-499's obligation is now claimed satisfied by a list that omits it (MAJOR-3). |
| **M6** unformatted candidates | **CLOSED** | Verified: `cargo fmt --all -- --check` exits 0 at `abf3d5d`, `9a986c6` and `22bbd96`; counts 277/315/267 reproduce; all three re-benched. |

Round 1's four MINORs (m1-m4) appear in no row of §0's table and are not
acknowledged anywhere. m1's target claim (the `rules-truth risk` row saying
"same") was deleted with the row rather than answered, which disposes of it;
m2's target defect survives in the tree (MINOR-4).

### B1 — CLOSED, and the archiving is real

The test is whether the matrix still leans on the requirement anywhere. It does
not.

```
$ git grep -n -i "memory bound\|footprint bound\|memory footprint\|derivation in a test" \
    c9befe6 -- docs/ configs/ tools/ crates/ | LC_ALL=C sort
```

Inside `matrix_wp19_storage.md` there are exactly **two** hits, both at lines
26-27, both inside §0's fix table describing what was removed. §5, §6 and §7
contain none. The requirement itself now exists in the tree at
`docs/experiments/wp19_storage_DISPATCH.md:95-96`, tracked, citable at a SHA:

> Design decides and records, from the memo: storage layout; ... memory
> footprint bound stated as a number with its derivation in a test, not prose
> (D-483).

That is a DESIGN deliverable in the operator's dispatch, and the archive says so
in its own status block rather than smuggling it back in as scope. O-4's
rejection now rests on the committed test and the measurement, and I verified
both independently (§R). **CLOSED.**

One observation the architect may want, offered as a note and not a finding: even
had the requirement stood, it would not have rejected O-4. A dense box has a
perfectly derivable footprint — `grid.rs:181-183` at `22bbd96` ships
`box_bytes()` for exactly that purpose. The requirement was never O-4's problem;
the lattice-edge test is.

---

## BLOCKING

### BLOCKING-1 — O-3's disposal has no surviving support: the decision table's cell is contradicted by §6, and §6's premise is false against the code

**Claim attacked, first half.** §4.1, the performance table, the only row cell
that disposes of O-3:

> | **nps ratio, early** | ... | ESTIMATED, bounded above by O-4 |
> | **nps ratio, late** | ... | ESTIMATED, bounded above by O-4 |

**Claim attacked, second half.** §6, the argument that cell rests on:

> The argument: O-3 must compute a hash and then probe. O-4 does neither and is
> already below O-2. If the cheapest possible per-lookup arithmetic does not beat
> O-2, arithmetic is not the binding cost, and a hand-rolled table that adds
> hashing back cannot recover it.

**(a) The document contradicts its own cell, in the section that owns the
argument.** §6, four lines later:

> **Where that argument is not airtight, said plainly:** O-4's per-lookup work is
> cheapest but its *locality* is not — it scatters 18 indices across a sparse
> box, where a probing table concentrates them in a smaller array. **A
> hand-rolled table could plausibly beat O-4.**

§4.1 says O-3 <= O-4. §6 says O-3 may exceed O-4. One document, two incompatible
claims about the same cell, and the incompatible cell is in the table the single
surviving ground reads. This is precisely the defect the document invokes D-423
for twice in its own text.

**(b) The premise "O-4's per-lookup work is cheapest" is false against the code
it describes.** §2 states it as fact:

> **No hashing and no probing: one bounds check, one multiply-add, one load.**

The shipped lookup at the governing revision:

```
$ git show 22bbd96:crates/pistol-eval/src/grid.rs | cat -n | sed -n '70,84p'
    70	    fn inside(&self, q: i32, r: i32) -> bool {
    71	        q >= self.q0 && q < self.q0 + self.w && r >= self.r0 && r < self.r0 + self.h
    72	    }
    73
    74	    fn offset(&self, axis: i32, q: i32, r: i32) -> usize {
    75	        let plane = (self.w * self.h) as usize;
    76	        axis as usize * plane + (q - self.q0) as usize * self.h as usize + (r - self.r0) as usize
    77	    }
    78
    79	    fn get_at(&self, axis: i32, q: i32, r: i32) -> Cell {
    80	        if !self.inside(q, r) {
    81	            return (0, 0);
    82	        }
    83	        self.cells[self.offset(axis, q, r)]
    84	    }
```

One lookup is: **four comparisons with two adds** (`inside`), **two multiplies**
(`self.w * self.h` recomputed on every call, plus the `* self.h` stride), two
subtractions, two adds, and then a **bounds-checked** `Vec` index — not "one
bounds check, one multiply-add, one load". `plane` is a function of `w` and `h`,
which change only in `grow_to` (`#[cold]`, `#[inline(never)]`), and it is
recomputed per lookup rather than cached as a stride. A store that cached the
stride, or masked instead of branching four ways, would do strictly less
arithmetic than this. **O-4 as implemented is not the arithmetic floor.**

**(c) The two facts together destroy the inference.** §6's argument needs O-4 to
be a floor. §6 concedes it is not the locality floor; (b) shows it is not the
arithmetic floor either. A row that is a floor on neither axis bounds nothing.
O-4's 1.837 therefore says nothing about O-3, and §4.1's "bounded above by O-4"
has no support of any kind.

**(d) O-3 is dominated by neither measured row on the two axes the matrix itself
identifies.** O-2 (hashbrown) pays two arrays per lookup — a control byte and a
bucket; O-4 pays a sparse box and the arithmetic in (b). A linear-probe table
over the dense `u64` key has neither: no control array, no sparse box, one small
contiguous array. That is not a prediction that O-3 wins; it is the observation
that the field's fast rows differ by **2.6 % early and 3.9 % late** (§R) and the
unmeasured row sits between them on both axes with no bound in either direction.

**(e) The document's own conclusion overreaches on the same premise.** §4.1:

> Per-lookup arithmetic is therefore not where the remaining time is, and the
> storage layer has little left to give.

No whole-engine measurement in this document bounds the ceiling from above. The
only upper bounds available in this project are table-only `k` figures, which
D-258 and D-499 forbid quoting as brackets in exactly this package — and the
matrix correctly says so about round 1's fit in §4.3, then makes an unbounded
claim of the same kind in the opposite direction one section earlier. "Little
left to give" is an unsupported inference presented as a consequence of the
measurement.

**Consequence for the selection.** The recommendation rests on one ground, and
the ground is speed. On speed the field is incomplete by the row D-225 named —
*"open addressing against a PACKED btree measures k = 4.1"*, which D-249
independently reproduced at *"a floor-subtracted 4.07/4.20/4.17"* — and the
matrix's exclusion of that row does not survive. Applying D-323's own four
discriminators for selecting despite an incomplete field, every one points the
other way here:

| D-323's test for selecting anyway | D-323 (S-N) | Here (O-3) |
|---|---|---|
| the missing row dominates only a row already dead | yes | **no — it would dominate the survivor** |
| it is not costed against the survivor | not costed | **costed: "~120-160 lines plus its tests"** |
| it has no stated criterion | none | **stated: the same `bench_delta.sh` run** |
| it is red on a correct engine in its only written form | yes | **n/a** |

That is D-318's disposition, not D-323's, and D-318's words fit: *"Selecting one
of five on a ground that separates none of them is forcing a survivor under
another name"* — here the ground separates the measured rows cleanly, and the
defect is the mirror image the dispatch asked me to check for: **a single
surviving ground that does discriminate, applied to a field that is incomplete on
that exact ground.**

**Reproducer.** (a) is reproduced by reading `docs/experiments/matrix_wp19_storage.md`
at `c9befe6`, §4.1's O-3 column against §6's fourth paragraph. (b) is reproduced
by the `git show` above. Neither needs a build.

---

## MAJOR

### MAJOR-1 — §5's MEASURED line number does not reproduce: the overflow is at `grid.rs:129`, not `grid.rs:112`

**Claim attacked.** §5.1:

> O-4's box must then span the lattice: `w * h` **overflows `i32`** at
> `grid.rs:112` and the run panics.

**Run.** Worktree `/home/tom/wp19-rt2` at `22bbd96`, own `CARGO_TARGET_DIR`:

```
$ cargo test -p pistol-eval --locked --no-fail-fast
...
---- eval_windows_stop_at_the_edge_of_the_addressable_lattice stdout ----
thread '...' panicked at crates/pistol-eval/src/grid.rs:129:37:
attempt to multiply with overflow
test result: FAILED. 3 passed; 1 failed; ...
```

`grid.rs:129` is `cells: vec![(0, 0); 3 * (nw * nh) as usize],` and column 37 is
`nw * nh` — the multiply the matrix names. `grid.rs:112` is
`(self.q0 + self.w).max(q + GROWTH_SLACK + 1),`, an addition that cannot overflow
at these magnitudes and is not the panic site.

**Consequence for the selection.** None. The failure is real, the mechanism is
exactly the one described, and O-4's rejection stands on it. But this project has
a recorded pattern — D-291, D-318, D-324, three separate authoring sessions — of
MEASURED cells that do not reproduce, always inside a cell supporting the
author's own recommendation, and **this is a fourth instance, in the one section
that rejects a row, in the revision written to close three of them.** Recorded
here because D-318 ruled that the recurrence and not the instance is the finding.

Everything else in §5 reproduces: the test name and its `:40` line, its quoted
comment verbatim, the `~25.8 GB` arithmetic (`3 * 65536^2 * 2 = 25,769,803,776`
B), the `O(R^2)` characterisation (`cells` is `3 * w * h`, `w` and `h` spanning
the played extent), and the claim that O-4 passes everything else — 30 of 31
`pistol-eval` tests green, both equality assertions in the rotated-unwind test
(`eval_incremental_tests.rs:127-131` and `:140`) among them.

### MAJOR-2 — §3's CLAIM-HOME sentence is false in both of its halves

**Claim attacked.** §3:

> **This is the only place in this document that states the obligation** (D-423).
> The table below carries FREE or the measured line cost, and no other section
> re-argues it.

Both halves fail against the document itself.

1. **There is no such table.** §4.1 carries governing revision, nps, ttd, node
   identity and artifact; §4.2 carries memory; §4.4 carries line counts, storage
   module and dependency. No row anywhere in §4 carries "FREE" or a line cost for
   canonical equality. The sentence points a reader at a cell that does not
   exist, and the D-498 cost consequently appears in the document only as prose.
2. **§7 re-argues it.** *"**D-498 is not a filter.** It is a ~12-line `impl` that
   O-3 and O-4 would each have to write and `std` supplies free."*

The measured figure is also loose: the hand-written comparison at `22bbd96` is

```
$ git show 22bbd96:crates/pistol-eval/src/grid.rs | cat -n | sed -n '49,59p'
```

lines 49-59 inclusive — an `impl PartialEq` of 9 lines, a blank, and
`impl Eq for WindowGrid {}` — **11 lines**, not 12, and 10 if the blank is not
counted. Trivial in magnitude; noted because §3 presents it as the section's one
MEASURED contribution.

**Consequence for the selection.** None: D-498 is agreed by both rounds to be a
small cost rather than a filter, and no row turns on it. Recorded because the
sentence is the document's own claim of compliance with a named rule, made in the
paragraph that invokes it, and this project's record (D-423, D-424 part 3) treats
that shape as material.

### MAJOR-3 — D-499's obligations are claimed satisfied by a list that swaps out the two hardest ones, and the outside-check obligation is still undischarged

**Claim attacked.** §4.1:

> The instrument is `tools/bench_delta.sh` (on D-289's DRIVEN list), which
> satisfies D-499's carried obligations: it measures the **shipped structure**,
> builds **both sides in one run** from named revisions in throwaway worktrees,
> digests every binary, hoists construction out of the timed region, and asserts
> per-position **node identity** under both budgets in every rep.

D-499's five carried obligations, quoted (`docs/decisions.md:1066`):

> measure at the level of the SHIPPED STRUCTURE and not an isolated table; colour
> plies through `GameState` and never `i % 2`; hoist store construction OUT of the
> timed region; keep the comparand in the same run so the document can fail the
> decision it defends; and check the instrument against something OUTSIDE it.

The list in §4.1 carries three of the five (shipped structure, hoist, comparand
in the same run) and adds two things that are not D-499 obligations at all
(binary digests, node identity). **The two it omits are the colour-plies
obligation and the outside check** — and the outside check is the one D-258
recorded a burnt instrument for.

§4.3 is offered as the replacement:

> A genuine outside check now exists, and it is worth more than an agreement.

It is not one. What §4.3 records is a prediction built on `k = 4.4-4.9` being
falsified — and the document then explains, correctly, that the prior was
inadmissible in the first place: *"it inherited `k = 4.4-4.9`, which is exactly
the table-only figure **D-258 says may not be used this way**."* A comparand that
was already discredited by a standing ADR before the comparison carries no
information about the instrument when it disagrees. §4.3 is a genuine and useful
finding **about the prior**; it is not a check of the instrument against anything
outside it, and D-499's obligation remains open.

**Consequence for the selection.** None directly — the instrument is the same for
all three measured rows, so a systematic error in it would move them together.
It matters because the undischarged obligation is the one that would catch a
systematic error, and the field's decisive margin (2.6-3.9 %) is small enough
that a systematic error is the failure mode worth guarding.

### MAJOR-4 — §4.2's MEASURED numbers name no artifact and no revision, breaking the document's own preamble

**Claim attacked.** The preamble:

> **Every number is marked MEASURED or ESTIMATED (D-291)**, and every MEASURED
> number names the artifact and the revision it came from.

§4.2's sixteen figures are headed *"MEASURED with a counting global allocator"*
and name neither. There is no allocator-probe artifact in `artifacts/`
(`ls artifacts/ | grep wp19` lists three bench files, two superseded bench files,
a receipts file, three CI logs, a dry-run and a census — no memory probe), the
probe source is not committed, and no revision is cited.

**Consequence for the selection.** None: §4.2's numbers run against the
recommended row, and the recommendation explicitly does not rest on them.
Recorded because §4.2's figures are unreproducible **from the record** — I could
only re-derive them by writing a second probe, which I did (§R), and which is not
what "names the artifact and the revision" is supposed to make necessary.

---

## MINOR

### MINOR-1 — "per live entry" labels a peak-per-entry figure

§4.2's columns are `peak / B-per-entry / retained`, and `B-per-entry` is
`peak / n` — verified for all twelve cells. The prose then reads:

> **O-2 is the most expensive row per live entry**, 1.7x-2.8x O-0.

`1.7x-2.8x` is the peak-per-entry ratio (54.7/19.7 = 2.78 at n=120; 33.9/19.5 =
1.74 at n=386). Per **live** entry — the quantity the phrase names, and the one
round 1's M3 measured — O-2 is 4368/120 = 36.4 and 8720/386 = 22.6 against O-0's
18.9 and 19.0, i.e. **1.19x-1.93x**. The direction and the ranking are unchanged
at every occupancy and the header discloses the basis, so nothing turns on it;
the overstatement is against the recommended row.

### MINOR-2 — "All four runs exited 0" (§4.1) and "all three runs" (§7) count the same set differently

§4.1's table names three artifacts; three runs exist at the governing revisions;
§7 says *"Node identity holds in all three runs"*. "Four" has no referent
(counting the two superseded runs would give five). All five bench artifacts do
carry `EXIT=0`, verified.

### MINOR-3 — "reproduce the first pair to within 0.5 %" holds for nps and not for time-to-depth

§7's claim reproduces on the nps ratios: O-1 1.202 -> 1.198 (0.33 %) and
1.239 -> 1.242 (0.24 %); O-2 1.788 -> 1.783 (0.28 %) and 1.900 -> 1.909
(0.47 %). On time-to-depth the same pairs move by up to **1.29 %** (O-1
1.244 -> 1.228; O-2 2.008 -> 2.034). D-374's standing lesson makes ttd the
primary unit and nps the context, and §7 quotes only nps. The ordering is
invariant under either unit (ttd: O-2 1.915/2.034 > O-4 1.805/1.920 > O-1
1.228/1.263), so the recommendation is unaffected — but the run-to-run
reproducibility a reader should carry into the O-2/O-4 comparison is 1.3 %, not
0.5 %. I re-checked the margin against the larger figure and it still holds
(§R).

### MINOR-4 — round 1's m2 survives in the tree at both candidate revisions

`crates/pistol-eval/src/handcrafted.rs:84-86` at `22bbd96`:

```
    /// The windows holding at least one stone, and what they hold. Ordered, so
    /// nothing in this crate can make a value depend on iteration order.
    windows: WindowGrid,
```

False of a `WindowGrid` as it was false of a `HashMap`. A candidate-patch defect
rather than a matrix defect, carried here so the design step inherits it rather
than rediscovering it a third time.

---

## Per-row verdict

| Row | Verdict | On what |
|---|---|---|
| **O-0 incumbent** | **Stands as the abort target** | Its cells reproduce; it is the cheapest per live entry at every occupancy measured and retains only 1.4-4.6 % of peak on unwind. Not selectable — the package exists to move it. |
| **O-1 packed tree** | **Measured and beaten** | 1.198 / 1.242 nps, 1.228 / 1.263 ttd, node identity holds, fmt-clean at 277 lines. D-225's "evaluate it FIRST" is discharged. It is the worst row on memory at the document's own occupancy (23.2-23.7 B/e). No finding against it. |
| **O-2 hashed** | **Fastest measured; not selectable on this document** | Every performance cell reproduces exactly and the margin over O-4 is real (5-8x the within-run IQR, 3-4x the cross-run drift). Its two withdrawn grounds are withdrawn honestly, and §4.2 is measured against its own interest. It falls only because the single surviving ground is speed and the field is incomplete on speed (BLOCKING-1). |
| **O-3 open addressing** | **Still not disposed of, and now on stronger evidence than in round 1** | Round 1 said its exclusion understated hashbrown's structure. Revision 2's replacement exclusion is contradicted by its own §6 and rests on a premise the code falsifies. It is the row D-225 named, it is costed, it has a criterion, and it is bounded in neither direction. |
| **O-4 direct** | **Disposed of, correctly, on two measured grounds** | Round 1's stop is fully repaired. The lattice-edge test fails at `22bbd96` (reproduced, though at `grid.rs:129` and not `:112`), the `O(R^2)` ground is stated properly against rule 1's unbounded lattice, and it is measured slower than O-2 in the same instrument. This is the part of revision 2 that most clearly works. |

---

## What the stop hands back

The architect has a live two-way choice and the matrix is close enough to
selection that both branches are cheap. Stated because a stop that names no next
move wastes the round:

1. **Commission the O-3 measurement.** A linear-probe table over the same packed
   `u64` key, no control array, backward-shift or tombstone-free deletion, with a
   written `PartialEq` — the matrix costs it at *"~120-160 lines plus its tests"*
   — and one `tools/bench_delta.sh` run against the same `a5c5661` baseline,
   ~8 minutes. That closes the field on the exact ground the recommendation uses,
   and O-4 has already demonstrated the module-extraction shape it would land in
   (`grid.rs`, 184 lines, `handcrafted.rs` back under the cap at 267).
2. **Rule that O-2 is selected provisionally**, with O-3 registered as an owed
   measurement and a named flip trigger in the ADR line, on the ground that a
   1.783/1.909 measured improvement is worth banking now and the container is a
   private field behind a three-method surface that O-4 proved swappable. This is
   an architect's call about budget, not a matrix's call about evidence, which is
   why this round cannot make it.

Either way, four items should be corrected before a selection record quotes this
document: §4.1's O-3 cell (BLOCKING-1a), §5's `grid.rs:112` (MAJOR-1), §3's
CLAIM-HOME sentence (MAJOR-2), and §4.1's D-499 list (MAJOR-3).

---

## §R — What reproduced exactly

Stated at length, because almost everything in this document does reproduce and
the verdict above should not be read as doubting its measurements.

**The three bench runs, verbatim from the artifacts.** Every ratio in §4.1 is the
instrument's own printed line:

- O-1 `abf3d5d` (`wp19_mx_bench_O1_fmt_v1.txt`): early base 248089.0 (IQR
  1224.7) -> cand 297192.5 (IQR 587.3), **nps ratio 1.198**, ttd 156 -> 127 ms,
  **1.228**; late base 204445.6 (IQR 498.8) -> cand 254005.1 (IQR 385.5), **nps
  ratio 1.242**, ttd 235 -> 186 ms, **1.263**.
- O-2 `9a986c6` (`wp19_mx_bench_O2_fmt_v1.txt`): early base 246868.4 (IQR 608.3)
  -> cand 440140.4 (IQR 2570.2), **1.783**, ttd 157 -> 82 ms, **1.915**; late
  base 203616.2 (IQR 1811.4) -> cand 388779.2 (IQR 905.6), **1.909**, ttd
  240 -> 118 ms, **2.034**.
- O-4 `22bbd96` (`wp19_mx_bench_O4_v1.txt`): early base 245860.4 (IQR 1505.3) ->
  cand 427029.8 (IQR 604.9), **1.737**, ttd 157 -> 87 ms, **1.805**; late base
  203286.4 (IQR 740.2) -> cand 373447.9 (IQR 1934.9), **1.837**, ttd
  240 -> 125 ms, **1.920**.

All three carry `bench_delta: node identity holds per position, both budgets, all
reps` and `EXIT=0`; so do both superseded runs. The baseline binary digest is
`8dc2f92…` in all five. `tt_bytes 268435456` is the identity block's own line.

**The O-2 > O-4 margin is real, checked three ways.** The candidate nps gap is
13110.6 early and 15331.3 late, against a largest within-run IQR of 2570.2 and
1934.9 — **5.1x and 7.9x**. The three runs' baselines agree to 0.91 % (early) and
0.57 % (late), so cross-run drift is about a third of the ordering gap of 2.6 %
and 3.9 %. Time-to-depth separates them by 5 ms and 7 ms at IQRs of 1.0 and 0.0.
Re-checked against MINOR-3's true 1.3 % ttd reproducibility rather than §7's
claimed 0.5 %: ttd gaps of 5.7 % and 5.9 % are still 4.5x that. Every IQR in
every band is under 0.6 % of its median, far inside `tools/bench_delta.sh:437-444`'s
10 % NOISY gate. **The ordering O-2 > O-4 > O-1 > O-0 is not a noise artifact.**

**§4.2's memory table reproduces**, against a counting `GlobalAlloc` I wrote from
scratch (`/home/tom/wp19-rt2-probe`, `--release`, real `pistol_core::Window`):

```
n=120   O-1 peak 2848 B/e 23.7 retained 128 (4.5%)   O-2 peak 6560   B/e 54.7 retained 4368   (66.6%)
n=256   O-1 peak 5952 B/e 23.2 retained 128 (2.2%)   O-2 peak 13088  B/e 51.1 retained 8720   (66.6%)
n=386   O-1 peak 9056 B/e 23.5 retained 128 (1.4%)   O-2 peak 13088  B/e 33.9 retained 8720   (66.6%)
n=4000  O-1 peak 93920 B/e 23.5 retained 128 (0.1%)  O-2 peak 208928 B/e 52.2 retained 139280 (66.7%)
```

**Byte-identical to §4.2 on the O-1 and O-2 rows, all sixteen figures.** The O-0
row differs by 104-200 B at n=120/256/386 (2264/4744/7320 against the matrix's
2368/4944/7520) and is **exact at n=4000** (77984/104) — `BTreeMap` node fill
depends on key distribution and my window enumeration is not the matrix's, so
this is my probe and not a discrepancy in the document.

**§4.2's reconciliation of the two 139280s is correct and I derived it a third
way.** `HashMap`'s array at capacity 8192 buckets is `8192*(16+1)+16 = 139280` B;
the previous 4096-bucket array is `4096*17+16 = 69648` B; a resize holds both, so
peak is `139280+69648 = 208928` and `139280/208928 = 66.67 %`. Round 1's "100 %
of peak" is the same 139280 measured without the transient overlap. Both figures
are right about different quantities, exactly as §4.2 says.

**O-4's test failure, and only that failure.** `cargo test -p pistol-eval
--locked --no-fail-fast` at `22bbd96`: 6 + 3 + 4 + 2 + 9 + 7 = **31 tests, 30
pass, 1 fails** — `eval_windows_stop_at_the_edge_of_the_addressable_lattice`,
`attempt to multiply with overflow`. `eval_incremental_tests` 3/3 including
`eval_apply_undo_roundtrip`, which carries both `assert_eq!(eval, fresh, …)`
assertions D-498 names as its driving test; `eval_delta_tests` 6/6. §5's "passes
every `pistol-eval` test **except** the lattice-edge one, both equality tests
included" is exactly right.

**§4.4's line counts, all five.**

```
a5c5661 handcrafted.rs 255   abf3d5d 277   9a986c6 315   22bbd96 267   22bbd96 grid.rs 184
```

**M6's fix is real.** `cargo fmt --all -- --check` exits **0** at all three
governing revisions (`abf3d5d`, `9a986c6`, `22bbd96`), each in its own detached
worktree.

**`grep -c handcrafted docs/rule9_justifications.md` returns 0**, at HEAD and at
`c9befe6`. §4.4's remedy claim is sound: 267 + 184 both sit under the ~300 soft
cap with no justification entry owed.

**The `size_of` figures**, measured against the real types: `Window` 6, `Coord`
4, `Axis` 1, `Counts` 2, `(u64, Counts)` 16, `(Window, Counts)` 8.

**§1's re-derived observable surface.** Every cited line is what the document says
it is at `a5c5661`: `:81` is `#[derive(Debug, Clone, PartialEq, Eq)]`, `:86` the
field, `:130` `entry(window).or_default()`, `:151` the `Entry::Occupied` bind,
`:168-172` the emptied-window removal, `:226` `get(&window).copied().unwrap_or_default()`.
M2 is properly closed, and the two-instrument derivation (derive list **plus**
grep) is the right repair.

**B1's archive is tracked and citable.** `wp19_storage_DISPATCH.md` exists at
`c9befe6` and carries the disputed sentence at `:95-96`; the matrix leans on it
nowhere outside §0's account of the fix.

**§4.3 is honest about what it withdraws.** The baseline binary really is
`8dc2f92…` in both round-1 runs, so round 1's M5 was correct and revision 2's
withdrawal of the "outside check" claim is correct. My finding is only about what
replaced it.

**§5's arithmetic and characterisation.** `3 * 65536^2 * 2 B = 25,769,803,776 B`
— "~25.8 GB" is right in decimal GB. `O(R^2)` is right: `cells` is
`3 * (nw * nh)` over a box spanning the played extent, against `O(stones)` for
every other row on a lattice rule 1 declares unbounded.

---

## Strongest surviving attack

Not offered — the verdict is FAIL and no row is selectable on this document. If
the architect takes branch 2 above and rules O-2 selected provisionally, the
attack its ADR line must carry, phrased for verbatim quotation, is:

> O-2 is the fastest shape ever measured against anything, and the record should
> say only that. The one row that could contest it on the one ground the
> selection uses — O-3, the hand-rolled probing table D-225 named and D-249
> reproduced at a table-only 4.07-4.20x over a packed tree — was never
> implemented, and the matrix's two attempts to exclude it both fail: §4.1's cell
> "bounded above by O-4" is contradicted four paragraphs later by §6's own
> concession that "a hand-rolled table could plausibly beat O-4", and §6's
> argument needs O-4 to be the arithmetic floor when the shipped lookup at
> `22bbd96` is four comparisons, two multiplies including a `w * h` recomputed on
> every call, and a bounds-checked index — not the "one bounds check, one
> multiply-add, one load" §2 claims. A row that is a floor on neither arithmetic
> nor locality bounds nothing, so O-4's 1.837 says nothing about O-3, and "the
> storage layer has little left to give" is an inference the measurements do not
> carry. O-2 is selected on a field that is incomplete on the exact axis it is
> selected for.
