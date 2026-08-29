# WP-1.5d (B) PRE-REGISTRATION REVISION 2 — SCOPED RE-REVIEW

## Header

| | |
|---|---|
| artefact | `docs/experiments/wp15d_b_prereg.md` **revision 2** |
| named revision | `b75c1f6b448f20a636d35444d9beb8dc54e5ef56` |
| matches HEAD? | **YES.** `git rev-parse HEAD` → `b75c1f6b448f20a636d35444d9beb8dc54e5ef56`; `git status --short` → empty. Branch `dev` |
| prior revision | `0dcd0db` (revision 1), FAILED at `docs/experiments/wp15d_b_prereg_REVIEW.md` — 3 BLOCKING, 7 MAJOR, 4 MINOR |
| reviewer | fresh context; did not write revision 1 or revision 2, and is not the reviewer of revision 1 |
| worktree created | **`/home/tom/Projects/pistol-wt-pr2`** (detached at `b75c1f6`), NOT removed, per the dispatch |

**SCOPING — PROVED CLEAN.**

```
$ git diff 0dcd0db..b75c1f6 --name-status
A	configs/instrument_staged_snk_v0.toml
M	docs/experiments/wp15d_b_prereg.md
A	docs/experiments/wp15d_b_prereg_REVIEW.md
```

Three paths, exactly the three the dispatch licenses: the prereg, the new engine
config, and the landed review report. `1207 insertions(+), 31 deletions(-)`.
**No source file, no `tools/` script, no ADR, no other config was touched.** The
instrument (`crates/pistol-search/tests/wp15d_b_measurement.rs`) is untouched, so
its governing revision `70cb580` still holds and §1's pin does not reopen.

**DIGESTS VERIFIED.**

```
$ sha256sum artifacts/wp15d_b_measurement_v1.txt artifacts/wp15d_b_dryrun_v1.txt
46aaf3fbafbc93bb4fca6816c023e6611a21a1fe739871f4b3ad945f78eefe3e  artifacts/wp15d_b_measurement_v1.txt
c318aa225f832f7744bf0c894e6a474636a512b789bdd0010e3a942916750a93  artifacts/wp15d_b_dryrun_v1.txt
```

Both match the dispatch's stated digests exactly.

**WHAT I READ**: `CLAUDE.md` and `docs/process.md` in full; the prereg revision 2
in full; `docs/experiments/wp15d_b_prereg_REVIEW.md` in full; `wp17_sprt_prereg.md`
and `wp16_sprt_prereg.md` as shape precedents; ADRs D-22, D-74, D-88, D-159,
D-175, D-190 (via citation), D-374, D-376, D-377, D-388, D-395, D-401, D-402,
D-410, D-423, D-424, D-427, D-465, D-478, D-482, D-483, D-484;
`crates/pistol-arena/src/{config,validate,openings,sprt,score,conclusion}.rs` and
`src/bin/arena.rs`; both attribution checkers in full at the functions that
compute the registered criterion; `tools/bench_delta.sh`; `tools/config_check.sh`.

**WHAT I RAN**: recomputation of every corpus/spread/calibration/sensitivity
figure from the registered artifact; byte-level `diff` of all three "verbatim"
quotations against their artifact lines; non-comment `diff` of the two engine
configs; arena-schema key-by-key comparison against §4C, including an empirical
validation of §4C's TOML block through the committed `validate_arena_config`
example; `git diff bfdf933..b75c1f6` over the five files §4B pins; and **a
falsification experiment against §4B's agreement criterion**, running BOTH
attribution checkers end-to-end on a deliberately corrupted arena report
(NEW FINDING 1).

---

# CLOSURE LEDGER

| # | finding (abbreviated) | status |
|---|---|---|
| **BLOCKING 1** | SPRT verdict parameters registered nowhere | **CLOSED** |
| **BLOCKING 2** | Criterion 1″, second instrument, agreement criterion, consequence, dry run all absent | **CLOSED BUT INTRODUCED SOMETHING** → NEW 1 (BLOCKING) |
| **BLOCKING 3** | No arena config named or committed | **CLOSED** |
| **MAJOR 1** | §7.3's universal node-count claim false in 14/168 cells | **CLOSED BUT INTRODUCED SOMETHING** → NEW 2 (MAJOR) |
| **MAJOR 2** | §2's falsifiability branch unreachable; §7.2 rests on an inert grid point | **CLOSED** |
| **MAJOR 3** | Verdict space not total — the (1.10, 1.25] band | **CLOSED** (band only; see NEW 3) |
| **MAJOR 4** | IQR gate's 10 % threshold first appears in the results | **CLOSED** |
| **MAJOR 5** | No cost statement | **CLOSED** |
| **MAJOR 6** | Rule 6's reporting requirements never registered or read | **PARTIAL** → NEW 3 (MAJOR) |
| **MAJOR 7** | §7.7's margin argued off the wrong workload | **CLOSED BUT INTRODUCED SOMETHING** → NEW 4 (MAJOR) |
| **MINOR 1** | "worst single median anywhere in the run" overclaimed | **CLOSED** |
| **MINOR 2** | §6.3's unstated denominator | **CLOSED** |
| **MINOR 3** | Revision 1 unreviewed before the run it governs | **CLOSED** |
| **MINOR 4** | "verbatim" quotations reflowed | **CLOSED** |

### Ledger detail

