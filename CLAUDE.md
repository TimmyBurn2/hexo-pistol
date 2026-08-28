# CLAUDE.md — pistol (working name; operator may rename)

pistol is a maximum-strength CLASSICAL search engine (alpha-beta + threat search, no
MCTS) for hex-lattice Connect(6,2,1) — the game the mantis project trains an
AlphaZero-style system for. Rust cargo workspace. Design source of record:
docs/research/minimax_report.md (BUILD/PROTOTYPE/SKIP verdicts + staged plan are the
prior; SPRT results are the judge) — read it before proposing any search or eval
technique.

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
- Deterministic instrument mode — fixed depth (in TURNS) or fixed nodes,
  single-thread, stable tie-breaking, CPU eval (hard rule 4) — is first-class: every
  strength claim comes from an instrument mode.
- Extension axes the architecture must keep cheap: new eval backends, new budget
  kinds, new search features, alternative candidate policies, and the future API
  layer.

## Commands

- Build/test: `cargo build --workspace --locked` / `cargo test --workspace --locked`
  (add `--release` to ship).
- CI: `tools/ci.sh` — sole definition of the gates, no truer list exists elsewhere
  (its own header says so); it prints `gate N/19: <name>` as each one runs.
- Determinism: `tools/determinism.sh` — the hard-rule-4 self-test (same position +
  budget twice ⇒ identical bestmove + node count).
- Bench: `tools/bench_delta.sh` — pre-registered hotspot, IQR-gated, per-side
  revision mode; reports nps AND time-to-depth per hard rule 5.

## Map

- crates/pistol-core — board, hex geometry, rules, win detection, pair-move
  generation, zobrist (lazy per-cell keys; side-to-move + intra-turn phase in the
  key). Zero deps beyond std. THE one source of game truth.
- crates/pistol-eval — `Eval` trait + implementations (v0 handcrafted 3-axis
  line-window tables; later incremental codebook net). Incremental contract:
  apply/undo per placed stone.
- crates/pistol-search — PVS + iterative deepening + TT + move ordering + threat-only
  quiescence; budget handling; `SearchInfo`.
- crates/pistol-solver — threat generation, TSS/DBS, df-pn family (later stages;
  starts as the threat generator only).
- crates/pistol-engine — composition: the `Engine` trait (`new_game / set_position /
  go(Budget) -> BestMove + SearchInfo`), config load + validation. The ONLY seam the
  future API layer wraps.
- crates/pistol-arena — match runner: paired openings, SPRT, Elo, distinct-game
  dedupe, per-side compute accounting.
- crates/pistol-cli — binaries: line-protocol engine (I/O mirrors `Engine` 1:1),
  perft, bench, selftest.
- crates/pistol-api — RESERVED, empty until the API spec lands (hard rule 11).

## Hard rules

1. **Config.** Explicit + complete; `serde(deny_unknown_fields)`; missing key =
   error; NO code-side default for any tunable — a default lives in exactly one
   schema place. `Budget` is a closed enum {depth_turns, nodes, movetime_ms}; an
   absent budget is an error, never a fallback.
2. **Rules truth.** Game geometry, legality, and win detection live in pistol-core
   only; no other crate re-implements them. The radius-8 legal region is a pinned
   constant; the SEARCH candidate policy is config, never a literal — do not conflate
   the two.
3. **Fail loud.** No silent fallback, swallowed error, or skip-with-default.
   Wrong-kind/wrong-shape input raises a named error.
4. **Determinism law.** In instrument mode nothing nondeterministic may influence
   move choice: no unseeded hash-iteration order on choice paths (fixed-seed hasher
   or sorted iteration), no time-based tie-breaks, no thread races.
   `tools/determinism.sh` is the CI gate.
5. **Bench discipline.** A perf-sensitive change ships with a pre-registered
   hotspot, expected gain bracket, and abort threshold; one change = one commit =
   one IQR-gated bench; report nps AND time-to-depth. A measured structural floor is
   a finding, not a failure.
