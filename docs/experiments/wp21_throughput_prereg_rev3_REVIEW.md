# REVIEW-design (fresh context) — `wp21_throughput_prereg.md` revision 3, ROUND 2

**NAMED REVISION.** `docs/experiments/wp21_throughput_prereg.md` as of commit
**`114b5623fe40698bb0cf73b04990424935981e26`**, branch `overnight2-stopped`.

**DOES IT STILL MATCH HEAD?** **YES, and HEAD moved under me during the review.**
When I started, `git rev-parse HEAD` was `114b562`; it is now
`64dbd6ca8a8774fcd7d78de42633df50a9782ed7`. The reviewed file is unaffected:

```
$ git diff 114b562 HEAD --stat
 docs/experiments/arc3_ledger.md | 103 +++++++++++++++++++++++++++++++++++++
$ git diff 114b562 HEAD    -- docs/experiments/wp21_throughput_prereg.md | wc -l
0
$ git diff 114b562         -- docs/experiments/wp21_throughput_prereg.md | wc -l   # vs working tree
0
```

The file is byte-identical at `114b562`, at HEAD and in the working tree. Note
that the file itself was last written at `709ad32`; `114b562` touched only
`docs/decisions.md` (adding **D-581**), which matters for finding **MAJOR 1** and
**MAJOR 2** below.

**REVIEWER CONSTRAINT HONOURED.** No `cargo` invocation of any kind. Everything
below is `/usr/bin/grep`, `git grep`, `git show`, `git log`, `git diff`, `awk`,
`sed`, `cut`, `sort`, `sha256sum`, `python3` over text, and reading files. I
modified nothing but this report.

**WHAT I READ.** `CLAUDE.md`; `docs/process.md` (all of it);
`docs/decisions.md` D-570 through **D-581** (full text of D-576 and D-581);
`docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md` (all 754 lines);
`docs/experiments/matrix_label_cache_key.md` revision 2;
`docs/experiments/matrix_label_cache_key_REDTEAM.md` (finding list + FATAL 1,
MAJOR 5, MAJOR 6, MAJOR 7 in full); `docs/experiments/wp21_prereg.md` revision 4
(§3, §6.1, §8); `docs/experiments/arc3_ledger.md`; `tools/ci.sh`;
`tools/determinism.sh` **in full**; `tools/label_cache_count.py` in full;
`tools/cold_label_check.py`;
`crates/pistol-arena/src/{capture.rs,exchange.rs,labels.rs,labels_file.rs,usage.rs,bin/arena.rs}`;
`crates/pistol-search/src/{heuristics.rs,params.rs,pvs.rs,search.rs}`;
`crates/pistol-core/src/{turn.rs,symmetry.rs}`;
`crates/pistol-cli/src/random_openings/mod.rs`;
`crates/pistol-cli/tests/fixtures/random_openings_v2.txt`;
`configs/{instrument_v0.toml,arena_wp20_label_pilot.toml,arena_wp20_label_pilot_dryrun.toml}`;
`artifacts/arc3_leverB_41_count_v3.txt`;
`/home/tom/pistol-runs/wp20pilot-artifacts/{capture_v1.txt,corpus_v1.txt}`.

## VERDICT: **FAIL**

Revision 3 is a large improvement: **17 of round 1's 22 findings are genuinely
CLOSED, and I verified every one of them against the code rather than against the
sentence that claims them.** §4.1's whole count block re-derives exactly under
code I wrote myself, the play-order trace verifies limb by limb, and the key
selection survives everything I threw at it.

It fails on three BLOCKING findings and eight MAJOR, and the shape of the failure
is the arc's own named defect one more time — **a claim asserted at a convenient
scope rather than derived at the true one**:

1. **§1 and §5 are computed on `wp21_prereg.md`'s SUPERSEDED revision-3
   arithmetic** — 12 870 s / 85.6% / replay 3% / 7.15 h — every one of which
   revision 4 of the cited document explicitly repudiates **in the same commit**,
   while this document's own ONE LINE quotes revision 4's 7.74 h and promises not
   to restate it.
2. **§2's a-priori argument is FALSE**, and I refuted it with a computation over
   the sweep's own committed opening range that takes about two seconds. At
   `k = 2` the symmetry fold merges **3 119 of the sweep's 3 487** openings'
   prefixes, and **792 within tranche scope** — so the flip clause revision 3
   registers has its first conjunct answered YES before the run.
3. **§7 claims a dry run that §7.1 does not record**, and §7.1's limb 1 is false
   of one of §7's own commands.

Plus: §2's seat list is still the four the matrix's own red team called out as
wrong (D-581 at HEAD says five); §2 records a "strongest attack" the arc has
superseded; §4.1.1's second instrument is in bijection-by-construction with the
first; and **both** of the document's sha-anchoring claims are false — neither
the receipt's sha256 nor the instrument's appears in any tracked file.

---

# DISPOSITION OF ROUND 1

