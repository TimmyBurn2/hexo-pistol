# WP-1.5d (B) — PRE-REGISTRATION: the safety-net cap's calibration, bench and SPRT

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
It spans widely on purpose: if the benefit does not decay across it, the rule
selects the largest point and **that outcome is a finding about the channel**
rather than a calibration, recorded as such.

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

**THE BOOK IS NEARLY SPENT AND THIS DOCUMENT SAYS SO**: after this run every
slice of `random_openings_v1.txt` is consumed, and the next SPRT in this project
needs a regenerated or extended book. Recorded here because a later session
should not discover it at launch.

---

## 5. OUTCOME HANDLING, WRITTEN BEFORE GAME ONE

| outcome | what happens |
|---|---|
| calibration selects no K | no SPRT. The package closes as a measured finding; the gate stays `0` |
| corpus ABORT fires | no SPRT, whatever the calibration said. The package closes on the bench |
| SPRT `h1` | the committed config moves to the selected K, the closure pin is re-recorded with digests, and the 1.8 arc's re-test clause is considered if a material nps jump landed |
| SPRT `h0` | the gate stays `0`. A measured finding, not a failure; the mechanism stays landed and oracle-gated |
| `inconclusive_at_game_cap` | reported as such; no config moves |
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

- **SLOT 6.1** — `SENS/TRAJECTORY=incumbent`: ____ searches, ____ safety-net-bearing, ____ diverged.
- **SLOT 6.2** — `SENS/TRAJECTORY=capped`: ____ searches, ____ safety-net-bearing, ____ diverged.
- **SLOT 6.3** — the reading: if the divergence rate on both trajectories is low
  enough that 500 pairs cannot see it, **this document says so before game one
  and the honest expectation is `h0`-or-inconclusive**, with the value living on
  the bench rather than being claimed as strength. No side channel promotes a
  bench number to a strength claim (rule 6).

---

## 7. FILL-IN SLOTS — the slot pass (D-427)

Filled at the run's own launch, because a document and its configs drift between
revisions and D-427 is the record of what that costs.

- **7.1** artifact `artifacts/wp15d_b_measurement_v1.txt` sha256 ____
- **7.2** `CAL/SELECTED` line, quoted verbatim ____
- **7.3** the corpus bench ratio, and the bracket verdict ____
- **7.4** the spread report, ON-seat `safety_net_capped_rows` per position ____
- **7.5** `binary_sha256` for both seats, confirmed live ____
- **7.6** the arena config's `openings_skip`, read from the file and compared against §4's **1500** ____
- **7.7** `hang_timeout_ms` margin against §3's worst single median ____
- **7.8** the governed run's own revision ____

---

## 8. WHAT FLIPS THIS DOCUMENT

An amendment to any registered value above reopens its review, however small the
diff. The instrument's governing revision moving does the same. **Registered
numbers never move after the run** (D-374): a bracket missed is a finding, and
the change is reverted and the number recorded.

## 9. REVIEW STATE

Fresh-context review at the revision that governs the run, **one fix round**. A
second document failure returns the package to the architect with the report, and
no further round is self-granted.
