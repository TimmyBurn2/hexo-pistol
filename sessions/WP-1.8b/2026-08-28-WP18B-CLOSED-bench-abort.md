# WP-1.8b — CLOSED. Solver wired behind a gate that is false everywhere; M4 applied; bench abort + operator overrule foreclose the SPRT.

**Session date**: 2026-08-27/28
**Machine**: `omarchy` — `/home/tom/Projects/HeXO-AlphaBeta`, branch `dev`
**From**: `cad2d09` → closure HEAD (D-441)
**Status**: **COMPLETE.** No strength claim; the closure ADR is D-441.
**WP-1.8c is next** (the OR-side enumeration cost is its input).

---

## ONE LINE FOR THE MORNING

The sealbot platform merged (CI green, D-438/439/440), the anchor probe read
**branch B** (v0 proves the winners only two turns after sealbot's collapse —
but proves the *loser* wins pistol's own search missed), the **M4 widening**
landed through three review rounds and moved game 1's winner-proof **to the
collapse turn itself** (a true value flip: v0 refutes t42 in 955 nodes, M4
proves it in 10,726, the tree independently re-verified in 37 s), the solver
is **wired into pistol-search behind `[solver] on_search_path = false`** in
every committed config with solver nodes counted against the shared budget —
and the rule-5 bench **aborted its registered bracket by two orders of
magnitude** (ON nps ≤ 0.02 vs ≥ 0.5), so per the pre-registered clause and
**your overrule** the governed SPRT was not run: the cost finding is the
verdict, and WP-1.8c owns the strength question at viable cost.

---

## 1. The result

| | |
|---|---|
| merge | `96e2d85`, CI 19/19 green on the staged tree, D-438 (renumbered anchor), D-439 (verbatim), D-440 (pin re-recorded at merged HEAD, byte-reproduced twice) |
| probe | branch **B** registered before the run; 85/85 positions; winner-proofs at g1 t44/t46 and g2 t39/t41 — all AFTER the collapse bounds 42/37; the loser-win diagnostic (g2 t10/t12) recorded |
| M4 | applied (arm B: raiser × legal-region-outside-`C`); gates (a)–(d) green; the M4-4 theorem confirmed; probe re-run: **g1 t44→t42 (the collapse turn, a true value flip)**, g2 unchanged; 48 wall-caps vs v0's 35 |
| wiring | `pistol-search` calls the solver at trigger nodes and the root, both directions; `solve_defender` is a thin wrapper (zero df-pn changes); shared-budget accounting with two independent counters; cap with spent-means-store-nothing; schema v3 in all 13 configs; determinism 4th seat |
| bench | **ABORTED**: OFF 223,668 nps; ON 9–240+ s/search at cap 16384; ratio ≤ 0.02 vs ≥ 0.5 (`artifacts/wp18b_bench_v1.txt`) |
| SPRT | **FORECLOSED BY OPERATOR OVERRULE** (D-441 records it in D-424's own form): both verdict branches keep the gate false, so the run could not change any conclusion |
| verdict class | measured cost finding; the strength question at viable cost is WP-1.8c's |

## 2. The three bugs the closing gates caught (the honest story)

1. **REVIEW-impl C-1**: a mover live five at a gate-on root panicked the
   answer construction (`Turn::pair(at, at)`) — 9 of 20 determinism-fixture
   cases. Fixed: the completing stone pairs with the least legal partner
   (rule 4 ends the turn on it; the partner is turn-legality, not proof).
2. **W-2's mechanism, live**: the cap's unwind could *raise* a stored proof
   entry via merge — the determinism seat reproduced `SOLVER_CHILD_ZONE`.
   Fixed: **spent-means-store-nothing** (the unwind touches no entry;
   spent-before-stall turns the solve into `Unknown`).
3. **The trigger the root forgot**: the root block called both directions
   unconditionally, and a QUIET root's defender call is the zero-plan AND
   root the §2 red team proved unreachable *under the trigger* —
   `SOLVER_NO_PLAN`, reproduced on the seat's own session. The reviewer had
   flagged the unconditional root calls as a cost note (S-4); it was this
   correctness bug in a costume. Fixed: the trigger gates the root too.

Plus my own: three gate-script plumbing bugs (an empty-string budget default,
an underscore encoding that ate `depth_turns`, a read-loop var), two probe
driver bugs caught by its own dry-run discipline (a shared-scratch race that
silently dropped 40 of 85 cases; a doubled path prefix), and two position
feeds in the wrong grammar that the engine refused loudly while I chased the
refusals as engine bugs.

## 3. Process record

- Merge review: **APPROVE** (one pass). M4: matrix red-team (FALLS→fix),
  REVIEW-design (1 MAJOR→fix), REVIEW-impl (FAIL→fix: a false receipt
  corrected to the stronger true one, the flip independently verified, the
  three-site agreement test). Section 2: matrix red-team (FALLS: the parity
  law, the sealbot substring parser, dead zero-plan machinery), REVIEW-design
  (1 BLOCKING + 5 MAJOR→fix: the cap designed, mid-turn guards, the bench
  corrected and its trigger frequency MEASURED 8/24), REVIEW-impl (1 BLOCKING
  + warnings→fix). The bench abort, the probe branch, and the dry-run catches
  are all pre-registered instruments doing their job.
- The SPRT prereg (section 4) was never authored: the overrule arrived after
  the bench abort and before the prereg, so nothing was withdrawn that
  governed a run.

## 4. Environment

Login shell; long jobs detached (the determinism gate's solver seat is
minutes-per-leg and was polled, never foregrounded); mutations in worktrees;
D-401 honoured throughout.

## 5. Where to pick up

**WP-1.8c**: the OR-side arm-B enumeration (`|R|·|L|` per visit; MEASURED
|R| 5.2 mean / |L| 480 mean on the anchor positions) is the remaining
hotspot — the AND side already got its minimal-cover fast path. Inputs on
disk: `artifacts/wp18b_bench_v1.txt`, `artifacts/wp18b_probe_v{1,2}_*.txt`,
`artifacts/wp18a_tt_knee_v1.txt`, and the design's §2a/§7 amendments. The
SPRT seat exists, is gated, and runs the moment a viable cap has a
pre-registration. Open debts: the deep-shape family for gate (b)
(unscheduled, as before); the oracle-check driving test (WP-1.8a's recorded
debt); D-414/D-422/D-427 (final-cleanup WP, unchanged).
