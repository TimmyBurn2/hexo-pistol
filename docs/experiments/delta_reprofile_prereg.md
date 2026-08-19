# Post-delta profile — OBSERVATION RECORD (adjudicating hypotheses RETIRED)

**REVISION 3. Supersedes revisions 1 and 2. This document registers no
threshold and adjudicates nothing.** Revisions 1 and 2 pre-registered a profile
to adjudicate two hypotheses (H1' residual eval share, H3 window storage). Six
fresh-context rounds in one session — one governing-revision review and five
red-teams, the seventh on this document counting the review that failed
revision 1 — established that the profile cannot adjudicate what
it registers and that its licensing answer is already determined by the record.
The operator retired the adjudicating version on 2026-08-19; what remains is an
observation profile whose numbers feed WP-1.5a's design and license nothing.

Successor to WP-1.3 Run 2 (D-192, D-193). Registered under D-216, retired under
D-223.

## 0. Why the adjudicating version was retired

Six findings, each reproduced by at least two independent fresh contexts and
re-verified in the dispatching session. They are recorded here rather than in
the ADR line alone, because the next session to reach for a profile on this
engine will meet all six again.

**0.1 The registered quantity does not contain the cost it was about.** H3-e was
the share of samples whose stack carries an `alloc::collections::btree::` frame
instantiated over the eval's map types. In the release build there are **zero**
`get` and **zero** `entry` instantiations over `<Window, Counts>`: the seven
surviving eval-family symbols are `map::entry::Entry` (1), `OccupiedEntry` (2),
`VacantEntry` (1) and `node::BalancingContext` (3) — insert, split, steal, merge
and remove. The search *descent* is inlined into `delta`'s `get()`, into
`apply`'s `entry()` and into `undo`'s removal alike. **H3-e therefore counts
btree node RESTRUCTURING, not map ACCESS**, and no threshold on it adjudicates
the storage question. `V_delta ≡ 0` is provable from the symbol table before any
run, so §4.5's precondition could never pass and the §4.6 probe was never a
fallback — it was the certain path.

**0.2 The proposed repair renames every frame the document keyed on.** Building
with `debug = "line-tables-only"` and dropping `--no-inline` does make the
descent visible: inside `delta`'s body the line tables resolve `search_tree`,
`get`, `search_node`, `find_key_index` and `cmp`, with 81 resolved frames
carrying the literal `pistol_eval::window::Window`. But `perf script` renders
DWARF-resolved frames with their short `DW_AT_name`, not their demangled path:
`pistol_search::ordering::order` prints as `order (inlined)`. Measured on one
`perf.data`, caller-keyed literals go from **99.4% to 0.00%** when the flag is
dropped. Every caller test in the old §4.2, §4.4 and §5 reads zero under the
instrument meant to rescue them.

**0.3 That instrument is also inaccurate, and silently.** On a calibrated
synthetic with the same shape and flags, inline-expanded attribution reported
**0.00% for a component that was 76% of runtime** — LLVM had fully unrolled the
loop, and the unrolled instructions lost both their `DW_TAG_inlined_subroutine`
record and their line number (479 of ~824 code bytes resolving to line 0). There
is no diagnostic. `debug = 2` does not fix it. Where the records do survive the
btree share is biased **low by 4 to 8 percentage points** against an independent
outlined reference. In pistol's own `delta`, **62 of 296 instructions carry no
line information** — attribution dark matter inside the adjudicating function.

**0.4 The licensing answer was already determined.** From facts in the record
and no profiling: `delta` sits on ~56.5% of samples, and its body is 18
`BTreeMap::get` calls (`WINDOWS_PER_CELL` = 3 axes x `WINDOW_LEN` 6) against a
map of a few hundred entries, versus roughly 15-20 straight-line operations of
other work per window. That predicts an eval-map share of **34-48%**. For it to
fall under a 15% bar the within-`delta` map share would have to drop below
26.5%, which a calibrated synthetic needed ~450 extra ALU operations per window
to achieve. **P(H3-e >= 15%) > 0.95 before the run**, so no bar registered now
could fail, and registering one would have been the contamination the
pre-registration existed to prevent.

**0.5 H1'-a is redundant with the official bench, in the document's own words.**
Revision 2's §2.1 already conceded that a below-bracket bench and a confirmed
H1'-a are "one fact reported by two instruments, not two independent findings".
D-215's bench measures the delta speedup directly and better. H1'-a is retired
rather than replicated; `BAR_a`, `S_min`, `M` and the whole Amdahl apparatus go
with it.

**0.6 One run per binary was never enough for the margin it claimed.** Eight
replications of the residual on a quiesced machine give SD **0.457 pp** over a
range of 1.24 pp (55.96-57.20), and background load moves the quantity ~+1 pp;
a separate single run read 58.69. Revisions 1 and 2 took **one** run per binary
and covered the gap with a theoretical 3.0 pp margin, while D-215's own bench
discipline requires >= 5 IQR-gated repetitions for a far cheaper claim.

**0.7 What survived the attack, and is worth keeping.** The 15% bar was, by
measurement, correct — and better founded than the argument that produced it.
Standalone benchmarks of the option matrix's own candidates on this CPU give
k = 7.2-7.5 (open addressing, fixed-seed hash) and k = 7.9-8.5 (dense
ring-indexed array) on the lookup path. Solving `1/(1 - p + p/k) >= 1.15` for
the required share returns **exactly 15.00% at k = 7.67**, and every defensible
k puts the bar within [14.8%, 17.4%] — a span narrower than the document's own
3.0 pp instrument margin. A proposal to replace 15% with a payoff-derived bar at
`k_floor = 3` was rejected: `k_floor = 3` is outside the measured range, and at
a true share of 18% the proposed rule would have REFUTED work whose measured
payoff is 1.184x.

## 1. What is licensed on the record, without this profile

- **`Eval::delta` delivered.** Dev-machine bench ~2.0-2.2x nps on both bands
  (D-215, advisory); the official operator verdict lands as D-220.
- **Eval-under-ordering remains a hotspot.** The residual sits at ~56.5% of
  samples, far above H1'-b's 20% line. This is an OBSERVATION, not an
  adjudicated confirmation, and it licenses further eval-side work exactly as
  D-192's bar did.
- **The window map is the dominant cost inside `delta`** by the §0.4 prediction,
  and its replacements are measured (§0.7). **WP-1.5a is licensed to open its
  D-196 option matrix** on that basis rather than on a profile threshold.
- **The matrix must carry an option the earlier list omitted.** Packing `Window`
  (`{ axis: Axis, start: Coord { q: i16, r: i16 } }`) into an order-preserving
  `u64` key — `axis << 32 | (q + 32768) << 16 | (r + 32768)`, exactly the
  derived `(axis, q, r)` lexicographic order — measures **1.54x to 1.89x on the
  lookup path** and 1.25-1.30x on surgery in a standalone bench, for roughly
  twenty lines and no structural change.
- **The determinism law does not constrain this map.** `self.windows` is used in
  exactly three operations — `entry()` in `apply`, `entry()` in `undo`, `get()`
  in `delta` — and is **never iterated**. Its ordering is unobservable, so an
  unordered or hashed replacement cannot affect move choice. D-32's
  no-hasher-near-a-played-value rule is about `Board` occupancy, which IS
  iterated; it does not reach the eval's window map. This is the fact the
  matrix's cheapest options turn on, and it is recorded here so the matrix does
  not have to re-derive it.

## 2. The observation profile

One profile, pinned, adjudicating nothing. It exists because several numbers are
free once `perf record` has run at all, and because WP-1.5a's design is better
made with them than without.

**2.1 Pinning.** Candidate at `bdfca3f`, NOT HEAD. `git diff f31cffe..bdfca3f
-- crates/` is exactly `pistol-eval/src/eval.rs`, `pistol-eval/src/handcrafted.rs`,
`pistol-eval/tests/eval_delta_tests.rs`, `pistol-search/src/ordering.rs`,
`pistol-search/src/position.rs`. HEAD adds five `pistol-cli` corpus files from
D-218/D-219 which are almost entirely dead-code-eliminated (1952 bytes of binary
difference) and yet shift the residual by **2.23 pp** through pure code layout —
measured, three repetitions per binary. An optional control at `f31cffe` (the
parent of the delta commit, already radius 2) may be taken for the pre-delta
split; it is no longer required by anything.

**2.2 Instrument.** The CLASSIC instrument, unchanged: release build with
`-C force-frame-pointers=yes`, `perf record --call-graph fp`,
`perf script --no-inline -F comm,tid,ip,sym`. `--no-inline` is RETAINED, and
§0.2 and §0.3 are the reason: the inline-expanded alternative is differently
blind and biased low. Its limits are now documented rather than discovered.

**2.3 Workload.** Fixed nodes 50 000 per position, every position of both bands
of `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`,
`configs/instrument_v0.toml`. The whole workload runs in about 15 seconds.

**2.4 Repetitions.** >= 3 per binary; medians reported with their spread. No
margin is derived from them, because nothing is adjudicated.

**2.5 Quantities recorded.** None is a threshold; each is context.

- **`delta`'s own SELF share** (`perf report --no-children`, one line off the
  same `perf.data`, zero extra cost). This is the single most useful number in
  the profile: a hard upper bound on the inlined lookup component, and the one
  quantity §0.1's inlining does not hide.
