# DECISION-RED-TEAM (fresh context) — `docs/experiments/matrix_label_cache_key.md` revision 1

**NAMED REVISION.** `31315f8ffcc4bddb58744a74608caf74a94768f3` (`dev`).

**DOES IT STILL MATCH HEAD?** It did when this review began and it does not now.
HEAD moved to `0321cc1637ce479bcfb54fead098df91ec4167c3` DURING this review
(`fix(arc3): the counter's test suite lands rustfmt-clean…`). The move does not
reach the subject: `git diff 31315f8 HEAD -- docs/experiments/matrix_label_cache_key.md tools/label_cache_count.py`
is **empty**; the only changed file is
`crates/pistol-arena/tests/label_cache_count_tests.rs` (+61/−14, formatting).
The working tree additionally carries an uncommitted `docs/experiments/arc3_ledger.md`
(+25). **Everything below is judged against `31315f8`'s bytes and holds at
`0321cc1`.**

**CONSTRAINT HONOURED.** No `cargo` invocation of any kind. Every fact below
comes from `/usr/bin/grep`, `git grep`, `git show`, `git log`, `git diff`,
`sha256sum`, `python3` over text, or reading files.

**WHAT I READ.** `CLAUDE.md`; `docs/process.md`; `docs/decisions.md` (D-5, D-7,
D-8, D-137, D-291, D-423, D-424, D-540, D-562, D-570, D-576);
`docs/experiments/matrix_label_cache_key.md`; `docs/experiments/wp21_throughput_prereg.md`;
`docs/experiments/wp21_prereg.md`; `docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md`;
`docs/experiments/arc3_ledger.md` (committed and working-tree);
`crates/pistol-arena/src/{capture.rs,exchange.rs}`;
`crates/pistol-arena/tests/label_cache_count_tests.rs`;
`crates/pistol-search/src/{heuristics.rs,ordering.rs,staged.rs,candidates.rs,search.rs,pvs.rs,params.rs}`;
`crates/pistol-solver/src/{lib.rs,query.rs,table.rs}`;
`crates/pistol-eval/src/{eval.rs,handcrafted.rs,lib.rs}`;
`crates/pistol-engine/src/instance.rs`; `crates/pistol-core/src/turn.rs`;
`crates/pistol-cli/src/random_openings/mod.rs`; `configs/instrument_v0.toml`;
`tools/{label_cache_count.py,cold_label_check.py,determinism.sh,ci.sh}`;
`artifacts/arc3_leverB_41_count_v3.txt`; the pilot artifacts at
`/home/tom/pistol-runs/wp20pilot-artifacts/{capture_v1.txt,corpus_v1.txt,report_v1.txt}`.

---

## VERDICT

**The OPTION survives.** K1 — the `position` line's exact bytes — is the right
key, and I could not find a channel by which it returns a wrong answer. Its
failure direction is the only admissible one for a cache: it can miss a saving,
never write a wrong label.

**The MATRIX does not survive.** Its decisive claim is marked MEASURED and is a
generalisation by a factor of **102,346** from a corpus that cannot support it;
its stated tie-breaker against the coarser keys is **false about the shipped
tool**; the cost cell it prints as MEASURED measures a different operation in a
different program; the seat list in its single most load-bearing paragraph is
wrong, and wrong by transcribing prose that the script it cites explicitly
forbids anyone to transcribe; the gate set it says K2/K5's soundness depends on
is three times wider than the code makes it; both citations in its account of
why it exists are unverifiable at its own revision; and the registration it
promises to settle still says the opposite key at HEAD.

**1 FATAL, 6 MAJOR, 5 minor.**

---

# FATAL

## FATAL 1 — **THE DECISIVE ROW IS MARKED `MEASURED` FOR A POPULATION 10⁵ TIMES SMALLER THAN THE ONE IT DECIDES, AND THE MATRIX HAS NO STRUCTURAL ARGUMENT IN ITS PLACE**

The quote, `matrix_label_cache_key.md:78-80`:

> **Neither mechanism is a transposition**, so no amount of folding reaches a
> record the exact key misses. MEASURED.

and `:66`:

> **A fold that has never merged a pair is not a trade.**

