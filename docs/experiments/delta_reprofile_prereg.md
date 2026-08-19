# Pre-registration — post-delta re-profile (H1' residual, H3 window storage)

**REVISION 2 — thresholds SETTLED by operator ruling of 2026-08-19, committed,
and governing nothing until it passes review.** Revision 1 (at `bdfca3f`) was
attacked by a fresh-context review under the governing-revision law (D-195) and
returned **FAILS as governing revision** on one BLOCKING and two MAJOR findings.
This revision answers them and carries the operator's rulings on every number it
adds. Per D-195 the revision that governs a run must itself pass a fresh-context
review before the first run it governs: **no measurement authorized by this
document may be taken until the review dispatched against this commit returns
PASS.**

Successor to WP-1.3 Run 2 (D-192, D-193), to run on operator hardware AFTER
`Eval::delta` (D-214) landed. Fixed before the run, per CLAUDE.md rule 5 and
D-114's one-profile-one-hypothesis law; the fresh-context REVIEW-design round
of the delta work package attacked revision 1's thresholds before they were
committed (its F5/F6/F7 findings reshaped them — D-216 records how).

## 0. What revision 2 changes, and the rulings that settled it

### 0.1 The six findings of the governing-revision review

Nothing the review verified SOUND is re-opened: the Amdahl arithmetic
(66.8 / 40.8 / 72.7), the H1' symbol rules, and consistency with
`tools/bench_delta.sh` and D-192/D-193/D-214..D-217 stand as revision 1 wrote
them.

- **F1 BLOCKING — the H3 visibility precondition could FALSE-PASS.** Revision 1
  checked that btree symbols exist in the binary and appear in `perf report`.
  Both can be true while the quantity H3 measures is invisible, because the
  surviving symbols belong to the apply/undo **surgery** path while `delta`'s
  `get()` is inlined and leaves no frame. Worse, the pre-registered command was
  doubly broken: `grep -c 'btree.*search'` matches the substring `pistol_search`
  in symbols that are not btree descent at all, and `> 0` is a shell
  REDIRECTION, not a comparison. Answered in §4: a family classification rule, a
  stack-level counting rule, a positive control at `f31cffe`, corrected literal
  commands, a numeric success check, and an outcome table in which a failed
  control can never yield REFUTE.
- **F2 MAJOR — the under-delivery bar was inherited from the wrong radius.**
  70% was derived from D-192's radius-3 pooled share (76.27%); this run is
  radius 2, where the pre-delta share is plausibly 79–80% and full success would
  then read above 70% and falsely CONFIRM under-delivery. Answered in §3 and §5:
  the bar is now a pre-registered FORMULA over a radius-2 share MEASURED on the
  pre-delta binary before any post-delta sample is taken.
- **F3 MAJOR — the fallback probe's measured cost fed no rule.** Answered in
  §4.6: a numeric maximum acceptable distortion, a distortion bound formula, and
  a near-bar INCONCLUSIVE zone.
- **F4 MINOR — H1'-a / H1'-b double-fire precedence unstated.** Answered in §5.3.
- **F5 MINOR — the run was not conditioned on the official bench verdict and did
  not require the profiled binary's digest.** Answered in §2.
- **F6 NOTE — "pooled" was inherited without citation.** Answered in §1.

### 0.2 The operator's rulings — the nine thresholds

Every number revision 2 adds or changes was put to the operator and ruled on.
All nine are APPROVED as proposed; each is restated where it is used, with its
derivation. Under D-195 these rulings settle the numbers, not the review: the
review dispatched against this commit may still find any of them unsound.

| # | threshold | ruling |
| --- | --- | --- |
| 1 | `BAR_a = ceil_0.5(100 - (100 - P2) x 1.4 + 3.0)` over the CONTROL RUN's measured radius-2 P2 (§5.1) | APPROVED. The 1.4 factor is the pre-registered bracket floor and **stays fixed**; the measured speedup is never substituted for it — a bar built from the thing it judges is no bar. |
| 2 | instrument margin M = 3.0 pp, decomposed 0.9 sampling + 1.0 frame-pointer + 1.0 Amdahl idealisation (§5.1) | APPROVED |
| 3 | positive-control bar V_ord >= 10.0% pooled at `f31cffe` (§4.4) | APPROVED. Control below bar ⇒ the METHOD failed ⇒ **every H3 verdict this run is INCONCLUSIVE**. Never REFUTE on a failed control. |
| 4 | visibility check V_delta >= 1.0% pooled (§4.5) | APPROVED, with its conservatism recorded (§4.5): a genuinely cheap map read that reads below 1% yields INCONCLUSIVE, not REFUTE — the safe failure direction. |
| 5 | probe cost C <= 5% accepted (§4.6) | APPROVED |
| 6 | 5% < C <= 15% accepted with a near-bar INCONCLUSIVE zone (§4.6) | APPROVED |
| 7 | C > 15%: the profile is not the workload, the probe run is invalid (§4.6) | APPROVED |
| 8 | H1'-b hotspot license >= 20% (§5.2) | unchanged, APPROVED |
| 9 | H3 threshold >= 15% (§4.3) | unchanged, but applied to the SPLIT quantity H3-e |

