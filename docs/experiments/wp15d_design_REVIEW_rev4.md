# WP-1.5d — SCOPED RE-REVIEW, design revision 4

## Header

**Artefact.** `docs/experiments/wp15d_design.md` at **revision 4**, commit
**`bd1faf4df57f9ad716e82f7f58e08fcdf20b1738`**.

**Does it match HEAD?** **YES.**

```
$ git rev-parse HEAD
bd1faf4df57f9ad716e82f7f58e08fcdf20b1738
$ git status --short
(no output — clean)
```

**Governing grant.** `docs/decisions.md` D-480 — ONE capped continuation round,
scoped to `wp15d_design_REVIEW_rev3.md`'s ten enumerated remedies and nothing
else, under a single scoped re-review where **any NEW finding is a STOP and a
split**.

### THE SCOPING DIFF — PROVED, NOT ASSERTED

```
$ git diff 980bbb5..bd1faf4 -- . ':(exclude)docs/experiments/wp15d_design.md' \
      ':(exclude)docs/decisions.md' ':(exclude)docs/experiments/wp15d_design_REVIEW_rev3.md'
(no output)
```

**Empty. The round stayed inside its scope.** The whole diff is three files and
nothing else:

```
$ git diff --stat 980bbb5..bd1faf4
 docs/decisions.md                            |   4 +
 docs/experiments/wp15d_design.md             | 196 ++++--
 docs/experiments/wp15d_design_REVIEW_rev3.md | 950 +++++++++++++++++++++++++++
 3 files changed, 1085 insertions(+), 65 deletions(-)
```

`docs/decisions.md` gains D-480 (the grant itself) and
`wp15d_design_REVIEW_rev3.md` is the prior report being landed — both are the
grant's own paper. No code, no config, no tool, no artifact was touched.

**The mechanism sections are unchanged in substance.** Reading
`git diff 980bbb5..bd1faf4 -- docs/experiments/wp15d_design.md` hunk by hunk, the
edits fall in the preamble, §3, §4's table, §5, §7 row 5, and §8. **§1, §2
(2.1–2.5) and §6 (6.1–6.4) carry no diff hunk at all.** The guard, the site, the
truncate-before-promote ordering, the K-counts-cells argument, the tie-break
argument and the whole store-rule section are byte-identical to revision 3. This
matches D-480's "may not revisit … the mechanism".

### WHAT I READ

`docs/experiments/wp15d_design.md` (rev 4, all 608 lines);
`docs/experiments/wp15d_design_REVIEW_rev3.md` (all ten findings and their
one-clause remedies); `docs/experiments/wp15d_design_REVIEW.md`;
`CLAUDE.md`; `docs/process.md` (all 79 lines); `docs/decisions.md` D-480, D-478,
D-479, D-395, D-424, D-423, D-374, D-291, D-74, D-22, D-9, D-7;
`tools/bench_block.sh` header and `--grammar` handling; `tools/determinism.sh`
SEATS/BUDGETS; `crates/pistol-search/src/pvs.rs`,
`crates/pistol-search/src/search.rs`, `crates/pistol-search/src/info.rs`,
`crates/pistol-search/src/staged.rs`, `crates/pistol-search/src/tt/entry.rs`,
`crates/pistol-engine/src/validate.rs`, `crates/pistol-cli/src/bin/pistol.rs`;
`configs/instrument_staged_v0.toml`, `configs/gate_staged_v0.toml`.

### WHAT I RAN

Worktree **`/home/tom/Projects/pistol-wt-rr4`** (detached at `bd1faf4`), its own
`CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rr4-target`, both on `/home`. The
live tree was never edited. **The worktree is LEFT IN PLACE as the dispatch
directs**, and it carries uncommitted probe edits to
`crates/pistol-search/src/pvs.rs` — an `eprintln!` at §2.1's site plus an
env-gated implementation of §2.2's guard and §6.3's store rule (`SNK` = K,
`SNSCOPE=1` selects the `ply > 1` mutant, `SNDRY=1` counts the predicate without
truncating). That patch is what every measurement below was taken with.

### DIGESTS VERIFIED

Against `artifacts/wp15d_worktree_export_receipt_v3.txt`:

```
$ sha256sum artifacts/wp15d_turn_axis_v1.txt artifacts/wp15d_turn_axis_book_v1.txt \
            artifacts/wp15d_m2_evidence_instrument_v2.txt
5a64034e3ab178beecded86359692a2076a66ae89d3e4961c83b1ac4d082a3ae  artifacts/wp15d_turn_axis_v1.txt
43fa71ce9cc6e99cf69aee40493096c4a3d30301ed97cd1c88794df5c3026c10  artifacts/wp15d_turn_axis_book_v1.txt
f73608dd3693762e02968e6ec9a4c8078ac109fcc44bf3580ab8b1fc437d632c  artifacts/wp15d_m2_evidence_instrument_v2.txt
```

