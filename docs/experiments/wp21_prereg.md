# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 4.

> **ONE LINE FOR THE MORNING.** The book's unconsumed range LESS A RESERVED
> 1,000-OPENING HOLDOUT — **3 487 openings**, **~93 100 distinct positions
> ESTIMATED**, **~53 hours of labelling ESTIMATED from MEASURED per-unit
> rates** — is partitioned into sixteen tranches run **N at a time, N a
> REGISTERED SLOT with incumbent 8** (§1); every value below is fixed before
> tranche one, every criterion is quoted here, and a tranche that fails one is
> **VOID as a whole and RE-RUN WHOLE**, never repaired.

**REVISION 4, AND WHAT MOVED.** Revision 3 went to a fresh-context review and
came back **FAIL** — 4 BLOCKING, 12 MAJOR, 7 minor
(`wp21_prereg_rev3_REVIEW.md`, at `fce50bc5b00baab9066f1e7bdf10c48c025b7755`).
Every finding is disposed of at the section that owns it. **THE FOUR THAT CHANGE
WHAT A RUN WOULD MEAN**:

- **`D-576` did not exist** when revision 3 cited it as the ground for the whole
  cache amendment. It does now (arc III §0); the citation is real.
- **THE REPLAY ESTIMATE APPLIED A FOUR-WORKER PILOT RATE UNDER A CRITERION THAT
  MANDATES ONE WORKER.** T-B requires `--workers 1`; the pilot's replay ran at
  `--workers 4` (`wp20_pilot_prereg.md:1059`) and
  `crates/pistol-arena/src/replay.rs:20,33` confirms replay fans out. §3 applied
  the `x4` correction to the play line and not to the replay line. **Corrected, a
  tranche is 13 928 s = 3.87 h and the eight-way wall 7.74 h**, not 7.15 — and the
  throughput study's lever-exclusion argument (*"replay 3%"*) is really **10.1%**.
- **THE RUN LOG COULD NOT TELL A CACHED TRANCHE FROM AN UNCACHED ONE**, so §6.1's
  protective rule was unverifiable by a successor. §5 now records each tranche's
  command verbatim.
- **THE VOID RULE CONTRADICTED THE GOVERNING DISPATCH.** The dispatch says a
  failed tranche is *"VOID **and re-run whole**"*; revision 3 said VOID and
  nothing about re-running, which let tranches vanish with no obligation to
  report the shortfall against §3's registered 93 076.

**AND ONE THING THE REVIEW CONFIRMED RATHER THAN BROKE**, recorded because a
review that only breaks things is not a review: it re-derived §2's partition in
full — `15x218 + 1x217 = 3487`, `skip(17) = 3500` landing exactly on the
holdout's first opening — verified the book holds 4 500 openings by three
independent sources, and confirmed `tools/wp21_tranche_config.py`'s `slice_of`
matches §2 and that `no_tranche_reaches_the_reserved_holdout` genuinely drives
the SHIPPED script over all sixteen tranches.

**AN AMENDMENT REOPENS THE REVIEW, so revision 4 owes its own fresh-context
review before the first run it governs.**

**REVISION 3, AND WHAT MOVED.** The architect's arc III rulings reach this
document in three places, and the wall arithmetic moves with them.

- **THE SWEEP RUNS WITH A LABEL CACHE** (R1, landed **D-576**). §3's capture
  figures are re-derived with the **MEASURED hit rate 0.5323**, not scaled in
  prose, and both bounds are carried — the uncached wall this document already
  registered, and the cached one — because §4.4 of
  `wp21_throughput_prereg.md` forbids a cached tranche before its byte-identity
  comparison returns.
- **T-A IS AMENDED**: cache hits and cache misses are sampled **separately** at
  the registered stride, both byte-equal in fresh processes. A stride over an
  undifferentiated record list can sample 200 misses and no hit, and a cold check
  that never exercises a hit says nothing about the cache.
