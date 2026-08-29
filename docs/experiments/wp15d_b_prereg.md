# WP-1.5d (B) — PRE-REGISTRATION: the safety-net cap's calibration, bench and SPRT

**REVISION 3 — a further round, scoped by the architect to the four findings of `wp15d_b_prereg_REVIEW_rev2.md` and nothing else.** Revision 2 FAILED (1 BLOCKING, 3 MAJOR), the cap returned the package, and the architect granted this round. **REVISION 2 was the one fix round.** Revision 1 (`0dcd0db`) FAILED its
fresh-context review (`docs/experiments/wp15d_b_prereg_REVIEW.md`: 3 BLOCKING,
7 MAJOR, 4 MINOR). **No sixth provenance defect was found** — every figure
reproduced from the registered artifact and matched no pre-split one, the
selection rule and excluded set were verified against the instrument, and the
commit clock showed registration preceded the run. It failed on OMISSIONS and on
sentences claiming more than their receipts. What changed:

| revision 1 | revision 2 | why |
|---|---|---|
| the SPRT's `elo0`/`elo1`/`alpha`/`beta`/game cap registered nowhere | §4A registers all five | BLOCKING 1 |
| Criterion 1'' invoked once, undefined; no second instrument, no dry run | §4B registers all three with their pins and consequences | BLOCKING 2 |
| no arena config named | §4C names it and registers every value it carries | BLOCKING 3 |
| "the grid validated itself at its own top end" | **WITHDRAWN — measured false.** The decay is at K = 64; K = 128 is inert | MAJOR 2 |
| "every corpus cell is `nodes=50176`" | false in 14 of 168 cells; restated | MAJOR 1 |
| the IQR threshold first appeared in the results | registered in §3 | MAJOR 4 |
| the (1.10, 1.25] band had two instructions and no row | one row, in §5 | MAJOR 3 |
| no cost statement, no rule-6 reporting | §3 and §4A | MAJOR 5, 6 |
| §7.7's margin argued off the wrong workload | restated | MAJOR 7 |
| four MINORs | closed, including §6.3's unnamed denominator | MINOR 1–4 |

