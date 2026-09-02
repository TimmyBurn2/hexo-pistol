# OPTION MATRIX — the label cache's key. Revision 3.

**REVISION 3, AND IT REPLACES AN A-PRIORI ARGUMENT WITH A MEASUREMENT THAT
REFUTES IT.** Revision 2's §2.1 argued that a book deduped by `canonical_form`
cannot hold two openings that transpose or mirror at `k <= opening_turns`.
**THE ARGUMENT IS FALSE AT `k = 2`** — deduping the WHOLE opening says nothing
about its PREFIXES — and it was caught by
`wp21_throughput_prereg_rev3_REVIEW.md`'s BLOCKING 2 with a two-second
computation the matrix never ran. §2.1 is replaced by the measurement, taken with
a committed instrument (`tools/opening_prefix_fold.py`,
receipt `artifacts/arc3_opening_prefix_fold.txt`). **THE SELECTION DOES NOT MOVE,
and the measurement strengthens it rather than weakening it**: the fold's entire
measured yield sits at the one depth where a symmetry key would answer with a
mirrored position's `bestmove`.

**REVISION 2, AFTER A FRESH-CONTEXT DECISION-RED-TEAM.** Round 1 returned **the
OPTION survives, the MATRIX does not** — 1 FATAL, 6 MAJOR, 5 minor
(`matrix_label_cache_key_REDTEAM.md`, at `31315f8`). Every finding is disposed of
below at the row or section that owns it, and **one is rejected with its attempted
reproducer** (§5). The FATAL is the one that changes what this document may
claim: **the decisive row was marked MEASURED for a population 10^5 times smaller
than the one it decides**, and the fix is not a softer adjective — it is an
a-priori argument for the part of the range that has one, an honest blank for the
part that does not, and a flip clause the governed run can actually satisfy.

**THE DECISION.** `arena --capture` asks the engine once per asked prefix. A
label cache memoises `(totals, bestmove)` under some notion of *the same
question*. **Which notion** is a named decision with more than one viable option,
so CLAUDE.md's Process section requires this matrix and a fresh-context
DECISION-RED-TEAM **before selection**.

**WHY IT EXISTS AT ALL, AND IT IS A FINDING AGAINST THIS ARC.** The key was
chosen in an UNCOMMITTED revision 2 of `wp21_throughput_prereg.md` and consumed as
settled by an uncommitted revision 3 of `wp21_prereg.md`, before any matrix was
written; the review that caught it is `wp21_throughput_prereg_rev2_REVIEW.md`
M11. **This matrix is therefore written AFTER a selection was recorded**, and the
selection is re-opened here: the red team attacks the matrix, and whatever
survives is what the registrations then say.

**REVISION 1 CITED THOSE REVISIONS AS IF THEY WERE IN THE TREE AND THEY WERE
NOT** — at `31315f8` the committed `wp21_throughput_prereg.md` is **revision 1**,
whose §2 still registers the stone-set key, and the committed `wp21_prereg.md`
mentions no cache at all. **So the ADR (D-576) and the committed registrations
CONTRADICT EACH OTHER at that revision**, which is a real defect and not a
citation slip. **THE AMENDMENT OBLIGATION IS NAMED HERE**: revision 3 of the
throughput study and revision 4 of the sweep registration land in the same commit
as this matrix's revision 2, and until they do, the committed registrations are
the operative text and D-576 is ahead of them.

**EVERY NUMERIC CLAIM IS MARKED MEASURED OR ESTIMATED** (D-291). The MEASURED
ones come from `artifacts/arc3_leverB_41_count_v3.txt`, taken by
`tools/label_cache_count.py` over the pilot's own capture
(`capture_v1.txt`, sha256 `4563f050…`).

---

## 1. THE FIELD, DERIVED FROM WHAT THE ENGINE IS ACTUALLY ASKED

