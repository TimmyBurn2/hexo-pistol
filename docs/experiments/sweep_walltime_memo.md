# The sweep's wall time — where it goes, and the three levers. A MEMO, not a registration.

**WHAT THIS IS.** The operator asked what the sweep will cost in wall time, part
by part, whether making the engine faster first would help, and whether running
tranches concurrently can conflict or duplicate work. This memo answers those
three from the code and from the pilot's MEASURED rates. **It registers nothing**
— `docs/experiments/wp21_throughput_prereg.md` is the registration for the two
levers that need measuring, and this memo is the arithmetic behind it.

---

## 1. WHERE THE TIME GOES

Every per-unit rate below is **MEASURED** in the WP-2.0 pilot
(`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`, `docs/experiments/wp20_CLOSURE.md`);
every total is **ESTIMATED** by multiplying it out.

```
MEASURED  records per opening   742 / 13    = 57.0769
MEASURED  distinct per opening  347 / 13    = 26.6923
MEASURED  duplication factor    742 / 347   =  2.1383
MEASURED  seconds per label     657 / 742   =  0.8854   serial, one channel
MEASURED  seconds per game      21.505 / 26 =  0.8271   at n_workers = 4
```

**ONE TRANCHE — 218 openings, 436 games, 12 443 records:**

| pass | ESTIMATED | share |
|---|---|---|
| 1, play | 1 442 s = 0.40 h | 11.2% |
| **2, capture (labelling)** | **11 017 s = 3.06 h** | **85.6%** |
| 3, replay | 352 s = 0.10 h | 2.7% |
| cold check | 57 s | 0.4% |
| **tranche total** | **12 869 s = 3.57 h** | |

**THE SWEEP IS ONE THING WEARING FOUR NAMES.** Capture is 85.6% and every other
pass is engine time too, so **~97% of this sweep is the engine searching under a
NODE budget**. A lever that is not on that is not a lever: worker counts, replay
concurrency and the cold stride together cannot move 3% of the total.

**AGGREGATE, sixteen tranches, at contention 1.00 — a LOWER BOUND:**

| concurrency | waves | ESTIMATED wall |
|---|---|---|
| 1 | 16 | 57.19 h |
| 2 | 8 | 28.60 h |
| 4 | 4 | 14.30 h |
| **8 (registered)** | **2** | **7.15 h** |
| 16 | 1 | 3.57 h |

**CONTENTION IS NOT IN THOSE NUMBERS AND IS NOT GUESSED HERE.** The 0.8854 s rate
was measured with the box otherwise idle. Eight concurrent engines share L3 and
memory bandwidth; the realised factor is what
`wp21_throughput_prereg.md` §3 measures. **A realistic 8-way figure is the table
value times that factor**, and nothing in the registered plan depends on it.

---

## 2. LEVER ONE — THE LABEL CACHE. The big one, and it is not an engine change.

`capture::run` asks the engine at **every asked prefix of every game**. The
pilot's own run log reads `n 26  distinct-n 13  (13 duplicate games)` —
**exactly 2.0 at the game level, and structural**: `arena --capture` refuses a
report whose two seats do not attest one engine, so a self-match is the only
capturable shape, and one deterministic engine plays each opening identically in
both seats. With cross-game transpositions on top, the pilot measured
**742 records over 347 distinct positions = 2.1383**.

**IT IS SOUND BECAUSE OF SOMETHING THE CODE ALREADY DOES.** `capture::ask` sends
`newgame` before every `position`/`go`; `new_game` calls `Searcher::clear`, which
clears the transposition table, the heuristics and the solver. The binary's own
usage text says it: *"One `newgame` precedes every ask, so no label is produced on
a table another ask warmed."* **No state crosses an ask**, so a label is a pure
function of (position, budget, binary, config) and re-using one is re-use rather
than approximation.

**IT DOES NOT TOUCH D-562(2).** That ruling forbids DROPPING a duplicate at
capture, because dropping one destroys the 2.14x duplication factor D-560's
arithmetic rests on. **A cache drops no record** — every record is still written,
with the same bytes — so the duplication survives because the duplication
survives.

| | capture/tranche | tranche | 8-way wall | serial total |
|---|---|---|---|---|
| registered | 11 017 s | 3.57 h | **7.15 h** | 48.95 h |
| with the cache | 5 152 s | 1.95 h | **3.89 h** | 22.89 h |

---

## 3. LEVER TWO — MAKING THE ENGINE FASTER. What it is worth, exactly.

