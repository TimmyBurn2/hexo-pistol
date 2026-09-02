# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 2.

> **ONE LINE FOR THE MORNING.** The book's unconsumed range LESS A RESERVED
> 1,000-OPENING HOLDOUT — **3 487 openings**, **~93 100 distinct positions
> ESTIMATED**, **~49 hours of labelling ESTIMATED from a MEASURED per-unit
> rate** — is partitioned into sixteen tranches run eight at a time; every value
> below is fixed before tranche one, every criterion is quoted here, and a
> tranche that fails one is VOID as a whole rather than repaired.

**REVISION 2, AND WHAT MOVED.** Revision 1 took the whole remaining book and
said in its own preamble what that cost: *"after this registration the book is
fully claimed"*, leaving the Stage-3 detector's SPRT and the WP-1.5d resolution run
without a slice. **D-568 settles it the other way**: the LAST 1,000 openings of
`book_v2` are a HOLDOUT reserved for governed runs and are never labelled, and
this sweep takes what remains. Every number in §2 and §3 is re-derived for the
new range rather than scaled in prose, and the generator and its tests move with
them. **AN AMENDMENT REOPENS THE REVIEW however small the diff (CLAUDE.md,
Process), so revision 2 owes a fresh-context review before the first run it
governs, and revision 1's review does not transfer.**

Governing revision: the WP-2.0b closure head, named in §8 with its digest.
Governing dispatch: the overnight arc II dispatch, Phase 2. This document is
the pre-registration `docs/book_v2_ledger.md`'s rule asks for, and its ledger
row is added in the same commit.

**THE RUN IS HELD BY THE OPERATOR AND THIS DOCUMENT DOES NOT LICENSE STARTING
IT.** The operator's words during the resumption: *"wait before you launch the
sweep"*. A registration says what a run WOULD be; it has never been the thing
that starts one, and tranche one waits on the operator whatever else is green.
`docs/experiments/overnight2_ledger.md` §2 carries the hold and the obligations
still owed under it.

**THE CENSUS IS OFF FOR THIS SWEEP, BY THE DISPATCH'S OWN WORDS** — *"Seat:
committed config, gates OFF, census OFF (D-56p). Labels are the play policy's
output; nothing else."* This supersedes D-562(3)'s *"census ON from game one"*
for this run and for this run only. **What follows from it is stated here so no
closure has to discover it**: this sweep starts no clock against D-537's
minimum, and the census count that arc owes comes from Phase 4's own registered
run over this corpus's positions.

**THE SWEEP LEAVES A HOLDOUT, WHICH RETIRES THE FORK REVISION 1 LEFT OPEN.**
`docs/experiments/wp21_DISPATCH.md` lists as open decision (3) *"re-size the
sweep so the two standing book claimants keep a slice"*; D-562(3) registered the
opposite — *"the full book_v2 yield"* — and revision 1 applied it as the
architect default. **D-568 supersedes that for this run**: a contiguous
1,000-opening holdout is reserved and the sweep takes the rest.

**THE RULE THAT FIXES THE HOLDOUT IS STATED HERE, BEFORE TRANCHE ONE, AND IT IS
THE WHOLE PROTECTION**: the holdout is **the LAST 1,000 openings of the book**,
`3500..4499`. It is chosen by position and by nothing else — not by yield, not by
label quality, not by anything this sweep could observe — because a holdout
chosen after seeing which openings label well is not a holdout. It is recorded
in `docs/book_v2_ledger.md` as **RESERVED FOR GOVERNED RUNS**, and **no tranche
may reach it**: `crates/pistol-arena/tests/wp21_tranche_config_tests.rs`'s
`no_tranche_reaches_the_reserved_holdout` drives the shipped generator over all
sixteen tranches and fails if any slice crosses `3500`.

**WHO IT IS FOR, AND WHAT IT IS NOT.** The two standing claimants — the Stage-3
detector's SPRT and the WP-1.5d ±21.5 resolution run — now each have a range to
draw from. **The holdout is not allocated between them here**: dividing it is a
decision for the package that first needs it, and this registration only
guarantees that something is left to divide.

