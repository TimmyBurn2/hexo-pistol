# WP-2.2 Phase 2a — HANDUP. Nothing is selected here.

**Governing revision**: `695b6708` (`dev`) plus the closing commits this document
names. **NO SELECTION AND NO ADR OF SELECTION** (D-708): the field is ranked, the
strongest surviving attack on each row is quoted, and the two questions that
decide Phase 2b are put to the architect.

---

## 1. THE TWO QUESTIONS

### Q1 — WHICH ROW?

The books fund **one acceptance arm at Δ = 10** (§1.6 of the matrix; D-705 says
so itself, and `book_v3_ledger.md` already names R7 as its claimant). So this is
not "which is best" but "which is run", and it is close to irreversible.

| rank | row | what it costs to BUILD | what it buys, measured | strongest surviving attack |
|---|---|---|---|---|
| 1 | **R-H-EXT** | nothing to fit or ship — **but 3 to 6 integers with no procedure and no metric to choose their values** | unmeasured; the calculus has listed its terms as SPRT-gated candidates since v1.0 and nobody has gated them | **it may not be a Phase-2 row at all** — Phase 2's field is the learned family and this is Stage-0 work nobody did. Three red teams would rank it second or not at all. |
| 2 | **R-A4-CLASS**, `L = 9` or `L = 7`, rung T4 | a fit and a backend | **1.4 % at `L = 7`, 10.8 % at `L = 9`** over a quotient that only counts stones — real and small | it is ranked on the only criterion in the field that was registered, could fail, and was run; **rewarding the row that happens to have been measured is a bias**, and its `t`-merge defect (27 of 335 classes) is unrepaired |
| 3 | **R-A5-TOPK** | a fit and a backend | nothing measured | **its parameter count is 811 or 143 depending on the evaluator's summand**, and its kill — one bucket scores all novel structure alike — has never been measured by anything |
| 4 | **R-A1-L7F** | a fit and a backend | nothing measured | below the covering minimum, **and §2.5 measures that at `L = 7` stone counts explain as much as the threat tuple** |
| 5 | **R-A1-L8F** | a fit and a backend | nothing measured | below the covering minimum; **the enumeration that would settle whether that matters has never been run** (`eval_families` §8: *"minutes of compute, not run"*) |
| 6 | **R-A3-L11F+F7** | a fit with virtual features and a coalescing step | nothing measured | its source disowns the remedy at 200 000x this corpus's size; density 1.16 positions per parameter |
| 7 | **R-A2-L11F** | a trainer that does not exist here, a dependency, ≥ 4 nets, a digest discipline, a shape check | nothing measured | three games measure the α-β width optimum BELOW the accuracy optimum, and **≥ 4 nets against one funded arm is a selection problem no other row has** |
| — | R-A1-L11, R-C-SPSA, R-D-W1 | — | — | killed: 0.73 positions per parameter; 60 500 openings of tuning on a barred book; settled by rule (D-704) |

**THE RANKING IS CONTESTED AND THE DISAGREEMENT IS THE USEFUL PART.** Three red
teams would order it differently — round 1 and round 2 put `R-A5-TOPK` first;
round 3 would order A5 → L7F → L8F → A4 and **not rank R-H-EXT at all until the
operator rules**. The matrix's answer is in its §6 and §14; the architect should
read the disagreement, not the rank.

### Q2 — HOW MANY SEEDS, AND IT IS REALLY "HOW MANY ACCEPTANCE RUNS"?

**MEASURED** (`artifacts/wp22_phase2a/seed_pilot/`):

- **Closed-form rows have no training seed.** One fit; ≥ 8 game-level splits for
  any validation number, which is worth **±3.5 %** on a single split.
- **Individual weights move by up to 65 %** of the table's scale between splits
  at 33 parameters and above. A fitted table is not a stable object here.
- **But offline loss ORDERS table sizes cleanly**: paired over the same eight
  splits, K = 64 beats K = 32 beats K = 8 on **8 of 8**, `t` = 12.05 to 14.86.
- **A trained row needs ≥ 4 nets** by the reference trainer's own standard —
  **and the books fund ONE acceptance run**, so four nets cannot each be
  accepted. That is R-A2's real cost and it is a selection problem, not a
  compute problem.

**Two sub-questions ride with Q2 and neither is this session's:**

1. **May `book_v3` fund a row that does not answer R7?** Its ledger names the R7
   acceptance SPRT as *"THE REASON THIS BOOK EXISTS"*. The matrix's argument is
   that R7 — *does this corpus move Elo at all* — is answered by whichever
   family's acceptance SPRT runs (D-704), so the claim and the rows are the same
   run. **That is an argument, not a measurement.**
2. **May `book_v2`'s 1 000-opening holdout be spent on a screen?** It is never
   labelled, is the only corpus-disjoint v2 sample, and carries two standing
   claims. MEASURED: it clears D-653's 0.9 floor at Δ = 30 / 1 000 pairs (0.9189)
   and Δ = 40 / 500 pairs (0.9067).

---

## 2. WHAT PHASE 2b OWES BEFORE ITS FIRST SPRT

1. **A play seat, named.** Three configs carry `mode = "play"` and the two
   candidates differ by **2.8x in throughput and 2.8x in depth** at 500 ms
   (172 627 nps / 1.21 turns against 478 718 / 3.42).
2. **An ADR moving the arena's movetime refusal**, or D-705's time-matched arm is
   not runnable for any row. `crates/pistol-arena/src/validate.rs:45-51` is
   *"the one refusal this crate exists to make loudly"* and its own text defers
   wall-clock to Stage 4.