### 0.3 The H3 split — an amendment under D-216's pre-run window, exercised

D-216 permits amendment to this pre-registration BEFORE its first run, at the
cost of reopening its review. That window is exercised here. Revision 1
registered ONE H3 over every `alloc::collections::btree::` frame. Revision 2
splits the registered quantity in two, and only one half adjudicates:

- **H3-e — pistol-eval's `BTreeMap<Window, Counts>` share.** Adjudicated at
  **>= 15%**. Licensed action: the window-storage option matrix in **WP-1.5a**.
- **H3-c — the candidate-set `BTreeSet` share (`candidates::within_radius`).**
  **MEASUREMENT ONLY. No licensed action, no verdict.** Three reasons, recorded
  so the split cannot later be read as an oversight:
  (a) that structure is scheduled for supersession by **WP-1.5b**'s Staged
  candidate policy — confirming it would license work on code with weeks to
  live; (b) full-set iteration and window-keyed lookup are different access
  patterns, so its share is not evidence about the eval map; (c) the split
  RAISES the effective bar for the expensive structural work package, which is
  the correct direction for a bar to move. Its measured share is context for
  WP-1.5b's design and nothing more.

**A fact the split ruling did not have, found while writing the counting rule
and recorded here because it strengthens the same conclusion: the binary holds
FOUR btree families, not two.** On a release build of the dev tree, of 94
demangled `alloc::collections::btree::` symbols, 7 are instantiated over
`<pistol_eval::window::Window, pistol_eval::handcrafted::Counts>` (H3-e), 6 over
`<pistol_core::coord::Coord, alloc::collections::btree::set_val::SetValZST>`
(the `BTreeSet<Coord>` shape — shared by `pistol_search::candidates` AND
`pistol_core::movegen`'s region set, indistinguishable by type arguments alone,
which is why §4.2 attributes them by CALLER), and **16 over
`<pistol_core::coord::Coord, pistol_core::board::Player>` — which is
`pistol_core::board::Board::stones`, the board itself**, touched by every
`place`/`undo` on every node. The rest are `toml` and `gimli` maps off the hot
path. D-193's 23.21% therefore spans at least four structures, the largest
symbol family being one that neither H3-e nor H3-c names. §4.2 counts all four
and names the remainder; only H3-e adjudicates.

### 0.4 The four flagged calls

- **ABOVE-BRACKET official bench** (§2.1): **PASS**, plus a recorded calibration
  note — our prediction machinery under-predicted; the perf claim stands. It
  does not block the profile.
- **Full-workload control run at `f31cffe`** (§3): APPROVED as specified. It is
  the evidence backbone of F1 (positive control) and F2 (radius-2 baseline) at
  once; a partial control reopens the false-pass class this revision exists to
  close.
- **H3 adjudicated before H1', both off the control run** (§6.1): APPROVED.
  Precedence and double-fire rules per F4 stand unchanged.
- **Document length** (§6.3): experiment records under `docs/experiments/` are
  EXEMPT from rule 9's soft cap **by class**, recorded as an ADR line.

## 1. Configuration

- `configs/instrument_v0.toml` — candidate radius 2, the committed instrument
  policy (D-194). **The radius confound is now confined.** Revision 1 named it
  because the only pre-delta number available was D-192's radius-3 profile.
  This revision takes its own pre-delta measurement at radius 2 (§3), so the
  pre/post pair THIS document generates is like-for-like. The confound survives
  only where a verdict sentence compares against D-192's published figures: no
  sentence may say "the share fell from 76.27%" without naming that 76.27% was
  measured at radius 3.
- Release build, `--locked`, `overflow-checks` on (the committed profile), with
  `-C force-frame-pointers=yes`; `perf record --call-graph fp` — dwarf
  unwinding is broken on this machine (wp13_results §4; root cause still
  undiagnosed, §6d). Frame-pointer codegen cost was measured at +1.0% to +3.6%
  wall-clock at identical node counts (D-192); it cannot move either threshold
  below and is accepted as the instrument's cost. **The control profile (§3)
  uses the identical flags**, so the flag is not a difference between the two
  binaries being compared.
- Workload: fixed nodes 50 000 per position, every position of both bands of
  `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`; samples pooled over
  both bands, per-band shares reported beside the pooled figure.
- **Pooled and inclusive are wp13_prereg §5's definitions, cited rather than
  re-invented (F6):** "**Pooled** means the raw sample counts of the two
  `perf.data` files are summed; **inclusive** means the fraction of samples
  whose stack contains the frame." Every share in this document is inclusive
  over the pooled denominator unless it says SELF. D-192's run gives the scale
  of that denominator: 24 810 pooled samples (14 544 early, 10 266 late).
- Artifacts (perf.data, reports) to the workbench, never the repository
  (CLAUDE.md rule 8). The ADR lines record the numbers, the counting commands
  and the sample counts.

## 2. Preconditions on the run — bench verdict and binary identity (F5)

The re-profile does not start until all four hold, and the record states each.