**The evidence for the counts is sound and I re-derived it independently.** My
own parser over `/home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt`
(sha256 `4563f0500e957fef28b7b0aba82ba2dbceca0e8a3d74e892924fb034dcde1808`,
matching the receipt) gives 742 records, 347 distinct `position` lines, 347
distinct sorted `(cell, player)` lists, 395 hits, hit rate
`0.532345013477089`, multiplicity histogram `[(2, 345), (26, 2)]`, the two 26×
lines exactly `position start` and `position start moves 0,0`, and 26 games in
13 identical pairs. Every figure in `artifacts/arc3_leverB_41_count_v3.txt`
reproduces. **The numbers are not the defect.**

**The defect is the quantifier.** *"No amount of folding reaches a record the
exact key misses"* is a claim about the sweep. What was measured is a claim
about the pilot. The gap, measured:

| | pilot | one tranche (218 openings) | the sweep (3,487) |
|---|---|---|---|
| distinct games | 13 | 218 | 3,487 |
| game pairs | 78 | 23,653 | 6,077,841 |
| cross-game `(pair, k)` transposition opportunities, `k ≥ 2` | **1,576 MEASURED** | ~627,714 | **~161,296,550** |
| factor vs the pilot | 1× | 398× | **102,346×** |

(`k = 0` and `k = 1` are excluded: those two prefixes are byte-identical across
games and the exact key already folds them. `avg 28.538` asked prefixes per
distinct game, MEASURED from the capture.)

Transpositions found at those 1,576 opportunities: **0**. A rule-of-three bound
on a zero over 1,576 trials puts the per-opportunity rate at **≤ 1.904 × 10⁻³**
at 95 %. Applied to the sweep's ~1.61 × 10⁸ opportunities, that admits **up to
~307,000 transposing events**. The pilot's zero does not distinguish
*structurally impossible* from *merely unobserved at 13 games*, and the matrix
asserts the first while having measured only the second.

**AND THE MECHANISM MAKES IT WORSE, NOT BETTER.** The corpus is deterministic
self-play. Two games that ever reach the same position are IDENTICAL from that
ply on. So a cross-game transposition at depth `k` does not yield one extra
hit — it yields roughly `(game_length − k)` of them, and the games' whole tails
collapse together. The fold's yield is heavy-tailed: **zero in most small
corpora and large when it fires.** A zero at 13 games is exactly the modal
observation of such a process, and the matrix reads it as the process's mean.

**A STRUCTURAL ARGUMENT WAS AVAILABLE IN SECONDS AND IS NOT MADE.** The book's
generator dedupes openings by `canonical_form`
(`crates/pistol-cli/src/random_openings/mod.rs:174`:
`if !seen.insert(canonical_form(&state.played().collect::<Vec<_>>()))`), and the
pilot's report header reads `opening_turns 3`
(`/home/tom/pistol-runs/wp20pilot-artifacts/report_v1.txt:8`). Distinct
canonical forms imply distinct stone sets, so **no two openings can transpose or
mirror onto each other at `k ≤ 3` — by construction, not by measurement.** That
is a real a-priori result covering part of the range, it is worth stating, and
it also says that 156 of the pilot's 1,576 opportunities carried no evidence at
all. Beyond `k = 3` the matrix has neither a measurement that scales nor an
argument. Under D-291 the *"in seconds"* test is whether the derivation was
available on the machine the matrix was written on: this one was — it is a
`git grep` and a report header.

**AND §4's FLIP CONDITION IS UNREACHABLE BY THE PLAN IT GOVERNS.**
`matrix_label_cache_key.md:111-113`:

> **WHAT WOULD FLIP THE SELECTION.** A measured corpus in which the exact key's
> hit rate is materially below a coarser key's …

No such corpus will ever be measured. The sweep IS the corpus, and under this
selection its captures are never counted under a coarser key, so the sweep
generates no evidence about its own flip condition. A flip clause that the
governed run cannot satisfy is prose, not a criterion (D-424's test: it changes
what nobody may conclude).

**WHAT WOULD FIX IT.** Three things, none of which reopen K1:
1. Re-mark the row honestly — *"MEASURED at 13 openings; the sweep's population
   is 10⁵ times larger and this number does not transfer"*.
2. State the canonical-form argument for `k ≤ opening_turns` as the a-priori
   part, and admit there is nothing for `k > opening_turns`.
3. Replace the unreachable flip clause with one the sweep CAN produce: have
   `capture::run`'s cache count, per tranche, how many of its misses share a
   `key_pos`/`key_full` with an earlier miss. That number costs the arena a sort
   it can already do, it is exactly the coarser keys' yield at scale, and it
   turns the sweep into the measurement the matrix says it wants — settling
   D-562(2)'s open three-key question at the same time.