All three match the receipt line for line. Every artifact cell quoted below is
from one of these three files, at these digests.

---

## CLOSURE LEDGER

One row per finding of `wp15d_design_REVIEW_rev3.md`.

| # | sev | finding (short) | status |
|---|---|---|---|
| 1 | BLOCKING | §8 registers a per-position counter its own registered instrument cannot read | **CLOSED** |
| 2 | BLOCKING | §8's corpus paragraph quotes an incumbent from another artifact, a seat from another scope, three ratios of the wrong sign | **CLOSED** |
| 3 | BLOCKING | §5's mate exclusion makes `lift`'s population K-dependent; `lift` defined twice; D-395 did the opposite | **CLOSED** |
| 4 | MAJOR | §7 row 5's assertion is not expressible with §4's observables | **CLOSED BUT INTRODUCED SOMETHING** |
| 5 | MAJOR | §8's spread anchor has no instrument/artifact/digest/K, and its r = 3 config is not in the tree | **CLOSED** |
| 6 | MAJOR | §5's VOID threshold cannot fire; the reason given that it can is refuted by the code | **CLOSED** |
| 7 | MAJOR | §8's/§9's `1 / 0 / 0 / 0` depth vector is the r = 3 reading under an r = 2 seat | **CLOSED** |
| 8 | MINOR | §7 row 5's pool figure is 18, not 19 | **CLOSED** |
| 9 | MINOR | §4's "151 of 153" crosses two budgets | **CLOSED** |
| 10 | MINOR | §3's named check can never fire, in the paragraph that deletes another for never firing | **CLOSED BUT INTRODUCED SOMETHING** |

### Row-by-row

**1 — CLOSED.** §8 now splits the legs explicitly: *"The timing legs are
`tools/bench_block.sh`'s; the counter leg is a harness in the
`crates/pistol-search/` test tree, `crates/pistol-search/tests/wp15d_bench_counters.rs`,
calling `Searcher::search` directly and reading `SearchOutcome::info.stages`."*
I verified the observable exists and is populated on every path:
`search.rs:457` is `outcome.info.stages = run.stages;`, executed after the
completed-depth, salvage and fallback arms alike, so the zeroed
`StageCounters::default()` in the salvage/fallback constructors is always
overwritten. `crates/pistol-cli/src/report.rs` is untouched by this route, so §7
row 1's sha-pinned transcript survives — the other horn of the rev3 trap is
closed too. §4 adds the harness as a new file with its governing revision named
in the receipt.

**2 — CLOSED.** Every cell re-checked against
`artifacts/wp15d_turn_axis_v1.txt`'s `S2/CORPUS` block:

```
$ /usr/bin/grep -n "S2/CORPUS" artifacts/wp15d_turn_axis_v1.txt | sed 's/depths=.*//'
202:test zz_s2_corpus ... S2/CORPUS/nodes50000/r2/scope0/K0 sum_nodes=1104026 sum_ms=4776 sn_rows=66 capped=0
203:S2/CORPUS/nodes50000/r2/scope4/K4 sum_nodes=1104026 sum_ms=4828 sn_rows=17 capped=9
204:S2/CORPUS/nodes50000/r2/scope4/K8 sum_nodes=1104026 sum_ms=4795 sn_rows=18 capped=10
205:S2/CORPUS/nodes50000/r2/scope4/K16 sum_nodes=1104026 sum_ms=4790 sn_rows=21 capped=13
206:S2/CORPUS/nodes50000/r2/scope1/K8 sum_nodes=1104026 sum_ms=4779 sn_rows=17 capped=17
```

Incumbent **4 776** ✓ (the cross-run 4 825 is gone). Ratios against it:
4828/4776 = 1.010888 → **1.011** ✓; 4795/4776 = 1.003978 → **1.004** ✓;
4790/4776 = 1.002931 → **1.003** ✓. All three ≥ 1.000, agreeing with §8's own
cost-channel hotspot ✓. `scope1/K8` is quoted only to be excluded, and named
correctly as the rejected `ply > 0` scope (the artifact's legend: scope 1 =
"every ply but 0") ✓. The two structural claims also hold: Σ nodes is 1 104 026
on all five seats, and the `depths=` field is byte-identical across all five —

```
$ sed -n '202,206p' artifacts/wp15d_turn_axis_v1.txt | sed -n 's/.*depths=\(.*\)/\1/p' | LC_ALL=C sort | uniq -c
      5 3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3
```

— 24 values, as stated ✓.

