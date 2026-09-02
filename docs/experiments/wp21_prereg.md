# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 1.

> **ONE LINE FOR THE MORNING.** The whole remaining book — 4 487 openings,
> **~119 800 distinct positions ESTIMATED**, **~63 hours of labelling MEASURED
> per unit** — is partitioned into sixteen tranches run eight at a time; every
> value below is fixed before tranche one, every criterion is quoted here, and
> a tranche that fails one is VOID as a whole rather than repaired.

Governing revision: the WP-2.0b closure head, named in §8 with its digest.
Governing dispatch: the overnight arc II dispatch, Phase 2. This document is
the pre-registration `docs/book_v2_ledger.md`'s rule asks for, and its ledger
row is added in the same commit.

**THE CENSUS IS OFF FOR THIS SWEEP, BY THE DISPATCH'S OWN WORDS** — *"Seat:
committed config, gates OFF, census OFF (D-56p). Labels are the play policy's
output; nothing else."* This supersedes D-562(3)'s *"census ON from game one"*
for this run and for this run only. **What follows from it is stated here so no
closure has to discover it**: this sweep starts no clock against D-537's
minimum, and the census count that arc owes comes from Phase 4's own registered
run over this corpus's positions.

**THE SWEEP TAKES THE WHOLE REMAINING BOOK, WHICH IS A FORK THE DISPATCH
SETTLES AND A CONSEQUENCE IT DOES NOT HIDE.** `docs/experiments/wp21_DISPATCH.md`
lists as open decision (3) *"re-size the sweep so the two standing book
claimants keep a slice"*, and D-562(3) registers the opposite — *"the full
book_v2 yield"*. The overnight dispatch's Phase 2 says *"tranche partition of
the full book_v2 range"*, so **the architect default is applied: the full
range**. What follows is recorded here and in the ledger rather than discovered
later: **after this registration the book is fully claimed**, and the Stage-3
detector's SPRT and the WP-1.5d ±21.5 resolution run — both standing claimants
— have no unconsumed slice of v2 left. They need a successor book or a ruling,
and this document does not give them one.

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
(`docs/book_v2_ledger.md`). This run takes **`13..4499`, 4 487 openings**, in
sixteen tranches:

```
4487 = 7 x 281 + 9 x 280
tranche  1..7   281 openings each
tranche  8..16  280 openings each
skip(1) = 13; skip(n+1) = skip(n) + take(n)
```

The tranche boundaries are arithmetic, not a choice made after seeing anything.

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

ESTIMATED  games      4487 x 2        =   8 974
ESTIMATED  records    4487 x 57.0769  = 256 104
ESTIMATED  distinct   4487 x 26.6923  = 119 768
ESTIMATED  capture    256 104 x 0.8854 = 226 766 s = 62.99 h SERIAL
```

Per tranche, at the larger take of 281 openings:

```
ESTIMATED  games   562        records 16 039        distinct 7 501
ESTIMATED  capture 16 039 x 0.8854      = 14 201 s = 3.94 h
ESTIMATED  play    562 x 0.8271 x 4     =  1 859 s = 0.52 h   (x4: the measured rate is a 4-worker THROUGHPUT and this seat runs one)
ESTIMATED  replay  562 x 0.81           =    455 s = 0.13 h   (from the pilot's 21 s over 26 games)
ESTIMATED  cold    81 x 0.9             =     73 s           (stride 200, see §4)
```

**WALL, at eight concurrent tranches: two waves x ~4.6 h = ~9.2 hours, and that
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

1. **It starts no census and no clock.** §0's ruling.
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
