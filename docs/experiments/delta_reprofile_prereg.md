# Pre-registration — post-delta re-profile (H1' residual, H3 window storage)

Successor to WP-1.3 Run 2 (D-192, D-193), to run on operator hardware AFTER
`Eval::delta` (D-214) landed. Fixed before the run, per CLAUDE.md rule 5 and
D-114's one-profile-one-hypothesis law; the fresh-context REVIEW-design round
of the delta work package attacked this document's thresholds before they
were committed (its F5/F6/F7 findings reshaped them — D-216 records how).
Per the governing-revision review law, if this document is amended before the
run, the amendment reopens the review.

## Configuration

- `configs/instrument_v0.toml` — candidate radius 2, the committed instrument
  policy (D-194). NAMED CONFOUND: D-192 profiled radius 3 (its
  pre-registration pinned the config before Run 1 moved the policy), so this
  profile and D-192's are NOT a like-for-like before/after — the radius
  changes the candidate count per node and with it the ordering-eval share.
  The comparison is directional; no sentence in the verdicts may say "the
  share fell from 76.27%" without naming the confound.
- Release build, `--locked`, `overflow-checks` on (the committed profile),
  with `-C force-frame-pointers=yes`; `perf record --call-graph fp` — dwarf
  unwinding is broken on this machine (wp13_results §4; root cause still
  undiagnosed, §6d). Frame-pointer codegen cost was measured at +1.0% to
  +3.6% wall-clock at identical node counts (D-192); it cannot move either
  threshold below and is accepted as the instrument's cost.
- Workload: fixed nodes 50 000 per position, every position of both bands of
  `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`; samples pooled
  over both bands, per-band shares reported beside the pooled figure.
- Artifacts (perf.data, reports) to the workbench, never the repository
  (CLAUDE.md rule 8). The ADR lines record the numbers, the counting
  commands and the sample counts.

## H1' — residual eval share under ordering

**Quantity:** share of pooled samples whose stack contains one of
`<HandcraftedV0 as Eval>::{apply, undo, value, delta}` AND passes through
`pistol_search::ordering::order`. The symbol set is D-192's plus `delta`,
because ordering's scoring call is now `Eval::delta` behind `Box<dyn Eval>` —
non-inlinable into `order`, the same attribution argument D-192 made, and the
addition that keeps the quantity meaningful now that `apply`/`undo`/`value`
may vanish from ordering stacks entirely.

**Two thresholds, because one cannot do both jobs.** Amdahl on D-192's own
numbers: pre-delta the ordering-eval share was 76.3 of 100 units; an accepted
bench speedup S in [1.4, 2.5] leaves total time 100/S with the other 23.7
units untouched, PREDICTING a residual share of (100/S − 23.7)/(100/S) —
**66.8% at S = 1.4, 40.8% at S = 2.5** (72.7% even at the 1.15 abort line).
A flat 20% bar would therefore fire on full success and adjudicate nothing.

- **H1'-a, under-delivery: >= 70%.** Above the whole accepted bracket's
  Amdahl ceiling. Confirmed means the profile and the bench disagree about
  what happened — the WP is re-opened, starting with whether the bench
  measured the pinned binaries. Refuted means delta delivered where it was
  aimed.
- **H1'-b, hotspot license: >= 20%** (D-192's bar, carrying D-192's meaning:
  "is eval-under-ordering still a hotspot at all?"). Confirmed — which the
  arithmetic above EXPECTS even on full success — licenses a further
  eval-side work package; it indicts nothing. Refuted closes H1 outright.

## H3 — the window map (`BTreeMap<Window, Counts>`)

Registered as its own hypothesis line per D-114 (a third hypothesis is a new
line and a new profile, never an addition to D-192/D-193 after their numbers).

**Quantity:** share of pooled samples whose stack contains an
`alloc::collections::btree::` symbol — the D-193 counting rule: BTreeMap node
operations, NEVER the H2 allocation entry points (`__rust_alloc` and family);
D-193's first pass over-matched by 40x on exactly this distinction and the
rule is spelled here so it cannot be re-invented loosely.

**Threshold: >= 15% confirms.** Pre-delta the map sat at 23.21% of stacks
with two of the six self-time leaders (D-193's recorded observation). Delta
removes ordering's map surgery (entry insert/remove) but keeps up to 18
lookups per probe, and the denominator shrinks with the speedup, so the share
is not predictable from the old number — which is why it is measured.
Confirmed licenses the window-storage work package — its own D-196 option
matrix (sorted vec, open-addressing with a fixed-seed hash, dense
ring-indexed arrays) and its own rule-5 bench, BEFORE WP-1.5 builds threat
tables on the same structure question. Refuted: the storage stays until a
later profile reopens it.

**Visibility precondition, adjudicated BEFORE the threshold** (WP-1.3 §5's
defect class: a threshold is no protection when the measured quantity is
wrong): post-delta the only btree work under ordering is `get()`, a small
monomorphized body release codegen may fully inline — and an inlined callee
leaves NO frame under fp unwinding, so H3 could read ~0% while btree descent
dominates `delta`'s self time, exonerating the structure by symbol
invisibility. Before counting, the operator verifies btree search/descent
symbols are PRESENT in the profiled binary (`nm -C <binary> | grep -c
'btree.*search'` > 0) and appear in `perf report`. If they are inlined away,
the named fallback is a probe build with the map read behind an
`#[inline(never)]` helper, its wall-clock cost measured at identical node
counts (the frame-pointer-detour precedent, D-192), and H3 counted on that
build — recorded as a deviation, never silently.

## Verdict discipline

One ADR line per hypothesis (H1' and H3 separately), written from the numbers
with no threshold moved after any sample is taken. A confirmed H3 licenses
the window-storage WP; it does not choose among its options — that is the
WP's own matrix. Anything else this profile surfaces is an observation for a
future pre-registration, adjudicated never (D-114, D-193's precedent).
