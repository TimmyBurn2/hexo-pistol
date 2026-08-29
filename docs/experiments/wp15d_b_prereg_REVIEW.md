# WP-1.5d (B) PRE-REGISTRATION — FRESH-CONTEXT REVIEW

## Header

| | |
|---|---|
| artefact | `docs/experiments/wp15d_b_prereg.md`, **revision 1**, at commit **`0dcd0db`** |
| HEAD | `0dcd0db` — **the artefact revision IS HEAD**; `git status --short` prints nothing (clean tree) |
| reviewer | fresh context; did not write the document, the instrument, or the run |
| registered artifact | `artifacts/wp15d_b_measurement_v1.txt` — digest **VERIFIED**: `sha256sum` returns `46aaf3fbafbc93bb4fca6816c023e6611a21a1fe739871f4b3ad945f78eefe3e`, matching §7.1 exactly |
| instrument | `crates/pistol-search/tests/wp15d_b_measurement.rs` at **`70cb580`** — **VERIFIED**: `git log --oneline -1 70cb580` names it; `git diff --stat 70cb580 HEAD -- <file>` prints nothing (unchanged since); `git merge-base --is-ancestor 70cb580 4ec470f` → YES |
| verification worktree | **`/home/tom/Projects/pistol-wt-pr`** (detached at `0dcd0db`, `CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-pr/target-review`). **NOT REMOVED**, per the dispatch. Contains the probe test `crates/pistol-search/tests/probe_cap.rs` and its log `probe.log`. The live tree was never edited. |

**Read in full:** the artefact; `CLAUDE.md`; `docs/process.md`; `docs/decisions.md`
D-22, D-74, D-159, D-374, D-388, D-395, D-401, D-402, D-427, D-465, D-478–D-484;
`docs/experiments/wp17_sprt_prereg.md`; `wp16_sprt_prereg.md` §5/§7A/§8;
`crates/pistol-search/tests/wp15d_b_measurement.rs`; `crates/pistol-search/src/info.rs`;
`crates/pistol-search/src/pvs.rs` §safety-net cap; `crates/pistol-arena/src/config.rs`;
`configs/arena_wp1*.toml`.

**Ran:** full recomputation of the corpus ratio, the IQR gate over all 168
position-seats, every §6 percentage, every §7.2/§7.3/§7.4/§7.7 figure, and the
selection rule by hand from the `CAL/SEAT` lines. Plus **one targeted partial
re-run of the instrument** (grid `[16, 100, 120, 128]`, `CAL_TAKE = 150`,
benches and SENS stubbed out) in the worktree, to test §7.2's load-bearing
claim directly. Full 45-minute re-run not taken; not needed for any finding below.

---

# VERDICT: **FAIL**

**3 BLOCKING, 7 MAJOR, 4 MINOR.**

The good news first, because it is substantial and it bears on how this report
should be read. **I did not find a sixth instance of the defect this package
keeps failing on.** Every number in §6, §7.1, §7.2, §7.3, §7.4 and §7.7
reproduces from `artifacts/wp15d_b_measurement_v1.txt` and from nowhere else. I
checked each figure against the pre-split artifacts D-482 declares inadmissible
(`artifacts/wp15d_m2_*`, `wp15d_turn_axis*`, `docs/experiments/matrix_M2.md`) and
**no figure in the document matches a pre-split artifact**. The commit clock
proves the registration preceded the run by ~52 minutes. On its own stated
terms — one run, one instrument, one artifact — this revision is clean.

It fails on what is **absent** and on what its **sentences claim beyond their
receipts**. The three BLOCKING findings are all omissions of things the governing
dispatch and `docs/process.md` require and that the two precedent documents
(`wp16`, `wp17`) both carry. The most interesting MAJOR is measured, not argued:
§7.2's "the grid validated itself at its own top end" rests on a grid point at
which the mechanism is **inert**, and the branch §2 registers to make the
calibration falsifiable **cannot fire by construction**.

---

# FINDINGS

## BLOCKING 1 — The SPRT's verdict parameters are registered nowhere: not in this document, not in any committed config. §5 reads verdicts that only those parameters define.

**Claim (§5, lines 155–166), verbatim:**

> | SPRT `h1` | the committed config moves to the selected K … |
> | SPRT `h0` | the gate stays `0`. A measured finding, not a failure … |
> | `inconclusive_at_game_cap` | reported as such; no config moves |

**The attack.** `h0`, `h1` and `inconclusive_at_game_cap` are not primitives.
They are the output of an SPRT defined by `elo0`, `elo1`, `alpha`, `beta` and a
game cap. **§4 registers none of them.** The §4 table registers arena, engines,
binaries, book, `openings_skip`, `openings_take`, budget, `turn_cap`,
`n_workers` and `hang_timeout_ms` — and stops. The hypothesis boundary and the
error rates, which are the single most classic after-the-numbers knob an SPRT
has, are left unfixed.

This is not a pointer-versus-restatement question (D-423). There is no pointer.
The document never names `elo1` at all.

**REPRODUCED.**

```
$ /usr/bin/grep -n "elo0\|elo1\|alpha\|beta\|game_cap\|max_games" docs/experiments/wp15d_b_prereg.md
(no match on any of elo0, elo1, max_games, game_cap; the only 'beta'/'alpha'
 substring hits are inside unrelated words)
```