| # | round-1 finding | status | evidence |
|---|---|---|---|
| **B1** | D-576 does not exist | **CLOSED** | `/usr/bin/grep -c "^D-576" docs/decisions.md` → **1**, at `docs/decisions.md:1220`. I read D-576 in full; it contains the literal string *"THE CACHE MAY NOT BE REPAIRED INTO PASSING"*, so §4.5's *"D-576 says so in its own words"* is now a real quotation. |
| **B2** | registered command refused; no dry run | **PARTIAL** | Word order fixed: §3.2 and §7 both spell `--capture <src> --out <out> --label-nodes <n>`, matching `crates/pistol-arena/src/bin/arena.rs:51` exactly. **The dry run is still not taken or recorded** — see **BLOCKING 3**. |
| **M1** | gate 6 → gate 9; coverage overstated | **PARTIAL** | Gate number correct: `tools/ci.sh:104-105` is `step "gate 9/19: cross-process determinism"`. Coverage caveat added (§2, *"AND THE GATE DOES NOT RUN THIS SWEEP'S SEAT OR THIS SWEEP'S BUDGET"*). **But the seat list is four and the array has five** — see **MAJOR 1**. |
| **M2** | §1/§2 contradict on cross-game transpositions | **CLOSED** | §1 now reads *"REVISION 2 SAID THE EXCESS OVER 2.0 WAS 'the cross-game transpositions on top'. IT IS NOT, AND §4.1's COUNT SAYS WHAT IT IS"* and points at §4.1 as owner. The refuted sentence is gone. |
| **M3** | receipt lacks the quoted rows | **CLOSED** | `artifacts/arc3_leverB_41_count_v3.txt` carries **every** row §4.1 quotes, under the cache's own key, plus two more. I re-derived all of them independently (see WHAT SURVIVED, S1). One residue: **minor 1**. |
| **M4** | key is 1 of 4 args; invariant unregistered | **CLOSED** | §2.0 registers it as a named obligation. Verified: `capture.rs:333` computes `label_go_line` once, `:340` is the game loop, `:341` the prefix loop — all outside it. |
| **M5** | no OFF switch registered | **CLOSED** | §4.2 registers `--label-cache`, optional trailing word, default OFF, following `--census`'s precedent at `arena.rs:59-72` / `usage.rs:14`. |
| **M6** | "no cached tranche before byte-identity" has no mechanism | **CLOSED** | §4.4 bullet 2 now gives both: the default-off flag as mechanism, and a run-log obligation (`cmp -s` exit line + timestamp above the first `--label-cache` tranche block, verbatim flag in every tranche command, closure names who read it, void rule for a violation). |
| **M7** | C3 unevaluable from any output | **PARTIAL** | Restated over the MEAN, which the instrument does yield, and the 120 s watchdog citation is right (`arena_wp20_label_pilot.toml:39`). **But the 12 s threshold cannot be crossed by any N in the field** — see **MAJOR 7**. |
| **M8** | disagreement pre-attributed to the ENGINE | **CLOSED** | §4.4 now registers separation-before-naming (two uncached runs over one report), names the four cache-side candidate causes, and cites the pilot's `capture-determinism exit=0`. This is a full inversion of the defect. |
| **M9** | applicable saving stated as prose | **PARTIAL** | The 1.63 h is now computed. **But it is computed on the superseded baseline (BLOCKING 1), and §5 withdraws the "later sweeps" appeal and then re-makes it verbatim in the next paragraph, double-discounting (MAJOR 5).** |
| **M10(a)** | replication, not a second instrument | **PARTIAL** | §4.1.1 registers one. **It is not independent of the compared stage, and it was registered after the first instrument had already run** — see **MAJOR 3**. |
| **M10(b)** | instrument has no governing revision | **CLOSED** | §8 names `tools/label_cache_count.py`; the tree's sha256 is `1a890b53…`, matching the receipt. Residue at **minor 7** (the "revision" is prose, not a SHA). |
| **M10(c)** | receipt not durable, not anchored | **OPEN** | Both claimed anchorings are false — see **MAJOR 4**. |
| **M11** | key selected with no OPTION MATRIX / red team | **CLOSED** | `docs/experiments/matrix_label_cache_key.md` rev 2 exists, ranks five keys with MEASURED/ESTIMATED marks, was attacked by `matrix_label_cache_key_REDTEAM.md`, and the surviving attack is recorded in D-576/D-581. §2.1 defers to it. |
| **M12** | "~171 s" units error over a wrong count | **CLOSED** | `/usr/bin/grep -v '^#' corpus_v1.txt \| awk -F'\t' '$1<6' \| wc -l` → **152**. 152 × 0.8854 = 134.6 s ≈ ~135 s ✓; 5 × 3 × 135 = 2 025 s ✓. Residue at **minor 4**. |
| **m1** | C1/C2 named the wrong artifact | **CLOSED** | §3.5 says *"capture file — pass 2's output, five TAB fields, not a corpus"* and says so again in the criterion row. |
| **m2** | "no state crosses an ask" literally false | **CLOSED** | §2 now states the exception by name and cites it. Verified: `instance.rs:112-119` carries the comment *"`new_game`'s `clear` does not touch it"*; `search.rs:83` is `census_folds`. |
| **m3** | BTreeMap rationale misread rule 4 | **CLOSED** | §4.2 now gives the cost reason and concedes rule 4 admits a fixed-seed hasher. |
| **m4** | canonicalisation attributed to `position_line` | **CLOSED** | §2 now attributes it to `turn.rs:90` (*"`first < second`"*) and `turn.rs:213-215` (*"An uncanonical pair is refused rather than reordered"*) — both verified verbatim — and notes `exchange.rs:154-161` merely calls `turn.to_string()`, also verified. |
| **m5** | machine-seconds algebra not exact | **CLOSED** | §3.3 states it is an upper bound, says where it is worst, and says it cancels. |
| **m6** | `search_nodes` no longer sums to machine work | **CLOSED** | §4.2 registers it, with the reason a later consumer would not see it. |
| **m7** | play pass has no registered instrument | **PARTIAL** | §3.1 names a config, but the named config is not the one the study runs — see **MAJOR 6**. |
| **m8** | D-423: the 742/347/2.1383 block repeated | **PARTIAL** | §4.1 is declared owner and §1/§2 point there. **The same defect has moved to a different number: the WALL is now stated in three mutually inconsistent places** — **BLOCKING 1**. |

**Tally: 17 CLOSED, 6 PARTIAL, 1 OPEN.**

---

# BLOCKING

## BLOCKING 1 — **§1 and §5 are computed on `wp21_prereg.md`'s SUPERSEDED revision-3 arithmetic, which revision 4 explicitly repudiates in the same commit; the document restates a wall its own ONE LINE promises not to restate; and the corrected tranche is transcribed wrong.**

Three quotes from the reviewed file.

§1, line 74:

> Per tranche, from `wp21_prereg.md` §3's own arithmetic: capture
> **11 017 s of 12 870 s — 85.6%**. Play is 11%, replay 3%, the cold check 0.4%.

§5's saving block:

> ```
>   wave 1, uncached   12 869 s          (registered; 13 928 s under wp21_prereg
>                                         revision 4's replay correction)
>   wave 2, cached      7 004 s
>   realised           19 873 s = 5.52 h   against 25 738 s = 7.15 h uncached
> ```

And the document's own ONE LINE:

> The sweep's registered wall is **7.74 h** … `wp21_prereg.md` §3 owns that number
> **and this document does not restate it**.

**What the cited section actually says**, at the same commit
(`/usr/bin/sed -n '225,290p' docs/experiments/wp21_prereg.md`, revision 4):

```
ESTIMATED  tranche                        13 925 s = 3.87 h
UNCACHED, N = 8, sixteen tranches      2 waves x 13 925 s = 27 850 s = 7.74 h
CACHED from wave two, N = 8            13 925 + 8 060     = 21 985 s = 6.11 h
```

and, in its own words:

> **THE `x4` ON THE REPLAY LINE IS REVISION 4's CORRECTION AND IT MOVES THE
> WALL.** … **A tranche is 3.87 h, not 3.57**, and the consequence reaches the
> sibling study: capture is **79.1%** of a tranche rather than 85.6%, and replay
> is **10.1%** rather than 3%.

and, at `wp21_prereg.md:516-517`:

> (D-423; revision 3 quoted `7.15 h` here, a figure this document did not carry
> and which was wrong in any case under the replay correction)

So, itemised:

- **`12 870` / `12 869` / `13 928` are three different values for one quantity in
  one document**, and the registered value is **`13 925`**. The parenthetical that
  acknowledges the correction transcribes it wrong by 3 s.
- **`85.6%` and `replay 3%` are the figures revision 4 names as corrected-away.**
  `11 017 / 13 925 = 79.1%`, and replay is `1 409 / 13 925 = 10.1%`. §1 attributes
  both to *"`wp21_prereg.md` §3's own arithmetic"*, a section that says the
  opposite three paragraphs down.
- **`7.15 h` is the figure `wp21_prereg.md` §6.1 explicitly records as revision
  3's error.** Revision 3 of *this* document re-commits it.
- **§5's `wave 2, cached 7 004 s` is not the registered figure**: the registered
  cached tranche is `13 925 − 11 017 + 5 152 = 8 060 s`, and the registered
  realised wall is `21 985 s = 6.11 h`, not `19 873 s = 5.52 h`.
