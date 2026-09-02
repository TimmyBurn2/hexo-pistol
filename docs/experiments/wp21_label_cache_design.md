# WP-2.1 lever B — the label cache. DESIGN, revision 4.

**REVISION 4, AFTER ROUND 3 RETURNED FAIL** — 4 BLOCKING, 8 MAJOR, 4 minor
(`wp21_label_cache_design_rev3_REVIEW.md`); round 3 of five under D-585. **TWO OF
THE FOUR CHANGE THE PACKAGE RATHER THAN THE PROSE.** **X2 IS DELETED**: its
condition is unreachable, so it was never a guard (§3). **X3 GAINS THE ENGINE ITS
TEST NEEDS**: no stub behaviour in the tree can write an unsolicited line, so the
guard this package ADDS had a registered mutant that survived the entire suite —
the sharpest finding any of the three rounds produced (§1 row 9, §6 T4). The
other two are this document contradicting itself in sections a fix did not
reach: §9 still carried *"42 searches a tranche"* seventy lines below §1.1's
correction of it, and the header still said the design was not on the citation
gate's list after it had joined.

**REVISION 3, AFTER ROUND 2 RETURNED FAIL** — 4 BLOCKING, 7 MAJOR, 4 minor
(`wp21_label_cache_design_rev2_REVIEW.md`), with round 1's 22 findings dispositioned
**7 CLOSED, 6 PARTIAL, 9 OPEN**. Revision 2's header claimed every finding was
disposed of at the section that owns it; `git diff 239f21f fde1497` shows **§4 and
§5 have no hunks at all**, which is that claim refuted by the diff. Four things
move here and three of them are the ones the round-2 review found were never
touched: **the gate citation in §2's body** (revision 2 fixed the ONE LINE and left
the body byte-identical), **§6's T2**, **§1's change list**, and **§5's mutation
set**.

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
name, restated in the design that cites it, and revision 2 fixed the ONE LINE
while leaving §2's body byte-identical. Gate 9 is `tools/ci.sh:109-110`.

**AND THE CLAIM THAT THE CITATION GATE WOULD HAVE CAUGHT THEM IS FALSE, WHICH
D-584 RECORDS.** `tools/design_citation_check.py` refuses a path the tree does not
hold and a line past end-of-file; `tools/ci.sh:104-105` is **in range** in a
205-line file, so the gate returns exit 0 on it. **This document has been on
`tools/governing_citation_check.sh`'s list since `fde1497`** and the wrong
citation was green there for two revisions. What the gate buys is ROT — a path or
line that moved — and nothing else; what catches a wrong-but-in-range citation is
a reviewer re-deriving it, which is what happened three times.

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
| 1a | `crates/pistol-arena/src/capture.rs`, in `run`'s prefix loop | **the LOOKUP**: before `ask`, `position` is looked up in the map; a hit takes the stored pair and makes no ask. **Revision 2 dropped this row and revision 1 had it**, while §5 went on mutating it as a site |
| 1b | same loop, after a miss's `ask` | **the INSERT**: the POST-`normalise` pair is stored under `position` |
| 2 | `crates/pistol-arena/src/capture.rs` | the stray-line guard hoisted out of `ask` so a hit pays it too (§3, X3) |
| 3 | `crates/pistol-arena/src/capture.rs` | the two coarser-fold counters, computed on a MISS from the replayed prefix (§1.1) |
| 4 | `crates/pistol-arena/src/capture.rs` | **`CaptureCounts { asks, records, hits, key_pos_collisions, key_full_collisions }`**, returned beside the records. `asks` is INCREMENTED AT THE CALL TO `ask` AND NOWHERE ELSE — **not derived from the records**, because deriving it the way `tools/label_cache_count.py:167,175,179-181` derives the same quantity would make it a function of the capture file, which is identical cached or not, and T2 would go green on a dead cache. Revision 3 made the whole suite hinge on this number and registered neither a field nor an increment site |
| 5 | `crates/pistol-arena/src/passes.rs` | `capture` threads the mode in and **prints the counts** — `:82-96` is where every line this pass writes is written |
| 6 | `crates/pistol-arena/src/bin/arena.rs` | the `--label-cache` word, and X1's refusal arms |
| 7 | `crates/pistol-arena/src/usage.rs` | the word, the default, and what it does |
| 8 | `crates/pistol-arena/tests/label_cache_tests.rs` | the tests §6 registers |
| 9 | `crates/pistol-arena/src/bin/stub_engine.rs` | **a `Behave` variant that writes an UNSOLICITED line before it is asked** — the engine X3's test needs and the tree does not have. `git grep -n unsolicited -- crates` returns four hits and **none in any `tests/`**, and all eighteen existing variants write `bestmove` last, so **X3's mutant survives the whole suite today**. A test-only binary gains a test-only behaviour |
| 10 | `tools/cold_label_check.py` | `--partition` **landed at `f1acc57`**; what is still owed is `wp21_prereg.md` revision 4's **ten-sampled-record floor** — a class with fewer than ten sampled records on a tranche that should have both is a VOID — with its own mutant and its own test |

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
the book's own shape at `k = 2`, **per tranche and not once**:

```
tranche  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16   sum
merges  42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46   792
```

**EACH TRANCHE HAS ITS OWN FLOOR AND THE RUN LOG CARRIES THAT TRANCHE'S**, from
the same instrument, taken before the sweep. A closure reading a tranche's count
against 42 would read fourteen tranches as a discovery of between 2 and 18
collisions that are the book.

**REVISIONS 1 AND 2 SAID "at least 42 per tranche" AND 42 IS TRANCHE ONE'S** —
the minimum across the sixteen, not the value at any other. D-583 carries the
same error and is corrected by its own successor line.

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
`tools/determinism.sh` is **CI gate 9 of 20** (`tools/ci.sh:109-110`; the total
is `:21`'s `readonly GATE_TOTAL=20`, and `:104-105` is the comment above the gate,
which revisions 1 and 2 both cited as if it were the gate) and
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
| ~~X2~~ | **DELETED at revision 4.** It refused a hit whose cached `bestmove` or `totals` was empty | **THE CONDITION IS UNREACHABLE.** `ask` returns `Ok((totals, line))` only where `line` starts with `bestmove ` (`capture.rs:266`, `:273`) and `totals` is a line `classify` matched through `exchange::totals_of` (`:276`) — so no engine, honest or hostile, can cause an empty insert. **A refusal that cannot fire is not a guard** (D-572's own words about `manifest_row`), and revisions 1 to 3 registered a mutant for it whose kill criterion named a test §6 never had |
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

D-568's standing law: a mutation set is specified against call sites enumerated by
a `git grep` receipt recorded in the mutation document, never against prose.
**THE SITES DO NOT EXIST YET, so the receipt is owed AT IMPL and this section is
not it.** **REVISION 2 LEFT THIS SECTION UNTOUCHED WHILE ADDING A GUARD (X3) AND
A SEAM (the mode) THAT IT DOES NOT COVER**, which is D-553's own class: a guard
with no registered mutant is a guard whose failure nothing would notice.

What IMPL owes, one mutant per defect class per site:

| site (§1 row) | mutant | dies at |
|---|---|---|
| 1a, the lookup | REMOVED — every ask made | T2: `asks == records` on a cached run |
| 1a, the lookup | INVERTED — a miss treated as a hit | T1: the first record's bytes |
| 1a, the lookup | keyed on something other than the asked `position` | T1 on a report holding a transposition |
| 1b, the insert | REMOVED | T2 |
| 1b, the insert | stores the RAW totals rather than the post-`normalise` pair | T1: ` nps <n> time <n>` in a hit's record |
| 1, the mode | threaded but never consulted — `On` behaves as `Off` | T2 |
| 4, the count | reports `records` rather than the asks actually made | **T2's CACHED arm.** Revision 3 named the UNCACHED arm, where `asks == records` is what an honest count also reports, so the mutant survived exactly where its criterion pointed |
| 4, the count | derived from the record list rather than incremented at the ask | T2's cached arm, for the same reason: a count that is a function of the capture file cannot differ between the two runs |
| X1 | the arm REMOVED | **the refusal must NAME BOTH WORDS**: deleting the arm drops the combination through `bin/arena.rs:79-85`'s catch-all, which still refuses and still exits 2, so a test asserting only "refused" cannot tell them apart |
| X3 | the hoisted guard REMOVED | **T4, and only with row 9's stub behaviour**: the stray must arrive at a prefix the cached run treats as a HIT, because that is the only prefix where the two passes differ. Without such an engine the mutant survives the whole suite, which is what round 3 found |
| the counters | REMOVED, and INVERTED | T5 |
| 10, the cold check's ten-record floor | the threshold removed, and set to zero | its own test: a capture whose HIT class holds nine records is a VOID and not a pass |

## 6. THE TESTS, ENUMERATED HERE BECAUSE REVISION 1's CHANGE LIST HAD NO TEST ROW

`crates/pistol-arena/tests/label_cache_tests.rs`, driving the shipped `arena`:

| # | what it pins |
|---|---|
| T1 | a cached capture of a report is **byte-identical** to an uncached one — §4.4's criterion in miniature, at a toy budget, so the package carries its own shakedown |
| T2 | the cached run makes **fewer engine asks** than the uncached one, **read off the count `arena --capture` prints** (§1 row 4), and the uncached run's asks equal its record count. **REVISION 2 SAID "measured from the stub's own count" AND `stub_engine.rs` HAS NO COUNTER** — a test specified against an instrument that does not exist |
| T3 | `--label-cache` with `--census` is **refused naming both words**, before any file is claimed |
| T4 | a stray engine line **arriving at a prefix the cached run treats as a HIT** is refused at the same prefix in both runs — X3's guard, which is the one a hit would otherwise skip. **It needs §1 row 9's stub behaviour**: no engine in the tree writes an unsolicited line, so without it this row cannot be written and X3's mutant survives everything |
| T5 | the counters report zero on a report whose prefixes hold no transposition and no mirror, and non-zero on a fixture built to hold one of each |
| T7 | `--label-cache` given twice is refused, and `--label-cache` in any position but last is refused — the parser is a positional literal match, so every accepted spelling is an arm somebody wrote |
| T6 | the record order and every field is unchanged — the same assertion T1 makes, taken over a report with a forfeit and a rule-4 win, so the truncated-turn path is covered |

**T2 IS THE ONLY ROW A DEAD CACHE FAILS, AND THAT IS WHY ROUND 2 CALLED IT
BLOCKING.** A cache that is parsed and then dropped yields the uncached bytes, so
**T1 and T6 pass**; T3 is a CLI arm; T4's guard is unconditional; T5's counters
are unconditional. **The registration's own fallback cannot fail either**:
`tools/label_cache_count.py` reads the capture file and returns the same number
whether the run was cached or not, because the file is identical by construction —
which is the criterion's whole point and makes it useless as an existence check.

**SO THE ASK COUNT IS A REGISTERED OUTPUT AND NOT A DIAGNOSTIC.** `arena --capture`
prints `asks <n> records <n>` with the hit rate; T2 asserts `asks < records`
cached and `asks == records` uncached. A cache that is dropped really does ask
every time, so it reports `asks == records` and fails. **The only way to pass T2
without a working cache is to lie about the count**, which is a different defect
with its own registered mutant (§5).

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
| ~~this document joins the citation gate's list~~ | **DISCHARGED at `fde1497`** — and the reason given for it was wrong: the gate would NOT have refused revision 1's citations, because `tools/ci.sh:104-105` is in range (D-584). The document was on the list for two revisions with the wrong citation green |
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
   answer (D-576), and what the folds are worth is MEASURED per tranche at
   **42 to 60 searches, 792 over the sweep** — under one percent of a tranche's
   misses. **D-583 gave 42 as the per-tranche figure and 42 is TRANCHE ONE'S**;
   **D-584 corrects it**, and §1.1 carries the sixteen. Revision 3 corrected §1.1
   and left this sentence, seventy lines below, saying the superseded number.
3. **It changes no criterion of `wp21_prereg.md` §4.** T-A1 and T-A2 are that
   document's, and this package supplies the derivation and the instrument they
   need — the instrument's `--partition` half landed at `f1acc57`; its
   ten-sampled-record floor is still owed and is §1 row 8.
4. **It settles nothing about D-562(2)**, and revision 1 claimed it did.