- **THE WORKER COUNT IS A REGISTERED SLOT UNTIL LEVER A MEASURES IT** (arc III
  §1): the eight of §3 is the incumbent, not the answer, and the number the sweep
  runs at comes from the concurrency study's measured median throughput under its
  own §3.4 decision rule.

**AND AN AMENDMENT REOPENS THE REVIEW however small the diff, so revision 3 owes
a fresh-context review before the first run it governs; revision 2's does not
transfer — and revision 2 never had one.**

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

Governing revision: the WP-2.0b closure head, whose digest §8 carries **a slot
for, not yet a value** — and filling that slot is an amendment that reopens this
review.
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

**D-568 SAYS "3,500" AND THE ARITHMETIC SAYS 3,487, and the difference is
recorded rather than rounded away.** The book holds 4 500 openings; the pilot
consumed `0..12`; the holdout takes `3500..4499`. What is left is
`13..3499` — **3 487 openings**, not 3 500, because the ruling's round number is
the book's remainder before the pilot's thirteen are subtracted. Every figure
below is derived from 3 487.

**REVISIONS 1 TO 3 ATTRIBUTED THE 3,500 TO `wp21_DISPATCH.md`, AND IT IS NOT
THERE**: `/usr/bin/grep -n '3,500\|3500\|3 500' docs/experiments/wp21_DISPATCH.md`
returns nothing — that document registers *"the full `book_v2` range"*. The number
is **D-568's**, the architect's ruling. The misattribution mattered more than a
wrong pointer: this is the paragraph whose whole job is to record a departure from
a governing document, and it named a document that never said the thing departed
from. `docs/experiments/overnight2_ledger.md` §2 repeats it verbatim and is
corrected with it.

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
| **concurrent tranches** | **a REGISTERED SLOT**, incumbent 8 | filled from `wp21_throughput_prereg.md` lever A's measured median throughput under its own §3.4 rule, before tranche one. **ANY selected N other than the incumbent 8 reopens this document**, because §3's wall arithmetic and §6.1's registered consequence are both stated at N=8. Revision 3's trigger — *"if the tranche count is re-registered"* — **can never fire**: every member of the study's field {1,2,4,8,16} divides 16, so N could have moved the wall four-fold with nothing reopened |
| **label cache** | **ON**, keyed on the `position` line's exact bytes | R1 / D-576, key per `wp21_throughput_prereg.md` §2 revision 2. Tranche one and everything in its wave run UNCACHED as §4.4's referent; nothing runs cached until that comparison returns byte-identity |
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
MEASURED   seconds per replayed game 21 / 26     = 0.8077   (at --workers 4; wp20_pilot_prereg.md:1059)
MEASURED   hit rate under the cache key           = 0.5323  (artifacts/arc3_leverB_41_count_v3.txt)

ESTIMATED  games      3487 x 2        =   6 974
ESTIMATED  records    3487 x 57.0769  = 199 027
ESTIMATED  distinct   3487 x 26.6923  =  93 076
ESTIMATED  capture    199 027 x 0.885445 = 176 227 s = 48.95 h SERIAL
```

**THE SIX INPUTS, AND WHERE EACH IS READ** — revision 3 enumerated four and used
six, which is the shape a reader cannot check. 13 openings, 26 games, 742
records, the play wall and the replay wall are in
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`; **the 347 distinct positions are NOT** —
that log reports `distinct-n 13`, which is distinct GAMES — and come from
`docs/experiments/wp20_CLOSURE.md`; **the 0.5323 hit rate** comes from
`artifacts/arc3_leverB_41_count_v3.txt`, taken by `tools/label_cache_count.py`.