**Revision 1.** Written BEFORE the measurement run and before game one. Governed
by `docs/decisions.md` **D-482** (one run, one instrument, one artifact; no
pre-split number admissible) and **D-483** (a design carries no measured
numbers; every number a prereg consumes is produced post-implementation by a
registered instrument and cited from that run's artifact by digest).

**WHAT IS BEING JUDGED.** `safety_net_top_k` — the safety-net candidate cap
landed and gated off by session (A), `docs/decisions.md` **D-484** — armed at the
K this document's own calibration selects, against the committed engine.

**WHAT IS NOT.** The option field (matrix M2 and its three red teams), the
selection of the scope (D-478), and the mechanism (cleared twice with zero
correctness findings, D-484). None of those is reopened by a verdict here.

**NO NUMBER FROM BEFORE THE SPLIT APPEARS IN THIS DOCUMENT.** The option field's
figures — every lift, ratio, divergence and width it measured — are motivation
and history and are cited nowhere below as data. That rule is not a formality:
this work package failed five document reviews on numbers quoted away from the
run that produced them, which is what D-479 and D-483 exist for.

---

## 1. THE INSTRUMENT, AND ITS GOVERNING REVISION

| | |
|---|---|
| instrument | `crates/pistol-search/tests/wp15d_b_measurement.rs` |
| governing revision | **`70cb580`** — a change to it reopens this review exactly as an amendment to this document does (`docs/process.md`, "Instrument governing revision") |
| command | `cargo test --release -p pistol-search --test wp15d_b_measurement -- --ignored --nocapture --test-threads=1` |
| budget | `Stop::Nodes(50_000)` throughout — reproducible, and instrument mode refuses a wall-clock budget (D-22, D-478) |
| `quiet_radius` | **PINNED at 2**, the committed instrument document's value, and NOT swept |
| `tt_bytes` | `SMALL_TT` (`1 << 20`), the test harness's value, stated rather than implied |
| ordering heuristics | all three OFF, as in every committed config |

**WHY `quiet_radius` IS PINNED RATHER THAN SWEPT.** The addendum permits one
axis pinned with a stated reason. The reason is attribution: the SPRT compares
ONE changed key against the committed engine, and a run that moved two could not
say which one a verdict belonged to (CLAUDE.md rule 6). A `quiet_radius` sweep is
a different experiment and is not licensed here.

**ONE RUN, ONE ARTIFACT.** The instrument emits the calibration sweep, both bench
fixtures and the sensitivity receipt in a single pass into
`artifacts/wp15d_b_measurement_v1.txt`, whose digest is recorded at §7. **No
figure below may come from anywhere else**, and a second run for any reason
supersedes the first wholesale rather than being merged with it.

---

## 2. THE CALIBRATION — grid, channel, direction, rule, and the undefined case

**THE GRID.** `K ∈ {4, 8, 16, 32, 64, 128}`, plus the incumbent seat `K = 0`.

**THE FALSIFIABILITY BRANCH, REWRITTEN SO THE INSTRUMENT CAN ACTUALLY REACH IT**
(review MAJOR 2). Revision 1 registered "if the benefit does not decay across the
grid, the rule selects the largest point". **That branch was unreachable by
construction**: at a K above every pool the cap never fires, so that seat is the
incumbent relabelled, its gain is identically 0, and it can never satisfy a
positive threshold. A criterion the defect it names cannot falsify is no
criterion (`docs/process.md`). The branch is therefore keyed on the largest grid
point at which **the treatment is actually applied**:

> **`K_bind` = the largest grid point with `capped_rows > 0`.** If
> `gain(K_bind) ≥ 0.75 × max gain` — i.e. the benefit has NOT decayed anywhere the
> cap still binds — the rule selects `K_bind` and **that outcome is a finding
> about the channel rather than a calibration**, recorded as such. Grid points
> above `K_bind` are INERT and carry no information about decay; they bound the
> pool size from above, which is a different fact and is reported as one.

**THE POPULATION.** The book's openings `0..999` —
`crates/pistol-cli/tests/fixtures/random_openings_v1.txt`, sha-pinned. Every one
of them was consumed by an earlier verdict and is spent for verdict purposes,
and all of them are **disjoint from the `1500..1999` slice §4's SPRT draws**. K
is therefore chosen on a different sample from the one the verdict is read on.

**THE EXCLUDED SET, FIXED ACROSS SEATS.** An opening whose search stopped because
it PROVED a mate did not fail to reach depth; it finished. Such openings are
excluded — but the excluded set is the union over ALL seats, the incumbent
included, computed once and printed by index (`CAL/EXCLUDED`). **Per-seat
exclusion would let a larger K shrink its own denominator**, and D-395's
precedent is the union form: it compared "19 of the 24 … on BOTH sides".

**THE CHANNEL AND ITS DIRECTION.** `mean(K)` = the MEAN completed `depth_turns`
over that fixed population. **Larger is better.** It carries no threshold, and
that is deliberate: a "reaches depth ≥ n" count would need an `n` chosen from
data D-482 makes inadmissible, which is the after-the-numbers choice a
pre-registration exists to forbid.

**THE UNDEFINED CASE.** There is none, and the reason is structural rather than
lucky: completed `depth_turns` is a non-negative integer on every search, and
under `Stop::Nodes` the first iteration is not abortable (`search.rs`,
`pvs.rs`'s *"a non-abortable iteration completes"*, D-74), so no seat can fail
to produce one. The instrument prints the full histogram per seat, so a depth-0
opening would be visible if the reasoning above were ever wrong.

**THE SELECTION RULE, APPLIED BY THE INSTRUMENT AND NOT BY A READER:**

> `gain(K) = mean(K) − mean(0)`.
> **If `max gain ≤ 0`, NO K is selected**, no SPRT is run, and the work package
> closes as a measured finding.
> **Otherwise K is the LARGEST grid point with `gain(K) ≥ 0.75 × max gain`**,
> ties to the larger K.

**Why that rule.** It prefers the WEAKEST prune that keeps three quarters of the
measured benefit, so it cannot select a grid extreme by construction: a rule
maximising the channel would take the smallest K, and one minimising the cost
would take the largest. It is computed inside the instrument
(`fn selection`, `70cb580`) and printed as `CAL/SELECTED`, so the choice is not a
step a human takes after seeing the table.

---

## 3. THE RULE-5 BENCH — brackets, directions and abort bounds

Both fixtures, every seat, **five reps**, per-position MEDIAN, at the same
`Stop::Nodes(50_000)`. The instrument prints every rep so the median is
checkable and not asserted.

**DIRECTIONS, in this repository's own convention** (D-388, D-395, D-398):
time-to-depth ratio is **ON/OFF and LARGER IS WORSE**. It is the gate. `nps` is
NOT read as a gate at all — across seats with different candidate policies it is
not a like-for-like unit (D-374).

| fixture | bracket | ABORT |
|---|---|---|
| `bench_positions_v1` (corpus) | Σ median ms ON / OFF **≤ 1.10** | **> 1.25** |
| `spread_v1` | **REPORTED, NOT GATED** | — |

**THE BAND BETWEEN THEM IS NOT A GAP** (review MAJOR 3). A ratio in
**(1.10, 1.25]** misses the bracket without reaching the abort: the SPRT is
**NOT run**, the number is recorded as a finding, and the change is not
re-scoped inside this WP to chase it (CLAUDE.md rule 5's own "a measured
structural floor is a finding, not a failure", and D-374: registered numbers do
not move). §5 carries this as its own row rather than leaving it to be inferred.

**THE IQR GATE, REGISTERED HERE AND NOT IN THE RESULTS** (review MAJOR 4).
Per position, the IQR of its five per-rep times must be **≤ 10 % of that
position's own median** — the D-215/D-362 convention. A position exceeding it
WITHHOLDS the verdict and is re-measured, both seats, before any ratio is read.
Revision 1 stated this threshold for the first time while reporting the result,
which is the after-the-numbers move §2 refuses two sections earlier.

**THE COST, ON THIS DOCUMENT'S OWN FACE** (`docs/process.md`; review MAJOR 5).
The measurement run: 7 seats × 1000 openings + 7 × 24 × 5 + 7 × 4 × 5 searches at
50 000 nodes, **MEASURED 2662.25 s** single-threaded, one launch and one read.
The governed run: 500 openings × 2 seats over 4 workers at the same budget —
**ESTIMATED 1.5–3 hours** wall from WP-1.7's own comparable run, plus one
Criterion 1'' pass and one second-instrument pass of a few seconds each.
Operator attention: one launch, one read, one slot pass.

**WHAT THE RUN REPORTS, AND WHICH LINES ARE READ** (rule 6; review MAJOR 6). The
arena's own report, unchanged by this WP: `counts n distinct_n wins_a capped
losses_a forfeits decided`, the `pentanomial` line, `llr_pair`, `nelo_pair` with
its `ci95`, and the two `timing_engine` lines. **`distinct_n` is reported beside
`n`** — identical games are deduped and a claim quotes both. **Per-side compute
is reported** from the two `timing_engine` lines. **The protocol is the line
protocol** (D-88), the instrument is fixed-nodes, and no wall-clock-only claim is
made anywhere (rule 6).

**THE CORPUS IS A NO-REGRESSION CHECK AND IS REGISTERED AS ONE.** The cap
narrows a row this fixture rarely takes, so a large gain here is not expected and
would itself want explaining. **If the ABORT fires the SPRT is not run** and the
package closes on the bench, the WP-1.8c shape (D-465).

**THE SPREAD FIXTURE IS REPORTED, NOT GATED, AND THE REASON IS STATED SO THE
REPORT CANNOT BE READ AS A GATE.** The debt that fixture represents (D-95) is
defined at a wall-clock budget, which instrument mode refuses (D-22), so nothing
measured here at `Stop::Nodes` can discharge it. D-478 leaves that debt open and
re-points it at its own package; this document does not touch it. What is
reported is the ON seat's own `safety_net_capped_rows` per position — the count
on the tree that actually ran, never inferred from the OFF seat's.

---

## 4. THE SPRT ARM

| | |
|---|---|
| arena | `target/release/arena`, built `--release --locked` at the run's revision |
| engine A | the cap ARMED at §2's selected K — **slot A because the arena's statistic is slot A's score and H1 is "the capped seat is stronger"**; reading that sign backwards is the one mistake this document exists to prevent |
| engine B | `configs/instrument_staged_v0.toml`, the committed engine, unchanged |
| binaries | both seats `target/release/pistol`, bound by `binary_sha256`, re-confirmed at the slot pass |
| book | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt`, sha-pinned |
| `openings_skip` | **1500 — THE FRESH SLICE, FIXED HERE AND NOT AT LAUNCH** (D-427's lesson). Consumed to date: `0..499` (WP-1.5b, and retired by D-402), `500..999` (WP-1.6, D-427), `1000..1499` (WP-1.7). This draws `1500..1999`, disjoint from all three by construction, and is the last unconsumed slice of the 2000-opening book |
| `openings_take` | 500 |
| budget | `nodes`, 50000 |
| `turn_cap` | 40 |
| `n_workers` | 4 |
| `hang_timeout_ms` | 120000 — liveness only, never an adjudication (D-159); its margin is confirmed at the slot pass against §3's own worst single search |

---

## 4A. THE VERDICT PARAMETERS — registered here, not authored at launch

Review BLOCKING 1: revision 1 read `h0`/`h1`/`inconclusive_at_game_cap` in §5
while the parameters that DEFINE those verdicts appeared nowhere, so they would
have been written into a config at launch, unreviewed, with the calibration
numbers already on the page.

| key | value | why this value |
|---|---|---|
| `elo0` | **0.0** | H0: the capped seat is no stronger. Every SPRT in this project uses this null |
| `elo1` | **15.0** | H1's alternative, the same as WP-1.6's and WP-1.7's, so a verdict here is comparable with theirs rather than measured against a bar moved for it |
| `alpha` | **0.05** | as every prior run |
| `beta` | **0.05** | as every prior run |
| game cap | **`openings_take` × 2 = 1000 games (500 pairs)** — the boundary `inconclusive_at_game_cap` names |
| pair floor | **100 pairs**, below which no verdict is read however the LLR crossed — §2's own floor in every prior prereg |

**H1 IS "THE CAPPED SEAT IS STRONGER", AND ENGINE A IS THE CAPPED SEAT.** The
arena's statistic is slot A's score; reading that sign backwards is the single
mistake this document exists to prevent, and §4 states the seat assignment for
the same reason.

---

## 4B. THE DOUBT, ITS INSTRUMENT, THE SECOND INSTRUMENT, AND THE DRY RUN

Review BLOCKING 2: revision 1 invoked Criterion 1'' once and defined nothing.

**THE STAGE UNDER DOUBT**: everything between the two engine processes and the
printed verdict — the arena's seat bookkeeping, pairing, referee and scoring.
This WP changes which engine sits in a seat, not the arena that scores them.

**THE INSTRUMENTS, EACH WITH THE REVISION THAT GOVERNS THIS RUN** — a change to
any reopens this review exactly as an amendment to this document would:

1. **The warm-replay pass** — `crates/pistol-arena/src/{seats,transcript,replay,replay_report}.rs`
   and `bin/arena.rs`'s `--replay` mode, at **`a14912a`**, the last commit that
   touched the crate. **Not the `bfdf933` its predecessors pinned, and the
   difference is checked rather than assumed**: `git diff bfdf933..HEAD --
   crates/pistol-arena/src/{replay,replay_report,seats,transcript}.rs
   src/bin/arena.rs` is **181 deletions and 0 insertions**, all of them the
   `//!` header sweep (D-443); the only non-comment change anywhere in the crate
   since is three lines in a TEST STUB engine (`FirstLegal`), outside the replay
   path.
2. **The statistics layer** — `tools/wp16_warm_attribution_check.py` at
   **`6c929da`**, the last commit that touched it.
3. **THE SECOND INSTRUMENT** — `tools/wp15b_attribution_check.py` at
   **`a80a864`**, the last commit that touched it. **It does not share the stage
   under doubt**: it is the COLD checker, replaying each game from its move list
   without the warm pass's per-engine subprocess state, so a defect in the warm
   replay's seat bookkeeping cannot reach it. Two instruments blind to the same
   stage are one instrument reported twice (`docs/process.md`).
4. **The binaries those actually run** — `target/release/pistol` and
   `target/release/arena`, built `--release --locked` at the run's revision;
   **rebuild means re-record**, and §7.5 re-confirms at launch.

**CRITERION 1'', quoted verbatim from `docs/experiments/wp16_warm_replay_design.md`
§4 point 4, in full and without ellipsis:**

> **Criterion 1''.** A report is a measurement iff (a) zero
> divergence-confirmed inversions — every divergence found in point 2 above
> resolves to either "no divergence" or "confirmed inversion" (the
> other-engine match case), never left unclassified — and (b) every
> NON-INERT pair (point 3's exclusion, forfeits always non-inert) is
> directly attributed by first divergence. A DETERMINISM VIOLATION (point
> 2's other branch) is checked FIRST and, if found anywhere, stops the
> whole evaluation before (a)/(b) are even asked, per its own exit code.
> The old clause (b)'s adversarial-reassignment machinery is KEPT, but only
> as a cross-check run over the INERT pairs alone (expected to be a no-op,
> since point 3's theorem already fixes their bucket) — its result is
> cited in the report as confirming evidence, not as the thing the verdict
> depends on.

**THE EXIT TAXONOMY, the instrument's own constants at `6c929da`**:
`ATTRIBUTABLE = 0`, `NOT_A_MEASUREMENT = 1`, `NO_ANSWER = 2`,
`DETERMINISM_VIOLATION = 3` (`tools/wp16_warm_attribution_check.py:133-136`).
What each may be concluded to mean is **WP-1.6 §5's table, imported by reference
and not restated** — that partition was deleted once for being restated (D-424)
and reborn copies are how it failed three reviews (D-423).

**THE AGREEMENT CRITERION, ITS REGISTERED CONSEQUENCE, AND THE TWO WAYS THIS
DOCUMENT GOT IT WRONG BEFORE GETTING IT RIGHT.**

Revision 1 registered "both checkers exit 0". The dry run falsified it: the two
answer to different criteria by design, so that criterion **always fails**.
Revision 2 replaced it with agreement on the two instruments' `1b` and `1c`
COUNTS. The review falsified that too, and worse: `1b`'s count increments BEFORE
its adjudication and a mismatch goes to `failures` rather than to the count
(`tools/wp16_warm_attribution_check.py:835-841`), so the counts are invariant
under a corrupted report and that criterion **always passes**. Two vacuities, in
opposite directions. The criterion is therefore stated over what the instruments
FLAG, not what they COUNT:

> **THE AGREEMENT CRITERION.** The governed report is a measurement only if all
> three hold: **(i)** the warm pass exits `ATTRIBUTABLE (0)` — not `NO_ANSWER`,
> which is how it refuses a report it cannot vouch for; **(ii) BOTH instruments
> report ZERO `1b` move-list mismatches and ZERO `1c` rebuild mismatches** — the
> per-game FINDINGS, never the counts, which the review proved invariant; and
> **(iii)** neither instrument refuses to read the documents at all.
>
> **`1a` is NOT a term, and the reason is the review's** (rev-2 review's own
> corrected finding): a wrong winner does not change which engine moved, so `1a`
> — which asks each labelled engine what it would have played — is blind to a
> referee inversion. It remains the discriminator for a SEAT MISLABEL, a
> different defect, and its failure list is reported as context beside the
> criterion rather than folded into it.
>
> **The cold checker's clause-(b) robustness verdict is explicitly NOT a term.**
> Criterion 1″ supersedes clause (b) for this project's runs (D-401), so a
> clause-(b) failure beside a Criterion 1″ pass is the expected shape and is
> reported as context. That exclusion is what keeps (ii) from collapsing into
> revision 1's always-fails criterion.
>
> **THE REGISTERED CONSEQUENCE**, fixed before either runs: **any of (i), (ii) or
> (iii) failing makes the run NOT A MEASUREMENT.** The verdict is not read,
> neither `h0` nor `h1`, and the package returns to the architect with both
> reports — D-401's own disposition, and not a re-run.

**AND IT IS SHOWN TO FAIL, WHICH IS THE WHOLE POINT** — receipt
`artifacts/wp15d_b_criterion_falsification_v1.txt`:

| input | what happens | criterion |
|---|---|---|
| the clean WP-1.7 report | warm exits 0; cold's `1a`/`1b` lists are **empty** (its one failure is clause (b)) | **PASSES** |
| game 2's `result` flipped `p1_win`→`p2_win` | warm exits **2**, refusing: the replay document is bound to the report's sha256 and "the two documents are not about each other" | **FAILS (i)** |
| game 2's move list reordered, replay doc **rebound** to the mutated digest so the binding cannot be what refuses | warm exits **2** on a structural invariant, before any engine is asked: the pair's two games "differ at turn 2, which is inside the 3-turn book" | **FAILS (i)** |
| **a self-consistent REFEREE INVERSION** — every decided game's winner flipped, the document left internally consistent | the rev-2 review built this and ran both checkers end to end: the counts are byte-identical to the honest run (`1b: 459`, `1c: 682/341`), **and both emit 459 `FAIL 1b` lines** | **FAILS (ii)** — on the findings, which is exactly why the terms are findings and not counts |
| an internally consistent seat mislabel | the cold `1a` asks each labelled engine what it would play and flags `answers[mover] != played[free]` (`tools/wp15b_attribution_check.py:286-290`) | reported as CONTEXT, not a term — argued from the code, not demonstrated |

**AND ONE ATTEMPTED REPRODUCTION THAT DID NOT REACH THE TERM IT AIMED AT,
RECORDED RATHER THAN DRESSED UP AS A SECOND DEMONSTRATION.** This session
rebuilt the referee inversion independently — 459 decided games flipped, the
replay document rebound to the new digest — and the warm pass exited **2**,
refusing before `1b` was reached, because flipping `result` alone leaves the
`score_a`, `llr` and pentanomial fields inconsistent with it. **That falsifies
term (i), not term (ii)**, so it does NOT independently reproduce the review's
construction, which was self-consistent across those fields too. The review's
measurement stands on the review's own receipt; this session's does not
corroborate it and is not offered as if it did.

**THAT IS WHY THE SECOND INSTRUMENT IS NOT BLIND TO THE SAME STAGE.** Both
re-derive attribution by asking the engines themselves; neither takes the
arena's seat bookkeeping on trust. `docs/process.md`'s test — "two instruments
blind to the same stage are one instrument reported twice" — is answered by the
third row above, where the two react to the same corruption through different
mechanisms.

**THE DRY RUN — TAKEN, with its input, its criterion, its defect class and what
it found** (`docs/process.md`).

- *Input*: `artifacts/wp17_governed_run_v1.txt` and
  `artifacts/wp17_governed_replay_v1.txt` — WP-1.7's own preserved governed
  report and replay, a REAL instance of the kind this run will produce,
  differing from it only in identity. Never the governed report itself, which
  does not exist yet.
- *Criterion*: the warm pass reproduces the verdict WP-1.7 recorded from it, and
  the two instruments agree on (i) and (ii).
- *Defect class excluded*: **a checker that cannot read this arena's report at
  all** — a schema or field-name drift since `6c929da`/`a80a864` that would make
  the governed pass exit non-zero and be mistaken for a void run. Exit status
  alone cannot exclude it; reproducing a KNOWN verdict from a known report does,
  because a checker reading the wrong fields cannot land on the right answer.
- **WHAT IT RECORDED:**

```
warm_attribution_check: (b): 3 inert pair(s) excluded by theorem, 338 pair(s) directly attributed at their first differing searched turn, 0 unattributable
warm_attribution_check: cross-check: reassigning all 3 inert pair(s) leaves the verdict `h0` unchanged, as the theorem says it must
warm_attribution_check: 1b: 459 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 682 game(s) and 341 pair(s) rebuilt off the score_a path
warm_attribution_check: PASS — 0 failure(s)

attribution_check: 1b: 459 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 682 games and 341 pairs rebuilt off the score_a path
attribution_check: FAIL 1a robustness FAILS: … moves the verdict from `h0` to `inconclusive_at_game_cap`
```

  The warm pass exits **0** and reproduces WP-1.7's own `h0` at 341 pairs. The
  cold checker exits **non-zero on clause (b)**. **(i) 459 = 459 and (ii)
  682 games / 341 pairs = 682 / 341: the agreement criterion HOLDS.**
- **AND THE DRY RUN EARNED ITS PLACE BY FALSIFYING THIS DOCUMENT.** Revision 2
  first registered the criterion as "both checkers exit 0 on it". On a real
  report of the kind, that is FALSE for a reason that has nothing to do with any
  defect — and had it been registered, the governed run would have been declared
  not-a-measurement on its own designed behaviour. The criterion above is what
  replaced it. A dry run on a synthetic stand-in would not have found this,
  which is the whole of `docs/process.md`'s "only a real instance of the kind
  exercises ATTRIBUTION".

---

## 4C. THE ARENA CONFIG — named, with every value it carries registered

Review BLOCKING 3. The document that IS the experiment is
**`configs/arena_wp15d_cap_vs_staged.toml`**, authored and committed at the
governed run's own launch, because it must carry the launch-time binary digest
and nothing else about it may drift. Every other value is fixed HERE:

```toml
schema_version = 2
[run]
openings_file = "crates/pistol-cli/tests/fixtures/random_openings_v1.txt"
openings_take = 500
openings_skip = 1500          # §4's fresh slice, FIXED HERE, not at launch
turn_cap      = 40
n_workers     = 4
hang_timeout_ms = 120000
[budget]
kind  = "nodes"
value = 50000
[sprt]
elo0 = 0.0                    # §4A
elo1 = 15.0
alpha = 0.05
beta  = 0.05
[engine_a]                    # THE CAPPED SEAT — H1 is about slot A
label  = "staged_snk"
binary = "target/release/pistol"
binary_sha256 = "<filled at launch, §7.5>"
config = "configs/instrument_staged_snk_v0.toml"
[engine_b]
label  = "staged"
binary = "target/release/pistol"
binary_sha256 = "<filled at launch, §7.5>"
config = "configs/instrument_staged_v0.toml"
```

**`configs/instrument_staged_snk_v0.toml` IS COMMITTED WITH THIS REVISION** and
is `configs/instrument_staged_v0.toml` with **one key changed** —
`safety_net_top_k = 16`, §2's selected value — verified by diffing the two
documents' non-comment lines, which differ on exactly that line. That is what
makes a verdict attributable to the key under test.

**THE SLOT PASS (D-427) COMPARES THE AUTHORED FILE AGAINST THIS BLOCK, KEY BY
KEY**, and §7.6 is that comparison rather than a note to look.

---

**THE BOOK IS NEARLY SPENT AND THIS DOCUMENT SAYS SO**: after this run every
slice of `random_openings_v1.txt` is consumed, and the next SPRT in this project
needs a regenerated or extended book. Recorded here because a later session
should not discover it at launch.

---

## 5. OUTCOME HANDLING, WRITTEN BEFORE GAME ONE

| outcome | what happens |
|---|---|
| calibration selects no K | no SPRT. The package closes as a measured finding; the gate stays `0` |
| corpus ratio in **(1.10, 1.25]** | **no SPRT.** The bracket is missed without the abort being reached; the number is recorded as a finding and nothing is re-scoped to chase it (rule 5, D-374) |
| corpus ABORT fires (> 1.25) | no SPRT, whatever the calibration said. The package closes on the bench |
| a position fails the IQR gate | the verdict is WITHHELD until that position is re-measured on both seats; no ratio is read before then |
| the two attribution instruments DISAGREE | **the run is not a measurement.** The verdict is not read, and the package returns to the architect with both reports (§4B) |
| fewer than 100 pairs when the LLR crosses | no verdict is read, however it crossed (§4A's floor) |
| SPRT `h1` | the committed config moves to the selected K, the closure pin is re-recorded with digests, and the 1.8 arc's re-test clause is considered if a material nps jump landed |
| SPRT `h0` | the gate stays `0`. A measured finding, not a failure; the mechanism stays landed and oracle-gated |
| `inconclusive_at_game_cap` | reported as such; no config moves. The gate stays `0` |
| `inconclusive_degenerate` | **the arena's fourth token**, which revision 2 did not route: the pentanomial is degenerate, so no verdict is available at any n. Reported as such, no config moves, and the run is NOT re-drawn on a fresh slice inside this WP — the book has none left (§4) |
| Criterion 1'' fails on the governed report | **the run is not a measurement — not `h0`, not `h1`.** The verdict is not read. D-401's own precedent, and a hard stop |
| arena exit 2 | the run is VOID; the operator reads the instrument's own printed message |

**One re-run is licensed only on a receipted environment fault**, never on a
verdict anyone dislikes.

---

## 6. THE HONEST EXPECTATION, WITH ITS RECEIPT — **SLOT, filled before game one**

The addendum requires the book-class sensitivity receipt to sit here **before**
game one, not after. It is `SENS/TRAJECTORY=…` from the registered run, taken at
the selected K on the calibration slice — a sample DISJOINT from the governed
one, so it is a prediction about the governed run and never a look at it.

**FILLED BEFORE GAME ONE**, from `artifacts/wp15d_b_measurement_v1.txt` at the
selected K = 16, quoted verbatim beside the artifact lines they come from:

```
SENS/TRAJECTORY=incumbent games=25 turn_cap=40 K=16 searches=595 decided_early=20 bearing=125 diverged=29
SENS/TRAJECTORY=capped games=25 turn_cap=40 K=16 searches=672 decided_early=15 bearing=135 diverged=39
```

- **6.1 — the incumbent's own trajectory**: 595 searches, **125
  safety-net-bearing (21.0 %)**, **29 diverged (4.87 %)**.
- **6.2 — the capped engine's own trajectory**: 672 searches, **135 bearing
  (20.1 %)**, **39 diverged (5.80 %)**. Both trajectories are measured because a
  divergence rate read on one engine's path is a rate on a distribution the
  other never walks.
- **6.3 — THE READING, STATED BEFORE GAME ONE.** The class occurs on about a
  fifth of governed-shape searches, and the played turn changes on
  **4.9 %–5.8 % OF ALL SEARCHES** — `29/595` and `39/672`, the denominator named
  because "of them" would otherwise read as the bearing subset, where the same
  divergences are `29/125 = 23.2 %` and `39/135 = 28.9 %` (review MINOR 2). The
  conclusion below holds under either denominator. **That is inside what 500 paired openings can
  see, so this document does NOT predict `h0`-or-inconclusive on grounds of
  insensitivity**, and it does not predict `h1` either: a turn that differs is
  not a turn that is better, and which it is is exactly what the SPRT is for and
  what nothing measured here can anticipate. **The honest expectation is
  therefore that the run is INFORMATIVE, direction unknown.**
- **6.4 — and what would have made it uninformative, recorded so the reading is
  falsifiable**: a divergence rate near zero, or a bearing rate showing the class
  absent from governed play. Neither holds.

---

## 7. FILL-IN SLOTS — the slot pass (D-427)

Filled at the run's own launch, because a document and its configs drift between
revisions and D-427 is the record of what that costs.

**7.1 — THE ARTIFACT.** `artifacts/wp15d_b_measurement_v1.txt`, sha256
`46aaf3fbafbc93bb4fca6816c023e6611a21a1fe739871f4b3ad945f78eefe3e`. Instrument
`70cb580`, run at tree revision `4ec470f`, 2662.25 s, exit 0.

**7.2 — THE SELECTION**, the instrument's own line, verbatim and unwrapped —
revision 1 reflowed it across three lines and called it verbatim (review
MINOR 4):

```
CAL/SELECTED K=16 rule=largest-K-within-75pc-of-best-gain base_mean=2.0740 best_gain=+0.3640 threshold=+0.2730 gains=[K4:+0.3640 K8:+0.3370 K16:+0.2990 K32:+0.2350 K64:+0.0140 K128:+0.0000]
```

**K = 16.** The rule is satisfied at an interior grid point: K16's gain
(+0.2990) clears the 0.75 threshold (+0.2730) and K32's (+0.2350) does not.

**THE DECAY IS MEASURED, BUT NOT WHERE REVISION 1 SAID IT WAS. That claim is
WITHDRAWN** (review MAJOR 2, which measured it). Revision 1 argued the grid
validated itself at K = 128, where `capped_rows=0`. **That is the grid running
off the end of the pool-size distribution, not benefit decaying**: where the cap
never fires, the seat is the incumbent relabelled, and the artifact shows it
literally — K = 128 and K = 0 agree on the mean, on every histogram bucket and on
every counter. The reviewer's partial re-run puts the pool maximum in
**(100, 120]**, so the whole region above it is an inert shelf.

**The evidence the argument wanted is in the same artifact, at K = 64, and
revision 1 never cited it**: there the cap fires **818 937** times — more than at
any smaller K — and still yields `gain = +0.0140`. **A cap that binds harder than
any other on the grid and buys essentially nothing is decay under real
treatment**, which is what makes the selection a choice among points that differ.
`K_bind` (§2) is therefore 64 on this book, `gain(64) = +0.0140` is far below the
`+0.2730` threshold, and §2's falsifiability branch is reachable and did not
fire.

**7.3 — THE CORPUS BENCH.** Σ per-position medians, 24 positions × 5 reps:
incumbent **4800 ms**, K = 16 seat **4807 ms**, **ratio ON/OFF = 1.0015**,
larger-is-worse. Bracket ≤ 1.10: **PASS**, and far from the 1.25 ABORT. **IQR
gate: 0 of 168 position-seats exceeded 10 % of their own median.**

**THE LIKE-FOR-LIKE CLAIM, CORRECTED** (review MAJOR 1). Revision 1 wrote "every
corpus cell is `nodes=50176`", which is false in **14 of the 168 cells**: two
positions are structurally degenerate and terminate early on every seat
(`p13 = 151` nodes, `p16 = 3`), exactly the two D-395 excluded from its own
ratio for the same reason. What is true, and is what the ratio needs, is that
**each position's node count is IDENTICAL ACROSS ALL SEVEN SEATS** — the budget
is a reproducible node stop, so the seats do equal work per position and the
Σ-median ratio compares time for the same work. The two degenerate positions contribute
**0.00 %** of the incumbent's Σ — both read `median_ms=0` in the artifact — so
they cannot move a ratio at all. **Revision 2 said 0.6 % here, sourced from
nothing**, in the very sentence correcting a provenance defect; it is the sixth
instance in this work package and it is recorded rather than quietly replaced.

**7.4 — THE SPREAD REPORT, NOT A GATE.** The ON seat's own
`safety_net_capped_rows`, on the tree that ran:

| stones | incumbent capped | K = 16 capped | median ms OFF → ON |
|---|---|---|---|
| 11 | 0 | **95** | 133 → 133 |
| 21 | 0 | **71** | 449 → 470 |
| 51 | 0 | **152** | 2761 → 2748 |
| 99 | 0 | **0** | 10596 → 10573 |

**The cap fires on this fixture and does not move completed depth on it** — every
seat reads `depth=1` at every position. That is reported, not gated, and it
discharges nothing: D-95's debt is defined at a wall-clock budget instrument mode
refuses (D-22), and D-478 leaves it open at its own package.

**7.5 — `binary_sha256` for both seats**: filled at the governed run's launch,
which is the only moment it can be true of.

**7.6 — `openings_skip`**: to be read from the arena config at launch and
compared against §4's registered **1500**. This is D-427's own slot and the
reason it exists.

**7.7 — `hang_timeout_ms` MARGIN.** The worst single median **in the two BENCH
sections** — the calibration prints only per-seat `sum_ms` and no per-search
figure, so no maximum exists for it in the artifact (review MINOR 1) — is
**10 596 ms** (spread, 99 stones, incumbent seat — D-74's non-interruptible
first iteration). Against `hang_timeout_ms = 120000` that is a **11.3×** margin.
**That is below the project's ~24× convention and is flagged here rather than at
launch.** The reasoning is now stated at the strength the evidence supports
(review MAJOR 7): **neither bench fixture is the governed workload**, so neither
figure confirms the timeout on the workload it actually guards. `spread_v1` is
not played at all; `bench_positions_v1` is 15- and 35-stone positions and its
worst single median is **491 ms (a 244× margin)**, which is indicative and no
more. **The governed workload is the book**, and the registered run walked it in
the SENS section but emitted no timing there — an observability gap in the
instrument, recorded rather than papered over. **The registered discharge is
therefore the SLOT PASS, and its input is FIXED HERE so its outcome is not
choosable at launch** (review NEW 4 — revision 2 left the sample unspecified,
which makes a gate that can stop the launch also a gate whose result the launcher
selects):

> **THE TIMEOUT PROBE.** Before launch, the ARMED seat
> (`configs/instrument_staged_snk_v0.toml`) is run at the registered budget
> `go nodes 50000` over **the first 50 openings of the governed slice —
> `random_openings_v1.txt` lines `1500..1549`, in file order, none skipped** —
> through the shipped `target/release/pistol`, reading the `time` field off each
> `info totals` line. **The statistic is the MAXIMUM of those 50.** If
> `hang_timeout_ms / max < 24`, the launch STOPS and the margin is reported to
> the architect (D-376's own watchdog rule, whose form this follows). The 50
> openings are part of the governed slice and are played by the run itself, so
> this consumes nothing and biases nothing — it measures the engine's speed, not
> any game's outcome.

**7.8 — the governed run's own revision**: filled at launch.

---

## 8. WHAT FLIPS THIS DOCUMENT

An amendment to any registered value above reopens its review, however small the
diff. The instrument's governing revision moving does the same. **Registered
numbers never move after the run** (D-374): a bracket missed is a finding, and
the change is reverted and the number recorded.

## 9. REVIEW STATE, AND ONE SEQUENCING BREACH RECORDED RATHER THAN LEFT IN THE GIT LOG

**Revision 1 governed the measurement run and was not fresh-context reviewed
before it** (review MINOR 3). CLAUDE.md is explicit: the revision that governs a
run "must itself pass a fresh-context review before the first run it governs".
The session's own addendum sequenced registration → run → slot pass and said
nothing about where the review sits, and I followed the addendum without noticing
it conflicted with the general rule. **Recorded here because the document's face
is where a reader looks, and the git log is not.**

**What the rule protects held, and that is a mitigation rather than an excuse**:
the commit clock shows the instrument (`70cb580`) and the prereg (`4ec470f`)
landed ~52 minutes before the run began; the selection rule is CODE INSIDE the
instrument rather than a step a human took after seeing the table; and no
registered threshold moved — the review verified all three independently.
**A future (B)-shaped package reviews its pre-registration before its
measurement run, not after.**

This revision is **the one fix round**. A second document failure returns the
package to the architect with the report, and no further round is self-granted.
