# CLAUDE.md — pistol (working name; operator may rename)

pistol is a maximum-strength CLASSICAL search engine (alpha-beta + threat search, no
MCTS) for hex-lattice Connect(6,2,1) — the same game the mantis project trains an
AlphaZero-style system for. Rust cargo workspace. Design source of record:
docs/research/minimax_report.md — its BUILD/PROTOTYPE/SKIP verdict table and staged
plan are the prior; SPRT match results are the judge. Read it before proposing any
search or eval technique.

## Game rules (pinned — every line below has a pinning test in pistol-core)

1. Board: unbounded 2-D hexagonal lattice, axial coordinates (q, r); 3 line axes;
   6 neighbours per cell.
2. Win: ≥6 own stones contiguous along one axis. Overlines (7+) win. No bans, no
   exact-six, no variants.
3. Turn 1 = ONE stone (origin WLOG). Every later turn = TWO stones by the mover.
4. A win completes the instant any single placed stone forms ≥6; the turn's second
   stone is then not played. Sudden death is scored in TURNS; the completing stone is
   the tactically relevant unit.
5. Legal placement: within hex-distance 8 of an existing stone (union of radius-8
   balls around all stones). This is a game RULE — a named constant in pistol-core —
   never a search knob.
6. No captures. No rule-level draws; matches may impose a turn cap — the engine
   treats a cap as an evaluation horizon, never as a game rule.

## Design point

- Deployment budget: a strong move consistently within **0.5 s** (stretch 0.1–0.3 s)
  on a single workstation; online evaluation on CPU. GPU is offline-only (net
  training, opening solving).
- Deterministic instrument mode is a first-class requirement: fixed depth (in TURNS)
  or fixed nodes, single-thread, stable tie-breaking, CPU eval. Every strength claim
  comes from an instrument mode.
- Extension axes the architecture must keep cheap: new eval backends (handcrafted →
  incremental codebook net), new budget kinds, new search features, alternative
  candidate policies, and the future API layer.

## Map

- crates/pistol-core — board, hex geometry, rules, win detection, pair-move
  generation, zobrist (lazy per-cell keys; side-to-move + intra-turn phase in the
  key). Zero deps beyond std. THE one source of game truth.
- crates/pistol-eval — `Eval` trait + implementations (v0: handcrafted 3-axis
  line-window pattern tables; later: incremental codebook net). Contract is
  incremental: apply/undo per placed stone.
- crates/pistol-search — PVS + iterative deepening + TT + move ordering + threat-only
  quiescence; budget handling; `SearchInfo`.
- crates/pistol-solver — threat generation, TSS/DBS, df-pn family (later stages;
  starts as the threat generator only).
- crates/pistol-engine — composition: the `Engine` trait
  (`new_game / set_position / go(Budget) -> BestMove + SearchInfo`), config
  load + validation. The ONLY seam the future API layer wraps.
- crates/pistol-arena — match runner: paired openings, SPRT, Elo, distinct-game
  dedupe, per-side compute accounting.
- crates/pistol-cli — binaries: line-protocol engine (I/O mirrors `Engine` 1:1),
  perft, bench, selftest.
- crates/pistol-api — RESERVED, empty until the API spec lands.
- configs/ — explicit, complete, schema-validated configs. docs/decisions.md —
  append-only ADR log. docs/research/ — the report. docs/ROADMAP.md — stage plan.

## Hard rules

1. **Config.** Explicit + complete; `serde(deny_unknown_fields)`; missing key =
   error; NO code-side default for any tunable — a default lives in exactly one
   schema place. `Budget` is a closed enum {depth_turns, nodes, movetime_ms}; an
   absent budget is an error, never a fallback.
2. **Rules truth.** Game geometry, legality, and win detection live in pistol-core
   only; no other crate re-implements them. The radius-8 legal region is a pinned
   constant; the SEARCH candidate policy is config, never a literal. Do not conflate
   the two radii.
3. **Fail loud.** No silent fallback, swallowed error, or skip-with-default.
   Wrong-kind/wrong-shape input raises a named error.
4. **Determinism law.** In instrument mode nothing nondeterministic may influence
   move choice: no unseeded hash-iteration order on choice paths (fixed-seed hasher
   or sorted iteration), no time-based tie-breaks, no thread races. A determinism
   self-test (same position + budget twice ⇒ identical bestmove + node count) is a
   CI gate.
5. **Bench discipline.** A perf-sensitive change ships with a pre-registered hotspot,
   expected gain bracket, and abort threshold; one change = one commit = one
   IQR-gated bench; report nps AND time-to-depth. A measured structural floor is a
   finding, not a failure.
6. **Strength claims.** Ship instrument (fixed depth/nodes), protocol, n, distinct-n
   (identical games deduped), and per-side compute. Search/eval changes are accepted
   or rejected by SPRT over paired balanced openings. Never wall-clock-only.
7. **Tests.** Behavior-named. Pair-move perft against a brute-force reference is the
   movegen oracle; golden win/no-win boards pin rules 2 and 4; tactical suites are
   sha-pinned fixtures with a 10 MB/file ceiling.
8. **Artifacts.** Nets, books, match logs, bench outputs are never committed; a
   committed manifest may sha-index them.
9. **Files.** Single responsibility, ~300-line soft cap; exceeding requires a
   why-justification comment that never states a line count (counts are derived,
   never asserted).
10. **Decisions.** Every non-obvious design choice = one ADR line in
    docs/decisions.md (`D-n: choice — reason — what flips it`). Silent architecture
    drift is a breach; amend the ADR instead.