- **The four-family btree split** by the awk classifier of revision 2's §4.2,
  which is NOT reproduced here and is read from
  `git show ce606e0:docs/experiments/delta_reprofile_prereg.md` — eval
  `<Window, Counts>`, candidate/movegen `<Coord, SetValZST>` attributed by
  caller frame, board `<Coord, Player>`, and everything else named individually.
  Two known limits are recorded beside it rather than repaired, since it
  adjudicates nothing: it understates the candidate family (the `SpecFromIterNested`
  drain frame lands in `other`, and ~70% of that frame's samples are
  `Board::is_legal_placement`, not btree descent, so folding it in would
  over-count); and it keys its `other` histogram on the instruction pointer, so
  one symbol appears as many rows and the two bands never merge.
- **The engine's own identity handshake** (`id config`, `id eval`,
  `id candidate_policy`, `id weights_sha256`), captured into the run record.
  This is a direct check on what the run actually did, and it closes the
  wrong-config and wrong-radius holes that no digest or threshold reaches —
  `configs/instrument_r2_v0.toml` is a DIFFERENT file that is also radius 2, so
  a config digest alone does not say which one ran.
- **Digests** of every binary, beside its revision and literal build command.
- **Pooled counts** as the per-band sum of numerators over the per-band sum of
  denominators — which is what §1 of the retired revision already meant by
  "pooled", and which needs no new command. `perf script -i A -i B` silently
  honours only the LAST `-i`; that is the one command-level trap worth naming.