**BLOCKING 1 — CLOSED.** §4A registers `elo0 = 0.0`, `elo1 = 15.0`,
`alpha = 0.05`, `beta = 0.05`, game cap `= openings_take × 2 = 1000 games
(500 pairs)` and a **100-pair floor**, each with the source that fixes it. All
four SPRT keys also appear in §4C's config block. Verified against precedent:
`elo1 = 15.0` matches `wp16_sprt_prereg.md:283` and `wp17_sprt_prereg.md:73`; the
100-pair floor is the standing D-190 convention at `wp16_sprt_prereg.md:297` and
`wp17_sprt_prereg.md:80`. §5 gains a matching row for the floor. Nothing is left
to be authored at launch.

**BLOCKING 3 — CLOSED.** §4C names `configs/arena_wp15d_cap_vs_staged.toml` and
registers every value it will carry. I checked the block key-by-key against the
arena schema at `b75c1f6`: every struct is `deny_unknown_fields` with **no
`Option` and no `serde(default)` anywhere** (pinned by
`crates/pistol-arena/tests/config_tests.rs:54-69`), so all 23 keys are mandatory —
and **all 23 are present, with no unknown key**. Empirically, the §4C block
written verbatim to a file and passed to the committed `validate_arena_config`
example is refused **only** on the `binary_sha256` placeholder, and validates
`ok`, exit 0, once a real 64-hex digest is substituted — which is exactly what
§7.5 defers. `configs/instrument_staged_snk_v0.toml` is committed and differs from
`configs/instrument_staged_v0.toml` on **exactly one non-comment line**:

```
$ diff <(grep -vE '^\s*#|^\s*$' configs/instrument_staged_v0.toml) \
       <(grep -vE '^\s*#|^\s*$' configs/instrument_staged_snk_v0.toml)
11c11
< safety_net_top_k = 0
---
> safety_net_top_k = 16
```

and `16` is the value §2's rule selected (`CAL/SELECTED K=16`). One residual note,
not a finding: `openings_skip + openings_take = 1500 + 500 = 2000` sits exactly on
the 2000-opening book's boundary (`crates/pistol-arena/src/openings.rs:98-108`
refuses `>` total). It passes with zero slack, which §4C's own "the book is nearly
spent" paragraph already records.

**MAJOR 2 — CLOSED, and this is the strongest fix in the revision.** §7.2 does not
soften the withdrawn claim, it deletes it and says so ("**That claim is
WITHDRAWN**"). The replacement argument rests on K = 64, and both its figures
reproduce from the artifact:

```
$ grep '^CAL/SEAT K=64\|^CAL/SEAT K=128\|^CAL/SEAT K=0 ' artifacts/wp15d_b_measurement_v1.txt
CAL/SEAT K=0 mean_depth=2.0740 ... capped_rows=0 ...
CAL/SEAT K=64 mean_depth=2.0880 ... capped_rows=818937 ...
CAL/SEAT K=128 mean_depth=2.0740 ... capped_rows=0 ...
```

`818937` ✓ (and it is the largest `capped_rows` on the grid ✓);
`gain(64) = 2.0880 − 2.0740 = +0.0140` ✓, matching `CAL/SELECTED`'s own
`K64:+0.0140`. **`K_bind` computed from the artifact = 64**, since K = 128 is the
only larger grid point and it has `capped_rows = 0`. §7.2's "`K_bind` (§2) is
therefore 64 on this book" is correct.

**Is §2's rewritten branch reachable?** Yes. The branch fires iff
`gain(K_bind) ≥ 0.75 × max gain`. Because K_bind is by definition a point where
the cap *does* fire, its gain is not identically zero — unlike revision 1's K=128
formulation, which required `0 ≥ 0.75 × max gain`. Had the benefit persisted at
K = 64 the branch would have fired; it did not, because `+0.0140` is far below the
`+0.2730` threshold. The branch is also *consistent* with the main selection rule
rather than a second rule competing with it: grid points above `K_bind` carry
gain 0 and can never clear a positive threshold, so "largest grid point with
`gain ≥ 0.75 × max gain`" and "`K_bind`" coincide precisely when the branch
fires. I recomputed the whole selection by hand and it is correct:
threshold `0.75 × 0.3640 = 0.2730`; K16 `+0.2990 ≥ 0.2730`; K32 `+0.2350 <
0.2730`; largest qualifying point **K = 16** ✓.

**MAJOR 3 — CLOSED for the band.** §3 registers the (1.10, 1.25] band explicitly
("**THE BAND BETWEEN THEM IS NOT A GAP**") and §5 carries it as its own row with
the no-SPRT consequence. The two conflicting instructions revision 1 left are
gone. MAJOR 3's *secondary* "also uncovered" list (degenerate verdicts, arena
exit 1) is not closed and is raised as NEW 3.

**MAJOR 4 — CLOSED.** §3 registers the gate before the results: "the IQR of its
five per-rep times must be **≤ 10 % of that position's own median**", with the
withhold-and-re-measure consequence. The threshold is the real repository
convention — `tools/bench_delta.sh:437-444` ("a spread above 10% of the median is
NOISY … verdict withheld, rerun"). I recomputed the gate over all 168 cells: **0
violations**, matching §7.3. *Residue, not a finding*: the review's secondary
suggestion — one clause noting the gate is vacuous on the 14 cells whose median
is 0 — was not taken. §7.3 does now name those two positions, so the fact is
recoverable.

**MAJOR 5 — CLOSED.** §3 states the measurement run at **MEASURED 2662.25 s**
(matching the artifact header) and the governed run at **ESTIMATED 1.5–3 hours**,
D-291-marked, plus operator attention. *Immaterial note*: the itemisation
(`7 × 1000 + 7 × 24 × 5 + 7 × 4 × 5`) omits the sensitivity pass's 1267 searches,
though the MEASURED total covers the whole run. The cost rule's purpose —
proportion visible on the document's face — is served.

**MINOR 1 — CLOSED.** §7.7 now reads "The worst single median **in the two BENCH
sections**", and states why no CAL maximum exists (the section prints only
per-seat `sum_ms`). I confirmed: `CAL/SEAT` lines carry `sum_ms` and no
per-opening time.

