# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 3.

> **ONE LINE.** The sweep's registered wall is **7.74 h at N concurrent tranches
> with N a registered slot at incumbent 8, and that is a LOWER BOUND with
> contention assumed at 1.00** — `wp21_prereg.md` §3 owns that number and this
> document does not restate it. This study measures the two things that could
> move it: **what concurrency the box actually pays for**, and **whether the
> 2.14x duplication the corpus is REQUIRED to carry has to be paid for twice in
> SEARCH time as well**. It changes no label, no criterion and no captured byte;
> it may only change how long producing them takes. **On THIS sweep the cache's
> applicable saving is 1.63 h ESTIMATED against ~1.43 h of registered
> verification (§5) — roughly a wash, stated before the run.**

**REVISION 3, AND WHAT MOVED.** Revision 2 went to its first fresh-context
review and came back **FAIL** — 2 BLOCKING, 12 MAJOR, 8 minor
(`wp21_throughput_prereg_rev2_REVIEW.md`, at
`fce50bc5b00baab9066f1e7bdf10c48c025b7755`). Every finding is disposed of below
at the section that owns it. **THE MOST USEFUL THING THE REVIEW PRODUCED IS AN
ATTACK THAT FAILED**: asked to break the key's soundness, it hunted the TT
generation counter, the census collector, `Instant::now()` on the ordering path,
hash iteration order, `Board::stones()`, the eval's incremental apply/undo,
`ThreatState`'s rebuild order and the solver, and **found no within-process
channel by which two identical `position` lines could answer differently**. That
is recorded in §2 as the reason the key stands.

**THE FOUR THAT CHANGED SOMETHING.** (1) `D-576` did not exist when revision 2
cited it; it does now, and §0 of the arc landed it. (2) **§3.2's registered
command was refused by the shipped binary** — the word order is positional at
`crates/pistol-arena/src/bin/arena.rs:51` — and no dry run had been taken, which
is exactly the discipline that would have caught it; §7 is that dry run.
(3) The §4.1 receipt did not contain the rows §2 and §4.1 quoted from it; the
count is re-taken by a committed instrument, `tools/label_cache_count.py`.
(4) The KEY was selected without an OPTION MATRIX, which CLAUDE.md calls the same
breach as silent architecture drift; the matrix is
`docs/experiments/matrix_label_cache_key.md` and it is attacked by a
fresh-context DECISION-RED-TEAM.

**AN AMENDMENT REOPENS THE REVIEW however small the diff, so revision 3 owes its
own fresh-context review before the first run it governs.**

**REVISION 2, AND WHAT MOVED.** The architect ruled on lever B before this
document was reviewed (arc III §R, R1; landed as **D-576**): the cache is
**APPROVED**, the sweep runs with it, and the cold-label criterion of
`wp21_prereg.md` §4 is amended to sample cache hits and misses separately.
**THAT CHANGES WHAT THIS STUDY IS FOR ON LEVER B**: §4.5's *"take it only if"*
becomes *"it is taken; these are the checks that can still remove it"*. Four
things move, each at the section that owns it — §2 (revision 1's soundness
argument for its key is FALSE, and the key that is unconditionally sound is
registered here), §4.1 (its count is
TAKEN, and the circumstances are recorded rather than smoothed), §4.4 (its
tranche-sized referent becomes work the sweep must do anyway) and §4.5. Lever A
is untouched. **AN AMENDMENT REOPENS THE REVIEW however small the diff
(CLAUDE.md, Process), and revision 1 never had one, so revision 2 owes the first
fresh-context review this document has ever had, before the first run it
governs.**

**WHY IT IS REGISTERED RATHER THAN JUST RUN.** The output is a number that
decides a run parameter of a 3,487-opening sweep, and a concurrency picked after
seeing which setting looked fastest on the day is the after-the-numbers decision
`docs/process.md` forbids. The decision rule is in §3.4 and §4.5, before either
lever is measured.

**WHAT IT MAY NOT DO** (stated first, because a throughput study is exactly where
this gets lost): it may not change the label budget, the game budget, the turn
cap, the seat, the criteria of `wp21_prereg.md` §4, or which openings are swept.
A faster sweep that answers a different question is not a faster sweep.

---

## 1. THE TWO LEVERS, AND WHY THESE TWO

**THE COST IS CAPTURE AND ALMOST NOTHING ELSE.** Per tranche, from
`wp21_prereg.md` §3's own arithmetic: capture **11 017 s of 12 870 s — 85.6%**.
Play is 11%, replay 3%, the cold check 0.4%. **A lever that is not on the capture
pass cannot matter**, which is what rules out the obvious ones: `n_workers` on
pass 1, a faster replay, a wider cold stride.

| lever | what it attacks | code change |
|---|---|---|
| **A — concurrency** | the sweep runs 16 tranches 8 at a time on a box with 8 physical cores and 16 threads. Whether 8 is the throughput optimum is **UNMEASURED** | none |
| **B — the label cache** | `capture::run` asks the engine at **every asked prefix of every game**, and the pilot MEASURED 742 records over **347 distinct positions**. The same position is searched 2.14 times at `nodes 400000` | `pistol-arena`, and it is a package with its own review |

**LEVER B'S SIZE IS NOT AN ESTIMATE AT THE GAME LEVEL.** The pilot's run log
reads `n 26 distinct-n 13 (13 duplicate games)` — **exactly 2.0**, and it is
structural rather than a setting: `arena --capture` refuses a report whose two
seats do not attest one engine (D-558(2)), so a self-match is the only capturable
shape, and a self-match of one deterministic engine plays each opening
identically in both seats. **Every asked prefix of every game is therefore asked
at least twice.**

**REVISION 2 SAID THE EXCESS OVER 2.0 WAS "the cross-game transpositions on top".
IT IS NOT, AND §4.1's COUNT SAYS WHAT IT IS.** The multiplicity structure of the
pilot's own capture is **345 `position` lines asked twice and TWO asked 26 times**
— and the two are `position start` and `position start moves 0,0`, byte-identical
across all thirteen openings because game rule 3 forces turn 1 to the origin.
That is cross-GAME sharing of one spelling, which scales with the number of games
rather than with the density of the position space, and a successor sent looking
for a transposition fold would find none. **§4.1 OWNS THIS NUMBER AND EVERY OTHER
SECTION POINTS THERE** (D-423).