3. **A play-seat pentanomial.** Every power figure in this package is tilted from
   `2,3,8,7,4` — a 24-pair instrument-seat sample that `book_v3_registration.md`
   §P6 already calls *"a thin base"*.
4. **The per-move root-score grammar** (D-707), which `search_next_2026-09.md`
   §1.4 names as the one obstacle to the cheapest decisive depth experiment.
5. **The evaluator's summand** — per cell, per stone, or per window. It fixes
   R-A5's parameter count and it is not deferrable, because it decides what a
   row's density even means (§4's currency table).

---

## 3. STAGE E's RECEIPTS

Full detail in `hex_threat_enum_v1.md` §7 and `wp22_phase2a_STOP_E.md`.

- **`k` is COMPUTED**: 25 / 98 / 357 / 357 at `L = 7, 9, 11, 13`; the ladder at
  `L = 11` is 357 / 121 / 47 / 36. **`k(13) = k(11)`** and `L = 13` buys **one
  class and one code** for 18 % more window traffic.
- **`THM-WINDOW`'s covering length is 11**, derived from the tuple's reach and
  confirmed by enumeration — and §6.6 states in terms that this does **not**
  close THM-WINDOW, whose own question (a four needs 8 cells, a five needs 7) is
  window-relative and still unrun.
- **The registered criterion fails at 12 of 16 cells**; its four passes are
  withdrawn as class-count monotone.
- **The corrected nested test says the enum ADDS at all sixteen**, 1.36x to
  3.50x, at twelve replicates with a null matched on the join's own cell count.
- **The `t` merge**: 27 of the 335 classes present, 5 348 of 57 996 patterns,
  repairable at T4 for `k` 357 → 384 and at T2 for 47 → 58 (+86 % parameters).
- **Maximum exact `DEF-T` on one axis is 2**, over all 177 147 lines — so
  `LAW-OVERLOAD`'s `t ≥ 3` is unreachable by any single-axis class.
- **Four independent implementations agree with zero disagreements**, and the
  census is replicated three times with no cell disagreement.

---

## 4. THE BOOKS ARITHMETIC

Closed form cross-checked against two of this project's own figures (D-628's
3 554 and fishtest's 5 233 at Δ = 10), and every power figure RUN on
`sprt_power.rs` rather than computed, cross-checking `book_v3`'s committed
0.9045 digit for digit.

| arm | book | cap | power |
|---|---|---|---|
| node-matched, Δ = 10 | `book_v3`, 8 500 openings | 8 000 pairs | **0.9045** |
| the same split between two arms | `book_v3` | 4 000 each | **0.6970** — below D-653's floor |
| screen, Δ = 30 | `book_v2` holdout, 1 000 openings | 1 000 pairs | **0.9189** |
| screen, Δ = 40 | half the holdout | 500 pairs | 0.9067 |

**`book_v3` funds one arm.** `R-C-SPSA` needs **60 500 openings** of tuning —
7.1x the whole book, on a book D-644 bars it from — which is its kill.

---

## 6. EXPORT RECEIPT (D-469)

`artifacts/` is gitignored (hard rule 8), so an artifact directory is evidence
only while a tracked document carries its receipt's own digest. **This table is
that anchor for the whole package**, and `tools/receipt_digest_check.py` — added
this round after the same class recurred three times — verifies it.

| directory | files | `sha256sum -c` | receipt digest |
|---|---|---|---|
| `census` | 8 | clean | `aab7f4f6a8d450fac5609dce4635a21ea86ec658e4c42c11c9d89b98835e14a8` |
| `census_r3` | 8 | clean | `9bb8fc6f2e0500bcf10f48e493dd9e3615897d935b73b63d8a8f47bf179e9c39` |
| `census_r4` | 8 | clean | `aa88c298ac422a87df6505afcc33fcb1e2cb156783e81ec525b13dc4681bbdd1` |
| `census_r5` | 8 | clean | `7948e47a9e80ffc7c83d50079df8c14835a45d38c21bf272d96490b02d808fbe` |
| `clockfix_confirm` | 6 | clean | `7678648c5dcf2418a5a0eb320f0faac3835c21ae792723742e737fac2345bb66` |
| `n6_mutants` | 4 | clean | `36b4c999ba780800e40369cff20641db4982b05d3a3497bf84a88e4c40607d2e` |
| `nps_seat` | 6 | clean | `211b6e1002f32ee89baace6ed4ff510302b2aad7817094e109dcac121d51ca9e` |
| `seed_pilot` | 2 | clean | `f3518ad221413a042a616d3c6348458afc089fba6f30382734c802540592c7bd` |
| `sprt_power` | 4 | clean | `d82fce37add53453423e264a6220e2b47636302c7f2980c4a60f6c600005709b` |

Every verification worktree named in these receipts was exported before removal
(D-469), and `git worktree list` shows the main tree alone.

**AND THE CHECKER CAUGHT THIS DOCUMENT.** `receipt_digest_check.py` failed the
first draft of this HANDUP — it named `seed_pilot/` and printed none of its
digests — which is the fourth instance of that class in this package and the
second the checker found rather than a reviewer.

---

## 5. WHAT THIS PACKAGE DID NOT DO

No selection, no ADR of selection, no strength claim, no Elo, no governed run, no
opening spent, and nothing under `configs/` touched. Stage E stopped and split
after four rounds; the matrix's own gate is at its fourth round as this document
is written, and **if that round FAILs this HANDUP is superseded by a STOP with
the matrix's split proposal.**