**MINOR 2 — CLOSED.** §6.3 names the denominator ("**4.9 %–5.8 % OF ALL
SEARCHES** — `29/595` and `39/672`") and gives the alternate reading
(`29/125 = 23.2 %`, `39/135 = 28.9 %`). All four recomputed: `29/595 = 4.874 %`,
`39/672 = 5.804 %`, `29/125 = 23.2 %`, `39/135 = 28.89 %` ✓.

**MINOR 3 — CLOSED.** §9 records the sequencing breach on the document's own
face, states the rule it broke, separates mitigation from excuse, and commits a
future (B)-shaped package to reviewing before running.

**MINOR 4 — CLOSED, verified byte-for-byte.** All three "verbatim" quotations are
now byte-identical to their artifact lines:

```
$ diff <(sed -n '42p' artifacts/wp15d_b_measurement_v1.txt)   <(awk '/^CAL\/SELECTED/'   docs/experiments/wp15d_b_prereg.md) && echo IDENTICAL
IDENTICAL
$ diff <(sed -n '242,243p' artifacts/wp15d_b_measurement_v1.txt) <(awk '/^SENS\/TRAJECTORY/' docs/experiments/wp15d_b_prereg.md) && echo IDENTICAL
IDENTICAL
```

---

# VERDICT: **FAIL**

**1 BLOCKING, 3 MAJOR — all four introduced or left open by revision 2 itself.**

I want the shape of this to be plain, because it bears on what the architect
should do with it. **Ten of the fourteen findings are cleanly closed, and two of
the three BLOCKINGs are closed properly and completely.** The revision is not
sloppy work. BLOCKING 3's config block is complete against a schema with 23
mandatory keys and validates empirically; MAJOR 2's fix withdraws a false claim
outright rather than hedging it, and replaces it with a measured argument that
verifies at every step; MINOR 4's three quotations are now byte-identical. The
dry run in §4B is genuine and it did real work — it falsified this document's own
first criterion before a reviewer had to, which is exactly what
`docs/process.md`'s dry-run discipline is for, and it deserves to be recorded as
a success of the process rather than buried under the finding that follows.

It fails on one thing that is structural and three that are not cosmetic:

- **the replacement agreement criterion cannot fail.** Revision 2 correctly
  discarded "both checkers exit 0" — the dry run proved it false by design — and
  replaced it with a criterion that is invariant under every defect in the stage
  it names. I reproduced this: a corrupted arena report carrying a **different
  verdict** yields *identical* criterion terms from both instruments. This is
  the exact failure mode `docs/process.md` names in its own words, and it means
  BLOCKING 2's substantive requirement — a second instrument whose agreement
  could fail — is still unmet.
- **a sixth provenance defect, in the sentence that fixes the fifth.** §7.3's
  new "0.6 %" reproduces from nothing in the registered artifact; the measured
  value is 0.0 %.
- **§5 still cannot route two of the arena's five verdict tokens**, one of which
  MAJOR 6 named explicitly.
- **§7.7's new registered discharge is a gate that can stop the launch, with an
  unspecified input sample** — so its outcome is choosable at launch.

Per the dispatch, this returns the package to the architect.

---

# NEW FINDINGS

## NEW 1 — BLOCKING. §4B's replacement agreement criterion is invariant under the defect class it names: no defect in the arena's seat bookkeeping, pairing, referee or scoring can make its two terms disagree, and the sentence justifying it misdescribes what the instruments compute.

**Claim, verbatim (`docs/experiments/wp15d_b_prereg.md:302-312`):**

> The criterion is stated instead over the
> quantities the two compute INDEPENDENTLY of each other and of the stage under
> doubt:
>
> > **THE AGREEMENT CRITERION.** On the governed report, the two instruments must
> > agree exactly on (i) `1b`'s count of decided non-forfeit games adjudicated
> > against the move list, and (ii) `1c`'s counts of games and of pairs rebuilt off
> > the `score_a` path. Both derive these by re-deriving the outcome from the
> > recorded move list rather than from the arena's own bookkeeping, so a defect in
> > the stage under doubt would have to corrupt both identically to pass.

And the stage under doubt, at `:283-285`:

> **THE STAGE UNDER DOUBT**: everything between the two engine processes and the
> printed verdict — the arena's seat bookkeeping, pairing, referee and scoring.

### The attack

**(a) The justification is false about both terms.**

Term (i) is `judged` in `tools/wp16_warm_attribution_check.py:832-847`. The count
increments *before* the adjudication, and a mismatch goes to `failures`, never to
the count:

```python
for game in report["games"]:
    if game["result"] == "capped" or game["end"] != "normal":
        continue
    judged += 1                                   # <- counted here
    by_p1 = len(report["moves"][game["game"]]) % 2 == 1
    if by_p1 != (game["result"] == "p1_win"):
        failures.append(...)                      # <- the adjudication lands here
notes.append(f"1b: {judged} decided non-forfeit game(s) adjudicated against the move list")
```

So (i) is a **count of report rows passing a filter on the report's own `end` and
`result` fields** — the arena's own bookkeeping, which the document says it is
*not* derived from. The move-list adjudication result is excluded from the number
by construction.

Term (ii) is `len(games)` and `len(buckets)` at
`tools/wp16_warm_attribution_check.py:886`. These are the count of `game` lines
in the report and the count of consecutive pairs of them. **Neither consults the
move list at all.**

**(b) The two instruments are not independent of each other; they are the same
code.** The warm checker's own docstring at
`tools/wp16_warm_attribution_check.py:833` says: *"MOVES -> RESULT, by game rule
3. **Carried from WP-1.5b unchanged.**"* — WP-1.5b being
`tools/wp15b_attribution_check.py`, the second instrument. I diffed the two
implementations line by line: they are the same algorithm, differing only in
hoisting `claimed` into a local and in the note's `game(s)`/`games` wording
(`wp16_warm_attribution_check.py:832-847` vs `wp15b_attribution_check.py:370-385`;
same for 1c at `:850-886` vs `:387-431`). Two components running identical
carried-over code over the same input file cannot disagree except by parsing the
file differently.

**(c) The document states the vacuity as though it were the virtue.** Line 303-304
says the criterion is over quantities computed "INDEPENDENTLY … **of the stage
under doubt**". A quantity independent of the stage under doubt carries no
information about whether that stage is defective. That is the criterion's own
self-refutation on its face.

**(d) `docs/process.md` names this exact construction as a non-criterion**
(`docs/process.md`, "Criterion and defect class"):

> A criterion that is a property the named defect class PRESERVES — **internal
> agreement between components sharing an input**, output shape, plausible
> magnitude, exit status — passes vacuously and is not a criterion; it must be one
> that defect could falsify. … **This binds ANY registered criterion, dry-run or
> governed alike.**

and again in "Cost, replication, and the second instrument":

> two instruments blind to the same stage are one instrument reported twice, and
> **their agreement is invariant under a defect in what they are both blind to**

The registered criterion is the first item on that list verbatim: internal
agreement between components sharing an input.

**(e) Revision 2 removed the only term with discriminating power and excluded the
rest.** The document explicitly rules the cold checker's own independent verdict
out of the criterion (`:319-322`: "The cold checker's own clause-(b) VERDICT is
explicitly NOT an agreement term"), and exit status with it. What remains is the
second instrument's *only* contribution — and it is analytically constant.

### REPRODUCED

I built an arena report carrying a defect in the named stage — **the referee
declares the opposite winner** — and made it internally self-consistent exactly
as a defective arena would: `counts wins_a/losses_a` swapped, every `pair`
bucket mirrored `b → 4−b`, the pentanomial mirrored, and `llr_pair`/`verdict`
recomputed with the checker's own ported `recompute_verdict`. The replay was
re-bound to it by digest, as `arena --replay` would do on a real run.

The defect changes the answer:

```
honest   : llr_pair last -2.991090221   verdict h0
corrupted: llr_pair last  1.719877307   verdict inconclusive_at_game_cap
```

Command and output, warm pass on the corrupted report:

```
$ python3 tools/wp16_warm_attribution_check.py mutated_report.txt mutated_replay.txt target/release/pistol
warm_attribution_check: 1b: 459 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 682 game(s) and 341 pair(s) rebuilt off the score_a path
```

Against the honest report's own recorded figures (`artifacts/wp15d_b_dryrun_v1.txt`):

```
warm_attribution_check: 1b: 459 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 682 game(s) and 341 pair(s) rebuilt off the score_a path
```

And the cold checker — the registered SECOND INSTRUMENT — on the same corrupted
report:

```
$ python3 tools/wp15b_attribution_check.py mutated_report.txt target/release/pistol
attribution_check: 1a: 1364 turns replayed, 78 of them discriminating, 78 of 682 games directly attributed by replay
attribution_check: 1b: 459 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 682 games and 341 pairs rebuilt off the score_a path
```

against its own lines on the HONEST report, from the registered dry-run artifact
`artifacts/wp15d_b_dryrun_v1.txt`:

```
attribution_check: 1a: 1364 turns replayed, 78 of them discriminating, 78 of 682 games directly attributed by replay
attribution_check: 1b: 459 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 682 games and 341 pairs rebuilt off the score_a path
```

**Byte-identical.** So on a report whose verdict the defect changed from `h0` to
`inconclusive_at_game_cap`:

| | honest report | corrupted report | criterion |
|---|---|---|---|
| warm (i) `1b` | 459 | 459 | agrees |
| cold (i) `1b` | 459 | 459 | agrees |
| warm (ii) `1c` | 682 / 341 | 682 / 341 | agrees |
| cold (ii) `1c` | 682 / 341 | 682 / 341 | agrees |

**"The agreement criterion HOLDS" on a run that is not a measurement.** Its
registered consequence ("any disagreement … makes the run NOT A MEASUREMENT")
cannot fire.

**And the signal that does reveal the defect sits in exactly the channel §4B
excludes.** Both instruments detected the corruption — each emitted **459**
`FAIL 1b game …` lines (`grep -c 'FAIL 1b game'` = 459 on both), the warm pass
exiting 1 and the cold `FAIL — 459 failure(s)`. The defect is loudly visible in
the two checkers' FAILURE sets and completely invisible in the two counts the
criterion registers. Revision 2 chose the counts and ruled the failures out.

The general form is stronger than this one instance: because both terms are
computed from the report's own line structure by identical carried-over code,
**no defect whatsoever — in seat bookkeeping, pairing, referee or scoring — can
make the two instruments disagree on them.** The only way the criterion could
fail is if the two parsed the same file differently, which is a claim about the
parsers, not about the arena.

Note honestly what *did* catch this defect: the warm pass exited **1** with 459
`FAIL 1b …` lines, so §5's *separate* "Criterion 1″ fails" row would stop the
run. That is the **first** instrument doing its job. It is not the second
instrument, and BLOCKING 2 was raised because `docs/process.md` requires a second
one whose agreement criterion could fail. As written, the second instrument does
no work at all.

### What would close it

The repair is not to reinstate "both exit 0" — the dry run correctly killed that,
because the cold checker's clause (b) fails by design on this project's runs. It
is to state the criterion over the two checkers' **1b and 1c FINDINGS rather than
their counts**, with clause (b) excluded exactly as §4B already excludes it:

> the two instruments must agree that 1b reports zero move-list mismatches and
> 1c zero rebuild mismatches; the cold checker's clause-(b) verdict is not an
> agreement term.

That is defect-sensitive where the registered version is not: under the mutation
above it moves from 0 to 459 on both instruments, so the criterion fires and the
registered consequence does its work — while a clause-(b) failure beside a
Criterion 1″ pass, the shape the dry run found, still passes.

One caveat worth registering honestly, because it bears on how much a second
instrument can be asked to do here: my mutation left even the cold checker's `1a`
line unchanged (`1364/78/78` on both reports), since a referee that names the
wrong winner does not change which engine moved. So `1a` is not the discriminator
either for this defect class. The quantity that carries the signal is the
mismatch set, and that is what the criterion should name.

---

## NEW 2 — MAJOR. §7.3's newly introduced "0.6 %" reproduces from nothing in the registered artifact; the measured value is 0.0 %. It is a sixth provenance defect, inside the sentence that corrects the fifth.

**Claim, verbatim (`docs/experiments/wp15d_b_prereg.md:530-531`):**

> The two degenerate positions
> contribute 0.6 % of the incumbent's Σ and cannot move a ratio of 1.0015.

**The attack.** Both degenerate positions read `median_ms=0`, with every one of
their five reps `0`, at **every one of the seven seats**. They contribute
**0 ms** of the incumbent's `Σ = 4800 ms` — **0.000 %**, not 0.6 %.

**REPRODUCED.**

```
$ grep -E '^BENCH/CORPUS K=0 p(13|16) ' artifacts/wp15d_b_measurement_v1.txt
BENCH/CORPUS K=0 p13 stones=35 depth=1 nodes=151 median_ms=0 iqr_ms=0 capped_rows=0 reps=[0, 0, 0, 0, 0]
BENCH/CORPUS K=0 p16 stones=31 depth=1 nodes=3   median_ms=0 iqr_ms=0 capped_rows=0 reps=[0, 0, 0, 0, 0]

$ # recomputed over all 168 cells
degenerate positions: ['p13', 'p16'] {'p13': 151, 'p16': 3}
degenerate sum of incumbent medians: 0 = 0.000% of 4800
```

And the string does not occur in the artifact at all:

```
$ grep -n '0\.6' artifacts/wp15d_b_measurement_v1.txt
$ echo $?
1
```

I looked for any quantity the figure could be a mis-transcription of, and found
none: as a share of nodes the two positions are `154 / 1104026 = 0.014 %`; as a
share of positions they are `2/24 = 8.3 %`; as a share of cells, `14/168 = 8.3 %`.
**0.6 % corresponds to nothing in the run.**

**Why this is MAJOR and not MINOR.** The conclusion it supports survives — it
survives *more strongly*, since 0 % cannot move a ratio even more surely than
0.6 % cannot — so under CLAUDE.md's overrule test the claim changes nothing
anyone may conclude. But it is not prose that constrains nothing; it is a
**numeric assertion with no source**, and D-483 is explicit that "every number a
prereg consumes is produced post-implementation by a registered instrument and
cited from that run's artifact by digest". The document opens by declaring
"**No sixth provenance defect was found**" (`:5`) and by pledging that "**NO
NUMBER FROM BEFORE THE SPLIT APPEARS IN THIS DOCUMENT**" (`:37`) — this number is
not from before the split, it is from nowhere, which the same rule forbids for
the same reason. It sits inside the sentence written to fix MAJOR 1, which was
itself a false numeric universal. That is the pattern the package keeps failing
on, reproduced one round later at one remove.

**The fix is one clause**: state the measured 0.0 %, or delete the sentence — the
preceding sentence (per-position node identity across seats, which I verified
holds at all 24 positions × 7 seats) already carries the argument.

---

## NEW 3 — MAJOR. §5's verdict space is still not total: the arena emits five verdict tokens and §5 routes three. `inconclusive_degenerate` — the case MAJOR 6 named explicitly — has no row, and neither does `invalid_forfeit`.

**MAJOR 6's own words, which revision 2 answered only in part:**

> `distinct_n` is not decorative here. … two deterministic seats differing in one
> candidate-policy key **can legitimately play identical games** … A `verdict
> inconclusive_degenerate` with `distinct_n == n/2` is a live outcome for this
> matchup and **§5 cannot route it**. That is a second, independent hole in §5's
> totality.

**What revision 2 closed.** §3 now registers the rule-6 reporting requirements —
`distinct_n` beside `n`, per-side compute from the two `timing_engine` lines, and
the line protocol (D-88) (`:174-179`). That limb is properly closed.

**What it did not.** §5 gained no row for the verdict those reports would signal.

**REPRODUCED.**

```
$ grep -n 'degenerate' docs/experiments/wp15d_b_prereg.md
525:positions are structurally degenerate and terminate early on every seat
530:Σ-median ratio compares time for the same work. The two degenerate positions
```

Both hits are §7.3 discussing *bench positions*. The word never appears as a
verdict. The arena's own token set is five (`crates/pistol-arena/src/sprt.rs:188-193`):

```rust
Verdict::H0                       => "h0",
Verdict::H1                       => "h1",
Verdict::InconclusiveAtGameCap    => "inconclusive_at_game_cap",
Verdict::InconclusiveDegenerate   => "inconclusive_degenerate",
Verdict::InvalidForfeit           => "invalid_forfeit",
```

Both unrouted tokens are reachable on this run, not theoretical.
`InconclusiveDegenerate` is returned whenever `sample.is_degenerate()`
(`score.rs:172-175`) — every pair in one pentanomial bucket, which for two
closely-matched deterministic seats at `turn_cap = 40` is the modal degenerate
shape; WP-1.7's own report shows 223 of 682 games capped, and an all-capped
sample lands every pair in bucket 2. `InvalidForfeit` is returned whenever any
game forfeits (`score.rs:169-171`, D-158) — and §4 registers `hang_timeout_ms`
whose whole purpose is a liveness event that can produce one.

**The precedent shows the cost of the gap and how cheaply it closes.**
`wp17_sprt_prereg.md:131-136` routes `inconclusive_degenerate` in two rows, gives
`invalid_forfeit` its own row ("**The run is not a measurement.** Investigate the
forfeit's cause (D-158)"), and closes the remainder with a single import line:

> | `arena_report_aborted`; a pre-game refusal with NO REPORT AT ALL; Criterion 1''
> fails; the instruments disagree; any checker exit other than 0 | **WP-1.6 §5's
> rows govern, imported by reference and not restated here** |

This document uses import-by-reference correctly elsewhere — §4B does exactly
that for the exit taxonomy, citing D-423/D-424 as the reason not to restate. One
such row in §5 would close this without adding a word of restatement.

**Why it is MAJOR.** The dispatch's own sufficiency test is whether someone else
can "launch, and read the verdict without inventing a decision". If the arena
prints `verdict inconclusive_degenerate`, this document does not say whether the
gate moves, whether the WP closes, or whether the run is a measurement — the
launching session would have to decide after seeing it. Also still uncovered, as
MAJOR 3's secondary list noted: **arena exit 1** (`RUN_FAILED`,
`src/bin/arena.rs:61`) — §5 routes only exit 2 (`REFUSED`, `:63`).

---

## NEW 4 — MAJOR. §7.7's newly registered slot-pass discharge is a gate that can STOP the launch, and its input sample is unspecified — so whether it stops is choosable at launch. Its own cited precedent, D-376, is the counter-example.

**Claim, verbatim (`docs/experiments/wp15d_b_prereg.md:568-572`):**

> **The registered discharge is
> therefore the SLOT PASS**: before launch, the worst single search of the ARMED
> seat over a sample of the book is measured and compared against
> `hang_timeout_ms`, and if the margin is under 24× the launch STOPS and the
> margin is reported to the architect, per D-376's own watchdog rule.

**What is right about it.** MAJOR 7's substance is closed: §7.7 no longer claims
the corpus fixture is "the workload it actually guards", marks 491 ms / 244× as
"indicative and no more", names the observability gap in the instrument, and
records the 11.3× spread margin rather than discarding it. Every figure
reproduces — worst spread median `10596 ms → 120000/10596 = 11.33×` ✓, worst
corpus median `491 ms → 120000/491 = 244.4×` ✓ (both at K=16, `p23` for the
corpus). Adding a real discharge step is the right instinct.

**The attack.** The step is registered as a **gate with a hard consequence** —
"the launch STOPS" — but three of its four inputs are left to the launching
session:

| what D-376 registered before running | what §7.7 registers |
|---|---|
| **the command, verbatim** (`printf 'position start moves <M>\ngo nodes 50000\nquit\n' \| target/release/pistol --config …`) | none |
| **the fixture, sha-pinned**, "all 24 positions … none skipped" | "a sample of the book" — size unstated, membership unstated |
| **the procedure**: "run the 24-invocation sweep TWICE, independently", replication per the cheap-run clause | none |
| **the threshold and consequence**: ~24×, else launch STOPS | ✓ the one thing §7.7 does register |

D-376 is the document §7.7 cites, and it is the project's own demonstration that
a launch-time watchdog probe gets *fully* specified before it runs — down to the
fixture digest and a two-run replication requirement — precisely because it can
stop a run. D-377 then records both runs' worst figures (685 ms / 688 ms) and
applies the rule to the larger.

**Why the gap is load-bearing rather than clerical.** The measured quantity is a
worst-case over a sample, and worst-case-over-a-sample is monotone in the
sample. §7.4's own numbers show the spread across the *stone count* this gate is
sensitive to — 133 ms at 11 stones against 10 573 ms at 99 stones on the armed
seat, a factor of 79 — and the governed workload plays to `turn_cap = 40`,
i.e. into that range. **So the choice of which and how many book openings to walk
determines whether the margin lands above or below 24×, and therefore whether
the launch stops.** A session that samples early positions discharges the gate; a
session that samples deep ones may not. That is an after-the-numbers degree of
freedom on a registered gate — the thing §2 of this same document refuses two
sections earlier, in its own words, when it declines a `"reaches depth ≥ n"`
channel because `n` "would need … data D-482 makes inadmissible".

There is a second, smaller gap in the same step: the document says the CAL
section "emitted no timing there — an observability gap in the instrument", and
§1 pins the instrument at `70cb580` with "a change to it reopens this review".
So the discharge cannot be taken with the registered instrument as it stands, and
§7.7 does not name what else takes it. D-376's answer was a raw binary invocation
with no `tools/` wrapper, explicitly so that no new instrument owing
`SHELL_CHECKLIST.md` coverage is created — that path is available here and would
cost one paragraph.

**Fix:** register the command, the sample (which openings, how many), the config,
the budget, and whether it replicates — D-376's five fields — or delete the 24×
gate and record the margin as a reported finding instead of a stop condition.

---

# WHAT I CHECKED AND FOUND SOUND

**This section is the larger part of the work and should be weighed as such.**

### No number in the document is misattributed, with the single exception of NEW 2

I re-verified **every** figure revision 2 carries, not only the ones the dispatch
named. All of the following reproduce from
`artifacts/wp15d_b_measurement_v1.txt` and from nowhere else:

| figure | where | verified |
|---|---|---|
| `CAL/SELECTED` line, whole | §7.2 | byte-identical to artifact line 42 |
| `base_mean=2.0740`, `best_gain=+0.3640`, `threshold=+0.2730` | §7.2 | ✓, and `0.75 × 0.3640 = 0.2730` exactly |
| every gain K4…K128 | §7.2 | ✓ each equals `mean(K) − 2.0740` from the `CAL/SEAT` lines |
| K = 16 selected, K32 excluded | §7.2 | ✓ recomputed by hand |
| `capped_rows=818937` at K = 64 | §7.2 | ✓, and it is the grid maximum |
| `gain(64) = +0.0140` | §7.2 | ✓ |
| `K_bind = 64` | §7.2 | ✓ derived independently from `capped_rows > 0` |
| K=128 ≡ K=0 on mean, histogram and every counter | §7.2 | ✓ |
| corpus Σ medians 4800 / 4807, ratio 1.0015 | §7.3 | ✓ `4807/4800 = 1.001458` |
| "false in **14 of the 168 cells**" | §7.3 | ✓ `7 × nodes=151`, `7 × nodes=3`, `154 × nodes=50176` |
| `p13 = 151`, `p16 = 3` | §7.3 | ✓ |
| node count identical across all seven seats, per position | §7.3 | ✓ checked at all 24 positions — this is the premise the like-for-like reading needs, and it holds |
| IQR gate, 0 of 168 | §7.3 | ✓ recomputed |
| spread table, all 12 data cells | §7.4 | ✓ every cell (0/95, 0/71, 0/152, 0/0; 133→133, 449→470, 2761→2748, 10596→10573) |
| "every seat reads `depth=1` at every position" | §7.4 | ✓ all 28 `BENCH/SPREAD` lines |
| 10 596 ms, 11.3× | §7.7 | ✓ `120000/10596 = 11.33` |
| **491 ms, 244×** | §7.7 | ✓ worst corpus median is 491 (K=16, p23); `120000/491 = 244.4` |
| both `SENS/TRAJECTORY` lines | §6 | byte-identical to artifact lines 242–243 |
| **`29/595` = 4.87 %, `39/672` = 5.80 %** | §6.3 | ✓ |
| **`29/125` = 23.2 %, `39/135` = 28.9 %** | §6.3 | ✓ |
| bearing 125/595 = 21.0 %, 135/672 = 20.1 % | §6.1-6.2 | ✓ |
| 2662.25 s, exit 0, instrument `70cb580`, tree `4ec470f` | §7.1 | ✓ artifact header |
| `CAL/EXCLUDED … =0 indices=[]` | §2 | ✓ the excluded set is empty and is the union form, printed once |

### The dry run is real, and its recorded figures reproduce

`artifacts/wp15d_b_dryrun_v1.txt` verifies at the dispatch's digest. Its input is
WP-1.7's own preserved governed report and replay — a real instance of the kind,
never the governed workload, exactly as `docs/process.md`'s dry-run discipline
requires. The four figures §4B quotes from it are in the file verbatim
(`1b: 459` both, `1c: 682 / 341` both), and the honest run's warm pass reproduces
WP-1.7's own recorded `h0` at 341 pairs — which is the defect class §4B names
(a checker that cannot read this arena's report at all) genuinely excluded, since
a checker reading the wrong fields could not land on the right answer.

**And the dry run did the thing dry runs exist to do.** It falsified the
document's first criterion before a reviewer had to. §4B records that plainly
(`:356-364`) rather than quietly swapping the criterion. That is the process
working, and NEW 1 is a finding about the replacement, not about the dry run.

### Every revision pin in §4B is correct

| pin | claim | verified |
|---|---|---|
| warm replay at `a14912a` | "the last commit that touched the crate" | ✓ `git log -1 -- crates/pistol-arena/` → `a14912a` |
| the `bfdf933..HEAD` difference | "**181 deletions and 0 insertions**" | ✓ exactly: 30+57+30+33+31 = 181, all `0` insertions |
| "all of them the `//!` header sweep (D-443)" | | ✓ the commit is the CLAUDE.md restructure + file-top header sweep |
| "the only non-comment change anywhere in the crate since is three lines in a TEST STUB engine (`FirstLegal`)" | | ✓ `stub_engine.rs` is `3 25`; every other file in the crate is `0 <n>` |
| statistics layer at `6c929da` | last commit touching it | ✓ |
| second instrument at `a80a864` | last commit touching it | ✓ |
| exit taxonomy `0/1/2/3` at `:133-136` | | ✓ constants present as stated |
| instrument at `70cb580` | last commit touching it | ✓, and untouched by this diff |

Criterion 1″ is quoted in full and without ellipsis against
`docs/experiments/wp16_warm_replay_design.md` §4 point 4, as the precedent
requires. The cold checker genuinely does not share the *warm replay's* seat
bookkeeping — that part of §4B item 3 is true; NEW 1 is that the *criterion* does
not exploit the independence.

### The seat assignment, the slice accounting, and the disjointness

- **Engine A is the capped seat**, stated twice with the reason (the arena's
  statistic is slot A's score, so H1 "the capped seat is stronger" needs A
  capped). I confirmed the arena scores slot A (`score_a` in
  `crates/pistol-arena/src/score.rs`), so the sign is right.
- **Slice accounting is correct.** `0..499` (WP-1.5b), `500..999` (WP-1.6),
  `1000..1499` (WP-1.7 — confirmed against
  `artifacts/wp17_governed_run_v1.txt`'s own `openings_skip 1000`), this run
  `1500..1999`. Disjoint by construction, and `openings_skip = 1500` is fixed in
  the document rather than at launch, which is D-427's own lesson.
- **The calibration sample and the verdict sample are disjoint.** Calibration ran
  openings `0..999` (`population=1000`), the SPRT draws `1500..1999`. So K is
  chosen on a different sample from the one the verdict is read on — the
  document's claim, and it holds.
- **The book-exhaustion note** is honest and correctly placed.

### The calibration's structure

- The **channel is threshold-free** and the reason given (a `"reaches depth ≥ n"`
  count would need an `n` chosen from inadmissible data) is sound and is the same
  standard NEW 4 finds §7.7 falling short of.
- The **selection rule runs inside the instrument** (`fn selection`, `70cb580`)
  and prints `CAL/SELECTED`, so the choice is not a human step after seeing the
  table — I confirmed the printed line is the rule's own output and recomputed it
  independently.
- The rule **cannot select a grid extreme by construction**, as claimed: a
  channel-maximising rule would take the smallest K, a cost-minimising rule the
  largest, and the 0.75 rule took an interior point (K = 16).
- The **excluded set is the union over all seats**, computed once and printed by
  index — the D-395 precedent — and is empty on this run, so per-seat
  denominator shrinkage cannot have occurred.
- `quiet_radius` **pinned at 2 with a stated attribution reason**, and the one
  changed key is verified to be the only difference between the two seat configs.

### The bench registration

Directions are stated in the repository's own convention (time-to-depth ON/OFF,
larger is worse, and it is the gate; `nps` explicitly *not* a gate across seats
with different candidate policies, per D-374). The corpus is registered as a
no-regression check rather than a place a gain is expected. The spread fixture is
reported-not-gated with the reason stated so the report cannot be read as a gate,
and the D-95 debt is correctly left where D-478 put it rather than claimed
discharged. The abort's consequence (no SPRT, package closes on the bench) is the
WP-1.8c shape (D-465) and is routed in §5.

### §9's recorded breach

§9 is the right disposition for MINOR 3: it puts the sequencing failure on the
document's face, quotes the rule it broke, and separates what held (commit clock,
selection-rule-as-code, no threshold moved — all three of which I re-verified
independently) from an excuse. I checked the commit clock myself: the instrument
`70cb580` and the prereg's tree revision `4ec470f` both precede the run recorded
in the artifact header, and no registered threshold in revision 2 differs from
revision 1 except where the review demanded it.

---

# WHAT WOULD FLIP THIS TO PASS

Four edits, none of which requires a new measurement, and none of which touches
the run that has been taken:

1. **NEW 1** — restate the agreement criterion over quantities the two
   instruments compute by *different mechanisms* (the warm pass's `W coverage` /
   `W classification` against the cold checker's `1a` line, or a recomputed
   verdict token), so that a defect in seat bookkeeping, pairing, referee or
   scoring could make them disagree. Keep the registered consequence as it
   stands — it is correct; it is the criterion it attaches to that cannot fire.
2. **NEW 2** — replace "0.6 %" with the measured 0.0 %, or delete the clause.
3. **NEW 3** — add §5 rows for `inconclusive_degenerate` and `invalid_forfeit`,
   plus one import-by-reference row for the remaining arena exits, in
   `wp17_sprt_prereg.md:136`'s style.
4. **NEW 4** — register the slot-pass probe's command, sample and procedure at
   D-376's level of specificity, or demote the 24× stop-condition to a reported
   finding.

Nothing in the calibration, the bench, the selection of K = 16, the seat
assignment, the slice, the config, or the artifact's numbers needs to change.
The measurement run stands, and NEW 1–4 are all defects of the *document's*
registered decision procedure rather than of the data it rests on.

---

**Worktree created and left in place, per the dispatch:**
`/home/tom/Projects/pistol-wt-pr2` (detached at `b75c1f6`, own
`CARGO_TARGET_DIR`, on `/home`). The previous reviewer's
`/home/tom/Projects/pistol-wt-pr` was read but not modified or removed. No file
in the live tree was edited other than this report.