**AND THE CAPTURE LINE IS SHOWN WITH THE RATE IT IS COMPUTED WITH.** Revision 3
printed `199 027 x 0.8854 = 176 228`, which does not reproduce: `0.8854` gives
176 218.51 and the printed answer comes from the unrounded `657/742 = 0.885445`.
A shown derivation that does not reproduce is the one thing a section titled *"THE
ARITHMETIC, SHOWN"* may not be.

Per tranche, at the larger take of 218 openings:

```
ESTIMATED  games   436        records 12 443        distinct 5 819
ESTIMATED  capture 12 443 x 0.8854      = 11 017 s = 3.06 h
ESTIMATED  play    436 x 0.8271 x 4     =  1 442 s = 0.40 h   (x4: the measured rate is a 4-worker THROUGHPUT and this seat runs one)
ESTIMATED  replay  436 x 0.8077 x 4     =  1 409 s = 0.39 h   (x4 FOR THE SAME REASON, and T-B mandates --workers 1)
ESTIMATED  cold    64 x 0.8854          =     57 s           (34 hit samples + 30 miss samples at stride 200, see §4)
                                          ------------------
ESTIMATED  tranche                        13 925 s = 3.87 h
```

**THE `x4` ON THE REPLAY LINE IS REVISION 4's CORRECTION AND IT MOVES THE WALL.**
Revisions 1 to 3 applied the four-worker correction to the play line and not to
the replay line, without saying why the two pilot rates differed — and the one
they skipped is the one whose criterion, T-B, **forces serial execution**. The
pilot's replay ran `--workers 4` (`wp20_pilot_prereg.md:1059`) and
`crates/pistol-arena/src/replay.rs:20,33` shows replay really does fan out. **A
tranche is 3.87 h, not 3.57**, and the consequence reaches the sibling study:
capture is **79.1%** of a tranche rather than 85.6%, and replay is **10.1%**
rather than 3%.

**THE COLD-CHECK LINE IS RE-DERIVED FOR T-A's AMENDED FORM** rather than carried
over: two strides rather than one, `ceil(6624/200) + ceil(5819/200) = 34 + 30 =
64` samples, at the MEASURED per-label rate rather than at revision 3's unsourced
`0.9`.

**THE SAME TRANCHE WITH THE CACHE, DERIVED FROM THE MEASURED HIT RATE AND NOT
FROM THE 2.14 SUMMARY.** `wp21_throughput_prereg.md` §4.1's count over the
pilot's own **capture**, taken by `tools/label_cache_count.py` under the cache's
own key — the `position` line's exact bytes: 742 asked prefixes, **347 distinct**,
hit rate **0.5323 MEASURED**. Only a MISS costs a search:

```
ESTIMATED  labels searched   12 443 x (1 - 0.5323) = 5 819   (= 218 x 26.6923, the distinct count, as it must)
ESTIMATED  capture, cached   5 819 x 0.885445      = 5 152 s = 1.43 h
ESTIMATED  tranche, cached   13 925 - 11 017 + 5 152 = 8 060 s = 2.24 h
```

**THE HIT RATE AND THE DUPLICATION FACTOR ARE ONE NUMBER READ TWO WAYS** —
`1 - 1/2.1383 = 0.5323` — and they are shown together so a reader can see that
the cached capture figure is the distinct count times the measured per-label
rate, and not a discount applied to a total.