**AND IT DOES NOT TOUCH D-562(2).** That ruling forbids DEDUP AT CAPTURE because
*"a capture that dropped a duplicate would destroy the 2.14x duplication factor
D-560's whole arithmetic rests on"*. **A cache drops no record.** Every record
the uncached pass writes, the cached pass writes, with the same bytes; what
changes is only whether the engine was asked again for an answer already known.
The duplication factor survives intact because the duplication survives intact.

---

## 2. THE SOUNDNESS ARGUMENT FOR LEVER B, MADE BEFORE ANY MEASUREMENT

**A LABEL IS A PURE FUNCTION OF (position, budget, binary, config), AND THAT IS
STRUCTURAL IN THIS TREE RATHER THAN HOPED FOR.** `crates/pistol-arena/src/capture.rs`'s
`ask` sends `newgame` before every `position`/`go`
(`for line in [NEW_GAME, position, go]`); `pistol-engine`'s `new_game` calls
`Searcher::clear`, which clears the transposition table, the heuristic tables and
the solver. **NO STATE THAT CAN MOVE AN ANSWER CROSSES AN ASK — and revision 2
said "no state crosses an ask", which is literally false.** The exception is the
trigger-census collector, whose own comment at
`crates/pistol-engine/src/instance.rs:112-119` says *"`new_game`'s `clear` does
not touch it"*, and `Searcher::census_folds` (`search.rs:83`), which `clear` does
not reset either. Neither can move a `bestmove` or a node count, and §4.2 refuses
to combine a census with the cache for the separate reason that a hit performs no
search — so the one piece of state that does cross an ask is the one the cache is
forbidden to run beside. So two asks at one position return the same bytes, and
that is what makes a cache a re-use rather than an approximation.

**REVISION 2 REPLACES THE KEY, BECAUSE REVISION 1's SOUNDNESS ARGUMENT FOR IT IS
FALSE.** Revision 1 said: *"Two prefixes whose move orders differ but whose stones
agree replay to the SAME `GameState` — the board is a set of stones and the stone
count fixes both `to_move` and `phase` — so they must take the same label."*
**THEY DO NOT REPLAY TO THE SAME `GameState`.** `GameState` carries `played()`,
the stones in PLAY ORDER, and the search reads it — **by exactly one path, traced
here rather than asserted.** `crates/pistol-search/src/heuristics.rs:191-193`
(`last_stone`) is consulted at `:155` under `gates.countermove`; two transposed
prefixes have different last stones, so under that gate they can order the root
differently and return a different `bestmove` and node count. **THE OTHER TWO
GATES DO NOT REACH IT, and an earlier draft of this paragraph said they did**:
`:89`'s walk over `played()` sits inside `record_cutoff`, gated at its call site
by `params.ordering.any()` (`pvs.rs:491-498`, `params.rs:114-116`) rather than by
`killers`, and its result feeds only `pair_killers` — written at `Phase::Second`
nodes, where the stone is search-placed, and every capture root is `Phase::First`
— and the countermove table, read only at `:155`. `history` never reads
`played()` at all. **THE SWEEP'S OWN SEAT IS SAFE BY ONE CONFIG VALUE** —
`configs/instrument_v0.toml:81` sets `countermove = false` — **and one value's
shadow is not a soundness argument.**

**THE KEY IS THE `position` LINE ITSELF, THE EXACT BYTES THE ASK IS MADE WITH.**
A hit returns the answer to a question spelled identically, so the cache's
soundness rests on a law this project already gates: **the same binary, the same
config, the same `position` and the same `go`, after a `newgame`, return the same
bytes — hard rule 4, and `tools/determinism.sh` is CI gate 9 of 19**
(`tools/ci.sh:104-105`, *"gate 9/19: cross-process determinism"*; **revision 2
said gate 6, which is config validation**). The gate's `C vs D` limb is the
cache's own question in the gate's own words: *"the same positions under one
budget, C with one process per position and D with all of them in one session …
this pair is that question"* — whether `newgame` really clears what the previous
position left in the table.

**AND THE GATE DOES NOT RUN THIS SWEEP'S SEAT OR THIS SWEEP'S BUDGET, WHICH IS
STATED HERE RATHER THAN LEFT FOR A READER TO FIND.** Its `SEATS` array is
**five** seats, not the four an earlier revision transcribed from the script's own
stale header comment — `radius`, `staged`, `staged-heuristics`, `staged-solver`
and `staged-safety-net-cap` (`tools/determinism.sh:67-80`, read off the `SEATS`
array rather than off the prose above it) — and its budgets are `depth_turns 4`
and `nodes 200000`, not `configs/instrument_v0.toml` at `nodes 400000`. So
*"already gated"* is true of the PROPERTY and not of the CONFIGURATION the sweep
runs.

**AND THE SHARPER FACT IS THAT NO LIMB OF THAT GATE EVER ASKS THE SAME POSITION
TWICE INSIDE ONE PROCESS**: `A vs B` runs one script in two processes; `C vs D`
runs one-process-per-position against all-positions-in-one-session, and the
session limb asks each position once. **A cache hit is exactly "the same position,
asked again, in the same process", and that is the one shape the determinism gate
never takes** (D-581). **What covers the configuration is §4.4's own
byte-identity run**, taken at the sweep's seat and the sweep's budget, and that
is the reason §4.4 may not be weakened. No claim about what the search reads from
the state is needed, and none is made.

