# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 10.

> **ONE LINE FOR THE MORNING.** The book's unconsumed range less a reserved
> 1 000-opening holdout — **3 487 openings**, **~93 076 distinct positions
> ESTIMATED**, **~49 hours of serial labelling ESTIMATED from MEASURED per-unit
> rates** (§3) — is partitioned into sixteen tranches run **N at a time, N a
> REGISTERED SLOT with incumbent 8** (§1); every value is fixed before tranche
> one, every criterion is quoted (§4), and a tranche that fails one is **VOID as
> a whole and RE-RUN WHOLE**, never repaired.

**GOVERNING**: the arc III GROUNDWORK dispatch (§6 of it: cache ON, 3 487
openings, the holdout reserved, box idle during tranches, tranches detached and
polled) and the overnight arc II dispatch's Phase 2 as `overnight2_ledger.md` §2
carries it; D-537, D-560, D-562, D-568, D-576, D-581, D-584, D-586. The sibling
study `wp21_throughput_prereg.md` owns the cache's criterion and the worker
count; the cache design `wp21_label_cache_design.md` owns the cache. **This
document's revision history is in `arc3_ledger.md` and nowhere here.**

**THE HOLD IS SPENT.** The prior arc's *"wait before you launch the sweep"* was a
mid-arc instruction to the arc it interrupted; the arc III dispatch names the
sweep in its title, amends its parameters and asks for its output at closure
(`arc3_ledger.md` F-0.4). Tranche one runs when §9.1's dry run and this document's
review are green, and not before.

**THE CENSUS IS OFF FOR THIS SWEEP — D-596**, which records the dispatch's *"gates
OFF, census OFF"* as the ADR line hard rule 10 asks for and gives its grounds (a
gate-off seat records nothing, D-563; the cache refuses a census by name). It
supersedes D-562(3) for this run only: the sweep starts no clock against D-537's
minimum, and the census count the arc owes comes from the census run over this
corpus's positions (arc III §6).

**THE HOLDOUT IS THE LAST 1 000 OPENINGS OF THE BOOK, `3500..4499`**, chosen by
position and by nothing this sweep could observe (D-568), recorded in
`docs/book_v2_ledger.md` as RESERVED FOR GOVERNED RUNS, and out of every
tranche's reach by a test that drives the shipped generator over all sixteen
(`crates/pistol-arena/tests/wp21_tranche_config_tests.rs`,
`no_tranche_reaches_the_reserved_holdout`). **D-568 says 3 500 and the arithmetic
says 3 487**: the book holds 4 500, the pilot consumed `0..12`, the holdout takes
`3500..4499`, and `13..3499` is 3 487 openings. The ruling's round number is the
book's remainder before the pilot's thirteen; every figure here is derived from
3 487. `overnight2_ledger.md` §2 records the same reading and is a record, left as
written; `docs/book_v2_ledger.md`'s row for this sweep names this revision.

---

## 1. THE SEAT, AND WHAT IS FIXED ABOUT IT