**2.1 The official bench verdict.** D-215 makes the operator's own run of
`tools/bench_delta.sh` the official verdict; the dev-machine numbers in that
line are advisory. The script prints one token per band. What each token does to
this profile is fixed here, and no threshold moves to accommodate one:

- **`PASS`** — the profile runs.
- **`ABORT`** (nps ratio < 1.15): D-215 reverts the change. There is no
  post-delta binary to profile and this document is void. Unchanged by any
  ruling: the abort floor is the original pre-registration's and stands.
- **`BELOW-BRACKET`** (1.15x–1.4x): **the change is KEPT, flagged
  under-bracket, and the profile RUNS.** Revision 1 called for an amendment
  here; the operator ruled otherwise, and the reasoning is recorded because it
  looks like double-counting and is not. BAR_a is built at S_min = 1.4 (§5.1)
  whatever the bench measured, so a below-bracket speedup will very likely also
  trip H1'-a's under-delivery bar. **That is one fact reported by two
  instruments, not two independent findings**: the bench says the speedup missed
  the bracket, the profile says the residual eval share is higher than a
  bracket-floor speedup predicts, and the ADR lines must say so in those words
  rather than compounding them into a larger claim.
- **`ABOVE-BRACKET`** (> 2.5x): **PASS.** The profile runs. The record carries a
  **calibration note**: the prediction machinery under-predicted the gain, the
  perf claim stands, and the miss is a fact about our forecasting, not a doubt
  about the binaries. The bar built at S_min = 1.4 remains conservative in this
  direction by construction — a larger S predicts a SMALLER residual, so the bar
  can only be too high, never too low.
- **`TTD-MISS`** or **`NOISY`**: the harness withheld or failed a verdict.
  Blocked until a clean run exists. Unchanged.

**2.2 The profiled binary's digest is in the record.** `sha256sum` of every
binary this experiment touches, beside the revision it was built from and the
literal build command: the pre-delta CONTROL binary (revision `f31cffe`), the
post-delta CANDIDATE binary, the two binaries the official bench measured, and
the fallback probe binary if §4.6 fires. A run whose record is missing a digest
is not a measurement this project keeps.

**2.3 The profiled binary is not the benched binary, and the record says so.**
The profiling build adds `-C force-frame-pointers=yes`, so its digest
NECESSARILY differs from the digest the official bench measured. That is not a
defect; hiding it would be. The record states both digests and asserts the one
property that matters: **identical revision, clean tree, flags differing only by
the frame-pointer switch.** A dirty tree at build time voids the run.

**2.4 The pre/post pair is clean.** `git diff --stat f31cffe..<candidate-rev>
-- crates/` must show the delta change and nothing else. At the revision this
draft was written the diff is exactly `pistol-eval/src/eval.rs`,
`pistol-eval/src/handcrafted.rs`, `pistol-eval/tests/eval_delta_tests.rs`,
`pistol-search/src/ordering.rs`, `pistol-search/src/position.rs`. Any further
engine file in that diff must be named in the record and its effect on the
H1'-a derivation stated, because the derivation assumes the only thing between
the two binaries is `delta`.

## 3. The pre-run control profile at `f31cffe` — one run, three jobs

Run BEFORE any post-delta sample, on the pre-delta binary at revision
`f31cffe` (the parent of the delta commit `9f9cbe9`, verified), built with §1's
flags and driven with §1's workload and `configs/instrument_v0.toml` — which
already stands at radius 2 at that revision (verified), so the control needs no
config edit and introduces no second confound.

It exists to serve three purposes at once, off one `perf.data` pair:

1. **F1's positive control.** Demonstrate that the counting method of §4.2 FINDS
   btree-under-ordering where it is known present. Pre-delta, ordering reaches
   the map through `apply`/`value`/`undo` and D-193 recorded btree node
   operations on 23.21% of stacks. Adjudicated in §4.4.
2. **F2's baseline.** Measure P2 — the radius-2 pre-delta eval-under-ordering
   share — which §5.1's bar formula consumes.
3. **The split's own baseline.** Report the pre-delta H3-e, H3-c, board-map and
   other-family shares (§4.2's classifier, run unchanged on the control's
   `perf.data`). This costs no extra run and is the honest predecessor figure
   for a post-delta H3-e that will otherwise have only D-193's unsplit 23.21% to
   be compared against. It adjudicates nothing.

**The control is a full-workload run and not a subset.** P2 feeds a threshold
and must be pooled over the same denominator shape as the post-delta run, and a
partial control reopens exactly the false-pass class this revision exists to
close: a method validated on a slice is not validated on the workload.

## 4. H3 — the window map (`BTreeMap<Window, Counts>`)

Registered as its own hypothesis line per D-114 (a third hypothesis is a new
line and a new profile, never an addition to D-192/D-193 after their numbers).

### 4.1 The registered quantities

The D-193 counting rule is inherited unchanged as the FRAME filter: a frame
counts when its demangled symbol contains `alloc::collections::btree::` — btree
node operations, **NEVER the H2 allocation entry points** (`__rust_alloc` and
family); D-193's first pass over-matched by 40x on exactly this distinction and
the rule is spelled here so it cannot be re-invented loosely.

What revision 2 changes is the DENOMINATOR of the claim, not the filter. Two
quantities are registered, both inclusive over §1's pooled denominator:

- **H3-e** = share of pooled samples carrying at least one btree frame of the
  **eval family**. **Adjudicated at >= 15%.**
- **H3-c** = share of pooled samples carrying at least one btree frame of the
  **candidate-set family**. **Measured, never adjudicated** (§0.3).

A sample counts ONCE per family however many matching frames it carries; a
sample may count toward both, and the record reports the overlap.

### 4.2 The family classification rule

A btree frame is classified by its demangled symbol's type arguments, and where
those are ambiguous, by the nearest enclosing caller on the same stack:

- **eval** — the symbol contains `pistol_eval::window::Window`. Unambiguous: no
  other structure in the binary is keyed by that type.
- **candidate** — the symbol contains BOTH `pistol_core::coord::Coord` and
  `alloc::collections::btree::set_val::SetValZST` (the `BTreeSet<Coord>` shape)
  AND the stack carries a `pistol_search::candidates::candidate_cells` frame
  nearer the root. The caller test is required, not decorative:
  `pistol_core::movegen::region_cells` builds a `BTreeSet<Coord>` of the
  identical instantiation, so type arguments alone cannot tell the two apart.
  `within_radius` itself does NOT survive as a symbol (verified — it is inlined
  into `candidate_cells`, where it appears only inside a `SpecFromIterNested`
  instantiation), which is why `candidate_cells` is the attribution frame.
- **movegen** — the same `BTreeSet<Coord>` shape under a
  `pistol_core::movegen::region_cells` frame. Reported, adjudicates nothing.
- **board** — the symbol contains `pistol_core::board::Player`. This is
  `Board::stones`, the largest symbol family in the binary (§0.3). Reported,
  adjudicates nothing, and named in the record because a large board-map share
  is the single most likely reason for a post-delta H3-e far below D-193's
  unsplit 23.21%.
- **other** — everything else, **named individually in the record with its
  demangled symbol and its frame count**, never rolled into a total. A
  candidate-set frame whose `candidate_cells` caller was inlined away lands here
  by construction; that is the honest outcome, and the record shows it.

The literal classifier, applied to each band's `perf.data` and to the pooled
pair:

```sh
perf script -i "$PERF" --no-inline -F comm,tid,ip,sym \
| awk -v RS='' -v FS='\n' '
    function firstf(s,   i) { for (i = 1; i <= NF; i++) if (index($i, s) > 0) return i; return 0 }
    { total++
      cand = firstf("pistol_search::candidates::candidate_cells")
      mgen = firstf("pistol_core::movegen::region_cells")
      e = 0; c = 0; m = 0; b = 0; o = 0
      for (i = 1; i <= NF; i++) {
        if ($i !~ /alloc::collections::btree::/)          continue   # D-193 frame filter
        if ($i ~ /__rust_(alloc|dealloc|realloc)/)        continue   # never the H2 entry points
        if (index($i, "pistol_eval::window::Window") > 0) { e = 1; continue }
        if (index($i, "pistol_core::board::Player")  > 0) { b = 1; continue }
        if (index($i, "set_val::SetValZST") > 0 && index($i, "pistol_core::coord::Coord") > 0) {
          if (cand > 0 && i < cand) { c = 1; continue }
          if (mgen > 0 && i < mgen) { m = 1; continue }
        }
        o = 1; other[$i]++ }
      if (e) ne++; if (c) nc++; if (m) nm++; if (b) nb++; if (o) no++ }
    END { printf "H3-e eval windows   %6d / %6d = %.2f%%\n", ne, total, 100 * ne / total
          printf "H3-c candidate set  %6d / %6d = %.2f%%\n", nc, total, 100 * nc / total
          printf "     movegen region %6d / %6d = %.2f%%\n", nm, total, 100 * nm / total
          printf "     board stones   %6d / %6d = %.2f%%\n", nb, total, 100 * nb / total
          printf "     other          %6d / %6d = %.2f%%\n", no, total, 100 * no / total
          for (s in other) printf "     other symbol %6d  %s\n", other[s], s }'
```

The four family lines and the named `other` symbols all go into the record. Only
the H3-e line adjudicates.

**The three commands of §4.2 and §4.4 were exercised before this document was
committed**, on a synthetic six-sample `perf script` block carrying one stack of
each family (eval-under-`delta`, candidate-set under `candidate_cells`,
`BTreeSet<Coord>` under `region_cells`, board map, a `toml` map, and one stack
with no btree frame at all). Each landed in exactly its own bucket, the `toml`
frame was named in `other`, and the btree-free stack counted toward the
denominator only. That is a syntax-and-attribution check on invented input, not
evidence about the engine; the run's own numbers come from the real profile.

### 4.3 H3-e's threshold: >= 15% confirms

