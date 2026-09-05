# WP-2.2 Phase 1 design — REVIEW-design, revision 1.

**Document reviewed**: `docs/experiments/wp22_phase1_design.md`, its own first
line stating *revision 2*.
**Named revision**: `15569b1` (`dev`). **It matches HEAD**, and the working tree
was clean at the start of this review (`git status --porcelain` empty).
**Reviewer**: fresh context, not the author, not the implementer.

**VERDICT: FAIL.** The arithmetic is sound and reproduces byte-for-byte, and the
premise the whole phase rests on survives a much harder attack than the one that
was run — but four things block the registered SPRT: §7 registers no SPRT
bounds and so cannot return a verdict (**M-1**) and leaves its openings source
to be chosen at run time (**M-2**); the phase's actual result, D-616's
directional mechanism, is contradicted by the corpus and the design's §8
successor is named on it (**M-3**); and the "exact constrained solve" §2
advertises is exact only for the gap constraints, projecting at the bounds —
inert on the data as fitted, and NOT inert on the successor §8 names (**M-4**).

**What this does NOT say.** No finding here says a shipped number is wrong. The
fitted table `[1, 21, 22, 64, 65]`, the 4.6x, the 1873/3882, the 0-of-500 oracle
and the diagnostics all reproduce. The FAIL is about what the document licenses
a successor to conclude and to run next.

---

## Re-derivation ledger

Every number below was re-derived with commands **this reviewer chose**, at a
scope the author's commands did not have (process.md, *Re-derivation*). Scratch
harnesses live in this session's scratchpad and are named where they matter.

| # | claim | how re-derived (scope chosen here) | reproduced? |
|---|---|---|---|
| 1 | 89 805 deduped positions | `grep -vc '^#' artifacts/arc3r_sweep_deduped_manifest.txt` = 89 819 − 14 headers | **YES** |
| 2 | extract writes 74 672 rows, skips 15 133 | re-ran `tools/texel/extract.py` from the sixteen tranche corpora into a fresh path | **YES** |
| 3 | the row file is a pure function of its inputs | `sha256sum` of my rows file = `1489f43b…`, equal to `artifacts/wp22_phase1/texel_rows_identity.txt` | **YES, byte-identical** |
| 4 | every fit diagnostic in `fit_diagnostics.txt` | re-ran `tools/texel/fit.py` on my own rows file | **YES, line-for-line** |
| 5 | eval rows: 2 009 (2.69 %) carry a five-window; 29 165 (39.06 %) a four-window | own probe recomputing features from the corpus, counting **per-side** five-window counts as well as the signed difference | **YES** |
| 6 | mate rows: 15 133 = 8 074 `mate_in` + 7 059 `mated_in`; 1 873 (12.38 %) carry a five-window | same probe, mate rows included (the shipped `extract.py` drops them, so this is a different instrument) | **YES** |
| 7 | **4.6x** and **1 873 / 3 882 = 48 %** | ratio of the two rates above; and the same figures recomputed over *positions holding a five-window on either side* rather than over `f5 != 0`: **1 877 / 3 886 = 48.3 %** | **YES, and robust to the sharper definition** |
| 8 | the oracle: extractor value == `HandcraftedV0::value` | **7 478 positions, not 500**: all 3 886 five-window positions (mate rows included) + a stride-25 draw of 3 592 across all sixteen tranches and all three score kinds, against `target/release/examples/static_eval` | **YES — 0 disagreements** |
| 9 | the identity holds **at and beyond the clamp** | the same 7 478 positions re-run at `[1,10,100,2000,15999]` and `[15995,…,15999]`, forcing 3 503 / 3 664 / 127 saturating positions | **YES — 0 disagreements** |
| 10 | "a 1-point perturbation kills 410 of 500" | per-entry kill rates on my stride sample: `table.1` 98.2 %, `.2` 92.3 %, `.3` 79.4 %, `.4` 43.0 %, **`.5` 3.9 %** | **shape reproduced; see M-4** |
| 11 | loader test 1 — inert to presentation | ran `target/release/pistol` under `configs/instrument_v0.toml` with `weights_file` redirected to `artifacts/wp22_phase1_loader/same_values.toml`, `go depth_turns 4`, `nps`/`time` stripped | **YES — byte-identical, nodes 1891 / 6545 / 150634 / 383891** |
| 12 | loader test 2 — not inert to values | same, `perturbed.toml` | **YES — differs from depth 1** |
| 13 | the fitted table changes play | same, `artifacts/wp22_phase1_weights/eval_v0_texel_fit.toml` | **YES — `bestmove 1,1/5,1` vs committed `-1,1/0,1`** |
| 14 | `weights_file` is a required key with no code-side default | `crates/pistol-engine/src/config.rs:296-304` — `#[serde(deny_unknown_fields)]`, `pub weights_file: PathBuf`, no `serde(default)` | **YES — §3's premise holds** |
| 15 | all three artifact receipts | `sha256sum -c RECEIPT.sha256` in each of the three directories | **YES — 14 of 14 OK** |
| 16 | CI gate 18 | `bash tools/texel_tests.sh` → `test_texel: all checks passed (including census_classes)`, `EXIT=0` | **YES** |
| 17 | citation gate | `bash tools/governing_citation_check.sh` → `wp22_phase1_design.md: 10 citation(s) checked, 0 unreproduced` | **YES** |
| 18 | rows whose label saturates the band | own probe over all 74 672 rows: **0**; max `|label|` = 5 514 | **the rule is inert — see M-5** |
| 19 | rows whose *prediction* saturates | own probe, committed table **0 / 74 672**, fitted table **0 / 74 672** | **new measurement — see M-5** |
| 20 | validation slice | own count: 9 406 / 74 672 = **0.1260**, i.e. 1-in-8 | **design correct, `fit.py` comment wrong — see m-3** |
| 21 | sign structure of the top two regressors | own probe over all 89 805 positions: mover-relative `k=5` **never positive**; `k=4` positive in 1 593, all of them mate rows | **new measurement — see M-3** |
| 22 | the censored surrogate | own refit entering mate rows at the band edge, `±16000` | **new measurement — `w5` goes further negative, see M-3** |

