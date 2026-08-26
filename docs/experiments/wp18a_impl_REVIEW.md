# WP-1.8a REVIEW-impl — the solver, the fixture, the gates

**Verdict: FAIL — one BLOCKING plus eight MAJORS and minors; every finding
closed in code or recorded in design §9a by the fix round (`e668dfa`).**

**Provenance (read this first).** This file is a RECONSTRUCTION from two
surviving records: §9a's "REVIEW-impl amendments" list in
`docs/experiments/wp18a_design.md` (the findings the document itself
records: M-1, M-2, m-1, m-2, m-3, m-7) and the fix-round commit message of
`e668dfa` (the full fix list, which names the finding classes the review
carried: B-1, M-1..M-8 and the minors). The dispatching session's verbatim
report was lost to the session crash, and findings recoverable from neither
record are NOT invented here. The reviewed revision is the implementation
state at `3916afd`/`923475f` (the fix round landed as `e668dfa`).

---

## The recorded findings (§9a's list)

- **M-1 — σ-class narrowing, RECORDED**: the implemented σ class is |σ| = 2
  ONLY. The registered |σ| = 1 and |σ| = 3 require pre-placement stones
  beyond σ itself (a turn places exactly two stones) and the design never
  specified those stones' zone class — the gap the implementation hit at
  `perturb()` and closed by refusing to guess. The 17.5% of pairs refused
  on region collision (25,346 of 145,000) is announced in the gate's
  output and is a third narrowing the design also never specified.
- **M-2 — fixture composition actuals, RECORDED**: the bounded set's
  registered composition was authored without measuring (the D-291 class).
  What shipped: stone counts 7-21 (53/61 over the registered ≤ 10); depth
  3-4 positions ZERO (the depth-win family degenerated to one-node wins);
  the riposte family was eight byte-identical duplicates until this review
  caught it — now eight distinct positions, re-pinned, re-verified by R3'
  in gate (a). Registered "depth 3-4" positions are licensed-not-scheduled.
- **m-1 — mutation venues, RECORDED**: M-A died at gate (b); M-B, M-C and
  M-D died in the lib suite and at compile, not at the four oracle gates.
  The design's receipt table said "a gate dies" for all four; the honest
  reading is "the CI gate complex catches each" (the lib tests and the
  oracle gates run in the same `tools/ci.sh`).
- **m-2 — verifier independence is partial, RECORDED**: the verifier
  shares `ZoneP::add_graded`/`union_with` with the solver, so a
  grading-index defect passes the zone cross-check identically in both.
  The verifier's plan families, threat moves, blocking pairs and EP-1 scan
  ARE independent (board reads, no ThreatState); the zone arithmetic is
  shared and the design's "two independent constructions" overstated it.
- **m-3 — (c1) scope, RECORDED**: the replay checks move legality, not
  that the move still creates its hot window; (c2)'s revaluation is the
  value-axis compensation.
- **m-7 — measured figures, now with homes**: the σ sweep's placements /
  refusals (gate (c) stdout, `artifacts/wp18a_selftest_v1.txt` context);
  the deep fixture's seesaw total (`artifacts/wp18a_selftest_deep_v1.txt`);
  the corpus measurement (87+ unbounded searches, 0 deep wins, both
  players 1200+ Elo — a scratch probe over
  `timmyburn/hexo-bootstrap-corpus`@`1a82e15`, never committed).

## The fix round's landed closures (`e668dfa`, by finding class)

- B-1: rule-9 justification comments on the over-cap files.
- M-1..M-8 and the minors, as the commit names them: the union-of-stones
  tripwire; the exact min-hitting-set verifier check; the named wall-cap
  watchdogs (`VERIFIER-OVERRUN` / `SIGMA-SAMPLE-OVERRUN`); the single TOML
  parser feeding bin+example+gates (the config law); `#[ignore]`
  release-only gates driven by `--ignored` in the script; the riposte
  family deduped to eight distinct positions; the stall guard on the
  root's own answer.

**Note for the record:** the union-of-stones tripwire this fix round landed
was itself later found order-dependent by the RED-TEAM round (it still ran
during the walk, against stones-so-far); the after-walk fix landed with the
red-team fix round. The sequence — design claim, impl, REVIEW-impl fix,
RED-TEAM wound, final fix — is the process working, and §9a records all
of it.