**THE ARITHMETIC IS EXACT BECAUSE THE BUDGET IS NODES.** A label is 400 000
nodes; wall is `nodes / nps`. So an nps gain of `g` multiplies ~97% of the sweep
by `1/(1+g)`:

| nps gain | ESTIMATED 8-way wall | speedup |
|---|---|---|
| +5% | 6.81 h | 1.05x |
| +10% | 6.50 h | 1.10x |
| +20% | 5.96 h | 1.20x |
| +50% | 4.77 h | 1.50x |
| **+113.8%** | **3.34 h** | **2.14x — the point where an engine change equals the cache** |

**SO THE HONEST COMPARISON IS: the cache is worth a doubling of engine speed.**
No plausible micro-optimisation is worth a doubling.

### 3.1 AND THE TWO KINDS OF SPEED CHANGE COST COMPLETELY DIFFERENT THINGS

**(a) A change that does NOT move move-choice is nearly free of evidence cost.**
This project already owns the instrument that proves it: `tools/baseline_snapshot.sh`
plus `wp20b_design.md` §9's extraction rule — the byte-identity check run today,
which showed both registered referents MATCHING at the post-remedy binary. A
speed change that still MATCHES both referents changes no label, so the corpus it
produces is the same corpus.

**(b) A change that moves move-choice is a different engine.** It changes every
label, so it re-registers the sweep; it needs SPRT over paired balanced openings
(CLAUDE.md rule 6); and it invalidates the pilot's measured rates, which is what
the whole arithmetic above is extrapolated from.

### 3.2 THE ONE (a)-SHAPED LEVER THAT IS SITTING UNCLAIMED

`Cargo.toml`'s `[profile.release]` sets **`overflow-checks = true` and nothing
else**. There is no `lto`, no `codegen-units`, no `panic` setting — Rust's
defaults are `lto = false`, `codegen-units = 16`. The comment says why:

> *"Still deliberately no codegen tuning here — that is what D-14 defers, and it
> stays deferred until there is a bench to judge it by (CLAUDE.md rule 5)."*

**D-14's own words name the condition, and it is now met**: *"there is no code to
bench yet — the first benched work package, which sets them with numbers
attached."* `tools/bench_delta.sh` exists and carries a hotspot, a bracket and an
abort threshold.

**AND THE CLASSIC HAZARD IS ABSENT, MEASURED RATHER THAN ASSUMED.** LTO and
`codegen-units = 1` change results when floating-point reassociation moves a
number. `/usr/bin/grep -rn "f32\|f64"` over `pistol-core/src`, `pistol-eval/src`,
`pistol-search/src` and `pistol-solver/src` returns **two hits, both in
`pistol-solver/src/bin/solver-cost.rs`** — a cost-reporting binary, not the
move-choice path. **The engine's search and eval are integer throughout**, so
codegen tuning cannot change an answer through arithmetic, and the byte-identity
instrument would catch it if something else did.

**WHAT IT WOULD COST**: hard rule 5's discipline — a pre-registered hotspot, an
expected gain bracket, an abort threshold, one change, one commit, one IQR-gated
bench reporting nps AND time-to-depth. Plus longer builds, which the
mutation harness pays 31 times a run.

**WHAT IT MIGHT BUY**: **UNKNOWN, and this memo will not guess it.** Typical
figures for `lto = "fat"` + `codegen-units = 1` on compute-heavy Rust are in the
5-20% range; this codebase's own prior is discouraging — D-223 retired a
profiling package that found pistol-eval's inlined map a dead end. **The bracket
is the pre-registration's to set, not this memo's.**

### 3.3 WHAT IS NOT A LEVER, SAID SO NOBODY SPENDS A DAY ON IT

- **Turning `overflow-checks` off.** MEASURED at **1.1-1.3%** (D-127) and it is
  the one class of error a fail-loud project cannot otherwise catch. Not worth
  1.2% and not this memo's call anyway.
- **Keeping the transposition table warm across label asks.** It would be free
  speed only if warm answers equal cold ones — and the whole point of criterion
  **T-A**'s cold check is that this is verified per tranche rather than assumed.
  Changing it would make every label depend on ask order.
- **A bigger `tt_bytes`.** Under a NODE budget a bigger table does not change how
  many nodes are searched; it changes what they find. That is a move-choice
  change, i.e. class (b).

---

## 4. LEVER THREE — CONCURRENCY, AND WHETHER IT CAN CONFLICT

### 4.1 THE CONFLICT ANALYSIS, ITEM BY ITEM

