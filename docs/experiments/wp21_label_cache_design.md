# WP-2.1 lever B — the label cache. DESIGN, revision 2.

**REVISION 2, AFTER A FRESH-CONTEXT REVIEW-design THAT RETURNED FAIL** — 6
BLOCKING, 12 MAJOR, 4 minor (`wp21_label_cache_design_REVIEW.md`). Every finding
is disposed of at the section that owns it, and **one is rejected with its
reproducer** (§7). The three that changed the mechanism rather than the prose:
**the memo is no longer a parameter** (§2.0), **it stores post-`normalise` bytes
and the design says so** (§2.1), and **the stray-line guard is hoisted so a hit
still pays it** (§3, X3).

> **ONE LINE.** `arena --capture` asks the engine at every asked prefix of every
> game, and the pilot MEASURED **742 asks over 347 distinct questions**. This
> package memoises the answer to a question the pass has already asked, keyed on
> **the `position` line's exact bytes**, so the cache's soundness is the
> determinism law **CI gate 9 of 20** already enforces and is not a new claim
> about the search. It drops no record, changes no field, and is required to be
> BYTE-IDENTICAL to the pass that exists today.

**REVISION 1's ONE LINE SAID "CI gate 6", WHICH IS CONFIG VALIDATION**
(`tools/ci.sh:97-98`) — the exact error the governing registration corrects by
name, restated in the design that cites it. Gate 9 is `:109-110`; `:104-105` is
the comment above it, which revision 1 also cited wrongly. **The design is not on
`tools/governing_citation_check.sh`'s list**, and these three would have been
caught if it were; adding it is §8's first obligation.