---

# MAJOR

## MAJOR 2 — **THE CHECKER ROW IS FALSE ABOUT THE SHIPPED TOOL, AND IT IS ONE OF THE RECOMMENDATION'S THREE REASONS**

`matrix_label_cache_key.md:56`:

> | **does the checker need it too?** | **no** — `tools/cold_label_check.py` partitions on the same string | yes, a second implementation | yes | yes | yes |

and `:98-100`:

> It costs the least code, **needs no second implementation in the cold-label
> checker**, and folds every hit the coarser options fold …

`tools/cold_label_check.py` at `31315f8` does not partition on anything.
`/usr/bin/grep -n "sampled = " tools/cold_label_check.py` → `:227`:

```
sampled = [(at, row) for at, row in enumerate(rows) if at % stride == 0]
```

Its sample is **every record whose zero-based index is a multiple of `stride`**
(`:228-231`, which prints exactly that). The word "partition" does not occur in
the file; the string `position` occurs only as the record's third field passed
to `ask_cold`. There is **no notion of a cache key and no hit/miss split**.

This matters twice over.

**(a) The row's "no" is false for K1 too.** D-576's amended criterion —
*"sample cache hits and misses separately at the registered stride"* — requires
the checker to know which records were hits. The shipped tool cannot answer that
under ANY of the five keys. The honest cell for K1 is *"yes — but the
re-derivation is a first-occurrence scan over a string the record already
carries"*, which is still a genuine advantage over K2/K3/K5 and is the true
shape of the argument. As written, a present-tense claim about a committed tool
describes work that does not exist.

**(b) Correctly read, this axis is evidence AGAINST K1, not for it.** Under K1 a
hit's record carries a `position` line byte-identical to the miss it was folded
from. `cold_label_check.py` re-asks that line in a fresh process and compares.
So **under K1 the cold check cannot fail on a cache defect** — it reproduces the
same bytes exactly when hard rule 4 holds, which is the assumption under test.
Under K2/K3/K5 it *could* catch a bad fold. K1 makes the external referent
vacuous against the cache. That does not make K1 wrong (its soundness needs no
check), but the matrix presents the checker column as a reason to prefer K1 when
it is really a reason the whole verification burden falls on §4.4 alone.

**Fix.** Correct the cell to *"yes, trivially"*; delete "needs no second
implementation in the cold-label checker" from §4 or restate it as "needs the
cheapest one"; and record that under K1 the cold check is not a check of the
cache.

## MAJOR 3 — **BOTH CITATIONS IN "WHY IT EXISTS AT ALL" ARE UNVERIFIABLE AT THE NAMED REVISION, AND THE REGISTRATION AT HEAD STILL REGISTERS K2**

`matrix_label_cache_key.md:9-12`:

> The key was chosen in `wp21_throughput_prereg.md` **revision 2** and consumed
> as settled by `wp21_prereg.md` §1's seat table before any matrix was written.

Neither half checks out at `31315f8` or at HEAD.

```
$ git log --oneline -- docs/experiments/wp21_throughput_prereg.md
57c660c docs(wp21): the sweep reserves a 1,000-opening holdout, …
$ git show HEAD:docs/experiments/wp21_throughput_prereg.md | head -1
# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 1.
$ /usr/bin/grep -c "cache\|D-576" docs/experiments/wp21_prereg.md
0
```

The throughput prereg has **exactly one commit** and is **revision 1**;
revision 2 exists only as the `git stash create` object
`fce50bc5b00baab9066f1e7bdf10c48c025b7755` that
`wp21_throughput_prereg_rev2_REVIEW.md`'s header names, on a branch (`overnight2-stopped`)
that the ledger records as deleted. `wp21_prereg.md` is **revision 2** at HEAD
and **does not mention the cache at all** — no §1 seat table row consumes this
key, because no such row exists.

**AND THE MATERIAL CONSEQUENCE IS WORSE THAN A BAD CITATION.** The committed
revision 1 §2 still registers the key the matrix declines:

> **THE KEY IS THE POSITION AND NOT ITS SPELLING, AND NOT A HASH.** Two prefixes
> whose move orders differ but whose stones agree replay to the SAME
> `GameState` … The key is therefore the replayed position's **sorted
> `(cell, player)` list**, compared exactly.

