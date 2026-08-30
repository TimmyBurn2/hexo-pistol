# `book_v2` — size registration, generation receipt, and the instrument that grounds it

**What this registers**: the SIZE of `random_openings_v2.txt`, with grounds; the
seeded, re-executable command that generates it; and the instrument that
measures the grounds, with its dry run. Written **before** the measurement, and
in particular before the size is chosen — §4's decision rule is registered so
that the number cannot be picked after the numbers are seen.

**What this is not**: not a strength claim, not an SPRT, not a config move. No
committed config flips to v2 in this arc (the dispatch's §2); the arena configs
that draw from it are each their own pre-registration and each takes a row in
`docs/book_v2_ledger.md`.

---

## 1. Why the size is a registered quantity at all

D-505 requires `book_v2` be *"seeded, re-executable generation with the command
AND the seed committed; fresh ranges by construction; and a size registered with
its grounds, covering the SPRT's worst-case n — a book sized to the expected n is
a book that runs out mid-run, which is the failure this line exists to prevent."*

D-187 is the precedent and the reason: the v1 book was raised from 500 openings
to 2000 **because the book is the entire sample a paired SPRT can draw on, so it
does not merely size a run — it fixes the design's error rates.** At 500 pairs
the error rates WP-1.3 had declared could not be delivered.

## 2. What the book must cover

Two claimants, and only one is scheduled:

1. **The Stage-3 detector's own SPRT** — one slice of the standing shape (500
   openings, the shape every governed run in Stage 1 used).
2. **The WP-1.5d ±21.5 resolution run** — **LICENSED, NOT SCHEDULED** (D-505),
   and bound by that line to run on `book_v2` under a NEW pre-registration. Its
   predecessor read `nelo_pair +16.9 ci95 21.5` with `llr_pair 1.167` against
   `±2.944` after the full 500-pair cap (D-491, D-492): an interval spanning
   zero, at a bound of `elo1 = 15.0`. **The n that resolves it is the worst case
   this book must cover**, and it is what §3's instrument measures.

## 3. The instrument, and what it measures

`crates/pistol-arena/examples/sprt_power.rs`, at the revision this document is
committed at. A change to it reopens this registration exactly as an amendment
would (`docs/process.md`, instrument governing revision).

**What it does**: simulates the sequential test the arena actually runs — this
crate's own `Sample::of_pairs`, `Unit::Pair` and `crossing`, not a
re-implementation — over pairs drawn from an **exponential tilt of a real
pentanomial**. The bucket shape is a governed run's own `p0..p4`, so the
pair-score variance is play's rather than a coin's; the tilt moves the mean to a
target normalized Elo while keeping that shape. The tilt is used and a
re-weighting of the two extreme buckets is not, because the latter changes the
variance the LLR divides by — the quantity the whole test is denominated in.

**What it reports**: the fraction of runs reaching `h1`, `h0`, and neither
before the pair cap. Under `truth = elo0` the `h1` fraction is the achieved
alpha; under `truth = elo1` it is the achieved power.

### 3.1 The dry run — input of the same kind, and an EXTERNAL referent

**The input**: D-187's own registered configuration — `elo0 = 0`, `elo1 = 25`,
`alpha = beta = 0.05`, at pair caps **500** and **2000**. This is not the
registered workload (§4 measures at `elo1 = 15.0`, the WP-1.5d bound) and
differs from it only in the numbers that identify the case.

**Criterion A — the referent is external, and it is the whole point.** D-187
records, measured by a DIFFERENT harness in a different session:

> at 500 pairs, `elo0 = 0`, `elo1 = 25`, `alpha = beta = 0.05` achieves alpha
> 0.030 and power **0.569** against its own alternative … at 2000 pairs the same
> bounds achieve alpha 0.049 and power 0.945.

