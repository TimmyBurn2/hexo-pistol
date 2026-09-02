# OPTION MATRIX — the label cache's key. Revision 1.

**THE DECISION.** `arena --capture` asks the engine once per asked prefix. A
label cache memoises `(totals, bestmove)` under some notion of *the same
question*. **Which notion** is a named decision with more than one viable option,
so CLAUDE.md's Process section requires this matrix and a fresh-context
DECISION-RED-TEAM **before selection**.

**WHY IT EXISTS AT ALL, AND IT IS A FINDING AGAINST THIS ARC.** The key was
chosen in `wp21_throughput_prereg.md` revision 2 and consumed as settled by
`wp21_prereg.md` §1's seat table before any matrix was written. The review that
caught it is `wp21_throughput_prereg_rev2_REVIEW.md` M11. **This matrix is
therefore written AFTER a selection was recorded**, and the honest consequence is
that the selection is re-opened here: the red-team attacks the matrix, and
whatever survives is what the registrations then say.

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
| **is a wrong hit possible in this tree?** | no | **YES** — `heuristics.rs:155` reads `last_stone` under `gates.countermove`; `:89` walks play order under `gates.killers` | YES, plus the frame error; D-137: the tie-break is not symmetry-invariant | YES, with probability rather than a mechanism | YES, same as K2 |
| **what makes it safe** | hard rule 4, gate 9 of 19 | three config values being `false`, unnamed by the key | nothing: the frame error is unconditional | a probability argument | three config values being `false` |
| **cost per lookup** | one string compare | replay + sort, ≤80 stones | replay + 12 images + sort | replay + 80 XORs | canonicalise the sequence |
| **cost per lookup, ESTIMATED against a 885 ms search** | ~0 | ~0 | 22.99 µs MEASURED at the census (D-570) | ~0 | ~0 |
| **lines of new code** | ~15 ESTIMATED | ~40 ESTIMATED | ~45 ESTIMATED | ~35 ESTIMATED | ~45 ESTIMATED |
| **does the checker need it too?** | no — `tools/cold_label_check.py` partitions on the same string | yes, a second implementation | yes | yes | yes |

**THE ROW THAT DECIDES IT IS THE FIRST THREE READ TOGETHER.** Every option folds
the same 395 hits on the only corpus anyone has measured, and four of the five
buy that identical yield with a class of wrong answer. **A fold that has never
merged a pair is not a trade.**

**AND THE MEASUREMENT SAYS WHY THE FOLDS ARE IDLE**, which matters more than the
counts: the duplication is **345 prefixes asked exactly twice** — the two seats
replaying one deterministic engine — plus **two prefixes asked 26 times**, which
are `position start` and `position start moves 0,0`, byte-identical across all
thirteen openings because game rule 3 forces turn 1 to the origin. **Neither
mechanism is a transposition**, so no amount of folding reaches a record the
exact key misses. MEASURED.

---

## 3. THE FAILURE MODES, ONE PER OPTION

- **K1 fails by MISSING.** A future corpus holding a genuine cross-game
  transposition loses that saving. It cannot answer wrongly, because the question
  it answers is the question it was asked.
- **K2 and K5 fail by ANSWERING.** Both are sound only while `killers`, `history`
  and `countermove` are all `false`. `configs/instrument_v0.toml:79-81` has them
  false today; nothing binds a future seat to that, and the key does not say it
  depends on it. **A cache whose soundness is a config value's shadow is a cache
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
anything outside the ask itself: two identical questions get one answer, which is
hard rule 4 and is gated by `tools/determinism.sh` at **gate 9 of 19**
(`tools/ci.sh:104-105`). It costs the least code, needs no second implementation
in the cold-label checker, and folds every hit the coarser options fold on the
only population that has been measured.

**THE STRONGEST ATTACK ON K1, STATED RATHER THAN ANSWERED.** *Gate 9 does not run
the sweep's seat or the sweep's budget.* Its `SEATS` array is
radius / staged / staged-with-heuristics / staged-with-solver, and its budgets
are `depth_turns 4` and `nodes 200000` — not `configs/instrument_v0.toml` at
`nodes 400000`. So *"already gated"* is true of the property and not of the
configuration the sweep runs. **What closes it is not an argument but §4.4's own
byte-identity run**, which compares an uncached and a cached capture **at the
sweep's seat and the sweep's budget** — and that is the criterion the lever is
abandoned on. The attack is recorded as the reason §4.4 may not be weakened.

**WHAT WOULD FLIP THE SELECTION.** A measured corpus in which the exact key's
hit rate is materially below a coarser key's — which would mean genuine
cross-game transpositions exist at scale — **and** a seat that pins the three
ordering gates off as a rule rather than as a value. Both would have to be true;
the first alone only makes the miss more expensive, and the second alone buys
nothing.
