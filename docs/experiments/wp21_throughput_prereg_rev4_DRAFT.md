**DRAFT — NOT GOVERNING. Written at the arc III STOP (`arc3_ledger.md` §1e) as the whole rewrite of `wp21_throughput_prereg.md` revision 3 that answers its round-2 review; unreviewed; it governs nothing until a successor lands it as `wp21_throughput_prereg.md` and dispatches its review. It cites D-588, an ADR line NOT YET APPENDED, whose content is stated in the ledger §1e; its dry-run and instrument slots are empty.**

# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 4.

> **ONE LINE.** `wp21_prereg.md` §3 owns the sweep's wall — **7.74 h uncached at
> N = 8, a lower bound with contention at 1.00** — and this document does not
> restate it. It measures the two things that could move it: **what concurrency
> the box actually pays for** (lever A), and **whether the 2.14x duplication the
> corpus is REQUIRED to carry must be paid for twice in SEARCH time** (lever B,
> the label cache). It changes no label, no criterion and no captured byte; it
> may only change how long producing them takes. **On THIS sweep the cache's net
> saving is ~0.20 h ESTIMATED once its own verification is on the critical path**
> (§5) — the lever is taken for a verified capability, not for this sweep's wall.

**GOVERNING**: D-576 (the cache and its amended criterion), D-581, D-584, D-586,
D-587, D-588 (the key and what is claimed about it); `wp21_prereg.md` revision 5
§1 (the slot lever A fills), §3 (the wall), §4 (the criteria this study may not
touch), §6.1; `wp21_label_cache_design.md` revision 6 (what lever B builds);
`matrix_label_cache_key.md` revision 3 as corrected by D-586 through D-588. **This
document's revision history is in `arc3_ledger.md` and nowhere here.**

**WHY IT IS REGISTERED RATHER THAN JUST RUN.** The output is a number that decides
a run parameter of a 3 487-opening sweep, and a concurrency picked after seeing
which setting looked fastest on the day is the after-the-numbers decision
`docs/process.md` forbids. The decision rules are §3.4 and §4.5, before either
lever is measured.

**WHAT IT MAY NOT DO**: change the label budget, the game budget, the turn cap,
the seat, `wp21_prereg.md` §4's criteria, or which openings are swept. A faster
sweep that answers a different question is not a faster sweep.

---

## 1. THE TWO LEVERS, AND WHY THESE TWO

**THE COST IS CAPTURE AND LITTLE ELSE.** Per tranche, from `wp21_prereg.md` §3's
13 927 s: capture **11 017 s — 79.1%**; play 1 443 s, 10.4%; replay 1 409 s,
10.1%; the cold check 58 s, 0.4%. Play and replay are each a tenth of a tranche,
and neither is a lever: **T-B mandates `--workers 1`** for replay, and pass 1 is
already one process per tranche with N tranches on the box — a faster pass 1 or
replay would have to come from the engine, which this study may not touch.

| lever | what it attacks | code change |
|---|---|---|
| **A — concurrency** | the sweep runs 16 tranches N at a time on a box with 8 physical cores and 16 threads. Whether 8 is the throughput optimum is **UNMEASURED** | none |
| **B — the label cache** | `capture::run` asks the engine at **every asked prefix of every game**, and the pilot MEASURED 742 records over **347 distinct positions**: the same position is searched 2.14 times at `nodes 400000` | `pistol-arena`, a package with its own design and review |

**LEVER B's SIZE IS STRUCTURAL AT THE GAME LEVEL AND MEASURED AT THE PREFIX
LEVEL.** `arena --capture` refuses a report whose two seats do not attest one
engine (D-558(2)), so a self-match is the only capturable shape, and one
deterministic engine plays each opening identically in both seats — the pilot's
`n 26 distinct-n 13 (13 duplicate games)` — so every asked prefix is asked at
least twice. §4.1 owns the number beyond that.