The keys are mandatory in the arena schema, so they cannot be defaulted away:

```
$ /usr/bin/grep -n "elo0\|elo1\|alpha\|beta" crates/pistol-arena/src/config.rs
154:    pub elo0: f64,
156:    pub elo1: f64,
158:    pub alpha: f64,
160:    pub beta: f64,
```

Hard rule 1 forbids a code-side default for any tunable, so these four values
**must** be authored somewhere before the run — and as the document stands they
will be authored at launch, by the launching session, unreviewed, after the
calibration numbers are already on the page. That is precisely the ordering
D-483 exists to abolish.

**The precedent shows the shape.** `wp17_sprt_prereg.md:72–77`:

> - **H0**: `elo0 = 0.0` — the heuristics-ON seat is no stronger.
> - **H1**: `elo1 = 15.0` — **FIXED by this WP's dispatch** …
> - `alpha = 0.05`, `beta = 0.05` — the values every prior SPRT run used.

with a matching fill-in slot at `wp17_sprt_prereg.md:340`: "**9.1 `elo1`.** FIXED
at `15.0` (§2), by this WP's dispatch. Not re-opened."

**Fix:** register `elo0`, `elo1`, `alpha`, `beta` and the game cap in §4, with
the source that fixes `elo1` named.

---

## BLOCKING 2 — Criterion 1'', the second instrument with its registered agreement criterion and registered consequence, and the dry run are all absent. §5 invokes Criterion 1'' without defining it or pointing anywhere.

**Claim (§5, line 164), verbatim:**

> | Criterion 1'' fails on the governed report | **the run is not a measurement — not `h0`, not `h1`.** The verdict is not read. D-401's own precedent, and a hard stop |

**The attack.** This is the document's only occurrence of the string
"Criterion". It is invoked as a gate whose failure voids the verdict, and the
document never says what it is, which instrument evaluates it, what that
instrument's governing revision is, or what its exit codes mean. A reader
launching from this document cannot evaluate the row.

**REPRODUCED.**

```
$ /usr/bin/grep -n "Criterion\|second instrument\|dry run\|dry-run\|replay\|1''" \
    docs/experiments/wp15d_b_prereg.md
164:| Criterion 1'' fails on the governed report | ... |
```

One hit. No warm-replay instrument, no second instrument, no agreement
criterion, no consequence for disagreement, no dry run — and no dry-run config:

```
$ ls configs/ | /usr/bin/grep -i "15d"
(empty)
```

**Three separate binding requirements are unmet.**

1. **Criterion 1''.** `wp17_sprt_prereg.md:179–214` registers the warm-replay
   pass at `bfdf933`, the statistics layer `tools/wp16_warm_attribution_check.py`
   at `6c929da`, the binaries by digest, and then quotes Criterion 1'' **in full
   and without ellipsis** plus its exit-code taxonomy. This document has none of it.

2. **The second instrument.** `docs/process.md:58–77` is unambiguous and it
   binds here — the governed run is cheap:

   > Where the run is cheap, doubt about the instrument is answered by
   > REPLICATION and by a SECOND INSTRUMENT whose agreement criterion is
   > registered before either runs … A registered agreement criterion carries a
   > REGISTERED CONSEQUENCE: the pre-registration states, before either
   > instrument runs, what DISAGREEMENT does to the verdict, or the criterion
   > leaves standing the after-the-numbers decision it exists to forbid. AND IT
   > NAMES THE STAGE UNDER DOUBT, and says how the second instrument does not
   > share it.

   `wp17_sprt_prereg.md:194` and `:222–232` discharge all four clauses
   (instrument + revision, criterion quoted, consequence, stage under doubt).
   This document discharges none.

3. **The dry run.** `docs/process.md:33–43`:

   > A pre-registration's literal commands are exercised before its review
   > passes, on an input of the SAME KIND as the registered workload … The
   > pre-registration records the dry-run input and its output.

   "before its review passes" — this review. There is no §8, no dry-run config,
   no recorded input or output. `wp16` §8 and `wp17` §8 both carry one, with
   per-criterion defect classes.

**How heavy.** Heavy. This is not three stylistic omissions; it is the entire
apparatus that decided WP-1.6's fate (D-401: Criterion 1' failed on the governed
report and the run was not read). The §5 row shows the author knows the
apparatus applies. Item 1 is arguably dischargeable by an explicit
import-by-reference in `wp17`'s style ("WP-1.6 §5's rows govern, imported by
reference and not restated here"), which is cheap. Items 2 and 3 are real work.

---

## BLOCKING 3 — No arena config is named or committed. §7.6 is D-427's own slot and it points at nothing.

**Claim (§7.6, lines 260–262), verbatim:**

> **7.6 — `openings_skip`**: to be read from the arena config at launch and
> compared against §4's registered **1500**. This is D-427's own slot and the
> reason it exists.