6. **Strength claims.** Ship instrument (fixed depth/nodes), protocol, n, distinct-n
   (identical games deduped), and per-side compute. Search/eval changes are accepted
   or rejected by SPRT over paired balanced openings. Never wall-clock-only.
7. **Tests.** Behavior-named. Pair-move perft against a brute-force reference is the
   movegen oracle; golden win/no-win boards pin rules 2 and 4; tactical suites are
   sha-pinned fixtures with a 10 MB/file ceiling.
8. **Artifacts.** Nets, books, match logs, bench outputs are never committed; a
   committed manifest may sha-index them.
9. **Files.** Single responsibility, ~300-line soft cap; exceeding requires a
   why-justification entry in docs/rule9_justifications.md — not a comment in the
   file, which a comment sweep deletes by accident — that never states a line
   count (counts are derived, never asserted).
10. **Decisions.** Every non-obvious design choice = one ADR line in
    docs/decisions.md (`D-n: choice — reason — what flips it`). Silent architecture
    drift is a breach; amend the ADR instead.
11. **API deferral.** pistol-api stays empty until the API layer is specified. The
    `Engine` trait + line protocol ARE the contract the API will adapt; adding
    transport dependencies anywhere else is a breach.

## Process (every non-trivial change)

DESIGN → REVIEW-design (fresh context, attacks the premise) → IMPL → REVIEW-impl
(fresh context, not the implementer, checks against the design) → RED-TEAM on
rules/data paths (adversarial inputs). Pre-register verdicts before experiments — no
post-hoc threshold moves. Reviewers flag correctness and requirement gaps, not style.

A named decision with more than one viable option is settled by an OPTION MATRIX —
options, costs, failure modes, recommendation — attacked by a fresh-context
DECISION-RED-TEAM subagent BEFORE selection; the surviving option's ADR line records
the strongest surviving attack. An option adopted without a matrix, or a matrix never
attacked, is the same breach as silent architecture drift. Every numeric claim in the
matrix is marked **MEASURED** or **ESTIMATED**; an estimate that could have been
measured in seconds is a finding (D-291).

A pre-registration is reviewed at the revision that GOVERNS the run — that revision
must itself pass a fresh-context review before the first run it governs, and reviews
of superseded revisions do not transfer — and an amendment reopens the review however
small the diff. Dry-run discipline, the instrument's own governing revision, the
criterion/defect-class a registered output must satisfy, the
cost/replication/second-instrument rule for a cheap instrument under doubt, and the
tools/ review's coverage rule are detailed methodology binding exactly as this file
would — see docs/process.md.

REVIEW-design, REVIEW-impl and RED-TEAM are dispatched as subagents with fresh
contexts; the implementing session never reviews its own work. A WP is not landable
while its reviews are outstanding. A session that cannot dispatch subagents states so
and stops after IMPL; the operator launches the reviews. A reviewer finding is
verified with a minimal reproducer before its fix lands; a finding that cannot be
reproduced is recorded as rejected with the attempted reproducer. Each review is
dispatched against a NAMED REVISION — a commit SHA, or a `git stash create` SHA where
the work is uncommitted — stated in the reviewer's report header together with
whether it still matches HEAD. Mutation testing runs in a separate git worktree,
never the live tree: a mutation is a deliberate break, and a break left in the tree
the implementing session is editing is indistinguishable from a regression. A
worktree is REMOVED only after its gitignored `artifacts/` and `sessions/` are
exported to the main tree with a digest receipt — a `sha256sum` list committed or
sha-anchored — because removal takes them with it, and WP-1.8c's four review reports
survive only in a transcript.

