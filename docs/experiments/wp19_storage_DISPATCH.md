# WP-1.9 (eval window-map storage) — the commissioning dispatch, archived verbatim

**Why this file exists.** `docs/experiments/matrix_wp19_storage.md` revision 1
quoted a deliverable — *"memory footprint bound stated as a number with its
derivation in a test"* — and attributed it to "the round". The fresh-context
DECISION-RED-TEAM could not check that attribution and reported it as a
requirement that exists nowhere in the tree (`matrix_wp19_storage_REDTEAM.md`
B1), which was correct: `git grep` found all six occurrences inside the citing
document. The requirement is REAL and it is in the operator's dispatch. **The
dispatch was simply never in the tree.**

This is the second time in this designation's history that a reviewer could not
check an adjudication because the dispatch existed nowhere a revision could cite
— the stopped stage-Q package hit it too, and archived its dispatch to
`sessions/WP-1.9/wp19_DISPATCH.md`. **That remedy was incomplete**: `/sessions/`
is gitignored (`.gitignore:25`), so a reviewer pinned to a revision still cannot
resolve it. This copy is in `docs/experiments/`, which is tracked, so it can be
cited at a SHA like any other governing document.

**Provenance.** Operator dispatch opening this session, transcribed from the
session's first message. Nothing is paraphrased and nothing is omitted. Its
`## Game rules` block is CLAUDE.md's own, restated by the dispatch for a fresh
session.

**Status of its claims.** A dispatch is not registered scope. This one says so
itself — *"This dispatch asserts NO mechanism"* — and binds the package to
D-225/D-249/the ROADMAP entry instead. Where its prose and the registered scope
disagree, `docs/experiments/wp19_storage_scope_memo.md` records which governs.
Specifically: the memory-footprint-bound sentence below is a DISPATCH
requirement, not a registered one, and any document leaning on it must cite it
here and say so.

---

# [GROUNDWORK] WP-1.9: eval window-map storage (D-225/D-249), Stage-1 finish

Full round in ONE session: §0 -> premise verification -> design ->
REVIEW-design -> impl -> REVIEW-impl -> equivalence or SPRT track ->
closure, including the Stage-1 arc closure. Overnight capable, delegation
granted in-dispatch (D-382). Long jobs detached, polled. D-401 never read.

Starts only after WP-1.5d's closure commit is on dev with CI green.

Read first: CLAUDE.md, docs/process.md, ROADMAP (the WP-1.9 entry at its
current line), decisions tail (D-473 on), D-225 and D-249 IN FULL, D-220
(Eval::delta), D-192 (ordering hotspot), the eval crate's window/map code,
WP-1.5d closure summary.

## Premise rule (binding, D-477 form)