**The attack.** *Which* arena config? §4's table names `target/release/arena`
(the binary) and `configs/instrument_staged_v0.toml` (engine B's config). It
never names the arena match config that must carry `openings_skip`,
`openings_take`, `turn_cap`, `n_workers`, `hang_timeout_ms` and the four SPRT
parameters of BLOCKING 1. No such file exists.

**REPRODUCED.**

```
$ /usr/bin/grep -n "configs/arena" docs/experiments/wp15d_b_prereg.md
(no match)
$ ls configs/ | /usr/bin/grep -i "15d"
(empty)
```

Contrast `wp17_sprt_prereg.md:104`, which names
`configs/arena_wp17_heuristics_vs_staged.toml` and records that it was "authored
and committed at" a revision, with a §9.7 slot re-confirming it.

**Why this is blocking and not clerical.** D-427 — the ADR §7.6 cites — is the
record of a slot pass catching `openings_skip = 0` in a config authored at an
earlier prereg revision, which "would have re-played the retired sample". The
lesson D-427 draws, in its own words, is that the slice is "**Decided in the
pre-registration itself — D-427's lesson — never at launch**" (quoted from
`configs/arena_wp17_heuristics_vs_staged.toml:38`). A slot that says "read it
from the config at launch" without naming a config that exists and has been
reviewed **reinstates exactly the failure mode D-427 records**. Combined with
BLOCKING 1, the file that will carry the verdict-defining parameters does not
exist and will be written after this review closes.

---

## MAJOR 1 — §7.3's universal node-count claim is falsified by 14 of the 168 cells, and the inference it supports rests on a property the node budget guarantees.

**Claim (§7.3, lines 238–240), verbatim:**

> No seat's node counts differ — every corpus cell is `nodes=50176` — so the ratio is a like-for-like time comparison.

**The attack, part 1 — the universal is false.**

**REPRODUCED.**

```
$ /usr/bin/grep '^BENCH/CORPUS' artifacts/wp15d_b_measurement_v1.txt \
    | /usr/bin/grep -oE 'nodes=[0-9]+' | LC_ALL=C sort | uniq -c
      7 nodes=151
      7 nodes=3
    154 nodes=50176
```

Fourteen of 168 corpus cells are not `nodes=50176`. Positions `p13`
(`nodes=151`) and `p16` (`nodes=3`) read so at **every one of the seven seats**.
"every corpus cell is `nodes=50176`" is not true of 8.3 % of the cells.

**The attack, part 2 — the inference is weaker than stated.** Under
`Stop::Nodes(50_000)` every search that does not terminate early stops at the
same node count *by construction of the budget*. Equal node counts across seats
are therefore what the budget produces, not evidence that the seats searched
comparable trees. The observation does carry *some* information — it shows no
seat terminated early where another did not — but that is a much weaker
statement than "so the ratio is a like-for-like time comparison."

**What survives.** The *conclusion* is fine, on a corrected premise: node counts
are identical **across seats, per position** (`50176` at 22 positions, `151` at
p13, `3` at p16, at all seven seats), which is the property the like-for-like
reading actually needs. The sentence needs rewriting, not the section.

The gate figures themselves reproduce exactly — see WHAT I CHECKED AND FOUND SOUND.

---

## MAJOR 2 — §2's falsifiability branch cannot fire by construction, and §7.2 rests its validation on a grid point at which the mechanism is inert. **Measured.**

**Claim A (§2, lines 53–56), verbatim:**

> **THE GRID.** `K ∈ {4, 8, 16, 32, 64, 128}`, plus the incumbent seat `K = 0`.
> It spans widely on purpose: if the benefit does not decay across it, the rule
> selects the largest point and **that outcome is a finding about the channel**
> rather than a calibration, recorded as such.

**Claim B (§7.2, lines 227–233), verbatim:**

> **AND THE GRID VALIDATED ITSELF AT ITS OWN TOP END**, which §2 registered as the
> thing that would make the rule meaningful rather than vacuous: at K = 128 the
> instrument reports `capped_rows=0` and a mean identical to the incumbent's to
> four figures (2.0740 both). **The cap never binds there** — no pool on this book
> exceeds 128 — so the benefit decays to exactly zero inside the registered grid,
> and the selection is a choice among points that differ rather than a pick from a
> flat line.

**The attack.** At `K = 128` the cap is **inert**: `capped_rows=0` means it never
fires, so the K=128 seat is not a dose of the treatment — it is the incumbent
seat re-run under a different label. The artifact confirms this is literal: K=128
and K=0 agree on *every* printed quantity.

```
CAL/SEAT K=0   mean_depth=2.0740 ... depth_hist=[0, 3, 926, 65, 6, 0, 0, 0] capped_rows=0 upper_withheld=0 exact_withheld=0
CAL/SEAT K=128 mean_depth=2.0740 ... depth_hist=[0, 3, 926, 65, 6, 0, 0, 0] capped_rows=0 upper_withheld=0 exact_withheld=0
```

Two consequences follow, and §7.2 states the opposite of both.

**(a) §2's registered branch is unreachable.** For "the benefit does not decay
across [the grid]" to fire, the rule would have to select K=128, i.e.
`gain(128) ≥ 0.75 × max gain`. But `gain(128) ≡ 0` whenever the cap is inert
there, so the branch requires `0 ≥ 0.75 × max gain` with `max gain > 0` —
impossible. K=128 **can never be selected on this book**. §2's stated reason for
spanning the grid widely describes an outcome the instrument cannot produce.
Under `docs/process.md:45–56` that is the named defect:

> A criterion that is a property the named defect class PRESERVES … passes
> vacuously and is not a criterion; it must be one that defect could falsify.

**(b) The top-end zero measures the pool-size ceiling, not benefit decay.**
§7.2 reads `capped_rows=0` at K=128 as *validation*. It is the opposite: it is
the grid running off the end of the candidate-pool distribution.

**REPRODUCED — targeted partial re-run**, worktree `/home/tom/Projects/pistol-wt-pr`,
grid `[16, 100, 120, 128]`, `CAL_TAKE = 150`, everything else at `70cb580`:

```
$ cargo test --release -p pistol-search --test probe_cap -- --ignored --nocapture --test-threads=1
CAL/SEAT K=0   mean_depth=2.0733 population=150 depth_hist=[0, 1, 138, 10, 1, 0, 0, 0] capped_rows=0
CAL/SEAT K=16  mean_depth=2.3067 population=150 depth_hist=[0, 0, 114, 26, 10, 0, 0, 0] capped_rows=118396
CAL/SEAT K=100 mean_depth=2.0733 population=150 depth_hist=[0, 1, 138, 10, 1, 0, 0, 0] capped_rows=1118
CAL/SEAT K=120 mean_depth=2.0733 population=150 depth_hist=[0, 1, 138, 10, 1, 0, 0, 0] capped_rows=0
CAL/SEAT K=128 mean_depth=2.0733 population=150 depth_hist=[0, 1, 138, 10, 1, 0, 0, 0] capped_rows=0
DONE_0
```

The maximum pool on this book is in **(100, 120]**, not "128". At K=100 the cap
fires 1118 times and moves the mean by **exactly nothing** (2.0733 both,
histogram byte-identical). So the region K ∈ [100, 128] is a flat inert shelf,
and K=128 sits on it. §7.2's "the selection is a choice among points that differ
rather than a pick from a flat line" is true of K4–K32 and **false of the top of
the grid it cites as its evidence**.

**What survives, and it is the fix.** The decay *is* genuinely measured — but at
**K=64**, which §7.2 never cites: there the cap fires **818,937** times (more
than at any smaller K) and still yields `gain = +0.0140`. That is a binding cap
producing near-zero benefit, which is real decay under real treatment and is
exactly the evidence §7.2 wants. The argument should rest there, and §2's
non-decay branch should be rewritten to something the instrument can actually
falsify (e.g. keyed on the largest grid point at which `capped_rows > 0`).

**The selection itself is unaffected** — see WHAT I CHECKED AND FOUND SOUND.

---

## MAJOR 3 — The verdict space is not total: the corpus ratio band (1.10, 1.25] has two conflicting instructions and no §5 row.

**Claims, verbatim.** §3 (line 114):

> | `bench_positions_v1` (corpus) | Σ median ms ON / OFF **≤ 1.10** | **> 1.25** |

§3 (lines 119–120): "**If the ABORT fires the SPRT is not run** and the package
closes on the bench". §5 (line 160): "| corpus ABORT fires | no SPRT, whatever
the calibration said. The package closes on the bench |". §8 (lines 281–282):

> **Registered numbers never move after the run** (D-374): a bracket missed is a finding, and the change is reverted and the number recorded.

**The attack.** A ratio of, say, 1.15 misses the bracket without firing the
ABORT. §3 and §5 then say the SPRT runs (only the ABORT stops it). §8 says the
change is reverted. Those cannot both happen, and §5 — the table whose whole
purpose is that every branch is handled before game one — has no row for the band.

**This case is live, not hypothetical.** D-374 is the ADR §8 itself cites, and it
records this exact shape occurring: "ttd leg cleared … abort threshold not
approached, nps leg missed ABOVE bracket at 1.568x/1.566x", discharged by
architect ruling with the miss "recorded as a FINDING". So the project has
already been in the (bracket, abort) band once and needed an architect to get out.

**NOT REPRODUCED as a run outcome** (the bench came in at 1.0015, far below
1.10) — this is a gap in the document's branch coverage, verified by reading:

```
$ /usr/bin/grep -n "^| " docs/experiments/wp15d_b_prereg.md | sed -n '/calibration selects no K/,/arena exit 2/p'
```

returns seven rows: no-K, ABORT, `h1`, `h0`, `inconclusive_at_game_cap`,
Criterion 1'' failure, arena exit 2. No band row. This is the same MAJOR
`wp17` revision 1 took ("the verdict space was not total").

Also uncovered: arena exit 1 and exit 3, and a pre-game refusal with no report —
all three of which `wp17:136` handles by explicit import of WP-1.6 §5's rows.

---

## MAJOR 4 — The IQR gate's 10 % threshold appears for the first time in the results section. It was never registered.

**Claim (§7.3, lines 237–238), verbatim:**

> **IQR gate: 0 of 168 position-seats exceeded 10 % of their own median.**

**The attack.** §3 is the section that registers the bench. It registers the
corpus bracket, the ABORT, the direction convention, five reps and the
per-position median. **It does not register an IQR gate at all**, and the number
`10 %` occurs nowhere in the document before line 237 — which is inside §7, the
fill-in slot pass, written after the run.

**REPRODUCED.**