A capture ask is four things: a `position` line, a `go` line, a binary and a
config (`capture.rs`'s `ask` sends `newgame`, then those two lines, on a process
fixed by the seat). Within one `capture::run` the last three are loop-invariant —
`label_go_line` is computed once at `capture.rs:333`, outside the game loop — so
**every candidate key below is a notion of sameness over the `position` line
alone**, and they differ only in how much they fold.

| id | key | folds | where the notion already exists |
|---|---|---|---|
| **K1** | the `position` line's exact bytes | nothing | the line itself |
| **K2** | the replayed position's sorted `(cell, player)` list | transpositions | `GameState::key`, `key_pos` |
| **K3** | `canonical_form` over the stones | transpositions AND symmetries | `key_full`, D-570's census identity |
| **K4** | a 128-bit zobrist of the position | transpositions, and collisions | `GameState::key`'s own value |
| **K5** | `canonical_sequence` over the prefix | symmetries of the SEQUENCE | `key_seq` |

---

## 2. THE MATRIX

| | **K1** exact bytes | **K2** stone set | **K3** canonical form | **K4** zobrist | **K5** canonical sequence |
|---|---|---|---|---|---|
| **hits on the pilot corpus** | **395** MEASURED | **395** MEASURED | **395** MEASURED | 395 ESTIMATED (K2's, less collisions) | **395** MEASURED |
| **distinct keys** | **347** MEASURED | **347** MEASURED | **347** MEASURED | 347 ESTIMATED | **347** MEASURED |
| **hit rate** | **0.5323** MEASURED | 0.5323 MEASURED | 0.5323 MEASURED | 0.5323 ESTIMATED | 0.5323 MEASURED |
| **what a wrong hit returns** | *impossible*: the same question | another play order's answer | a symmetry image's `bestmove` — **a move in the wrong frame** | a different position's answer entirely | another play order's answer |
| **is a wrong hit possible in this tree?** | no | **YES, and by exactly ONE path** — `heuristics.rs:155` reads `last_stone(state)` under `gates.countermove`, so two transposed roots can order the root differently | YES, plus the frame error; D-137: the tie-break is not symmetry-invariant | YES, with probability rather than a mechanism | YES, same as K2 |
| **what makes it safe** | hard rule 4, and the gate below | **one** config value being `false`, unnamed by the key | nothing: the frame error is unconditional | a probability argument | one config value being `false` |
| **cost per lookup** | one string compare | replay + sort, ≤80 stones | replay + 12 images + sort | replay + 80 XORs | canonicalise the sequence |
| **cost per lookup, ESTIMATED against a 885 ms search** | ~0 | ~0 | ~0, and the one number anybody has MEASURED for this fold is **22.99 µs per firing for `CensusKeys::at` INSIDE THE ENGINE** (D-570) — a different operation in a different program from an arena lookup, and 0.0026% of an 885 ms search either way. **Revision 1 put it in this cell as if it were the lookup's cost** | ~0 | ~0 |
| **lines of new code** | ~15 ESTIMATED | ~40 ESTIMATED | ~45 ESTIMATED | ~35 ESTIMATED | ~45 ESTIMATED |
| **does the checker need it too?** | **yes, trivially** — a first-occurrence scan over a string the record already carries | yes, a second implementation of the fold | yes | yes | yes |

**THE FIRST THREE ROWS ARE MEASURED AT THIRTEEN OPENINGS AND THE SWEEP IS 3 487.
REVISION 1 MARKED THEM `MEASURED` AND WROTE A CLAIM ABOUT THE SWEEP, WHICH IS THE
FATAL.** What was measured: on the pilot's capture every option folds the same 395
hits. What was claimed: *"no amount of folding reaches a record the exact key
misses"*. The gap, MEASURED by the red team and reproduced here:

| | pilot | one tranche (218) | the sweep (3 487) |
|---|---|---|---|
| cross-game `(pair, k)` transposition opportunities, `k >= 2` | **1 576 MEASURED** | ~627 714 ESTIMATED | **~161 296 550 ESTIMATED** |
| factor against the pilot | 1x | 398x | **102 346x** |
| transpositions observed | **0 MEASURED** | — | — |

**A RULE-OF-THREE BOUND ON A ZERO OVER 1 576 TRIALS CONSTRAINS THE
PER-OPPORTUNITY RATE ONLY TO <= 1.9e-3 AT 95%**, which at the sweep's scale admits
**up to ~307 000 merges** the exact key would miss. And the mechanism makes the
estimate worse rather than better: the corpus is deterministic self-play, so two
games that ever transpose are **identical from that ply on** — the yield is
heavy-tailed, zero in most small corpora and large when it fires, and **a zero at
thirteen openings is the modal observation of such a process, not its mean.**

**WHAT THE DUPLICATION ACTUALLY IS, MEASURED**: 345 prefixes asked exactly twice —
the two seats replaying one deterministic engine — plus **two** prefixes asked 26
times, which are `position start` and `position start moves 0,0`, byte-identical
across all thirteen openings because game rule 3 forces turn 1 to the origin.
**Neither is a transposition**, so nothing in the pilot's duplication is evidence
about folding either way.

### 2.1 WHAT THE OPENINGS ACTUALLY FOLD, MEASURED — AND REVISION 2's A-PRIORI ARGUMENT WAS FALSE

Revision 2 claimed: *"no two openings can transpose or mirror onto each other at
`k <= 3` — a result, not an observation."* **It is not a result.** The book's
generator dedupes by `canonical_form` over the WHOLE opening; that constrains the
openings and says nothing about their prefixes, and two openings distinct at five
stones can share a mirrored three-stone prefix. `tools/opening_prefix_fold.py`
over the sweep's own window:

| `k` | exact-key classes | stone-set classes | symmetry classes |
|---|---|---|---|
| 1 | 1 | 1 | 1 |
| 2 | **2 327** | **2 327** | **368** |
| 3 | 3 487 | 3 487 | 3 487 |

and over tranche one's 218 openings, which is the scope the cache actually has —
the memo is built inside `capture::run` and dropped with it:

| `k` | exact-key classes | stone-set classes | symmetry classes |
|---|---|---|---|
| 1 | 1 | 1 | 1 |
| 2 | **213** | **213** | **171** |
| 3 | 218 | 218 | 218 |

**THREE READINGS, and each changes something revision 2 said.**

- **`k = 1` folds completely under every key** — the one forced origin stone.
- **`k = 2` IS WHERE THE ARGUMENT FAILED.** The symmetry fold merges 213 classes
  into 171. The STONE-SET key merges nothing extra there (213 = 213), because a
  pair token is canonically spelled, so the difference is symmetry alone.
- **`k = 3` folds nothing under any key**, which is `canonical_form`'s dedupe —
  the only depth at which revision 2's claim held, stated now as the measurement
  it always was.

**AND WHAT IT IS WORTH IS SMALL IN THE UNIT THAT DECIDES ANYTHING.** A symmetry
key would save **213 - 171 = 42 searches per tranche**: **0.72% of the tranche's
ESTIMATED 5 819 misses**, 0.34% of its records. Revision 2's *"the fold merges
nothing"* was wrong; *"the fold merges a great deal and it is worth under one
percent"* is right, and only the second is a fact about the run.

**AND THE 42 ARE EXACTLY THE RECORDS WHERE THE FOLD WOULD BE WRONG.** Each is a
prefix whose symmetry partner is a DIFFERENT position; answering it from the
partner returns a `bestmove` in the wrong frame, and node counts that D-137 says
are not symmetry-invariant. **The fold's entire measured yield sits in the region
where it is most wrong**, which is a better argument for K1 than the one revision 2
made up.

### 2.2 BEYOND THE OPENING, NOTHING IS MEASURED AND THE RUN MEASURES IT

The table above covers `k <= opening_turns`. Deeper prefixes are the sweep's own
business and §4's flip clause is the instrument: the cache counts, per tranche,
how many of its MISSES share a `key_pos` or a `key_full` with an earlier miss.

---

## 3. THE FAILURE MODES, ONE PER OPTION

- **K1 fails by MISSING.** A future corpus holding a genuine cross-game
  transposition loses that saving. It cannot answer wrongly, because the question
  it answers is the question it was asked.
- **K2 and K5 fail by ANSWERING.** Both are sound only while **`countermove` is
  `false`** — and revision 1 said all three ordering gates, which is **three times
  wider than the code makes it**. Traced: `heuristics.rs:89`'s walk over
  `state.played()` sits inside `record_cutoff`, whose call site is gated by
  `params.ordering.any()` (`pvs.rs:491-498`, `params.rs:114-116`) rather than by
  `killers`; its `last`/`second_last` feed **only** `pair_killers` — written at
  `Phase::Second` nodes, where the stone is search-placed, and every capture root
  is `Phase::First` — and `countermove`, which is READ only at `:155` under
  `gates.countermove`. `history` never reads `played()` at all. **So the single
  path from a root's play order to a search choice is the countermove table.**
  `configs/instrument_v0.toml:81` has it `false` today; nothing binds a future
  seat to that, and the key does not say it depends on it. **A cache whose soundness is a config value's shadow is a cache
  that breaks silently when the config changes** — and §4.3 of the registration
  says nothing downstream would notice.
- **K3 fails by ANSWERING IN THE WRONG FRAME**, unconditionally: the `bestmove`
  of a symmetry image is not a move in this position's frame, and node counts are
  not symmetry-invariant (D-137).
- **K4 fails by COLLISION**, at a rate no criterion can bound. A probability
  argument is not a criterion.

---

## 4. RECOMMENDATION — **K1**, and the strongest attack it must survive

**K1.** It is the only option whose soundness argument does not depend on
anything outside the ask itself: two identical questions get one answer. It costs
the least code, needs only the cheapest checker change of the five, and folds
every hit the coarser options fold **on the only population that has been
measured** — which §2 now says is thirteen openings and not the sweep.

**THE ARGUMENT FOR K1 DOES NOT REST ON THAT ROW, AND REVISION 1 LET IT LOOK AS IF
IT DID.** Even if a coarser fold merges 307 000 records at the sweep's scale, what
K1 loses is a **saving**. What K2, K3, K4 and K5 buy that saving with is a class of
**wrong answer** — and §4.3 of the governing registration says a wrong label is
invisible downstream. **A missed saving is the only error direction a cache may
have**, and that is the whole selection.

**THE STRONGEST ATTACK ON K1, STATED RATHER THAN ANSWERED — AND REVISION 1's WAS
NOT THE STRONGEST.** Revision 1 recorded that gate 9 does not run the sweep's seat
or budget. True, and weaker than this: **no limb of gate 9 ever asks the same
position twice inside one process.** Its `A vs B` limb compares two processes over
one script; its `C vs D` limb compares one-process-per-position against
all-positions-in-one-session — and in the session limb each position is asked
once. **A cache hit is exactly "the same position, asked again, in the same
process", and that is the one shape the determinism gate never takes.** The seat
and budget gap compounds it: five seats (`radius`, `staged`, `staged-heuristics`,
`staged-solver`, `staged-safety-net-cap` — **revision 1 said four, transcribed
from the script's own stale header comment in the paragraph that forbids
transcription**), none of them `configs/instrument_v0.toml`, at `depth_turns 4`
and `nodes 200000`, not `nodes 400000`.

**WHAT CLOSES IT IS NOT AN ARGUMENT BUT A RUN**: `wp21_throughput_prereg.md`
§4.4's byte-identity between an uncached and a cached capture, taken at the
sweep's own seat and budget, over a report with 12 443 records of which ~6 624 are
hits. That run asks the same position twice in one process ~6 624 times and
compares every byte against a pass that never did. **It is the reason §4.4 may not
be weakened, and the reason the whole verification burden falls there.**

**AND UNDER K1 THE COLD-LABEL CHECK IS A CHECK OF THE MEMO, NOT OF THE KEY**,
which is worth saying because the red team read it as vacuous (§5 rejects that
reading with its reproducer). K1 has no equivalence relation to get wrong, so
there is nothing about the KEY for T-A2 to falsify; what T-A2 still falsifies is
an implementation that returns the wrong entry, mutates one after insertion, or
emits records out of order.

**WHAT WOULD FLIP THE SELECTION — AND REVISION 1's CLAUSE COULD NEVER BE
OBSERVED.** It asked for *"a measured corpus in which the exact key's hit rate is
materially below a coarser key's"*, and under this selection the sweep's captures
are never counted under a coarser key, so **the governed run generated no evidence
about its own flip condition**. That is prose, not a criterion (D-424).

**THE FLIP CLAUSE THE RUN CAN SATISFY, REGISTERED HERE**: the cache counts, per
tranche, **how many of its own MISSES share a `key_pos` or a `key_full` with an
earlier miss**. Two counters, no extra search, one sort the arena already does,
reported in the run log beside the hit rate. That number **is** the coarser keys'
yield at the scale that matters. If it is materially above zero across the sweep,
K2/K3 become live for a future capture — under a seat that pins `countermove`
`false` as a rule rather than as a value — and this matrix is re-taken with the
sweep's own measurement in its decisive row. **It also settles D-562(2)'s open
three-key question in the same pass**, which no other instrument in this arc was
going to do.

---

## 5. THE ONE RED-TEAM FINDING REJECTED, WITH THE REPRODUCER ATTEMPTED

**MAJOR 2(b): *"under K1 the cold check cannot fail on a cache defect … K1 makes
the external referent vacuous against the cache."*** REJECTED.

The reasoning, followed to the record it is about. Let record R be a HIT: its
`position` field is the line P, and its `(totals, bestmove)` came from the memo
rather than from a search. `tools/cold_label_check.py` spawns a fresh process,
asks P, and compares. **If the memo returned the entry belonging to some other
position Q, R carries `answer(Q)` and the fresh ask returns `answer(P)`, and the
two differ.** The check fails, which is what a check failing means.

What IS vacuous under K1 is a check of the key's EQUIVALENCE — and K1 has no
equivalence, which is the point of choosing it. The finding conflates *"the key
cannot be wrong"* with *"the memo cannot be wrong"*, and only the first is true.
The cell is corrected to *"yes, trivially"* and §4 no longer claims K1 *"needs no
second implementation in the cold-label checker"*; the vacuity claim is not
adopted.

---

## 6. WHAT IS COSTED HERE THAT REVISION 1 DID NOT COST

**K0 — NO CACHE AT ALL, which revision 1's field omitted and which is the baseline
every other option must beat.** Zero new code, zero new failure modes, and the
sweep runs at §3's uncached wall — **7.74 h at N = 8**, against **6.11 h** with
the cache from wave two. **The cache's applicable saving on this sweep is 1.63 h
ESTIMATED against ~1.43 h of registered verification** (`wp21_prereg.md` §3,
`wp21_throughput_prereg.md` §5), which is roughly a wash. **K0 is not selected
because D-576 rules the cache APPROVED**, and this row records what that ruling
costs and buys rather than leaving the baseline unstated.

**THREE OPTIONS THE FIELD ALSO OMITTED, each costed and each dismissed on its own
ground.** A **persistent on-disk cache**: it would be an artifact class with a
format version, a digest and a manifest row (D-572) for a saving that lives inside
one tranche. A key on **`(position, go)` as a pair**: correct but redundant, since
`label_go_line` is computed once outside both loops (`capture.rs:333` against
`:340`, `:341`) — and the registration now carries that as a named lifetime
invariant instead, which is the cheaper way to buy the same guarantee. A
**tranche-scoped key**: identical to K1 within one `capture::run`, which is the
only scope the memo has.

**AND EVERY ROW OF §2's TABLE DESCRIBES THE PILOT'S 742-RECORD CAPTURE**, thirteen
openings, `nodes 400000`, `configs/instrument_v0.toml` — stated once here rather
than left for a reader to infer, because §2's first three rows are the ones the
FATAL was about.