That is K2, and the sentence that justifies it is the one D-576 and the ledger's
F-1.4 both record as FALSE. So at HEAD the tree carries a committed ADR (D-576,
`docs/decisions.md:1220`) and a committed pre-registration that **contradict each
other on the key**, and the matrix — whose closing promise at `:14-16` is *"the
selection is re-opened here … whatever survives is what the registrations then
say"* — names no document to amend and no amendment obligation. A successor
following the matrix's own pointer lands on K2.

**Fix.** Replace the two citations with what is actually in the tree (D-576 at
`docs/decisions.md:1220` and, if the reviewer's stash object is to be cited,
cite it as a stash object). Then state the obligation explicitly: **whatever
this red team leaves standing, `wp21_throughput_prereg.md` §2 must be amended
before it governs a run, and an amendment reopens its review however small the
diff.**

## MAJOR 4 — **THE ONE CELL MARKED `MEASURED` IN THE COST ROW MEASURES A DIFFERENT OPERATION IN A DIFFERENT PROGRAM**

`matrix_label_cache_key.md:54`:

> | **cost per lookup, ESTIMATED against a 885 ms search** | ~0 | ~0 | **22.99 µs MEASURED at the census (D-570)** | ~0 | ~0 |

D-570's 22.99 µs is *"the cost … 22.99 µs a firing"* of `CensusKeys::at` — the
census's `pistol_core::canonical_key` fold, taken **inside the engine**, on a
board the search already holds, at a census firing. K3's cell in this matrix is
the cost of a **lookup in `pistol-arena`**, whose own row one line above
(`:53`) prices as *"replay + 12 images + sort"* — a different program, a
different input path, and a replay the census never pays. The cell shares its
denominator with nothing it is placed beside.

Under D-291 this is the named defect verbatim: *"a number that LOOKS measured
and is not, because that is what a red-team cannot attack."* Its own header
(`:19-22`) claims the MEASURED numbers all come from
`artifacts/arc3_leverB_41_count_v3.txt`; this one does not, and the header does
not say so.

**And the cell is arithmetically inert.** 22.99 µs against the row's own 885 ms
is **0.0026 %** — the same "~0" every other cell carries. So the only number in
the cost row that appears to discriminate discriminates by nothing, while
reading as K3's penalty. The honest reading is that **cost per lookup is not a
decision axis at all**: every option is free against a 885 ms search.

**Fix.** Mark it ESTIMATED-by-analogy with its provenance stated, or delete the
row and say in one line that no option's lookup cost is measurable against the
search.

## MAJOR 5 — **THE PLAY-ORDER CLAIM NAMES THE WRONG GATE AT `:89`, AND THE GATE SET IS THREE TIMES WIDER THAN THE CODE MAKES IT**

`matrix_label_cache_key.md:51`:

> **YES** — `heuristics.rs:155` reads `last_stone` under `gates.countermove`;
> **`:89` walks play order under `gates.killers`**

and `:85-89`:

> **K2 and K5 fail by ANSWERING.** Both are sound only while `killers`,
> `history` and `countermove` are **all** `false`.

**The `:155` half is right.** Verified: `heuristics.rs:153-159` is the
`gates.countermove` block and `:155` is `&& let Some(at) = last_stone(state)`,
with `last_stone` at `:191-193` returning `state.played().last()`.

**The `:89` half names the wrong gate.** `heuristics.rs:89` is
`for played in state.played()` inside `record_cutoff` (`:70`), and
`record_cutoff`'s only production call site is `crates/pistol-search/src/pvs.rs:491-498`:

```rust
if let CandidatePolicy::Staged(params) = self.policy
    && params.ordering.any()
    && best_score >= beta
    && forced_bound.is_some_and(|forced| best_index >= forced)
{
    self.heuristics.record_cutoff(self.position.state(), ply, best_cell);
}
```

`params.ordering.any()` is `killers || history || countermove`
(`crates/pistol-search/src/params.rs:114-116`) — **not** `gates.killers`.

**And the wider claim is over-broad. The dependence is ONE gate, not three.**
Traced site by site at `31315f8`:

- **`history`** is written at `:76-79` keyed `(mover, cutoff)` and read at
  `:162-175` keyed `(mover, at)`. Nothing on its path reads `played()`. **Play-order
  independent.**