**3 — CLOSED.** `lift(K)` is now a single definition: *"a COUNT over the FIXED
population `2 000 − |{openings mate-terminated on ANY seat, the incumbent control
included}|`"*, and the "numerator and denominator" sentence is deleted with the
per-seat rule. That is D-395's own shape ("19 of the 24 … on BOTH sides"). Three
things I checked rather than assumed:

- **The exclusion is observable.** Mate-termination is a real mechanism, not a
  supposition: `search.rs:357` is `if is_mate(score) { break; }` inside the
  deepening loop, so a seat/opening pair is mate-terminated iff
  `is_mate(outcome.info.score)` with `depth_turns < 3`. The harness reads both
  off `SearchOutcome`.
- **It is computable — as a second pass, which is all the design claims.** It is
  NOT computable before the seats run, and the design does not say it is: *"That
  excluded set is computed once over all seats."* Run seven seats, record per
  (seat, opening), union the mate-terminated indices, then count. Deterministic
  function of the run, not a post-hoc choice, so D-374 is satisfied.
- **It interacts correctly with the 90 % rule.** The population is now identical
  at every K, which is exactly what comparing counts across seats requires. The
  degeneracy rev3 found — a seat shrinking its own denominator — cannot occur,
  because there is no denominator and the excluded set is seat-invariant.

**4 — CLOSED BUT INTRODUCED SOMETHING.** The expressibility half is genuinely
closed: row 5 no longer asserts "the emitted width at that node" and instead
asserts a whole-search counter, `safety_net_capped_rows`, which §4 does provide.
**But the registered VALUE is false.** `== 19` is a count of predicate
satisfaction on the UNCAPPED tree; on the capped tree the counter reads **9**.
See NEW FINDING 1.

**5 — CLOSED.** The r = 3 cells are dropped entirely, so the un-committed
`instr_r3.toml` no longer underwrites anything ✓. K is stated as measured
invariance ✓ — and I verified it, over more of the grid than the design claims:

```
$ # uncapped tree, configs/instrument_staged_v0.toml (quiet_radius = 2), go nodes 50000
p0 stones 11: K4=95  K8=95  K16=95  K32=95  K48=95  K64=95
p1 stones 21: K4=5   K8=5   K16=5   K32=5   K48=5   K64=5
p2 stones 51: K4=152 K8=152 K16=152 K32=152 K48=152 K64=152
p3 stones 99: K4=0   K8=0   K16=0   K32=0   K48=0   K64=0
```

**95 / 5 / 152 / 0 reproduces exactly**, and the invariance holds not only over
K ∈ {4, 8, 16} as claimed but over §5's whole grid to 64 — which retires rev3's
"invariance at 64 is not established" caveat. The remaining part of rev3's
attack, "no instrument, no artifact, no digest", is answered structurally rather
than by naming one: the design de-registers the figure — *"That figure governs
nothing; the registered number is the ON seat's counter"* — and
`docs/process.md`'s instrument rule binds an artefact that produces **a
registered number**. A number that governs nothing is context, the same category
§8 already uses for the nps ratio and for `safety_net_stores_withheld`. I record
the residual honestly: a later reader still has no committed producer to re-run
for this cell, and my reproduction is evidence it is true, not a substitute for
one. It is below finding threshold because nothing concludes from it.

**6 — CLOSED.** The VOID threshold is deleted, not re-argued, and the histogram
reporting is kept. The code citations are correct: `search.rs:323` is
`let abortable = depth_turns > 1 || fallback.is_some();`, `fallback` is `None`
under `Stop::DepthTurns | Stop::Nodes` (`search.rs:232-238`), and `pvs.rs:730` is
`assert!(score.is_some(), "a non-abortable iteration completes")`. Completed
depth 0 is unreachable at the registered budget, so deleting the threshold is
right on D-424's test. The design also now grounds the "defined for every
opening" claim in the artifact rather than in argument, and it holds: the book
incumbent's `depth_hist` is `[0, 10, 1869, 113, 8, 0, 0, 0]` — index 0 is **0**.

**7 — CLOSED.** Verified at the named seat and at every capped seat of the same
radius:

```
$ /usr/bin/grep -n "S1/r2/nocap" artifacts/wp15d_turn_axis_v1.txt
33: S1/r2/nocap/K0 p00 stones= 11 depth=1 ...
34: S1/r2/nocap/K0 p01 stones= 21 depth=1 ...
35: S1/r2/nocap/K0 p02 stones= 51 depth=0 ...
36: S1/r2/nocap/K0 p03 stones= 99 depth=0 ...
```

