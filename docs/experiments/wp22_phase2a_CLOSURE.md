# WP-2.2 Phase 2a — closure.

**Closing revision**: named in §6 below, with its CI receipt.
**Exit**: **STOP AND SPLIT at both gates.** `wp22_phase2a_STOP_E.md` carries
Stage E's split, `wp22_phase2a_STOP_SUMMARY.md` carries the matrix's, and D-710
is the ADR.

## 1. What the dispatch asked for, and what it got

| dispatch item | outcome |
|---|---|
| §0 tail, paste block, v7 amendment, clock-fix confirmation, green from the gate's log | **complete** |
| E1 derivation memo, premises quoted, calculus IDs only | **complete** — and it failed four reviews |
| E2 REVIEW-design PASS, or STOP E with the default applied | **STOP E**, at four rounds under D-709 |
| E3 receipts: `k` per `L`, codes, observations, purity, stabilisation `L` | **complete**, replicated three times |
| Matrix: all rows, killed rows shown, every number marked, digests, two arms, ledger quoted, seed pilot run, failed-precedent check | **complete** — and it failed four DECISION-RED-TEAM rounds |
| Red team, one revision, second red team confirms by attack | **four rounds each**, under the operator's grant |
| HANDUP, or STOP with a split proposal | **STOP with the split** |
| Export receipt, worktrees removed | **complete**, nine directories, all clean |
| CI green at the closing revision | §6 |

## 2. THE ONE DEFECT, FIVE TIMES

Nine review rounds, nine FAILs, and one defect: **a referent not matched on the
property that drives its score.** D-710 records all five faces. The shortest
statement of what it cost: **the fifth instance was created by the fix for the
fourth**, in a document whose own §8 offers "enumerations over the whole pattern
space" as the remedy for that class — the enumeration was over the code space
and the statistic counts the observed population.

## 3. WHAT IS TRUE AND NOT IN DOUBT

**No round found an arithmetic error.** Four independent implementations of the
tuple agree with zero disagreements; three census runs agree cell for cell.

- `k` = 25 / 98 / 357 / 357 at `L` = 7 / 9 / 11 / 13; the `L = 11` ladder is
  357 / 121 / 47 / 36. **`k(13) = k(11)`**, and `L = 13` buys one class and one
  code for 18 % more window traffic.
- The covering length is **11**, derived and confirmed — and `THM-WINDOW` is
  **not** closed by it, which the memo states in terms.
- Max exact `DEF-T` on one axis is **2** over all 177 147 lines.
- The `t` merge: **27 of the 335 classes present**, 5 348 of 57 996 patterns,
  repairable at T4 for `k` 357 → 384.
- Reversal invariance 0 of 59 049; swap equivariance 0 mismatches, 187 orbits.

## 4. WHAT THE PACKAGE MEASURED THAT OUTLIVES ITS STOP

1. **The arena refuses a movetime budget by name**
   (`crates/pistol-arena/src/validate.rs:45-51`), so D-705's time-matched arm is
   not runnable for any row until an ADR moves it.
2. **`book_v3` funds exactly ONE acceptance arm** at Δ = 10 — 8 000 pairs, power
   **0.9045**, and **0.6970** if split between two — which is D-705's own stated
   premise, now measured.
3. **The instrument seat runs at 529 255 nps**, discharging the premise memo's
   OWED §5 figure; the two play seats differ by **2.8x** in throughput and depth.
4. **The eval's profile share is 44.70 %**, not the 31.77 % three documents used.
5. **A game-level split seed moves individual weights by up to 65 %** — and
   offline loss still orders table sizes cleanly, paired 8 of 8 at `t` = 12–15.
6. **`eval_families_2026-09.md` §A1's "1.4 observations per parameter" is the
   RECIPROCAL**; the real density is 0.73.
7. **On this corpus, the threat enum adds real but SMALL structure over counting
   stones** — 1.4 % at `L = 7`, 10.8 % at `L = 9` — and every affordable coarse
   rung scores BELOW a stone-count quotient on the unnested comparison.

## 5. WHAT LANDED IN THE TREE

- **`tools/hex_enum/`** — six modules and two suites behind **CI gate 19**
  (`GATE_TOTAL` 21 → 22): the enum, its ladder, the corpus census, the seed
  pilot, and the referent machinery. Every file under rule 9's cap.
- **`tools/receipt_digest_check.py`** — the structural fix for a printed-digest
  class that recurred three times. It caught three more on its first run and a
  fourth in the summary written after it existed. **Not a CI gate**, because
  `artifacts/` is gitignored.
- **Nine ADR lines**: D-702 … D-709 (the paste block and the operator's grant)
  and **D-710** (this stop).
- **Documents**: the enum memo and its four reports; the matrix, its arithmetic
  pass and its four red-team reports; the two stop documents; the §0 receipt.
- **Nothing under `configs/`. No governed run. No opening spent. No Elo.**

## 6. CI AT THE CLOSING REVISION

Recorded in §6 of this file at close; see `artifacts/wp22_phase2a/ci_closing.txt`
for the gate's own log (D-674). The commits between the revision CI's scratch
tree was taken at and the closing revision are **docs-only**, and the diff is
quoted below.

## 7. WHAT A SUCCESSOR READS FIRST

`wp22_phase2a_STOP_SUMMARY.md` — it carries the split, the two architect
questions, the field with each row's strongest surviving attack, and the export
receipt. Then D-710, then `wp22_phase2a_STOP_E.md`.
