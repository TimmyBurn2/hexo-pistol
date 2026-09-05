# WP-2.2 — final summary: what was done, why, and what it cost

## ONE LINE FOR THE MORNING

**There is no Stage-2 Elo number. The finding that replaces it is that BOTH of
this package's measurement targets were mis-specified in the same way — Phase 1's
fit is censored by dropping mate rows, and §B's floor counts a unit whose 86
observations are 10 independent trials — and each was found by measuring the
thing rather than arguing about it.**

Read `wp22_STOP_SUMMARY.md` for the STOP that interrupted §B, and D-616 and
D-618 for the two findings. This document is the whole record.

## §1 What this session was asked for, and what it actually produced

The dispatch asked for Stage-2's eval: Texel tuning first (Phase 1), then the
learned family (Phase 2), with §B's census running alongside. **It did not get
there, and the honest headline is a finding rather than an Elo number.** What it
did produce is below, in the order it happened, with the reasoning that drove
each turn.

## §2 §0 — the ground state, and why the receipt was checked before anything was destroyed

The dispatch said "remove the thirteen exported worktrees (219-file digest
receipt exists)". The receipt's EXISTENCE was given; its COVERAGE was not.

**So the receipt was verified first**: 219 lines, `sha256sum -c` clean,
per-worktree counts compared against the live trees one by one. That check is
what found the gap — **the receipt covers `artifacts/` and `sessions/` only**,
and five worktrees held evidence outside both: session logs, suites, probe
outputs, generator scripts, plus 1.7 GB of re-derivable build products. Those
were exported to `artifacts/wp22_worktree_residue_export/` (19 files, own
receipt) before removal, with the build products excluded and the exclusion
named.

`w2` also carried two modified tracked files. Rather than assume they were
stale, I checked: `dev` already carried both (`root_reorder: _` at
`config_validate_tests.rs:191`, and a rustfmt reformat of a `let`-chain). Nothing
was lost.

**Why this mattered**: D-469 exists because WP-1.8c's four review reports
survived only in a transcript. A receipt that is trusted rather than read is the
same failure with a document in front of it.

## §3 §R — the five rulings, and the citation that does not resolve

R1-R5 landed as **D-610 through D-614**, each quoting the operator verbatim and
stating what it forecloses and what it leaves open.

One thing I did not smooth over: **R2's own text cites `D-56q`, which is not a
line that exists.** D-611 records that and names D-570 as the line meant, on
D-575's precedent — a dispatch citing a D-line the tree does not hold is
recorded, never invented.

## §4 R2's tabulation — D-611 closed with data

D-611 said the disagreement classes are closed "by that data" and that a
successor may not close them by reasoning. So I counted, recomputing from the
manifest body rather than reading its derived header — the two agree at **2 369**.

**Zero are transposition disagreements.** All 2 369 are symmetry equivalences
(2 323 also sharing `key_seq`, 46 sharing `key_full` alone). The question
D-562(2) framed — "which key rules a disagreement" — was framed around a class
this corpus does not contain, and every consumer's rule handles what it does
contain. D-611's flip clause does not fire.

**Collateral worth its own line**: the symmetry fold yields **2.6 % at the root
population** where D-560 and D-570 measured zero in tree. Not a contradiction —
different populations — but a successor reading "the symmetry fold yields zero"
must now ask which one.

## §5 Phase 1 — the premise findings, the oracle, and the fit that reported against itself

**Two premise findings before any code ran.** The dispatch asked for a weights
loader behind a config key with a digest in the identity line; **that already
ships** (`config.eval.weights_file`, `weights_sha256`, and an arena that refuses
a mismatch). And the dispatch's "byte-identity when the key is absent" **cannot
be built** — hard rule 1 makes an absent key a named error and there are no
code-side literals to fall back to. So the phase was smaller than the dispatch
assumed, and the two tests that carry the property it was reaching for replaced
that clause.

**The eval is LINEAR in its five weights**, which makes the whole fit exact and
seedless. That is not asserted — an oracle checks the offline extractor against
`HandcraftedV0::value` on 500 positions: **0 disagree**, while perturbing one
table entry by 1 makes **410 of 500** disagree. The corpus join is verified on
**all 89 805 rows**, not sampled.

**Then the fit reported against itself.** Unconstrained least squares wants a
table that is non-monotone and negative at the top (`w5 = -7.05` — five own
stones worth less than nothing). The exact constrained optimum is
`[1, 21, 22, 64, 65]`, with two of four gap constraints BINDING: one-from-a-win
valued at 65 against two-from-a-win's 64, where the committed table says 1500
against 300.

**The mechanism is measured, and it is the design's own exclusion rule.** A
position holding a five-stone window is **4.6x likelier to be scored `mate_in`
than `eval`** (12.38 % of 15 133 mate rows against 2.69 % of 74 672 eval rows).
Dropping mate rows by kind — correct in isolation, since a mate score is the
search's band — discards **1 873 of the 3 882** positions carrying that
evidence, and discards exactly the ones where the window converted. `w5` is
fitted to nearly-complete lines that did not win.