```
$ /usr/bin/grep -n "IQR\|iqr\|10 %\|10%" docs/experiments/wp15d_b_prereg.md
237:larger-is-worse. Bracket ≤ 1.10: **PASS**, and far from the 1.25 ABORT. **IQR
238:gate: 0 of 168 position-seats exceeded 10 % of their own median.** No seat's
```

Both hits are in §7.3. Nothing in §1–§3.

**Why it matters even though it passed.** It passed cleanly (I recomputed: 0 of
168, see below), so nothing is being laundered here. But the document is
self-inconsistent on exactly this point. §2 lines 72–75 refuses a threshold on
the calibration channel on these grounds, in its own words:

> a "reaches depth ≥ n" count would need an `n` chosen from data D-482 makes
> inadmissible, which is the after-the-numbers choice a pre-registration exists
> to forbid.

A gate whose threshold first appears beside its own passing result is that
choice, whichever way it came out. CLAUDE.md hard rule 5 requires the bench be
"IQR-gated", so the gate belongs in §3 with its threshold.

**Secondary, minor:** the gate is vacuously satisfied on the 14 cells whose
median is 0 (`iqr > 0.10 × 0` is false for `iqr = 0`, always). Worth one clause
when the gate moves into §3.

---

## MAJOR 5 — No cost statement. `docs/process.md` requires one on the document's own face.

**The attack.** `docs/process.md:58–61`:

> A pre-registration states what its governed run COSTS — wall time, operator
> attention, machine hours — so the proportion between the document and the run
> is visible on the document's own face.

**REPRODUCED.** The document states the *measurement* run's cost (§7.1,
"2662.25 s, exit 0") but says nothing about the governed SPRT's — no wall-time
estimate, no worker-hours, nothing. `wp16` has a §7 "Costs"; `wp17` has a cost
table at lines 159–163 with per-row MEASURED/ESTIMATED marking.