**AND IT DOES NOT TOUCH D-562(2).** That ruling forbids dedup AT CAPTURE because a
dropped duplicate would destroy the duplication factor D-560's arithmetic rests
on. **A cache drops no record**: every record the uncached pass writes, the cached
pass writes, with the same bytes; only whether the engine was asked again changes.

---

## 2. THE SOUNDNESS ARGUMENT FOR LEVER B

**A LABEL IS A PURE FUNCTION OF (position, budget, binary, config), AND THAT IS
STRUCTURAL.** `capture.rs`'s `ask` sends `newgame` before every `position`/`go`;
`pistol-engine`'s `new_game` calls `Searcher::clear`, which clears the
transposition table, the heuristic tables and the solver. **No state that can move
an answer crosses an ask.** The exception is the trigger-census collector, which
`clear` does not touch and which cannot move a `bestmove` or a node count — and
§4.2 refuses to combine a census with the cache for a separate reason.

**THE KEY IS THE `position` LINE ITSELF, THE EXACT BYTES THE ASK IS MADE WITH.**
Selected by `matrix_label_cache_key.md`, attacked by a fresh-context red team,
landed as D-576 and corrected in its supporting claims by D-581, D-584 and D-588.
A hit answers a question spelled identically, so the cache's soundness rests on
hard rule 4 as `tools/determinism.sh` gates it — same binary, same config, same
`position`, same `go`, after `newgame`, same bytes — and not on any claim about
what the search reads out of `GameState`. **Such a claim would be false at one
gate**: the search reads play order through `last_stone` under `countermove`
(D-581), so a stone-set key would be unsound under a config value this seat
happens to set `false`, and a soundness argument that is one value's shadow is
not one.

**WHAT GATE 9 DOES AND DOES NOT COVER, read off the script and not its header.**
Its `SEATS` array holds **five** seats — `radius`, `staged`, `staged-heuristics`,
`staged-solver`, `staged-safety-net-cap` — none of them
`configs/instrument_v0.toml`; its budgets are `depth_turns 4` and `nodes 200000`,
with the solver seat's own `depth_turns 2` / `nodes 10000`; none is `nodes 400000`.
**And no limb of it asks the same `(position, go)` pair twice inside one
process**: its A/B session repeats every position once per budget, so a repeated
`position` line after `newgame` IS exercised, at a different `go` — the identical
ask, which is a hit, is the repeat it never takes (D-588, correcting D-581's
*"never asks the same position twice"*). **That is the strongest surviving attack
on the key, recorded as a cost**, and what closes it is §4.4's byte-identity run at
the sweep's own seat and budget, which asks the same position at the same `go` in
one process ~6 600 times a tranche and compares every byte against a pass that
never did. The supporting evidence is the within-process channel hunt a fresh
context ran and lost — the TT generation counter, the census collector,
`Instant::now()` on the ordering path, hash iteration order, `Board::stones()`,
the eval's apply/undo, `ThreatState`'s rebuild, the solver — which found no channel
(`wp21_throughput_prereg_rev2_REVIEW.md`).

**WHAT THE COARSER KEYS WOULD BUY, MEASURED WHERE IT CAN BE.** On the pilot's 742
records the exact key, the sorted stone list and the twelve-image symmetry fold
all give **347** (§4.1). That population is thirteen openings and does not
transfer; over the sweep's own range the book's prefixes fold at `k = 2` —
**42 to 60 merges per tranche, 792 in all, every one a symmetry image whose
`bestmove` would be in the wrong frame** — and at `k = 3` the book's whole-opening
dedupe holds (D-584, `artifacts/arc3_opening_prefix_fold.txt`). Beyond
`k = opening_turns` nothing is known in advance, and **the cache measures it**:
two counters, per tranche, of how many of its own MISSES share a stone set or a
canonical form with an earlier miss. They cost one sort and twelve images per miss
(D-586, D-587), they decide nothing about a label, each cached tranche's
`key_full_collisions` is read against its own floor, and D-586's one-percent line
is what re-takes the matrix. **They settle nothing about D-562(2)**: a counter
compares a key with itself and adjudicates no disagreement between keys.