**THE DISPATCH SAYS "3,500" AND THE ARITHMETIC SAYS 3,487, and the difference is
recorded rather than rounded away.** The book holds 4 500 openings; the pilot
consumed `0..12`; the holdout takes `3500..4499`. What is left is
`13..3499` — **3 487 openings**, not 3 500, because the dispatch's round number
is the book's remainder before the pilot's thirteen are subtracted. Every figure
below is derived from 3 487.

---

## 1. THE SEAT, AND WHAT IS FIXED ABOUT IT

| what | value | where it comes from |
|---|---|---|
| engine config, both seats | `configs/instrument_v0.toml` | the pilot's seat, unchanged (D-560's whole cost model is extrapolated from it) |
| solver gate | **off** — `on_search_path = false` at `instrument_v0.toml:113` | the committed value; arming it is D-563's open question and is not taken here |
| game budget | `nodes 50000` | the standing instrument budget, as the pilot |
| **label budget** | `nodes 400000` | the pilot's RULE-2, unchanged; a command-line argument to `arena --capture` and not a config key |
| turn cap | `40` | the pilot's, unchanged. An evaluation horizon and never a game rule (game rule 6) |
| `n_workers` **per tranche** | **1** | eight tranches run at once, and a tranche that also fanned out four ways would oversubscribe the box eight-fold |
| `hang_timeout_ms` | `120000` | the pilot's, unchanged |
| SPRT block | the pilot's | present because the schema requires it. **No strength claim is made**: both seats are one engine, every pair scores alike, and the verdict is `inconclusive_degenerate` by construction (D-156) |

---

## 2. THE PARTITION

The book holds **4 500** openings. `0..12` are consumed by the pilot
(`docs/book_v2_ledger.md`); `3500..4499` are the reserved holdout. This run takes
**`13..3499`, 3 487 openings**, in sixteen tranches:

```
3487 = 15 x 218 + 1 x 217
tranche  1..15  218 openings each
tranche  16     217 openings
skip(1) = 13; skip(n+1) = skip(n) + take(n);  skip(17) = 3500, the holdout's first
```

The tranche boundaries are arithmetic, not a choice made after seeing anything.
The remainder is spread over the FIRST tranches rather than dropped into the
last, so no tranche is materially larger than another and §3's per-tranche wall
holds for every one of them.

