# WP-2.1 lever B — the label cache. DESIGN, revision 1.

> **ONE LINE.** `arena --capture` asks the engine at every asked prefix of every
> game, and the pilot MEASURED **742 asks over 347 distinct questions**. This
> package memoises the answer to a question the pass has already asked, keyed on
> **the `position` line's exact bytes**, so the cache's soundness is the
> determinism law CI gate 6 already enforces and is not a new claim about the
> search. It drops no record, changes no field, and is required to be
> BYTE-IDENTICAL to the pass that exists today.

**GOVERNING REGISTRATION**: `docs/experiments/wp21_throughput_prereg.md`
revision 2, §2 (the key and why), §4.2 (what is implemented), §4.4 (the
criterion), §4.5 (the decision rule). **GOVERNING RULING**: D-576.
**THIS DOCUMENT DESIGNS ONLY WHAT §4.2 NAMES.**

---

## 1. WHAT CHANGES, AND THE COMPLETE LIST IS SHORT

| # | site | change |
|---|---|---|
| 1 | `crates/pistol-arena/src/capture.rs` | `run` takes a `&mut LabelCache`; before `ask`, a lookup on the `position` string; after `ask`, an insert |
| 2 | `crates/pistol-arena/src/capture.rs` | `LabelCache`, a two-state type: `Off`, or `On(BTreeMap<String, (String, String)>)` |
| 3 | `crates/pistol-arena/src/passes.rs` | `capture` threads the cache from the binary's argument |
| 4 | `crates/pistol-arena/src/bin/arena.rs` | one optional trailing word, `--label-cache` |
| 5 | `crates/pistol-arena/src/usage.rs` | the word, and what it does, in the usage text |
| 6 | `tools/cold_label_check.py` | `--partition hits\|misses\|all`, defaulting to nothing: the argument is required, and the summary line names the class |
| 7 | `crates/pistol-arena/src/capture.rs` | **the coarser-fold counters**: how many MISSES share a `key_pos` or a `key_full` with an earlier miss, printed with the hit rate |

### 1.1 THE TWO COUNTERS, AND WHY THEY ARE PART OF THE CACHE RATHER THAN A STUDY

`matrix_label_cache_key.md` §4's flip clause: the matrix chose the finest key on
a population 10^5 times smaller than the sweep, and revision 1's flip condition
could never be observed by the run it governed. **The remedy is two counters
inside the cache itself.**

On a MISS the memo already holds every earlier miss's `position`. The counters
additionally hold, for each miss, its transposition key (the replayed position's
sorted `(cell, player)` list) and its symmetry key (`canonical_form` over the
stones), and count how many misses collide on each. **They decide nothing** — the
cache still keys on the `position` line alone and a collision on a coarser key
changes no answer — so a bug in a counter cannot mislabel a record; it can only
misreport a number.

They are printed once per capture, beside the hit rate, and the run log carries
them per tranche. **That number is the coarser keys' yield at the scale that
matters**, it is what re-takes this matrix if it is materially above zero, and it
settles D-562(2)'s open three-key question in the same pass.

**THE COST IS ONE REPLAY PER MISS**, which the cache does not otherwise need — at
most eighty placements and twelve symmetry images against a search of ~885 ms,
paid on the 47% of asks that miss. ESTIMATED at well under 0.1% of a capture; it
is MEASURED on the dry run and the number goes in the run log.

**NOTHING ELSE.** No format version moves (D-572), no column is added, no
manifest field changes, and `capture_sha256` is untouched — the cached pass and
the uncached pass are the same instrument answering the same questions, which is
exactly what §4.4 requires them to prove.

---

## 2. THE KEY, AND WHY IT IS THE QUESTION RATHER THAN A DIGEST OF IT

The key is the `position` line `capture::position_line` built for this prefix —
the same `String` the record then carries in its `position` field. A hit returns
the `(totals, bestmove)` an identical `position`, after an identical `newgame`,
under an identical `go`, on the same process, already produced.

**SO THE SOUNDNESS OBLIGATION IS DISCHARGED BY A GATE THAT ALREADY RUNS.** Hard
rule 4: in instrument mode nothing nondeterministic may influence move choice.
`tools/determinism.sh` is **CI gate 9 of 19** (`tools/ci.sh:104-105`) and
compares two processes on the move, the node count, the score, the depth and the
whole PV — every field but `nps` and `time`, which is the same normalisation
`capture::normalise` applies, across **five** seats.

**AND THE GATE NEVER TAKES THE CACHE'S OWN SHAPE, WHICH IS SAID HERE BECAUSE IT IS
THE PACKAGE'S STRONGEST ATTACK.** Its `A vs B` limb runs one script in two
processes; its `C vs D` limb runs one-process-per-position against
all-positions-in-one-session — and in the session limb **each position is asked
once**. *"The same position, asked again, in the same process"* is exactly a cache
hit and exactly the shape gate 9 never takes. None of its five seats is
`configs/instrument_v0.toml` and neither of its budgets is `nodes 400000`.
**What closes it is §4.4's byte-identity run at the sweep's own seat and budget**,
which asks the same position twice in one process ~6 624 times per tranche and
compares every byte against a pass that never did. **No claim about what the search reads out of
`GameState` is made, and revision 1 of the registration made one that is false**
(the search reads `played()` under two of the three ordering gates:
`heuristics.rs:155`, `:89`).