- The ONE LINE **restates the wall in two other sections after promising it does
  not**, which is D-423 (*"a claim the document makes twice is a defect
  waiting"*) materialising for the second consecutive round — round 1's M2 and m8
  were the same defect on the duplication ratio.

**What survives.** I re-derived the saving under revision 4's numbers and it is
**unchanged**, because the replay correction is additive to both waves:
`27 850 − 21 985 = 5 865 s = 1.63 h`, exactly §5's figure. **So the "roughly a
wash / net negative" conclusion holds.** That is why this is blocking on the
document and not on the lever.

**What does not survive.** §1's lever selection is argued from the stale share:

> **A lever that is not on the capture pass cannot matter**, which is what rules
> out the obvious ones: `n_workers` on pass 1, **a faster replay**, a wider cold
> stride.

At replay = 10.1% and 1 409 s per tranche, replay is over three times the share
§1 dismisses it at. The exclusion may still be right (`wp21_prereg` §3 records
that T-B *mandates* `--workers 1`, so the lever is closed by criterion rather than
by size) — but the document must argue it from the criterion, not from a 3% that
the governing registration retired.

**Fix.** Delete every wall and share figure from §1 and §5 and point at
`wp21_prereg.md` §3, which the ONE LINE already declares the owner. Where §5 must
compute the saving, compute it from `13 925 / 27 850 / 21 985` and print
`6.11 h against 7.74 h, ESTIMATED SAVING 1.63 h`. Re-argue §1's replay exclusion
from T-B's serial mandate rather than from a share.

## BLOCKING 2 — **§2's a-priori argument is FALSE, and it is false in exactly the direction that pre-answers the flip clause revision 3 registers.**

§2, the paragraph that carries the whole a-priori half of the red team's FATAL
remedy:

> **What IS a-priori**: the book dedupes openings by `canonical_form`
> (`crates/pistol-cli/src/random_openings/mod.rs:174`), so **no two openings can
> transpose or mirror onto each other at `k <= opening_turns`**.

(`matrix_label_cache_key.md` §2.1 states the same thing as *"`k <= 3` IS CLOSED BY
CONSTRUCTION AND NEEDED NO MEASUREMENT … a result, not an observation"*, and
deducts 156 of the pilot's 1 576 trials on that ground.)

**The dedupe is applied to the FULL opening only.** `random_openings/mod.rs:172-177`:

```
let plies = draw(&mut rng, &cells, k_stones)?;
let state = place_all(&plies)?;
if !seen.insert(canonical_form(&state.played().collect::<Vec<_>>())) {
```

`place_all` replays the **whole** draw before `canonical_form` sees it. Distinct
canonical forms at `k = opening_turns` imply nothing whatever about `k < opening_turns`
— two openings sharing their first two turns and diverging at the third are
perfectly legal draws, and the generator never looks at a prefix.

**Refuted by measurement over the sweep's own range.** Scope: the committed book
`crates/pistol-cli/tests/fixtures/random_openings_v2.txt`, openings **13..3499** —
the exact range `wp21_prereg.md` registers — with the symmetry group read out of
`crates/pistol-core/src/symmetry.rs` (`rotate` = `(q,r) -> (-r, q+r)` at `:138-142`;
reflection = `Coord::new(cell.r, cell.q)` at `:90-92`; twelve images, no
translation), which is the same group `tools/label_cache_count.py:138-147` uses.

```
$ python3 - <<'EOF'   # over crates/pistol-cli/tests/fixtures/random_openings_v2.txt
   ... k-turn prefixes of openings[13:3500], folded under canonical_form's group ...
EOF
sweep 13..3499   k=1: n=3487 exact_distinct=1    stoneset_distinct=1    folded_distinct=1
sweep 13..3499   k=2: n=3487 exact_distinct=2327 stoneset_distinct=2327 folded_distinct=368
sweep 13..3499   k=3: n=3487 exact_distinct=3487 stoneset_distinct=3487 folded_distinct=3487
```

- At **`k = opening_turns = 3`** the claim is **true**: 3 487 of 3 487 distinct
  under every key. That is the dedupe doing its job.
- At **`k = 2`** the claim is **false**: **1 160 of 3 487 openings share a `k = 2`
  prefix byte-identically with an earlier one**, and under the symmetry fold
  **3 119 of 3 487 — 89.4% — collapse onto one of 368 classes.**
- At `k <= 1` it is trivially false, and §4.1's own *"two `position` line(s) asked
  26 time(s)"* — three paragraphs later in the same document — is the pilot's
  measurement of it.

**And it is not an artefact of sweep-wide scope.** The cache lives in one
`capture::run`, i.e. one tranche. Scoped to contiguous 218-opening tranches:

```
tranche  0: n=218 exact_distinct=213 folded_distinct=171  extra symmetry merges 42
tranche  1: n=218 exact_distinct=209 folded_distinct=158  extra symmetry merges 51
...
tranche 15: n=217 exact_distinct=210 folded_distinct=164  extra symmetry merges 46
TOTAL n=3487 sum_exact=3403 sum_folded=2611     extra symmetry merges = 792
```

**42 to 60 extra merges per tranche, 792 across the sweep, from `k = 2` alone** —
before anything at `k > opening_turns` is counted.

**Why this is blocking rather than a correction.**

1. A registration may not carry a false statement of fact, and this one is
   marked *"a result, not an observation"* and is used to **deduct 156 trials
   from the rule-of-three denominator** in the matrix. The deducted class is
   precisely the class that folds at scale.
2. §2 registers a new flip clause: *"the cache counts, per tranche, how many of
   its own MISSES share a `key_pos` or a `key_full` with an earlier miss … it is
   the matrix's registered flip clause"*, and D-581 makes the selection flip
   conditional on that count being *"materially above zero"*. **The `key_full`
   half of that count is now known in advance to be ~48 per tranche**, derivable
   in seconds from a committed fixture. `docs/process.md`'s closing sentence names
   exactly this: *"neither catches a run whose answer is already known before it
   is taken — that defect is judged, not checked."* And CLAUDE.md's D-291: *"an
   estimate that could have been measured in seconds is a finding."*
3. It sharpens the honest reading of the selection rather than reversing it:
   `key_pos` merges **nothing** extra at `k = 2` (2 327 = 2 327 — turn 1 is
   forced to the origin, so there is only one turn to reorder), so the whole
   `k = 2` yield belongs to `key_full`, the one key that returns a `bestmove` in
   the wrong frame. **The selection is safer than the document argues, for a
   reason the document does not have.**

**Fix.** Replace `k <= opening_turns` with `k = opening_turns` in §2 and in
`matrix_label_cache_key.md` §2.1; restore the 156 deducted trials; state the
measured `k = 2` fold (792 tranche-scoped merges, `key_full` only) as a MEASURED
row; and re-word the flip clause so its `key_full` limb is a confirmation of a
known number rather than a discovery — or drop the `key_full` counter and keep
the `key_pos` one, which is the limb still genuinely unknown.

## BLOCKING 3 — **§7 claims a dry run that §7.1 does not record, and §7.1's limb 1 is false of one of §7's own commands.**

§7's preamble:

> The commands are here, in full, and **§7.1 records the run that took them.**

§7.1's limb 1:

> 1. **every command above is accepted by the shipped binary and exits 0** — which
>    is the limb revision 2 failed without knowing;

**Neither sentence is true.**

**(a) §7.1 records no output at all.** It records what the dry run *exercises*
(input kind, four limbs, what a failure means) and not one byte of what it
*produced*: no exit status, no transcript, no receipt path, no sha256, no date.
`docs/process.md`, **Dry-run discipline**, binding *"exactly as CLAUDE.md text
would"*:

> **The pre-registration records the dry-run input and its output.**

§8's instrument table lists five entries and no dry-run receipt.

**(b) One of §7's commands cannot be accepted by the shipped binary.** §7 prints:

```
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/cached.txt   --label-nodes 400000 \
      --label-cache
```

Scope: every argument arm in the shipped binary.

```
$ /usr/bin/awk 'NR>=39&&NR<=85' crates/pistol-arena/src/bin/arena.rs
   ["--capture", source, "--out", out, "--label-nodes", nodes]            => ...   (:51)
   ["--capture", source, "--out", out, "--label-nodes", nodes, "--census"] => ...   (:59-67)
   _ => Err("... --capture, --out and --label-nodes, ... each in that order")       (:79-85)

$ git grep -n "label-cache\|label_cache" -- crates configs
crates/pistol-arena/tests/label_cache_count_tests.rs:17,142   (the PYTHON tool's path)
tools/label_cache_count.py:24,47,200,202                      (the PYTHON tool's own prefix)
```

There is no `--label-cache` anywhere in production code. The catch-all at `:79-85`
refuses it. §7.1's own last paragraph concedes it — *"Limbs 2 and 3 cannot be
taken until the cache exists"* — which contradicts limb 1's *"every command
above"* two paragraphs earlier.

**(c) The document says so itself and stops short of the consequence.** §7.1
closes: *"THE REVIEW THAT GOVERNS THE RUN IS TAKEN AFTER BOTH PARTS ARE
RECORDED."* CLAUDE.md's Process section: *"A pre-registration is reviewed at the
revision that GOVERNS the run — that revision must itself pass a fresh-context
review before the first run it governs."* **By its own sentence, revision 3 is not
that revision.** A registration that declares its own review non-governing cannot
pass that review; it can only be told what it must add.

**Fix.** Take limb 1 now — it costs one `arena --config … --out …` on
`configs/arena_wp20_label_pilot_dryrun.toml` and two `arena --capture` runs at
`--label-nodes 2000` — and paste the input path, the commands verbatim, and the
exit lines into §7.1. Restate limb 1 as *"every command that does not name
`--label-cache`"*. Move the `--label-cache` commands into a §7.2 explicitly
marked as registered-against-code-that-does-not-exist, and say plainly that the
governing review is the one taken after the cache package lands.

---

# MAJOR

## MAJOR 1 — **§2's `SEATS` list is four; the array has five; and D-581, landed at HEAD to correct exactly this, says five.**

§2:

> Its `SEATS` array is radius / staged / staged-with-heuristics /
> staged-with-solver and its budgets are `depth_turns 4` and `nodes 200000`

Scope: the array itself, `tools/determinism.sh:66-81`.

```
SEATS=(
	"radius configs/gate_v0.toml ..."
	"staged configs/gate_staged_v0.toml ..."
	"staged-heuristics configs/gate_staged_heuristics_v0.toml ..."
	"staged-solver configs/gate_staged_solver_v0.toml ... depth_turns-2 nodes-10000"
	"staged-safety-net-cap configs/gate_staged_snk_v0.toml ..."
)
```

**Five.** The fifth is `staged-safety-net-cap` (D-478, D-482). The four-name list
is the **stale enumeration in the script's own header comment**
(`tools/determinism.sh:20-24`), which sits in the paragraph that reads:

> THE SEATS ARE THE `SEATS` ARRAY BELOW AND ARE NOT COUNTED IN PROSE — a count in
> a comment is a second place for a fact the array already states, and **it went
> stale the first time a seat was added.**

This is not a new discovery: `matrix_label_cache_key_REDTEAM.md` **MAJOR 6** found
it, `matrix_label_cache_key.md` revision 2 fixed it (`:171-172`, *"five seats …
revision 1 said four, transcribed from the script's own stale header comment"*),
and **D-581, landed at `114b562`, says *"it has **five** seats, not the four an
earlier draft transcribed"***. **The fix reached the matrix and the ADR and did
not reach this document** — the one the matrix §4 itself calls *"the governing
registration"*.

Secondary: *"its budgets are `depth_turns 4` and `nodes 200000`"* is also
incomplete. `tools/determinism.sh:78` gives the solver seat its own overrides
(`depth_turns-2 nodes-10000`), and `SOLVER_LAYOUT_BUDGET="nodes 10000"` replaces
`LAYOUT_BUDGET` for its C-vs-D leg (`:236-239`). The conclusion (*"not
`nodes 400000`"*) is unaffected; the enumeration is wrong.

**Fix.** Cite the array, not the comment: five seats, named, and note the solver
seat's own budgets. And fix `tools/determinism.sh:20-24`, which is a live stale
fact in a CI gate's own documentation and is now the second document it has
misled.

## MAJOR 2 — **§2 records as "the strongest attack" one the arc has superseded, and does not carry the one D-581 and the matrix now record.**

§2:

> **AND THE STRONGEST ATTACK ON THE KEY WAS ATTEMPTED AND FAILED**, which is worth
> more than the argument above. … **It found none.**

That is round 1's A1 (the within-process-channel hunt), and it did fail — I
re-checked and could not break it either. But the arc has since recorded a
**different** strongest surviving attack, in two places, neither of which this
document mentions:

- `matrix_label_cache_key_REDTEAM.md` **MAJOR 7**: *"THE RECORDED 'STRONGEST
  ATTACK' IS NOT THE STRONGEST; A SHARPER ONE ABOUT GATE 9 IS AVAILABLE AND
  UNRECORDED."*
- `matrix_label_cache_key.md:164-181` and **D-581**: *"no limb of gate 9 ever asks
  the same position twice inside one process … **THAT IS THE STRONGEST SURVIVING
  ATTACK AND IT IS RECORDED AS A COST**."*

CLAUDE.md, Process: *"the surviving option's ADR line records the strongest
surviving attack."* The ADR does. The registration that governs the run records a
different one and calls it the strongest. A successor reading §2 will believe the
key's residual risk is a hunt that came back empty, when the arc's own position is
that the residual risk is a gate-coverage gap closed only by §4.4.

**Fix.** Replace §2's "strongest attack" paragraph with D-581's — corrected per
MAJOR 8 — keep the failed channel hunt as the supporting evidence it is, and cite
D-581 by number. The document currently cites D-576 five times and D-581 not at
all, though D-581 corrects two of D-576's supporting claims.

## MAJOR 3 — **§4.1.1's second instrument is in bijection-by-construction with the first, checks the one column that has no derivation, and was registered after the first instrument had already run.**

§4.1.1:

> **THE STAGE UNDER DOUBT** is **the derivation of a key from a record** …
> **THE SECOND INSTRUMENT DOES NOT SHARE THAT STAGE.** It derives its key from
> the **corpus** file's `moves` column rather than from the capture's `position`
> line …
> ```
> /usr/bin/grep -v '^#' <corpus_v1.txt> | cut -f3 | LC_ALL=C sort -u | wc -l
> ```
> **THE AGREEMENT CRITERION, REGISTERED HERE**: the two instruments must return
> the **same distinct count** and the same asked-prefix count.

I ran it. Scope: the pilot corpus the receipt sha-pins.

```
$ /usr/bin/grep -v '^#' /home/tom/pistol-runs/wp20pilot-artifacts/corpus_v1.txt \
    | cut -f3 | LC_ALL=C sort -u | wc -l
347
```

**It agrees.** Four problems with what that agreement is worth.

**(a) The agreement is guaranteed by a refusal in the code that writes the
corpus.** `crates/pistol-arena/src/labels.rs:124-148`:

```
let prefix = &game.moves[..record.turns_played];
...
let sent = if prefix.is_empty() { format!("{} start", POSITION) }
           else { exchange::position_line(prefix) };
if sent != record.position {
    return Err(refuse(format!("game {} turn {}: the captured position is not \
                              the report's own prefix", ...)));
}
```

and `:200`: `moves: render_turns(prefix)`. So the corpus's `moves` column and the
capture's `position` field are two renderings of the **same** `&game.moves[..k]`
slice, and `arena --labels` **refuses to write the corpus at all** unless they
match. Both renderings are joins of `Turn::to_string()`
(`exchange.rs:154-161`, `labels.rs:93-99`), which is injective on turn slices. The
two counts were in bijection before either instrument ran. `docs/process.md`: *"two
instruments blind to the same stage are one instrument reported twice."*

**(b) It checks the only column with no derivation, and leaves the derivation
unchecked.** For the exact key, `tools/label_cache_count.py:167` is `exact.add(position)`
— a raw string added to a set. There is no derivation to doubt. The stage §4.1.1
names *is* real, but it lives in `stones_of` (`:101-121`), `axial` (`:124-132`)
and `images` (`:138-147`) — 47 lines of parsing, player-alternation and hex
symmetry — which produce the *sorted-stone* and *symmetry-folded* columns. **Those
are the two columns §2, §2.1 and D-576 lean on for "the fold merges nothing", and
the registered second instrument touches neither.**

**(c) The registered criterion asks for a number the registered instrument does
not produce.** *"the same distinct count **and the same asked-prefix count**"* —
`sort -u | wc -l` returns one number. Half the criterion cannot be evaluated by
the instrument it is registered over.

**(d) It was not "registered before either runs".** The section's own heading is
*"REGISTERED BEFORE EITHER RUNS"*. The first instrument's receipt is
`artifacts/arc3_leverB_41_count_v3.txt`, `date: 2026-09-02T18:13:44+02:00`; the
commit carrying §4.1.1 is `709ad32`, `2026-09-02 18:58:43 +0200` — **45 minutes
later**. `docs/process.md` requires *"a SECOND INSTRUMENT whose agreement
criterion is registered before either runs"*, and revision 2 registered no second
instrument (round 1's M10(a) is that it registered a re-take). The second
instrument was chosen after the first instrument's number was known.

**(e) And it has still never been run and recorded.** The v3 receipt's command
block records `python3 tools/label_cache_count.py …` and, as its second command,
`… | awk -F'\t' '$1<6' | wc -l` — the **lever-A record count**, not §4.1.1's
pipeline. Round 1's M10 fix said *"it should be registered, not inherited from a
reviewer"*; a reviewer has now run it for the second consecutive round.

**Fix.** Register a second instrument that reaches the stage under doubt: derive
the sorted-stone and symmetry-folded counts a second way (the corpus already
carries a `key_full` column — `labels.rs:228` region — computed inside
`pistol-core`, which is a genuinely different implementation of the same fold),
and register agreement on **all three** columns. Drop the asked-prefix limb or
add `| wc -l` to the pipeline. Take the run and put its output in the receipt.

## MAJOR 4 — **Both of the document's sha-anchoring claims are false, and the instrument's sha is "anchored" inside a gitignored file.**

§4.1.1:

> **AND THE RECEIPT IS SHA-ANCHORED**, because `artifacts/` is gitignored (hard
> rule 8 permits it only if *"a committed manifest may sha-index them"*): **the
> receipt's own sha256 is recorded in this document's §8** alongside the
> instrument's.

§8, the row it points at:

> | the receipts | `artifacts/arc3_leverB_41_count_v3.txt` | **sha-anchored in
> `docs/experiments/arc3_ledger.md` §1**, because `artifacts/` is gitignored
> (hard rule 8) |

§4.1.1 says the sha is in §8. §8 says it is in the ledger. Scope: the whole
tracked tree.

```
$ sha256sum artifacts/arc3_leverB_41_count_v3.txt
cbad0786505e8d7958610a2ff24d8b4186de29fd85b0127d60df7b04b4e342ed  artifacts/arc3_leverB_41_count_v3.txt
$ git grep -n "cbad0786"
(no output)
$ git grep -n "arc3_leverB_41_count_v3" docs/experiments/arc3_ledger.md
(no output)
$ /usr/bin/grep -n "arc3_leverB_41_count" docs/experiments/arc3_ledger.md
68:| lever B §4.1's count, taken UNGOVERNED | done, and **re-take owed** | `artifacts/arc3_leverB_41_count.txt` |
250:The sweep registration's review found it: `artifacts/arc3_leverB_41_count.txt`
282:not. Receipt: `artifacts/arc3_leverB_41_count_v2.txt`, which SUPERSEDES `_v1`.
```

**The `_v3` receipt is not named in the ledger, and its sha256 appears in no
tracked file.** The ledger anchors `_v1` and `_v2`, both of which §4.1 declares
superseded.

Second limb. §8's instrument row:

> | `tools/label_cache_count.py` | … | committed at the arc III §1 head; **sha256
> in `artifacts/arc3_leverB_41_count_v3.txt`** |

```
$ sha256sum tools/label_cache_count.py
1a890b5331c302cf97372603e7ec21a41b08e6131390d92b78b0168bc77c1e18  tools/label_cache_count.py
$ git grep -n "1a890b53"
(no output)
$ git check-ignore -v artifacts/arc3_leverB_41_count_v3.txt
.gitignore:19:artifacts/	artifacts/arc3_leverB_41_count_v3.txt
```

The instrument's digest is anchored **inside the gitignored receipt it is meant to
anchor**. Hard rule 8 permits a gitignored artifact only where *"a committed
manifest may sha-index them"* — a manifest that is itself gitignored indexes
nothing, and the script's own committed bytes are the only thing keeping this
honest today.

This is round 1's M10(c) verbatim, one revision later, now asserted twice.

**Fix.** Put both digests — `cbad0786…` for the receipt and `1a890b53…` for the
instrument — in §8's own table, in the committed document, and delete the two
pointers. §8 is a committed manifest; use it as one.

## MAJOR 5 — **§5 withdraws an appeal and then makes it again verbatim in the next paragraph, and the second paragraph double-discounts a saving the first has already discounted.**

§5, consecutive paragraphs:

> Revision 2 answered it with *"the cache's value is mostly in the sweeps after
> this one"*, and **there is no sweep after this one to point at** … **That appeal
> is withdrawn.**

> **AND THE HONEST FIGURE FOR THIS SWEEP IS SMALLER THAN THAT** … At the
> registered N = 8 that is half the sweep, and the ESTIMATED saving falls to a
> fraction of a wave. **THE CACHE'S VALUE IS MOSTLY IN THE SWEEPS AFTER THIS
> ONE**, where §4.4 has already returned and every tranche runs cached …

The withdrawn appeal is re-made in the paragraph immediately after the withdrawal,
in the same capitals, and the closing sentence *"That is the trade, stated before
the run"* appears in both. The second paragraph is revision 2 text that survived
the edit that repudiated it.

**And it is arithmetically wrong on top.** *"§4.4 forbids a cached tranche before
the comparison returns, so tranche one and everything running beside it in wave
one are uncached … the ESTIMATED saving falls to a fraction of a wave"* — but the
1.63 h computed three paragraphs above **is already the wave-one-uncached figure**
(`wave 1, uncached` + `wave 2, cached`). §5 discounts for wave one twice, and the
second discount reduces a number that already carries it. `wp21_prereg.md` §3
records the same 1.63 h as `CACHED from wave two`, confirming which it is.

This is round 1's M9 half-remedied: the number was computed, and the prose M9
asked to be dropped was left standing next to it.

**Fix.** Delete the second paragraph entirely. Everything true in it is already in
the first, correctly.

## MAJOR 6 — **§3.1's play-pass instrument is an uncommitted mutation of a committed config, is absent from §8, and carries a pinned `binary_sha256` the document never registers.**

§3.1:

> **`configs/arena_wp20_label_pilot.toml` as committed, with `openings_skip = 0`
> and `openings_take = 3`**, which is the pilot's own seat … **and the only change
> is the take.**

```
$ /usr/bin/grep -n "openings_take\|openings_skip\|hang_timeout_ms\|binary_sha256" \
      configs/arena_wp20_label_pilot.toml
27:openings_take = 13
28:openings_skip = 0
45:hang_timeout_ms = 120000
68:binary_sha256 = "180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476"
74:binary_sha256 = "180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476"
```

*"As committed"* and *"with `openings_take = 3`"* cannot both hold: the committed
take is **13**. §7's command names the instrument as *"`<the pilot config with
openings_take = 3>`"* — a file with no path, no name and no digest, which is
precisely the case `docs/process.md`'s **Instrument governing revision** rule
covers (*"a `tools/` script, **a scratchpad harness**, or a command block the
document prints … living there is not what makes the rule apply"*). §8's table
lists five instruments and not this one, though it is the instrument that produces
the report every number in §3 is measured over, and though §3.4's 120 s guard
reads `hang_timeout_ms` off it.

Two consequences the document does not state:

- **The binary is pinned to the pilot's**, `180b4c40…`, on both seats. Either the
  current `target/release/pistol` digests to that (in which case §5's *"152
  records MEASURED"* transfers, and the document should say why) or `arena` will
  refuse the play pass outright (in which case the study cannot start). Neither
  branch is registered.
- §8's `arena` row reads *"the sweep's own binary digest, `wp21_prereg.md` §8"*,
  and `wp21_prereg.md` §8's first line is **"THE SLOT IS EMPTY UNTIL IT IS
  FILLED, AND THIS DOCUMENT NO LONGER CLAIMS OTHERWISE."** §8's only binary
  citation points at an admittedly empty slot — while C4, the criterion the
  document calls *"the one that keeps the rest honest"*, compares this study's
  realised rate against a rate measured by a different binary.

**Fix.** Commit the take-3 variant as `configs/arena_wp21_throughput_playpass.toml`
(the tree already carries `arena_wp20_label_pilot_dryrun.toml` at take 2 as the
precedent), name it in §3.1, §7 and §8, and state in §8 which binary digest this
study runs at and how it relates to the pilot's `180b4c40…` that C4's referent was
measured on.

## MAJOR 7 — **C3's 12 s threshold is 13.6x the measured baseline; no N in the registered field can cross it, so the criterion still cannot fire.**

§3.5:

> | **C3** | the realised MEAN seconds-per-label stays below **12 s** at the
> selected N, a ten-fold margin against the 120 s watchdog | … | **yes** |

Round 1's M7 asked for C3 to be restated over a quantity the instrument yields.
That half is done, and done well — §3.4's derivation of *why* nothing measures a
single label (`capture::normalise` at `capture.rs:66-96` strips ` nps <n> time <n>`;
`/usr/bin/grep -c " time " capture_v1.txt` → 0) is correct and I re-verified both.
The other half of M7 is untouched: *"A criterion whose registered consequence can
never fire on the evidence the run produces is not a criterion."*

The baseline is the document's own MEASURED **0.8854 s/label**. For the realised
mean to reach 12 s the contention factor `c(N)` must reach **13.6**. The field is
`{1, 2, 4, 8, 16}` on an 8-core / 16-thread box (round 1's S8 confirmed the
`lscpu`). Sixteen processes on sixteen threads cannot produce a 13.6x per-process
slowdown; a plausible worst case at N = 16 is 2–4. **C3 cannot fail, and C4 —
which fires at 1.06 s/label, a 1.2x factor — would catch a pathological box
twelve times sooner.**

`docs/process.md`, **Criterion and defect class**: *"A criterion that is a property
the named defect class PRESERVES … passes vacuously and is not a criterion; it
must be one that defect could falsify."* C3 as registered is a design element
wearing a criterion's label — the exact confusion §3.5's own preamble says it
exists to prevent (*"Listing the two in one column is how a document comes to
report four criteria met when two of them were never at risk"*).

**Fix.** Either register C3 at a threshold a measurable contention factor can
cross — the natural one is stated in §3.4's own SMT note, *"a per-tranche time at
16 that is more than 2x the time at 8"*, i.e. `c(N) <= 4`, giving ~3.5 s — and
keep the 120 s watchdog as the backstop it already is; or move C3 into the DESIGN
ELEMENTS column and say the watchdog is what excludes that defect class.

## MAJOR 8 — **D-581's own load-bearing sentence, which revision 3 is about to import, is too strong: gate 9's A/B limb asks every position twice in one process.**

Scoped as a finding against `docs/decisions.md:1230` (D-581) and
`matrix_label_cache_key.md:164-170`, because MAJOR 2 requires revision 3 to import
this text and it must not import it as written.

D-581:

> **AND THE SHARPER FACT IS THAT NO LIMB OF IT EVER ASKS THE SAME POSITION TWICE
> INSIDE ONE PROCESS**: `A vs B` runs one script in two processes; `C vs D` runs
> one-process-per-position against all-positions-in-one-session, and the session
> limb asks each position once.

Scope: `run_seat`'s session builder, `tools/determinism.sh`.

```
BUDGETS=("depth_turns 4" "nodes 200000")
...
	for budget in "${budgets[@]}"; do
		for position in "${positions[@]}"; do
			printf 'newgame\nposition %s\ngo %s\n' "$position" "$budget" >>"$script"
			goes=$((goes + 1))
		done
	done
...
	for run in A B; do
		"$ENGINE" --config "$config" <"$script" >"$WORK/$name.raw.$run"
```

`$script` is `budgets × positions`, and **one process consumes all of it**. With
two budgets, **every position appears twice in run A's process and twice in run
B's**, each preceded by `newgame`. That holds on all five seats (the solver seat's
overrides are also two budgets, `depth_turns-2 nodes-10000`).

So the true statement is narrower and the gate is *stronger* than D-581 credits:
**no limb asks the same `(position, budget)` pair twice in one process.** Gate 9
does test that a repeated `position` line in one process, after `newgame`, is
unaffected by the earlier ask at that same line — it just varies the `go` line
across the repeat. That is one argument-step short of the cache's shape rather
than a shape the gate never takes.

The conclusion survives — §4.4 is still what closes the gap, because the gate
never repeats the *identical ask*, and none of its five seats is
`configs/instrument_v0.toml` at `nodes 400000`. But an ADR that overstates a gate's
blind spot is the same defect class as one that overstates its coverage, and this
one is stated in bold as *"THE SHARPER FACT"*.

**Fix.** In D-581 and in `matrix_label_cache_key.md` §4, replace *"never asks the
same position twice inside one process"* with *"never asks the same
`(position, go)` pair twice inside one process — its A/B session repeats every
position once per budget, and the repeat it never takes is the identical one."*
Then import the corrected sentence into §2 per MAJOR 2.

---

# minor

**minor 1 — §4.1's code block is a trimmed transcript, not the instrument's
output.** The block is introduced by
`python3 tools/label_cache_count.py --capture <the pilot's capture_v1.txt>` and
then prints eight lines. The script prints ten, each prefixed
`label_cache_count: ` (`tools/label_cache_count.py:46-47`); the block drops the
prefix, the `capture …, body sha256 …` line, `cache hits 395` and
`cache misses 347`. The numbers shown are all correct, and the receipt carries the
full output — but round 1's M3 was precisely a hand-shaped transcript, and
CLAUDE.md's closure rule is that a claim *"cites the gate's own log output"*.
Paste the ten lines.

**minor 2 — D-576 still cites the `_v1` receipt for the row round 1 found it does
not carry.** `docs/decisions.md:1220`: *"the distinct counts … are **347, 347, 347,
347** (`artifacts/arc3_leverB_41_count.txt`)"*. §4.1 declares `_v3` the receipt
that *"SUPERSEDES `_v1` and `_v2`"*. D-581 corrects two of D-576's claims and not
this one, so the ADR log and the registration now cite different receipts for one
number. Amend D-576's citation, or have D-581 do it.

**minor 3 — the ONE LINE and §5 disagree on the trade.** ONE LINE: *"1.63 h
ESTIMATED against ~1.43 h of registered verification (§5) — **roughly a wash**"*.
§5: *"roughly a wash on its own and **net negative** once the study that justifies
it is counted"*, against 1.43 + ~1.0 + ~0.27 = ~2.70 h. Both are defensible
readings of different denominators; stating two in one document is D-423. Say it
once, in §5.

**minor 4 — "~1.0 h" is the top of its own bracket, and the bracket may not hold
at N = 16.** `5 × 3 × 135 s = 2 025 s = 0.5625 h`; at *"contention 1.0-2.0 across
the field"* the range is 0.56–1.13 h, midpoint 0.84 h. Reporting the top is
conservative and fine; but the bracket assumes `c(16) <= 2.0` on an SMT-shared
8-core box, which §3.4's own SMT note treats as the case worth flagging. Say the
range, or widen the top.

**minor 5 — §7 does not print the command §7.1 limb 2 exercises.** Limb 2 is
*"`--label-cache` together with `--census` is REFUSED by name"*, and no §7 command
carries both words. §7's preamble says the commands are there *"in full"*.

**minor 6 — the dry-run input is a sub-range of the registered workload.** §7.1
uses openings `0..0`; §3.1 registers `0..2`. `docs/process.md` requires the dry run
be *"on an input of the SAME KIND as the registered workload … and never on the
registered workload itself"*. The report differs in identity, so this is arguably
satisfied, and D-539 means no sample is consumed either way — but round 1's B2 fix
asked explicitly for *"a small report that is **not** the registered `0..2` one"*,
and `configs/arena_wp20_label_pilot_dryrun.toml` (take 2, skip 0) is the same
overlap. Draw the dry-run openings from `3..3` and the overlap disappears.

**minor 7 — §8's instrument "revision" is prose, not a revision.** *"committed at
the arc III §1 head"* is not a commit SHA, and `docs/process.md` asks for the
artefact to be *"named in the pre-registration WITH ITS REVISION"* so *"a change
to it reopens the review"*. Once MAJOR 4's digest lands in §8 this is discharged;
until then a reader has neither a SHA nor a digest.

**minor 8 — §2.1's "Landed as D-576" is now half-superseded.** D-581 corrects
D-576's play-order sentence and its seat count. §2.1's *"Landed as D-576"* attaches
to the fold-declining argument, which D-581 leaves standing, so the citation is not
wrong — but the document cites D-576 five times and D-581 zero times while
carrying D-581's corrected play-order trace in §2. Cite the line the text came
from.

---

# WHAT SURVIVED ATTACK

**S1 — §4.1's entire count block re-derives exactly, under code I wrote from
scratch.** Scope: the pilot capture the receipt sha-pins
(`sha256sum` → `4563f050…`, matching the receipt's `input sha256`).

```
body lines: 742
asked prefixes: 742
distinct position lines: 347
ratio: 2.1383
hit rate: 0.5323
  345 line(s) asked 2 time(s)
  2 line(s) asked 26 time(s)
the ones asked 26x: ['position start', 'position start moves 0,0']
distinct sorted stone lists: 347
body_sha256: 1ea2710bc101495eb2d4980882c94badd6e7331878c6b7a545cc469f54183e60
```

The `body_sha256` matches the value the capture's own header declares, so the
instrument's self-check (`label_cache_count.py:64-80`) is doing real work. Every
figure in §4.1's block is right to the digits printed, the multiplicity structure
is exactly as claimed, and the two 26x lines are the two the document names.
**Round 1's M3 is fully discharged** — the receipt carries every quoted row.

**S2 — the symmetry-folded column is genuinely 347, checked two ways.** I
re-implemented `canonical_form`'s group from `crates/pistol-core/src/symmetry.rs`
(reflection `(q,r) -> (r,q)` at `:90-92`, rotation `(q,r) -> (-r, q+r)` at
`:138-142`, `Symmetry::ALL` = 12) and got **347**; with an extra
translation-normalisation step (which `canonical_form` does *not* do) I also got
**347**. So *"all four columns return 347"* is right, and the fold really does
merge nothing on the pilot population.

**S3 — the play-order trace verifies limb by limb, and it is the corrected one.**
Every citation in §2's *"THE OTHER TWO GATES DO NOT REACH IT"* paragraph checks:

- `heuristics.rs:191-193` is `fn last_stone` / `state.played().last().map(...)` / `}` — exact.
- `heuristics.rs:155` is `&& let Some(at) = last_stone(state)`, inside the
  `if gates.countermove` arm opened at `:153`. Exact.
- `heuristics.rs:89` is `for played in state.played()`, inside `record_cutoff`
  (opened `:70`).
- `pvs.rs:491-498` is the call site, and `:492` is `&& params.ordering.any()` —
  **not** `gates.killers`, exactly as the document says and contrary to D-576.
- `params.rs:114-116` is `pub const fn any(self) -> bool { self.killers || self.history || self.countermove }`.
- `pair_killers` is written only under `state.phase() == Phase::Second`
  (`heuristics.rs:93-100`), and every capture root is `Phase::First` because
  `capture.rs:342` slices whole turns (`position_line(&game.moves[..k])`).
- `history` reads `set.cells` and `self.history.get(&(mover, at))` and never
  `played()` (`heuristics.rs:163-181`).
- `git grep -n "played()" -- crates/pistol-search crates/pistol-engine` returns six
  hits; the only production search reads are `:89` and `:192` (`:412` is a test,
  `ordering.rs:135` is a test *name*).
- `configs/instrument_v0.toml:81` is `countermove = false`.

**The corrected trace is right, and D-581 is right to have corrected D-576.**

**S4 — the gate-9 citation is exact.** `tools/ci.sh:104-105` is
`step "gate 9/19: cross-process determinism"` / `gate "determinism" tools/determinism.sh`,
and `/usr/bin/grep -n 'gate [0-9]*/19' tools/ci.sh` returns exactly 19 steps, 1
through 19. §2's quoted C-vs-D text matches `tools/determinism.sh:14-18` verbatim,
and the coverage caveat (*"not `configs/instrument_v0.toml` at `nodes 400000`"*) is
correct on all five seats. Only the seat *count* is wrong (MAJOR 1).

**S5 — the matrix's scale numbers all re-derive, including the one I most expected
to be invented.** Scope: the pilot capture, grouping games by opening.

```
cross-opening (k>=2) overlapping-prefix pairs: 1576     <- matches "1 576 MEASURED"
C(3487,2) x 26.538 = 161 293 744                        <- matches "~161 296 550"
C(218,2)  x 26.538 =     627 703                        <- matches "~627 714"
161 296 550 / 1 576 = 102 345.5                         <- matches "102 346"
3 / 1576 = 1.9036e-3                                    <- matches "<= 1.9e-3 at 95%"
1.9036e-3 x 161 296 550 = 307 037                       <- matches "~307 000"
```

**1 576 is MEASURED and exactly right.** The rule-of-three bound is applied
correctly and the matrix is candid that the trials are not independent
(*"the yield is heavy-tailed … a zero at thirteen openings is the modal
observation"*). The whole scale argument is sound; only the a-priori paragraph
that sits beside it is not (BLOCKING 2).

**S6 — §2.0's scope invariant is real and its citations are exact.**
`capture.rs:333` `let go = label_go_line(...)`, `:340` `for game in &transcript.games`,
`:341` `for k in asked_prefixes(game)?`. The `go` line is loop-invariant, the memo
would live inside `with_seats`'s closure, and the invariant is registered as an
obligation the cache's own review checks. Round 1's M4 is fully closed.

**S7 — the isolation chain and the canonicality argument still hold.**
`capture.rs:247` is `for line in [pistol_cli::protocol::NEW_GAME, position, go]`.
`turn.rs:90` is *"[`Turn::Pair`] is **canonical**: `first < second`"*;
`turn.rs:215` is *"An uncanonical pair is refused rather than reordered"*;
`exchange.rs:154-161` is `position_line`, which does nothing but join
`turn.to_string()`. §2's *"the key folds NOTHING"* is correct and now correctly
attributed.

**S8 — §4.4's mechanism is a real mechanism.** M6's fix landed with all three
parts: the default-off flag, the run-log ordering obligation with a named reader,
and the void rule. It is checkable by a successor reading the log alone, which was
the point.

**S9 — the 1.63 h saving is robust to revision 4's correction.** I re-derived it
on the corrected tranche: `27 850 − 21 985 = 5 865 s = 1.629 h`, identical to §5's
figure, because the replay correction is additive to both waves. §5's *conclusion*
("roughly a wash, net negative with the study") survives BLOCKING 1 intact; only
its inputs and its intermediate figures are stale.

**S10 — the key selection itself survives everything.** I attacked it from the
fold side (BLOCKING 2 found a large `key_full` yield the document denies) and it
still holds: `key_full` buys that yield with a `bestmove` in the wrong frame
(D-137's non-symmetry-invariant tie-break), and `key_pos` buys nothing at all at
`k = 2`. §2's principle — *"a lost saving, never a wrong answer, which is the only
direction an error in a cache may fall"* — is the right principle and it decides
this correctly. **The finding makes the selection safer, not shakier.**

**S11 — 152 records is MEASURED and correct.**
`/usr/bin/grep -v '^#' corpus_v1.txt | /usr/bin/awk -F'\t' '$1<6' | wc -l` → **152**,
matching the receipt's own `LEVER A's WORKLOAD` line. M12 is fully discharged; the
D-291 defect is not repeated.

**S12 — §3.5's separation of design elements from criteria, and §3.6's registered
consequences, remain the best part of the document.** C1's consequence — *"THE
SWEEP STOPS, not this study"* — is the correct escalation, C4 is a genuine external
referent, and §3.2's divides-16 rule with its registered reopening is right.

---

# ATTACKS I ATTEMPTED AND REJECTED

**A1 — "the 347 in the symmetry-folded column is wrong, or the instrument's
symmetry group is not `canonical_form`'s."** Rejected. I read
`crates/pistol-core/src/symmetry.rs:44-79` (`Symmetry::ALL`, twelve entries),
`:88-98` (`checked_apply`: reflect by transposition, then `sixths` rotations) and
`:138-142` (`rotate`), reimplemented them, and matched
`tools/label_cache_count.py:138-147` exactly. Both give 347, and so does a version
with translation normalisation added. See S2.

**A2 — "the second instrument disagrees with the first."** Rejected on the
evidence: it returns **347**. But the agreement is worth much less than §4.1.1
claims, for the reasons in MAJOR 3 — I went looking for a disagreement and found
instead that a disagreement was structurally impossible
(`labels.rs:143` refuses the corpus otherwise), which is the more interesting
finding.

**A3 — "revision 4's replay correction changes the 1.63 h and therefore the
lever's verdict."** Rejected. The correction adds `1 057 s` to both waves and
cancels in the difference; the saving is unchanged to three figures. This is why
BLOCKING 1 is a document defect and not a decision defect — I checked before
rating it.

**A4 — "`record_cutoff`'s countermove WRITE is a second play-order path §2
missed."** Genuinely play-order dependent — `heuristics.rs:101-107` computes
`opponent_last` from `last`/`second_last`, and at a `Phase::First` root that IS the
root's own last played stone, so two transposed prefixes write different
countermove keys. **Rejected as a finding**: the write is consequential only
through the read at `:155`, which is gated by `gates.countermove`. §2's *"THE
SINGLE PATH … is `countermove`"* survives. Recorded here so the next reviewer does
not have to re-derive it.

**A5 — "the memo could outlive `capture::run` and the registration would not
notice."** Rejected — §2.0 registers exactly that invariant, in the words round 1's
M4 asked for, and makes it an obligation of the cache's own review.

**A6 — "a command in §3.2 or §7 still has `--label-nodes` before `--out`."**
Rejected: every occurrence in the document spells the binary's positional order.
B2's first half is properly fixed.

**A7 — "C4 will fire spuriously because openings `0..2` are a different
composition from the pilot's."** Rejected, as round 1 rejected it (A3, mean
`search_nodes` ratio 1.0042). I additionally checked that the record count the
study's rate is divided by is the MEASURED 152 rather than an estimate, so the
statistic and the referent are commensurable.

**A8 — "the pilot config as committed is take-3 and §3.1 is right."** Rejected:
it is take-13. That became MAJOR 6.

**A9 — "a capture root could be `Phase::Second`, making `pair_killers`
play-order dependent."** Rejected: `capture.rs:342` builds every root from
`&game.moves[..k]`, whole turns only, so every root is `Phase::First`. (And
`heuristics.rs:99` indexes `pair_killers[ply - 1]`, which would underflow at a
`Phase::Second` root — the code could not survive it either.)

**A10 — "the 1.5 threshold moved after the count was seen."** Not re-attempted:
round 1 settled it on timestamps (S5, sixteen minutes, right order) and revision 3
did not move the threshold. `/usr/bin/grep -n "1.5" ` over §4.1 shows the same
registered floor.

**A11 — "the `--label-cache` default-off registration violates hard rule 1's
'no code-side default'."** Rejected. `usage.rs:14` spells `[--census]` as the
same shape and D-558's package landed with it; an absent-means-off command word is
the tree's existing convention for capture-mode switches, and §4.2 names
`usage.rs` as the single place the default lives. Worth a reviewer's glance at the
cache package's own review, not a finding here.