**Every offline diagnostic improves while this is true** (validation MSE 705 709
against 999 066, Spearman 0.319 against 0.209). That is precisely the reading
D-614 forecloses, and it is why the finding exists as D-616 rather than as a
weight table shipped on a better loss.

**One correction folded in rather than hidden**: the first implementation
PROJECTED the unconstrained solution onto the schema. That is not the
constrained optimum, and the two answers differ here. Replaced by exhaustive
enumeration of the sixteen active sets — exact, no step size, no tolerance, no
seed — and pinned by a test so it cannot come back quietly.

## §6 §B — three registrations, three FAILs, and why that was a STOP rather than a fourth try

The cap registration went through **three fresh-context reviews and failed all
three**, improving materially each time. The rounds are worth recording because
two of them reversed each other:

| round | revision | verdict | the finding that mattered |
|---|---|---|---|
| 1 | rev 2 | FAIL, 13 MAJOR | D-563's cap figures are at `--nodes 400000`; the registered seat is 50 000. The prior was imported across an 8x budget change |
| 2 | rev 3 | FAIL, 7 (3 measured) | **M2**: round 1's advice — drop the corpus join, the census counts symmetry-invariant keys — is FALSE, measured at 229 against 236 firings |
| 3 | rev 4 | FAIL, 5 MAJOR | **B4**: the registered rule already returns 2048 on the document's own dry run, in 100.00 % of 20 000 bootstrap replicates, and the UNDERPOWERED fallback is 2048 too |

**Round 2 reversing round 1 is the most instructive thing here.** Round 1 argued
the join was unnecessary because `canonical_key` is symmetry-invariant — which
is TRUE about the keys and says nothing about the SEARCH, whose tie-breaks are
coordinate-lexicographic. I took the advice, and round 2 measured it wrong. I
reproduced the measurement before reverting. **A measurement beats an argument,
including a prior reviewer's**, and D-615 pins it so the seductive version
cannot be re-derived into the tree.

**Two findings I reproduced against myself in round 3**: the search command I
wrote into §10.2 **does not run** — I dropped the dash-escapes when transcribing
it, inside the paragraph invoking D-601/D-602 against transcription defects —
and §2's population count moved because **my own dry-run outputs joined the set
I was counting**.

**Why STOP.** The dispatch grants three rounds per gate, the third
remedies-only. That was spent. The STOP protocol says a document failing after
the granted rounds is a STOP: "split, no self-granted round". Writing a fifth
revision would have been the one move the protocol names and forbids.

## §7 What the STOP surfaced, and what measuring it afterwards changed

The STOP handed the operator one question: *is the calibration worth running
when its own rule already returns 2048?*

**Measuring the question properly turned it into a better one.** Three numbers,
all from the same 100 positions at all three caps:

1. **The floor is not binding at any cap.** 28 distinct keys are cleared in
   **5.6 minutes** at cap 2048 and inside 35 at either larger rung. The
   calibration is a **100-minute** instrument for a decision worth at most half
   an hour of run time.
2. **The caps are COMPLEMENTARY, not ordered.** Cap 16384 proves four roots
   2048 cannot; 2048 proves two neither larger cap does; union **12** against
   8/8/9. D-563's structural foreclosure is confirmed — and so is its converse,
   because a large cap spends the same node budget on fewer, deeper calls and
   never reaches positions the small cap fires at.
3. **The floor's letter can be met by ONE root.** At cap 2048 the 86 keys come
   from 8 roots distributed `[35, 33, 11, 3, 1, 1, 1, 1]` — so **28 keys are
   reachable from a single search tree**. D-537's floor is a one-sample BINOMIAL
   power calculation (p0 = 8/14, p1 = 12/14, n = 28, c = 21), and a binomial
   assumes independent trials. At cap 16384 the same 28 keys would need at least
   nine roots.

**Fact 3 inverts the ranking.** On the registered quantity cap 2048 wins by 6x;
on independence it is the worst of the three, because its keys pile up inside a
couple of trees.

None of the three reviews found facts 2 or 3, and neither did the registration.
They are cheap — minutes of arithmetic over data already on disk — which is
D-291's point exactly: an estimate that could have been measured in seconds is a
finding.

## §8 The decision — how it was taken, and why it is not the one I recommended

The operator returned the STOP with an instruction to reason it out and take the
best option. CLAUDE.md fixes the vehicle for exactly that: *"A named decision
with more than one viable option is settled by an OPTION MATRIX — options,
costs, failure modes, recommendation — attacked by a fresh-context
DECISION-RED-TEAM subagent BEFORE selection."*

**Before writing the matrix I measured the things the argument turned on**,
because D-291 says an estimate that could have been measured in seconds is a
finding. Three measurements, all over one 100-position sample at all three caps,
all from data already on disk:

1. **The floor is not binding at any cap.** 28 distinct keys are cleared in 5.6
   minutes at cap 2048, inside 35 at either larger rung. The calibration was a
   **100-minute instrument for a decision worth half an hour**.
2. **The caps are complementary, not ordered**, and I verified the mechanism
   per position: cap 2048 *fires at* the four roots only 16384 proves — 28, 12,
   12, 14 times — and never finishes; while at the roots only 2048 proves, cap
   16384 fires just twice, its budget spent on a couple of deep calls. Breadth
   against depth, caused by the shared node budget.
3. **The floor's letter can be met by ONE root.** Cap 2048's 86 keys distribute
   `[35, 33, 11, 3, 1, 1, 1, 1]`; entry 98 alone supplies 33.

The matrix listed five options and recommended **E** — retire the calibration,
census at two caps, union reported as coverage. **The red team wounded E in three
places and proposed a sixth option I had not listed, and I took that instead.**

**Why E fell.** E's primary count was to be taken at cap 2048, the arm most
inflated by clustering — and the red team showed the inflation destroys the test
the floor exists to power. D-537's `n = 28` comes from a one-sample **binomial**,
which assumes independent trials; `matrix_stage3_detector.md` §5.4 defines its
alternative `p1 = 12/14` as a bound **over the census columns**. So rows sharing
a column signature return one verdict to every hypothesis the test can entertain
— correlation of 1 **by the alternative's own definition**, not by assumption.

**I then measured the red team's own quantity more sharply than it had, and it
is worse.** Against the actual candidate field of `tools/stage3_census_rank.py`,
cap 2048's **86 win-proving keys are 10 distinguishable trials** — an 8.6x
collapse, where the red team reported 3.5x over raw column tuples.

**And that same measurement demoted the question that cost this session three
review rounds.** The distinguishable-trial rate is **0.10 / 0.07 / 0.08** per
position across the three caps — a spread of **1.4x**, against the key count's
4.5x. **On the currency that decides whether detector round 3 may open, the cap
very nearly does not bind.** The retired calibration was measuring the wrong
thing, not measuring it badly.

**Selected: option F.** Retire the calibration unrun. One census, cap 2048,
`--nodes 50000`, single instrument, sized on distinguishable trials, reporting
three counts side by side — D-537's literal key figure, the root figure, and the
trial figure. **`n = 800`, ESTIMATED 45 minutes.**

**Why no operator ruling was needed**: D-537 lets a successor register a LARGER
minimum with grounds and never a smaller one, and 28 trials is strictly larger
than 28 keys, in the same currency and the tightening direction. D-570's identity
is supplemented, never replaced, and all three counts are reported.

**The strongest surviving attack on F, recorded in D-618 as the Process section
requires**: if the operator rules that D-537's counted unit is the canonical key
and may not be supplemented — that the independence defect belongs to the FLOOR
and is round 3's to fix when it registers — then the third tally is premature.
F accommodates that by reporting all three counts and adjudicating the floor on
D-537's own unit. **It does not refute it.**

**Before registering n = 800 I checked one thing that could have made F
infeasible**: distinct verdict-vectors are a coupon-collector quantity, so a
ceiling near 10 would put 28 out of reach. The cumulative curve is still climbing
over the second fifty positions at both caps examined — no ceiling visible — and
it climbs **lumpily**, sitting at 2 through sixty positions and reaching 10 over
the next forty. That lumpiness is why n is registered near three times the point
estimate rather than at it.

## §9 What this cost, honestly

- **Three review rounds on a registration that was then retired unrun.** The
  calibration consumed most of the session's review budget and produced no cap.
  Its dry runs were not wasted — every measurement that decided D-618 came from
  them — but the document itself was a 100-minute instrument for a question that
  the same data answered for free.
- **Two CI runs discarded**: one killed for drifting under itself, one VOIDed
  at gate 20 by a defect of mine (D-617).
- **I followed a reviewer's advice into a defect.** Round 1 argued the corpus
  join was unnecessary; round 2 measured that it was not. D-615 pins it.
- **Two errors I made and reproduced against myself**: a search command written
  into a registration that does not run (dropped dash-escapes, inside the
  paragraph citing D-601/D-602 against transcription defects), and a population
  count that moved because my own outputs joined the set I was counting.

## §10 What is owed next

1. **Run the census** once its revision-2 review passes (~45 min), and state
   §B's closure line: the three counts against the floor of 28, and detector
   round 3 licensed or the shortfall stated.
2. **Phase 1's SPRT** — candidate prepared and validated
   (`[1, 21, 22, 64, 65]`, sha256 `834b4f7f…`), expectation registered: it
   loses, and h0 is the finding that the v0 feature set is the limit.
3. **R3's seat-swap anchor** — configured, needs a quiet box.
4. **Phase 2** — not started. Its premise memo should quote D-616: the corpus
   cannot be fitted by dropping its mate rows, and the censored likelihood is
   the named successor.