**THE MAP IS ORDERED, NOT HASHED.** `BTreeMap` owes nothing to hard rule 4's
hash-order clause because it has no hasher. The comparison is a string compare
against a search of ~885 ms.

---

## 3. THE REFUSALS, EACH NAMED AND EACH WITH THE DEFECT IT EXCLUDES

| # | refusal | the defect it excludes |
|---|---|---|
| **X1** | `--label-cache` together with `--census` is refused **before any game**, naming both words | a cache hit performs no search and so emits no census row: a cached census capture writes fewer rows than positions asked, and nothing downstream could tell that from a quiet search |
| **X2** | a hit whose cached `bestmove` or `totals` is empty is refused naming the position | an insert of a value the ask never produced; a guard over the map's own invariant rather than over the engine |

**X1 IS THE ONE THAT MATTERS AND IT IS A CONFIG-TIME REFUSAL**, not a runtime
one: it fires from the argument parse, before `--out` is claimed, so the run
leaves no file behind (D-200's shape).

**THERE IS NO "CACHE DISAGREES" REFUSAL, AND THAT IS DELIBERATE.** A cache that
verified its own hits by re-asking would cost exactly what it saves. The
verification is §4.4's byte-identity against an uncached run, which is an
EXTERNAL referent; an internal self-check would be the vacuous-criterion class
this project has already paid for six times.

---

## 4. WHAT THE COLD CHECK GAINS, AND WHY IT IS A REQUIRED ARGUMENT

R1 amends `wp21_prereg.md` T-A: cache hits and cache misses are sampled
**separately** at the registered stride, both byte-equal in fresh processes. A
stride over an undifferentiated record list can sample two hundred misses and no
hit at all.

**THE PARTITION IS DERIVED FROM THE CAPTURE, NOT RECORDED IN IT.** Walk the
records in order; a record whose `position` field has not been seen before is a
MISS, every later record with that field is a HIT. `tools/cold_label_check.py`
gains `--partition`, and it is **required** rather than defaulted: hard rule 1's
spirit and this project's own experience — a default here would silently answer
about `all` while a criterion said `hits`.

**AND ON A HIT THE CHECK IS A CHECK OF THE MEMO, NOT OF THE KEY.** The hit's own
`position` is byte-identical to its miss's, so a fresh process asked at it must
return the same bytes the cache handed back — and if the memo returned the entry
belonging to some other position, the record carries that position's answer and
the fresh ask does not. **A cache that returned the wrong entry fails here at the
first sampled hit.** What T-A2 cannot falsify is the key's EQUIVALENCE, because
K1 has none; that is the point of choosing it, not a gap.

**AN EMPTY CLASS IS A VOID AND NOT A PASS**, registered at `wp21_prereg.md` §4:
fewer than ten sampled records in a class a tranche should have makes T-A a VOID.
A filter over an already-read list would otherwise print `0 of 0 … agree` and exit
0, which is the vacuous pass `docs/process.md` forbids.

---

## 5. THE MUTATION SET — SPECIFIED AGAINST CALL SITES ENUMERATED BY A `git grep` RECEIPT

D-568's standing law: a mutation set is specified against call sites enumerated
by a `git grep` receipt recorded in the mutation document, never against prose.
**THE SITES DO NOT EXIST YET, so the receipt is owed AT IMPL and this section is
not it.** What IMPL owes, per defect class per site:

- the lookup: removed (every ask made) — dies at the hit-count test;
- the lookup: inverted (a miss treated as a hit) — dies at the first record;
- the insert: removed — dies at the hit-count test;
- the insert: keyed on something other than the asked `position` — dies at a
  two-seat fixture holding a transposition, where a coarser key returns the
  other order's answer;
- X1: the refusal's condition negated — dies at its own test;
- each counter: removed, and inverted — dies at a fixture built to hold exactly
  one transposing pair and exactly one mirrored pair, where the expected counts
  are 1 and 1 rather than 0 and 0.

---

## 6. WHAT THIS DESIGN DOES NOT DO

1. **It does not persist across processes.** A per-run map, dropped at exit.
   A file-backed cache would be an artifact class with a format version, a digest
   and a manifest row (D-572), for a saving that lives inside one tranche.
2. **It does not fold transpositions or symmetries.** MEASURED at zero yield on
   the pilot corpus, and both folds cost a class of wrong answer (D-576).
3. **It changes no criterion of `wp21_prereg.md` §4** — T-A's amendment is R1's,
   registered there, and this package only supplies the derivation it needs.