**1 / 1 / 0 / 0** ✓, at `movetime 500` (the S1 harness's own
`run(&st, radius, Some(500), …)`, `wp15d_turn_axis_v1.txt:340`) on `spread_v1`.
§9's "with the cap armed and without it" also holds: `S1/r2/except-root-turn` is
`1 / 1 / 0 / 0` at K = 4, 8, 16 and 32 alike. The r = 3 provenance of the old
`1 / 0 / 0 / 0` is stated, and the artifact's own header licenses the cell as
SCOPING-ONLY, which is how §8 and §9 now use it.

**8 — CLOSED.** Measured at the design's own site:

```
SNPROBE ply=0 tfr=0 sn=true pool=1     (x2)
SNPROBE ply=1 tfr=1 sn=true pool=18    (x1)
SNPROBE ply=2 tfr=1 sn=true pool=22/26/27 (x18)
```

The ply-1 pool is **18** ✓ — the radius-2 ball's 19 cells minus the occupied
origin, `within_radius`'s `is_legal_placement` filter. The row's other parts hold
too: the ply-1 node is at `turns_from_root() == 1`, it is a safety-net row, and
it is the only node where the two spellings differ.

**9 — CLOSED.** Re-measured over the 20 `^position ` lines of
`tactical_staged_v0.txt` under `configs/gate_staged_v0.toml`, at both of
`tools/determinism.sh`'s budgets (`depth_turns 4`, `nodes 200000`):

```
budget=depth_turns 4    SAFETY_NET_rows=151 rows_pool_gt_8=151 pool_range=33..52
budget=nodes 200000     SAFETY_NET_rows=153 rows_pool_gt_8=153 pool_range=33..52
```

**151 of 151 and 153 of 153**, pool range **33..52** — exactly as §4 now states,
and the crossed fraction is gone. The conclusion the row rests on (K = 8 binds on
that seat, so `gate_staged_snk_v0.toml` is not a vacuous determinism seat) is
carried by the corrected numbers.

**10 — CLOSED BUT INTRODUCED SOMETHING.** §3's deletion is correct on the merits
(see WHAT I CHECKED AND FOUND SOUND). But §4's table was not brought along with
it and still sends the implementer to §3 for a check §3 no longer names. See
NEW FINDING 2.

---

## VERDICT: **FAIL**

Two new findings — one MAJOR, one MINOR, **neither a correctness finding**. Both
sit in the two ledger rows marked CLOSED BUT INTRODUCED SOMETHING, which is the
category D-480's cap turns on. Under D-480 this is a **STOP and a SPLIT**, and
the next step is the architect's.

I record the shape of the result plainly, because it bears on what the architect
decides. Eight of the ten remedies closed cleanly and I could reproduce every
number behind them. The three BLOCKINGs are all closed, including the one the
rev3 report called a closed trap on both sides. The mechanism was not touched and
remains, across four reviews now, without a correctness finding. What failed is
narrower than the round: **one registered test constant, and one stale
cross-reference left by the round's own last edit.**

---

## NEW FINDINGS

### NEW FINDING 1 — MAJOR. §7 row 5 registers `safety_net_capped_rows == 19` for the shipped guard; the shipped guard produces **9**. The 19 is the predicate's count on the UNCAPPED tree, and capping is what changes the tree. **Registration / implementability finding, not correctness.**

**Claim attacked, verbatim (`wp15d_design.md:458`):**

> "**The assertion is stated in the observable §4 actually provides** (re-review
> MAJOR 4 — `info.rs`'s counters are whole-search totals, so "the emitted width at
> that node" is not expressible): at an empty-board root, `depth_turns 2`,
> `safety_net_top_k = 8`, **`safety_net_capped_rows == 19` shipped and `== 18`
> under the mutant**. This is the only position at which the two spellings differ
> at all"

**THE ATTACK.** `safety_net_capped_rows` is a counter on the tree the search
actually walks. `StageCounters`' every existing field is of that kind — *"Nodes
that took the WIN-NOW row"*, *"Nodes that took the BATCHED row"*
(`info.rs:26-43`) — so `safety_net_capped_rows` counts nodes at which the cap
FIRED, with the cap armed.

But firing the cap is what changes the tree. At an empty-board root the ply-1
node's pool is 18; truncating it to K = 8 removes ten of the eighteen ply-2
children, and every one of those children was itself a capped row. So the shipped
tree has 1 ply-1 capped row + 8 ply-2 capped rows = **9**, not 19.

19 is what the predicate `turns_from_root() > 0 && used_quiet_safety_net &&
pool > 8` counts when you evaluate it and DO NOT truncate — the uncapped tree's
18 ply-2 rows plus the 1 ply-1 row. That is the rev3 reviewer's probe number
(`wp15d_design_REVIEW_rev3.md:498-510`), and revision 4 adopted it verbatim as
the value of a counter measured on the capped tree. **The mutant's 18 is right
by coincidence**: under `ply > 1` the ply-1 node is not truncated, so its 18
children survive and are each capped — 18 on both trees.