**THE CACHE'S EQUIVALENCE IS THE FINEST THERE IS.** Keying on the `position` line
folds nothing — not symmetries, not transpositions, not a re-spelling of a turn,
because a `Turn::Pair` is canonical by its type invariant and `FromStr` refuses an
uncanonical spelling — so it is strictly finer than `key_seq`, `key_pos` and
`key_full` alike, and where a corpus holds a transposition the cache MISSES it: a
lost saving, never a wrong answer, the only error direction a cache may have.

### 2.0 THE KEY IS ONE ARGUMENT OF FOUR, AND THE INVARIANT THAT MAKES THAT SAFE

§2's first sentence names four arguments and §4.2 keys on `position` alone. That
is safe only under a scope invariant: **the memo is constructed inside
`capture::run` and dropped with it, never a field of a longer-lived value, never
shared between two invocations.** Within one `run` the `go` line is computed once
outside both loops and the binary and config are the process's, so `position` is
the only argument that varies. The design makes the invariant structural — `run`
takes a MODE and builds the map itself — and the criterion that decides the lever
is §4.4's external referent, not this argument.

### 2.1 THE KEY THE RULING NAMED IS COARSER, AND IT IS DECLINED

R1 said *"the WP-2.0b canonical identity"*, which folds symmetries as well as
transpositions: a hit would return the `bestmove` computed for a symmetry IMAGE of
the position — a move in the wrong frame — with node counts that are not
symmetry-invariant (D-137). Declined on correctness (D-576); the ruling's substance
— the cache, and the criterion amended to sample hits and misses separately — is
taken in full.

---

## 3. LEVER A — CONCURRENCY. THE MEASUREMENT.

### 3.1 The workload, fixed before any setting runs

**On the pilot's consumed range, so this study spends nothing.** Openings
**`0..2`** (3 openings, 6 games) are played once into ONE report, and every
concurrent process in every setting captures THAT SAME REPORT. **THE PLAY PASS'S
INSTRUMENT IS THE SHIPPED GENERATOR**: `tools/wp21_tranche_config.py --skip 0
--take 3 --pilot-range --binary-sha256 <the current binary's digest>`, which
writes the sweep's own seat (§1 of `wp21_prereg.md`: `configs/instrument_v0.toml`
on both seats, `nodes 50000`, turn cap 40, `hang_timeout_ms 120000`) over the
pilot's range with the digest of the binary this study actually runs — a
committed config could not, because every committed `binary_sha256` predates the
toolchain change (arc III F-1.11). §3.4's 120 s guard reads `hang_timeout_ms` off
that config. Its command is in §7.

### 3.2 The settings and the reps

**THE FIELD IS {1, 2, 4, 8, 16} AND 12 IS DELIBERATELY ABSENT.** With sixteen
tranches, wall is `ceil(16/N)` waves, so only an N that divides 16 gives balanced
waves; N = 12 runs twelve and then four and cannot beat N = 8. **The consequence
is a rule: if the sweep's concurrency changes, the tranche count changes with it,
so that `TR mod N == 0`** — and every member of this field divides 16, so nothing
in it reopens the partition.