**AND THE STRONGEST ATTACK ON THE KEY WAS ATTEMPTED AND FAILED**, which is worth
more than the argument above. Revision 2's reviewer hunted for any within-process
channel that could make the second ask at one `position` line answer differently:
the TT generation counter (`Table::clear` resets it to 0), the census collector
(cannot move an answer, and §4.2 refuses the combination), `Instant::now()` on
the ordering path (`ordering.rs` performs zero clock reads under `Stop::Nodes`),
hash iteration order (`Board` is a `BTreeMap`), `Board::stones()`, the eval's
incremental apply/undo, `ThreatState`'s rebuild order (`Position::reset_to`
rebuilds from a freshly replayed state) and the solver. **It found none.** **A
128-bit zobrist would have been the obvious key and is deliberately not used**:
a cache that answers from a collision returns a label for a position nobody
played, and a probability argument is not a criterion. The key here is not a
digest of the question; it IS the question.

**THE COARSER KEYS BUY NOTHING ON THE ONLY POPULATION ANYONE HAS MEASURED — AND
THAT POPULATION IS THIRTEEN OPENINGS, NOT 3 487.** Over the pilot's own capture the
distinct counts under the exact key, the sorted stone list and the twelve-image
symmetry fold are **347, 347, 347**
(`artifacts/arc3_leverB_41_count_v3.txt`), and the hit rate is 0.5323 under all
three. **THAT DOES NOT TRANSFER, AND `matrix_label_cache_key.md` §2 says by how
much**: the pilot offers 1 576 cross-game transposition opportunities and the
sweep ~161 million, a factor of **102 346**, and a rule-of-three bound on the
pilot's zero admits **up to ~307 000 merges** at the sweep's scale. **AND THE A-PRIORI ARGUMENT AN EARLIER REVISION OFFERED IN ITS PLACE IS FALSE**:
it said the book's `canonical_form` dedupe forbids two openings transposing or
mirroring at `k <= opening_turns`. Deduping the WHOLE opening constrains nothing
about its PREFIXES. Measured over the sweep's own window with
`tools/opening_prefix_fold.py` (receipt
`artifacts/arc3_opening_prefix_fold.txt`), at `k = 2` the symmetry fold takes
tranche one's **213** exact-key classes down to **171**, and the dedupe's zero
appears only at `k = 3`. **WHAT THAT IS WORTH IS 42 SEARCHES A TRANCHE — 0.72% of
its ESTIMATED 5 819 misses** — and every one of the 42 is a prefix whose symmetry
partner is a different position, so it is exactly where a symmetry key would
return a `bestmove` in the wrong frame.

**THE SELECTION DOES NOT REST ON THAT ROW.** Where a corpus does hold a
transposition this cache MISSES it — **a lost saving, never a wrong answer**,
which is the only direction an error in a cache may fall, and every coarser key
buys that saving with a class of wrong answer instead.

**AND THE SWEEP MEASURES THE ROW NOBODY COULD MEASURE BEFOREHAND.** The cache
counts, per tranche, **how many of its own MISSES share a `key_pos` or a
`key_full` with an earlier miss** — two counters, no extra search, reported in the
run log beside the hit rate. That is the coarser keys' yield at the scale that
matters; it is the matrix's registered flip clause; and it settles D-562(2)'s open
three-key question in the same pass.

**THE CACHE'S EQUIVALENCE IS THE FINEST ONE THERE IS, AND SAYING SO CLOSES A
QUESTION A REVIEWER WOULD OTHERWISE HAVE TO DERIVE.** Keying on the `position`
line folds NOTHING — not symmetries, not transpositions, not even a re-spelling
of one turn, because a `Turn::Pair` is canonical by its own type invariant
(`crates/pistol-core/src/turn.rs:90`, *"`first < second`"*) and `FromStr` refuses
an uncanonical spelling rather than reordering it (`turn.rs:213-215`).
**Revision 2 attributed this to `exchange::position_line`, which merely calls
`turn.to_string()`** (`exchange.rs:154-161`); the conclusion held and the
attribution did not. It is strictly finer than `key_seq`, `key_pos` and
`key_full` alike. **D-562(2)'s unsettled three-key question is therefore not
merely untouched but unreachable from here**: the cache decides nothing about the
CORPUS — every record is written, with its own `position` string — and it cannot
express a disagreement between two keys because it only ever compares a key with
itself. *Which key rules a disagreement* stays WP-2.0b's transposition question,
as D-562(2) says.

### 2.0 THE KEY IS ONE ARGUMENT OF FOUR, AND THE INVARIANT THAT MAKES THAT SAFE IS REGISTERED HERE

The first sentence of this section says a label is a pure function of
**(position, budget, binary, config)** and §4.2 registers the key as the
`position` line alone. **That is safe only under a scope invariant, and revision 2
gestured at it without registering it.**

**THE INVARIANT, REGISTERED**: *the memo is constructed inside `capture::run` and
dropped with it; it is never a field of a longer-lived value, and it is never
shared between two invocations.* Within one `capture::run` the other three
arguments are fixed — `label_go_line` is computed **once** at
`crates/pistol-arena/src/capture.rs:333`, outside both the game loop (`:340`) and
the prefix loop (`:341`), and the binary and the config are the process's — so
the `position` line is the only argument that varies and the key is complete.

**THIS IS AN OBLIGATION THE CACHE'S OWN REVIEW CHECKS**, not a remark: an
implementer who lifts the memo to a struct reused across two `--label-nodes`
values silently makes the key wrong, and §4.3 says exactly what that costs —
nothing downstream would notice.

**THE CRITERION THAT DECIDES IT IS AN EXTERNAL REFERENT AND IS REGISTERED HERE**:
§4.4. It is not internal agreement, not a plausible speedup, and not the cold
check alone.

### 2.1 THE KEY THE RULING NAMES IS COARSER STILL, AND IT ADMITS A WRONG ANSWER

**R1 NAMES A DIFFERENT IDENTITY AND THE DIFFERENCE IS NOT COSMETIC.** R1: *"the
sweep caches labels by the WP-2.0b canonical identity"*. D-570's canonical
identity folds **symmetries as well as transpositions**. A cache keyed on it
answers a position with the `bestmove` computed for a **symmetry image** of that
position — a move in the wrong frame, which is a wrong label and not a slower
one — and with node counts that are not symmetry-invariant, because the search's
lexicographic tie-break is not (D-137).