| shared thing | conflict? | why, from the code |
|---|---|---|
| the report / capture / corpus paths | **no, and it is ENFORCED** | `--out` is claimed with `create_new`/`O_EXCL` (`outpath.rs:9-24`). Two runs naming one path is a NAMED REFUSAL before any game, never a silent overwrite. A refusal before any game removes the empty claim again |
| temp / scratch files | **none exist** | `/usr/bin/grep -rn "temp_dir\|/tmp\|tempfile"` over `pistol-arena/src`, `pistol-cli/src`, `pistol-engine/src` returns nothing |
| the census file | **no** | derived from `--out` (`outpath.rs::census_path`), so distinct wherever `--out` is. And this sweep runs census-OFF |
| the binary, configs, weights, the book | **no** | read-only. `identity::verify_respawn` re-reads them per game; concurrent reads do not conflict |
| memory | **no** | `tt_bytes = 268435456` = 256 MiB per engine process. Play seats TWO processes per tranche, capture ONE. Peak with 8 tranches all in play: 16 x 256 MiB = **4 GiB**. At 16 tranches, 8 GiB. The box has **46 GB** |
| CPU | **the real constraint** | only one engine per tranche SEARCHES at a time — the referee alternates — so a tranche is ~1 active core. 8 tranches = 8 cores = this box's **8 physical cores**; 16 would use SMT siblings |
| the hang watchdog | **no, with two orders of headroom** | `hang_timeout_ms = 120000` against a MEASURED **0.885 s** per label |

### 4.2 CAN CONCURRENCY CHANGE A LABEL?

**Structurally, no.** Every budget in this sweep is a NODE budget — `nodes 50000`
for games, `--label-nodes 400000` for labels. Machine load cannot change a
node-budgeted answer; that is CLAUDE.md hard rule 4 and `tools/determinism.sh` is
its gate. **The throughput study registers C1 to TEST it rather than assert it**,
because "the labels cannot depend on load" is exactly the kind of claim that is
true until it is not, and its registered consequence is that **the SWEEP stops,
not the study**.

### 4.3 CAN CONCURRENCY PRODUCE DUPLICATES?

**Across tranches: no, and it is pinned by two tests.** The partition is
contiguous, disjoint and exhaustive over `13..3499`
(`the_sixteen_tranches_partition_the_books_unconsumed_range_exactly`), and no
slice reaches the holdout (`no_tranche_reaches_the_reserved_holdout`). No opening
is labelled by two tranches, whatever order they run in.

**Within a tranche: yes, 2.14x, and it is the corpus's declared shape** rather
than a defect — D-558(2) structural, D-562(2) deduped at assembly and never at
capture. That is lever one's whole opportunity.

### 4.4 THE FINDING: THE CONCURRENCY MUST DIVIDE THE TRANCHE COUNT

Wall is `ceil(16 / N)` waves. **N = 12 is strictly no better than N = 8**: it runs
twelve tranches and then four, leaving two thirds of the box idle for the entire
second wave. Only `N ∈ {1, 2, 4, 8, 16}` gives balanced waves at sixteen
tranches. **If the concurrency changes, the tranche count changes with it** —
registered in `wp21_throughput_prereg.md` §3.2 and §3.4.

---

## 5. THE THREE LEVERS TOGETHER

| plan | ESTIMATED 8-way wall | vs registered |
|---|---|---|
| registered | 7.15 h | 1.00x |
| engine +10% nps | 6.50 h | 1.10x |
| engine +20% nps | 5.96 h | 1.20x |
| **label cache** | **3.89 h** | **1.84x** |
| cache + engine +10% | 3.54 h | 2.02x |
| cache at 16 concurrent, SMT worth +30% aggregate | 2.99 h | 2.39x |

**THE ORDER TO TAKE THEM IN, AND THE REASON IS RATIO RATHER THAN SIZE.**

1. **The cache.** Largest gain, and its correctness criterion is byte-identity
   against an uncached run — an external referent that shares no code with it.
2. **Concurrency.** No code, and the measurement is an hour.
3. **The engine.** Real, permanent, and worth a package **on its own merits** —
   it makes every future run faster, not just this sweep. But as a wall-time
   lever for THIS sweep it has the worst ratio of uncertain gain to review cost,
   and it would need a doubling to match what the cache gives for free.

**AND THE HONEST TOTAL**: even at 2.39x the sweep is **three hours**, against a
registered 7.15 h that is itself a lower bound. **None of these levers is the
difference between feasible and infeasible.** They are the difference between
one evening and two.
