# Sealbot notes — comparison and idea harvest (ramora0/sealbot)

Status: analysis only. No arena code pairing pistol against sealbot lives in
this repo (operator rule); head-to-head happens via the hexo-bridge/HeXO
platform, which referees externally. Perf numbers below were measured on a
single sandbox core, Aug 2026, sealbot `current/` built with -O2; treat them
as order-of-magnitude, not lab-grade.

## What sealbot is

C++ (pybind11) iterative-deepening alpha-beta for the same game family:
unbounded hex lattice, 6-in-a-row, 1-then-2 stones, 3 axes. Single-thread.
Already bridged to the HeXO platform (hexo-bridge ships a
`seal_perf_engine.py` example) — it is the natural first external opponent.

Three rule/representation deltas that matter before any comparison:

1. **No radius-8 placement rule.** sealbot's game.py allows any empty cell;
   its engine plays "colony" moves at centroid distance max_r+3, which can
   exceed hex-distance 8 from every existing stone. If the HeXO server
   accepts those moves, the platform does not enforce pistol's rule 5. This
   must be resolved against the htttx spec before pistol plays there: a
   platform-legal opponent move outside pistol's rule-5 region would be
   refused by set_position (fail loud) and cost the game.
   *Resolved (docs/decisions.md D-101): the operator confirmed against the
   htttx spec and the HeXO server that radius 8 IS a rule the platform
   enforces, so nothing in pistol moves; the paragraph above stands as the
   question that was asked and this line is its answer.*
2. **Bounded arrays.** Flat 140x140 (coords in [-70, 69]); positions outside
   crash or are silently skipped. Pistol's unbounded board is strictly more
   correct.
3. **Doubles for eval/scores** — cross-platform nondeterminism pistol's
   integer-only design excludes by construction.

## Measured performance (single core, this sandbox)

Fixed depth (turns), time_limit effectively off, random distance-<=2 midgame
positions, seed 42:

| stones | d2 | d3 | d4 | d5 | nps |
|---|---|---|---|---|---|
| 5  | 3 ms | 28 ms | 375 ms | 1.40 s | 0.65-1.1M |
| 11 | 3 ms | 38 ms | 365 ms | 4.16 s | 0.6-0.9M |
| 21 (threats present) | 0.5 ms | 2 ms | 11 ms | 109 ms | ~0.4M |

At 0.5 s/move: depth 4 at 5-11 stones; depth 5-6 in threat-dense positions
(must-block filtering collapses branching); instant answers when a forced
line exists. Note their node = one turn-node (native pair), pistol's node =
one ply; nps is not directly comparable, time-to-depth is.

Pistol WP-06 reference (release, single core, origin): 150-275k nps;
r=1 d5 625 ms / d6 9.5 s; r=3 d3 323 ms / d4 27 s; 0.5 s => depth 3 at r=3.

Sealbot reaches ~1-3 turns deeper in the same wall-clock, PLUS threat-only
quiescence on top. The gap is not C++ (that is maybe 3x nps): it is almost
entirely the candidate policy — sealbot searches ~50 pairs per node, pistol
at r=3 searches thousands. This is the report's prediction ("brute widening
loses") measured in the wild, and it is what Stage 1 exists to fix.

## Where the branching goes: sealbot's mechanisms

1. **Hard candidate cap by static eval delta.** All empty cells within
   hex-distance 2 of a stone, scored by the single-stone eval delta
   (`_move_delta`, incremental, ~18 window lookups), sorted, capped at 15
   (root 20). Pairs come from a precomputed index list with i+j <= 14 on the
   sorted ranks — ~50-60 pairs/node. Deterministic (delta + coord tie-break;
   history deliberately excluded from selection).
2. **Must-block filter.** Maintained per-window (countA, countB) tables plus
   "hot" sets (windows with >=4 own, 0 opponent). If the opponent has
   near-complete windows, only turns hitting every such window survive; if
   none do, fall back to the unfiltered set. Cheap, huge in sharp positions.