---

## MAJOR findings

### M-1. §7 registers an SPRT that cannot return a verdict.

**What is wrong.** §7 (lines 121-130) fixes seats, a node budget and a reporting
list. It fixes **no `elo0`, no `elo1`, no `alpha`, no `beta`, no maximum n and
no stopping rule.** Those four are not optional: the arena's config schema
*requires* them — `configs/arena_wp20_label_pilot.toml` carries an `[sprt]`
block with all four and says in its own comment *"Present because the schema
requires them"*. §7 also fixes no `openings_skip`/`openings_take`, no
`turn_cap`, no `n_workers` and no `hang_timeout_ms`, and names no arena config
file. It states no COST and records no DRY RUN, both of which `docs/process.md`
makes binding on a pre-registration (*Cost, replication, and the second
instrument*; *Dry-run discipline*).

**Why it matters.** "Reported: n, distinct_n, pentanomial, llr_pair, per-side
compute" is a reporting list, not a decision rule. A run whose bounds are chosen
after the seats are built is the post-hoc threshold move CLAUDE.md's Process
section forbids. As written, whoever runs this picks `elo0`/`elo1` at run time,
and the verdict is theirs rather than the registration's.

**Minimal fix.** §7 states `elo0`, `elo1`, `alpha`, `beta`, the maximum pair
count, the arena config's path, and the wall-clock cost; and it records a dry
run on an opening slice of the same kind that is not the registered slice.

### M-2. §7's openings source is undetermined, and the ledger's own rule is unmet.