This is the round's own named defect class, fifth occurrence: *"a number quoted
away from the run that produced it"*, here a number quoted away from the tree
that produced it. And it is the one number in revision 4 that does not obey the
preamble's own mechanical rule — *"every number below now names the artifact, the
seat, the budget and the radius it comes from, or it is deleted"*: this cell
names a seat and a budget but no artifact and no producer, and it is the only
number I checked that turned out false.

**REPRODUCED.** Worktree `/home/tom/Projects/pistol-wt-rr4` at `bd1faf4`, with
§2.2's guard and §6.3's store rule implemented at the design's own site
(`pvs.rs`, immediately after the empty-set check and before
`set.promote_table_move`), `SNSCOPE=1` selecting the `ply > 1` mutant:

```
$ B=/home/tom/Projects/pistol-wt-rr4-target/release/pistol
$ for scope in 0 1; do
    printf 'newgame\ngo depth_turns 2\nquit\n' \
      | SNK=8 SNSCOPE=$scope $B --config configs/instrument_staged_v0.toml 2>&1 >/dev/null \
      | /usr/bin/grep -c 'truncated=true'
  done

### §7 row 5, exactly as registered: empty-board root, depth_turns 2, safety_net_top_k = 8, quiet_radius 2
SHIPPED  turns_from_root() > 0 -> safety_net_capped_rows = 9
MUTANT   ply > 1               -> safety_net_capped_rows = 18

### the same predicate counted WITHOUT truncating (the uncapped tree) -- where 19 comes from
SHIPPED  turns_from_root() > 0 -> predicate satisfied on the UNCAPPED tree = 19
MUTANT   ply > 1               -> predicate satisfied on the UNCAPPED tree = 18
```

The per-row breakdown shows the mechanism directly:

```
### shipped, cap armed
      2 SNPROBE ply=0 tfr=0 sn=true truncated=false
      1 SNPROBE ply=1 tfr=1 sn=true truncated=true
      8 SNPROBE ply=2 tfr=1 sn=true truncated=true     <- 8, not 18
### mutant, cap armed
      2 SNPROBE ply=0 tfr=0 sn=true truncated=false
      1 SNPROBE ply=1 tfr=1 sn=true truncated=false
     18 SNPROBE ply=2 tfr=1 sn=true truncated=true
```

The result is stable with and without §6.3's store rule (9 both ways), and the
gate-off control reads 0 at `SNK=0`, as §7 row 1 requires.

**Why MAJOR and not BLOCKING.** The falsifier genuinely EXISTS and the row's
structure is right: 9 ≠ 18 discriminates the two spellings at the same position,
in the same observable, at the same K. The design's purpose for row 5 is
achievable and the correction is one number. Nothing here touches the engine's
correctness.

