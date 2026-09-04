# Optimization arc — the GROUNDWORK dispatch, archived verbatim

**Why this file is tracked.** The dispatch is the governing document of this arc
and it reached the session as prompt text that no tree holds. Arc III's ledger
(`arc3_ledger.md` F-0.2) recorded what that costs a successor; this copy is the
remedy. Everything below the rule is the operator's text, unedited except that
the game-rules block — identical to CLAUDE.md's — is elided with a pointer.

---

# [GROUNDWORK] Optimization arc: width first, then the pruning family

Runs after Arc III closes and the box is quiet. One package at a time,
each its own closure, each SPRT or byte-identity judged, none bundled.
Order is RULED here from the audit's receipts; a session may not
reorder. Loop grant: up to three rounds per review gate, third
remedies-only (this arc only). D-401 never read. Standing hazards.

Read first: CLAUDE.md, docs/process.md, decisions tail (D-570 on), the
three audit files (A-rows, C-rows, the sealbot study §6), D-263, D-433,
D-428, D-491/492, D-534, WP-1.5b's design (Tier T), the arc II/III
instrument state, book_v2 holdout ledger (governed runs draw from the
1,000 reserved openings, consumed ranges receipted).

## Game rules (verbatim, binding)

*(The six rules, identical to CLAUDE.md's "Game rules" section; not repeated.)*

## Standing rules for every package below

- Perf packages: rule 5. Hotspot = the audit's perf receipt (A-02/03/04),
  re-profiled at the package's own SHA before the bracket is registered
  (D-477: never inherited). Bracket registered before measuring, both
  bands, both fixture sets, direction per convention (D-388/395/398).
  Byte-identity of search output via the two-binary diff procedure.
  Rebuild means re-record. Mutants at call sites (D-55y), run green
  before REVIEW-impl.
- Strength packages: rule 6. Gate default off; SPRT at 50 000 nodes,
  paired openings from the holdout range (ledger), elo0/elo1/floor/caps
  as registered, warm-replay Criterion 1'', second instrument agreement
  as registered, slot pass D-427, honest expectation quoting the
  package's own measured inputs; h0 closes as a measured finding, no
  ablation, no re-read. Committed configs move only on h1; pin
  re-recorded.
- Documents: mechanisms, invariants, tests only (D-483); numbers cited
  from artifacts by digest (D-479); passed-section freeze (D-54y);
  citation checker green before review (D-543).
- Fresh strongest subagents for reviews and red teams; never small.

## Tranche 1: perf, byte-identical (three packages, this order)

P1 — Threat-state laziness (A-03). The 36.1 percent maintenance paid on
every place/undo regardless of whether the node queries threat state.
Design chooses lazy update, batched update at query, or cheaper
structures; the state is a cache of the board, so output is byte-
identical by construction and the two-binary diff is the whole
correctness proof, plus the solver's oracle gates re-run (the solver
reads this state). Bench bracket registered; expectation: nps up,
magnitude UNMEASURED.

P2 — Legality per candidate (A-04). Replace the O(stones) per-cell test
with a maintained region (bounding structure or per-stone-ball
occupancy). Byte-identical outputs required; the false comment at
board.rs:148-151 deleted; the 257/1,025/2,049/4,097-stone growth series
re-taken as the receipt. Bench bracket registered.

P3 — Codegen (R2): LTO, codegen-units, opt-level, portable only.
Byte-identity, bench, pin re-recorded. `target-cpu=native` and PGO
excluded from pinned binaries by ruling.

Tranche 1 closure: one line with the combined nps factor over the
Arc-III baseline, MEASURED, both terms named (D-479).

## Tranche 2: instruments (two packages)

I1 — Width histogram (C-2 prerequisite). Per-row-class Tier-T set
sizes and quiet-ball sizes per node, on the line protocol behind a
token, no committed config change, byte-identity when off. This is the
number the width cap is designed against; without it the cap is opp_hot
again. [ROUTINE+], mutants at call sites, nps spot-check.

I2 — Budget-overrun VOID class (A-01 detection, R3). The instrument
VOIDs a game whose first iteration exceeds a registered multiple of the
node budget; the multiple is derived from the landed runs' measured
first-iteration ratios (artifact-cited), never from the pathological
reproducers. Red-team the refusal path (it touches the verdict).

## Tranche 3: width (the lever, C-2)

W1 — Tier-T width cap with fail-low re-widening. Full heavy shape:
premise memo (I1's measured histograms quoted), option matrix (delta-
ranked top-K; proximity-ranked; per-row-class caps; two-radius; staged
widening on fail-low; root-turn bound per A-01; NO-CAP null), DECISION-
RED-TEAM before selection, design, reviews, impl behind a gate,
calibration of K by a pre-stated rule on I1's data, bench, SPRT.
Registered risk from the sealbot study: colony blindness; the design
states the compensating mechanism (fail-low re-widening, FILTERED-row
exemption) and its test. Honest expectation: the largest lever in the
field, and the one with a measured failure mode; h0 is a legitimate
finding. On h1 this is the first depth-per-time change since 1.5b.

W2 — Root re-ordering by the previous iteration's scores (sealbot study
§6 item 3). Small-diff SPRT, warm-replay attribution. Cheap; runs after
W1 regardless of W1's verdict.

## Tranche 4: the pruning family, one at a time

S1 — Aspiration windows (C-1). Window from the last iteration's score,
±δ in eval units as a config key, mate ⇒ full width, fail-low/high
widening rule registered. SPRT; second instrument: node-matched time-
to-depth bench. Expectation: small or null at fixed nodes.

S2 — One-cell forced-reply extension (C-8). Extend one turn on a
FILTERED row with exactly one cover cell; extension budget per line as
a config key. SPRT. Expectation: null-to-small at eval v0 (D-428's
reasoning); recorded so it is measured, not assumed.

S3 — LMR on the unforced range (C-3). Whole-turn reductions only (D-111
forbids a mid-turn horizon); never on FILTERED/WIN-NOW rows; only at a
registered minimum depth; re-search on scan > alpha via the existing
PVS hook. SPRT with attribution showing the reduced subtree changed.
Runs AFTER W1 by ruling: LMR is the cap seen from the depth side, and
its "late" range is W1's width.

DEFERRED by ruling to Stage 2: C-5 futility, C-6 razoring, C-7 null-
move analogue (eval-margin techniques on an eval measured to misread
horizons, D-428). C-4 IID: no package, trigger never fires. C-9: Stage 4.

## Arc closure

One table, MEASURED: nps factor from tranche 1; W1's verdict with n,
distinct_n, pentanomial, llr_pair; each S-package's verdict; committed
config state. Then a sealbot anchor v4 on the final engine (book_v1, N =
100, movetime, gates as committed) for direction only. ROADMAP updated;
D-534 restated; exports; CI; summary, ONE LINE FOR THE MORNING first.

## STOP protocol

Per Arc III, plus: a byte-identity mismatch in tranche 1 = STOP that
package, not the arc (the next package may proceed on the prior SHA if
independent; record); any document failing after the granted rounds =
STOP and split; the arc never reorders packages to route around a STOP.

*(The operator's closing instruction: do not start until the other agent has
finished and cleaned up its worktree or branch; earliest start 23:00 UTC.)*