The instrument must reproduce all four figures — 0.030 / 0.569 / 0.049 / 0.945
— to within **±0.02 absolute**. *Defect class excluded: a simulation whose
stopping rule, whose tilt, or whose pair accounting is not the one the arena
runs.* A wrong stopping rule or a mis-scaled `t` moves these by far more than
0.02, and it could not land on all four by accident. **This is the criterion
`docs/process.md` asks for first: a value computed by something that does not
share the suspect input.**

**Criterion B — the tilt hits the effect it was asked for.** The instrument
prints `tilted_t` and `target`; they must agree to 1e-6. *Defect class: a
bisection that reports success on a bracket it never entered.*

**Criterion C — degeneracy and refusal.** A `--truth` outside what the bucket
shape reaches is refused by name with exit 1, and `--buckets` that are all zero
likewise. *Defect class: an instrument that cannot say no, and one that clamps a
target it cannot reach and reports the clamp as the answer.*

**Criterion D — determinism.** The same argv twice gives byte-identical output
(CLAUDE.md rule 4). *Defect class: an unseeded simulation whose registered
number cannot be re-checked.*

**REGISTERED CONSEQUENCE OF CRITERION A FAILING, stated before it runs.** A miss
is **not** licence to adopt whichever number is convenient. It means this
instrument's model is not the one D-187 used, and then: the discrepancy is
recorded as a finding with both numbers; the size grounds fall back to the
**LARGER** of (this instrument's answer, the figure D-187's own arithmetic gives
by scaling its 1480-pair 95 %-power figure by `(25/elo1)²`); and the fallback is
named in the ADR line. The larger is taken because the failure this registration
exists to prevent is a book that runs out.

**THE DRY-RUN RECORD**, taken before the sweep, artifact `artifacts/book_v2_power_v1.txt`:

| criterion | referent | observed | verdict |
|---|---|---|---|
| A — alpha at 500 pairs | D-187: **0.030** | **0.0280** | **MET** (Δ 0.002) |
| A — power at 500 pairs | D-187: **0.569** | **0.5672** | **MET** (Δ 0.002) |
| A — alpha at 2000 pairs | D-187: **0.049** | **0.0472** | **MET** (Δ 0.002) |
| A — power at 2000 pairs | D-187: **0.945** | **0.9465** | **MET** (Δ 0.0015) |
| B — the tilt hits its target | `tilted_t` vs `target` | equal to 6 dp at truths 0, 15, 25 | **MET** |
| C — an unreachable target is refused | — | `FAIL: target t 4070.43… is outside what this bucket shape reaches ([-2287.01, 3100.76])`, exit 1 | **MET** |
| C — zero buckets, a short bucket list, zero runs, an unknown option | — | four distinct named refusals, exit 1 each | **MET** |
| D — determinism | the same argv twice | byte-identical | **MET** |

**AND A FIFTH FIGURE THIS DOCUMENT DID NOT REGISTER LANDS TOO, WHICH IS WORTH
MORE THAN THE FOUR THAT DID.** D-187 also records that raising the cap from 500
to 2000 *"raises the EXPECTED pairs played from 374 to about 520"*. The
instrument answers **375.2** and **522.5** — a quantity nothing in criterion A
constrains, from the same run. Four registered figures could in principle be hit
by a model tuned to them; a fifth unregistered one landing to within 1.2 and 2.5
pairs is what says the model IS the one D-187 used. **Criterion A's registered
consequence therefore does not fire**, and the fallback it names is not taken.

## 4. THE SIZE DECISION RULE — registered before the measurement