**What is wrong.** §7 line 127 says *"the holdout openings ledger; if the
holdout is thin, a `book_v3` slice is registered and generated first"*. **"Thin"
has no threshold**, so the openings source is decided at run time. The holdout
is `docs/book_v2_ledger.md`'s third row — `openings_skip 3500`, `openings_take
1000` (line 43) — and that ledger states a rule the design does not meet: *"A
new pre-registration takes the next unconsumed range, **adds its row here in the
same commit that adds its arena config**"* (lines 16-19). The ledger also lists
two standing claimants on those 1 000 openings and says explicitly that
*"dividing it between them is not decided here"* (lines 66-68). Phase 1's SPRT
is a third claimant that appears nowhere in that document.

**Why it matters.** A consumed range is spent whether or not the run finishes.
An SPRT that draws from `3500..4499` without a ledger row silently spends part
of a reservation two other packages are counting on, and a successor reading the
ledger will believe the range is untouched.

**Minimal fix.** §7 names the exact `openings_skip`/`openings_take`, and the
commit that lands the arena config also lands the ledger row. Delete the
"if the holdout is thin" clause: 1 000 openings is larger than the 600-pair cap
the optimization arc's own runs used (`docs/ROADMAP.md:499`), so the branch
protects against nothing and only licenses a later choice.

### M-3. §8's "the exclusion drops the rows where the window CONVERTED" is measurably false in its direction — and it is the claim §8's successor is named on.

**What is wrong.** §8 (lines 146-153), `wp22_phase1_fit_finding.md:71-79` and
D-616 all say the same thing three ways: the excluded mate rows are *"dropped on
exactly one side of the question: the side where the five-window converted"* /
*"REMOVES THEM ON THE SIDE WHERE SUCH A WINDOW WON"*, so *"`w5` is estimated
from 2 009 positions selected for having a nearly complete line that did NOT
win"*. `docs/ROADMAP.md:472-473` repeats it. **Measured, over the corpus:**

- **Every one of the 3 886 positions holding a five-stone window has it owned by
  the side NOT to move** (2 009 eval + 1 866 mate single-owner + 11 both-sides;
  0 positions where the mover owns one). Over **all 89 805** deduped positions
  the mover-relative `k=5` regressor is `< 0` in 3 882 and `> 0` in **zero** —
  it has no positive variation anywhere in the corpus, mate rows included.
  `k=4` is one-sided too, but only in the rows the fit KEEPS: across all 89 805
  positions it is positive in 1 593, and **every one of those 1 593 is a mate
  row the exclusion drops**, leaving 29 165 negative and zero positive in the
  74 672 rows fitted.
- Of the **1 873** discarded mate rows carrying a five-window, **1 346 (71.9 %)
  are `mate_in`** — the side to move, which does *not* own the window, is the
  one that mates — and only **520 (27.8 %)** are `mated_in`. Spot-checked: the
  first three are `mate_in 1` positions where the opponent holds two or three
  unconverted five-windows.

So the majority of the removed rows are exactly the case the finding says they
are not: a nearly complete line that **did not win**. The direction is reversed.

**Why it matters, and this is the whole reason it is MAJOR.** §8 registers the
censored likelihood as the successor *because* the dropped rows are supposed to
carry the converting evidence that would lift `w5`. Run as a crude censored
surrogate — mate rows entered at the band edge, `±16000`, which is an upper
bound on the censoring effect and not a Tobit fit — the corpus says the
opposite:

```
eval rows only (as shipped)   unconstrained w = [ 1.45, 22.53, 17.91,   68.93,    -2.64]
+ mate rows at +/-16000       unconstrained w = [-70.59, 30.32, -57.85, 1666.43, -2085.39]
```

`w5` does not rise above `w4`; it goes **further negative**, and the 4-5 gap
still binds in the constrained answer. Including the mate rows moves the SCALE
(by a factor of ~22) and leaves the flat top exactly where it was. **The named
successor addresses the 48 % count and not the mechanism.** For `w4` the
censoring story is exactly right — the 1 593 positions where the mover owns the
four-window are ALL mate rows, so the exclusion removes 100 % of that
regressor's positive variation and a censored likelihood restores it. For `w5`
it is not: no position in the corpus, of any kind, has the side to move owning a
five-stone window, so `w5` has no positive variation to restore and the
one-sidedness survives the fix §8 names.

**Minimal fix.** Correct the directional sentence in §8, in
`wp22_phase1_fit_finding.md` and in the D-616 amendment to what is measured —
the exclusion is non-random in the *label* (it removes every decided position),
not in the *owner* — and state the one-sidedness of the `k=4`/`k=5` regressors
as the finding that constrains the successor. A censored likelihood may still be
the right next step; it may not be named on a mechanism the corpus contradicts.

### M-4. The "exact constrained optimum" is exact only for the four gap constraints; the two BOUNDS are still handled by the projection §2 says was removed.

**What is wrong.** §2 lines 55-66 replace revision 1's projection with
*"exhaustive enumeration of the sixteen active sets"*, each *"solved exactly"*.
`tools/texel/fit.py:110-141` enumerates the sixteen **gap** patterns — and then,
at line 136, applies `w = [max(1.0, min(float(EVAL_MAX - 1), x)) for x in w]`.
That is a projection onto the box `w1 >= 1`, `w5 <= 15999`, applied *before* the
feasibility test, silently, with no report that it fired. It is the same defect
the same paragraph declares fatal: *"a projected point can sit on a face the
true optimum never touches"*.

**Minimal reproducer** (`repro_bound.py`, synthetic rows generated from
`[-40, 20, 90, 400, 2000]` so the data wants `w1` far below the bound):

```
unconstrained : [-40.0000, 20.0000, 90.0000, 400.0000, 2000.0000]
fit.py answer : [  1.0000, 20.0000, 90.0000, 400.0000, 2000.0000]  objective -3.548877e+10
true optimum  : [  1.0000, 20.6474, 88.9504, 396.6478, 1987.0789]  objective -3.549038e+10
rounded: fit.py [1, 20, 90, 400, 2000]   vs true [1, 21, 89, 397, 1987]
```

**Why it matters.** On the corpus as fitted the bound is slack (`w1 = 1.4241`),
so **Phase 1's published table is unaffected** — this is a latent defect, not a
wrong number. But it is latent only for the objective §2 registers. Under the
censored surrogate of M-3 the bound **binds immediately**, and the shipped
routine then returns a point that is measurably not the optimum:

```
shipped answer  [1.0000, 15.6219, 16.6219, 1439.4750, 1440.4750]  objective -3.037968e+11
w1 in the active set
                [1.0000,  2.0000,  3.0000, 1624.4168, 1625.4168]  objective -3.146556e+11