**THE FOLD IS DECLINED ON TWO GROUNDS RATHER THAN ONE.** First, correctness: the
key above cannot return another position's answer, and the canonical key can.
Second, **the fold has never yielded anything any measurement in this project has
found**: WP-2.0b §9 measured 798 in-tree firings on two committed fixtures at
both caps and found distinct `key` equal to distinct `key_pos` in every cell, and
§4.1's count finds all four columns returning **347** on the pilot's 742 records.
**A fold that has never merged a pair, in exchange for a class of wrong answer, is
not a trade this study takes.** Landed as D-576.

**THE KEY IS SETTLED BY AN OPTION MATRIX AND NOT BY THIS SECTION.**
`docs/experiments/matrix_label_cache_key.md` ranks five candidate keys — the
exact bytes, the sorted stone list, `canonical_form`, a 128-bit zobrist and
`canonical_sequence` — with every numeric claim marked MEASURED or ESTIMATED, and
is attacked by a fresh-context DECISION-RED-TEAM. **Revision 2 selected the key
in prose and that was a process breach**, caught as MAJOR 11 of its own review;
this section now states the outcome and the matrix carries the argument.

**THE RULING'S SUBSTANCE IS TAKEN IN FULL AND ONLY ITS KEY IS DECLINED**: the
cache is built, the sweep runs with it, and the cold-label criterion is amended
to sample hits and misses separately — which under this key is a stronger check
than under R1's, because a hit and its miss are then the SAME `position` line
asked twice and any difference is the cache's alone.

---

## 3. LEVER A — CONCURRENCY. THE MEASUREMENT.

### 3.1 The workload, fixed before any setting runs

**ON THE PILOT'S ALREADY-CONSUMED RANGE, so this study spends nothing.**
`docs/book_v2_ledger.md` records `0..12` as consumed by the WP-2.0 pilot, and
D-539 says the pilot is not corpus — re-reading it buys no evidence about
anything and costs no unseen opening. **Openings `0..2`** (3 openings, 6 games)
are played once into ONE report, and every concurrent process in every setting
captures THAT SAME REPORT. One workload, so the settings differ in concurrency
and in nothing else.

**THE PLAY PASS'S OWN INSTRUMENT IS REGISTERED HERE**, because revision 2 named
the workload and no config to produce it, and §3.4's 120 s guard reads
`hang_timeout_ms` off that report: **`configs/arena_wp20_label_pilot.toml` as
committed, with `openings_skip = 0` and `openings_take = 3`**, which is the
pilot's own seat — `hang_timeout_ms = 120000`, `configs/instrument_v0.toml` on
both seats — and the only change is the take. Its command is in §7 with the
others.

### 3.2 The settings and the reps

**THE FIELD IS {1, 2, 4, 8, 16} AND 12 IS DELIBERATELY ABSENT.** With sixteen
tranches, wall is `ceil(16/N)` waves, so only an N that DIVIDES 16 gives balanced
waves. N=12 runs twelve tranches and then four, leaving two thirds of the box
idle for the whole second wave — it cannot beat N=8 and can only tie it. **The
consequence is a rule and not a note: if the sweep's concurrency changes, the
TRANCHE COUNT changes with it**, so that `TR mod N == 0`. A study that measured
12 and recommended it would be recommending a wave structure nobody costed.

Concurrency **N ∈ {1, 2, 4, 8, 16}**, **3 reps each**, run in the fixed order
`1, 2, 4, 8, 16` and then that order twice more — **not** three reps of one
setting before the next, so a thermal ramp lands on every setting equally rather
than on the one that ran last. (This is `wp20b_perf_guard.sh`'s rotation lesson,
which cost that package a whole run.)

At each setting, N processes each run
`arena --capture <the report> --out <distinct path> --label-nodes 400000`
concurrently, started together, and the setting ends when the LAST one exits.

**THE WORD ORDER IS THE BINARY'S AND NOT A PREFERENCE.**
`crates/pistol-arena/src/bin/arena.rs:51` matches the words POSITIONALLY —
`["--capture", source, "--out", out, "--label-nodes", nodes]` — and its own
refusal says *"each in that order"*. **Revision 2 registered the words in a
different order, so its registered command was refused by the shipped binary**,
and §7's dry run is what would have caught it before a reviewer did. Every
command this document registers is in §7, taken.

### 3.3 The statistic, and it is one

**Realised seconds per label at concurrency N** =
(wall from first start to last exit) x N / (N x records) = wall / records,
where `records` is the report's own record count — so the statistic is simply
**the wall of one process's capture at that concurrency**, and the aggregate
**throughput** is `N x records / wall`.

**THE MACHINE-SECONDS STEP IS NOT EXACT AND IT CANCELS, WHICH IS SAID HERE RATHER
THAN GLOSSED.** `(wall from first start to last exit) x N` multiplies the MAXIMUM
process wall by N, so where processes finish unevenly it over-states machine
seconds, most at the settings with the highest variance. It cancels in the
reported statistic — the expression reduces to `wall / records` — so no
conclusion moves; the derivation as written is an upper bound rather than an
identity.

Reported per setting: **median over the three reps** of the throughput, and the
**contention factor** `c(N)` = (realised seconds per label) / **0.8854**, the
pilot's MEASURED serial rate.

### 3.4 THE DECISION RULE, REGISTERED BEFORE THE RUN

**The sweep runs at the N whose median THROUGHPUT is highest**, with two guards
that fire before it:

- **A tie inside 5% goes to the SMALLER N.** Two settings that produce the same
  throughput are not the same run: the smaller one leaves the box able to answer.
- **The selected N must divide the tranche count**, per §3.2. An N that does not
  is not selected; the tranche count is re-registered to a multiple of it, and
  that re-registration reopens `wp21_prereg.md`'s review.