3. **Unblockable-double-threat check.** If the opponent's hot windows cannot
   all be covered by any two cells, return a mate score without searching —
   a one-step VCDT test.
4. **Instant-win probe from hot windows** at every node and in quiescence.
5. **Threat-only quiescence** with stand-pat, depth cap 16, threat turns
   generated from the hot-window empties.
6. **729-pattern codebook eval** (3^6 ternary patterns over length-6
   windows, per axis), incrementally maintained via per-window pattern
   indices; weights baked from CMA-ES optimization (experiments/cma). A
   miniature of the report's Rapfi codebook idea, handcrafted-weight tier.

## Idea verdicts for pistol (mapped to the report's table and the roadmap)

| Sealbot idea | Verdict | Where |
|---|---|---|
| Top-K candidates by incremental eval delta, staged/widening rather than hard-capped | PROTOTYPE as a CandidatePolicy variant, SPRT vs radius | Stage 1, first candidate-policy experiment after r2-vs-r3 |
| Must-block filter from window-count threat tables | BUILD — this is conservative-defense-lite, the cousin of CTSS the report already mandates | Stage 1 threat generator |
| Window-count + hot-window threat tables | BUILD — the Stage-1 threat generator's natural data structure; pistol-eval's incremental window map is already 80% of it (keep rules truth in core, threat semantics in solver/search per Hard Rule 2) | Stage 1 |
| Unblockable-double-threat early mate | BUILD — one-step VCDT, near-free once hot windows exist | Stage 1 |
| Threat-only quiescence with stand-pat | Already BUILD in the plan; sealbot is working evidence it holds up in this exact game | Stage 1 |
| 3^6 pattern codebook + CMA-ES weight tuning | Evidence for Stage 2's direction; CMA on a 729-entry handcrafted codebook is a cheap PROTOTYPE rung between eval v0 and the distilled net | Stage 2 |
| Colony play / colony blindness (their own TODO admits "inability to block colonies") | Do not copy the hack; DO harvest the weakness: add distant-cluster attack/defense positions to the tactical fixtures, and treat it as the canonical failure mode of any capped candidate policy | Stage 1 fixtures |

## What NOT to copy

- The **uncompensated hard cap**: top-15-by-delta with no widening is unsound
  pruning; their colony-blocking failure is the symptom. Pistol's version
  must stage/widen (threat evidence or fail-low reopens the set) — the
  report's design, and the difference between "fast" and "fast and sound".
- Doubles, exception-driven time control with full-array memcpy rollback,
  and the stateful extract_pv — pistol's determinism law and triangular PV
  are already the better answers.
- TT keyed by the bare 64-bit hash with no verification distinct from the
  index (collisions merge silently). Pistol's D-8 layout is stronger.

## Strength calibration (operator ruling, 2026-08-19)

Recorded so no report drifts into treating either engine as a human-strength
proxy (docs/decisions.md D-197):

- pistol is NOT at strong-human strength, and will not be after the current
  work packages (the WP-1.3/1.4 era). Nothing in this repository claims
  otherwise.
- sealbot is strong, but strong humans exploit its weaknesses decently well —
  the colony blindness above is the canonical example. sealbot is a
  MILESTONE on pistol's road, not a proxy for human strength.
- The calibration order the operator rules: pistol below sealbot; sealbot
  below strong humans who exploit it.
- No human-strength claim is made without measurement, ever. CLAUDE.md
  rule 6 applies to claims about humans exactly as it applies to claims
  about engines: beating sealbot licenses the claim "beats sealbot" and
  nothing more.

## Comparison protocol (when we get there)

Via hexo-bridge (loopback for smoke tests, HeXO server for refereed games) —
outside this repo. Report per-side compute, wall-clock, and the rule-delta
caveats above. Honest expectation setting: at Stage-0 pistol (depth 3 at
0.5 s, no quiescence, no threat search) sealbot's depth 4-6 + quiescence +
must-block should win most games. The right first milestone is Stage-1
tactical core; compare after, not before.