```

Two different weight tables. So the defect sits directly on the critical path of
the successor §8 names, and `test_texel.py` does not cover it:
`test_constrained_fit_respects_the_schema` asserts feasibility, not optimality,
and `test_constrained_beats_projection` pins the **gap** projection only.

**Minimal fix.** Either enumerate the bounds as active-set members too (16 gap
patterns x the two bounds = 64 candidates, each an equality-constrained solve),
or — the smaller change, and the one hard rule 3 asks for — make `fit.py`
REFUSE when the clamp at line 136 alters any component, rather than returning a
projected point labelled *"constrained optimum"*. Add a test in the shape of the
reproducer above. The same paragraph's `if z is None: continue`
(`fit.py:133-134`) is a second silent skip and should name itself too.

### M-5. §4's correctness gate has no shipped instrument, no registered sampling rule, and no receipted artifact — and the sample it was run on had almost no power over the parameter the phase is about.

**What is wrong.** Three separate things, all in §4 (lines 83-97):

1. **The instrument does not exist.** `tools/texel/features.py:5` names
   `verify_against_engine` as *"what says so"*. `grep -rn verify_against_engine`
   over the whole tree returns that one docstring line and nothing else. The
   oracle was an ad-hoc harness that was never committed and is not in
   `git log --diff-filter=D`. `docs/process.md`'s *Instrument governing
   revision* clause covers exactly this: *"a `tools/` script, a scratchpad
   harness, or a command block the document prints — is named in the
   pre-registration WITH ITS REVISION"*.
2. **The run has no artifact.** `artifacts/wp22_phase1/RECEIPT.sha256` covers
   five files — `extract.log`, `fit_diagnostics.txt`, `mate_row_probe.txt`,
   `texel_rows_head.txt`, `texel_rows_identity.txt`. **There is no oracle log.**
   The `0 of 500` / `410 of 500` numbers are consumed by §4's own gate, by
   `wp22_phase1_fit_finding.md:14-16`, by D-616 and by `docs/ROADMAP.md:464-467`,
   and are cited from no artifact by digest. D-483 requires the opposite.
3. **The draw is unregistered, and the default draw is nearly blind to `w5`.**
   §4 says only *"a registered sample of corpus positions"* — no size, no
   stratum, no clamp case. Measured on a representative draw, a `+1`
   perturbation of `table.5` disagrees on **3.9 %** of positions, against 79.4 %
   for `table.3`. Only 2.69 % of eval rows carry a five-window at all, so an
   unstratified 500-draw exercises the `f5` path in roughly **13** positions —
   for the one table entry the entire phase's finding is about. An oracle drawn
   that way passes whether or not the extractor's `f5` bookkeeping is right.

**Why it matters.** §4 is the gate everything else rests on: *"A single mismatch
stops the phase."* A gate whose instrument is not in the tree cannot be re-run
by a successor, cannot be reviewed, and cannot be shown to have been run at all.

**Why this is a process finding and not a correctness one.** I re-ran the oracle
at 15x the size and at strata the original could not have contained — all 3 886
five-window positions including the 1 877 mate ones, plus a stride-25 draw of
3 592 across all sixteen tranches and all three score kinds, plus two
clamp-forcing weight tables that saturate up to 3 664 of them — and got
**0 disagreements everywhere**. The extractor is right. The gate is unreceipted.

**Minimal fix.** Ship the oracle as `tools/texel/verify_against_engine.py` (the
name the code already promises) with a test driving it, register the sample rule
in §4 — *every* position holding a four- or five-stone window, plus a stride
draw, plus at least one weight table that saturates the clamp — and receipt the
run in `artifacts/wp22_phase1/`.

### M-6. §5's human-corpus holdout is blocked by the ROADMAP and by D-453.

**What is wrong.** §5 lines 107-111 use the human corpus `b2fe61eb…` for
"outcome correlation"; premise §7 calls it *"a DISTRIBUTION holdout … so it
answers whether a fit to engine labels transfers"*. `docs/ROADMAP.md:530-532`
says, in terms: *"**D-434's Stage-2 Texel-style calibration and independent
holdout are BLOCKED** until a POPULATION-GRADE corpus supersedes it"*, and D-453
says ARTIFACT-GRADE licenses statements *about the artifact* and nothing
*"generalizing to the platform's players"*. "Whether a fit transfers to human
play" is exactly such a generalization.

**Why it matters.** §5 cites D-453 by name while making the use D-453 forbids.
Nothing gates on it (D-614), so the cost is that a successor reads a licensed
holdout where there is none.

**Minimal fix.** Delete the human-corpus paragraph from §5, or restate it as a
statement about the artifact with an explicit line that it licenses no transfer
claim. Either way the design should say why it is not the blocked use.

### M-7. The design breaches D-483 in §2 while invoking D-483 in §5.

**What is wrong.** §5 line 104 says the diagnostics *"live in artifacts, never
in a document (D-483)"*. §2 lines 60-66 carry two measured weight vectors
(`[1,22,23,69,70]` against `[1,21,22,64,65]`) and the measured claim that *"two
of the four do bind"*. D-483 is unambiguous: *"design documents carry no
measured numbers — mechanisms, invariants and tests only"*.

**Why it matters.** Small in itself, but it is the fifth-instance standing cure
the document cites against itself, and a design that carries its run's numbers
is the shape D-483 exists to stop: the next revision argues from them.

**Minimal fix.** Replace both vectors with a pointer to
`wp22_phase1_fit_finding.md` and `artifacts/wp22_phase1/fit_diagnostics.txt`.
Keep the MECHANISM sentence ("a projected point can sit on a face the true
optimum never touches"), which is what §2 needs.

### M-8. §2's stage list still describes revision 1, and so does the shipped script's own docstring.

**What is wrong.** §2 line 40 still reads *"`tools/texel/fit.py` — the
closed-form solve, **the projection onto the schema's constraint set**, and the
diagnostics"* — the mechanism lines 55-66 of the same section say was replaced.
`tools/texel/fit.py:5-6` is worse: *"Where the constraint set binds, a
deterministic projected descent finishes from the committed weights"* describes
neither what ships nor what revision 1 did.

**Why it matters.** CLAUDE.md's code-style rule that a comment saying the
opposite of the code is a defect (the class D-578 records at A-04), and D-423:
a claim the document makes twice is a defect waiting — here the two statements
have already diverged.

**Minimal fix.** One-line edits to both.

---

## MINOR findings

- **m-1. The stated governing revision predates the instruments.** Line 3 says
  *"Governing revision: `71fa6f1`"*. `git ls-tree -r 71fa6f1 -- tools/texel/`
  returns **nothing** — none of the three scripts §2 names exists at that
  revision. The document itself was last edited at `162608b`, also after
  `71fa6f1`, and the two run receipts were taken at `06efd52` and `29c954d`,
  later still. Fix: state the revision that actually governs, and name each
  instrument with its own revision per `docs/process.md`.
- **m-2. `D-55y` (§3 line 81) is not a decision key.**
  `docs/experiments/wp20_dispatches.md:559-565` says so in terms and resolves it
  to **D-553**, the call-site mutant law. This is the class D-565 and D-611 both
  ruled on. Fix: cite D-553.
- **m-3. `fit.py:32` says the split is "1-in-16"; it is 1-in-8.** The code tests
  `r[6][-1] in "01"` — two of sixteen hex digits. Measured 9 406 / 74 672 =
  0.1260. The design (line 52) has it right; the shipped comment does not.
- **m-4. §3 calls two receipted manual runs "tests".** Nothing in the tree
  re-runs them: `grep -rn weights_file crates/*/tests` finds only missing-file
  and empty-path refusals (`protocol_tests.rs:263`,
  `config_validate_tests.rs:113-130`), and `eval_weights_tests.rs` tests the
  loader's validation, not the engine's sensitivity to the file's *values*. Both
  properties reproduce when run by hand (ledger 11, 12), but a regression that
  made the engine ignore `weights_file` would pass CI. D-553 asks for a test that
  drives the call site. Fix: land both as behavior-named tests, or say plainly in
  §3 that they are a receipted run and not a gate.
- **m-5. §5 promises two diagnostics the instrument does not produce.** There is
  no by-depth breakdown (`fit.py:76` reads `_depth` and discards it) and no
  human-corpus outcome correlation anywhere in `tools/`. Fix: produce them or
  drop them from §5.
- **m-6. §1's "changes … no gate" is stale at HEAD.** `tools/ci.sh:189-190` is
  `gate 18/21: offline texel and census tooling`, and it runs
  `tools/texel_tests.sh`, half of whose checks are Phase 1's fit.
- **m-7. Premise §4's mismatch citation points at the wrong code.** The table row
  *"the arena REFUSES a mismatch (`crates/pistol-arena/src/handshake.rs:116-157`)"*
  is not what that span does: `handshake.rs:116-164` refuses a **duplicated or
  malformed** weights line and checks nothing across seats. The refusals of a
  *mismatch* are `capture.rs:145-163` (a capture's two seats must attest one
  engine) and `replay.rs:228-250` (a replay against its source report). **This is
  good news for §7** — the SPRT's two seats carrying different tables is *not*
  refused, which I verified by running both — but the premise as written would
  tell a successor the run is impossible.
- **m-8. Premise §4's `pistol.rs:88` is a blank line** at `71fa6f1` and at HEAD;
  the `weights_file` read is line 90. (The `weights_sha256` citation, line 97,
  is exact.) `wp22_phase1_premise.md` and `wp22_phase1_fit_finding.md` are **not
  on `tools/governing_citation_check.sh`'s list** (only the design is, line 59),
  so the memo carrying the phase's load-bearing quotations rots unchecked.
- **m-9. `rank_correlation` uses the no-ties Spearman formula** on data with many
  tied predictions (e.g. every all-zero feature row). Deterministic, and it
  gates nothing, but the reported ρ is biased. Diagnostic only.
- **m-10. The train/validation split is per-position, not per-game or
  per-opening.** Positions from one game land on both sides, so premise §7's
  claim that the split *"answers overfitting alone"* overstates it. With five
  parameters and 65 266 rows overfitting cannot arise, which is the honest thing
  for §5 to say instead.

---

## QUESTIONS (unsubstantiated — flagged, not asserted)

- **Q-1. Is the "binding constraint" finding partly a scale artifact?** The
  schema's gaps are ABSOLUTE (`w[k+1] >= w[k] + 1`) while the objective's scale
  is free (premise §3). At the fitted scale a 1-unit gap is 1.5 % of `w5`; at the
  committed scale it is 0.07 % and invisible. What is scale-free — and what the
  finding should lean on — is that the **unconstrained** `w5` is *negative*
  (`-7.05`), which no rescaling repairs. I did not find a reading under which
  this changes the conclusion, only the sentence.
- **Q-2. Premise §3 registered three consequences the design does not carry.**
  §3 says *"the fit registers a normalisation, and the diagnostics report
  saturation rate … Scale is not fitted against the labels in the same objective
  as the ratios."* §2 does the opposite on all three: one squared-error
  objective over absolute labels, no normalisation, no saturation-rate
  diagnostic. Measured, this is inert — **0 of 74 672 rows saturate under either
  table** — so §3's own mechanism has no purchase on this corpus. But the design
  drops a registered consequence of its own premise without saying it is
  dropping it, and the fitted scale did collapse by ~23x, which is what §3
  predicted. Is the right fix to carry the normalisation, or to record in the
  premise that §3's clamp mechanism is measurably absent here?
- **Q-3. The labels are 400 000-node searches; the SPRT plays at 50 000.** §7
  does not discuss the gap. Not obviously wrong — the seat is the standing
  instrument — but the target of the fit and the instrument of the verdict are a
  factor of eight apart and the design does not say why that is acceptable.
- **Q-4. `docs/ROADMAP.md:489-492` and `:511-512` place SPSA/Texel tuning in
  Stage 4** (*"Lazy SMP and SPSA/Texel tuning are untouched by this and remain
  Stage 4's own work"*), and `configs/eval_v0_weights.toml:46-47` says the same
  (*"SPSA/Texel replaces these in Stage 4"*). The ROADMAP now also carries a
  Stage-2 paragraph about this very phase (`:464-478`), so the document says
  both. Under hard rule 10 and the ROADMAP's own "changed only by ADR", which is
  it? I found no ADR pulling Texel tuning into Stage 2; D-613 and D-614 assume
  Phase 1 exists but neither moves it.

---

## What I attacked that SURVIVED

Stated plainly, because the FAIL above should not be read as doubt about these.

1. **Premise §2 — the v0 score is linear in the five weights, clamp included.**
   This is the claim everything rests on and it is the one I attacked hardest.
   Re-derived at 7 478 positions against the engine's own `HandcraftedV0::value`
   — 15x the registered sample, drawn from strata the registered sample could
   not have held (all 3 886 five-window positions, 1 877 of them mate-kind; a
   stride draw across all sixteen tranches and all three score kinds) — and at
   three weight tables including two that force the clamp on up to 3 664 of
   them. **0 disagreements in every configuration.** The clamp-boundary question
   is answered: both sides compute `max(-M, min(M, ·))` and agree exactly, and
   `clamp(-x) = -clamp(x)` makes the ordering of clamp and sign immaterial.
   `features.py`'s omission of the addressable-lattice check is also safe —
   `Coord` is `i16` (`coord.rs:19-33`) and no corpus position comes within four
   orders of magnitude of the range.
2. **The corpus join.** `extract.py:54-58` verifies `key_full` on **every** row,
   not a sample; all 89 805 passed on my re-run.
3. **Full reproducibility.** My rows file is **byte-identical** to the receipted
   one (`sha256 1489f43b…`) and every line of `fit_diagnostics.txt` reproduces.
   There is genuinely no seed, no learning rate and no stopping rule in the
   answer — §2's stronger-determinism claim is true.
4. **The active-set enumeration is correct where it applies.** For a convex QP,
   the optimum's active set is among the sixteen, each reduced problem is the
   equality-constrained minimum, and taking the feasible minimiser is exact. The
   reduced normal equations `M'AM z = M'(b - As)` are formed correctly
   (`fit.py:125-132`). M-4 is about the *bounds*, which are outside this
   enumeration — not about the enumeration.
5. **§3's engine-side claim.** *"Byte-identity when the key is absent" cannot be
   built* is correct: `weights_file` is a required `PathBuf` with no
   `serde(default)` under `deny_unknown_fields`, and there are no literal
   weights in code. The two replacement properties both hold when run
   (ledger 11-12), and the SPRT's two seats carrying different tables is **not**
   refused by the arena (m-7).
6. **D-616's counts.** 4.6x, 1 873 / 3 882 = 48 %, 12.38 % vs 2.69 % — all
   reproduced from the corpus by a second instrument, and robust to the sharper
   "holds a five-window on either side" definition (48.3 %). What fails is the
   *direction* the finding attaches to them (M-3), not the counts.
7. **1-in-8 is more than enough validation** for a five-parameter linear model at
   9 406 rows, and D-614 makes it gate nothing regardless.
8. **All three artifact receipts verify** (14 of 14), **CI gate 18 passes**, and
   **the citation gate is green** on the design's ten citations.

---

## What must happen before the SPRT runs

In dependency order. Items 1-3 block; 4-8 are cheap and should ride along.

1. **§7 gains `elo0`, `elo1`, `alpha`, `beta`, a max-n, an arena config path, an
   explicit `openings_skip`/`openings_take`, a cost statement and a dry-run
   record** (M-1, M-2), and the ledger row lands with the config.
2. **§8 and `wp22_phase1_fit_finding.md` are corrected on the direction of the
   censoring**, and D-616 is amended rather than re-read (M-3). The one-sided
   `k=4`/`k=5` regressors go in as the finding that actually constrains the
   successor.
3. **`fit.py` either enumerates the bounds or refuses when it clamps** (M-4),
   with the reproducer above as its test. This does not change Phase 1's table;
   it stops the successor inheriting a silent projection.
4. Ship and receipt the oracle; register its sampling rule (M-5).
5. Resolve the human-corpus holdout (M-6) and Q-4's Stage-2/Stage-4 conflict.
6. Move §2's measured vectors into the artifact (M-7).
7. Fix §2's stale stage list and `fit.py`'s stale docstrings (M-8, m-3).
8. Fix `D-55y` → D-553, the governing revision, and the two premise citations
   (m-1, m-2, m-7, m-8); consider adding the premise memo to the citation gate's
   governing list.

**One thing worth saying to whoever acts on this.** Phase 1's code is in better
shape than its documents. Every number it produced reproduces exactly, the
extractor is right under attacks it was never asked to survive, and the fit is
genuinely hyperparameter-free. The blocking findings are all about what the
documents license next — an SPRT with no bounds, a successor named on a reversed
mechanism, and an exactness claim broader than the code. None of them requires
re-running the fit.