- **`killers`** reads `self.killers[ply]` (cutoff cells — search-determined, not
  root-order determined) and `self.pair_killers[ply]`. `pair_killers` is written at
  `:95-101` only when `state.phase() == Phase::Second`, where `last` is the stone
  the SEARCH just placed. **Every capture root is `Phase::First`** — `capture.rs`
  builds `position` lines from complete `Turn`s and `asked_prefixes` (`:26-42`)
  drops the winning last prefix, so no asked line carries a truncated turn — hence
  no root play order reaches `pair_killers`. **Play-order independent for this
  workload.**
- **`countermove`** is the only channel: written at `:105-107` from
  `opponent_last`, which at a `Phase::First` root and at `Phase::Second` ply-1
  nodes IS the root's last played stone, and read at `:155` through `last_stone`.

So the correct statement is **"K2 and K5 are sound while `countermove` is
`false`"**. The error is in the conservative direction and does not move the
recommendation — but §4's flip clause at `:113-116` makes *"a seat that pins the
three ordering gates off as a rule"* a precondition of any flip, and that
precondition is two-thirds larger than the code requires. **A flip condition
inflated against the option it would flip to is not a neutral error.**

**The enumeration, printed with its scope.** `git grep -n "played()" -- crates/`
returns **22 lines**, `LC_ALL=C sort`ed. Outside `#[cfg(test)]` and the arena's
stub engine, the SEARCH-side hits are exactly **two**: `heuristics.rs:89` and
`heuristics.rs:192`. `heuristics.rs:412` is inside `#[cfg(test)] mod tests`
(module opens at `heuristics.rs:243-244`) and is **not** a production read — the
matrix's implied count of two is right; its attribution of one of them is not.
The remaining production hits (`arena/src/openings.rs:205`,
`cli/src/random_openings/mod.rs:174`) are outside the search and cannot affect a
label.

**Fix.** Correct the gate at `:89` to `params.ordering.any()`, narrow §3's
K2/K5 clause to `countermove`, and narrow §4's flip clause to match.

## MAJOR 6 — **THE SEAT LIST IN THE RECOMMENDATION'S OWN ATTACK PARAGRAPH IS WRONG, AND WRONG BY TRANSCRIBING PROSE THE SCRIPT FORBIDS TRANSCRIBING**

`matrix_label_cache_key.md:101-104`:

> *Gate 9 does not run the sweep's seat or the sweep's budget.* Its `SEATS`
> array is **radius / staged / staged-with-heuristics / staged-with-solver** …

The array at `tools/determinism.sh:67-81` has **five** entries. The fifth,
`:80`:

```
"staged-safety-net-cap configs/gate_staged_snk_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"
```

The four-name list the matrix prints is the **stale enumeration in the script's
own header comment** (`tools/determinism.sh:20-24`) — which sits in the same
paragraph that reads:

> THE SEATS ARE THE `SEATS` ARRAY BELOW AND ARE NOT COUNTED IN PROSE — a count
> in a comment is a second place for a fact the array already states, and it
> went stale the first time a seat was added.

The matrix read the comment instead of the array and inherited the exact defect
the comment exists to warn about — D-424(3)'s *"a claim the document makes twice
is a defect waiting"*, caught in the wild. It does not change the attack's
conclusion (none of the five seats is `configs/instrument_v0.toml`, and the
budgets at `:84` are `depth_turns 4` and `nodes 200000`, not `nodes 400000`),
which is why this is MAJOR and not FATAL. But the matrix's single most
load-bearing paragraph cites a fact it did not check, against a script that told
it not to.

**Fix.** Cite the array, not the comment — and, separately, fix
`tools/determinism.sh:20-24`, which is a live defect in a CI gate's own
documentation.

## MAJOR 7 — **THE RECORDED "STRONGEST ATTACK" IS NOT THE STRONGEST; A SHARPER ONE ABOUT GATE 9 IS AVAILABLE AND UNRECORDED**

`matrix_label_cache_key.md:100-101` records the attack as *"Gate 9 does not run
the sweep's seat or the sweep's budget."* True, and worth recording. **But it
stands in front of a worse one, which the matrix does not state:**