**THE CONFIGS ARE GENERATED, NOT HAND-WRITTEN, AND THE REASON IS THE PILOT'S
OWN.** Sixteen near-identical committed documents is the two-documents-one-claim
defect at scale (D-423), and every one of them would carry a `binary_sha256`
that **cannot be true until the closure binary exists** — which is why the pilot
deferred that field to its slot pass. `tools/wp21_tranche_config.py` writes them
from the values in §1 with the tranche's two integers and the measured binary
digest as its arguments; each generated config's own sha256 goes in the run log.
**The generator is the document that fixes the values**, it carries a test
driving the shipped script (`docs/process.md`'s tools/ coverage rule), and an
edit to it reopens this registration.

---

## 3. THE ARITHMETIC, SHOWN

Every per-unit rate is **MEASURED** in the pilot; every total is **ESTIMATED**
by multiplying it out. **The four inputs and where each is actually read**, kept
apart because three of them are in the run log and the fourth is not:
13 openings, 26 games, 742 records and the two wall figures are in
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`; **the 347 distinct positions are NOT**
— that log reports `distinct-n 13`, which is distinct GAMES — and the figure is
read from `docs/experiments/wp20_CLOSURE.md` (*"742 records -> 347 distinct
positions"*, and *"key_seq = key_pos = key_full = 347"*), which is where D-560
reads it too.

```
MEASURED   records per opening     742 / 13   = 57.0769
MEASURED   distinct per opening    347 / 13   = 26.6923
MEASURED   duplication factor      742 / 347  =  2.1383
MEASURED   seconds per label       657 / 742  =  0.8854   (serial, one seat, one channel)
MEASURED   seconds per game        21.505 / 26 = 0.8271   (at n_workers = 4)

ESTIMATED  games      3487 x 2        =   6 974
ESTIMATED  records    3487 x 57.0769  = 199 027
ESTIMATED  distinct   3487 x 26.6923  =  93 076
ESTIMATED  capture    199 027 x 0.8854 = 176 228 s = 48.95 h SERIAL
```

Per tranche, at the larger take of 218 openings:

```
ESTIMATED  games   436        records 12 443        distinct 5 819
ESTIMATED  capture 12 443 x 0.8854      = 11 017 s = 3.06 h
ESTIMATED  play    436 x 0.8271 x 4     =  1 442 s = 0.40 h   (x4: the measured rate is a 4-worker THROUGHPUT and this seat runs one)
ESTIMATED  replay  436 x 0.81           =    353 s = 0.10 h   (from the pilot's 21 s over 26 games)
ESTIMATED  cold    63 x 0.9             =     57 s           (stride 200, see §4)
                                          ------------------
ESTIMATED  tranche                        12 869 s = 3.57 h
```

**WALL, at eight concurrent tranches: two waves x ~3.6 h = ~7.2 hours, and that
is a LOWER BOUND.** The per-label rate was measured with the box otherwise idle;
eight concurrent engines share memory bandwidth and boost budget, and **the
contention factor is not known and is not guessed here**. It is MEASURED from
wave one — wave one's realised seconds-per-label against the pilot's 0.8854 —
and reported in the closure. Nothing in this registration depends on it.

---

## 4. PER-TRANCHE CRITERIA, QUOTED

A tranche passes only if **every** line below holds. The criteria are the
pilot's, at production stride where a stride is affordable and unchanged where
it is not.

| id | criterion, as it must read | the defect it excludes |
|---|---|---|
| **T-A** | `cold_label_check: N of N sampled record(s) agree byte for byte`, exit 0, **stride 200** | a capture whose warm long-lived process answers differently from a fresh one — the claim `newgame` is supposed to make true (D-540). **The referent is EXTERNAL**: a fresh process shares no table with the capture pass |
| **T-B** | `arena: replayed G of G game(s) … 0 divergence(s)`, exit 0, `--workers 1` | a report whose recorded moves are not what its attested engines answer |
| **T-C** | **zero forfeits**: pass 1's summary reports no forfeit, and the corpus's `end` column holds `normal` on every record | a game ended by the driver rather than by the rules |
| **T-D** | `corpus_check: … ok, N record(s)`, exit 0 | a corpus the shipped loader will not read |
| **T-E** | a `capture_manifest` row and a `corpus_manifest` row, each carrying `body_sha256` | an artifact nothing binds (rule 8, D-469) |
| **T-F**, **tranche one only** | a second capture over a **registered sub-range** of tranche one's report is byte-identical to the first over the same range | a capture that is not a function of its inputs. It is a SUB-RANGE and not the whole tranche because a full re-run doubles a 63-hour sweep, and the pilot already ran the whole-corpus form once (`capture-determinism exit=0`) |

**THE SUB-RANGE FOR T-F IS REGISTERED HERE, BEFORE TRANCHE ONE RUNS: the first
20 openings of tranche one** — `openings_skip = 13`, `openings_take = 20` — run
as its own play/capture pair, twice, and compared with `cmp -s`. Twenty is the
smallest take that exceeds the pilot's thirteen, so the check is not weaker than
the one the pilot passed.

**THE VOID RULE.** A tranche that fails any criterion is **VOID AS A WHOLE**:
its artifacts are kept, its ledger row records the failure, and **no part of it
enters the corpus**. A void is not a repair and a tranche is never partially
kept — a corpus assembled from the passing half of a failed tranche is a corpus
whose contents depend on which half failed.

**TWO CONSECUTIVE VOIDS = STOP**, per the dispatch. The second void is not a
third attempt: two in a row says the fault is in the seat or the instrument
rather than in a tranche, and continuing would spend a day producing artifacts
nobody may read.

---

## 4.1 WHERE THE SIXTEEN TRANCHES WRITE, AND WHY IT IS REGISTERED

**REVISION 1 FIXED THE CONFIGS AND NOT THE OUTPUT PATHS**, which is a gap in the
run plan rather than in the correctness argument, and it is closed here.

```
<SWEEP_DIR>/tranche-<n>/report.txt      pass 1's report        (arena --out)
<SWEEP_DIR>/tranche-<n>/capture.txt     pass 2's capture       (arena --capture --out)
<SWEEP_DIR>/tranche-<n>/corpus.txt      pass 3's corpus        (arena --labels --out)
<SWEEP_DIR>/tranche-<n>/tranche-<n>.toml   the generated config
```

**`<SWEEP_DIR>` IS ON `/home` AND NEVER ON `/tmp`.** CLAUDE.md's Environment
section: this machine's `/tmp` is a **24 GiB RAM-backed tmpfs**, and a sweep that
filled it would take every other running command down with it. The sweep's own
output is ESTIMATED in the tens of gigabytes across sixteen tranches.

**A COLLISION IS A REFUSAL AND NOT A CORRUPTION, and that is enforced rather than
trusted**: `--out` is claimed with `create_new`/`O_EXCL`
(`crates/pistol-arena/src/outpath.rs:9-24`), so two runs naming one path is a
named refusal *before any game*, and a refusal before any game removes the empty
claim again. **The per-tranche directory is therefore belt and braces**: the
O_EXCL claim is what makes concurrent tranches safe; the directory layout is what
makes a resume point readable.

---

## 5. THE RUN LOG AND THE RESUME POINT

One run log, appended to, one block per tranche, in the pilot's own shape:
revision, binary digests, then each pass with its exit status and its seconds.
`docs/book_v2_ledger.md` carries **one row for this whole registration** — the
range is consumed by the document, not by the tranche — and the per-tranche
state lives in the run log, which is **the resume point**. A successor reads the
log, finds the last tranche with a PASS or VOID verdict, and starts at the next
one. **A completed tranche is never re-run**: its capture is 4 hours and its
answer is already recorded.

---

## 6. ASSEMBLY, AT THE END AND NOT BEFORE

Two manifests, both **MEASURED** counts and neither estimated:

- the **RAW** manifest: every passing tranche's corpus, with its digest and its
  record count. Raw is the record.
- the **DEDUPED** manifest: one record per distinct position under **D-562(2)'s
  default** — three-key distinct, **the deeper label wins, ties to the first**.
  Deduped is the training input.

**The dedup runs at ASSEMBLY and never at capture** (D-562(2)): a capture that
dropped a duplicate would destroy the 2.14x duplication factor D-560's whole
arithmetic rests on, and the duplication is structural rather than a setting.

Reported at closure, all MEASURED: **records**, **distinct positions**, and
**outcome coverage** — the fraction of records whose game was decided, which is
the subset D-562(1) permits outcome to enter on.

---

## 7. WHAT THIS RUN DOES NOT DO

1. **It starts no census and no clock.** The census ruling in this document's preamble.
2. **It makes no strength claim.** Both seats are one engine (D-156).
3. **It fits no score.** Detector round 3 re-opens on D-537's count, which is
   Phase 4's business and not this sweep's.
4. **It does not settle the solver cap.** That is Phase 3's registered
   experiment, whose records are excluded from this corpus by construction.

---

## 8. THE GOVERNING REVISION AND THE BINARIES

Filled from the closure head before tranche one, and quoted in the run log:
the revision, `target/release/pistol`, `target/release/arena`,
`target/release/corpus-check`, `tools/cold_label_check.py` and
`tools/wp21_tranche_config.py`, each with its sha256. An instrument that
produces a registered number is named with its revision (`docs/process.md`), and
a change to any of them reopens this registration.