## 3. What this document does NOT do

- It licenses nothing and closes nothing. Every number in §2.5 is an
  observation for WP-1.5a's design, adjudicated never (D-114).
- It does not pre-register WP-1.5a. **That work package owes its own
  pre-registration under CLAUDE.md rule 5 — a named hotspot, an expected-gain
  bracket and an abort threshold — reviewed in fresh context before
  implementation.** Its bracket should be stated in whole-engine nps against
  `tools/bench_delta.sh`, since that is the instrument the project trusts and
  the one that moots every question this document could not answer.
- It does not revive the time-matched r2-vs-r3 experiment, deferred under D-222.

## 4. Corrections to the record

Two claims landed in D-221 and in revision 2 that this round falsified; D-224
amends them.

- "**the rest are `toml` and `gimli` off the hot path**" — of 94 demangled
  `alloc::collections::btree::` symbols, 7 + 6 + 16 + 20 + 39 = 88. **Six are
  unaccounted**: four std `<usize, ThreadInfo>` maps, and two
  `Vec<Coord> as SpecFromIterNested<..btree::set::IntoIter<Coord>..>` symbols
  which are on **4.21% of pooled samples — more than the eval family itself
  post-delta (2.89%)**. They are not off the hot path.
- "**the largest symbol family in the binary**", said of `<Coord, Player>` (16):
  false. `toml` has 39 and `gimli` 20. It is the largest ENGINE-side family, and
  that is what both places should have said.