**No limb of gate 9 asks the same `position` twice in one process.**
`run_seat` builds run A/B as one session over `${budgets[@]} × ${positions[@]}`
with `newgame` before each, and compares two PROCESSES on the *same script*
(`tools/determinism.sh:238-250`). Run C is one process per position; run D is
one session over the same positions once each; C vs D compares layouts
(`:252-283`). **The cache's actual question — "does ask #2 of a byte-identical
`position` in this process return ask #1's bytes?" — is the one arrangement gate
9 never builds.** What gate 9 gates is *"`newgame` clears what a DIFFERENT
position left behind"* (its own words at `:15-18`). That implies the cache's
property only if the clear is total, which is an argument about code, not a gate.

The matrix's §4 claims K1's soundness *"reduces to hard rule 4 and gate 9"* and
that *"No claim about what the search reads is needed and none is made"*
(D-576's phrasing). The honest version: it reduces to **hard rule 4 plus §4.4's
byte-identity run**, with gate 9 covering an adjacent property at other seats and
other budgets. That is not a weaker position — §4.4 IS strong, and the matrix
already says it is what closes the gap — but the recommendation should not claim
gate 9 asks the cache's question when it does not.

**For the record, the code does support the argument**, and I checked it because
the matrix says it does not need to: `Instance::new_game` (`instance.rs:85-88`)
calls `Searcher::clear` (`search.rs:262-271`), which clears the transposition
table, the heuristic tables **and** the solver (`solver.reset()`). So §2's
*"No state crosses an ask"* is true of the code as committed.

**Fix.** Add the sharper attack beside the recorded one, and restate §4's
soundness sentence as "hard rule 4 plus §4.4", not "hard rule 4 and gate 9".

---

# minor

## minor 8 — the baseline the five options must beat is nowhere in the matrix

The field is five keys. **"No cache at all" — the pass that exists today — is
not a row, not a column, and not costed.** The matrix has no BENEFIT axis at
all: its two cost rows are *cost per lookup* (~0 for every option, MAJOR 4) and
*lines of new code* (15–45, all trivial), and it never states what any option
SAVES. On `wp21_throughput_prereg.md` §5's own ESTIMATED figures the lever saves
*"3.26 h of the 7.15 h eight-way wall"* and its registered verification costs
*"~2 × (11 min + 3 h)"* — **the check is registered at roughly twice the wall
saving**. (The serial picture is the other way: 48.95 h → 22.89 h.) The cache
also forecloses `--census` by name (D-576), a capability cost no row carries.

I rate this **minor and not FATAL** for one reason only: D-576 takes the cache
*"in full"* as an architect ruling, so the baseline is arguably outside this
matrix's remit. But the matrix cannot both say *"the selection is re-opened
here"* (`:14-16`) and exclude the option a reader reaches for first. One row
saying "the baseline is ruled in by D-576 and is not reopened here" would close
it.

## minor 9 — three options are missing from the field; one is cheaply costed and negligible, and saying so is the point

- **A key on `(position, go, binary, config)`** — i.e. a cache that outlives one
  `capture::run`. The matrix forecloses it in §1 (`:30-35`) by noting
  `label_go_line` is computed once at `capture.rs:333`, which is true *within*
  one run and is exactly why the fuller key is the only one that can be shared
  **across the sweep's sixteen tranches**. Its yield, derived from the pilot's
  own shape: sixteen per-tranche caches perform **92,572** searches against a
  single global cache's **92,542** — **30 searches, 0.032 %** — because the only
  prefixes shared across openings are the two rule-3-forced ones. One line would
  have costed it; not naming it leaves a reader to assume it was large.
- **A persistent on-disk cache** — same yield as above plus a cross-run identity
  obligation, so it is dominated. Still a row.
- **A normalised-but-not-folded key** — correctly absent, and the matrix should
  say why: `Turn`'s pair token is canonical by construction
  (`crates/pistol-core/src/turn.rs:90, 154-169` — *"An uncanonical pair is
  refused rather than reordered"*) and `exchange::position_line`
  (`exchange.rs:154-161`) spells the move list through `Turn::to_string`, so
  there is no spelling variation left to normalise. A reader cannot tell an
  option that was rejected from one that was never seen.

## minor 10 — the hit rate is the pilot's, and the matrix does not mark which population any row describes

Derived from the pilot's own shape (13 distinct games, **28.538** asked prefixes
per distinct game MEASURED, 2 prefixes shared by every game):
`asked = 2·G·28.538`, `distinct = G·26.538 + 2`.

| openings | asked | distinct | hit rate |
|---|---|---|---|
| 13 (pilot) | 742 | 347 | **0.5323** |
| 218 (one tranche) | 12,443 | 5,787 | 0.5349 |
| 3,487 (the sweep) | 199,027 | 92,542 | **0.5350** |

The direction is favourable and the size is 0.3 pp, so nothing moves — but every
cell in the matrix's first three rows carries a 13-opening number used to decide
a 3,487-opening run, and none is marked as such. (This also cross-checks
`wp21_prereg.md`'s *"~93 100 distinct positions ESTIMATED"* against 92,542 from
the pilot's shape — they agree to 0.6 %.)

## minor 11 — the K4 row over-costs zobrist and over-states its uncertainty

`:53` prices K4 as *"replay + 80 XORs"*. `GameState` already maintains its
128-bit key (D-8), so the arena pays a replay it needs under K2/K3/K5 anyway and
**zero XORs of its own**. And its distinct count is not in doubt in any
interesting sense: 347 items under a 128-bit hash collide with probability
~347²/2¹²⁹ ≈ 10⁻³⁴. The ESTIMATED marking is correct per D-291; the cell reads
as though the number were open.

## minor 12 — the matrix ships at a revision whose own CI run failed

The working-tree `docs/experiments/arc3_ledger.md` records
`artifacts/arc3_ci_dev_31315f8.txt` — **"`ci: FAIL: formatting`, EXIT=1"** — for
`31315f8`, this matrix's own named revision, fixed at `0321cc1` (now HEAD). The
matrix file and `tools/label_cache_count.py` are byte-identical across the two,
so nothing in this report is affected. Recorded because a named revision that a
report is judged at should be a green one, and because the ledger entry carrying
this fact is still uncommitted.

---

# THE STRONGEST SURVIVING ATTACK

> **The matrix's decisive row is a zero counted over 1,576 cross-game
> transposition opportunities and applied to a run with ~161,000,000 of them — a
> factor of 102,346 — and it is marked MEASURED. A rule-of-three bound on that
> zero constrains the per-opportunity rate only to ≤1.9×10⁻³, which at the
> sweep's scale admits up to ~307,000 merges the exact key will miss; and because
> the corpus is deterministic self-play, two games that ever transpose are
> identical thereafter, so the fold's yield is heavy-tailed — zero in most small
> corpora, large when it fires. K1 is still the right key, because a missed
> saving is the only error direction a cache may have and every coarser key buys
> that saving with a class of wrong answer. But "a fold that has never merged a
> pair is not a trade" is a claim about 13 openings wearing the marking of a
> claim about 3,487, and the sweep it governs is arranged so that the flip
> condition §4 registers can never be observed.**

The remedy that makes the selection safe rather than merely lucky: have the
cache count, per tranche, how many of its own misses share a `key_pos` or
`key_full` with an earlier miss. That number is the coarser keys' yield at the
scale that matters, it costs a sort the arena can already do, and it converts
§4's unreachable flip clause into one the governed run actually produces —
settling D-562(2)'s open three-key question in the same pass.

---

# WHAT SURVIVED

1. **K1 itself.** I looked for a channel by which two byte-identical `position`
   lines asked twice in one `capture::run` process, each after `newgame`, could
   return different bytes, and found none. `Instance::new_game`
   (`instance.rs:85-88`) → `Searcher::clear` (`search.rs:262-271`) clears the TT,
   the heuristic tables and the solver. `pistol-solver`'s queries hand back
   sorted, deduplicated vectors (`lib.rs:90-98`; `query.rs:256-266`
   `fill_empties`), and its one `HashMap` uses a seedless SplitMix64 and is
   never iterated on a choice path (`table.rs:171`, `lib.rs:93-98`).
   `candidates::within_radius` builds through a `BTreeSet`. Every `staged.rs`
   tier sorts and dedups. `ordering::order` performs zero clock reads with
   `deadline: None`, which instrument mode always passes. `capture::normalise`
   strips the only two machine-dependent fields, by gate 9's own rule.
2. **The error-direction argument**, which is the real reason K1 wins: K1 can
   only miss a saving; K2/K5 can answer from another play order, K3 returns a
   move in a mirrored frame with node counts D-137 says are not
   symmetry-invariant, K4 answers from a collision. §3's per-option failure modes
   are all correct as stated.
3. **The counts.** 742 / 347 / 395 / 0.5323 and the multiplicity structure
   `[(2, 345), (26, 2)]` reproduce exactly under an independent parser sharing no
   code with `tools/label_cache_count.py`.
4. **The instrument.** `tools/label_cache_count.py` refuses on a bad digest, a
   bad arity, a control character in a path, and calls a void a void rather than
   an answer of zero. Its coarser columns are **live**, not decorative — see the
   rejected attacks below.
5. **The honesty of the "why it exists" paragraph's substance.** The matrix
   opens by calling its own existence a finding against the arc, and records
   D-576's contradiction rather than hiding it. That is the right shape. Only its
   citations are wrong (MAJOR 3).
6. **§4.4's status as the criterion that closes the gap.** *"A single differing
   byte voids lever B"*, and the refusal to repair-and-recompare, is the
   correctly-designed part of this package.

---

# ATTACKS I ATTEMPTED AND REJECTED

1. **"The symmetry fold is dead, so 347 is trivially equal to 347."** Rejected by
   measurement. I drove `tools/label_cache_count.py`'s `images()` directly: a
   mirrored pair folds together, a 60° rotation folds together, a stone list has
   exactly **12** distinct images, two different shapes fold apart, and a colour
   swap folds apart. The shipped test suite carries the same positive controls
   driven as a program —
   `crates/pistol-arena/tests/label_cache_count_tests.rs::two_transposed_prefixes_are_two_cache_keys_and_one_stone_set`
   and `::a_mirrored_position_folds_only_under_the_symmetry_column`. The columns
   can fail; they did not.
2. **"`Searcher::clear` does not clear the solver, so K1 is unsound at a solver
   seat."** Rejected: `search.rs:266-270` calls `solver.reset()` — *"Wholesale
   (design wp18b §1)"*.
3. **"Play order reaches the search through the threat state or the candidate
   generator, so K2 is unsound independently of the gates."** Rejected: every
   producing path sorts (`fill_empties`, `tier_f`, `tier_t_union`, `filtered`,
   `within_radius`). Ironically this makes K2 **safer** than the matrix says,
   which is MAJOR 5's other half.
4. **"`pistol-eval`'s incremental state is application-order dependent, so K2 is
   unsound on the eval axis too."** Rejected: `crates/pistol-eval/src/eval.rs:26`
   makes order-independence part of the `Eval` contract in those words —
   *"never on the order they arrived in"* — and `handcrafted.rs:135-139`
   documents that its map is never iterated on a value path (D-498). The matrix
   never checks this axis; it happens to be safe.
5. **"`stones_of`'s parity-derived player mis-assigns colours on a truncated
   final turn."** Rejected: only a game's FINAL turn can be truncated (rule 4),
   and `capture::asked_prefixes` (`capture.rs:26-42`) drops exactly that prefix,
   so no captured `position` line carries an odd turn. The script's own docstring
   states the rule correctly.
6. **"A cross-tranche persistent cache would materially raise the yield."**
   Rejected by measurement: 30 searches of 92,572, **0.032 %** (retained as
   minor 9 because the option should still have been named).
7. **"The hit rate collapses at sweep scale, so the 1.5 floor is not really
   cleared."** Rejected: it *rises*, to 0.5350 (duplication 2.1507 against the
   registered 1.5 floor). Retained as minor 10 only for the unmarked population.
8. **"A cached run's remaining MISSES could answer differently from the uncached
   run's, because the engine process sees a shorter ask sequence."** Real, but
   **§4.4 closes it**: the cached run's answers for the duplicated half are
   copies of the first half's, while the uncached run's come from asks made after
   371 prior ones, so any bleed shows up as a byte difference. The matrix's own
   §4 already points at §4.4 for this.
9. **"D-576 is committed and append-only, so 'the selection is re-opened here' is
   a fiction and this red team is cosmetic."** Rejected — **a real re-opening is
   available and cheap.** `docs/decisions.md` is append-only, not immutable in
   effect: D-137, D-194 and D-562 are each amended or superseded by a later line,
   and D-576 carries its own flip clause. Re-opening costs exactly one new ADR
   line recording the change of key plus an amendment to
   `wp21_throughput_prereg.md` §2, and that amendment reopens that document's
   review — which it **already owes** under MAJOR 3, because its committed §2
   registers K2 and contradicts D-576 today. So the cost of a genuine re-opening
   is one ADR line and a review that is owed regardless. **The re-opening is
   real; it simply does not fire, because K1 survives.** What must still happen
   is the amendment: the tree may not go into a governed run with a registration
   and an ADR naming different keys.