This dispatch asserts NO mechanism. The registered scope of WP-1.9 lives
in D-225 as amended by D-249 and the ROADMAP entry. First deliverable is
a SCOPE MEMO quoting all three at file:line plus the current eval code
sites they bind. If the registered scope is stale, ambiguous, or
conflicts with anything landed since D-249 (the staged generator, the
solver, D-483's laws): STOP before design, memo on disk, back to the
architect. A premise failure here is a success of the process, not a
failed session (WP-1.5c precedent).

## Game rules (verbatim, binding)

1. Board: unbounded 2-D hexagonal lattice, axial coordinates (q, r); 3 line axes;
   6 neighbours per cell.
2. Win: >=6 own stones contiguous along one axis. Overlines (7+) win. No bans, no
   exact-six, no variants.
3. Turn 1 = ONE stone (origin WLOG). Every later turn = TWO stones by the mover.
4. A win completes the instant any single placed stone forms >=6; the turn's second
   stone is then not played. Sudden death is scored in TURNS; the completing stone is
   the tactically relevant unit.
5. Legal placement: within hex-distance 8 of an existing stone (union of radius-8
   balls around all stones). This is a game RULE — a named constant in pistol-core —
   never a search knob.
6. No captures. No rule-level draws; matches may impose a turn cap — the engine
   treats a cap as an evaluation horizon, never as a game rule.

## §0 First actions

1. Append the paste block D-lines at the next free numbers.
2. Confirm dev green at the WP-1.5d closure HEAD, cited from the gate
   log's own lines.

## Scope (shape only; content comes from the scope memo)

Window-map storage for the eval: the per-axis window pattern state,
stored and updated incrementally under the real move unit (one stone at
turn 1, two after; rule 4 truncation included), as D-225/D-249 register
it. Explicit non-goals regardless of what the memo says: no learned
weights, no codebook training, no new eval terms, no search changes, no
solver contact. This is Stage-2 substrate, not Stage 2.

Design decides and records, from the memo: storage layout; incremental
update and UNDO paths (the undo path is where incremental state rots:
it gets its own tests and its own mutant); rebuild-from-scratch path as
the internal oracle; interaction with Eval::delta (D-220) stated with
quoted sites; memory footprint bound stated as a number with its
derivation in a test, not prose (D-483).

## The two-track rule (decided by measured fact, not preference)

- Track E (equivalence): if the design's claim is that every eval output
  is bit-identical to the incumbent, then the proof is: incremental vs
  rebuild-from-scratch agreement at every node over the determinism
  fixtures AND a full governed-shape game set (both seats, both
  budgets), plus gate-off byte-identity of search output over the 115-
  position receipt set (WP-1.5d(A) precedent). NO SPRT: identity is the
  strongest oracle, a strength run adds nothing. Bench bracket still
  registered and taken (this package exists to be faster or enable
  Stage 2; a regression is an abort per rule 5, hotspot from a profile
  receipt only).
- Track S (SPRT): if ANY eval output differs on ANY input, however
  justified, the package is strength-relevant: full prereg per the
  standing shape (quote-not-paraphrase at a named SHA, fresh
  openings_skip slice with consumed-range receipts, warm-replay
  Criterion 1'', second instrument agreement as registered, slot pass
  D-427, one-run-one-instrument-one-artifact for every number, D-479/
  D-483 throughout).
- The track is claimed in the design and VERIFIED by the equivalence
  harness before the choice binds; a single mismatch flips E to S
  mechanically, no discretion, recorded.

## Rules that bind (pointers)

CLAUDE.md and docs/process.md, all. Receipts rule. Mutant-dies rule.
CLAIM-HOME, D-346, D-424. D-7 determinism (new state cleared on newgame,
seat added). D-479 (both terms of any ratio name artifact and seat).
D-483 (no measured numbers in the design; brackets registered in the
measurement step). D-469 export before worktree removal. Registered
numbers never move (D-374). Caps: one fix round per review; fail outside
the diff = STOP; a document failing twice = STOP and split, no third
revision self-granted (D-481 precedent).

## Development round

1. Scope memo (the premise gate).
2. Design doc: mechanisms, invariants, tests only. REVIEW-design: fresh,
   strongest, one fix round.
3. Impl behind a gate where the design says one is meaningful; default =
   incumbent behaviour. Tests: incremental-vs-rebuild agreement
   (property test over random legal sequences including rule-4
   truncations and turn-1 single stones); undo restores byte-identical
   state; newgame clears. Mutation receipts: break the undo path -> the
   agreement test dies; skip one axis in the update -> dies; skip the
   newgame clear -> determinism seat dies.
4. REVIEW-impl: fresh, strongest, one fix round.
5. Track E or S per the two-track rule, executed as written.
6. Closure: config/pin consequences per track verdict; ADR lines;
   artifacts exported with digests; ROADMAP updated.

## Stage-1 arc closure (part of this WP's closure)

On this package's closure, record in ROADMAP and one D-line: Stage 1
complete — 1.5b staged generation (h1), 1.5c stage-Q census (planning
finding), 1.5d safety-net cap (its recorded verdict), 1.6 quiescence
(h0, re-test scheduled Stage-2 exit), 1.7 ordering heuristics (h0), 1.8
arc (solver certified, gated, re-test on nps jump), 1.9 window-map
storage (this verdict). Open Stage-1 debts routed by name to the
final-cleanup package. Next per D-471 as amended: the Stage-3 detector.

## STOP protocol

STOP on: premise failure; determinism exit 3; CI red after one fix
round; failure outside the diff; equivalence mismatch that the session
is tempted to argue rather than flip to Track S; any cap exhausted. On
STOP: tree clean or WIP on `wp19-stopped`, never dev; no detached
processes, receipt; exports per D-469; summary at sessions/WP-1.9/,
plain language first, the decision owed named.

## Paste block (append at next free D-numbers)

D-49a: Roadmap amendment to D-471: WP-1.9 (eval window-map storage per
D-225/D-249) runs after WP-1.5d closes and before the Stage-3 detector —
it touches engine code and is therefore serial, not parallel Stage-2
prep; on its closure Stage 1 is recorded complete and the detector
package opens; D-471's flip condition is unchanged and now lands on
prepared substrate.

D-49b: Two-track law for substrate refactors: a package claiming
bit-identical outputs proves it by incremental-vs-oracle agreement plus
gate-off byte-identity and takes NO strength run (identity is the
strongest oracle); a single output mismatch flips the package to the
full SPRT track mechanically, no discretion; the claimed track is
verified by harness before it binds.

## DONE

- Scope memo landed, premises quoted, or a clean premise-STOP.
- Design and impl through their reviews, mutants dead as listed.
- Track E receipts (agreement + byte-identity + bench bracket) or
  Track S verdict (n, distinct_n, pentanomial, llr_pair) recorded.
- Stage-1 arc closure D-line and ROADMAP entry landed.
- `tools/ci.sh` all gates at closure HEAD, tree clean, artifacts
  exported with digests, summary on disk, ONE LINE FOR THE MORNING
  first.

---

## The four architect rulings that opened the package

Transcribed from the operator's second message, which answered the scope memo's
four owed rulings. Landed as D-496, D-497, D-498 and D-499 respectively.

D-49w: WP-1.9 (eval window-map storage) runs NOW: the ROADMAP.md:294-296
and :320-323 precedence of WP-1.10 is displaced by name, on the memo's
verified grounds that neither recorded reason is a technical blocker at
HEAD (gate-off byte-identity is a direct two-binary diff per the
WP-1.5d(A) precedent and does not route through search_oracle_check.sh;
bench_delta.sh is on D-289's DRIVEN list); the Stage-1 arc closure will
record Stage 1 CLOSED WITH NAMED RESIDUE — WP-1.4 and WP-1.10 carried
open, licensed, listed in the closure line — never a silent completeness
claim.

D-49x: The storage option matrix + DECISION-RED-TEAM is owed and is
added to the WP-1.9 round as its first design step (matrix -> red-team ->
selection -> design, WP-1.5d shape); an unattacked matrix is the breach
CLAUDE.md says it is, Track E's being settled notwithstanding — the
track is settled, the container is not.

D-49y: D-225's blanket "never iterated" licence is NARROWED, not
struck: the surviving obligation is canonical equality — any store shape
must yield iteration-order-independent, tombstone-independent comparison
of carried state, pinned by the rotated-unwind test
(eval_incremental_tests.rs:118-140) as the driving test; a store
retaining tombstones that reach the derived PartialEq fails the
obligation by construction.

D-49z: D-258 BINDS for this package; five of its six obligations
transplant; the sixth ("carry all ten maintained sets") is STRUCK with
its ground recorded — the referent is the solver's sets (CLASS_COUNT 5 x
2 seats), pistol-eval holds one map, and an obligation without a
referent is D-424's deletable distinction.