Pre-delta the unsplit btree count sat at 23.21% of stacks with two of the six
self-time leaders (D-193's recorded observation). H3-e is a STRICT SUBSET of
that count, so the split raises the effective bar — which is the correct
direction for the expensive structural work package and is why the operator
ruled it so. Delta removes ordering's map surgery (entry insert/remove) but
keeps up to 18 lookups per probe, and the denominator shrinks with the speedup,
so the share is not predictable from the old number — which is why it is
measured.

**Confirmed** (H3-e >= 15%) licenses the window-storage work package **WP-1.5a**
— its own D-196 option matrix (sorted vec, open-addressing with a fixed-seed
hash, dense ring-indexed arrays) and its own rule-5 bench, BEFORE WP-1.5 builds
threat tables on the same structure question. It licenses the matrix; it does
not choose among its options. **Refuted** (H3-e < 15%): the storage stays until
a later profile reopens it.

### 4.4 The visibility precondition — counting rule and positive control (F1)

Adjudicated BEFORE the threshold (WP-1.3 §5's defect class: a threshold is no
protection when the measured quantity is wrong). Post-delta the only btree work
under ordering is `get()`, a small monomorphized body release codegen may fully
inline — and an inlined callee leaves NO frame under fp unwinding, so H3-e could
read ~0% while btree descent dominates `delta`'s self time, exonerating the
structure by symbol invisibility.

**Revision 1's check was not a check.** Two observations on a release build of
the dev tree (no frame pointers; codegen differs from the profiling build in
stack setup, not in these inlining decisions) show why:

- `nm -C target/release/pistol | grep -c 'btree.*search'` returns **1**, and the
  single match is
  `<alloc::vec::Vec<Coord> as SpecFromIterNested<Coord, Filter<alloc::collections::btree::set::IntoIter<Coord>, pistol_search::candidates::within_radius::{closure#0}>>>`
  — the pattern matched `btree::set` followed by the crate name `pistol_search`.
  There is no btree search-descent symbol in the binary at all. The check passes
  on a symbol that has nothing to do with the map.
- Of the btree symbols instantiated over `<Window, Counts>`, every one is
  SURGERY — `entry::Entry<..>::or_default`, `node::BalancingContext<..>`'s
  steal/merge, mutable `node::Handle<..>` — i.e. the `apply`/`undo` path, which
  the search's own `place`/`undo` still exercises. No `get` instantiation
  survives. A binary-presence check therefore passes precisely in the world
  where `delta`'s map read is invisible.

**The precondition is measured on STACKS, not in the symbol table.** Define

> **V_delta** = the share of pooled samples whose stack contains BOTH an
> **eval-family** btree frame (§4.2's classification) AND a frame of
> `<pistol_eval::handcrafted::HandcraftedV0 as pistol_eval::eval::Eval>::delta`,
> **with the btree frame nearer the leaf than the `delta` frame** — i.e. the
> eval's own map work was called from `delta`.

Revision 1 defined this over ANY btree frame. Restricting it to the eval family
follows the split: V_delta must be the visibility of the quantity H3-e
adjudicates, or it guards nothing. The restriction can only make the
precondition harder to pass — i.e. push toward INCONCLUSIVE — which is the safe
direction (§4.5).

Attribution rules, stated so they cannot be re-invented:

- A sample counts ONCE however many matching frames it carries (inclusive, per
  §1's citation), never once per frame.
- "Nearer the leaf" is `perf script`'s own stack order: the leaf is printed
  first, so the btree line's index must be strictly less than the `delta`
  line's index. The first matching line of each kind is the one used.
- A btree frame with NO `delta` frame below it does not count toward V_delta; it
  is search-path surgery and belongs to H3-e's own figure, not to the
  precondition.
- Frames matching the H2 allocation entry points are excluded here exactly as
  in H3's own rule (D-193).
- `delta` is reached through `Box<dyn Eval>` and so cannot be inlined into
  `order`; it survives as a `T` symbol in the binary (verified), which is what
  makes this attribution possible at all.

The literal command, applied to each band's `perf.data` and to the pooled pair:

```sh
perf script -i "$PERF" --no-inline -F comm,tid,ip,sym \
| awk -v RS='' -v FS='\n' -v CALLER='Eval>::delta' -v FAMILY='pistol_eval::window::Window' '
    { total++; b = 0; c = 0
      for (i = 1; i <= NF; i++) {
        if (b == 0 && $i ~ /alloc::collections::btree::/ && index($i, FAMILY) > 0) b = i
        if (c == 0 && index($i, CALLER) > 0)                                       c = i
      }
      if (b > 0 && c > 0 && b < c) hit++ }
    END { printf "%d / %d = %.2f%%\n", hit, total, 100 * hit / total }'
```

**The positive control.** The same command is run on the pre-delta control
binary at `f31cffe` (§3) with `CALLER='ordering::order'` and
**`FAMILY='alloc::collections::btree::'`** — a family string every btree frame
already satisfies, so the control counts ALL btree families under ordering. (It
is written as the redundant literal rather than as an empty string on purpose:
`index(s, "")` is 1 in GNU awk but is not portably specified, and a control that
silently counts nothing would fail in the one direction this document cannot
afford.) That is deliberate: the control validates the
PIPELINE — fp unwinding, `perf script` stack order, the awk attribution — and a
pipeline check should not be narrowed by the classification it is meant to
license. Call that number **V_ord**. The eval-family control number is recorded
beside it (§3's job 3) as information, not as the control.

**Control bar: V_ord at `f31cffe` >= 10.0% of that run's pooled samples**
(operator ruling 3). Derivation, from the two recorded pre-delta numbers and
nothing else: btree node operations were on 23.21% of stacks (D-193); eval work
was on 85.62% of stacks anywhere and 76.27% under ordering (D-192), so ordering
carries 76.27 / 85.62 = 89.1% of the eval's work. If as little as HALF of the
23.21% is the eval's own map rather than another family's, the expected control
reading is 0.5 x 23.21 x 0.891 = **10.3%**. 10.0% is that conservative floor
rounded down — far enough above zero that noise cannot reach it, far enough
below the ~20% a fully-eval-owned map would give that a radius or workload
difference cannot fail it. §0.3's four-family finding makes the bar MORE
conservative, not less: the board map is also touched under ordering, and every
additional family under `order` can only raise V_ord.

**What the control does and does not establish.** It validates the toolchain and
the counting rule on a build where the quantity is known present. It cannot
control for `get()`'s inlining, which is the very thing being measured. That
asymmetry is the point: a control PASS plus V_delta ~ 0 is then readable as
genuine invisibility, whereas without the control the same reading is
indistinguishable from a broken pipeline.

**Outcome rule (binding, operator ruling 3): if the control FAILS its bar, the
METHOD failed, and EVERY H3 verdict this run is INCONCLUSIVE — never REFUTE.** A
method that cannot see btree frames where btree frames are known to be cannot
support the claim that the map is not a hotspot. H3-c's measured share is
reported in the same case, marked method-unvalidated, and licenses nothing (it
never did). The control number, the command, and the sample counts are recorded
either way, and the failure becomes method debt for the next profile.

### 4.5 The success check, and the direction it fails in

**Precondition PASSES when V_delta >= 1.0% of pooled samples** (operator ruling
4). Derivation: H3-e adjudicates at 15%, and a quantity visible only below 1%
cannot support or refute a 15% claim. At the D-192 denominator scale (24 810
pooled) 1.0% is about 250 samples, whose 95% confidence half-width is about
0.12 pp — a reading that is real rather than noise. Below 1.0% the precondition
FAILS and §4.6 fires.

**The check's conservatism, recorded because it is a known cost and not a
defect.** A map read that is genuinely cheap — the world in which H3-e's honest
verdict is REFUTE — produces exactly the same sub-1% reading as an invisible
one. This check cannot tell those apart, and so it yields **INCONCLUSIVE where
REFUTE would have been correct**. That is the safe failure direction and the
reason the bar is accepted at 1.0%: the cost of the error is one re-measurement,
whereas the error it prevents — exonerating a structure by symbol invisibility —
would close a real question on evidence that never existed. INCONCLUSIVE here
licenses nothing and closes nothing; it carries the question forward by name.

Two subsidiary checks are recorded beside it and neither is the precondition:

```sh
# btree symbols instantiated over the eval's own map types (0 is informative,
# non-zero is NOT a pass — the surviving symbols may all be surgery).
EVAL_BTREE_SYMS="$(nm -C "$BIN" | grep -F 'alloc::collections::btree::' \
                                | grep -cF 'pistol_eval::window::Window')"
echo "eval-typed btree symbols: $EVAL_BTREE_SYMS"

# The corrected form of revision 1's broken line: a COMPARISON, not the
# redirection `> 0` that would have created a file named 0 and always passed.
[ "$EVAL_BTREE_SYMS" -gt 0 ] || echo "no eval-typed btree symbol survives"
```

`perf report` is inspected and quoted in the record as context, never as the
precondition.

### 4.6 The fallback probe and its cost rule (F1, F3)

If the precondition fails, the named fallback is a probe build in which the map
read is taken behind an `#[inline(never)]` helper on `HandcraftedV0` — a
throwaway build whose diff is recorded and which never lands on `dev`.

**The helper's counting rule, which revision 1 omitted.** `#[inline(never)]`
stops the helper being inlined INTO `delta`; it does NOT stop `get()` inlining
INTO the helper, so btree frames may still be absent on the probe build. The
helper frame is therefore the proxy for the map read, and on the probe build
H3-e is counted as the deduplicated union of:

- samples carrying a frame of the helper symbol
  (`pistol_eval::handcrafted::HandcraftedV0::window_counts` or whatever name the
  probe gives it — the record states the exact symbol) with a `delta` frame
  below it, and
- samples carrying an **eval-family** btree frame under §4.2's classification
  (the surgery the search path still performs).

A sample in both components counts once; the two components are reported
separately as well as summed. H3-c and the other families are counted on the
probe build by §4.2 unchanged and are unaffected by the helper.

**The probe's own success check:** the helper must appear as a symbol
(`nm -C "$BIN" | grep -cF 'window_counts'`, compared with `-gt 0`) AND on
>= 1.0% of pooled samples, by §4.5's derivation. A probe whose helper is itself
invisible has failed, and H3-e is INCONCLUSIVE.

**The cost rule (F3), which revision 1 measured but never used.** Let C be the
probe build's wall-clock cost at identical node counts — median over the four
calibration positions D-192 used for the frame-pointer cost, >= 3 reps, measured
against the same-revision NON-probe profiling build so that frame pointers are
held constant and C isolates the helper. The worst case is that all of C lands
inside the counted frames, so a true share p reads as

> p' = p (1 + C) / (1 + p C),  and  D_max = p' - p evaluated at p = 0.15.

- **C <= 5.0%: accepted** (ruling 5). D_max = 0.63 pp. 5.0% is the next round
  figure above the +3.6% frame-pointer detour this project already accepted as
  an instrument's cost (D-192), and 0.63 pp is small against a 15% bar. H3-e is
  counted on the probe build and adjudicates normally, recorded as a deviation.
- **5.0% < C <= 15.0%: accepted with a near-bar zone** (ruling 6). At
  C = 15.0%, D_max = 1.87 pp. A share falling inside [15 - D_max, 15 + D_max]
  percentage points is **INCONCLUSIVE** — the reading cannot be told from the
  instrument. Outside that zone it adjudicates normally. The zone is applied
  symmetrically: the helper inflates the counted path, but cache displacement
  can deflate it, and neither direction is excluded.
- **C > 15.0%: the profile is not the workload and the probe run is INVALID**
  (ruling 7). Above 1.87 pp of distortion on a 15% bar — over a tenth of the
  quantity — the probe measures itself rather than the engine. Its numbers are
  recorded as method debt and are NOT entered as evidence about the engine's
  window map. H3-e is **INCONCLUSIVE**: the window-storage WP is neither
  licensed nor closed, and the next profile inherits the question.

### 4.7 H3 outcome table — every branch, fixed before the run

| control (§4.4) | V_delta (§4.5) | probe cost C (§4.6) | H3-e verdict |
| --- | --- | --- | --- |
| FAIL | any | any | **INCONCLUSIVE** — never REFUTE |
| PASS | >= 1.0% | probe not needed | >= 15% CONFIRM, < 15% REFUTE, on the main profile |
| PASS | < 1.0% | C <= 5.0% | >= 15% CONFIRM, < 15% REFUTE, on the probe build, recorded as a deviation |
| PASS | < 1.0% | 5.0% < C <= 15.0% | as above, EXCEPT a share within D_max of 15% is INCONCLUSIVE |
| PASS | < 1.0% | C > 15.0% | **INCONCLUSIVE**, probe run invalid |
| PASS | < 1.0% | probe not run | **INCONCLUSIVE** |

**H3-c has no row because it has no verdict.** Its share is reported in every
branch, and marked method-unvalidated whenever the control fails.

## 5. H1' — residual eval share under ordering

**Quantity:** share of pooled samples whose stack contains one of
`<HandcraftedV0 as Eval>::{apply, undo, value, delta}` AND passes through
`pistol_search::ordering::order`. The symbol set is D-192's plus `delta`,
because ordering's scoring call is now `Eval::delta` behind `Box<dyn Eval>` —
non-inlinable into `order`, the same attribution argument D-192 made, and the
addition that keeps the quantity meaningful now that `apply`/`undo`/`value` may
vanish from ordering stacks entirely. On the pre-delta control binary (§3) the
same quantity is counted over D-192's three symbols, because `delta` does not
exist there; that count is **P2**.

**Two thresholds, because one cannot do both jobs.** Amdahl: with a pre-delta
ordering-eval share of P2 out of 100 units, an accepted bench speedup S leaves
total time 100/S with the other (100 - P2) units untouched, PREDICTING a
residual share of 1 - (100 - P2) x S / 100. On D-192's radius-3 P2 = 76.27 that
is **66.8% at S = 1.4 and 40.8% at S = 2.5** (72.7% even at the 1.15 abort
line). A flat 20% bar would therefore fire on full success and adjudicate
nothing.

### 5.1 H1'-a, under-delivery — the bar is a formula, not an inherited number (F2)

Revision 1 set 70%, from the radius-3 P2. This run is radius 2. At a plausible
radius-2 P2 of 79–80 the same arithmetic predicts a 71.3–72.0% residual ON FULL
SUCCESS, which a 70% bar would call under-delivery. The bar is therefore
computed from the MEASURED radius-2 P2 of §3's control run (operator ruling 1):

> **BAR_a = ceil to the next 0.5 pp of [ 100 - (100 - P2) x S_min + M ]**,
> with **S_min = 1.4** and **M = 3.0** percentage points.

- **S_min = 1.4 is a FIXED CONSTANT of this pre-registration, not the measured
  speedup** (ruling 1, stated in the operator's own terms: *a bar built from the
  thing it judges is no bar*). It is the accepted bracket's floor. Substituting
  the bench's measured S would make the bar move with the result it is supposed
  to adjudicate, which is the post-hoc threshold move CLAUDE.md rule 5 forbids.
  1.4 also maximises the predicted residual and therefore the bar; any larger
  measured S only lowers the prediction, so 1.4 is the choice that cannot make
  H1'-a fire falsely. A BELOW-BRACKET bench (§2.1) does not lower it either —
  the expected consequence is a flagged under-delivery, and §2.1 records why
  that is one fact reported twice rather than two.
- **M = 3.0 pp** is the instrument margin (ruling 2), from three named
  components: pooled sampling error at n ~ 25 000 is <= 0.3 pp at 1 sigma, so
  **0.9 pp** at 3 sigma; the frame-pointer instrument's own cost (+1.0% to
  +3.6% wall-clock, D-192) moves a ~72% share by at most **1.0 pp** if it lands
  entirely on one side of the split; and **1.0 pp** for the Amdahl model's
  idealisation, since the bench's S is an nps ratio rather than a pure time
  split. 0.9 + 1.0 + 1.0 = 2.9, rounded to 3.0.
- Sanity: at D-192's P2 = 76.27 the formula returns 100 - 33.222 + 3.0 = 69.78,
  ceiling 0.5 → **70.0%** — it reproduces revision 1's bar exactly at revision
  1's input, which is what a formula replacing a number should do.

**BAR_a is computed and written into the run record BEFORE the post-delta
profile is started**, from the control run's P2 and from no other quantity. That
ordering is the whole protection: the bar is a pre-registered function of a
pre-delta measurement, never a number chosen after a post-delta sample.

Confirmed (residual >= BAR_a) means the profile and the bench disagree about
what happened — the WP is re-opened, starting with whether the bench measured
the pinned binaries (§2.2's digests are the first evidence consulted). Refuted
means delta delivered where it was aimed.

### 5.2 H1'-b, hotspot license: >= 20%

D-192's bar, carrying D-192's meaning: "is eval-under-ordering still a hotspot
at all?" Confirmed — which the arithmetic above EXPECTS even on full success —
licenses a further eval-side work package; it indicts nothing. Refuted closes H1
outright. Unchanged by this revision (ruling 8).

### 5.3 Precedence when both fire (F4)

The two bars sit on the SAME quantity, so a share at or above BAR_a is at or
above 20% by construction. Three consequences, fixed before the run:

1. **"H1'-a confirmed and H1'-b refuted" is arithmetically impossible.** A
   report containing it is a counting error, not a result, and the count is
   redone before anything is written to `docs/decisions.md`.
2. **Both ADR lines are written.** H1'-a and H1'-b answer different questions
   and D-114 gives each its own line; a double fire does not collapse them.
3. **H1'-a takes precedence over H1'-b's license.** When H1'-a confirms, the
   further eval-side work package H1'-b would license is SUSPENDED until the
   H1'-a re-opening resolves, and H1'-b's ADR line records the suspension by
   name. Licensing new eval work off a share that the same profile says is
   anomalous would build on a number under investigation.

## 6. Verdict discipline

### 6.1 Order of adjudication

Fixed before the run: **H3 is adjudicated before H1'**, and both draw their
pre-delta quantities from the SAME control run at `f31cffe` (§3). The order is
recorded because it is the order in which the numbers are written down, and
because H3's visibility precondition can invalidate H3's verdict without
touching H1' — whereas nothing in H1' can invalidate H3. §5.3's precedence and
double-fire rules are internal to H1' and stand unchanged.

### 6.2 One line per hypothesis

One ADR line per hypothesis, written from the numbers with no threshold moved
after any sample is taken. A confirmed H3-e licenses WP-1.5a's window-storage
matrix; it does not choose among its options — that is the WP's own matrix.
H3-c gets a line that records a number and licenses nothing, which is what
"measurement only" means in this project. INCONCLUSIVE is a first-class outcome
of this document and not a failure to report: it licenses nothing, closes
nothing, and carries the method debt forward by name. Anything else this profile
surfaces is an observation for a future pre-registration, adjudicated never
(D-114, D-193's precedent).

### 6.3 ADR numbering, fixed here so no line lands on a taken number

D-218 and D-219 stand as landed (the SB-65 placement-distance work); they are
not this experiment's. The numbering for what follows:

- **D-220 — RESERVED for the official delta bench verdict** (D-215's successor,
  the operator's own run of `tools/bench_delta.sh`). This commit writes D-220 as
  a RESERVATION line rather than leaving a hole in an append-only log; its body
  is completed in place when the bench runs, which D-220 records as a deliberate
  and single exception to the append-only discipline.
- **D-221** — this revision: the operator's rulings, the H3-e/H3-c split, and
  the `docs/experiments/` class exemption from rule 9's soft cap ("a governing
  document's single-artifact integrity outweighs the file-size discipline; the
  mechanized gate's `.rs` scope is deliberate, not incidental").
- **D-222** — the deferral recorded in §7 (amending D-212).
- The re-profile's own lines follow the bench: **H3-e, H3-c, H1'-a, H1'-b**, one
  each, plus the line recording this document's own review round.

Any earlier runbook or draft naming a different number for the bench verdict is
superseded by this section.

## 7. What this document does not settle

**The time-matched r2-vs-r3 play-mode experiment (D-212) is DEFERRED, not
cancelled.** `configs/play_v0.toml` stays at radius 3, already justified by
WP-1.3's 1.9x wall-clock finding. WP-1.5b supersedes the radius policy outright,
so tuning it now is spend on code with weeks to live. **The deferral flips only
if platform play precedes WP-1.5b** — at which point the radius in the shipping
play config is a live question again and D-212's licensed experiment is the way
to answer it.