11. **API deferral.** pistol-api stays empty until the API layer is specified. The
    `Engine` trait + line protocol ARE the contract the API will adapt; adding
    transport dependencies anywhere else is a breach.

## Process (every non-trivial change)

DESIGN → REVIEW-design (fresh context, attacks the premise) → IMPL → REVIEW-impl
(fresh context, not the implementer, checks against the design) → RED-TEAM on
rules/data paths (adversarial inputs). Pre-register verdicts before experiments; no
post-hoc threshold moves. Reviewers flag correctness and requirement gaps, not style.

A named design decision with more than one viable option is settled by an OPTION
MATRIX — options, costs, failure modes, recommendation — attacked by a fresh-context
DECISION-RED-TEAM subagent BEFORE selection; the surviving option's ADR line records
the strongest surviving attack. An option adopted without a matrix, or a matrix never
attacked, is the same breach as silent architecture drift.

A pre-registration is reviewed at the revision that GOVERNS the run: the revision
that governs a run must itself pass a fresh-context review before the first run it
governs. Reviews of superseded revisions do not transfer — an amendment reopens the
review, however small the diff.

THE INSTRUMENT HAS A GOVERNING REVISION TOO. An artefact that produces a registered
number — a `tools/` script, a scratchpad harness, or a command block the document
prints — is named in the pre-registration WITH ITS REVISION, and a change to it
reopens the review exactly as an amendment to the document does. `tools/` is where
such artefacts usually live; living there is not what makes the rule apply. Without
this, a run stands on an instrument whose own review had failed and is licensed by
argument rather than by this text.

A pre-registration's literal commands are exercised before its review passes, on an
input of the SAME KIND as the registered workload — the same sort of artefact,
differing only in identity — and never on the registered workload itself. A synthetic
stand-in exercises syntax; only a real instance of the kind exercises ATTRIBUTION,
which is where a command that counted the wrong symbols passed a synthetic dry run and
still shipped. The dry run is not a governed sample and does not consume the
pre-registration's first run. The pre-registration records the dry-run input and its
output. This constrains the dry run's input; it constrains no reviewer, who may run
anything, the registered workload included.

AND IT RECORDS WHAT THAT OUTPUT MUST SHOW, together with the DEFECT CLASS the
criterion is meant to exclude. Recording without a criterion is a dry run nothing can
fail. A criterion that is a property the named defect class PRESERVES — internal
agreement between components sharing an input, output shape, plausible magnitude,
exit status — passes vacuously and is not a criterion; it must be one that defect
could falsify. An externally derived referent, a value computed by something that does
not share the suspect input, is the operationalisation that reliably achieves this and
is what a reviewer looks for first: sufficient, not necessary. This binds ANY
registered criterion, dry-run or governed alike.

A pre-registration states what its governed run COSTS — wall time, operator attention,
machine hours — so the proportion between the document and the run is visible on the
document's own face. Where the run is cheap, doubt about the instrument is answered by
REPLICATION and by a SECOND INSTRUMENT whose agreement criterion is registered before
either runs, never by a margin derived to defend a single sample. A registered
agreement criterion carries a REGISTERED CONSEQUENCE: the pre-registration states,
before either instrument runs, what DISAGREEMENT does to the verdict, or the criterion
leaves standing the after-the-numbers decision it exists to forbid. A derived margin is
the instrument of a measurement that cannot be taken again, and it is the wrong
instrument for a workload measured in seconds. Neither this rule nor the dry-run rule
is mechanized, and neither catches a run whose answer is already known before it is
taken — that defect is judged, not checked.

REVIEW-design, REVIEW-impl and RED-TEAM are dispatched as subagents with fresh
contexts; the implementing session never reviews its own work. A WP is not landable
while its reviews are outstanding. A session that cannot dispatch subagents states so
and stops after IMPL; the operator launches the reviews. A reviewer finding is
verified with a minimal reproducer before its fix lands; a finding that cannot be
reproduced is recorded as rejected with the attempted reproducer.

A review is dispatched against a NAMED REVISION — a commit SHA, or a `git stash
create` SHA where the work is uncommitted — and every reviewer states that revision
in its report header together with whether it still matches HEAD. Mutation testing
runs in a separate git worktree, never the live tree: a mutation is a deliberate
break, and a break left in the tree the implementing session is editing is
indistinguishable from a regression.

A change under tools/ is reviewed against tools/SHELL_CHECKLIST.md — the review
prompt cites it and the reviewer answers its items by name — because three
consecutive rounds found ONE class in those scripts: shell under `set -euo
pipefail` parsing unvalidated output and failing as EXIT-0-WRONG-ANSWER. Its
coverage rule is the binding one: any tools/ script that produces a recorded
number carries at least one test driving the shipped script. The checklist is
judged, not mechanized.

## Roadmap pointer

Stages per the research report: 0 foundations (correct + reproducible) → 1 tactical
core (threat gen, ordering, quiescence, df-pn) → 2 incremental codebook eval →
3 forcing search (TSS/DBS/CTSS/RZOP) → 4 Lazy SMP + tuning → 5 opening book + full
harness. Stage gates and work-package cuts live in docs/ROADMAP.md (authored in
session 1, changed only by ADR).

## Workflow

- All work on `dev`; `main` = gate-passing merges only. One feature = one commit,
  `type(scope): what changed and why it matters`, one line. Commit only when the
  operator asks. Merges/pushes are operator acts.
- CI gates, all locally runnable scripts under tools/: fresh-clone build;
  `cargo test --workspace --locked` + clippy (`-D clippy::all`); perft oracle;
  determinism self-test; artifact rejection; config validation; file-justification
  check.
- A gate or test claim in any report cites the gate's own log output, never a
  wrapper's exit status.