- **Any N whose realised MEAN seconds-per-label exceeds 12 s is REFUSED whatever
  its throughput.** **REVISION 2 WROTE THIS OVER "a single label" AND NOTHING
  MEASURES A SINGLE LABEL'S WALL**: `capture::normalise`
  (`crates/pistol-arena/src/capture.rs:66-96`) STRIPS ` nps <n> time <n>` before
  a record is written — `/usr/bin/grep -c " time " capture_v1.txt` returns
  **0** — and `arena` prints one line per capture pass, not per label. The only
  per-label quantity the instrument yields is the mean, which is §3.3's own
  statistic. **THE MARGIN IS REGISTERED AND IT IS TEN-FOLD**: 12 s against the
  120 s `hang_timeout_ms` watchdog, chosen so that a single label an order of
  magnitude worse than the mean still does not reach the watchdog, and stated
  before any setting runs. A setting whose mean crosses 12 s is refused; the
  watchdog itself remains the backstop and fires as a run failure, not as C3.
- **If the highest-throughput N is 16, the wall figure reported for it carries
  the note that 16 is SMT-shared**, because a per-tranche time at 16 that is more
  than 2x the time at 8 means the second wave was cheaper than the sharing.

**WHAT THE RESULT MAY AND MAY NOT DO.** It may set `wp21_prereg.md`'s
concurrency, which is a run parameter and not a criterion, by an amendment that
reopens that document's review. **It may not** change tranche count, partition,
or any §4 criterion.

### 3.5 THE CRITERIA, AND THE DESIGN ELEMENTS THEY ARE NOT

**AND C1 AND C2 NAMED THE WRONG ARTIFACT IN REVISION 2** — *"corpus file"* — when
the registered command is `arena --capture`, whose output is a CAPTURE
(`capture_file.rs`, `CAPTURE_FORMAT_VERSION`); a corpus is pass 3's output and
this study never runs pass 3. The check was right and the noun was wrong.

**Kept apart on purpose.** A design element is something this study DOES; a
criterion is something the data can FAIL. Listing the two in one column is how a
document comes to report four criteria met when two of them were never at risk.

**DESIGN ELEMENTS** — fixed here, and no result can falsify them: one report
captured at every setting (so the settings differ in concurrency and nothing
else); three reps in rotated setting order (so drift lands on every setting
equally); the statistic is aggregate THROUGHPUT and not per-process time (so a
setting is not penalised for doing N times the work).

**CRITERIA** — each with the defect class it excludes, and each falsifiable:

| # | criterion | the defect class it excludes | can it fail? |
|---|---|---|---|
| **C1** | every process's **capture file** — pass 2's output, five TAB fields, not a corpus — at every N and every rep, is byte-identical to every other's | a capture whose answers depend on machine load | **yes** — and §3.6 says what happens then |
| **C2** | every process's record count equals the report's own asked-prefix count | a process that exited early and looked fast | yes |
| **C3** | the realised MEAN seconds-per-label stays below **12 s** at the selected N, a ten-fold margin against the 120 s watchdog | a throughput win that risks the `hang_timeout_ms` watchdog and so buys a voided tranche | yes |
| **C4** | the N=1 realised seconds-per-label is within 20% of the pilot's MEASURED 0.8854 | a study whose serial baseline is not the sweep's baseline — i.e. a box, a binary or a workload that makes the whole comparison about something else | yes |

**C4 IS THE ONE THAT KEEPS THE REST HONEST.** Without it every other number is
internally consistent and externally unmoored: a run on a throttled box would
produce a perfectly self-agreeing scaling curve for a machine the sweep will not
run on. It is an EXTERNAL referent — the pilot's rate, measured by a different
run of a different document — which is what `docs/process.md` says a reviewer
looks for first.

### 3.6 THE REGISTERED CONSEQUENCE OF EACH FAILURE

`docs/process.md` requires an agreement criterion to carry a registered
consequence, stated before the run.

| failure | consequence |
|---|---|
| **C1** | **THE SWEEP STOPS, not this study.** Two concurrent captures of one report disagreeing means the sweep's labels depend on machine load, which no concurrency setting fixes and which voids the registered plan itself. It is reported as a finding about the ENGINE or the harness, and `wp21_prereg.md` may not run until it is closed. |
| **C2** | the affected setting is VOID and is re-run once; a second void at the same N drops that N from the field rather than being investigated here |
| **C3** | that N is refused whatever its throughput, per §3.4 |
| **C4** | **the whole study is VOID.** No N is selected, the sweep keeps its registered 8, and the gap between this box and the pilot's is the finding. |

---

## 4. LEVER B — THE LABEL CACHE. WHAT IS REGISTERED BEFORE THE CODE EXISTS.

### 4.1 The measurement that comes FIRST, and needs no code. **TAKEN.**

Over the pilot's own report, count **asked prefixes** and **distinct replayed
positions** under §2's exact key. The ratio is what a cache could save, MEASURED
on a real report rather than argued from the pilot's 742/347 summary. If the
ratio is below **1.5**, **lever B is dropped and no code is written** — that
threshold is registered here, before the count.

**THE COUNT, RE-TAKEN BY A COMMITTED INSTRUMENT UNDER THE CACHE'S OWN KEY.**
Revision 2 quoted a four-row block from `artifacts/arc3_leverB_41_count.txt` and
**that receipt carried three rows, under the SORTED-STONE key revision 2 had
replaced**. The row the document quoted did not exist in the file it cited. That
is this arc's own named defect — a claim asserted where it was convenient rather
than derived where it is true — and the remedy is an instrument rather than a
corrected sentence.