**Why MAJOR and not MINOR.** This is the sole falsifier for the round's headline
correction — the `turns_from_root() > 0` vs `ply > 1` guard — and it is the fix
for the original REVIEW-design BLOCKING 2 ("no test could kill the `ply > 1`
mutant"). An implementer who writes the registered assertion writes
`assert_eq!(info.stages.safety_net_capped_rows, 19)` and ships a RED test on
correct code; the only way out is to invent a replacement number, which is the
after-the-numbers move D-374 exists to forbid. A registered expectation that is
false at its own registered seat is the class this whole round was granted to
eliminate.

**What closes it.** One number: `safety_net_capped_rows == 9` shipped, `== 18`
under the `ply > 1` mutant. It would be worth adding the config the cell is taken
under — the count depends on `quiet_radius = 2` with the WP-1.7 heuristics and
the solver OFF, which is `configs/instrument_staged_v0.toml` as committed — since
the row currently names a radius but no document.

---

### NEW FINDING 2 — MINOR. §3 deletes the validation check; §4's table still sends the implementer to §3 for "the check it adds" and tells them to add "the SAME check again" in a second crate. **Implementability finding, not correctness.**

**Claims attacked, verbatim (`wp15d_design.md:253` and `:256`):**

> "| `crates/pistol-engine/src/validate.rs:81-92` | destructures all fields with
> no `..` — **will not compile** without the new one; **§3 names the check it
> adds** |"

> "| `crates/pistol-search/src/search.rs` | **the SAME check again**, because a
> `SearchParams` can be built in code and never pass through a document (rule 1)
> — the two-crate pattern `radius` and `q_depth_turns` already follow |"

against `wp15d_design.md:213`:

> "**THERE IS NO VALIDATION CHECK, AND THAT IS THE ANSWER RATHER THAN AN
> OMISSION.**"

**THE ATTACK.** Revision 4 rewrote §3 to delete the check and left §4's table
rows untouched — they are context lines in the diff, carried over from revision 3
where the check still existed. The document now says both that there is no check
and that two files add one.

The `validate.rs` row is merely stale: its operative content (destructures with
no `..`, will not compile) is correct and I verified it —
`validate.rs:81-92` lists all ten `Staged` fields with no `..`. Only the trailing
clause dangles.

**The `search.rs` row is the substantive half, and it leaves a decision to
invent.** It is the ONLY reason `search.rs` appears in the enumeration: the
row's whole content is "the SAME check again". With no check there is nothing for
that row to describe — and `search.rs` is then not a change site at all, because
it holds no `StagedParams` literal and reads staged fields by name
(`staged.quiet_radius`, `staged.q_depth_turns`), so a new field does not force it
to compile-error:

```
$ /usr/bin/grep -c "StagedParams {" crates/pistol-search/src/search.rs
0
```

So an implementer must choose between (a) §3 — add no check, and drop `search.rs`
from the change set; and (b) §4 — add a check in both crates, which means either
re-introducing the never-firing `usize` check §3 just deleted (regressing rev3
MINOR 10) or inventing a bound §3 explicitly forbids ("**No upper bound**"). §4
is the section whose entire purpose is an exact enumeration — round 1's BLOCKING 3
was that it was short by six files — and it is now long by one.

**REPRODUCED.** Every "check" in the document, at `bd1faf4`:

```
$ /usr/bin/grep -n "check" docs/experiments/wp15d_design.md
120:**In `pvs.rs`, immediately after the empty-set check and BEFORE
217:`u64`-representable-as-`usize` check, **which on this target can never fail** —
218:and it did so in the paragraph that deletes another check for never firing
253:| `crates/pistol-engine/src/validate.rs:81-92` | ... §3 names the check it adds |
256:| `crates/pistol-search/src/search.rs` | the SAME check again, because a ...
513:the derivation is checkable rather than asserted.
```

Lines 253 and 256 are the only two forward references to a check, and §3
(213-224) names none. The diff confirms the rows were not part of the revision-4
edit:

```
$ git diff 980bbb5..bd1faf4 -- docs/experiments/wp15d_design.md | sed -n '/@@ -245,9/,/@@ -301,25/p'
(the validate.rs and search.rs rows appear as unchanged context lines)
```

**Why MINOR.** §3's statement is bold, unambiguous and argued, so a careful
implementer resolves the contradiction toward "no check" and the residue is one
file wrongly listed in an enumeration and one dangling clause. Nothing about the
engine's behaviour is at stake, and no measurement depends on it. It is recorded
because §4 is the section a previous round made BLOCKING for being wrong about
its own file list, and because leaving a claim in one section that another
section has deleted is precisely the D-423 hygiene this document invokes twice.

**What closes it.** Delete the trailing clause from the `validate.rs` row, and
delete the `search.rs` row (or replace its content with whatever non-check reason
keeps it a change site, if one exists).

---

## WHAT I CHECKED AND FOUND SOUND

**§3's deletion is right, and "the type is the domain" is TRUE of
`safety_net_top_k: u64` as this design specifies it.** I checked this against
hard rules 1 and 3 rather than taking the paragraph's word.

- Rule 1 asks for explicit, complete, `deny_unknown_fields` config with no
  code-side default and missing-key = error. A plain `u64` field with no
  `#[serde(default)]` delivers exactly that; a validator adds nothing to it.
- Rule 3 asks that wrong-kind/wrong-shape input raise a named error. Serde
  refuses a negative, fractional or non-numeric value for a `u64` field by name.
  There is no shape left that reaches the search.
- The value domain really is exhausted by the type. `0` is the off-value; every
  `n > 0` is a legal ceiling; `set.cells.truncate(k as usize)` is total on a
  64-bit target; a K above every pool is a no-op the design deliberately admits.
- **The `radius` / `q_depth_turns` precedent does not transfer, and that is the
  point.** Those two-crate checks exist because each has a domain the type cannot
  express: `search.rs:87-107` refuses `radius == 0` ("a radius of 0 reaches only
  occupied cells") and `i16::try_from(radius).is_err()` because `Coord` steps are
  `i16`; `search.rs:138-146` refuses `q_depth_turns > MAX_Q_EXTENSION_PLIES / 2`
  because `MAX_PLY` was sized for it. `safety_net_top_k` indexes a `Vec` length.
  There is no analogous constraint, so there is no analogous check — which is why
  the §4 rows that still assert one are FINDING 2 rather than a defence of the
  deleted check.

**§8's two-instrument split is sound, and `docs/process.md`'s second-instrument
rule does not apply to it.** That rule governs *"doubt about the instrument …
answered by REPLICATION and by a SECOND INSTRUMENT whose agreement criterion is
registered before either runs"*, and it warns that *"two instruments blind to the
same stage are one instrument reported twice"*. §8's two instruments are not
redundant measurements of one quantity awaiting agreement — they carry disjoint
legs: `bench_block.sh` produces wall times over the line protocol,
`wp15d_bench_counters.rs` produces `StageCounters` fields that the line protocol
structurally cannot carry. There is nothing for an agreement criterion to be
about, so registering one would be the never-firing prose §3 and §5 delete. The
design keeps the one rule that does bind — *"a number from one is never quoted
beside a number from the other without saying which produced it"* — and §9
correctly defers the genuine second-instrument machinery to the pre-registration,
where a governed run lives.

**`SearchOutcome::info.stages` does carry what §8 wants.** `SearchInfo` is
constructed with `stages: run.stages` in the completed-depth arm
(`search.rs:348`) and with `StageCounters::default()` in the salvage and fallback
arms — but `search.rs:457` overwrites unconditionally with
`outcome.info.stages = run.stages;` before `Ok(outcome)`. Under `Stop::Nodes` the
salvage arm is unreachable anyway (`search.rs:232-238` gives `fallback = None`
and `salvage = None`), so both the calibration and the bench counter leg read a
true whole-search total.

**§8's ADVISORY spread anchor is coherent as written.** The dispatch asked
whether an uncapped-tree ADVISORY figure can sit beside a registered ON-seat
counter. It can, because the spread band is REPORTED, NOT GATED — no verdict
rests on either number — and the design says in terms which is which and that the
advisory one governs nothing. The residual I flag without raising: the anchor is
a poor predictor of the number it anchors, since the ON seat's counter is taken
on a different (capped) tree at a different budget from the S1 cells; but as
nothing concludes from it, that costs nothing. Reported here so the architect can
see it was considered.

**§7's mutant table is otherwise coherent.** Row 5's uniqueness claim — *"the
only position at which the two spellings differ at all"* — is true by rule 3:
`turns_from_root() > 0` and `ply > 1` can differ only where ply 1 already belongs
to a new turn, which requires the root turn to be one stone, which is turn 1
alone. At any root of turn ≥ 2 both plies 0 and 1 sit in the root turn and both
spellings agree. Rule 4 opens no exception, because a first stone that wins
returns before a ply-1 node exists.

**§2's site and §6.3's store rule are implementable as written**, which I
confirmed by implementing them. The guard's site is real
(`pvs.rs:322-328`: the `set.cells.is_empty()` check, then
`set.promote_table_move(table_move)`); `turns_from_root()` is in scope as a `Run`
method (`pvs.rs:523-525`); `used_quiet_safety_net` is already on `StagedSet`
(`staged.rs:65-69`). The one mechanical move the design does not spell — hoisting
`truncated` out of the `let cells = match self.policy { … }` arm so §6.3 can read
it at the store — has an exact precedent two lines above the same match
(`let mut forced_bound: Option<usize> = None;`, `pvs.rs:283`), and `Bound` derives
`PartialEq` (`tt/entry.rs:22`), so `bound == Bound::Lower` compiles. Lifting the
inline bound expression out of the `Record` literal (`pvs.rs:457-463`) is the
shape §6.3 prints. None of this required a decision I had to invent.

**§4's three enumerations reproduce exactly**, at `bd1faf4`:

```
$ /usr/bin/grep -l 'kind = "staged"' configs/*.toml | LC_ALL=C sort | wc -l
12
$ /usr/bin/grep -rln 'kind = "staged"' crates/ --include=*.rs
crates/pistol-engine/tests/common/mod.rs
$ /usr/bin/grep -rln 'StagedParams {' crates/ --include=*.rs | wc -l
11
$ /usr/bin/grep -rn 'StagedParams {' crates/ --include=*.rs | wc -l
18
```

Twelve documents, one embedded TOML, eleven files and eighteen sites — and all
eleven literal-holding files appear in the table. Round 1's BLOCKING 3 stays
closed.

**`pistol.rs`'s exclusion is correct.** `pistol.rs:154-158` destructures
`CandidatePolicy::Staged { quiet_radius, quiet_top_k, .. }`, so the new field
forces no change there and the handshake line is untouched — which is what keeps
D-356 and `U2_node_protocol.md` out of this WP.

**`tools/determinism.sh`'s seat count is right.** The `SEATS` array holds four
entries (`radius`, `staged`, `staged-heuristics`, `staged-solver`), so
`gate_staged_snk_v0.toml` is genuinely the **fifth** seat, and `BUDGETS` is
`("depth_turns 4" "nodes 200000")` — the two budgets §4's 151/153 cells are taken
at.

**§5's cost and flatness cells all reproduce** from
`artifacts/wp15d_turn_axis_book_v1.txt` (digest verified above), reading `lift`
as `Σ depth_hist[3..]`:

```
S4/BOOK/nodes50000/r2/scope0/K0   lift(>=3)= 121  depth0=0  pool_mean=78.16  cut/in_pool= 0.0%  ms=376953
S4/BOOK/nodes50000/r2/scope4/K4   lift(>=3)= 535  depth0=0  pool_mean=77.66  cut/in_pool=79.6%  ms=425606
S4/BOOK/nodes50000/r2/scope4/K8   lift(>=3)= 524  depth0=0  pool_mean=77.70  cut/in_pool=69.3%  ms=426503
S4/BOOK/nodes50000/r2/scope4/K16  lift(>=3)= 514  depth0=0  pool_mean=77.72  cut/in_pool=57.7%  ms=429505
```

**535 / 524 / 514** ✓, **79.6 % / 69.3 % / 57.7 %** ✓, pool mean **77.7** ✓,
incumbent depth-0 count **0** ✓, sweep anchor **377–430 s** ✓ (376 953 … 429 505
ms). The K = 32 cell is in the round-3 red-team report as stated
(`matrix_M2_REDTEAM_round3.md:507`, `ge3=490`) ✓, so the "flat across the low
grid, decays above it" shape the 90 % rule depends on is the measured shape. §8's
book anchor checks too: 426 503 / 376 953 = 1.1314 → **1.131×** ✓.

### Attacks that did NOT reproduce

- **"The block's fourth seat is `scope1/K8`."** Counted down the artifact's
  `S2/CORPUS` lines, `scope1/K8` is the fifth. But it is the fourth *capped* seat,
  which is the sense rev3's report used ("the fourth of 'the four seats'"), and
  the seat is named by its own exact `scope1/K8` identifier, so no reader can be
  pointed at the wrong row. Ordinal wording, not a number quoted away from its
  run — below finding threshold.
- **A larger K flattering itself through the fixed population.** I pushed on the
  union exclusion in both directions. It cannot flatter any seat, because the
  excluded set is identical at every K by construction. A residual asymmetry
  exists in principle — an opening a small K mate-terminates spuriously is
  removed from a large K that would have counted it, so the union biases mildly
  AGAINST large K — but D-395 is exact precedent for the union shape, the design
  prints the excluded set by opening index so its magnitude is auditable rather
  than asserted, and the book histograms suggest the set is small (the incumbent
  has 10 openings at depth 1 out of 2 000). Not a finding.
- **§7 row 1 endangered by the new counters.** It is not: the counter leg runs in
  the `pistol-search` test tree, `report.rs`'s explicit field list is untouched,
  and the gate-off transcript is unchanged. I confirmed the gate-off control
  reads `safety_net_capped_rows = 0` at `SNK=0`.
- **`lift`'s exclusion being uncomputable.** It is computable, as a second pass
  over seats that have all run, which is what the design says.

---

## SUMMARY FOR THE OPERATOR

Revision 4 stayed inside D-480's scope — the diff proves it — and touched no
option, no selection and no mechanism. Eight of the ten remedies are closed and
every number behind them reproduces against the artifact, digest, seat, budget
and radius it now names. All three BLOCKINGs are closed. The mechanism is still
without a correctness finding after four reviews.

Two rows closed but introduced something, and that is the cap's trigger:

1. **MAJOR** — §7 row 5's registered `safety_net_capped_rows == 19` is the
   uncapped tree's predicate count; with the cap armed the counter reads **9**.
   The mutant's 18 is right. The falsifier still exists and still discriminates;
   the registered constant is wrong, and as written the test ships red on correct
   code. One number closes it.
2. **MINOR** — §3 deleted the validation check and §4's table was not brought
   along; two rows still point at it, and one of them (`search.rs`) is now
   probably not a change site at all. Two clauses close it.

Both are the same shape as the eight that closed: a claim stated away from what
produced it. Neither reaches the code. Per D-480 the WP splits here and the next
step is the architect's.

**Worktrees used and LEFT IN PLACE, as directed:**
`/home/tom/Projects/pistol-wt-rr4` (detached at `bd1faf4`, carries the
uncommitted probe patch to `crates/pistol-search/src/pvs.rs` that every
measurement above was taken with) and its build directory
`/home/tom/Projects/pistol-wt-rr4-target`. Nothing was removed.