THE OPERATOR OVERRULE, a first-class move and not an escape hatch. Where the CODE is
done — tested, green, its mutations dying — and what blocks it is a claim in a
governing DOCUMENT or a rule of this file, the session states the problem in a SHORT
paragraph and asks the operator to OVERRULE. The paragraph names three things and no
more: the finding, why the blocked claim does no work, and what would be deleted. The
operator's answer is an ADR line, and the deletion is the fix. THE TEST IS WHETHER THE
DISPUTED CLAIM CHANGES WHAT ANYONE MAY CONCLUDE — where both sides of a distinction
license the same conclusion it is not a distinction, and it is DELETED rather than
refined. IT IS NOT A LICENCE OVER CORRECTNESS: a finding that names a way the code can
produce a wrong answer is never overruled, only fixed, and the overrule reaches only
prose that constrains nothing (D-424).

A pre-registration registers only what constrains the CONCLUSION; operational
guidance lives in the instrument's own printed message, not in a registered, reviewed
rule — a rule that cannot change any reading is protecting nothing and is prose a
reviewer must still attack. A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING —
state it once, in the section that owns it, and have every other section point there
instead (D-423).

**Closure.** All work on `dev`; `main` = gate-passing merges only. One feature = one
commit, `type(scope): what changed and why it matters`, one line. Commit only when
the operator asks; merges/pushes are operator acts. A gate or test claim in any
report cites the gate's own log output, never a wrapper's exit status.

## Code style (Rust)

- rustfmt default settings and clippy clean (`-D clippy::all`) are mechanical law; a
  style dispute rustfmt can settle is not discussed in reviews.
- Comments say WHY, never WHAT: no narration, no restating names, types, or asserts.
  Line comments (`//`), own line, one space after the sigil. Brief — if a comment
  needs a paragraph, the code or the design doc is the wrong shape.
- No file-top narrative headers. No `//!` module descriptions in ordinary modules; a
  module's name and its public item docs carry its purpose. `//!` is permitted only
  at a crate root, at most a few lines, only if rustdoc genuinely needs it.
- Public items get `///` docs, with `# Errors` / `# Panics` sections where a caller
  would reasonably handle them; no doc that restates the signature.
- Naming follows RFC 430 shapes; no domain-convention exceptions.
- Tests are named for the behaviour pinned, not the function called; deterministic;
  no wall-clock waits.
- When brevity and reviewability collide, reviewability wins. When a comment and a
  test could carry the same fact, the test carries it.

## Environment

- Toolchain: rustup-managed `cargo`/`rustc` on PATH; no pinned toolchain file here.
- The agent shell's `grep` is wrapped by the harness (D-265) — multithreaded,
  order-nondeterministic, visiting a different file set than plain `grep`. Anything
  RECORDED or ADJUDICATED is produced with `/usr/bin/grep` or `git grep`, pinned to a
  revision, sorted `LC_ALL=C sort`; a captured transcript is evidence of shape, never
  "the exact output".
- Never export `CARGO_TARGET_DIR` around `cargo test` in the live tree: several
  gate-test suites build their own scratch cargo workspaces, and a shared target
  directory makes one fixture read another's dep-info. Verification/mutation work
  happens in a separate `git worktree add --detach` with its own `CARGO_TARGET_DIR`,
  never the live tree.
- This machine's `/tmp` is a 24 GiB RAM-backed tmpfs (`tools/ci.sh` preflights it);
  build-heavy work goes on real disk, not `/tmp`. A long-running background job runs
  detached (`setsid nohup`) and is polled, never watched synchronously.

## Pointers

- docs/decisions.md — append-only ADR log (hard rule 10).
- docs/process.md — pre-registration methodology detail (Process section above).
- docs/rule9_justifications.md — where hard rule 9's why lives, one entry per file.
- docs/process_readings.md — the T-bucket of adversarial prereg-paragraph readings
  and their status.
- docs/research/ — the design report (minimax_report.md) and the threat calculus.
- docs/ROADMAP.md — the stage plan (0 foundations through 5 opening book/harness),
  changed only by ADR; its own headers name each stage.
- configs/ — explicit, complete, schema-validated configs (hard rule 1).
- tools/SHELL_CHECKLIST.md — the tools/ review checklist (Process section above).