**THE WALL IS STATED ONCE, HERE, AND EVERY OTHER SECTION POINTS AT IT** (D-423;
revision 3's §6.1 quoted a figure this document did not carry).

```
UNCACHED, N = 8, sixteen tranches      2 waves x 13 925 s = 27 850 s = 7.74 h
CACHED from wave two, N = 8            13 925 + 8 060     = 21 985 s = 6.11 h
                                       ESTIMATED SAVING ON THIS SWEEP  1.63 h
SERIAL, uncached                       3 487 x 57.0769 x 0.885445 = 48.95 h of capture
```

**AND IT IS A LOWER BOUND.** The per-label rate was measured with the box
otherwise idle; N concurrent engines share memory bandwidth and boost budget, and
**the contention factor is not known and is not guessed here**. It is MEASURED
from wave one — wave one's realised seconds-per-label against the pilot's
0.885445 — and reported in the closure. **Nothing in this registration's
criteria, partition, seat or budget depends on it**, and the cached figure is
reported beside the uncached one at closure rather than instead of it.

---

## 4. PER-TRANCHE CRITERIA, QUOTED

A tranche passes only if **every** line below holds. The criteria are the
pilot's, at production stride where a stride is affordable and unchanged where
it is not.

| id | criterion, as it must read | the defect it excludes |
|---|---|---|
| **T-A1** | `cold_label_check: N of N sampled MISS record(s) agree byte for byte`, exit 0, **stride 200 over cache MISSES** | a capture whose warm long-lived process answers differently from a fresh one — the claim `newgame` is supposed to make true (D-540). **IN FORCE ON EVERY TRANCHE**, cached or not. **The referent is EXTERNAL**: a fresh process shares no table with the capture pass and no cache at all |
| **T-A2** | `cold_label_check: N of N sampled HIT record(s) agree byte for byte`, exit 0, **stride 200 over cache HITS** | a cache that answers a hit differently from the search that produced the miss. **IN FORCE ONLY ON TRANCHES RUN CACHED**, and the closure reports the two counts separately — revision 3 put both defect classes in one row, so a closure saying *"T-A passed on all sixteen"* would have reported one criterion answered on sixteen tranches and a second answered on eight |
| **T-B** | `arena: replayed G of G game(s) … 0 divergence(s)`, exit 0, `--workers 1` | a report whose recorded moves are not what its attested engines answer |
| **T-C** | **zero forfeits**: the report's own `counts n … forfeits N` line (`crates/pistol-arena/src/conclusion.rs:81`) and its `first_player_wins … forfeits N` line (`:111`) both read zero, and the corpus's `end` column holds `normal` on every record — readable off `corpus_check`'s own `end 1 (normal)`. **Revision 3 said "pass 1's summary", and the pilot's stdout summary carries no forfeit token at all**: the counts live in the report file | a game ended by the driver rather than by the rules |
| **T-D** | `corpus_check: … ok, N record(s)`, exit 0 | a corpus the shipped loader will not read |
| **T-E** | a `capture_manifest` row and a `corpus_manifest` row, each carrying `body_sha256` | an artifact nothing binds (rule 8, D-469) |
| **T-F**, **tranche one only** | a second capture over a **registered sub-range** of tranche one's report is byte-identical to the first over the same range | a capture that is not a function of its inputs. It is a SUB-RANGE and not the whole tranche because a full re-run doubles a 63-hour sweep, and the pilot already ran the whole-corpus form once (`capture-determinism exit=0`) |

**HOW T-A TELLS A HIT FROM A MISS, WITHOUT A NEW COLUMN.** It is derivable from
the capture itself, and under the cache's key it is one line of arithmetic: walk
the records in order; a record whose `position` field has not been seen before is
a MISS, and every later record with that same field is a HIT. The capture grammar
gains no field — a new column would be a format version bump (D-572) for a fact
the file already determines. **On an UNCACHED tranche the partition is still
taken and both strides still run**: the derivation is over the record list, not
over the cache, so every tranche's numbers have the same shape and are
comparable. What differs is which DEFECT CLASS is in force, which is why T-A is
two rows.

**THE INSTRUMENT DOES NOT EXIST YET AND THE CRITERION QUOTES ITS AMENDED OUTPUT,
NOT ITS CURRENT ONE.** `tools/cold_label_check.py` as shipped takes one
`--stride`, samples the whole record list, and prints one
`N of N sampled record(s) agree byte for byte`. It gains a **required**
`--partition hits|misses|all` — required rather than defaulted, because a default
here would silently answer about `all` while a criterion said `hits` — and its
summary line names the class, so two invocations cannot be mistaken for one class
sampled twice:

```
tools/cold_label_check.py --capture <path> --binary <path> --engine-config <path> \
                          --stride 200 --partition hits
cold_label_check: 34 of 34 sampled HIT record(s) agree byte for byte
```

**THE AMENDED SCRIPT IS ITSELF A `tools/` CHANGE** and carries
`docs/process.md`'s coverage rule: a test driving the SHIPPED script, exactly as
`tools/wp21_tranche_config.py` and `tools/label_cache_count.py` do.

**AND AN EMPTY CLASS IS A VOID, NOT A PASS.** Revision 3 was silent, and both
reachable behaviours are wrong: `records_of()` raises `Void` on an empty list
(exit 2, which T-A's *"exit 0"* would read as a failure and the script reads as
no answer at all), while a filter over an already-read list prints
`0 of 0 … agree` and exits 0 — a **vacuous pass**, which `docs/process.md`
forbids outright. **REGISTERED**: a class with fewer than **10 sampled records**
on a tranche that should have both makes the tranche's T-A a **VOID** and not a
pass, and the void rule below applies. On a full tranche neither class is empty
(ESTIMATED 6 624 hits, 5 819 misses); the rule exists for T-F's 20-opening
sub-range and for any other capture this instrument is pointed at.

**THE SUB-RANGE FOR T-F IS REGISTERED HERE, BEFORE TRANCHE ONE RUNS: the first
20 openings of tranche one** — `openings_skip = 13`, `openings_take = 20` — run
as its own play/capture pair, twice, and compared with `cmp -s`.

**TWENTY IS NOT "the smallest take that exceeds the pilot's thirteen" — FOURTEEN
IS**, and revisions 1 to 3 said otherwise. The choice stands and was
pre-registered; only its stated ground was arithmetically false. The true ground:
twenty is a round number above the pilot's thirteen, fixed before tranche one,
and the check is not weaker than the one the pilot passed at any take above
thirteen.

**THE VOID RULE, IN THE GOVERNING DISPATCH'S OWN WORDS: a failed tranche is
"VOID and re-run whole".** Revision 3 said VOID and stopped there, which is not
what the dispatch says and which left the sweep able to deliver less than §3
registers with nothing obliged to say so.

A tranche that fails any criterion is **VOID AS A WHOLE**: its artifacts are
kept, its run-log block records the failure and the criterion that failed, **no
part of it enters the corpus**, and **the tranche is RE-RUN WHOLE**. A void is
not a repair and a tranche is never partially kept — a corpus assembled from the
passing half of a failed tranche is a corpus whose contents depend on which half
failed.

**TWO CONSECUTIVE VOIDS = STOP**, per the dispatch — where *consecutive* means
two voids in a row **on the same tranche**, or two voids in a row in the run log
across tranches, whichever comes first. The second void is not a third attempt:
two in a row says the fault is in the seat or the instrument rather than in a
tranche, and continuing would spend a day producing artifacts nobody may read.

**AND ANY SHORTFALL IS REPORTED AGAINST §3's REGISTERED FIGURE.** If the sweep
ends with fewer than sixteen passing tranches for any reason, the closure states
the delivered distinct-position count **beside the registered ESTIMATED 93 076**
and names every tranche that did not land and why. A corpus that is quietly
smaller than the plan is a corpus whose consumer cannot tell a plan from a
result.

---

## 4.1 WHERE THE SIXTEEN TRANCHES WRITE, AND WHY IT IS REGISTERED

**REVISION 1 FIXED THE CONFIGS AND NOT THE OUTPUT PATHS**, which is a gap in the
run plan rather than in the correctness argument, and it is closed here.

```
<SWEEP_DIR>/tranche-<n>/report.txt         pass 1's report      (arena --out)
<SWEEP_DIR>/tranche-<n>/capture.txt        pass 2's capture     (arena --capture --out)
<SWEEP_DIR>/tranche-<n>/corpus.txt         pass 3's corpus      (arena --labels --out)
<SWEEP_DIR>/tranche-<n>/tranche-<n>.toml   the generated config

# tranche one only, and revision 3 registered none of these four
<SWEEP_DIR>/tranche-1/capture-cached.txt   the CACHED re-capture of tranche one's
                                           own report, which is the referent half
                                           of wp21_throughput_prereg.md §4.4
<SWEEP_DIR>/tf/tf.toml                     T-F's sub-range config, skip 13 take 20
<SWEEP_DIR>/tf/report.txt                  T-F's own play pass
<SWEEP_DIR>/tf/capture-{a,b}.txt           T-F's two captures, compared with cmp -s
```

**FOUR PATHS REVISION 3 LEFT OUT, AND TWO OF THEM `O_EXCL` WOULD HAVE TURNED INTO
A REFUSAL MID-SWEEP.** §4.4's cached re-capture and T-F's duplicate pair are both
second writes; with one registered capture path per tranche the second write
lands on a claimed path and `outpath::claim` refuses it *before any game* — the
very property this section praises, firing against the run's own plan.

**AND T-F's CONFIG HAS A PRODUCER, WHICH REVISION 3 ALSO LEFT OUT.**
`tools/wp21_tranche_config.py` writes only the sixteen tranche slices —
`--tranche 1` writes `take = 218`, not 20 — so it gains a `--skip/--take` form
that writes an arbitrary window under the same fixed values, and T-F's config
comes from the SHIPPED generator like every other. Hand-writing it is the defect
§2 says the generator exists to prevent. The generator's change reopens this
registration, which is why it is named here rather than improvised at run time.

**TRANCHE ONE THEREFORE CARRIES MORE THAN §3's PER-TRANCHE BLOCK**, and it is
costed rather than absorbed:

```
ESTIMATED  tranche one's cached re-capture   5 152 s = 1.43 h
ESTIMATED  T-F's pair (20 openings)          2 x (40 x 57.0769 x 0.885445) = 2 x 2 022 s = 1.12 h
ESTIMATED  T-F's two play passes             2 x 40 x 0.8271 x 4          =   265 s = 0.07 h
                                             ----------------------------------------
ESTIMATED  tranche one's surcharge                                          2.62 h
```

**`<SWEEP_DIR>` IS ON `/home` AND NEVER ON `/tmp`.** CLAUDE.md's Environment
section: this machine's `/tmp` is a **24 GiB RAM-backed tmpfs**, and a sweep that
filled it would take every other running command down with it. The sweep's own
output is ESTIMATED in the tens of gigabytes across sixteen tranches.

**A COLLISION IS A REFUSAL AND NOT A CORRUPTION, and that is enforced rather than
trusted**: `--out` is claimed with `create_new`/`O_EXCL`
(`crates/pistol-arena/src/outpath.rs:10-25`, the function with its doc at `:6-25`;
revisions 1 to 3 cited `9-24`, which is neither), so two runs naming one path is a
named refusal *before any game*, and a refusal before any game removes the empty
claim again. **The per-tranche directory is therefore belt and braces**: the
O_EXCL claim is what makes concurrent tranches safe; the directory layout is what
makes a resume point readable.

---

## 5. THE RUN LOG AND THE RESUME POINT

One run log, appended to, one block per tranche, in the pilot's own shape:
revision, binary digests, then each pass **with its command VERBATIM**, its exit
status and its seconds.

**AND EACH CACHED TRANCHE'S BLOCK CARRIES THE THREE NUMBERS THE CACHE PRINTS**:
its hit rate, and **how many of its own MISSES shared a `key_pos` or a `key_full`
with an earlier miss**. Those two counters are
`matrix_label_cache_key.md` §4's registered flip clause — the coarser keys' yield
at the scale that matters, which no measurement before the sweep could reach — and
they settle D-562(2)'s open three-key question in the same pass. They decide
nothing about a label: the cache keys on the `position` line alone and a
collision on a coarser key changes no answer.

**THE COMMAND IS RECORDED VERBATIM AND THAT IS NOT BOOKKEEPING.** Revision 3
recorded *"revision, binary digests, exit status, seconds"*, under which **nothing
in the log distinguishes a cached tranche from an uncached one** — so §6.1's
protective rule was unverifiable by a successor and the closure could not say
which tranches rested on a check that had returned. With the command in the log
the presence or absence of `--label-cache` is a fact a reader reads rather than a
sentence a reader believes. **The `cmp -s` line of §6.1's byte-identity
comparison, with its timestamp, appears in the run log BEFORE the first block
whose command carries `--label-cache`**; a block carrying the flag with no such
line above it is a **VOID** tranche under §4's void rule.
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

## 6.1 THE CACHE, AND THE TWO RULES THAT GOVERN WHICH TRANCHES USE IT

Registered in `wp21_throughput_prereg.md` §4.4 revision 2 and repeated here only
as a pointer, because a claim a document makes twice is a defect waiting (D-423):
**tranche one's corpus of record is its UNCACHED capture**, which is §4.4's
tranche-sized referent; **no tranche runs cached until that comparison returns
byte-identity**; and the **wave schedule is derived in the run log** once lever A
fixes N, because a schedule constrains no conclusion while those two rules do.

**IF THE COMPARISON FAILS, THE SWEEP DOES NOT STOP.** Lever B is abandoned per
its own registered consequence, every remaining tranche runs uncached at **the
bound §3 states** — §3 owns the wall and this section does not restate it
(D-423; revision 3 quoted `7.15 h` here, a figure this document did not carry and
which was wrong in any case under the replay correction). **The disagreement's
CAUSE is not pre-attributed**: `wp21_throughput_prereg.md` §4.4 registers that
the cache implementation and `newgame`'s isolation are separated by re-running
the uncached pass twice before either is named. **Nothing in this registration's
criteria, partition, seat or budget depends on the cache** — which is the
property that makes it safe to take at all.

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

**THE SLOT IS EMPTY UNTIL IT IS FILLED, AND THIS DOCUMENT NO LONGER CLAIMS
OTHERWISE.** Revision 3's preamble said the governing revision was *"named in §8
with its digest"*; §8 named none. The preamble now says the slot is filled before
tranche one, and **filling it is an amendment that reopens this document's
review** — which is the honest cost of a slot.

Filled from the closure head before tranche one, and quoted in the run log:
the revision, `target/release/pistol`, `target/release/arena`,
`target/release/corpus-check`, `tools/cold_label_check.py`,
`tools/wp21_tranche_config.py` and **`tools/label_cache_count.py`**, each with its
sha256. An instrument that produces a registered number is named with its
revision (`docs/process.md`), and a change to any of them reopens this
registration.

**TWO INSTRUMENTS REVISION 3 DID NOT NAME, AND BOTH PRODUCE REGISTERED NUMBERS.**

1. **The producer of §3's 0.5323 and 2.1383.** It was a scratchpad script named
   in a receipt and in no document. `docs/process.md` names the scratchpad case
   explicitly — *"a `tools/` script, a scratchpad harness, or a command block the
   document prints … living there is not what makes the rule apply"*. It is now
   `tools/label_cache_count.py`, committed, with a test driving the shipped
   script, and its second instrument and agreement criterion are registered at
   `wp21_throughput_prereg.md` §4.1.1.
2. **The assembly instrument for §6's two manifests.** §6 registers MEASURED
   counts including a DEDUPED manifest under D-562(2)'s three-key rule, and **no
   such tool exists**: `arena --labels` writes a corpus, not a manifest, and
   `tools/` carries no assembly script. A registered MEASURED output with no
   registered instrument is a number whose producer is chosen after the run.
   **REGISTERED**: assembly is `tools/wp21_assemble.py`, which reads the passing
   tranches' corpora and writes both manifests, and it carries the coverage
   rule's test. **It does not exist yet**; it is a precondition of §6 and not of
   tranche one, and the closure may not report §6's counts until it is landed and
   reviewed.

---

## 9. THE COMMANDS, AND THE DRY RUN THAT PROVES THEY ARE THE BINARY'S

`docs/process.md`'s dry-run discipline, and the governing dispatch asks for it by
name — *"dry run on a stand-in tranche"*. **Revisions 1 to 3 carried three
parenthetical flag fragments and the words "dry run" nowhere**, and the sibling
study's registered `arena --capture` command turned out to be REFUSED by the
shipped binary for a word-order reason a dry run would have caught in seconds.

```
# 0 — the tranche's config, from the SHIPPED generator
tools/wp21_tranche_config.py --tranche <n> --out <SWEEP_DIR>/tranche-<n>/tranche-<n>.toml \
                             --binary-sha256 <the closure binary's digest>

# 1 — play
arena --config <SWEEP_DIR>/tranche-<n>/tranche-<n>.toml \
      --out <SWEEP_DIR>/tranche-<n>/report.txt

# 2 — capture. The word order is POSITIONAL (crates/pistol-arena/src/bin/arena.rs:51)
arena --capture <SWEEP_DIR>/tranche-<n>/report.txt \
      --out <SWEEP_DIR>/tranche-<n>/capture.txt --label-nodes 400000
#     ... and, from the first tranche after §6.1's comparison returns, with
#     --label-cache appended as the last word

# 3 — corpus
arena --labels <SWEEP_DIR>/tranche-<n>/capture.txt \
      --report <SWEEP_DIR>/tranche-<n>/report.txt \
      --out <SWEEP_DIR>/tranche-<n>/corpus.txt

# T-A, twice, one invocation per class
tools/cold_label_check.py --capture <SWEEP_DIR>/tranche-<n>/capture.txt \
      --binary target/release/pistol --engine-config configs/instrument_v0.toml \
      --stride 200 --partition misses
tools/cold_label_check.py ... --partition hits

# T-B
arena --replay <SWEEP_DIR>/tranche-<n>/report.txt \
      --out <SWEEP_DIR>/tranche-<n>/replay.txt --workers 1

# T-D
corpus-check <SWEEP_DIR>/tranche-<n>/corpus.txt
```

**`<SWEEP_DIR>` IS ON `/home` AND NEVER ON `/tmp`**, per §4.1.

### 9.1 THE DRY RUN — a stand-in tranche, and what a failure means

Input of the same KIND and never the registered workload: **a stand-in tranche of
one opening from the pilot's already-consumed range** (`skip = 0, take = 1`,
which spends nothing — `docs/book_v2_ledger.md` records `0..12` consumed and
D-539 says the pilot is not corpus), at `--label-nodes 2000` rather than 400 000.
Seconds rather than hours.

1. every command above is **accepted by the shipped binary and exits 0** — the
   limb whose absence let a refused command reach a registration;
2. the generator writes a config `arena` accepts, and `--tranche` and the
   `--skip/--take` form both do;
3. `tools/cold_label_check.py --partition hits` and `--partition misses` each
   print a line **naming the class**, and their sample counts sum to the
   single-stride count;
4. `corpus-check` reads the produced corpus and reports its record count;
5. the whole sequence leaves exactly the files §4.1 registers and no others.

**A DRY-RUN FAILURE STOPS THE SWEEP AND IS REPORTED AS A FINDING**, not worked
around by editing a command until something runs. Limb 3 cannot be taken until
`tools/cold_label_check.py`'s `--partition` lands, so the dry run is taken in two
parts and **the review that governs tranche one is taken after both are
recorded.**