**THE INSTRUMENT IS `tools/label_cache_count.py`**, committed, with a test suite
driving the shipped script (`crates/pistol-arena/tests/label_cache_count_tests.rs`,
`docs/process.md`'s tools/ coverage rule). It reads a CAPTURE file — the artifact
the cache would actually serve — verifies the body against its own
`# body_sha256`, and counts under three keys at once. The receipt records the
command with its scope, the instrument's sha256 and the input's.

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

**RATIO 2.1383 >= 1.5, SO LEVER B SURVIVES §4.1**, and the hit rate the sweep's
wall is re-derived with is **0.5323 MEASURED**. Receipt
`artifacts/arc3_leverB_41_count_v3.txt`, which SUPERSEDES `_v1` and `_v2`.

**THE MULTIPLICITY LINE IS THE ONE THAT CHANGES A READING**, and §1 now points
here for it: the duplication is 345 prefixes asked twice plus two asked 26 times,
and the two are `position start` and `position start moves 0,0`. There is no
cross-game transposition in this corpus at all.

### 4.1.1 THE SECOND INSTRUMENT, AND ITS AGREEMENT CRITERION, REGISTERED BEFORE EITHER RUNS

`docs/process.md`: where a run is cheap, doubt about the instrument is answered
by REPLICATION **and by a SECOND INSTRUMENT whose agreement criterion is
registered before either runs**, naming the stage under doubt and saying how the
second instrument does not share it.

**THE STAGE UNDER DOUBT** is the derivation of a key from a record — everything
between reading the file and counting a set. **Re-running the same script over
the same sha-pinned input cannot reach it**: the script is deterministic, so the
re-take returns the same five numbers by construction and could fail only if
someone edited the script. **Revision 2 registered exactly that re-take as its
remedy, and it was a fig leaf.**

**THE SECOND INSTRUMENT DOES NOT SHARE THAT STAGE.** It derives its key from the
**corpus** file's `moves` column rather than from the capture's `position` line —
a different file, written by a different pass (`arena --labels` rather than
`arena --capture`), through a different field — and reaches the count with a
one-line `sort -u` rather than a parser:

```
/usr/bin/grep -v '^#' <corpus_v1.txt> | cut -f3 | LC_ALL=C sort -u | wc -l
```

**THE AGREEMENT CRITERION, REGISTERED HERE**: the two instruments must return the
**same distinct count** and the same asked-prefix count. A disagreement is a
finding about the instruments and **suspends lever B's §4.1 clearance** until it
is closed; it is not resolved by preferring one.

**AND THE RECEIPT IS SHA-ANCHORED**, because `artifacts/` is gitignored (hard
rule 8 permits it only if *"a committed manifest may sha-index them"*): the
receipt's own sha256 is recorded in this document's §8 alongside the instrument's.

### 4.2 What is implemented if it survives

A memo in `capture::run`: §2's key — the `position` line's exact bytes — to the
`(totals, bestmove)` the engine returned. Nothing else changes: every record is
still written, in the same order, with the same fields, and the record's own
`position` field is the string that was the key.

**THE SWITCH IS `--label-cache`, AN OPTIONAL TRAILING WORD ON `arena --capture`,
AND ITS DEFAULT IS OFF.** Revision 2 registered no switch at all, so the package
could equally have been implemented as an unconditional memo — which would make
§4.4's uncached referent unobtainable and `--census` permanently refused, without
violating anything the registration said. The shape follows `--census`'s own
precedent at `crates/pistol-arena/src/bin/arena.rs:57-72`: an optional trailing
word, absent meaning off, named in `crates/pistol-arena/src/usage.rs` which is
the one place the default lives. **THE DEFAULT BEING OFF IS ALSO §4.4's
MECHANISM** — see there.

**IT IS AN ORDERED MAP AND NOT A HASH MAP**, and the reason is cost rather than
rule 4: a `HashMap` used only for point lookups exposes no iteration order and is
not a choice path, and rule 4 admits a fixed-seed hasher in any case
(**revision 2 gave the wrong reason for the right choice**). A `BTreeMap` needs
no hasher decision at all, and the comparison is one string compare against a
search of ~885 ms.

**AND ONE CONSEQUENCE IS REGISTERED HERE BECAUSE A LATER CONSUMER WOULD NOT SEE
IT**: under the cache, **53% of corpus records carry `search_nodes` for a search
that was not run**. Nothing in this sweep sums that column — §6 reports records,
distinct positions and outcome coverage, and §7.2 makes no strength claim, so
hard rule 6's per-side compute does not arise — but a consumer summing
`search_nodes` as machine work would over-count by the duplication factor.

**AND IT REFUSES TO RUN WITH A CENSUS BY NAME.** A cache hit makes no search, so
it emits no census row: a cached census capture would silently write fewer rows
than positions asked. `--census` together with the cache is a named refusal
before any game, not a combination that quietly under-reports. This sweep runs
census-OFF (`wp21_prereg.md`'s preamble), so nothing is lost by it.

### 4.3 What it costs to be wrong

A cache that returns a label for the wrong position writes a corpus whose records
are silently mislabelled, and **nothing downstream would notice**: the record
carries a plausible score for a plausible position. That is why §4.4's criterion
is byte-identity against an uncached run and not a sample.

### 4.4 THE CRITERION, AND IT IS AN EXTERNAL REFERENT

**An uncached capture and a cached capture of the SAME report produce
BYTE-IDENTICAL CAPTURE files** — pass 2's output, five TAB fields, not a corpus,
which is pass 3's and which this study never runs. The uncached pass is the referent and it shares no
code with the cache — it is the pass that exists today. Run over the pilot's
report (742 records), and over one tranche-sized report, and compared with
`cmp -s`.

**REVISION 2 FIXES WHICH TRANCHE-SIZED REPORT, AND THE REASON IS THAT AS WRITTEN
THE LEVER LOST WALL.** §5 prices this leg at *"~2 x (11 min + 3 h)"* against an
ESTIMATED saving of 3.26 h: a tranche-sized referent captured for the check alone
costs more than the cache returns. **The referent is therefore TRANCHE ONE'S OWN
REPORT** — a capture the sweep must take whatever this study concludes — captured
uncached and captured cached, and compared with `cmp -s`. Three things follow and
each is registered here:

- **tranche one's corpus of record is the UNCACHED one.** It is the referent, and
  a referent is not a by-product.
- **no tranche after tranche one runs cached until the comparison returns
  byte-identity — and this has a MECHANISM and a NAMED CHECKER, because revision 2
  stated it as a rule nobody was made responsible for.** The mechanism is §4.2's
  default: `--label-cache` is absent unless typed, so a tranche runs uncached
  unless somebody wrote the word. The checker is the run log: **the `cmp -s`
  exit line and its timestamp appear in the run log BEFORE the first tranche
  block whose command carries `--label-cache`**, and `wp21_prereg.md` §5 records
  the flag verbatim in every tranche's command block so a successor can read
  which tranches were cached without believing a sentence. The closure states who
  read it. A tranche block carrying the flag with no `cmp -s` line above it is a
  VOID tranche under `wp21_prereg.md` §4's void rule.
- **the wave schedule is NOT registered here and is derived in the run log once
  lever A fixes N.** A schedule constrains no conclusion; the two rules above do.
  The closure reports the realised wall against both bounds — the registered
  uncached 7.15 h and the cached figure §5 derives — rather than against whichever
  one flatters the lever.

**A SINGLE DIFFERING BYTE VOIDS LEVER B**, and the cache is not repaired and
re-compared: a cache whose first byte-identity run failed is a cache whose
correctness is being fitted to the check.

The registered **consequence of disagreement** is stated here as
`docs/process.md` requires: **lever B is abandoned and the sweep runs uncached.**

**AND THE DIAGNOSIS IS NOT REGISTERED IN ADVANCE, WHICH REVISION 2 GOT BACKWARDS.**
It said a disagreement *"is recorded as a finding about `newgame`'s isolation …
a defect in the ENGINE and not in the cache"*. That inference is invalid: §2's
argument establishes that the KEY is sound, not that a brand-new memo stores,
looks up and returns the right entry. A byte difference is at least as likely to
be the cache — a wrong entry returned, records emitted out of order, an entry
mutated after insertion, a hit that skips the `no_tab` guard — as `newgame`,
which four determinism-gate seats exercise on every CI run. **Writing the verdict
before the evidence, pointing away from the new code, is the after-the-numbers
move inverted.**

**WHAT IS REGISTERED INSTEAD**: the two candidate causes — the cache
implementation and `newgame`'s isolation — are **separated before either is
named**, by re-running the UNCACHED pass twice over the same report and comparing.
If the two uncached runs differ, the finding is the engine's; if they agree, it is
the cache's. The pilot shows that costs one 11-minute run and that the uncached
pass is self-identical today (`capture-determinism exit=0`, two identical sha256
lines in `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`).

### 4.5 The decision rule

Lever B is taken **only if** §4.1's ratio is at least 1.5 **and** §4.4 returns
byte-identity on both reports **and** the cached capture's own measured
seconds-per-distinct-label is within 5% of the uncached seconds-per-label — the
last one excluding a cache whose bookkeeping ate its own saving.

**REVISION 2: THE FIRST CONJUNCT IS DISCHARGED AND THE OTHER TWO ARE NOT, AND R1
DOES NOT DISCHARGE THEM.** §4.1's ratio is **2.1383**, above the registered 1.5.
The architect's approval settles *whether the cache is built*; it cannot settle
*whether the cache is right*, which is what §4.4 and the 5% clause are for. **A
failure of either still abandons the lever**, per §4.4's own consequence, and
D-576 says so in its own words: the cache may not be repaired into passing.

---

## 5. THE COST OF THIS STUDY, ON ITS OWN FACE

`docs/process.md`: a pre-registration states what its governed run costs.

| part | ESTIMATED |
|---|---|
| the one play pass over openings `0..2` | ~5 s |
| lever A, 5 settings x 3 reps, **152 records MEASURED** per process, ~135 s at N=1 and rising with N | **~1.0 h**, ESTIMATED at contention 1.0-2.0 |
| lever B §4.1's count | seconds — it reads a report |
| lever B §4.4's two byte-identity pairs | the pilot pair is ~11 min + ~5 min. **The tranche-sized pair now costs the CACHED capture alone (~1.43 h ESTIMATED)**, because revision 2 makes its referent tranche one's own uncached capture — work the sweep does anyway — rather than a capture taken for the check |

**REVISION 2's "~171 s per process" WAS A UNITS ERROR OVER A COUNT THAT WAS
ITSELF WRONG, AND THE RIGHT NUMBER WAS A FILE AWAY** (D-291). 171 is not a time:
it is `3 x 57.0769`, an estimated RECORD count. The exact count is in the pilot's
own committed corpus — **152 records** for openings `0..2`, MEASURED
(`artifacts/arc3_leverB_41_count_v3.txt`) — and at the pilot's MEASURED
0.8854 s/label that is **~135 s** at N=1. The aggregate is re-derived from it:
`5 settings x 3 reps x 135 s` = 2 025 s of N=1-equivalent work, and at contention
1.0-2.0 across the field **~1.0 h ESTIMATED**.

**AGAINST WHAT IT COULD SAVE, AND THE APPLICABLE NUMBER IS COMPUTED HERE RATHER
THAN LEFT AS PROSE.** Revision 2 stated the counterfactual saving to four figures
and the applicable one as *"a fraction of a wave"*, which is the shape this
project calls a claim asserted where it was convenient. Both are ESTIMATED by
multiplying the pilot's MEASURED per-unit rates, the same arithmetic
`wp21_prereg.md` §3 uses, and they carry the same limits:

```
COUNTERFACTUAL (every tranche cached, no verification cost)
  saving 3.26 h of the 7.15 h eight-way wall; serial capture 48.95 h -> 22.89 h

APPLICABLE ON THIS SWEEP, at the incumbent N = 8 and §4.4's rule that no
tranche runs cached before the comparison returns:
  wave 1, uncached   12 869 s          (registered; 13 928 s under wp21_prereg
                                        revision 4's replay correction)
  wave 2, cached      7 004 s
  realised           19 873 s = 5.52 h   against 25 738 s = 7.15 h uncached
  ESTIMATED SAVING                        1.63 h
  against §5's own registered costs: 1.43 h (tranche one's cached re-capture,
  on the critical path by construction) + ~1.0 h (lever A) + ~16 min (the pilot
  pair)
```

**SO ON THIS SWEEP THE LEVER IS ROUGHLY A WASH ON ITS OWN AND NET NEGATIVE ONCE
THE STUDY THAT JUSTIFIES IT IS COUNTED. THAT IS THE TRADE, STATED BEFORE THE
RUN.** Revision 2 answered it with *"the cache's value is mostly in the sweeps
after this one"*, and **there is no sweep after this one to point at**:
`docs/book_v2_ledger.md` shows `0..12` consumed, `13..3499` taken here, and
`3500..4499` reserved and never labelled. That appeal is withdrawn. **WHAT THE
LEVER BUYS ON THIS SWEEP IS A VERIFIED CAPABILITY AND A SMALLER SECOND WAVE**,
and the decision to take it is the architect's (D-576/R1), taken against this
number rather than against the counterfactual.

**AND THE HONEST FIGURE FOR THIS SWEEP IS SMALLER THAN THAT, WHICH IS SAID HERE
RATHER THAN DISCOVERED AT CLOSURE.** §4.4 forbids a cached tranche before the
comparison returns, so tranche one and everything running beside it in wave one
are uncached whatever the cache is worth. At the registered N = 8 that is half
the sweep, and the ESTIMATED saving falls to a fraction of a wave. **THE CACHE'S
VALUE IS MOSTLY IN THE SWEEPS AFTER THIS ONE**, where §4.4 has already returned
and every tranche runs cached; on this sweep it buys a verified capability and a
smaller second wave. That is the trade, stated before the run.

---

## 6. WHAT THIS STUDY DOES NOT DO

1. **It does not parallelise a capture internally.** `capture::run` walks one
   channel by construction, and N concurrent tranches already give the box N-way
   parallelism — an internal split would buy the same throughput and add a
   deterministic-merge obligation for nothing.
2. **It does not touch the two seats.** The duplication is structural (D-558(2));
   removing it would mean a capturable shape the arena refuses.
3. **It makes no strength claim and produces no corpus.** Every artifact it
   writes is scratch and is deleted; nothing it produces may enter a corpus.

---

## 7. THE COMMANDS, AND THE DRY RUN THAT PROVES THEY ARE THE BINARY'S

`docs/process.md`'s dry-run discipline: the literal commands a registration
governs are exercised before the review passes. **Revision 2 carried three
parenthetical flag fragments and the word "dry run" nowhere, and one of the
fragments was refused by the shipped binary.** The commands are here, in full,
and §7.1 records the run that took them.

```
# pass 1 — the one play pass, openings 0..2 into ONE report
arena --config <the pilot config with openings_take = 3> --out <SCRATCH>/report.txt

# lever A — one of the N concurrent processes at each setting
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/cap-N<n>-rep<r>-p<i>.txt \
      --label-nodes 400000

# lever B §4.4 — the pilot pair
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/uncached.txt --label-nodes 400000
arena --capture <SCRATCH>/report.txt --out <SCRATCH>/cached.txt   --label-nodes 400000 \
      --label-cache
cmp -s <SCRATCH>/uncached.txt <SCRATCH>/cached.txt

# §4.1's count, and its second instrument
python3 tools/label_cache_count.py --capture <a capture file>
/usr/bin/grep -v '^#' <a corpus file> | cut -f3 | LC_ALL=C sort -u | wc -l
```

**`<SCRATCH>` IS ON `/home` AND NEVER ON `/tmp`** — CLAUDE.md's Environment
section; this machine's `/tmp` is a 24 GiB RAM-backed tmpfs. Every artifact this
study writes is scratch and is deleted (§6.3).

### 7.1 THE DRY RUN — what it exercises, and what a failure means

Input of the same KIND and never the registered workload: **openings `0..0`**,
one opening, two games, at `--label-nodes 2000` rather than 400 000, which makes
the whole thing seconds rather than minutes.

1. every command above is accepted by the shipped binary and exits 0 — which is
   the limb revision 2 failed without knowing;
2. `--label-cache` together with `--census` is REFUSED by name, before any game,
   leaving no output file;
3. the cached and uncached captures of the one report are byte-identical under
   `cmp -s` — at a toy budget, so this is a shakedown and NOT §4.4's criterion,
   which is registered at the sweep's own seat and budget;
4. `tools/label_cache_count.py` reports a hit rate above zero on that capture,
   so the cache was actually exercised rather than merely present.

**A DRY-RUN FAILURE STOPS THE STUDY AND IS REPORTED AS A FINDING**; it is not
worked around by changing a command until something runs. Limbs 2 and 3 cannot be
taken until the cache exists, so the dry run is taken in two parts: limb 1 now,
against the shipped binary, and limbs 2-4 when the cache package lands. **THE
REVIEW THAT GOVERNS THE RUN IS TAKEN AFTER BOTH PARTS ARE RECORDED.**

---

## 8. THE INSTRUMENTS, WITH THEIR REVISIONS AND DIGESTS

`docs/process.md`, *Instrument governing revision*: an artefact that produces a
registered number — a `tools/` script, a scratchpad harness, or a command block
the document prints — is named here WITH ITS REVISION, and living in `tools/` is
not what makes the rule apply.

| instrument | what it produces | revision / digest |
|---|---|---|
| `tools/label_cache_count.py` | §4.1's counts and the multiplicity structure | committed at the arc III §1 head; sha256 in `artifacts/arc3_leverB_41_count_v3.txt` |
| §4.1.1's `sort -u` pipeline | the second instrument's distinct count | the command IS its revision; printed in §7 |
| `arena` | every capture in §3 and §4.4 | the sweep's own binary digest, `wp21_prereg.md` §8 |
| `cmp -s` | §4.4's byte-identity verdict | POSIX, no revision |
| the receipts | `artifacts/arc3_leverB_41_count_v3.txt` | sha-anchored in `docs/experiments/arc3_ledger.md` §1, because `artifacts/` is gitignored (hard rule 8) |