| what | value | where it comes from |
|---|---|---|
| engine config, both seats | `configs/instrument_v0.toml` | the pilot's seat, unchanged; D-560's whole cost model is extrapolated from it |
| solver gate | **off** — `on_search_path = false` at `configs/instrument_v0.toml:113` | the committed value; arming it is D-563's open question and is not taken here |
| game budget | `nodes 50000` | the standing instrument budget, as the pilot |
| **label budget** | `nodes 400000` | the pilot's, unchanged; a command-line argument to `arena --capture`, never a config key |
| turn cap | `40` | the pilot's; an evaluation horizon and never a game rule (game rule 6) |
| `n_workers` **per tranche** | **1** | N tranches run at once, and a tranche that also fanned out four ways would oversubscribe the box |
| **concurrent tranches** | **16 — the slot, FILLED** | by `wp21_throughput_prereg.md` lever A under its own §3.4 rule, run before tranche one on the idle box: median throughputs 1.098, 2.177, 4.353, 7.985, **10.314** over N = 1, 2, 4, 8, 16, so the smallest N at or above 95 % of the highest (9.798) is 16 and the incumbent 8 sits at 77.4 %. C1 (93 of 93 capture files byte-identical), C2 (93 of 93 at the report's own 152 asked prefixes, every `rc` 0), C3 (1.5514 s per label, `c(16) = 1.7521`, bar 3.54 s) and C4 (N = 1's median rep is rep 1 at 0.910660 s, +2.85 % on the pilot's 0.885445, bar 20 %) all PASS. **The median rep is taken on the exact throughput** `N x records / wall_s`, not on the harness's three-decimal printing, under which reps 1 and 3 tie at 1.098; exactly, rep 3 is 1.098020, rep 1 is 1.098107 and rep 2 is 1.099330, so rep 1 is the median and the two candidates differ by 0.008 % of throughput and 0.01 points of C4's twenty. Receipt `artifacts/arc3r_leverA_c68e69e.txt`, sha256 `5eb62d6c909699d76078b96f71801741d544fda98c3a58cbb07a83209eeeef01`. **The answer was not the incumbent, so it reopened this document; revision 8 is that amendment** — §3's wall and §6.1 are re-derived at one wave |
| **label cache** | **OFF on every tranche of this sweep** | D-576; the design is `wp21_label_cache_design.md` revision 10. The gate is unchanged and at the selected N = 16 it admits no tranche, for the reason §6.1 gives and which is not restated here |
| `hang_timeout_ms` | `120000` | the pilot's |
| SPRT block | the pilot's | present because the schema requires it. No strength claim: both seats are one engine and the verdict is `inconclusive_degenerate` by construction (D-156) |

---

## 2. THE PARTITION

The book holds **4 500** openings. `0..12` are consumed by the pilot
(`docs/book_v2_ledger.md`); `3500..4499` are the holdout. This run takes
**`13..3499`, 3 487 openings**, in sixteen tranches:

```
3487 = 15 x 218 + 1 x 217
tranche  1..15  218 openings each
tranche  16     217 openings
skip(1) = 13;  skip(n+1) = skip(n) + take(n);  skip(17) = 3500, the holdout's first
```

The boundaries are arithmetic, not a choice made after seeing anything; the
remainder goes to the first tranches so no tranche is materially larger than
another and §3's per-tranche wall holds for every one.

**THE CONFIGS ARE GENERATED, NOT HAND-WRITTEN.** Sixteen near-identical committed
documents is D-423 at scale, and each would carry a `binary_sha256` that cannot be
true until the closure binary exists. `tools/wp21_tranche_config.py` writes them
from §1's values with the tranche's two integers and the measured binary digest as
its arguments, prints each generated config's own sha256 for the run log, carries a
test driving the shipped script (`docs/process.md`'s coverage rule), and an edit to
it reopens this registration. **Its window form (`--skip/--take`) writes T-F's
sub-range and, under `--pilot-range`, the dry run's stand-in from the pilot's
consumed `0..12`** — a range a dry run may re-read because it yields no corpus and
spends no unseen opening (D-539). Both forms refuse the holdout.

---

## 3. THE ARITHMETIC, SHOWN

Every per-unit rate is **MEASURED** in the pilot; every total is **ESTIMATED** by
multiplying it out. **THE NINE INPUTS, AND WHERE EACH IS READ.**
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt` carries seven: 13 openings, 26 games
(`n 26`), 742 records, the capture wall (`capture1 seconds=657`), the play wall
(`wall 21505 ms at 4 workers`), the replay wall (`replay seconds=21`) and the cold
wall (`cold seconds=671`, stride 1 over 742). The **347 distinct positions are
NOT there** — that log's `distinct-n 13` is distinct GAMES — and are read from
`docs/experiments/wp20_CLOSURE.md` (*"742 records -> 347 distinct positions"*).
The **hit rate 0.5323** is `artifacts/arc3_leverB_41_count_v3.txt`, taken by
`tools/label_cache_count.py` under the cache's own key. **One rounding of each
rate, the unrounded quotient, is used in every line below**:

```
MEASURED   records per opening      742 / 13      = 57.0769
MEASURED   distinct per opening     347 / 13      = 26.6923
MEASURED   seconds per label        657 / 742     =  0.885445   (warm, one seat, one channel)
MEASURED   seconds per cold sample  671 / 742     =  0.904313   (one fresh process per sample)
MEASURED   seconds per game         21.505 / 26   =  0.827115   (at n_workers = 4)
MEASURED   seconds per replayed game 21 / 26      =  0.807692   (at --workers 4)
MEASURED   hit rate under the cache key           =  0.5323     (395 / 742)

ESTIMATED  games     3487 x 2                     =   6 974
ESTIMATED  records   3487 x 57.0769               = 199 027
ESTIMATED  distinct  3487 x 26.6923               =  93 076
ESTIMATED  capture   199 027 x 0.885445           = 176 227 s = 48.95 h SERIAL
```

Per tranche, at the larger take of 218 openings:

```
ESTIMATED  games 436     records 12 443     distinct 5 819     hits 6 624
ESTIMATED  capture  12 443 x 0.885445            = 11 018 s = 3.06 h
ESTIMATED  play     436 x 0.827115 x 4           =  1 442 s = 0.40 h   (x4: the measured rate is a 4-worker throughput and this seat runs one)
ESTIMATED  replay   436 x 0.807692 x 4           =  1 409 s = 0.39 h   (x4 for the same reason; T-B mandates --workers 1)
ESTIMATED  cold     (34 + 30) x 0.904313         =     58 s             (two strides, §4: ceil(6624/200) + ceil(5819/200))
                                                   ------------------
ESTIMATED  tranche                                 13 927 s = 3.87 h
```

**THE WALL, STATED ONCE HERE AT THE SELECTED N = 16, AND EVERY OTHER SECTION
POINTS AT IT** (D-423). Sixteen tranches at once is ONE wave, so the sweep's wall
is one tranche's; and the capture term is no longer estimated at the pilot's
serial rate but MEASURED at the concurrency the sweep runs at — lever A's median
rep at N = 16, **1.551364 s per label** (`c(16) = 1.7521`):

```
MEASURED   seconds per label at N = 16                       =  1.551364   (lever A, §1)
ESTIMATED  capture   12 443 x 1.551364                       = 19 304 s = 5.36 h
ESTIMATED  play      436 x 0.827115 x 4                      =  1 442 s = 0.40 h
ESTIMATED  replay    436 x 0.807692 x 4                      =  1 409 s = 0.39 h
ESTIMATED  cold      (34 + 30) x 0.904313                    =     58 s
                                                                ------------------
ESTIMATED  one tranche at N = 16, and the sweep is one wave   = 22 213 s = 6.17 h

for comparison, at the incumbent N = 8 (MEASURED 1.001925 s per label, two waves):
           capture 12 443 x 1.001925 = 12 467 s; tranche 15 376 s; 2 waves = 30 752 s = 8.54 h
SERIAL, uncached  3 487 x 57.0769 x 0.885445                 = 176 228 s = 48.95 h
```

**THE CACHE BUYS NOTHING ON THIS SWEEP AND IS NOT USED BY ANY TRANCHE OF IT**
(§6.1): one wave has no second wave to serve, and the gate cannot return before
the wave it would have to precede. The arithmetic that priced a cached second
wave at a net 0.20 h is gone with the second wave, and the sibling's ONE LINE
already says the lever is taken for a verified capability rather than for this
sweep's wall.

**AND TRANCHE ONE CARRIES A SURCHARGE THE WALL ABOVE DOES NOT HOLD**: the cached
re-capture — 5 819 misses at the pilot's rate, `5 819 x 0.885445 = 5 152 s =
1.43 h`, taken AFTER the wave as §4.4's capability verification and on no
tranche's critical path — plus T-F's sub-range pair
(§4.1): `2 x (20 x 57.0769 x 0.885445) = 2 x 1 011 s = 0.56 h` of capture and
`2 x 40 x 0.827115 x 4 = 265 s = 0.07 h` of play — **2.06 h in all**. T-F's 0.63 h
runs BEFORE the wave, the box otherwise idle, because the wave's realised
seconds-per-label is read for contention and a job beside it would be in that
reading; the referent re-capture is taken AFTER the wave, on no tranche's
critical path.

**THE CAPTURE TERM IS MEASURED AT THE SWEEP'S OWN CONCURRENCY; THE OTHER THREE
ARE LOWER BOUNDS.** Capture is 87 % of the tranche above and its rate is lever A's
median rep at N = 16, taken with sixteen captures actually running — so it is not
an idle-box rate extrapolated, and `c(16) = 1.7521` is measured rather than
guessed. **Play, replay and the cold check keep their uncontended rates** (four
workers, an idle box) and are therefore lower bounds, worth `1 442 + 1 409 + 58
= 2 909 s = 0.81 h` of the 6.17 h, which is also 6.17 less the capture term's
5.36;
the wave's own realised seconds-per-label is reported at closure against both.
Nothing in this registration's criteria, partition, seat or budget depends on
any of it.

---

## 4. PER-TRANCHE CRITERIA, QUOTED

A tranche passes only if **every** line holds. The criteria are the pilot's, at
production stride where a stride is affordable and unchanged where it is not.

| id | criterion, as it must read | the defect it excludes |
|---|---|---|
| **T-A1** | `cold_label_check: N of N sampled MISSES record(s) agree byte for byte`, exit 0, **stride 200 over cache MISSES** | a capture whose warm long-lived process answers differently from a fresh one — the claim `newgame` is supposed to make true (D-540). **In force on every tranche.** The referent is EXTERNAL: a fresh process shares no table with the capture pass and no cache at all |
| **T-A2** | `cold_label_check: N of N sampled HITS record(s) agree byte for byte`, exit 0, **stride 200 over cache HITS** | **on a CACHED tranche**: a cache that answers a hit differently from the search that produced the miss. **On an UNCACHED tranche the same invocation is in force under T-A1's class**: a hit record there is the second in-process ask of an identical `position` line, which is exactly the shape gate 9 never takes (D-581), and a second ask disagreeing with a fresh process is a warm/cold defect whatever produced it. **In force on all sixteen, and the closure reports the two counts separately** |
| **T-B** | `arena: replayed G of G game(s) … 0 divergence(s)`, exit 0, `--workers 1` | a report whose recorded moves are not what its attested engines answer |
| **T-C** | **zero forfeits**: the report's own `counts n … forfeits N` line (`crates/pistol-arena/src/conclusion.rs:81`) and its `first_player_wins … forfeits N` line (`:111`) both read zero, and the corpus's `end` column holds `normal` on every record, readable off `corpus_check`'s own `end 1 (normal)` | a game ended by the driver rather than by the rules |
| **T-D** | `corpus_check: … ok, N record(s)`, exit 0 | a corpus the shipped loader will not read |
| **T-F**, **before tranche one** | over a separate play of the **registered sub-range** — openings `13..32`, the first twenty of tranche one's range — two captures are byte-identical (`cmp -s` exit 0), AND that play's capture records equal, record for record, tranche one's own records over the same twenty openings (`cmp -s` over the two bodies, headers aside, exit 0), because play is deterministic and the sub-range's games ARE tranche one's first forty | a capture that is not a function of its inputs; the second half is the external referent — two runs of one report agreeing is the same instrument twice. A SUB-RANGE because a full re-run doubles a 49-hour sweep, and the pilot already ran the whole-corpus form once (`capture-determinism exit=0`). **REGISTERED CONSEQUENCE: a T-F failure is the sibling's C1 class — THE SWEEP STOPS**, and no tranche is re-run to cure it, because a capture instrument that is not a function of its inputs is not cured by running it again. **AND WHAT THAT COSTS DIFFERS BETWEEN THE HALVES AT N = 16, WHICH IS A COST OF THE SELECTED N AND IS RECORDED RATHER THAN HIDDEN**: the first half runs BEFORE the wave and stops it before a second of sweep compute is spent; the second half needs tranche one's capture, which in a single wave exists only when the sweep is over, so it cannot save compute and instead VOIDS the corpus the sweep just produced — nothing is delivered and no tranche is re-run. At the incumbent N = 8 the second half would have stopped the sweep between the waves and saved half of it; that saving is gone with the second wave |

**HOW T-A TELLS A HIT FROM A MISS, WITHOUT A NEW COLUMN.** Walk the records in
order; a record whose `position` field has not been seen before is a MISS, every
later record with that field is a HIT. The capture grammar gains no field — a new
column would be a format version bump (D-572) for a fact the file already
determines — and the partition is taken on cached and uncached tranches alike, so
every tranche's numbers have one shape. `tools/cold_label_check.py` takes a
**required** `--partition hits|misses|all` (required, not defaulted: a default
would answer about `all` while a criterion said `hits`) and names the class on its
summary line, so two invocations cannot be mistaken for one class sampled twice:

```
tools/cold_label_check.py --capture <path> --binary <path> --engine-config <path> \
                          --stride 200 --partition hits
cold_label_check: 34 of 34 sampled HITS record(s) agree byte for byte
```

**AN UNDER-FILLED CLASS IS A VOID, NOT A PASS.** `partitioned()` voids an empty
class; a filter over an already-read list would otherwise print `0 of 0 … agree`
and exit 0, the vacuous pass `docs/process.md` forbids. **REGISTERED: fewer than
ten SAMPLED records in the class makes the invocation a VOID (exit 2)**, and a
void T-A voids the tranche. **The floor is in samples, so the stride must fit the
capture**: a full tranche at stride 200 samples 34 hits and 30 misses (§3); the
floor therefore binds only a capture smaller than 1 801 records per class, which
no tranche is. T-A runs on the sixteen tranche captures and on nothing else: T-F's
two captures are compared with `cmp -s` and sampled by nothing, and §9.1's
stand-in runs at stride 1. **A T-A VOID IS READ BY WHAT IT NAMES**: a VOID naming
the floor voids the tranche (the capture is too thin to sample); a VOID naming the
ENGINE — a spawn failure, a timeout under `--timeout-s` — is a fact about the
referent process and not the capture, and the invocation is re-taken once over the
same capture; a second engine VOID voids the tranche.

**THE VOID RULE, IN THE DISPATCH'S OWN WORDS: a failed tranche is "VOID and re-run
whole".** A tranche that fails any criterion is VOID AS A WHOLE: its artifacts are
kept, its run-log block records the failure and the criterion, **no part of it
enters the corpus**, and **the tranche is RE-RUN WHOLE** under a fresh output
directory (`tranche-<n>-run2/`, §4.1). A void is never partially kept: a corpus
assembled from the passing half of a failed tranche is a corpus whose contents
depend on which half failed.

**TWO CONSECUTIVE VOIDS = STOP**, per the dispatch — two voids in a row on the
same tranche, or two voids in a row in the run log across tranches, whichever
comes first. Two in a row says the fault is in the seat or the instrument, and
continuing would spend a day producing artifacts nobody may read.

**ANY SHORTFALL IS REPORTED AGAINST §3's REGISTERED FIGURE.** If the sweep ends
with fewer than sixteen passing tranches, the closure states the delivered
distinct-position count beside the ESTIMATED 93 076 and names every tranche that
did not land and why.

## 4.1 WHERE THE SIXTEEN TRANCHES WRITE, AND WHY IT IS REGISTERED

```
<SWEEP_DIR>/tranche-<n>/arena_tranche-<n>.toml   the generated config
<SWEEP_DIR>/tranche-<n>/report.txt         pass 1's report        (arena --out)
<SWEEP_DIR>/tranche-<n>/capture.txt        pass 2's capture       (arena --capture --out)
<SWEEP_DIR>/tranche-<n>/corpus.txt         pass 3's corpus        (arena --labels --out)
<SWEEP_DIR>/tranche-<n>/replay.txt         T-B's replay report    (arena --replay --out)
<SWEEP_DIR>/tranche-<n>-run<k>/…           a re-run of a VOID tranche, k from 2

# tranche one only
<SWEEP_DIR>/tranche-1/capture-cached.txt   the CACHED re-capture of tranche one's own
                                           report — the sibling's §4.4 referent half; if
                                           tranche one is VOID, the pair's paths are its
                                           passing `-run<k>`'s
<SWEEP_DIR>/tf/arena_tf.toml               T-F's sub-range config, skip 13 take 20
<SWEEP_DIR>/tf/report.txt                  T-F's own play pass
<SWEEP_DIR>/tf/capture-{a,b}.txt           T-F's two captures, compared with cmp -s

# the whole sweep
<SWEEP_DIR>/run_log.txt                    §5
<SWEEP_DIR>/assembly/{raw,deduped}_manifest.txt   §6
```

**EVERY SECOND WRITE HAS ITS OWN PATH**, because `--out` is claimed with
`create_new`/`O_EXCL` (`crates/pistol-arena/src/outpath.rs:6-25`): two runs naming
one path is a refusal *before any game*, and a plan with one capture path per
tranche would have refused tranche one's own cached re-capture, which §6.1 takes
after the wave.

**`<SWEEP_DIR>` IS ON `/home` AND NEVER ON `/tmp`**, because a tmpfs does not
survive a reboot and this sweep spans days.

---

## 5. THE RUN LOG AND THE RESUME POINT

One run log, appended to, one block per tranche, in the pilot's own shape: the
revision, `rustc --version` and `cargo --version`, every binary and instrument
digest (§8), then each pass **with its command VERBATIM**, its exit status, its
seconds, and its printed manifest row.

**AND EACH TRANCHE'S BLOCK CARRIES THE COUNTS LINE THE CAPTURE PRINTS.** Every
tranche of this sweep runs uncached (§6.1), so every block carries the OFF shape,
`arena: label cache off: asks A records R`, and `asks == records` on each is the
mechanical corroboration that the flag was absent. **D-586's flip clause reads
nothing on this sweep**: its two collision counters are printed only by the ON
shape, so the per-tranche floors in `artifacts/arc3_opening_prefix_fold.txt` are
read by no tranche here and the clause waits for a run that caches. They decide
nothing about a label in any case.

**THE COMMAND IS RECORDED VERBATIM AND THAT IS NOT BOOKKEEPING**: with it, the
presence of `--label-cache` is a fact a reader reads, corroborated by the counts
line's `on`/`off`, rather than a sentence a reader believes. **THE CHECKER OF
§6.1's GATE IS THIS LOG**: the `cmp -s` **`exit=0`** line of the comparison, with
its timestamp, appears BEFORE the first block whose PASS-2 command — the one that
writes `capture.txt` — carries `--label-cache`; a block whose pass 2 carries the
flag with no such line above it, or with only a non-zero one, is a VOID tranche
under §4's void rule. Tranche
one's `capture-cached.txt` re-capture is the comparison's own referent, carries
the flag, precedes the line by construction, and is the one flagged command that
may.

**THE RESUME POINT.** A successor reads the log and **starts at the first tranche
that has no PASS verdict** — a VOID tranche is an unfinished one and is re-run
whole under its next `-run<k>` directory, per §4. **A tranche with no PASS and no
VOID whose directory holds any claimed file is INTERRUPTED** — a reboot, a kill, a
capture stopped mid-ask — and is treated as VOID: re-run whole under `-run<k>`,
the partial directory kept, because `--out` is claimed with `O_EXCL` and a
re-launch in place would refuse at the claim. **An INTERRUPTED tranche does not
count toward §4's two consecutive VOIDs**, which counts criterion VOIDs only —
the fault a reboot names is not the seat's or the instrument's. A tranche with a
PASS is never re-run: its capture is ~3 hours and its answer is recorded. `docs/book_v2_ledger.md`
carries one row for this whole registration; the per-tranche state lives only in
the run log.

---

## 6. ASSEMBLY, AT THE END AND NOT BEFORE

`tools/wp21_assemble.py` reads every PASSING tranche's corpus and writes two
manifests, both **MEASURED**:

- the **RAW** manifest: one row per corpus with its own `body_sha256`, its record
  count and its `capture_sha256`. Raw is the record.
- the **DEDUPED** manifest: one row per distinct position under **D-562(2)'s
  default** — the three keys agreeing, the deeper label winning, ties to the first
  in tranche order — as an INDEX into the raw corpora (tranche, record line, the
  three keys, depth, result, end). Deduped is the training input; it is an index
  and not a merged corpus because a merged corpus would need one
  `experiment_sha256` and sixteen tranches have sixteen.

**The dedup runs at ASSEMBLY and never at capture** (D-562(2)): a capture that
dropped a duplicate would destroy the 2.14x duplication factor D-560's arithmetic
rests on, and the cache drops nothing (the sibling's §1).

Reported at closure, all MEASURED off the deduped manifest's own header:
**records**, **distinct positions**, **decided** (a win by the rules with
`end normal`, the subset D-562(1) lets outcome enter on), **outcome coverage** =
decided / distinct, and **key disagreements** — distinct positions sharing at
least one key value with another distinct position, an order-free count, kept
distinct because which key rules such a pair is what D-562(2) leaves open. The
instrument also refuses, as a VOID, a corpus given twice and corpora labelled at
different `label_go` lines.

## 6.1 THE CACHE, AND WHY NO TRANCHE OF THIS SWEEP RUNS CACHED

**The gate is unchanged and it is what settles this**: tranche one's corpus of
record is its UNCACHED capture, and no tranche runs cached until the sibling's
§4.4 comparison of that capture with tranche one's cached re-capture returns
byte-identity, with §5's run log as the checker. **At the selected N = 16 the
sixteen tranches are one wave, so tranche one's uncached capture exists only when
the wave ends, and the comparison cannot return before the thing it would have to
precede. EVERY TRANCHE OF THIS SWEEP THEREFORE RUNS UNCACHED — a re-run of a VOID
tranche included**, because the gate would have returned by then and putting one
tranche's corpus on a different instrument from the other fifteen is not worth a
saving §3 prices at 0.20 h on a wall that no longer has a second wave.
**§4.4's comparison is still taken, after the wave**, at tranche one's own report:
it is the capability verification the sibling registers, its byte-identity or its
single differing byte is a finding of this arc, and it governs no tranche of this
sweep. Everything else — the criterion, the consequence of a failure, the
diagnosis — is the sibling's §4.4 and is not restated here. **Nothing in this
registration's criteria, partition, seat or budget depends on the cache.**

---

## 7. WHAT THIS RUN DOES NOT DO

1. **Start a census or a clock** — the census paragraph above.
2. **Make a strength claim** — both seats are one engine (D-156).
3. **Fit a score** — detector round 3 re-opens on D-537's count, arc III §6's
   business.
4. **Settle the solver cap** — the calibration's registered experiment, whose
   records this corpus excludes by construction.
5. **Reach the holdout, or re-read the pilot's range as corpus.**

---

## 8. THE GOVERNING REVISION, THE COMPILER AND THE INSTRUMENTS

**THE SLOTS BELOW ARE FILLED FROM THE CLOSURE HEAD, AND CHANGING ANY OF THEM IS AN
AMENDMENT THAT REOPENS THIS REVIEW** — the honest cost of a slot. They name the
revision tranche one runs at, quoted again in the run log's first block:

| instrument | revision / digest |
|---|---|
| the tree | **`0c4f3b4`** for every binary and instrument below — the commit the label cache package closed at; the commit that lands this revision is docs-only above it (`git diff --stat 0c4f3b4 <this commit> -- crates tools configs` is empty), so the digests are its own |
| `rustc --version`, `cargo --version` | **`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`** — recorded beside every digest; **a toolchain change between tranches VOIDS the tranches after it until every digest is re-recorded** (arc III F-1.11, F-1.14; D-577's *"rebuild means re-record"*) |
| `target/release/pistol`, `target/release/arena`, `target/release/corpus-check` | `78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749`, `a1a405cb44d21f1a70918f44b15553f0a90d23f02e9f959458545707a69614c3`, `efbb76b643fb72fc4024168a278542836a94d1470ebd831cdb707890593e3abf` — the `--release --locked` build at that commit |
| `tools/cold_label_check.py` | `6386d6bfe2eaf48789dd9cbc9f89794573e3339e829ef5bd66fd4a134ff2238e` — with the ten-sample floor landed (`MIN_SAMPLED = 10`) |
| `tools/wp21_tranche_config.py` | `586b4e7fad77c7577073e78b2ce313bde84003fa71eb5b507c2c156d62932afb` — with the `--pilot-range` form landed; its header comment no longer names a validator (`wp21_prereg_rev6_REVIEW.md`'s m7, this document's round 4) |
| `tools/label_cache_count.py` | `1a890b5331c302cf97372603e7ec21a41b08e6131390d92b78b0168bc77c1e18` — the producer of §3's 0.5323; its receipt `artifacts/arc3_leverB_41_count_v3.txt` is `cbad0786505e8d7958610a2ff24d8b4186de29fd85b0127d60df7b04b4e342ed` |
| `tools/wp21_assemble.py` | `a367d84754162b65e24bbff98b8a791dc3e9d2678e7f653a71457e4853fee39b` — §6's instrument. Its round-2 PASS (`wp21_assemble_REVIEW_round2.md`) reviewed the script at `9c4366c`; this digest is `5b17132`'s, the reviewer's own executed minors applied and run (`arc3_ledger.md` §2.4), and that diff has its own scoped round 3 (`wp21_assemble_REVIEW_round3.md`); **§6 does not run before that round passes** |
| `artifacts/arc3_opening_prefix_fold.txt` | the per-tranche floors D-586 reads the counters against: sha256 `b6d4751e24e3594d7da827687bc3a35630c3b5052c6e6cd8f3d260076e3e97de`, taken at this revision; the sixteen floors are transcribed in D-584 |

An instrument that produces a registered number is named with its revision
(`docs/process.md`), and a change to any of them after the slots are filled
reopens this registration.

---

## 9. THE COMMANDS, AND THE DRY RUN THAT PROVES THEY ARE THE BINARY'S

```
# 0 — the tranche's config, from the SHIPPED generator. Its acceptance is the arena's
#     own strict parse at pass 1 (`serde(deny_unknown_fields)`, hard rule 1); no
#     second validator is registered.
tools/wp21_tranche_config.py --tranche <n> --out <SWEEP_DIR>/tranche-<n>/arena_tranche-<n>.toml \
                             --binary-sha256 <the closure binary's digest>

# 1 — play
arena --config <SWEEP_DIR>/tranche-<n>/arena_tranche-<n>.toml --out <SWEEP_DIR>/tranche-<n>/report.txt

# 2 — capture. The word order is POSITIONAL (`crates/pistol-arena/src/bin/arena.rs:52-59`,
#     the tail parsed by `crates/pistol-arena/src/usage.rs:111`);
#     --label-cache is the LAST word, and §6.1 admits it on NO tranche of this
#     sweep, so it appears on no pass-2 command below
arena --capture <SWEEP_DIR>/tranche-<n>/report.txt --out <SWEEP_DIR>/tranche-<n>/capture.txt \
      --label-nodes 400000

# 3 — corpus
arena --labels <SWEEP_DIR>/tranche-<n>/capture.txt --report <SWEEP_DIR>/tranche-<n>/report.txt \
      --out <SWEEP_DIR>/tranche-<n>/corpus.txt

# T-A, twice, one invocation per class
tools/cold_label_check.py --capture <SWEEP_DIR>/tranche-<n>/capture.txt \
      --binary target/release/pistol --engine-config configs/instrument_v0.toml \
      --stride 200 --partition misses
tools/cold_label_check.py … --stride 200 --partition hits

# T-B
arena --replay <SWEEP_DIR>/tranche-<n>/report.txt --out <SWEEP_DIR>/tranche-<n>/replay.txt --workers 1

# T-D
corpus-check <SWEEP_DIR>/tranche-<n>/corpus.txt

# T-F, BEFORE the wave, the box otherwise idle
tools/wp21_tranche_config.py --skip 13 --take 20 --out <SWEEP_DIR>/tf/arena_tf.toml \
                             --binary-sha256 <the closure binary's digest>
arena --config <SWEEP_DIR>/tf/arena_tf.toml --out <SWEEP_DIR>/tf/report.txt
arena --capture <SWEEP_DIR>/tf/report.txt --out <SWEEP_DIR>/tf/capture-a.txt --label-nodes 400000
arena --capture <SWEEP_DIR>/tf/report.txt --out <SWEEP_DIR>/tf/capture-b.txt --label-nodes 400000
cmp -s <SWEEP_DIR>/tf/capture-a.txt <SWEEP_DIR>/tf/capture-b.txt
# T-F's second half, once tranche one's capture exists: the bodies, headers aside.
#     T-F's forty games are tranche one's first forty, so the records coincide.
cmp -s <(grep -v '^#' <SWEEP_DIR>/tf/capture-a.txt) \
       <(grep -v '^#' <SWEEP_DIR>/tranche-1/capture.txt | head -n $(grep -v '^#' <SWEEP_DIR>/tf/capture-a.txt | wc -l))

# tranche one only, AFTER the wave: the sibling's §4.4 referent pair
arena --capture <SWEEP_DIR>/tranche-1/report.txt --out <SWEEP_DIR>/tranche-1/capture-cached.txt \
      --label-nodes 400000 --label-cache
cmp -s <SWEEP_DIR>/tranche-1/capture.txt <SWEEP_DIR>/tranche-1/capture-cached.txt

# assembly, once, over the passing tranches in order
tools/wp21_assemble.py --out-dir <SWEEP_DIR>/assembly --corpus <SWEEP_DIR>/tranche-1/corpus.txt … 
```

**THE BOX IS OTHERWISE IDLE DURING A TRANCHE** — no cargo, no bench, no other
session's job, no T-F — checked with `ps` before the wave is launched, because
the per-label rate is a wall-clock quantity and a concurrent job voids the
contention measurement the wave is read for.

### 9.1 THE DRY RUN — a stand-in tranche, and what a failure means

Input of the same KIND and never the registered workload: **a stand-in tranche of
one opening from the pilot's consumed range**, written by the shipped generator's
`--skip 0 --take 1 --pilot-range` form (a re-read of a consumed range yields no
corpus and spends no unseen opening, D-539), at `--label-nodes 2000` rather than
400 000 and **`--stride 1`** rather than 200 — so that two games of one opening
give each cold-check class its ten samples. Seconds rather than hours.

1. every command above, T-F's and the assembly included, is **accepted by the
   shipped binary and exits 0**, the capture run twice — once without and once with
   `--label-cache` — and the two compared with `cmp -s`;
2. the generator writes a config `arena` accepts: the stand-in, by being played.
   The `--tranche` and `--skip/--take` forms come from the same `document()`
   template (`tools/wp21_tranche_config.py`) and differ from it in the two
   integers and the header comment only — a `diff` of any two written forms shows
   nothing else — and they are not played by a dry run;
3. `--partition hits` and `--partition misses` each print a line **naming the
   class**, and **the checker's two class sizes equal the cached counts line's
   `asks` and `hits`** — the arena's memo and the checker's first-seen walk are two
   implementations of one partition, in two languages, and the sum alone would
   survive a misclassification;
4. `corpus-check` reads the produced corpus and reports its record count, and
   `tools/wp21_assemble.py` over that one corpus reports it again;
5. the whole sequence leaves exactly the files §4.1 registers for `tranche-1/`,
   `tf/` and `assembly/`, and no others — and no `run_log.txt`, because a dry run
   appends no block; the log IS the artifact, under `artifacts/`.

**A DRY-RUN FAILURE STOPS THE SWEEP AND IS REPORTED AS A FINDING**, not worked
around by editing a command until something runs. **The dry run's input and its
output are recorded here**, per `docs/process.md`:

**RECORD.** Taken before this revision's review was dispatched, on the box
otherwise idle, log `artifacts/arc3r_dryrun_sweep_0c4f3b4_v4.txt` (sha256
`c91dd08d00ecd319cc52460fb387ec39c851e33fb7f554d2bee92f1814d9ebda`), `<DRY>` =
`/home/tom/pistol-runs/arc3r-dryrun/sweep`. Its head line: `== dry run at 73979e7675696022b81e429f87b5437f8ac47db3, Thu Sep  3 07:34:12 AM UTC 2026, rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0 (797e8a9bc 2026-08-05)`.
The stand-in config's sha256: `00b95f90c5bc48450f6a1dbc89ef23b07d95ac6926f3f342ab88359694df3dd4`. **Every command and its exit
line, verbatim**, each argument shell-quoted as the driver printed it, so a line
pasted back runs as it ran (the three `bash -c` lines included):

```
$ tools/wp21_tranche_config.py --skip 0 --take 1 --pilot-range --out <DRY>/tranche-1/arena_tranche-1.toml --binary-sha256 78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749
exit=0
$ sha256sum <DRY>/tranche-1/arena_tranche-1.toml
exit=0
$ target/release/arena --config <DRY>/tranche-1/arena_tranche-1.toml --out <DRY>/tranche-1/report.txt
exit=0
$ target/release/arena --capture <DRY>/tranche-1/report.txt --out <DRY>/tranche-1/capture.txt --label-nodes 2000
exit=0
$ target/release/arena --capture <DRY>/tranche-1/report.txt --out <DRY>/tranche-1/capture-cached.txt --label-nodes 2000 --label-cache
exit=0
$ cmp -s <DRY>/tranche-1/capture.txt <DRY>/tranche-1/capture-cached.txt
exit=0
$ target/release/arena --labels <DRY>/tranche-1/capture.txt --report <DRY>/tranche-1/report.txt --out <DRY>/tranche-1/corpus.txt
exit=0
$ tools/cold_label_check.py --capture <DRY>/tranche-1/capture.txt --binary target/release/pistol --engine-config configs/instrument_v0.toml --stride 1 --partition misses
exit=0
$ tools/cold_label_check.py --capture <DRY>/tranche-1/capture.txt --binary target/release/pistol --engine-config configs/instrument_v0.toml --stride 1 --partition hits
exit=0
$ target/release/arena --replay <DRY>/tranche-1/report.txt --out <DRY>/tranche-1/replay.txt --workers 1
exit=0
$ target/release/corpus-check <DRY>/tranche-1/corpus.txt
exit=0
$ tools/wp21_tranche_config.py --skip 0 --take 1 --pilot-range --out <DRY>/tf/arena_tf.toml --binary-sha256 78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749
exit=0
$ target/release/arena --config <DRY>/tf/arena_tf.toml --out <DRY>/tf/report.txt
exit=0
$ target/release/arena --capture <DRY>/tf/report.txt --out <DRY>/tf/capture-a.txt --label-nodes 2000
exit=0
$ target/release/arena --capture <DRY>/tf/report.txt --out <DRY>/tf/capture-b.txt --label-nodes 2000
exit=0
$ cmp -s <DRY>/tf/capture-a.txt <DRY>/tf/capture-b.txt
exit=0
$ bash -c cmp\ -s\ \<\(grep\ -v\ \'\^#\'\ <DRY>/tf/capture-a.txt\)\ \<\(grep\ -v\ \'\^#\'\ <DRY>/tranche-1/capture.txt\ \|\ head\ -n\ \$\(grep\ -v\ \'\^#\'\ <DRY>/tf/capture-a.txt\ \|\ wc\ -l\)\)
exit=0
$ tools/wp21_assemble.py --out-dir <DRY>/assembly --corpus <DRY>/tranche-1/corpus.txt
exit=0
$ bash -c tools/cold_label_check.py\ --capture\ <DRY>/tranche-1/capture.txt\ --binary\ target/release/pistol\ --engine-config\ configs/instrument_v0.toml\ --stride\ 1\ --partition\ misses\ \|\ /usr/bin/grep\ -o\ \'of\ which\ \[0-9\]\*\ are\ MISSES\'
exit=0
$ bash -c tools/cold_label_check.py\ --capture\ <DRY>/tranche-1/capture.txt\ --binary\ target/release/pistol\ --engine-config\ configs/instrument_v0.toml\ --stride\ 1\ --partition\ hits\ \|\ /usr/bin/grep\ -o\ \'of\ which\ \[0-9\]\*\ are\ HITS\'
exit=0
```

**The printed lines the limbs read**, verbatim (limb 3: the checker's `17 are
MISSES` / `17 are HITS` against the cached line's `asks 17 … hits 17`):

```
arena: captured 34 position(s) from 2 game(s) at go nodes 2000
arena: label cache off: asks 34 records 34
arena: captured 34 position(s) from 2 game(s) at go nodes 2000
arena: label cache on: asks 17 records 34 hits 17 key_pos_collisions 0 key_full_collisions 0 fold_ms 0
cold_label_check: 34 record(s) in capture.txt, of which 17 are MISSES; the sample is every MISSES record whose zero-based position within that class is a multiple of 1, which is 17 of them
cold_label_check: 17 of 17 sampled MISSES record(s) agree byte for byte
cold_label_check: 34 record(s) in capture.txt, of which 17 are HITS; the sample is every HITS record whose zero-based position within that class is a multiple of 1, which is 17 of them
cold_label_check: 17 of 17 sampled HITS record(s) agree byte for byte
arena: replayed 2 of 2 game(s) from /home/tom/pistol-runs/arc3r-dryrun/sweep/tranche-1/report.txt, 0 divergence(s)
corpus_check: /home/tom/pistol-runs/arc3r-dryrun/sweep/tranche-1/corpus.txt ok, 34 record(s), capture_sha256 4c99f011128a7a87c3ce54c54965fa548d6f0cb2d19be2fd5291601bf92dd3e9
arena: captured 34 position(s) from 2 game(s) at go nodes 2000
arena: label cache off: asks 34 records 34
arena: captured 34 position(s) from 2 game(s) at go nodes 2000
arena: label cache off: asks 34 records 34
wp21_assemble: records 34
wp21_assemble: distinct positions 17
wp21_assemble: decided 17
wp21_assemble: outcome coverage 1.0000
wp21_assemble: key disagreements 0
of which 17 are MISSES
of which 17 are HITS
```

Limb 5, the files left behind, and no others:

```
./assembly/deduped_manifest.txt
./assembly/raw_manifest.txt
./tf/arena_tf.toml
./tf/capture-a.txt
./tf/capture-b.txt
./tf/report.txt
./tranche-1/arena_tranche-1.toml
./tranche-1/capture-cached.txt
./tranche-1/capture.txt
./tranche-1/corpus.txt
./tranche-1/replay.txt
./tranche-1/report.txt
```