**GOVERNING REGISTRATION**: `docs/experiments/wp21_throughput_prereg.md`
**revision 3** — §2 (the key and why), §2.0 (the memo's lifetime invariant),
§2.1 (the fold that was declined), §4.2 (what is implemented), §4.4 (the
criterion), §4.5 (the decision rule) — together with
`docs/experiments/wp21_prereg.md` **revision 4** §1, §4's T-A1/T-A2 and §5.
**GOVERNING RULINGS**: D-576, D-581, D-583. **Revision 1 of this document named
revision 2 of the registration, which failed its own review and did not contain
§2.0 or §2.1 at all** — the two sections this design most depends on.
**THIS DOCUMENT DESIGNS ONLY WHAT THE REGISTRATION NAMES.**

---

## 1. WHAT CHANGES, AND THE COMPLETE LIST IS SHORT

| # | site | change |
|---|---|---|
| 1 | `crates/pistol-arena/src/capture.rs` | `run` takes a `LabelCache` **MODE** — a two-state enum, not a map — and **builds the map itself**, beside `label_go_line` at `:333` |
| 2 | `crates/pistol-arena/src/capture.rs` | the stray-line guard hoisted out of `ask` so a hit pays it too (§3, X3) |
| 3 | `crates/pistol-arena/src/capture.rs` | the two coarser-fold counters, and a `CaptureCounts` the pass returns beside its records |
| 4 | `crates/pistol-arena/src/passes.rs` | `capture` threads the mode in and **prints the counts** — `:82-96` is where every line this pass writes is written, and revision 1 registered no print site at all |
| 5 | `crates/pistol-arena/src/bin/arena.rs` | the `--label-cache` word, and X1's refusal arms |
| 6 | `crates/pistol-arena/src/usage.rs` | the word, the default, and what it does |
| 7 | `crates/pistol-arena/tests/capture_tests.rs` and a new `label_cache_tests.rs` | the tests §6 registers — **revision 1's change list had no test row, which is how a design comes to owe tests nobody enumerated** |
| 8 | `tools/cold_label_check.py` | **already landed** at `f1acc57`. What is still owed there is `wp21_prereg.md` revision 4's **ten-sampled-record floor**, which neither the tree nor revision 1's list carries |

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
most **79** placements (`turn_cap = 40`, so a prefix is at most 79 stones) and
twelve symmetry images against a search of ~885 ms, paid on the 47% of asks that
miss. ESTIMATED at well under 0.1% of a capture; it is MEASURED on the dry run
and the number goes in the run log.

**AND THE `key_full` COUNTER'S ANSWER IS ALREADY PARTLY KNOWN, WHICH THE CLOSURE
MUST NOT READ AS A DISCOVERY.** `artifacts/arc3_opening_prefix_fold.txt` MEASURES
the book's own shape: within tranche one's 218 openings the symmetry fold takes
213 `k = 2` classes to 171, so **at least 42 `key_full` collisions per tranche are
the OPENING BOOK's structure and not a search-space transposition**. The counter
reports its total and the run log records that floor beside it; a closure that
reported 42 as a finding would be reporting the book.

**AND D-581's FLIP CLAUSE HAS TWO CONJUNCTS, NOT ONE.** The selection flips only
if the count is materially above that floor **AND** a seat pins `countermove`
`false` as a rule rather than as a value. Revision 1 carried the first and
dropped the second.

**THE CLAIM THAT THIS "SETTLES D-562(2)" IS WITHDRAWN.** The registration says in
its own §2 that the three-key question is *"not merely untouched but unreachable
from here"*, because the cache only ever compares a key with itself. The counters
measure how much a coarser key WOULD fold; they adjudicate no disagreement
between keys, which is what D-562(2) is about.

**NOTHING ELSE.** No format version moves (D-572), no column is added, no
manifest field changes, and `capture_sha256` is untouched — the cached pass and
the uncached pass are the same instrument answering the same questions, which is
exactly what §4.4 requires them to prove.

---

## 2.0 THE MEMO IS NOT A PARAMETER, AND THAT IS THE REVIEW'S FIRST BLOCKING FINDING

The registration registers the memo's lifetime as an invariant: *"constructed
inside `capture::run` and dropped with it; never a field of a longer-lived value,
and never shared between two invocations."* **Revision 1 made `run` take a
`&mut LabelCache` threaded from the binary**, which turns that invariant into a
property of every caller — and `capture::run` is `pub` (`capture.rs:326`), so
"every caller" includes ones that do not exist yet. **An invariant a REVIEW-impl
cannot check by reading the function is not an invariant.**

**THE SHAPE INSTEAD**: `run` takes a **MODE**

```
pub enum LabelCache { Off, On }
```

and builds its own `BTreeMap<String, (String, String)>` locally, beside
`label_go_line` at `:333` — inside the closure, dropped when `run` returns.
**The invariant becomes structural**: there is no way to hand a memo in, so there
is no way to share one. The map's type appears in no signature.

## 2.1 WHICH SIDE OF `normalise` THE MEMO SITS ON, AND REVISION 1 DID NOT SAY

`run` builds each record as `totals: normalise(&totals)?` (`capture.rs:355-358`),
and `capture::normalise` strips ` nps <n> time <n>` — the two fields gate 9 itself
excludes because they measure the machine and not the search.

**THE MEMO STORES THE POST-`normalise` PAIR: exactly the two strings the record
carries.** A hit therefore reproduces the record's bytes by construction rather
than by argument.

**AND THE ALTERNATIVE IS NOT A STYLE CHOICE.** A memo holding the RAW totals
would write ` nps <n> time <n>` into the 53% of records that are hits — the same
machine-dependent numbers the pass exists to strip — and §4.4's byte-identity
would fail on every cached capture, on a machine-dependent difference that says
nothing about the cache. **Worse, it might not fail**: `normalise` is idempotent
on an already-stripped line only in the sense that it REFUSES it (it errors when
no `nps` field is present), so the raw-side memo fails loudly on a second
normalise — which is the good case. The bad case is a memo storing raw and
normalising on the way out, where the answer depends on where the code happens to
call it. **The design fixes the side rather than leaving it to the implementer.**

## 2. THE KEY, AND WHY IT IS THE QUESTION RATHER THAN A DIGEST OF IT

The key is the `position` line `capture::position_line` built for this prefix —
the same `String` the record then carries in its `position` field. A hit returns
the `(totals, bestmove)` an identical `position`, after an identical `newgame`,
under an identical `go`, on the same process, already produced.

**AND THE KEY IS NOT `key_pos`.** Revision 1 called the transposition key *"the
replayed position's sorted `(cell, player)` list"* and pointed at `key_pos`. **The
tree's `key_pos` is `GameState::key`, a 128-bit zobrist** (`labels.rs:202`), not a
stone list; the two fold the same equivalence and are not the same object, and a
counter built to the wrong one would be a hash where the design promises an exact
comparison. **The counters compare stone lists and canonical forms, never a
digest.**

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
(the search reads `played()` at `heuristics.rs:89` under `params.ordering.any()`
— under ALL three gates, not two — but the only read that can move a CHOICE is
`last_stone` at `:155` under `countermove` alone, D-581).

**THE MAP IS ORDERED, NOT HASHED.** `BTreeMap` owes nothing to hard rule 4's
hash-order clause because it has no hasher. The comparison is a string compare
against a search of ~885 ms.

---

## 3. THE REFUSALS, EACH NAMED AND EACH WITH THE DEFECT IT EXCLUDES

| # | refusal | the defect it excludes |
|---|---|---|
| **X1** | `--label-cache` together with `--census` is refused **before any game**, naming both words | a cache hit performs no search and so emits no census row: a cached census capture writes fewer rows than positions asked, and nothing downstream could tell that from a quiet search |
| **X2** | a hit whose cached `bestmove` or `totals` is empty is refused naming the position | an insert of a value the ask never produced; a guard over the map's own invariant rather than over the engine |
| **X3** | **the stray-line guard runs on every prefix, hit or miss** — `channel.unsolicited()` is HOISTED out of `ask` into `run`'s loop | a guard that silently stops running on 53% of prefixes. `capture.rs:241-246` refuses an engine that spoke before it was asked; skipping the ask skips the check, so a stray line would be attributed to a LATER prefix or missed at the end of a game. **The uncached and cached passes must refuse the same input at the same place**, which is a stronger requirement than byte-identity on well-behaved input |

**X1 IS THE ONE THAT MATTERS AND IT IS A CONFIG-TIME REFUSAL**, not a runtime
one: it fires from the argument parse, before `--out` is claimed, so the run
leaves no file behind (D-200's shape). **VERIFIED, not assumed**: `bin/arena.rs`'s
whole `match words` runs at `:39-86` and `outpath::claim` is at `:89`.

**AND X1's MUTANT NEEDS A KILL CRITERION REVISION 1 DID NOT STATE.** The parser
is a positional literal match, so DELETING X1's arm does not make the combination
legal — it falls through to the catch-all at `:79-85`, which still refuses and
still exits 2. **A test asserting only "refused" cannot tell the two apart.** The
registered kill criterion is therefore that the refusal **names both words**,
`--label-cache` and `--census`, which the catch-all's usage text does not do.

**THE SECOND CENSUS GUARD INSIDE `ask` IS MOOT AND IS SAID SO RATHER THAN LEFT
UNSAID.** `capture.rs:288-295` refuses a census row on an ask that did not request
one; a hit makes no ask and receives no lines. Under X1 a cached run is never a
census run, so there is no census row for a hit to miss. **X3's guard is not moot
in that way**, which is why it is hoisted and this one is not.

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

## 6. THE TESTS, ENUMERATED HERE BECAUSE REVISION 1's CHANGE LIST HAD NO TEST ROW

`crates/pistol-arena/tests/label_cache_tests.rs`, driving the shipped `arena`:

| # | what it pins |
|---|---|
| T1 | a cached capture of a report is **byte-identical** to an uncached one — §4.4's criterion in miniature, at a toy budget, so the package carries its own shakedown |
| T2 | the cached run makes **fewer engine asks** than the uncached one, measured from the stub's own count, so the cache is exercised rather than merely present |
| T3 | `--label-cache` with `--census` is **refused naming both words**, before any file is claimed |
| T4 | a stray engine line is refused **at the same prefix** in both runs — X3's guard, which is the one a hit would otherwise skip |
| T5 | the counters report zero on a report whose prefixes hold no transposition and no mirror, and non-zero on a fixture built to hold one of each |
| T6 | the record order and every field but nothing else is unchanged — the same assertion T1 makes, taken over a report with a forfeit and a rule-4 win, so the truncated-turn path is covered |

## 7. THE ONE REVIEW FINDING REJECTED, WITH THE REPRODUCER ATTEMPTED

**BLOCKING 3, in part: *"`no_tab` appears zero times in the design, though the
registration names 'a hit that skips the `no_tab` guard' as a candidate cache
defect."*** The concern is REJECTED for the mechanism it names; the normalise half
of the same finding is ACCEPTED in full and is §2.1.

**THE REPRODUCER IS THREE LINES OF THE FILE THE FINDING IS ABOUT.**
`capture.rs:355-360`:

```
totals: normalise(&totals)?,
bestmove,
};
no_tab(&record)?;
```

`no_tab` runs in `run`, on the CONSTRUCTED RECORD, **after** the record is built
and **outside** `ask`. A hit skips `ask`; it does not skip record construction,
and it therefore does not skip `no_tab`. **The guard cannot be skipped by this
design without deleting a line that has nothing to do with the cache** — which is
what its registered mutant is for. The design now says so rather than leaving a
reviewer to re-derive it.

## 8. WHAT THIS PACKAGE OWES

| obligation | discharged by |
|---|---|
| **this document joins `tools/governing_citation_check.sh`'s list** | the IMPL commit — revision 1 carried three wrong citations that the gate would have refused |
| REVIEW-design | **round 1 FAIL** (6B/12M/4m); this revision answers it and owes its own |
| IMPL | the change list of §1 |
| REVIEW-impl, fresh context, not the implementer | a subagent, against this document |
| RED-TEAM on the cache path | a subagent: a report with a forfeit, a rule-4 win, a transposition, a mirror, a stray line, a `--census` combination, and a second `--label-cache` word |
| the tools/ coverage rule | §6's suite drives the shipped `arena` |
| **a mutation set against call sites enumerated by a `git grep` receipt** (D-568) | the IMPL package; the sites do not exist yet and this line is not the receipt |
| an ADR line recording the strongest surviving attack | at closure |

## 9. WHAT THIS DESIGN DOES NOT DO

1. **It does not persist across processes.** A per-run map, dropped at exit — now
   structurally, since §2.0 gives the map no way out of `run`.
2. **It does not fold transpositions or symmetries.** Both cost a class of wrong
   answer (D-576), and what the folds are worth is MEASURED at 42 searches a
   tranche, 0.72% of its misses (D-583).
3. **It changes no criterion of `wp21_prereg.md` §4.** T-A1 and T-A2 are that
   document's, and this package supplies the derivation and the instrument they
   need — the instrument's `--partition` half landed at `f1acc57`; its
   ten-sampled-record floor is still owed and is §1 row 8.
4. **It settles nothing about D-562(2)**, and revision 1 claimed it did.