This is not bookkeeping. The cost rule is the premise of the second-instrument
rule in the same `process.md` section ("**Where the run is cheap**, doubt about
the instrument is answered by REPLICATION and by a SECOND INSTRUMENT"). Without
a cost statement the document never reaches the question BLOCKING 2 answers.
D-291's marking discipline applies to whatever estimate is added.

---

## MAJOR 6 — CLAUDE.md rule 6's reporting requirements are never registered or read.

**The attack.** Hard rule 6 (`CLAUDE.md:90–92`):

> **Strength claims.** Ship instrument (fixed depth/nodes), protocol, n, distinct-n
> (identical games deduped), and per-side compute.

§4 registers the instrument (`nodes`, 50000 ✓) and `openings_take = 500` (n ✓).
**`distinct_n`, per-side compute and the protocol are registered nowhere, and §5
never reads them.**

**REPRODUCED.**

```
$ /usr/bin/grep -n "distinct\|per-side\|compute\|protocol" docs/experiments/wp15d_b_prereg.md
(no match)
```

`distinct_n` is not decorative here. `wp17:131–132` devotes two full outcome
rows to it, because two deterministic seats differing in one candidate-policy
key **can legitimately play identical games** — and the safety-net cap is exactly
such a key: §7.3 shows it fires only 13 times across the whole 24-position
corpus. A `verdict inconclusive_degenerate` with `distinct_n == n/2` is a live
outcome for this matchup and §5 cannot route it. That is a second, independent
hole in §5's totality (see MAJOR 3).

---

## MAJOR 7 — §7.7 confirms `hang_timeout_ms` NO-CHANGE on a fixture that is not the governed workload either, while the run walked the governed workload and emitted no timing.

**Claim (§7.7, lines 264–271), verbatim:**

> **That is below the project's ~24× convention and is flagged here rather than at
> launch**: the SPRT plays `random_openings_v1`, not `spread_v1`, and the worst
> single median on `bench_positions_v1` is **491 ms — a 244× margin**. The spread figure is not
> in the governed run's own workload, and §4's timeout is therefore confirmed
> NO-CHANGE on the workload it actually guards, with the reasoning recorded.

**The attack.** The arithmetic is right and I reproduce every figure (below).
The *inference* substitutes one non-governed fixture for another.
`bench_positions_v1` is **not** "the workload it actually guards": its positions
carry 15–35 stones, while the governed SPRT plays games from
`random_openings_v1` out to `turn_cap = 40`, reaching ~80 stones. On stone count
— the variable that drives D-74's non-abortable first iteration, which §7.7
itself names as the cause — `spread_v1`'s 51- and 99-stone positions are
*closer* to a late governed position than the corpus's are. The document
discards the 11.3× figure as off-workload and adopts the 244× figure as
on-workload, when neither fixture is the governed workload.

**REPRODUCED (the figures are all correct).**

```
worst spread median  = 10596 ms  →  120000/10596 = 11.33x   ✓ "11.3×"
worst corpus median  =   491 ms  →  120000/491   = 244.4x   ✓ "244×"
(worst single reps: 10623 and 502 → 11.3x and 239x; immaterial)
```

**What makes it a MAJOR rather than a nitpick: the run had the right data and
did not print it.** The SENS section walks 25 games to `turn_cap = 40` on
governed-shape trajectories — 595 and 672 searches — and emits no timing at all
(`crates/pistol-search/tests/wp15d_b_measurement.rs`, `fn sensitivity`). A
governed-shape worst-search time was one `println!` away.

**Honest counterweight, which caps the severity.** D-159 makes the consequence a
lost run, not a wrong answer: "Silence abandons the RUN and is never
adjudicated; a child that EXITED WITH A CODE forfeits, and a child killed by a
SIGNAL … abandons the run like silence." An under-margined timeout costs a
re-run, it does not corrupt a verdict. And a crude bound from the artifact's own
totals suggests the margin is in fact ample: the SENS section accounts for
roughly 590 s of the 2662 s run across ~2534 searches, ≈ 233 ms mean. The
document should say that, from a receipt, instead of arguing from the wrong
fixture.

---

## MINOR 1 — "the worst single median anywhere in the run" is not established for the whole run.

**Claim (§7.7, lines 264–266):** "The worst single median anywhere in the run is
**10 596 ms**".

The CAL section emits only per-seat `sum_ms` (`sum_ms=214827` at K=16 over 1000
openings, ≈ 215 ms mean); it never prints a per-opening time, so no per-search
maximum exists for it in the artifact. The claim is established for the two
BENCH sections only. **REPRODUCED**: `/usr/bin/grep '^CAL/SEAT'` shows `sum_ms`
and no per-opening figure. Rewrite as "anywhere in the two bench sections".

## MINOR 2 — §6.3's percentages have an unstated denominator whose natural reading is wrong by ~5×.

**Claim (§6.3, lines 193–195):** "The class occurs on about a fifth of
governed-shape searches and the played turn changes on **4.9 %–5.8 %** of them".

"of them" most naturally binds to the immediately preceding quantity — the fifth
of searches on which the class occurs. Under that reading the figures are wrong:
`29/125 = 23.2 %` and `39/135 = 28.9 %`. The stated 4.9 %/5.8 % are `29/595` and
`39/672`, i.e. fractions of **all** searches. **REPRODUCED** by recomputation.

Mitigating, and why this is MINOR not MAJOR: §6.1 and §6.2 give both denominators
explicitly one paragraph earlier, and §6.3's conclusion ("INFORMATIVE, direction
unknown") holds under either reading. Fix by naming the denominator.

## MINOR 3 — Revision 1 was never fresh-context reviewed before the measurement run it governs.

CLAUDE.md: "that revision must itself pass a fresh-context review before the
first run it governs". `artifacts/wp15d_b_measurement_v1.txt`'s own header
declares "Governing pre-registration: docs/experiments/wp15d_b_prereg.md
revision 1", and §1–§3 register that run's instrument, budget, grid, selection
rule and brackets. This review is the first. §9 states the review as forthcoming
while §6–§7 already carry the run's numbers.

**Weighted honestly: the protection the rule exists for held.** The commit clock
proves the registration preceded the run, the selection rule is code inside the
instrument rather than a human step, and no threshold moved. Recording it so the
sequencing is on the document's face rather than only in the git log.

## MINOR 4 — "verbatim" quotations are reflowed.

§7.2 presents `CAL/SELECTED` as three indented lines; the artifact line 42 is
one line. §6's second quote reads `SENS/TRAJECTORY=capped    games=25` with
alignment padding; artifact line 243 has a single space. **Content verified
identical in every case** (whitespace-normalised comparison of all three quotes
returns `True`); only the `capped` line's raw whitespace differs. Trivial, but
the document claims "verbatim" twice and this package's history is quotation
fidelity.

---

# WHAT I CHECKED AND FOUND SOUND

**This is the substantial part of the report and should be read as such.**

### Provenance — no sixth instance found

I traced every figure in §6, §7.1–§7.4 and §7.7 to
`artifacts/wp15d_b_measurement_v1.txt` and checked each against the pre-split
sources D-482 declares inadmissible — `docs/experiments/matrix_M2.md`,
`artifacts/wp15d_m2_evidence_v1/v2.txt`, `artifacts/wp15d_m2_evidence_instrument_v1/v2.txt`,
`artifacts/wp15d_turn_axis_v1.txt`, `artifacts/wp15d_turn_axis_book_v1.txt`.
**No figure in the document matches a pre-split artifact.** The document's
opening claim at lines 17–21 — "NO NUMBER FROM BEFORE THE SPLIT APPEARS IN THIS
DOCUMENT" — holds as far as I can test it.

### The registration genuinely preceded the run

The single most important integrity check for a pre-registration, and it passes
with a clear margin:

```
70cb580  2026-08-29 19:47:30 +0200   (the instrument)
4ec470f  2026-08-29 19:48:50 +0200   (prereg revision 1 — grid, rule, brackets)
         2026-08-29 21:24:15         (artifact mtime; wall 2662.25 s ⇒ start ≈ 20:40)
0063ac2  2026-08-29 21:25:15 +0200   (slot pass, §6/§7 filled)
```

The grid, the channel, the selection rule and both bench brackets were committed
**~52 minutes before the run began**, and `git merge-base --is-ancestor 70cb580
4ec470f` confirms the instrument predates the document. No bracket moved after
the run.

### The selection rule, recomputed by hand from the `CAL/SEAT` lines

| K | mean_depth | gain = mean(K) − 2.0740 | ≥ 0.2730 ? |
|---|---|---|---|
| 0 | 2.0740 | — | — |
| 4 | 2.4380 | +0.3640 (max) | yes |
| 8 | 2.4110 | +0.3370 | yes |
| **16** | **2.3730** | **+0.2990** | **yes ← largest qualifying** |
| 32 | 2.3090 | +0.2350 | no |
| 64 | 2.0880 | +0.0140 | no |
| 128 | 2.0740 | +0.0000 | no |

`threshold = 0.75 × 0.3640 = 0.2730` exactly. Largest K with `gain ≥ 0.2730` is
16. **§7.2's `K = 16` reproduces**, and §7.2's arithmetic gloss ("K16's gain
(+0.2990) clears the 0.75 threshold (+0.2730) and K32's (+0.2350) does not") is
correct. The selection is unaffected by MAJOR 2: the threshold is set by
`gain(K4)` and the argmax runs over binding points, so the inert K=128 seat
enters neither.

I read `fn selection` at `70cb580` line-by-line against §2's registered rule.
**It matches exactly**: `gain(K) = mean(K) − mean(0)`; `best ≤ 0.0` returns the
`CAL/SELECTED none` branch with the package-closes verdict string; otherwise
`threshold = 0.75 * best` and `chosen = max` over qualifying K — `.max()` giving
ties to the larger K as registered. The rule runs inside the instrument, so
§2's "not a step a human takes after seeing the table" is true of the code.

### The excluded set is the union over all seats, not per-seat

Verified in `fn calibration`: `mate_any` accumulates across the whole seat loop
(`if one.mate && !mate_any.contains(&i) { mate_any.push(i) }`), and every seat's
`kept` vector is filtered against the **completed** `mate_any` after the loop,
because `rows` is collected first and the means computed second. §2's "the
excluded set is the union over ALL seats, the incumbent included, computed once"
is exactly what the code does; D-395's union precedent is honoured. In this run
the set was empty (`CAL/EXCLUDED mate_terminated_on_any_seat=0 indices=[]`), so
it never bit — but the mechanism is correct and would have been checkable.

### The pinned instrument parameters

Verified in `fn searcher` and the module constants at `70cb580`:
`Stop::Nodes(NODES)` with `const NODES: u64 = 50_000` ✓; `const QUIET_RADIUS: u32 = 2`
used for every seat and swept nowhere ✓; `tt_bytes: common::SMALL_TT` ✓; all
three ordering heuristics `false` ✓; `GRID = [4, 8, 16, 32, 64, 128]` ✓. §1's
table is accurate in every row.

### The calibration and SPRT populations are disjoint

`CAL_SKIP = 0`, `CAL_TAKE = 1000` → openings `0..999`, and the instrument asserts
the slice is whole. §4 draws `1500..1999`. Disjoint ✓. The SENS receipt reads
`cal.take(25)` = openings `0..24`, also disjoint from the governed slice, so §6's
"a prediction about the governed run and never a look at it" holds.

### §4's slice accounting is correct — verified against the committed configs

| slice | consumed by | evidence |
|---|---|---|
| `0..499` | WP-1.5b | `configs/arena_wp15b_staged_vs_r2.toml`: `openings_take = 500`, `openings_skip = 0` |
| `500..999` | WP-1.6 | `configs/arena_wp16_defensive_only_vs_staged.toml`: `openings_take = 500`, `openings_skip = 500` |
| `1000..1499` | WP-1.7 | `configs/arena_wp17_heuristics_vs_staged.toml`: `openings_take = 500`, `openings_skip = 1000` |
| `1500..1999` | **this run — fresh** | disjoint from all three by construction |

Book size confirmed: `/usr/bin/grep -c "start moves"
crates/pistol-cli/tests/fixtures/random_openings_v1.txt` → **2000**. So
`skip 1500, take 500` is within the book and is the last unconsumed slice.
§4's "THE BOOK IS NEARLY SPENT AND THIS DOCUMENT SAYS SO" is accurate and worth
keeping. §4's compressed "(WP-1.5b, and retired by D-402)" for `0..499` is fair:
WP-1.5b consumed it, and D-402 separately retired the D-401 run that also drew it.

### §7.3's gate figures — recomputed independently

```
Σ per-position medians:  K=0  → 4800 ms;  K=16 → 4807 ms
ratio ON/OFF = 4807/4800 = 1.0014583…  → "1.0015"          ✓
bracket ≤ 1.10 → PASS, far from the 1.25 ABORT             ✓
IQR gate: 0 of 168 position-seats with iqr > 0.10 × median ✓
cell count: 168 = 24 positions × 7 seats                   ✓
```

All reproduce. The direction convention (ON/OFF, larger-is-worse) matches
D-388/D-395/D-398, and §3's refusal to gate on `nps` across seats with different
candidate policies is right under D-374.

### §7.4's spread table — every cell reproduces

| stones | doc: incumbent capped | artifact | doc: K=16 capped | artifact | doc: median OFF→ON | artifact |
|---|---|---|---|---|---|---|
| 11 | 0 | 0 ✓ | 95 | 95 ✓ | 133 → 133 | 133 → 133 ✓ |
| 21 | 0 | 0 ✓ | 71 | 71 ✓ | 449 → 470 | 449 → 470 ✓ |
| 51 | 0 | 0 ✓ | 152 | 152 ✓ | 2761 → 2748 | 2761 → 2748 ✓ |
| 99 | 0 | 0 ✓ | 0 | 0 ✓ | 10596 → 10573 | 10596 → 10573 ✓ |

"every seat reads `depth=1` at every position" — confirmed, all 28
`BENCH/SPREAD` lines are `depth=1`. §7.4's refusal to let the spread report
discharge D-95's debt is correct and correctly grounded (D-22 refuses the
wall-clock budget the debt is defined at; D-478 leaves it open at its own package).

### §6's percentages

`125/595 = 21.01 %` → "21.0 %" ✓; `29/595 = 4.874 %` → "4.87 %" ✓;
`135/672 = 20.09 %` → "20.1 %" ✓; `39/672 = 5.804 %` → "5.80 %" ✓. Both quoted
`SENS/TRAJECTORY` lines are content-identical to artifact lines 242–243.

§6.2's justification for measuring both trajectories — "a divergence rate read on
one engine's path is a rate on a distribution the other never walks" — is
correct about what the instrument does: `driver_capped` changes which engine's
move is *played*, while both counters read the incumbent's own search at every
position, which is the right way to hold the measured engine fixed and vary the
path. §6.4 states a falsifier and the run does not meet it. **§6.3's reading
("INFORMATIVE, direction unknown") is supported and does not overclaim** — it
explicitly declines to predict either `h0` or `h1`, which is the honest position
given the receipt.

### The corpus bracket is not a vacuous criterion (process.md item 7)

I tested this specifically. The cap fires only 13 times across the entire K=16
corpus seat (2 of 24 positions), so the bracket **cannot** detect "the cap is
expensive when it fires". But that is not the defect §3 registers it against:
§3 registers it as a **no-regression check**, and the defect class it *can*
falsify is real — the always-paid cost of the guard on every safety-net row,
which would show up across all 24 positions. That class is genuinely tested and
genuinely excluded (K=128, where the guard is armed but never binds, comes in at
4799 ms against the incumbent's 4800 ms). §3's framing — "The cap narrows a row
this fixture rarely takes, so a large gain here is not expected and would itself
want explaining" — is honest about the fixture's limits. **Not vacuous.** If
anything §3 could state which class it cannot catch, but I am not raising that
as a finding.

### The mechanism and the counters

`crates/pistol-search/src/pvs.rs:345–351` — the cap condition is
`safety_net_top_k > 0 && turns_from_root() > 0 && used_quiet_safety_net &&
cells.len() > cap`, matching D-484's root-turn exemption spelled in turns. The
`usize::try_from(...).unwrap_or(usize::MAX)` saturation is correct and its
comment explains why (an `as` narrowing would `truncate(0)` on a 32-bit target).
Truncation precedes `promote_table_move`, so promotion cannot re-admit a cut
cell. `safety_net_capped_rows` counts exactly the rows where truncation
happened; `batched_quiet_safety_net` counts nodes that used the quiet-ball
safety net, which is the right "the class occurs" counter for §6.1's
"safety-net-bearing" label. Counter semantics match the document's usage.

### §5's rows that are present are correctly specified

The `h1`/`h0`/no-K/ABORT/Criterion-1''/exit-2 rows each state a consequence
before game one, and the no-K and ABORT rows correctly close the package without
running the SPRT. "One re-run is licensed only on a receipted environment fault,
never on a verdict anyone dislikes" is the right guard. §4's slot-A reasoning —
"slot A because the arena's statistic is slot A's score and H1 is 'the capped
seat is stronger'" — is the correct orientation and worth keeping stated. My
MAJOR 3 and MAJOR 6 are about rows that are **missing**, not rows that are wrong.

---

# SUMMARY FOR THE FIX ROUND

The numbers are clean; the document is not finished. Ranked by what the one fix
round should attack first:

1. **BLOCKING 1** — register `elo0`/`elo1`/`alpha`/`beta`/game cap in §4.
2. **BLOCKING 3** — name and commit the arena config; give §7.6 a referent.
3. **BLOCKING 2** — Criterion 1'' (importable by reference, cheap), the second
   instrument with agreement criterion **and** registered consequence, and the
   dry run (real work).
4. **MAJOR 2** — rewrite §2's falsifiability branch so it can fire, and move
   §7.2's evidence from the inert K=128 point to the binding K=64 point.
5. **MAJOR 1, 4** — correct §7.3's node-count sentence; move the IQR threshold
   into §3.
6. **MAJOR 3, 6** — close §5's verdict space: the (1.10, 1.25] band, the
   degenerate `distinct_n` cases, arena exits 1 and 3, no-report.
7. **MAJOR 5, 7** — add the cost statement; re-ground the `hang_timeout_ms`
   confirmation on a governed-shape receipt.
8. **MINOR 1–4** — wording and sequencing.

**Worktree left in place as instructed: `/home/tom/Projects/pistol-wt-pr`**
(detached at `0dcd0db`; probe test at `crates/pistol-search/tests/probe_cap.rs`,
log at `probe.log`, build dir `target-review/`). The live tree was not modified
at any point.