Let **`P`** be the smallest pair cap, taken from the measured sweep, at which
the WP-1.5d configuration (`elo0 = 0`, `elo1 = 15.0`, `alpha = beta = 0.05`,
bucket shape = D-491's governed pentanomial `30/75/277/68/50`) reaches **power
≥ 0.90** under `truth = elo1`.

Then:

```
n_openings = ceil_to_500( P + 500 )
```

- **`P`** is the resolution run's worst case.
- **`+ 500`** is the Stage-3 detector's own standing slice, which must not
  compete with it for the same openings.
- **`ceil_to_500`** because every slice this project has ever drawn is a
  multiple of 500, and a book whose tail is a stub range nobody can use is
  a book with a smaller usable size than its own header claims.

**The sweep is fixed here too, so `P` is read off a grid chosen before the
numbers**: pair caps `500, 1000, 1500, 2000, 3000, 4000, 5000, 6000, 8000`,
40,000 runs each, seed 1. **Power 0.90 and not 0.95**: D-187 chose 0.95 for a
book that had to cover an unknown future; this book covers two named claimants,
and the 0.95 threshold at `elo1 = 15` costs roughly twice the openings for a
run that is licensed rather than scheduled. **The threshold is registered here,
before the sweep, precisely because moving it afterwards is the post-hoc
threshold move CLAUDE.md forbids.**

**If `P` exceeds 8000** — the grid's top — the size is **not** extrapolated. The
grid is extended in one amendment, reviewed, and re-run; a number off the end of
a measured grid is an estimate wearing a measurement's clothes.

### 4.1 THE MEASURED SWEEP, and `P`

`artifacts/book_v2_power_v1.txt`, the registered grid, `truth = elo1 = 15.0`:

| pair cap | power (`h1`) | alpha-side (`h0`) | inconclusive |
|---|---|---|---|
| 500 | 0.1071 | 0.0059 | 0.8870 |
| 1000 | 0.4017 | 0.0209 | 0.5773 |
| 1500 | 0.6163 | 0.0316 | 0.3522 |
| 2000 | 0.7440 | 0.0374 | 0.2186 |
| 3000 | 0.8731 | 0.0446 | 0.0822 |
| **4000** | **0.9222** | 0.0454 | 0.0324 |
| 5000 | 0.9411 | 0.0468 | 0.0121 |
| 6000 | 0.9471 | 0.0476 | 0.0053 |
| 8000 | 0.9516 | 0.0474 | 0.0010 |

**`P = 4000`** — the smallest cap in the grid at power ≥ 0.90; 3000 answers
0.8731 and is below. Applying §4's rule: `n_openings = ceil_to_500(4000 + 500)`
= **4500**.

**WHAT THE 500-PAIR ROW SAYS ABOUT WP-1.5d, recorded because it is the finding
this measurement was asked for.** At the n the resolution run's predecessor
actually had, the design's power against its own alternative is **0.107**. So
D-491's `inconclusive_at_game_cap` is not a surprise and not a shrug: at 500
pairs and `elo1 = 15.0` the test reaches a boundary about one time in ten
even when the alternative is TRUE. That is the instrument's own account of why
a resolution run needs an order more openings, and it is measured rather than
argued.

**Refused by construction**: `N_OPENINGS_CEILING` is 100,000
(`crates/pistol-cli/src/random_openings/config.rs`), so a size past it is a
named refusal from the generator rather than a silent truncation.

## 5. The other registered generation parameters, and why each is v1's

`k_stones = 5`, `max_radius = 5` — **unchanged from
`configs/random_openings_v1.toml`, deliberately**. The two books must be drawn
from the same population or a slice of one is not comparable with a slice of the
other, and D-175's no-balance-filter argument is arithmetic about `k ≤ 5` that
holds only at that value.

**`seed`**: a NEW seed, and its only job is to be different from v1's
`20260818` so the ranges are fresh by construction. Registered value:
**`20260830`** — the date this package was authored, by the same convention v1
used, and recorded here rather than chosen for any property of the book it
makes.

**Freshness is checked, not asserted**: `the_two_books_share_no_opening` in
`crates/pistol-cli/tests/random_openings_document_tests.rs` compares the two
books' payload lines as sets and requires the intersection to be empty.

## 6. The generation record

| what | value |
|---|---|
| command | `cargo run -p pistol-cli --bin random-openings -- --config configs/random_openings_v2.toml --out-dir crates/pistol-cli/tests/fixtures` |
| config | `configs/random_openings_v2.toml` — `book = "v2"`, `k_stones = 5`, `max_radius = 5`, `seed = 20260830` |
| `n_openings` | **4500** — §4's rule applied to the measured `P = 4000` |
| what the run reported | `openings 4500`, `sampled from 91 cells within 5 of the origin`, `candidates drawn 4505 (5 discarded as a symmetry duplicate)` |
| sha256 of the fixture | `829361a9ae61d0d4369b5291bfc893133fa8160867f11cc638b11f432b6cc29a`, pinned in the test file as `RANDOM_OPENINGS_V2_SHA256` and regenerated byte for byte by `random_openings_v2_is_what_this_build_produces` |
| power measurement | `artifacts/book_v2_power_v1.txt`, digest in `docs/experiments/overnight_export_receipt.md` |
| `random_openings_v1.txt` | **byte-untouched**, mtime unchanged, and still regenerated byte for byte by its own pinning test |

### 6.1 THE OVERLAP WITH v1 — measured, pinned, and NOT eliminated

**Freshness is a claim about RANGES and at that level it is absolute**: the two
books are different files from different seeds, so no range of v2 is a range of
v1 and no governed run over v2 can be a re-read of a consumed sample. That is
what D-505 asks for and it holds by construction.

**At the level of an individual POSITION it is not absolute, and it cannot be.**
Both books draw independently from ONE finite pool. The pool is countable: the
origin is fixed, P2 takes an unordered pair of the remaining 90 cells and P1 an
unordered pair of the 88 left, so there are `C(90,2) × C(88,2) = 15,331,140`
distinct assignments — about **1.28 million** once the twelve lattice symmetries
are folded, which is the granularity each book dedupes at.

| overlap | expected from independent drawing | **MEASURED** |
|---|---|---|
| identical payload lines | `2000 × 4500 / 15,331,140` = **0.59** | **1** |
| positions up to symmetry | `2000 × 4500 / 1,277,595` = **7.04** | **10** |

Both are what chance gives (Poisson: `P(X ≥ 1 | λ=0.59) = 0.44`;
`P(X = 10 | λ=7.04) ≈ 0.08`), and 10 shared positions is **0.22 %** of v2 — about
one position in a 500-opening slice.

**A FILTER AGAINST v1 WAS CONSIDERED AND DECLINED, and the reason is not
convenience.** The generator could reject a candidate whose canonical form
appears in v1, which would make the position-level claim absolute too. It would
also make this file's bytes depend on another file's, and a fixture reproducible
only from its own header is exactly the property D-147, D-152 and D-177 exist to
keep — the property that makes `random_openings_v2_is_what_this_build_produces`
a test rather than an errand. **Re-rolling the seed until the overlap is zero was
also declined**, for a sharper reason: §5 registered that the seed is *"recorded
rather than chosen for any property of the book it makes"*, and choosing it for
zero overlap is choosing it for a property of the book it makes.

**Both counts are PINNED EXACTLY** by
`the_two_books_overlap_only_as_far_as_independent_drawing_makes_them`. A pin
fails on any change; a bound would quietly absorb one.

**`random_openings_v1.txt` is never written by this package.** The generator's
output name is now a closed set keyed by the document's own `[generate] book`,
so a v2 document cannot name v1's file; `the_two_books_are_written_to_different_files`
is the test that says the set has two distinct members. The hazard this replaces
— a compile-time constant that made a `_v2` config overwrite v1's committed
bytes — is recorded in `docs/experiments/stage3_detector_CLOSURE.md` §5.

## 7. What flips or reopens this

- Any change to the instrument (§3) or to a registered number in §4 or §5,
  before the measurement.
- Criterion A failing, which does not reopen the document but does fire its
  registered consequence.
- Nothing about the measured `P` reopens §4: the rule was registered before the
  sweep, and applying it is arithmetic.