**3 reps each, in the fixed order `1, 2, 4, 8, 16` and then that order twice
more** — not three reps of one setting before the next, so a thermal ramp lands on
every setting equally (`wp20b_perf_guard.sh`'s rotation lesson). At each setting N
processes each run `arena --capture <the report> --out <distinct path>
--label-nodes 400000` concurrently, started together; the setting ends when the
last exits. The word order is the binary's (`bin/arena.rs:51`, positional).

### 3.3 The statistic, and it is one

**Realised seconds per label at concurrency N** = (wall from first start to last
exit) / `records`, where `records` is the report's own record count — the wall of
one process's capture at that concurrency — and the aggregate **throughput** is
`N x records / wall`. Multiplying the maximum process wall by N over-states
machine-seconds where processes finish unevenly; it cancels in the statistic, so no
conclusion moves. Reported per setting: **median throughput over the three reps**
and the **contention factor** `c(N)` = realised seconds per label / **0.885445**,
the pilot's MEASURED serial rate.

### 3.4 THE DECISION RULE, REGISTERED BEFORE THE RUN

**The sweep runs at the N whose median THROUGHPUT is highest**, with guards that
fire before it:

- **A tie inside 5% goes to the SMALLER N.** The smaller one leaves the box able
  to answer.
- **The selected N must divide the tranche count** (§3.2).
- **Any N whose realised MEAN seconds-per-label exceeds C3's threshold is REFUSED
  whatever its throughput** (§3.5).
- **If the highest-throughput N is 16, its wall figure carries the note that 16 is
  SMT-shared**: a per-tranche time at 16 more than 2x the time at 8 means the
  second wave was cheaper than the sharing.

**WHAT THE RESULT MAY AND MAY NOT DO.** It fills `wp21_prereg.md` §1's slot — a
run parameter, not a criterion — by an amendment that reopens that document's
review if the answer is not the incumbent 8. It may not change the tranche count,
the partition, or any §4 criterion.

### 3.5 THE CRITERIA, AND THE DESIGN ELEMENTS THEY ARE NOT

A design element is something this study DOES; a criterion is something the data
can FAIL. **Design elements**: one report captured at every setting; three reps in
rotated order; the statistic is aggregate throughput. **Criteria**, each with the
defect class it excludes and each falsifiable:

| # | criterion | the defect class it excludes |
|---|---|---|
| **C1** | every process's **capture file** — pass 2's five-TAB-field output, not a corpus — at every N and every rep is byte-identical to every other's | a capture whose answers depend on machine load |
| **C2** | every process's record count equals the report's own asked-prefix count | a process that exited early and looked fast |
| **C3** | the realised MEAN seconds-per-label at the selected N is at most **3.54 s** — `c(N) <= 4`, four times the MEASURED 0.885445 | a setting whose contention eats what its parallelism buys. **Four** because §3.4's own SMT note treats a 2x per-tranche slowdown at 16 as the sign that sharing cost more than it paid, and a factor of four is the point at which N = 16 could not beat N = 4 at all; the 120 s `hang_timeout_ms` watchdog stays the backstop and fires as a run failure, not as C3 |
| **C4** | the N = 1 realised seconds-per-label is within 20% of the pilot's MEASURED 0.885445 | a study whose serial baseline is not the sweep's — a box, a binary or a workload that makes the comparison about something else |

**C4 IS THE ONE THAT KEEPS THE REST HONEST**: an external referent, measured by a
different run of a different document. **C4's referent was measured on the
pilot's binary** (`180b4c40…`, rustc 1.97.1) and this study runs the closure
binary under rustc 1.98.0; a difference inside 20% is what the criterion tolerates,
and one outside it is exactly the finding C4 exists to surface.

### 3.6 THE REGISTERED CONSEQUENCE OF EACH FAILURE

| failure | consequence |
|---|---|
| **C1** | **THE SWEEP STOPS, not this study.** Two concurrent captures of one report disagreeing means the sweep's labels depend on machine load; it is a finding about the engine or the harness and `wp21_prereg.md` may not run until it is closed |
| **C2** | the affected setting is VOID and re-run once; a second void at the same N drops that N from the field |
| **C3** | that N is refused whatever its throughput |
| **C4** | **the whole study is VOID.** No N is selected, the sweep keeps its incumbent 8, and the gap between this box and the pilot's is the finding |

---

## 4. LEVER B — THE LABEL CACHE. WHAT IS REGISTERED.

### 4.1 The measurement that came first. **TAKEN.**

Over the pilot's own capture, the asked prefixes and the distinct `position` lines
under the cache's own key; **if the ratio were below 1.5, lever B would have been
dropped and no code written** — the threshold was registered before the count.
The instrument is `tools/label_cache_count.py`, committed with a test driving the
shipped script; it verifies the capture's body against its `# body_sha256` and
counts under three keys at once:

```
python3 tools/label_cache_count.py --capture <the pilot's capture_v1.txt>

asked prefixes                          742
distinct `position` lines (cache key)   347
distinct sorted (cell, player) lists    347
distinct symmetry-folded stone lists    347
duplication factor (asked/cache key)    2.1383
hit rate                                0.5323
   345 `position` line(s) asked 2 time(s)
     2 `position` line(s) asked 26 time(s)
```

**RATIO 2.1383 >= 1.5, so lever B survives**, and 0.5323 MEASURED is what
`wp21_prereg.md` §3 re-derives the wall with. Receipt
`artifacts/arc3_leverB_41_count_v3.txt`; its digest and the instrument's are in §8.
**The multiplicity line is the one that changes a reading**: 345 prefixes asked
twice plus two asked 26 times — `position start` and `position start moves 0,0`,
byte-identical across all thirteen openings because rule 3 forces turn 1 to the
origin. Cross-GAME sharing of one spelling, scaling with the number of games, and
no cross-game transposition in this corpus at all.

**THE SECOND INSTRUMENT, AND WHAT ITS AGREEMENT IS WORTH.** `docs/process.md`
asks for a second instrument that does not share the stage under doubt. The stage
under doubt is the DERIVATION of the two coarser keys from a `position` line —
`stones_of`, the player alternation, the twelve images — which is 47 lines of the
script; the exact-key count is a raw string added to a set and has no derivation
to doubt. The second instrument is the corpus `arena --labels` writes from the
same capture, whose `key_pos` and `key_full` columns are computed by
`pistol-core` (`labels.rs`, `GameState::key` and `canonical_form`) — a different
implementation of the same two folds in a different language:

```
/usr/bin/grep -v '^#' <corpus_v1.txt> | cut -f5 | LC_ALL=C sort -u | wc -l     # key_pos, the transposition fold
/usr/bin/grep -v '^#' <corpus_v1.txt> | cut -f6 | LC_ALL=C sort -u | wc -l     # key_full, the symmetry fold
```

**THE AGREEMENT CRITERION**: the script's sorted-stone count equals the corpus's
distinct `key_pos`, and its symmetry-folded count equals the corpus's distinct
`key_full`. **Consequence of disagreement**: a finding about the instruments that
suspends lever B's clearance until closed; it is not resolved by preferring one.
**And it is a REPLICATION, not a pre-registered second instrument**: the first
instrument had run before this pipeline was registered, which `docs/process.md`
names as the order that does not count. It is recorded as what it is. Taken:
**347 and 347**, agreeing with the script's 347 and 347 (§7.1).

### 4.2 What is implemented

`wp21_label_cache_design.md` revision 6, in one paragraph: a memo inside
`capture::run`, keyed on the `position` line's exact bytes, holding the
**post-`normalise`** `(totals, bestmove)` pair — exactly the two strings the
record carries, so a hit reproduces the record's bytes by construction. Every
record is still written, in the same order, with the same fields. **The switch is
`--label-cache`, an optional LAST word on `arena --capture`, absent meaning off**,
named in `crates/pistol-arena/src/usage.rs`, the `--census` precedent
(`bin/arena.rs:59-74`); **the default being off is §4.4's mechanism.** The pass
prints one counts line — the mode, `asks`, `records`, `hits`, the two collision
counters and `fold_ms` — and `asks` is a counter at the call to the engine, never
derived from the file. **It refuses to run with a census, by name, in either
order**: a hit performs no search and emits no census row, so a cached census
capture would write fewer rows than positions asked. The map is a `BTreeMap`:
no hasher, one string compare where a hit replaces a search.

**ONE CONSEQUENCE IS REGISTERED HERE BECAUSE A LATER CONSUMER WOULD NOT SEE IT**:
under the cache, ~53% of corpus records carry `search_nodes` for a search that was
not run. Nothing in this sweep sums that column, but a consumer summing it as
machine work would over-count by the duplication factor.

### 4.3 What it costs to be wrong

A cache that returns a label for the wrong position writes a corpus whose records
are silently mislabelled, and nothing downstream would notice. That is why §4.4 is
byte-identity against an uncached run and not a sample.

### 4.4 THE CRITERION, AND IT IS AN EXTERNAL REFERENT

**An uncached capture and a cached capture of the SAME report produce
BYTE-IDENTICAL capture files** — pass 2's output, compared with `cmp -s`. The
uncached pass is the referent and shares no code with the cache. Taken twice: over
the pilot's report (742 records, ~11 min uncached and ~5 min cached) and **over
tranche one's own report**, whose uncached capture the sweep must take anyway —
so tranche one's corpus of record is its UNCACHED capture, and the only added
cost is its cached re-capture (1.43 h ESTIMATED, on the critical path,
`wp21_prereg.md` §3).

- **no tranche runs cached until the comparison returns byte-identity**, with a
  MECHANISM (the flag is absent unless typed) and a NAMED CHECKER (the run log:
  the `cmp -s` exit line with its timestamp appears before the first block whose
  command carries `--label-cache`, `wp21_prereg.md` §5);
- **a single differing byte voids lever B**, and the cache is not repaired and
  re-compared: a cache whose first byte-identity run failed is one whose
  correctness is being fitted to the check. **Consequence: lever B is abandoned
  and the sweep runs uncached**;
- **the diagnosis is not registered in advance.** The two candidate causes — the
  cache, and `newgame`'s isolation — are separated before either is named, by
  re-running the UNCACHED pass twice over the same report: if the two uncached
  runs differ the finding is the engine's, if they agree it is the cache's.

### 4.5 The decision rule

Lever B stays taken **only if** §4.4 returns byte-identity on both reports **and**
the cached capture's measured seconds per MISS — its wall over its `asks` — is
within 5% of the uncached capture's seconds per record, the last excluding a cache
whose bookkeeping ate its own saving. The architect's approval (D-576) settled
whether the cache is built; it cannot settle whether it is right, and a failure of
either clause abandons the lever per §4.4.

---

## 5. THE COST OF THIS STUDY, ON ITS OWN FACE

| part | ESTIMATED |
|---|---|
| the one play pass over openings `0..2` | ~5 s |
| lever A: 5 settings x 3 reps, **152 records MEASURED** per process (`artifacts/arc3_leverB_41_count_v3.txt`), ~135 s at N = 1 and rising with N | **~1.0 h** at contention 1.0–2.0 |
| lever B §4.1 | seconds — it reads a file |
| lever B §4.4's pilot pair | ~11 min + ~5 min |
| lever B §4.4's tranche-sized pair | the cached re-capture alone, ~1.43 h, on the sweep's critical path |

**AGAINST WHAT IT SAVES**, all from `wp21_prereg.md` §3 and not restated: the
cached schedule with its gate is 7.54 h against 7.74 h uncached — **~0.20 h net on
this sweep**, and 1.63 h only if the verification were free. Counting lever A's
hour and the pilot pair, **the lever is net negative on this sweep's wall**. There
is no later sweep to point at: the book's remainder is this sweep's and its last
thousand are the holdout. **What the lever buys is a verified capability and a
smaller second wave**, and the decision to take it is the architect's (D-576),
taken against this number.

---

## 6. WHAT THIS STUDY DOES NOT DO

1. **Parallelise a capture internally.** `capture::run` walks one channel, and N
   tranches already give the box N-way parallelism; an internal split would add a
   deterministic-merge obligation for nothing.
2. **Touch the two seats.** The duplication is structural (D-558(2)).
3. **Make a strength claim or produce a corpus.** Every artifact it writes is
   scratch on `/home` and is deleted; nothing it produces may enter a corpus.

---

## 7. THE COMMANDS, AND THE DRY RUN THAT PROVES THEY ARE THE BINARY'S

```
# the play pass — the sweep's seat over the pilot's consumed range, from the SHIPPED generator
tools/wp21_tranche_config.py --skip 0 --take 3 --pilot-range --out <SCRATCH>/playpass.toml \
                             --binary-sha256 <the current binary's digest>
arena --config <SCRATCH>/playpass.toml --out <SCRATCH>/report.txt

# lever A — one of the N concurrent processes at each setting
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/cap-N<n>-rep<r>-p<i>.txt --label-nodes 400000

# lever B §4.4 — the pilot pair
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/uncached.txt --label-nodes 400000
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/cached.txt   --label-nodes 400000 --label-cache
cmp -s <SCRATCH>/uncached.txt <SCRATCH>/cached.txt

# §4.1's count, and its second instrument
python3 tools/label_cache_count.py --capture <a capture file>
/usr/bin/grep -v '^#' <the corpus from that capture> | cut -f5 | LC_ALL=C sort -u | wc -l
/usr/bin/grep -v '^#' <the corpus from that capture> | cut -f6 | LC_ALL=C sort -u | wc -l
```

**`<SCRATCH>` is on `/home`, never `/tmp`** (a 24 GiB tmpfs). **The box is
otherwise idle for lever A** — no cargo, no bench, no other session's job, checked
with `ps` before the first setting — because a concurrent job is contention nobody
registered.

### 7.1 THE DRY RUN — what it exercised, and its record

Input of the same kind and never the registered workload: the generator's
`--skip 0 --take 1 --pilot-range` stand-in, one opening, two games, at
`--label-nodes 2000`. Its limbs:

1. every command above is accepted by the shipped binary and exits 0;
2. `--label-cache` with `--census` is REFUSED naming both words, in both orders,
   before any game and leaving no output file;
3. the cached and uncached captures of the one report are byte-identical under
   `cmp -s` — at a toy budget, a shakedown and not §4.4's criterion;
4. the cached run's counts line reads `asks < records`, and the uncached run's
   `asks == records` — the cache was exercised, read off the count the pass
   prints and not off the file, which is identical either way;
5. §4.1's script and its second instrument agree on the stand-in's corpus.

**A DRY-RUN FAILURE STOPS THE STUDY AND IS REPORTED AS A FINDING.**

**RECORD — SLOT.** Filled from the run before this revision's review is
dispatched: the stand-in config's sha256, every command's exit line, the two
counts lines, the refusal text in both orders, the `cmp -s` exit, the three
counts of limb 5. **The review that governs the run is taken after the record is
filled.**

---

## 8. THE INSTRUMENTS, WITH THEIR REVISIONS AND DIGESTS

| instrument | what it produces | revision / digest |
|---|---|---|
| `tools/label_cache_count.py` | §4.1's counts | `1a890b5331c302cf97372603e7ec21a41b08e6131390d92b78b0168bc77c1e18` |
| `artifacts/arc3_leverB_41_count_v3.txt` | §4.1's receipt (gitignored; anchored here) | `cbad0786505e8d7958610a2ff24d8b4186de29fd85b0127d60df7b04b4e342ed` |
| the `sort -u` pipelines | §4.1's second instrument | the commands ARE the revision; printed in §7 |
| `tools/wp21_tranche_config.py` | §3.1's play-pass config | **SLOT** — its sha256 with the `--pilot-range` form landed |
| `arena`, `pistol` | every capture in §3 and §4.4 | **SLOT** — the closure binaries' digests, `wp21_prereg.md` §8, with `rustc --version` beside them |
| `cmp -s` | §4.4's verdict | POSIX, no revision |
| the pilot's rate 0.885445 | C4's referent | `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`, `capture1 seconds=657` over 742 records, on binary `180b4c40…` under rustc 1.97.1 |
